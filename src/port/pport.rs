use std::convert::TryInto;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

use super::InnerPort;

use crate::direction::{Dir, InOut, Input, MaybeRead, MaybeWrite, Output, PortDirection, Read, Write};
use crate::dump::IterValues;
use crate::port::portconnector::PortConnector;
use crate::Ieee1164;
use std::sync::Weak;

#[allow(unused)]
use crate::{models::gates::AndGate, Signal};

/// A `Port` is the connection between a model (e.g. an [`AndGate`]) and a signal.
///
/// A `Port` has a Direction, either [`Input`], [`Output`] or [`InOut`]. Depending on the direction
/// you can read, write or do both on the `Port`. You can't for example read on an [`Output`] port,
/// but only write to it.
///
/// You can `clone` a `Port` as often as you want, or even [`drop`], it doesn't affect other ports.
#[derive(Debug, Clone)]
pub struct Port<T, D: PortDirection> {
    pub(crate) inner: Arc<InnerPort<T>>,
    _marker: PhantomData<D>,
}

impl<T: Default, D: PortDirection> Default for Port<T, D> {
    fn default() -> Self {
        Port::new(T::default())
    }
}

impl<T, D: PortDirection> Port<T, D> {
    /// Create a new Port with an inner value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use logical::Port;
    /// use logical::direction::Input;
    ///
    /// let port = Port::<u32, Input>::new(3u32);
    /// assert_eq!(3u32, port.value());
    /// ```
    ///
    /// You can't however read the value of an [`Output`] `Port`.
    ///
    /// ```rust,compile_fail,E0599
    /// use logical::Port;
    /// use logical::direction::Output;
    ///
    /// let port = Port::<u32, Output>::new(3u32);
    /// assert_eq!(3u32, port.value());
    /// ```
    ///
    /// But of course, you can do both an an [`InOut`] `Port`.
    ///
    /// ```rust
    /// use logical::Port;
    /// use logical::direction::InOut;
    ///
    /// let mut port = Port::<u32, InOut>::new(3u32);
    /// assert_eq!(3u32, port.value());
    /// port.replace(5);
    /// assert_eq!(5, port.value());
    /// ```
    pub fn new(value: T) -> Self {
        Port {
            inner: Arc::new(InnerPort {
                value: RwLock::new(value),
                signal: Weak::new(),
            }),
            _marker: PhantomData,
        }
    }

    /// Create a Port with an already exiting `InnerPort`. This is only useful, if you have to
    /// convert a Port from one Direction to another and can't use the `TryFrom` trait.
    /// Otherwise please you clone!!
    pub(crate) fn new_with_arc(arc: Arc<InnerPort<T>>) -> Self {
        Port {
            inner: arc,
            _marker: PhantomData,
        }
    }
}

impl<T, D> Port<T, D>
where
    D: PortDirection,
{
    pub(crate) fn _connect(&mut self, _signal: WeakSignal<T>) {
        //FIXME
        //std::mem::replace(&mut self.inner.signal, signal);
    }

    /// Returns whether this `Port` is connected to a [`Signal`].
    ///
    /// ```rust
    /// # use logical::{Ieee1164, Port, Signal};
    /// # use logical::direction::Output;
    /// let port = Port::<_, Output>::new(Ieee1164::_U);
    /// assert!(!port.is_connected());
    ///
    /// let mut signal = Signal::default();
    /// signal.connect(&port);
    /// //assert!(port.is_connected());
    /// ```
    // FIXME!
    pub fn is_connected(&self) -> bool {
        self.inner.signal.upgrade().is_some()
    }
}

impl<T, W> Port<T, Dir<Read, W>>
where
    T: Clone,
    W: MaybeWrite,
    Dir<Read, W>: PortDirection,
{
    /// Returns a copy of the inner value. `Clone` is needed, because on how the values are
    /// internally stored. If `T` is not `Clone` use [`Port::with_value`].
    ///
    /// ```rust
    /// # use logical::Port;
    /// # use logical::direction::Input;
    /// let port = Port::<_, Input>::new(5u32);
    /// assert_eq!(5, port.value());
    /// ```
    pub fn value(&self) -> T {
        self.inner.value.read().unwrap().clone()
    }
}

impl<T, W> Port<T, Dir<Read, W>>
where
    W: MaybeWrite,
    Dir<Read, W>: PortDirection,
{
    /// Accepts a [`FnOnce`] which accepts `&T` and executes it with the inner value.
    /// This function is useful, if `T` does not implement `Clone`.
    ///
    /// ```rust
    /// # use logical::Port;
    /// # use logical::direction::Input;
    /// let port = Port::<_, Input>::new(5u32);
    /// port.with_value(|value| assert_eq!(&5, value));
    /// ```
    pub fn with_value<F: FnOnce(&T)>(&self, f: F) {
        f(&self.inner.value.read().unwrap());
    }
}

impl<T, R> Port<T, Dir<R, Write>>
where
    R: MaybeRead,
    Dir<R, Write>: PortDirection,
{
    /// Replaces the internal value with `value` and returns the old value.
    ///
    /// If you intend to modify the inner value, use `with_value_mut` instead.
    ///
    /// ```rust
    /// # use logical::Port;
    /// # use logical::direction::Output;
    /// let mut port = Port::<_, Output>::new(5u32);
    /// port.replace(9u32);
    /// ```
    pub fn replace(&mut self, value: T) -> T {
        std::mem::replace(&mut self.inner.value.write().unwrap(), value)
    }

    /// Accepts a `FnOnce` which accepts a `&mut T`, so you can modify the inner values, instead of
    /// replacing it.
    ///
    /// ```rust
    /// # use logical::Port;
    /// # use logical::direction::Output;
    /// let mut port = Port::<_, Output>::new(String::from("ABC"));
    /// port.with_value_mut(|value| {
    ///     value.push('D');
    ///     assert_eq!("ABCD", value);
    /// });
    /// ```
    pub fn with_value_mut<F: FnOnce(&mut T)>(&mut self, f: F) {
        f(&mut self.inner.value.write().unwrap());
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

impl<T, D: PortDirection> PartialEq for Port<T, D> {
    fn eq(&self, other: &Port<T, D>) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T, D: PortDirection> Eq for Port<T, D> {}

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

//pub trait CanConnect {
//    fn can_connect(&self, other: &Self) -> bool;
//}
//
//impl<T: CanConnect, D: PortDirection> CanConnect for Port<T, D> {
//    fn can_connect(&self, other: &Self) -> bool {
//        self.inner
//            .value
//            .read()
//            .unwrap()
//            .can_connect(&other.inner.value.read().unwrap())
//    }
//}

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
        s.replace(LogicVector::from_int_value(3, 8));
        //assert_eq!(*s.inner.value.read().unwrap(), 3);
    }

    #[test]
    fn reset() {
        let mut s = Port::<_, InOut>::default();
        s.replace(LogicVector::from_int_value(5, 8));
        //assert_eq!(s.value(), 5);
        s.replace(LogicVector::from_int_value(6, 8));
        //assert_eq!(s.value(), 6);
    }

    #[test]
    fn reset_before_reading() {
        let mut s = Port::<_, InOut>::default();
        s.replace(LogicVector::from_int_value(4, 8));
        s.replace(LogicVector::from_int_value(8, 8));
        //assert_eq!(s.value(), 8);
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
