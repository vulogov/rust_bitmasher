use bitmasher::interleave::*;
use bitmasher::utf8util::*;

/// Ensure utf8_to_fixed_bytes copies bytes correctly and pads as expected.
#[test]
fn test_utf8_to_fixed_bytes_basic() {
    // ASCII: "Hello"
    let (data, used) = utf8_to_fixed_bytes::<8>("Hello");
    assert_eq!(used, 5);
    assert_eq!(&data[..5], b"Hello");
    assert_eq!(&data[5..], &[0, 0, 0]);  // padded

    // Unicode: "café" → 5 bytes in UTF-8
    let (data2, used2) = utf8_to_fixed_bytes::<8>("café");
    assert_eq!(used2, 5);
    assert_eq!(&data2[..5], "café".as_bytes());
}

/// Test round‑trip: interleave -> deinterleave returns original string.
#[test]
fn test_interleave_roundtrip_binary() {
    let s = "café🙂Русский"; // complicated UTF‑8 string
    let inter = interleave_with_random_bytes(s);
    let recovered = deinterleave_original_bytes(&inter).unwrap();
    assert_eq!(recovered, s);
}

/// Same but using the hex wrapper version.
#[test]
fn test_interleave_roundtrip_hex() {
    let s = "Hello🙂ØΩЖ"; // another mix of unicode
    let hex = interleave_with_random_bytes_hex(s);
    let recovered = deinterleave_original_bytes_from_hex(&hex).unwrap();
    assert_eq!(recovered, s);
}

/// Make sure interleave actually inserts random bytes between original bytes
#[test]
fn test_interleave_structure() {
    let s = "hi";  // UTF‑8: [0x68, 0x69]
    let inter = interleave_with_random_bytes(s);
    assert_eq!(inter.len(), 3); // original[0], random[1], original[2]

    assert_eq!(inter[0], b'h');
    assert_eq!(inter[2], b'i');
    assert_ne!(inter[1], b'h'); // highly unlikely equal
    assert_ne!(inter[1], b'i'); // highly unlikely equal
}

/// Ensure deinterleaver rejects invalid UTF-8
#[test]
fn test_deinterleave_invalid_utf8() {
    // Create interleaved bytes manually but corrupt the original byte
    let bad = vec![0xFF, 0x01, 0x00]; // [original=0xFF (invalid), random, original=0x00]
    let res = deinterleave_original_bytes(&bad);
    assert!(res.is_err());
}
