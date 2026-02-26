use compact_thrift_runtime::{
    read_collection_len_and_type, CompactThriftInput, CompactThriftOutput, CompactThriftProtocol,
    ThriftError,
};

/// Skip reading the redundant `path_in_schema` field while still allowing to write it.
#[allow(clippy::box_collection)] // minimize heap size
#[derive(Default, Clone, Debug, PartialEq)]
pub struct PathInSchema(Option<Vec<String>>);

impl PathInSchema {
    pub fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(b) => b.len(),
        }
    }

    pub fn as_slice(&self) -> &[String] {
        match &self.0 {
            None => &[],
            Some(b) => b.as_slice(),
        }
    }

    pub fn into_vec(self) -> Vec<String> {
        self.0.unwrap_or_else(|| vec![])
    }
}

impl From<Vec<String>> for PathInSchema {
    fn from(v: Vec<String>) -> Self {
        Self(Some(v))
    }
}

impl From<String> for PathInSchema {
    fn from(s: String) -> Self {
        Self::from(vec![s])
    }
}

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
        self.0.write_thrift(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::path_in_schema::PathInSchema;
    use compact_thrift_runtime::{CompactThriftInputSlice, CompactThriftProtocol};

    #[test]
    fn path_in_schema_size() {
        assert_eq!(size_of::<PathInSchema>(), 24);
        assert_eq!(size_of::<Option<PathInSchema>>(), 24);
    }

    #[test]
    fn path_in_schema_roundtrip() {
        let mut buf = vec![];
        let write_path = PathInSchema::from(vec!["a".into(), "b".into()]);
        write_path.write_thrift(&mut buf).unwrap();

        let mut input = CompactThriftInputSlice::new(&buf);
        let read_path = PathInSchema::read_thrift(&mut input).unwrap();
        assert_eq!(read_path.len(), 0);

        let mut input = CompactThriftInputSlice::new(&buf);
        let read_path_as_vec = Vec::<String>::read_thrift(&mut input).unwrap();

        assert_eq!(&read_path_as_vec, write_path.as_slice());
    }
}
