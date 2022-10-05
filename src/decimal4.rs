use crate::multiplier2::Multiplier;
use crate::types::Fix;

struct M<const POW_OF_TEN: u32> {}

impl<const POW_OF_TEN: u32> M<POW_OF_TEN> {
    const MULTIPLIER_FROM: Multiplier = compute_multiplier_from(POW_OF_TEN);
    const MULTIPLIER_TO: Multiplier = compute_multiplier_to(POW_OF_TEN);
}

/// Compute `Multiplier` for 2^Fix::INT_NBITS / 10^p.
const fn compute_multiplier_from(pow_of_ten: u32) -> Multiplier {
    Multiplier::from_str("0.8")
        .pow(pow_of_ten)
        .shl(Fix::INT_NBITS as i32 - 3 * pow_of_ten as i32)
}

/// Compute `Multiplier` for 10^p / 2^Fix::INT_NBITS.
const fn compute_multiplier_to(pow_of_ten: u32) -> Multiplier {
    Multiplier::from_str("0.625")
        .pow(pow_of_ten)
        .shr(Fix::INT_NBITS as i32 - 4 * pow_of_ten as i32)
}

#[test]
fn test348738473847() {
    for k in 0..=20 {
        println!("{} FROM {:?}", k, compute_multiplier_from(k));
        println!("{}   TO {:?}", k, compute_multiplier_to(k));
    }
}
