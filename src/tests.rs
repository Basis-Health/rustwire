#[cfg(test)]
mod tests {
    use crate::{
        create_header, extract_field_by_tag, extract_multiple_fields_by_tag, replace_field_with,
    };
    use prost::Message;

    /// Test extracting a single field from a simple message.
    #[test]
    fn extract_single_field() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
        }

        let foo = Foo {
            bar: "Me".to_string(),
        };
        let enc = foo.encode_to_vec();

        let bar = extract_field_by_tag(&enc, 1).unwrap();
        assert_eq!(bar, b"Me");
    }

    /// Test extracting a single field from a message with multiple fields.
    #[test]
    fn extract_single_field_from_message_with_multiple_fields() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            baz: ::prost::alloc::string::String,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            baz: "You".to_string(),
        };
        let enc = foo.encode_to_vec();

        let bar = extract_field_by_tag(&enc, 1).unwrap();
        assert_eq!(bar, b"Me");
    }

    /// Test extracting multiple fields from a message.
    #[test]
    fn extract_multiple_fields() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            baz: ::prost::alloc::string::String,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            baz: "You".to_string(),
        };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1, 2]);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"Me");
        assert_eq!(fields[1].0, 2);
        assert_eq!(fields[1].1, b"You");
    }

    /// Test extracting a subset of fields from a message.
    #[test]
    fn extract_subset_of_fields() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            baz: ::prost::alloc::string::String,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            baz: "You".to_string(),
        };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1]);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"Me");
    }

    /// Test extracting a field that does not exist in the message.
    #[test]
    fn extract_missing_field() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            baz: ::prost::alloc::string::String,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            baz: "You".to_string(),
        };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[3]);
        assert_eq!(fields.len(), 0);
    }

    /// Test extracting a varint field from a message.
    #[test]
    fn extract_field_with_varint() {
        #[derive(Message)]
        struct Foo {
            #[prost(uint64, tag = "1")]
            bar: u64,
        }

        let foo = Foo { bar: 42 };
        let enc = foo.encode_to_vec();

        let bar = extract_field_by_tag(&enc, 1).unwrap();
        assert_eq!(bar, b"\x2A");
    }

    /// Test extracting the maximum value of a u64 field.
    #[test]
    fn extract_u64_max() {
        #[derive(Message)]
        struct Foo {
            #[prost(uint64, tag = "1")]
            bar: u64,
        }

        let foo = Foo { bar: u64::MAX };
        let enc = foo.encode_to_vec();

        let bar = extract_field_by_tag(&enc, 1).unwrap();
        assert_eq!(bar, b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01");
    }

    /// Test extracting multiple fields with different types.
    #[test]
    fn extract_u64_max_and_string() {
        #[derive(Message)]
        struct Foo {
            #[prost(uint64, tag = "1")]
            bar: u64,
            #[prost(string, tag = "2")]
            baz: ::prost::alloc::string::String,
        }

        let foo = Foo {
            bar: u64::MAX,
            baz: "Me".to_string(),
        };
        let enc = foo.encode_to_vec();

        let bar = extract_field_by_tag(&enc, 1).unwrap();
        assert_eq!(bar, b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01");

        let baz = extract_field_by_tag(&enc, 2).unwrap();
        assert_eq!(baz, b"Me");
    }

    /// Test extracting multiple varint fields from a message.
    #[test]
    fn extract_multiple_fields_with_varint() {
        #[derive(Message)]
        struct Foo {
            #[prost(uint64, tag = "1")]
            bar: u64,
            #[prost(uint64, tag = "2")]
            baz: u64,
        }

        let foo = Foo { bar: 42, baz: 43 };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1, 2]);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"\x2A");
        assert_eq!(fields[1].0, 2);
        assert_eq!(fields[1].1, b"\x2B");
    }

    /// Test extracting a field while skipping another field.
    #[test]
    fn extract_field_skip_string() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(uint64, tag = "2")]
            baz: u64,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            baz: 42,
        };
        let enc = foo.encode_to_vec();

        let baz = extract_field_by_tag(&enc, 2).unwrap();
        assert_eq!(baz, b"\x2A");
    }

    /// Test extracting optional fields from a message.
    #[test]
    fn extract_optional_field() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(uint64, optional, tag = "2")]
            baz: ::core::option::Option<u64>,
            #[prost(uint64, tag = "3")]
            qux: u64,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            baz: Some(42),
            qux: 43,
        };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1, 3]);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"Me");
        assert_eq!(fields[1].0, 3);
        assert_eq!(fields[1].1, b"\x2B");

        let baz = extract_field_by_tag(&enc, 2).unwrap();
        assert_eq!(baz, b"\x2A");

        let foo = Foo {
            bar: "Me".to_string(),
            baz: None,
            qux: 43,
        };

        let enc = foo.encode_to_vec();
        let fields = extract_multiple_fields_by_tag(&enc, &[1, 3]);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"Me");
        assert_eq!(fields[1].0, 3);
        assert_eq!(fields[1].1, b"\x2B");

        let baz = extract_field_by_tag(&enc, 2);
        assert_eq!(baz, None);
    }

    /// Test extracting repeated fields from a message.
    #[test]
    fn extract_repeated_field() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(uint64, repeated, tag = "2")]
            baz: ::prost::alloc::vec::Vec<u64>,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            baz: vec![42, 43],
        };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1, 2]);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"Me");
        assert_eq!(fields[1].0, 2);
        assert_eq!(fields[1].1, b"\x2A\x2B");
    }

    /// Test extracting specific fields while skipping repeated fields.
    #[test]
    fn extract_skip_repeated_field() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(uint64, repeated, tag = "2")]
            baz: ::prost::alloc::vec::Vec<u64>,
            #[prost(uint64, tag = "3")]
            qux: u64,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            baz: vec![42, 43],
            qux: 44,
        };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1, 3]);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"Me");
        assert_eq!(fields[1].0, 3);
        assert_eq!(fields[1].1, b"\x2C");
    }

    /// Test extracting a nested message field.
    #[test]
    fn test_extract_nested_message() {
        #[derive(Message)]
        struct Bar {
            #[prost(uint64, tag = "1")]
            baz: u64,
        }

        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(message, tag = "2")]
            qux: ::core::option::Option<Bar>,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            qux: Some(Bar { baz: 42 }),
        };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1, 2]);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"Me");
        assert_eq!(fields[1].0, 2);
        assert_eq!(fields[1].1, b"\x08\x2A");

        let bar = extract_field_by_tag(&enc, 2).unwrap();
        assert_eq!(bar, b"\x08\x2A");
    }

    /// Test extracting a nested message from a nested field.
    #[test]
    fn test_extract_nested_message_from_nested_field() {
        #[derive(Message)]
        struct Bar {
            #[prost(uint64, tag = "1")]
            baz: u64,
        }

        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(message, tag = "2")]
            qux: ::core::option::Option<Bar>,
        }

        #[derive(Message)]
        struct Baz {
            #[prost(message, tag = "1")]
            foo: ::core::option::Option<Foo>,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            qux: Some(Bar { baz: 42 }),
        };

        let baz = Baz { foo: Some(foo) };

        let enc = baz.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1]);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].0, 1);

        let enc_foo = fields[0].1;
        let fields = extract_multiple_fields_by_tag(&enc_foo, &[1, 2]);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"Me");
        assert_eq!(fields[1].0, 2);
        assert_eq!(fields[1].1, b"\x08\x2A");
    }

    /// Test extracting a double field.
    #[test]
    fn test_extract_double() {
        #[derive(Message)]
        struct Foo {
            #[prost(double, tag = "1")]
            bar: f64,
        }

        let foo = Foo { bar: 42.0 };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1]);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, 42.0f64.to_le_bytes());
    }

    /// Test extracting a float field.
    #[test]
    fn test_extract_float() {
        #[derive(Message)]
        struct Foo {
            #[prost(float, tag = "1")]
            bar: f32,
        }

        let foo = Foo { bar: 42.0 };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1]);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, 42.0f32.to_le_bytes());
    }

    /// Test replacing a field with new data.
    #[test]
    fn test_replace_field() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
            #[prost(uint64, repeated, tag = "2")]
            baz: ::prost::alloc::vec::Vec<u64>,
        }

        let foo = Foo {
            bar: "Me".to_string(),
            baz: vec![42, 43],
        };
        let mut enc = foo.encode_to_vec();

        let new_bar_enc = vec![0x0A, 0x03, b'Y', b'o', b'u'];
        replace_field_with(&mut enc, 1, &new_bar_enc);

        let fields = extract_multiple_fields_by_tag(&enc, &[1, 2]);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, b"You");
    }

    /// Test replacing a nested message with a specific field.
    #[test]
    fn test_replace_nested_message_with_field() {
        #[derive(Message, PartialEq, Clone)]
        struct User {
            #[prost(string, tag = "1")]
            name: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            uid: ::prost::alloc::string::String,
        }

        #[derive(Message, PartialEq)]
        struct Summary {
            #[prost(uint64, tag = "1")]
            count: u64,
            #[prost(message, tag = "2")]
            user: ::core::option::Option<User>,
        }

        let user = User {
            name: "Alice".to_string(),
            uid: "123".to_string(),
        };
        let summary = Summary {
            count: 1,
            user: Some(user.clone()),
        };

        let mut enc = summary.encode_to_vec();
        let enc_orig = enc.clone();

        let uid = vec![0x12, 0x03, b'1', b'2', b'3'];
        replace_field_with(&mut enc, 2, &uid);

        #[derive(Message, PartialEq)]
        struct SummaryUid {
            #[prost(uint64, tag = "1")]
            count: u64,
            #[prost(string, tag = "2")]
            uid: ::prost::alloc::string::String,
        }

        let should_be = SummaryUid {
            count: 1,
            uid: "123".to_string(),
        };

        let summary_uid = SummaryUid::decode(enc.as_slice()).unwrap();
        assert_eq!(summary_uid, should_be);

        let mut user_enc = user.encode_to_vec();
        user_enc.insert(0, user_enc.len() as u8);
        user_enc.insert(0, 18);

        replace_field_with(&mut enc, 2, &user_enc);

        let new_summary = Summary::decode(enc.as_slice()).unwrap();
        assert_eq!(new_summary, summary);
    }

    /// Test extracting a string field longer than 255 bytes.
    #[test]
    fn test_extract_string_longer_than_255() {
        #[derive(Message)]
        struct Foo {
            #[prost(string, tag = "1")]
            bar: ::prost::alloc::string::String,
        }

        let foo = Foo {
            bar: "A".repeat(512),
        };
        let enc = foo.encode_to_vec();

        let fields = extract_multiple_fields_by_tag(&enc, &[1]);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].0, 1);
        assert_eq!(fields[0].1, "A".repeat(512).as_bytes());
    }

    /// Test creating a header for a message.
    #[test]
    fn test_create_header() {
        let data = b"Hello";
        let header = create_header(1, 2, data);

        let expected = vec![(1 << 3) | 2, 5];
        assert_eq!(header, expected);
    }
}
