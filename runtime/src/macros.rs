#[macro_export]
macro_rules! thrift {
    ($(#[$($def_attrs:tt)*])* struct $identifier:ident { $($definitions:tt)* } $($remainder:tt)*) => {
        $crate::thrift_struct!($(#[$($def_attrs)*])* struct $identifier { $($definitions)* });
        $crate::thrift!($($remainder)*);
    };
    ($(#[$($def_attrs:tt)*])* union $identifier:ident { $($definitions:tt)* } $($remainder:tt)*) => {
        $crate::thrift_union!($(#[$($def_attrs)*])* union $identifier { $($definitions)* });
        $crate::thrift!($($remainder)*);
    };
    ($(#[$($def_attrs:tt)*])* enum $identifier:ident { $($definitions:tt)* } $($remainder:tt)*) => {
        $crate::thrift_enum!($(#[$($def_attrs)*])* enum $identifier { $($definitions)* });
        $crate::thrift!($($remainder)*);
    };
    ($(#[$($def_attrs:tt)*])* namespace $identifier:ident $namespace:ident $($remainder:tt)*) => {
        $crate::thrift!($($remainder)*);
    };

    () => {};
}

#[macro_export]
macro_rules! thrift_struct {
    ($(#[$($def_attrs:tt)*])* struct $identifier:ident { $($(#[$($field_attrs:tt)*])* $field_id:literal : $required_or_optional:ident $field_type:ident $(< $element_type:ident >)? $field_name:ident $(= $default_value:literal)? $(;)?)* }) => {
        $(#[cfg_attr(not(doctest), $($def_attrs)*)])*
        #[derive(Default, Clone, Debug, PartialEq)]
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        pub struct $identifier {
            $($(#[cfg_attr(not(doctest), $($field_attrs)*)])* pub $field_name: $crate::required_or_optional!($required_or_optional $crate::field_type!($field_type $($element_type)?))),*
        }

        impl $identifier {
            #[allow(clippy::too_many_arguments)]
            pub fn new($($field_name: impl Into<$crate::required_or_optional!($required_or_optional $crate::field_type!($field_type $($element_type)?))>),*) -> Self {
                Self {
                    $($field_name: $field_name.into(),)*
                }
            }
        }

        #[allow(non_snake_case)]
        impl <'i> $crate::CompactThriftProtocol<'i> for $identifier {
            const FIELD_TYPE: u8 = 12;

            #[inline(never)]
            fn fill_thrift<T: $crate::CompactThriftInput<'i>>(&mut self, input: &mut T) -> std::result::Result<(), $crate::ThriftError> {
                let mut last_field_id = 0_i16;
                $($crate::required_flag!($required_or_optional $field_name);)*
                loop {
                    let field_type = input.read_field_header(&mut last_field_id)?;
                    if field_type == 0 {
                        break;
                    }

                    match last_field_id {
                        $($field_id => {
                            $crate::required_set!($required_or_optional $field_name);
                            self.$field_name.fill_thrift_field(input, field_type)?;
                        }),*
                        _ => {
                            input.skip_field(field_type)?;
                        }
                    }
                }

                $($crate::required_check!($required_or_optional $identifier $field_name);)*

                Ok(())
            }

            fn write_thrift<T: $crate::CompactThriftOutput>(&self, output: &mut T) -> std::result::Result<(), $crate::ThriftError> {
                #[allow(unused_variables)]
                #[allow(unused_mut)]
                let mut last_field_id = 0_i16;
                $(self.$field_name.write_thrift_field(output, $field_id, &mut last_field_id)?;)*
                output.write_byte(0)?;
                Ok(())
            }
        }
    }
}

#[macro_export]
macro_rules! thrift_union {
    ($(#[$($def_attrs:tt)*])* union $identifier:ident { $($(#[$($field_attrss:tt)*])* $field_id:literal : $field_type:ident $(< $element_type:ident >)? $field_name:ident $(;)?)* }) => {
        $(#[cfg_attr(not(doctest), $($def_attrs)*)])*
        #[derive(Clone, Debug, PartialEq)]
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        pub enum $identifier {
            $($(#[cfg_attr(not(doctest), $($field_attrss)*)])* $field_name($crate::field_type!($field_type $($element_type)?))),*
        }

        impl Default for $identifier {
            fn default() -> Self {
                $crate::union_default!($($field_name;)*)
            }
        }

        #[allow(non_snake_case)]
        impl <'i> $crate::CompactThriftProtocol<'i> for $identifier {
            const FIELD_TYPE: u8 = 12;

            #[inline(never)]
            fn fill_thrift<T: $crate::CompactThriftInput<'i>>(&mut self, input: &mut T) -> std::result::Result<(), $crate::ThriftError> {
                let mut last_field_id = 0_i16;
                let field_type = input.read_field_header(&mut last_field_id)?;

                if field_type == 0 {
                    return Err($crate::ThriftError::InvalidType);
                }

                match last_field_id {
                    $($field_id => {
                        *self = Self::$field_name(Default::default());
                        #[allow(unreachable_patterns)]
                        match self {
                            Self::$field_name(inner) => inner.fill_thrift(input)?,
                            _ => unsafe { std::hint::unreachable_unchecked() },
                        }
                    }),*
                    _ => {
                        return Err($crate::ThriftError::MissingField(concat!(stringify!($struct_name), "\0").into()))
                    }
                }
                let stop = input.read_byte()?;
                if stop != 0 {
                    return Err($crate::ThriftError::MissingStop)
                }

                Ok(())
            }

            fn write_thrift<T: $crate::CompactThriftOutput>(&self, output: &mut T) -> std::result::Result<(), $crate::ThriftError> {
                let mut last_field_id = 0_i16;
                match self {
                    $(Self::$field_name(inner) => inner.write_thrift_field(output, $field_id, &mut last_field_id)?),*
                }
                output.write_byte(0)?;
                Ok(())
            }
        }
    }
}

#[macro_export]
macro_rules! thrift_enum {
    ($(#[$($def_attrs:tt)*])* enum $identifier:ident { $($(#[$($field_attrss:tt)*])* $field_name:ident = $field_value:literal;)* }) => {
        $(#[$($def_attrs)*])*
        #[derive(Default, Debug, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[allow(non_camel_case_types)]
        pub struct $identifier(pub i32);

        impl From<i32> for $identifier {
            #[inline]
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl $identifier {
            $(pub const $field_name: Self = Self($field_value);)*
        }

        impl <'i> $crate::CompactThriftProtocol<'i> for $identifier {
            const FIELD_TYPE: u8 = 5; // i32

            #[inline]
            fn fill_thrift<T: $crate::CompactThriftInput<'i>>(&mut self, input: &mut T) -> std::result::Result<(), $crate::ThriftError> {
                self.0 = input.read_i32()?;
                Ok(())
            }

            #[inline]
            fn write_thrift<T: $crate::CompactThriftOutput>(&self, output: &mut T) -> std::result::Result<(), $crate::ThriftError> {
                output.write_i32(self.0)
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! union_default {
    ($head:ident; $($tail:ident;)*) => {
        Self::$head(Default::default())
    };
    () => {
        Self
    };
}

#[doc(hidden)]
#[macro_export]

macro_rules! field_type {
    (list $element_type:ident) => { std::vec::Vec< $crate::field_type!($element_type) > };
    (set $element_type:ident) => { std::vec::Vec< $crate::field_type!($element_type) > };
    (binary) => { std::vec::Vec<u8> };
    (string) => { std::string::String };
    ($field_type:ty) => { $field_type };
    (Box $element_type:ident) => { std::boxed::Box< $crate::field_type!($element_type) > };
    (Rc $element_type:ident) => { std::rc::Rc< $crate::field_type!($element_type) > };
    (Arc $element_type:ident) => { std::sync::Arc< $crate::field_type!($element_type) > };
}

#[doc(hidden)]
#[macro_export]
macro_rules! required_or_optional {
    (required $field_type:ty) => { $field_type };
    (optional $field_type:ty) => { Option<$field_type> };
}

#[doc(hidden)]
#[macro_export]
macro_rules! required_flag {
    (required $field_name:ident) => { let mut $field_name = false; };
    (optional $field_name:ident) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! required_set {
    (required $field_name:ident) => { $field_name = true; };
    (optional $field_name:ident) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! required_check {
    (required $struct_name:ident $field_name:ident) => {
        if !$field_name {
            return Err($crate::ThriftError::MissingField(concat!(stringify!($struct_name), "::", stringify!($field_name), "\0").into()))
        }
    };
    (optional $struct_name:ident $field_name:ident) => {};
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    thrift! {
        /** doc */
        struct SomeStructure {
            /** doc */
            1: required i64 offset;
            2: optional i64 length;
            3: optional list<i64> foobar;
            4: optional string data;
        }
        struct AnotherStructure {
            1: required i64 foobar;
        }

        struct MilliSeconds {}
        struct MicroSeconds {}
        struct NanoSeconds {}
        union TimeUnit {
          1: MilliSeconds MILLIS
          2: MicroSeconds MICROS
          3: NanoSeconds NANOS
        }
        enum Type {
          BOOLEAN = 0;
          INT32 = 1;
          INT64 = 2;
          INT96 = 3;  // deprecated, only used by legacy implementations.
          FLOAT = 4;
          DOUBLE = 5;
          BYTE_ARRAY = 6;
          FIXED_LEN_BYTE_ARRAY = 7;
        }
        enum CompressionCodec {
          UNCOMPRESSED = 0;
          SNAPPY = 1;
          GZIP = 2;
          LZO = 3;
          BROTLI = 4;  // Added in 2.4
          LZ4 = 5;     // DEPRECATED (Added in 2.4)
          ZSTD = 6;    // Added in 2.4
          LZ4_RAW = 7; // Added in 2.9
        }
    }

    thrift! {
        struct ReferenceCounted {
            1: required Rc<String> rc_string;
            2: required Arc<String> arc_string;
            3: required Rc<str> rc_str;
            4: required Arc<str> arc_str;
        }
    }

    #[test]
    pub fn test_constructor() {
        let _s = SomeStructure::new(1_i64, 2_i64, Some(vec![3_i64]), Some("foo".into()));
        let _r = ReferenceCounted::default();
    }
}