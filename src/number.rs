use std::fmt::{self, Debug};

#[derive(Clone, PartialEq, Copy)]
enum N {
    Pos(u64),
    Neg(i64),
    Float(f64),
}

#[derive(Clone, PartialEq)]
pub struct Number {
    n: N,
}

impl Number {
    #[inline]
    pub fn is_i64(&self) -> bool {
        match self.n {
            N::Pos(v) => v <= i64::max_value() as u64,
            N::Neg(_) => true,
            N::Float(_) => false,
        }
    }

    #[inline]
    pub fn is_u64(&self) -> bool {
        match self.n {
            N::Pos(_) => true,
            N::Neg(_) | N::Float(_) => false,
        }
    }

    #[inline]
    pub fn is_f64(&self) -> bool {
        match self.n {
            N::Float(_) => true,
            N::Pos(_) | N::Neg(_) => false,
        }
    }

    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::Pos(n) => {
                if n <= i64::max_value() as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            N::Neg(n) => Some(n),
            N::Float(_) => None,
        }
    }

    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::Pos(n) => Some(n),
            N::Neg(_) | N::Float(_) => None,
        }
    }

    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::Pos(n) => Some(n as f64),
            N::Neg(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
    }

    #[inline]
    pub fn from_f64(f: f64) -> Number {
        Number { n: N::Float(f) }
    }

    #[inline]
    pub fn from_i64(f: i64) -> Number {
        Number { n: N::Neg(f) }
    }

    #[inline]
    pub fn from_u64(f: u64) -> Number {
        Number { n: N::Pos(f) }
    }
}

impl Debug for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = formatter.debug_tuple("Number");
        match self.n {
            N::Pos(i) => {
                debug.field(&i);
            }
            N::Neg(i) => {
                debug.field(&i);
            }
            N::Float(f) => {
                debug.field(&f);
            }
        }
        debug.finish()
    }
}

// pub trait ToNumber {
//     fn to_number(self) -> Number;
// }
