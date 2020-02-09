#![allow(unused)]

use byteorder::{ByteOrder, NetworkEndian};

use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;
use std::fmt::{self, Display};
use std::mem::MaybeUninit;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::os::raw::{c_int, c_uint, c_void};
use std::ptr;
use std::{io, mem};

use crate::integrations::osx::ffi::libproc::*;
use crate::types::error::Error;
use crate::types::{AddressFamilyFlags, ProtocolFlags};
use crate::{ProtocolSocketInfo, SocketInfo, TcpSocketInfo, TcpState, UdpSocketInfo};

pub type PID = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProcFDInfo {
    pub proc_fd: i32,
    pub proc_fdtype: ProcFDType,
}

impl Default for ProcFDInfo {
    fn default() -> Self {
        ProcFDInfo {
            proc_fd: 0,
            // Atalk == 0
            proc_fdtype: ProcFDType::Atalk,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Primitive)]
pub enum ProcType {
    ProcAllPIDS = 1,
    ProcPGRPOnly = 2,
    ProcTTYOnly = 3,
    ProcUIDOnly = 4,
    ProcRUIDOnly = 5,
    ProcPPIDOnly = 6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Primitive)]
pub enum ProcFDType {
    Atalk = 0,
    Vnode = 1,
    Socket = 2,
    PSHM = 3,
    PSEM = 4,
    Kqueue = 5,
    Pipe = 6,
    FsEvents = 7,
    NetPolicy = 9,
}

// Adapter from proc_info.h
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Primitive)]
pub enum SockInfo {
    Generic = 0,
    In = 1,
    Tcp = 2,
    Un = 3,
    Ndrv = 4,
    Kern_event = 5,
    Kern_ctl = 6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Primitive)]
pub enum SocketFamily {
    AF_UNSPEC = 0,
    /* unspecified */
    AF_UNIX = 1,
    /* local to host (pipes) */
    AF_INET = 2,
    /* internetwork: UDP, TCP, etc. */
    AF_IMPLINK = 3,
    /* arpanet imp addresses */
    AF_PUP = 4,
    /* pup protocols: e.g. BSP */
    AF_CHAOS = 5,
    /* mit CHAOS protocols */
    AF_NS = 6,
    /* XEROX NS protocols */
    AF_ISO = 7,
    /* ISO protocols */
    AF_ECMA = 8,
    /* European computer manufacturers */
    AF_DATAKIT = 9,
    /* datakit protocols */
    AF_CCITT = 10,
    /* CCITT protocols, X.25 etc */
    AF_SNA = 11,
    /* IBM SNA */
    AF_DECnet = 12,
    /* DECnet */
    AF_DLI = 13,
    /* DEC Direct data link interface */
    AF_LAT = 14,
    /* LAT */
    AF_HYLINK = 15,
    /* NSC Hyperchannel */
    AF_APPLETALK = 16,
    /* Apple Talk */
    AF_ROUTE = 17,
    /* Internal Routing Protocol */
    AF_LINK = 18,
    /* Link layer interface */
    pseudo_AF_XTP = 19,
    /* eXpress Transfer Protocol (no AF) */
    AF_COIP = 20,
    /* connection-oriented IP, aka ST II */
    AF_CNT = 21,
    /* Computer Network Technology */
    pseudo_AF_RTIP = 22,
    /* Help Identify RTIP packets */
    AF_IPX = 23,
    /* Novell Internet Protocol */
    AF_SIP = 24,
    /* Simple Internet Protocol */
    pseudo_AF_PIP = 25,
    /* Help Identify PIP packets */
    AF_NDRV = 27,
    /* Network Driver 'raw' access */
    AF_ISDN = 28,
    /* Integrated Services Digital Network */
    pseudo_AF_KEY = 29,
    /* Internal key-management function */
    AF_INET6 = 30,
    /* IPv6 */
    AF_NATM = 31,
    /* native ATM access */
    AF_SYSTEM = 32,
    /* Kernel event messages */
    AF_NETBIOS = 33,
    /* NetBIOS */
    AF_PPP = 34,
    /* PPP communication protocol */
    pseudo_AF_HDRCMPLT = 35,
    /* Used by BPF to not rewrite headers output routine */
    AF_RESERVED_36 = 36,
    /* Reserved for internal usage */
    AF_IEEE80211 = 37,
    /* IEEE 802.11 protocol */
    AF_UTUN = 38,
    AF_MAX = 40,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum PidInfoFlavor {
    // list of struct proc_fdinfo
    ListFDs = PROC_PIDLISTFDS as isize,
    // struct proc_taskallinfo
    TaskAllInfo = PROC_PIDTASKALLINFO as isize,
    TBSDInfo = 3,
    TaskInfo = 4,
    ThreadInfo = 5,
    ListThreads = 6,
    RegionInfo = 7,
    RegionPathInfo = 8,
    VNodePathInfo = 9,
    ThreadPathInfo = 10,
    PathInfo = 11,
    WorkQueueInfo = 12,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Primitive)]
