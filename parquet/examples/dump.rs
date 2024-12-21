use std::fs::File;
use compact_thrift_rs::*;
use parquet_format::format::FileMetaData;
use parquet_format::get_metadata_chunk;

pub fn main() {
    let mut file = File::open("data/alltypes_tiny_pages.parquet").unwrap();
    let data = get_metadata_chunk(&mut file).unwrap();
    let mut input = CompactThriftInputSlice::new(&data);

    let fmd = FileMetaData::read(&mut input).unwrap();
    println!("{:#?}", fmd);
}