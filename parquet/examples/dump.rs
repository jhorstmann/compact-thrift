use std::fs::File;
use compact_thrift_runtime::*;
use compact_thrift_parquet::format::FileMetaData;
use compact_thrift_parquet::get_metadata_chunk;

pub fn main() {
    let mut file = File::open("data/alltypes_tiny_pages.parquet").unwrap();
    let data = get_metadata_chunk(&mut file).unwrap();
    let mut input = CompactThriftInputSlice::new(&data);

    let fmd = FileMetaData::read_thrift(&mut input).unwrap();
    println!("{:#?}", fmd);
}