use crate::{
    helpers::i128_shr,
    types::{from_u32, Fix},
};

/// Extended precision fixed-point numbers for internal computations.
type Ext = fixed::types::I8F120;

const LOWER_BOUND: Fix = Fix::LN_2.saturating_mul(from_u32(Fix::FRAC_NBITS).saturating_neg());
const UPPER_BOUND: Fix = Fix::LN_2.saturating_mul(from_u32(Fix::INT_NBITS - 1));

const HALF: Fix = Fix::unwrapped_from_str("0.5");
const TWO: Ext = Ext::unwrapped_from_str("2.0");
const P1: Ext = Ext::unwrapped_from_str("0.166666666666666019037");
const P2: Ext = Ext::unwrapped_from_str("-0.00277777777770155933842");
const P3: Ext = Ext::unwrapped_from_str("0.0000661375632143793436117");
const P4: Ext = Ext::unwrapped_from_str("-0.00000165339022054652515390");
const P5: Ext = Ext::unwrapped_from_str("0.0000000413813679705723846039");

pub fn checked_exp(x: Fix) -> Option<Fix> {
    if x <= LOWER_BOUND {
        return Some(Fix::ZERO);
    }
    if x >= UPPER_BOUND {
        return None;
    }
    let (k, r): (Fix, Ext) = reduce_arg(x);
    let exp_r: Ext = special_exp(r)?;
    let exp_x: Fix = recover(k, exp_r);
    Some(exp_x)
}

/// Returns pair (k, r) where x = k * ln(2) + r and |r| <= ln(2) / 2.
fn reduce_arg(x: Fix) -> (Fix, Ext) {
    // k is integer from [-Fix::FRAC_NBITS, Fix::INT_NBITS - 1]
    let k: Fix = (Fix::LOG2_E * x + HALF).ceil();
    let r: Ext = Ext::from_num(x) - Ext::LN_2 * Ext::from_num(k);
    (k, r)
}

/// Exponential function for r with |r| <= ln(2) / 2.
fn special_exp(r: Ext) -> Option<Ext> {
    let rr: Ext = r * r;
    let c: Ext = r - rr * (P1 + rr * (P2 + rr * (P3 + rr * (P4 + rr * P5))));
    let exp_r: Ext = Ext::ONE + (r * c.checked_div(TWO - c)? + r);
    Some(exp_r)
}

fn recover(k: Fix, exp_r: Ext) -> Fix {
    let shift: i32 = (Ext::FRAC_NBITS - Fix::FRAC_NBITS) as i32 - k.to_num::<i32>();
    Fix::from_bits(i128_shr(exp_r.to_bits(), shift))
}
