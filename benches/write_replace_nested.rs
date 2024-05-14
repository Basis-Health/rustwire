#![feature(test)]

extern crate test;

use prost::Message;
use rustwire::{create_header, extract_field_by_tag, replace_field_with, Variant};
use test::Bencher;

#[derive(Clone, Message, PartialEq)]
pub struct User {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(int32, tag = "2")]
    pub id: i32,
    #[prost(string, tag = "3")]
    pub email: String,
}

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

    // user field
    #[prost(message, tag = "5")]
    pub user: Option<User>,
}

#[derive(Clone, PartialEq, Message)]
pub struct OutMessage {
    #[prost(int32, tag = "1")]
    pub field1: i32,
    #[prost(string, tag = "2")]
    pub field2: String,
    #[prost(uint64, tag = "3")]
    pub field3: u64,
    #[prost(bool, tag = "4")]
    pub field4: bool,
    #[prost(int32, tag = "5")]
    pub user_id: i32,
}

pub fn default_message() -> ThisMessage {
    let default_string = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".repeat(10);
    let mut default_bytes = Vec::new();
    for i in 0..3397 {
        default_bytes.push(i as u8);
    }

    let user = User {
        name: "John Doe".to_string(),
        id: 12345,
        email: "john@doe.com".to_string(),
    };

    ThisMessage {
        field1: 42,
        field2: default_string,
        field3: 1234567890,
        field4: true,
        user: Some(user),
    }
}

fn prost_replace(encoded_message: &[u8]) -> Vec<u8> {
    let mut message = ThisMessage::decode(encoded_message).unwrap();
    let user = message.user.take().unwrap();
    let out_message = OutMessage {
        field1: message.field1,
        field2: message.field2,
        field3: message.field3,
        field4: message.field4,
        user_id: user.id,
    };

    let mut out_bytes = Vec::new();
    out_message.encode(&mut out_bytes).unwrap();
    out_bytes
}

fn rustwire_replace(encoded_message: &[u8], user_id: i32) -> Vec<u8> {
    // first read id from user field
    let user_field = extract_field_by_tag(encoded_message, 5).unwrap();
    let id = extract_field_by_tag(&user_field, 2).unwrap();

    let header = create_header(5, Variant::Varint.into(), &id);
    let replace_with = [header, user_id.to_le_bytes().to_vec()].concat();
    let result = replace_field_with(&mut encoded_message.to_vec(), 5, &replace_with).unwrap();
    result
}

// Benchmark for extracting a field using rustwire
#[bench]
fn bench_rustwire_replace(b: &mut Bencher) {
    let encoded_message = default_message().encode_to_vec();

    b.iter(|| {
        let result = extract_field_by_tag(&encoded_message, 5).unwrap();
        test::black_box(result);
    });
}

// Benchmark for deserializing the message and getting the field manually
#[bench]
fn bench_prost_replace(b: &mut Bencher) {
    let encoded_message = default_message().encode_to_vec();

    b.iter(|| {
        let result = prost_replace(&encoded_message);
        test::black_box(result);
    });
}
