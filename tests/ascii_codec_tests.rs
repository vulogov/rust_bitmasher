use bitmasher::ascii_codec::{encode_bytes_ascii, decode_bytes_ascii};

#[test]
fn test_encode_single_line() {
    let input = [1u8, 20u8, 255u8];
    let encoded = encode_bytes_ascii(&input, 10);
    assert_eq!(encoded, "001 020 255"); // no trailing space
}

#[test]
fn test_encode_multi_line_exact_wrap() {
    let input = [1, 2, 3, 4];
    let encoded = encode_bytes_ascii(&input, 2);
    assert_eq!(encoded, "001 002\n003 004"); // no trailing newline
}

#[test]
fn test_encode_multi_line_last_short() {
    let input = [1, 2, 3, 4, 5];
    let encoded = encode_bytes_ascii(&input, 2);
    assert_eq!(encoded, "001 002\n003 004\n005"); // no trailing newline
}

#[test]
fn test_roundtrip_arbitrary() {
    let original = vec![7u8, 19, 250, 0, 99, 255];
    let encoded = encode_bytes_ascii(&original, 3);
    let decoded = decode_bytes_ascii(&encoded).expect("decode OK");
    assert_eq!(decoded, original);
}

#[test]
fn test_decode_invalid_token_length() {
    let bad = "01";
    assert!(decode_bytes_ascii(bad).is_err());
}

#[test]
fn test_decode_invalid_number() {
    let bad = "999";
    assert!(decode_bytes_ascii(bad).is_err());
}

#[test]
fn test_decode_invalid_chars() {
    let bad = "0x1";
    assert!(decode_bytes_ascii(bad).is_err());
}
