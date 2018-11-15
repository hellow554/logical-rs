use std::sync::RwLock;

mod portconnector;
mod portdirection;
mod pport;

pub(crate) use self::portconnector::PortConnector;

pub use self::portdirection::{Dir, InOut, Input, MaybeRead, MaybeWrite, Off, Output, PortDirection, Read, Write};
pub use self::pport::Port;

#[derive(Debug)]
pub(self) struct InnerPort<T> {
    value: RwLock<T>,
}
