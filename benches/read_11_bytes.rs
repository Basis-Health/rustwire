#![feature(test)]

extern crate test;

use prost::Message;
use rustwire::{extract_field_by_tag, Variant};
use test::Bencher;

#[derive(Message)]
pub struct SimpleMessage {
    #[prost(int32, tag = "1")]
    pub field1: i32,
    #[prost(string, tag = "2")]
    pub field2: ::prost::alloc::string::String,
}

fn prost_extraction(encoded_message: &[u8]) -> Option<String> {
    SimpleMessage::decode(encoded_message)
        .ok()
        .map(|msg| msg.field2)
}

// Benchmark for extracting a field using rustwire
#[bench]
fn bench_rustwire_extraction(b: &mut Bencher) {
    let encoded_message = b"\x08\x01\x12\x07\x74\x65\x73\x74\x69\x6e\x67";
    let tag_number = 2;

    b.iter(|| {
        let result = extract_field_by_tag(encoded_message, tag_number).unwrap();
        let result = String::from_utf8(result.to_vec()).unwrap();
        test::black_box(result);
    });
}

// Benchmark for deserializing the message and getting the field manually
#[bench]
fn bench_prost_extraction(b: &mut Bencher) {
    let encoded_message = b"\x08\x01\x12\x07\x74\x65\x73\x74\x69\x6e\x67";

    b.iter(|| {
        let result = prost_extraction(encoded_message).unwrap();
        test::black_box(result);
    });
}
