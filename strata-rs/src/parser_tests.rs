#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::value::Value;

    #[test]
    fn parse_null() {
        let value = parse("null");
        assert_eq!(value.unwrap(), Value::Null);
    }

    #[test]
    fn parse_integer() {
        let value = parse("42");
        assert_eq!(value.unwrap(), Value::Int(42));
    }

    #[test]
    fn parse_string() {
        let value = parse(r#""hello""#);
        assert_eq!(value.unwrap(), Value::String("hello".into()));
    }

    #[test]
    fn parse_empty_list() {
        let value = parse("[]");
        assert_eq!(value.unwrap(), Value::List(vec![]));
    }

    #[test]
    fn parse_list_of_ints() {
        let value = parse("[1, 2, 3]");
        assert_eq!(
            value.unwrap(),
            Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3),])
        );
    }

    #[test]
    fn parse_list_with_trailing_comma() {
        let value = parse("[1, 2,]");
        assert_eq!(
            value.unwrap(),
            Value::List(vec![Value::Int(1), Value::Int(2),])
        );
    }

    #[test]
    fn parse_nested_list() {
        let value = parse("[1, [2, 3]]");
        assert_eq!(
            value.unwrap(),
            Value::List(vec![
                Value::Int(1),
                Value::List(vec![Value::Int(2), Value::Int(3),])
            ])
        );
    }

    #[test]
    fn parse_empty_map() {
        let value = parse("{}");
        assert_eq!(value.unwrap(), Value::Map(Default::default()));
    }

    #[test]
    fn parse_simple_map() {
        let value = parse("{ a: 1, b: 2 }");

        use std::collections::BTreeMap;
        let mut expected = BTreeMap::new();
        expected.insert("a".into(), Value::Int(1));
        expected.insert("b".into(), Value::Int(2));

        assert_eq!(value.unwrap(), Value::Map(expected));
    }

    #[test]
    fn parse_map_with_trailing_comma() {
        let value = parse("{ a: 1, }");

        use std::collections::BTreeMap;
        let mut expected = BTreeMap::new();
        expected.insert("a".into(), Value::Int(1));

        assert_eq!(value.unwrap(), Value::Map(expected));
    }

    #[test]
    fn parse_nested_map() {
        let value = parse("{ outer: { inner: 42 } }");

        use std::collections::BTreeMap;

        let mut inner_map = BTreeMap::new();
        inner_map.insert("inner".into(), Value::Int(42));

        let mut outer_map = BTreeMap::new();
        outer_map.insert("outer".into(), Value::Map(inner_map));

        assert_eq!(value.unwrap(), Value::Map(outer_map));
    }

    #[test]
    fn parse_map_shorthand() {
        let value = parse("user { id: 42 }");

        use std::collections::BTreeMap;

        let mut inner_map = BTreeMap::new();
        inner_map.insert("id".into(), Value::Int(42));

        let mut outer_map = BTreeMap::new();
        outer_map.insert("user".into(), Value::Map(inner_map));

        assert_eq!(value.unwrap(), Value::Map(outer_map));
    }

    #[test]
    fn parse_nested_shorthand() {
        let value = parse("a { b { c: 1 } }");

        use std::collections::BTreeMap;

        let mut c_map = BTreeMap::new();
        c_map.insert("c".into(), Value::Int(1));

        let mut b_map = BTreeMap::new();
        b_map.insert("b".into(), Value::Map(c_map));

        let mut a_map = BTreeMap::new();
        a_map.insert("a".into(), Value::Map(b_map));

        assert_eq!(value.unwrap(), Value::Map(a_map));
    }

    #[test]
    fn parse_full_example_2_4() {
        let input = r#"
            user {
              id: 42
              name: "Gencho"
              active: true
              skills: ["rust", "svelte", "systems"]
              avatar_hash: 0x9f86d081884c7d659a2feaa0c55ad015
            }
        "#;

        let value = parse(input);

        use std::collections::BTreeMap;

        let mut user_map = BTreeMap::new();
        user_map.insert("id".into(), Value::Int(42));
        user_map.insert("name".into(), Value::String("Gencho".into()));
        user_map.insert("active".into(), Value::Bool(true));
        user_map.insert(
            "skills".into(),
            Value::List(vec![
                Value::String("rust".into()),
                Value::String("svelte".into()),
                Value::String("systems".into()),
            ]),
        );
        user_map.insert(
            "avatar_hash".into(),
            Value::Bytes(vec![
                0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65, 0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a,
                0xd0, 0x15,
            ]),
        );

        let mut root_map = BTreeMap::new();
        root_map.insert("user".into(), Value::Map(user_map));

        assert_eq!(value.unwrap(), Value::Map(root_map));
    }

    #[test]
    fn int_out_of_range_rejected() {
        let input = "9223372036854775808";
        assert!(parse(input).is_err());
    }

    #[test]
    fn duplicate_keys_last_write_wins() {
        let input = r#"
            a {
                x: 1
                x: 2
            }
        "#;

        let value = parse(input);

        use std::collections::BTreeMap;

        let mut inner_map = BTreeMap::new();
        inner_map.insert("x".into(), Value::Int(2));

        let mut root_map = BTreeMap::new();
        root_map.insert("a".into(), Value::Map(inner_map));

        assert_eq!(value.unwrap(), Value::Map(root_map));
    }
}
