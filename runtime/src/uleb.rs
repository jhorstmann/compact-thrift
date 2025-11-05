use crate::CompactThriftInput;
use crate::CompactThriftOutput;
use crate::ThriftError;

pub(crate) fn decode_uleb<'i, I: CompactThriftInput<'i> + ?Sized>(input: &mut I) -> Result<u64, ThriftError> {
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

pub(crate) fn encode_uleb<O: CompactThriftOutput + ?Sized>(output: &mut O, mut value: u64) -> Result<(), ThriftError> {
    while value > 0x7F {
        output.write_byte((value as u8) | 0x80)?;
        value >>= 7;
    }
    output.write_byte(value as u8)
}