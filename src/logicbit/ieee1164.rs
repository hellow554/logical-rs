#![allow(clippy::just_underscores_and_digits)]

use super::{Ieee1164Value, Resolve};
use std::convert::TryFrom;
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

macro_rules! expand_op_ieee1164 {
    ($func_name:ident, $trait_name:ident, $fn_name:ident) => {
        expand_op!($func_name, $trait_name, $fn_name, Ieee1164, Ieee1164, Ieee1164);
    };
}

/// Represents an [Ieee1164](https://en.wikipedia.org/wiki/IEEE_1164) value. For a better usage,
/// there are associated constants, which all start with an underscore, e.g. [`Ieee1164::_U`].
///
/// The three binary logical operators are defined on this struct so you can use them.
///
/// # Examples
///
/// ```rust
/// # use logical::Ieee1164;
/// let a = Ieee1164::_1;
/// let b = Ieee1164::_0;
/// assert_eq!(Ieee1164::_0, a & b);
/// assert_eq!(Ieee1164::_1, a | b);
/// assert_eq!(Ieee1164::_1, a ^ b);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ieee1164 {
    /// Uninitialized is the default value. It represents an unknown or invalid value
    Uninitialized,
    /// Represents a strong [`Ieee1164Value`]
    Strong(Ieee1164Value),
    /// Represents a weak [`Ieee1164Value`]
    Weak(Ieee1164Value),
    /// Represents high-impedance
    HighImpedance,
    /// Represents a don't-care
    DontCare,
}

impl Ieee1164 {
    /// Uninitialized is the default for an `Ieee1164`. It represents an unknown or invalid value
    pub const _U: Ieee1164 = Ieee1164::Uninitialized;
    /// Represents a conflicted value between two strong values
    pub const _X: Ieee1164 = Ieee1164::Strong(Ieee1164Value::Unknown);
    /// Represents a strong 1
    pub const _1: Ieee1164 = Ieee1164::Strong(Ieee1164Value::One);
    /// Represents a strong 0
    pub const _0: Ieee1164 = Ieee1164::Strong(Ieee1164Value::Zero);
    /// Represents a conflicted value between two weak values
    pub const _W: Ieee1164 = Ieee1164::Weak(Ieee1164Value::Unknown);
    /// Represents a weak 1
    pub const _H: Ieee1164 = Ieee1164::Weak(Ieee1164Value::One);
    /// Represents a weak 1
    pub const _L: Ieee1164 = Ieee1164::Weak(Ieee1164Value::Zero);
    /// Represents high-impedance
    pub const _Z: Ieee1164 = Ieee1164::HighImpedance;
    /// Represents a don't-care
    pub const _D: Ieee1164 = Ieee1164::DontCare;
}

impl Default for Ieee1164 {
    fn default() -> Self {
        Ieee1164::Uninitialized
    }
}

impl TryFrom<char> for Ieee1164 {
    type Error = ();

    fn try_from(c: char) -> Result<Self, ()> {
        Ok(match c.to_ascii_lowercase() {
            'u' => Ieee1164::_U,
            'x' => Ieee1164::_X,
            '0' => Ieee1164::_0,
            '1' => Ieee1164::_1,
            'z' => Ieee1164::_Z,
            'w' => Ieee1164::_W,
            'l' => Ieee1164::_L,
            'h' => Ieee1164::_H,
            '*' | '-' | 'd' => Ieee1164::_D,
            _ => return Err(()),
        })
    }
}

impl From<Ieee1164> for char {
    fn from(i: Ieee1164) -> Self {
        match i {
            Ieee1164::_U => 'U',
            Ieee1164::_X => 'X',
            Ieee1164::_0 => '0',
            Ieee1164::_1 => '1',
            Ieee1164::_Z => 'Z',
            Ieee1164::_W => 'W',
            Ieee1164::_L => 'L',
            Ieee1164::_H => 'H',
            Ieee1164::_D => '-',
        }
    }
}

// this will make the tables shorter
const _U: Ieee1164 = Ieee1164::_U;
const _X: Ieee1164 = Ieee1164::_X;
const _0: Ieee1164 = Ieee1164::_0;
const _1: Ieee1164 = Ieee1164::_1;
const _Z: Ieee1164 = Ieee1164::_Z;
const _W: Ieee1164 = Ieee1164::_W;
const _L: Ieee1164 = Ieee1164::_L;
const _H: Ieee1164 = Ieee1164::_H;
const _D: Ieee1164 = Ieee1164::_D;

