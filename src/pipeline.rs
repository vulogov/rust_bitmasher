extern crate alloc;

use alloc::{vec::Vec, string::String};

use crate::interleave::interleave_with_random_bytes;
use crate::BitArray;
use crate::keygen::hkdf_sha512_same_len;
use crate::utf8util::{utf8_bytes_to_ord_usize};

/// Result object containing all pipeline outputs.
pub struct PipelineResult<const N: usize> {
    pub original: String,
    pub interleaved: Vec<u8>,
    pub bitarray_final: BitArray<N>,
    pub hkdf_key: Vec<u8>,
    pub ordinals: Vec<usize>,
}


/// 1. `input: &str`
/// 2. Interleave UTF-8 bytes
/// 3. Store into BitArray
/// 4. HKDF-SHA512 key = same length as BitArray bytes
/// 5. ordinals = utf8_bytes_to_ord_usize(input.as_bytes())
/// 6. rotate BitArray left/right based on ordinals:
///        even ordinal → rotate_left(ord)
///        odd ordinal  → rotate_right(ord)
///
/// `N` = storage size of the BitArray.
///
/// If input is longer than N bytes, it is truncated.
pub fn process_str_pipeline<const N: usize>(
    input: &str,
    salt: Option<&[u8]>,
    info: &[u8],
) -> PipelineResult<N> {
    // ------------------------------------------------------------
    // Step 1: UTF-8 bytes
    // ------------------------------------------------------------
    let utf8 = input.as_bytes();

    // ------------------------------------------------------------
    // Step 2: Interleave
    // ------------------------------------------------------------
    let inter = interleave_with_random_bytes(input);

    //  Fit interleaved bytes into BitArray storage
    let mut storage = [0u8; N];
    let used = inter.len().min(N);
    storage[..used].copy_from_slice(&inter[..used]);

    // Effective bit length = used * 8
    let bit_len = used * 8;

    // ------------------------------------------------------------
    // Step 3: Build BitArray
    // ------------------------------------------------------------
    let mut bits = BitArray::<N>::new(storage, bit_len);

    // ------------------------------------------------------------
    // Step 4: HKDF-SHA512
    // ------------------------------------------------------------
    let hkdf_key = hkdf_sha512_same_len(bits.as_bytes(), salt, info);

    // ------------------------------------------------------------
    // Step 5: Convert UTF‑8 → ordinals
    // ------------------------------------------------------------
    let ords = utf8_bytes_to_ord_usize(utf8);

    // ------------------------------------------------------------
    // Step 6: Rotate BitArray based on each ordinal
    // ------------------------------------------------------------
    for &o in &ords {
        if o % 2 == 0 {
            bits.rotate_left(o);
        } else {
            bits.rotate_right(o);
        }
    }

    // ------------------------------------------------------------
    // Step 7: Return struct
    // ------------------------------------------------------------
    PipelineResult {
        original: input.to_string(),
        interleaved: inter,
        bitarray_final: bits,
        hkdf_key,
        ordinals: ords,
    }
}
