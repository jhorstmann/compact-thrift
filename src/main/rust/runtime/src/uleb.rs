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

/// Safety: `input` needs to contains at least 16 bytes
#[target_feature(enable="sse2")]
#[inline]
pub(crate) unsafe fn skip_uleb_sse2(input: &[u8]) -> &[u8] {
    use std::arch::x86_64::*;
    debug_assert!(input.len() >= 16);
    let chunk = _mm_loadu_si128(input.as_ptr().cast());
    let mask = !_mm_movemask_epi8(chunk);
    let len = (mask << 1).trailing_zeros() as usize;
    input.get_unchecked(len..)
}

#[inline(never)]
pub(crate) fn skip_uleb_fallback(input: &[u8]) -> &[u8] {
    let mut i = 0_usize;
    while i < input.len() && (input[i] & 0x80) != 0 {
        i += 1;
    }
    &input[i+1..]
}

pub(crate) fn encode_uleb<O: CompactThriftOutput + ?Sized>(output: &mut O, mut value: u64) -> Result<(), ThriftError> {
    while value > 0x7F {
        output.write_byte((value as u8) | 0x80)?;
        value >>= 7;
    }
    output.write_byte(value as u8)
}