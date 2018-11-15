use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

mod private {
    pub trait Sealed {}
}

use self::private::Sealed;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Read;
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Write;
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Off;

pub trait MaybeRead {}
pub trait MaybeWrite {}

impl MaybeRead for Read {}
impl MaybeWrite for Write {}
impl MaybeRead for Off {}
impl MaybeWrite for Off {}

pub trait PortDirection: Default + Debug + PartialEq + Eq + Hash + Clone + Copy + Sealed {
    type Opposite: PortDirection;
    const IS_INPUT: bool = false;
    const IS_OUTPUT: bool = false;
    const IS_INOUT: bool = false;
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Dir<R: MaybeRead, W: MaybeWrite>(PhantomData<(R, W)>);

impl<R: MaybeRead, W: MaybeWrite> Sealed for Dir<R, W> {}

impl PortDirection for Dir<Read, Write> {
    type Opposite = Self;
    const IS_INOUT: bool = true;
}
impl PortDirection for Dir<Read, Off> {
    type Opposite = Dir<Off, Write>;
    const IS_INPUT: bool = true;
}
impl PortDirection for Dir<Off, Write> {
    type Opposite = Dir<Read, Off>;
    const IS_OUTPUT: bool = true;
}

pub type Input = Dir<Read, Off>;
pub type Output = Dir<Off, Write>;
pub type InOut = Dir<Read, Write>;
