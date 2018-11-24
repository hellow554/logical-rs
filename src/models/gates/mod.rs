mod mux;
pub use self::mux::Mux;

use crate::direction::{Input, Output};

use crate::dump::IterPorts;
use crate::{Ieee1164, Port, Updateable};

macro_rules! create_simple_2i1o_gate {
    ($name:ident, $func:ident) => {
        #[derive(Debug, Default, Clone)] //TODO: remove Clone!
        pub struct $name {
            pub a: Port<Ieee1164, Input>,
            pub b: Port<Ieee1164, Input>,
            pub z: Port<Ieee1164, Output>,
            _private: (),
        }

        impl Updateable for $name {
            fn update(&mut self) {
                self.z.set_value($func(self.a.value(), self.b.value()));
            }
        }

        impl IterPorts for $name {
            fn iter_ports<F>(&self, mut f: F)
            where
                F: FnMut(&str, &Port<Ieee1164, Output>),
            {
                f("a", &Port::new_with_arc(self.a.inner.clone()));
                f("b", &Port::new_with_arc(self.b.inner.clone()));
                f("z", &Port::new_with_arc(self.z.inner.clone()));
            }
        }
    };
}

fn and(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    a & b
}
create_simple_2i1o_gate!(AndGate, and);

fn nand(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    !(a & b)
}
create_simple_2i1o_gate!(NandGate, nand);

fn or(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    a | b
}
create_simple_2i1o_gate!(OrGate, or);

fn nor(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    !(a | b)
}
create_simple_2i1o_gate!(NorGate, nor);

fn xor(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    a ^ b
}
create_simple_2i1o_gate!(XorGate, xor);

fn xnor(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    !(a ^ b)
}
create_simple_2i1o_gate!(XnorGate, xnor);
