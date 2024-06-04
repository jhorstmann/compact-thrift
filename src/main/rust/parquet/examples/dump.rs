use std::fs::File;
use compact_thrift_rs::*;
use parquet_format::format::FileMetaData;
use parquet_format::read_metadata;

pub fn main() {
    let mut file = File::open("data/alltypes_tiny_pages.parquet").unwrap();
    let data = read_metadata(&mut file).unwrap();
    let mut input = SliceInput::new(&data);

    let fmd = FileMetaData::read(&mut input).unwrap();
    println!("{:#?}", fmd);
}