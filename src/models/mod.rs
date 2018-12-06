//! This module will contain different models that can be used to perform certain calculations. E.g.
//! Gates, like [`AndGate`], [`Mux`], [`Switch`], but also complex gates, like ['Alu`] or [`Rom1kx8`].

pub mod gates;
pub mod inputs;
pub mod outputs;
pub mod rtlib;

#[allow(unused)]
use self::{
    gates::{AndGate, Mux},
    inputs::Switch,
    rtlib::memory::Rom1kx8,
};
