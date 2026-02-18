// Run: cargo run --example key_ppke --features std
use bitmasher::ppke::{
    export_key_password_protected_ascii_file,
    import_key_password_protected_ascii_file,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plaintext_key = b"\x01\x02\x03super\xFFsecret\x00key"; // demo bytes
    let password = b"correct horse battery staple";

    let prefix = "===BEGIN BITMASHER KEY===\n";
    let suffix = "\n===END BITMASHER KEY===\n";

    // Export (PBKDF2 iters = default, salt_len=16)
    export_key_password_protected_ascii_file(
        "mykey.ppke.asc",
        plaintext_key,
        password,
        8,
        prefix,
        suffix,
        None,         // use default iterations
        16,           // salt_len
    )?;
    println!("Exported key to mykey.ppke.asc");

    // Import
    let roundtrip = import_key_password_protected_ascii_file(
        "mykey.ppke.asc",
        password,
        prefix,
        suffix,
    )?;
    println!("Imported key: {:02X?}", roundtrip);

    assert_eq!(roundtrip, plaintext_key);
    println!("Roundtrip OK.");
    Ok(())
}
