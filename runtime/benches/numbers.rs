use compact_thrift_rs::{CompactThriftInput, CompactThriftOutput, CompactThriftProtocol, SliceInput};
use criterion::*;
use std::fs::File;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    let mut numbers = (0..1000).map(|_| (rng.next_u64() >> 16) as i64).collect::<Vec<i64>>();

    let mut data = Vec::new();
    numbers.write(&mut data).unwrap();

    let mut input = SliceInput::new(&data);

    let result = Vec::<i64>::read(&mut input.clone()).unwrap();
    assert_eq!(&result, &numbers);

    c.benchmark_group("numbers")
        .throughput(Throughput::Bytes(data.len() as u64))
        .bench_function("read", |b| {
            b.iter(|| Vec::<i64>::read(&mut input.clone()).unwrap())
        })
        .bench_function("skip", |b| {
            b.iter(|| input.clone().skip_field(<Vec<i64> as CompactThriftProtocol>::FIELD_TYPE).unwrap())
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
