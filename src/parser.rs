use anyhow::{Error, Result};

mod util;

pub enum RedisType {
    Array(Vec<RedisType>),
    BulkString(String),
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
        b':' => parse_integer(buffer),
        b'$' => parse_bulk_string(buffer),
        // b'*' => parse_array(bytes),
        // b'-' => parse_error(bytes),
        _ => panic!("Invalid RESP type"),
    }
}

fn parse_simple_string(buffer: &[u8]) -> Result<Parsed> {
    let bytes_read = util::read_until_crlf(buffer).unwrap();

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
        RedisType::SimpleString(x) => {
            assert_eq!(x, String::from("OK"));
            assert_eq!(parsed.bytes_read, 2);
        }
        _ => panic!("Incorrect type"),
    }
}

fn parse_integer(buffer: &[u8]) -> Result<Parsed> {
    let bytes_read = util::read_until_crlf(buffer).unwrap();
    let int = i64::from_str_radix(std::str::from_utf8(bytes_read)?, 10)?;

    return Ok(Parsed {
        redis_type: RedisType::Integer(int),
        bytes_read: bytes_read.len(),
    });
}

#[test]
fn test_parsing_negative_integer() {
    let parsed = parse_integer(b"-1000\r\n").unwrap();
    match parsed.redis_type {
        RedisType::Integer(x) => {
            assert_eq!(x, -1000);
            assert_eq!(parsed.bytes_read, 5);
        }
        _ => panic!("Incorrect type"),
    }
}

// <length>\r\n<data>\r\n
fn parse_bulk_string(buffer: &[u8]) -> Result<Parsed> {
    let parsed_len = parse_integer(buffer)?;
    let len = match parsed_len.redis_type {
        RedisType::Integer(int) => int,
        _ => return Err(Error::msg("Bulk string length incorrect redis type")),
    };

    let mut bytes_read = parsed_len.bytes_read + 2;

    let str_data = &buffer[bytes_read..bytes_read + len as usize];
    let out_str = std::str::from_utf8(str_data)?.to_string();

    bytes_read += len as usize;

    Ok(Parsed {
        redis_type: RedisType::BulkString(out_str),
        bytes_read: bytes_read,
    })
}

#[test]
fn test_parse_bulk_string() {
    let parsed = parse_bulk_string(b"5\r\nhello\r\n").unwrap();
    match parsed.redis_type {
        RedisType::BulkString(str) => {
            assert_eq!(str, "hello");
            assert_eq!(parsed.bytes_read, 8)
        }
        _ => panic!("Incorrect type")
    }
}

// *<number-of-elements>\r\n<element-1>...<element-n>
// fn parse_array(bytes: Vec<u8>) -> Vec<Resp> {}
