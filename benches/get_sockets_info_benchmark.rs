extern crate easybench;
extern crate netstat;

use easybench::bench;
use netstat::*;

fn main() {
    println!(
        "get_sockets_info: {}",
        bench(|| {
            let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
            let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
            get_sockets_info(af_flags, proto_flags).unwrap();
        })
    );
}
