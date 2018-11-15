use crate::logicbit::Ieee1164;
use crate::port::Port;
use crate::port::{Input, Output};
use crate::Updateable;
use std::fmt;

#[derive(Clone)]
pub struct OrGate {
    pub a: Port<Ieee1164, Input>,
    pub b: Port<Ieee1164, Input>,
    pub z: Port<Ieee1164, Output>,
    _private: (),
}

impl fmt::Debug for OrGate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OrGate {{ a: {:?}, b: {:?}, z: {:?} }}", self.a, self.b, self.z)
    }
}

impl Updateable for OrGate {
    fn update(&mut self) {
        self.z.set_value(self.a.value() | self.b.value());
    }
}

impl Default for OrGate {
    fn default() -> Self {
        Self::new()
    }
}

impl OrGate {
    pub fn new() -> Self {
        Self {
            a: Port::default(),
            b: Port::default(),
            z: Port::default(),
            _private: (),
        }
    }
}
