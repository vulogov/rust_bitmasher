use bitmasher::utf8util::utf8_to_fixed_bytes;

#[test]
fn test_utf8_to_fixed_bytes_truncation() {
    let long = "Привет мир"; // > 8 bytes
    let (data, used) = utf8_to_fixed_bytes::<8>(long);

    assert_eq!(used, 8);
    assert_eq!(&data[..8], &long.as_bytes()[..8]);
}
