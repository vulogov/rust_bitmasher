// examples/interleave.rs
//
// Run with:
//   cargo run --example interleave --features std

use bitmasher::interleave::{
    interleave_with_random_bytes,
    interleave_with_random_bytes_hex,
    deinterleave_original_bytes,
    deinterleave_original_bytes_from_hex,
};

fn main() {
    println!("=== Interleave Example ===");

    // Any UTF‑8 string works (ASCII or multi-byte characters)
    let original = "café🙂 Привет мир 🦀";
    let utf8_bytes = original.as_bytes();

    println!("Original string:  {original}");
    println!("Original bytes:   {utf8_bytes:02X?}");

    // -----------------------------------------------------------------------
    // 1) BINARY INTERLEAVE
    // -----------------------------------------------------------------------
    let inter = interleave_with_random_bytes(original);
    println!("\n--- Binary interleave ---");
    println!("Interleaved bytes: {inter:02X?}");
    println!("Interleaved len:   {}", inter.len());

    // Recover the original string
    let recovered = deinterleave_original_bytes(&inter).expect("deinterleave OK");
    println!("Recovered string:  {recovered}");
    println!("Binary roundtrip OK: {}", recovered == original);

    // -----------------------------------------------------------------------
    // 2) HEX INTERLEAVE (safe for printing, logs, files, etc.)
    // -----------------------------------------------------------------------
    let inter_hex = interleave_with_random_bytes_hex(original);
    println!("\n--- Hex interleave ---");
    println!("Hex interleave:    {inter_hex}");

    let recovered_hex =
        deinterleave_original_bytes_from_hex(&inter_hex).expect("hex decode OK");
    println!("Recovered (hex):   {recovered_hex}");
    println!("Hex roundtrip OK:  {}", recovered_hex == original);

    // -----------------------------------------------------------------------
    // Summary
    // -----------------------------------------------------------------------
    println!("\nFinal result: All interleave/deinterleave operations succeeded.");
}
