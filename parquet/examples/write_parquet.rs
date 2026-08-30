use compact_thrift_parquet::format::{
    ColumnChunk, ColumnMetaData, CompressionCodec, DataPageHeader, DictionaryPageHeader, Encoding,
    FieldRepetitionType, FileMetaData, PageHeader, PageType, RowGroup, SchemaElement, Type,
};
use compact_thrift_parquet::EncodingSet;
use compact_thrift_runtime::CompactThriftProtocol;
use std::fs::File;
use std::io::Write;

pub fn main() {
    let mut buffer = Vec::with_capacity(512);
    buffer.extend(b"PAR1");

    let dict_page_offset = buffer.len();
    dbg!(dict_page_offset);

    let dict_data = &42_u32.to_le_bytes();

    let dict_page_header = PageHeader {
        type_: PageType::DICTIONARY_PAGE,
        uncompressed_page_size: dict_data.len() as i32,
        compressed_page_size: dict_data.len() as i32,
        dictionary_page_header: Some(DictionaryPageHeader {
            num_values: 1,
            encoding: Encoding::PLAIN_DICTIONARY,
            is_sorted: None,
        }),
        ..Default::default()
    };

    dict_page_header.write_thrift(&mut buffer).unwrap();
    buffer.extend_from_slice(dict_data);

    let data_page_offset = buffer.len();
    dbg!(data_page_offset);

    let data = &[
        1_u8,         // bitwidth
        (1 << 1) | 0, // one rle encoded element
        0,            // rle value using bitwidth.div_ceil(8) bits
    ];

    let data_page_header = PageHeader {
        type_: PageType::DATA_PAGE,
        uncompressed_page_size: data.len() as i32,
        compressed_page_size: data.len() as i32,
        data_page_header: Some(DataPageHeader {
            num_values: 1,
            encoding: Encoding::RLE_DICTIONARY,
            definition_level_encoding: Encoding::RLE,
            repetition_level_encoding: Encoding::RLE,
            statistics: None,
        }),
        ..Default::default()
    };

    data_page_header.write_thrift(&mut buffer).unwrap();
    buffer.extend_from_slice(data);

    let column_data_len = buffer.len() - dict_page_offset;
    dbg!(column_data_len);

    let rg = RowGroup {
        columns: vec![ColumnChunk {
            meta_data: Some(ColumnMetaData {
                type_: Type::INT32,
                encodings: EncodingSet::from_iter([
                    Encoding::PLAIN_DICTIONARY,
                    Encoding::RLE_DICTIONARY,
                ]),
                codec: CompressionCodec::UNCOMPRESSED,
                num_values: 1,
                total_compressed_size: column_data_len as i64,
                total_uncompressed_size: column_data_len as i64,
                data_page_offset: data_page_offset as i64,
                dictionary_page_offset: Some(dict_page_offset as i64),
                ..Default::default()
            }),
            ..Default::default()
        }],
        total_byte_size: column_data_len as i64,
        num_rows: 1,
        sorting_columns: None,
        file_offset: Some(dict_page_offset as i64),
        total_compressed_size: Some(column_data_len as i64),
        ordinal: Some(1),
    };

    let fmd = FileMetaData {
        version: 1,
        schema: vec![
            SchemaElement {
                type_: None,
                repetition_type: Some(FieldRepetitionType::REQUIRED),
                name: "record".into(),
                num_children: Some(1),
                ..Default::default()
            },
            SchemaElement {
                type_: Some(Type::INT32),
                repetition_type: Some(FieldRepetitionType::REQUIRED),
                name: "column".into(),
                ..Default::default()
            },
        ],
        num_rows: 1,
        row_groups: vec![rg],
        created_by: Some("hand".into()),
        ..Default::default()
    };

    let metadata_offset = buffer.len();
    fmd.write_thrift(&mut buffer).unwrap();
    let metadata_len = (buffer.len() - metadata_offset) as u32;
    dbg!(metadata_len);

    buffer.extend(metadata_len.to_le_bytes());
    buffer.extend(b"PAR1");

    let mut f = File::create("test.parquet").unwrap();
    f.write_all(&buffer).unwrap();
}
