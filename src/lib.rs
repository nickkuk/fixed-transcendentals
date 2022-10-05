#![cfg_attr(not(test), no_std)]

mod arith;
mod decimal;
// mod exp;
mod helpers;
// mod ln;
// mod sqrt;
mod types;

// #[cfg(feature = "substrate")]
// mod substrate;

// #[cfg(feature = "substrate")]
// extern crate alloc;

pub use types::Fixed;

// pub mod int256;
// pub mod int256_test;
// pub mod int256_test2;
// pub mod decimal2;

// mod decimal3;
// mod multiplier;

// impl Fixed {
//     pub fn saturating_from_decimal_12_old(dec: i128) -> Fixed {
//         Fixed::saturating_from_decimal_old::<12>(dec)
//     }
//     pub fn saturating_from_decimal_12(dec: i128) -> Fixed {
//         Fixed::saturating_from_decimal::<12>(dec)
//     }
// }

// pub type Fix = fixed::types::I64F64;
// type FixP = fixed::types::I40F88;
// type FixPP = fixed::types::I2F126;

// const FACTOR: Fix = Fix::unwrapped_from_str("18446744.073709551616");
// const FACTOR_P: FixP = FixP::unwrapped_from_str("18446744.073709551616");
// const FACTOR_PP: FixPP = FixPP::unwrapped_from_str("1.099511627776");

// pub fn from_decimal1(dec: i128) -> Fix {
//     Fix::from_bits(dec).saturating_mul(FACTOR)
// }

// pub fn from_decimal2(dec: i128) -> Fix {
//     Fix::from_bits(dec).saturating_mul_add(FACTOR_P, Fix::ZERO)
// }

// pub fn from_decimal3(dec: i128) -> Fix {
//     Fix::from_bits(dec << 24).saturating_mul_add(FACTOR_PP, Fix::ZERO)
// }

#[test]
fn test384783478347() {
    for k in 0..=18 {
        // let dec = 10_i128.pow(k);
        println!("{k}");
        // println!("{}", Fixed::saturating_from_decimal::<4>(dec));
        // println!("{}", Fixed::saturating_from_decimal2::<4>(dec));
        // let fix = Fixed::from_i64(dec as i64);
        // println!("{}", fix.saturating_to_decimal::<18>());
        // println!("{}", fix.saturating_to_decimal2::<18>());
        println!();
    }
}

// #[test]
// fn test34873847() {
//     let e = Fix::E;
//     println!("{}", e * e * e * e);
//     println!("{}", (e * e) * (e * e));
// }

/*
54.598150033144239078110261202860878402790737038614068725826593958
54.5981500331442390744
54.59815003314423907416

*/

// type FixU = fixed::types::U64F64;
// type FixI = fixed::types::I64F64;

// pub fn mulu(x: FixU, y: FixU) -> FixU {
//     x.wrapping_mul(y)
// }

// pub fn muli(x: FixI, y: FixI) -> FixI {
//     x.wrapping_mul(y)
// }

// type FixU = fixed::types::U64F64;
// type ExtU = fixed::types::U128F0;
// type FixI = fixed::types::I64F64;
// type ExtI = fixed::types::I128F0;

// pub fn test_checked_mul_u(x: FixU) -> Option<FixU> {
//     x.checked_mul_add(ExtU::ONE << 10, FixU::ZERO)
// }

// pub fn test_saturating_mul_u(x: FixU) -> FixU {
//     x.saturating_mul_add(ExtU::ONE << 10, FixU::ZERO)
// }

// pub fn test_checked_mul_i(x: FixI) -> Option<FixI> {
//     x.checked_mul_add(ExtI::ONE << 10, FixI::ZERO)
// }

// pub fn test_saturating_mul_i(x: FixI) -> FixI {
//     x.saturating_mul_add(ExtI::ONE << 10, FixI::ZERO)
// }

// pub mod int256_test0;
// pub mod int256_test0a;
// pub mod int256_test3;
// pub mod int256_test4;
// pub mod int256_test5;
// pub mod int256_test6;
// pub mod int256_test7;
// pub mod int256_test8;
// pub mod fix64;
pub mod multiplier2;
mod ui256;
// pub mod fix128;
pub mod decimal4;

// pub use ui256::i256;

// pub fn test0(u: i256) -> Option<i128> {
//     u.shr_trunc(0)
// }

// #[test]
// fn test3487384734() {
//     dbg!(u64::MAX as i128);
//     dbg!(-1_i64 as u128 == u128::MAX);
// }
