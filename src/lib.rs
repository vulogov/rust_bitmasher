extern crate log;

pub mod ascii_codec;
pub mod utf8util;
pub mod interleave;
pub mod keygen;
pub mod ppke;
pub mod pipeline;
pub mod pipeline_v2;
pub mod pipeline_inverse;
pub mod pipeline_decode;
pub mod pipeline_decode_v2;

/// Fixed-size bit array over `N` bytes, with effective `bit_len` bits in use.
/// Bits are indexed from 0..bit_len-1, bit 0 is the LSB of data[0].
#[derive(Clone)]
pub struct BitArray<const N: usize> {
    data: [u8; N],
    bit_len: usize,
}

impl<const N: usize> BitArray<N> {
    /// Create a new BitArray from bytes and an effective bit length (<= N*8).
    /// Unused tail bits in the last byte (if bit_len % 8 != 0) are masked to 0.
    pub fn new(data: [u8; N], bit_len: usize) -> Self {
        assert!(bit_len <= N * 8, "bit_len exceeds storage");
        let mut s = Self { data, bit_len };
        s.mask_tail();
        s
    }

    /// Returns immutable access to the underlying bytes.
    pub fn as_bytes(&self) -> &[u8; N] { &self.data }

    /// Returns mutable access to the underlying bytes.
    /// If you modify high bits, call `mask_tail` to re‑mask.
    pub fn as_bytes_mut(&mut self) -> &mut [u8; N] { &mut self.data }

    /// Total number of bits in use.
    pub fn bit_len(&self) -> usize { self.bit_len }

    /// Get bit i (0..bit_len-1). Panics on out-of-range.
    pub fn get_bit(&self, i: usize) -> bool {
        assert!(i < self.bit_len, "bit index out of range");
        let byte = i / 8;
        let off  = i % 8;
        ((self.data[byte] >> off) & 1) != 0
    }

    /// Set bit i to `val`. Panics on out-of-range.
    pub fn set_bit(&mut self, i: usize, val: bool) {
        assert!(i < self.bit_len, "bit index out of range");
        let byte = i / 8;
        let off  = i % 8;
        if val { self.data[byte] |= 1 << off; }
        else   { self.data[byte] &= !(1 << off); }
    }
    /// Rotate left by `k` bits across the first `bit_len` bits (contiguous bit ring).
    pub fn rotate_left(&mut self, k: usize) {
        let bit_len = self.bit_len;
        if bit_len == 0 { return; }
        let k = k % bit_len;
        if k == 0 { return; }

        let mut out = [0u8; N];

        for i in 0..bit_len {
            let src_byte = i / 8;
            let src_off  = i % 8;
            let bit = (self.data[src_byte] >> src_off) & 1;

            let j = (i + k) % bit_len;
            let dst_byte = j / 8;
            let dst_off  = j % 8;

            if bit != 0 {
                out[dst_byte] |= 1 << dst_off;
            }
        }

        // copy back exactly participating bytes
        let bytes_used = (bit_len + 7) / 8;
        for i in 0..bytes_used {
            self.data[i] = out[i];
        }

        // zero unused bytes & mask tail bits
        self.mask_tail();
    }

    pub fn rotate_right(&mut self, k: usize) {
        let bit_len = self.bit_len;
        if bit_len == 0 { return; }
        let k = k % bit_len;
        if k == 0 { return; }
        self.rotate_left(bit_len - k);
    }

    /// Clear any bits above `bit_len` in the last participating byte, and zero out bytes beyond it.
    fn mask_tail(&mut self) {
        if self.bit_len == 0 {
            for b in &mut self.data { *b = 0; }
            return;
        }

        let byte_count = (self.bit_len + 7) / 8;

        // Zero bytes not participating
        for i in byte_count..N { self.data[i] = 0; }

        // If last byte is partial, mask the high bits (above bit_len)
        let rem_bits = self.bit_len % 8;
        if rem_bits != 0 {
            let mask = ((1u16 << rem_bits) - 1) as u8;
            let last = &mut self.data[byte_count - 1];
            *last &= mask;
        }
    }
}
