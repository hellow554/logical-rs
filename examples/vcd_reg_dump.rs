use logical::dump::Vcd;
use logical::{Ieee1164, LogicVector};

fn main() {
    let v = LogicVector::from_int_value(5, 8).unwrap();
    assert_eq!(v.width(), 8);
    assert_eq!(v, 5);

    let mut dumper = Vcd::new("VCD Example");
    let mut foo: LogicVector;

    foo = vec![Ieee1164::_Z; 16].into();
    dumper.serialize_logivector("foo", &foo);
    dumper.tick();

    foo = vec![Ieee1164::_U; 16].into();
    dumper.serialize_logivector("foo", &foo);
    dumper.tick();

    foo = vec![Ieee1164::_X; 16].into();
    dumper.serialize_logivector("foo", &foo);
    dumper.tick();

    foo = vec![Ieee1164::_W; 16].into();
    dumper.serialize_logivector("foo", &foo);
    dumper.tick();

    foo = vec![Ieee1164::_D; 16].into();
    dumper.serialize_logivector("foo", &foo);
    dumper.tick();

    foo = vec![Ieee1164::_1; 16].into();
    dumper.serialize_logivector("foo", &foo);
    dumper.tick();

    let one = LogicVector::from_int_value(1, 16).unwrap();
    for _ in 0..90 {
        foo = foo + &one;
        dumper.serialize_logivector("foo", &foo);
        dumper.tick();
    }

    dumper.dump("/home/marcel/b.vcd").unwrap();
}
