#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct U256(pub [u128; 2]);

impl U256 {
    #[inline]
    pub const fn from_words(hi: u128, lo: u128) -> Self {
        #[cfg(target_endian = "little")]
        {
            U256([lo, hi])
        }
        #[cfg(target_endian = "big")]
        {
            U256([hi, lo])
        }
    }
}

pub fn umulddi3(a: &u128, b: &u128) -> U256 {
    const BITS_IN_DWORD_2: u32 = 64;
    const LOWER_MASK: u128 = u128::MAX >> BITS_IN_DWORD_2;

    let mut low = (a & LOWER_MASK) * (b & LOWER_MASK);
    let mut t = low >> BITS_IN_DWORD_2;
    low &= LOWER_MASK;
    t += (a >> BITS_IN_DWORD_2) * (b & LOWER_MASK);
    low += (t & LOWER_MASK) << BITS_IN_DWORD_2;
    let mut high = t >> BITS_IN_DWORD_2;
    t = low >> BITS_IN_DWORD_2;
    low &= LOWER_MASK;
    t += (b >> BITS_IN_DWORD_2) * (a & LOWER_MASK);
    low += (t & LOWER_MASK) << BITS_IN_DWORD_2;
    high += t >> BITS_IN_DWORD_2;
    high += (a >> BITS_IN_DWORD_2) * (b >> BITS_IN_DWORD_2);

    U256::from_words(high, low)
}
