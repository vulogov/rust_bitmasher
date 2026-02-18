use std::time::{SystemTime, UNIX_EPOCH};

/// Tiny, dependency-free PRNG (XorShift64) we can seed from system time.
/// This avoids pulling in external crates for randomness.
#[derive(Clone)]
struct XorShift64 {
    state: u64,
}
impl XorShift64 {
    fn new(seed: u64) -> Self {
        // Avoid zero state
        Self { state: if seed == 0 { 0x9E3779B97F4A7C15 } else { seed } }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
    fn next_u8(&mut self) -> u8 {
        (self.next_u64() & 0xFF) as u8
    }
}

/// Generate a seed from system time.
fn seed_from_time() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    // Fold u128 -> u64
    (nanos as u64) ^ ((nanos >> 64) as u64)
}

/// 1) Interleave original UTF-8 bytes with one random byte between each original byte.
/// Output layout: [o0, r0, o1, r1, ..., o(n-1)]
/// (No trailing random byte after the last original.)
///
/// Returns binary Vec<u8>. If you need a printable form, use the hex variant below.
pub fn interleave_with_random_bytes(input: &str) -> Vec<u8> {
    let bytes = input.as_bytes();
    let n = bytes.len();
    if n == 0 {
        return Vec::new();
    }

    let mut rng = XorShift64::new(seed_from_time());
    // Length: n originals + (n-1) randoms
    let mut out = Vec::with_capacity(n + (n - 1));
    for (i, &b) in bytes.iter().enumerate() {
        out.push(b);
        if i + 1 < n {
            out.push(rng.next_u8());
        }
    }
    out
}

/// Hex-encode helper (uppercase, no 0x prefix).
fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0F) as usize] as char);
    }
    s
}

/// Hex-decode helper.
fn from_hex(s: &str) -> Result<Vec<u8>, String> {
    fn val(c: u8) -> Result<u8, String> {
        match c {
            b'0'..=b'9' => Ok(c - b'0'),
            b'a'..=b'f' => Ok(10 + (c - b'a')),
            b'A'..=b'F' => Ok(10 + (c - b'A')),
            _ => Err(format!("invalid hex character: {}", c as char)),
        }
    }
    let bytes = s.as_bytes();
    if bytes.len() % 2 != 0 {
        return Err("hex string length must be even".into());
    }
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for i in (0..bytes.len()).step_by(2) {
        let hi = val(bytes[i])?;
        let lo = val(bytes[i + 1])?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

/// Convenience: interleave and return a HEX STRING (printable, UTF-8 safe).
pub fn interleave_with_random_bytes_hex(input: &str) -> String {
    let bin = interleave_with_random_bytes(input);
    to_hex(&bin)
}

/// 2) Deinterleave previously generated binary buffer.
/// Inverse of `interleave_with_random_bytes`.
/// Keeps original bytes at even indices (0, 2, 4, ...).
/// Returns the original UTF-8 string.
pub fn deinterleave_original_bytes(interleaved: &[u8]) -> Result<String, String> {
    if interleaved.is_empty() {
        return Ok(String::new());
    }
    let mut orig = Vec::with_capacity((interleaved.len() + 1) / 2);
    for (i, &b) in interleaved.iter().enumerate() {
        if i % 2 == 0 {
            orig.push(b);
        }
    }
    String::from_utf8(orig).map_err(|e| format!("not valid UTF-8: {e}"))
}

/// Convenience: deinterleave from a HEX STRING input (pairs of hex chars).
pub fn deinterleave_original_bytes_from_hex(hex: &str) -> Result<String, String> {
    let bin = from_hex(hex)?;
    deinterleave_original_bytes(&bin)
}
