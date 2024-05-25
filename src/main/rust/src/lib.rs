use std::io::{Error as IOError, ErrorKind, Read};

pub const MAX_BINARY_LEN: usize = 1024*1024;
pub const MAX_COLLECTION_LEN: usize = 10_000_000;

#[derive(Clone,Copy,Debug)]
pub enum ThriftError {
    InvalidNumber,
    InvalidString,
    InvalidBinaryLen,
    InvalidCollectionLen,
    MissingField,
    DuplicateField,
    InvalidType,
    ReserveError,
    IO(ErrorKind)
}

impl From<IOError> for ThriftError {
    fn from(e: IOError) -> Self {
        Self::IO(e.kind())
    }
}

impl From<ErrorKind> for ThriftError {
    fn from(kind: ErrorKind) -> Self {
        Self::IO(kind)
    }
}

#[inline]
fn decode_uleb<const MAX_BITS: u32, I: CompactThriftInput>(input: &mut I) -> Result<u64, ThriftError> {
    let mut shift = 0_u32;
    let mut value = 0_u64;
    loop {
        let byte = input.read_byte()?;

        value |= ((byte & 0x7F) as u64) << shift;

        if (byte & 0x80) == 0 {
            break;
        }

        shift += 7;

        if shift >= MAX_BITS {
            return Err(ThriftError::InvalidNumber);
        }
    }
    Ok(value)
}

#[inline(always)]
fn zigzag16(i: u16) -> i16 {
    (i >> 1) as i16 ^ -((i & 1) as i16)
}

#[inline(always)]
fn zigzag32(i: u32) -> i32 {
    (i >> 1) as i32 ^ -((i & 1) as i32)
}

#[inline(always)]
fn zigzag64(i: u64) -> i64 {
    (i >> 1) as i64 ^ -((i & 1) as i64)
}

pub trait CompactThriftInput {
    fn read_byte(&mut self) -> Result<u8, ThriftError>;
    fn read_len(&mut self) -> Result<usize, ThriftError> where Self: Sized {
        let len = decode_uleb::<28, _>(self)?;
        Ok(len as _)
    }
    fn read_i16(&mut self) -> Result<i16, ThriftError> where Self: Sized {
        let i = decode_uleb::<16, _>(self)?;
        Ok(zigzag16(i as _))
    }
    fn read_i32(&mut self) -> Result<i32, ThriftError> where Self: Sized{
        let i = decode_uleb::<32, _>(self)?;
        Ok(zigzag32(i as _))
    }
    fn read_i64(&mut self) -> Result<i64, ThriftError> where Self: Sized{
        let i = decode_uleb::<64, _>(self)?;
        Ok(zigzag64(i as _))
    }
    fn read_double(&mut self) -> Result<f64, ThriftError>;
    fn read_binary(&mut self) -> Result<Vec<u8>, ThriftError>;
    fn read_string(&mut self) -> Result<String, ThriftError> {
        let binary = self.read_binary()?;
        String::from_utf8(binary).map_err(|_| ThriftError::InvalidString)
    }
    fn skip_binary(&mut self) -> Result<(), ThriftError> {
        self.read_binary()?;
        Ok(())
    }
}

#[inline]
fn read_collection_len_and_type<T: CompactThriftInput>(input: &mut T) -> Result<(u32, u8), ThriftError> {
    let header = input.read_byte()?;
    let field_type = header & 0x0F;
    let maybe_len = (header & 0xF0) >> 4;
    let len = if maybe_len != 0x0F {
        // high bits set high if count and type encoded separately
        maybe_len as usize
    } else {
        input.read_len()?
    };

    if len > MAX_COLLECTION_LEN {
        return Err(ThriftError::InvalidCollectionLen)
    }

    Ok((len as u32, field_type))
}

#[inline]
fn read_map_len_and_types<T: CompactThriftInput>(input: &mut T) -> Result<(u32, u8, u8), ThriftError> {
    let len = input.read_len()?;
    if len == 0 {
        return Ok((0, 0, 0))
    }
    let entry_type = input.read_byte()?;
    // TODO: check order of nibbles
    let key_type = entry_type >> 4;
    let val_type = entry_type & 0x0F;

    if len > MAX_COLLECTION_LEN {
        return Err(ThriftError::InvalidCollectionLen)
    }

    Ok((len as u32, key_type, val_type))
}

