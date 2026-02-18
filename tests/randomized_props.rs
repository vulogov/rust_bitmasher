// tests/randomized_props.rs
use bitmasher::BitArray;

mod common;
use common::{XorShift64, rotate_left_ref, assert_tail_masked};

#[test]
fn randomized_contiguous_rotation_matches_reference() {
    let mut rng = XorShift64::new(0xC0FFEE_F00D_F00D);

    for n_bytes in 1..=8 {
        for _case in 0..200 {
            // Random input buffer of length n_bytes
            let mut input = vec![0u8; n_bytes];
            rng.fill_bytes(&mut input);

            // Random bit_len in [0, n_bytes*8]
            let bit_len = rng.gen_range(0, n_bytes * 8 + 1);
            // Random k with values beyond bit_len to exercise modulo paths
            let k = if bit_len == 0 { 0 } else { rng.gen_range(0, 5 * bit_len + 8) };

            // Reference result using bit-accurate model
            let expected = rotate_left_ref(&input, bit_len, k);

            match n_bytes {
                1 => {
                    let mut b = BitArray::<1>::new([input[0]], bit_len);
                    b.rotate_left(k);
                    assert_eq!(&b.as_bytes()[..], &expected[..1], "Mismatch for N=1, bit_len={bit_len}, k={k}");
                    assert_tail_masked(b.as_bytes(), bit_len);
                }
                2 => {
                    let mut b = BitArray::<2>::new([input[0], input[1]], bit_len);
                    b.rotate_left(k);
                    assert_eq!(&b.as_bytes()[..], &expected[..2], "Mismatch for N=2, bit_len={bit_len}, k={k}");
                    assert_tail_masked(b.as_bytes(), bit_len);
                }
                3 => {
                    let mut b = BitArray::<3>::new([input[0], input[1], input[2]], bit_len);
                    b.rotate_left(k);
                    assert_eq!(&b.as_bytes()[..], &expected[..3], "Mismatch for N=3, bit_len={bit_len}, k={k}");
                    assert_tail_masked(b.as_bytes(), bit_len);
                }
                4 => {
                    let mut b = BitArray::<4>::new([input[0], input[1], input[2], input[3]], bit_len);
                    b.rotate_left(k);
                    assert_eq!(&b.as_bytes()[..], &expected[..4], "Mismatch for N=4, bit_len={bit_len}, k={k}");
                    assert_tail_masked(b.as_bytes(), bit_len);
                }
                5 => {
                    let mut b = BitArray::<5>::new([input[0], input[1], input[2], input[3], input[4]], bit_len);
                    b.rotate_left(k);
                    assert_eq!(&b.as_bytes()[..], &expected[..5], "Mismatch for N=5, bit_len={bit_len}, k={k}");
                    assert_tail_masked(b.as_bytes(), bit_len);
                }
                6 => {
                    let mut b = BitArray::<6>::new([input[0], input[1], input[2], input[3], input[4], input[5]], bit_len);
                    b.rotate_left(k);
                    assert_eq!(&b.as_bytes()[..], &expected[..6], "Mismatch for N=6, bit_len={bit_len}, k={k}");
                    assert_tail_masked(b.as_bytes(), bit_len);
                }
                7 => {
                    let mut b = BitArray::<7>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6]], bit_len);
                    b.rotate_left(k);
                    assert_eq!(&b.as_bytes()[..], &expected[..7], "Mismatch for N=7, bit_len={bit_len}, k={k}");
                    assert_tail_masked(b.as_bytes(), bit_len);
                }
                8 => {
                    let mut b = BitArray::<8>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]], bit_len);
                    b.rotate_left(k);
                    assert_eq!(&b.as_bytes()[..], &expected[..8], "Mismatch for N=8, bit_len={bit_len}, k={k}");
                    assert_tail_masked(b.as_bytes(), bit_len);
                }
                _ => unreachable!(),
            }
        }
    }
}

