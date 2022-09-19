pub fn mul_u128(x: u128, y: u128) -> u128 {
    x.wrapping_mul(y)
}

pub fn mul_i128(x: i128, y: i128) -> i128 {
    x.wrapping_mul(y)
}

#[inline]
const fn u128_lo_hi(u: u128) -> (u64, u64) {
    (u as u64, (u >> 64) as u64)
}

pub fn mul_u128b(x: u128, y: u128) -> u128 {
    let (xl, xh) = u128_lo_hi(x);
    let (yl, yh) = u128_lo_hi(y);
    let ll = (xl as u128).wrapping_mul(yl as u128);
    let (a0, c64) = u128_lo_hi(ll);
    let a64 = c64
        .wrapping_add(xl.wrapping_mul(yh))
        .wrapping_add(xh.wrapping_mul(yl));
    (a0 as u128) | ((a64 as u128) << 64)
}
