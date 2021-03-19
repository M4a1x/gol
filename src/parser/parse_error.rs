use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    InvalidDigit(String),
    InvalidFormat(String),
    UnknownFileExtension(String),
    Empty,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidDigit(digit) => write!(f, "Invalid digit: {}", digit),
            Self::InvalidFormat(line) => write!(f, "Invalid format: {}", line),
            Self::UnknownFileExtension(format) => write!(f, "Unknown file format: {}", format),
            Self::Empty => write!(f, "Empty string"),
        }
    }
}

impl Error for ParseError {}
