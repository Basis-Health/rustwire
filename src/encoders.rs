/// Encodes a 64-bit unsigned integer (`u64`) into its varint representation.
///
/// Varints are a compact encoding scheme for integers that uses a variable number of bytes
/// depending on the magnitude of the value. Smaller values require fewer bytes for encoding.
///
/// The encoding process follows these steps:
/// 1. Take the least significant 7 bits of the value and set the most significant bit to 0.
/// 2. If there are remaining bits in the value, set the most significant bit of the current byte to 1.
/// 3. Append the current byte to the buffer.
/// 4. Shift the value right by 7 bits.
/// 5. Repeat steps 1-4 until the value becomes 0.
///
/// # Arguments
///
/// * `value` - The `u64` value to be encoded as a varint.
///
/// # Returns
///
/// A `Vec<u8>` containing the varint-encoded bytes of the input value.
///
/// # Example
///
/// ```
/// use rustwire::encode_varint;
///
/// let value: u64 = 42;
/// let encoded = encode_varint(value);
/// assert_eq!(encoded, vec![0x2A]);
/// ```
pub fn encode_varint(value: u64) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut value = value;

    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;

        if value != 0 {
            byte |= 0x80;
        }

        buffer.push(byte);

        if value == 0 {
            break;
        }
    }

    buffer
}

/// Encodes a single-precision floating-point number (`f32`) into its binary representation.
///
/// The encoding process converts the `f32` value into its little-endian byte representation
/// and appends the bytes to a buffer.
///
/// # Arguments
///
/// * `value` - The `f32` value to be encoded.
///
/// # Returns
///
/// A `Vec<u8>` containing the binary representation of the input `f32` value.
///
/// # Example
///
/// ```
/// use rustwire::encode_float;
///
/// let value: f32 = 3.14;
/// let encoded = encode_float(value);
/// assert_eq!(encoded, vec![0xC3, 0xF5, 0x48, 0x40]);
/// ```
pub fn encode_float(value: f32) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&value.to_le_bytes());
    buffer
}

/// Encodes a double-precision floating-point number (`f64`) into its binary representation.
///
/// The encoding process converts the `f64` value into its little-endian byte representation
/// and appends the bytes to a buffer.
///
/// # Arguments
///
/// * `value` - The `f64` value to be encoded.
///
/// # Returns
///
/// A `Vec<u8>` containing the binary representation of the input `f64` value.
///
/// # Example
///
/// ```
/// use rustwire::encode_double;
///
/// let value: f64 = 2.71828;
/// let encoded = encode_double(value);
/// assert_eq!(encoded, vec![0x90, 0xF7, 0xAA, 0x95, 0x9, 0xBF, 0x5, 0x40]);
/// ```
pub fn encode_double(value: f64) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&value.to_le_bytes());
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_varint() {
        let value = 150;
        let bytes = encode_varint(value);
        assert_eq!(bytes, [0x96, 0x01]);
    }

    #[test]
    fn test_encode_varint_large() {
        let value = 624485;
        let bytes = encode_varint(value);
        assert_eq!(bytes, [0xE5, 0x8E, 0x26]);
    }

    #[test]
    fn test_encode_float() {
        let value = 3.14;
        let bytes = encode_float(value);
        assert_eq!(bytes, [0xC3, 0xF5, 0x48, 0x40]);
    }

    #[test]
    fn test_encode_double() {
        let value = 3.14;
        let bytes = encode_double(value);
        assert_eq!(bytes, [0x1F, 0x85, 0xEB, 0x51, 0xB8, 0x1E, 0x09, 0x40]);
    }
}
