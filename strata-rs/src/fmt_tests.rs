#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::fmt::{fmt, FormatOptions};
    use crate::value::Value;

    #[test]
    fn fmt_primitives() {
        assert_eq!(fmt(FormatOptions::PRETTY, &Value::Null), "null");
        assert_eq!(fmt(FormatOptions::PRETTY, &Value::Bool(true)), "true");
        assert_eq!(fmt(FormatOptions::PRETTY, &Value::Int(42)), "42");
    }

    #[test]
    fn fmt_string() {
        let v = Value::String("hello".into());
        assert_eq!(fmt(FormatOptions::PRETTY, &v), "\"hello\"");
    }

    #[test]
    fn fmt_bytes() {
        let v = Value::Bytes(vec![0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(fmt(FormatOptions::PRETTY, &v), "[deadbeef]");
    }

    #[test]
    fn fmt_list() {
        let v = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);

        assert_eq!(fmt(FormatOptions::PRETTY, &v), "[1, 2, 3]");
    }

    #[test]
    fn fmt_map_multiline() {
        let mut map = BTreeMap::new();
        map.insert("id".to_string(), Value::Int(42));
        map.insert("active".to_string(), Value::Bool(true));

        let v = Value::Map(map);

        let expected = r#"{
  active: true
  id: 42
}"#;

        assert_eq!(fmt(FormatOptions::PRETTY, &v), expected);
    }

    #[test]
    fn fmt_nested_structure() {
        let mut inner = BTreeMap::new();
        inner.insert("x".to_string(), Value::Int(1));

        let mut outer = BTreeMap::new();
        outer.insert("nested".to_string(), Value::Map(inner));

        let v = Value::Map(outer);

        let expected = r#"{
  nested: {
    x: 1
  }
}"#;

        assert_eq!(fmt(FormatOptions::PRETTY, &v), expected);
    }

    #[test]
    fn fmt_ast_mode() {
        let v = Value::Int(7);

        let output = fmt(FormatOptions::AST, &v);

        assert!(output.contains("Int"));
        assert!(output.contains("7"));
    }

    #[test]
    fn fmt_is_deterministic() {
        let mut map = BTreeMap::new();
        map.insert("b".to_string(), Value::Int(2));
        map.insert("a".to_string(), Value::Int(1));

        let v = Value::Map(map);

        let first = fmt(FormatOptions::PRETTY, &v);
        let second = fmt(FormatOptions::PRETTY, &v);

        assert_eq!(first, second);
    }
}
