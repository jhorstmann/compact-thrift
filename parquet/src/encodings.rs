use std::fmt::{Debug, Formatter};
use crate::format::Encoding;
use compact_thrift_runtime::{
    read_collection_len_and_type, CompactThriftInput, CompactThriftOutput, CompactThriftProtocol,
    ThriftError,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct EncodingSet(u32);

impl EncodingSet {
    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = Encoding> {
        let mut mask = self.0;
        std::iter::from_fn(move || {
            if mask == 0 {
                None
            } else {
                let encoding = Encoding(mask.trailing_zeros() as i32);
                mask &= mask - 1;
                Some(encoding)
            }
        })
    }
}

impl Debug for EncodingSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'i> CompactThriftProtocol<'i> for EncodingSet {
    const FIELD_TYPE: u8 = 9;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let (len, element_type) = read_collection_len_and_type(input)?;
        if element_type != 5 {
            return Err(ThriftError::InvalidType);
        }

        let mut mask = 0_u32;
        for _ in 0..len as usize {
            let value = input.read_i32()?;
            if value < 0 || value >= 31 {
                return Err(ThriftError::InvalidNumber);
            }
            mask |= 1 << value;
        }
        self.0 = mask;

        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        for item in self.iter() {
            output.write_i32(item.0)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::encodings::EncodingSet;
    use crate::format::Encoding;
    use compact_thrift_runtime::{CompactThriftInputSlice, CompactThriftProtocol};

    fn test_roundtrip(encodings: Vec<Encoding>) {
        let mut buf = vec![];
        encodings.write_thrift(&mut buf).unwrap();
        let set = EncodingSet::read_thrift(&mut CompactThriftInputSlice::new(&buf)).unwrap();
        let vec = set.iter().collect::<Vec<_>>();
        assert_eq!(vec, encodings);
    }

    #[test]
    fn test_empty_encodings() {
        test_roundtrip(vec![])
    }

    #[test]
    fn test_plain() {
        test_roundtrip(vec![Encoding::PLAIN])
    }

    #[test]
    fn test_rle() {
        test_roundtrip(vec![Encoding::RLE])
    }

    #[test]
    fn test_multi() {
        test_roundtrip(vec![
            Encoding::PLAIN,
            Encoding::RLE_DICTIONARY,
            Encoding::BYTE_STREAM_SPLIT,
        ])
    }

    #[test]
    #[should_panic(expected = "InvalidNumber")]
    fn test_invalid() {
        test_roundtrip(vec![Encoding(32)])
    }
}
