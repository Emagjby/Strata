use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::value::Value;
use std::collections::BTreeMap;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    lookahead: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(input);
        let lookahead = lexer.next_token()?;

        Ok(Self { lexer, lookahead })
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        self.lookahead = self.lexer.next_token()?;
        Ok(())
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.lookahead.kind == kind {
            self.advance()
        } else {
            Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken {
                    expected: "different token",
                    found: "unexpected token",
                },
                span: self.lookahead.span,
            })
        }
    }

    fn parse_list(&mut self) -> Result<Value, ParseError> {
        self.expect(TokenKind::LBracket)?;

        let mut items = Vec::new();

        // empty list
        if self.lookahead.kind == TokenKind::RBracket {
            self.advance()?; // consume ']'
            return Ok(Value::List(items));
        }

        loop {
            // parse value
            let element = self.parse_value()?;
            items.push(element);

            match self.lookahead.kind {
                TokenKind::Comma => {
                    self.advance()?; // consume ','

                    // allow trailing comma
                    if self.lookahead.kind == TokenKind::RBracket {
                        break;
                    }
                }

                TokenKind::RBracket => break,

                _ => {
                    return Err(ParseError {
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: "',' or ']'",
                            found: "token",
                        },
                        span: self.lookahead.span,
                    });
                }
            }
        }

        self.expect(TokenKind::RBracket)?;
        Ok(Value::List(items))
    }

    fn parse_map(&mut self) -> Result<Value, ParseError> {
        self.expect(TokenKind::LBrace)?;

        let mut map = BTreeMap::new();

        // empty map
        if self.lookahead.kind == TokenKind::RBrace {
            self.advance()?; // consume '}'
            return Ok(Value::Map(map));
        }

        loop {
            // key must be identifier
            let key = match &self.lookahead.kind {
                TokenKind::Ident(name) => {
                    let key = name.clone();
                    self.advance()?;
                    key
                }

                _ => {
                    return Err(ParseError {
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: "identifier",
                            found: "token",
                        },
                        span: self.lookahead.span,
                    });
                }
            };

            let value = if self.lookahead.kind == TokenKind::LBrace {
                // shorthand entry: key { ... }
                self.parse_map()?
            } else {
                // normal entry: key : value
                self.expect(TokenKind::Colon)?;
                self.parse_value()?
            };

            map.insert(key, value);

            match self.lookahead.kind {
                TokenKind::Comma => {
                    self.advance()?;

                    // allow trailing comma
                    if self.lookahead.kind == TokenKind::RBrace {
                        break;
                    }
                }

                TokenKind::RBrace => break,

                TokenKind::Ident(_) => {
                    // implicit separator via newline
                    continue;
                }

                _ => {
                    return Err(ParseError {
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: "',' or '}'",
                            found: "token",
                        },
                        span: self.lookahead.span,
                    });
                }
            }
        }

        self.expect(TokenKind::RBrace)?;
        Ok(Value::Map(map))
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        match &self.lookahead.kind {
            TokenKind::Null => {
                self.advance()?;
                Ok(Value::Null)
            }

            TokenKind::True => {
                self.advance()?;
                Ok(Value::Bool(true))
            }

            TokenKind::False => {
                self.advance()?;
                Ok(Value::Bool(false))
            }

            TokenKind::Int(number) => {
                let v = *number;
                self.advance()?;
                Ok(Value::Int(v))
            }

            TokenKind::String(string) => {
                let v = string.clone();
                self.advance()?;
                Ok(Value::String(v))
            }

            TokenKind::Bytes(bytes) => {
                let v = bytes.clone();
                self.advance()?;
                Ok(Value::Bytes(v))
            }

            TokenKind::LBracket => self.parse_list(),
            TokenKind::LBrace => self.parse_map(),

            TokenKind::Ident(name) => {
                let key = name.clone();
                self.advance()?;

                // identifier followed by '{' -> shorthand
                if self.lookahead.kind == TokenKind::LBrace {
                    let inner = self.parse_map()?;
                    let mut map = BTreeMap::new();
                    map.insert(key, inner);
                    Ok(Value::Map(map))
                } else {
                    Err(ParseError {
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: "map or value",
                            found: "identifier",
                        },
                        span: self.lookahead.span,
                    })
                }
            }

            _ => Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken {
                    expected: "value",
                    found: "token",
                },
                span: self.lookahead.span,
            }),
        }
    }
}

pub fn parse(input: &str) -> Result<Value, ParseError> {
    let mut parser = Parser::new(input)?;
    let parsed_value = parser.parse_value()?;

    if parser.lookahead.kind != TokenKind::EOF {
        return Err(ParseError {
            kind: ParseErrorKind::UnexpectedToken {
                expected: "EOF",
                found: "extra input",
            },
            span: parser.lookahead.span,
        });
    }

    Ok(parsed_value)
}
