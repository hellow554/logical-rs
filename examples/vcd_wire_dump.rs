use logical::dump::Vcd;
use logical::models::{
    gates::{Mux, XorGate},
    inputs::Switch,
    outputs::Led,
};
use logical::{Circuit, Ieee1164, Ieee1164Value, Signal};

fn main() {
    let mut val = Ieee1164::Strong(Ieee1164Value::One);

    let xor = XorGate::default();
    let mux = Mux::default();
    let mut input1 = Switch::new_with_value(val);
    let input2 = Switch::new_with_value(Ieee1164::Strong(Ieee1164Value::One));
    let mut mux_switch = Switch::new_with_value(Ieee1164::Strong(Ieee1164Value::Zero));
    let output = Led::default();

    let mut sig_input_signal = Signal::new();
    sig_input_signal.connect(&input1.output).unwrap();
    sig_input_signal.connect(&xor.a).unwrap();

    let mut sig_input_mux = Signal::new();
    sig_input_mux.connect(&input2.output).unwrap();
    sig_input_mux.connect(&mux.a).unwrap();

    let mut sig_mux_switch = Signal::new();
    sig_mux_switch.connect(&mux_switch.output).unwrap();
    sig_mux_switch.connect(&mux.s).unwrap();

    let mut sig_rec = Signal::new();
    sig_rec.connect(&mux.b).unwrap();
    sig_rec.connect(&output.input).unwrap();
    sig_rec.connect(&xor.z).unwrap();

    let mut sig_mux_xor = Signal::new();
    sig_mux_xor.connect(&mux.z).unwrap();
    sig_mux_xor.connect(&xor.b).unwrap();

    let mut circuit = Circuit::new();
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
    mux_switch.set_value(Ieee1164::Strong(Ieee1164Value::One));
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
            input1.set_value(val);
        }
    }

    dumper.dump("/home/marcel/a.vcd").unwrap();
}
