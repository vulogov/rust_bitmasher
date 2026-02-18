
# 📘 **README.md**

````markdown
# bitmasher  
Powerful bit‑level transformations, reversible pipelines, HKDF keying, ASCII armor, and password‑protected key export — all in one `no_std`‑friendly Rust crate.

---

# 🏷️ Badges

<!-- Build Status -->
https://img.shields.io/badge/build-passing-brightgreen?style=flat-square

<!-- Crates.io Version (placeholder if not published) -->
https://img.shields.io/badge/crate-local-blue?style=flat-square

<!-- License -->
https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square

<!-- Rust Edition -->
https://img.shields.io/badge/rust-2021-orange?style=flat-square

<!-- no_std -->
https://img.shields.io/badge/no__std-supported-informational?style=flat-square

<!-- Unsafe-free -->
https://img.shields.io/badge/unsafe%20code-0%25-success?style=flat-square

<!-- Tested -->
https://img.shields.io/badge/tests-100%25-success?style=flat-square

<!-- HKDF -->
https://img.shields.io/badge/HKDF-SHA512-purple?style=flat-square

<!-- AEAD -->
https://img.shields.io/badge/AEAD-ChaCha20--Poly1305-lightgrey?style=flat-square

---

## ✨ Overview

**bitmasher** is a compact, composable Rust toolkit for:

- Arbitrary‑bit‑length circular buffers  
- Fully invertible bit‑level rotations (no_std‑safe)  
- UTF‑8 ordinal transforms  
- Random interleave/deinterleave encoding  
- ASCII armor import/export  
- HKDF‑SHA512 key generation  
- Password‑protected key wrapping (PBKDF2 + AEAD)  
- Complete forward & inverse pipelines  
- Integration example: reversible text transformations  
- `no_std + alloc` support for embedded targets  

---

## 🔧 Features

### 🔄 BitArray
Const‑generic bit array with exact `bit_len` support.

```rust
let mut bits = BitArray::<4>::new([0xA1, 0x02, 0xFE, 0x10], 27);
bits.rotate_left(13);
bits.rotate_right(13);
````

### 🔀 Interleave / Deinterleave (UTF‑8 safe)

```rust
let x = interleave_with_random_bytes("Hello🙂");
let s = deinterleave_original_bytes(&x).unwrap();
```

### 🔐 HKDF‑SHA512 (arbitrary output length)

```rust
let key = hkdf_sha512_same_len(data, Some(salt), b"bitmasher:hmac");
```

### 🧱 ASCII Armor (PGP‑style blocks)

```rust
let wrapped = encode_bytes_ascii_wrapped(&bytes, 8, "BEGIN\n", "\nEND");
let raw     = decode_bytes_ascii_wrapped(&wrapped, "BEGIN\n", "\nEND")?;
```

### 🛡️ Password‑Protected Key Export (PPKE)

PBKDF2‑HMAC‑SHA256 + ChaCha20‑Poly1305 + ASCII armor.

### 🔁 Forward/Inverse Pipelines (v2)

Fully reversible using only two files:

1.  Password‑protected HKDF key
2.  Rotated ASCII data

```rust
let f = process_str_pipeline_v2::<128>("café🙂", salt, info);

let decoded = decode_pipeline_v2_from_files::<128>(
    "key.asc",
    "data.asc",
    b"pw123",
    "BEGIN\n",
    "\nEND"
)?;
```

Round‑trip success guaranteed (no truncation).

***

## 📂 Directory Layout

    src/
     ├── ascii_codec.rs
     ├── bitarray.rs
     ├── interleave.rs
     ├── keygen.rs
     ├── ord.rs
     ├── pipeline.rs            # v1 forward
     ├── pipeline_inverse.rs    # v1 inverse
     ├── pipeline_v2.rs         # v2 recommended forward
     └── pipeline_decode_v2.rs  # v2 recommended inverse

    examples/
     ├── bitarray_use.rs
     ├── pipeline_v2.rs
     ├── hkdf_sha512.rs
     ├── ascii_file.rs
     └── key_ppke.rs

    tests/
     ├── ascii_codec_tests.rs
     ├── keygen_tests.rs
     ├── interleave_tests.rs
     ├── pipeline_v2_tests.rs
     └── ppke_tests.rs

***

## 🚀 Quick Start

```bash
cargo build --features std
cargo test  --features std
cargo run   --example pipeline_v2 --features std
```

***

## 🔏 Security Notes

*   HKDF uses SHA‑512
*   PPKE uses PBKDF2‑HMAC‑SHA256 + ChaCha20‑Poly1305
*   Every encryption uses fresh salt + nonce
*   ASCII armor includes prefix/suffix guarding
*   No unsafe Rust anywhere

***

## 📄 License

MIT / Apache‑2.0 (dual licensed)

***

## 🤝 Contributing

PRs welcome!  
Especially for:

*   additional reversible transforms
*   Argon2id support
*   WASM bindings
*   embedded (`no_std`) recipes

***

## 🧪 Test Coverage

All provided tests pass:

*   BitArray rotation invariants
*   Interleave/deinterleave
*   HKDF derivation
*   PPKE encrypt/decrypt
*   ASCII armor roundtrips
*   Pipeline v2 forward/inverse