#[allow(clippy::trivially_copy_pass_by_ref)]
fn and(a: &Ieee1164, b: &Ieee1164) -> Ieee1164 {
    const TTABLE: [[Ieee1164; 9]; 9] = [
        //U   X   0   1   Z   W   L   H   -
        [_U, _U, _0, _U, _U, _U, _0, _U, _U], // U
        [_U, _X, _0, _X, _X, _X, _0, _X, _X], // X
        [_0, _0, _0, _0, _0, _0, _0, _0, _0], // 0
        [_U, _X, _0, _1, _X, _X, _0, _1, _X], // 1
        [_U, _X, _0, _X, _X, _X, _0, _X, _X], // Z
        [_U, _X, _0, _X, _X, _X, _0, _X, _X], // W
        [_0, _0, _0, _0, _0, _0, _0, _0, _0], // L
        [_U, _X, _0, _1, _X, _X, _0, _1, _X], // H
        [_U, _X, _0, _X, _X, _X, _0, _X, _X], // -
    ];

    TTABLE[a.to_index()][b.to_index()]
}
expand_op_ieee1164!(and, BitAnd, bitand);

#[allow(clippy::trivially_copy_pass_by_ref)]
fn or(a: &Ieee1164, b: &Ieee1164) -> Ieee1164 {
    const TTABLE: [[Ieee1164; 9]; 9] = [
        //U   X   0   1   Z   W   L   H   -
        [_U, _U, _U, _1, _U, _U, _U, _1, _U], // U
        [_U, _X, _X, _1, _X, _X, _X, _1, _X], // X
        [_U, _X, _0, _1, _X, _X, _0, _1, _X], // 0
        [_1, _1, _1, _1, _1, _1, _1, _1, _1], // 1
        [_U, _X, _X, _1, _X, _X, _X, _1, _X], // Z
        [_U, _X, _X, _1, _X, _X, _X, _1, _X], // W
        [_U, _X, _0, _1, _X, _X, _0, _1, _X], // L
        [_1, _1, _1, _1, _1, _1, _1, _1, _1], // H
        [_U, _X, _X, _1, _X, _X, _X, _1, _X], // -
    ];

    TTABLE[a.to_index()][b.to_index()]
}
expand_op_ieee1164!(or, BitOr, bitor);

#[allow(clippy::trivially_copy_pass_by_ref)]
fn xor(a: &Ieee1164, b: &Ieee1164) -> Ieee1164 {
    const TTABLE: [[Ieee1164; 9]; 9] = [
        //U   X   0   1   Z   W   L   H   -
        [_U, _U, _U, _U, _U, _U, _U, _U, _U], // U
        [_U, _X, _X, _X, _X, _X, _X, _X, _X], // X
        [_U, _X, _0, _1, _X, _X, _0, _1, _X], // 0
        [_U, _X, _1, _0, _X, _X, _1, _0, _X], // 1
        [_U, _X, _X, _X, _X, _X, _X, _X, _X], // Z
        [_U, _X, _X, _X, _X, _X, _X, _X, _X], // W
        [_U, _X, _0, _1, _X, _X, _0, _1, _X], // L
        [_U, _X, _1, _0, _X, _X, _1, _0, _X], // H
        [_U, _X, _X, _X, _X, _X, _X, _X, _X], // -
    ];

    TTABLE[a.to_index()][b.to_index()]
}
expand_op_ieee1164!(xor, BitXor, bitxor);

