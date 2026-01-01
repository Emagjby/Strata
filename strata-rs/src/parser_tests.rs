#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::value::Value;

    #[test]
    fn parse_null() {
        let mut p = Parser::new("null");
        assert_eq!(p.parse_value(), Some(Value::Null));
    }
 
    #[test]
    fn parse_integer() {
        let mut p = Parser::new("42");
        assert_eq!(p.parse_value(), Some(Value::Int(42)));
    }

    #[test]
    fn parse_string() {
        let mut p = Parser::new(r#""hello""#);
        assert_eq!(p.parse_value(), Some(Value::String("hello".into())));
    }

    #[test]
    fn parse_empty_list() {
        let mut p = Parser::new("[]");
        assert_eq!(p.parse_value(), Some(Value::List(vec![])));
    }

    #[test]
    fn parse_list_of_ints() {
        let mut p = Parser::new("[1, 2, 3]");
        assert_eq!(
            p.parse_value(),
            Some(Value::List(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
            ]))
        );
    }

    #[test]
    fn parse_list_with_trailing_comma() {
        let mut p = Parser::new("[1, 2,]");
        assert_eq!(
            p.parse_value(),
            Some(Value::List(vec![
                Value::Int(1),
                Value::Int(2),
            ]))
        );
    }

    #[test]
    fn parse_nested_list() {
        let mut p = Parser::new("[1, [2, 3]]");
        assert_eq!(
            p.parse_value(),
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
        let mut p = Parser::new("{}");
        assert_eq!(
            p.parse_value(),
            Some(Value::Map(Default::default()))
        );
    }

    #[test]
    fn parse_simple_map() {
        let mut p = Parser::new("{ a: 1, b: 2 }");

        use std::collections::BTreeMap;
        let mut expected = BTreeMap::new();
        expected.insert("a".into(), Value::Int(1));
        expected.insert("b".into(), Value::Int(2));

        assert_eq!(p.parse_value(), Some(Value::Map(expected)));
    }

    #[test]
    fn parse_map_with_trailing_comma() {
        let mut p = Parser::new("{ a: 1, }");

        use std::collections::BTreeMap;
        let mut expected = BTreeMap::new();
        expected.insert("a".into(), Value::Int(1));

        assert_eq!(p.parse_value(), Some(Value::Map(expected)));
    }

    #[test]
    fn parse_nested_map() {
        let mut p = Parser::new("{ outer: { inner: 42 } }");

        use std::collections::BTreeMap;

        let mut inner = BTreeMap::new();
        inner.insert("inner".into(), Value::Int(42));

        let mut outer = BTreeMap::new();
        outer.insert("outer".into(), Value::Map(inner));

        assert_eq!(p.parse_value(), Some(Value::Map(outer)));
    }

    #[test]
    fn parse_map_shorthand() {
        let mut p = Parser::new("user { id: 42 }");

        use std::collections::BTreeMap;

        let mut inner = BTreeMap::new();
        inner.insert("id".into(), Value::Int(42));

        let mut outer = BTreeMap::new();
        outer.insert("user".into(), Value::Map(inner));

        assert_eq!(p.parse_value(), Some(Value::Map(outer)));
    }

    #[test]
    fn parse_nested_shorthand() {
        let mut p = Parser::new("a { b { c: 1 } }");

        use std::collections::BTreeMap;

        let mut c = BTreeMap::new();
        c.insert("c".into(), Value::Int(1));

        let mut b = BTreeMap::new();
        b.insert("b".into(), Value::Map(c));

        let mut a = BTreeMap::new();
        a.insert("a".into(), Value::Map(b));

        assert_eq!(p.parse_value(), Some(Value::Map(a)));
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

        let mut p = Parser::new(input);

        use std::collections::BTreeMap;

        let mut user = BTreeMap::new();
        user.insert("id".into(), Value::Int(42));
        user.insert("name".into(), Value::String("Gencho".into()));
        user.insert("active".into(), Value::Bool(true));
        user.insert(
            "skills".into(),
            Value::List(vec![
                Value::String("rust".into()),
                Value::String("svelte".into()),
                Value::String("systems".into()),
            ]),
        );
        user.insert(
            "avatar_hash".into(),
            Value::Bytes(vec![
                0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65,
                0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a, 0xd0, 0x15,
            ]),
        );

        let mut root = BTreeMap::new();
        root.insert("user".into(), Value::Map(user));

        assert_eq!(p.parse_value(), Some(Value::Map(root)));
    }
}
