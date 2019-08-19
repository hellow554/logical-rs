use crate::direction::{Input, Output};
use crate::{LogicVector, Port, Updateable};

/// Computes the two's complement of the applied value.
#[derive(Debug)]
pub struct TwosComplement {
    /// Input `Port`
    pub a: Port<LogicVector, Input>,
    /// Output `Port`
    pub y: Port<LogicVector, Output>,
}

impl Updateable for TwosComplement {
    fn update(&mut self) -> bool {
        let old_value = self.y.inner.value.read().unwrap().clone();
        let a = self.a.value();
        self.y.with_value_mut(|y| *y = (!a).incr());
        old_value != *self.y.inner.value.read().unwrap()
    }
}
