use crate::direction::{Input, Output};
use crate::dump::IterPorts;
use crate::{Ieee1164, Port, Updateable};

/// A Multiplexer can be seen as an `if`-statement. If [`Mux::s`] is driven low, [`Mux::a`] is
/// outputted on [`Mux::z`], if [`Mux::s`] is driven high [`Mux::b`] will be outputted.
/// If `Mux::s` has any other value [`Ieee1164::_X`] will be returned.
///
/// # Example
///
/// ```rust
/// use logical::{Circuit, Ieee1164, Port, Signal, Updateable};
/// use logical::direction::{Input, Output};
/// use logical::models::gates::Mux;
///
/// let mux = Mux::default();
/// let port_a = Port::<_, Output>::new(Ieee1164::_H);
/// let port_b = Port::<_, Output>::new(Ieee1164::_L);
/// let mut port_s = Port::<_, Output>::default();
/// let port_z = Port::<_, Input>::default();
///
/// let mut sig_a = Signal::default();
/// sig_a.connect(&port_a);
/// sig_a.connect(&mux.a);
///
/// let mut sig_b = Signal::default();
/// sig_b.connect(&port_b);
/// sig_b.connect(&mux.b);
///
/// let mut sig_s = Signal::default();
/// sig_s.connect(&port_s);
/// sig_s.connect(&mux.s);
///
/// let mut sig_z = Signal::default();
/// sig_z.connect(&port_z);
/// sig_z.connect(&mux.z);
///
/// let mut circuit = Circuit::default();
/// circuit.add_updater(&sig_a);
/// circuit.add_updater(&sig_b);
/// circuit.add_updater(&sig_s);
/// circuit.add_updater(&mux);
/// circuit.add_updater(&sig_z);
///
/// circuit.tick();
/// assert_eq!(Ieee1164::_X, port_z.value());
///
/// port_s.replace(Ieee1164::_0);
/// circuit.tick();
/// assert_eq!(Ieee1164::_H, port_z.value());
///
/// port_s.replace(Ieee1164::_1);
/// circuit.tick();
/// assert_eq!(Ieee1164::_L, port_z.value());
/// ```
#[derive(Debug, Default, Clone)]
pub struct Mux {
    /// First input `Port`
    pub a: Port<Ieee1164, Input>,
    /// Second input `Port`
    pub b: Port<Ieee1164, Input>,
    /// Selector `Port`
    pub s: Port<Ieee1164, Input>,
    /// Output `Port`
    pub z: Port<Ieee1164, Output>,
    _private: (),
}

impl Updateable for Mux {
    fn update(&mut self) -> bool {
        //let old_value = self.z.inner;
        let old_value = self.z.inner.value.read().unwrap().clone();
        self.z.replace(if self.s.value().is_1H() {
            self.b.value()
        } else if self.s.value().is_0L() {
            self.a.value()
        } else {
            Ieee1164::_X
        });

        old_value != *self.z.inner.value.read().unwrap()
    }
}

impl IterPorts for Mux {
    fn iter_ports<F>(&self, mut f: F)
    where
        F: FnMut(&str, &Port<Ieee1164, Output>),
    {
        f("a", &Port::new_with_arc(self.a.inner.clone()));
        f("b", &Port::new_with_arc(self.b.inner.clone()));
        f("s", &Port::new_with_arc(self.s.inner.clone()));
        f("z", &Port::new_with_arc(self.z.inner.clone()));
    }
}
