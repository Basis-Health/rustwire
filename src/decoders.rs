pub(crate) fn decode_varint(bytes: &[u8], offset: usize) -> Option<(u64, usize)> {
    let mut result = 0u64;
    let mut shift = 0;
    for i in offset..bytes.len() {
        let byte = bytes[i];
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            return Some((result, i + 1));
        }
        shift += 7;
        if shift >= 64 {
            return None;
        }
    }
    None
}

pub(crate) fn decode_float(encoded_message: &[u8], offset: usize) -> Option<usize> {
    if offset + 4 <= encoded_message.len() {
        Some(offset + 4)
    } else {
        None
    }
}

pub(crate) fn decode_double(encoded_message: &[u8], offset: usize) -> Option<usize> {
    if offset + 8 <= encoded_message.len() {
        Some(offset + 8)
    } else {
        None
    }
}
