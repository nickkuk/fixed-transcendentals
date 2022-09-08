use fixed::FixedI128;

/// Extended-precision fixed-point number for intermediate computations.
type Ext = fixed::types::I2F126;

const HALF: Ext = Ext::unwrapped_from_str("0.5");

/// A type that encapsulates the concept of "multiplying by the constant".
/// It can be evaluated at program compilation time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Multiplier {
    /// Either factor is zero or 0.5 < |factor| < 2.
    factor: Ext,
    /// Left shift for positive value, right shift for negative.
    shift: i32,
}

impl Multiplier {
    pub const ONE: Multiplier = Multiplier {
        factor: Ext::ONE,
        shift: 0,
    };
    pub const fn from_str(str: &str) -> Self {
        Self {
            factor: Ext::unwrapped_from_str(str),
            shift: 0,
        }
        .normalize()
    }
    const fn normalize(mut self) -> Self {
        if self.factor.is_zero() {
            return Self {
                factor: Ext::ZERO,
                shift: 0,
            };
        }
        if self.factor.to_bits() == Ext::MIN.to_bits() {
            return Self {
                factor: Ext::MIN.wrapping_shr(1),
                shift: self.shift + 1,
            };
        }
        while self.factor.to_bits().wrapping_abs() <= HALF.to_bits() {
            self.factor = self.factor.wrapping_shl(1);
            self.shift -= 1;
        }
        if self.factor.to_bits().wrapping_abs() < Ext::ONE.to_bits() && self.shift > 0 {
            self.factor = self.factor.wrapping_shl(1);
            self.shift -= 1;
        } else if self.factor.to_bits().wrapping_abs() > Ext::ONE.to_bits() && self.shift < 0 {
            self.factor = self.factor.wrapping_shr(1);
            self.shift += 1;
        }
        self
    }
    pub const fn mul(self, other: Self) -> Self {
        let (r, shift) = shiftable_mul(self.factor, other.factor);
        Self {
            factor: r,
            shift: self.shift + other.shift + shift as i32,
        }
        .normalize()
    }
    pub const fn pow(self, exp: u32) -> Self {
        if exp == 0 {
            return Self::ONE;
        }
        let mut r = self;
        let mut i = exp;
        while i > 1 {
            r = r.mul(self);
            i -= 1;
        }
        r
    }
    pub const fn shl(self, shift: i32) -> Self {
        Self {
            factor: self.factor,
            shift: self.shift + shift,
        }
        .normalize()
    }
    pub const fn shr(self, shift: i32) -> Self {
        Self {
            factor: self.factor,
            shift: self.shift - shift,
        }
        .normalize()
    }
    #[inline]
    pub const fn run_checked<Frac>(self, x: FixedI128<Frac>) -> Option<FixedI128<Frac>> {
        if self.factor.is_zero() {
            return Some(FixedI128::ZERO);
        }
        if self.shift == 0 {
            if self.factor.to_bits() == -Ext::ONE.to_bits() {
                x.checked_neg()
            } else if self.factor.to_bits() == Ext::ONE.to_bits() {
                Some(x)
            } else if self.factor.to_bits().wrapping_abs() < Ext::ONE.to_bits() {
                Some(x.wrapping_mul_add(self.factor, FixedI128::ZERO))
            } else {
                x.checked_mul_add(self.factor, FixedI128::ZERO)
            }
        } else if self.shift < 0 {
            let shift = self.shift.unsigned_abs();
            if shift >= i128::BITS {
                return Some(FixedI128::ZERO);
            }
            let r = if self.factor.to_bits() == -Ext::ONE.to_bits() {
                x.wrapping_shr(shift).wrapping_neg()
            } else if self.factor.to_bits() == Ext::ONE.to_bits() {
                x.wrapping_shr(shift)
            } else {
                x.wrapping_mul_add(self.factor, FixedI128::ZERO)
                    .wrapping_shr(shift)
            };
            Some(r)
        } else {
            let shift = self.shift.unsigned_abs();
            if shift > max_shl(x.to_bits()) {
                return None;
            }
            let x = x.wrapping_shl(shift);
            if self.factor.to_bits() == -Ext::ONE.to_bits() {
                x.checked_neg()
            } else if self.factor.to_bits() == Ext::ONE.to_bits() {
                Some(x)
            } else {
                x.checked_mul_add(self.factor, FixedI128::ZERO)
            }
        }
    }
    #[inline]
    pub const fn run_saturating<Frac>(self, x: FixedI128<Frac>) -> FixedI128<Frac> {
        if self.factor.is_zero() {
            return FixedI128::ZERO;
        }
        if self.shift == 0 {
            if self.factor.to_bits() == -Ext::ONE.to_bits() {
                x.saturating_neg()
            } else if self.factor.to_bits() == Ext::ONE.to_bits() {
                x
            } else if self.factor.to_bits().wrapping_abs() < Ext::ONE.to_bits() {
                x.wrapping_mul_add(self.factor, FixedI128::ZERO)
            } else {
                x.saturating_mul_add(self.factor, FixedI128::ZERO)
            }
        } else if self.shift < 0 {
            let shift = self.shift.unsigned_abs();
            if shift >= i128::BITS {
                return FixedI128::ZERO;
            }
            if self.factor.to_bits() == -Ext::ONE.to_bits() {
                x.wrapping_shr(shift).wrapping_neg()
            } else if self.factor.to_bits() == Ext::ONE.to_bits() {
                x.wrapping_shr(shift)
            } else {
                x.wrapping_mul_add(self.factor, FixedI128::ZERO)
                    .wrapping_shr(shift)
            }
        } else {
            let shift = self.shift.unsigned_abs();
            // if shift > max_shl(x.to_bits()) {
            //     if self.factor.is_negative() == x.is_negative() {
            //         return FixedI128::MAX;
            //     } else {
            //         return FixedI128::MIN;
            //     }
            // }
            let x = x.wrapping_shl(shift);
            if self.factor.to_bits() == -Ext::ONE.to_bits() {
                x.saturating_neg()
            } else if self.factor.to_bits() == Ext::ONE.to_bits() {
                x
            } else {
                x.saturating_mul_add(self.factor, FixedI128::ZERO)
            }
        }
    }
}

