#[must_use]
pub(crate) const fn costau_2(theta: u32) -> i32 {
    const A_0: u64 = 0x1000000000000000;
    const E_0: i32 = -60;
    const E_OUT: i32 = -30;

    let mut x = theta & 0x3FFFFFFF;
    let invert_x = theta << 1 & 0x80000000 != 0;
    let invert_y = (theta ^ theta << 1) & 0x80000000 != 0;

    if invert_x {
        x = 0x40000000u32.wrapping_sub(x);
    }

    let x = x as u64;
    let xx = x * x;
    let mut y = const { A_0 + !(!0 << (E_OUT - E_0 - 1)) } - xx;
    y >>= const { E_OUT - E_0 };
    let mut y = y as i32;

    if invert_y {
        y = y.wrapping_neg();
    }

    y
}

#[must_use]
pub(crate) const fn costau_4(theta: u32) -> i32 {
    const A_0: u64 = 0x8000000000000000;
    const A_2: u64 = 0x9CAC4C9693EB74C2;
    const A_4: u64 = 0xE56264B49F5BA611;
    const E_0: i32 = -63;
    const E_2: i32 = -59;
    const E_4: i32 = -58;
    const E_X: i32 = -32;
    const E_XX: i32 = -34;
    const E_OUT: i32 = -30;

    let mut x = theta & 0x3FFFFFFF;
    let invert_x = theta << 1 & 0x80000000 != 0;
    let invert_y = (theta ^ theta << 1) & 0x80000000 != 0;

    if invert_x {
        x = 0x40000000u32.wrapping_sub(x);
    }

    let x = x as u64;
    let mut xx = x * x;
    xx += const { !(!0 << (E_XX - (E_X + E_X) - 1)) };
    xx >>= const { E_XX - (E_X + E_X) };
    let mut y = const { (A_4 + !(!0 << (E_2 - (E_XX + E_4) - 1))) >> (E_2 - (E_XX + E_4)) };
    y = const { A_2 + !(!0 << (E_0 - (E_XX + E_2) - 1)) } - xx * y;
    y >>= const { E_0 - (E_XX + E_2) };
    y = const { A_0 + !(!0 << (E_OUT - E_0 - 1)) } - xx * y;
    y >>= const { E_OUT - E_0 };
    let mut y = y as i32;

    if invert_y {
        y = y.wrapping_neg();
    }

    y
}

#[must_use]
pub(crate) const fn costau_6(theta: u32) -> i32 {
    const A_0: u64 = 0x8000000000000000;
    const A_2: u64 = 0x9DE408E97E84C3F6;
    const A_4: u64 = 0x81571F58D0AA1ED5;
    const A_6: u64 = 0x9C6FBB2D6970EFBA;
    const E_0: i32 = -63;
    const E_2: i32 = -59;
    const E_4: i32 = -57;
    const E_6: i32 = -57;
    const E_X: i32 = -32;
    const E_XX: i32 = -34;
    const E_OUT: i32 = -30;

    let mut x = theta & 0x3FFFFFFF;
    let invert_x = theta << 1 & 0x80000000 != 0;
    let invert_y = (theta ^ theta << 1) & 0x80000000 != 0;

    if invert_x {
        x = 0x40000000u32.wrapping_sub(x);
    }

    let x = x as u64;
    let mut xx = x * x;
    xx += const { !(!0 << (E_XX - (E_X + E_X) - 1)) };
    xx >>= const { E_XX - (E_X + E_X) };
    let mut y = const { (A_6 + !(!0 << (E_4 - (E_XX + E_6) - 1))) >> (E_4 - (E_XX + E_6)) };
    y = const { A_4 + !(!0 << (E_2 - (E_XX + E_4) - 1)) } - xx * y;
    y >>= const { E_2 - (E_XX + E_4) };
    y = const { A_2 + !(!0 << (E_0 - (E_XX + E_2) - 1)) } - xx * y;
    y >>= const { E_0 - (E_XX + E_2) };
    y = const { A_0 + !(!0 << (E_OUT - E_0 - 1)) } - xx * y;
    y >>= const { E_OUT - E_0 };
    let mut y = y as i32;

    if invert_y {
        y = y.wrapping_neg();
    }

    y
}

