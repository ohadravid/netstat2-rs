//! Cross-platform library to retrieve network sockets information.
//! Tries to be optimal by using low-level OS APIs instead of command line utilities.
//! Provides unified interface and returns data structures which may have additional fields depending on platform.
//!
//! # Example
//!
//! ```rust
//! use netstat2::*;
//!
//! # fn main() -> Result<(), netstat2::error::Error> {
//! let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
//! let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
//! let sockets_info = get_sockets_info(af_flags, proto_flags)?;
//!
//! for si in sockets_info {
//!     match si.protocol_socket_info {
//!         ProtocolSocketInfo::Tcp(tcp_si) => println!(
//!             "TCP {}:{} -> {}:{} {:?} - {}",
//!             tcp_si.local_addr,
//!             tcp_si.local_port,
//!             tcp_si.remote_addr,
//!             tcp_si.remote_port,
//!             si.associated_pids,
//!             tcp_si.state
//!         ),
//!         ProtocolSocketInfo::Udp(udp_si) => println!(
//!             "UDP {}:{} -> *:* {:?}",
//!             udp_si.local_addr, udp_si.local_port, si.associated_pids
//!         ),
//!     }
//! }
//! #     Ok(())
//! # }
//! ```
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

#[macro_use]
extern crate bitflags;

mod integrations;
mod types;

pub use crate::integrations::*;
pub use crate::types::*;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, TcpListener, UdpSocket};
    use std::process;

    #[test]
    fn listening_tcp_socket_is_found() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();

        let open_port = listener.local_addr().unwrap().port();
        let pid = process::id();

        let af_flags = AddressFamilyFlags::all();
        let proto_flags = ProtocolFlags::TCP;

        let sock_info = get_sockets_info(af_flags, proto_flags).unwrap();

        assert!(!sock_info.is_empty());

        let sock = sock_info
            .into_iter()
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

    #[test]
    fn listening_udp_socket_is_found() {
        let listener = UdpSocket::bind("127.0.0.1:0").unwrap();

        let open_port = listener.local_addr().unwrap().port();
        let pid = process::id();

        let af_flags = AddressFamilyFlags::all();
        let proto_flags = ProtocolFlags::UDP;

        let sock_info = get_sockets_info(af_flags, proto_flags).unwrap();

        assert!(!sock_info.is_empty());

        let sock = sock_info
            .into_iter()
            .find(|s| s.associated_pids.contains(&pid))
            .unwrap();

        assert_eq!(
            sock.protocol_socket_info,
            ProtocolSocketInfo::Udp(UdpSocketInfo {
                local_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
                local_port: open_port,
            })
        );
        assert_eq!(sock.associated_pids, vec![pid]);
    }

    #[test]
    fn result_is_ok_for_any_flags() {
        let af_flags_combs = (0..AddressFamilyFlags::all().bits() + 1)
            .filter_map(AddressFamilyFlags::from_bits)
            .collect::<Vec<AddressFamilyFlags>>();
        let proto_flags_combs = (0..ProtocolFlags::all().bits() + 1)
            .filter_map(ProtocolFlags::from_bits)
            .collect::<Vec<ProtocolFlags>>();
        for af_flags in af_flags_combs.iter() {
            for proto_flags in proto_flags_combs.iter() {
                assert!(get_sockets_info(*af_flags, *proto_flags).is_ok());
            }
        }
    }

    #[test]
    fn result_is_empty_for_empty_flags() {
        let sockets_info =
            get_sockets_info(AddressFamilyFlags::empty(), ProtocolFlags::empty()).unwrap();
        assert!(sockets_info.is_empty());
    }
}
