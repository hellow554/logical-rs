use std::convert::TryInto;
use std::iter::FromIterator;
use std::sync::{Arc, RwLock, Weak};

use crate::direction::Dir;
use crate::direction::{Input, MaybeRead, MaybeWrite, Output, PortDirection};
use crate::logicbit::Resolve;
use crate::port::PortConnector;
use crate::{Port, Updateable};

#[derive(Debug)]
struct InnerSignal<T> {
    input_ports: RwLock<Vec<PortConnector<T, Input>>>,
    output_ports: RwLock<Vec<PortConnector<T, Output>>>,
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

/// A `Signal` is the connection between two ore more [`Port`]s. It is used to transfer data
/// from one `Port~ to another.
///
/// A cloned Signal is equal to an other Signal. You can clone them as often as you like. To
/// actually create a new `Signal` use the [`Default::default`] constructor.

/// To transfer values from one `Port` to another you have to connect those `Port`s to the "same"
/// `Signal`.
///
/// # Example
///
/// ```rust
/// use logical::{Ieee1164, Port, Signal, Updateable};
/// use logical::direction::{Input, Output};
///
/// let from = Port::<_, Output>::new(Ieee1164::_1);
/// let to = Port::<_, Input>::default();
/// let mut signal = Signal::default();
///
/// signal.connect(&from);
/// signal.connect(&to);
/// assert_eq!(Ieee1164::default(), to.value());
///
/// signal.update();
/// assert_eq!(Ieee1164::_1, to.value());
/// ```
///
/// You can explicitly disconnect a signal and it will no longer have any effect on other `Port`s.
///
/// ```rust
/// use logical::{Ieee1164, Port, Signal, Updateable};
/// use logical::direction::{Input, Output};
///
/// let mut from = Port::<_, Output>::new(Ieee1164::_1);
/// let to = Port::<_, Input>::default();
/// let mut signal = Signal::default();
///
/// signal.connect(&from);
/// signal.connect(&to);
/// signal.update();
/// assert_eq!(Ieee1164::_1, to.value());
///
/// signal.disconnect(&from);
/// from.replace(Ieee1164::_0);
/// signal.update();
/// assert_eq!(Ieee1164::_1, to.value());
/// ```
///
/// If you like you can construct a Signal from an [`Iterator`], but because of the type system,
/// they all have to be the same direction.
///
/// ```rust
/// use logical::{Ieee1164, Port, Signal, Updateable};
/// use logical::direction::{Input, InOut, Output};
///
/// let port_a = Port::<Ieee1164, Output>::default();
/// let port_b = Port::<_, Output>::default();
/// let port_c = Port::<_, Output>::default();
/// let port_d = Port::<_, Input>::default();
///
/// let mut signal: Signal<_> = [port_a, port_b, port_c].iter().cloned().collect();
/// signal.connect(&port_d);
/// ```
//TOOD: can we circumvent this restriction?
#[derive(Debug, Clone)]
pub struct Signal<T> {
    inner: Arc<InnerSignal<T>>,
}

impl<T> Default for Signal<T> {
    fn default() -> Self {
        Signal {
            inner: Arc::new(InnerSignal {
                input_ports: RwLock::new(vec![]),
                output_ports: RwLock::new(vec![]),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionError {
    AlreadyConnected,
    MismatchWidth(usize, usize),
}

impl<T> Signal<T> {
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

    /// Connects a [`Port`] to this `Signal`. A `Signal` is only connected once to the same `Port`.
    /// If you try to connect it more than once you will get an [`ConnectionError::AlreadyConnected`]
    /// error.
    ///
    /// For an example see the [`Signal`] documentation.
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

        // opposite order because portconnectors use the opposite direction to do something
        // (e.g. read from a an output port).
        if D::IS_OUTPUT || D::IS_INOUT {
            let connector = port.try_into().unwrap();
            if !in_guard.contains(&connector) {
                in_guard.push(connector);
            }
        }
        if D::IS_INPUT || D::IS_INOUT {
            let connector = port.try_into().unwrap();
            if !out_guard.contains(&connector) {
                out_guard.push(connector);
            }
        }

        Ok(())
    }

    /// Disconnects a [`Port`] from this `Signal`
    ///
    /// For an example see the [`Signal`] documentation.
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
                guard.retain(PortConnector::is_valid);
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
    use crate::{Ieee1164, Port};

    #[test]
    fn signal_no_value_no_port() {
        let mut s = Signal::<Ieee1164>::default();
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
        let mut s = Signal::default();

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
        let mut s = Signal::default();
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
        let mut s = Signal::default();

        s.connect(&p1).unwrap();
        s.connect(&p2).unwrap();
        s.connect(&o).unwrap();
        s.update();
        assert_eq!(val_a.resolve(val_b), o.value());
    }

    #[test]
    fn signal_port_out_of_scope() {
        let mut s = Signal::default();

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
        let mut s = Signal::default();

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
