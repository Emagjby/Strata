use std::io;

#[derive(Debug)]
pub enum StrataError {
    Parse(ParseError),
    Encode(EncodeError),
    Decode(DecodeError),
    Io(io::Error),
    Internal(&'static str),
}

// Decode errors
#[derive(Debug, PartialEq, Eq)]
pub struct DecodeError {
    pub kind: DecodeErrorKind,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeErrorKind {
    InvalidTag(u8),
    UnexpectedEOF,
    InvalidVarint,
    InvalidUtf8,
    TrailingBytes,
}

// Parse errors
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
    // syntax
    UnexpectedToken {
        expected: &'static str,
        found: &'static str,
    },
    MalformedBytesLiteral,

    // semantic
    IntegerOutOfRange,
}

// Encode errors
#[derive(Debug, PartialEq, Eq)]
pub enum EncodeError {
    DuplicateKey,
    // InvalidUtf8 is unreachable in Rust because String
    // is UTF-8 by construction.
    // It exists for spec completeness and non-Rust
    // implementations.
    InvalidUtf8,
    InvalidInteger,
}

impl From<ParseError> for StrataError {
    fn from(err: ParseError) -> Self {
        StrataError::Parse(err)
    }
}

impl From<EncodeError> for StrataError {
    fn from(err: EncodeError) -> Self {
        StrataError::Encode(err)
    }
}

impl From<DecodeError> for StrataError {
    fn from(err: DecodeError) -> Self {
        StrataError::Decode(err)
    }
}

impl From<io::Error> for StrataError {
    fn from(err: io::Error) -> Self {
        StrataError::Io(err)
    }
}
