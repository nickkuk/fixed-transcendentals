use crate::{helpers::i128_shr, inner::from_u32};

type Fix = fixed::types::I64F64;
/// `FixP` is more accurate version of `Fix` for internal computations.
type FixP = fixed::types::I8F120;

const LOWER_BOUND: Fix = Fix::LN_2.saturating_mul(from_u32(Fix::FRAC_NBITS).saturating_neg());
const UPPER_BOUND: Fix = Fix::LN_2.saturating_mul(from_u32(Fix::INT_NBITS - 1));

const TWO: FixP = FixP::unwrapped_from_str("2.0");
const P1: FixP = FixP::unwrapped_from_str("0.166666666666666019037");
const P2: FixP = FixP::unwrapped_from_str("-0.00277777777770155933842");
const P3: FixP = FixP::unwrapped_from_str("0.0000661375632143793436117");
const P4: FixP = FixP::unwrapped_from_str("-0.00000165339022054652515390");
const P5: FixP = FixP::unwrapped_from_str("0.0000000413813679705723846039");

pub fn checked_exp(x: Fix) -> Option<Fix> {
    if x <= LOWER_BOUND {
        return Some(Fix::ZERO);
    }
    if x >= UPPER_BOUND {
        return None;
    }
    // k is integer from [-Fix::FRAC_NBITS, Fix::INT_NBITS - 1]
    let k: Fix = (Fix::LOG2_E * x).round();
    let r: FixP = FixP::from_num(x) - FixP::LN_2 * FixP::from_num(k);
    let rr: FixP = r * r;
    let c: FixP = r - rr * (P1 + rr * (P2 + rr * (P3 + rr * (P4 + rr * P5))));
    let exp_r: FixP = FixP::ONE + (r * c.checked_div(TWO - c)? + r);
    let shift: i32 = (FixP::FRAC_NBITS - Fix::FRAC_NBITS) as i32 - k.to_num::<i32>();
    Some(Fix::from_bits(i128_shr(exp_r.to_bits(), shift)))
}
