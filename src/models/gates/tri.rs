use crate::direction::{Input, Output};
use crate::dump::IterPorts;
use crate::{Ieee1164, Port, Updateable};

/// A `Tristate-buffer` can be used if you need multiple signals to drive a single [`Signal`].
///
/// This is often used if multiple signals must drive a single one, e.g. a s.c. bus. If [`TriBuffer::s`] is
/// driven high, [`TriBuffer::z`] will be the value of [`TriBuffer::a`]. If it's driven low, it will
/// output [`'Ieee1164::_Z`]. In other cases it will be [`Ieee1164::_X`].
#[derive(Debug, Default, Clone)]
pub struct TriBuffer {
    /// Input `Port`
    pub a: Port<Ieee1164, Input>,
    /// Enable `Port`
    pub s: Port<Ieee1164, Input>,
    /// Output `Port`
    pub z: Port<Ieee1164, Output>,
    _private: (),
}

impl Updateable for TriBuffer {
    fn update(&mut self) -> bool {
        let new_value = if self.s.value().is_1H() {
            self.a.value()
        } else if self.s.value().is_0L() {
            Ieee1164::_Z
        } else {
            Ieee1164::_X
        };
        let old_value =  self.z.replace(new_value);

        old_value != new_value
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
