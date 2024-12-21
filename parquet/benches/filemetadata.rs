use compact_thrift_rs::{CompactThriftInput, CompactThriftProtocol, CompactThriftInputSlice};
use criterion::*;
use parquet_format::format::FileMetaData;
use parquet_format::{get_metadata_chunk, get_page_index_chunk, get_page_index_range, read_page_index};
use std::fs::File;
use std::io::Read;
use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn criterion_benchmark(c: &mut Criterion) {
    let mut file = File::open("data/alltypes_tiny_pages.parquet").unwrap();
    let mut data = Vec::default();
    file.read_to_end(&mut data).unwrap();

    let fmd_chunk = get_metadata_chunk(&mut file).unwrap();
    let fmd_input = CompactThriftInputSlice::new(&fmd_chunk);

    let fmd = FileMetaData::read(&mut fmd_input.clone()).unwrap();

    let maybe_page_index_range = get_page_index_range(&fmd);

    c.benchmark_group("filemetadata")
        .throughput(Throughput::Bytes(fmd_chunk.len() as u64))
        .bench_function("read", |b| {
            b.iter(|| FileMetaData::read(&mut fmd_input.clone()).unwrap())
        })
        .bench_function("fill", |b| {
            b.iter(|| {
                let mut fmd = FileMetaData::default();
                fmd.fill(&mut fmd_input.clone()).unwrap()
            })
        })
        .bench_function("skip", |b| {
            b.iter(|| fmd_input.clone().skip_field(FileMetaData::FIELD_TYPE).unwrap())
        });
    if let Some(page_index_range) = maybe_page_index_range {
        c.benchmark_group("pageindex")
            .throughput(Throughput::Bytes(page_index_range.len() as u64))
            .bench_function("get_range", |b| {
                b.iter(|| get_page_index_range(&fmd).unwrap())
            })
            .bench_function("read", |b| {
                let (chunk, range) = get_page_index_chunk(&mut file, &fmd).unwrap().unwrap();
                assert!(!range.is_empty());
                assert!(!chunk.is_empty());
                b.iter(|| read_page_index(&chunk, range.start, &fmd).unwrap())
            });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
