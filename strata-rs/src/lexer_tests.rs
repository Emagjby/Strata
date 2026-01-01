#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn lex_identifiers_and_keywords() {
        let mut lexer = Lexer::new("foo null true false bar");

        assert_eq!(lexer.next_token(), Some(Token::Ident("foo".into())));
        assert_eq!(lexer.next_token(), Some(Token::Null));
        assert_eq!(lexer.next_token(), Some(Token::True));
        assert_eq!(lexer.next_token(), Some(Token::False));
        assert_eq!(lexer.next_token(), Some(Token::Ident("bar".into())));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn lex_integers() {
        let mut lexer = Lexer::new("0 42 -7 -0");

        assert_eq!(lexer.next_token(), Some(Token::Int(0)));
        assert_eq!(lexer.next_token(), Some(Token::Int(42)));
        assert_eq!(lexer.next_token(), Some(Token::Int(-7)));
        assert_eq!(lexer.next_token(), Some(Token::Int(0)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn lex_bytes_literal() {
        let mut lexer = Lexer::new("0xDEADBEEF");

        assert_eq!(lexer.next_token(), Some(Token::Bytes(vec![0xDE, 0xAD, 0xBE, 0xEF])));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn lex_string_with_escapes() {
        let mut lexer = Lexer::new(r#""hello\n\"world\"""#);

        assert_eq!(lexer.next_token(), Some(Token::String("hello\n\"world\"".into())));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn lex_mixed_input() {
        let mut lexer = Lexer::new(r#"
            user {
                id: 42,
                active: true,
                hash: 0xFF00
            }
        "#);

        use Token::*;

        let exp = [
            Ident("user".into()),
            LBrace,
            Ident("id".to_string()),
            Colon,
            Int(42),
            Comma,
            Ident("active".to_string()),
            Colon,
            True,
            Comma,
            Ident("hash".to_string()),
            Colon,
            Bytes(vec![0xFF, 0x00]),
            RBrace
        ];

        for tok in exp {
            assert_eq!(lexer.next_token(), Some(tok));
        }

        assert_eq!(lexer.next_token(), None);
    }
}
