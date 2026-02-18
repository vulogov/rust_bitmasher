extern crate alloc;

use alloc::{string::String, vec::Vec};

use crate::BitArray;
use crate::interleave::deinterleave_original_bytes;
use crate::keygen::hkdf_sha512_same_len;
use crate::pipeline::PipelineResult;

/// Result of inverting the forward pipeline.
pub struct InverseResult<const N: usize> {
    /// BitArray restored to the exact pre-rotation state (what forward had before rotating).
    pub restored_bitarray: BitArray<N>,
    /// The recovered interleaved bytes (prefix of length `used_bytes`).
    pub restored_interleaved: Vec<u8>,
    /// The reconstructed UTF-8 string (if decoding succeeded and not truncated mid-char).
    pub restored_original: Option<String>,
    /// HKDF re-derived over the restored pre-rotation bytes.
    pub hkdf_key_rederived: Vec<u8>,
    /// Whether the re-derived HKDF matches the forward pipeline’s HKDF.
    pub hkdf_matches: bool,
    /// Number of meaningful bytes (bit_len / 8).
    pub used_bytes: usize,
    /// True if the forward pipeline truncated interleaved bytes due to small N.
    pub truncated: bool,
}

/// Invert the forward pipeline:
/// - Undo all rotations using the `ordinals` in reverse order.
/// - Recover the used bytes and deinterleave them to the original UTF-8 string.
/// - Re-derive HKDF-SHA512 and check it matches the forward HKDF.
///
/// Requirements for full recovery:
/// - `N >= result.interleaved.len()` in the forward pass (no truncation).
pub fn invert_pipeline<const N: usize>(
    result: &PipelineResult<N>,
    salt: Option<&[u8]>,
    info: &[u8],
) -> InverseResult<N> {
    // 1) Start from the final rotated BitArray and reverse all rotations.
    let mut restored = result.bitarray_final.clone();

    // Inverse: apply ordinals in reverse order with opposite rotation.
    for &o in result.ordinals.iter().rev() {
        if o % 2 == 0 {
            // forward did rotate_left(o)
            restored.rotate_right(o);
        } else {
            // forward did rotate_right(o)
            restored.rotate_left(o);
        }
    }

    // 2) Extract the used bytes from the restored BitArray.
    let used_bytes = restored.bit_len() / 8; // forward always used whole bytes
    let restored_prefix = &restored.as_bytes()[..used_bytes];

    // Compare capacity vs original interleaved length to signal truncation.
    let truncated = result.interleaved.len() > N;

    // 3) Deinterleave to recover UTF-8 string (if possible).
    let (restored_interleaved, restored_original) = {
        let v = restored_prefix.to_vec();
        match deinterleave_original_bytes(&v) {
            Ok(s) => (v, Some(s)),
            Err(_) => (v, None), // could have been truncated mid-original-byte
        }
    };

    // 4) Re-derive HKDF over the restored pre-rotation bytes and compare.
    let hkdf_key_rederived = hkdf_sha512_same_len(restored.as_bytes(), salt, info);
    let hkdf_matches = hkdf_key_rederived == result.hkdf_key;

    InverseResult {
        restored_bitarray: restored,
        restored_interleaved,
        restored_original,
        hkdf_key_rederived,
        hkdf_matches,
        used_bytes,
        truncated,
    }
}
