use std::hint::unreachable_unchecked;
use std::io::{Read, Seek, SeekFrom, Error as IOError};
use std::ops::Range;
use compact_thrift_rs::{CompactThriftProtocol, CompactThriftInputSlice, ThriftError};
use crate::format::{ColumnChunk, ColumnIndex, FileMetaData, OffsetIndex};

#[rustfmt::skip]
pub mod format;

#[derive(Debug)]
pub enum ParquetError {
    Thrift(ThriftError),
    InvalidMagic,
    Io(IOError),
}

impl From<IOError> for ParquetError {
    fn from(value: IOError) -> Self {
        Self::Io(value)
    }
}

impl From<ThriftError> for ParquetError {
    fn from(value: ThriftError) -> Self {
        Self::Thrift(value)
    }
}

const PARQUET_MAGIC: [u8; 4] = [b'P', b'A', b'R', b'1'];

pub fn get_metadata_chunk<R: Read + Seek>(input: &mut R) -> Result<Vec<u8>, ParquetError> {
    let _pos = input.seek(SeekFrom::End(-8))?;
    let mut footer = [0_u8; 8];
    input.read_exact(&mut footer)?;
    if footer[4..] != PARQUET_MAGIC {
        return Err(ParquetError::InvalidMagic);
    }

    let metadata_len = u32::from_le_bytes(footer[0..4].try_into().unwrap()) as usize;
    input.seek(SeekFrom::End(-(metadata_len as i64) - 8))?;

    let mut buf = Vec::with_capacity(metadata_len);
    // Safety: Readers should not look at uninitialized bytes, only write into the slice
    unsafe {
        buf.set_len(metadata_len);
    }

    input.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn get_page_index_range(file_metadata: &FileMetaData) -> Option<Range<usize>> {
    let mut min = i64::MAX;
    let mut max = i64::MIN;

    for rg in &file_metadata.row_groups {
        for c in &rg.columns {
            match (c.offset_index_offset, c.offset_index_length, c.column_index_offset, c.column_index_length) {
                (Some(oo), Some(ol), Some(co), Some(cl)) => {
                    min = min.min(oo);
                    max = max.max(oo + ol as i64);

                    min = min.min(co);
                    max = max.max(co + cl as i64);
                }
                _ => {

                }
            }
        }
    }

    if min >= 0 && min < i64::MAX && min < max {
        Some(min as usize..max as usize)
    } else {
        None
    }
}

pub fn get_page_index_chunk<R: Read + Seek>(input: &mut R, file_metadata: &FileMetaData) -> Result<Option<(Vec<u8>, Range<usize>)>, ParquetError> {
    if let Some(range) = get_page_index_range(file_metadata) {
        input.seek(SeekFrom::Start(range.start as u64))?;

        let mut buf = Vec::with_capacity(range.len());
        // Safety: Readers should not look at uninitialized bytes, only write into the slice
        unsafe {
            buf.set_len(range.len());
        }

        input.read_exact(&mut buf)?;

        Ok(Some((buf, range)))
    } else {
        Ok(None)
    }
}

fn read_page_index_for_column_chunk<'i>(chunk: &'i [u8], chunk_offset: usize, column_chunks: &[ColumnChunk]) -> Result<Vec<Option<(OffsetIndex, ColumnIndex)>>, ParquetError> {
    column_chunks.iter().map(|cc| {
        let offset_index_range = if let (Some(offset), Some(length)) = (cc.offset_index_offset, cc.offset_index_length) {
            assert!(offset as usize >= chunk_offset);
            assert!(length > 0);
            let start = (offset as usize - chunk_offset) as usize;
            start..start+length as usize
        } else {
            return Ok(None)
        };
        let column_index_range = if let (Some(offset), Some(length)) = (cc.column_index_offset, cc.column_index_length) {
            assert!(offset as usize >= chunk_offset);
            assert!(length > 0);
            let start = (offset as usize - chunk_offset) as usize;
            start..start+length as usize
        } else {
            return Ok(None)
        };

        let mut res = Ok(Some((OffsetIndex::default(), ColumnIndex::default())));

        match &mut res {
            Ok(Some((offset_index, column_index))) => {
                offset_index.fill(&mut CompactThriftInputSlice::new(&chunk[offset_index_range]))?;
                column_index.fill(&mut CompactThriftInputSlice::new(&chunk[column_index_range]))?;
            }
            _ => unsafe { unreachable_unchecked() }
        }

        res
    }).collect()
}

pub fn read_page_index<'i>(chunk: &'i [u8], chunk_offset: usize, file_metadata: &FileMetaData) -> Result<Vec<Vec<Option<(OffsetIndex, ColumnIndex)>>>, ParquetError> {
    file_metadata.row_groups.iter().map(|rg| {
        read_page_index_for_column_chunk(chunk, chunk_offset, &rg.columns)
    }).collect()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use compact_thrift_rs::{CompactThriftProtocol, CompactThriftInputSlice};
    use crate::format::FileMetaData;
    use crate::get_metadata_chunk;

    #[test]
    fn test_read_metadata() {
        let mut file = File::open("data/alltypes_tiny_pages.parquet").unwrap();
        let metadata_chunk = get_metadata_chunk(&mut file).unwrap();
        dbg!(metadata_chunk.len());

        let mut input = CompactThriftInputSlice::new(&metadata_chunk);
        let metadata = FileMetaData::read(&mut input).unwrap();
        dbg!(&metadata);
    }
}