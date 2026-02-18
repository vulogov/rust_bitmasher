use bitmasher::keygen::{
    export_key_ascii_file,
    import_key_ascii_file,
    hkdf_sha512_same_len,
};

#[test]
fn test_key_export_import() {
    let input = b"abcdefgh12345678";
    let key = hkdf_sha512_same_len(input, None, b"info");

    let prefix = "BEGIN\n";
    let suffix = "\nEND";

    export_key_ascii_file("tmp_key.asc", &key, 4, prefix, suffix).unwrap();
    let imported = import_key_ascii_file("tmp_key.asc", prefix, suffix).unwrap();

    assert_eq!(imported, key);

    std::fs::remove_file("tmp_key.asc").unwrap();
}
