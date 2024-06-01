use compact_thrift_rs::{CompactThriftInput, CompactThriftProtocol, SliceInput};
use criterion::*;
use parquet_format::format::FileMetaData;
use parquet_format::read_metadata;
use std::fs::File;

fn criterion_benchmark(c: &mut Criterion) {
    let mut file = File::open("data/alltypes_tiny_pages.parquet").unwrap();
    let data = read_metadata(&mut file).unwrap();
    let input = SliceInput::new(&data);
    c.benchmark_group("filemetadata")
        .throughput(Throughput::Bytes(1721))
        .bench_function("read", |b| {
            b.iter(|| FileMetaData::read(&mut input.clone()))
        })
        .bench_function("fill", |b| {
            let mut fmd = FileMetaData::default();
            b.iter(|| fmd.fill(&mut input.clone()))
        })
        .bench_function("skip", |b| {
            b.iter(|| input.clone().skip_field(FileMetaData::FIELD_TYPE))
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
