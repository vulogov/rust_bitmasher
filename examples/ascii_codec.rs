// examples/ascii_codec.rs
// Run with: cargo run --example ascii_codec

extern crate alloc;

use alloc::{string::String, vec::Vec};

/// Encode a slice of u8 bytes into an ASCII string.
/// Each byte becomes a 3‑digit decimal value (000–255).
/// Lines contain `per_line` encoded bytes separated by spaces.
/// Lines are separated by '\n', with **no trailing space or newline**.
pub fn encode_bytes_ascii(input: &[u8], per_line: usize) -> String {
    assert!(per_line >= 1, "per_line must be >= 1");

    let mut out = String::new();
    for (i, &b) in input.iter().enumerate() {
        // Emit zero-padded width 3
        match b {
            0..=9   => out.push_str("00"),
            10..=99 => out.push('0'),
            _       => {}
        }
        out.push_str(&b.to_string());

        // Add separators only if not the last item
        let is_last = i + 1 == input.len();
        if !is_last {
            let at_line_pos = (i % per_line) + 1; // position after writing this item
            if at_line_pos < per_line {
                out.push(' ');
            } else {
                out.push('\n');
            }
        }
    }
    out
}

/// Decode ASCII text produced by `encode_bytes_ascii()` back into raw bytes.
/// Accepts lines separated by '\n', tokens separated by spaces.
/// Each token must be exactly 3 digits 000–255.
pub fn decode_bytes_ascii(input: &str) -> Result<Vec<u8>, &'static str> {
    let mut out = Vec::new();
    for token in input.split(|c| c == ' ' || c == '\n').filter(|t| !t.is_empty()) {
        if token.len() != 3 {
            return Err("token is not 3 characters");
        }
        let val: u32 = token.parse().map_err(|_| "invalid decimal number")?;
        if val > 255 {
            return Err("value out of range 0–255");
        }
        out.push(val as u8);
    }
    Ok(out)
}

fn main() {
    // Example A: ASCII data
    let data = b"Hello, world!"; // &[u8]
    let encoded = encode_bytes_ascii(data, 5);
    println!("Encoded (per_line=5):\n{encoded}");

    let decoded = decode_bytes_ascii(&encoded).expect("decode OK");
    assert_eq!(decoded, data);
    println!("Decoded equals original: {}", decoded == data);

    // Example B: UTF-8 data (Unicode)
    let s = "café🙂"; // Unicode string
    let utf8 = s.as_bytes(); // &[u8]
    let encoded_utf8 = encode_bytes_ascii(utf8, 4);
    println!("\nUTF-8 Encoded (per_line=4):\n{encoded_utf8}");

    let decoded_utf8 = decode_bytes_ascii(&encoded_utf8).expect("decode OK");
    assert_eq!(decoded_utf8, utf8);
    println!("UTF-8 Decoded equals original bytes: {}", decoded_utf8 == utf8);
}
