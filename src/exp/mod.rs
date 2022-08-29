mod rational;

impl crate::Fixed {
    pub fn checked_exp(self) -> Option<Self> {
        rational::checked_exp(self.0).map(Self)
    }
    pub fn saturating_exp(self) -> Self {
        self.checked_exp().unwrap_or(Self::MAX)
    }
}
