use std::sync::{Arc, RwLock};

use crate::logicbit::Resolve;

use crate::direction::{Dir, Input, MaybeRead, MaybeWrite, Output, Read, Write};

use crate::port::PortConnector;
use crate::port::PortDirection;
use crate::{Port, Updateable};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct Signal<T> {
    inner: Arc<InnerSignal<T>>,
}

#[derive(Debug)]
struct InnerSignal<T> {
    input_ports: RwLock<Vec<PortConnector<T, Input>>>,
    output_ports: RwLock<Vec<PortConnector<T, Output>>>,
}

impl<T> Default for Signal<T> {
    fn default() -> Self {
        Signal::new()
    }
}

impl<T> Signal<T> {
    pub fn new() -> Self {
        Signal {
            inner: Arc::new(InnerSignal {
                input_ports: RwLock::new(vec![]),
                output_ports: RwLock::new(vec![]),
            }),
        }
    }

    pub fn connect_as_input<R>(&mut self, port: &Port<T, Dir<R, Write>>)
    where
        R: MaybeRead,
        Dir<R, Write>: PortDirection,
    {
        let connector = port.try_into().unwrap(); //TODO: how can get rid of `try_into` and use `into` because we know, that this is safe!
        let mut guard = self.inner.input_ports.write().unwrap();
        if !guard.contains(&connector) {
            guard.push(connector);
        }
    }

    pub fn connect_as_output<W>(&mut self, port: &Port<T, Dir<Read, W>>)
    where
        W: MaybeWrite,
        Dir<Read, W>: PortDirection,
    {
        let connector = port.try_into().unwrap();
        let mut guard = self.inner.output_ports.write().unwrap();
        if !guard.contains(&connector) {
            guard.push(connector);
        }
    }

    pub fn disconnect_input<R>(&mut self, port: &Port<T, Dir<R, Write>>)
    where
        R: MaybeRead,
        Dir<R, Write>: PortDirection,
    {
        let connector = port.try_into().unwrap();
        self.inner.input_ports.write().unwrap().remove_item(&connector);
    }

    pub fn disconnect_output<W>(&mut self, port: &Port<T, Dir<Read, W>>)
    where
        W: MaybeWrite,
        Dir<Read, W>: PortDirection,
    {
        let connector = port.try_into().unwrap();
        self.inner.output_ports.write().unwrap().remove_item(&connector);
    }

    fn remove_expired_portconnector(&mut self) {
        macro_rules! filter {
            ($vec:expr) => {
                let mut guard = $vec.write().unwrap();
                let v = std::mem::replace(&mut *guard, vec![]);
                *guard = v.into_iter().filter(PortConnector::is_valid).collect()
            };
        };

        filter!(self.inner.input_ports);
        filter!(self.inner.output_ports);
    }
}

impl<T> Updateable for Signal<T>
where
    for<'a> &'a T: Resolve<&'a T, Output = T>,
    T: Clone + std::fmt::Debug,
{
    fn update(&mut self) {
        self.remove_expired_portconnector();

        let in_guard = self.inner.input_ports.write().unwrap();

        let mut iter = in_guard.iter();

        let first_port = loop {
            if let Some(first) = iter.next() {
                if first.is_valid() {
                    break Some(first);
                }
            } else {
                break None;
            }
        };

        if let Some(first) = first_port {
            //we hold a read guard, so nobody can mutate our in/inout list, so we are free to use unwrap here
            let r = iter
                .filter_map(|pc| pc.value())
                .fold(first.value().unwrap(), |e, s| e.resolve(&s));

            self.inner
                .output_ports
                .write()
                .unwrap()
                .iter_mut()
                .for_each(|p| p.set_value(r.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::InOut;
    use crate::{Ieee1164, Ieee1164Value, Port};

    #[test]
    fn signal_no_value_no_port() {
        let mut s = Signal::<Ieee1164>::new();
        let port = Port::<_, Input>::default();
        s.connect_as_output(&port);
        assert_eq!(Ieee1164::default(), port.value());
        s.update();
        assert_eq!(Ieee1164::default(), port.value());
        for _ in 0..1_000 {
            s.update();
            assert_eq!(Ieee1164::default(), port.value());
        }
    }

    #[test]
    fn signal_single_port() {
        let i = Port::<_, Input>::default();
        let mut p = Port::<_, Output>::default();
        let mut s = Signal::new();

        s.connect_as_output(&i);
        s.connect_as_input(&p);
        assert_eq!(Ieee1164::default(), i.value());
        s.update();
        assert_eq!(Ieee1164::Uninitialized, i.value());
        for _ in 0..1_000 {
            s.update();
            assert_eq!(Ieee1164::Uninitialized, i.value());
        }

        let val = Ieee1164::Strong(Ieee1164Value::One);
        p.set_value(val);
        s.update();
        assert_eq!(val, i.value());

        for _ in 0..1_000 {
            s.update();
            assert_eq!(val, i.value());
        }
    }

    #[test]
    fn signal_after_remove() {
        let val_a = Ieee1164::Strong(Ieee1164Value::One);
        let val_b = Ieee1164::Strong(Ieee1164Value::Zero);
        let mut p = Port::<_, InOut>::new(val_a);
        let o = Port::<_, Input>::default();
        let mut s = Signal::new();
        s.connect_as_input(&p);
        s.connect_as_output(&o);
        s.update();
        assert_eq!(val_a, o.value());

        s.disconnect_input(&p);
        p.set_value(val_b);
        s.update();
        assert_ne!(val_b, o.value());
    }

    #[test]
    fn signal_multiple_ports() {
        let val_a = Ieee1164::Weak(Ieee1164Value::One);
        let val_b = Ieee1164::Strong(Ieee1164Value::Zero);
        let p1 = Port::<_, InOut>::new(val_a);
        let p2 = Port::<_, InOut>::new(val_b);
        let o = Port::<_, Input>::new(Ieee1164::DontCare);
        let mut s = Signal::new();

        s.connect_as_input(&p1);
        s.connect_as_input(&p2);
        s.connect_as_output(&o);
        s.update();
        assert_eq!(val_a.resolve(val_b), o.value());
    }

    #[test]
    fn signal_port_out_of_scope() {
        let mut s = Signal::new();

        let val_a = Ieee1164::Strong(Ieee1164Value::One);
        let val_b = Ieee1164::Strong(Ieee1164Value::Zero);

        let p1 = Port::<_, InOut>::new(val_a);
        let o = Port::<_, Input>::default();
        s.connect_as_input(&p1);
        s.connect_as_output(&o);

        {
            let p2 = Port::<_, InOut>::new(val_b);
            s.connect_as_input(&p2);
            s.update();
            assert_eq!(val_a.resolve(val_b), o.value());
        }

        s.update();
        assert_eq!(val_a, o.value());
    }

    #[test]
    fn disallow_multiple_connects() {
        let mut s = Signal::new();

        let val = Ieee1164::Strong(Ieee1164Value::One);
        let p = Port::<_, InOut>::new(val);
        s.connect_as_input(&p);
        s.connect_as_input(&p);
        assert_eq!(1, s.inner.input_ports.read().unwrap().len());

        s.connect_as_output(&p);
        s.connect_as_output(&p);
        assert_eq!(1, s.inner.output_ports.read().unwrap().len());
    }
}
