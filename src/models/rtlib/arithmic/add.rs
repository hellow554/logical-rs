use crate::direction::{Input, Output};
use crate::{LogicVector, Port};

/// This models an actual adder that will add up both inputs.
///
/// This struct ensures that all inputs will always have the same length.
#[derive(Debug)]
pub struct Add {
    /// First input `Port`
    pub a: Port<LogicVector, Input>,
    /// Second input `Port`
    pub b: Port<LogicVector, Input>,
    /// Output `Port`, sum of [`Add::a`] and [`Add::b`]
    pub s: Port<LogicVector, Output>,
    _private: (),
}
