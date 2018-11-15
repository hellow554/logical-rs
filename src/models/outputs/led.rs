use crate::port::Input;
use crate::Port;

#[derive(Debug, Default)]
pub struct Led<T> {
    pub input: Port<T, Input>,
    _private: (),
}

impl<T: Clone> Led<T> {
    pub fn value(&self) -> T {
        self.input.value()
    }
}
