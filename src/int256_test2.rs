fn i128_lo_hi(i: i128) -> (i128, i128) {
    (i & !(!0 << 64), i >> 64)
}

fn i128_to_words(i: i128) -> (i64, u64) {
    ((i >> 64) as i64, i as u64)
}

/* pub fn wide_mul_i128(lhs: i128, rhs: i128) -> (u128, i128, i128, i128) {
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

    (ll_rl, lh_rl, ll_rh, lh_rh)
} */

pub fn wide_mul_i128_x(lhs: i128, rhs: i128) -> (u128, i128, i128, i128) {
    let (lh, ll) = i128_to_words(lhs);
    let (rh, rl) = i128_to_words(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1; must be unsigned to hold all range!
    let ll_rl = (ll as u128).wrapping_mul(rl as u128);
    // -2^127 + 2^63 <= lh_rl <= 2^127 - 2^64 - 2^63 + 1; lh_rl unit is 2^64
    let lh_rl = (lh as i128).wrapping_mul(rl as i128);
    // -2^127 + 2^63 <= ll_rh <= 2^127 - 2^64 - 2^63 + 1; ll_rh unit is 2^64
    let ll_rh = (ll as i128).wrapping_mul(rh as i128);
    // -2^126 + 2^63 <= lh_rh <= 2^126; lh_rh unit is 2^128
    let lh_rh = (lh as i128).wrapping_mul(rh as i128);

    (ll_rl, lh_rl, ll_rh, lh_rh)
}

#[test]
fn test23847834734() {
    let n: i128 = !(!0 << 64);
    println!("{:x}", n);
}

#[test]
fn test3847834738() {
    dbg!(i128_to_words(-1));
}
