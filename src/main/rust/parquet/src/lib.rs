use std::io::{Read, Seek, SeekFrom, Error as IOError};
use compact_thrift_rs::ThriftError;

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

const PARQUET_MAGIC: [u8; 4] = [b'P', b'A', b'R', b'1'];

pub fn read_metadata<R: Read + Seek>(input: &mut R) -> Result<Vec<u8>, ParquetError> {
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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use compact_thrift_rs::{CompactThriftProtocol, SliceInput};
    use crate::format::FileMetaData;
    use crate::read_metadata;

    #[test]
    fn test_read_metadata() {
        let mut file = File::open("data/alltypes_tiny_pages.parquet").unwrap();
        let metadata_chunk = read_metadata(&mut file).unwrap();
        dbg!(metadata_chunk.len());

        let mut input = SliceInput::new(&metadata_chunk);
        let metadata = FileMetaData::read(&mut input).unwrap();
        dbg!(&metadata);
    }
}