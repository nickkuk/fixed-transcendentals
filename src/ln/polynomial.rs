use crate::helpers::i128_shr;

type Fix = fixed::types::I64F64;
/// `FixP` is more accurate version of `Fix` for internal computations.
type FixP = fixed::types::I8F120;

const TWO: FixP = FixP::unwrapped_from_str("2.0");
const LG1: FixP = FixP::unwrapped_from_str("0.6666666666666735130");
const LG2: FixP = FixP::unwrapped_from_str("0.3999999999940941908");
const LG3: FixP = FixP::unwrapped_from_str("0.2857142874366239149");
const LG4: FixP = FixP::unwrapped_from_str("0.2222219843214978396");
const LG5: FixP = FixP::unwrapped_from_str("0.1818357216161805012");
const LG6: FixP = FixP::unwrapped_from_str("0.1531383769920937332");
const LG7: FixP = FixP::unwrapped_from_str("0.1479819860511658591");

pub fn checked_ln(x: Fix) -> Option<Fix> {
    let (k, f1): (i32, FixP) = reduce_arg(x)?;
    let ln_f1: FixP = special_ln(f1)?;
    let ln_x: FixP = recover(k, ln_f1);
    Some(Fix::from_num(ln_x))
}

/// Returns pair (k, f1) where x = 2^k * f1
/// and f1 = 1 + f is from [sqrt(2)/2; sqrt(2)).
fn reduce_arg(x: Fix) -> Option<(i32, FixP)> {
    // l is integer from [-Fix::FRAC_NBITS, Fix::INT_NBITS - 2]
    let l = x.checked_int_log2()?;
    let shift: i32 = l - (FixP::FRAC_NBITS - Fix::FRAC_NBITS) as i32;
    let f1: FixP = FixP::from_bits(i128_shr(x.to_bits(), shift));
    if f1 < FixP::SQRT_2 {
        Some((l, f1))
    } else {
        Some((l + 1, f1 >> 1))
    }
}

/// Natural logarithm for f1 from [sqrt(2)/2; sqrt(2)).
fn special_ln(f1: FixP) -> Option<FixP> {
    let f: FixP = f1 - FixP::ONE;
    let s: FixP = f.checked_div(TWO + f)?;
    let z: FixP = s * s;
    let w: FixP = z * z;
    let t1: FixP = w * (LG2 + w * (LG4 + w * LG6));
    let t2: FixP = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    let r: FixP = t2 + t1;
    Some(f - s * (f - r))
}

fn recover(k: i32, ln_f1: FixP) -> FixP {
    FixP::LN_2 * FixP::from_num(k) + ln_f1
}
