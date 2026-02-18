use bitmasher::keygen::*;

#[test]
fn test_csprng_key_same_len_std() {
    let input = b"any input decides length only";
    let k1 = csprng_key_same_len_std(input);
    let k2 = csprng_key_same_len_std(input);

    assert_eq!(k1.len(), input.len());
    assert_eq!(k2.len(), input.len());
    // Probability of equality is negligible; assert inequality (best-effort).
    assert_ne!(k1, k2);
}

#[test]
fn test_hkdf_same_len_deterministic() {
    let input = b"secret material";
    let salt  = Some(b"non-secret salt".as_ref());
    let info  = b"bitmasher:hkdf:test";

    let k1 = hkdf_key_same_len(input, salt, info);
    let k2 = hkdf_key_same_len(input, salt, info);

    assert_eq!(k1, k2);
    assert_eq!(k1.len(), input.len());

    // Changing salt changes output
    let k3 = hkdf_key_same_len(input, Some(b"another salt"), info);
    assert_ne!(k1, k3);

    // Changing info changes output
    let k4 = hkdf_key_same_len(input, salt, b"bitmasher:hkdf:other");
    assert_ne!(k1, k4);
}


#[test]
fn test_hkdf_sha512_same_len_with_utf8_input() {
    // Regular UTF-8 string (allowed to contain non-ASCII)
    let input = "super secret seed 🦀".as_bytes();

    let salt  = Some(b"non-secret salt".as_ref());
    let info  = b"bitmasher:hkdf512:test";

    let k1 = hkdf_sha512_same_len(input, salt, info);
    let k2 = hkdf_sha512_same_len(input, salt, info);

    assert_eq!(k1, k2);
    assert_eq!(k1.len(), input.len());
}

#[test]
fn test_hkdf_sha512_length_match() {
    let input = b"hello world"; // 11 bytes
    let salt  = Some(b"SALT".as_ref());
    let info  = b"INFO";

    let out = hkdf_sha512_same_len(input, salt, info);

    assert_eq!(out.len(), input.len());
}
