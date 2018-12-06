use crate::direction::Output;
use crate::{LogicVector, Port};

#[derive(Debug)]
pub struct VectorInput {
    pub port: Port<LogicVector, Output>,
    _private: (),
}

impl VectorInput {
    pub fn with_width(width: u8) -> Self {
        Self {
            port: Port::new(LogicVector::with_width(width)),
            _private: (),
        }
    }

    /// Creates this input with the given [`LogicVector`] as inner value.
    pub fn with_logicvector(lv: LogicVector) -> Self {
        Self {
            port: Port::new(lv),
            _private: (),
        }
    }
}
