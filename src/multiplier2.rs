/// Extended-precision fixed-point number for intermediate computations.
type Ext = fixed::types::U0F128;

const HALF: Ext = Ext::unwrapped_from_str("0.5");

/// A type that encapsulates the concept of "multiplying by the constant".
/// It can be evaluated at program compilation time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Multiplier(Inner);

impl Multiplier {
    pub const ZERO: Self = Self(Inner::Zero);
    pub const ONE: Self = Self(Inner::SignExp(SignExp::ONE));
    pub const fn from_str(str: &str) -> Self {
        let inner = Inner::General {
            sign_exp: SignExp::ONE,
            factor: Ext::unwrapped_from_str(str),
        };
        Self(inner.normalize())
    }
    pub const fn neg(self) -> Self {
        let inner = match self.0 {
            Inner::Zero => Inner::Zero,
            Inner::SignExp(sign_exp) => Inner::SignExp(sign_exp.neg()),
            Inner::General { sign_exp, factor } => Inner::General {
                sign_exp: sign_exp.neg(),
                factor,
            },
        };
        Self(inner)
    }
    pub const fn mul(self, other: Self) -> Self {
        let inner = match (self.0, other.0) {
            (Inner::Zero, _) => Inner::Zero,
            (_, Inner::Zero) => Inner::Zero,
            (Inner::SignExp(sign_exp1), Inner::SignExp(sign_exp2)) => {
                Inner::SignExp(sign_exp1.mul(sign_exp2))
            }
            (
                Inner::SignExp(sign_exp1),
                Inner::General {
                    sign_exp: sign_exp2,
                    factor,
                },
            ) => Inner::General {
                sign_exp: sign_exp1.mul(sign_exp2),
                factor,
            },
            (
                Inner::General {
                    sign_exp: sign_exp1,
                    factor,
                },
                Inner::SignExp(sign_exp2),
            ) => Inner::General {
                sign_exp: sign_exp1.mul(sign_exp2),
                factor,
            },
            (
                Inner::General {
                    sign_exp: sign_exp1,
                    factor: factor1,
                },
                Inner::General {
                    sign_exp: sign_exp2,
                    factor: factor2,
                },
            ) => Inner::General {
                sign_exp: sign_exp1.mul(sign_exp2),
                factor: factor1.wrapping_mul(factor2),
            }
            .normalize(),
        };
        Self(inner)
    }
    pub const fn pow(self, exp: u32) -> Self {
        if exp == 0 {
            return Self::ONE;
        }
        let mut r = self;
        let mut i = exp;
        while i > 1 {
            r = r.mul(self);
            i -= 1;
        }
        r
    }
    pub const fn shl(self, shift: i32) -> Self {
        let inner = match self.0 {
            Inner::Zero => Inner::Zero,
            Inner::SignExp(sign_exp) => Inner::SignExp(sign_exp.shl(shift)),
            Inner::General { sign_exp, factor } => Inner::General {
                sign_exp: sign_exp.shl(shift),
                factor,
            },
        };
        Self(inner)
    }
    pub const fn shr(self, shift: i32) -> Self {
        self.shl(-shift)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Inner {
    Zero,
    SignExp(SignExp),
    General {
        sign_exp: SignExp,
        /// 0.5 < factor < 1.
        factor: Ext,
    },
}

impl Inner {
    const fn normalize(self) -> Self {
        match self {
            Inner::General {
                mut sign_exp,
                mut factor,
            } => {
                if factor.is_zero() {
                    return Self::Zero;
                }
                while factor.to_bits() < HALF.to_bits() {
                    factor = factor.wrapping_shl(1);
                    sign_exp = sign_exp.shr(1);
                }
                if factor.to_bits() != HALF.to_bits() {
                    Self::General { sign_exp, factor }
                } else {
                    Self::SignExp(sign_exp.shr(1))
                }
            }
            _ => self,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SignExp {
    sign: bool,
    exp: i32,
}

impl SignExp {
    const ONE: Self = Self {
        sign: false,
        exp: 0,
    };
    const fn neg(self) -> Self {
        Self {
            sign: !self.sign,
            exp: self.exp,
        }
    }
    const fn mul(self, other: Self) -> Self {
        Self {
            sign: self.sign ^ other.sign,
            exp: self.exp + other.exp,
        }
    }
    const fn shl(self, shift: i32) -> Self {
        Self {
            sign: self.sign,
            exp: self.exp + shift,
        }
    }
    const fn shr(self, shift: i32) -> Self {
        self.shl(-shift)
    }
}
