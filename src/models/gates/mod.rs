//! This module provides logic gates that work with [`Ieee1164`], e.g. [`AndGate`], [`OrGate`],
//! [`Buffer`], [`Mux`], etc.

mod mux;
mod tri;

pub use self::mux::Mux;
pub use self::tri::TriBuffer;

use crate::direction::{Input, Output};

use crate::dump::IterPorts;
use crate::{Ieee1164, Port, Updateable};

macro_rules! create_simple_1i1o_gate {
    ($name:ident, $func:ident, $doc:tt) => {
        #[derive(Debug, Default, Clone)] //TODO: remove Clone!
        #[doc = $doc]
        pub struct $name {
            /// Input `Port`
            pub a: Port<Ieee1164, Input>,
            /// Output `Port`
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
    ($name:ident, $func:ident, $doc:tt) => {
        #[derive(Debug, Default, Clone)] //TODO: remove Clone!
        #[doc = $doc]
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
create_simple_2i1o_gate!(
    AndGate,
    and,
    "A simple 2-input AND Gate. It performs the logical AND \
     operation on both inputs and outputs that value."
);

fn nand(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    !(a & b)
}
create_simple_2i1o_gate!(
    NandGate,
    nand,
    "A simple 2-input NAND Gate. It performs the logical NAND \
     operation on both inputs and outputs that value."
);

fn or(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    a | b
}
create_simple_2i1o_gate!(
    OrGate,
    or,
    "A simple 2-input OR Gate. It performs the logical OR \
     operation on both inputs and outputs that value."
);

fn nor(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    !(a | b)
}
create_simple_2i1o_gate!(
    NorGate,
    nor,
    "A simple 2-input NOR Gate. It performs the logical NOR \
     operation on both inputs and outputs that value."
);

fn xor(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    a ^ b
}
create_simple_2i1o_gate!(
    XorGate,
    xor,
    "A simple 2-input XOR Gate. It performs the logical XOR \
     operation on both inputs and outputs that value."
);

fn xnor(a: Ieee1164, b: Ieee1164) -> Ieee1164 {
    !(a ^ b)
}
create_simple_2i1o_gate!(
    XnorGate,
    xnor,
    "A simple 2-input XNOR Gate. It performs the logical XNOR \
     operation on both inputs and outputs that value."
);

fn buf(a: Ieee1164) -> Ieee1164 {
    a
}
create_simple_1i1o_gate!(
    Buffer,
    buf,
    "A simple Buffer Gate. It outputs the same value as its \
     input."
);

fn inv(a: Ieee1164) -> Ieee1164 {
    !a
}
create_simple_1i1o_gate!(Inverter, inv, "A simple Not Gate. It outputs the negation of its input");

fn weak_buf(a: Ieee1164) -> Ieee1164 {
    match a {
        Ieee1164::_U | Ieee1164::_X | Ieee1164::_W | Ieee1164::_D => Ieee1164::_W,
        Ieee1164::_1 | Ieee1164::_H => Ieee1164::_H,
        Ieee1164::_0 | Ieee1164::_L => Ieee1164::_L,
        Ieee1164::_Z => Ieee1164::_Z,
    }
}
create_simple_1i1o_gate!(
    WeakBuffer,
    weak_buf,
    "A buffer which transforms Strong values into Weak \
     values."
);

fn weak_inv(a: Ieee1164) -> Ieee1164 {
    match a {
        Ieee1164::_U | Ieee1164::_X | Ieee1164::_W | Ieee1164::_D => Ieee1164::_W,
        Ieee1164::_1 | Ieee1164::_H => Ieee1164::_L,
        Ieee1164::_0 | Ieee1164::_L => Ieee1164::_H,
        Ieee1164::_Z => Ieee1164::_Z,
    }
}
create_simple_1i1o_gate!(
    WeakInverter,
    weak_inv,
    "A buffer which transforms Strong values into Weak\
     values and inverts them."
);
