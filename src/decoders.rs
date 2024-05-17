pub(crate) fn decode_varint(bytes: &[u8], offset: usize) -> Option<(u64, usize)> {
    let mut result = 0u64;
    let mut shift = 0;
    for (i, byte) in bytes.iter().enumerate().skip(offset) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_varint() {
        let bytes = [0x96, 0x01];
        let (result, new_offset) = decode_varint(&bytes, 0).unwrap();
        assert_eq!(result, 150);
        assert_eq!(new_offset, 2);
    }

    #[test]
    fn test_decode_float() {
        let bytes = [0x00, 0x00, 0x48, 0x40];
        let new_offset = decode_float(&bytes, 0).unwrap();
        assert_eq!(new_offset, 4);
    }

    #[test]
    fn test_decode_double() {
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x40];
        let new_offset = decode_double(&bytes, 0).unwrap();
        assert_eq!(new_offset, 8);
    }
}
