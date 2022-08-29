use fixed_sqrt::FixedSqrt;

impl crate::Fixed {
    pub fn checked_sqrt(self) -> Option<Self> {
        if self.0.is_negative() {
            None
        } else {
            Some(Self(self.0.sqrt()))
        }
    }
}
