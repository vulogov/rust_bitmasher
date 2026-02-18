extern crate log;
extern crate alloc;

use ring::hkdf::{self, KeyType, Prk, Salt};

/// A wrapper that allows HKDF output of arbitrary length using ring's HKDF API.
struct HkdfLen(usize);

impl KeyType for HkdfLen {
    fn len(&self) -> usize {
        self.0
    }
}

use alloc::vec::Vec;

use crate::ascii_codec::{
    encode_bytes_ascii_wrapped,
    decode_bytes_ascii_wrapped,
};

/// Derive a cryptographically strong key of the same length as `input` using HKDF-SHA256.
/// Deterministic: same `input` + `salt` + `info` => same key.
/// - `salt` and `info` are optional context separation parameters (can be empty).
/// - If you do not have a salt, pass `None` to use HKDF with an empty salt (still safe).
pub fn hkdf_key_same_len(
    input: &[u8],
    salt: Option<&[u8]>,
    info: &[u8],
) -> Vec<u8> {
    let salt_obj = Salt::new(hkdf::HKDF_SHA256, salt.unwrap_or(&[]));
    let prk: Prk = salt_obj.extract(input);

    let mut out = vec![0u8; input.len()];
    let key_type = HkdfLen(input.len());

    prk.expand(&[info], key_type)
        .expect("HKDF expand failed")
        .fill(&mut out)
        .expect("HKDF fill failed");

    out
}


/// HKDF‑SHA512 key derivation of same length as input.
/// Deterministic: (input, salt, info) → same key.
/// Safe even with empty salt (RFC 5869 allows it).
pub fn hkdf_sha512_same_len(
    input: &[u8],
    salt: Option<&[u8]>,
    info: &[u8],
) -> Vec<u8> {
    // Use HKDF-SHA512 instead of SHA256
    let salt_obj = Salt::new(hkdf::HKDF_SHA512, salt.unwrap_or(&[]));
    let prk: Prk = salt_obj.extract(input);

    let mut out = vec![0u8; input.len()];
    let key_type = HkdfLen(input.len());

    prk.expand(&[info], key_type)
        .expect("HKDF-SHA512 expand failed")
        .fill(&mut out)
        .expect("HKDF-SHA512 fill failed");

    out
}


/// Generate a cryptographically-strong random key whose length equals `input.len()`.
/// Non-deterministic; the `input` content is ignored except for its length.
///
/// Requires `std`. Uses the OS CSPRNG via `rand::rngs::OsRng`.
pub fn csprng_key_same_len_std(input: &[u8]) -> Vec<u8> {
    use rand::rngs::OsRng;
    use rand::RngCore;

    let mut key = vec![0u8; input.len()];
    OsRng.fill_bytes(&mut key);
    key
}

pub fn export_key_ascii_file(
    path: &str,
    key: &[u8],
    per_line: usize,
    prefix: &str,
    suffix: &str,
) -> std::io::Result<()> {
    use std::fs;

    let wrapped = encode_bytes_ascii_wrapped(key, per_line, prefix, suffix);
    fs::write(path, wrapped)
}

pub fn import_key_ascii_file(
    path: &str,
    prefix: &str,
    suffix: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use std::fs;

    let content = fs::read_to_string(path)?;
    let decoded = decode_bytes_ascii_wrapped(&content, prefix, suffix)
        .map_err(|e| format!("decode error: {e}"))?;

    Ok(decoded)
}

pub fn derive_and_export_hkdf_sha512(
    input: &[u8],
    salt: Option<&[u8]>,
    info: &[u8],
    path: &str,
    per_line: usize,
    prefix: &str,
    suffix: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let key = crate::keygen::hkdf_sha512_same_len(input, salt, info);
    export_key_ascii_file(path, &key, per_line, prefix, suffix)?;
    Ok(())
}

pub fn import_and_use_key<F>(
    path: &str,
    prefix: &str,
    suffix: &str,
    mut callback: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(&[u8]),
{
    let key = import_key_ascii_file(path, prefix, suffix)?;
    callback(&key);
    Ok(())
}
