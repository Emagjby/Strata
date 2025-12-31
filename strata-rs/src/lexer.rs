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

    fn hex_digit(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
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
                while let Some(b) = self.peek() {
                    self.pos += 1;
                    if b == b'\n' {
                        break;
                    }
                }
                continue;
            }

            // line comment with #
            if self.peek() == Some(b'#') {
                self.pos += 1;
                while let Some(b) = self.peek() {
                    self.pos += 1;
                    if b == b'\n' {
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

        while let Some(b) = self.peek() {
            if matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_') {
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

        while let Some(b) = self.peek() {
            if matches!(b, b'0'..=b'9') {
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

        let val = text.parse::<i64>().ok()?;

        Some(Token::Int(val))
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

        while let Some(b) = self.peek() {
            if matches!(b, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') {
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
            let hi = Self::hex_digit(hex[i])?;
            let lo = Self::hex_digit(hex[i + 1])?;
            bytes.push((hi << 4) | lo);
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

        while let Some(b) = self.peek() {
            match b {
                b'"' => {
                    // closing quote
                    self.pos += 1;
                    return Some(Token::String(out));
                }

                b'\\' => {
                    //escape seq
                    self.pos += 1;
                    let esc = self.peek()?;
                    self.pos += 1;

                    match esc {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),

                        b'u' => {
                            // \uXXXX
                            let mut codepoint: u32 = 0;

                            for _ in 0..4 {
                                let h = self.peek()?;
                                self.pos += 1;
                                let v = Self::hex_digit(h)? as u32;
                                codepoint = (codepoint << 4) | v;
                            }

                            let ch = char::from_u32(codepoint)?;
                            out.push(ch);
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
                    if b >= 0x80 {
                        return None; // non-ASCII not allowed in v1 str
                    }

                    out.push(b as char);
                    self.pos += 1;
                }
            }
        }

        None
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_ignored();

        let b = self.peek()?;

        match b {
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
