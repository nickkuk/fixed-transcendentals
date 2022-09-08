use crate::{
    helpers::i128_shl,
    types::{from_i64, Fix},
};

struct C<const POW_OF_TEN: u32> {}

impl<const POW_OF_TEN: u32> C<POW_OF_TEN> {
    const FACTOR_FROM: Fix = from_i64(2_i64.pow(Fix::INT_NBITS - POW_OF_TEN))
        .saturating_div(from_i64(5_i64.pow(POW_OF_TEN)));
    const FACTOR_TO: Fix = from_i64(5_i64.pow(POW_OF_TEN))
        .saturating_div(from_i64(2_i64.pow(Fix::INT_NBITS - POW_OF_TEN)));
}

const fn compute_factor_from1(pow_of_ten: u32) -> Fix {
    type Ext = fixed::types::I1F127;
    if pow_of_ten == 0 {
        return Fix::MAX;
    }
    let v: Ext = Ext::unwrapped_from_str("0.8");
    let mut x: Ext = v;
    let mut i = 0;
    while i + 1 < pow_of_ten {
        x = x.unwrapped_mul(v);
        i += 1;
    }
    let shift =
        Fix::INT_NBITS as i32 - 3 * pow_of_ten as i32 - (Ext::FRAC_NBITS - Fix::FRAC_NBITS) as i32;
    Fix::from_bits(i128_shl(x.to_bits(), shift))
}

const fn compute_factor_from2(pow_of_ten: u32) -> Fix {
    from_i64(2_i64.saturating_pow(Fix::INT_NBITS - pow_of_ten))
        .saturating_div(from_i64(5_i64.pow(pow_of_ten)))
}

#[test]
fn test384738473479() {
    for k in 0..=25 {
        dbg!(compute_factor_from1(k));
        dbg!(compute_factor_from2(k));
    }
}

#[test]
fn test38473847() {
    // let x = crate::Fixed::unwrapped_from_str("100.0");

    // dbg!(Fix::from_bits(100_i128 * 2_i128.pow(64)));

    // dbg!(crate::Fixed::saturating_from_decimal::<12>(10_i128.pow(10)));
    // dbg!(crate::Fixed::saturating_from_decimal::<12>(
    //     2 * 10_i128.pow(10)
    // ));
    // dbg!(crate::Fixed::saturating_from_decimal::<12>(
    //     10 * 10_i128.pow(10)
    // ));
    // dbg!(crate::Fixed::saturating_from_decimal::<12>(
    //     100 * 10_i128.pow(10)
    // ));
    // dbg!(crate::Fixed::saturating_from_decimal::<12>(
    //     1000 * 10_i128.pow(10)
    // ));

    // dbg!(C::<1>::FACTOR_FROM);
    // dbg!(C::<2>::FACTOR_FROM);
    // dbg!(C::<9>::FACTOR_FROM);
    // dbg!(C::<12>::FACTOR_FROM);
    // dbg!(C::<18>::FACTOR_FROM);

    // dbg!(Fix::unwrapped_from_str("1844674407370955161.6"));

    // type Ext = fixed::types::I1F127;
    // dbg!(Ext::MAX);

    type Ext = fixed::types::I40F88;
    let f: Ext = Ext::unwrapped_from_str("18446744.073709551616");
    let x: Fix = Fix::from_bits(10_i128.pow(12));
    dbg!(x.saturating_mul_add(f, Fix::ZERO));
}

/*
18.446744073709551616
18.446744073709551616

18446744.073709551616

18446744073.70955161599999999996
18446744073.709551616

184467440737095516.15999999999999999997
184467440737095516.16

1844674407370955161.6
1844674407370955161.4
*/

#[test]
fn test83748374() {
    // dbg!(C::<1>::FACTOR_FROM);
    dbg!(fixed::types::I62F66::unwrapped_from_str(
        "1844674407370955161.6"
    ));
}

/*

1844674407370955161.6
1844674407370955161.4

*/
