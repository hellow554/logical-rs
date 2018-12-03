use crate::{Ieee1164, Ieee1164Value};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SanityChecked {
    MoreThanOne(u8),
    NoOne(u8),
    OneAboveWidth(u8),
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Masks {
    _U: u128,
    _X: u128,
    _1: u128,
    _0: u128,
    _W: u128,
    _H: u128,
    _L: u128,
    _Z: u128,
    _D: u128,
}

impl Masks {
    pub fn get(&self, index: u8) -> Ieee1164 {
        for m in self.iter() {
            if m.1 >> index & 1 == 1 {
                return m.0;
            }
        }
        panic!("No bit set on {}", index)
    }

    pub fn set(&mut self, index: u8, value: Ieee1164) {
        for m in self.iter_mut() {
            if m.0 == value {
                *m.1 |= 1 << index;
            } else {
                *m.1 &= !(1 << index);
            }
        }
    }

    pub fn sanity_check(&self, width: u8) -> Result<(), SanityChecked> {
        for d in 0..128 {
            let mut has_one = false;
            for mask in self {
                if (mask.1 >> d) & 1 == 1 {
                    if has_one {
                        return Err(SanityChecked::MoreThanOne(d));
                    }
                    if d > width {
                        return Err(SanityChecked::OneAboveWidth(d));
                    }
                    has_one = true;
                }
            }
            if d < width && !has_one {
                return Err(SanityChecked::NoOne(d));
            }
        }

        Ok(())
    }

    pub fn iter(&self) -> Iter {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut {
        self.into_iter()
    }
}

impl Index<Ieee1164> for Masks {
    type Output = u128;

    fn index(&self, index: Ieee1164) -> &u128 {
        match index {
            Ieee1164::Uninitialized => &self._U,
            Ieee1164::Strong(Ieee1164Value::Unknown) => &self._X,
            Ieee1164::Strong(Ieee1164Value::One) => &self._1,
            Ieee1164::Strong(Ieee1164Value::Zero) => &self._0,
            Ieee1164::Weak(Ieee1164Value::Unknown) => &self._W,
            Ieee1164::Weak(Ieee1164Value::One) => &self._H,
            Ieee1164::Weak(Ieee1164Value::Zero) => &self._L,
            Ieee1164::HighImpedance => &self._Z,
            Ieee1164::DontCare => &self._D,
        }
    }
}

impl IndexMut<Ieee1164> for Masks {
    fn index_mut(&mut self, index: Ieee1164) -> &mut u128 {
        match index {
            Ieee1164::Uninitialized => &mut self._U,
            Ieee1164::Strong(Ieee1164Value::Unknown) => &mut self._X,
            Ieee1164::Strong(Ieee1164Value::One) => &mut self._1,
            Ieee1164::Strong(Ieee1164Value::Zero) => &mut self._0,
            Ieee1164::Weak(Ieee1164Value::Unknown) => &mut self._W,
            Ieee1164::Weak(Ieee1164Value::One) => &mut self._H,
            Ieee1164::Weak(Ieee1164Value::Zero) => &mut self._L,
            Ieee1164::HighImpedance => &mut self._Z,
            Ieee1164::DontCare => &mut self._D,
        }
    }
}

impl<'a> IntoIterator for &'a Masks {
    type Item = (Ieee1164, &'a u128);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        Iter { mask: self, pos: 0 }
    }
}

impl<'a> IntoIterator for &'a mut Masks {
    type Item = (Ieee1164, &'a mut u128);
    type IntoIter = IterMut<'a>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        IterMut { mask: self, pos: 0 }
    }
}

pub struct Iter<'a> {
    mask: &'a Masks,
    pos: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Ieee1164, &'a u128);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.pos < 9 {
            let res = match self.pos {
                0 => (Ieee1164::_U, &self.mask._U),
                1 => (Ieee1164::_X, &self.mask._X),
                2 => (Ieee1164::_1, &self.mask._1),
                3 => (Ieee1164::_0, &self.mask._0),
                4 => (Ieee1164::_W, &self.mask._W),
                5 => (Ieee1164::_H, &self.mask._H),
                6 => (Ieee1164::_L, &self.mask._L),
                7 => (Ieee1164::_Z, &self.mask._Z),
                8 => (Ieee1164::_D, &self.mask._D),
                _ => unreachable!(),
            };
            self.pos += 1;
            Some(res)
        } else {
            None
        }
    }
}

pub struct IterMut<'a> {
    mask: &'a mut Masks,
    pos: u8,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (Ieee1164, &'a mut u128);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.pos < 9 {
            let res = match self.pos {
                0 => (Ieee1164::_U, &mut self.mask._U),
                1 => (Ieee1164::_X, &mut self.mask._X),
                2 => (Ieee1164::_1, &mut self.mask._1),
                3 => (Ieee1164::_0, &mut self.mask._0),
                4 => (Ieee1164::_W, &mut self.mask._W),
                5 => (Ieee1164::_H, &mut self.mask._H),
                6 => (Ieee1164::_L, &mut self.mask._L),
                7 => (Ieee1164::_Z, &mut self.mask._Z),
                8 => (Ieee1164::_D, &mut self.mask._D),
                _ => unreachable!(),
            };
            self.pos += 1;
            Some(unsafe { std::mem::transmute(res) }) //FIXME: Get rid of this unsafe!
        } else {
            None
        }
    }
}
