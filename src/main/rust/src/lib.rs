use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Error as IOError, ErrorKind, Read, Write};
use std::str::{from_utf8, from_utf8_unchecked};

pub const MAX_BINARY_LEN: usize = 1024*1024;
pub const MAX_COLLECTION_LEN: usize = 10_000_000;

#[derive(Clone,Copy,Debug)]
pub enum ThriftError {
    InvalidNumber,
    InvalidString,
    InvalidBinaryLen,
    InvalidCollectionLen,
    MissingField,
    MissingValue,
    MissingStop,
    DuplicateField,
    InvalidType,
    ReserveError,
    IO(ErrorKind)
}

impl Display for ThriftError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Error for ThriftError {

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

fn decode_uleb<'i, I: CompactThriftInput<'i> + ?Sized>(input: &mut I) -> Result<u64, ThriftError> {
    let mut shift = 0_u32;
    let mut value = 0_u64;
    loop {
        let byte = input.read_byte()?;

        // overlong sequences are not treated as an error for performance reasons
        value |= ((byte & 0x7F) as u64).wrapping_shl(shift);
        shift += 7;

        if (byte & 0x80) == 0 {
            return Ok(value);
        }
    }
}

fn encode_uleb<O: CompactThriftOutput + ?Sized>(output: &mut O, mut value: u64) -> Result<(), ThriftError> {
    while value > 0x7F {
        output.write_byte((value as u8) | 0x80)?;
        value >>= 7;
    }
    output.write_byte(value as u8)
}

#[inline(always)]
fn zigzag_decode16(i: u16) -> i16 {
    (i >> 1) as i16 ^ -((i & 1) as i16)
}

#[inline(always)]
fn zigzag_decode32(i: u32) -> i32 {
    (i >> 1) as i32 ^ -((i & 1) as i32)
}

#[inline(always)]
fn zigzag_decode64(i: u64) -> i64 {
    (i >> 1) as i64 ^ -((i & 1) as i64)
}

#[inline(always)]
fn zigzag_encode16(i: i16) -> u16 {
    ((i << 1) ^ (i >> 15)) as u16
}

#[inline(always)]
fn zigzag_encode32(i: i32) -> u32 {
    ((i << 1) ^ (i >> 31)) as u32
}

#[inline(always)]
fn zigzag_encode64(i: i64) -> u64 {
    ((i << 1) ^ (i >> 63)) as u64
}

pub trait CompactThriftInput<'i> {
    fn read_byte(&mut self) -> Result<u8, ThriftError>;
    fn read_len(&mut self) -> Result<usize, ThriftError> {
        let len = decode_uleb(self)?;
        Ok(len as _)
    }
    fn read_i16(&mut self) -> Result<i16, ThriftError> {
        let i = decode_uleb(self)?;
        Ok(zigzag_decode16(i as _))
    }
    fn read_i32(&mut self) -> Result<i32, ThriftError> {
        let i = decode_uleb(self)?;
        Ok(zigzag_decode32(i as _))
    }
    fn read_i64(&mut self) -> Result<i64, ThriftError> {
        let i = decode_uleb(self)?;
        Ok(zigzag_decode64(i as _))
    }
    fn read_double(&mut self) -> Result<f64, ThriftError>;
    fn read_binary(&mut self) -> Result<Cow<'i, [u8]>, ThriftError>;
    fn read_string(&mut self) -> Result<Cow<'i, str>, ThriftError> {
        let binary = self.read_binary()?;
        let _ = from_utf8(binary.as_ref()).map_err(|_| ThriftError::InvalidString)?;
        // Safety: just checked for valid utf8
        unsafe {
            match binary {
                Cow::Owned(v) => Ok(Cow::Owned(String::from_utf8_unchecked(v))),
                Cow::Borrowed(v) => Ok(Cow::Borrowed(from_utf8_unchecked(v))),
            }
        }
    }
    fn skip_integer(&mut self) -> Result<(), ThriftError> {
        let _ = self.read_i64()?;
        Ok(())
    }
    fn skip_binary(&mut self) -> Result<(), ThriftError> {
        self.read_binary()?;
        Ok(())
    }
    fn skip_field(&mut self, field_type: u8) -> Result<(), ThriftError> {
        skip_field(self, field_type)
    }
    fn read_field_header(&mut self, last_field_id: &mut i16) -> Result<u8, ThriftError> {
        let field_header = self.read_byte()?;

        if field_header == 0 {
            return Ok(0)
        }

        let field_type = field_header & 0x0F;
        let field_delta = field_header >> 4;
        if field_delta != 0 {
            *last_field_id += field_delta as i16;
        } else {
            *last_field_id = self.read_i16()?;
        }

        Ok(field_type)
    }
}