#[test]
fn randomized_roundtrip_identity() {
    let mut rng = XorShift64::new(0xDEAD_BEEF_CAFE);

    for n_bytes in 1..=8 {
        for _case in 0..200 {
            let mut input = vec![0u8; n_bytes];
            rng.fill_bytes(&mut input);

            let bit_len = rng.gen_range(0, n_bytes * 8 + 1);
            let k = if bit_len == 0 { 0 } else { rng.gen_range(0, 3 * bit_len + 1) };

            match n_bytes {
                1 => {
                    let mut b = BitArray::<1>::new([input[0]], bit_len);
                    let orig = *b.as_bytes();
                    b.rotate_left(k);
                    b.rotate_right(k);
                    assert_eq!(*b.as_bytes(), orig, "ROL then ROR must be identity (N=1, bit_len={bit_len}, k={k})");
                    assert_tail_masked(b.as_bytes(), bit_len);

                    let mut c = BitArray::<1>::new([input[0]], bit_len);
                    let orig2 = *c.as_bytes();
                    c.rotate_right(k);
                    c.rotate_left(k);
                    assert_eq!(*c.as_bytes(), orig2, "ROR then ROL must be identity (N=1, bit_len={bit_len}, k={k})");
                    assert_tail_masked(c.as_bytes(), bit_len);
                }
                2 => {
                    let mut b = BitArray::<2>::new([input[0], input[1]], bit_len);
                    let orig = *b.as_bytes();
                    b.rotate_left(k);
                    b.rotate_right(k);
                    assert_eq!(*b.as_bytes(), orig, "ROL then ROR must be identity (N=2, bit_len={bit_len}, k={k})");
                    assert_tail_masked(b.as_bytes(), bit_len);

                    let mut c = BitArray::<2>::new([input[0], input[1]], bit_len);
                    let orig2 = *c.as_bytes();
                    c.rotate_right(k);
                    c.rotate_left(k);
                    assert_eq!(*c.as_bytes(), orig2, "ROR then ROL must be identity (N=2, bit_len={bit_len}, k={k})");
                    assert_tail_masked(c.as_bytes(), bit_len);
                }
                3 => {
                    let mut b = BitArray::<3>::new([input[0], input[1], input[2]], bit_len);
                    let orig = *b.as_bytes();
                    b.rotate_left(k);
                    b.rotate_right(k);
                    assert_eq!(*b.as_bytes(), orig, "ROL then ROR must be identity (N=3, bit_len={bit_len}, k={k})");
                    assert_tail_masked(b.as_bytes(), bit_len);

                    let mut c = BitArray::<3>::new([input[0], input[1], input[2]], bit_len);
                    let orig2 = *c.as_bytes();
                    c.rotate_right(k);
                    c.rotate_left(k);
                    assert_eq!(*c.as_bytes(), orig2, "ROR then ROL must be identity (N=3, bit_len={bit_len}, k={k})");
                    assert_tail_masked(c.as_bytes(), bit_len);
                }
                4 => {
                    let mut b = BitArray::<4>::new([input[0], input[1], input[2], input[3]], bit_len);
                    let orig = *b.as_bytes();
                    b.rotate_left(k);
                    b.rotate_right(k);
                    assert_eq!(*b.as_bytes(), orig, "ROL then ROR must be identity (N=4, bit_len={bit_len}, k={k})");
                    assert_tail_masked(b.as_bytes(), bit_len);

                    let mut c = BitArray::<4>::new([input[0], input[1], input[2], input[3]], bit_len);
                    let orig2 = *c.as_bytes();
                    c.rotate_right(k);
                    c.rotate_left(k);
                    assert_eq!(*c.as_bytes(), orig2, "ROR then ROL must be identity (N=4, bit_len={bit_len}, k={k})");
                    assert_tail_masked(c.as_bytes(), bit_len);
                }
                5 => {
                    let mut b = BitArray::<5>::new([input[0], input[1], input[2], input[3], input[4]], bit_len);
                    let orig = *b.as_bytes();
                    b.rotate_left(k);
                    b.rotate_right(k);
                    assert_eq!(*b.as_bytes(), orig, "ROL then ROR must be identity (N=5, bit_len={bit_len}, k={k})");
                    assert_tail_masked(b.as_bytes(), bit_len);

                    let mut c = BitArray::<5>::new([input[0], input[1], input[2], input[3], input[4]], bit_len);
                    let orig2 = *c.as_bytes();
                    c.rotate_right(k);
                    c.rotate_left(k);
                    assert_eq!(*c.as_bytes(), orig2, "ROR then ROL must be identity (N=5, bit_len={bit_len}, k={k})");
                    assert_tail_masked(c.as_bytes(), bit_len);
                }
                6 => {
                    let mut b = BitArray::<6>::new([input[0], input[1], input[2], input[3], input[4], input[5]], bit_len);
                    let orig = *b.as_bytes();
                    b.rotate_left(k);
                    b.rotate_right(k);
                    assert_eq!(*b.as_bytes(), orig, "ROL then ROR must be identity (N=6, bit_len={bit_len}, k={k})");
                    assert_tail_masked(b.as_bytes(), bit_len);

                    let mut c = BitArray::<6>::new([input[0], input[1], input[2], input[3], input[4], input[5]], bit_len);
                    let orig2 = *c.as_bytes();
                    c.rotate_right(k);
                    c.rotate_left(k);
                    assert_eq!(*c.as_bytes(), orig2, "ROR then ROL must be identity (N=6, bit_len={bit_len}, k={k})");
                    assert_tail_masked(c.as_bytes(), bit_len);
                }
                7 => {
                    let mut b = BitArray::<7>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6]], bit_len);
                    let orig = *b.as_bytes();
                    b.rotate_left(k);
                    b.rotate_right(k);
                    assert_eq!(*b.as_bytes(), orig, "ROL then ROR must be identity (N=7, bit_len={bit_len}, k={k})");
                    assert_tail_masked(b.as_bytes(), bit_len);

                    let mut c = BitArray::<7>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6]], bit_len);
                    let orig2 = *c.as_bytes();
                    c.rotate_right(k);
                    c.rotate_left(k);
                    assert_eq!(*c.as_bytes(), orig2, "ROR then ROL must be identity (N=7, bit_len={bit_len}, k={k})");
                    assert_tail_masked(c.as_bytes(), bit_len);
                }
                8 => {
                    let mut b = BitArray::<8>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]], bit_len);
                    let orig = *b.as_bytes();
                    b.rotate_left(k);
                    b.rotate_right(k);
                    assert_eq!(*b.as_bytes(), orig, "ROL then ROR must be identity (N=8, bit_len={bit_len}, k={k})");
                    assert_tail_masked(b.as_bytes(), bit_len);

                    let mut c = BitArray::<8>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]], bit_len);
                    let orig2 = *c.as_bytes();
                    c.rotate_right(k);
                    c.rotate_left(k);
                    assert_eq!(*c.as_bytes(), orig2, "ROR then ROL must be identity (N=8, bit_len={bit_len}, k={k})");
                    assert_tail_masked(c.as_bytes(), bit_len);
                }
                _ => unreachable!(),
            }
        }
    }
}

