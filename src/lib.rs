#![feature(nll, try_from, vec_remove_item)]

//! Logical is a digital network simulator. It is named after the german word "Logical" which
//! describes a puzzle that follows the rules of logic.
//!
//! You can build arbitrary complex networks, which then can be simulated. It supports Ieee1164
//! conform values, like strong and weak drives, uninitialized, high impedance and don't care logic.
//! For more information about these take a look at [`Ieee1164`] Type
//!
//! It is also possible to generate tracefiles in various formats, see the the [`dump`] module.
//!
//! # Usage
//!
//! This crate will be on [crates.io](https://crates.io/crates/logical) and can be used by adding
//! `logical` to your dependencies in your projects `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! logical = "0.2"
//! ```
//!
//! Afterwards you can use it in your 2018-rust-project
//!
//! ```rust
//! use logical;
//! ```
//!
//! # Example: connect one port to an other
//!
//! Normally you will connect one [`Port`] to one [`Signal`] as input and then connect an other port as
//! output to that same signal. On [`Updateable::update`] the value from the input will be transfered to
//! the output.
//!
//! ```rust
//! use logical::{Ieee1164, Port, Signal, Updateable};
//! use logical::direction::{Input, Output};
//!
//! let from = Port::<_, Output>::new(Ieee1164::_1);
//! let to = Port::<_, Input>::default();
//! let mut signal = Signal::new();
//!
//! signal.connect_as_input(&from);
//! signal.connect_as_output(&to);
//!
//! signal.update();
//!
//! assert_eq!(Ieee1164::_1, to.value());
//! ```
//!
//! # Example: multiple ports
//!
//! If you have more than one connector the value on the signal will be determined based on the
//! [`Resolve`] trait. In this case a high-impedance value will be overriden by the Strong zero
//! value and therefore result in 0.
//!
//! ```rust
//! use logical::{Ieee1164, Port, Signal, Updateable};
//! use logical::direction::{Input, Output};
//!
//! let from1 = Port::<_, Output>::new(Ieee1164::_Z);
//! let from2 = Port::<_, Output>::new(Ieee1164::_0);
//! let to = Port::<_, Input>::default();
//! let mut signal = Signal::new();
//!
//! signal.connect_as_input(&from1);
//! signal.connect_as_input(&from2);
//! signal.connect_as_output(&to);
//!
//! signal.update();
//!
//! assert_eq!(Ieee1164::_0, to.value());
//! ```

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
mod mac;
mod circuit;
pub mod dump;
mod logicbit;
pub(self) mod port;
mod signal;

pub mod models;

pub use self::circuit::Circuit;
pub use self::logicbit::{Ieee1164, Ieee1164Value, LogicVector, Resolve};
pub use self::port::Port;
pub use self::signal::Signal;

pub mod direction {
    pub use super::port::{Dir, InOut, Input, MaybeRead, MaybeWrite, Off, Output, PortDirection, Read, Write};
}

pub trait Updateable {
    fn update(&mut self);
}
