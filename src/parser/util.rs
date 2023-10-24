use anyhow::Result;

// Read buffer until you encounter a CRLF
// return whats been read and the length
pub fn read_until_crlf(buf: &[u8]) -> Option<&[u8]> {
    for (i, pair) in buf.windows(2).enumerate() {
        if pair == [b'\r', b'\n'] {
            return Some(&buf[..(i)]);
        }
    }
    return None;
}

#[test]
fn test_read_until_crlf_empty() {
    // Test when the input buffer is empty.
    let buf: &[u8] = &[];
    assert_eq!(read_until_crlf(buf), None);
}

#[test]
fn test_read_until_crlf_no_crlf() {
    // Test when there is no CRLF in the buffer.
    let buf = b"Hello, World!";
    assert_eq!(read_until_crlf(buf), None);
}

#[test]
fn test_read_until_crlf_multiple_crlf() {
    // Test when there are multiple CRLF sequences in the buffer.
    let buf = b"Line 1\r\nLine 2\r\nLine 3";
    assert_eq!(read_until_crlf(buf), Some(b"Line 1" as &[u8]));
}

/** convert byte to string removing unescaped \r\n */
pub fn to_string(buf: &[u8]) -> Result<String> {
    let input = std::str::from_utf8(buf)?.to_string();

    let output = input.replace(r"\r\n", "\r\n");

    return Ok(output);
}

#[test]
fn test_to_string_removes_unescaped_characters() {
    let expected = std::str::from_utf8(b"hello\r\nworld").unwrap();
    let result = to_string("hello\r\nworld".as_bytes()).unwrap();
    assert_eq!(result, expected)
}
