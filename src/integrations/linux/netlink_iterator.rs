use crate::integrations::linux::ffi::{tcp_info, *};
use crate::types::error::*;
use crate::types::*;
use libc::*;
use std;
use std::io;
use std::mem::size_of;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[cfg(target_env = "musl")]
#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}

const TCPF_ALL: __u32 = 0xFFF;
const SOCKET_BUFFER_SIZE: size_t = 8192;

pub struct NetlinkIterator {
    protocol: __u8,
    recv_buf: [u8; SOCKET_BUFFER_SIZE],
    socket: i32,
    nlh: *const nlmsghdr,
    numbytes: isize,
    nlmsg_ok: bool,
}

impl NetlinkIterator {
    pub unsafe fn new(family: __u8, protocol: __u8) -> Result<Self, Error> {
        let socket = socket(AF_NETLINK as i32, SOCK_DGRAM, NETLINK_INET_DIAG);

        if socket == -1 {
            return Result::Err(Error::OsError(io::Error::last_os_error()));
        }

        send_diag_msg(socket, family, protocol)?;
        Ok(NetlinkIterator {
            protocol,
            socket,
            recv_buf: [0u8; SOCKET_BUFFER_SIZE as usize],
            nlh: std::ptr::null(),
            numbytes: 0,
            nlmsg_ok: false,
        })
    }
}

impl Iterator for NetlinkIterator {
    type Item = Result<SocketInfo, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            loop {
                if !self.nlmsg_ok {
                    let buf_ptr = &mut self.recv_buf[0] as *mut u8 as *mut c_void;
                    self.numbytes = recv(self.socket, buf_ptr, SOCKET_BUFFER_SIZE, 0);
                    self.nlh = buf_ptr as *const u8 as *const nlmsghdr;
                }
                self.nlmsg_ok = NLMSG_OK!(self.nlh, self.numbytes);
                if self.nlmsg_ok {
                    if (&*self.nlh).nlmsg_type == NLMSG_DONE as u16 {
                        return None;
                    }
                    if (&*self.nlh).nlmsg_type == NLMSG_ERROR as u16 {
                        // TODO: parse error code from msg properly
                        // https://www.infradead.org/~tgr/libnl/doc/core.html#core_errmsg
                        return Some(Result::Err(Error::NetLinkError));
                    }
                    let diag_msg = NLMSG_DATA!(self.nlh) as *const inet_diag_msg;
                    let rtalen =
                        (&*self.nlh).nlmsg_len as usize - NLMSG_LENGTH!(size_of::<inet_diag_msg>());
                    let socket_info = parse_diag_msg(&*diag_msg, self.protocol, rtalen);
                    self.nlh = NLMSG_NEXT!(self.nlh, self.numbytes);
                    return Some(socket_info);
                }
            }
        }
    }
}

impl Drop for NetlinkIterator {
    fn drop(&mut self) {
        unsafe {
            let _ = try_close(self.socket);
        }
    }
}

unsafe fn send_diag_msg(sockfd: c_int, family: __u8, protocol: __u8) -> Result<(), Error> {
    let mut sa: sockaddr_nl = std::mem::zeroed();
    sa.nl_family = AF_NETLINK as sa_family_t;
    sa.nl_pid = 0;
    sa.nl_groups = 0;
    let mut conn_req = inet_diag_req_v2 {
        family,
        protocol,
        ext: 1 << (INET_DIAG_INFO - 1),
        pad: 0,
        states: TCPF_ALL,
        id: Default::default(),
    };
    let mut nlh = nlmsghdr {
        nlmsg_len: NLMSG_LENGTH!(size_of::<inet_diag_req_v2>()) as __u32,
        nlmsg_type: SOCK_DIAG_BY_FAMILY,
        nlmsg_flags: (NLM_F_DUMP | NLM_F_REQUEST) as u16,
        nlmsg_seq: 0,
        nlmsg_pid: 0,
    };
    let mut iov = [
        iovec {
            iov_base: &mut nlh as *mut _ as *mut c_void,
            iov_len: size_of::<nlmsghdr>() as size_t,
        },
        iovec {
            iov_base: &mut conn_req as *mut _ as *mut c_void,
            iov_len: size_of::<inet_diag_req_v2>() as size_t,
        },
    ];

    let mut msg: msghdr = std::mem::zeroed();
    msg.msg_name = &mut sa as *mut _ as *mut _;
    msg.msg_namelen = size_of::<sockaddr_nl>() as c_uint;
    msg.msg_iov = &mut iov[0];
    msg.msg_iovlen = 2;

    match sendmsg(sockfd, &msg, 0) {
        -1 => Result::Err(Error::OsError(io::Error::last_os_error())),
        _ => Result::Ok(()),
    }
}

unsafe fn parse_diag_msg(
    diag_msg: &inet_diag_msg,
    protocol: __u8,
    rtalen: usize,
) -> Result<SocketInfo, Error> {
    let src_port = u16::from_be(diag_msg.id.sport);
    let dst_port = u16::from_be(diag_msg.id.dport);
    let src_ip = parse_ip(diag_msg.family, &diag_msg.id.src)?;
    let dst_ip = parse_ip(diag_msg.family, &diag_msg.id.dst)?;

    let sock_info = match protocol as i32 {
        IPPROTO_TCP => SocketInfo {
            protocol_socket_info: ProtocolSocketInfo::Tcp(TcpSocketInfo {
                local_addr: src_ip,
                local_port: src_port,
                remote_addr: dst_ip,
                remote_port: dst_port,
                state: parse_tcp_state(diag_msg, rtalen),
            }),
            associated_pids: Vec::with_capacity(0),
            inode: diag_msg.inode,
            uid: diag_msg.uid,
        },
        IPPROTO_UDP => SocketInfo {
            protocol_socket_info: ProtocolSocketInfo::Udp(UdpSocketInfo {
                local_addr: src_ip,
                local_port: src_port,
            }),
            associated_pids: Vec::with_capacity(0),
            inode: diag_msg.inode,
            uid: diag_msg.uid,
        },
        _ => return Err(Error::UnknownProtocol(protocol)),
    };

    Ok(sock_info)
}

unsafe fn parse_ip(family: u8, bytes: &[__be32; 4]) -> Result<IpAddr, Error> {
    let addr = match family as i32 {
        AF_INET => IpAddr::V4(Ipv4Addr::from(
            *(&bytes[0] as *const __be32 as *const [u8; 4]),
        )),
        AF_INET6 => IpAddr::V6(Ipv6Addr::from(
            *(bytes as *const [__be32; 4] as *const u8 as *const [u8; 16]),
        )),
        _ => return Err(Error::UnsupportedSocketFamily(family as u32)),
    };

    Ok(addr)
}

unsafe fn parse_tcp_state(diag_msg: &inet_diag_msg, rtalen: usize) -> TcpState {
    let mut len = rtalen as isize;
    let mut attr = (diag_msg as *const inet_diag_msg).offset(1) as *const rtattr;
    while RTA_OK!(attr, len) {
        if (&*attr).rta_type == INET_DIAG_INFO as u16 {
            let tcpi = &*(RTA_DATA!(attr) as *const tcp_info);
            return TcpState::from(tcpi.state);
        }
        attr = RTA_NEXT!(attr, len);
    }
    TcpState::TimeWait
}

unsafe fn try_close(sockfd: c_int) -> Result<(), Error> {
    match close(sockfd) {
        -1 => Result::Err(Error::OsError(io::Error::last_os_error())),
        _ => Result::Ok(()),
    }
}
