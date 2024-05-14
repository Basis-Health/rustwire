# Rustwire

[![Latest Version](https://img.shields.io/crates/v/rustwire.svg)](https://crates.io/crates/rustwire)
![docs.rs](https://img.shields.io/docsrs/rustwire)

A Rust library designed for decoding and manipulating Protobuf-encoded messages.

This is a dependecy-free helper crate and is not meant to be a full-fledged Protobuf library. It is designed to be used in conjunction with a full Protobuf library like `prost` or `protobuf`. The main use case for us for example is to extract nested Protobuf and replace them with just the id of said nested Protobuf.

## Speed

This library is designed to manipulate encoded messages, as fast as possible thus without decoding. To understand the impact of this, we created three benchmarks (all benchmarks read one field from a message):

### Reading

| Benchmark | Rustwire | Prost  |
| --------- | -------- | ------ |
| 11 bytes  | 83 ns    | 62 ns  |
| 75 bytes  | 207 ns   | 240 ns |
| 4k bytes  | 63 ns    | 488 ns |

This performance comes mostly from the fact that Rustwire does not decode unintresting fields, while Prost does. This means that Rustwire is faster when you only need to read a few fields from a message.

### Writing

In this benchmark we replace a field in a message with a new value. This is one of the main use cases for Rustwire. Let's assume we have the following message:

```protobuf
message User {
    string name = 1;
    int32 id = 2;
    string email = 3;
}

message ThisMessage {
    int32 field1 = 1;
    string field2 = 2;
    uint64 field3 = 3;
    bool field4 = 4;
    User user = 5;
}
```

We want to replace the `User` field with just the `id` field. This is how you would do it with Rustwire:

```rust
let user_field = extract_field_by_tag(encoded_message, 5).unwrap();
let id = extract_field_by_tag(&user_field, 2).unwrap();

let header = create_header(5, Variant::Varint.into(), &id);
let replace_with = [header, user_id.to_le_bytes().to_vec()].concat();
let result = replace_field_with(&mut encoded_message.to_vec(), 5, &replace_with).unwrap();
```

in prost you would have to decode the message, change the field and then encode it again. This is how you would do it with prost:

```rust
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
```

You could propably also use `prost-reflect` to do this, but that would be even slower.

The benchmarks for this are as follows:

- Rustwire: 31 ns
- Prost: 532 ns

This is a huge difference, and the message size was rather small. As seen in the reading benchmarks, the difference will only increase with larger messages.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
