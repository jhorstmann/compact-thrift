use compact_thrift_rs::{CompactThriftInput, CompactThriftProtocol, CompactThriftInputSlice};
use criterion::*;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    let numbers = (0..1000).map(|_| (rng.next_u64() >> 16) as i64).collect::<Vec<i64>>();

    let mut data = Vec::new();
    numbers.write_thrift(&mut data).unwrap();

    let input = CompactThriftInputSlice::new(&data);

    let result = Vec::<i64>::read_thrift(&mut input.clone()).unwrap();
    assert_eq!(&result, &numbers);

    c.benchmark_group("InputSlice")
        .throughput(Throughput::Bytes(data.len() as u64))
        .bench_function("read", |b| {
            b.iter(|| Vec::<i64>::read_thrift(&mut input.clone()).unwrap())
        })
        .bench_function("skip", |b| {
            b.iter(|| input.clone().skip_field(<Vec<i64> as CompactThriftProtocol>::FIELD_TYPE).unwrap())
        });

    c.benchmark_group("StdRead")
        .throughput(Throughput::Bytes(data.len() as u64))
        .bench_function("read", |b| {
            b.iter(|| {
                let read = &mut data.as_slice();
                Vec::<i64>::read_thrift(read).unwrap()
            })
        })
        .bench_function("skip", |b| {
            b.iter(|| {
                let read = &mut data.as_slice();
                read.skip_field(<Vec<i64> as CompactThriftProtocol>::FIELD_TYPE).unwrap()
            })
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
