use crate::direction::{Input, Output};
use crate::logicbit::mask_from_width;
use crate::{Ieee1164, LogicVector, Port, Updateable};

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

impl Updateable for Add {
    fn update(&mut self) {
        let a = self.a.value();
        let b = self.b.value();
        self.s.with_value_mut(|v| match (a.as_u128(), b.as_u128()) {
            (Some(a), Some(b)) => v
                .replace_with_int(a.wrapping_add(b) & mask_from_width(v.width()))
                .unwrap(),
            _ => v.set_all_to(Ieee1164::_U),
        });
    }
}
