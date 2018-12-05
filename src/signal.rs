use std::convert::TryInto;
use std::sync::{Arc, RwLock, Weak};

use crate::direction::{Input, Output, PortDirection};
use crate::logicbit::Resolve;
use crate::port::PortConnector;
use crate::{Port, Updateable};

#[derive(Debug, Clone)]
pub struct Signal<T> {
    inner: Arc<InnerSignal<T>>,
}

#[derive(Debug, Clone)]
pub(crate) struct WeakSignal<T> {
    inner: Weak<InnerSignal<T>>,
}

impl<T> Default for WeakSignal<T> {
    fn default() -> Self {
        WeakSignal { inner: Weak::new() }
    }
}

impl<T> WeakSignal<T> {
    pub(crate) fn is_strong(&self) -> bool {
        self.inner.upgrade().is_some()
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionError {
    AlreadyConnected,
    MismatchWidth(usize, usize),
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

    //    pub fn can_connect<D>(&self, port: &Port<T, D>)
    //    where
    //        D: PortDirection,
    //    {
    //        let in_guard = self.inner.input_ports.read().unwrap();
    //        let out_guard = self.inner.input_ports.read().unwrap();
    //
    //        (if let Some(i) = in_guard.iter().next() {
    //            i.can_connect(&port)
    //        } && if let Some(o) = out_guard.iter().next() {
    //            o.can_connect(&port);
    //        })
    //    }

    pub fn connect<D>(&mut self, port: &Port<T, D>) -> Result<(), ConnectionError>
    where
        D: PortDirection,
    {
        if port.is_connected() {
            return Err(ConnectionError::AlreadyConnected);
        }
        // TODO: check length

        let mut in_guard = self.inner.input_ports.write().unwrap();
        let mut out_guard = self.inner.output_ports.write().unwrap();

        //reversed order because our vectors contains portconnectors and have the opposite direction
        if D::IS_INPUT || D::IS_INOUT {
            let connector = port.try_into().unwrap();
            if !out_guard.contains(&connector) {
                out_guard.push(connector);
            }
        }
        if D::IS_OUTPUT || D::IS_INOUT {
            let connector = port.try_into().unwrap();
            if !in_guard.contains(&connector) {
                in_guard.push(connector);
            }
        }

        Ok(())
    }

    pub fn disconnect<D>(&mut self, port: &Port<T, D>)
    where
        D: PortDirection,
    {
        let mut in_guard = self.inner.input_ports.write().unwrap();
        let mut out_guard = self.inner.output_ports.write().unwrap();
        if D::IS_OUTPUT || D::IS_INOUT {
            let connector = port.try_into().unwrap();
            in_guard.remove_item(&connector);
        }
        if D::IS_INPUT || D::IS_INOUT {
            let connector = port.try_into().unwrap();
            out_guard.remove_item(&connector);
        }
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
        s.connect(&port).unwrap();
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

        s.connect(&i).unwrap();
        s.connect(&p).unwrap();
        assert_eq!(Ieee1164::default(), i.value());
        s.update();
        assert_eq!(Ieee1164::Uninitialized, i.value());
        for _ in 0..1_000 {
            s.update();
            assert_eq!(Ieee1164::Uninitialized, i.value());
        }

        let val = Ieee1164::_1;
        p.replace(val);
        s.update();
        assert_eq!(val, i.value());

        for _ in 0..1_000 {
            s.update();
            assert_eq!(val, i.value());
        }
    }

    #[test]
    fn signal_after_disconnect() {
        let val_a = Ieee1164::_1;
        let val_b = Ieee1164::_0;
        let mut p = Port::<_, InOut>::new(val_a);
        let o = Port::<_, Input>::default();
        let mut s = Signal::new();
        s.connect(&p).unwrap();
        s.connect(&o).unwrap();
        s.update();
        assert_eq!(val_a, o.value());

        s.disconnect(&p);
        p.replace(val_b);
        s.update();
        assert_ne!(val_b, o.value());
    }

    #[test]
    fn signal_multiple_ports() {
        let val_a = Ieee1164::_H;
        let val_b = Ieee1164::_0;
        let p1 = Port::<_, InOut>::new(val_a);
        let p2 = Port::<_, InOut>::new(val_b);
        let o = Port::<_, Input>::new(Ieee1164::_D);
        let mut s = Signal::new();

        s.connect(&p1).unwrap();
        s.connect(&p2).unwrap();
        s.connect(&o).unwrap();
        s.update();
        assert_eq!(val_a.resolve(val_b), o.value());
    }

    #[test]
    fn signal_port_out_of_scope() {
        let mut s = Signal::new();

        let val_a = Ieee1164::_1;
        let val_b = Ieee1164::_0;

        let p1 = Port::<_, Output>::new(val_a);
        let o = Port::<_, Input>::default();
        s.connect(&p1).unwrap();
        s.connect(&o).unwrap();

        {
            let p2 = Port::<_, Output>::new(val_b);
            s.connect(&p2).unwrap();
            s.update();
            assert_eq!(val_a.resolve(val_b), o.value());
        }

        s.update();
        assert_eq!(val_a, o.value());
    }

    #[test]
    fn disallow_multiple_connects() {
        let mut s = Signal::new();

        let val = Ieee1164::_1;
        let p = Port::<_, InOut>::new(val);
        s.connect(&p).unwrap();
        s.connect(&p).unwrap();
        assert_eq!(1, s.inner.input_ports.read().unwrap().len());

        s.connect(&p).unwrap();
        s.connect(&p).unwrap();
        assert_eq!(1, s.inner.output_ports.read().unwrap().len());
    }
}
