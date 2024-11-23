//! This implementation is based on the `examples/dump_ipv4.rs` from `https://github.com/rust-netlink/netlink-packet-sock-diag`.
use crate::types::error::*;
use crate::types::*;
use std;
use std::io;
use std::mem::size_of;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use netlink_packet_core::{
    NetlinkHeader, NetlinkMessage, NetlinkPayload, NLM_F_DUMP, NLM_F_REQUEST,
};
use netlink_packet_sock_diag::inet::InetResponse;
use netlink_packet_sock_diag::{
    constants::*,
    inet::{ExtensionFlags, InetRequest, SocketId, StateFlags},
    SockDiagMessage,
};
use netlink_sys::{protocols::NETLINK_SOCK_DIAG, Socket, SocketAddr};

const SOCKET_BUFFER_SIZE: usize = 8192;

pub struct NetlinkIterator {
    protocol: u8,
    recv_buf: [u8; SOCKET_BUFFER_SIZE],
    socket: Socket,
    offset: usize,
    size: usize,
    is_done: bool,
}

impl NetlinkIterator {
    pub fn new(family: u8, protocol: u8) -> Result<Self, Error> {
        let mut socket = Socket::new(NETLINK_SOCK_DIAG)?;
        let _port_number = socket.bind_auto()?.port_number();
        socket.connect(&SocketAddr::new(0, 0))?;

        let mut nl_hdr = NetlinkHeader::default();
        nl_hdr.flags = NLM_F_REQUEST | NLM_F_DUMP;
        let mut packet = NetlinkMessage::new(
            nl_hdr,
            SockDiagMessage::InetRequest(InetRequest {
                family,
                protocol,
                extensions: ExtensionFlags::empty(),
                states: StateFlags::all(),
                socket_id: SocketId::new_v4(),
            })
            .into(),
        );

        packet.finalize();

        let mut buf = vec![0; packet.buffer_len()];
        packet.serialize(&mut buf[..]);
        socket.send(&buf[..], 0)?;

        Ok(NetlinkIterator {
            protocol,
            socket,
            recv_buf: [0u8; SOCKET_BUFFER_SIZE as usize],
            offset: 0,
            size: 0,
            is_done: false,
        })
    }

    fn try_read_next_packet(&mut self) -> Result<Option<SocketInfo>, Error> {
        if self.is_done {
            return Ok(None);
        }

        loop {
            if self.offset >= self.size {
                self.size = self.socket.recv(&mut &mut self.recv_buf[..], 0)?;
                self.offset = 0;
            }

            let bytes = &self.recv_buf[self.offset..self.size];

            let rx_packet: NetlinkMessage<SockDiagMessage> =
                match NetlinkMessage::deserialize(bytes) {
                    Ok(rx_packet) => rx_packet,
                    Err(e) => {
                        // Avoid endless loop in case of a deserialization failure.
                        self.is_done = true;
                        return Err(Error::from(e));
                    }
                };
            self.offset += rx_packet.header.length as usize;

            match rx_packet.payload {
                NetlinkPayload::Noop => {}
                NetlinkPayload::InnerMessage(SockDiagMessage::InetResponse(response)) => {
                    return Ok(Some(parse_diag_msg(&response, self.protocol)?));
                }
                NetlinkPayload::Done(_) => {
                    self.is_done = true;
                    return Ok(None);
                }
                _ => return Ok(None),
            }
        }
    }
}

impl Iterator for NetlinkIterator {
    type Item = Result<SocketInfo, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_read_next_packet().transpose()
    }
}

fn parse_diag_msg(diag_msg: &InetResponse, protocol: u8) -> Result<SocketInfo, Error> {
    let src_port = diag_msg.header.socket_id.source_port;
    let dst_port = diag_msg.header.socket_id.destination_port;
    let src_ip = diag_msg.header.socket_id.source_address;
    let dst_ip = diag_msg.header.socket_id.destination_address;

    let sock_info = match protocol {
        IPPROTO_TCP => SocketInfo {
            protocol_socket_info: ProtocolSocketInfo::Tcp(TcpSocketInfo {
                local_addr: src_ip,
                local_port: src_port,
                remote_addr: dst_ip,
                remote_port: dst_port,
                state: TcpState::from(diag_msg.header.state),
            }),
            associated_pids: vec![],
            inode: diag_msg.header.inode,
            uid: diag_msg.header.uid,
        },
        IPPROTO_UDP => SocketInfo {
            protocol_socket_info: ProtocolSocketInfo::Udp(UdpSocketInfo {
                local_addr: src_ip,
                local_port: src_port,
            }),
            associated_pids: vec![],
            inode: diag_msg.header.inode,
            uid: diag_msg.header.uid,
        },
        _ => return Err(Error::UnknownProtocol(protocol)),
    };

    Ok(sock_info)
}
