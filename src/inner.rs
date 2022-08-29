pub(crate) type Inner = fixed::types::I64F64;

/// Constant version of `Inner::from_num`.
pub(crate) const fn from_i64(num: i64) -> Inner {
    Inner::from_bits((num as i128) << Inner::FRAC_NBITS)
}

/// Constant version of `Inner::from_num`.
pub(crate) const fn from_u32(num: u32) -> Inner {
    from_i64(num as i64)
}

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serdeize", derive(serde::Serialize, serde::Deserialize))]
pub struct Fixed(pub(crate) Inner);

impl Fixed {
    pub const MIN: Self = Self(Inner::MIN);
    pub const MAX: Self = Self(Inner::MAX);
    pub const ZERO: Self = Self(Inner::ZERO);
    pub const ONE: Self = Self(Inner::ONE);
    pub const fn from_bits(bits: i128) -> Self {
        Self(Inner::from_bits(bits))
    }
    pub const fn to_bits(self) -> i128 {
        self.0.to_bits()
    }
    pub const fn from_i64(num: i64) -> Self {
        Self(from_i64(num))
    }
    pub const fn unwrapped_from_str(s: &str) -> Self {
        Self(Inner::unwrapped_from_str(s))
    }
}

impl core::fmt::Debug for Fixed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{:?}", self.0)
    }
}

impl core::fmt::Display for Fixed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
