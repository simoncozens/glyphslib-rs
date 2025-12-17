use serde::{de, ser};
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

impl LineColumn {
    pub fn from_pos(s: &str, pos: usize) -> Self {
        let mut line = 1usize;
        let mut col = 1usize;
        let bytes = s.as_bytes();
        let mut i = 0usize;
        while i < pos && i < bytes.len() {
            if bytes[i] == b'\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
            i += 1;
        }
        LineColumn { line, column: col }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Unexpected character '{ch}' at line {}, column {}", .lc.line, .lc.column)]
    UnexpectedChar { ch: char, lc: LineColumn },
    #[error("Unterminated string at line {}, column {}", .lc.line, .lc.column)]
    UnclosedString { lc: LineColumn },
    #[error("Unterminated data block at line {}, column {}", .lc.line, .lc.column)]
    UnclosedData { lc: LineColumn },
    // Internal-use variants (no position); convert with Error::at()
    #[error("Data block did not contain valid paired hex digits")]
    BadDataInternal,
    #[error("Unknown escape code")]
    UnknownEscapeInternal,
    #[error("Invalid unicode escape sequence: '{seq}'")]
    InvalidUnicodeEscapeInternal { seq: String },
    #[error("Expected string, found '{token_name}'")]
    NotAStringInternal { token_name: &'static str },
    #[error("Data block did not contain valid paired hex digits at line {}, column {}", .lc.line, .lc.column)]
    BadData { lc: LineColumn },
    #[error("Unknown escape code at line {}, column {}", .lc.line, .lc.column)]
    UnknownEscape { lc: LineColumn },
    #[error("Invalid unicode escape sequence: '{seq}' at line {}, column {}", .lc.line, .lc.column)]
    InvalidUnicodeEscape { seq: String, lc: LineColumn },
    #[error("Expected string, found '{token_name}' at line {}, column {}", .lc.line, .lc.column)]
    NotAString {
        token_name: &'static str,
        lc: LineColumn,
    },
    #[error("Missing '=' at line {}, column {}", .lc.line, .lc.column)]
    ExpectedEquals { lc: LineColumn },
    #[error("Missing ',' at line {}, column {}", .lc.line, .lc.column)]
    ExpectedComma { lc: LineColumn },
    #[error("Missing ';' at line {}, column {}", .lc.line, .lc.column)]
    ExpectedSemicolon { lc: LineColumn },
    #[error("Missing '{{' at line {}, column {}", .lc.line, .lc.column)]
    ExpectedOpenBrace { lc: LineColumn },
    #[error("Missing '}}' at line {}, column {}", .lc.line, .lc.column)]
    ExpectedCloseBrace { lc: LineColumn },
    #[error("Missing '(' at line {}, column {}", .lc.line, .lc.column)]
    ExpectedOpenParen { lc: LineColumn },
    #[error("Missing ')' at line {}, column {}", .lc.line, .lc.column)]
    ExpectedCloseParen { lc: LineColumn },
    #[error("Expected character '{ch}' at line {}, column {}", .lc.line, .lc.column)]
    ExpectedChar { ch: char, lc: LineColumn },
    #[error("Expected numeric value at line {}, column {}", .lc.line, .lc.column)]
    ExpectedNumber { lc: LineColumn },
    #[error("Expected string value at line {}, column {}", .lc.line, .lc.column)]
    ExpectedString { lc: LineColumn },
    #[error("Expected '{expected}', found '{found}'")]
    UnexpectedDataType {
        expected: &'static str,
        found: &'static str,
    },
    #[error("Unexpected token '{name}' at line {}, column {}", .lc.line, .lc.column)]
    UnexpectedToken { name: &'static str, lc: LineColumn },
    #[error("parsing failed: '{0}'")]
    Parse(String),
    #[error("serializing failed: '{0}'")]
    Serialize(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Serialize(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Parse(msg.to_string())
    }
}

impl Error {
    pub fn at(self, s: &str, pos: usize) -> Self {
        let lc = LineColumn::from_pos(s, pos);
        match self {
            Error::BadDataInternal => Error::BadData { lc },
            Error::UnknownEscapeInternal => Error::UnknownEscape { lc },
            Error::InvalidUnicodeEscapeInternal { seq } => Error::InvalidUnicodeEscape { seq, lc },
            Error::NotAStringInternal { token_name } => Error::NotAString { token_name, lc },
            other => other,
        }
    }
}
