use crate::lexer::{Lexer, Token};
use std::collections::BTreeMap;
use crate::value::Value;

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
            let element = self.parse_value()?;
            items.push(element);

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

    fn parse_map(&mut self) -> Option<Value> {
        // expect '{'
        if self.lookahead != Some(Token::LBrace) {
            return None;
        }
        self.advance(); // consume '{'
        
        let mut map = BTreeMap::new();

        // empty map
        if self.lookahead == Some(Token::RBrace) {
            self.advance(); // consume '}'
            return Some(Value::Map(map));
        }

        loop {
            // key must be identifier
            let key = match self.lookahead.clone()? {
                Token::Ident(name) => {
                    self.advance();
                    name
                }

                _ => return None,
            };

            let value = if self.lookahead == Some(Token::LBrace) {
                // shorthand entry: key { ... }
                self.parse_map()?
            } else {
                // normal entry: key : value
                if self.lookahead != Some(Token::Colon) {
                    return None;
                }
                self.advance(); // consume ':'
                self.parse_value()?
            };

            map.insert(key, value);

            match self.lookahead {
                Some(Token::Comma) => {
                    self.advance();

                    // allow trailing comma
                    if self.lookahead == Some(Token::RBrace) {
                        break;
                    }
                }

                Some(Token::RBrace) => break,

                Some(Token::Ident(_)) => {
                    // continue parsing
                }

                _ => return None,
            }
        }

        // consume closing '}'
        if self.lookahead != Some(Token::RBrace) {
            return None;
        }
        self.advance();

        Some(Value::Map(map))
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

            Token::Int(number) => {
                self.advance();
                Some(Value::Int(number))
            }

            Token::String(string) => {
                self.advance();
                Some(Value::String(string))
            }


            Token::Bytes(bytes) => {
                self.advance();
                Some(Value::Bytes(bytes))
            }

            Token::Ident(name) => {
                self.advance();

                // identifier followed by '{' -> shorthand
                if self.lookahead == Some(Token::LBrace) {
                    let inner = self.parse_map()?;

                    let mut map = BTreeMap::new();
                    map.insert(name, inner);

                    Some(Value::Map(map))
                } else {
                    None
                }
            }

            Token::LBracket => self.parse_list(),
            Token::LBrace => self.parse_map(),

            _ => None,
        }
    }
}

pub fn parse(input: &str) -> Option<Value> {
    let mut parser = Parser::new(input);

    let parsed_value = parser.parse_value()?;

    if parser.lookahead.is_some() {
        return None;
    }

    Some(parsed_value)
}
