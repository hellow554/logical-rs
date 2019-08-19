use crate::Updateable;

/// A `Circuit` is a combination of connected logic elements
///
/// A `Circuit` holds references of [`Updateable`](Updateable) structs
/// ([`Port`](crate::port::Port), ...), which are updated on every call of
/// tick.
#[derive(Default)]
pub struct Circuit {
    updater: Vec<Box<dyn Updateable>>,
}

impl Circuit {
    /// The update tick function
    /// This function propagates the logic values by one Updateable element.
    ///
    /// Returns `true` as long as at least one element's output value has changed in the circuit.
    ///
    /// ```
    /// let mut circuit = Circuit::default();
    /// /* Configure updaters */
    ///
    /// let mut clock_cycles = 0_u32;
    /// while circuit.tick() && clock_cycles < 100 { // 100 is a arbitrary value, to avoid looping if we have an oscillating circuit
    ///     clock_cycles += 1;
    /// }
    /// ```
    pub fn tick(&mut self) -> bool {
        self.updater.iter_mut().fold(false, |acc, u| acc | u.update())
    }

    /// Add an [`Updateable`](Updateable) to the `Circuit`
    ///
    /// ```
    /// let mut sig = Signal::default();
    /// // Configure signal here
    ///
    /// let or = OrGate::default(); // Gates do not need to be mutable
    /// // Connect gate here
    ///
    /// let mut circuit = Circuit::default();
    /// circuit.add_updater(&sig);
    /// circuit.add_updater(&or);
    /// ```
    pub fn add_updater<T: Updateable + Clone + 'static>(&mut self, updater: &T) {
        self.updater.push(Box::new(updater.clone()))
    }
}
