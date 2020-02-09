#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use netstat2::*;

    #[bench]
    fn bench_new(b: &mut test::Bencher) {
        b.iter(|| {
            let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
            let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
            get_sockets_info(af_flags, proto_flags).unwrap();
        });
    }
}
