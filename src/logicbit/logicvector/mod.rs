mod masks;
use self::masks::{Masks, SanityChecked};

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor};
use std::str::FromStr;

use crate::{Ieee1164, Resolve};

#[allow(unused)]
macro_rules! expand_op_logicvector {
    ($func_name:ident, $trait_name:ident, $fn_name:ident) => {
        expand_op!(
            $func_name,
            $trait_name,
            $fn_name,
            LogicVector,
            LogicVector,
            LogicVector
        );
    };
}

macro_rules! unsafe_version {
    ($safe_name:ident, $unsafe_name:ident, $lhs:ty, $rhs:ty, $output:ty) => {
        fn $unsafe_name(lhs: &$lhs, rhs: &$rhs) -> $output {
            $safe_name(lhs, rhs).unwrap()
        }
    };
}

macro_rules! unsafe_version_logicvector {
    ($safe_name:ident, $unsafe_name:ident) => {
        unsafe_version!($safe_name, $unsafe_name, LogicVector, LogicVector, LogicVector);
    };
}

#[inline(always)]
fn mask_from_width(width: u8) -> u128 {
    if width != 128 {
        ((1 << width) - 1)
    } else {
        std::u128::MAX
    }
}

#[inline(always)]
fn assert_width(width: u8) -> bool {
    width != 0 && width <= 128
}

/// A logicvector is an vector containing [`Ieee1164`] as values.
///
/// # Invariant
///
/// There are the following invariants for this struct.
///
///   1. The width is always not equals zero.
///   2. The width is limited to 128.
///
/// If any of these limitations are violated a panic will occur.
///
#[derive(Debug, Clone)]
pub struct LogicVector {
    masks: Masks,
    width: u8,
}

impl LogicVector {
    /// Accepts a [`Ieee1164`] and set that value to the whole range of `width`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use logical::{Ieee1164, LogicVector};
    /// let lv = LogicVector::from_ieee_value(Ieee1164::_0, 8);
    /// assert_eq!(8, lv.width());
    /// assert!(lv.is_000());
    /// ```
    pub fn from_ieee_value(value: Ieee1164, width: u8) -> Self {
        assert!(assert_width(width));
        let mut s = Self {
            masks: Masks::default(),
            width,
        };
        s.masks[value] = std::u128::MAX & mask_from_width(width);
        debug_assert_eq!(Ok(()), s.sanity_check());
        s
    }

    /// Tries to convert an integer value with a given width to a `Logicvector`.
    ///
    /// It will return `None` if the invariants are violated (e.g. the `width` is `0` or greater
    /// than `128`), or the binary size of `value` is greater than `width`.
    ///
    /// # Examples
    ///
    /// This example is successful because the `width` (`8`) is greater than 0, less than 129 and
    /// the bit representation of `42` (`0b101010`) fits into 8 bits.
    ///
    /// ```rust
    /// use logical::LogicVector;
    /// let lv = LogicVector::from_int_value(42, 8).unwrap();
    /// assert_eq!(lv.as_u128(), Some(42));
    /// ```
    ///
    /// This example will return `None`, because `42` (`0b101010`) does not fit into 5 bits, but
    /// need 6 instead.
    ///
    /// ```rust
    /// use logical::LogicVector;
    /// let lv = LogicVector::from_int_value(42, 5);
    /// assert!(lv.is_none());
    /// ```
    pub fn from_int_value(value: u128, width: u8) -> Option<Self> {
        let zeros = value.leading_zeros() as u8;
        if assert_width(width) && width >= (128 - zeros) {
            let mut masks = Masks::default();
            masks[Ieee1164::_1] = value;
            masks[Ieee1164::_0] = (!value) & mask_from_width(width);

            debug_assert_eq!(Ok(()), masks.sanity_check(width));
            Some(Self { masks, width })
        } else {
            None
        }
    }

    /// Creates a LogicVector with the given width and all values are set to [`Ieee1164::_U`]
    /// (undefined). It is a shortcut for
    ///
    /// ```text
    /// LogicVector::from_ieee_value(Ieee1164::_U, width);
    /// ```
    pub fn with_width(width: u8) -> Self {
        assert!(assert_width(width));
        Self::from_ieee_value(Ieee1164::default(), width)
    }
}

