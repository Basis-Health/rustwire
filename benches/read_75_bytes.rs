#![feature(test)]

extern crate test;

use prost::Message;
use rustwire::{extract_field_by_tag, Variant};
use test::Bencher;

#[derive(Message)]
pub struct LargerMessage {
    #[prost(int32, tag = "1")]
    pub field1: i32,
    #[prost(string, tag = "2")]
    pub field2: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub field3: i64,
    #[prost(int32, tag = "4")]
    pub field4: i32,
    #[prost(int64, tag = "5")]
    pub field5: i64,
    #[prost(int32, tag = "6")]
    pub field6: i32,
    #[prost(int64, tag = "7")]
    pub field7: i64,
    #[prost(int32, tag = "8")]
    pub field8: i32,
    #[prost(int64, tag = "9")]
    pub field9: i64,
    #[prost(int32, tag = "10")]
    pub field10: i32,
    #[prost(int64, tag = "11")]
    pub field11: i64,
    #[prost(int32, tag = "12")]
    pub field12: i32,
    #[prost(int64, tag = "13")]
    pub field13: i64,
    #[prost(int32, tag = "14")]
    pub field14: i32,
    #[prost(int64, tag = "15")]
    pub field15: i64,
    #[prost(int32, tag = "16")]
    pub field16: i32,
    #[prost(int64, tag = "17")]
    pub field17: i64,
    #[prost(int32, tag = "18")]
    pub field18: i32,
    #[prost(int64, tag = "19")]
    pub field19: i64,
    #[prost(int32, tag = "20")]
    pub field20: i32,
    #[prost(int64, tag = "21")]
    pub field21: i64,
    #[prost(int32, tag = "22")]
    pub field22: i32,
    #[prost(int64, tag = "23")]
    pub field23: i64,
    #[prost(int32, tag = "24")]
    pub field24: i32,
    #[prost(int64, tag = "25")]
    pub field25: i64,
}

// 75 bytes
fn default_larger_message() -> LargerMessage {
    LargerMessage {
        field1: 1,
        field2: "testing".to_string(),
        field3: 1,
        field4: 1,
        field5: 1,
        field6: 1,
        field7: 1,
        field8: 1,
        field9: 1,
        field10: 1,
        field11: 1,
        field12: 1,
        field13: 1,
        field14: 1,
        field15: 1,
        field16: 1,
        field17: 1,
        field18: 1,
        field19: 1,
        field20: 1,
        field21: 1,
        field22: 1,
        field23: i64::MAX,
        field24: 1,
        field25: 1,
    }
}

fn prost_extraction(encoded_message: &[u8]) -> Option<i32> {
    LargerMessage::decode(encoded_message)
        .ok()
        .map(|msg| msg.field22)
}

// Benchmark for extracting a field using rustwire
#[bench]
fn bench_rustwire_extraction(b: &mut Bencher) {
    let encoded_message = default_larger_message().encode_to_vec();
    let tag_number = 22;

    b.iter(|| {
        let result = extract_field_by_tag(&encoded_message, tag_number).unwrap();
        let result = test::black_box(result);
    });
}

// Benchmark for deserializing the message and getting the field manually
#[bench]
fn bench_prost_extraction(b: &mut Bencher) {
    let encoded_message = default_larger_message().encode_to_vec();

    b.iter(|| {
        let result = prost_extraction(&encoded_message).unwrap();
        test::black_box(result);
    });
}
