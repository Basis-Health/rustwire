
pub fn extract_field(encoded_message: &[u8], field_number: u32) -> Option<&[u8]> {
    let mut index = 0;
    while index < encoded_message.len() {
        let (field_number_from_wire, wire_type) = extract_field_number_and_wire_type(&encoded_message[index..]);
        let (field_length, varint_length) = extract_field_length(&encoded_message[index + 1..]);

        if field_number_from_wire == field_number {
            let start = index + 1 + varint_length;
            let end = start + field_length as usize;
            return Some(&encoded_message[start..end]);
        }

        index += 1 + varint_length + field_length as usize;
    }

    None
}

pub fn extract_multiple_fields<'a>(encoded_message: &'a[u8], field_numbers: &[u32]) -> Vec<(u32, &'a[u8])> {
    let mut index = 0;
    let mut result = Vec::new();

    while index < encoded_message.len() {
        let (field_number_from_wire, wire_type) = extract_field_number_and_wire_type(&encoded_message[index..]);
        let (field_length, varint_length) = extract_field_length(&encoded_message[index + 1..]);

        if field_numbers.contains(&field_number_from_wire) {
            let start = index + 1 + varint_length;
            let end = start + field_length as usize;
            result.push((field_number_from_wire, &encoded_message[start..end]));
        }

        index += 1 + varint_length + field_length as usize;
    }

    result
}




// TODO: Implement this function, and test whether it's faster than the previous one
pub fn extract_multiple_fields_lazy<'a>(encoded_message: &'a[u8], field_numbers: &[u32]) -> Vec<(u32, &'a[u8])> {
    extract_multiple_fields(encoded_message, field_numbers)
}

fn extract_field_number_and_wire_type(data: &[u8]) -> (u32, u32) {
    let field_number_and_wire_type = data[0];
    let wire_type = field_number_and_wire_type & 0b0000_0111;
    let field_number = field_number_and_wire_type >> 3;
    (field_number as u32, wire_type as u32)
}

fn extract_field_length(data: &[u8]) -> (u32, usize) {
    let mut field_length = 0u32;
    let mut shift = 0;
    let mut index = 0;

    loop {
        let byte = data[index];
        field_length |= ((byte & 0b0111_1111) as u32) << shift;
        shift += 7;
        index += 1;

        if byte & 0b1000_0000 == 0 {
            break;
        }
    }

    (field_length, index)
}

fn get_varint_length(value: u32) -> usize {
    match value {
        0..=0x7f => 1,
        0x80..=0x3fff => 2,
        0x4000..=0x1fffff => 3,
        0x200000..=0xfffffff => 4,
        _ => 5,
    }
}

#[cfg(test)]
mod tests {
    use prost::Message;
    use super::*;

    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeviceInfo {
        #[prost(string, tag = "1")]
        pub uid: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub source_id: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub meta_source: ::prost::alloc::string::String,
        #[prost(string, optional, tag = "10")]
        pub device_id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "11")]
        pub source_name: ::core::option::Option<::prost::alloc::string::String>,
    }

    fn device_info() -> DeviceInfo {
        DeviceInfo {
            uid: "uid".to_string(),
            source_id: "source_id".to_string(),
            meta_source: "meta_source".to_string(),
            device_id: Some("device_id".to_string()),
            source_name: Some("source_name".to_string()),
        }
    }

    #[test]
    fn test_extract_field() {
        let device_info = device_info();
        let encoded = device_info.encode_to_vec();
        let field = extract_field(&encoded, 1).unwrap();
        assert_eq!(field, b"uid");
    }

    #[test]
    fn test_extract_multiple_fields() {
        let device_info = device_info();
        let encoded = device_info.encode_to_vec();
        let fields = extract_multiple_fields(&encoded, &[1, 2]);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"uid");
        assert_eq!(fields[1].0, 2);
        assert_eq!(fields[1].1, b"source_id");
    }

    #[test]
    fn test_replace_field() {
        let mut device_info = device_info();
        let mut encoded = device_info.encode_to_vec();
        replace_field(&mut encoded, 1, b"new_uid");
        let new_device_info = DeviceInfo::decode(&*encoded).unwrap();
        assert_eq!(new_device_info.uid, "new_uid");
        assert_eq!(new_device_info.source_id, "source_id");
    }

    #[test]
    fn test_replace_multiple_fields() {
        let mut device_info = device_info();
        let mut encoded = device_info.encode_to_vec();
        let mut field_values: Vec<(u32, &[u8])> = vec![(1, b"new_uid"), (2, b"new_source_id")];
        replace_multiple_fields(&mut encoded, &mut field_values);
        let new_device_info = DeviceInfo::decode(&*encoded).unwrap();
        assert_eq!(new_device_info.uid, "new_uid");
        assert_eq!(new_device_info.source_id, "new_source_id");
    }
}