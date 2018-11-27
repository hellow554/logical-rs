use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut};

use crate::{Ieee1164, Ieee1164Value};
use std::cmp::Ordering;
use std::fmt;
use std::convert::TryFrom;

macro_rules! expand_op_logicvector {
    ($func_name:ident, $trait_name:ident, $fn_name:ident) => {
        expand_op!(
            $func_name,
            $trait_name,
            $fn_name,
            LogicVector,
            LogicVector,
            Option<LogicVector>
        );
    };
}

#[derive(Debug, Clone)]
pub struct LogicVector {
    inner: Vec<Ieee1164>, //TODO: maybe use masks instead of actual bits... may be faster or so
}

impl LogicVector {
    pub fn with_value(value: u128, width: usize) -> Option<Self> {
        let zeros = value.leading_zeros() as usize;
        if width < (128 - zeros) {
            return None;
        }
        let mut v: LogicVector = Self::from_str(&format!("{:b}", value)).unwrap();
        v.set_width_with_value(width, Ieee1164::_0);
        Some(v)
    }

    pub fn from_str(s: &str) -> Result<Self, LogicVectorConversionError> {
        s.chars()
            .try_fold(vec![], |mut v , c| {
                v.push(Ieee1164::try_from(c).map_err(|_| LogicVectorConversionError::InalidChar(c))?);
                Ok(v)
            })
            .map(|v| v.into())
    }

    pub fn with_width(width: usize) -> Self {
        assert_ne!(width, 0);
        LogicVector {
            inner: vec![Ieee1164::default(); width],
        }
    }

    pub fn width(&self) -> usize {
        self.inner.len()
    }

    pub fn set_width(&mut self, new_width: usize) {
        self.set_width_with_value(new_width, Ieee1164::_U);
    }

    pub fn set_width_with_value(&mut self, new_width: usize, value: Ieee1164) {
        let old_width = self.width();
        self.inner = match old_width.cmp(&new_width) {
            Ordering::Equal => return,
            Ordering::Less => [vec![value; new_width - old_width].as_slice(), &self.inner].concat(),
            Ordering::Greater => self.inner.as_slice()[(old_width - new_width)..].to_vec(),
        };
    }

    fn to_u128(&self) -> Option<u128> {
        if self.has_UXZ() {
            return None;
        }
        Some(self.inner.iter().fold(0, |s, e| {
            (s << 1)
                | if e.is_1H() {
                    1
                } else if e.is_0L() {
                    0
                } else {
                    unreachable!("Logic error?!")
                }
        }))
    }
}

impl Index<usize> for LogicVector {
    type Output = Ieee1164;

    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for LogicVector {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.inner[index]
    }
}

fn and(lhs: &LogicVector, rhs: &LogicVector) -> Option<LogicVector> {
    if lhs.width() != rhs.width() {
        return None;
    }
    Some(
        lhs.inner
            .iter()
            .zip(rhs.inner.iter())
            .map(|(l, r)| l & r)
            .collect::<Vec<_>>()
            .into(),
    )
}
expand_op_logicvector!(and, BitAnd, bitand);

fn or(lhs: &LogicVector, rhs: &LogicVector) -> Option<LogicVector> {
    if lhs.width() != rhs.width() {
        return None;
    }
    Some(
        lhs.inner
            .iter()
            .zip(rhs.inner.iter())
            .map(|(l, r)| l | r)
            .collect::<Vec<_>>()
            .into(),
    )
}
expand_op_logicvector!(or, BitOr, bitor);

fn xor(lhs: &LogicVector, rhs: &LogicVector) -> Option<LogicVector> {
    if lhs.width() != rhs.width() {
        return None;
    }
    Some(
        lhs.inner
            .iter()
            .zip(rhs.inner.iter())
            .map(|(l, r)| l ^ r)
            .collect::<Vec<_>>()
            .into(),
    )
}
expand_op_logicvector!(xor, BitXor, bitxor);

fn add(lhs: &LogicVector, rhs: &LogicVector) -> Option<LogicVector> {
    if lhs.width() != rhs.width() {
        return None;
    }
    if let (Some(a), Some(b)) = (lhs.to_u128(), rhs.to_u128()) {
        LogicVector::with_value((a + b) & ((1 << lhs.width()) - 1), lhs.width())
    } else {
        Some(LogicVector::with_width(lhs.width()))
    }
}
expand_op_logicvector!(add, Add, add);

