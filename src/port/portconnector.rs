use std::fmt;
use std::marker::PhantomData;
use std::sync::Weak;

use super::portdirection::{Dir, MaybeRead, MaybeWrite, Read, Write};
use super::{portdirection::PortDirection, InnerPort, Port};

#[derive(Clone)]
pub(crate) struct PortConnector<T, D: PortDirection> {
    inner: Weak<InnerPort<T>>,
    _marker: PhantomData<D>,
}

impl<T: fmt::Debug, D: PortDirection> fmt::Debug for PortConnector<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PortConnector {{ value: {:?} }} ", self.to_port(),)
    }
}

impl<T, D: PortDirection> PartialEq for PortConnector<T, D> {
    fn eq(&self, other: &PortConnector<T, D>) -> bool {
        match (self.to_port(), other.to_port()) {
            (Some(ref a), Some(ref b)) => a.eq(b),
            // even when both ports are not upgradeable does not mean, that they are equal!
            _ => false,
        }
    }
}

impl<T, D: PortDirection> PortConnector<T, D> {
    pub(super) fn new_with_weak(weak: Weak<InnerPort<T>>) -> Self {
        PortConnector {
            inner: weak,
            _marker: PhantomData,
        }
    }
}

impl<T, D: PortDirection> PortConnector<T, D> {
    fn to_port(&self) -> Option<Port<T, D>> {
        self.inner.upgrade().map(Port::new_with_arc)
    }

    pub fn is_valid(&self) -> bool {
        self.inner.upgrade().is_some()
    }
}

impl<T, W> PortConnector<T, Dir<Read, W>>
where
    T: Clone,
    W: MaybeWrite,
    Dir<Read, W>: PortDirection,
{
    pub fn value(&self) -> Option<T> {
        self.inner.upgrade().map(|i| i.value.read().unwrap().clone())
    }
}

impl<T, R> PortConnector<T, Dir<R, Write>>
where
    R: MaybeRead,
    Dir<R, Write>: PortDirection,
{
    pub fn set_value(&mut self, value: T) {
        if let Some(port) = self.to_port() {
            *port.inner.value.write().unwrap() = value;
        }
    }
}
