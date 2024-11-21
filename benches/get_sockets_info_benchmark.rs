use criterion::{black_box, criterion_group, criterion_main, Criterion};
use netstat2::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get_sockets_info", |b| b.iter(|| {
        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
        let _result = get_sockets_info(black_box(af_flags), proto_flags).unwrap();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);