pub enum TCPSocketState {
    CLOSED = 0,
    /* closed */
    LISTEN = 1,
    /* listening for connection */
    SYN_SENT = 2,
    /* active, have sent syn */
    SYN_RECEIVED = 3,
    /* have send and received syn */
    ESTABLISHED = 4,
    /* established */
    CLOSE_WAIT = 5,
    /* rcvd fin, waiting for close */
    FIN_WAIT_1 = 6,
    /* have closed, sent fin */
    CLOSING = 7,
    /* closed xchd FIN; await FIN ACK */
    LAST_ACK = 8,
    /* had fin and close; await FIN ACK */
    FIN_WAIT_2 = 9,
    /* have closed, fin is acked */
    TIME_WAIT = 10,
    /* in 2*msl quiet wait after close */
}

impl From<TCPSocketState> for TcpState {
    fn from(s: TCPSocketState) -> Self {
        match s {
            TCPSocketState::CLOSED => TcpState::Closed,
            TCPSocketState::LISTEN => TcpState::Listen,
            TCPSocketState::SYN_SENT => TcpState::SynSent,
            TCPSocketState::SYN_RECEIVED => TcpState::SynReceived,
            TCPSocketState::ESTABLISHED => TcpState::Established,
            TCPSocketState::CLOSE_WAIT => TcpState::CloseWait,
            TCPSocketState::FIN_WAIT_1 => TcpState::FinWait1,
            TCPSocketState::CLOSING => TcpState::Closing,
            TCPSocketState::LAST_ACK => TcpState::LastAck,
            TCPSocketState::FIN_WAIT_2 => TcpState::FinWait2,
            TCPSocketState::TIME_WAIT => TcpState::TimeWait,
        }
    }
}

impl ProcFDInfo {
    fn try_from_proc_fdinfo(other: proc_fdinfo) -> Result<Self, Error> {
        Ok(ProcFDInfo {
            proc_fd: other.proc_fd,
            proc_fdtype: ProcFDType::from_i32(other.proc_fdtype as i32)
                .ok_or_else(|| Error::NotAValidFDType(other.proc_fdtype))?,
        })
    }
}

// TODO: This can be extended to hold different kinds of FDInformation (tasks, thread, etc..)
pub enum FDInformation {
    SocketInfo(socket_fdinfo),

    #[doc(hidden)]
    __Nonexhaustive,
}

pub fn list_pids(proc_types: ProcType) -> Result<Vec<PID>, Error> {
    let number_of_pids;

    unsafe {
        number_of_pids = proc_listpids(proc_types as c_uint, 0, ptr::null_mut(), 0);
    }

    if number_of_pids < 0 {
        return Err(Error::FailedToListProcesses(io::Error::from_raw_os_error(
            number_of_pids,
        )));
    }

    let mut pids: Vec<PID> = Vec::new();
    pids.resize_with(number_of_pids as usize, Default::default);

    let return_code = unsafe {
        proc_listpids(
            proc_types as c_uint,
            0,
            pids.as_mut_ptr() as *mut c_void,
            (pids.len() * mem::size_of::<PID>()) as i32,
        )
    };

    if return_code <= 0 {
        return Err(Error::FailedToListProcesses(io::Error::from_raw_os_error(
            return_code,
        )));
    }

    // Sometimes the OS returns excessive zero elements, so we truncate them.
    Ok(pids.into_iter().filter(|f| *f > 0).collect())
}

