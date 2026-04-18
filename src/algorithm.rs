// |error| <= 5.60096 × 10^-2
// ULP = 2^-6 = 1.56250 × 10^-2
#[must_use]
pub(crate) const fn cospi_i8_2(theta: i8) -> i8 {
    const A: u16 = COEFFICIENT + ROUND;
    const BITS: i32 = i8::BITS.cast_signed();
    const COEFFICIENT: u16 = 1 << (BITS - 2) * 2;
    const E_OUT: i32 = -(BITS - 2);
    const E_XX: i32 = -(BITS - 2) * 2;
    const MSB: i8 = 1 << (BITS - 1);
    const SHIFT: u32 = (E_OUT - E_XX).cast_unsigned();
    const ROUND: u16 = !(!0 << (SHIFT - 1));

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u16;
    let xx = x.wrapping_mul(x);
    let y = A.wrapping_sub(xx) >> SHIFT;
    let mut y = y as i8;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 9.18799 × 10^-4
// ULP = 2^-6 = 1.56250 × 10^-2
#[must_use]
pub(crate) const fn cospi_i8_4(theta: i8) -> i8 {
    const BITS: i32 = i8::BITS.cast_signed();
    const COEFFICIENTS: [(u16, i32); 3] = [(0x8000, -15), (0x9CAC, -11), (0xE562, -10)];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i8 = 1 << (BITS - 1);
    const ROUND_XX: u16 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 3] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 3];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u16; 3] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 3];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u16;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    let mut y = y as i8;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 5.60096 × 10^-2
// ULP = 2^-14 = 6.10352 × 10^-5
#[must_use]
pub(crate) const fn cospi_i16_2(theta: i16) -> i16 {
    const A: u32 = COEFFICIENT + ROUND;
    const BITS: i32 = i16::BITS.cast_signed();
    const COEFFICIENT: u32 = 1 << (BITS - 2) * 2;
    const E_OUT: i32 = -(BITS - 2);
    const E_XX: i32 = -(BITS - 2) * 2;
    const MSB: i16 = 1 << (BITS - 1);
    const ROUND: u32 = !(!0 << (SHIFT - 1));
    const SHIFT: u32 = (E_OUT - E_XX).cast_unsigned();

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u32;
    let xx = x.wrapping_mul(x);
    let y = A.wrapping_sub(xx) >> SHIFT;
    let mut y = y as i16;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 9.18799 × 10^-4
// ULP = 2^-14 = 6.10352 × 10^-5
#[must_use]
pub(crate) const fn cospi_i16_4(theta: i16) -> i16 {
    const BITS: i32 = i16::BITS.cast_signed();
    const COEFFICIENTS: [(u32, i32); 3] = [(0x80000000, -31), (0x9CAC4C97, -27), (0xE56264B5, -26)];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i16 = 1 << (BITS - 1);
    const ROUND_XX: u32 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 3] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 3];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u32; 3] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 3];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u32;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    let mut y = y as i16;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 9.20285 × 10^-6
// ULP = 2^-14 = 6.10352 × 10^-5
#[must_use]
pub(crate) const fn cospi_i16_6(theta: i16) -> i16 {
    const BITS: i32 = i16::BITS.cast_signed();
    const COEFFICIENTS: [(u32, i32); 4] = [
        (0x80000000, -31),
        (0x9DE408E9, -27),
        (0x81571F59, -25),
        (0x9C6FBB2D, -25),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i16 = 1 << (BITS - 1);
    const ROUND_XX: u32 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 4] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 4];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u32; 4] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 4];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u32;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    let mut y = y as i16;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 5.60096 × 10^-2
// ULP = 2^-30 = 9.31323 × 10^-10
#[must_use]
pub(crate) const fn cospi_i32_2(theta: i32) -> i32 {
    const A: u64 = COEFFICIENT + ROUND;
    const BITS: i32 = i32::BITS.cast_signed();
    const COEFFICIENT: u64 = 1 << (BITS - 2) * 2;
    const E_OUT: i32 = -(BITS - 2);
    const E_XX: i32 = -(BITS - 2) * 2;
    const MSB: i32 = 1 << (BITS - 1);
    const ROUND: u64 = !(!0 << (SHIFT - 1));
    const SHIFT: u32 = (E_OUT - E_XX).cast_unsigned();

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u64;
    let xx = x.wrapping_mul(x);
    let y = A.wrapping_sub(xx) >> SHIFT;
    let mut y = y as i32;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 9.18799 × 10^-4
// ULP = 2^-30 = 9.31323 × 10^-10
#[must_use]
pub(crate) const fn cospi_i32_4(theta: i32) -> i32 {
    const BITS: i32 = i32::BITS.cast_signed();
    const COEFFICIENTS: [(u64, i32); 3] = [
        (0x8000000000000000, -63),
        (0x9CAC4C9693EB74C2, -59),
        (0xE56264B49F5BA612, -58),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i32 = 1 << (BITS - 1);
    const ROUND_XX: u64 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 3] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 3];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u64; 3] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 3];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u64;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    let mut y = y as i32;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 9.20285 × 10^-6
