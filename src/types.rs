pub(crate) type Fix = fixed::types::I64F64;

/// Constant version of `Fix::from_num`.
pub(crate) const fn from_i64(num: i64) -> Fix {
    Fix::from_bits((num as i128) << Fix::FRAC_NBITS)
}

/// Constant version of `Fix::from_num`.
pub(crate) const fn from_u32(num: u32) -> Fix {
    from_i64(num as i64)
}

// #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(feature = "serdeize", derive(serde::Serialize, serde::Deserialize))]
// #[repr(transparent)]
pub struct Fixed(pub(crate) Fix);

impl Fixed {
    pub const MIN: Self = Self(Fix::MIN);
    pub const MAX: Self = Self(Fix::MAX);
    pub const ZERO: Self = Self(Fix::ZERO);
    pub const ONE: Self = Self(Fix::ONE);
    pub const fn from_bits(bits: i128) -> Self {
        Self(Fix::from_bits(bits))
    }
    pub const fn to_bits(self) -> i128 {
        self.0.to_bits()
    }
    pub const fn from_i64(num: i64) -> Self {
        Self(from_i64(num))
    }
    pub const fn unwrapped_from_str(s: &str) -> Self {
        Self(Fix::unwrapped_from_str(s))
    }
}

// impl core::fmt::Debug for Fixed {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
//         write!(f, "{:?}", self.0)
//     }
// }

// impl core::fmt::Display for Fixed {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
//         write!(f, "{}", self.0)
//     }
// }