pub fn list_all_fds_for_pid(pid: PID) -> Result<Vec<ProcFDInfo>, Error> {
    // We need to call proc_pidinfo twice, one time to get needed buffer size.
    // A second time to actually populate buffer.
    let buffer_size = unsafe {
        proc_pidinfo(
            pid as c_int,
            PROC_PIDLISTFDS as c_int,
            0,
            ptr::null_mut(),
            0,
        )
    };

    if buffer_size <= 0 {
        return Err(Error::FailedToListProcesses(io::Error::from_raw_os_error(
            buffer_size,
        )));
    }

    let number_of_fds = buffer_size as usize / mem::size_of::<proc_fdinfo>();

    let mut fds: Vec<proc_fdinfo> = Vec::new();
    fds.resize_with(number_of_fds as usize, || proc_fdinfo {
        proc_fd: 0,
        proc_fdtype: 0,
    });

    let return_code = unsafe {
        proc_pidinfo(
            pid as c_int,
            PROC_PIDLISTFDS as c_int,
            0,
            fds.as_mut_ptr() as *mut c_void,
            buffer_size,
        )
    };

    if return_code <= 0 {
        Err(Error::FailedToListProcesses(io::Error::from_raw_os_error(
            return_code,
        )))
    } else {
        Ok(fds
            .into_iter()
            .map(|fd| ProcFDInfo::try_from_proc_fdinfo(fd).unwrap_or_default())
            .collect())
    }
}

pub fn get_fd_information(pid: PID, fd: ProcFDInfo) -> Result<FDInformation, Error> {
    match fd.proc_fdtype {
        ProcFDType::Socket => {
            let mut sinfo: MaybeUninit<socket_fdinfo> = MaybeUninit::uninit();

            let return_code = unsafe {
                proc_pidfdinfo(
                    pid,
                    fd.proc_fd,
                    PROC_PIDFDSOCKETINFO as i32,
                    sinfo.as_mut_ptr() as *mut c_void,
                    mem::size_of::<socket_fdinfo>() as i32,
                )
            };

            if return_code < 0 {
                Err(Error::FailedToQueryFileDescriptors(
                    io::Error::from_raw_os_error(return_code),
                ))
            } else {
                Ok(FDInformation::SocketInfo(unsafe { sinfo.assume_init() }))
            }
        }
        _ => Err(Error::UnsupportedFileDescriptor),
    }
}

fn get_local_addr(family: SocketFamily, saddr: in_sockinfo) -> Result<IpAddr, Error> {
    // Unsafe because of union access, but we check the type of address before accessing.
    match family {
        SocketFamily::AF_INET => {
            let addr = unsafe { saddr.insi_laddr.ina_46.i46a_addr4.s_addr };
            Ok(IpAddr::V4(Ipv4Addr::from(u32::from_be(addr))))
        }
        SocketFamily::AF_INET6 => {
            let addr = unsafe { &saddr.insi_laddr.ina_6.__u6_addr.__u6_addr8 };
            let mut ipv6_addr = [0_u16; 8];
            NetworkEndian::read_u16_into(addr, &mut ipv6_addr);
            Ok(IpAddr::V6(Ipv6Addr::from(ipv6_addr)))
        }
        _ => Err(Error::UnsupportedSocketFamily(family as u32)),
    }
}

fn get_remote_addr(family: SocketFamily, saddr: in_sockinfo) -> Result<IpAddr, Error> {
    // Unsafe because of union access, but we check the type of address before accessing.
    match family {
        SocketFamily::AF_INET => {
            let addr = unsafe { saddr.insi_faddr.ina_46.i46a_addr4.s_addr };
            Ok(IpAddr::V4(Ipv4Addr::from(u32::from_be(addr))))
        }
        SocketFamily::AF_INET6 => {
            let addr = unsafe { &saddr.insi_faddr.ina_6.__u6_addr.__u6_addr8 };
            let mut ipv6_addr = [0_u16; 8];
            NetworkEndian::read_u16_into(addr, &mut ipv6_addr);
            Ok(IpAddr::V6(Ipv6Addr::from(ipv6_addr)))
        }
        _ => Err(Error::UnsupportedSocketFamily(family as u32)),
    }
}

fn parse_tcp_socket_info(pid: PID, fd: ProcFDInfo, sinfo: socket_fdinfo) -> Option<TcpSocketInfo> {
    let sock_info = sinfo.psi;
    let family = match SocketFamily::from_i32(sock_info.soi_family) {
        Some(family) => family,
        None => return None,
    };
    let socket_kind = SockInfo::from_i32(sock_info.soi_kind)?;

    // Access to union field in unsafe, but we already checked that this is a TCP connection.
    assert!(socket_kind == SockInfo::Tcp);
    let tcp_in = unsafe { sock_info.soi_proto.pri_tcp };

    let tcp_sockaddr_in = tcp_in.tcpsi_ini;

    let connection_state = TCPSocketState::from_i32(tcp_in.tcpsi_state)?;
    let remote_address = get_remote_addr(family, tcp_sockaddr_in).ok()?;
    let local_address = get_local_addr(family, tcp_sockaddr_in).ok()?;

    let lport_bytes: [u8; 4] = i32::to_le_bytes(tcp_sockaddr_in.insi_lport);
    let fport_bytes: [u8; 4] = i32::to_le_bytes(tcp_sockaddr_in.insi_fport);

    let socket_info = TcpSocketInfo {
        local_addr: local_address,
        local_port: NetworkEndian::read_u16(&lport_bytes),
        remote_addr: remote_address,
        remote_port: NetworkEndian::read_u16(&fport_bytes),
        state: connection_state.into(),
    };

    Some(socket_info)
}

