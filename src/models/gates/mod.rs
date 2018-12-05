mod mux;
mod tri;

pub use self::mux::Mux;
pub use self::tri::TriBuffer;

use crate::direction::{Input, Output};

use crate::dump::IterPorts;
use crate::{Ieee1164, Ieee1164Value, Port, Updateable};

macro_rules! create_simple_1i1o_gate {
    ($name:ident, $func:ident) => {
        #[derive(Debug, Default, Clone)] //TODO: remove Clone!
        pub struct $name {
            pub a: Port<Ieee1164, Input>,
            pub z: Port<Ieee1164, Output>,
            _private: (),
        }

        impl Updateable for $name {
            fn update(&mut self) {
                self.z.replace($func(self.a.value()));
            }
        }

        impl IterPorts for $name {
            fn iter_ports<F>(&self, mut f: F)
            where
                F: FnMut(&str, &Port<Ieee1164, Output>),
            {
                f("a", &Port::new_with_arc(self.a.inner.clone()));
                f("z", &Port::new_with_arc(self.z.inner.clone()));
            }
        }
    };
}

macro_rules! create_simple_2i1o_gate {
    ($name:ident, $func:ident) => {
        #[derive(Debug, Default, Clone)] //TODO: remove Clone!
        pub struct $name {
            /// First input port
            pub a: Port<Ieee1164, Input>,
            /// Second input port
            pub b: Port<Ieee1164, Input>,
            /// Output port
            pub z: Port<Ieee1164, Output>,
            _private: (),
        }

        impl Updateable for $name {
            fn update(&mut self) {
                self.z.replace($func(self.a.value(), self.b.value()));
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

fn buf(a: Ieee1164) -> Ieee1164 {
    a
}
create_simple_1i1o_gate!(Buffer, buf);

fn inv(a: Ieee1164) -> Ieee1164 {
    !a
}
create_simple_1i1o_gate!(Inverter, inv);

fn weak_buf(a: Ieee1164) -> Ieee1164 {
    match a {
        Ieee1164::_U | Ieee1164::_X | Ieee1164::_W | Ieee1164::_D => Ieee1164::_W,
        Ieee1164::_1 | Ieee1164::_H => Ieee1164::_H,
        Ieee1164::_0 | Ieee1164::_L => Ieee1164::_L,
        Ieee1164::_Z => Ieee1164::_Z,
    }
}
create_simple_1i1o_gate!(WeakBuffer, weak_buf);

fn weak_inv(a: Ieee1164) -> Ieee1164 {
    match a {
        Ieee1164::_U | Ieee1164::_X | Ieee1164::_W | Ieee1164::_D => Ieee1164::_W,
        Ieee1164::_1 | Ieee1164::_H => Ieee1164::_L,
        Ieee1164::_0 | Ieee1164::_L => Ieee1164::_H,
        Ieee1164::_Z => Ieee1164::_Z,
    }
}
create_simple_1i1o_gate!(WeakInverter, weak_inv);
