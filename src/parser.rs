use anyhow::{Error, Result};

mod util;

pub enum RedisType {
    Array(Vec<RedisType>),
    BulkString(Option<Vec<u8>>),
    Error(String),
    Integer(i64),
    SimpleString(String),
}

pub struct Parsed {
    redis_type: RedisType,
    bytes_read: usize,
}

pub fn parse_resp(buffer: Vec<u8>) -> Result<Parsed> {
    let first_byte = buffer[0];
    let buffer = &buffer[1..];
    match first_byte {
        b'+' => parse_simple_string(buffer),
        // b':' => parse_integer(buffer),
        // b'-' => parse_error(bytes),
        // b'$' => parse_bulk_string(bytes),
        // b'*' => parse_array(bytes),
        _ => panic!("Invalid RESP type"),
    }
}

fn parse_simple_string(buffer: &[u8]) -> Result<Parsed> {
    let bytes_read = match util::read_until_crlf(buffer) {
        Some(b) => b,
        None => return Err(Error::msg("failed to read bytes")),
    };

    let out_str = std::str::from_utf8(bytes_read)?.to_string();
    return Ok(Parsed {
        bytes_read: bytes_read.len(),
        redis_type: RedisType::SimpleString(out_str),
    });
}

#[test]
fn test_parsing_simple_string() {
    let parsed = parse_simple_string(b"OK\r\n").unwrap();
    match parsed.redis_type {
        RedisType::SimpleString(x) => assert_eq!(x, String::from("OK")),
        _ => panic!("Incorrect type"),
    }
}

// fn parse_integer(buffer: &[u8]) -> Result<Parsed> {
//     return RedisType::Integer(i64::from_ne_bytes(
//         bytes[1..bytes.len() - 2].try_into().unwrap(),
//     ));
// }

// *<number-of-elements>\r\n<element-1>...<element-n>
// fn parse_array(bytes: Vec<u8>) -> Vec<Resp> {}
