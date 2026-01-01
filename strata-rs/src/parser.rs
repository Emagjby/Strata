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

    fn parse_list(&mut self) -> Option<Value> {
        // expect '['
        if self.lookahead != Some(Token::LBracket) {
            return None;
        }
        self.advance(); // consume '['

        let mut items = Vec::new();

        // empty list
        if self.lookahead == Some(Token::RBracket) {
            self.advance(); // consume ']'
            return Some(Value::List(items));
        }

        loop {
            // parse value
            let val = self.parse_value()?;
            items.push(val);

            match self.lookahead {
                Some(Token::Comma) => {
                    self.advance(); // consume ','

                    // allow trailing comma
                    if self.lookahead == Some(Token::RBracket) {
                        break;
                    }
                }
                
                Some(Token::RBracket) => break,
                _ => return None,
            }
        }

        // consume closing ']'
        if self.lookahead != Some(Token::RBracket) {
            return None;
        }
        self.advance();

        Some(Value::List(items))
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

            Token::LBracket => self.parse_list(),

            _ => None,
        }
    }
}