// ULP = 2^-30 = 9.31323 × 10^-10
#[must_use]
pub(crate) const fn cospi_i32_6(theta: i32) -> i32 {
    const BITS: i32 = i32::BITS.cast_signed();
    const COEFFICIENTS: [(u64, i32); 4] = [
        (0x8000000000000000, -63),
        (0x9DE408E97E84C3F6, -59),
        (0x81571F58D0AA1ED5, -57),
        (0x9C6FBB2D6970EFBB, -57),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i32 = 1 << (BITS - 1);
    const ROUND_XX: u64 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 4] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 4];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u64; 4] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 4];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u64;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    let mut y = y as i32;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 5.98045 × 10^-8
// ULP = 2^-30 = 9.31323 × 10^-10
#[must_use]
pub(crate) const fn cospi_i32_8(theta: i32) -> i32 {
    const BITS: i32 = i32::BITS.cast_signed();
    const COEFFICIENTS: [(u64, i32); 5] = [
        (0x8000000000000000, -63),
        (0x9DE9D6CE594BC7A2, -59),
        (0x81DEAA9595D7601A, -57),
        (0xAA7CBC1DE16B509A, -57),
        (0xE0F8CB5ADCE6EF86, -58),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i32 = 1 << (BITS - 1);
    const ROUND_XX: u64 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 5] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 5];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u64; 5] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 5];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u64;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    y = const { A[4] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[4] };
    let mut y = y as i32;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 2.70068 × 10^-10
// ULP = 2^-30 = 9.31323 × 10^-10
#[must_use]
pub(crate) const fn cospi_i32_10(theta: i32) -> i32 {
    const BITS: i32 = i32::BITS.cast_signed();
    const COEFFICIENTS: [(u64, i32); 6] = [
        (0x8000000000000000, -63),
        (0x9DE9E633E9CD50C3, -59),
        (0x81E0F292F44046CA, -57),
        (0xAAE847A7763AB7C3, -57),
        (0xF0944D347C4F3D50, -58),
        (0xC7DBDBF95FF7AC7C, -59),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i32 = 1 << (BITS - 1);
    const ROUND_XX: u64 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 6] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 6];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u64; 6] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 6];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u64;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    y = const { A[4] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[4] };
    y = const { A[5] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[5] };
    let mut y = y as i32;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 5.60096 × 10^-2
// ULP = 2^-62 = 2.16840 × 10^-19
#[must_use]
pub(crate) const fn cospi_i64_2(theta: i64) -> i64 {
    const A: u128 = COEFFICIENT + ROUND;
    const BITS: i32 = i64::BITS.cast_signed();
    const COEFFICIENT: u128 = 1 << (BITS - 2) * 2;
    const E_OUT: i32 = -(BITS - 2);
    const E_XX: i32 = -(BITS - 2) * 2;
    const MSB: i64 = 1 << (BITS - 1);
    const ROUND: u128 = !(!0 << (SHIFT - 1));
    const SHIFT: u32 = (E_OUT - E_XX).cast_unsigned();

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u128;
    let xx = x.wrapping_mul(x);
    let y = A.wrapping_sub(xx) >> SHIFT;
    let mut y = y as i64;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 9.18799 × 10^-4
// ULP = 2^-62 = 2.16840 × 10^-19
#[must_use]
pub(crate) const fn cospi_i64_4(theta: i64) -> i64 {
    const BITS: i32 = i64::BITS.cast_signed();
    const COEFFICIENTS: [(u128, i32); 3] = [
        (0x80000000000000000000000000000000, -127),
        (0x9CAC4C9693EB74C23ECED9AAB47AB986, -123),
        (0xE56264B49F5BA611F676CD55A3D5CC2E, -122),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i64 = 1 << (BITS - 1);
    const ROUND_XX: u128 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 3] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 3];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u128; 3] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 3];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u128;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    let mut y = y as i64;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 9.20285 × 10^-6
