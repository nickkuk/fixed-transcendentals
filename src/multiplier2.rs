/// Extended-precision fixed-point number for intermediate computations.
type Ext = fixed::types::U0F128;

const HALF: Ext = Ext::unwrapped_from_str("0.5");

/// A type that encapsulates the concept of "multiplying by the constant".
/// It can be evaluated at program compilation time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Multiplier(Inner);

impl Multiplier {
    pub const ZERO: Self = Self(Inner::Zero);
    pub const ONE: Self = Self(Inner::Shift {
        sign: false,
        shift: 0,
    });
    pub const fn from_str(str: &str) -> Self {
        let inner = Inner::General {
            sign: false,
            shift: 0,
            factor: Ext::unwrapped_from_str(str),
        };
        Self(inner.normalize())
    }
    pub const fn neg(self) -> Self {
        let inner = match self.0 {
            Inner::Zero => Inner::Zero,
            Inner::Shift { sign, shift } => Inner::Shift { sign: !sign, shift },
            Inner::General {
                sign,
                shift,
                factor,
            } => Inner::General {
                sign: !sign,
                shift,
                factor,
            },
        };
        Self(inner)
    }
}

/// Shift is left for positive value, right for negative.
/// 0.5 < factor < 1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Inner {
    Zero,
    Shift { sign: bool, shift: i32 },
    General { sign: bool, shift: i32, factor: Ext },
}

impl Inner {
    const fn normalize(self) -> Self {
        match self {
            Self::General {
                sign,
                mut shift,
                mut factor,
            } => {
                while factor.to_bits() < HALF.to_bits() {
                    factor = factor.wrapping_shl(1);
                    shift -= 1;
                }
                if factor.to_bits() != HALF.to_bits() {
                    Self::General {
                        sign,
                        shift,
                        factor,
                    }
                } else {
                    Self::Shift {
                        sign,
                        shift: shift - 1,
                    }
                }
            }
            _ => self,
        }
    }
}

#[test]
fn test384738473847() {
    dbg!(fixed::types::I0F128::MIN);
    dbg!(fixed::types::I0F128::MAX);
}
