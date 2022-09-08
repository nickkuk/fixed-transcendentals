#[derive(Clone, Copy)]
pub struct U256 {
    pub lo: u128,
    pub hi: u128,
}

#[derive(Clone, Copy)]
pub struct I256 {
    pub lo: u128,
    pub hi: i128,
}

fn u128_lo_hi(u: u128) -> (u128, u128) {
    (u & !(!0 << 64), u >> 64)
}

fn i128_lo_hi(i: i128) -> (i128, i128) {
    (i & !(!0 << 64), i >> 64)
}

pub fn wide_mul_u128(lhs: u128, rhs: u128) -> U256 {
    let (ll, lh) = u128_lo_hi(lhs);
    let (rl, rh) = u128_lo_hi(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1
    let ll_rl = ll.wrapping_mul(rl);
    // 0 <= lh_rl <= 2^128 - 2^65 + 1; lh_rl unit is 2^64
    let lh_rl = lh.wrapping_mul(rl);
    // 0 <= ll_rh <= 2^128 - 2^65 + 1; ll_rh unit is 2^64
    let ll_rh = ll.wrapping_mul(rh);
    // 0 <= lh_rh <= 2^128 - 2^65 + 1; lh_rh unit is 2^128
    let lh_rh = lh.wrapping_mul(rh);

    // 0 <= col0 <= 2^64 - 1
    // 0 <= col64a <= 2^64 - 2
    let (col0, col64a) = u128_lo_hi(ll_rl);

    // 0 <= col64b <= 2^128 - 2^64 - 1
    let col64b = col64a.wrapping_add(lh_rl);

    // 0 <= col64c <= 2^128 - 1
    // 0 <= col192 <= 1
    let (col64c, col192) = col64b.overflowing_add(ll_rh);
    let col192 = if col192 { 1u128 } else { 0u128 };

    // 0 <= col64 <= 2^64 - 1
    // 0 <= col128 <= 2^64 - 1
    let (col64, col128) = u128_lo_hi(col64c);

    // Since both col0 and col64 fit in 64 bits, ans0 sum will never overflow.
    let ans0 = col0.wrapping_add(col64 << 64);
    // Since lhs * rhs fits in 256 bits, ans128 sum will never overflow.
    let ans128 = lh_rh.wrapping_add(col128).wrapping_add(col192 << 64);
    U256 {
        lo: ans0,
        hi: ans128,
    }
}

fn wide_mul_i128(lhs: i128, rhs: i128) -> I256 {
    let (ll, lh) = i128_lo_hi(lhs);
    let (rl, rh) = i128_lo_hi(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1; must be unsigned to hold all range!
    let ll_rl = (ll as u128).wrapping_mul(rl as u128);
    // -2^127 + 2^63 <= lh_rl <= 2^127 - 2^64 - 2^63 + 1; lh_rl unit is 2^64
    let lh_rl = lh.wrapping_mul(rl);
    // -2^127 + 2^63 <= ll_rh <= 2^127 - 2^64 - 2^63 + 1; ll_rh unit is 2^64
    let ll_rh = ll.wrapping_mul(rh);
    // -2^126 + 2^63 <= lh_rh <= 2^126; lh_rh unit is 2^128
    let lh_rh = lh.wrapping_mul(rh);

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
    let ans0 = col0.wrapping_add((col64 as u128) << 64);
    // Since lhs * rhs fits in 256 bits, ans128 sum will never overflow.
    let ans128 = lh_rh.wrapping_add(col128).wrapping_add(col192 << 64);
    I256 {
        lo: ans0,
        hi: ans128,
    }
}
