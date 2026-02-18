use bitmasher::BitArray;

fn main() {
    println!("=== BitArray Practical Usage Example ===\n");

    // 1) Initialize a BitArray
    let bits = BitArray::<2>::new([0b1010_0001, 0b0000_0011], 16);

    println!("Initial bytes:  {:02X?}", bits.as_bytes());
    println!("bit_len:        {}", bits.bit_len());

    // 2) Clone so the original stays available
    let mut left_rot = bits.clone();
    left_rot.rotate_left(5);
    println!("After rotate_left(5): {:02X?}", left_rot.as_bytes());

    let mut right_rot = left_rot.clone();
    right_rot.rotate_right(5);
    println!("After rotate_right(5): {:02X?}", right_rot.as_bytes());

    // SAFE: `bits` was cloned, not moved
    println!("Returns to original:   {}", right_rot.as_bytes() == bits.as_bytes());

    // 3) Partial bit-length example
    let partial = BitArray::<2>::new([0b1110_1101, 0b0000_0011], 13);

    let mut rot = partial.clone();
    rot.rotate_left(3);

    println!("\nPartial 13-bit example:");
    println!("Original: {:02X?}", partial.as_bytes());
    println!("ROL3:     {:02X?}", rot.as_bytes());
}
