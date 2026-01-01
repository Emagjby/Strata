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
}
