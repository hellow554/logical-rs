use std::convert::TryInto;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

use super::InnerPort;

use crate::direction::{Dir, InOut, Input, MaybeRead, MaybeWrite, Output, PortDirection, Read, Write};
use crate::dump::IterValues;
use crate::port::portconnector::PortConnector;
use crate::Ieee1164;

#[derive(Debug, Clone)]
pub struct Port<T, D: PortDirection> {
    pub(crate) inner: Arc<InnerPort<T>>,
    _marker: PhantomData<D>,
}

impl<T, D: PortDirection> PartialEq for Port<T, D> {
    fn eq(&self, other: &Port<T, D>) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T, D: PortDirection> Eq for Port<T, D> {}

impl<T: Default, D: PortDirection> Default for Port<T, D> {
    fn default() -> Self {
        Port::new(T::default())
    }
}

impl<T, D: PortDirection> Port<T, D> {
    pub fn new(value: T) -> Self {
        Port {
            inner: Arc::new(InnerPort {
                value: RwLock::new(value),
            }),
            _marker: PhantomData,
        }
    }
}

impl<T, D: PortDirection> Port<T, D> {
    pub(crate) fn new_with_arc(arc: Arc<InnerPort<T>>) -> Self {
        Port {
            inner: arc,
            _marker: PhantomData,
        }
    }
}

impl<T, W> Port<T, Dir<Read, W>>
where
    T: Clone,
    W: MaybeWrite,
    Dir<Read, W>: PortDirection,
{
    pub fn value(&self) -> T {
        self.inner.value.read().unwrap().clone()
    }
}

impl<T, R> Port<T, Dir<R, Write>>
where
    R: MaybeRead,
    Dir<R, Write>: PortDirection,
{
    pub fn set_value(&mut self, value: T) {
        *self.inner.value.write().unwrap() = value;
    }
}

impl<T, D: PortDirection> Into<PortConnector<T, D::Opposite>> for Port<T, D> {
    fn into(self) -> PortConnector<T, <D as PortDirection>::Opposite> {
        PortConnector::new_with_weak(Arc::downgrade(&self.inner))
    }
}

impl<T> Into<PortConnector<T, Input>> for Port<T, InOut> {
    fn into(self) -> PortConnector<T, Input> {
        PortConnector::new_with_weak(Arc::downgrade(&self.inner))
    }
}

impl<T> Into<PortConnector<T, Output>> for Port<T, InOut> {
    fn into(self) -> PortConnector<T, Output> {
        PortConnector::new_with_weak(Arc::downgrade(&self.inner))
    }
}

impl<T, D> TryInto<PortConnector<T, Input>> for Port<T, D>
where
    D: PortDirection,
{
    type Error = ();

    fn try_into(self) -> Result<PortConnector<T, Input>, <Self as TryInto<PortConnector<T, Input>>>::Error> {
        if D::IS_OUTPUT || D::IS_INOUT {
            Ok(PortConnector::new_with_weak(Arc::downgrade(&self.inner)))
        } else {
            Err(())
        }
    }
}

impl<T, D> TryInto<PortConnector<T, Output>> for Port<T, D>
where
    D: PortDirection,
{
    type Error = ();

    fn try_into(self) -> Result<PortConnector<T, Output>, <Self as TryInto<PortConnector<T, Output>>>::Error> {
        if D::IS_INPUT || D::IS_INOUT {
            Ok(PortConnector::new_with_weak(Arc::downgrade(&self.inner)))
        } else {
            Err(())
        }
    }
}

impl<T, D> TryInto<PortConnector<T, Input>> for &Port<T, D>
where
    D: PortDirection,
{
    type Error = ();

    fn try_into(self) -> Result<PortConnector<T, Input>, <Self as TryInto<PortConnector<T, Input>>>::Error> {
        if D::IS_OUTPUT || D::IS_INOUT {
            Ok(PortConnector::new_with_weak(Arc::downgrade(&self.inner)))
        } else {
            Err(())
        }
    }
}

impl<T, D> TryInto<PortConnector<T, Output>> for &Port<T, D>
where
    D: PortDirection,
{
    type Error = ();

    fn try_into(self) -> Result<PortConnector<T, Output>, <Self as TryInto<PortConnector<T, Output>>>::Error> {
        if D::IS_INPUT || D::IS_INOUT {
            Ok(PortConnector::new_with_weak(Arc::downgrade(&self.inner)))
        } else {
            Err(())
        }
    }
}

impl<D> IterValues for Port<Ieee1164, D>
where
    D: PortDirection,
{
    fn iter_values<F>(&self, mut f: F)
    where
        F: FnMut(&Ieee1164),
    {
        f(&self.inner.value.read().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::{InOut, Input, Output};
    use crate::LogicVector;

    #[test]
    fn reading() {
        let s = Port::<_, Input>::new(3);
        assert_eq!(s.value(), 3);
    }

    #[test]
    fn set() {
        let mut s = Port::<_, Output>::default();
        s.set_value(Integer::new_with_value(3u8, 8));
        assert_eq!(*s.inner.value.read().unwrap(), 3u8);
    }

    #[test]
    fn reset() {
        let mut s = Port::<_, InOut>::default();
        s.set_value(Integer::new_with_value(5u8, 8));
        assert_eq!(s.value(), 5u8);
        s.set_value(Integer::new_with_value(6u8, 8));
        assert_eq!(s.value(), 6u8);
    }

    #[test]
    fn reset_before_reading() {
        let mut s = Port::<_, InOut>::default();
        s.set_value(Integer::new_with_value(4u8, 8));
        s.set_value(Integer::new_with_value(8u8, 8));
        assert_eq!(s.value(), 8u8);
    }

    //fn write_on_input_should_not_compile() {
    // ```compile_fail
    //     use logical::{Port, Integer, port::Output};
    //     let mut s = Port::<Integer, Output>::new();
    //     s.set_value(Integer::new_with_value(4u8, 8));
    //     assert_eq!(Integer::default(), s.value())
    // ```
    //}
}
