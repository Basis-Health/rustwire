#![feature(test)]

extern crate test;

use prost::Message;
use rustwire::{extract_field_by_tag, Variant};
use test::Bencher;

#[derive(Clone, PartialEq, Message)]
pub struct ThisMessage {
    #[prost(int32, tag = "1")]
    pub field1: i32,
    #[prost(string, tag = "2")]
    pub field2: String,
    #[prost(uint64, tag = "3")]
    pub field3: u64,
    #[prost(bool, tag = "4")]
    pub field4: bool,
    #[prost(float, tag = "5")]
    pub field5: f32,
    #[prost(double, tag = "6")]
    pub field6: f64,
    #[prost(bytes, tag = "7")]
    pub field7: Vec<u8>,
    #[prost(sint32, tag = "8")]
    pub field8: i32,
    #[prost(fixed32, tag = "9")]
    pub field9: u32,
    #[prost(sfixed32, tag = "10")]
    pub field10: i32,
}

pub fn default_message() -> ThisMessage {
    let default_string = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".repeat(10);
    let mut default_bytes = Vec::new();
    for i in 0..3397 {
        default_bytes.push(i as u8);
    }

    ThisMessage {
        field1: 42,
        field2: default_string,
        field3: 1234567890,
        field4: true,
        field5: 3.14,
        field6: 2.71828,
        field7: default_bytes,
        field8: -123,
        field9: 987654321,
        field10: -987654321,
    }
}

fn prost_extraction(encoded_message: &[u8]) -> Option<i32> {
    ThisMessage::decode(encoded_message)
        .ok()
        .map(|msg| msg.field8)
}

// Benchmark for extracting a field using rustwire
#[bench]
fn bench_rustwire_extraction(b: &mut Bencher) {
    let encoded_message = default_message().encode_to_vec();
    let tag_number = 8;

    b.iter(|| {
        let result = extract_field_by_tag(&encoded_message, tag_number).unwrap();
        let result = test::black_box(result);
    });
}

// Benchmark for deserializing the message and getting the field manually
#[bench]
fn bench_prost_extraction(b: &mut Bencher) {
    let encoded_message = default_message().encode_to_vec();

    b.iter(|| {
        let result = prost_extraction(&encoded_message).unwrap();
        test::black_box(result);
    });
}
