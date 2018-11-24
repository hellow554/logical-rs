#![allow(clippy::just_underscores_and_digits)]

use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::Resolve;

macro_rules! expand_op_ieee1164value {
    ($func_name:ident, $trait_name:ident, $fn_name:ident) => {
        expand_op!(
            $func_name,
            $trait_name,
            $fn_name,
            Ieee1164Value,
            Ieee1164Value,
            Ieee1164Value
        );
    };
}

/// An Ieee1164Value is either `Zero`, `One` or `Unknown`, also known as Three-valued logic.
/// You can use any binary, logical operation on it (or, and, xor) or use not and combine them as
/// needed.
///
/// # Example: ¬((a ⊼ b) ⊻ c)
///
/// ```rust
/// # use logical::Ieee1164Value;
/// fn perform(a: Ieee1164Value, b: Ieee1164Value, c: Ieee1164Value) -> Ieee1164Value {
///     !(!(a & b) ^ c)
/// }
/// # assert_eq!(perform(Ieee1164Value::Zero, Ieee1164Value::Zero, Ieee1164Value::Zero), Ieee1164Value::Zero);
/// # assert_eq!(perform(Ieee1164Value::Zero, Ieee1164Value::Zero, Ieee1164Value::One), Ieee1164Value::One);
/// # assert_eq!(perform(Ieee1164Value::Zero, Ieee1164Value::One, Ieee1164Value::Zero), Ieee1164Value::Zero);
/// # assert_eq!(perform(Ieee1164Value::Zero, Ieee1164Value::One, Ieee1164Value::One), Ieee1164Value::One);
/// # assert_eq!(perform(Ieee1164Value::One, Ieee1164Value::Zero, Ieee1164Value::Zero), Ieee1164Value::Zero);
/// # assert_eq!(perform(Ieee1164Value::One, Ieee1164Value::Zero, Ieee1164Value::One), Ieee1164Value::One);
/// # assert_eq!(perform(Ieee1164Value::One, Ieee1164Value::One, Ieee1164Value::Zero), Ieee1164Value::One);
/// # assert_eq!(perform(Ieee1164Value::One, Ieee1164Value::One, Ieee1164Value::One), Ieee1164Value::Zero);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ieee1164Value {
    /// Represents a logic zero or false
    Zero,
    /// Represents a logic one or true
    One,
    /// Represents an unknown value
    Unknown,
}

use self::Ieee1164Value as Ie;

const _0: Ieee1164Value = Ieee1164Value::Zero;
const _1: Ieee1164Value = Ieee1164Value::One;
const _U: Ieee1164Value = Ieee1164Value::Unknown;

impl Default for Ie {
    fn default() -> Self {
        Ie::Unknown
    }
}

impl fmt::Display for Ieee1164Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Ie::Zero => '0',
                Ie::One => '1',
                Ie::Unknown => 'U',
            }
        )
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn and(a: &Ie, b: &Ie) -> Ie {
    match (*a, *b) {
        (_0, _) | (_, _0) => _0,
        (_1, _1) => _1,
        _ => _U,
    }
}
expand_op_ieee1164value!(and, BitAnd, bitand);

#[allow(clippy::trivially_copy_pass_by_ref)]
fn or(a: &Ie, b: &Ie) -> Ie {
    match (*a, *b) {
        (_1, _) | (_, _1) => _1,
        (_0, _0) => _0,
        _ => _U,
    }
}
expand_op_ieee1164value!(or, BitOr, bitor);

fn not(a: Ie) -> Ie {
    match a {
        _0 => _1,
        _U => _U,
        _1 => _0,
    }
}
impl Not for Ieee1164Value {
    type Output = Self;
    fn not(self) -> Self::Output {
        not(self)
    }
}
impl<'a> Not for &'a Ieee1164Value {
    type Output = Ieee1164Value;
    fn not(self) -> Self::Output {
        not(*self)
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn xor(a: &Ie, b: &Ie) -> Ie {
    match (*a, *b) {
        (_U, _) | (_, _U) => _U,
        (a, b) if a == b => _0,
        _ => _1,
    }
}
expand_op_ieee1164value!(xor, BitXor, bitxor);

#[allow(clippy::trivially_copy_pass_by_ref)]
fn resolve(a: &Ie, b: &Ie) -> Ie {
    match (*a, *b) {
        (a, b) if a == b => a,
        _ => _U,
    }
}
expand_op_ieee1164value!(resolve, Resolve, resolve);

#[cfg(test)]
mod tests {
    use super::*;

    const BIN_VAL: [[Ie; 2]; 9] = [
        [_0, _0],
        [_0, _1],
        [_0, _U],
        [_1, _0],
        [_1, _1],
        [_1, _U],
        [_U, _0],
        [_U, _1],
        [_U, _U],
    ];

    fn test(test_vector: &[Ie; 9], operator: &str, func: fn(&Ie, &Ie) -> Ie) {
        for ([a, b], e) in BIN_VAL.iter().zip(test_vector) {
            assert_eq!(func(a, b), *e, "{} {} {} ≠ {}", a, operator, b, e);
        }
    }

    #[test]
    fn t_and() {
        const EXPECTED: [Ie; 9] = [_0, _0, _0, _0, _1, _U, _0, _U, _U];
        test(&EXPECTED, "&", and);
    }

    #[test]
    fn t_or() {
        const EXPECTED: [Ie; 9] = [_0, _1, _U, _1, _1, _1, _U, _1, _U];
        test(&EXPECTED, "|", or);
    }

    #[test]
    fn t_xor() {
        const EXPECTED: [Ie; 9] = [_0, _1, _U, _1, _0, _U, _U, _U, _U];
        test(&EXPECTED, "⊻", xor);
    }

    #[test]
    fn t_not() {
        assert_eq!(_1, not(_0));
        assert_eq!(_0, not(_1));
        assert_eq!(_U, not(_U));
    }

    #[test]
    fn t_resolve() {
        const EXPECTED: [Ie; 9] = [_0, _U, _U, _U, _1, _U, _U, _U, _U];
        test(&EXPECTED, "resolves", resolve);
    }
}
