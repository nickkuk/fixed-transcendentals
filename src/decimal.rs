use crate::inner::{from_i64, Inner};

impl crate::Fixed {
    /// Saturating cast from a fixed-point number with the power of ten denominator.
    pub const fn saturating_from_decimal<const POW_OF_TEN: u32>(dec: i128) -> Self {
        Self::from_bits(dec).saturating_mul(Self(C::<POW_OF_TEN>::FACTOR))
    }
}

struct C<const POW_OF_TEN: u32> {}

impl<const POW_OF_TEN: u32> C<POW_OF_TEN> {
    const FACTOR: Inner = from_i64(2_i64.pow(Inner::INT_NBITS - POW_OF_TEN))
        .saturating_div(from_i64(5_i64.pow(POW_OF_TEN)));
}
