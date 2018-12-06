use logical::dump::Vcd;
use logical::models::{
    gates::{Mux, XorGate},
    inputs::Switch,
    outputs::Led,
};
use logical::{Circuit, Ieee1164, Signal};

fn main() {
    let mut val = Ieee1164::_1;

    let xor = XorGate::default();
    let mux = Mux::default();
    let mut input1 = Switch::new(val);
    let input2 = Switch::new(Ieee1164::_1);
    let mut mux_switch = Switch::new(Ieee1164::_0);
    let output = Led::default();

    let mut sig_input_signal = Signal::default();
    sig_input_signal.connect(&input1).unwrap();
    sig_input_signal.connect(&xor.a).unwrap();

    let mut sig_input_mux = Signal::default();
    sig_input_mux.connect(&input2).unwrap();
    sig_input_mux.connect(&mux.a).unwrap();

    let mut sig_mux_switch = Signal::default();
    sig_mux_switch.connect(&mux_switch).unwrap();
    sig_mux_switch.connect(&mux.s).unwrap();

    let mut sig_rec = Signal::default();
    sig_rec.connect(&mux.b).unwrap();
    sig_rec.connect(&output).unwrap();
    sig_rec.connect(&xor.z).unwrap();

    let mut sig_mux_xor = Signal::default();
    sig_mux_xor.connect(&mux.z).unwrap();
    sig_mux_xor.connect(&xor.b).unwrap();

    let mut circuit = Circuit::default();
    circuit.add_updater(&xor);
    circuit.add_updater(&mux);
    circuit.add_updater(&sig_input_signal);
    circuit.add_updater(&sig_input_mux);
    circuit.add_updater(&sig_mux_switch);
    circuit.add_updater(&sig_rec);
    circuit.add_updater(&sig_mux_xor);

    circuit.tick();
    circuit.tick();
    circuit.tick();
    mux_switch.replace(Ieee1164::_1);
    circuit.tick();

    let mut dumper = Vcd::new("VCD Example");

    for i in 0..90 {
        dumper.serialize_ports(&xor);
        circuit.tick();
        circuit.tick();
        dumper.tick();
        dumper.tick();
        if i % 20 == 0 {
            val = !val;
            input1.replace(val);
        }
    }

    dumper.dump("/home/marcel/a.vcd").unwrap();
}
