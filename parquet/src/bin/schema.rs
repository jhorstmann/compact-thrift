use compact_thrift_rs::{CompactThriftInputSlice, CompactThriftProtocol};
use parquet_format::format::{
    ConvertedType, DecimalType, FieldRepetitionType, FileMetaData, IntType, LogicalType,
    MicroSeconds, MilliSeconds, NanoSeconds, SchemaElement, TimeType, TimeUnit, TimestampType,
    Type,
};
use parquet_format::{get_metadata_chunk, ParquetError};
use std::fmt::{Display, Formatter};
use std::fs::File;

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum PhysicalType {
    BOOLEAN,
    INT32,
    INT64,
    INT96,
    FLOAT,
    DOUBLE,
    BYTE_ARRAY,
    FIXED_LEN_BYTE_ARRAY(u32),
}

impl PhysicalType {
    fn try_from_schema_element(
        element_type: Type,
        type_length: Option<i32>,
    ) -> Result<PhysicalType, ParquetError> {
        match element_type {
            Type::BOOLEAN => Ok(PhysicalType::BOOLEAN),
            Type::BYTE_ARRAY => Ok(PhysicalType::BYTE_ARRAY),
            Type::INT32 => Ok(PhysicalType::INT32),
            Type::INT64 => Ok(PhysicalType::INT64),
            Type::INT96 => Ok(PhysicalType::INT96),
            Type::FLOAT => Ok(PhysicalType::FLOAT),
            Type::DOUBLE => Ok(PhysicalType::DOUBLE),
            Type::FIXED_LEN_BYTE_ARRAY => {
                let len = type_length.ok_or(ParquetError::Schema(
                    "Missing length for FIXED_LEN_BYTE_ARRAY type",
                ))?;
                if len <= 0 {
                    return Err(ParquetError::Schema(
                        "Invalid length for FIXED_LEN_BYTE_ARRAY type",
                    ));
                }
                Ok(PhysicalType::FIXED_LEN_BYTE_ARRAY(len as u32))
            }
            Type(id) => Err(ParquetError::UnknownType(id)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SchemaTimeUnit {
    Millis,
    Micros,
    Nanos,
}

impl From<TimeUnit> for SchemaTimeUnit {
    fn from(time_unit: TimeUnit) -> Self {
        match time_unit {
            TimeUnit::MILLIS(_) => SchemaTimeUnit::Millis,
            TimeUnit::MICROS(_) => SchemaTimeUnit::Micros,
            TimeUnit::NANOS(_) => SchemaTimeUnit::Nanos,
        }
    }
}

impl From<SchemaTimeUnit> for TimeUnit {
    fn from(value: SchemaTimeUnit) -> Self {
        match value {
            SchemaTimeUnit::Millis => TimeUnit::MILLIS(MilliSeconds {}),
            SchemaTimeUnit::Micros => TimeUnit::MICROS(MicroSeconds {}),
            SchemaTimeUnit::Nanos => TimeUnit::NANOS(NanoSeconds {}),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeOptions {
    is_adjusted_to_utc: bool,
    time_unit: SchemaTimeUnit,
}

impl From<&TimeType> for TimeOptions {
    fn from(time_type: &TimeType) -> Self {
        Self {
            is_adjusted_to_utc: time_type.isAdjustedToUTC,
            time_unit: time_type.unit.clone().into(),
        }
    }
}

impl From<&TimestampType> for TimeOptions {
    fn from(timestamp_type: &TimestampType) -> Self {
        Self {
            is_adjusted_to_utc: timestamp_type.isAdjustedToUTC,
            time_unit: timestamp_type.unit.clone().into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecimalOptions {
    precision: u8,
    scale: i8,
}

impl DecimalOptions {
    pub fn try_new(precision: i32, scale: i32) -> Result<Self, ParquetError> {
        if precision <= 0 || precision > 127 {
            return Err(ParquetError::Schema("Invalid precision for Decimal type"));
        }
        if scale < -128 || scale > precision {
            return Err(ParquetError::Schema("Invalid scale for Decimal type"));
        }
        Ok(Self {
            precision: precision as u8,
            scale: scale as i8,
        })
    }
}

impl TryFrom<&DecimalType> for DecimalOptions {
    type Error = ParquetError;

    fn try_from(decimal_type: &DecimalType) -> Result<Self, Self::Error> {
        Self::try_new(decimal_type.precision, decimal_type.scale)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntegerOptions {
    bitwidth: u8,
    is_signed: bool,
}

impl TryFrom<&IntType> for IntegerOptions {
    type Error = ParquetError;

    fn try_from(int_type: &IntType) -> Result<Self, Self::Error> {
        // according to thrift schema, "bitWidth must be 8, 16, 32, or 64"
        // but type_length in combination with converted type does not have such restriction
        if int_type.bitWidth <= 0 {
            return Err(ParquetError::Schema("Invalid bitwidth for Integer type"));
        }
        Ok(Self {
            bitwidth: int_type.bitWidth as u8,
            is_signed: int_type.isSigned,
        })
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PrimitiveLogicalType {
    String,
    Enum,
    Decimal(DecimalOptions),
    Date,
    Time(TimeOptions),
    Timestamp(TimeOptions),
    Interval,
    Integer(IntegerOptions),
    Unknown,
    Json,
    Bson,
    UUID,
    Float16,
}

impl PrimitiveLogicalType {
    pub fn try_from_converted_type(
        converted_type: &ConvertedType,
        precision: Option<i32>,
        scale: Option<i32>,
        _type_length: Option<i32>,
    ) -> Result<PrimitiveLogicalType, ParquetError> {
        match *converted_type {
            ConvertedType::UTF8 => Ok(PrimitiveLogicalType::String),
            ConvertedType::ENUM => Ok(PrimitiveLogicalType::Enum),
            ConvertedType::DATE => Ok(PrimitiveLogicalType::Date),
            ConvertedType::JSON => Ok(PrimitiveLogicalType::Json),
            ConvertedType::BSON => Ok(PrimitiveLogicalType::Bson),
            ConvertedType::DECIMAL => {
                let precision =
                    precision.ok_or(ParquetError::Schema("Missing precision for decimal type"))?;
                let scale = scale.ok_or(ParquetError::Schema("Missing scale for decimal type"))?;
                let options = DecimalOptions::try_from(&DecimalType { precision, scale })?;
                Ok(PrimitiveLogicalType::Decimal(options))
            }
            ConvertedType::TIME_MICROS => Ok(PrimitiveLogicalType::Time(TimeOptions {
                time_unit: SchemaTimeUnit::Micros,
                is_adjusted_to_utc: true,
            })),
            ConvertedType::TIME_MILLIS => Ok(PrimitiveLogicalType::Time(TimeOptions {
                time_unit: SchemaTimeUnit::Millis,
                is_adjusted_to_utc: true,
            })),
            ConvertedType::TIMESTAMP_MICROS => Ok(PrimitiveLogicalType::Timestamp(TimeOptions {
                time_unit: SchemaTimeUnit::Micros,
                is_adjusted_to_utc: true,
            })),
            ConvertedType::TIMESTAMP_MILLIS => Ok(PrimitiveLogicalType::Timestamp(TimeOptions {
                time_unit: SchemaTimeUnit::Millis,
                is_adjusted_to_utc: true,
            })),
            ConvertedType::INTERVAL => Ok(PrimitiveLogicalType::Interval),
            // TODO: consider SchemaElement::type_len for bitwidth
            ConvertedType::INT_8 => Ok(PrimitiveLogicalType::Integer(IntegerOptions {
                bitwidth: 8,
                is_signed: true,
            })),
            ConvertedType::INT_16 => Ok(PrimitiveLogicalType::Integer(IntegerOptions {
                bitwidth: 16,
                is_signed: true,
            })),
            ConvertedType::INT_32 => Ok(PrimitiveLogicalType::Integer(IntegerOptions {
                bitwidth: 32,
                is_signed: true,
            })),
            ConvertedType::INT_64 => Ok(PrimitiveLogicalType::Integer(IntegerOptions {
                bitwidth: 64,
                is_signed: true,
            })),
            ConvertedType::UINT_8 => Ok(PrimitiveLogicalType::Integer(IntegerOptions {
                bitwidth: 8,
                is_signed: false,
            })),
            ConvertedType::UINT_16 => Ok(PrimitiveLogicalType::Integer(IntegerOptions {
                bitwidth: 16,
                is_signed: false,
            })),
            ConvertedType::UINT_32 => Ok(PrimitiveLogicalType::Integer(IntegerOptions {
                bitwidth: 32,
                is_signed: false,
            })),
            ConvertedType::UINT_64 => Ok(PrimitiveLogicalType::Integer(IntegerOptions {
                bitwidth: 64,
                is_signed: false,
            })),
            ConvertedType::MAP | ConvertedType::LIST => Err(ParquetError::Schema(
                "Unsupported map or list converted type for primitive",
            )),
            ConvertedType(id) => Err(ParquetError::UnknownType(id)),
        }
    }

    pub fn try_from_logical_type(logical_type: &LogicalType) -> Result<Self, ParquetError> {
        Self::try_from(logical_type)
    }

    pub fn precision(&self) -> Option<i32> {
        match self {
            PrimitiveLogicalType::Decimal(DecimalOptions { precision, .. }) => {
                Some(*precision as _)
            }
            _ => None,
        }
    }

    pub fn scale(&self) -> Option<i32> {
        match self {
            PrimitiveLogicalType::Decimal(DecimalOptions { scale, .. }) => Some(*scale as _),
            _ => None,
        }
    }
}

impl TryFrom<&LogicalType> for PrimitiveLogicalType {
    type Error = ParquetError;

    fn try_from(logical_type: &LogicalType) -> Result<Self, Self::Error> {
        match logical_type {
            LogicalType::STRING(_) => Ok(PrimitiveLogicalType::String),
            LogicalType::ENUM(_) => Ok(PrimitiveLogicalType::Enum),
            LogicalType::DECIMAL(decimal_type) => Ok(PrimitiveLogicalType::Decimal(
                DecimalOptions::try_from(decimal_type)?,
            )),
            LogicalType::DATE(_) => Ok(PrimitiveLogicalType::Date),
            LogicalType::TIME(time_type) => Ok(PrimitiveLogicalType::Time(time_type.into())),
            LogicalType::TIMESTAMP(timestamp_type) => {
                Ok(PrimitiveLogicalType::Timestamp(timestamp_type.into()))
            }
            LogicalType::INTEGER(int_type) => Ok(PrimitiveLogicalType::Integer(
                IntegerOptions::try_from(int_type)?,
            )),
            LogicalType::UNKNOWN(_) => Ok(PrimitiveLogicalType::Unknown),
            LogicalType::JSON(_) => Ok(PrimitiveLogicalType::Json),
            LogicalType::BSON(_) => Ok(PrimitiveLogicalType::Bson),
            LogicalType::UUID(_) => Ok(PrimitiveLogicalType::UUID),
            LogicalType::FLOAT16(_) => Ok(PrimitiveLogicalType::Float16),
            LogicalType::MAP(_) | LogicalType::LIST(_) => Err(ParquetError::Schema(
                "Unsupported map or list logical type for primitive",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GroupLogicalType {
    List,
    Map,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RepetitionType {
    Required,
    Optional,
    Repeated,
}

impl Display for RepetitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RepetitionType::Required => f.write_str("REQUIRED"),
            RepetitionType::Optional => f.write_str("OPTIONAL"),
            RepetitionType::Repeated => f.write_str("REPEATED"),
        }
    }
}

impl TryFrom<&FieldRepetitionType> for RepetitionType {
    type Error = ParquetError;

    fn try_from(repetition_type: &FieldRepetitionType) -> Result<Self, Self::Error> {
        match *repetition_type {
            FieldRepetitionType::REQUIRED => Ok(RepetitionType::Required),
            FieldRepetitionType::OPTIONAL => Ok(RepetitionType::Optional),
            FieldRepetitionType::REPEATED => Ok(RepetitionType::Repeated),
            FieldRepetitionType(id) => Err(ParquetError::UnknownType(id)),
        }
    }
}

/// An i32 without the alignment requirements, to reduce the size of CommonField.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldId([u8; 4]);

impl From<i32> for FieldId {
    fn from(value: i32) -> Self {
        Self(value.to_ne_bytes())
    }
}

impl From<FieldId> for i32 {
    fn from(value: FieldId) -> Self {
        i32::from_ne_bytes(value.0)
    }
}

#[derive(Debug)]
pub struct CommonField {
    pub name: String,
    pub field_id: Option<FieldId>,
    pub repetition: RepetitionType,
    /// the definition level at which a value is non-null
    pub def_level: u8,
    /// the repetition level at which a value continues a previous list item
    pub rep_level: u8,
}

#[derive(Debug)]
pub struct PrimitiveField {
    pub common: CommonField,
    pub physical_type: PhysicalType,
    pub logical_type: Option<PrimitiveLogicalType>,
    pub index: u32,
}

#[derive(Debug)]
pub struct GroupField {
    pub common: CommonField,
    pub children: Vec<SchemaField>,
    pub logical_type: Option<GroupLogicalType>,
}

#[derive(Debug)]
pub enum SchemaField {
    Group(GroupField),
    Primitive(PrimitiveField),
}

impl SchemaField {
    pub fn find_field_by_path<'field>(&'field self, path: &[&str]) -> Option<&'field SchemaField> {
        if let Some((name, remainder)) = path.split_first() {
            match self {
                SchemaField::Group(field) => field
                    .children
                    .iter()
                    .find(|child| child.name() == *name)
                    .and_then(|child| child.find_field_by_path(remainder)),
                SchemaField::Primitive(_) => {
                    // return an error instead?
                    None
                }
            }
        } else {
            Some(self)
        }
    }

    pub fn common(&self) -> &CommonField {
        match self {
            SchemaField::Group(field) => &field.common,
            SchemaField::Primitive(field) => &field.common,
        }
    }

    pub fn name(&self) -> &str {
        self.common().name.as_str()
    }

    pub fn definition_level(&self) -> u8 {
        self.common().def_level
    }

    pub fn repetition_level(&self) -> u8 {
        self.common().rep_level
    }

    pub fn is_optional(&self) -> bool {
        self.common().repetition == RepetitionType::Optional
    }
}

fn field_from_schema_element(
    element: &SchemaElement,
    index: &mut usize,
    def_level: u8,
    rep_level: u8,
    remaining_elements: usize,
) -> Result<SchemaField, ParquetError> {
    let repetition = element
        .repetition_type
        .map(|rt| RepetitionType::try_from(&rt))
        .transpose()?
        .unwrap_or(RepetitionType::Required);
    let common = CommonField {
        name: element.name.clone(),
        field_id: element.field_id.map(FieldId::from),
        repetition,
        def_level,
        rep_level,
    };

    let num_children = element.num_children.unwrap_or(0);
    if num_children == 0 {
        let type_ = element.type_.ok_or(ParquetError::Schema(
            "Missing physical type for SchemaElement without children",
        ))?;
        let physical_type = PhysicalType::try_from_schema_element(type_, element.type_length)?;
        let logical_type = if let Some(logical_type) = element.logicalType.as_ref() {
            Some(PrimitiveLogicalType::try_from_logical_type(logical_type)?)
        } else if let Some(converted_type) = element.converted_type.as_ref() {
            Some(PrimitiveLogicalType::try_from_converted_type(
                converted_type,
                element.precision,
                element.scale,
                element.type_length,
            )?)
        } else {
            None
        };
        let field = PrimitiveField {
            common,
            physical_type,
            logical_type,
            index: *index as u32,
        };
        *index += 1;
        Ok(SchemaField::Primitive(field))
    } else if num_children < 0 || num_children as usize > remaining_elements {
        return Err(ParquetError::Schema("Invalid number of children"));
    } else {
        let logical_type = if let Some(logical_type) = element.logicalType.as_ref() {
            match logical_type {
                LogicalType::MAP(_) => Some(GroupLogicalType::Map),
                LogicalType::LIST(_) => Some(GroupLogicalType::List),
                _ => return Err(ParquetError::Schema("Invalid logical type for group type")),
            }
        } else if let Some(converted_type) = element.converted_type.as_ref() {
            match *converted_type {
                ConvertedType::MAP => Some(GroupLogicalType::Map),
                ConvertedType::LIST => Some(GroupLogicalType::List),
                ConvertedType::MAP_KEY_VALUE => Some(GroupLogicalType::Map), // should this be a separate type?
                _ => {
                    return Err(ParquetError::Schema(
                        "Invalid converted type for group type",
                    ))
                }
            }
        } else {
            None
        };
        let children = Vec::with_capacity(num_children as usize);
        let field = GroupField {
            common,
            children,
            logical_type,
        };
        Ok(SchemaField::Group(field))
    }
}

fn convert_schema_inner<'schema>(
    elements: &'schema [SchemaElement],
    index: &mut usize,
    def_level: u8,
    rep_level: u8,
    nesting: u8,
) -> Result<(SchemaField, &'schema [SchemaElement]), ParquetError> {
    if elements.is_empty() {
        return Err(ParquetError::Schema("Missing schema elements"));
    }

    let root_element = &elements[0];
    let mut remainder_elements = &elements[1..];

    let repetition_type = root_element
        .repetition_type
        .unwrap_or(FieldRepetitionType::REQUIRED);
    // cannot overflow because nesting level has a lower limit than u8::MAX
    let def_level = def_level + (repetition_type == FieldRepetitionType::OPTIONAL) as u8;
    let rep_level = rep_level + (repetition_type == FieldRepetitionType::REPEATED) as u8;

    let mut current = field_from_schema_element(
        root_element,
        index,
        def_level,
        rep_level,
        remainder_elements.len(),
    )?;

    if let SchemaField::Group(GroupField { children, .. }) = &mut current {
        // limit recursion
        let nesting = nesting + 1;
        if nesting >= 128 {
            return Err(ParquetError::Schema("Schema is too deeply nested"));
        };

        let num_children = root_element.num_children.unwrap_or(0) as usize;

        let mut i = 0;
        while i < num_children {
            let (child_field, remainder) =
                convert_schema_inner(remainder_elements, index, def_level, rep_level, nesting)?;
            remainder_elements = remainder;
            children.push(child_field);
            i += 1;
        }
    }
    Ok((current, remainder_elements))
}

fn convert_schema(elements: &[SchemaElement]) -> Result<SchemaField, ParquetError> {
    if elements.is_empty() {
        return Err(ParquetError::Schema("Empty schema"));
    }

    let mut index = 0;

    let (root, remainder) = convert_schema_inner(elements, &mut index, 0, 0, 0)?;

    if !remainder.is_empty() {
        Err(ParquetError::Schema(
            "Trailing schema elements after root group",
        ))
    } else if root.common().repetition != RepetitionType::Required {
        Err(ParquetError::Schema(
            "Root element must have repetition type required",
        ))
    } else {
        Ok(root)
    }
}

fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!("  ");
    }
}

fn pretty_print_schema(field: &SchemaField, indent: usize) {
    print_indent(indent);
    match field {
        SchemaField::Group(field) => {
            print!("{} group {}", field.common.repetition, field.common.name);
            if let Some(logical_type) = field.logical_type {
                print!(" ({logical_type:?})")
            }

            println!(" {{");
            for child in &field.children {
                pretty_print_schema(child, indent + 1);
            }

            print_indent(indent);
            println!("}}")
        }
        SchemaField::Primitive(field) => {
            print!(
                "{} {:?} {}",
                field.common.repetition, field.physical_type, field.common.name
            );
            if let Some(logical_type) = field.logical_type.as_ref() {
                print!(" ({logical_type:?})")
            }
            println!(";");
        }
    }
}

pub fn main() {
    let mut file = File::open("parquet/data/alltypes_tiny_pages.parquet").unwrap();
    let data = get_metadata_chunk(&mut file).unwrap();
    let mut input = CompactThriftInputSlice::new(&data);

    let fmd = FileMetaData::read(&mut input).unwrap();

    println!("version: {}", fmd.version);
    println!("num of rows: {}", fmd.num_rows);

    if let Some(created_by) = fmd.created_by.as_ref() {
        println!("created by: {created_by}");
    }

    let schema = convert_schema(&fmd.schema).unwrap();

    pretty_print_schema(&schema, 0);
}