#[inline]
fn read_collection_len_and_type<'i, T: CompactThriftInput<'i> + ?Sized>(input: &mut T) -> Result<(u32, u8), ThriftError> {
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
fn read_map_len_and_types<'i, T: CompactThriftInput<'i> + ?Sized>(input: &mut T) -> Result<(u32, u8, u8), ThriftError> {
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

fn skip_field<'i, T: CompactThriftInput<'i> + ?Sized>(input: &mut T, field_type: u8) -> Result<(), ThriftError> {
    match field_type {
        0..=2 => {
            // nothing else to read for STOP, TRUE, FALSE.
        }
        3 => {
            input.read_byte()?;
        }
        4..=6 => {
            // since we do not error on overlong sequences,
            // skipping for all integer types works the same.
            input.skip_integer()?;
        }
        7 => {
            input.read_double()?;
        }
        8 => {
            // thrift does not distinguish binary and string types in field_type,
            // consequently there is no utf8 validation for skipped strings.
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

#[inline]
fn write_field_header<T: CompactThriftOutput>(output: &mut T, field_type: u8, field_id: i16, last_field_id: &mut i16) -> Result<(), ThriftError> {
    let field_delta = field_id.wrapping_sub(*last_field_id);

    if field_delta > 15 {
        output.write_byte(field_type)?;
        output.write_i16(field_delta)?
    } else {
        output.write_byte(field_type | ((field_delta as u8) << 4))?;
    }
    *last_field_id = field_id;
    Ok(())
}


impl<R: Read + ?Sized> CompactThriftInput<'static> for R {
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

    fn read_binary(&mut self) -> Result<Cow<'static, [u8]>, ThriftError> {
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
        Ok(buf.into())
    }

}

pub struct SliceInput<'a>(&'a [u8]);

impl <'a> SliceInput<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self(slice)
    }

    pub fn as_slice(&self) -> &'a [u8] {
        self.0
    }
}

impl <'a> From<&'a [u8]> for SliceInput<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Self(slice)
    }
}

impl <'i> CompactThriftInput<'i> for SliceInput<'i> {
    #[inline]
    fn read_byte(&mut self) -> Result<u8, ThriftError> {
        if self.0.len() < 1 {
            return Err(ThriftError::from(ErrorKind::UnexpectedEof))
        }
        let first = self.0[0];
        self.0 = &self.0[1..];
        Ok(first)
    }

    #[inline]
    fn read_double(&mut self) -> Result<f64, ThriftError> {
        if self.0.len() < 8 {
            return Err(ThriftError::from(ErrorKind::UnexpectedEof))
        }
        let value = f64::from_le_bytes(self.0[..8].try_into().unwrap());
        self.0 = &self.0[8..];
        Ok(value)
    }

    fn read_binary(&mut self) -> Result<Cow<'i, [u8]>, ThriftError> {
        let len = self.read_len()?;
        if len > MAX_BINARY_LEN {
            return Err(ThriftError::InvalidBinaryLen);
        }
        if self.0.len() < len {
            return Err(ThriftError::from(ErrorKind::UnexpectedEof))
        }
        let (first, rest) = std::mem::replace(&mut self.0, &mut []).split_at(len);
        self.0 = rest;
        Ok(Cow::Borrowed(first))
    }

    fn skip_binary(&mut self) -> Result<(), ThriftError> {
        let len = self.read_len()?;
        if len > MAX_BINARY_LEN {
            return Err(ThriftError::InvalidBinaryLen);
        }
        if self.0.len() < len {
            return Err(ThriftError::from(ErrorKind::UnexpectedEof))
        }
        self.0 = &self.0[len..];
        Ok(())
    }
}

pub trait CompactThriftOutput {
    fn write_byte(&mut self, value: u8) -> Result<(), ThriftError>;
    fn write_len(&mut self, value: usize) -> Result<(), ThriftError>;
    fn write_i16(&mut self, value: i16) -> Result<(), ThriftError>;
    fn write_i32(&mut self, value: i32) -> Result<(), ThriftError>;
    fn write_i64(&mut self, value: i64) -> Result<(), ThriftError>;
    fn write_double(&mut self, value: f64) -> Result<(), ThriftError>;
    fn write_binary(&mut self, value: &[u8]) -> Result<(), ThriftError>;
    fn write_string(&mut self, value: &str) -> Result<(), ThriftError> {
        self.write_binary(value.as_bytes())
    }
}

impl <W: Write> CompactThriftOutput for W {
    fn write_byte(&mut self, value: u8) -> Result<(), ThriftError> {
        self.write(&[value])?;
        Ok(())
    }

    fn write_len(&mut self, value: usize) -> Result<(), ThriftError> {
        encode_uleb(self, value as _)
    }

    fn write_i16(&mut self, value: i16) -> Result<(), ThriftError> {
        encode_uleb(self, zigzag_encode16(value) as _)
    }

