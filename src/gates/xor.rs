use crate::logicbit::Ieee1164;
use crate::port::Port;
use crate::port::{Input, Output};
use crate::Updateable;

#[derive(Debug, Clone)]
pub struct XorGate {
    pub a: Port<Ieee1164, Input>,
    pub b: Port<Ieee1164, Input>,
    pub z: Port<Ieee1164, Output>,
    _private: (),
}

impl Updateable for XorGate {
    fn update(&mut self) {
        self.z.set_value(self.a.value() ^ self.b.value());
    }
}

impl Default for XorGate {
    fn default() -> Self {
        Self::new()
    }
}

impl XorGate {
    pub fn new() -> Self {
        Self {
            a: Port::default(),
            b: Port::default(),
            z: Port::default(),
            _private: (),
        }
    }
}
