use crate::multiplier::Multiplier;
use crate::types::Fix;

impl crate::Fixed {
    /// Checked cast from a fixed-point number with the power of ten denominator.
    pub const fn checked_from_decimal<const POW_OF_TEN: u32>(dec: i128) -> Option<Self> {
        match M::<POW_OF_TEN>::MULTIPLIER_FROM.run_checked(Fix::from_bits(dec)) {
            Some(x) => Some(Self(x)),
            None => None,
        }
    }
    /// Saturating cast from a fixed-point number with the power of ten denominator.
    pub const fn saturating_from_decimal<const POW_OF_TEN: u32>(dec: i128) -> Self {
        Self(M::<POW_OF_TEN>::MULTIPLIER_FROM.run_saturating(Fix::from_bits(dec)))
    }
    /// Checked cast to a fixed-point number with the power of ten denominator.
    pub const fn checked_to_decimal<const POW_OF_TEN: u32>(self) -> Option<i128> {
        match M::<POW_OF_TEN>::MULTIPLIER_TO.run_checked(self.0) {
            Some(x) => Some(x.to_bits()),
            None => None,
        }
    }
    /// Saturating cast to a fixed-point number with the power of ten denominator.
    pub const fn saturating_to_decimal<const POW_OF_TEN: u32>(self) -> i128 {
        M::<POW_OF_TEN>::MULTIPLIER_TO
            .run_saturating(self.0)
            .to_bits()
    }
}

struct M<const POW_OF_TEN: u32> {}

impl<const POW_OF_TEN: u32> M<POW_OF_TEN> {
    const MULTIPLIER_FROM: Multiplier = compute_multiplier_from(POW_OF_TEN);
    const MULTIPLIER_TO: Multiplier = compute_multiplier_to(POW_OF_TEN);
}

/// Compute `Multiplier` for 2^Fix::INT_NBITS / 10^p.
const fn compute_multiplier_from(pow_of_ten: u32) -> Multiplier {
    Multiplier::from_str("1.6")
        .pow(pow_of_ten)
        .shl(Fix::INT_NBITS as i32 - 4 * pow_of_ten as i32)
}

/// Compute `Multiplier` for 10^p / 2^Fix::INT_NBITS.
const fn compute_multiplier_to(pow_of_ten: u32) -> Multiplier {
    Multiplier::from_str("0.625")
        .pow(pow_of_ten)
        .shr(Fix::INT_NBITS as i32 - 4 * pow_of_ten as i32)
}
