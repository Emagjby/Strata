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
}