fn parse_udp_socket_info(pid: PID, fd: ProcFDInfo, sinfo: socket_fdinfo) -> Option<UdpSocketInfo> {
    let sock_info = sinfo.psi;
    let family = match SocketFamily::from_i32(sock_info.soi_family) {
        Some(family) => family,
        None => return None,
    };
    let socket_kind = SockInfo::from_i32(sock_info.soi_kind)?;

    // Access to union field in unsafe, but we already checked that this is a In connection.
    assert!(socket_kind == SockInfo::In);
    let in_socket_info = unsafe { sock_info.soi_proto.pri_in };

    let local_address = get_local_addr(family, in_socket_info).ok()?;

    let lport_bytes: [u8; 4] = i32::to_le_bytes(in_socket_info.insi_lport);

    let sock_info = UdpSocketInfo {
        local_addr: local_address,
        local_port: NetworkEndian::read_u16(&lport_bytes),
    };

    Some(sock_info)
}

pub fn iterate_netstat_info(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
) -> Result<impl Iterator<Item = Result<SocketInfo, Error>>, Error> {
    let ipv4 = af_flags.contains(AddressFamilyFlags::IPV4);
    let ipv6 = af_flags.contains(AddressFamilyFlags::IPV6);
    let tcp = proto_flags.contains(ProtocolFlags::TCP);
    let udp = proto_flags.contains(ProtocolFlags::UDP);

    let pids = list_pids(ProcType::ProcAllPIDS)?;

    let mut results = vec![];

    for pid in pids {
        // This will fail on PermissionDenied if we are not sufficiently privileged.
        // We do not return on a specific pid failure,
        // since some of them may fail randomly (unexpectedly closed etc..)
        let fds = match list_all_fds_for_pid(pid) {
            Ok(fds) => fds,
            Err(e) => {
                continue;
            }
        };

        for fd in fds {
            if fd.proc_fdtype == ProcFDType::Socket {
                let fd_information = match get_fd_information(pid, fd) {
                    Ok(fd_information) => fd_information,
                    Err(e) => {
                        results.push(Err(e));
                        continue;
                    }
                };

                match fd_information {
                    FDInformation::SocketInfo(sinfo) => {
                        if ipv4 && sinfo.psi.soi_family == AF_INET as i32
                            || ipv6 && sinfo.psi.soi_family == AF_INET6 as i32
                        {
                            if tcp && sinfo.psi.soi_protocol == IPPROTO_TCP as i32 {
                                if let Some(row) = parse_tcp_socket_info(pid, fd, sinfo) {
                                    results.push(Ok(SocketInfo {
                                        protocol_socket_info: ProtocolSocketInfo::Tcp(row),
                                        associated_pids: vec![pid as u32],
                                    }));
                                }
                            } else if udp && sinfo.psi.soi_protocol == IPPROTO_UDP as i32 {
                                if let Some(row) = parse_udp_socket_info(pid, fd, sinfo) {
                                    results.push(Ok(SocketInfo {
                                        protocol_socket_info: ProtocolSocketInfo::Udp(row),
                                        associated_pids: vec![pid as u32],
                                    }));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(results.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_pids() {
        println!("{:#?}", list_pids(ProcType::ProcAllPIDS).unwrap());
        assert!(list_pids(ProcType::ProcAllPIDS).unwrap().len() > 5);
    }

    #[test]
    fn test_list_fds_for_pid() {
        let pids = list_pids(ProcType::ProcAllPIDS).unwrap();
        for pid in pids.iter().take(100) {
            if let Ok(fds) = list_all_fds_for_pid(*pid) {
                println!("{} {:#?}", pid, fds);
                assert!(!fds.is_empty());
            }
        }
    }

    #[test]
    fn test_netstat() {
        let ns: Vec<_> = iterate_netstat_info(AddressFamilyFlags::all(), ProtocolFlags::all())
            .unwrap()
            .collect();
        println!("{:#?}", ns);
        assert!(!ns.is_empty());
    }
}
