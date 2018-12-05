//! Lets try to build a full adder!

use std::thread::sleep;
use std::time::Duration;

use logical::models::{
    gates::{AndGate, OrGate, XorGate},
    inputs::Switch,
    outputs::Led,
};
use logical::{Circuit, Ieee1164, Ieee1164Value, Signal};

fn main() {
    //first halfadder
    let and1 = AndGate::default();
    let xor1 = XorGate::default();

    //second halfadder
    let and2 = AndGate::default();
    let xor2 = XorGate::default();

    //cout or
    let or = OrGate::default();

    //inputs
    let mut x = Switch::default();
    let mut y = Switch::default();
    let mut c = Switch::default();

    //outputs
    let cout = Led::default();
    let s = Led::default();

    //connections
    //from x to and and xor of ha1
    let mut s_x = Signal::<Ieee1164>::new();
    s_x.connect(&x.output).unwrap();
    s_x.connect(&and1.a).unwrap();
    s_x.connect(&xor1.a).unwrap();

    //from y to and and xor of ha1
    let mut s_y = Signal::new();
    s_y.connect(&y.output).unwrap();
    s_y.connect(&and1.b).unwrap();
    s_y.connect(&xor1.b).unwrap();

    //from and of ha1 to or
    let mut s_ha1_o = Signal::new();
    s_ha1_o.connect(&and1.z).unwrap();
    s_ha1_o.connect(&or.a).unwrap();

    //from from xor of ha1 to ha2
    let mut s_ha1_ha2 = Signal::new();
    s_ha1_ha2.connect(&xor1.z).unwrap();
    s_ha1_ha2.connect(&and2.a).unwrap();
    s_ha1_ha2.connect(&xor2.a).unwrap();

    //from cin to and2 and xor2
    let mut s_cin_ha2 = Signal::new();
    s_cin_ha2.connect(&c.output).unwrap();
    s_cin_ha2.connect(&and2.b).unwrap();
    s_cin_ha2.connect(&xor2.b).unwrap();

    //from and2 to xor
    let mut s_ha2_o = Signal::new();
    s_ha2_o.connect(&and2.z).unwrap();
    s_ha2_o.connect(&or.b).unwrap();

    //from or to cout
    let mut s_or_cout = Signal::new();
    s_or_cout.connect(&or.z).unwrap();
    s_or_cout.connect(&cout.input).unwrap();

    //from xor2 to s
    let mut s_ha_s = Signal::new();
    s_ha_s.connect(&xor2.z).unwrap();
    s_ha_s.connect(&s.input).unwrap();

    let mut circuit = Circuit::new();
    circuit.add_updater(&s_x);
    circuit.add_updater(&s_y);
    circuit.add_updater(&s_ha1_o);
    circuit.add_updater(&s_ha1_ha2);
    circuit.add_updater(&s_cin_ha2);
    circuit.add_updater(&s_ha2_o);
    circuit.add_updater(&s_or_cout);
    circuit.add_updater(&s_ha_s);
    circuit.add_updater(&and1);
    circuit.add_updater(&and2);
    circuit.add_updater(&xor1);
    circuit.add_updater(&xor2);
    circuit.add_updater(&or);

    const _0: Ieee1164 = Ieee1164::_0; // this helps to keep the
    const _1: Ieee1164 = Ieee1164::_1; // matrix clean and short

    const VALUES: [[Ieee1164; 5]; 8] = [
        [_0, _0, _0, _0, _0],
        [_0, _0, _1, _1, _0],
        [_0, _1, _0, _1, _0],
        [_0, _1, _1, _0, _1],
        [_1, _0, _0, _1, _0],
        [_1, _0, _1, _0, _1],
        [_1, _1, _0, _0, _1],
        [_1, _1, _1, _1, _1],
    ];

    for triple in VALUES.iter() {
        x.set_value(triple[0]);
        y.set_value(triple[1]);
        c.set_value(triple[2]);

        for _ in 0..3 {
            circuit.tick();
        }

        println!(
            "{} + {} + {} = {}{}",
            triple[0],
            triple[1],
            triple[2],
            cout.value(),
            s.value()
        );
        assert_eq!(triple[3], s.value());
        assert_eq!(triple[4], cout.value());
        sleep(Duration::from_secs(1));
    }
}