impl LogicVector {
    /// Returns the width of this LogicVector.
    ///
    /// ```rust
    /// # use logical::LogicVector;
    /// assert_eq!(7, LogicVector::with_width(7).width());
    /// ```
    pub fn width(&self) -> u8 {
        self.width
    }

    /// Set's the width of this LogicVector. For further information see [`LogicVector::resize`].
    /// This is a shortcut for
    /// ```text
    /// LogicVector::resize(new_width, Ieee1164::_U);
    /// ```
    pub fn set_width(&mut self, new_width: u8) {
        self.resize(new_width, Ieee1164::_U);
        debug_assert_eq!(Ok(()), self.sanity_check());
    }

    /// Resizes the LogicVector to `new_width`. There are three possible cases.
    ///
    ///  1. `new_width == self.width()`
    ///     - nothing changes
    ///  2. `new_width < self.width()`
    ///     - The LogicVector will get cropped to the new size. The cropped values will be returned
    ///       as a new LogicVector, whereas the width is equal to the difference between the old
    ///       and new width.
    ///  3. `new_width > self.width()`
    ///     - The LogicVector will be set to the new_length and the new bits will be set to `value`.
    ///
    /// # Examples
    ///
    /// Using the same length will not change anything.
    ///
    /// ```rust
    /// # use logical::{Ieee1164, LogicVector};
    /// let mut lv1 = LogicVector::from_int_value(42, 8).unwrap();
    /// let lv2 = lv1.clone();
    /// let cropped = lv1.resize(8, Ieee1164::_U);
    ///
    /// assert!(cropped.is_none());
    /// assert_eq!(lv1, lv2);
    /// ```
    ///
    /// Making the LogicVector smaller will return the cropped values.
    ///
    /// ```rust
    /// # use logical::{Ieee1164, LogicVector};
    /// let mut lv = LogicVector::from_int_value(58, 7).unwrap();
    /// let cropped = lv.resize(4, Ieee1164::_U).unwrap();
    ///
    /// assert_eq!(4, lv.width());
    /// assert_eq!(3, cropped.width());
    /// assert_eq!(Some(0b011), cropped.as_u128());
    /// assert_eq!(Some(0b1010), lv.as_u128());
    /// ```
    ///
    /// Making the LogicVector greater will set the upper most bits to `value`.
    ///
    /// ```rust
    /// # use logical::{Ieee1164, LogicVector};
    /// let mut lv = LogicVector::from_int_value(42, 6).unwrap();
    /// lv.resize(8, Ieee1164::_1);
    ///
    /// assert_eq!(Some(0b11101010), lv.as_u128());
    /// ```
    ///
    /// ```rust
    /// # use logical::{Ieee1164, LogicVector};
    /// let mut lv = LogicVector::from_int_value(42, 8).unwrap();
    /// lv.resize(10, Ieee1164::_1);
    ///
    /// assert_eq!(Some(0b1100101010), lv.as_u128());
    /// ```
    pub fn resize(&mut self, new_width: u8, value: Ieee1164) -> Option<LogicVector> {
        fn resize_mask(old: u8, new: u8) -> u128 {
            match (old, new) {
                (a, b) if a >= b => unreachable!("`old` cannot be greater/equal than `new`!"),
                (128, 128) => std::u128::MAX,
                (a, 128) => std::u128::MAX & !((1 << a) - 1),
                (a, b) => ((1 << b) - 1) & !((1 << a) - 1),
            }
        }

        assert!(assert_width(new_width));
        let old_width = self.width();
        self.width = new_width as u8;

        let res = match old_width.cmp(&new_width) {
            Ordering::Equal => None,
            Ordering::Less => {
                let mask = resize_mask(old_width, new_width);

                for m in &mut self.masks {
                    if m.0 == value {
                        *m.1 |= std::u128::MAX & mask;
                    } else {
                        *m.1 &= !(std::u128::MAX & mask);
                    }
                }
                None
            }
            Ordering::Greater => {
                let mut nv = Masks::default();

                let mask_nv = resize_mask(new_width, old_width);
                let mask_ov = mask_from_width(new_width);
                for (m_new, m_old) in nv.iter_mut().zip(self.masks.iter_mut()) {
                    assert_eq!(m_new.0, m_old.0);
                    *m_new.1 = (*m_old.1 & mask_nv) >> new_width;
                    *m_old.1 &= std::u128::MAX & mask_ov;
                }

                Some(LogicVector {
                    masks: nv,
                    width: old_width - new_width,
                })
            }
        };
        if let Some(ref nv) = res {
            debug_assert_eq!(Ok(()), nv.sanity_check());
        }
        debug_assert_eq!(Ok(()), self.sanity_check());
        res
    }

