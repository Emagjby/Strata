#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::value::Value;

    #[test]
    fn parse_null() {
        let mut parser = Parser::new("null");
        assert_eq!(parser.parse_value(), Some(Value::Null));
    }
 
    #[test]
    fn parse_integer() {
        let mut parser = Parser::new("42");
        assert_eq!(parser.parse_value(), Some(Value::Int(42)));
    }

    #[test]
    fn parse_string() {
        let mut parser = Parser::new(r#""hello""#);
        assert_eq!(parser.parse_value(), Some(Value::String("hello".into())));
    }

    #[test]
    fn parse_empty_list() {
        let mut parser = Parser::new("[]");
        assert_eq!(parser.parse_value(), Some(Value::List(vec![])));
    }

    #[test]
    fn parse_list_of_ints() {
        let mut parser = Parser::new("[1, 2, 3]");
        assert_eq!(
            parser.parse_value(),
            Some(Value::List(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
            ]))
        );
    }

    #[test]
    fn parse_list_with_trailing_comma() {
        let mut parser = Parser::new("[1, 2,]");
        assert_eq!(
            parser.parse_value(),
            Some(Value::List(vec![
                Value::Int(1),
                Value::Int(2),
            ]))
        );
    }

    #[test]
    fn parse_nested_list() {
        let mut parser = Parser::new("[1, [2, 3]]");
        assert_eq!(
            parser.parse_value(),
            Some(Value::List(vec![
                Value::Int(1),
                Value::List(vec![
                    Value::Int(2),
                    Value::Int(3),
                ])
            ]))
        );
    }

    #[test]
    fn parse_empty_map() {
        let mut parser = Parser::new("{}");
        assert_eq!(
            parser.parse_value(),
            Some(Value::Map(Default::default()))
        );
    }

    #[test]
    fn parse_simple_map() {
        let mut parser = Parser::new("{ a: 1, b: 2 }");

        use std::collections::BTreeMap;
        let mut expected = BTreeMap::new();
        expected.insert("a".into(), Value::Int(1));
        expected.insert("b".into(), Value::Int(2));

        assert_eq!(parser.parse_value(), Some(Value::Map(expected)));
    }

    #[test]
    fn parse_map_with_trailing_comma() {
        let mut parser = Parser::new("{ a: 1, }");

        use std::collections::BTreeMap;
        let mut expected = BTreeMap::new();
        expected.insert("a".into(), Value::Int(1));

        assert_eq!(parser.parse_value(), Some(Value::Map(expected)));
    }

    #[test]
    fn parse_nested_map() {
        let mut parser = Parser::new("{ outer: { inner: 42 } }");

        use std::collections::BTreeMap;

        let mut inner_map = BTreeMap::new();
        inner_map.insert("inner".into(), Value::Int(42));

        let mut outer_map = BTreeMap::new();
        outer_map.insert("outer".into(), Value::Map(inner_map));

        assert_eq!(parser.parse_value(), Some(Value::Map(outer_map)));
    }

    #[test]
    fn parse_map_shorthand() {
        let mut parser = Parser::new("user { id: 42 }");

        use std::collections::BTreeMap;

        let mut inner_map = BTreeMap::new();
        inner_map.insert("id".into(), Value::Int(42));

        let mut outer_map = BTreeMap::new();
        outer_map.insert("user".into(), Value::Map(inner_map));

        assert_eq!(parser.parse_value(), Some(Value::Map(outer_map)));
    }

    #[test]
    fn parse_nested_shorthand() {
        let mut parser = Parser::new("a { b { c: 1 } }");

        use std::collections::BTreeMap;

        let mut c_map = BTreeMap::new();
        c_map.insert("c".into(), Value::Int(1));

        let mut b_map = BTreeMap::new();
        b_map.insert("b".into(), Value::Map(c_map));

        let mut a_map = BTreeMap::new();
        a_map.insert("a".into(), Value::Map(b_map));

        assert_eq!(parser.parse_value(), Some(Value::Map(a_map)));
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

        let mut parser = Parser::new(input);

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
                0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65,
                0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a, 0xd0, 0x15,
            ]),
        );

        let mut root_map = BTreeMap::new();
        root_map.insert("user".into(), Value::Map(user_map));

        assert_eq!(parser.parse_value(), Some(Value::Map(root_map)));
    }

    #[test]
    fn int_out_of_range_rejected() {
        let input = "9223372036854775808";
        assert!(crate::parser::parse(input).is_none());
    }

    #[test]
    fn duplicate_keys_last_write_wins() {
        let input = r#"
            a {
                x: 1
                x: 2
            }
        "#;

        let parsed_value = crate::parser::parse(input).unwrap();

        use std::collections::BTreeMap;

        let mut inner_map = BTreeMap::new();
        inner_map.insert("x".into(), Value::Int(2));

        let mut root_map = BTreeMap::new();
        root_map.insert("a".into(), Value::Map(inner_map));

        assert_eq!(parsed_value, Value::Map(root_map));
    }
}
