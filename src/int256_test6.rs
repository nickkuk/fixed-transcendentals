fn low_64(a: u128) -> u128 {
    a & ((1 << 64) - 1)
}

fn high_64(a: u128) -> u128 {
    a >> 64
}

pub struct Double128 {
    high: u128,
    low: u128,
}

impl Double128 {
    fn left_shift_64(scaled_value: u128) -> Self {
        Self {
            high: scaled_value >> 64,
            low: scaled_value << 64,
        }
    }

    fn add(self, b: Self) -> Self {
        let (low, overflow) = self.low.overflowing_add(b.low);
        let carry = overflow as u128; // 1 if true, 0 if false.
        let high = self.high.wrapping_add(b.high).wrapping_add(carry as u128);
        Double128 { high, low }
    }

    pub fn product_of(a: u128, b: u128) -> Self {
        // Split a and b into hi and lo 64-bit parts
        let (a_low, a_high) = (low_64(a), high_64(a));
        let (b_low, b_high) = (low_64(b), high_64(b));
        // a = (a_low + a_high << 64); b = (b_low + b_high << 64);
        // ergo a*b = (a_low + a_high << 64)(b_low + b_high << 64)
        //          = a_low * b_low
        //          + a_low * b_high << 64
        //          + a_high << 64 * b_low
        //          + a_high << 64 * b_high << 64
        // assuming:
        //        f = a_low * b_low
        //        o = a_low * b_high
        //        i = a_high * b_low
        //        l = a_high * b_high
        // then:
        //      a*b = (o+i) << 64 + f + l << 128
        let (f, o, i, l) = (
            a_low * b_low,
            a_low * b_high,
            a_high * b_low,
            a_high * b_high,
        );
        let fl = Self { high: l, low: f };
        let i = Self::left_shift_64(i);
        let o = Self::left_shift_64(o);
        fl.add(i).add(o)
    }
}
