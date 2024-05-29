#[allow(non_snake_case)]
// Generated on 2024-05-29T21:19:49.416375107Z
use std::borrow::Cow;
use std::marker::PhantomData;
use compact_thrift_rs::*;

#[derive(Default, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct Type(i32);

impl Type {
    pub const BOOLEAN: i32 = 0;
    pub const INT32: i32 = 1;
    pub const INT64: i32 = 2;
    pub const INT96: i32 = 3;
    pub const FLOAT: i32 = 4;
    pub const DOUBLE: i32 = 5;
    pub const BYTE_ARRAY: i32 = 6;
    pub const FIXED_LEN_BYTE_ARRAY: i32 = 7;

    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for Type {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl <'i> CompactThriftProtocol<'i> for Type {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}


#[derive(Default, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct ConvertedType(i32);

impl ConvertedType {
    pub const UTF8: i32 = 0;
    pub const MAP: i32 = 1;
    pub const MAP_KEY_VALUE: i32 = 2;
    pub const LIST: i32 = 3;
    pub const ENUM: i32 = 4;
    pub const DECIMAL: i32 = 5;
    pub const DATE: i32 = 6;
    pub const TIME_MILLIS: i32 = 7;
    pub const TIME_MICROS: i32 = 8;
    pub const TIMESTAMP_MILLIS: i32 = 9;
    pub const TIMESTAMP_MICROS: i32 = 10;
    pub const UINT_8: i32 = 11;
    pub const UINT_16: i32 = 12;
    pub const UINT_32: i32 = 13;
    pub const UINT_64: i32 = 14;
    pub const INT_8: i32 = 15;
    pub const INT_16: i32 = 16;
    pub const INT_32: i32 = 17;
    pub const INT_64: i32 = 18;
    pub const JSON: i32 = 19;
    pub const BSON: i32 = 20;
    pub const INTERVAL: i32 = 21;

    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for ConvertedType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl <'i> CompactThriftProtocol<'i> for ConvertedType {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}


#[derive(Default, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct FieldRepetitionType(i32);

impl FieldRepetitionType {
    pub const REQUIRED: i32 = 0;
    pub const OPTIONAL: i32 = 1;
    pub const REPEATED: i32 = 2;

    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for FieldRepetitionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl <'i> CompactThriftProtocol<'i> for FieldRepetitionType {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}

#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct SizeStatistics<'i> {
    pub unencoded_byte_array_data_bytes: Option<i64>,
    pub repetition_level_histogram: Option<Vec<i64>>,
    pub definition_level_histogram: Option<Vec<i64>>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for SizeStatistics<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
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
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for Statistics<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct StringType<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for StringType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct UUIDType<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for UUIDType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MapType<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for MapType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ListType<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for ListType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct EnumType<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for EnumType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DateType<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for DateType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Float16Type<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for Float16Type<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct NullType<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for NullType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DecimalType<'i> {
    pub scale: i32,
    pub precision: i32,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for DecimalType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut scale_set_: bool = false;
        let mut precision_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MilliSeconds<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for MilliSeconds<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MicroSeconds<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for MicroSeconds<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct NanoSeconds<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for NanoSeconds<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum TimeUnit<'i> {
    MILLIS(MilliSeconds<'i>),
    MICROS(MicroSeconds<'i>),
    NANOS(NanoSeconds<'i>),
}
impl Default for TimeUnit<'_> {
    fn default() -> Self {
        Self::MILLIS(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for TimeUnit<'i> {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let field_type = input.read_byte()?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        let field_delta = (field_type & 0xF0) >> 4;
        let field_id = if field_delta != 0 {
            field_delta as i16
        } else {
            input.read_i16()?
        };

        match field_id {
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

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::MILLIS(inner) => inner.write_field(output, 1, &mut last_field_id),
            Self::MICROS(inner) => inner.write_field(output, 2, &mut last_field_id),
            Self::NANOS(inner) => inner.write_field(output, 3, &mut last_field_id),
        }
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct TimestampType<'i> {
    pub isAdjustedToUTC: bool,
    pub unit: TimeUnit<'i>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for TimestampType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut isAdjustedToUTC_set_: bool = false;
        let mut unit_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct TimeType<'i> {
    pub isAdjustedToUTC: bool,
    pub unit: TimeUnit<'i>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for TimeType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut isAdjustedToUTC_set_: bool = false;
        let mut unit_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IntType<'i> {
    pub bitWidth: i8,
    pub isSigned: bool,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for IntType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut bitWidth_set_: bool = false;
        let mut isSigned_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct JsonType<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for JsonType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct BsonType<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for BsonType<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum LogicalType<'i> {
    STRING(StringType<'i>),
    MAP(MapType<'i>),
    LIST(ListType<'i>),
    ENUM(EnumType<'i>),
    DECIMAL(DecimalType<'i>),
    DATE(DateType<'i>),
    TIME(TimeType<'i>),
    TIMESTAMP(TimestampType<'i>),
    INTEGER(IntType<'i>),
    UNKNOWN(NullType<'i>),
    JSON(JsonType<'i>),
    BSON(BsonType<'i>),
    UUID(UUIDType<'i>),
    FLOAT16(Float16Type<'i>),
}
impl Default for LogicalType<'_> {
    fn default() -> Self {
        Self::STRING(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for LogicalType<'i> {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let field_type = input.read_byte()?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        let field_delta = (field_type & 0xF0) >> 4;
        let field_id = if field_delta != 0 {
            field_delta as i16
        } else {
            input.read_i16()?
        };

        match field_id {
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

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::STRING(inner) => inner.write_field(output, 1, &mut last_field_id),
            Self::MAP(inner) => inner.write_field(output, 2, &mut last_field_id),
            Self::LIST(inner) => inner.write_field(output, 3, &mut last_field_id),
            Self::ENUM(inner) => inner.write_field(output, 4, &mut last_field_id),
            Self::DECIMAL(inner) => inner.write_field(output, 5, &mut last_field_id),
            Self::DATE(inner) => inner.write_field(output, 6, &mut last_field_id),
            Self::TIME(inner) => inner.write_field(output, 7, &mut last_field_id),
            Self::TIMESTAMP(inner) => inner.write_field(output, 8, &mut last_field_id),
            Self::INTEGER(inner) => inner.write_field(output, 10, &mut last_field_id),
            Self::UNKNOWN(inner) => inner.write_field(output, 11, &mut last_field_id),
            Self::JSON(inner) => inner.write_field(output, 12, &mut last_field_id),
            Self::BSON(inner) => inner.write_field(output, 13, &mut last_field_id),
            Self::UUID(inner) => inner.write_field(output, 14, &mut last_field_id),
            Self::FLOAT16(inner) => inner.write_field(output, 15, &mut last_field_id),
        }
    }
}
#[derive(Default, Clone, Debug)]
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
    pub logicalType: Option<LogicalType<'i>>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for SchemaElement<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut name_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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

#[derive(Default, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct Encoding(i32);

impl Encoding {
    pub const PLAIN: i32 = 0;
    pub const PLAIN_DICTIONARY: i32 = 2;
    pub const RLE: i32 = 3;
    pub const BIT_PACKED: i32 = 4;
    pub const DELTA_BINARY_PACKED: i32 = 5;
    pub const DELTA_LENGTH_BYTE_ARRAY: i32 = 6;
    pub const DELTA_BYTE_ARRAY: i32 = 7;
    pub const RLE_DICTIONARY: i32 = 8;
    pub const BYTE_STREAM_SPLIT: i32 = 9;

    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for Encoding {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl <'i> CompactThriftProtocol<'i> for Encoding {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}


#[derive(Default, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct CompressionCodec(i32);

impl CompressionCodec {
    pub const UNCOMPRESSED: i32 = 0;
    pub const SNAPPY: i32 = 1;
    pub const GZIP: i32 = 2;
    pub const LZO: i32 = 3;
    pub const BROTLI: i32 = 4;
    pub const LZ4: i32 = 5;
    pub const ZSTD: i32 = 6;
    pub const LZ4_RAW: i32 = 7;

    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for CompressionCodec {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl <'i> CompactThriftProtocol<'i> for CompressionCodec {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}


#[derive(Default, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct PageType(i32);

impl PageType {
    pub const DATA_PAGE: i32 = 0;
    pub const INDEX_PAGE: i32 = 1;
    pub const DICTIONARY_PAGE: i32 = 2;
    pub const DATA_PAGE_V2: i32 = 3;

    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for PageType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl <'i> CompactThriftProtocol<'i> for PageType {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}


#[derive(Default, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct BoundaryOrder(i32);

impl BoundaryOrder {
    pub const UNORDERED: i32 = 0;
    pub const ASCENDING: i32 = 1;
    pub const DESCENDING: i32 = 2;

    pub fn value(&self) -> i32 {
        self.0
    }
}
impl From<i32> for BoundaryOrder {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl <'i> CompactThriftProtocol<'i> for BoundaryOrder {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        self.0 = input.read_i32()?;
        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(self.0)
    }
}

#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DataPageHeader<'i> {
    pub num_values: i32,
    pub encoding: Encoding,
    pub definition_level_encoding: Encoding,
    pub repetition_level_encoding: Encoding,
    pub statistics: Option<Statistics<'i>>,
    __phantom_lifetime: PhantomData<&'i ()>,
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
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IndexPageHeader<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for IndexPageHeader<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DictionaryPageHeader<'i> {
    pub num_values: i32,
    pub encoding: Encoding,
    pub is_sorted: Option<bool>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for DictionaryPageHeader<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut num_values_set_: bool = false;
        let mut encoding_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
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
    __phantom_lifetime: PhantomData<&'i ()>,
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
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct SplitBlockAlgorithm<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for SplitBlockAlgorithm<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum BloomFilterAlgorithm<'i> {
    BLOCK(SplitBlockAlgorithm<'i>),
}
impl Default for BloomFilterAlgorithm<'_> {
    fn default() -> Self {
        Self::BLOCK(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for BloomFilterAlgorithm<'i> {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let field_type = input.read_byte()?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        let field_delta = (field_type & 0xF0) >> 4;
        let field_id = if field_delta != 0 {
            field_delta as i16
        } else {
            input.read_i16()?
        };

        match field_id {
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

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::BLOCK(inner) => inner.write_field(output, 1, &mut last_field_id),
        }
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct XxHash<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for XxHash<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum BloomFilterHash<'i> {
    XXHASH(XxHash<'i>),
}
impl Default for BloomFilterHash<'_> {
    fn default() -> Self {
        Self::XXHASH(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for BloomFilterHash<'i> {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let field_type = input.read_byte()?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        let field_delta = (field_type & 0xF0) >> 4;
        let field_id = if field_delta != 0 {
            field_delta as i16
        } else {
            input.read_i16()?
        };

        match field_id {
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

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::XXHASH(inner) => inner.write_field(output, 1, &mut last_field_id),
        }
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Uncompressed<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for Uncompressed<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum BloomFilterCompression<'i> {
    UNCOMPRESSED(Uncompressed<'i>),
}
impl Default for BloomFilterCompression<'_> {
    fn default() -> Self {
        Self::UNCOMPRESSED(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for BloomFilterCompression<'i> {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let field_type = input.read_byte()?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        let field_delta = (field_type & 0xF0) >> 4;
        let field_id = if field_delta != 0 {
            field_delta as i16
        } else {
            input.read_i16()?
        };

        match field_id {
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

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::UNCOMPRESSED(inner) => inner.write_field(output, 1, &mut last_field_id),
        }
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct BloomFilterHeader<'i> {
    pub numBytes: i32,
    pub algorithm: BloomFilterAlgorithm<'i>,
    pub hash: BloomFilterHash<'i>,
    pub compression: BloomFilterCompression<'i>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for BloomFilterHeader<'i> {
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
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct PageHeader<'i> {
    pub r#type: PageType,
    pub uncompressed_page_size: i32,
    pub compressed_page_size: i32,
    pub crc: Option<i32>,
    pub data_page_header: Option<DataPageHeader<'i>>,
    pub index_page_header: Option<IndexPageHeader<'i>>,
    pub dictionary_page_header: Option<DictionaryPageHeader<'i>>,
    pub data_page_header_v2: Option<DataPageHeaderV2<'i>>,
    __phantom_lifetime: PhantomData<&'i ()>,
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
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct KeyValue<'i> {
    pub key: Cow<'i, str>,
    pub value: Option<Cow<'i, str>>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for KeyValue<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut key_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct SortingColumn<'i> {
    pub column_idx: i32,
    pub descending: bool,
    pub nulls_first: bool,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for SortingColumn<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut column_idx_set_: bool = false;
        let mut descending_set_: bool = false;
        let mut nulls_first_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct PageEncodingStats<'i> {
    pub page_type: PageType,
    pub encoding: Encoding,
    pub count: i32,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for PageEncodingStats<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut page_type_set_: bool = false;
        let mut encoding_set_: bool = false;
        let mut count_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
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
    pub encoding_stats: Option<Vec<PageEncodingStats<'i>>>,
    pub bloom_filter_offset: Option<i64>,
    pub bloom_filter_length: Option<i32>,
    pub size_statistics: Option<SizeStatistics<'i>>,
    __phantom_lifetime: PhantomData<&'i ()>,
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
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct EncryptionWithFooterKey<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for EncryptionWithFooterKey<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct EncryptionWithColumnKey<'i> {
    pub path_in_schema: Vec<Cow<'i, str>>,
    pub key_metadata: Option<Cow<'i, [u8]>>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for EncryptionWithColumnKey<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut path_in_schema_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ColumnCryptoMetaData<'i> {
    ENCRYPTION_WITH_FOOTER_KEY(EncryptionWithFooterKey<'i>),
    ENCRYPTION_WITH_COLUMN_KEY(EncryptionWithColumnKey<'i>),
}
impl Default for ColumnCryptoMetaData<'_> {
    fn default() -> Self {
        Self::ENCRYPTION_WITH_FOOTER_KEY(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for ColumnCryptoMetaData<'i> {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let field_type = input.read_byte()?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        let field_delta = (field_type & 0xF0) >> 4;
        let field_id = if field_delta != 0 {
            field_delta as i16
        } else {
            input.read_i16()?
        };

        match field_id {
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

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::ENCRYPTION_WITH_FOOTER_KEY(inner) => inner.write_field(output, 1, &mut last_field_id),
            Self::ENCRYPTION_WITH_COLUMN_KEY(inner) => inner.write_field(output, 2, &mut last_field_id),
        }
    }
}
#[derive(Default, Clone, Debug)]
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
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for ColumnChunk<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut file_offset_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct RowGroup<'i> {
    pub columns: Vec<ColumnChunk<'i>>,
    pub total_byte_size: i64,
    pub num_rows: i64,
    pub sorting_columns: Option<Vec<SortingColumn<'i>>>,
    pub file_offset: Option<i64>,
    pub total_compressed_size: Option<i64>,
    pub ordinal: Option<i16>,
    __phantom_lifetime: PhantomData<&'i ()>,
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
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct TypeDefinedOrder<'i> {
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for TypeDefinedOrder<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ColumnOrder<'i> {
    TYPE_ORDER(TypeDefinedOrder<'i>),
}
impl Default for ColumnOrder<'_> {
    fn default() -> Self {
        Self::TYPE_ORDER(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for ColumnOrder<'i> {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let field_type = input.read_byte()?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        let field_delta = (field_type & 0xF0) >> 4;
        let field_id = if field_delta != 0 {
            field_delta as i16
        } else {
            input.read_i16()?
        };

        match field_id {
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

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::TYPE_ORDER(inner) => inner.write_field(output, 1, &mut last_field_id),
        }
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct PageLocation<'i> {
    pub offset: i64,
    pub compressed_page_size: i32,
    pub first_row_index: i64,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for PageLocation<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut offset_set_: bool = false;
        let mut compressed_page_size_set_: bool = false;
        let mut first_row_index_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct OffsetIndex<'i> {
    pub page_locations: Vec<PageLocation<'i>>,
    pub unencoded_byte_array_data_bytes: Option<Vec<i64>>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for OffsetIndex<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut page_locations_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
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
    __phantom_lifetime: PhantomData<&'i ()>,
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
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct AesGcmV1<'i> {
    pub aad_prefix: Option<Cow<'i, [u8]>>,
    pub aad_file_unique: Option<Cow<'i, [u8]>>,
    pub supply_aad_prefix: Option<bool>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for AesGcmV1<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct AesGcmCtrV1<'i> {
    pub aad_prefix: Option<Cow<'i, [u8]>>,
    pub aad_file_unique: Option<Cow<'i, [u8]>>,
    pub supply_aad_prefix: Option<bool>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for AesGcmCtrV1<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum EncryptionAlgorithm<'i> {
    AES_GCM_V1(AesGcmV1<'i>),
    AES_GCM_CTR_V1(AesGcmCtrV1<'i>),
}
impl Default for EncryptionAlgorithm<'_> {
    fn default() -> Self {
        Self::AES_GCM_V1(Default::default())
    }
}
impl <'i> CompactThriftProtocol<'i> for EncryptionAlgorithm<'i> {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let field_type = input.read_byte()?;

        if field_type == 0 {
            return Err(ThriftError::InvalidType);
        }

        let field_delta = (field_type & 0xF0) >> 4;
        let field_id = if field_delta != 0 {
            field_delta as i16
        } else {
            input.read_i16()?
        };

        match field_id {
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

        Ok(())
    }

    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let mut last_field_id = 0_i16;
        match self {
            Self::AES_GCM_V1(inner) => inner.write_field(output, 1, &mut last_field_id),
            Self::AES_GCM_CTR_V1(inner) => inner.write_field(output, 2, &mut last_field_id),
        }
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct FileMetaData<'i> {
    pub version: i32,
    pub schema: Vec<SchemaElement<'i>>,
    pub num_rows: i64,
    pub row_groups: Vec<RowGroup<'i>>,
    pub key_value_metadata: Option<Vec<KeyValue<'i>>>,
    pub created_by: Option<Cow<'i, str>>,
    pub column_orders: Option<Vec<ColumnOrder<'i>>>,
    pub encryption_algorithm: Option<EncryptionAlgorithm<'i>>,
    pub footer_signing_key_metadata: Option<Cow<'i, [u8]>>,
    __phantom_lifetime: PhantomData<&'i ()>,
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
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct FileCryptoMetaData<'i> {
    pub encryption_algorithm: EncryptionAlgorithm<'i>,
    pub key_metadata: Option<Cow<'i, [u8]>>,
    __phantom_lifetime: PhantomData<&'i ()>,
}

impl <'i> CompactThriftProtocol<'i> for FileCryptoMetaData<'i> {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let mut encryption_algorithm_set_: bool = false;
        let mut last_field_id = 0_i16;
        loop {
            let field_header = input.read_byte()?;

            if field_header == 0 {
                break;
            }

            let field_type = field_header & 0x0F;
            let field_delta = field_header >> 4;
            if field_delta != 0 {
                last_field_id += field_delta as i16;
            } else {
                last_field_id = input.read_i16()?;
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

