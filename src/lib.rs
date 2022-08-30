#![cfg_attr(not(test), no_std)]

mod arith;
mod decimal;
mod exp;
mod helpers;
mod inner;
mod ln;
mod sqrt;

#[cfg(feature = "substrate")]
mod substrate;

#[cfg(feature = "substrate")]
extern crate alloc;

pub use inner::Fixed;
