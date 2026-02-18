// tests/common/mod.rs

#[derive(Clone, Copy)]
pub struct XorShift64(u64);
impl XorShift64 {
    pub fn new(seed: u64) -> Self { Self(seed.max(1)) } // avoid zero state
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    pub fn next_usize(&mut self) -> usize { (self.next_u64() as usize) & 0x7FFF_FFFF }
    pub fn gen_range(&mut self, low: usize, high_exclusive: usize) -> usize {
        if low >= high_exclusive { low } else { low + (self.next_usize() % (high_exclusive - low)) }
    }
    pub fn fill_bytes(&mut self, buf: &mut [u8]) {
        for b in buf { *b = (self.next_u64() & 0xFF) as u8; }
    }
}

// Bit helpers for reference model
pub fn get_bit(bytes: &[u8], i: usize) -> bool {
    let byte = i / 8;
    let off  = i % 8;
    ((bytes[byte] >> off) & 1) != 0
}
pub fn set_bit(bytes: &mut [u8], i: usize, val: bool) {
    let byte = i / 8;
    let off  = i % 8;
    if val { bytes[byte] |= 1 << off; } else { bytes[byte] &= !(1 << off); }
}
pub fn mask_tail_mut(out: &mut [u8], bit_len: usize) {
    let byte_count = (bit_len + 7) / 8;
    for i in byte_count..out.len() { out[i] = 0; }
    let rem = bit_len % 8;
    if rem != 0 && byte_count > 0 {
        let mask = ((1u16 << rem) - 1) as u8;
        out[byte_count - 1] &= mask;
    }
    if bit_len == 0 {
        for b in out { *b = 0; }
    }
}
pub fn rotate_left_ref(input: &[u8], bit_len: usize, k: usize) -> Vec<u8> {
    let mut out = vec![0u8; input.len()];
    if bit_len == 0 { return out; }
    let k = k % bit_len;
    for i in 0..bit_len {
        let v = get_bit(input, i);
        let j = (i + k) % bit_len;
        set_bit(&mut out, j, v);
    }
    mask_tail_mut(&mut out, bit_len);
    out
}
pub fn assert_tail_masked(buf: &[u8], bit_len: usize) {
    let byte_count = (bit_len + 7) / 8;
    for i in byte_count..buf.len() {
        assert_eq!(buf[i], 0, "non-participating byte must be zero at index {i}");
    }
    let rem = bit_len % 8;
    if rem != 0 && byte_count > 0 {
        let mask = ((1u16 << rem) - 1) as u8;
        let last = buf[byte_count - 1];
        assert_eq!(last & !mask, 0, "bits above bit_len must be zero in the last byte");
    }
    if bit_len == 0 {
        for (i, b) in buf.iter().enumerate() {
            assert_eq!(*b, 0, "bit_len=0 ⇒ whole buffer must be zero (idx {i})");
        }
    }
}
