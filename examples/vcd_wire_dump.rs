use logical::dump::Vcd;
use logical::models::{
    gates::{Mux, XorGate},
    inputs::Switch,
    outputs::Led,
};
use logical::{circuit, signal, Circuit, Ieee1164, Signal};

fn main() {
    let mut val = Ieee1164::_1;

    let xor = XorGate::default();
    let mux = Mux::default();
    let mut input1 = Switch::new(val);
    let input2 = Switch::new(Ieee1164::_1);
    let mut mux_switch = Switch::new(Ieee1164::_0);
    let output = Led::default();

    let sig_input_signal = signal!(input1, xor.a);
    let sig_input_mux = signal!(input2, mux.a);
    let sig_mux_switch = signal!(mux_switch, mux.s);
    let sig_rec = signal!(mux.b, output, xor.z);
    let sig_mux_xor = signal!(mux.z, xor.b);

    let mut circuit = circuit!(
        xor,
        mux,
        sig_input_signal,
        sig_input_mux,
        sig_mux_switch,
        sig_rec,
        sig_mux_xor
    );

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
