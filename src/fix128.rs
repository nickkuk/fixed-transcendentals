// use core::ops::Not;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fix128 {
    lo: u64,
    hi: u64,
}

#[inline]
const fn u128_lo_hi(x: u128) -> (u64, u64) {
    (x as u64, (x >> 64) as u64)
}

#[inline]
const fn neg_u64(x: u64) -> u64 {
    1 + !x
}

#[inline]
const fn wide_add_u64(x: u64, y: u64) -> u128 {
    (x as u128) + (y as u128)
}

impl core::ops::Add for Fix128 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self
    }
}

#[test]
fn test384738473497() {
    println!("{:064b}", !1u64);
    println!("{:064b}", neg_u64(1));
    println!("{:064b}", neg_u64(0));
}
