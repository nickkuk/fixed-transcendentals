#![allow(dead_code, non_camel_case_types)]

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct u256 {
    lo: u128,
    hi: u128,
}

impl u256 {
    #[inline]
    pub const fn shr_trunc(self, sh: u32) -> Option<u128> {
        if sh >= 128 {
            Some(self.hi >> (sh - 128))
        } else if sh == 0 {
            if self.hi == 0 {
                Some(self.lo)
            } else {
                None
            }
        } else {
            let lo = self.lo >> sh;
            let hi = self.hi << (128 - sh);
            if self.hi >> sh == 0 {
                Some(lo | hi)
            } else {
                None
            }
        }
    }
    #[inline]
    pub const fn checked_shl_trunc(self, shift: i32) -> Option<u128> {
        None
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct i256 {
    lo: u128,
    hi: i128,
}

impl i256 {
    #[inline]
    pub const fn shr_trunc(self, sh: u32) -> Option<i128> {
        if sh >= 128 {
            Some(self.hi >> (sh - 128))
        } else if sh == 0 {
            let ans = self.lo as i128;
            if self.hi == ans >> 127 {
                Some(ans)
            } else {
                None
            }
        } else {
            let lo = (self.lo >> sh) as i128;
            let hi = self.hi << (128 - sh);
            let ans = lo | hi;
            if self.hi >> sh == ans >> 127 {
                Some(ans)
            } else {
                None
            }
        }
    }
}

#[inline]
const fn u128_lo_hi(u: u128) -> (u64, u64) {
    (u as u64, (u >> 64) as u64)
}

#[inline]
const fn i128_lo_hi(i: i128) -> (u64, i64) {
    (i as u64, (i >> 64) as i64)
}

#[inline]
const fn wide_mul_uu64(a: u64, b: u64) -> u128 {
    (a as u128).wrapping_mul(b as u128)
}

#[inline]
const fn wide_mul_ii64(a: i64, b: i64) -> i128 {
    (a as i128).wrapping_mul(b as i128)
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
    (a as i128).wrapping_mul(b as i128).wrapping_add(correction)
}

#[inline]
pub const fn wide_mul_uu128(lhs: u128, rhs: u128) -> u256 {
    let (ll, lh) = u128_lo_hi(lhs);
    let (rl, rh) = u128_lo_hi(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1
    let ll_rl = wide_mul_uu64(ll, rl);
    // 0 <= lh_rl <= 2^128 - 2^65 + 1; lh_rl unit is 2^64
    let lh_rl = wide_mul_uu64(lh, rl);
    // 0 <= ll_rh <= 2^128 - 2^65 + 1; ll_rh unit is 2^64
    let ll_rh = wide_mul_uu64(ll, rh);
    // 0 <= lh_rh <= 2^128 - 2^65 + 1; lh_rh unit is 2^128
    let lh_rh = wide_mul_uu64(lh, rh);

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
    u256 {
        lo: ans0,
        hi: ans128,
    }
}

#[inline]
pub const fn wide_mul_ii128(lhs: i128, rhs: i128) -> i256 {
    let (ll, lh) = i128_lo_hi(lhs);
    let (rl, rh) = i128_lo_hi(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1; must be unsigned to hold all range!
    let ll_rl = wide_mul_uu64(ll, rl);
    // -2^127 + 2^63 <= lh_rl <= 2^127 - 2^64 - 2^63 + 1; lh_rl unit is 2^64
    let lh_rl = wide_mul_ui64(rl, lh);
    // -2^127 + 2^63 <= ll_rh <= 2^127 - 2^64 - 2^63 + 1; ll_rh unit is 2^64
    let ll_rh = wide_mul_ui64(ll, rh);
    // -2^126 + 2^63 <= lh_rh <= 2^126; lh_rh unit is 2^128
    let lh_rh = wide_mul_ii64(lh, rh);

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
    i256 {
        lo: ans0,
        hi: ans128,
    }
}

#[inline]
pub const fn wide_mul_ui128(lhs: u128, rhs: i128) -> i256 {
    let (ll, lh) = u128_lo_hi(lhs);
    let (rl, rh) = i128_lo_hi(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1; must be unsigned to hold all range!
    let ll_rl = wide_mul_uu64(ll, rl);
    // 0 <= lh_rl <= 2^128 - 2^65 + 1; lh_rl unit is 2^64
    let lh_rl = wide_mul_uu64(rl, lh);
    // -2^127 + 2^63 <= ll_rh <= 2^127 - 2^64 - 2^63 + 1; ll_rh unit is 2^64
    let ll_rh = wide_mul_ui64(ll, rh);
    // -2^126 + 2^63 <= lh_rh <= 2^127 - 2^64 - 2^63 + 1; lh_rh unit is 2^128
    let lh_rh = wide_mul_ui64(lh, rh);

    // 0 <= col0 <= 2^64 - 1
    // 0 <= col64a <= 2^64 - 2
    let (col0, col64a) = u128_lo_hi(ll_rl);

    // 0 <= col64b <= 2^128 - 2^64 - 1
    let col64b = (col64a as u128).wrapping_add(lh_rl);

    // 0 <= col64c <= 2^64 - 1
    // 0 <= col128a <= 2^64 - 2
    let (col64c, col128a) = u128_lo_hi(col64b);

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
    i256 {
        lo: ans0,
        hi: ans128,
    }
}
