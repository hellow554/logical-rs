use crate::direction::Output;
use crate::{LogicVector, Port};

/// This struct can be used as a user-defined input, e.g. in a graphical environment.
#[derive(Debug)]
pub struct VectorInput {
    /// The output port
    pub port: Port<LogicVector, Output>,
    _private: (),
}

impl VectorInput {
    /// Create this struct with a defines width for the inner [`LogicVector`]
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
