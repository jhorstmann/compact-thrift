mod error;
pub mod macros;
mod protocol;
mod types;
mod uleb;

pub use error::*;
pub use protocol::*;

#[cfg(test)]
mod tests {
    use crate::{CompactThriftInput, CompactThriftOutput, CompactThriftProtocol, FieldName, skip_field, CompactThriftInputSlice, ThriftError};
    use crate::uleb::decode_uleb;

    #[test]
    fn test_size_of_error() {
        assert_eq!(std::mem::size_of::<ThriftError>(), 16);
        assert_eq!(std::mem::size_of::<Result<(), ThriftError>>(), 16);
    }

    #[test]
    fn test_field_name() {
        assert_eq!(std::mem::size_of::<FieldName>(), std::mem::size_of::<usize>());
        assert_eq!(&FieldName::from(c"foobar").to_string(), "foobar");
    }

    #[test]
    fn test_slice_input_read_byte() {
        let mut input = CompactThriftInputSlice::new(&[1]);
        assert_eq!(input.read_byte().unwrap(), 1);
    }

    #[test]
    fn test_read_uleb() {
        assert_eq!(decode_uleb(&mut CompactThriftInputSlice::new(&[1])).unwrap(), 1);
        assert_eq!(decode_uleb(&mut CompactThriftInputSlice::new(&[0b0111_1111])).unwrap(), 0b0111_1111);
        assert_eq!(decode_uleb(&mut CompactThriftInputSlice::new(&[0b1000_1111, 0b0111_0101])).unwrap(), 0b0011_1010_1000_1111);
    }

    #[test]
    fn test_read_uleb_overlong() {
        decode_uleb(&mut CompactThriftInputSlice::new(&[0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0b1000_0001, 0])).unwrap();
    }

    #[test]
    #[cfg(target_feature = "sse2")]
    fn test_skip_uleb_sse2() {
        use crate::uleb::skip_uleb_sse2;
        {
            let buf = &[0; 16];
            assert_eq!(unsafe { skip_uleb_sse2(buf) }, &buf[1..]);
        }
        {
            let buf = &mut [0; 16];
            buf[1] = 0x81;
            assert_eq!(unsafe { skip_uleb_sse2(buf) }, &buf[1..]);
        }
        {
            let buf = &mut [0; 16];
            buf[0] = 0x80;
            buf[1] = 0x01;
            assert_eq!(unsafe { skip_uleb_sse2(buf) }, &buf[2..]);
        }
    }

    #[test]
    fn test_slice_input_read_i32() {
        assert_eq!(CompactThriftInputSlice::new(&[0]).read_i32().unwrap(), 0);
        assert_eq!(CompactThriftInputSlice::new(&[1]).read_i32().unwrap(), -1);
        assert_eq!(CompactThriftInputSlice::new(&[2]).read_i32().unwrap(), 1);

        assert_eq!(CompactThriftInputSlice::new(&[0b0111_1111]).read_i32().unwrap(), -64);
        assert_eq!(CompactThriftInputSlice::new(&[0b1000_1111, 0b0111_0101]).read_i32().unwrap(), -7496);
        assert_eq!(CompactThriftInputSlice::new(&[0b1000_1111, 0b0111_0101, 0, 0]).read_i32().unwrap(), -7496);
        assert_eq!(CompactThriftInputSlice::new(&[0b1000_1111, 0b0111_0101, 0, 0, 0]).read_i32().unwrap(), -7496);
    }

    #[test]
    fn test_uleb_roundtrip() {
        let mut w = vec![];
        w.write_i64(1234567890).unwrap();
        let mut r = CompactThriftInputSlice::new(&w);
        assert_eq!(r.read_i64().unwrap(), 1234567890);
    }

    #[test]
    fn test_read_vec_bool() {
        let mut data = CompactThriftInputSlice::new(&[0x42, 0, 1, 1, 0]);
        let actual = Vec::<bool>::read(&mut data).unwrap();
        let expected = vec![false, true, true, false];
        assert_eq!(&actual, &expected);

        // also allow element type 1 for boolean
        let mut data = CompactThriftInputSlice::new(&[0x41, 0, 1, 1, 0]);
        let actual = Vec::<bool>::read(&mut data).unwrap();
        let expected = vec![false, true, true, false];
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn test_read_box() {
        let mut data = CompactThriftInputSlice::new(&[0x2]);
        let actual = Box::<i32>::read(&mut data).unwrap();
        let expected = Box::new(1);
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn test_read_option_box() {
        let mut data = CompactThriftInputSlice::new(&[0x2]);
        let actual = Option::<Box::<i32>>::read(&mut data).unwrap();
        let expected = Some(Box::new(1));
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn test_empty_vec_roundtrip() {
        let input = Vec::<i64>::default();
        let mut buffer = vec![];
        input.write(&mut buffer).unwrap();
        let result = Vec::<i64>::read(&mut CompactThriftInputSlice::new(&buffer)).unwrap();
        assert_eq!(&result, &input);
    }

    #[test]
    fn test_vec_bool_roundtrip() {
        let input = vec![true, false, false, true];
        let mut buffer = vec![];
        input.write(&mut buffer).unwrap();
        let result = Vec::<bool>::read(&mut CompactThriftInputSlice::new(&buffer)).unwrap();
        assert_eq!(&result, &input);
    }

    #[test]
    fn test_vec_integer_roundtrip() {
        let input = vec![1_i64, i64::MIN, i64::MAX, 9999999];
        let mut buffer = vec![];
        input.write(&mut buffer).unwrap();
        let result = Vec::<i64>::read(&mut CompactThriftInputSlice::new(&buffer)).unwrap();
        assert_eq!(&result, &input);
    }

    #[test]
    fn test_skip_vec_bool() {
        let input = vec![true, false, false, true];
        let mut buffer = vec![];
        input.write(&mut buffer).unwrap();
        let mut slice = CompactThriftInputSlice::new(&buffer);
        skip_field(&mut slice, Vec::<bool>::FIELD_TYPE, true).unwrap();
        assert_eq!(&slice.as_slice(), &[]);
    }

    #[test]
    fn test_skip_vec_integer() {
        let input = vec![1_i64, 999999999999, -1, i64::MAX];
        let mut buffer = vec![];
        input.write(&mut buffer).unwrap();
        let mut slice = CompactThriftInputSlice::new(&buffer);
        skip_field(&mut slice, Vec::<i64>::FIELD_TYPE, true).unwrap();
        assert_eq!(&slice.as_slice(), &[]);
    }
}