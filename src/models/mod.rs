//! This module will contain different models that can be used to perform certain calculations. E.g.
//! Gates, like [`AndGate`](crate::models::gates::AndGate), [`Mux`](crate::models::gates::Mux),
//! [`Switch`](crate::models::inputs::Switch), but also complex gates, like
//! [`Rom1kx8`](crate::models::rtlib::memory::rom::Rom1kx8).

pub mod gates;
pub mod inputs;
pub mod outputs;
pub mod rtlib;

#[allow(unused_imports)]
use self::{
    gates::{AndGate, Mux},
    inputs::Switch,
    rtlib::memory::Rom1kx8,
};
