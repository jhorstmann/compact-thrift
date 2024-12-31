use std::borrow::Cow;

use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;

use std::str::from_utf8_unchecked;
use std::str::from_utf8;

use crate::ThriftError;
use crate::uleb::*;

pub const MAX_BINARY_LEN: usize = 1024*1024;
pub const MAX_COLLECTION_LEN: usize = 10_000_000;

#[inline(never)] // full field ids are uncommon and inlining this bloats the code
fn read_full_field_id<'i, I: CompactThriftInput<'i> + ?Sized>(input: &mut I) -> Result<i16, ThriftError> {
    let field_id = decode_uleb(input)? as u16;
    Ok(zigzag_decode16(field_id))
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
        skip_field(self, field_type, false)
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
            *last_field_id = read_full_field_id(self)?;
        }

        Ok(field_type)
    }
}

pub(crate) fn read_collection_len_and_type<'i, T: CompactThriftInput<'i> + ?Sized>(input: &mut T) -> Result<(u32, u8), ThriftError> {
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

pub(crate) fn read_map_len_and_types<'i, T: CompactThriftInput<'i> + ?Sized>(input: &mut T) -> Result<(u32, u8, u8), ThriftError> {
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

fn skip_collection<'i, T: CompactThriftInput<'i> + ?Sized>(input: &mut T) -> Result<(), ThriftError> {
    let (len, element_type) = read_collection_len_and_type(input)?;
    match element_type {
        1..=3 => {
            // TRUE, FALSE, i8 stored as single byte
            for _ in 0..len {
                let _ = input.read_byte()?;
            }
        }
        4..=6 => {
            // since we do not error on overlong sequences,
            // skipping for all integer types works the same.
            for _ in 0..len {
                input.skip_integer()?;
            }
        }
        7 => {
            for _ in 0..len {
                input.read_double()?;
            }
        }
        8 => {
            // thrift does not distinguish binary and string types in field_type,
            // consequently there is no utf8 validation for skipped strings.
            for _ in 0..len {
                input.skip_binary()?;
            }
        }
        9 | 10 => {
            // list | set
            for _ in 0..len {
                skip_collection(input)?;
            }
        }
        11 => {
            // map
            for _ in 0..len {
                skip_map(input)?;
            }
        }
        12 => {
            for _ in 0..len {
                skip_field(input, 12, false)?;
            }
        }
        _ => {
            return Err(ThriftError::InvalidType)
        }
    }
    Ok(())
}

fn skip_map<'i, T: CompactThriftInput<'i> + ?Sized>(input: &mut T) -> Result<(), ThriftError> {
    let (len, key_type, val_type) = read_map_len_and_types(input)?;
    for _ in 0..len {
        skip_field(input, key_type, true)?;
        skip_field(input, val_type, true)?;
    }
    Ok(())
}

pub(crate) fn skip_field<'i, T: CompactThriftInput<'i> + ?Sized>(input: &mut T, field_type: u8, inside_collection: bool) -> Result<(), ThriftError> {
    match field_type {
        1..=2 => {
            // boolean stored inside the field header outside of collections
            if inside_collection {
                input.read_byte()?;
            }
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
            skip_collection(input)?;
        }
        11 => {
            // map
            skip_map(input)?;
        }
        12 => {
            // struct | union
            let mut last_field_id = 0_i16;
            loop {
                let field_type = input.read_field_header(&mut last_field_id)?;
                if field_type == 0 {
                    break;
                }
                skip_field(input, field_type, false)?;
            }
        }
        _ => {
            return Err(ThriftError::InvalidType)
        }
    }
    Ok(())
}

#[inline]
pub(crate) fn write_field_header<T: CompactThriftOutput>(output: &mut T, field_type: u8, field_id: i16, last_field_id: &mut i16) -> Result<(), ThriftError> {
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

#[derive(Clone)]
pub struct CompactThriftInputSlice<'a>(&'a [u8]);

impl <'a> CompactThriftInputSlice<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self(slice)
    }

    pub fn as_slice(&self) -> &'a [u8] {
        self.0
    }
}

impl <'a> From<&'a [u8]> for CompactThriftInputSlice<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Self(slice)
    }
}

impl <'i> CompactThriftInput<'i> for CompactThriftInputSlice<'i> {
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

    #[inline]
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

    #[inline]
    #[cfg(target_feature = "sse2")]
    fn skip_integer(&mut self) -> Result<(), ThriftError> {
        if self.0.len() >= 16 {
            self.0 = unsafe { skip_uleb_sse2(self.0) };
            Ok(())
        } else {
            self.0 = skip_uleb_fallback(self.0);
            Ok(())
        }
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

    fn read_thrift<T: CompactThriftInput<'i>>(input: &mut T) -> Result<Self, ThriftError> where Self: Default{
        let mut result = Self::default();
        Self::fill_thrift(&mut result, input)?;
        Ok(result)
    }
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError>;
    #[inline]
    fn fill_thrift_field<T: CompactThriftInput<'i>>(&mut self, input: &mut T, field_type: u8) -> Result<(), ThriftError> {
        if field_type != Self::FIELD_TYPE {
            return Err(ThriftError::InvalidType)
        }
        self.fill_thrift(input)
    }
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError>;
    #[inline]
    fn write_thrift_field<T: CompactThriftOutput>(&self, output: &mut T, field_id: i16, last_field_id: &mut i16) -> Result<(), ThriftError> {
        write_field_header(output, Self::FIELD_TYPE, field_id, last_field_id)?;
        self.write_thrift(output)?;
        Ok(())
    }
}