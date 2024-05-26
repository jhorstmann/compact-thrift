#[allow(non_snake_case)]
// Generated on 2024-05-26T12:36:27.283941578Z
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
impl CompactThriftProtocol for Type {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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
impl CompactThriftProtocol for ConvertedType {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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
impl CompactThriftProtocol for FieldRepetitionType {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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
pub struct SizeStatistics {
    pub unencoded_byte_array_data_bytes: Option<i64>,
    pub repetition_level_histogram: Option<Vec<i64>>,
    pub definition_level_histogram: Option<Vec<i64>>,
}

impl CompactThriftProtocol for SizeStatistics {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("SizeStatistics::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Statistics {
    pub max: Option<Vec<u8>>,
    pub min: Option<Vec<u8>>,
    pub null_count: Option<i64>,
    pub distinct_count: Option<i64>,
    pub max_value: Option<Vec<u8>>,
    pub min_value: Option<Vec<u8>>,
    pub is_max_value_exact: Option<bool>,
    pub is_min_value_exact: Option<bool>,
}

impl CompactThriftProtocol for Statistics {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("Statistics::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct StringType {
}

impl CompactThriftProtocol for StringType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("StringType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct UUIDType {
}

impl CompactThriftProtocol for UUIDType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("UUIDType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MapType {
}

impl CompactThriftProtocol for MapType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("MapType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ListType {
}

impl CompactThriftProtocol for ListType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("ListType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct EnumType {
}

impl CompactThriftProtocol for EnumType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("EnumType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DateType {
}

impl CompactThriftProtocol for DateType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("DateType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Float16Type {
}

impl CompactThriftProtocol for Float16Type {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("Float16Type::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct NullType {
}

impl CompactThriftProtocol for NullType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("NullType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DecimalType {
    pub scale: i32,
    pub precision: i32,
}

impl CompactThriftProtocol for DecimalType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("DecimalType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MilliSeconds {
}

impl CompactThriftProtocol for MilliSeconds {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("MilliSeconds::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MicroSeconds {
}

impl CompactThriftProtocol for MicroSeconds {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("MicroSeconds::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct NanoSeconds {
}

impl CompactThriftProtocol for NanoSeconds {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("NanoSeconds::write")
    }
}
#[derive(Clone, Debug)]
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
impl CompactThriftProtocol for TimeUnit {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("TimeUnit::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct TimestampType {
    pub isAdjustedToUTC: bool,
    pub unit: TimeUnit,
}

impl CompactThriftProtocol for TimestampType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("TimestampType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct TimeType {
    pub isAdjustedToUTC: bool,
    pub unit: TimeUnit,
}

impl CompactThriftProtocol for TimeType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("TimeType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IntType {
    pub bitWidth: i8,
    pub isSigned: bool,
}

impl CompactThriftProtocol for IntType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("IntType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct JsonType {
}

impl CompactThriftProtocol for JsonType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("JsonType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct BsonType {
}

impl CompactThriftProtocol for BsonType {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("BsonType::write")
    }
}
#[derive(Clone, Debug)]
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
impl CompactThriftProtocol for LogicalType {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("LogicalType::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct SchemaElement {
    pub r#type: Option<Type>,
    pub type_length: Option<i32>,
    pub repetition_type: Option<FieldRepetitionType>,
    pub name: String,
    pub num_children: Option<i32>,
    pub converted_type: Option<ConvertedType>,
    pub scale: Option<i32>,
    pub precision: Option<i32>,
    pub field_id: Option<i32>,
    pub logicalType: Option<LogicalType>,
}

impl CompactThriftProtocol for SchemaElement {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("SchemaElement::write")
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
impl CompactThriftProtocol for Encoding {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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
impl CompactThriftProtocol for CompressionCodec {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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
impl CompactThriftProtocol for PageType {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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
impl CompactThriftProtocol for BoundaryOrder {
    const FIELD_TYPE: u8 = 5; // i32
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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
pub struct DataPageHeader {
    pub num_values: i32,
    pub encoding: Encoding,
    pub definition_level_encoding: Encoding,
    pub repetition_level_encoding: Encoding,
    pub statistics: Option<Statistics>,
}

impl CompactThriftProtocol for DataPageHeader {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("DataPageHeader::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct IndexPageHeader {
}

impl CompactThriftProtocol for IndexPageHeader {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("IndexPageHeader::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DictionaryPageHeader {
    pub num_values: i32,
    pub encoding: Encoding,
    pub is_sorted: Option<bool>,
}

impl CompactThriftProtocol for DictionaryPageHeader {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("DictionaryPageHeader::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DataPageHeaderV2 {
    pub num_values: i32,
    pub num_nulls: i32,
    pub num_rows: i32,
    pub encoding: Encoding,
    pub definition_levels_byte_length: i32,
    pub repetition_levels_byte_length: i32,
    pub is_compressed: Option<bool>,
    pub statistics: Option<Statistics>,
}

impl CompactThriftProtocol for DataPageHeaderV2 {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("DataPageHeaderV2::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct SplitBlockAlgorithm {
}

impl CompactThriftProtocol for SplitBlockAlgorithm {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("SplitBlockAlgorithm::write")
    }
}
#[derive(Clone, Debug)]
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
impl CompactThriftProtocol for BloomFilterAlgorithm {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("BloomFilterAlgorithm::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct XxHash {
}

impl CompactThriftProtocol for XxHash {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("XxHash::write")
    }
}
#[derive(Clone, Debug)]
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
impl CompactThriftProtocol for BloomFilterHash {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("BloomFilterHash::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Uncompressed {
}

impl CompactThriftProtocol for Uncompressed {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("Uncompressed::write")
    }
}
#[derive(Clone, Debug)]
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
impl CompactThriftProtocol for BloomFilterCompression {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("BloomFilterCompression::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct BloomFilterHeader {
    pub numBytes: i32,
    pub algorithm: BloomFilterAlgorithm,
    pub hash: BloomFilterHash,
    pub compression: BloomFilterCompression,
}

impl CompactThriftProtocol for BloomFilterHeader {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("BloomFilterHeader::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct PageHeader {
    pub r#type: PageType,
    pub uncompressed_page_size: i32,
    pub compressed_page_size: i32,
    pub crc: Option<i32>,
    pub data_page_header: Option<DataPageHeader>,
    pub index_page_header: Option<IndexPageHeader>,
    pub dictionary_page_header: Option<DictionaryPageHeader>,
    pub data_page_header_v2: Option<DataPageHeaderV2>,
}

impl CompactThriftProtocol for PageHeader {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("PageHeader::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct KeyValue {
    pub key: String,
    pub value: Option<String>,
}

impl CompactThriftProtocol for KeyValue {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("KeyValue::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct SortingColumn {
    pub column_idx: i32,
    pub descending: bool,
    pub nulls_first: bool,
}

impl CompactThriftProtocol for SortingColumn {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("SortingColumn::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct PageEncodingStats {
    pub page_type: PageType,
    pub encoding: Encoding,
    pub count: i32,
}

impl CompactThriftProtocol for PageEncodingStats {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("PageEncodingStats::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ColumnMetaData {
    pub r#type: Type,
    pub encodings: Vec<Encoding>,
    pub path_in_schema: Vec<String>,
    pub codec: CompressionCodec,
    pub num_values: i64,
    pub total_uncompressed_size: i64,
    pub total_compressed_size: i64,
    pub key_value_metadata: Option<Vec<KeyValue>>,
    pub data_page_offset: i64,
    pub index_page_offset: Option<i64>,
    pub dictionary_page_offset: Option<i64>,
    pub statistics: Option<Statistics>,
    pub encoding_stats: Option<Vec<PageEncodingStats>>,
    pub bloom_filter_offset: Option<i64>,
    pub bloom_filter_length: Option<i32>,
    pub size_statistics: Option<SizeStatistics>,
}

impl CompactThriftProtocol for ColumnMetaData {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("ColumnMetaData::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct EncryptionWithFooterKey {
}

impl CompactThriftProtocol for EncryptionWithFooterKey {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("EncryptionWithFooterKey::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct EncryptionWithColumnKey {
    pub path_in_schema: Vec<String>,
    pub key_metadata: Option<Vec<u8>>,
}

impl CompactThriftProtocol for EncryptionWithColumnKey {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("EncryptionWithColumnKey::write")
    }
}
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum ColumnCryptoMetaData {
    ENCRYPTION_WITH_FOOTER_KEY(EncryptionWithFooterKey),
    ENCRYPTION_WITH_COLUMN_KEY(EncryptionWithColumnKey),
}
impl Default for ColumnCryptoMetaData {
    fn default() -> Self {
        Self::ENCRYPTION_WITH_FOOTER_KEY(Default::default())
    }
}
impl CompactThriftProtocol for ColumnCryptoMetaData {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("ColumnCryptoMetaData::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ColumnChunk {
    pub file_path: Option<String>,
    pub file_offset: i64,
    pub meta_data: Option<ColumnMetaData>,
    pub offset_index_offset: Option<i64>,
    pub offset_index_length: Option<i32>,
    pub column_index_offset: Option<i64>,
    pub column_index_length: Option<i32>,
    pub crypto_metadata: Option<ColumnCryptoMetaData>,
    pub encrypted_column_metadata: Option<Vec<u8>>,
}

impl CompactThriftProtocol for ColumnChunk {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("ColumnChunk::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct RowGroup {
    pub columns: Vec<ColumnChunk>,
    pub total_byte_size: i64,
    pub num_rows: i64,
    pub sorting_columns: Option<Vec<SortingColumn>>,
    pub file_offset: Option<i64>,
    pub total_compressed_size: Option<i64>,
    pub ordinal: Option<i16>,
}

impl CompactThriftProtocol for RowGroup {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("RowGroup::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct TypeDefinedOrder {
}

impl CompactThriftProtocol for TypeDefinedOrder {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("TypeDefinedOrder::write")
    }
}
#[derive(Clone, Debug)]
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
impl CompactThriftProtocol for ColumnOrder {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("ColumnOrder::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct PageLocation {
    pub offset: i64,
    pub compressed_page_size: i32,
    pub first_row_index: i64,
}

impl CompactThriftProtocol for PageLocation {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("PageLocation::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct OffsetIndex {
    pub page_locations: Vec<PageLocation>,
    pub unencoded_byte_array_data_bytes: Option<Vec<i64>>,
}

impl CompactThriftProtocol for OffsetIndex {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("OffsetIndex::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct ColumnIndex {
    pub null_pages: Vec<bool>,
    pub min_values: Vec<Vec<u8>>,
    pub max_values: Vec<Vec<u8>>,
    pub boundary_order: BoundaryOrder,
    pub null_counts: Option<Vec<i64>>,
    pub repetition_level_histograms: Option<Vec<i64>>,
    pub definition_level_histograms: Option<Vec<i64>>,
}

impl CompactThriftProtocol for ColumnIndex {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("ColumnIndex::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct AesGcmV1 {
    pub aad_prefix: Option<Vec<u8>>,
    pub aad_file_unique: Option<Vec<u8>>,
    pub supply_aad_prefix: Option<bool>,
}

impl CompactThriftProtocol for AesGcmV1 {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("AesGcmV1::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct AesGcmCtrV1 {
    pub aad_prefix: Option<Vec<u8>>,
    pub aad_file_unique: Option<Vec<u8>>,
    pub supply_aad_prefix: Option<bool>,
}

impl CompactThriftProtocol for AesGcmCtrV1 {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("AesGcmCtrV1::write")
    }
}
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum EncryptionAlgorithm {
    AES_GCM_V1(AesGcmV1),
    AES_GCM_CTR_V1(AesGcmCtrV1),
}
impl Default for EncryptionAlgorithm {
    fn default() -> Self {
        Self::AES_GCM_V1(Default::default())
    }
}
impl CompactThriftProtocol for EncryptionAlgorithm {
    const FIELD_TYPE: u8 = 12;
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("EncryptionAlgorithm::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct FileMetaData {
    pub version: i32,
    pub schema: Vec<SchemaElement>,
    pub num_rows: i64,
    pub row_groups: Vec<RowGroup>,
    pub key_value_metadata: Option<Vec<KeyValue>>,
    pub created_by: Option<String>,
    pub column_orders: Option<Vec<ColumnOrder>>,
    pub encryption_algorithm: Option<EncryptionAlgorithm>,
    pub footer_signing_key_metadata: Option<Vec<u8>>,
}

impl CompactThriftProtocol for FileMetaData {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("FileMetaData::write")
    }
}
#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct FileCryptoMetaData {
    pub encryption_algorithm: EncryptionAlgorithm,
    pub key_metadata: Option<Vec<u8>>,
}

impl CompactThriftProtocol for FileCryptoMetaData {
    const FIELD_TYPE: u8 = 12;

    #[inline(never)]
    #[allow(non_snake_case)]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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

    fn write<T: CompactThriftOutput>(&self, _output: &mut T) -> Result<(), ThriftError> {
        unimplemented!("FileCryptoMetaData::write")
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_compile() {

    }
}

