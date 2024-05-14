//! # Rustwire (encoded protobuf utils)
//!
//! This crate provides a set of utilities for working with Protocol Buffers (protobuf) in Rust.
//! It offers functions and types to manipulate encoded buffer messages efficiently.
//!
//! ## Features
//!
//! - Extract fields from an encoded protocol buffer message by tag number.
//! - Replace fields in an encoded protocol buffer message.
//! - Create headers for protocol buffer fields.
//! - Support for various wire types: varint, 64-bit, length-delimited, and 32-bit.
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! rustwire = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! Here's a quick example of how to use the `protobuf-utils` crate:
//!
//! ```rust
//! use rustwire::{extract_field_by_tag, replace_field_with, create_header, Variant};
//!
//! // Extract a field from an encoded message
//! let encoded_message = b"\x08\x01\x12\x07\x74\x65\x73\x74\x69\x6e\x67";
//! let tag_number = 2;
//! match extract_field_by_tag(encoded_message, tag_number) {
//!     Some(field_value) => println!("Field value: {:?}", field_value),
//!     None => println!("Field not found"),
//! }
//!
//! // Replace a field in an encoded message
//! let mut encoded_message = b"\x08\x01\x12\x07\x74\x65\x73\x74\x69\x6e\x67".to_vec();
//! let tag_number = 2;
//! let replace_with = b"Hello";
//! match replace_field_with(&mut encoded_message, tag_number, replace_with) {
//!     Some(old_value) => println!("Replaced field value: {:?}", old_value),
//!     None => println!("Field not found or error occurred"),
//! }
//!
//! // Create a header for a field
//! let tag_number = 1;
//! let variant = Variant::LengthDelimited;
//! let encoded_message = b"Hello, world!";
//! let header = create_header(tag_number, variant.into(), encoded_message);
//! let encoded_field = [&header[..], encoded_message].concat();
//! println!("Encoded field: {:?}", encoded_field);
//! ```
//!
//! For more detailed information and examples, please refer to the individual function and type
//! documentation.
//!
//! ## Contributing
//!
//! Contributions are welcome! If you find any issues or have suggestions for improvement, please
//! open an issue or submit a pull request on the [GitHub repository](https://github.com/Basis-Health/rustwire).
//!
//! ## License
//!
//! This crate is licensed under the [MIT License](https://opensource.org/licenses/MIT).

mod decoders;
mod tests;

/// Extracts a field with the given tag number from an encoded protobuf message.
///
/// This function iterates over the encoded message and searches for a field with the specified tag number.
/// If the field is found, its value is extracted and returned as a byte vector (`Vec<u8>`).
///
/// The function supports the following wire types:
/// - Varint (wire type 0)
/// - Length-delimited (wire type 2)
///
/// If the field is not found or if an error occurs during decoding, `None` is returned.
///
/// # Arguments
///
/// * `encoded_message` - A byte slice (`&[u8]`) containing the encoded protobuf message.
/// * `tag_number` - The tag number of the field to extract.
///
/// # Returns
///
/// * `Option<&[u8]>` - If the field is found, its value is returned as `Some(&[u8])`.
///                      If the field is not found or an error occurs, `None` is returned.
///
/// # Examples
///
/// ```
/// use rustwire::extract_field_by_tag;
///
/// let encoded_message = b"\x08\x01\x12\x07\x74\x65\x73\x74\x69\x6e\x67";
/// let tag_number = 2;
///
/// match extract_field_by_tag(encoded_message, tag_number) {
///     Some(field_value) => println!("Field value: {:?}", field_value),
///     None => println!("Field not found or invalid encoded message"),
/// }
/// ```
///
/// # Note
///
/// This function assumes a basic understanding of the protobuf encoding format and wire types.
/// It may need to be adapted to handle more complex field types or nested messages.
pub fn extract_field_by_tag(encoded_message: &[u8], tag_number: u64) -> Option<&[u8]> {
    let mut offset = 0;
    while offset < encoded_message.len() {
        let (tag, new_offset) = decoders::decode_varint(encoded_message, offset)?;
        offset = new_offset;

        let field_number = tag >> 3;
        let wire_type = tag & 0x07;

        if field_number == tag_number {
            return match wire_type {
                0 => decoders::decode_varint(encoded_message, offset).map(|(_, new_offset)| {
                    let varint_slice = &encoded_message[offset..new_offset];
                    offset = new_offset;
                    varint_slice
                }),
                1 => decoders::decode_double(encoded_message, offset).map(|new_offset| {
                    let double_slice = &encoded_message[offset..new_offset];
                    offset = new_offset;
                    double_slice
                }),
                2 => handle_length_delimited(encoded_message, offset),
                5 => decoders::decode_float(encoded_message, offset).map(|new_offset| {
                    let fixed32_slice = &encoded_message[offset..new_offset];
                    offset = new_offset;
                    fixed32_slice
                }),
                _ => None,
            };
        } else {
            offset = skip_field(encoded_message, wire_type, offset)?;
        }
    }
    None
}