/// At least one argument must be greater than `Ext::MIN`!
const fn shiftable_mul(x: Ext, y: Ext) -> (Ext, bool) {
    let (z, b) = x.overflowing_mul(y);
    if !b {
        (z, false)
    } else if z.is_negative() {
        (z.wrapping_shr(1).wrapping_sub(Ext::MIN), true)
    } else {
        ((z.wrapping_shr(1)).wrapping_add(Ext::MIN), true)
    }
}

/// Maximal possible parameter for left shift that can be done without overflow.
const fn max_shl(x: i128) -> u32 {
    if x > 0 {
        x.leading_zeros().saturating_sub(1)
    } else if x == 0 {
        u32::MAX
    } else {
        (!x).leading_zeros().saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shiftable_mul() {
        assert_eq!(
            shiftable_mul(Ext::MIN + Ext::DELTA, Ext::MIN),
            (Ext::MAX, true)
        );
        assert_eq!(
            shiftable_mul(Ext::MIN + Ext::DELTA, Ext::MIN + Ext::DELTA),
            (Ext::MAX - Ext::DELTA, true)
        );
        assert_eq!(
            shiftable_mul(Ext::MIN, Ext::MAX),
            (Ext::MIN + Ext::DELTA, true)
        );
        assert_eq!(
            shiftable_mul(Ext::MAX, Ext::MIN),
            (Ext::MIN + Ext::DELTA, true)
        );
        assert_eq!(
            shiftable_mul(Ext::MAX, Ext::MAX),
            (Ext::MAX - Ext::DELTA, true)
        );
    }
    #[test]
    fn test_possible_shl() {
        assert_eq!(max_shl(i128::MIN), 0);
        assert_eq!(max_shl(i128::MIN + 1), 0);
        assert_eq!(max_shl((i128::MIN >> 1) - 1), 0);
        assert_eq!(max_shl(i128::MIN >> 1), 1);
        assert_eq!(max_shl((i128::MIN >> 1) + 1), 1);
        assert_eq!(max_shl(-2), 126);
        assert_eq!(max_shl(-1), 127);
        assert_eq!(max_shl(1), 126);
        assert_eq!(max_shl(2), 125);
        assert_eq!(max_shl((i128::MAX >> 1) - 1), 1);
        assert_eq!(max_shl(i128::MAX >> 1), 1);
        assert_eq!(max_shl((i128::MAX >> 1) + 1), 0);
        assert_eq!(max_shl(i128::MAX - 1), 0);
        assert_eq!(max_shl(i128::MAX), 0);
    }
}
