/// Copy the UTF-8 bytes of `s` into a fixed array [u8; N].
/// - If s is longer than N bytes → truncate.
/// - If shorter → zero-pad the tail.
/// Returns (data, used_bytes).

pub fn utf8_to_fixed_bytes<const N: usize>(s: &str) -> ([u8; N], usize) {
    let bytes = s.as_bytes();
    let mut out = [0u8; N];
    let used = bytes.len().min(N);
    out[..used].copy_from_slice(&bytes[..used]);
    (out, used)
}

/// Convert UTF‑8 bytes to an array of usize ordinal values.
///
/// Example:
///   input:  b"café"
///   output: [99, 97, 102, 195, 169]
pub fn utf8_bytes_to_ord_usize(input: &[u8]) -> Vec<usize> {
    input.iter().map(|&b| b as usize).collect()
}

/// Convert UTF‑8 bytes to an array of usize ordinal array.
pub fn utf8_bytes_to_ord_array<const N: usize>(input: &[u8]) -> [usize; N] {
    let mut out = [0usize; N];
    let n = input.len().min(N);
    for i in 0..n {
        out[i] = input[i] as usize;
    }
    out
}


/// Convert a slice of usize ordinals (0..=255) back into raw UTF-8 bytes.
/// Returns an error if any value is out of byte range.
pub fn ord_usize_to_utf8_bytes(ord: &[usize]) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(ord.len());
    for (i, &v) in ord.iter().enumerate() {
        if v > 255 {
            return Err(format!(
                "ordinal at index {} is out of byte range: {} (>255)",
                i, v
            ));
        }
        out.push(v as u8);
    }
    Ok(out)
}


/// Convert a slice of usize ordinals into a UTF-8 `String`.
/// Fails if any value is >255 or the resulting byte sequence is not valid UTF-8.
pub fn ord_usize_to_string(ord: &[usize]) -> Result<String, String> {
    let bytes = ord_usize_to_utf8_bytes(ord)?;
    String::from_utf8(bytes).map_err(|e| format!("not valid UTF-8: {e}"))
}
