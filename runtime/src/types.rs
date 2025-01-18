use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;
use crate::protocol::*;
use crate::ThriftError;

impl <'i> CompactThriftProtocol<'i> for bool {
    const FIELD_TYPE: u8 = 2; // TRUE = 1, FALSE = 2

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let byte = input.read_byte()?;
        // according to the spec the bytes in a collection should be FALSE = 0, TRUE = 1,
        // but at least the java implementation writes the same values as for field types
        *self = byte == 1;
        Ok(())
    }

    #[inline]
    fn fill_thrift_field<T: CompactThriftInput<'i>>(&mut self, _input: &mut T, field_type: u8) -> Result<(), ThriftError> {
        // no error checking
        *self = field_type == 1;
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_byte(*self as u8)
    }

    fn write_thrift_field<T: CompactThriftOutput>(&self, output: &mut T, field_id: i16, last_field_id: &mut i16) -> Result<(), ThriftError> {
        let field_type = 1 + (!*self) as u8;
        write_field_header(output, field_type, field_id, last_field_id)
    }
}

impl <'i> CompactThriftProtocol<'i> for i8 {
    const FIELD_TYPE: u8 = 3;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_byte()? as i8;
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_byte(*self as u8)
    }
}

impl <'i> CompactThriftProtocol<'i> for i16 {
    const FIELD_TYPE: u8 = 4;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_i16()?;
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i16(*self)
    }
}

impl <'i> CompactThriftProtocol<'i> for i32 {
    const FIELD_TYPE: u8 = 5;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_i32()?;
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i32(*self)
    }
}

impl <'i> CompactThriftProtocol<'i> for i64 {
    const FIELD_TYPE: u8 = 6;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_i64()?;
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_i64(*self)
    }
}

impl <'i> CompactThriftProtocol<'i> for f64 {
    const FIELD_TYPE: u8 = 7;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_double()?;
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_double(*self)
    }
}

impl <'i> CompactThriftProtocol<'i> for Vec<u8> {
    const FIELD_TYPE: u8 = 8;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_binary()?.into_owned();
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_binary(self.as_slice())
    }
}

impl <'i> CompactThriftProtocol<'i> for String {
    const FIELD_TYPE: u8 = 8; // Same type as Binary?

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_string()?.into_owned();
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_string(self.as_str())
    }
}

impl <'i> CompactThriftProtocol<'i> for Cow<'i, [u8]> {
    const FIELD_TYPE: u8 = 8;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_binary()?;
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_binary(self.as_ref())
    }
}

impl <'i> CompactThriftProtocol<'i> for Cow<'i, str> {
    const FIELD_TYPE: u8 = 8;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_string()?;
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_string(self.as_ref())
    }
}

impl <'i, P: CompactThriftProtocol<'i> + Default> CompactThriftProtocol<'i> for Vec<P> {
    const FIELD_TYPE: u8 = 9;

    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        let (len, element_type) = read_collection_len_and_type(input)?;
        if element_type != P::FIELD_TYPE && !(P::FIELD_TYPE == bool::FIELD_TYPE && element_type == 1) {
            return Err(ThriftError::InvalidType);
        }
        self.clear();
        self.try_reserve(len as usize).map_err(|_| ThriftError::ReserveError)?;

        for _ in 0..len as usize {
            // workaround for unnecessary memcpy calls when using Vec::push(P::default()) with larger structs
            // https://github.com/rust-lang/rust/issues/125632
            self.extend((0..1).map(|_| P::default()));
            unsafe {
                self.last_mut().unwrap_unchecked().fill_thrift(input)?;
            }
        }

        Ok(())
    }

    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        let len = self.len();
        if len > MAX_COLLECTION_LEN {
            return Err(ThriftError::InvalidCollectionLen);
        }

        if len < 15 {
            let header = P::FIELD_TYPE | ((len as u8) << 4);
            output.write_byte(header)?;
        } else {
            output.write_byte(P::FIELD_TYPE | 0xF0)?;
            output.write_len(len)?;
        }
        self.iter().try_for_each(|value| {
            value.write_thrift(output)
        })
    }
}

