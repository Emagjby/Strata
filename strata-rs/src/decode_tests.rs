#[cfg(test)]
mod tests {
    use crate::decode::decode;
    use crate::error::DecodeErrorKind;
    use crate::value::Value;

    // null/bool/int
    #[test]
    fn decode_null() {
        let bytes = vec![0x00];
        assert_eq!(decode(&bytes).unwrap(), Value::Null);
    }

    #[test]
    fn decode_bool() {
        assert_eq!(decode(&[0x01]).unwrap(), Value::Bool(false));
        assert_eq!(decode(&[0x02]).unwrap(), Value::Bool(true));
    }

    #[test]
    fn decode_int() {
        // Int 1 -> 0x10 0x01
        assert_eq!(decode(&[0x10, 0x01]), Ok(Value::Int(1)));
    }

    // strings and bytes
    #[test]
    fn decode_string() {
        // 'hi'
        let bytes = vec![0x20, 0x02, b'h', b'i'];

        assert_eq!(decode(&bytes), Ok(Value::String("hi".into())));
    }

    #[test]
    fn decode_bytes() {
        let bytes = vec![0x21, 0x02, 0xaa, 0xbb];
        assert_eq!(decode(&bytes), Ok(Value::Bytes(vec![0xaa, 0xbb])));
    }

    // list decoding
    #[test]
    fn decode_list() {
        // [1, 2]
        let bytes = vec![0x30, 0x02, 0x10, 0x01, 0x10, 0x02];

        assert_eq!(
            decode(&bytes),
            Ok(Value::List(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    // map decoding
    #[test]
    fn decode_map() {
        // { 'a': 1 }
        let bytes = vec![
            0x40, 0x01, // map with 1 entry
            0x20, 0x01, b'a', // key: 'a'
            0x10, 0x01, // value: 1
        ];

        let mut map = std::collections::BTreeMap::new();
        map.insert("a".into(), Value::Int(1));

        assert_eq!(decode(&bytes), Ok(Value::Map(map)));
    }

    // nested structures
    #[test]
    fn decode_nested() {
        // { "x": [true, null] }
        let bytes = vec![
            0x40, 0x01, // map with 1 entry
            0x20, 0x01, b'x', // key: 'x
            0x30, 0x02, // list with 2 items
            0x02, // true
            0x00, // null
        ];

        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "x".into(),
            Value::List(vec![Value::Bool(true), Value::Null]),
        );

        assert_eq!(decode(&bytes), Ok(Value::Map(map)));
    }

    // safety tests v2.1 strict
    #[test]
    fn decode_truncated_string() {
        let bytes = vec![0x20, 0x05, b'h']; // length 5, but only 1 byte provided

        let err = decode(&bytes).unwrap_err();
        assert_eq!(err.kind, DecodeErrorKind::UnexpectedEOF);
        assert_eq!(err.offset, 2);
    }

    #[test]
    fn decode_invalid_tag() {
        let bytes = vec![0xFF]; // invalid tag

        let err = decode(&bytes).unwrap_err();
        assert_eq!(err.kind, DecodeErrorKind::InvalidTag(0xFF));
        assert_eq!(err.offset, 1);
    }

    #[test]
    fn decode_trailing_bytes() {
        let bytes = vec![0x00, 0x00];

        let err = decode(&bytes).unwrap_err();
        assert_eq!(err.kind, DecodeErrorKind::TrailingBytes);
        assert_eq!(err.offset, 1);
    }
}
