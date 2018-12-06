use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

mod private {
    /// Sealed trait which prevents others from defining their own `PortDirections`.
    pub trait Sealed {}
}

use self::private::Sealed;

/// Describes that a Port can be read
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Read;
/// Describes that a Port can be written
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Write;
/// Describes that the specific requirement (either read or write) is off
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Off;

/// A marker trait that says that the implementing struct can maybe read a value
pub trait MaybeRead {}
/// A marker trait that says that the implementing struct can maybe write a value
pub trait MaybeWrite {}

impl MaybeRead for Read {}
impl MaybeWrite for Write {}
impl MaybeRead for Off {}
impl MaybeWrite for Off {}

/// Trait for describing a `PortDirection`.
///
/// This is used for the [`Port`] struct so it can offer or
/// decline certain functions at compile time, e.g. reading an `Output` `Port` should not be
/// possible, but only writing to it.
///
/// The three possible states are described as [`Input`], [`Output`] and [`InOut`].
pub trait PortDirection: Default + Debug + PartialEq + Eq + Hash + Clone + Copy + Sealed {
    /// Specify the opposite of this `PortDirection`, e.g. `Output` for `Input` and vice-versa
    type Opposite: PortDirection;
    /// This Direction is [`Input`]
    const IS_INPUT: bool = false;
    /// This Direction is [`Output`]
    const IS_OUTPUT: bool = false;
    /// This Direction is [`InOut`]
    const IS_INOUT: bool = false;
}

/// This struct holds the possibility to either `Read` or `Write` (or both) to a `Port`. You can
/// specify that you port should be able to be read or to be written and use one of the directions.
/// The compile time guarantees from Rust will allow you to specify the exact direction and prevent
/// compiling instead of failing at runtime.
/// This comes with a cost of course, namely chaning the direction is quiet expensive if not
/// impossible at all, depending on the implementor.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Dir<R: MaybeRead, W: MaybeWrite>(PhantomData<(R, W)>);
impl<R: MaybeRead, W: MaybeWrite> Sealed for Dir<R, W> {}

impl PortDirection for InOut {
    type Opposite = Self;
    const IS_INOUT: bool = true;
}
impl PortDirection for Input {
    type Opposite = Dir<Off, Write>;
    const IS_INPUT: bool = true;
}
impl PortDirection for Output {
    type Opposite = Dir<Read, Off>;
    const IS_OUTPUT: bool = true;
}

/// Describes an `Input` direction. Reading is supported, writing is not.
pub type Input = Dir<Read, Off>;
/// Describes an `Output` direction. Writing is supported, reading is not.
pub type Output = Dir<Off, Write>;
/// Describes an `InOut` direction. Reading as well as writing is supported.
pub type InOut = Dir<Read, Write>;
