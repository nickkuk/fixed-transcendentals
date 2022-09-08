#[inline]
fn u128_lo_hi(u: u128) -> (u64, u64) {
    (u as u64, (u >> 64) as u64)
}

#[inline]
fn mul_u64_u64(a: u64, b: u64) -> u128 {
    (a as u128).wrapping_mul(b as u128)
}

pub fn wide_mul_u128_a(lhs: u128, rhs: u128) -> [u128; 2] {
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
    [ans0, ans128]
}

pub fn wide_mul_u128_b(lhs: u128, rhs: u128) -> [u128; 2] {
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

    // 0 <= col64c <= 2^64 - 1
    // 0 <= col128a <= 2^64 - 2
    let (col64c, col128a) = u128_lo_hi(col64b);

    // 0 <= col64d <= 2^128 - 2^64
    let col64d = (col64c as u128).wrapping_add(ll_rh);

    // 0 <= col64 <= 2^64 - 1
    // 0 <= col128b <= 2^64 - 1
    let (col64, col128b) = u128_lo_hi(col64d);

    // Since both col0 and col64 fit in 64 bits, ans0 sum will never overflow.
    let ans0 = (col0 as u128) | ((col64 as u128) << 64);
    // Since lhs * rhs fits in 256 bits, ans128 sum will never overflow.
    let ans128 = lh_rh
        .wrapping_add(col128a as u128)
        .wrapping_add(col128b as u128);
    [ans0, ans128]
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn wide_mul_u128_c(lhs: u128, rhs: u128) -> [u128; 2] {
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

        // 0 <= col64c <= 2^64 - 1
        // 0 <= col128a <= 2^64 - 2
        let (col64c, col128a) = u128_lo_hi(col64b);

        // 0 <= col64d <= 2^128 - 2^64
        let col64d = (col64c as u128).wrapping_add(ll_rh);

        // 0 <= col64e <= 2^64 - 1
        // 0 <= col128b <= 2^64 - 1
        let (col64e, col128b) = u128_lo_hi(col64d);

        // 0 <= ans0 <= 2^128 - 1
        let ans0 = (col0 as u128) | ((col64e as u128) << 64);

        // 0 <= ans128 <= 2^128 - 2
        let ans128 = (col128a as u128)
            .wrapping_add(col128b as u128)
            .wrapping_add(lh_rh);

        println!("   col0 = {:0128b}", col0);
        println!(" col64a = {:0128b}", col64a);
        println!(" col64b = {:0128b}", col64b);
        println!(" col64c = {:0128b}", col64c);
        println!("col128a = {:0128b}", col128a);
        println!(" col64d = {:0128b}", col64d);
        println!(" col64e = {:0128b}", col64e);
        println!("col128b = {:0128b}", col128b);
        println!("   ans0 = {:0128b}", ans0);
        println!(" ans128 = {:0128b}", ans128);

        // 0 <= ans <= 2^256 - 2^128 - 1
        [ans0, ans128]
    }

    #[test]
    fn test3438473847389478() {
        let _ = wide_mul_u128_c(u128::MAX, u128::MAX);
    }
}
