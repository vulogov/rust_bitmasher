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
        // 3-digit decimal, zero-padded
        match b {
            0..=9   => out.push_str("00"),
            10..=99 => out.push('0'),
            _       => {}
        }
        // write the decimal digits
        // (avoid format! to be friendlier to no_std + alloc if you later want)
        // but format! is fine if you always compile tests with std:
        // out.push_str(&b.to_string());
        // Manual fast push since above prepadded the width:
        // Convert u8 to decimal without allocation:
        let val = b as u8;
        // We already emitted the required leading zeros; now push the actual number.
        // For simplicity and clarity, use to_string() here (requires std). If you want
        // a pure alloc approach, keep format!/to_string() gated under std and add
        // a small decimal writer. For most crates, to_string() in tests is fine:
        out.push_str(&val.to_string());

        // Determine separators
        let is_last = i + 1 == input.len();
        if !is_last {
            let at_line_pos = (i % per_line) + 1; // position after adding this item
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
/// Accepts any number of lines separated by '\n', tokens separated by spaces.
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

/// Encode bytes as ASCII (3-digit decimals per byte),
/// and wrap with prefix + suffix.
pub fn encode_bytes_ascii_wrapped(
    data: &[u8],
    per_line: usize,
    prefix: &str,
    suffix: &str,
) -> String {
    let body = encode_bytes_ascii(data, per_line);
    let mut s = String::with_capacity(prefix.len() + body.len() + suffix.len());
    s.push_str(prefix);
    s.push_str(&body);
    s.push_str(suffix);
    s
}

/// Strip prefix/suffix and decode ASCII decimal encoding back to bytes.
pub fn decode_bytes_ascii_wrapped(
    text: &str,
    prefix: &str,
    suffix: &str,
) -> Result<Vec<u8>, &'static str> {
    if !text.starts_with(prefix) {
        return Err("missing prefix");
    }
    if !text.ends_with(suffix) {
        return Err("missing suffix");
    }

    let inner = &text[prefix.len() .. text.len() - suffix.len()];
    decode_bytes_ascii(inner)
}

pub fn read_ascii_file(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use std::fs;
    let content = fs::read_to_string(path)?;
    decode_bytes_ascii(&content).map_err(|e| e.into())
}

pub fn read_ascii_file_wrapped(
    path: &str,
    prefix: &str,
    suffix: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use std::fs;
    let content = fs::read_to_string(path)?;
    let decoded = decode_bytes_ascii_wrapped(&content, prefix, suffix)
        .map_err(|e| format!("decode error: {}", e))?;
    Ok(decoded)
}

pub fn write_ascii_file(
    path: &str,
    data: &[u8],
    per_line: usize,
) -> std::io::Result<()> {
    use std::fs;
    let encoded = encode_bytes_ascii(data, per_line);
    fs::write(path, encoded)
}

pub fn write_ascii_file_wrapped(
    path: &str,
    data: &[u8],
    per_line: usize,
    prefix: &str,
    suffix: &str,
) -> std::io::Result<()> {
    use std::fs;
    let encoded = encode_bytes_ascii_wrapped(data, per_line, prefix, suffix);
    fs::write(path, encoded)
}
