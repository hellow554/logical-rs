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
//! logical = "0.1"
//! ```
//!
//! Afterwards you can use it in your 2018-rust-project
//!
//! ```rust,ignore
//! use logical
//! ```
//!
//! # Example: connect one port to an other
//!
//! ```rust
//! use logical::{Ieee1164, Port, Signal, Updateable};
//! use logical::direction::{Input, Output};
//!
//! let from = Port::<_, Output>::new(Ieee1164::from('1'));
//! let to = Port::<_, Input>::default();
//! let mut signal = Signal::new();
//!
//! signal.connect_as_input(&from);
//! signal.connect_as_output(&to);
//!
//! signal.update();
//!
//! assert_eq!(Ieee1164::from('1'), to.value());
//! ```
//!

#[macro_use]
mod mac;
pub mod dump;
mod integer;
mod logicbit;
pub(self) mod port;
mod reactor;
mod signal;

pub mod gates;
pub mod models;

pub use self::integer::Integer;
pub use self::logicbit::{Ieee1164, Ieee1164Value, Resolve};
pub use self::port::Port;
pub use self::reactor::Reactor;
pub use self::signal::Signal;

pub trait Updateable {
    fn update(&mut self);
}

pub mod direction {
    pub use super::port::{Dir, InOut, Input, MaybeRead, MaybeWrite, Off, Output, PortDirection, Read, Write};
}

pub type IeeeSignal = Signal<Ieee1164>;
pub type IntegerSignal = Signal<Integer>;