impl <'i, P: CompactThriftProtocol<'i> + Default> CompactThriftProtocol<'i> for Option<P> {
    const FIELD_TYPE: u8 = P::FIELD_TYPE;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        if self.is_some() {
            return Err(ThriftError::DuplicateField);
        }
        // Safety: avoid generating drop calls, content is always None because of check above
        unsafe {
            std::ptr::write(self as *mut _, Some(P::default()));
            self.as_mut().unwrap_unchecked().fill_thrift(input)?;
        }
        Ok(())
    }

    #[inline]
    fn fill_thrift_field<T: CompactThriftInput<'i>>(&mut self, input: &mut T, field_type: u8) -> Result<(), ThriftError> {
        if self.is_some() {
            return Err(ThriftError::DuplicateField);
        }
        // Safety: avoid generating drop calls, content is always None because of check above
        unsafe {
            std::ptr::write(self as *mut _, Some(P::default()));
            self.as_mut().unwrap_unchecked().fill_thrift_field(input, field_type)?;
        }
        Ok(())
    }


    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        if let Some(value) = self.as_ref() {
            value.write_thrift(output)
        } else {
            Err(ThriftError::MissingValue)
        }
    }

    #[inline]
    fn write_thrift_field<T: CompactThriftOutput>(&self, output: &mut T, field_id: i16, last_field_id: &mut i16) -> Result<(), ThriftError> {
        // only write if present
        if let Some(value) = self.as_ref() {
            value.write_thrift_field(output, field_id, last_field_id)?;
        }
        Ok(())
    }
}

impl <'i, P: CompactThriftProtocol<'i> + Default> CompactThriftProtocol<'i> for Box<P> {
    const FIELD_TYPE: u8 = P::FIELD_TYPE;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        Box::as_mut(self).fill_thrift(input)
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        Box::as_ref(self).write_thrift(output)
    }
}

impl <'i, P: CompactThriftProtocol<'i> + Default> CompactThriftProtocol<'i> for Rc<P> {
    const FIELD_TYPE: u8 = P::FIELD_TYPE;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        Rc::get_mut(self).ok_or_else(|| ThriftError::DuplicateField)?.fill_thrift(input)
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        Rc::as_ref(self).write_thrift(output)
    }
}

impl <'i, P: CompactThriftProtocol<'i> + Default> CompactThriftProtocol<'i> for Arc<P> {
    const FIELD_TYPE: u8 = P::FIELD_TYPE;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        Arc::get_mut(self).ok_or_else(|| ThriftError::DuplicateField)?.fill_thrift(input)
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        Arc::as_ref(self).write_thrift(output)
    }
}

impl <'i> CompactThriftProtocol<'i> for Box<str> {
    const FIELD_TYPE: u8 = String::FIELD_TYPE;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = input.read_string()?.into_owned().into_boxed_str();
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_string(self.as_ref())
    }
}

impl <'i> CompactThriftProtocol<'i> for Rc<str> {
    const FIELD_TYPE: u8 = String::FIELD_TYPE;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = match input.read_string()? {
            Cow::Borrowed(s) => Rc::from(s),
            Cow::Owned(s) => Rc::from(s),
        };
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_string(self.as_ref())
    }
}

impl <'i> CompactThriftProtocol<'i> for Arc<str> {
    const FIELD_TYPE: u8 = String::FIELD_TYPE;

    #[inline]
    fn fill_thrift<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
        *self = match input.read_string()? {
            Cow::Borrowed(s) => Arc::from(s),
            Cow::Owned(s) => Arc::from(s),
        };
        Ok(())
    }

    #[inline]
    fn write_thrift<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
        output.write_string(self.as_ref())
    }
}
