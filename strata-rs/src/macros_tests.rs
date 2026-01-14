#[cfg(test)]
mod tests {
    use crate::value::Value;
    use crate::{bool, bytes, int, list, map, null, string};
    use std::collections::BTreeMap;

    #[test]
    fn macro_variants_construct_correctly() {
        let _ = null!();

        let v = bool!(true);
        assert!(matches!(v, Value::Bool(true)));

        let v = int!(42);
        assert!(matches!(v, Value::Int(_)));

        let v = string!("hello");
        assert!(matches!(v, Value::String(_)));

        let v = bytes!([0xde, 0xad, 0xbe, 0xef]);
        assert!(matches!(v, Value::Bytes(_)));

        let v = list![null!(), bool!(false)];
        assert!(matches!(v, Value::List(_)));

        let v = map! {
            "a" => null!()
        };
        assert!(matches!(v, Value::Map(_)));
    }

    #[test]
    fn nesting_works() {
        let v = map! {
            "list" => list![int!(1), int!(2), int!(3)],
            "nested" => map! {
                "x" => string!("y")
            },
        };

        match v {
            Value::Map(m) => {
                assert!(m.contains_key("list"));
                assert!(m.contains_key("nested"));
            }
            _ => panic!("Expected a map"),
        }
    }

    #[test]
    fn map_duplicate_keys_are_last_write_wins() {
        let v = map! {
            "a" => int!(1),
            "a" => int!(2),
        };

        match v {
            Value::Map(m) => {
                let got = m.get("a").expect("missing key a");
                assert!(matches!(got, Value::Int(2)));
            }
            _ => panic!("Expected a map"),
        }
    }

    #[test]
    fn macro_built_map_matches_manual_map_structure() {
        let macro_v = map! {
            "a" => int!(1),
            "b" => string!("x"),
            "b" => string!("y"), //last wins
        };

        // Manual equivalent
        let mut m = BTreeMap::<String, Value>::new();
        m.insert("a".to_owned(), Value::Int(1));
        m.insert("b".to_owned(), Value::String("y".to_owned()));
        let manual_v = Value::Map(m);

        assert_eq!(macro_v, manual_v);
    }

    #[test]
    fn macro_encoding_and_hash_match_manual_equivalent() {
        let macro_v = map! {
            "a" => int!(1),
            "a" => int!(2),
        };

        let mut m = std::collections::BTreeMap::new();
        m.insert("a".to_owned(), Value::Int(2));
        let manual_v = Value::Map(m);

        let macro_bytes = crate::encode::encode(&macro_v).expect("encoding failed");
        let manual_bytes = crate::encode::encode(&manual_v).expect("encoding failed");
        assert_eq!(macro_bytes, manual_bytes);

        let macro_hash = crate::hash::hash_value(&macro_v);
        let manual_hash = crate::hash::hash_value(&manual_v);
        assert_eq!(macro_hash, manual_hash);
    }
}