#[test]
fn randomized_modulo_and_edges() {
    let mut rng = XorShift64::new(0x1234_5678_9ABC_DEF0);

    for n_bytes in 1..=8 {
        // Exercise specific edge bit lengths; keep only valid values
        let mut bit_lens = vec![0usize, 1, 7, 8, 9, n_bytes * 8 - 1, n_bytes * 8];
        bit_lens.retain(|&bl| bl <= n_bytes * 8);

        for &bit_len in &bit_lens {
            for _case in 0..100 {
                let mut input = vec![0u8; n_bytes];
                rng.fill_bytes(&mut input);
                let k = if bit_len == 0 { 0 } else { rng.gen_range(0, 4 * bit_len + 5) };

                match n_bytes {
                    1 => {
                        let mut a1 = BitArray::<1>::new([input[0]], bit_len);
                        let mut a2 = BitArray::<1>::new([input[0]], bit_len);
                        if bit_len > 0 {
                            a1.rotate_left(k);
                            a2.rotate_left(k % bit_len);
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes(), "ROL modulo semantics failed (N=1, bit_len={bit_len}, k={k})");
                        } else {
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes());
                        }

                        let mut id = BitArray::<1>::new([input[0]], bit_len);
                        let orig = *id.as_bytes();
                        id.rotate_left(0);
                        assert_eq!(*id.as_bytes(), orig, "k=0 must be identity (N=1)");
                        if bit_len > 0 {
                            id.rotate_left(bit_len);
                            assert_eq!(*id.as_bytes(), orig, "k=bit_len must be identity (N=1)");
                        }
                        assert_tail_masked(id.as_bytes(), bit_len);
                    }
                    2 => {
                        let mut a1 = BitArray::<2>::new([input[0], input[1]], bit_len);
                        let mut a2 = BitArray::<2>::new([input[0], input[1]], bit_len);
                        if bit_len > 0 {
                            a1.rotate_left(k);
                            a2.rotate_left(k % bit_len);
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes(), "ROL modulo semantics failed (N=2, bit_len={bit_len}, k={k})");
                        } else {
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes());
                        }

                        let mut id = BitArray::<2>::new([input[0], input[1]], bit_len);
                        let orig = *id.as_bytes();
                        id.rotate_left(0);
                        assert_eq!(*id.as_bytes(), orig, "k=0 must be identity (N=2)");
                        if bit_len > 0 {
                            id.rotate_left(bit_len);
                            assert_eq!(*id.as_bytes(), orig, "k=bit_len must be identity (N=2)");
                        }
                        assert_tail_masked(id.as_bytes(), bit_len);
                    }
                    3 => {
                        let mut a1 = BitArray::<3>::new([input[0], input[1], input[2]], bit_len);
                        let mut a2 = BitArray::<3>::new([input[0], input[1], input[2]], bit_len);
                        if bit_len > 0 {
                            a1.rotate_left(k);
                            a2.rotate_left(k % bit_len);
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes(), "ROL modulo semantics failed (N=3, bit_len={bit_len}, k={k})");
                        } else {
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes());
                        }

                        let mut id = BitArray::<3>::new([input[0], input[1], input[2]], bit_len);
                        let orig = *id.as_bytes();
                        id.rotate_left(0);
                        assert_eq!(*id.as_bytes(), orig, "k=0 must be identity (N=3)");
                        if bit_len > 0 {
                            id.rotate_left(bit_len);
                            assert_eq!(*id.as_bytes(), orig, "k=bit_len must be identity (N=3)");
                        }
                        assert_tail_masked(id.as_bytes(), bit_len);
                    }
                    4 => {
                        let mut a1 = BitArray::<4>::new([input[0], input[1], input[2], input[3]], bit_len);
                        let mut a2 = BitArray::<4>::new([input[0], input[1], input[2], input[3]], bit_len);
                        if bit_len > 0 {
                            a1.rotate_left(k);
                            a2.rotate_left(k % bit_len);
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes(), "ROL modulo semantics failed (N=4, bit_len={bit_len}, k={k})");
                        } else {
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes());
                        }

                        let mut id = BitArray::<4>::new([input[0], input[1], input[2], input[3]], bit_len);
                        let orig = *id.as_bytes();
                        id.rotate_left(0);
                        assert_eq!(*id.as_bytes(), orig, "k=0 must be identity (N=4)");
                        if bit_len > 0 {
                            id.rotate_left(bit_len);
                            assert_eq!(*id.as_bytes(), orig, "k=bit_len must be identity (N=4)");
                        }
                        assert_tail_masked(id.as_bytes(), bit_len);
                    }
                    5 => {
                        let mut a1 = BitArray::<5>::new([input[0], input[1], input[2], input[3], input[4]], bit_len);
                        let mut a2 = BitArray::<5>::new([input[0], input[1], input[2], input[3], input[4]], bit_len);
                        if bit_len > 0 {
                            a1.rotate_left(k);
                            a2.rotate_left(k % bit_len);
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes(), "ROL modulo semantics failed (N=5, bit_len={bit_len}, k={k})");
                        } else {
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes());
                        }

                        let mut id = BitArray::<5>::new([input[0], input[1], input[2], input[3], input[4]], bit_len);
                        let orig = *id.as_bytes();
                        id.rotate_left(0);
                        assert_eq!(*id.as_bytes(), orig, "k=0 must be identity (N=5)");
                        if bit_len > 0 {
                            id.rotate_left(bit_len);
                            assert_eq!(*id.as_bytes(), orig, "k=bit_len must be identity (N=5)");
                        }
                        assert_tail_masked(id.as_bytes(), bit_len);
                    }
                    6 => {
                        let mut a1 = BitArray::<6>::new([input[0], input[1], input[2], input[3], input[4], input[5]], bit_len);
                        let mut a2 = BitArray::<6>::new([input[0], input[1], input[2], input[3], input[4], input[5]], bit_len);
                        if bit_len > 0 {
                            a1.rotate_left(k);
                            a2.rotate_left(k % bit_len);
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes(), "ROL modulo semantics failed (N=6, bit_len={bit_len}, k={k})");
                        } else {
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes());
                        }

                        let mut id = BitArray::<6>::new([input[0], input[1], input[2], input[3], input[4], input[5]], bit_len);
                        let orig = *id.as_bytes();
                        id.rotate_left(0);
                        assert_eq!(*id.as_bytes(), orig, "k=0 must be identity (N=6)");
                        if bit_len > 0 {
                            id.rotate_left(bit_len);
                            assert_eq!(*id.as_bytes(), orig, "k=bit_len must be identity (N=6)");
                        }
                        assert_tail_masked(id.as_bytes(), bit_len);
                    }
                    7 => {
                        let mut a1 = BitArray::<7>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6]], bit_len);
                        let mut a2 = BitArray::<7>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6]], bit_len);
                        if bit_len > 0 {
                            a1.rotate_left(k);
                            a2.rotate_left(k % bit_len);
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes(), "ROL modulo semantics failed (N=7, bit_len={bit_len}, k={k})");
                        } else {
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes());
                        }

                        let mut id = BitArray::<7>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6]], bit_len);
                        let orig = *id.as_bytes();
                        id.rotate_left(0);
                        assert_eq!(*id.as_bytes(), orig, "k=0 must be identity (N=7)");
                        if bit_len > 0 {
                            id.rotate_left(bit_len);
                            assert_eq!(*id.as_bytes(), orig, "k=bit_len must be identity (N=7)");
                        }
                        assert_tail_masked(id.as_bytes(), bit_len);
                    }
                    8 => {
                        let mut a1 = BitArray::<8>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]], bit_len);
                        let mut a2 = BitArray::<8>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]], bit_len);
                        if bit_len > 0 {
                            a1.rotate_left(k);
                            a2.rotate_left(k % bit_len);
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes(), "ROL modulo semantics failed (N=8, bit_len={bit_len}, k={k})");
                        } else {
                            assert_eq!(*a1.as_bytes(), *a2.as_bytes());
                        }

                        let mut id = BitArray::<8>::new([input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]], bit_len);
                        let orig = *id.as_bytes();
                        id.rotate_left(0);
                        assert_eq!(*id.as_bytes(), orig, "k=0 must be identity (N=8)");
                        if bit_len > 0 {
                            id.rotate_left(bit_len);
                            assert_eq!(*id.as_bytes(), orig, "k=bit_len must be identity (N=8)");
                        }
                        assert_tail_masked(id.as_bytes(), bit_len);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
