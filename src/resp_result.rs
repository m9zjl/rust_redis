use std::fmt::Formatter;
use std::string::FromUtf8Error;
use std::{fmt, usize};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RESPError {
    OutOfBounds(usize),
    FromUtf8,
    WrongType,
    Unknown,
}
pub type RESPResult<T> = Result<T, RESPError>;

pub fn resp_remove_type(value: char, buffer: &[u8], index: &mut usize) -> RESPResult<()> {
    if buffer[*index] != value as u8 {
        return Err(RESPError::WrongType);
    }
    *index += 1;
    Ok(())
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::FromUtf8 => write!(f, "can't convert from UTF-8"),
            RESPError::OutOfBounds(index) => write!(f, "Index {} is out of bounds", index),
            RESPError::WrongType => write!(f, "wrong type"),
        }
    }
}

impl From<FromUtf8Error> for RESPError {
    fn from(_value: FromUtf8Error) -> Self {
        return Self::FromUtf8;
    }
}
