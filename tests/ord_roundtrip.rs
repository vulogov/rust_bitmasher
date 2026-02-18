// tests/ord_roundtrip.rs
use bitmasher::utf8util::{ord_usize_to_string, ord_usize_to_utf8_bytes};
use bitmasher::utf8util::utf8_to_fixed_bytes; // if you exposed it
use bitmasher::interleave::{
    interleave_with_random_bytes, deinterleave_original_bytes
}; // optional, just for extra sanity checks

// You can inline this if not publicly exported from your lib.
fn utf8_bytes_to_ord_usize(input: &[u8]) -> Vec<usize> {
    input.iter().map(|&b| b as usize).collect()
}

#[test]
fn roundtrip_ascii() {
    let s = "Hello, world!";
    let ords = utf8_bytes_to_ord_usize(s.as_bytes());
    let bytes = ord_usize_to_utf8_bytes(&ords).expect("back to bytes");
    assert_eq!(bytes, s.as_bytes());
    let s2 = ord_usize_to_string(&ords).expect("back to string");
    assert_eq!(s2, s);
}

#[test]
fn roundtrip_unicode_multibyte() {
    let s = "café🙂ΩЖ"; // a mix of 1-, 2-, and 4-byte UTF-8
    let ords = utf8_bytes_to_ord_usize(s.as_bytes());
    let s2 = ord_usize_to_string(&ords).expect("UTF-8 reconstruction");
    assert_eq!(s2, s);
}

#[test]
fn invalid_ord_value_over_255() {
    // 300 is not a valid single byte
    let ords = vec![0u8 as usize, 255usize, 300usize];
    let err = ord_usize_to_utf8_bytes(&ords).unwrap_err();
    assert!(err.contains("out of byte range"), "got: {err}");
}

#[test]
fn invalid_utf8_after_reconstruction() {
    // This forms an invalid UTF-8 sequence: 0xC3 must be followed by a valid continuation (0x80..0xBF).
    let invalid = vec![0xC3usize, 0x00usize];
    let err = ord_usize_to_string(&invalid).unwrap_err();
    assert!(err.contains("not valid UTF-8"), "got: {err}");
}

#[test]
fn roundtrip_with_fixed_bytes_slice() {
    // Use your helper if you want to test partial buffers from utf8_to_fixed_bytes
    let (data, used) = utf8_to_fixed_bytes::<8>("🙂"); // emoji is 4 bytes
    let ords = utf8_bytes_to_ord_usize(&data[..used]);
    let s2 = ord_usize_to_string(&ords).expect("UTF-8 reconstruction");
    assert_eq!(s2, "🙂");
}

#[test]
fn extra_cross_check_with_interleave_pipeline() {
    let s = "Привет мир 🙂 café";
    // interleave -> deinterleave (binary pipeline)
    let inter = interleave_with_random_bytes(s);
    let recovered = deinterleave_original_bytes(&inter).expect("deinterleave");
    assert_eq!(recovered, s);

    // Now convert recovered bytes -> ords -> back to string
    let ords = utf8_bytes_to_ord_usize(recovered.as_bytes());
    let s2 = ord_usize_to_string(&ords).expect("reconstruct string");
    assert_eq!(s2, s);
}