// ULP = 2^-62 = 2.16840 × 10^-19
#[must_use]
pub(crate) const fn cospi_i64_6(theta: i64) -> i64 {
    const BITS: i32 = i64::BITS.cast_signed();
    const COEFFICIENTS: [(u128, i32); 4] = [
        (0x80000000000000000000000000000000, -127),
        (0x9DE408E97E84C3F66174B3CA1F41E2A6, -123),
        (0x81571F58D0AA1ED531D5DF865AB7BAC2, -121),
        (0x9C6FBB2D6970EFBAC03105DDDB0302B0, -121),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i64 = 1 << (BITS - 1);
    const ROUND_XX: u128 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 4] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 4];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u128; 4] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 4];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u128;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    let mut y = y as i64;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 5.98045 × 10^-8
// ULP = 2^-62 = 2.16840 × 10^-19
#[must_use]
pub(crate) const fn cospi_i64_8(theta: i64) -> i64 {
    const BITS: i32 = i64::BITS.cast_signed();
    const COEFFICIENTS: [(u128, i32); 5] = [
        (0x80000000000000000000000000000000, -127),
        (0x9DE9D6CE594BC7A1F949B92A8E4CAE3F, -123),
        (0x81DEAA9595D76019C0B56E6E77013811, -121),
        (0xAA7CBC1DE16B5099E760250A6BF8B16B, -121),
        (0xE0F8CB5ADCE6EF85CEF118D1E2180101, -122),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i64 = 1 << (BITS - 1);
    const ROUND_XX: u128 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 5] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 5];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u128; 5] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 5];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u128;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    y = const { A[4] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[4] };
    let mut y = y as i64;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 2.70068 × 10^-10
// ULP = 2^-62 = 2.16840 × 10^-19
#[must_use]
pub(crate) const fn cospi_i64_10(theta: i64) -> i64 {
    const BITS: i32 = i64::BITS.cast_signed();
    const COEFFICIENTS: [(u128, i32); 6] = [
        (0x80000000000000000000000000000000, -127),
        (0x9DE9E633E9CD50C35829BB5916F41C57, -123),
        (0x81E0F292F44046C99B8815A383C7252C, -121),
        (0xAAE847A7763AB7C310AD2745287545C7, -121),
        (0xF0944D347C4F3D50310D1F77AD06B279, -122),
        (0xC7DBDBF95FF7AC7BB716A45178B2CA65, -123),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i64 = 1 << (BITS - 1);
    const ROUND_XX: u128 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 6] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 6];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u128; 6] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 6];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u128;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    y = const { A[4] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[4] };
    y = const { A[5] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[5] };
    let mut y = y as i64;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 8.93703 × 10^-13
// ULP = 2^-62 = 2.16840 × 10^-19
#[must_use]
pub(crate) const fn cospi_i64_12(theta: i64) -> i64 {
    const BITS: i32 = i64::BITS.cast_signed();
    const COEFFICIENTS: [(u128, i32); 7] = [
        (0x80000000000000000000000000000000, -127),
        (0x9DE9E64DD40099B2AE05581449805B44, -123),
        (0x81E0F837C9D97A6441535E981705CEFC, -121),
        (0xAAE9E05107EC6B0B044E402EA25044DA, -121),
        (0xF0F92D3CD0EE7CE16A4CA0872FB1F2BF, -122),
        (0xD329360ED2E48D97A2840EA77DE1542D, -123),
        (0xF12D07A876AECB0497BF026C0ECDD964, -125),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i64 = 1 << (BITS - 1);
    const ROUND_XX: u128 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 7] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 7];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u128; 7] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 7];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u128;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    y = const { A[4] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[4] };
    y = const { A[5] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[5] };
    y = const { A[6] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[6] };
    let mut y = y as i64;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 2.25676 × 10^-15
// ULP = 2^-62 = 2.16840 × 10^-19
#[must_use]
pub(crate) const fn cospi_i64_14(theta: i64) -> i64 {
    const BITS: i32 = i64::BITS.cast_signed();
    const COEFFICIENTS: [(u128, i32); 8] = [
        (0x80000000000000000000000000000000, -127),
        (0x9DE9E64DF2155FF89BB1259251DAE91E, -123),
        (0x81E0F840D0B9FC3953207F5C6F11165C, -121),
        (0xAAE9E3EC7D920B2D7EEB39763B04BA2A, -121),
        (0xF0FA8086265ECA1135921CFE3167927A, -122),
        (0xD3683A9CE7050142E4D0655453390DC4, -123),
        (0xFCB0CA8D9D468A290CB9D2BDB0C6F240, -125),
        (0xD2974F2325E27BF0512BA1A0CD927E6E, -127),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i64 = 1 << (BITS - 1);
    const ROUND_XX: u128 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 8] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 8];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u128; 8] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 8];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u128;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    y = const { A[4] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[4] };
    y = const { A[5] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[5] };
    y = const { A[6] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[6] };
    y = const { A[7] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[7] };
    let mut y = y as i64;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 4.48780 × 10^-18
