#[cfg(test)]
mod tests {
    use crate::parser::{Parser, Value};

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
}
