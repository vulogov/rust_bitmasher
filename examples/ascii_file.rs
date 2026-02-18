use bitmasher::ascii_codec::{
    encode_bytes_ascii_wrapped,
    decode_bytes_ascii_wrapped,
    read_ascii_file_wrapped,
    write_ascii_file_wrapped,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = "Hello🙂".as_bytes(); // arbitrary utf8 bytes
    let prefix = "===BEGIN===\n";
    let suffix = "\n===END===";

    // 1. Encode to string
    let wrapped = encode_bytes_ascii_wrapped(data, 4, prefix, suffix);
    println!("Wrapped ASCII:\n{wrapped}");

    // 2. Decode straight from memory
    let decoded = decode_bytes_ascii_wrapped(&wrapped, prefix, suffix)?;
    assert_eq!(decoded, data);

    // 3. Write to file
    write_ascii_file_wrapped("encoded.txt", data, 4, prefix, suffix)?;

    // 4. Read back
    let from_file = read_ascii_file_wrapped("encoded.txt", prefix, suffix)?;
    assert_eq!(from_file, data);
    println!("File roundtrip OK");

    Ok(())
}