    /// Set's every bit to `value`.
    ///
    /// ```rust
    /// # use logical::{Ieee1164, LogicVector};
    /// let mut lv = LogicVector::with_width(8);
    /// assert!(lv.is_UUU());
    /// assert!(!lv.is_ZZZ());
    ///
    /// lv.set_all_to(Ieee1164::_Z);
    /// assert!(!lv.is_UUU());
    /// assert!(lv.is_ZZZ());
    /// ```
    pub fn set_all_to(&mut self, value: Ieee1164) {
        for mask in &mut self.masks {
            *mask.1 = if value == mask.0 {
                mask_from_width(self.width)
            } else {
                0
            }
        }
        debug_assert_eq!(Ok(()), self.sanity_check());
    }

    //TODO introduce proper error type
    //TODO documentation
    pub fn set_int_value(&mut self, value: u128) -> Result<(), ()> {
        std::mem::replace(self, Self::from_int_value(value, self.width()).ok_or(())?);
        Ok(())
    }

    /// Tries to convert this to a `u128`. This will fail if the LogicVector contains any other bits
    /// than [`Ieee1164::_0`] or [`Ieee1164::_1`].
    ///
    /// ```rust
    /// # use logical::LogicVector;
    /// let lv = LogicVector::from_int_value(55, 8).unwrap();
    /// assert_eq!(Some(55), lv.as_u128());
    /// ```
    ///
    /// ```rust
    /// # use logical::{Ieee1164, LogicVector};
    /// let mut lv = LogicVector::from_int_value(55, 8).unwrap();
    /// assert_eq!(Some(55), lv.as_u128());
    /// lv.set(7, Ieee1164::_X);
    /// assert_eq!(None, lv.as_u128());
    /// ```
    pub fn as_u128(&self) -> Option<u128> {
        if self.has_UXZ() {
            None
        } else {
            Some(self.masks[Ieee1164::_1])
        }
    }

    pub fn get(&self, idx: u8) -> Option<Ieee1164> {
        assert!(idx < 128);
        if idx < self.width() {
            Some(self.masks.get(idx))
        } else {
            None
        }
    }

    pub fn set(&mut self, idx: u8, value: Ieee1164) {
        assert!(idx < 128);
        if idx < self.width() {
            self.masks.set(idx, value)
        }
    }
}

impl LogicVector {
    fn sanity_check(&self) -> Result<(), SanityChecked> {
        self.masks.sanity_check(self.width)
    }
}

fn and(lhs: &LogicVector, rhs: &LogicVector) -> Option<LogicVector> {
    if lhs.width() != rhs.width() {
        return None;
    }

    let mut masks = Masks::default();

    if lhs.has_UXZ() || rhs.has_UXZ() {
        for _ in 0..lhs.width {
            unimplemented!()
        }
    } else {
        let idx_1 = Ieee1164::_1;
        let idx_0 = Ieee1164::_0;
        masks[idx_1] = lhs.masks[idx_1] & rhs.masks[idx_1];
        masks[idx_0] = lhs.masks[idx_0] & rhs.masks[idx_0];
    }

    Some(LogicVector {
        masks,
        width: lhs.width,
    })
}
unsafe_version_logicvector!(and, unsafe_and);
expand_op_logicvector!(unsafe_and, BitAnd, bitand);

