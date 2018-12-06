//! This module provides outputs for `Ieee1164` values.

use crate::direction::Input;
use crate::{Ieee1164, Port};

/// This struct can be used as an user-output.
///
/// This can be used in an graphical environment for example to display the state of a signal.
pub type Led = Port<Ieee1164, Input>;