    fn write_i32(&mut self, value: i32) -> Result<(), ThriftError> {
        encode_uleb(self, zigzag_encode32(value) as _)
    }

    fn write_i64(&mut self, value: i64) -> Result<(), ThriftError> {
        encode_uleb(self, zigzag_encode64(value) as _)
    }

    fn write_double(&mut self, value: f64) -> Result<(), ThriftError> {
        self.write(&value.to_le_bytes())?;
        Ok(())
    }

    fn write_binary(&mut self, value: &[u8]) -> Result<(), ThriftError> {
        if value.len() > MAX_BINARY_LEN {
            return Err(ThriftError::InvalidBinaryLen);
        }
        self.write_len(value.len())?;
        self.write(value)?;
        Ok(())
    }
}

pub trait CompactThriftProtocol<'i> {
    const FIELD_TYPE: u8;

    fn read<T: CompactThriftInput<'i>>(input: &mut T) -> Result<Self, ThriftError> where Self: Default{
        let mut result = Self::default();
        Self::fill(&mut result, input)?;
        Ok(result)
    }
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError>;
    #[inline]
    fn fill_field<T: CompactThriftInput<'i>>(&mut self, input: &mut T, field_type: u8) -> Result<(), ThriftError> {
        if field_type != Self::FIELD_TYPE {
            return Err(ThriftError::InvalidType)
        }
        self.fill(input)
    }
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError>;
    #[inline]
    fn write_field<T: CompactThriftOutput>(&self, output: &mut T, field_id: i16, last_field_id: &mut i16) -> Result<(), ThriftError> {
        write_field_header(output, Self::FIELD_TYPE, field_id, last_field_id)?;
        self.write(output)?;
        Ok(())
    }
}

impl <'i> CompactThriftProtocol<'i> for bool {
    const FIELD_TYPE: u8 = 2; // TRUE = 1, FALSE = 2

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let byte = input.read_byte()?;
        // according to the spec the bytes in a collection should be FALSE = 0, TRUE = 1,
        // but at least the java implementation writes the same values as for field types
        *self = byte == 1;
        Ok(())
    }

    #[inline]
    fn fill_field<T: CompactThriftInput<'i>>(&mut self, _input: &mut T, field_type: u8) -> Result<(), ThriftError> {
        // no error checking
        *self = field_type == 1;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_byte(*self as u8)
    }

    fn write_field<T: CompactThriftOutput>(&self, output: &mut T, field_id: i16, last_field_id: &mut i16) -> Result<(), ThriftError> {
        let field_type = 1 + (!*self) as u8;
        write_field_header(output, field_type, field_id, last_field_id)
    }
}

impl <'i> CompactThriftProtocol<'i> for i8 {
    const FIELD_TYPE: u8 = 3;

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_byte()? as i8;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_byte(*self as u8)
    }
}

impl <'i> CompactThriftProtocol<'i> for i16 {
    const FIELD_TYPE: u8 = 4;

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_i16()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i16(*self)
    }
}

impl <'i> CompactThriftProtocol<'i> for i32 {
    const FIELD_TYPE: u8 = 5;

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_i32()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(*self)
    }
}

impl <'i> CompactThriftProtocol<'i> for i64 {
    const FIELD_TYPE: u8 = 6;

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_i64()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i64(*self)
    }
}

impl <'i> CompactThriftProtocol<'i> for f64 {
    const FIELD_TYPE: u8 = 7;

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_double()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_double(*self)
    }
}

impl <'i> CompactThriftProtocol<'i> for Vec<u8> {
    const FIELD_TYPE: u8 = 8;

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_binary()?.to_vec();
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_binary(self.as_slice())
    }
}

impl <'i> CompactThriftProtocol<'i> for String {
    const FIELD_TYPE: u8 = 8; // Same type as Binary?

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_string()?.into_owned();
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_string(self.as_str())
    }
}

impl <'i> CompactThriftProtocol<'i> for Cow<'i, [u8]> {
    const FIELD_TYPE: u8 = 8;

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_binary()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_binary(self.as_ref())
    }
}

impl <'i> CompactThriftProtocol<'i> for Cow<'i, str> {
    const FIELD_TYPE: u8 = 8;

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_string()?;
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_string(self.as_ref())
    }
}