pub fn skip_field<T: CompactThriftInput>(input: &mut T, field_type: u8) -> Result<(), ThriftError> {
    match field_type {
        0..=2 => {
            // nothing else to read for STOP, TRUE, FALSE
        }
        3 => {
            input.read_byte()?;
        }
        4 => {
            input.read_i16()?;
        }
        5 => {
            input.read_i32()?;
        }
        6 => {
            input.read_i64()?;
        }
        7 => {
            input.read_double()?;
        }
        8 => {
            input.skip_binary()?;
        }
        9 | 10 => {
            // list | set
            let (len, element_type) = read_collection_len_and_type(input)?;
            for _ in 0..len {
                skip_field(input, element_type)?;
            }
        }
        11 => {
            // map
            let (len, key_type, val_type) = read_map_len_and_types(input)?;
            for _ in 0..len {
                skip_field(input, key_type)?;
                skip_field(input, val_type)?;
            }
        }
        12 => {
            // struct | union
            loop {
                let field_header = input.read_byte()?;

                if field_header == 0 {
                    break;
                }

                let field_type = field_header & 0x0F;
                let field_delta = field_header >> 4;
                if field_delta == 0 {
                    let _field_id = input.read_i16()?;
                }
                skip_field(input, field_type)?;
            }
        }
        _ => {
            return Err(ThriftError::InvalidType)
        }
    }
    Ok(())
}

impl<R: Read> CompactThriftInput for R {
    #[inline]
    fn read_byte(&mut self) -> Result<u8, ThriftError> {
        let mut buf = [0_u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_double(&mut self) -> Result<f64, ThriftError> {
        let mut buf = [0_u8; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }

    fn read_binary(&mut self) -> Result<Vec<u8>, ThriftError> {
        let len = self.read_len()?;
        if len > MAX_BINARY_LEN {
            return Err(ThriftError::InvalidBinaryLen);
        }
        let mut buf = Vec::with_capacity(len);
        // Safety: we trust the Read implementation to only write into buf,
        // and not to look at uninitialized bytes
        unsafe {
            buf.set_len(len);
        }
        self.read_exact(buf.as_mut_slice())?;
        Ok(buf)
    }

}

pub struct SliceInput<'a>(&'a [u8]);

impl <'a> SliceInput<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self(slice)
    }
}

impl <'a> From<&'a [u8]> for SliceInput<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Self(slice)
    }
}

impl CompactThriftInput for SliceInput<'_> {
    #[inline]
    fn read_byte(&mut self) -> Result<u8, ThriftError> {
        if self.0.len() < 1 {
            return Err(ThriftError::from(ErrorKind::UnexpectedEof))
        }
        let (first, rest) = std::mem::replace(self, Self(&mut [])).0.split_at(1);
        *self = Self(rest);
        Ok(first[0])
    }

    #[inline]
    fn read_double(&mut self) -> Result<f64, ThriftError> {
        if self.0.len() < 8 {
            return Err(ThriftError::from(ErrorKind::UnexpectedEof))
        }
        let (first, rest) = std::mem::replace(self, Self(&mut [])).0.split_at(8);
        *self = Self(rest);
        Ok(f64::from_le_bytes(first.try_into().unwrap()))
    }

    fn read_binary(&mut self) -> Result<Vec<u8>, ThriftError> {
        let len = self.read_len()?;
        if len > MAX_BINARY_LEN {
            return Err(ThriftError::InvalidBinaryLen);
        }
        if self.0.len() < len {
            return Err(ThriftError::from(ErrorKind::UnexpectedEof))
        }
        let (first, rest) = std::mem::replace(self, Self(&mut [])).0.split_at(len);
        let mut vec = Vec::<u8>::default();
        vec.try_reserve(len).map_err(|_|ThriftError::ReserveError)?;
        unsafe {
            vec.as_mut_ptr().copy_from_nonoverlapping(first.as_ptr(), len);
            vec.set_len(len);
        }
        *self = Self(rest);
        Ok(vec)
    }

    fn skip_binary(&mut self) -> Result<(), ThriftError> {
        let len = self.read_len()?;
        if len > MAX_BINARY_LEN {
            return Err(ThriftError::InvalidBinaryLen);
        }
        let len = len as usize;
        let (_first, rest) = std::mem::replace(self, Self(&mut [])).0.split_at(len);
        *self = Self(rest);
        Ok(())
    }
}

pub trait CompactThriftOutput {
    fn write_byte(&mut self, value: u8) -> Result<(), ThriftError>;
    fn write_i16(&mut self, value: i16) -> Result<(), ThriftError>;
    fn write_i32(&mut self, value: i32) -> Result<(), ThriftError>;
    fn write_i64(&mut self, value: i64) -> Result<(), ThriftError>;
    fn write_double(&mut self, value: f64) -> Result<(), ThriftError>;
    fn write_binary(&mut self, value: &[u8]) -> Result<(), ThriftError>;
    fn write_string(&mut self, value: &str) -> Result<(), ThriftError> {
        self.write_binary(value.as_bytes())
    }
}

pub trait CompactThriftProtocol {
    const FIELD_TYPE: u8;

