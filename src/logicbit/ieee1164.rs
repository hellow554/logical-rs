#![allow(clippy::just_underscores_and_digits)]

use super::{Ieee1164Value, Resolve};
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

macro_rules! expand_op_ieee1164 {
    ($func_name:ident, $trait_name:ident, $fn_name:ident) => {
        expand_op!($func_name, $trait_name, $fn_name, Ieee1164, Ieee1164, Ieee1164);
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ieee1164 {
    Uninitialized,
    Strong(Ieee1164Value),
    Weak(Ieee1164Value),
    HighImpedance,
    DontCare,
}

impl Default for Ieee1164 {
    fn default() -> Self {
        Ieee1164::Uninitialized
    }
}

impl From<char> for Ieee1164 {
    fn from(c: char) -> Self {
        match c.to_ascii_lowercase() {
            'u' => Ieee1164::Uninitialized,
            '0' => Ieee1164::Strong(Ieee1164Value::Zero),
            '1' => Ieee1164::Strong(Ieee1164Value::One),
            'z' => Ieee1164::HighImpedance,
            'w' => Ieee1164::Weak(Ieee1164Value::Unknown),
            'l' => Ieee1164::Weak(Ieee1164Value::Zero),
            'h' => Ieee1164::Weak(Ieee1164Value::One),
            '*' | '-' => Ieee1164::DontCare,
            'x' | _ => Ieee1164::Strong(Ieee1164Value::Unknown),
        }
    }
}

impl<'a> From<&'a Ieee1164> for char {
    fn from(i: &Ieee1164) -> Self {
        match *i {
            Ieee1164::Uninitialized => 'U',
            Ieee1164::Strong(Ieee1164Value::Unknown) => 'X',
            Ieee1164::Strong(Ieee1164Value::Zero) => '0',
            Ieee1164::Strong(Ieee1164Value::One) => '1',
            Ieee1164::HighImpedance => 'Z',
            Ieee1164::Weak(Ieee1164Value::Unknown) => 'W',
            Ieee1164::Weak(Ieee1164Value::Zero) => 'L',
            Ieee1164::Weak(Ieee1164Value::One) => 'H',
            Ieee1164::DontCare => '-',
        }
    }
}

const _U: Ieee1164 = Ieee1164::Uninitialized;
const _X: Ieee1164 = Ieee1164::Strong(Ieee1164Value::Unknown);
const _0: Ieee1164 = Ieee1164::Strong(Ieee1164Value::Zero);
const _1: Ieee1164 = Ieee1164::Strong(Ieee1164Value::One);
const _Z: Ieee1164 = Ieee1164::HighImpedance;
const _W: Ieee1164 = Ieee1164::Weak(Ieee1164Value::Unknown);
const _L: Ieee1164 = Ieee1164::Weak(Ieee1164Value::Zero);
const _H: Ieee1164 = Ieee1164::Weak(Ieee1164Value::One);
const _D: Ieee1164 = Ieee1164::DontCare;

fn and(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    const TTABLE: [[Ieee1164; 9]; 9] = [
        //    U   X   0   1   Z   W   L   H   -
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

fn or(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    const TTABLE: [[Ieee1164; 9]; 9] = [
        //    U   X   0   1   Z   W   L   H   -
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

fn xor(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    const TTABLE: [[Ieee1164; 9]; 9] = [
        //    U   X   0   1   Z   W   L   H   -
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
        Ieee1164::Uninitialized => Ieee1164::Uninitialized,
        Ieee1164::Weak(Ieee1164Value::Zero) | Ieee1164::Strong(Ieee1164Value::Zero) => {
            Ieee1164::Strong(Ieee1164Value::One)
        }
        Ieee1164::Weak(Ieee1164Value::One) | Ieee1164::Strong(Ieee1164Value::One) => {
            Ieee1164::Strong(Ieee1164Value::Zero)
        }
        Ieee1164::Strong(Ieee1164Value::Unknown)
        | Ieee1164::HighImpedance
        | Ieee1164::Weak(Ieee1164Value::Unknown)
        | Ieee1164::DontCare => Ieee1164::Strong(Ieee1164Value::Unknown),
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

fn resolve(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    const TTABLE: [[Ieee1164; 9]; 9] = [
        // U   X   0   1   Z   W   L   H   -
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
expand_resolve_op!(resolve, Ieee1164, Ieee1164, Ieee1164);

impl fmt::Display for Ieee1164 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}>", char::from(self))
    }
}

impl Ieee1164 {
    fn to_index(self) -> usize {
        match self {
            Ieee1164::Uninitialized => 0,
            Ieee1164::Strong(Ieee1164Value::Unknown) => 1,
            Ieee1164::Strong(Ieee1164Value::Zero) => 2,
            Ieee1164::Strong(Ieee1164Value::One) => 3,
            Ieee1164::HighImpedance => 4,
            Ieee1164::Weak(Ieee1164Value::Unknown) => 5,
            Ieee1164::Weak(Ieee1164Value::Zero) => 6,
            Ieee1164::Weak(Ieee1164Value::One) => 7,
            Ieee1164::DontCare => 8,
        }
    }
}

#[allow(non_snake_case)]
impl Ieee1164 {
    pub fn is_UXZ(self) -> bool {
        !(self.is_1H() || self.is_0L())
    }

    pub fn is_01(self) -> bool {
        self.is_0() || self.is_1()
    }

    pub fn is_1H(self) -> bool {
        self.is_1() || self.is_H()
    }

    pub fn is_0L(self) -> bool {
        self.is_0() || self.is_L()
    }

    pub fn is_U(self) -> bool {
        self == _U
    }

    pub fn is_X(self) -> bool {
        self == _X || self == _U || self == _Z || self == _W || self == _D
    }

    pub fn is_0(self) -> bool {
        self == _0
    }

    pub fn is_1(self) -> bool {
        self == _1
    }

    pub fn is_Z(self) -> bool {
        self == _Z
    }

    pub fn is_W(self) -> bool {
        self == _W
    }

    pub fn is_L(self) -> bool {
        self == _L
    }

    pub fn is_H(self) -> bool {
        self == _H
    }

    pub fn is_D(self) -> bool {
        self == _D
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Ieee1164::from('0'), Ieee1164::from('x') & Ieee1164::from('0'));
    }

    #[test]
    fn or() {
        assert_eq!(Ieee1164::from('1'), Ieee1164::from('L') | Ieee1164::from('H'))
    }

    #[test]
    fn xor() {
        let a = Ieee1164::from('1');
        let b = Ieee1164::from('1');
        assert_eq!(Ieee1164::from('0'), a ^ b);
        //TODO
    }

    #[test]
    fn not() {
        //TODO
    }

    #[test]
    fn is_01() {
        assert!(!Ieee1164::from('u').is_01());
        assert!(Ieee1164::from('0').is_01());
        assert!(Ieee1164::from('1').is_01());
        assert!(!Ieee1164::from('z').is_01());
        assert!(!Ieee1164::from('w').is_01());
        assert!(!Ieee1164::from('l').is_01());
        assert!(!Ieee1164::from('h').is_01());
        assert!(!Ieee1164::from('-').is_01());
        assert!(!Ieee1164::from('x').is_01());
    }

    #[test]
    fn is_uxz() {
        assert!(Ieee1164::from('u').is_UXZ());
        assert!(!Ieee1164::from('0').is_UXZ());
        assert!(!Ieee1164::from('1').is_UXZ());
        assert!(Ieee1164::from('z').is_UXZ());
        assert!(Ieee1164::from('w').is_UXZ());
        assert!(!Ieee1164::from('l').is_UXZ());
        assert!(!Ieee1164::from('h').is_UXZ());
        assert!(Ieee1164::from('-').is_UXZ());
        assert!(Ieee1164::from('x').is_UXZ());
    }
}
