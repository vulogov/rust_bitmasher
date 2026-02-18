//! ppke.rs — Password-Protected Key Export/Import (PPKE) utilities
//!
//! - KDF: PBKDF2-HMAC-SHA256 (from `ring`)
//! - AEAD: ChaCha20-Poly1305 (from `ring`)
//! - no_std + alloc for in-memory ops; std-gated I/O helpers for ASCII-armor files
//!
//! PPKE v1 Binary Layout (big-endian u32):
//!   0..4    : MAGIC = b"BMK1"
//!   4       : kdf_id   = 0x01  (PBKDF2-HMAC-SHA256)
//!   5       : aead_id  = 0x01  (ChaCha20-Poly1305)
//!   6..10   : iterations (u32)
//!   10      : salt_len  (u8)    (recommended: 16)
//!   11      : nonce_len (u8)    (ChaCha20-Poly1305 -> 12)
//!   12..16  : pt_len    (u32)   (plaintext length in bytes)
//!   16..    : salt      (salt_len bytes)
//!             nonce     (nonce_len bytes)
//!             ciphertext (pt_len + TAG(16) bytes)
//!
//! AAD during AEAD seal/open = the first 16 bytes (header) *only*.

extern crate alloc;

use alloc::vec::Vec;
use core::num::NonZeroU32;

use ring::{aead, pbkdf2, rand as ring_rand};
use ring::rand::SecureRandom;

use crate::ascii_codec::{decode_bytes_ascii_wrapped, encode_bytes_ascii_wrapped};

/// Magic/version marker for PPKE v1
const MAGIC: &[u8; 4] = b"BMK1";

/// Algorithm identifiers
const KDF_PBKDF2_HMAC_SHA256: u8 = 0x01;
const AEAD_CHACHA20_POLY1305:   u8 = 0x01;

/// Header constants
const HEADER_LEN: usize = 16; // 4 + 1 + 1 + 4 + 1 + 1 + 4

/// Recommended defaults
pub const DEFAULT_PBKDF2_ITERATIONS: u32 = 600_000;
pub const DEFAULT_SALT_LEN: usize = 16;

/// ChaCha20-Poly1305 specifics
const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;

// --------------------- Internal helpers ---------------------

#[inline]
fn u32_be(x: u32) -> [u8; 4] {
    x.to_be_bytes()
}

#[inline]
fn read_u32_be(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

/// Derive a 32-byte AEAD key using PBKDF2-HMAC-SHA256.
fn derive_aead_key_pbkdf2(password: &[u8], salt: &[u8], iter: NonZeroU32) -> [u8; 32] {
    let mut out = [0u8; 32];
    pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA256, iter, salt, password, &mut out);
    out
}

// --------------------- Public API: In-memory PPKE ---------------------

/// Encrypt `plaintext_key` with `password` into a PPKE v1 binary blob.
///
/// - `iterations`: if `None`, uses [`DEFAULT_PBKDF2_ITERATIONS`]
/// - `salt_len`: recommended 16
///
/// Returns the PPKE v1 binary payload. Suitable for ASCII armor via your ascii_codec.
pub fn export_key_password_protected(
    plaintext_key: &[u8],
    password: &[u8],
    iterations: Option<NonZeroU32>,
    salt_len: usize,
) -> Result<Vec<u8>, &'static str> {
    if salt_len == 0 {
        return Err("salt_len must be > 0");
    }

    let iter = iterations.unwrap_or_else(|| NonZeroU32::new(DEFAULT_PBKDF2_ITERATIONS).unwrap());

    // Random salt
    let rng = ring_rand::SystemRandom::new();
    let mut salt = vec![0u8; salt_len];
    rng.fill(&mut salt).map_err(|_| "rng salt")?;

    // Derive AEAD key
    let aead_key_bytes = derive_aead_key_pbkdf2(password, &salt, iter);

    // AEAD setup
    let unbound = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &aead_key_bytes)
        .map_err(|_| "aead key init")?;
    let key = aead::LessSafeKey::new(unbound);

    // Nonce
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes).map_err(|_| "rng nonce")?;
    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

    // Header (AAD)
    // | MAGIC(4) | kdf_id(1) | aead_id(1) | iter(4) | salt_len(1) | nonce_len(1) | pt_len(4) |
    let mut header = Vec::with_capacity(HEADER_LEN);
    header.extend_from_slice(MAGIC);
    header.push(KDF_PBKDF2_HMAC_SHA256);
    header.push(AEAD_CHACHA20_POLY1305);
    header.extend_from_slice(&u32_be(iter.get()));
    header.push(salt_len as u8);
    header.push(NONCE_LEN as u8);
    header.extend_from_slice(&u32_be(plaintext_key.len() as u32));

    // Seal (in-place, append tag)
    let mut buf = plaintext_key.to_vec();
    key.seal_in_place_append_tag(
        nonce,
        aead::Aad::from(&header),
        &mut buf,
    ).map_err(|_| "aead seal")?;

    // Assemble final payload
    let mut out = Vec::with_capacity(HEADER_LEN + salt_len + NONCE_LEN + buf.len());
    out.extend_from_slice(&header);
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&buf);

    Ok(out)
}