    fn read<T: CompactThriftInput>(input: &mut T) -> Result<Self, ThriftError> where Self: Default{
        let mut result = Self::default();
        Self::fill(&mut result, input)?;
        Ok(result)
    }
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError>;
    #[inline]
    fn fill_field<T: CompactThriftInput>(&mut self, input: &mut T, field_type: u8) -> Result<(), ThriftError> {
        if field_type != Self::FIELD_TYPE {
            return Err(ThriftError::InvalidType)
        }
        self.fill(input)
    }
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError>;
    fn write_field<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_byte(Self::FIELD_TYPE)?;
        self.write(output)?;
        Ok(())
    }
}

impl CompactThriftProtocol for bool {
    const FIELD_TYPE: u8 = 2; // TRUE = 1, FALSE = 2

    #[inline]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = match input.read_byte()? {
            0 => false,
            1 => true,
            _ => return Err(ThriftError::InvalidType)
        };
        Ok(())
    }

    #[inline]
    fn fill_field<T: CompactThriftInput>(&mut self, _input: &mut T, field_type: u8) -> Result<(), ThriftError> {
        *self = match field_type {
            1 => true,
            2 => false,
            _ => return Err(ThriftError::InvalidType),
        };
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_byte(*self as u8)
    }

    #[inline]
    fn write_field<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_byte(1 + (!*self) as u8)
    }
}

impl CompactThriftProtocol for i8 {
    const FIELD_TYPE: u8 = 3;

    #[inline]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_byte()? as i8;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_byte(*self as u8)
    }
}

impl CompactThriftProtocol for i16 {
    const FIELD_TYPE: u8 = 4;

    #[inline]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_i16()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i16(*self)
    }
}

impl CompactThriftProtocol for i32 {
    const FIELD_TYPE: u8 = 5;

    #[inline]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_i32()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(*self)
    }
}

impl CompactThriftProtocol for i64 {
    const FIELD_TYPE: u8 = 6;

    #[inline]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_i64()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i64(*self)
    }
}

impl CompactThriftProtocol for f64 {
    const FIELD_TYPE: u8 = 7;

    #[inline]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_double()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_double(*self)
    }
}

impl CompactThriftProtocol for Vec<u8> {
    const FIELD_TYPE: u8 = 8;

    #[inline]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_binary()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_binary(self.as_slice())
    }
}

impl CompactThriftProtocol for String {
    const FIELD_TYPE: u8 = 8; // Same type as Binary?

    #[inline]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_string()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_string(self.as_str())
    }
}


impl <P: CompactThriftProtocol + Default> CompactThriftProtocol for Vec<P> {
    const FIELD_TYPE: u8 = 9;

    #[inline]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let (len, element_type) = read_collection_len_and_type(input)?;
        if element_type != P::FIELD_TYPE {
            return Err(ThriftError::InvalidType);
        }
        // *self = (0..len).map(|_| P::read(input)).collect::<Result<Vec<P>, ThriftError>>()?;
        self.clear();
        self.try_reserve(len as usize).map_err(|_| ThriftError::ReserveError)?;
        for _ in 0..len {
            self.push(P::read(input)?);
        }
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        self.iter().try_for_each(|value| {
            value.write(output)
        })
    }
}

impl <P: CompactThriftProtocol + Default> CompactThriftProtocol for Option<P> {
    const FIELD_TYPE: u8 = P::FIELD_TYPE;

    #[inline]
    fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        if self.is_some() {
            return Err(ThriftError::DuplicateField);
        }
        // avoid generating drop calls for any old value
        let old = self.replace(P::default());
        std::mem::forget(old); // is always None because of check above
        unsafe {
            self.as_mut().unwrap_unchecked().fill(input)?;
        }
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        // no nulls, this method should not be used
        if let Some(value) = self.as_ref() {
            value.write_field(output)
        } else {
            P::default().write_field(output)
        }
    }

    #[inline]
    fn write_field<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        // only write if present
        if let Some(value) = self.as_ref() {
            value.write_field(output)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{CompactThriftInput, decode_uleb, SliceInput, ThriftError};

    #[test]
    fn test_size_of_error() {
        assert_eq!(std::mem::size_of::<ThriftError>(), 8);
    }

    #[test]
    fn test_size_of_collection_len_and_type() {
        assert_eq!(std::mem::size_of::<Option<(u32, u8)>>(), 8);
        assert_eq!(std::mem::size_of::<Result<(u32, u8), ThriftError>>(), 8);
    }

    #[test]
    fn test_slice_input_read_byte() {
        let mut input = SliceInput::new(&[1]);
        assert_eq!(input.read_byte().unwrap(), 1);
    }

    #[test]
    fn test_read_uleb() {
        assert_eq!(decode_uleb::<16, _>(&mut SliceInput::new(&[1])).unwrap(), 1);
        assert_eq!(decode_uleb::<16, _>(&mut SliceInput::new(&[0b0111_1111])).unwrap(), 0b0111_1111);
        assert_eq!(decode_uleb::<16, _>(&mut SliceInput::new(&[0b1000_1111, 0b0111_0101])).unwrap(), 0b0011_1010_1000_1111);
    }
}