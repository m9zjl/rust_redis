use crate::resp_result::{RESPError, RESPResult, resp_remove_type};
use std::fmt;
use std::fmt::write;

#[derive(Debug, PartialEq)]
pub enum RESP {
    SimpleString(String),
}

impl fmt::Display for RESP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match self {
            Self::SimpleString(data) => format!("{}\r\n", data),
        };
        write!(f, "{}", data)
    }
}

fn binary_extract_line_as_string(buffer: &[u8], index: &mut usize) -> RESPResult<String> {
    let line = binary_extract_line(buffer, index)?;
    Ok(String::from_utf8(line).unwrap())
}

fn parse_simple_string(buffer: &[u8], index: &mut usize) -> RESPResult<RESP> {
    resp_remove_type('+', buffer, index)?;

    let line = binary_extract_line_as_string(buffer, index)?;

    return Ok(RESP::SimpleString(line));
}

fn binary_extract_line(buffer: &[u8], index: &mut usize) -> RESPResult<Vec<u8>> {
    let mut output: Vec<u8> = Vec::new();
    if *index >= buffer.len() {
        return Err(RESPError::OutOfBounds(*index));
    }

    // if there is not enough space for /r/n return Error
    if buffer.len() - *index <= 2 {
        *index = buffer.len();
        return Err(RESPError::OutOfBounds(*index));
    }
    let mut separator_found: bool = false;
    let mut final_index: usize = *index;
    let left = *index;
    for i in left..buffer.len() {
        final_index = i;
        if b'\n' == buffer[i] && b'\r' == buffer[i - 1] {
            separator_found = true;
            break;
        }
    }

    *index = final_index + 1;
    if !separator_found {
        return Err(RESPError::OutOfBounds(*index));
    };
    output.extend_from_slice(&buffer[left..final_index - 1]);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::resp_result::resp_remove_type;

    use super::*;

    #[test]
    fn test_parse_simple_string() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index: usize = 0;
        let output = parse_simple_string(buffer, &mut index).unwrap();

        assert_eq!(output, RESP::SimpleString(String::from("OK")));
        assert_eq!(index, 5);
    }

    #[test]
    fn test_binary_remove_type() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index: usize = 0;
        resp_remove_type('+', buffer, &mut index).unwrap();
        assert_eq!(index, 1)
    }

    #[test]
    fn test_binary_remove_type_error() {
        let buffer = "*OK\r\n".as_bytes();
        let mut index: usize = 0;
        let error = resp_remove_type('+', buffer, &mut index).unwrap_err();
        assert_eq!(index, 0);
        assert_eq!(error, RESPError::WrongType);
    }

    #[test]
    fn test_extract_line() {
        let result = binary_extract_line(b"get a \r\n", &mut 0);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), b"get a ");
    }

    #[test]
    fn test_binary_extract_line_empty_buffer() {
        let buffer = "".as_bytes();
        let mut index: usize = 0;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 0);
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line_single_character() {
        let buffer = "O".as_bytes();
        let mut index: usize = 0;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 1);
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line_index_too_advanced() {
        let buffer = "OK".as_bytes();
        let mut index: usize = 1;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 2);
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line_no_separator() {
        let buffer = "OK".as_bytes();
        let mut index: usize = 0;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 2);
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line_half_separator() {
        let buffer = "OK\r".as_bytes();
        let mut index: usize = 0;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line_incorrect_separator() {
        let buffer = "OK\n".as_bytes();
        let mut index: usize = 0;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line() {
        let buffer = "OK\r\n".as_bytes();
        let mut index: usize = 0;
        let output = binary_extract_line(buffer, &mut index).unwrap();
        assert_eq!(output, "OK".as_bytes());
        assert_eq!(index, 4);
    }
}