/// Decrypt a PPKE v1 binary `payload` using `password` and return the original key bytes.
pub fn import_key_password_protected(payload: &[u8], password: &[u8]) -> Result<Vec<u8>, &'static str> {
    if payload.len() < HEADER_LEN {
        return Err("payload too short");
    }

    // Parse fixed header
    if &payload[0..4] != MAGIC {
        return Err("bad magic");
    }
    let kdf_id = payload[4];
    if kdf_id != KDF_PBKDF2_HMAC_SHA256 {
        return Err("unsupported kdf");
    }
    let aead_id = payload[5];
    if aead_id != AEAD_CHACHA20_POLY1305 {
        return Err("unsupported aead");
    }

    let iter = read_u32_be(&payload[6..10]);
    let salt_len = payload[10] as usize;
    let nonce_len = payload[11] as usize;
    let pt_len = read_u32_be(&payload[12..16]) as usize;

    if nonce_len != NONCE_LEN {
        return Err("bad nonce_len");
    }

    // Compute positions
    let pos_salt  = HEADER_LEN;
    let pos_nonce = pos_salt + salt_len;
    let pos_ct    = pos_nonce + nonce_len;

    if payload.len() < pos_ct {
        return Err("payload truncated");
    }

    let salt        = &payload[pos_salt .. pos_nonce];
    let nonce_bytes = &payload[pos_nonce .. pos_ct];

    let expected_ct_len = pt_len + TAG_LEN;
    if payload.len() < pos_ct + expected_ct_len {
        return Err("ciphertext truncated");
    }

    let ciphertext = &payload[pos_ct .. pos_ct + expected_ct_len];

    // Derive AEAD key
    let iter_nz = NonZeroU32::new(iter).ok_or("iterations == 0")?;
    let aead_key_bytes = derive_aead_key_pbkdf2(password, salt, iter_nz);

    let unbound = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &aead_key_bytes)
        .map_err(|_| "aead key init")?;
    let key = aead::LessSafeKey::new(unbound);

    // Mutable ciphertext buffer for in-place open
    let mut ct = ciphertext.to_vec();

    // Nonce + AAD must match exactly how we sealed
    let nonce = aead::Nonce::try_assume_unique_for_key(nonce_bytes)
        .map_err(|_| "nonce size")?;
    let aad = aead::Aad::from(&payload[0..HEADER_LEN]);

    // Open in place
    let pt = key.open_in_place(
        nonce,
        aad,
        &mut ct,
    ).map_err(|_| "aead open")?;

    Ok(pt.to_vec())
}

// --------------------- Public API: ASCII-armor file helpers (std only) ---------------------

/// Export a password-protected key to an ASCII-wrapped file.
pub fn export_key_password_protected_ascii_file(
    path: &str,
    plaintext_key: &[u8],
    password: &[u8],
    per_line: usize,
    prefix: &str,
    suffix: &str,
    iterations: Option<NonZeroU32>,
    salt_len: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    let blob = export_key_password_protected(plaintext_key, password, iterations, salt_len)?;
    let armored = encode_bytes_ascii_wrapped(&blob, per_line, prefix, suffix);
    fs::write(path, armored)?;
    Ok(())
}

/// Import a password-protected key from an ASCII-wrapped file.
pub fn import_key_password_protected_ascii_file(
    path: &str,
    password: &[u8],
    prefix: &str,
    suffix: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use std::fs;

    let armored = fs::read_to_string(path)?;
    let blob = decode_bytes_ascii_wrapped(&armored, prefix, suffix)
        .map_err(|e| format!("decode armor: {e}"))?;
    let key = import_key_password_protected(&blob, password)
        .map_err(|e| format!("decrypt: {e}"))?;
    Ok(key)
}
