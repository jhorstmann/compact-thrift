use std::error::Error;
use std::ffi::CStr;
use std::ffi::c_char;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::ErrorKind;
use std::io::Error as IOError;

#[derive(Clone,Debug)]
pub enum ThriftError {
    InvalidNumber,
    InvalidString,
    InvalidBinaryLen,
    InvalidCollectionLen,
    MissingField(FieldName),
    MissingValue,
    MissingStop,
    DuplicateField,
    InvalidType,
    ReserveError,
    IO(ErrorKind)
}

/// Store static strings used by field names as a single pointer to reduce size of the error enum.
#[derive(Clone)]
pub struct FieldName {
    name: *const c_char,
}

impl From<&'static CStr> for FieldName {
    fn from(value: &'static CStr) -> Self {
        Self {
            name: value.as_ptr()
        }
    }
}

impl Debug for FieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe { Debug::fmt(CStr::from_ptr(self.name), f) }
    }
}

impl Display for FieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe { Display::fmt(&CStr::from_ptr(self.name).to_string_lossy(), f) }
    }
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
