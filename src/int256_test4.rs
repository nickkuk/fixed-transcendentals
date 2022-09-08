#[inline]
const fn wide_mul_uu64(a: u64, b: u64) -> u128 {
    (a as u128) * (b as u128)
}

#[inline]
const fn wide_mul_ii64(a: i64, b: i64) -> i128 {
    (a as i128) * (b as i128)
}

#[inline]
const fn wide_mul_ui64(a: u64, b: i64) -> i128 {
    let a = a as i64;
    // if a has become negative, we need to add 2^64 * b to the answer
    let correction = if a.is_negative() {
        (b as i128) << 64
    } else {
        0
    };
    (a as i128) * (b as i128) + correction
}

pub fn wide_mul_uu128(x: u128, y: u128) -> [u128; 2] {
    const BITS: u32 = u64::BITS;
    const MASK: u128 = u128::MAX >> BITS;

    let a = x >> BITS;
    let b = x & MASK;
    let c = y >> BITS;
    let d = y & MASK;
    let ac = a * c;
    let ad = a * d;
    let bc = b * c;
    let bd = b * d;

    let mut lo = bd;
    let mut mid = lo >> BITS;
    lo &= MASK;
    mid += ad;
    lo += (mid & MASK) << BITS;
    let mut hi = mid >> BITS;
    mid = lo >> BITS;
    lo &= MASK;
    mid += bc;
    lo += (mid & MASK) << BITS;
    hi += mid >> BITS;
    hi += ac;

    [hi, lo]
}

#[test]
fn test3874384734() {
    const MASK: u128 = (1_u128 << 64) - 1;
    println!("{:0128b}", MASK);
    println!("{:0128b}", MASK * MASK);
    println!("{:0128b}", (MASK * MASK) >> 64);
    println!("{:0128b}", MASK - 1);
    println!("{:0128b}", MASK * MASK + MASK - 1);
}

/*
00000000000000000000000000000000000000000000000000000000000000001111111111111111111111111111111111111111111111111111111111111111
11111111111111111111111111111111111111111111111111111111111111100000000000000000000000000000000000000000000000000000000000000001
00000000000000000000000000000000000000000000000000000000000000001111111111111111111111111111111111111111111111111111111111111110
11111111111111111111111111111111111111111111111111111111111111101111111111111111111111111111111111111111111111111111111111111111
*/
