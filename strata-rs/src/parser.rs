use crate::lexer::{Lexer, Token};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    lookahead: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let lookahead = lexer.next_token();
        Self { lexer, lookahead }
    }

    fn advance(&mut self) {
        self.lookahead = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) -> Option<()> {
        if self.lookahead == Some(expected) {
            self.advance();
            Some(())
        } else {
            None
        }
    }

    pub fn parse_value(&mut self) -> Option<Value> {
        match self.lookahead.clone()? {
            Token::Null => {
                self.advance();
                Some(Value::Null)
            }

            Token::True => {
                self.advance();
                Some(Value::Bool(true))
            }

            Token::False => {
                self.advance();
                Some(Value::Bool(false))
            }

            Token::Int(n) => {
                self.advance();
                Some(Value::Int(n))
            }

            Token::String(s) => {
                self.advance();
                Some(Value::String(s))
            }


            Token::Bytes(b) => {
                self.advance();
                Some(Value::Bytes(b))
            }

            _ => None,
        }
    }
}
