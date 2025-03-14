use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum RESPError {
    OutOfBounds(usize),
    FromUtf8,
    WrongType,
}
pub type RESPResult<T> = Result<T, RESPError>;

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::FromUtf8 => write!(f, "can't convert from UTF-8"),
            RESPError::OutOfBounds(index) => write!(f, "Index {} is out of bounds", index),
            RESPError::WrongType => write!(f, "wrong type"),
        }
    }
}
