#[cfg(test)]
mod tests {
    use crate::parser::{Parser, Value};

    #[test]
    fn semantic_vector_config() {
        let input = r#"
            config {
                enabled: true
                retries: 3
                name: "strata"
                empty: null
            }
        "#;

        let mut p = Parser::new(input);

        use std::collections::BTreeMap;

        let mut config = BTreeMap::new();
        config.insert("enabled".into(), Value::Bool(true));
        config.insert("retries".into(), Value::Int(3));
        config.insert("name".into(), Value::String("strata".into()));
        config.insert("empty".into(), Value::Null);

        let mut root = BTreeMap::new();
        root.insert("config".into(), Value::Map(config));

        assert_eq!(p.parse_value(), Some(Value::Map(root)));
    }

    #[test]
    fn semantic_vector_key_ordering() {
        let input = r#"
            data {
                z: 1
                a: 2
                m: 3
            }
        "#;

        let mut p = Parser::new(input);

        use std::collections::BTreeMap;

        let mut data = BTreeMap::new();
        data.insert("a".into(), Value::Int(2));
        data.insert("m".into(), Value::Int(3));
        data.insert("z".into(), Value::Int(1));

        let mut root = BTreeMap::new();
        root.insert("data".into(), Value::Map(data));

        assert_eq!(p.parse_value(), Some(Value::Map(root)));
    }

    #[test]
    fn semantic_vector_profile() {
        let input = r#"
            profile {
                id: 9007199254740993
                avatar_hash: 0x9f86d081884c7d659a2feaa0c55ad015
                tags: ["logistics", "state", "integrity"]
            }
        "#;

        let mut p = Parser::new(input);

        use std::collections::BTreeMap;

        let mut profile = BTreeMap::new();
        profile.insert("id".into(), Value::Int(9007199254740993));
        profile.insert(
            "avatar_hash".into(),
            Value::Bytes(vec![
                0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65,
                0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a, 0xd0, 0x15,
            ]),
        );
        profile.insert(
            "tags".into(),
            Value::List(vec![
                Value::String("logistics".into()),
                Value::String("state".into()),
                Value::String("integrity".into()),
            ]),
        );

        let mut root = BTreeMap::new();
        root.insert("profile".into(), Value::Map(profile));

        assert_eq!(p.parse_value(), Some(Value::Map(root)));
    }
}
