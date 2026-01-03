use crate::error::{ParseError, ParseErrorKind, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Literals:
    Null,
    True,
    False,
    Int(i64),
    String(String),
    Bytes(Vec<u8>),

    // Identifiers:
    Ident(String),

    // Punctuation:
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,

    EOF,
}

pub struct Lexer<'a> {
    input: &'a [u8],
    offset: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            offset: 0,
            line: 1,
            column: 1,
        }
    }

    fn span(&self) -> Span {
        Span {
            offset: self.offset,
            line: self.line,
            column: self.column,
        }
    }

    fn error(&self, kind: ParseErrorKind) -> ParseError {
        ParseError {
            kind,
            span: self.span(),
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.offset).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.offset += 1;

        if byte == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(byte)
    }

    fn hex_digit(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }

    fn skip_ignored(&mut self) {
        loop {
            //skip whitespace
            while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
                self.bump();
            }

            // line comment with //
            if self.peek() == Some(b'/') && self.input.get(self.offset + 1) == Some(&b'/') {
                self.bump();
                self.bump();
                while let Some(current_byte) = self.peek() {
                    self.bump();
                    if current_byte == b'\n' {
                        break;
                    }
                }
                continue;
            }

            // line comment with #
            if self.peek() == Some(b'#') {
                self.bump();
                while let Some(current_byte) = self.peek() {
                    self.bump();
                    if current_byte == b'\n' {
                        break;
                    }
                }
                continue;
            }

            break;
        }
    }

    fn lex_identifier(&mut self) -> TokenKind {
        let start = self.offset;
        self.bump();

        while matches!(
            self.peek(),
            Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')
        ) {
            self.bump();
        }

        let ident = std::str::from_utf8(&self.input[start..self.offset])
            .unwrap()
            .to_string();

        match ident.as_str() {
            "null" => TokenKind::Null,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Ident(ident),
        }
    }

    fn lex_int(&mut self) -> Result<TokenKind, ParseError> {
        let start = self.offset;

        // opt leading '-'
        if self.peek() == Some(b'-') {
            self.bump();
        }

        let mut saw_digit = false;

        while matches!(self.peek(), Some(b'0'..=b'9')) {
            saw_digit = true;
            self.bump();
        }

        // must have at least 1 digit
        if !saw_digit {
            return Err(self.error(ParseErrorKind::IntegerOutOfRange));
        }

        let slice = &self.input[start..self.offset];
        let text = std::str::from_utf8(slice)
            .map_err(|_| self.error(ParseErrorKind::IntegerOutOfRange))?;

        let parsed_value = text
            .parse::<i64>()
            .map_err(|_| self.error(ParseErrorKind::IntegerOutOfRange))?;

        Ok(TokenKind::Int(parsed_value))
    }

    fn lex_bytes(&mut self) -> Result<TokenKind, ParseError> {
        // must start with 0x
        if self.peek() != Some(b'0') || self.input.get(self.offset + 1) != Some(&b'x') {
            return Err(self.error(ParseErrorKind::MalformedBytesLiteral));
        }

        // consume 0x
        self.bump();
        self.bump();

        let hex_start = self.offset;

        while matches!(self.peek(), Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) {
            self.bump();
        }

        let hex_len = self.offset - hex_start;

        // must have even number of hex digits and at least one byte
        if hex_len == 0 || hex_len % 2 != 0 {
            return Err(self.error(ParseErrorKind::MalformedBytesLiteral));
        }

        let hex = &self.input[hex_start..self.offset];
        let mut bytes = Vec::with_capacity(hex_len / 2);

        for i in (0..hex_len).step_by(2) {
            let high_nibble = Self::hex_digit(hex[i])
                .ok_or_else(|| self.error(ParseErrorKind::MalformedBytesLiteral))?;

            let low_nibble = Self::hex_digit(hex[i + 1])
                .ok_or_else(|| self.error(ParseErrorKind::MalformedBytesLiteral))?;

            bytes.push((high_nibble << 4) | low_nibble);
        }

        Ok(TokenKind::Bytes(bytes))
    }

    fn lex_string(&mut self) -> Result<TokenKind, ParseError> {
        self.bump(); // opening '"'

        let mut out = String::new();

        while let Some(current_byte) = self.peek() {
            match current_byte {
                b'"' => {
                    // closing quote
                    self.bump();
                    return Ok(TokenKind::String(out));
                }

                b'\\' => {
                    //escape seq
                    self.bump();
                    let escape_char = self
                        .bump()
                        .ok_or_else(|| self.error(ParseErrorKind::MalformedBytesLiteral))?;

                    match escape_char {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),

                        b'u' => {
                            // \uXXXX
                            let mut codepoint = 0u32;

                            for _ in 0..4 {
                                let hex_byte = self.bump().ok_or_else(|| {
                                    self.error(ParseErrorKind::MalformedBytesLiteral)
                                })?;
                                let digit = Self::hex_digit(hex_byte).ok_or_else(|| {
                                    self.error(ParseErrorKind::MalformedBytesLiteral)
                                })?;
                                codepoint = (codepoint << 4) | (digit as u32);
                            }

                            let unicode_char = char::from_u32(codepoint)
                                .ok_or_else(|| self.error(ParseErrorKind::MalformedBytesLiteral))?;
                            out.push(unicode_char);
                        }
                        _ => return Err(self.error(ParseErrorKind::MalformedBytesLiteral)),
                    }
                }

                b'\n' | b'\r' => {
                    // strings cant span lines
                    return Err(self.error(ParseErrorKind::MalformedBytesLiteral));
                }

                _ => {
                    if current_byte >= 0x80 {
                        return Err(self.error(ParseErrorKind::MalformedBytesLiteral));
                    }

                    out.push(current_byte as char);
                    self.bump();
                }
            }
        }

        Err(self.error(ParseErrorKind::MalformedBytesLiteral))
    }

    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_ignored();

        let start = self.span();

        let current_byte = match self.peek() {
            Some(b) => b,
            None => {
                return Ok(Token {
                    kind: TokenKind::EOF,
                    span: start,
                });
            }
        };

        let kind = match current_byte {
            b'{' => {
                self.bump();
                TokenKind::LBrace
            }
            b'}' => {
                self.bump();
                TokenKind::RBrace
            }
            b'[' => {
                self.bump();
                TokenKind::LBracket
            }
            b']' => {
                self.bump();
                TokenKind::RBracket
            }
            b':' => {
                self.bump();
                TokenKind::Colon
            }
            b',' => {
                self.bump();
                TokenKind::Comma
            }

            // string literal
            b'"' => self.lex_string()?,

            // bytes literal
            b'0' if self.input.get(self.offset + 1) == Some(&b'x') => self.lex_bytes()?,

            // integer literal
            b'-' | b'0'..=b'9' => self.lex_int()?,

            // identifier or keyword
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.lex_identifier(),

            _ => {
                return Err(self.error(ParseErrorKind::UnexpectedToken {
                    expected: "valid token",
                    found: "invalid character",
                }));
            }
        };

        Ok(Token { kind, span: start })
    }
}