/// Extracts multiple fields with the given tag numbers from an encoded protobuf message.
///
/// This function iterates over the encoded message and searches for fields with the specified tag numbers.
/// If a field is found and its tag number is present in the `tag_numbers` slice, its value is extracted
/// and stored in the returned vector of tuples.
///
/// The function supports the following wire types:
/// - Varint (wire type 0)
/// - Length-delimited (wire type 2)
///
/// If a field is not found or if an error occurs during decoding, it is skipped, and the function continues
/// processing the remaining fields.
///
/// # Arguments
///
/// * `encoded_message` - A byte slice (`&[u8]`) containing the encoded protobuf message.
/// * `tag_numbers` - A slice of `u64` values representing the tag numbers of the fields to extract.
///
/// # Returns
///
/// A vector of tuples `Vec<(u64, &[u8])>`, where each tuple contains the tag number and the corresponding
/// field value as a byte vector reference. The order of the tuples in the vector corresponds to the order of the
/// tag numbers in the `tag_numbers` slice.
///
/// # Examples
///
/// ```
/// use rustwire::extract_multiple_fields_by_tag;
///
/// let encoded_message = b"\x08\x01\x12\x07\x74\x65\x73\x74\x69\x6e\x67\x1a\x03\x61\x62\x63";
/// let tag_numbers = &[2, 3];
///
/// let fields = extract_multiple_fields_by_tag(encoded_message, tag_numbers);
/// for (tag_number, field_value) in fields {
///     println!("Tag: {}, Value: {:?}", tag_number, field_value);
/// }
/// ```
///
/// # Note
///
/// This function assumes a basic understanding of the protobuf encoding format and wire types.
/// It may need to be adapted to handle more complex field types or nested messages.
pub fn extract_multiple_fields_by_tag<'a>(
    encoded_message: &'a [u8],
    tag_numbers: &[u64],
) -> Vec<(u64, &'a [u8])> {
    let mut fields = Vec::new();
    let mut offset = 0;

    while offset < encoded_message.len() {
        let (tag, new_offset) = match decoders::decode_varint(encoded_message, offset) {
            Some((tag, new_offset)) => (tag, new_offset),
            None => break,
        };
        offset = new_offset;

        let field_number = tag >> 3;
        let wire_type = tag & 0x07;

        if tag_numbers.contains(&field_number) {
            let field_value = match wire_type {
                0 => handle_varint(encoded_message, offset).map(|new_offset| {
                    let value = &encoded_message[offset..new_offset];
                    offset = new_offset;
                    value
                }),
                1 => decoders::decode_double(encoded_message, offset).map(|new_offset| {
                    let double_slice = &encoded_message[offset..new_offset];
                    offset = new_offset;
                    double_slice
                }),
                2 => handle_length_delimited(encoded_message, offset).map(|value| {
                    offset += value.len() + 1; // Skip the length prefix
                    value
                }),
                5 => decoders::decode_float(encoded_message, offset).map(|new_offset| {
                    let float_slice = &encoded_message[offset..new_offset];
                    offset = new_offset;
                    float_slice
                }),
                _ => None,
            };

            if let Some(value) = field_value {
                fields.push((field_number, value));
            }
        } else {
            offset = match skip_field(encoded_message, wire_type, offset) {
                Some(new_offset) => new_offset,
                None => break,
            };
        }
    }

    fields
}

