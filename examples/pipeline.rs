use bitmasher::pipeline::process_str_pipeline;

fn main() {
    let salt = Some(b"bitmasher-salt".as_ref());
    let info = b"bitmasher:pipeline:test";

    let result = process_str_pipeline::<32>("café🙂 Rust!", salt, info);

    println!("Original: {}", result.original);
    println!("Interleaved: {:02X?}", result.interleaved);
    println!("HKDF key: {:02X?}", result.hkdf_key);
    println!("Ordinals: {:?}", result.ordinals);
    println!("Final BitArray bytes: {:02X?}", result.bitarray_final.as_bytes());
}
