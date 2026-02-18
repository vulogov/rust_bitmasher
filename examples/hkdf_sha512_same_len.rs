// examples/hkdf_sha512.rs
//
// Run with:
//   cargo run --example hkdf_sha512 --features std

use bitmasher::keygen::hkdf_sha512_same_len;

fn main() {
    // Input secret (UTF‑8 bytes)
    let input = "super secret seed value".as_bytes();

    // Optional salt (recommended, but can be empty)
    let salt = Some(b"my-hkdf-salt".as_ref());

    // Domain‑separation context
    let info = b"bitmasher:hkdf512:example";

    // Derive a key with exactly input.len() bytes
    let key = hkdf_sha512_same_len(input, salt, info);

    println!("Input bytes (len={}): {:?}", input.len(), input);
    println!("Derived HKDF‑SHA512 key (len={}): {:02X?}", key.len(), key);

    // Show determinism: same input → same output
    let key2 = hkdf_sha512_same_len(input, salt, info);
    println!("Repeat derivation matches original: {}", key == key2);

    // Changing salt changes the key
    let key3 = hkdf_sha512_same_len(input, Some(b"another salt"), info);
    println!("Changing salt changes key: {}", key != key3);

    // Changing info also changes the key
    let key4 = hkdf_sha512_same_len(input, salt, b"bitmasher:hkdf512:other");
    println!("Changing info changes key: {}", key != key4);
}
