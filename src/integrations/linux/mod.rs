#[macro_use]
mod ffi;

mod api;
mod ext;
pub mod filter;
mod netlink_iterator;
mod procfs;

pub use self::api::*;

#[cfg(test)]
mod tests {
    use super::filter;
    use super::*;
    use crate::error::Error;
    use crate::types::*;
    use std::net::{IpAddr, Ipv4Addr, TcpListener, UdpSocket};
    use std::process;

    #[test]
    fn listening_tcp_socket_is_found_filtered() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();

        let open_port = listener.local_addr().unwrap().port();
        let pid = process::id();

        let af_flags = AddressFamilyFlags::all();
        let proto_flags = ProtocolFlags::TCP;

        let sock_info = iterate_sockets_info_filtered(
            af_flags,
            proto_flags,
            &[filter::PortCondType::Src.eq(open_port)],
            true,
        )
        .unwrap();

        let sock = sock_info
            .filter_map(Result::ok)
            .find(|s| s.associated_pids.contains(&pid))
            .unwrap();

        assert_eq!(
            sock.protocol_socket_info,
            ProtocolSocketInfo::Tcp(TcpSocketInfo {
                local_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
                local_port: open_port,
                remote_addr: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                remote_port: 0,
                state: TcpState::Listen,
            })
        );
        assert_eq!(sock.associated_pids, vec![pid]);
    }
}
