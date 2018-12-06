//! This module provides generic inputs.

use crate::direction::Output;
use crate::{Ieee1164, Port};

/// This struct can be used as an user-output.
///
/// This can be used in an graphical environment for example to display the state of a signal.
pub type Switch = Port<Ieee1164, Output>;
