#[allow(non_snake_case)]
// Generated on 2024-06-10T21:37:58.251558082Z
use std::borrow::Cow;
use compact_thrift_rs::*;
#[derive(Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct Type(pub i32);

impl Type {
    pub const BOOLEAN: Self = Self(0);
    pub const INT32: Self = Self(1);
    pub const INT64: Self = Self(2);
    pub const INT96: Self = Self(3);
    pub const FLOAT: Self = Self(4);
    pub const DOUBLE: Self = Self(5);
    pub const BYTE_ARRAY: Self = Self(6);
    pub const FIXED_LEN_BYTE_ARRAY: Self = Self(7);

    const __NAMES: &'static [(i32, &'static str)] = &[
        (0, "BOOLEAN"),
        (1, "INT32"),
        (2, "INT64"),
        (3, "INT96"),
        (4, "FLOAT"),
        (5, "DOUBLE"),
        (6, "BYTE_ARRAY"),
        (7, "FIXED_LEN_BYTE_ARRAY"),
    ];

    #[inline]
    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for Type {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let name = match Self::__NAMES.binary_search_by_key(&self.0, |(i, _)| *i) {
            Ok(i) => &Self::__NAMES[i].1,
            Err(_) => "__UNKNOWN",
        };
        f.debug_tuple(name).field(&self.0).finish()
    }
}
impl <'i> CompactThriftProtocol<'i> for Type {
    const FIELD_TYPE: u8 = 5; // i32

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}

#[derive(Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct ConvertedType(pub i32);

impl ConvertedType {
    pub const UTF8: Self = Self(0);
    pub const MAP: Self = Self(1);
    pub const MAP_KEY_VALUE: Self = Self(2);
    pub const LIST: Self = Self(3);
    pub const ENUM: Self = Self(4);
    pub const DECIMAL: Self = Self(5);
    pub const DATE: Self = Self(6);
    pub const TIME_MILLIS: Self = Self(7);
    pub const TIME_MICROS: Self = Self(8);
    pub const TIMESTAMP_MILLIS: Self = Self(9);
    pub const TIMESTAMP_MICROS: Self = Self(10);
    pub const UINT_8: Self = Self(11);
    pub const UINT_16: Self = Self(12);
    pub const UINT_32: Self = Self(13);
    pub const UINT_64: Self = Self(14);
    pub const INT_8: Self = Self(15);
    pub const INT_16: Self = Self(16);
    pub const INT_32: Self = Self(17);
    pub const INT_64: Self = Self(18);
    pub const JSON: Self = Self(19);
    pub const BSON: Self = Self(20);
    pub const INTERVAL: Self = Self(21);

    const __NAMES: &'static [(i32, &'static str)] = &[
        (0, "UTF8"),
        (1, "MAP"),
        (2, "MAP_KEY_VALUE"),
        (3, "LIST"),
        (4, "ENUM"),
        (5, "DECIMAL"),
        (6, "DATE"),
        (7, "TIME_MILLIS"),
        (8, "TIME_MICROS"),
        (9, "TIMESTAMP_MILLIS"),
        (10, "TIMESTAMP_MICROS"),
        (11, "UINT_8"),
        (12, "UINT_16"),
        (13, "UINT_32"),
        (14, "UINT_64"),
        (15, "INT_8"),
        (16, "INT_16"),
        (17, "INT_32"),
        (18, "INT_64"),
        (19, "JSON"),
        (20, "BSON"),
        (21, "INTERVAL"),
    ];

    #[inline]
    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for ConvertedType {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for ConvertedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let name = match Self::__NAMES.binary_search_by_key(&self.0, |(i, _)| *i) {
            Ok(i) => &Self::__NAMES[i].1,
            Err(_) => "__UNKNOWN",
        };
        f.debug_tuple(name).field(&self.0).finish()
    }
}
impl <'i> CompactThriftProtocol<'i> for ConvertedType {
    const FIELD_TYPE: u8 = 5; // i32

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}

#[derive(Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct FieldRepetitionType(pub i32);

impl FieldRepetitionType {
    pub const REQUIRED: Self = Self(0);
    pub const OPTIONAL: Self = Self(1);
    pub const REPEATED: Self = Self(2);

