#[derive(Clone, Copy)]
pub struct I256 {
    pub lo: u128,
    pub hi: i128,
}

#[inline]
fn u128_lo_hi(u: u128) -> (u64, u64) {
    (u as u64, (u >> 64) as u64)
}

#[inline]
fn i128_lo_hi(i: i128) -> (u64, i64) {
    (i as u64, (i >> 64) as i64)
}

#[inline]
fn mul_u64_u64(a: u64, b: u64) -> u128 {
    (a as u128).wrapping_mul(b as u128)
}

#[inline]
fn mul_i64_i64(a: i64, b: i64) -> i128 {
    (a as i128).wrapping_mul(b as i128)
}

#[inline]
fn mul_u64_i64(a: u64, b: i64) -> i128 {
    let a = a as i64;
    // if a has become negative, we need to add 2^64 * b to the answer
    let correction = if a.is_negative() {
        (b as i128) << 64
    } else {
        0
    };
    (a as i128).wrapping_mul(b as i128).wrapping_add(correction)
}

pub fn wide_mul_i128_a(lhs: i128, rhs: i128) -> I256 {
    let (ll, lh) = i128_lo_hi(lhs);
    let (rl, rh) = i128_lo_hi(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1; must be unsigned to hold all range!
    let ll_rl = mul_u64_u64(ll, rl);
    // -2^127 + 2^63 <= lh_rl <= 2^127 - 2^64 - 2^63 + 1; lh_rl unit is 2^64
    let lh_rl = mul_u64_i64(rl, lh);
    // -2^127 + 2^63 <= ll_rh <= 2^127 - 2^64 - 2^63 + 1; ll_rh unit is 2^64
    let ll_rh = mul_u64_i64(ll, rh);
    // -2^126 + 2^63 <= lh_rh <= 2^126; lh_rh unit is 2^128
    let lh_rh = mul_i64_i64(lh, rh);

    // 0 <= col0 <= 2^64 - 1
    // 0 <= col64a <= 2^64 - 2
    let (col0, col64a) = u128_lo_hi(ll_rl);

    // -2^127 + 2^63 <= col64b <= 2^127 - 2^63 - 1
    let col64b = (col64a as i128).wrapping_add(lh_rl);

    // -2^127 <= col64c <= 2^127 - 1
    // -1 <= col192 <= 1
    let (col64c, col192) = col64b.overflowing_add(ll_rh);
    let col192 = if col192 {
        // col64b and ll_rh have the same sign, and col64c has the opposite sign.
        if col64b < 0 {
            -1i128
        } else {
            1i128
        }
    } else {
        0i128
    };

    // 0 <= col64 <= 2^64 - 1
    // -2^63 <= col128 <= 2^63 - 1
    let (col64, col128) = i128_lo_hi(col64c);

    // Since both col0 and col64 fit in 64 bits, ans0 sum will never overflow.
    let ans0 = (col0 as u128) | ((col64 as u128) << 64);
    // Since lhs * rhs fits in 256 bits, ans128 sum will never overflow.
    let ans128 = lh_rh
        .wrapping_add(col128 as i128)
        .wrapping_add(col192 << 64);
    I256 {
        lo: ans0,
        hi: ans128,
    }
}

pub fn wide_mul_i128_b(lhs: i128, rhs: i128) -> I256 {
    let (ll, lh) = i128_lo_hi(lhs);
    let (rl, rh) = i128_lo_hi(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1; must be unsigned to hold all range!
    let ll_rl = mul_u64_u64(ll, rl);
    // -2^127 + 2^63 <= lh_rl <= 2^127 - 2^64 - 2^63 + 1; lh_rl unit is 2^64
    let lh_rl = mul_u64_i64(rl, lh);
    // -2^127 + 2^63 <= ll_rh <= 2^127 - 2^64 - 2^63 + 1; ll_rh unit is 2^64
    let ll_rh = mul_u64_i64(ll, rh);
    // -2^126 + 2^63 <= lh_rh <= 2^126; lh_rh unit is 2^128
    let lh_rh = mul_i64_i64(lh, rh);

    // 0 <= col0 <= 2^64 - 1
    // 0 <= col64a <= 2^64 - 2
    let (col0, col64a) = u128_lo_hi(ll_rl);

    // -2^127 + 2^63 <= col64b <= 2^127 - 2^63 - 1
    let col64b = (col64a as i128).wrapping_add(lh_rl);

    // 0 <= col64c <= 2^64 - 1
    // -2^63 <= col128a <= 2^63 - 1
    let (col64c, col128a) = i128_lo_hi(col64b);

    // -2^127 + 2^63 <= col64d <= 2^127 - 2^63
    let col64d = (col64c as i128).wrapping_add(ll_rh);

    // 0 <= col64 <= 2^64 - 1
    // -2^63 <= col128b <= 2^63 - 1
    let (col64, col128b) = i128_lo_hi(col64d);

    // Since both col0 and col64 fit in 64 bits, ans0 sum will never overflow.
    let ans0 = (col0 as u128) | ((col64 as u128) << 64);
    // Since lhs * rhs fits in 256 bits, ans128 sum will never overflow.
    let ans128 = lh_rh
        .wrapping_add(col128a as i128)
        .wrapping_add(col128b as i128);
    I256 {
        lo: ans0,
        hi: ans128,
    }
}

#[test]
fn test837483749384739() {
    println!("{:0128b}", (1i128 << 64) - 1);
    println!("{:0128b}", (-1_i128 << 127) + (1_i128 << 63));
    println!("{:0128b}", ((-1_i128 << 127) + (1_i128 << 63)) >> 64);
    println!("{:0128b}", (1_u128 << 127) - (1_u128 << 63));
    println!("{:0128b}", ((1_u128 << 127) - (1_u128 << 63)) >> 64);
}
