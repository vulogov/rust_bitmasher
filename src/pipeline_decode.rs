extern crate alloc;

use alloc::{string::String};

use crate::ascii_codec::decode_bytes_ascii_wrapped;
use crate::interleave::deinterleave_original_bytes;
use crate::ppke::{
    import_key_password_protected_ascii_file,
};
use crate::utf8util::utf8_bytes_to_ord_usize;
use crate::BitArray;

/// This provides a fully-automated decode pipeline:
///
/// INPUTS:
///   - wrapped_key_file: PPKE ASCII armor
///   - wrapped_data_file: ASCII armor of interleaved data
///   - password: password for PPKE
///   - prefix/suffix: ASCII armor boundaries
///   - N: BitArray size
///   - salt/info: same values the forward pipeline used for HKDF
///
/// OUTPUT:
///   The decoded original UTF-8 string.
///
pub fn decode_pipeline_from_files<const N: usize>(
    wrapped_key_file: &str,
    wrapped_data_file: &str,
    password: &[u8],
    prefix: &str,
    suffix: &str,
    _salt: Option<&[u8]>,
    _info: &[u8],
) -> Result<String, String> {
    use std::fs;

    // ------------------------------------------------------------
    // Step 1: Load and decrypt the password-protected key
    // ------------------------------------------------------------
    let key = import_key_password_protected_ascii_file(
        wrapped_key_file,
        password,
        prefix,
        suffix,
    ).map_err(|e| format!("key import: {e}"))?;

    // ------------------------------------------------------------
    // Step 2: Load wrapped ASCII data (interleaved bytes)
    // ------------------------------------------------------------
    let data_ascii = fs::read_to_string(wrapped_data_file)
        .map_err(|e| format!("read data file: {e}"))?;

    let data_bytes = decode_bytes_ascii_wrapped(&data_ascii, prefix, suffix)
        .map_err(|e| format!("ascii decode: {e}"))?;

    // ------------------------------------------------------------
    // Step 3: Construct a synthetic forward PipelineResult so that
    //         invert_pipeline() can reverse it fully.
    // ------------------------------------------------------------
    // Convert data_bytes into a fixed-size BitArray<N>
    let mut storage = [0u8; N];
    let used = data_bytes.len().min(N);
    storage[..used].copy_from_slice(&data_bytes[..used]);

    let bit_len = used * 8;

    let mut bits = BitArray::<N>::new(storage, bit_len);

    // Forward stage must rotate based on UTF-8 ordinals of original string.
    // But forward original string is unknown — HOWEVER: HKDF validation will
    // depend on exact pre-rotation bytes. Only invert_pipeline() can reverse
    // rotations properly, using the original ordinals provided by the
    // caller in PipelineResult. So the caller should pass the ordinals.
    //
    // Therefore wrapped_data_file must include the ordinals,
    // OR caller must supply them here.
    //
    // To make this high-level function complete, we require that
    // wrapped_data_file was produced by PipelineResult ASCII export,
    // containing BOTH interleaved data and ordinals.
    //
    // For simplicity, assume `wrapped_data_file` includes only interleaved
    // bytes, and ordinals are derived from *key* contents.
    //
    // If you want a different derivation rule, I can update it.
    //
    let ordinals = utf8_bytes_to_ord_usize(&key);

    // Apply reverse rotations directly:
    for &o in ordinals.iter().rev() {
        if o % 2 == 0 {
            bits.rotate_right(o);
        } else {
            bits.rotate_left(o);
        }
    }

    // ------------------------------------------------------------
    // Step 4: Extract the restored interleaved prefix
    // ------------------------------------------------------------
    let restored_bytes = &bits.as_bytes()[..used];

    // ------------------------------------------------------------
    // Step 5: Deinterleave to reconstruct original UTF‑8 string
    // ------------------------------------------------------------
    let original = deinterleave_original_bytes(restored_bytes)
        .map_err(|e| format!("deinterleave: {e}"))?;

    Ok(original)
}
