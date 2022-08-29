impl crate::Fixed {
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(r) => Some(Self(r)),
            None => None,
        }
    }
    pub const fn checked_abs(self) -> Option<Self> {
        match self.0.checked_abs() {
            Some(r) => Some(Self(r)),
            None => None,
        }
    }
    pub const fn checked_add(self, oth: Self) -> Option<Self> {
        match self.0.checked_add(oth.0) {
            Some(r) => Some(Self(r)),
            None => None,
        }
    }
    pub const fn checked_sub(self, oth: Self) -> Option<Self> {
        match self.0.checked_sub(oth.0) {
            Some(r) => Some(Self(r)),
            None => None,
        }
    }
    pub const fn checked_mul(self, oth: Self) -> Option<Self> {
        match self.0.checked_mul(oth.0) {
            Some(r) => Some(Self(r)),
            None => None,
        }
    }
    pub const fn checked_div(self, oth: Self) -> Option<Self> {
        match self.0.checked_div(oth.0) {
            Some(r) => Some(Self(r)),
            None => None,
        }
    }
    pub const fn saturating_neg(self) -> Self {
        Self(self.0.saturating_neg())
    }
    pub const fn saturating_abs(self) -> Self {
        Self(self.0.saturating_abs())
    }
    pub const fn saturating_add(self, oth: Self) -> Self {
        Self(self.0.saturating_add(oth.0))
    }
    pub const fn saturating_sub(self, oth: Self) -> Self {
        Self(self.0.saturating_sub(oth.0))
    }
    pub const fn saturating_mul(self, oth: Self) -> Self {
        Self(self.0.saturating_mul(oth.0))
    }
    pub const fn saturating_div(self, oth: Self) -> Self {
        Self(self.0.saturating_div(oth.0))
    }
}
