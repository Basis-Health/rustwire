use crate::{decoders, Variant};

impl From<Variant> for u64 {
    fn from(variant: Variant) -> u64 {
        match variant {
            Variant::Varint => 0,
            Variant::SixtyFourBit => 1,
            Variant::LengthDelimited => 2,
            Variant::ThirtyTwoBit => 5,
        }
    }
}

#[inline]
pub(crate) fn handle_varint(encoded_message: &[u8], offset: usize) -> Option<usize> {
    decoders::decode_varint(encoded_message, offset).map(|(_, new_offset)| new_offset)
}

#[inline]
pub(crate) fn handle_length_delimited(encoded_message: &[u8], offset: usize) -> Option<&[u8]> {
    let (length, offset) = decoders::decode_varint(encoded_message, offset)?;
    let end_offset = offset + length as usize;
    if end_offset > encoded_message.len() {
        return None;
    }
    Some(&encoded_message[offset..end_offset])
}

#[inline]
pub(crate) fn skip_field(encoded_message: &[u8], wire_type: u64, offset: usize) -> Option<usize> {
    match wire_type {
        0 => decoders::decode_varint(encoded_message, offset).map(|(_, new_offset)| new_offset),
        1 => Some(offset + 8),
        2 => {
            let (length, offset) = decoders::decode_varint(encoded_message, offset)?;
            Some(offset + length as usize)
        }
        5 => Some(offset + 4),
        _ => None,
    }
}
