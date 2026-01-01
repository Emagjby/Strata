#[cfg(test)]
mod tests {
    use crate::encode::{encode_uleb128, encode_sleb128, encode_value};
    use crate::value::Value;
    use crate::framing::encode_framed;

    #[test]
    fn uleb128_basic() {
        let mut out = Vec::new();
        encode_uleb128(0, &mut out);
        assert_eq!(out, vec![0x00]);

        out.clear();
        encode_uleb128(1, &mut out);
        assert_eq!(out, vec![0x01]);

        out.clear();
        encode_uleb128(127, &mut out);
        assert_eq!(out, vec![0x7F]);

        out.clear();
        encode_uleb128(128, &mut out);
        assert_eq!(out, vec![0x80, 0x01]);
    }

    #[test]
    fn sleb128_basic() {
        let mut out = Vec::new();
        encode_sleb128(0, &mut out);
        assert_eq!(out, vec![0x00]);

        out.clear();
        encode_sleb128(1, &mut out);
        assert_eq!(out, vec![0x01]);

        out.clear();
        encode_sleb128(-1, &mut out);
        assert_eq!(out, vec![0x7F]);

        out.clear();
        encode_sleb128(127, &mut out);
        assert_eq!(out, vec![0xFF, 0x00]);

        out.clear();
        encode_sleb128(-128, &mut out);
        assert_eq!(out, vec![0x80, 0x7F]);
    }

    // primitives
    #[test]
    fn encode_null() {
        assert_eq!(encode_value(&Value::Null), vec![0x00]);
    }

    #[test]
    fn encode_bool() {
        assert_eq!(encode_value(&Value::Bool(false)), vec![0x01]);
        assert_eq!(encode_value(&Value::Bool(true)), vec![0x02]);
    }

    #[test]
    fn encode_int() {
        assert_eq!(encode_value(&Value::Int(1)), vec![0x10, 0x01]);
    }

    // string & bytes
    #[test]
    fn encode_string() {
        let value = Value::String("hi".into());
        assert_eq!(encode_value(&value), vec![0x20, 0x02, b'h', b'i']);
    }

    #[test]
    fn encode_bytes() {
        let value = Value::Bytes(vec![0xDE, 0xAD]);
        assert_eq!(encode_value(&value), vec![0x21, 0x02, 0xDE, 0xAD]);
    }

    // lists
    #[test]
    fn encode_list() {
        let value = Value::List(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(
            encode_value(&value),
            vec![0x30, 0x02, 0x10, 0x01, 0x10, 0x02]
        );
    }

    // maps
    #[test]
    fn encode_map_sorted() {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert("b".into(), Value::Int(2));
        map.insert("a".into(), Value::Int(1));

        let value = Value::Map(map);

        assert_eq!(
            encode_value(&value),
            vec![
                0x40, 0x02,
                0x20, 0x01, b'a', 0x10, 0x01,
                0x20, 0x01, b'b', 0x10, 0x02,
            ]
        );
    }

    // nested structures
    #[test]
    fn encode_nested() {
        let value = Value::List(vec![
            Value::Map({
                let mut m = std::collections::BTreeMap::new();
                m.insert("x".into(), Value::Int(1));
                m
            })
        ]);

        assert_eq!(
            encode_value(&value),
            vec![
                0x30, 0x01,
                0x40, 0x01,
                0x20, 0x01, b'x',
                0x10, 0x01
            ]
        );
    }

    // framing
    #[test]
    fn framed_vs_unframed() {
        let value = Value::Int(1);

        let unframed = encode_value(&value);
        let framed = encode_framed(&value);

        assert_eq!(unframed, vec![0x10, 0x01]);

        assert_eq!(
            framed,
            vec![
                b'S', b'T', b'R', b'A', b'T', b'A', b'1',
                0x01,
                0x10, 0x01
            ]
        );
    }
}