#[must_use]
pub(crate) const fn costau_8(theta: u32) -> i32 {
    const A_0: u64 = 0x8000000000000000;
    const A_2: u64 = 0x9DE9D6CE594BC7A1;
    const A_4: u64 = 0x81DEAA9595D76019;
    const A_6: u64 = 0xAA7CBC1DE16B5099;
    const A_8: u64 = 0xE0F8CB5ADCE6EF85;
    const E_0: i32 = -63;
    const E_2: i32 = -59;
    const E_4: i32 = -57;
    const E_6: i32 = -57;
    const E_8: i32 = -58;
    const E_X: i32 = -32;
    const E_XX: i32 = -34;
    const E_OUT: i32 = -30;

    let mut x = theta & 0x3FFFFFFF;
    let invert_x = theta << 1 & 0x80000000 != 0;
    let invert_y = (theta ^ theta << 1) & 0x80000000 != 0;

    if invert_x {
        x = 0x40000000u32.wrapping_sub(x);
    }

    let x = x as u64;
    let mut xx = x * x;
    xx += const { !(!0 << (E_XX - (E_X + E_X) - 1)) };
    xx >>= const { E_XX - (E_X + E_X) };
    let mut y = const { (A_8 + !(!0 << (E_6 - (E_XX + E_8) - 1))) >> (E_6 - (E_XX + E_8)) };
    y = const { A_6 + !(!0 << (E_4 - (E_XX + E_6) - 1)) } - xx * y;
    y >>= const { E_4 - (E_XX + E_6) };
    y = const { A_4 + !(!0 << (E_2 - (E_XX + E_4) - 1)) } - xx * y;
    y >>= const { E_2 - (E_XX + E_4) };
    y = const { A_2 + !(!0 << (E_0 - (E_XX + E_2) - 1)) } - xx * y;
    y >>= const { E_0 - (E_XX + E_2) };
    y = const { A_0 + !(!0 << (E_OUT - E_0 - 1)) } - xx * y;
    y >>= const { E_OUT - E_0 };
    let mut y = y as i32;

    if invert_y {
        y = y.wrapping_neg();
    }

    y
}

#[must_use]
pub(crate) const fn costau_10(theta: u32) -> i32 {
    const A_0: u64 = 0x8000000000000000;
    const A_2: u64 = 0x9DE9E633E9CD50C3;
    const A_4: u64 = 0x81E0F292F44046C9;
    const A_6: u64 = 0xAAE847A7763AB7C2;
    const A_8: u64 = 0xF0944D347C4F3D4D;
    const A_10: u64 = 0xC7DBDBF95FF7AC58;
    const E_0: i32 = -63;
    const E_2: i32 = -59;
    const E_4: i32 = -57;
    const E_6: i32 = -57;
    const E_8: i32 = -58;
    const E_10: i32 = -59;
    const E_X: i32 = -32;
    const E_XX: i32 = -34;
    const E_OUT: i32 = -30;

    let mut x = theta & 0x3FFFFFFF;
    let invert_x = theta << 1 & 0x80000000 != 0;
    let invert_y = (theta ^ theta << 1) & 0x80000000 != 0;

    if invert_x {
        x = 0x40000000u32.wrapping_sub(x);
    }

    let x = x as u64;
    let mut xx = x * x;
    xx += const { !(!0 << (E_XX - (E_X + E_X) - 1)) };
    xx >>= const { E_XX - (E_X + E_X) };
    let mut y = const { (A_10 + !(!0 << (E_8 - (E_XX + E_10) - 1))) >> (E_8 - (E_XX + E_10)) };
    y = const { A_8 + !(!0 << (E_6 - (E_XX + E_8) - 1)) } - xx * y;
    y >>= const { E_6 - (E_XX + E_8) };
    y = const { A_6 + !(!0 << (E_4 - (E_XX + E_6) - 1)) } - xx * y;
    y >>= const { E_4 - (E_XX + E_6) };
    y = const { A_4 + !(!0 << (E_2 - (E_XX + E_4) - 1)) } - xx * y;
    y >>= const { E_2 - (E_XX + E_4) };
    y = const { A_2 + !(!0 << (E_0 - (E_XX + E_2) - 1)) } - xx * y;
    y >>= const { E_0 - (E_XX + E_2) };
    y = const { A_0 + !(!0 << (E_OUT - E_0 - 1)) } - xx * y;
    y >>= const { E_OUT - E_0 };
    let mut y = y as i32;

    if invert_y {
        y = y.wrapping_neg();
    }

    y
}