/// Replaces a field with the specified tag number in the encoded message with the given replacement data.
///
/// This function modifies the `encoded_message` in-place and returns the old field value as an `Option<Vec<u8>>`.
///
/// # Arguments
///
/// * `encoded_message` - A mutable reference to a `Vec<u8>` containing the encoded message.
/// * `tag_number` - The tag number of the field to replace.
/// * `replace_with` - A byte slice (`&[u8]`) containing the replacement data.
///
/// # Returns
///
/// * `Option<Vec<u8>>` - If the field is found and replaced successfully, returns `Some(old_value)`, where `old_value` is the original field value as a `Vec<u8>`. If the field is not found or an error occurs, returns `None`.
///
/// # Example
///
/// ```rust
/// use rustwire::replace_field_with;
///
/// let mut encoded_message = vec![0x08, 0x01, 0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
/// let tag_number = 2;
/// let replace_with = b"Hello";
///
/// match replace_field_with(&mut encoded_message, tag_number, replace_with) {
///     Some(old_value) => println!("Replaced field value: {:?}", old_value),
///     None => println!("Field not found or error occurred"),
/// }
/// ```
///
/// # Notes
///
/// - This function modifies the `encoded_message` in-place.
/// - The function currently creates a copy of the encoded message during the replacement process. It would be more efficient to overwrite the existing data directly.
/// - The function supports the following wire types:
///   - Varint (wire type 0)
///   - 64-bit (wire type 1)
///   - Length-delimited (wire type 2)
///   - 32-bit (wire type 5)
/// - If the wire type is not supported, the function returns `None`.
pub fn replace_field_with(
    encoded_message: &mut Vec<u8>,
    tag_number: u64,
    replace_with: &[u8],
) -> Option<Vec<u8>> {
    let mut offset = 0;
    while offset < encoded_message.len() {
        let old_offset = offset;
        let (tag, new_offset) = decoders::decode_varint(encoded_message, offset)?;
        offset = new_offset;

        let field_number = tag >> 3;
        let wire_type = tag & 0x07;

        if field_number == tag_number {
            let old = match wire_type {
                0 => decoders::decode_varint(encoded_message, offset).map(|(_, new_offset)| {
                    let varint_slice = &encoded_message[offset..new_offset];
                    offset = new_offset;
                    varint_slice
                }),
                1 => decoders::decode_double(encoded_message, offset).map(|new_offset| {
                    let double_slice = &encoded_message[offset..new_offset];
                    offset = new_offset;
                    double_slice
                }),
                2 => handle_length_delimited(encoded_message, offset).map(|value| {
                    offset += value.len() + 1; // Skip the length prefix
                    value
                }),
                5 => decoders::decode_float(encoded_message, offset).map(|new_offset| {
                    let fixed32_slice = &encoded_message[offset..new_offset];
                    offset = new_offset;
                    fixed32_slice
                }),
                _ => None,
            }
            .map(|old| old.to_vec());

            // create two regsions, pre old_offset and post offset
            let pre_slice = &encoded_message[..old_offset];
            let post_slice = &encoded_message[offset..];

            // TODO: This is creating a copy right now, it would be better if it would just overwrite

            // create a new vec with the pre_slice, the replace_with and the post_slice
            let new_len = pre_slice.len() + replace_with.len() + post_slice.len();
            let mut new_encoded_message = Vec::with_capacity(new_len);
            new_encoded_message.extend_from_slice(pre_slice);
            new_encoded_message.extend_from_slice(replace_with);
            new_encoded_message.extend_from_slice(post_slice);

            // TODO: This is creating a copy right now, it would be better if it would just overwrite
            encoded_message.clear();
            encoded_message.extend_from_slice(&new_encoded_message);

            return old;
        } else {
            offset = skip_field(encoded_message, wire_type, offset)?;
        }
    }
    None
}

