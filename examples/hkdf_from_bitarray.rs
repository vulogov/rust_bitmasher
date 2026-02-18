// examples/hkdf_from_bitarray.rs
//
// Run with:
//   cargo run --example hkdf_from_bitarray --features std

use bitmasher::BitArray;
use bitmasher::keygen::hkdf_sha512_same_len;

fn main() {
    println!("=== HKDF-SHA512 Example Using BitArray ===");

    // ---------------------------------------------------------------------
    // Example A: Full-byte bitarray (16 bits)
    // ---------------------------------------------------------------------
    let bits_a = BitArray::<2>::new([0xA1, 0x03], 16); // 16-bit domain
    let input_a = bits_a.as_bytes();  // &[u8; 2]
    let salt    = Some(b"bitmasher-salt".as_ref());
    let info    = b"bitmasher:hkdf512:bitarray";

    let key_a = hkdf_sha512_same_len(input_a, salt, info);

    println!("Full-byte BitArray:");
    println!("  input bytes: {:02X?}", input_a);
    println!("  derived key: {:02X?}", key_a);

    // ---------------------------------------------------------------------
    // Example B: Partial-length BitArray (13-bit domain)
    // Only the first 13 bits are in the rotation domain; others masked.
    // ---------------------------------------------------------------------
    let bits_b = BitArray::<2>::new([0b1110_1101, 0b0000_0011], 13);
    let input_b = &bits_b.as_bytes()[..2]; // slice view over bytes

    let key_b = hkdf_sha512_same_len(input_b, salt, b"bitmasher:partial-domain");

    println!("\nPartial (13-bit) BitArray:");
    println!("  input bytes (masked): {:02X?}", input_b);
    println!("  derived key:          {:02X?}", key_b);

    // ---------------------------------------------------------------------
    // Example C: UTF‑8 string → BitArray → HKDF‑SHA512
    // ---------------------------------------------------------------------
    let s = "café🙂";
    let utf8 = s.as_bytes();

    // Fit into a BitArray<N>
    let n = utf8.len();
    let mut data = [0u8; 8];
    data[..n].copy_from_slice(utf8);

    let bits_c = BitArray::<8>::new(data, n * 8); // exact bit length in UTF-8

    let key_c = hkdf_sha512_same_len(bits_c.as_bytes(), salt, b"bitmasher:utf8-example");

    println!("\nUTF‑8 → BitArray → HKDF:");
    println!("  UTF‑8 bytes: {:02X?}", utf8);
    println!("  BitArray:    {:02X?}", bits_c.as_bytes());
    println!("  derived key: {:02X?}", key_c);

    // ---------------------------------------------------------------------
    println!("\nAll HKDF derivations succeeded.");
}
