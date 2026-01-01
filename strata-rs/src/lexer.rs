#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
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
    Comma
}

pub struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
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
                self.pos += 1;
            }

            // line comment with //
            if self.peek() == Some(b'/') && self.input.get(self.pos + 1) == Some(&b'/') {
                self.pos += 2;
                while let Some(current_byte) = self.peek() {
                    self.pos += 1;
                    if current_byte == b'\n' {
                        break;
                    }
                }
                continue;
            }

            // line comment with #
            if self.peek() == Some(b'#') {
                self.pos += 1;
                while let Some(current_byte) = self.peek() {
                    self.pos += 1;
                    if current_byte == b'\n' {
                        break;
                    }
                }
                continue;
            }

            break;
        }
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.pos;

        // first char is guaranteed to be [A-Za-z_]
        self.pos += 1;

        while let Some(current_byte) = self.peek() {
            if matches!(current_byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_') {
                self.pos += 1;
            } else {
                break;
            }
        }

        let ident = std::str::from_utf8(&self.input[start..self.pos])
            .expect("identifier must be valid ASCII")
            .to_string();

        match ident.as_str() {
            "null" => Token::Null,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident),
        }
    }

    fn lex_int(&mut self) -> Option<Token> {
        let start = self.pos;

        // opt leading '-'
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }

        let mut saw_digit = false;

        while let Some(current_byte) = self.peek() {
            if matches!(current_byte, b'0'..=b'9') {
                saw_digit = true;
                self.pos += 1;
            } else {
                break;
            }
        }

        // must have at least 1 digit
        if !saw_digit {
            self.pos = start;
            return None;
        }

        let slice = &self.input[start..self.pos];
        let text = std::str::from_utf8(slice).ok()?;

        let parsed_value = text.parse::<i64>().ok()?;

        Some(Token::Int(parsed_value))
    }

    fn lex_bytes(&mut self) -> Option<Token> {
        let start = self.pos;

        // must start with 0x
        if self.peek() != Some(b'0') || self.input.get(self.pos + 1) != Some(&b'x') {
            return None;
        }

        // consume 0x
        self.pos += 2;

        let hex_start = self.pos;

        while let Some(current_byte) = self.peek() {
            if matches!(current_byte, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') {
                self.pos += 1;
            } else {
                break;
            }
        }

        let hex_len = self.pos - hex_start;

        // must have even number of hex digits and at least one byte
        if hex_len == 0 || hex_len % 2 != 0 {
            self.pos = start;
            return None;
        }

        let hex = &self.input[hex_start..self.pos];
        let mut bytes = Vec::with_capacity(hex_len / 2);

        for i in (0..hex_len).step_by(2) {
            let high_nibble = Self::hex_digit(hex[i])?;
            let low_nibble = Self::hex_digit(hex[i + 1])?;
            bytes.push((high_nibble << 4) | low_nibble);
        }

        Some(Token::Bytes(bytes))
    }

    fn lex_string(&mut self) -> Option<Token> {
        // must start with "
        if self.peek() != Some(b'"') {
            return None;
        }

        //consume "
        self.pos += 1;

        let mut out = String::new();

        while let Some(current_byte) = self.peek() {
            match current_byte {
                b'"' => {
                    // closing quote
                    self.pos += 1;
                    return Some(Token::String(out));
                }

                b'\\' => {
                    //escape seq
                    self.pos += 1;
                    let escape_char = self.peek()?;
                    self.pos += 1;

                    match escape_char {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),

                        b'u' => {
                            // \uXXXX
                            let mut codepoint: u32 = 0;

                            for _ in 0..4 {
                                let hex_char = self.peek()?;
                                self.pos += 1;
                                let digit_value = Self::hex_digit(hex_char)? as u32;
                                codepoint = (codepoint << 4) | digit_value;
                            }

                            let unicode_char = char::from_u32(codepoint)?;
                            out.push(unicode_char);
                        }

                        _ => return None, // invalid escape
                    }
                }

                b'\n' | b'\r' => {
                    // strings cant span lines
                    return None;
                }

                _ => {
                    // regular UTF-8 byte
                    if current_byte >= 0x80 {
                        return None; // non-ASCII not allowed in v1 str
                    }

                    out.push(current_byte as char);
                    self.pos += 1;
                }
            }
        }

        None
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_ignored();

        let current_byte = self.peek()?;

        match current_byte {
            b'{' => { self.pos += 1; Some(Token::LBrace) }
            b'}' => { self.pos += 1; Some(Token::RBrace) }
            b'[' => { self.pos += 1; Some(Token::LBracket) }
            b']' => { self.pos += 1; Some(Token::RBracket) }
            b':' => { self.pos += 1; Some(Token::Colon) }
            b',' => { self.pos += 1; Some(Token::Comma) }

            // string literal
            b'"' => {
                self.lex_string()
            }

            // bytes literal
            b'0' if self.input.get(self.pos + 1) == Some(&b'x') => {
                self.lex_bytes()
            }

            // integer literal
            b'-' | b'0'..=b'9' => {
                self.lex_int()
            } 

            // identifier or keyword
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                Some(self.lex_identifier())
            }

            _ => {
                None
            }
        }
    }
}
