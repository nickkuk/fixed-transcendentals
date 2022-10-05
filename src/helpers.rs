pub const fn i128_shl(num: i128, shift: i32) -> i128 {
    if shift == 0 {
        num
    } else if shift > 0 {
        num << shift
    } else {
        num >> (-shift)
    }
}

pub const fn u128_checked_shl2(x: u128, shift: i32) -> u128 {
    // if shift <= -128 {
    //     0
    // } else if shift < 0 {

    // }
    // if shift == 0 {
    //     x
    // } else if shift < 0 {
    //     if shift <= -128 {

    //     }
    // }
    match shift {
        i32::MIN..=-128 => 0,
        -127..=-1 => x >> (-shift),
        _ => x << shift,
    }
}

pub const fn u128_checked_shl(x: u128, shift: u32) -> Option<u128> {
    if x <= shl_u128_max(shift) {
        Some(x << shift)
    } else {
        None
    }
}

pub const fn shl_u128_max(shift: u32) -> u128 {
    if shift < 128 {
        u128::MAX >> shift
    } else {
        0
    }
}

pub const fn shl_i128_max(shift: u32) -> i128 {
    if shift < 128 {
        i128::MAX >> shift
    } else {
        0
    }
}

pub const fn shl_i128_min(shift: u32) -> i128 {
    if shift < 128 {
        i128::MIN >> shift
    } else {
        0
    }
}

#[test]
fn test39478348347() {
    for k in 0..=128 {
        // println!("{:03} {:032x}", k, shl_u128_max(k));
        println!("{:03} {:032x}", k, shl_i128_max(k));
    }
}
