use bitmasher::keygen::{
    hkdf_sha512_same_len,
    export_key_ascii_file,
    import_key_ascii_file,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Key Export/Import Demo ===");

    // Input material (could be BitArray.as_bytes())
    let input = "super secret 🔑 material".as_bytes();
    let salt  = Some(b"bitmasher-salt".as_ref());
    let info  = b"bitmasher:keygen-demo";

    // Derive key
    let key = hkdf_sha512_same_len(input, salt, info);
    println!("Derived key: {:02X?}", key);

    // Save as ASCII armor
    let prefix = "===BEGIN KEY===\n";
    let suffix = "\n===END KEY===\n";

    export_key_ascii_file("mykey.asc", &key, 8, prefix, suffix)?;
    println!("Saved to file: mykey.asc");

    // Load it back
    let loaded = import_key_ascii_file("mykey.asc", prefix, suffix)?;
    println!("Loaded key: {:02X?}", loaded);

    assert_eq!(loaded, key);
    println!("Roundtrip OK.");

    Ok(())
}