fn or(lhs: &LogicVector, rhs: &LogicVector) -> Option<LogicVector> {
    if lhs.width() != rhs.width() {
        return None;
    }

    let mut masks = Masks::default();

    if lhs.has_UXZ() || rhs.has_UXZ() {
        for _ in 0..lhs.width {
            unimplemented!()
        }
    } else {
        let idx_1 = Ieee1164::_1;
        let idx_0 = Ieee1164::_0;
        masks[idx_1] = lhs.masks[idx_1] | rhs.masks[idx_1];
        masks[idx_0] = lhs.masks[idx_0] | rhs.masks[idx_0];
    }

    Some(LogicVector {
        masks,
        width: lhs.width,
    })
}
unsafe_version_logicvector!(or, unsafe_or);
expand_op_logicvector!(unsafe_or, BitOr, bitor);

fn xor(lhs: &LogicVector, rhs: &LogicVector) -> Option<LogicVector> {
    if lhs.width() != rhs.width() {
        return None;
    }

    let mut masks = Masks::default();

    if lhs.has_UXZ() || rhs.has_UXZ() {
        for _ in 0..lhs.width {
            unimplemented!()
        }
    } else {
        let idx_1 = Ieee1164::_1;
        let idx_0 = Ieee1164::_0;
        masks[idx_1] = lhs.masks[idx_1] ^ rhs.masks[idx_1];
        masks[idx_0] = lhs.masks[idx_0] ^ rhs.masks[idx_0];
    }

    Some(LogicVector {
        masks,
        width: lhs.width,
    })

    //TODO maybe replace by macro and only provide & | ^
}
unsafe_version_logicvector!(xor, unsafe_xor);
expand_op_logicvector!(unsafe_xor, BitXor, bitxor);

impl LogicVector {
    pub fn safe_add(&self, rhs: &LogicVector) -> Option<LogicVector> {
        if self.width() != rhs.width() {
            return None;
        }
        let width = self.width();
        if let (Some(a), Some(b)) = (self.as_u128(), rhs.as_u128()) {
            LogicVector::from_int_value((a + b) & mask_from_width(width), width)
        } else {
            Some(LogicVector::with_width(width))
        }
    }

    pub fn wrapping_add(&self, _rhs: &LogicVector) -> LogicVector {
        unimplemented!()
    }
}

fn add(lhs: &LogicVector, rhs: &LogicVector) -> LogicVector {
    //fast, unsafe version
    let width = lhs.width();
    assert_eq!(width, rhs.width());

    LogicVector::from_int_value(
        (lhs.as_u128().unwrap() + rhs.as_u128().unwrap()) & mask_from_width(width),
        width,
    )
    .unwrap()
}
expand_op_logicvector!(add, Add, add);

fn resolve(_lhs: &LogicVector, _rhs: &LogicVector) -> LogicVector {
    unimplemented!()
}
expand_op!(resolve, Resolve, resolve, LogicVector, LogicVector, LogicVector);

impl PartialEq for LogicVector {
    fn eq(&self, other: &LogicVector) -> bool {
        self.masks == other.masks
    }
}

impl Eq for LogicVector {}

