extern crate alloc;

use alloc::{string::String, vec::Vec};

use crate::BitArray;
use crate::interleave::interleave_with_random_bytes;
use crate::keygen::hkdf_sha512_same_len;

/// Result object for v2 pipeline.
#[derive(Clone)]
pub struct PipelineV2Result<const N: usize> {
    pub original: String,
    pub interleaved: Vec<u8>,    // raw interleaved bytes (before rotation)
    pub bitarray_final: BitArray<N>,
    pub hkdf_key: Vec<u8>,       // HKDF-SHA512 over BitArray bytes (length = N)
    pub used_bytes: usize,       // interleaved bytes actually stored (<= N)
}

/// Compute rotation ordinals from HKDF key bytes (each byte -> usize).
#[inline]
fn ordinals_from_key_bytes(key: &[u8]) -> Vec<usize> {
    key.iter().map(|&b| b as usize).collect()
}

/// v2 pipeline:
/// 1) Interleave input &str
/// 2) Copy into BitArray<N> (truncate/pad as needed)
/// 3) HKDF-SHA512 over BitArray bytes -> hkdf_key (len = N)
/// 4) ordinals := from hkdf_key bytes
/// 5) Rotate BitArray:
///      even o -> rotate_left(o), odd o -> rotate_right(o)
pub fn process_str_pipeline_v2<const N: usize>(
    input: &str,
    salt: Option<&[u8]>,
    info: &[u8],
) -> PipelineV2Result<N> {
    let inter = interleave_with_random_bytes(input);

    let mut storage = [0u8; N];
    let used = inter.len().min(N);
    storage[..used].copy_from_slice(&inter[..used]);

    let mut bits = BitArray::<N>::new(storage, used * 8);

    // HKDF over the full BitArray backing bytes; output length == N
    let hkdf_key = hkdf_sha512_same_len(bits.as_bytes(), salt, info);

    // Rotation schedule derived from hkdf_key bytes
    let ords = ordinals_from_key_bytes(&hkdf_key);
    for &o in &ords {
        if o % 2 == 0 { bits.rotate_left(o); } else { bits.rotate_right(o); }
    }

    PipelineV2Result {
        original: input.to_string(),
        interleaved: inter,
        bitarray_final: bits,
        hkdf_key,
        used_bytes: used,
    }
}
