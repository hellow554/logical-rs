//! Lets try to build a full adder!

use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use logical::gates::{AndGate, OrGate, XorGate};
use logical::models::inputs::Switch;
use logical::models::outputs::Led;
use logical::{Ieee1164, Ieee1164Value, Reactor, Signal};

fn main() -> Result<(), Box<Error>> {
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
    s_x.connect_as_input(&x.output);
    s_x.connect_as_output(&and1.a);
    s_x.connect_as_output(&xor1.a);

    //from y to and and xor of ha1
    let mut s_y = Signal::new();
    s_y.connect_as_input(&y.output);
    s_y.connect_as_output(&and1.b);
    s_y.connect_as_output(&xor1.b);

    //from and of ha1 to or
    let mut s_ha1_o = Signal::new();
    s_ha1_o.connect_as_input(&and1.z);
    s_ha1_o.connect_as_output(&or.a);

    //from from xor of ha1 to ha2
    let mut s_ha1_ha2 = Signal::new();
    s_ha1_ha2.connect_as_input(&xor1.z);
    s_ha1_ha2.connect_as_output(&and2.a);
    s_ha1_ha2.connect_as_output(&xor2.a);

    //from cin to and2 and xor2
    let mut s_cin_ha2 = Signal::new();
    s_cin_ha2.connect_as_input(&c.output);
    s_cin_ha2.connect_as_output(&and2.b);
    s_cin_ha2.connect_as_output(&xor2.b);

    //from and2 to xor
    let mut s_ha2_o = Signal::new();
    s_ha2_o.connect_as_input(&and2.z);
    s_ha2_o.connect_as_output(&or.b);

    //from or to cout
    let mut s_or_cout = Signal::new();
    s_or_cout.connect_as_input(&or.z);
    s_or_cout.connect_as_output(&cout.input);

    //from xor2 to s
    let mut s_ha_s = Signal::new();
    s_ha_s.connect_as_input(&xor2.z);
    s_ha_s.connect_as_output(&s.input);

    let mut r = Reactor::new();

    r.add_updater(&s_x);
    r.add_updater(&s_y);
    r.add_updater(&s_ha1_o);
    r.add_updater(&s_ha1_ha2);
    r.add_updater(&s_cin_ha2);
    r.add_updater(&s_ha2_o);
    r.add_updater(&s_or_cout);
    r.add_updater(&s_ha_s);
    r.add_updater(&and1);
    r.add_updater(&and2);
    r.add_updater(&xor1);
    r.add_updater(&xor2);
    r.add_updater(&or);

    const _0: Ieee1164 = Ieee1164::Strong(Ieee1164Value::Zero);
    const _1: Ieee1164 = Ieee1164::Strong(Ieee1164Value::One);

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
            r.tick();
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

    Ok(())
}
