# Rustwire

[![Latest Version](https://img.shields.io/crates/v/rustwire.svg)](https://crates.io/crates/rustwire)
![docs.rs](https://img.shields.io/docsrs/rustwire)

A Rust library designed for decoding and manipulating Protobuf-encoded messages.

This is a dependecy-free helper crate and is not meant to be a full-fledged Protobuf library. It is designed to be used in conjunction with a full Protobuf library like `prost` or `protobuf`. The main use case for us for example is to extract nested Protobuf and replace them with just the id of said nested Protobuf.

## What is Rustwire?

- **Fast**: Rustwire is designed to be as fast as possible, by not decoding the entire message.
- **Lightweight**: Rustwire is a small library with no dependencies.

## What is Rustwire not?

- **A full Protobuf library**: Rustwire is not designed to be a full Protobuf library. It is designed to be used in conjunction with a full Protobuf library like `prost` or `protobuf`.

- **A protobuf comoformant changeset utility**: The main use case of Rustwire is to replace nested Protobuf messages. Rustwire won't check if the changes you make to the message are valid, it will just replace the field with the new field.

## Compoments
Rustwire is made up of three main components:
- **Extractor**: The extractor is used to extract fields from a Protobuf-encoded message.
- **Replacer**: The replacer is used to replace fields in a Protobuf-encoded message.
- **Encoder**: The encoder is used to encode a field into a Protobuf-encoded message. This exists mostly for varint fields.


## Speed

This library is designed to manipulate encoded messages, as fast as possible thus without decoding. To understand the impact of this, we created three benchmarks (all benchmarks read one field from a message):

### Reading

| Benchmark | Rustwire | Prost  |
| --------- | -------- | ------ |
| 11 bytes  | 62 ns    | 83 ns  |
| 75 bytes  | 207 ns   | 240 ns |
| 4k bytes  | 63 ns    | 488 ns |

This performance comes mostly from the fact that Rustwire does not decode unintresting fields, while Prost does. This means that Rustwire is faster when you only need to read a few fields from a message.

### Writing

For this benchmark we assume that we have a message called `ThisMessage` with the following structure:

```protobuf
message ThisMessage {
    int32 field1 = 1;
    string field2 = 2;
    uint64 field3 = 3;
    bool field4 = 4;
    User user = 5;
}
```
The `User` message has the following structure:

```protobuf
message User {
    string name = 1;
    int32 id = 2;
    string email = 3;
}
```

Let's assume that we have many `ThisMessage` messages but only few `User`'s. Protobuf is a binary format, and can easily be used to store data in a database. To optimize the storage size, we want to replace the `User` message with just the `id` field. To do this in Prost, you would have to:
1. Create a new message with just the `id` field instead of the `User` message.

<details>
  <summary>The encoded message (<i>click to expand</i>)</summary>
  <!-- have to be followed by an empty line! -->

```protobuf
message OutMessage {
    int32 field1 = 1;
    string field2 = 2;
    uint64 field3 = 3;
    bool field4 = 4;
    int32 user_id = 5;
}
```
</details>

2. Decode the `ThisMessage` message, then extract the `User` message and replace it with the `id` field.
3. Encode the new message.

For our benchmark this took **532 ns**.


In Rustwire, you can
1. Extract the `User` message from the `ThisMessage` message.
```rust
let user_field = extract_field_by_tag(encoded_message, 5).unwrap();
let id = extract_field_by_tag(&user_field, 2).unwrap();
```
2. Create a new message with just the `id` field.
```rust
let header = create_header(5, Variant::Varint.into(), &id);
let replace_with = [header, id].concat();
```
3. Replace the `User` message with the `id` field.
```rust
let result = replace_field_with(&mut encoded_message.to_vec(), 5, &replace_with).unwrap();
```

This took **31 ns**.


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