impl <'i, P: CompactThriftProtocol<'i> + Default> CompactThriftProtocol<'i> for Vec<P> {
    const FIELD_TYPE: u8 = 9;

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let (len, element_type) = read_collection_len_and_type(input)?;
        if element_type != P::FIELD_TYPE && !(P::FIELD_TYPE == bool::FIELD_TYPE && element_type == 1) {
            return Err(ThriftError::InvalidType);
        }
        self.clear();
        self.try_reserve(len as usize).map_err(|_| ThriftError::ReserveError)?;

        // workaround for unnecessary memcpy calls when using Vec::push(P::default()) with larger structs

        struct SetLenOnDrop<'a, T> {
            vec: &'a mut Vec<T>,
            len: usize,
        }

        impl <'a, T> Drop for SetLenOnDrop<'a, T> {
            fn drop(&mut self) {
                unsafe {
                    self.vec.set_len(self.len)
                }
            }
        }

        let mut guard = SetLenOnDrop {
            vec: self,
            len: 0,
        };

        for i in 0..len as usize {
            // Safety: len was reserved
            unsafe {
                let ptr = guard.vec.as_mut_ptr().add(i);
                ptr.write(P::default());
                (*ptr).fill(input)?;
                guard.len += 1;
            }
        }
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        if self.len() > MAX_COLLECTION_LEN {
            return Err(ThriftError::InvalidCollectionLen);
        }
        output.write_len(self.len())?;
        self.iter().try_for_each(|value| {
            value.write(output)
        })
    }
}

impl <'i, P: CompactThriftProtocol<'i> + Default> CompactThriftProtocol<'i> for Option<P> {
    const FIELD_TYPE: u8 = P::FIELD_TYPE;

    #[inline]
    fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        if self.is_some() {
            return Err(ThriftError::DuplicateField);
        }
        // Safety: avoid generating drop calls, content is always None because of check above
        unsafe {
            std::ptr::write(self as *mut _, Some(P::default()));
            self.as_mut().unwrap_unchecked().fill(input)?;
        }
        Ok(())
    }

    #[inline]
    fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        if let Some(value) = self.as_ref() {
            value.write(output)
        } else {
            Err(ThriftError::MissingValue)
        }
    }

    #[inline]
    fn write_field<T: CompactThriftOutput>(&self, output: &mut T, field_id: i16, last_field_id: &mut i16) -> Result<(), ThriftError> {
        // only write if present
        if let Some(value) = self.as_ref() {
            write_field_header(output, Self::FIELD_TYPE, field_id, last_field_id)?;
            value.write(output)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{CompactThriftInput, CompactThriftOutput, CompactThriftProtocol, decode_uleb, encode_uleb, SliceInput, ThriftError};

    #[test]
    fn test_size_of_error() {
        assert_eq!(std::mem::size_of::<ThriftError>(), 1);
        assert_eq!(std::mem::size_of::<Result<(), ThriftError>>(), 1);
    }

    #[test]
    fn test_slice_input_read_byte() {
        let mut input = SliceInput::new(&[1]);
        assert_eq!(input.read_byte().unwrap(), 1);
    }

    #[test]
    fn test_read_uleb() {
        assert_eq!(decode_uleb(&mut SliceInput::new(&[1])).unwrap(), 1);
        assert_eq!(decode_uleb(&mut SliceInput::new(&[0b0111_1111])).unwrap(), 0b0111_1111);
        assert_eq!(decode_uleb(&mut SliceInput::new(&[0b1000_1111, 0b0111_0101])).unwrap(), 0b0011_1010_1000_1111);
    }

    #[test]
    fn test_read_uleb_overlong() {
        decode_uleb(&mut SliceInput::new(&[0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0])).unwrap();
    }

     #[test]
    fn test_slice_input_read_i32() {
        assert_eq!(SliceInput::new(&[0]).read_i32().unwrap(), 0);
        assert_eq!(SliceInput::new(&[1]).read_i32().unwrap(), -1);
        assert_eq!(SliceInput::new(&[2]).read_i32().unwrap(), 1);

        assert_eq!(SliceInput::new(&[0b0111_1111]).read_i32().unwrap(), -64);
        assert_eq!(SliceInput::new(&[0b1000_1111, 0b0111_0101]).read_i32().unwrap(), -7496);
        assert_eq!(SliceInput::new(&[0b1000_1111, 0b0111_0101, 0, 0]).read_i32().unwrap(), -7496);
        assert_eq!(SliceInput::new(&[0b1000_1111, 0b0111_0101, 0, 0, 0]).read_i32().unwrap(), -7496);
    }

    #[test]
    fn test_uleb_roundtrip() {
        let mut w = vec![];
        w.write_i64(1234567890).unwrap();
        let mut r = SliceInput::new(&w);
        assert_eq!(r.read_i64().unwrap(), 1234567890);
    }

    #[test]
    fn test_read_vec_bool() {
        let mut data = SliceInput::new(&[0x42, 0, 1, 1, 0]);
        let actual = Vec::<bool>::read(&mut data).unwrap();
        let expected = vec![false, true, true, false];
        assert_eq!(&actual, &expected);

        // also allow element type 1 for boolean
        let mut data = SliceInput::new(&[0x41, 0, 1, 1, 0]);
        let actual = Vec::<bool>::read(&mut data).unwrap();
        let expected = vec![false, true, true, false];
        assert_eq!(&actual, &expected);

    }
}