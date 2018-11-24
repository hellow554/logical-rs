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
    sig_input_signal.connect_as_input(&input1.output);
    sig_input_signal.connect_as_output(&xor.a);

    let mut sig_input_mux = Signal::new();
    sig_input_mux.connect_as_input(&input2.output);
    sig_input_mux.connect_as_output(&mux.a);

    let mut sig_mux_switch = Signal::new();
    sig_mux_switch.connect_as_input(&mux_switch.output);
    sig_mux_switch.connect_as_output(&mux.s);

    let mut sig_rec = Signal::new();
    sig_rec.connect_as_output(&mux.b);
    sig_rec.connect_as_output(&output.input);
    sig_rec.connect_as_input(&xor.z);

    let mut sig_mux_xor = Signal::new();
    sig_mux_xor.connect_as_input(&mux.z);
    sig_mux_xor.connect_as_output(&xor.b);

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
