// pub fn wide_mul_uu64(a: u64, b: u64) -> u128 {
//     (a as u128).wrapping_mul(b as u128)
// }

// pub fn mul_uu64(a: u64, b: u64) -> u64 {
//     ((a as u128).wrapping_mul(b as u128) >> 64) as u64
// }

#[derive(Clone, Copy)]
pub struct FixU32x32(u64);

impl FixU32x32 {
    pub fn multiply(self, other: Self) -> Self {
        let w = (self.0 as u128).wrapping_mul(other.0 as u128);
        Self((w >> 32) as u64)
    }
}

#[derive(Clone, Copy)]
pub struct FixI32x32(i64);

impl FixI32x32 {
    pub fn multiply(self, other: Self) -> Self {
        let w = (self.0 as i128).wrapping_mul(other.0 as i128);
        Self((w >> 32) as i64)
    }
}

#[derive(Clone, Copy)]
pub struct FixU32x32b {
    lo: u32,
    hi: u32,
}

impl FixU32x32b {
    pub fn multiply(self, other: Self) -> Self {
        let ll = (self.lo as u64).wrapping_mul(other.lo as u64);
        let lh = (self.lo as u64).wrapping_mul(other.hi as u64);
        let hl = (self.hi as u64).wrapping_mul(other.lo as u64);
        let hh = (self.hi as u64).wrapping_mul(other.hi as u64);
        self
    }
}
