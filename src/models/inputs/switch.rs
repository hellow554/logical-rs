use crate::port::Output;
use crate::Port;

#[derive(Debug, Default)]
pub struct Switch<T> {
    pub output: Port<T, Output>,
    _private: (),
}

impl<T> Switch<T> {
    pub fn new_with_value(value: T) -> Self {
        Self {
            output: Port::new(value),
            _private: (),
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.output.replace(value);
    }
}