/// Creates the header for a field in a protocol buffer message.
///
/// The header consists of the tag number, wire type variant, and the length of the encoded message
/// (if applicable). The function returns the header as a `Vec<u8>` without copying the encoded message.
///
/// # Arguments
///
/// * `tag_number` - The tag number of the field.
/// * `variant` - The wire type variant of the field. Valid values are:
///   - 0: Varint
///   - 1: 64-bit
///   - 2: Length-delimited
///   - 5: 32-bit
/// * `encoded_message` - A reference to the encoded message as a byte slice (`&[u8]`).
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded header bytes.
///
/// # Examples
///
/// ```
/// let tag_number = 1;
/// let variant = 2; // Length-delimited
/// let encoded_message = b"Hello, world!";
///
/// let header = create_header(tag_number, variant, encoded_message);
/// let encoded_field = [&header[..], encoded_message].concat();
///
/// println!("Encoded field: {:?}", encoded_field);
/// ```
///
/// In this example, the `create_header` function is called with the `tag_number`, `variant`, and
/// `encoded_message`. The resulting `header` is then concatenated with the `encoded_message` using
/// the `concat` method to form the complete encoded field. The encoded field is then printed.
///
/// # Notes
///
/// - The function does not copy the `encoded_message`. It only creates the header bytes based on the
///   provided `tag_number`, `variant`, and the length of the `encoded_message` (if applicable).
/// - The header bytes are encoded using base 128 varint encoding.
/// - If the `variant` is 2 (length-delimited), the length of the `encoded_message` is encoded as part
///   of the header.
pub fn create_header(tag_number: u64, variant: u64, encoded_message: &[u8]) -> Vec<u8> {
    let mut header = Vec::new();

    // Create the tag byte
    let tag_byte = (tag_number << 3) | variant;

    // Encode the tag byte using base 128 varint encoding
    let mut current = tag_byte;
    loop {
        if current < 128 {
            header.push(current as u8);
            break;
        } else {
            header.push(((current & 0x7F) | 0x80) as u8);
            current >>= 7;
        }
    }

    // If the variant is length-delimited (2), encode the length of the message
    if variant == 2 {
        let length = encoded_message.len() as u64;
        let mut current = length;
        loop {
            if current < 128 {
                header.push(current as u8);
                break;
            } else {
                header.push(((current & 0x7F) | 0x80) as u8);
                current >>= 7;
            }
        }
    }

    header
}

/// Represents the wire type variant of a field in a protocol buffer message.
///
/// The `Variant` enum provides a set of predefined wire types that can be used when creating
/// the header for a field. Each variant corresponds to a specific wire type value.
///
/// # Variants
///
/// * `Varint` - Represents the varint wire type (value 0).
/// * `SixtyFourBit` - Represents the 64-bit wire type (value 1).
/// * `LengthDelimited` - Represents the length-delimited wire type (value 2).
/// * `ThirtyTwoBit` - Represents the 32-bit wire type (value 5).
///
/// # Conversions
///
/// The `Variant` enum implements the `Into<u64>` trait, allowing conversion from a `Variant`
/// to its corresponding wire type value as a `u64`.
///
/// # Examples
///
/// ```
/// use Variant;
///
/// let variant = Variant::LengthDelimited;
/// let wire_type_value: u64 = variant.into();
/// assert_eq!(wire_type_value, 2);
/// ```
///
/// In this example, the `LengthDelimited` variant is created and then converted into its
/// corresponding wire type value using the `into()` method. The resulting `wire_type_value`
/// is of type `u64` and has a value of `2`.
#[derive(Debug, PartialEq, Eq)]
pub enum Variant {
    Varint,
    SixtyFourBit,
    LengthDelimited,
    ThirtyTwoBit,
}

impl Into<u64> for Variant {
    fn into(self) -> u64 {
        match self {
            Variant::Varint => 0,
            Variant::SixtyFourBit => 1,
            Variant::LengthDelimited => 2,
            Variant::ThirtyTwoBit => 5,
        }
    }
}

#[inline]
fn handle_varint(encoded_message: &[u8], offset: usize) -> Option<usize> {
    decoders::decode_varint(encoded_message, offset).map(|(_, new_offset)| new_offset)
}

#[inline]
fn handle_length_delimited(encoded_message: &[u8], offset: usize) -> Option<&[u8]> {
    let (length, offset) = decoders::decode_varint(encoded_message, offset)?;
    let end_offset = offset + length as usize;
    if end_offset > encoded_message.len() {
        return None;
    }
    Some(&encoded_message[offset..end_offset])
}

#[inline]
fn skip_field(encoded_message: &[u8], wire_type: u64, offset: usize) -> Option<usize> {
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
