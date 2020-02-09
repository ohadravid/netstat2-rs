//! Cross-platform library to retrieve network sockets information.
//! Tries to be optimal by using low-level OS APIs instead of command line utilities.
//! Provides unified interface and returns data structures which may have additional fields depending on platform.

#![allow(non_camel_case_types)]

#[macro_use]
extern crate bitflags;
extern crate libc;

mod integrations;
mod types;

pub use crate::integrations::*;
pub use crate::types::*;

// Cannot use `cfg(test)` here since `rustdoc` won't look at it.
#[cfg(debug_assertions)]
mod test_readme {
    macro_rules! calculated_doc {
        ($doc:expr, $id:ident) => {
            #[doc = $doc]
            enum $id {}
        }
    }

    calculated_doc!(include_str!("../README.md"), _DoctestReadme);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, TcpListener, IpAddr};
    use std::process;

    #[test]
    fn test_it_works() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();

        let open_port = listener.local_addr().unwrap().port();
        let pid = process::id();

        let af_flags = AddressFamilyFlags::all();
        let proto_flags = ProtocolFlags::all();

        let sock_info = get_sockets_info(af_flags, proto_flags).unwrap();

        assert!(sock_info.len() > 0);

        let sock = sock_info.into_iter().find(|s| s.associated_pids.contains(&pid)).unwrap();

        assert_eq!(sock.protocol_socket_info,
                   ProtocolSocketInfo::Tcp(TcpSocketInfo {
                       local_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
                       local_port: open_port,
                       remote_addr: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                       remote_port: 0,
                       state: TcpState::Listen,
                   }));
        assert_eq!(sock.associated_pids, vec![pid]);
    }

    #[test]
    fn result_is_ok_for_any_flags() {
        let af_flags_combs = (0..AddressFamilyFlags::all().bits() + 1)
            .filter_map(|x| AddressFamilyFlags::from_bits(x))
            .collect::<Vec<AddressFamilyFlags>>();
        let proto_flags_combs = (0..ProtocolFlags::all().bits() + 1)
            .filter_map(|x| ProtocolFlags::from_bits(x))
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
        assert!(sockets_info.len() == 0);
    }
}