    const __NAMES: &'static [(i32, &'static str)] = &[
        (0, "REQUIRED"),
        (1, "OPTIONAL"),
        (2, "REPEATED"),
    ];

    #[inline]
    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for FieldRepetitionType {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for FieldRepetitionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let name = match Self::__NAMES.binary_search_by_key(&self.0, |(i, _)| *i) {
            Ok(i) => &Self::__NAMES[i].1,
            Err(_) => "__UNKNOWN",
        };
        f.debug_tuple(name).field(&self.0).finish()
    }
}
impl <'i> CompactThriftProtocol<'i> for FieldRepetitionType {
    const FIELD_TYPE: u8 = 5; // i32

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct SizeStatistics {
    pub unencoded_byte_array_data_bytes: Option<i64>,
    pub repetition_level_histogram: Option<Vec<i64>>,
    pub definition_level_histogram: Option<Vec<i64>>,
}

impl SizeStatistics {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(unencoded_byte_array_data_bytes: impl Into<Option<i64>>, repetition_level_histogram: impl Into<Option<Vec<i64>>>, definition_level_histogram: impl Into<Option<Vec<i64>>>) -> Self {
        Self {
            unencoded_byte_array_data_bytes: unencoded_byte_array_data_bytes.into(),
            repetition_level_histogram: repetition_level_histogram.into(),
            definition_level_histogram: definition_level_histogram.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for SizeStatistics {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {

                    self.unencoded_byte_array_data_bytes.fill_field(input, field_type)?;
                }
                2 => {

                    self.repetition_level_histogram.fill_field(input, field_type)?;
                }
                3 => {

                    self.definition_level_histogram.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.unencoded_byte_array_data_bytes.write_field(output, 1, &mut last_field_id)?;
        self.repetition_level_histogram.write_field(output, 2, &mut last_field_id)?;
        self.definition_level_histogram.write_field(output, 3, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Statistics<'i> {
    pub max: Option<Cow<'i, [u8]>>,
    pub min: Option<Cow<'i, [u8]>>,
    pub null_count: Option<i64>,
    pub distinct_count: Option<i64>,
    pub max_value: Option<Cow<'i, [u8]>>,
    pub min_value: Option<Cow<'i, [u8]>>,
    pub is_max_value_exact: Option<bool>,
    pub is_min_value_exact: Option<bool>,
}

impl<'i> Statistics<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(max: impl Into<Option<Cow<'i, [u8]>>>, min: impl Into<Option<Cow<'i, [u8]>>>, null_count: impl Into<Option<i64>>, distinct_count: impl Into<Option<i64>>, max_value: impl Into<Option<Cow<'i, [u8]>>>, min_value: impl Into<Option<Cow<'i, [u8]>>>, is_max_value_exact: impl Into<Option<bool>>, is_min_value_exact: impl Into<Option<bool>>) -> Self {
        Self {
            max: max.into(),
            min: min.into(),
            null_count: null_count.into(),
            distinct_count: distinct_count.into(),
            max_value: max_value.into(),
            min_value: min_value.into(),
            is_max_value_exact: is_max_value_exact.into(),
            is_min_value_exact: is_min_value_exact.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for Statistics<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {

                    self.max.fill_field(input, field_type)?;
                }
                2 => {

                    self.min.fill_field(input, field_type)?;
                }
                3 => {

                    self.null_count.fill_field(input, field_type)?;
                }
                4 => {

                    self.distinct_count.fill_field(input, field_type)?;
                }
                5 => {

                    self.max_value.fill_field(input, field_type)?;
                }
                6 => {

                    self.min_value.fill_field(input, field_type)?;
                }
                7 => {

                    self.is_max_value_exact.fill_field(input, field_type)?;
                }
                8 => {

                    self.is_min_value_exact.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.max.write_field(output, 1, &mut last_field_id)?;
        self.min.write_field(output, 2, &mut last_field_id)?;
        self.null_count.write_field(output, 3, &mut last_field_id)?;
        self.distinct_count.write_field(output, 4, &mut last_field_id)?;
        self.max_value.write_field(output, 5, &mut last_field_id)?;
        self.min_value.write_field(output, 6, &mut last_field_id)?;
        self.is_max_value_exact.write_field(output, 7, &mut last_field_id)?;
        self.is_min_value_exact.write_field(output, 8, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct StringType {
}

impl StringType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for StringType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct UUIDType {
}

impl UUIDType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for UUIDType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MapType {
}

impl MapType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for MapType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ListType {
}

impl ListType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for ListType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct EnumType {
}

impl EnumType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for EnumType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DateType {
}

impl DateType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for DateType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Float16Type {
}

impl Float16Type {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for Float16Type {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct NullType {
}

impl NullType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for NullType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DecimalType {
    pub scale: i32,
    pub precision: i32,
}

impl DecimalType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(scale: impl Into<i32>, precision: impl Into<i32>) -> Self {
        Self {
            scale: scale.into(),
            precision: precision.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for DecimalType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut scale_set_: bool = false;
        let mut precision_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    scale_set_ = true;
                    self.scale.fill_field(input, field_type)?;
                }
                2 => {
                    precision_set_ = true;
                    self.precision.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !scale_set_ || !precision_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.scale.write_field(output, 1, &mut last_field_id)?;
        self.precision.write_field(output, 2, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MilliSeconds {
}

impl MilliSeconds {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for MilliSeconds {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MicroSeconds {
}

impl MicroSeconds {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for MicroSeconds {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct NanoSeconds {
}

impl NanoSeconds {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for NanoSeconds {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum TimeUnit {
    MILLIS(MilliSeconds),
    MICROS(MicroSeconds),
    NANOS(NanoSeconds),
}
impl Default for TimeUnit {
    fn default() -> Self {
        Self::MILLIS(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for TimeUnit {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        let field_type = input.read_field_header(&mut last_field_id)?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        match last_field_id {
            1 => {
                *self = Self::MILLIS(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::MILLIS(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            2 => {
                *self = Self::MICROS(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::MICROS(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            3 => {
                *self = Self::NANOS(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::NANOS(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            _ => {
                return Err(ThriftError::MissingField)
            }
        }
        let stop = input.read_byte()?;
        if stop != 0 {
            return Err(ThriftError::MissingStop)
        }

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::MILLIS(inner) => inner.write_field(output, 1, &mut last_field_id)?,
            Self::MICROS(inner) => inner.write_field(output, 2, &mut last_field_id)?,
            Self::NANOS(inner) => inner.write_field(output, 3, &mut last_field_id)?,
        }
        // STOP
        output.write_byte(0)
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct TimestampType {
    pub isAdjustedToUTC: bool,
    pub unit: TimeUnit,
}

impl TimestampType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(isAdjustedToUTC: impl Into<bool>, unit: impl Into<TimeUnit>) -> Self {
        Self {
            isAdjustedToUTC: isAdjustedToUTC.into(),
            unit: unit.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for TimestampType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut isAdjustedToUTC_set_: bool = false;
        let mut unit_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    isAdjustedToUTC_set_ = true;
                    self.isAdjustedToUTC.fill_field(input, field_type)?;
                }
                2 => {
                    unit_set_ = true;
                    self.unit.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !isAdjustedToUTC_set_ || !unit_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.isAdjustedToUTC.write_field(output, 1, &mut last_field_id)?;
        self.unit.write_field(output, 2, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct TimeType {
    pub isAdjustedToUTC: bool,
    pub unit: TimeUnit,
}

impl TimeType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(isAdjustedToUTC: impl Into<bool>, unit: impl Into<TimeUnit>) -> Self {
        Self {
            isAdjustedToUTC: isAdjustedToUTC.into(),
            unit: unit.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for TimeType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut isAdjustedToUTC_set_: bool = false;
        let mut unit_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    isAdjustedToUTC_set_ = true;
                    self.isAdjustedToUTC.fill_field(input, field_type)?;
                }
                2 => {
                    unit_set_ = true;
                    self.unit.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !isAdjustedToUTC_set_ || !unit_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.isAdjustedToUTC.write_field(output, 1, &mut last_field_id)?;
        self.unit.write_field(output, 2, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IntType {
    pub bitWidth: i8,
    pub isSigned: bool,
}

impl IntType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(bitWidth: impl Into<i8>, isSigned: impl Into<bool>) -> Self {
        Self {
            bitWidth: bitWidth.into(),
            isSigned: isSigned.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for IntType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut bitWidth_set_: bool = false;
        let mut isSigned_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    bitWidth_set_ = true;
                    self.bitWidth.fill_field(input, field_type)?;
                }
                2 => {
                    isSigned_set_ = true;
                    self.isSigned.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !bitWidth_set_ || !isSigned_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.bitWidth.write_field(output, 1, &mut last_field_id)?;
        self.isSigned.write_field(output, 2, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct JsonType {
}

impl JsonType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for JsonType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct BsonType {
}

impl BsonType {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for BsonType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum LogicalType {
    STRING(StringType),
    MAP(MapType),
    LIST(ListType),
    ENUM(EnumType),
    DECIMAL(DecimalType),
    DATE(DateType),
    TIME(TimeType),
    TIMESTAMP(TimestampType),
    INTEGER(IntType),
    UNKNOWN(NullType),
    JSON(JsonType),
    BSON(BsonType),
    UUID(UUIDType),
    FLOAT16(Float16Type),
}
impl Default for LogicalType {
    fn default() -> Self {
        Self::STRING(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for LogicalType {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        let field_type = input.read_field_header(&mut last_field_id)?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        match last_field_id {
            1 => {
                *self = Self::STRING(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::STRING(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            2 => {
                *self = Self::MAP(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::MAP(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            3 => {
                *self = Self::LIST(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::LIST(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            4 => {
                *self = Self::ENUM(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::ENUM(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            5 => {
                *self = Self::DECIMAL(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::DECIMAL(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            6 => {
                *self = Self::DATE(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::DATE(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            7 => {
                *self = Self::TIME(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::TIME(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            8 => {
                *self = Self::TIMESTAMP(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::TIMESTAMP(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            10 => {
                *self = Self::INTEGER(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::INTEGER(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            11 => {
                *self = Self::UNKNOWN(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::UNKNOWN(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            12 => {
                *self = Self::JSON(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::JSON(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            13 => {
                *self = Self::BSON(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::BSON(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            14 => {
                *self = Self::UUID(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::UUID(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            15 => {
                *self = Self::FLOAT16(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::FLOAT16(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            _ => {
                return Err(ThriftError::MissingField)
            }
        }
        let stop = input.read_byte()?;
        if stop != 0 {
            return Err(ThriftError::MissingStop)
        }

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::STRING(inner) => inner.write_field(output, 1, &mut last_field_id)?,
            Self::MAP(inner) => inner.write_field(output, 2, &mut last_field_id)?,
            Self::LIST(inner) => inner.write_field(output, 3, &mut last_field_id)?,
            Self::ENUM(inner) => inner.write_field(output, 4, &mut last_field_id)?,
            Self::DECIMAL(inner) => inner.write_field(output, 5, &mut last_field_id)?,
            Self::DATE(inner) => inner.write_field(output, 6, &mut last_field_id)?,
            Self::TIME(inner) => inner.write_field(output, 7, &mut last_field_id)?,
            Self::TIMESTAMP(inner) => inner.write_field(output, 8, &mut last_field_id)?,
            Self::INTEGER(inner) => inner.write_field(output, 10, &mut last_field_id)?,
            Self::UNKNOWN(inner) => inner.write_field(output, 11, &mut last_field_id)?,
            Self::JSON(inner) => inner.write_field(output, 12, &mut last_field_id)?,
            Self::BSON(inner) => inner.write_field(output, 13, &mut last_field_id)?,
            Self::UUID(inner) => inner.write_field(output, 14, &mut last_field_id)?,
            Self::FLOAT16(inner) => inner.write_field(output, 15, &mut last_field_id)?,
        }
        // STOP
        output.write_byte(0)
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct SchemaElement<'i> {
    pub r#type: Option<Type>,
    pub type_length: Option<i32>,
    pub repetition_type: Option<FieldRepetitionType>,
    pub name: Cow<'i, str>,
    pub num_children: Option<i32>,
    pub converted_type: Option<ConvertedType>,
    pub scale: Option<i32>,
    pub precision: Option<i32>,
    pub field_id: Option<i32>,
    pub logicalType: Option<LogicalType>,
}

impl<'i> SchemaElement<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(r#type: impl Into<Option<Type>>, type_length: impl Into<Option<i32>>, repetition_type: impl Into<Option<FieldRepetitionType>>, name: impl Into<Cow<'i, str>>, num_children: impl Into<Option<i32>>, converted_type: impl Into<Option<ConvertedType>>, scale: impl Into<Option<i32>>, precision: impl Into<Option<i32>>, field_id: impl Into<Option<i32>>, logicalType: impl Into<Option<LogicalType>>) -> Self {
        Self {
            r#type: r#type.into(),
            type_length: type_length.into(),
            repetition_type: repetition_type.into(),
            name: name.into(),
            num_children: num_children.into(),
            converted_type: converted_type.into(),
            scale: scale.into(),
            precision: precision.into(),
            field_id: field_id.into(),
            logicalType: logicalType.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for SchemaElement<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut name_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {

                    self.r#type.fill_field(input, field_type)?;
                }
                2 => {

                    self.type_length.fill_field(input, field_type)?;
                }
                3 => {

                    self.repetition_type.fill_field(input, field_type)?;
                }
                4 => {
                    name_set_ = true;
                    self.name.fill_field(input, field_type)?;
                }
                5 => {

                    self.num_children.fill_field(input, field_type)?;
                }
                6 => {

                    self.converted_type.fill_field(input, field_type)?;
                }
                7 => {

                    self.scale.fill_field(input, field_type)?;
                }
                8 => {

                    self.precision.fill_field(input, field_type)?;
                }
                9 => {

                    self.field_id.fill_field(input, field_type)?;
                }
                10 => {

                    self.logicalType.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !name_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.r#type.write_field(output, 1, &mut last_field_id)?;
        self.type_length.write_field(output, 2, &mut last_field_id)?;
        self.repetition_type.write_field(output, 3, &mut last_field_id)?;
        self.name.write_field(output, 4, &mut last_field_id)?;
        self.num_children.write_field(output, 5, &mut last_field_id)?;
        self.converted_type.write_field(output, 6, &mut last_field_id)?;
        self.scale.write_field(output, 7, &mut last_field_id)?;
        self.precision.write_field(output, 8, &mut last_field_id)?;
        self.field_id.write_field(output, 9, &mut last_field_id)?;
        self.logicalType.write_field(output, 10, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct Encoding(pub i32);

impl Encoding {
    pub const PLAIN: Self = Self(0);
    pub const PLAIN_DICTIONARY: Self = Self(2);
    pub const RLE: Self = Self(3);
    pub const BIT_PACKED: Self = Self(4);
    pub const DELTA_BINARY_PACKED: Self = Self(5);
    pub const DELTA_LENGTH_BYTE_ARRAY: Self = Self(6);
    pub const DELTA_BYTE_ARRAY: Self = Self(7);
    pub const RLE_DICTIONARY: Self = Self(8);
    pub const BYTE_STREAM_SPLIT: Self = Self(9);

    const __NAMES: &'static [(i32, &'static str)] = &[
        (0, "PLAIN"),
        (2, "PLAIN_DICTIONARY"),
        (3, "RLE"),
        (4, "BIT_PACKED"),
        (5, "DELTA_BINARY_PACKED"),
        (6, "DELTA_LENGTH_BYTE_ARRAY"),
        (7, "DELTA_BYTE_ARRAY"),
        (8, "RLE_DICTIONARY"),
        (9, "BYTE_STREAM_SPLIT"),
    ];

    #[inline]
    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for Encoding {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let name = match Self::__NAMES.binary_search_by_key(&self.0, |(i, _)| *i) {
            Ok(i) => &Self::__NAMES[i].1,
            Err(_) => "__UNKNOWN",
        };
        f.debug_tuple(name).field(&self.0).finish()
    }
}
impl <'i> CompactThriftProtocol<'i> for Encoding {
    const FIELD_TYPE: u8 = 5; // i32

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}

#[derive(Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct CompressionCodec(pub i32);

impl CompressionCodec {
    pub const UNCOMPRESSED: Self = Self(0);
    pub const SNAPPY: Self = Self(1);
    pub const GZIP: Self = Self(2);
    pub const LZO: Self = Self(3);
    pub const BROTLI: Self = Self(4);
    pub const LZ4: Self = Self(5);
    pub const ZSTD: Self = Self(6);
    pub const LZ4_RAW: Self = Self(7);

    const __NAMES: &'static [(i32, &'static str)] = &[
        (0, "UNCOMPRESSED"),
        (1, "SNAPPY"),
        (2, "GZIP"),
        (3, "LZO"),
        (4, "BROTLI"),
        (5, "LZ4"),
        (6, "ZSTD"),
        (7, "LZ4_RAW"),
    ];

    #[inline]
    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for CompressionCodec {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for CompressionCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let name = match Self::__NAMES.binary_search_by_key(&self.0, |(i, _)| *i) {
            Ok(i) => &Self::__NAMES[i].1,
            Err(_) => "__UNKNOWN",
        };
        f.debug_tuple(name).field(&self.0).finish()
    }
}
impl <'i> CompactThriftProtocol<'i> for CompressionCodec {
    const FIELD_TYPE: u8 = 5; // i32

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}

#[derive(Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct PageType(pub i32);

impl PageType {
    pub const DATA_PAGE: Self = Self(0);
    pub const INDEX_PAGE: Self = Self(1);
    pub const DICTIONARY_PAGE: Self = Self(2);
    pub const DATA_PAGE_V2: Self = Self(3);

    const __NAMES: &'static [(i32, &'static str)] = &[
        (0, "DATA_PAGE"),
        (1, "INDEX_PAGE"),
        (2, "DICTIONARY_PAGE"),
        (3, "DATA_PAGE_V2"),
    ];

    #[inline]
    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for PageType {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for PageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let name = match Self::__NAMES.binary_search_by_key(&self.0, |(i, _)| *i) {
            Ok(i) => &Self::__NAMES[i].1,
            Err(_) => "__UNKNOWN",
        };
        f.debug_tuple(name).field(&self.0).finish()
    }
}
impl <'i> CompactThriftProtocol<'i> for PageType {
    const FIELD_TYPE: u8 = 5; // i32

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}

#[derive(Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct BoundaryOrder(pub i32);

impl BoundaryOrder {
    pub const UNORDERED: Self = Self(0);
    pub const ASCENDING: Self = Self(1);
    pub const DESCENDING: Self = Self(2);

    const __NAMES: &'static [(i32, &'static str)] = &[
        (0, "UNORDERED"),
        (1, "ASCENDING"),
        (2, "DESCENDING"),
    ];

    #[inline]
    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for BoundaryOrder {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for BoundaryOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let name = match Self::__NAMES.binary_search_by_key(&self.0, |(i, _)| *i) {
            Ok(i) => &Self::__NAMES[i].1,
            Err(_) => "__UNKNOWN",
        };
        f.debug_tuple(name).field(&self.0).finish()
    }
}
impl <'i> CompactThriftProtocol<'i> for BoundaryOrder {
    const FIELD_TYPE: u8 = 5; // i32

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DataPageHeader<'i> {
    pub num_values: i32,
    pub encoding: Encoding,
    pub definition_level_encoding: Encoding,
    pub repetition_level_encoding: Encoding,
    pub statistics: Option<Statistics<'i>>,
}

impl<'i> DataPageHeader<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(num_values: impl Into<i32>, encoding: impl Into<Encoding>, definition_level_encoding: impl Into<Encoding>, repetition_level_encoding: impl Into<Encoding>, statistics: impl Into<Option<Statistics<'i>>>) -> Self {
        Self {
            num_values: num_values.into(),
            encoding: encoding.into(),
            definition_level_encoding: definition_level_encoding.into(),
            repetition_level_encoding: repetition_level_encoding.into(),
            statistics: statistics.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for DataPageHeader<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut num_values_set_: bool = false;
        let mut encoding_set_: bool = false;
        let mut definition_level_encoding_set_: bool = false;
        let mut repetition_level_encoding_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    num_values_set_ = true;
                    self.num_values.fill_field(input, field_type)?;
                }
                2 => {
                    encoding_set_ = true;
                    self.encoding.fill_field(input, field_type)?;
                }
                3 => {
                    definition_level_encoding_set_ = true;
                    self.definition_level_encoding.fill_field(input, field_type)?;
                }
                4 => {
                    repetition_level_encoding_set_ = true;
                    self.repetition_level_encoding.fill_field(input, field_type)?;
                }
                5 => {

                    self.statistics.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !num_values_set_ || !encoding_set_ || !definition_level_encoding_set_ || !repetition_level_encoding_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.num_values.write_field(output, 1, &mut last_field_id)?;
        self.encoding.write_field(output, 2, &mut last_field_id)?;
        self.definition_level_encoding.write_field(output, 3, &mut last_field_id)?;
        self.repetition_level_encoding.write_field(output, 4, &mut last_field_id)?;
        self.statistics.write_field(output, 5, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IndexPageHeader {
}

impl IndexPageHeader {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for IndexPageHeader {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DictionaryPageHeader {
    pub num_values: i32,
    pub encoding: Encoding,
    pub is_sorted: Option<bool>,
}

impl DictionaryPageHeader {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(num_values: impl Into<i32>, encoding: impl Into<Encoding>, is_sorted: impl Into<Option<bool>>) -> Self {
        Self {
            num_values: num_values.into(),
            encoding: encoding.into(),
            is_sorted: is_sorted.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for DictionaryPageHeader {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut num_values_set_: bool = false;
        let mut encoding_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    num_values_set_ = true;
                    self.num_values.fill_field(input, field_type)?;
                }
                2 => {
                    encoding_set_ = true;
                    self.encoding.fill_field(input, field_type)?;
                }
                3 => {

                    self.is_sorted.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !num_values_set_ || !encoding_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.num_values.write_field(output, 1, &mut last_field_id)?;
        self.encoding.write_field(output, 2, &mut last_field_id)?;
        self.is_sorted.write_field(output, 3, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DataPageHeaderV2<'i> {
    pub num_values: i32,
    pub num_nulls: i32,
    pub num_rows: i32,
    pub encoding: Encoding,
    pub definition_levels_byte_length: i32,
    pub repetition_levels_byte_length: i32,
    pub is_compressed: Option<bool>,
    pub statistics: Option<Statistics<'i>>,
}

impl<'i> DataPageHeaderV2<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(num_values: impl Into<i32>, num_nulls: impl Into<i32>, num_rows: impl Into<i32>, encoding: impl Into<Encoding>, definition_levels_byte_length: impl Into<i32>, repetition_levels_byte_length: impl Into<i32>, is_compressed: impl Into<Option<bool>>, statistics: impl Into<Option<Statistics<'i>>>) -> Self {
        Self {
            num_values: num_values.into(),
            num_nulls: num_nulls.into(),
            num_rows: num_rows.into(),
            encoding: encoding.into(),
            definition_levels_byte_length: definition_levels_byte_length.into(),
            repetition_levels_byte_length: repetition_levels_byte_length.into(),
            is_compressed: is_compressed.into(),
            statistics: statistics.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for DataPageHeaderV2<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut num_values_set_: bool = false;
        let mut num_nulls_set_: bool = false;
        let mut num_rows_set_: bool = false;
        let mut encoding_set_: bool = false;
        let mut definition_levels_byte_length_set_: bool = false;
        let mut repetition_levels_byte_length_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    num_values_set_ = true;
                    self.num_values.fill_field(input, field_type)?;
                }
                2 => {
                    num_nulls_set_ = true;
                    self.num_nulls.fill_field(input, field_type)?;
                }
                3 => {
                    num_rows_set_ = true;
                    self.num_rows.fill_field(input, field_type)?;
                }
                4 => {
                    encoding_set_ = true;
                    self.encoding.fill_field(input, field_type)?;
                }
                5 => {
                    definition_levels_byte_length_set_ = true;
                    self.definition_levels_byte_length.fill_field(input, field_type)?;
                }
                6 => {
                    repetition_levels_byte_length_set_ = true;
                    self.repetition_levels_byte_length.fill_field(input, field_type)?;
                }
                7 => {

                    self.is_compressed.fill_field(input, field_type)?;
                }
                8 => {

                    self.statistics.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !num_values_set_ || !num_nulls_set_ || !num_rows_set_ || !encoding_set_ || !definition_levels_byte_length_set_ || !repetition_levels_byte_length_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.num_values.write_field(output, 1, &mut last_field_id)?;
        self.num_nulls.write_field(output, 2, &mut last_field_id)?;
        self.num_rows.write_field(output, 3, &mut last_field_id)?;
        self.encoding.write_field(output, 4, &mut last_field_id)?;
        self.definition_levels_byte_length.write_field(output, 5, &mut last_field_id)?;
        self.repetition_levels_byte_length.write_field(output, 6, &mut last_field_id)?;
        self.is_compressed.write_field(output, 7, &mut last_field_id)?;
        self.statistics.write_field(output, 8, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct SplitBlockAlgorithm {
}

impl SplitBlockAlgorithm {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for SplitBlockAlgorithm {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum BloomFilterAlgorithm {
    BLOCK(SplitBlockAlgorithm),
}
impl Default for BloomFilterAlgorithm {
    fn default() -> Self {
        Self::BLOCK(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for BloomFilterAlgorithm {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        let field_type = input.read_field_header(&mut last_field_id)?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        match last_field_id {
            1 => {
                *self = Self::BLOCK(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::BLOCK(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            _ => {
                return Err(ThriftError::MissingField)
            }
        }
        let stop = input.read_byte()?;
        if stop != 0 {
            return Err(ThriftError::MissingStop)
        }

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::BLOCK(inner) => inner.write_field(output, 1, &mut last_field_id)?,
        }
        // STOP
        output.write_byte(0)
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct XxHash {
}

impl XxHash {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for XxHash {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum BloomFilterHash {
    XXHASH(XxHash),
}
impl Default for BloomFilterHash {
    fn default() -> Self {
        Self::XXHASH(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for BloomFilterHash {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        let field_type = input.read_field_header(&mut last_field_id)?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        match last_field_id {
            1 => {
                *self = Self::XXHASH(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::XXHASH(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            _ => {
                return Err(ThriftError::MissingField)
            }
        }
        let stop = input.read_byte()?;
        if stop != 0 {
            return Err(ThriftError::MissingStop)
        }

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::XXHASH(inner) => inner.write_field(output, 1, &mut last_field_id)?,
        }
        // STOP
        output.write_byte(0)
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Uncompressed {
}

impl Uncompressed {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for Uncompressed {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum BloomFilterCompression {
    UNCOMPRESSED(Uncompressed),
}
impl Default for BloomFilterCompression {
    fn default() -> Self {
        Self::UNCOMPRESSED(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for BloomFilterCompression {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        let field_type = input.read_field_header(&mut last_field_id)?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        match last_field_id {
            1 => {
                *self = Self::UNCOMPRESSED(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::UNCOMPRESSED(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            _ => {
                return Err(ThriftError::MissingField)
            }
        }
        let stop = input.read_byte()?;
        if stop != 0 {
            return Err(ThriftError::MissingStop)
        }

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::UNCOMPRESSED(inner) => inner.write_field(output, 1, &mut last_field_id)?,
        }
        // STOP
        output.write_byte(0)
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct BloomFilterHeader {
    pub numBytes: i32,
    pub algorithm: BloomFilterAlgorithm,
    pub hash: BloomFilterHash,
    pub compression: BloomFilterCompression,
}

impl BloomFilterHeader {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(numBytes: impl Into<i32>, algorithm: impl Into<BloomFilterAlgorithm>, hash: impl Into<BloomFilterHash>, compression: impl Into<BloomFilterCompression>) -> Self {
        Self {
            numBytes: numBytes.into(),
            algorithm: algorithm.into(),
            hash: hash.into(),
            compression: compression.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for BloomFilterHeader {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut numBytes_set_: bool = false;
        let mut algorithm_set_: bool = false;
        let mut hash_set_: bool = false;
        let mut compression_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    numBytes_set_ = true;
                    self.numBytes.fill_field(input, field_type)?;
                }
                2 => {
                    algorithm_set_ = true;
                    self.algorithm.fill_field(input, field_type)?;
                }
                3 => {
                    hash_set_ = true;
                    self.hash.fill_field(input, field_type)?;
                }
                4 => {
                    compression_set_ = true;
                    self.compression.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !numBytes_set_ || !algorithm_set_ || !hash_set_ || !compression_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.numBytes.write_field(output, 1, &mut last_field_id)?;
        self.algorithm.write_field(output, 2, &mut last_field_id)?;
        self.hash.write_field(output, 3, &mut last_field_id)?;
        self.compression.write_field(output, 4, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct PageHeader<'i> {
    pub r#type: PageType,
    pub uncompressed_page_size: i32,
    pub compressed_page_size: i32,
    pub crc: Option<i32>,
    pub data_page_header: Option<DataPageHeader<'i>>,
    pub index_page_header: Option<IndexPageHeader>,
    pub dictionary_page_header: Option<DictionaryPageHeader>,
    pub data_page_header_v2: Option<DataPageHeaderV2<'i>>,
}

impl<'i> PageHeader<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(r#type: impl Into<PageType>, uncompressed_page_size: impl Into<i32>, compressed_page_size: impl Into<i32>, crc: impl Into<Option<i32>>, data_page_header: impl Into<Option<DataPageHeader<'i>>>, index_page_header: impl Into<Option<IndexPageHeader>>, dictionary_page_header: impl Into<Option<DictionaryPageHeader>>, data_page_header_v2: impl Into<Option<DataPageHeaderV2<'i>>>) -> Self {
        Self {
            r#type: r#type.into(),
            uncompressed_page_size: uncompressed_page_size.into(),
            compressed_page_size: compressed_page_size.into(),
            crc: crc.into(),
            data_page_header: data_page_header.into(),
            index_page_header: index_page_header.into(),
            dictionary_page_header: dictionary_page_header.into(),
            data_page_header_v2: data_page_header_v2.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for PageHeader<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut r#type_set_: bool = false;
        let mut uncompressed_page_size_set_: bool = false;
        let mut compressed_page_size_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    r#type_set_ = true;
                    self.r#type.fill_field(input, field_type)?;
                }
                2 => {
                    uncompressed_page_size_set_ = true;
                    self.uncompressed_page_size.fill_field(input, field_type)?;
                }
                3 => {
                    compressed_page_size_set_ = true;
                    self.compressed_page_size.fill_field(input, field_type)?;
                }
                4 => {

                    self.crc.fill_field(input, field_type)?;
                }
                5 => {

                    self.data_page_header.fill_field(input, field_type)?;
                }
                6 => {

                    self.index_page_header.fill_field(input, field_type)?;
                }
                7 => {

                    self.dictionary_page_header.fill_field(input, field_type)?;
                }
                8 => {

                    self.data_page_header_v2.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !r#type_set_ || !uncompressed_page_size_set_ || !compressed_page_size_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.r#type.write_field(output, 1, &mut last_field_id)?;
        self.uncompressed_page_size.write_field(output, 2, &mut last_field_id)?;
        self.compressed_page_size.write_field(output, 3, &mut last_field_id)?;
        self.crc.write_field(output, 4, &mut last_field_id)?;
        self.data_page_header.write_field(output, 5, &mut last_field_id)?;
        self.index_page_header.write_field(output, 6, &mut last_field_id)?;
        self.dictionary_page_header.write_field(output, 7, &mut last_field_id)?;
        self.data_page_header_v2.write_field(output, 8, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct KeyValue<'i> {
    pub key: Cow<'i, str>,
    pub value: Option<Cow<'i, str>>,
}

impl<'i> KeyValue<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(key: impl Into<Cow<'i, str>>, value: impl Into<Option<Cow<'i, str>>>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for KeyValue<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut key_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    key_set_ = true;
                    self.key.fill_field(input, field_type)?;
                }
                2 => {

                    self.value.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !key_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.key.write_field(output, 1, &mut last_field_id)?;
        self.value.write_field(output, 2, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct SortingColumn {
    pub column_idx: i32,
    pub descending: bool,
    pub nulls_first: bool,
}

impl SortingColumn {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(column_idx: impl Into<i32>, descending: impl Into<bool>, nulls_first: impl Into<bool>) -> Self {
        Self {
            column_idx: column_idx.into(),
            descending: descending.into(),
            nulls_first: nulls_first.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for SortingColumn {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut column_idx_set_: bool = false;
        let mut descending_set_: bool = false;
        let mut nulls_first_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    column_idx_set_ = true;
                    self.column_idx.fill_field(input, field_type)?;
                }
                2 => {
                    descending_set_ = true;
                    self.descending.fill_field(input, field_type)?;
                }
                3 => {
                    nulls_first_set_ = true;
                    self.nulls_first.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !column_idx_set_ || !descending_set_ || !nulls_first_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.column_idx.write_field(output, 1, &mut last_field_id)?;
        self.descending.write_field(output, 2, &mut last_field_id)?;
        self.nulls_first.write_field(output, 3, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct PageEncodingStats {
    pub page_type: PageType,
    pub encoding: Encoding,
    pub count: i32,
}

impl PageEncodingStats {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(page_type: impl Into<PageType>, encoding: impl Into<Encoding>, count: impl Into<i32>) -> Self {
        Self {
            page_type: page_type.into(),
            encoding: encoding.into(),
            count: count.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for PageEncodingStats {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut page_type_set_: bool = false;
        let mut encoding_set_: bool = false;
        let mut count_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    page_type_set_ = true;
                    self.page_type.fill_field(input, field_type)?;
                }
                2 => {
                    encoding_set_ = true;
                    self.encoding.fill_field(input, field_type)?;
                }
                3 => {
                    count_set_ = true;
                    self.count.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !page_type_set_ || !encoding_set_ || !count_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.page_type.write_field(output, 1, &mut last_field_id)?;
        self.encoding.write_field(output, 2, &mut last_field_id)?;
        self.count.write_field(output, 3, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ColumnMetaData<'i> {
    pub r#type: Type,
    pub encodings: Vec<Encoding>,
    pub path_in_schema: Vec<Cow<'i, str>>,
    pub codec: CompressionCodec,
    pub num_values: i64,
    pub total_uncompressed_size: i64,
    pub total_compressed_size: i64,
    pub key_value_metadata: Option<Vec<KeyValue<'i>>>,
    pub data_page_offset: i64,
    pub index_page_offset: Option<i64>,
    pub dictionary_page_offset: Option<i64>,
    pub statistics: Option<Statistics<'i>>,
    pub encoding_stats: Option<Vec<PageEncodingStats>>,
    pub bloom_filter_offset: Option<i64>,
    pub bloom_filter_length: Option<i32>,
    pub size_statistics: Option<SizeStatistics>,
}

impl<'i> ColumnMetaData<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(r#type: impl Into<Type>, encodings: impl Into<Vec<Encoding>>, path_in_schema: impl Into<Vec<Cow<'i, str>>>, codec: impl Into<CompressionCodec>, num_values: impl Into<i64>, total_uncompressed_size: impl Into<i64>, total_compressed_size: impl Into<i64>, key_value_metadata: impl Into<Option<Vec<KeyValue<'i>>>>, data_page_offset: impl Into<i64>, index_page_offset: impl Into<Option<i64>>, dictionary_page_offset: impl Into<Option<i64>>, statistics: impl Into<Option<Statistics<'i>>>, encoding_stats: impl Into<Option<Vec<PageEncodingStats>>>, bloom_filter_offset: impl Into<Option<i64>>, bloom_filter_length: impl Into<Option<i32>>, size_statistics: impl Into<Option<SizeStatistics>>) -> Self {
        Self {
            r#type: r#type.into(),
            encodings: encodings.into(),
            path_in_schema: path_in_schema.into(),
            codec: codec.into(),
            num_values: num_values.into(),
            total_uncompressed_size: total_uncompressed_size.into(),
            total_compressed_size: total_compressed_size.into(),
            key_value_metadata: key_value_metadata.into(),
            data_page_offset: data_page_offset.into(),
            index_page_offset: index_page_offset.into(),
            dictionary_page_offset: dictionary_page_offset.into(),
            statistics: statistics.into(),
            encoding_stats: encoding_stats.into(),
            bloom_filter_offset: bloom_filter_offset.into(),
            bloom_filter_length: bloom_filter_length.into(),
            size_statistics: size_statistics.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for ColumnMetaData<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut r#type_set_: bool = false;
        let mut encodings_set_: bool = false;
        let mut path_in_schema_set_: bool = false;
        let mut codec_set_: bool = false;
        let mut num_values_set_: bool = false;
        let mut total_uncompressed_size_set_: bool = false;
        let mut total_compressed_size_set_: bool = false;
        let mut data_page_offset_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    r#type_set_ = true;
                    self.r#type.fill_field(input, field_type)?;
                }
                2 => {
                    encodings_set_ = true;
                    self.encodings.fill_field(input, field_type)?;
                }
                3 => {
                    path_in_schema_set_ = true;
                    self.path_in_schema.fill_field(input, field_type)?;
                }
                4 => {
                    codec_set_ = true;
                    self.codec.fill_field(input, field_type)?;
                }
                5 => {
                    num_values_set_ = true;
                    self.num_values.fill_field(input, field_type)?;
                }
                6 => {
                    total_uncompressed_size_set_ = true;
                    self.total_uncompressed_size.fill_field(input, field_type)?;
                }
                7 => {
                    total_compressed_size_set_ = true;
                    self.total_compressed_size.fill_field(input, field_type)?;
                }
                8 => {

                    self.key_value_metadata.fill_field(input, field_type)?;
                }
                9 => {
                    data_page_offset_set_ = true;
                    self.data_page_offset.fill_field(input, field_type)?;
                }
                10 => {

                    self.index_page_offset.fill_field(input, field_type)?;
                }
                11 => {

                    self.dictionary_page_offset.fill_field(input, field_type)?;
                }
                12 => {

                    self.statistics.fill_field(input, field_type)?;
                }
                13 => {

                    self.encoding_stats.fill_field(input, field_type)?;
                }
                14 => {

                    self.bloom_filter_offset.fill_field(input, field_type)?;
                }
                15 => {

                    self.bloom_filter_length.fill_field(input, field_type)?;
                }
                16 => {

                    self.size_statistics.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !r#type_set_ || !encodings_set_ || !path_in_schema_set_ || !codec_set_ || !num_values_set_ || !total_uncompressed_size_set_ || !total_compressed_size_set_ || !data_page_offset_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.r#type.write_field(output, 1, &mut last_field_id)?;
        self.encodings.write_field(output, 2, &mut last_field_id)?;
        self.path_in_schema.write_field(output, 3, &mut last_field_id)?;
        self.codec.write_field(output, 4, &mut last_field_id)?;
        self.num_values.write_field(output, 5, &mut last_field_id)?;
        self.total_uncompressed_size.write_field(output, 6, &mut last_field_id)?;
        self.total_compressed_size.write_field(output, 7, &mut last_field_id)?;
        self.key_value_metadata.write_field(output, 8, &mut last_field_id)?;
        self.data_page_offset.write_field(output, 9, &mut last_field_id)?;
        self.index_page_offset.write_field(output, 10, &mut last_field_id)?;
        self.dictionary_page_offset.write_field(output, 11, &mut last_field_id)?;
        self.statistics.write_field(output, 12, &mut last_field_id)?;
        self.encoding_stats.write_field(output, 13, &mut last_field_id)?;
        self.bloom_filter_offset.write_field(output, 14, &mut last_field_id)?;
        self.bloom_filter_length.write_field(output, 15, &mut last_field_id)?;
        self.size_statistics.write_field(output, 16, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct EncryptionWithFooterKey {
}

impl EncryptionWithFooterKey {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for EncryptionWithFooterKey {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct EncryptionWithColumnKey<'i> {
    pub path_in_schema: Vec<Cow<'i, str>>,
    pub key_metadata: Option<Cow<'i, [u8]>>,
}

impl<'i> EncryptionWithColumnKey<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(path_in_schema: impl Into<Vec<Cow<'i, str>>>, key_metadata: impl Into<Option<Cow<'i, [u8]>>>) -> Self {
        Self {
            path_in_schema: path_in_schema.into(),
            key_metadata: key_metadata.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for EncryptionWithColumnKey<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut path_in_schema_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    path_in_schema_set_ = true;
                    self.path_in_schema.fill_field(input, field_type)?;
                }
                2 => {

                    self.key_metadata.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !path_in_schema_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.path_in_schema.write_field(output, 1, &mut last_field_id)?;
        self.key_metadata.write_field(output, 2, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ColumnCryptoMetaData<'i> {
    ENCRYPTION_WITH_FOOTER_KEY(EncryptionWithFooterKey),
    ENCRYPTION_WITH_COLUMN_KEY(EncryptionWithColumnKey<'i>),
}
impl<'i> Default for ColumnCryptoMetaData<'i> {
    fn default() -> Self {
        Self::ENCRYPTION_WITH_FOOTER_KEY(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for ColumnCryptoMetaData<'i> {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        let field_type = input.read_field_header(&mut last_field_id)?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        match last_field_id {
            1 => {
                *self = Self::ENCRYPTION_WITH_FOOTER_KEY(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::ENCRYPTION_WITH_FOOTER_KEY(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            2 => {
                *self = Self::ENCRYPTION_WITH_COLUMN_KEY(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::ENCRYPTION_WITH_COLUMN_KEY(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            _ => {
                return Err(ThriftError::MissingField)
            }
        }
        let stop = input.read_byte()?;
        if stop != 0 {
            return Err(ThriftError::MissingStop)
        }

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::ENCRYPTION_WITH_FOOTER_KEY(inner) => inner.write_field(output, 1, &mut last_field_id)?,
            Self::ENCRYPTION_WITH_COLUMN_KEY(inner) => inner.write_field(output, 2, &mut last_field_id)?,
        }
        // STOP
        output.write_byte(0)
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ColumnChunk<'i> {
    pub file_path: Option<Cow<'i, str>>,
    pub file_offset: i64,
    pub meta_data: Option<ColumnMetaData<'i>>,
    pub offset_index_offset: Option<i64>,
    pub offset_index_length: Option<i32>,
    pub column_index_offset: Option<i64>,
    pub column_index_length: Option<i32>,
    pub crypto_metadata: Option<ColumnCryptoMetaData<'i>>,
    pub encrypted_column_metadata: Option<Cow<'i, [u8]>>,
}

impl<'i> ColumnChunk<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(file_path: impl Into<Option<Cow<'i, str>>>, file_offset: impl Into<i64>, meta_data: impl Into<Option<ColumnMetaData<'i>>>, offset_index_offset: impl Into<Option<i64>>, offset_index_length: impl Into<Option<i32>>, column_index_offset: impl Into<Option<i64>>, column_index_length: impl Into<Option<i32>>, crypto_metadata: impl Into<Option<ColumnCryptoMetaData<'i>>>, encrypted_column_metadata: impl Into<Option<Cow<'i, [u8]>>>) -> Self {
        Self {
            file_path: file_path.into(),
            file_offset: file_offset.into(),
            meta_data: meta_data.into(),
            offset_index_offset: offset_index_offset.into(),
            offset_index_length: offset_index_length.into(),
            column_index_offset: column_index_offset.into(),
            column_index_length: column_index_length.into(),
            crypto_metadata: crypto_metadata.into(),
            encrypted_column_metadata: encrypted_column_metadata.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for ColumnChunk<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut file_offset_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {

                    self.file_path.fill_field(input, field_type)?;
                }
                2 => {
                    file_offset_set_ = true;
                    self.file_offset.fill_field(input, field_type)?;
                }
                3 => {

                    self.meta_data.fill_field(input, field_type)?;
                }
                4 => {

                    self.offset_index_offset.fill_field(input, field_type)?;
                }
                5 => {

                    self.offset_index_length.fill_field(input, field_type)?;
                }
                6 => {

                    self.column_index_offset.fill_field(input, field_type)?;
                }
                7 => {

                    self.column_index_length.fill_field(input, field_type)?;
                }
                8 => {

                    self.crypto_metadata.fill_field(input, field_type)?;
                }
                9 => {

                    self.encrypted_column_metadata.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !file_offset_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.file_path.write_field(output, 1, &mut last_field_id)?;
        self.file_offset.write_field(output, 2, &mut last_field_id)?;
        self.meta_data.write_field(output, 3, &mut last_field_id)?;
        self.offset_index_offset.write_field(output, 4, &mut last_field_id)?;
        self.offset_index_length.write_field(output, 5, &mut last_field_id)?;
        self.column_index_offset.write_field(output, 6, &mut last_field_id)?;
        self.column_index_length.write_field(output, 7, &mut last_field_id)?;
        self.crypto_metadata.write_field(output, 8, &mut last_field_id)?;
        self.encrypted_column_metadata.write_field(output, 9, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct RowGroup<'i> {
    pub columns: Vec<ColumnChunk<'i>>,
    pub total_byte_size: i64,
    pub num_rows: i64,
    pub sorting_columns: Option<Vec<SortingColumn>>,
    pub file_offset: Option<i64>,
    pub total_compressed_size: Option<i64>,
    pub ordinal: Option<i16>,
}

impl<'i> RowGroup<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(columns: impl Into<Vec<ColumnChunk<'i>>>, total_byte_size: impl Into<i64>, num_rows: impl Into<i64>, sorting_columns: impl Into<Option<Vec<SortingColumn>>>, file_offset: impl Into<Option<i64>>, total_compressed_size: impl Into<Option<i64>>, ordinal: impl Into<Option<i16>>) -> Self {
        Self {
            columns: columns.into(),
            total_byte_size: total_byte_size.into(),
            num_rows: num_rows.into(),
            sorting_columns: sorting_columns.into(),
            file_offset: file_offset.into(),
            total_compressed_size: total_compressed_size.into(),
            ordinal: ordinal.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for RowGroup<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut columns_set_: bool = false;
        let mut total_byte_size_set_: bool = false;
        let mut num_rows_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    columns_set_ = true;
                    self.columns.fill_field(input, field_type)?;
                }
                2 => {
                    total_byte_size_set_ = true;
                    self.total_byte_size.fill_field(input, field_type)?;
                }
                3 => {
                    num_rows_set_ = true;
                    self.num_rows.fill_field(input, field_type)?;
                }
                4 => {

                    self.sorting_columns.fill_field(input, field_type)?;
                }
                5 => {

                    self.file_offset.fill_field(input, field_type)?;
                }
                6 => {

                    self.total_compressed_size.fill_field(input, field_type)?;
                }
                7 => {

                    self.ordinal.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !columns_set_ || !total_byte_size_set_ || !num_rows_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.columns.write_field(output, 1, &mut last_field_id)?;
        self.total_byte_size.write_field(output, 2, &mut last_field_id)?;
        self.num_rows.write_field(output, 3, &mut last_field_id)?;
        self.sorting_columns.write_field(output, 4, &mut last_field_id)?;
        self.file_offset.write_field(output, 5, &mut last_field_id)?;
        self.total_compressed_size.write_field(output, 6, &mut last_field_id)?;
        self.ordinal.write_field(output, 7, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct TypeDefinedOrder {
}

impl TypeDefinedOrder {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new() -> Self {
        Self {
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for TypeDefinedOrder {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {

        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ColumnOrder {
    TYPE_ORDER(TypeDefinedOrder),
}
impl Default for ColumnOrder {
    fn default() -> Self {
        Self::TYPE_ORDER(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for ColumnOrder {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        let field_type = input.read_field_header(&mut last_field_id)?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        match last_field_id {
            1 => {
                *self = Self::TYPE_ORDER(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::TYPE_ORDER(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            _ => {
                return Err(ThriftError::MissingField)
            }
        }
        let stop = input.read_byte()?;
        if stop != 0 {
            return Err(ThriftError::MissingStop)
        }

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::TYPE_ORDER(inner) => inner.write_field(output, 1, &mut last_field_id)?,
        }
        // STOP
        output.write_byte(0)
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct PageLocation {
    pub offset: i64,
    pub compressed_page_size: i32,
    pub first_row_index: i64,
}

impl PageLocation {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(offset: impl Into<i64>, compressed_page_size: impl Into<i32>, first_row_index: impl Into<i64>) -> Self {
        Self {
            offset: offset.into(),
            compressed_page_size: compressed_page_size.into(),
            first_row_index: first_row_index.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for PageLocation {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut offset_set_: bool = false;
        let mut compressed_page_size_set_: bool = false;
        let mut first_row_index_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    offset_set_ = true;
                    self.offset.fill_field(input, field_type)?;
                }
                2 => {
                    compressed_page_size_set_ = true;
                    self.compressed_page_size.fill_field(input, field_type)?;
                }
                3 => {
                    first_row_index_set_ = true;
                    self.first_row_index.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !offset_set_ || !compressed_page_size_set_ || !first_row_index_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.offset.write_field(output, 1, &mut last_field_id)?;
        self.compressed_page_size.write_field(output, 2, &mut last_field_id)?;
        self.first_row_index.write_field(output, 3, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct OffsetIndex {
    pub page_locations: Vec<PageLocation>,
    pub unencoded_byte_array_data_bytes: Option<Vec<i64>>,
}

impl OffsetIndex {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(page_locations: impl Into<Vec<PageLocation>>, unencoded_byte_array_data_bytes: impl Into<Option<Vec<i64>>>) -> Self {
        Self {
            page_locations: page_locations.into(),
            unencoded_byte_array_data_bytes: unencoded_byte_array_data_bytes.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for OffsetIndex {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut page_locations_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    page_locations_set_ = true;
                    self.page_locations.fill_field(input, field_type)?;
                }
                2 => {

                    self.unencoded_byte_array_data_bytes.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !page_locations_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.page_locations.write_field(output, 1, &mut last_field_id)?;
        self.unencoded_byte_array_data_bytes.write_field(output, 2, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ColumnIndex<'i> {
    pub null_pages: Vec<bool>,
    pub min_values: Vec<Cow<'i, [u8]>>,
    pub max_values: Vec<Cow<'i, [u8]>>,
    pub boundary_order: BoundaryOrder,
    pub null_counts: Option<Vec<i64>>,
    pub repetition_level_histograms: Option<Vec<i64>>,
    pub definition_level_histograms: Option<Vec<i64>>,
}

impl<'i> ColumnIndex<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(null_pages: impl Into<Vec<bool>>, min_values: impl Into<Vec<Cow<'i, [u8]>>>, max_values: impl Into<Vec<Cow<'i, [u8]>>>, boundary_order: impl Into<BoundaryOrder>, null_counts: impl Into<Option<Vec<i64>>>, repetition_level_histograms: impl Into<Option<Vec<i64>>>, definition_level_histograms: impl Into<Option<Vec<i64>>>) -> Self {
        Self {
            null_pages: null_pages.into(),
            min_values: min_values.into(),
            max_values: max_values.into(),
            boundary_order: boundary_order.into(),
            null_counts: null_counts.into(),
            repetition_level_histograms: repetition_level_histograms.into(),
            definition_level_histograms: definition_level_histograms.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for ColumnIndex<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut null_pages_set_: bool = false;
        let mut min_values_set_: bool = false;
        let mut max_values_set_: bool = false;
        let mut boundary_order_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    null_pages_set_ = true;
                    self.null_pages.fill_field(input, field_type)?;
                }
                2 => {
                    min_values_set_ = true;
                    self.min_values.fill_field(input, field_type)?;
                }
                3 => {
                    max_values_set_ = true;
                    self.max_values.fill_field(input, field_type)?;
                }
                4 => {
                    boundary_order_set_ = true;
                    self.boundary_order.fill_field(input, field_type)?;
                }
                5 => {

                    self.null_counts.fill_field(input, field_type)?;
                }
                6 => {

                    self.repetition_level_histograms.fill_field(input, field_type)?;
                }
                7 => {

                    self.definition_level_histograms.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !null_pages_set_ || !min_values_set_ || !max_values_set_ || !boundary_order_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.null_pages.write_field(output, 1, &mut last_field_id)?;
        self.min_values.write_field(output, 2, &mut last_field_id)?;
        self.max_values.write_field(output, 3, &mut last_field_id)?;
        self.boundary_order.write_field(output, 4, &mut last_field_id)?;
        self.null_counts.write_field(output, 5, &mut last_field_id)?;
        self.repetition_level_histograms.write_field(output, 6, &mut last_field_id)?;
        self.definition_level_histograms.write_field(output, 7, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct AesGcmV1<'i> {
    pub aad_prefix: Option<Cow<'i, [u8]>>,
    pub aad_file_unique: Option<Cow<'i, [u8]>>,
    pub supply_aad_prefix: Option<bool>,
}

impl<'i> AesGcmV1<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(aad_prefix: impl Into<Option<Cow<'i, [u8]>>>, aad_file_unique: impl Into<Option<Cow<'i, [u8]>>>, supply_aad_prefix: impl Into<Option<bool>>) -> Self {
        Self {
            aad_prefix: aad_prefix.into(),
            aad_file_unique: aad_file_unique.into(),
            supply_aad_prefix: supply_aad_prefix.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for AesGcmV1<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {

                    self.aad_prefix.fill_field(input, field_type)?;
                }
                2 => {

                    self.aad_file_unique.fill_field(input, field_type)?;
                }
                3 => {

                    self.supply_aad_prefix.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.aad_prefix.write_field(output, 1, &mut last_field_id)?;
        self.aad_file_unique.write_field(output, 2, &mut last_field_id)?;
        self.supply_aad_prefix.write_field(output, 3, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct AesGcmCtrV1<'i> {
    pub aad_prefix: Option<Cow<'i, [u8]>>,
    pub aad_file_unique: Option<Cow<'i, [u8]>>,
    pub supply_aad_prefix: Option<bool>,
}

impl<'i> AesGcmCtrV1<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(aad_prefix: impl Into<Option<Cow<'i, [u8]>>>, aad_file_unique: impl Into<Option<Cow<'i, [u8]>>>, supply_aad_prefix: impl Into<Option<bool>>) -> Self {
        Self {
            aad_prefix: aad_prefix.into(),
            aad_file_unique: aad_file_unique.into(),
            supply_aad_prefix: supply_aad_prefix.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for AesGcmCtrV1<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {

                    self.aad_prefix.fill_field(input, field_type)?;
                }
                2 => {

                    self.aad_file_unique.fill_field(input, field_type)?;
                }
                3 => {

                    self.supply_aad_prefix.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.aad_prefix.write_field(output, 1, &mut last_field_id)?;
        self.aad_file_unique.write_field(output, 2, &mut last_field_id)?;
        self.supply_aad_prefix.write_field(output, 3, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum EncryptionAlgorithm<'i> {
    AES_GCM_V1(AesGcmV1<'i>),
    AES_GCM_CTR_V1(AesGcmCtrV1<'i>),
}
impl<'i> Default for EncryptionAlgorithm<'i> {
    fn default() -> Self {
        Self::AES_GCM_V1(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for EncryptionAlgorithm<'i> {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        let field_type = input.read_field_header(&mut last_field_id)?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        match last_field_id {
            1 => {
                *self = Self::AES_GCM_V1(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::AES_GCM_V1(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            2 => {
                *self = Self::AES_GCM_CTR_V1(Default::default());
                #[allow(unreachable_patterns)]
                match self {
                    Self::AES_GCM_CTR_V1(inner) => inner.fill(input)?,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
            _ => {
                return Err(ThriftError::MissingField)
            }
        }
        let stop = input.read_byte()?;
        if stop != 0 {
            return Err(ThriftError::MissingStop)
        }

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::AES_GCM_V1(inner) => inner.write_field(output, 1, &mut last_field_id)?,
            Self::AES_GCM_CTR_V1(inner) => inner.write_field(output, 2, &mut last_field_id)?,
        }
        // STOP
        output.write_byte(0)
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct FileMetaData<'i> {
    pub version: i32,
    pub schema: Vec<SchemaElement<'i>>,
    pub num_rows: i64,
    pub row_groups: Vec<RowGroup<'i>>,
    pub key_value_metadata: Option<Vec<KeyValue<'i>>>,
    pub created_by: Option<Cow<'i, str>>,
    pub column_orders: Option<Vec<ColumnOrder>>,
    pub encryption_algorithm: Option<EncryptionAlgorithm<'i>>,
    pub footer_signing_key_metadata: Option<Cow<'i, [u8]>>,
}

impl<'i> FileMetaData<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(version: impl Into<i32>, schema: impl Into<Vec<SchemaElement<'i>>>, num_rows: impl Into<i64>, row_groups: impl Into<Vec<RowGroup<'i>>>, key_value_metadata: impl Into<Option<Vec<KeyValue<'i>>>>, created_by: impl Into<Option<Cow<'i, str>>>, column_orders: impl Into<Option<Vec<ColumnOrder>>>, encryption_algorithm: impl Into<Option<EncryptionAlgorithm<'i>>>, footer_signing_key_metadata: impl Into<Option<Cow<'i, [u8]>>>) -> Self {
        Self {
            version: version.into(),
            schema: schema.into(),
            num_rows: num_rows.into(),
            row_groups: row_groups.into(),
            key_value_metadata: key_value_metadata.into(),
            created_by: created_by.into(),
            column_orders: column_orders.into(),
            encryption_algorithm: encryption_algorithm.into(),
            footer_signing_key_metadata: footer_signing_key_metadata.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for FileMetaData<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut version_set_: bool = false;
        let mut schema_set_: bool = false;
        let mut num_rows_set_: bool = false;
        let mut row_groups_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    version_set_ = true;
                    self.version.fill_field(input, field_type)?;
                }
                2 => {
                    schema_set_ = true;
                    self.schema.fill_field(input, field_type)?;
                }
                3 => {
                    num_rows_set_ = true;
                    self.num_rows.fill_field(input, field_type)?;
                }
                4 => {
                    row_groups_set_ = true;
                    self.row_groups.fill_field(input, field_type)?;
                }
                5 => {

                    self.key_value_metadata.fill_field(input, field_type)?;
                }
                6 => {

                    self.created_by.fill_field(input, field_type)?;
                }
                7 => {

                    self.column_orders.fill_field(input, field_type)?;
                }
                8 => {

                    self.encryption_algorithm.fill_field(input, field_type)?;
                }
                9 => {

                    self.footer_signing_key_metadata.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !version_set_ || !schema_set_ || !num_rows_set_ || !row_groups_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.version.write_field(output, 1, &mut last_field_id)?;
        self.schema.write_field(output, 2, &mut last_field_id)?;
        self.num_rows.write_field(output, 3, &mut last_field_id)?;
        self.row_groups.write_field(output, 4, &mut last_field_id)?;
        self.key_value_metadata.write_field(output, 5, &mut last_field_id)?;
        self.created_by.write_field(output, 6, &mut last_field_id)?;
        self.column_orders.write_field(output, 7, &mut last_field_id)?;
        self.encryption_algorithm.write_field(output, 8, &mut last_field_id)?;
        self.footer_signing_key_metadata.write_field(output, 9, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct FileCryptoMetaData<'i> {
    pub encryption_algorithm: EncryptionAlgorithm<'i>,
    pub key_metadata: Option<Cow<'i, [u8]>>,
}

impl<'i> FileCryptoMetaData<'i> {
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub fn new(encryption_algorithm: impl Into<EncryptionAlgorithm<'i>>, key_metadata: impl Into<Option<Cow<'i, [u8]>>>) -> Self {
        Self {
            encryption_algorithm: encryption_algorithm.into(),
            key_metadata: key_metadata.into(),
        }
    }
}

impl <'i> CompactThriftProtocol<'i> for FileCryptoMetaData<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut encryption_algorithm_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_type = input.read_field_header(&mut last_field_id)?;
            if field_type == 0 {
                break;
            }

            match last_field_id {
                1 => {
                    encryption_algorithm_set_ = true;
                    self.encryption_algorithm.fill_field(input, field_type)?;
                }
                2 => {

                    self.key_metadata.fill_field(input, field_type)?;
                }
                _ => {
                    input.skip_field(field_type)?;
                }
            }
        }

        if !encryption_algorithm_set_ {
            return Err(ThriftError::MissingField)
        }


        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        self.encryption_algorithm.write_field(output, 1, &mut last_field_id)?;
        self.key_metadata.write_field(output, 2, &mut last_field_id)?;
        output.write_byte(0)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_compile() {

    }
}
