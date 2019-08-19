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
        let new_value = (!self.a.value()).incr();
        let old_value = self.y.replace(new_value.clone());
        old_value != new_value
    }
}
