use compact_thrift_runtime::{
    read_collection_len_and_type, CompactThriftInput, CompactThriftOutput, CompactThriftProtocol,
    ThriftError,
};

/// Skip reading the redundant `path_in_schema` field and write it as a single empty string.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct PathInSchema;

impl<'i> CompactThriftProtocol<'i> for PathInSchema {
    const FIELD_TYPE: u8 = <Vec<String> as CompactThriftProtocol>::FIELD_TYPE;

    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let (len, _) = read_collection_len_and_type(input)?;
        for _ in 0..len {
            input.skip_binary()?
        }
        Ok(())
    }

    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_byte(String::FIELD_TYPE | (1 << 4))?;
        output.write_binary(&[])
    }
}

#[cfg(test)]
mod tests {
    use crate::path_in_schema::PathInSchema;
    use compact_thrift_runtime::{CompactThriftInputSlice, CompactThriftProtocol};

    #[test]
    fn path_in_schema_size() {
        assert_eq!(size_of::<PathInSchema>(), 0);
        assert_eq!(size_of::<Option<PathInSchema>>(), 1);
    }

    #[test]
    fn path_in_schema_read() {
        let mut buf = vec![];
        let write_path = vec!["a".to_owned(), "b".to_owned()];
        write_path.write_thrift(&mut buf).unwrap();

        let mut input = CompactThriftInputSlice::new(&buf);
        let _read_path = PathInSchema::read_thrift(&mut input).unwrap();
        assert_eq!(input.as_slice(), &[]);
    }

    #[test]
    fn path_in_schema_write() {
        let mut buf = vec![];
        PathInSchema.write_thrift(&mut buf).unwrap();

        let mut input = CompactThriftInputSlice::new(&buf);
        let read_path = Vec::<String>::read_thrift(&mut input).unwrap();
        assert_eq!(read_path.as_slice(), &["".to_owned()]);
        assert_eq!(input.as_slice(), &[]);
    }
}