fn not(i: Ieee1164) -> Ieee1164 {
    match i {
        _U => _U,
        _L | _0 => _1,
        _H | _1 => _0,
        _ => Ieee1164::_X,
    }
}
impl Not for Ieee1164 {
    type Output = Self;
    fn not(self) -> Self::Output {
        not(self)
    }
}
impl<'a> Not for &'a Ieee1164 {
    type Output = Ieee1164;
    fn not(self) -> Self::Output {
        not(*self)
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn resolve(a: &Ieee1164, b: &Ieee1164) -> Ieee1164 {
    const TTABLE: [[Ieee1164; 9]; 9] = [
        //U   X   0   1   Z   W   L   H   -
        [_U, _U, _U, _U, _U, _U, _U, _U, _U], // U
        [_U, _X, _X, _X, _X, _X, _X, _X, _X], // X
        [_U, _X, _0, _X, _0, _0, _0, _0, _X], // 0
        [_U, _X, _X, _1, _1, _1, _1, _1, _X], // 1
        [_U, _X, _0, _1, _Z, _W, _L, _H, _X], // Z
        [_U, _X, _0, _1, _W, _W, _W, _W, _X], // W
        [_U, _X, _0, _1, _L, _W, _L, _W, _X], // L
        [_U, _X, _0, _1, _H, _W, _W, _H, _X], // H
        [_U, _X, _X, _X, _X, _X, _X, _X, _X], // -
    ];
    TTABLE[a.to_index()][b.to_index()]
}
expand_op_ieee1164!(resolve, Resolve, resolve);

impl fmt::Display for Ieee1164 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl Ieee1164 {
    fn to_index(self) -> usize {
        match self {
            Ieee1164::_U => 0,
            Ieee1164::_X => 1,
            Ieee1164::_0 => 2,
            Ieee1164::_1 => 3,
            Ieee1164::_Z => 4,
            Ieee1164::_W => 5,
            Ieee1164::_L => 6,
            Ieee1164::_H => 7,
            Ieee1164::_D => 8,
        }
    }
}

#[allow(non_snake_case)]
impl Ieee1164 {
    /// Checks whether this is either [`Ieee1164::_U`], [`Ieee1164::_X`], [`Ieee1164::_W`],
    /// [`Ieee1164::_Z`] or [`Ieee1164::_D`].
    pub fn is_UXZ(self) -> bool {
        !(self.is_1H() || self.is_0L())
    }

    /// Checks whether this is either [`Ieee1164::_0`] or [`Ieee1164::_1`]
    pub fn is_01(self) -> bool {
        self.is_0() || self.is_1()
    }

    /// Checks whether this is either [`Ieee1164::_1`] or [`Ieee1164::_H`]
    pub fn is_1H(self) -> bool {
        self.is_1() || self.is_H()
    }

    /// Checks whether this is either [`Ieee1164::_0`] or [`Ieee1164::_L`]
    pub fn is_0L(self) -> bool {
        self.is_0() || self.is_L()
    }

    /// Checks whether this is [`Ieee1164::_U`]
    pub fn is_U(self) -> bool {
        self == _U
    }

    /// Checks whether this is [`Ieee1164::_X`]
    pub fn is_X(self) -> bool {
        self == _X
    }

    /// Checks whether this is [`Ieee1164::_0`]
    pub fn is_0(self) -> bool {
        self == _0
    }

    /// Checks whether this is [`Ieee1164::_1`]
    pub fn is_1(self) -> bool {
        self == _1
    }

    /// Checks whether this is [`Ieee1164::_Z`]
    pub fn is_Z(self) -> bool {
        self == _Z
    }

    /// Checks whether this is [`Ieee1164::_W`]
    pub fn is_W(self) -> bool {
        self == _W
    }

    /// Checks whether this is [`Ieee1164::_L`]
    pub fn is_L(self) -> bool {
        self == _L
    }

    /// Checks whether this is [`Ieee1164::_H`]
    pub fn is_H(self) -> bool {
        self == _H
    }

    /// Checks whether this is [`Ieee1164::_D`]
    pub fn is_D(self) -> bool {
        self == _D
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and() {
        assert_eq!(Ieee1164::_0, Ieee1164::_X & Ieee1164::_0);
    }

    #[test]
    fn or() {
        assert_eq!(Ieee1164::_1, Ieee1164::_L | Ieee1164::_H)
    }

    #[test]
    fn xor() {
        let a = Ieee1164::_1;
        let b = Ieee1164::_1;
        assert_eq!(Ieee1164::_0, a ^ b);
        //TODO
    }

    #[test]
    fn not() {
        let a = Ieee1164::_1;
        assert_eq!(Ieee1164::_0, !a);
    }

    #[test]
    fn is_01() {
        assert!(!Ieee1164::_U.is_01());
        assert!(Ieee1164::_0.is_01());
        assert!(Ieee1164::_1.is_01());
        assert!(!Ieee1164::_Z.is_01());
        assert!(!Ieee1164::_W.is_01());
        assert!(!Ieee1164::_L.is_01());
        assert!(!Ieee1164::_H.is_01());
        assert!(!Ieee1164::_D.is_01());
        assert!(!Ieee1164::_X.is_01());
    }

    #[test]
    fn is_uxz() {
        assert!(Ieee1164::_U.is_UXZ());
        assert!(!Ieee1164::_0.is_UXZ());
        assert!(!Ieee1164::_1.is_UXZ());
        assert!(Ieee1164::_Z.is_UXZ());
        assert!(Ieee1164::_W.is_UXZ());
        assert!(!Ieee1164::_L.is_UXZ());
        assert!(!Ieee1164::_H.is_UXZ());
        assert!(Ieee1164::_D.is_UXZ());
        assert!(Ieee1164::_X.is_UXZ());
    }

    #[test]
    fn check_associated_consts() {
        // this testcase seems useless, but I want to make sure, that the associated consts do match
        // the proposed values!
        assert_eq!(Ieee1164::_U, Ieee1164::Uninitialized);
        assert_eq!(Ieee1164::_X, Ieee1164::Strong(Ieee1164Value::Unknown));
        assert_eq!(Ieee1164::_1, Ieee1164::Strong(Ieee1164Value::One));
        assert_eq!(Ieee1164::_0, Ieee1164::Strong(Ieee1164Value::Zero));
        assert_eq!(Ieee1164::_W, Ieee1164::Weak(Ieee1164Value::Unknown));
        assert_eq!(Ieee1164::_H, Ieee1164::Weak(Ieee1164Value::One));
        assert_eq!(Ieee1164::_L, Ieee1164::Weak(Ieee1164Value::Zero));
        assert_eq!(Ieee1164::_Z, Ieee1164::HighImpedance);
        assert_eq!(Ieee1164::_D, Ieee1164::DontCare);
    }
}
