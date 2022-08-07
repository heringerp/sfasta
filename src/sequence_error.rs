use std::{fmt, error::Error, io};

#[derive(Debug)]
pub enum SequenceError {
    FileNotFound,
    InvalidSymbol
}

impl Error for SequenceError {}

impl fmt::Display for SequenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileNotFound => write!(f, "File could not be found"),
            Self::InvalidSymbol => write!(f, "Sequence contains an invalid symbol")
        }
    }
}

impl From<io::Error> for SequenceError {
    fn from(err: io::Error) -> SequenceError {
        SequenceError::FileNotFound
    }
}
