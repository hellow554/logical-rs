use crate::direction::{Input, Output};
use crate::dump::IterPorts;
use crate::Port;
use crate::Updateable;
use crate::{Ieee1164, Ieee1164Value};

#[derive(Debug, Default, Clone)]
pub struct TriBuffer {
    pub a: Port<Ieee1164, Input>,
    pub s: Port<Ieee1164, Input>,
    pub z: Port<Ieee1164, Output>,
    _private: (),
}

impl Updateable for TriBuffer {
    fn update(&mut self) {
        self.z.set_value(if self.s.value().is_1H() {
            self.a.value()
        } else if self.s.value().is_0L() {
            Ieee1164::HighImpedance
        } else {
            Ieee1164::Strong(Ieee1164Value::Unknown)
        });
    }
}

impl IterPorts for TriBuffer {
    fn iter_ports<F>(&self, mut f: F)
    where
        F: FnMut(&str, &Port<Ieee1164, Output>),
    {
        f("a", &Port::new_with_arc(self.a.inner.clone()));
        f("s", &Port::new_with_arc(self.s.inner.clone()));
        f("z", &Port::new_with_arc(self.z.inner.clone()));
    }
}
