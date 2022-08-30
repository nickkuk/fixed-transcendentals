mod polynomial;

impl crate::Fixed {
    pub fn checked_ln(self) -> Option<Self> {
        polynomial::checked_ln(self.0).map(Self)
    }
}