// ULP = 2^-62 = 2.16840 × 10^-19
#[must_use]
pub(crate) const fn cospi_i64_16(theta: i64) -> i64 {
    const BITS: i32 = i64::BITS.cast_signed();
    const COEFFICIENTS: [(u128, i32); 9] = [
        (0x80000000000000000000000000000000, -127),
        (0x9DE9E64DF22EE24DC90D02763D374DC2, -123),
        (0x81E0F840DACDCEF0CC03C76006F219D8, -121),
        (0xAAE9E3F1E0495EE5ABDE02B31D2FEE8F, -121),
        (0xF0FA8340C0FB0271247DF995527FF669, -122),
        (0xD368F7EAE1C9CD9F7F6CB11D2BD6923A, -123),
        (0xFCE92B61BBC666471E98A883243A11FA, -125),
        (0xDB4AD6044D1E3D4F68DAA9DDBD3AB0C5, -127),
        (0x8B3CBF319204C34AC5ABD0A3D0DE5AD7, -129),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i64 = 1 << (BITS - 1);
    const ROUND_XX: u128 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 9] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 9];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u128; 9] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 9];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u128;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    y = const { A[4] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[4] };
    y = const { A[5] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[5] };
    y = const { A[6] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[6] };
    y = const { A[7] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[7] };
    y = const { A[8] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[8] };
    let mut y = y as i64;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}

// |error| <= 7.20675 × 10^-21
// ULP = 2^-62 = 2.16840 × 10^-19
#[must_use]
pub(crate) const fn cospi_i64_18(theta: i64) -> i64 {
    const BITS: i32 = i64::BITS.cast_signed();
    const COEFFICIENTS: [(u128, i32); 10] = [
        (0x80000000000000000000000000000000, -127),
        (0x9DE9E64DF22EF2C9F0FAF65DAE2A572D, -123),
        (0x81E0F840DAD6185EC2AFBC74D69716A0, -121),
        (0xAAE9E3F1E5FB51B41EEA5478D0BF9FD6, -121),
        (0xF0FA83448A0DA65F99EA6146295A030C, -122),
        (0xD368F94F3541423FB52E0475DEBD5C95, -123),
        (0xFCE9C4126C94A1AC4B9A0127FD375C62, -125),
        (0xDB70C9945C06C17048363E618DDEA436, -127),
        (0x904EF99BCA29D045E7AED6150BD725AC, -129),
        (0x90429BFB1BCB8E4E2857BFB8F5AF4752, -132),
    ];
    const E_OUT: i32 = -(BITS - 2);
    const E_X: i32 = -BITS;
    const E_XX: i32 = -(BITS + 2);
    const MSB: i64 = 1 << (BITS - 1);
    const ROUND_XX: u128 = !(!0 << (SHIFT_XX - 1));
    const SHIFT_XX: u32 = (E_XX - (E_X + E_X)).cast_unsigned();

    const SHIFTS: [u32; 10] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut shifts = [0; 10];

        let mut i = 0;
        while i < coefficients.len() - 1 {
            shifts[i] = (coefficients[i + 1].1 - (E_XX + coefficients[i].1)).cast_unsigned();

            i += 1;
        }

        shifts[i] = (E_OUT - coefficients[i].1).cast_unsigned();

        shifts
    };
    const A: [u128; 10] = {
        let mut coefficients = COEFFICIENTS;
        coefficients.reverse();
        let mut a = [0; 10];

        let mut i = 0;
        while i < coefficients.len() {
            let round = !(!0 << (SHIFTS[i] - 1));
            a[i] = coefficients[i].0 + round;

            i += 1;
        }

        a
    };

    let invert = (theta ^ theta << 1) & MSB;
    let x = (theta ^ invert) as u128;
    let xx = x.wrapping_mul(x).wrapping_add(ROUND_XX) >> SHIFT_XX;
    let mut y = const { A[0] >> SHIFTS[0] };
    y = const { A[1] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[1] };
    y = const { A[2] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[2] };
    y = const { A[3] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[3] };
    y = const { A[4] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[4] };
    y = const { A[5] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[5] };
    y = const { A[6] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[6] };
    y = const { A[7] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[7] };
    y = const { A[8] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[8] };
    y = const { A[9] }.wrapping_sub(xx.wrapping_mul(y)) >> const { SHIFTS[9] };
    let mut y = y as i64;

    if invert != 0 {
        y = y.wrapping_neg();
    }

    y
}