impl PartialEq<u128> for LogicVector {
    fn eq(&self, other: &u128) -> bool {
        if let Some(this) = self.as_u128() {
            this == *other
        } else {
            false
        }
    }
}

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! gen_has {
    ($name:ident, $value:expr) => {
        doc_comment! {
            concat!("This is a shortcut for [`LogicVector::has_ieee1164`].

```text
LogicVector::has_ieee1164(", stringify!($value), ")
```"),
            pub fn $name(&self) -> bool {
                self.has_ieee1164($value)
            }
        }
    };
}

macro_rules! gen_is {
    ($name:ident, $value:expr) => {
        doc_comment! {
            concat!("This is a shortcut for [`LogicVector::is_ieee1164`].

```text
LogicVector::is_ieee1164(", stringify!($value), ");
```"),
            pub fn $name(&self) -> bool {
                self.is_ieee1164($value)
            }
        }
    };
}

#[allow(non_snake_case)]
impl LogicVector {
    /// Checks if any bit is set to `value`.
    ///
    /// Returns true if so, false if that bit is not present in this LogicVector.
    pub fn has_ieee1164(&self, value: Ieee1164) -> bool {
        self.masks[value] != 0
    }

    /// Checks if all bits are set to `value`.
    ///
    /// Returns true if so, false if even one single bit is not set to `value` in this LogicVector.
    pub fn is_ieee1164(&self, value: Ieee1164) -> bool {
        self.masks[value] == std::u128::MAX & mask_from_width(self.width)
    }

    pub fn has_UXZ(&self) -> bool {
        self.has_U() || self.has_X() || self.has_W() || self.has_Z() || self.has_D()
    }

    gen_has!(has_U, Ieee1164::_U);
    gen_has!(has_X, Ieee1164::_X);
    gen_has!(has_0, Ieee1164::_0);
    gen_has!(has_1, Ieee1164::_1);
    gen_has!(has_W, Ieee1164::_W);
    gen_has!(has_H, Ieee1164::_H);
    gen_has!(has_L, Ieee1164::_L);
    gen_has!(has_Z, Ieee1164::_Z);
    gen_has!(has_D, Ieee1164::_D);

    gen_is!(is_UUU, Ieee1164::_U);
    gen_is!(is_XXX, Ieee1164::_X);
    gen_is!(is_000, Ieee1164::_0);
    gen_is!(is_111, Ieee1164::_1);
    gen_is!(is_WWW, Ieee1164::_W);
    gen_is!(is_HHH, Ieee1164::_H);
    gen_is!(is_LLL, Ieee1164::_L);
    gen_is!(is_ZZZ, Ieee1164::_Z);
    gen_is!(is_DDD, Ieee1164::_D);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LogicVectorConversionError {
    InalidChar(char),
    InvalidWidth,
}

impl From<Vec<Ieee1164>> for LogicVector {
    fn from(v: Vec<Ieee1164>) -> LogicVector {
        let len = v.len();
        assert!(assert_width(u8::try_from(len).unwrap()));

        let mut masks = Masks::default();
        for (i, v) in v.into_iter().enumerate() {
            masks[v] |= 1 << (len - (i + 1));
        }

        debug_assert_eq!(Ok(()), masks.sanity_check(len as u8));

        LogicVector {
            masks,
            width: len as u8,
        }
    }
}

impl FromStr for LogicVector {
    type Err = LogicVectorConversionError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        if !assert_width(u8::try_from(s.len()).map_err(|_| LogicVectorConversionError::InvalidWidth)?) {
            Err(LogicVectorConversionError::InvalidWidth)
        } else {
            s.chars()
                .try_fold(vec![], |mut v, c| {
                    v.push(Ieee1164::try_from(c).map_err(|_| LogicVectorConversionError::InalidChar(c))?);
                    Ok(v)
                })
                .map(|v| v.into())
        }
    }
}

impl fmt::Display for LogicVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO real formatting like padding etc
        let mut s = String::new();
        for i in (0..self.width).rev() {
            for mask in &self.masks {
                if (mask.1 >> i) & 1 == 1 {
                    s.push(mask.0.into());
                    continue;
                }
            }
        }
        write!(f, "{}", s)
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

        self.as_u128().partial_cmp(&other.as_u128())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{prop_assert, prop_assert_eq, prop_assume, proptest, proptest_helper};

    proptest! {
        #[test]
        fn atm_ctor_value(value in 1u64..) {
            let v = LogicVector::from_int_value(value as u128, 128);
            prop_assert!(v.is_some());
            let v = v.unwrap();
            prop_assert_eq!(v, value as u128);
        }

        #[test]
        fn atm_as_u128(val in 0u64..) {
            let v = LogicVector::from_int_value(val as u128, 64);
            prop_assert!(v.is_some());
            let mut v = v.unwrap();
            prop_assert_eq!(Ok(()), v.sanity_check());
            prop_assert_eq!(v.clone(), val as u128);
            v.resize(128, Ieee1164::_0);
            prop_assert_eq!(v.clone(), val as u128);
            v.resize(64, Ieee1164::_0);
            v.resize(128, Ieee1164::_1);
            prop_assert_eq!(v.clone(), ((std::u64::MAX as u128 )<< 64) | (val as u128));
        }

        #[test]
        fn atm_add(a1 in 0u64.., a2 in 0u64.., b1 in 0u64.., b2 in 0u64..) {
            let a = (a1 as u128) << 64| (a2) as u128;
            let b = (b1 as u128) << 64 | (b2) as u128;
            let c = a.checked_add(b);
            prop_assume!(c.is_some());
            let c = c.unwrap();

            let ia = LogicVector::from_int_value(a, 128);
            let ib = LogicVector::from_int_value(b, 128);

            prop_assert!(ia.is_some());
            prop_assert!(ib.is_some());

            let ia = ia.unwrap();
            let ib = ib.unwrap();

            prop_assert_eq!(ia.as_u128(), Some(a));
            prop_assert_eq!(ib.as_u128(), Some(b));

            let ic = ia + ib;
            prop_assert_eq!(ic.as_u128(), Some(c));
        }

        #[test]
        fn atm_to_string(ref a in "[ux10whlzd]{1,128}") {
            let lv = a.parse::<LogicVector>();
            prop_assert!(lv.is_ok());
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
        let v = LogicVector::from_int_value(5, 3);
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.width(), 3);
        assert_eq!(v, 5);
        let v = LogicVector::from_int_value(0, 128);
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.width(), 128);
        assert_eq!(v, 0);
        let v = LogicVector::from_int_value(5, 8);
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.width(), 8);
        assert_eq!(v, 5);
    }

    #[test]
    fn resize_smaller() {
        let mut v = LogicVector::with_width(5);
        assert_eq!(v.width(), 5);
        v.set_width(4);
        assert_eq!(v.width(), 4);
        v.set_width(3);
        assert_eq!(v.width(), 3);
        v.set_width(2);
        assert_eq!(v.width(), 2);
        v.set_width(1);
        assert_eq!(v.width(), 1);

        let mut v = LogicVector::from_int_value(31, 5).unwrap();
        assert_eq!(v.width(), 5);
        assert_eq!(v.as_u128(), Some(0b11111));
        v.set_width(4);
        assert_eq!(v.width(), 4);
        assert_eq!(v.as_u128(), Some(0b1111));
        v.set_width(3);
        assert_eq!(v.width(), 3);
        assert_eq!(v.as_u128(), Some(0b111));
        v.set_width(2);
        assert_eq!(v.width(), 2);
        assert_eq!(v.as_u128(), Some(0b11));
        v.set_width(1);
        assert_eq!(v.width(), 1);
        assert_eq!(v.as_u128(), Some(0b1));
    }

    #[test]
    fn resize_bigger() {
        let mut v = LogicVector::with_width(1);
        assert_eq!(v.width(), 1);
        v.set_width(2);
        assert_eq!(v.width(), 2);
        v.set_width(3);
        assert_eq!(v.width(), 3);
        v.set_width(4);
        assert_eq!(v.width(), 4);
        v.set_width(5);
        assert_eq!(v.width(), 5);

        let mut v = LogicVector::from_int_value(0, 1).unwrap();
        assert_eq!(v.width(), 1);
        assert_eq!(v, 0);
        v.resize(2, Ieee1164::_1);
        assert_eq!(v.width(), 2);
        assert_eq!(v, 0b10);
        v.resize(3, Ieee1164::_0);
        assert_eq!(v.width(), 3);
        assert_eq!(v, 0b010);
        v.resize(4, Ieee1164::_1);
        assert_eq!(v.width(), 4);
        assert_eq!(v, 0b1010);
        v.resize(5, Ieee1164::_0);
        assert_eq!(v.width(), 5);
        assert_eq!(v, 0b01010);
        v.resize(10, Ieee1164::_1);
        assert_eq!(v.width(), 10);
        assert_eq!(v, 0b1111101010);
    }

    #[test]
    fn add() {}

    #[test]
    fn to_string() {}
}
