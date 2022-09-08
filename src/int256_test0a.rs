#[derive(Clone, Copy)]
pub struct U256 {
    pub lo: u128,
    pub hi: u128,
}

#[inline]
fn u128_lo_hi(u: u128) -> (u64, u64) {
    (u as u64, (u >> 64) as u64)
}

#[inline]
fn mul_u64_u64(a: u64, b: u64) -> u128 {
    (a as u128) * (b as u128)
}

pub fn wide_mul_u128(lhs: u128, rhs: u128) -> U256 {
    let (ll, lh) = u128_lo_hi(lhs);
    let (rl, rh) = u128_lo_hi(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1
    let ll_rl = mul_u64_u64(ll, rl);
    // 0 <= lh_rl <= 2^128 - 2^65 + 1; lh_rl unit is 2^64
    let lh_rl = mul_u64_u64(lh, rl);
    // 0 <= ll_rh <= 2^128 - 2^65 + 1; ll_rh unit is 2^64
    let ll_rh = mul_u64_u64(ll, rh);
    // 0 <= lh_rh <= 2^128 - 2^65 + 1; lh_rh unit is 2^128
    let lh_rh = mul_u64_u64(lh, rh);

    // 0 <= col0 <= 2^64 - 1
    // 0 <= col64a <= 2^64 - 2
    let (col0, col64a) = u128_lo_hi(ll_rl);

    // 0 <= col64b <= 2^128 - 2^64 - 1
    let col64b = (col64a as u128).wrapping_add(lh_rl);

    // 0 <= col64c <= 2^128 - 1
    // 0 <= col192 <= 1
    let (col64c, col192) = col64b.overflowing_add(ll_rh);
    let col192 = if col192 { 1u128 } else { 0u128 };

    // 0 <= col64 <= 2^64 - 1
    // 0 <= col128 <= 2^64 - 1
    let (col64, col128) = u128_lo_hi(col64c);

    // Since both col0 and col64 fit in 64 bits, ans0 sum will never overflow.
    let ans0 = (col0 as u128) | ((col64 as u128) << 64);
    // Since lhs * rhs fits in 256 bits, ans128 sum will never overflow.
    let ans128 = lh_rh
        .wrapping_add(col128 as u128)
        .wrapping_add(col192 << 64);
    U256 {
        lo: ans0,
        hi: ans128,
    }
}
