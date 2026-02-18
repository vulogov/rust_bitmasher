// Run: cargo run --example pipeline_v2 --features std

use bitmasher::pipeline_v2::process_str_pipeline_v2;
use bitmasher::ppke::export_key_password_protected_ascii_file;
use bitmasher::ascii_codec::encode_bytes_ascii_wrapped;
use bitmasher::pipeline_decode_v2::decode_pipeline_v2_from_files;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let salt = Some(b"bitmasher-salt".as_ref());
    let info = b"bitmasher:pipeline:v2";

    let prefix = "==BEGIN==\n";
    let suffix = "\n==END==";

    let input = "café🙂 Rust!";
    let fwd = process_str_pipeline_v2::<128>(input, salt, info);

    // Export HKDF key (password-protected)
    export_key_password_protected_ascii_file(
        "key_v2.asc", &fwd.hkdf_key, b"pw123",
        8, prefix, suffix, None, 16
    )?;


    // Export **ROTATED** bytes
    let rotated_prefix = &fwd.bitarray_final.as_bytes()[..fwd.used_bytes];
    let ascii = encode_bytes_ascii_wrapped(rotated_prefix, 8, prefix, suffix);
    std::fs::write("data_v2.asc", ascii)?;


    // Decode back
    let recovered = decode_pipeline_v2_from_files::<128>(
        "key_v2.asc", "data_v2.asc", b"pw123", prefix, suffix, salt, info
    ).expect("decode v2");

    assert_eq!(recovered, input);
    println!("Recovered: {}", recovered);

    // cleanup
    std::fs::remove_file("key_v2.asc")?;
    std::fs::remove_file("data_v2.asc")?;
    Ok(())
}
