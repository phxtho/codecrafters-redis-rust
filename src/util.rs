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
