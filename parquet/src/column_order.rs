use compact_thrift_runtime::{CompactThriftInput, CompactThriftOutput, CompactThriftProtocol, FieldName, ThriftError};

#[derive(Debug,Clone,PartialEq)]
pub struct ColumnOrder(i16);

/// ColumnOrder used ot interpret column statistics. Modeled as an enum instead of a union here,
/// to support reading new column orders which might be defined later. We expect all variants to
/// only contain an empty struct as their payload, otherwise we would not be able to roundtrip
/// their contents.
///
/// See the thrift struct renamed [`crate::format::_ColumnOrder`] for documentation of the variants.
impl ColumnOrder {
    /// See [`crate::format::_ColumnOrder::TYPE_ORDER`]
    pub const TYPE_ORDER: ColumnOrder = ColumnOrder(1);

    /// See [`crate::format::_ColumnOrder::IEEE_754_TOTAL_ORDER`]
    pub const IEEE_754_TOTAL_ORDER: ColumnOrder = ColumnOrder(2);

    /// See [`crate::format::_ColumnOrder::INT96_TIMESTAMP_ORDER`]
    pub const INT96_TIMESTAMP_ORDER: ColumnOrder = ColumnOrder(3);
}

impl Default for ColumnOrder {
    fn default() -> Self {
        Self(0)
    }
}

const COLUMN_ORDER_CONTENT: FieldName = const { FieldName::from_str("ColumnOrder::Content\0") };

#[derive(Debug,Default,Clone)]
struct EmptyStruct {}

impl<'i> CompactThriftProtocol<'i> for EmptyStruct {
    const FIELD_TYPE: u8 = 12;

    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        let field_type = input.read_field_header(&mut last_field_id)?;
        if field_type != 0 {
            Err(ThriftError::UnknownVariant(COLUMN_ORDER_CONTENT, field_type as _))
        } else {
            Ok(())
        }
    }

    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_byte(0)?;
        Ok(())
    }
}

impl<'i> CompactThriftProtocol<'i> for ColumnOrder {
    const FIELD_TYPE: u8 = 12;

    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        let field_type = input.read_field_header(&mut last_field_id)?;

        if field_type == 0 {
            return Err(ThriftError::MissingField(COLUMN_ORDER_CONTENT));
        }

        EmptyStruct::default().fill_thrift(input)?;
        self.0 = last_field_id;

        let stop = input.read_byte()?;
        if stop != 0 {
            return Err(ThriftError::MissingStop)
        }

        Ok(())
    }

    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        EmptyStruct::default().write_thrift_field(output, self.0, &mut last_field_id)?;
        Ok(())
    }
}