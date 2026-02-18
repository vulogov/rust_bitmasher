use bitmasher::pipeline::process_str_pipeline;
use bitmasher::pipeline_inverse::invert_pipeline;

/// End-to-end: forward pipeline then inverse pipeline with adequate capacity (no truncation).
#[test]
fn pipeline_inverse_roundtrip_ok() {
    let salt = Some(b"bitmasher-salt".as_ref());
    let info = b"bitmasher:pipeline:test";

    // Choose N large enough for the interleaved size:
    // If input has length L, interleave_with_random_bytes produces L + (L-1) bytes.
    let input = "café🙂 Rust!";
    let forward = process_str_pipeline::<128>(input, salt, info); // ample space

    let inverse = invert_pipeline(&forward, salt, info);

    // Full round-trip recovery
    assert_eq!(inverse.truncated, false, "should not be truncated");
    assert_eq!(inverse.restored_original.as_deref(), Some(input));
    assert_eq!(inverse.restored_interleaved, forward.interleaved);
    assert!(inverse.hkdf_matches, "HKDF must match when fully invertible");

    // The restored prefix must match the BitArray used bytes.
    assert_eq!(
        &inverse.restored_bitarray.as_bytes()[..inverse.used_bytes],
        &inverse.restored_interleaved[..]
    );
}

/// Truncation scenario: N too small for full interleaved length.
/// In this case, we can only recover the prefix; HKDF still matches (it used the truncated bytes).
#[test]
fn pipeline_inverse_truncated_prefix_only() {
    let salt = Some(b"bitmasher-salt".as_ref());
    let info = b"bitmasher:pipeline:test";

    // "abcdef" length L = 6 -> interleave len = 11; pick N smaller (e.g., 8) to force truncation.
    let input = "abcdef";
    let forward = process_str_pipeline::<8>(input, salt, info);

    let inverse = invert_pipeline(&forward, salt, info);

    assert!(inverse.truncated, "must signal truncation");
    // We cannot assert restored_original == input, since tail is lost.
    // But HKDF check should still hold, because forward derived HKDF
    // on the truncated BitArray bytes.
    assert!(inverse.hkdf_matches, "HKDF must match even if truncated");

    // Restored interleaved equals the used bytes of the restored bitarray.
    assert_eq!(
        &inverse.restored_bitarray.as_bytes()[..inverse.used_bytes],
        &inverse.restored_interleaved[..]
    );

    // We can at least assert that the recovered prefix equals the forward interleaved prefix.
    let used = inverse.used_bytes;
    assert_eq!(&inverse.restored_interleaved[..], &forward.interleaved[..used]);
}
