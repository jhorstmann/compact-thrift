use compact_thrift_parquet::format::{CompressionCodec, Encoding, FileMetaData, PageHeader};
use compact_thrift_parquet::get_metadata_chunk;
use compact_thrift_runtime::{CompactThriftInputSlice, CompactThriftProtocol, ThriftError};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Clone)]
pub enum CompressedPage<'a> {
    DataV1 {
        num_values: usize,
        rep_level_encoding: Encoding,
        def_level_encoding: Encoding,
        data_encoding: Encoding,
        data: &'a [u8],
    },
    DataV2 {
        num_values: usize,
        is_compressed: bool,
        rep_level_len: usize,
        def_level_len: usize,
        data_encoding: Encoding,
        data: &'a [u8],
    },
    Dictionary {
        num_values: usize,
        encoding: Encoding,
        data: &'a [u8],
    },
}

pub struct PageIterator<'a> {
    buffer: &'a [u8],
}

#[derive(Debug)]
pub enum PageError {
    ThriftError(ThriftError),
    InvalidThriftValue {
        value: i32,
        source: &'static str,
    },
    InvalidPageHeader,
    InvalidPageSize {
        compressed_size: usize,
        remaining_bytes: usize,
    },
    UnsupportedCompressionCodec(CompressionCodec),
}

impl From<ThriftError> for PageError {
    fn from(e: ThriftError) -> Self {
        Self::ThriftError(e)
    }
}

fn thrift_i32_to_usize(value: i32, source: &'static str) -> Result<usize, PageError> {
    value
        .try_into()
        .map_err(|_| PageError::InvalidThriftValue { value, source })
}

impl<'a> PageIterator<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    pub fn next_page(&mut self) -> Result<Option<CompressedPage<'a>>, PageError> {
        if self.buffer.is_empty() {
            return Ok(None);
        }

        let mut input = CompactThriftInputSlice::new(std::mem::take(&mut self.buffer));
        let page = PageHeader::read_thrift(&mut input)?;

        self.buffer = input.as_slice();

        let compressed_size = thrift_i32_to_usize(
            page.compressed_page_size,
            "PageHeader::compressed_page_size",
        )?;
        let (data, remainder) = self
            .buffer
            .split_at_checked(compressed_size)
            .ok_or_else(|| PageError::InvalidPageSize {
                compressed_size,
                remaining_bytes: self.buffer.len(),
            })?;

        self.buffer = remainder;

        if page.index_page_header.is_some()
            || (page.data_page_header.is_some() as u8
                + page.dictionary_page_header.is_some() as u8
                + page.data_page_header_v2.is_some() as u8)
                != 1
        {
            return Err(PageError::InvalidPageHeader);
        }

        if let Some(data_page_header) = page.data_page_header {
            let num_values =
                thrift_i32_to_usize(data_page_header.num_values, "DataPageHeader::num_values")?;
            let rep_level_encoding = data_page_header.repetition_level_encoding;
            let def_level_encoding = data_page_header.definition_level_encoding;
            let data_encoding = data_page_header.encoding;
            Ok(Some(CompressedPage::DataV1 {
                num_values,
                rep_level_encoding,
                def_level_encoding,
                data_encoding,
                data,
            }))
        } else if let Some(data_page_header) = page.data_page_header_v2 {
            let num_values =
                thrift_i32_to_usize(data_page_header.num_values, "DataPageHeader::num_values")?;
            let is_compressed = data_page_header.is_compressed.unwrap_or(true);
            let data_encoding = data_page_header.encoding;
            let rep_level_len = thrift_i32_to_usize(
                data_page_header.repetition_levels_byte_length,
                "DataPageHeaderV2::repetition_levels_byte_length",
            )?;
            let def_level_len = thrift_i32_to_usize(
                data_page_header.definition_levels_byte_length,
                "DataPageHeaderV2::definition_levels_byte_length",
            )?;
            Ok(Some(CompressedPage::DataV2 {
                num_values,
                is_compressed,
                rep_level_len,
                def_level_len,
                data_encoding,
                data,
            }))
        } else if let Some(dict_page_header) = page.dictionary_page_header {
            let num_values = thrift_i32_to_usize(
                dict_page_header.num_values,
                "DictionaryPageHeader::num_values",
            )?;
            let encoding = dict_page_header.encoding;
            Ok(Some(CompressedPage::Dictionary {
                num_values,
                encoding,
                data,
            }))
        } else {
            Err(PageError::InvalidPageHeader)
        }
    }
}

pub fn main() {
    let mut file = File::open("parquet/data/alltypes_tiny_pages.parquet").unwrap();
    let data = get_metadata_chunk(&mut file).unwrap();
    let mut input = CompactThriftInputSlice::new(&data);

    let fmd = FileMetaData::read_thrift(&mut input).unwrap();

    let column_meta_data = fmd.row_groups[0].columns[0].meta_data.as_ref().unwrap();

    dbg!(&column_meta_data);

    let mut start = column_meta_data.data_page_offset;
    if let Some(dict_page_offset) = column_meta_data.dictionary_page_offset {
        if dict_page_offset < start {
            start = dict_page_offset;
        }
    }
    // let end = start + column_meta_data.total_compressed_size;

    file.seek(SeekFrom::Start(start as u64)).unwrap();
    let mut column_data = vec![0_u8; column_meta_data.total_compressed_size as usize];
    file.read_exact(&mut column_data).unwrap();
    let mut page_iter = PageIterator::new(&column_data);
    let mut counter = 0_usize;
    while let Some(page) = page_iter.next_page().unwrap() {
        match page {
            CompressedPage::Dictionary {
                encoding,
                num_values,
                data,
            } => {
                println!(
                    "Dictionary Page: encoding={encoding:?}, num_values={num_values}, len={}",
                    data.len()
                );
            }
            CompressedPage::DataV1 {
                data_encoding,
                num_values,
                data,
                ..
            } => {
                println!(
                    "Data Page V1  : encoding={data_encoding:?}, num_values={num_values}, len={}",
                    data.len()
                );
            }
            CompressedPage::DataV2 {
                data_encoding,
                num_values,
                data,
                is_compressed,
                ..
            } => {
                println!("Data Page V1  : encoding={data_encoding:?}, num_values={num_values}, len={}, compressed={is_compressed}", data.len());
            }
        }
        counter += 1;
    }
    println!("{counter} pages");
}
