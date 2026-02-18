extern crate alloc;

use alloc::string::String;

use crate::ascii_codec::decode_bytes_ascii_wrapped;
use crate::BitArray;
use crate::interleave::deinterleave_original_bytes;
use crate::keygen::{hkdf_sha512_same_len};
use crate::ppke::{import_key_password_protected_ascii_file};

#[inline]
fn ordinals_from_key_bytes(key: &[u8]) -> alloc::vec::Vec<usize> {
    key.iter().map(|&b| b as usize).collect()
}

/// v2 decode pipeline with HKDF integrity check:
/// - wrapped_key_file: PPKE ASCII (contains HKDF key, len == N)
/// - wrapped_data_file: ASCII (contains **ROTATED** bytes)
/// - password: PPKE password
/// - prefix/suffix: ASCII armor boundaries
/// - salt/info: HKDF params used in forward (for re-derivation check)
pub fn decode_pipeline_v2_from_files<const N: usize>(
    wrapped_key_file: &str,
    wrapped_data_file: &str,
    password: &[u8],
    prefix: &str,
    suffix: &str,
    salt: Option<&[u8]>,
    info: &[u8],
) -> Result<String, String> {
    use std::fs;

    // (1) Decrypt HKDF key (length must equal N)
    let hkdf_key = import_key_password_protected_ascii_file(
        wrapped_key_file, password, prefix, suffix
    ).map_err(|e| format!("key import: {e}"))?;
    if hkdf_key.len() != N {
        return Err(format!("hkdf_key length {} != N {}", hkdf_key.len(), N));
    }

    // (2) Load **ROTATED** bytes
    let ascii = fs::read_to_string(wrapped_data_file)
        .map_err(|e| format!("read data: {e}"))?;
    let rotated = decode_bytes_ascii_wrapped(&ascii, prefix, suffix)
        .map_err(|e| format!("decode ascii: {e}"))?;

    // (3) Recreate BitArray from rotated bytes
    let mut storage = [0u8; N];
    let used = rotated.len().min(N);
    storage[..used].copy_from_slice(&rotated[..used]);
    let mut bits = BitArray::<N>::new(storage, used * 8);

    // (4) Rebuild rotation schedule from hkdf_key bytes and inverse rotate (reverse order)
    let ords = ordinals_from_key_bytes(&hkdf_key);
    for &o in ords.iter().rev() {
        if o % 2 == 0 { bits.rotate_right(o); } else { bits.rotate_left(o); }
    }

    // (5) HKDF integrity check: re-derive HKDF on pre-rotation bytes and compare
    let rederived = hkdf_sha512_same_len(bits.as_bytes(), salt, info);
    if rederived != hkdf_key {
        return Err("hkdf mismatch: decoded payload does not match the decrypted key".into());
    }

    // (6) Deinterleave pre-rotation bytes back to original UTF-8 string
    let restored_interleaved = &bits.as_bytes()[..used];
    let original = deinterleave_original_bytes(restored_interleaved)
        .map_err(|e| format!("deinterleave: {e}"))?;

    Ok(original)
}
