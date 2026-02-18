use bitmasher::ascii_codec::*;

#[test]
fn test_wrapped_roundtrip() {
    let data = "abc123\u{FF}hi🙂".as_bytes();
    let prefix = "BEGIN\n";
    let suffix = "\nEND";

    let s = encode_bytes_ascii_wrapped(data, 3, prefix, suffix);
    let out = decode_bytes_ascii_wrapped(&s, prefix, suffix).unwrap();
    assert_eq!(out, data);
}

#[test]
fn test_ascii_file_roundtrip() {
    let data = b"test-data-xyz";
    let prefix = "<<";
    let suffix = ">>";

    write_ascii_file_wrapped("tmp_ascii.txt", data, 4, prefix, suffix).unwrap();
    let out = read_ascii_file_wrapped("tmp_ascii.txt", prefix, suffix).unwrap();
    assert_eq!(out, data);

    std::fs::remove_file("tmp_ascii.txt").unwrap();
}
