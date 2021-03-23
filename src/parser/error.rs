use std::convert::From;
use std::error::Error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum ParseError {
    InvalidDigit(String),
    InvalidFormat(String),
    UnknownFileExtension(String),
    IoError(io::Error),
    Empty,
}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidDigit(digit) => write!(f, "Invalid digit: {}", digit),
            Self::InvalidFormat(line) => write!(f, "Invalid format: {}", line),
            Self::UnknownFileExtension(format) => write!(f, "Unknown file format: {}", format),
            Self::IoError(io_error) => write!(f, "IO Error occured: {}", io_error),
            Self::Empty => write!(f, "Empty string"),
        }
    }
}

impl Error for ParseError {}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        ParseError::IoError(error)
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(error: num::ParseIntError) -> Self {
        ParseError::InvalidDigit(error.to_string())
    }
}
