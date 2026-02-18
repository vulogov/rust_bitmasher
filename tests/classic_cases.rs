// tests/classic_cases.rs
use bitmasher::BitArray;

#[test]
fn full_byte_len_left_right() {
    // Little-endian: [0xA1, 0x03] => 0x03A1
    let start = [0b1010_0001, 0b0000_0011];

    // Rotate left by 4 across the contiguous 16-bit bit ring
    let mut b = BitArray::<2>::new(start, 16);
    b.rotate_left(4);
    // ROL16(0x03A1, 4) = 0x3A10 => [0x10, 0x3A]
    assert_eq!(*b.as_bytes(), [0b0001_0000, 0b0011_1010]);

    // Rotate right by 4 returns to original
    b.rotate_right(4);
    assert_eq!(*b.as_bytes(), start);

    // Identity cases: k = 0 and k = bit_len
    let mut b = BitArray::<2>::new(start, 16);
    b.rotate_left(0);
    assert_eq!(*b.as_bytes(), start);
    b.rotate_left(16);
    assert_eq!(*b.as_bytes(), start);

    // k reduced modulo bit_len
    let mut b = BitArray::<2>::new(start, 16);
    b.rotate_left(20); // 20 % 16 == 4
    assert_eq!(*b.as_bytes(), [0x10, 0x3A]);

    // Cross-check against native u16 rotates for a range of k
    let uw = u16::from_le_bytes(start);
    for k in 0..=31 {
        let mut x = BitArray::<2>::new(start, 16);
        x.rotate_left(k);
        let got = u16::from_le_bytes(*x.as_bytes());
        let want = uw.rotate_left((k % 16) as u32);
        assert_eq!(got, want, "ROL mismatch for k={k}");
    }
    for k in 0..=31 {
        let mut x = BitArray::<2>::new(start, 16);
        x.rotate_right(k);
        let got = u16::from_le_bytes(*x.as_bytes());
        let want = uw.rotate_right((k % 16) as u32);
        assert_eq!(got, want, "ROR mismatch for k={k}");
    }
}

#[test]
fn partial_bit_len_masking() {
    // 2 bytes storage but only 10 bits used.
    let mut arr = [0u8; 2];
    // set bits 0,2,4,6,8
    for i in (0..10).step_by(2) {
        let byte = i / 8;
        let off  = i % 8;
        arr[byte] |= 1 << off;
    }
    let mut b = BitArray::<2>::new(arr, 10);

    // The last 6 bits of byte1 are unused and must be 0.
    assert_eq!(b.as_bytes()[1] & !0b0000_0011, 0);

    // Rotate left by 3 within 10-bit space and ensure the tail is masked.
    b.rotate_left(3);
    assert_eq!(b.as_bytes()[1] & !0b0000_0011, 0);

    // Full cycle returns to the same configuration.
    let before = *b.as_bytes();
    b.rotate_left(10);
    assert_eq!(*b.as_bytes(), before);
}

#[test]
fn equivalence_lr() {
    let mut b1 = BitArray::<3>::new([0xAA, 0x55, 0x0F], 20); // 20-bit domain
    let mut b2 = BitArray::<3>::new([0xAA, 0x55, 0x0F], 20);
    b1.rotate_left(7);
    b2.rotate_right(20 - 7);
    assert_eq!(*b1.as_bytes(), *b2.as_bytes());
}