impl PartialEq for LogicVector {
    fn eq(&self, other: &LogicVector) -> bool {
        if let (Some(a), Some(b)) = (self.to_u128(), other.to_u128()) {
            a == b
        } else {
            false
        }
    }
}

impl Eq for LogicVector {}

impl PartialEq<u128> for LogicVector
{
    fn eq(&self, other: &u128) -> bool {
        if let Some(this) = self.to_u128() {
            this == *other
        } else {
            false
        }
    }
}

#[allow(non_snake_case)]
impl LogicVector {
    fn contains(&self, value: Ieee1164) -> bool {
        self.inner.contains(&value)
    }
    pub fn has_U(&self) -> bool {
        self.contains(Ieee1164::Uninitialized)
    }

    pub fn has_X(&self) -> bool {
        self.contains(Ieee1164::Strong(Ieee1164Value::Unknown))
    }

    pub fn has_0(&self) -> bool {
        self.contains(Ieee1164::Strong(Ieee1164Value::Zero))
    }

    pub fn has_1(&self) -> bool {
        self.contains(Ieee1164::Strong(Ieee1164Value::One))
    }

    pub fn has_Z(&self) -> bool {
        self.contains(Ieee1164::HighImpedance)
    }

    pub fn has_W(&self) -> bool {
        self.contains(Ieee1164::Weak(Ieee1164Value::Unknown))
    }

    pub fn has_D(&self) -> bool {
        self.contains(Ieee1164::DontCare)
    }

    pub fn has_L(&self) -> bool {
        self.contains(Ieee1164::Weak(Ieee1164Value::Zero))
    }

    pub fn has_H(&self) -> bool {
        self.contains(Ieee1164::Weak(Ieee1164Value::One))
    }

    pub fn has_UXZ(&self) -> bool {
        self.inner.iter().any(|x| x.is_UXZ())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LogicVectorConversionError {
    InalidChar(char),
}

impl Into<LogicVector> for Vec<Ieee1164> {
    fn into(self) -> LogicVector {
        LogicVector { inner: self }
    }
}

impl fmt::Display for LogicVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in self.inner.iter() {
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}

impl PartialOrd for LogicVector {
    fn partial_cmp(&self, other: &LogicVector) -> Option<Ordering> {
        if self.width() != other.width() {
            return None;
        }
        if self.has_UXZ() || other.has_UXZ() {
            return None;
        }

        self.to_u128().partial_cmp(&other.to_u128())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{prop_assert, prop_assert_eq, proptest, proptest_helper};

    proptest! {
        #[test]
        fn atm_ctor_value(value in 0u64..) {
            let v = LogicVector::with_value(value as u128, 128);
            prop_assert!(v.is_some());
            prop_assert_eq!(v.unwrap(), value as u128);
        }
    }

    #[test]
    fn ctor_width() {
        for width in 1..=128 {
            let v = LogicVector::with_width(width);
            assert_eq!(width, v.width());
            assert!(v.has_U(), "{:?}", v);
            assert!(!v.has_X(), "{:?}", v);
            assert!(!v.has_0(), "{:?}", v);
            assert!(!v.has_1(), "{:?}", v);
            assert!(!v.has_Z(), "{:?}", v);
            assert!(!v.has_W(), "{:?}", v);
            assert!(!v.has_D(), "{:?}", v);
            assert!(!v.has_L(), "{:?}", v);
            assert!(!v.has_H(), "{:?}", v);
        }
    }

    #[test]
    fn ctor_value() {
        let v = LogicVector::with_value(5, 3);
        let v = v.unwrap();
        assert_eq!(v.width(), 3);
        assert_eq!(v, 5);
        let v = LogicVector::with_value(0, 128);
        let v = v.unwrap();
        assert_eq!(v.width(), 128);
        assert_eq!(v, 0);
    }

    #[test]
    fn test_resize() {
        let mut v = LogicVector::with_width(5);
        assert_eq!(v.width(), 5);
        v.set_width(10);
        assert_eq!(v.width(), 10);
        v.set_width(10);
        assert_eq!(v.width(), 10);
        v.set_width(3);
        assert_eq!(v.width(), 3);
    }

    #[test]
    fn test_resize_value() {
        let mut v = LogicVector::with_value(5, 8).unwrap();
        assert_eq!(v.width(), 8);
        assert_eq!(v, 5);
        v.set_width(10);
        assert_eq!(v.width(), 10);
        v.set_width(10);
        assert_eq!(v.width(), 10);
        v.set_width(3);
        assert_eq!(v.width(), 3);
    }
}
