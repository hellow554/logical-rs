use crate::logicbit::Ieee1164;
use crate::port::Port;
use crate::port::{Input, Output};
use crate::Updateable;

#[derive(Debug, Clone)]
pub struct AndGate {
    pub a: Port<Ieee1164, Input>,
    pub b: Port<Ieee1164, Input>,
    pub z: Port<Ieee1164, Output>,
    _private: (),
}

impl Updateable for AndGate {
    fn update(&mut self) {
        self.z.set_value(self.a.value() & self.b.value());
    }
}

impl Default for AndGate {
    fn default() -> Self {
        Self::new()
    }
}

impl AndGate {
    pub fn new() -> Self {
        Self {
            a: Port::default(),
            b: Port::default(),
            z: Port::default(),
            _private: (),
        }
    }
}
