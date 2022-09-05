use crate::types::{from_i64, Fix};

impl crate::Fixed {
    /// Checked cast from a fixed-point number with the power of ten denominator.
    pub const fn checked_from_decimal<const POW_OF_TEN: u32>(dec: i128) -> Option<Self> {
        Self::from_bits(dec).checked_mul(Self(C::<POW_OF_TEN>::FACTOR_FROM))
    }
    /// Saturating cast from a fixed-point number with the power of ten denominator.
    pub const fn saturating_from_decimal<const POW_OF_TEN: u32>(dec: i128) -> Self {
        Self::from_bits(dec).saturating_mul(Self(C::<POW_OF_TEN>::FACTOR_FROM))
    }
    /// Checked cast to a fixed-point number with the power of ten denominator.
    pub const fn checked_to_decimal<const POW_OF_TEN: u32>(self) -> Option<i128> {
        match self.0.checked_mul(C::<POW_OF_TEN>::FACTOR_TO) {
            Some(x) => Some(x.to_bits()),
            None => None,
        }
    }
    /// Saturating cast to a fixed-point number with the power of ten denominator.
    pub const fn saturating_to_decimal<const POW_OF_TEN: u32>(self) -> i128 {
        self.0.saturating_mul(C::<POW_OF_TEN>::FACTOR_TO).to_bits()
    }
}

struct C<const POW_OF_TEN: u32> {}

impl<const POW_OF_TEN: u32> C<POW_OF_TEN> {
    const FACTOR_FROM: Fix = from_i64(2_i64.pow(Fix::INT_NBITS - POW_OF_TEN))
        .saturating_div(from_i64(5_i64.pow(POW_OF_TEN)));
    const FACTOR_TO: Fix = from_i64(5_i64.pow(POW_OF_TEN))
        .saturating_div(from_i64(2_i64.pow(Fix::INT_NBITS - POW_OF_TEN)));
}
