use std::fmt;
use std::iter::FromIterator;

use crate::direction::{Input, Output};
use crate::{Ieee1164, LogicVector, Port, Updateable};

/// This struct represents a Read-only-memory with a size of 1kB (1024 bytes).
///
/// This rom consists of a 10-bit address line, a 8-bit data line, a chip-select and an
/// output-enable line which can be used to control the data output to be [`Ieee1164::_Z`]
/// (high-impedance) instead of outputting a value.
///
/// Althought it's a `Rom`, you can modify the values inside programmatically, but not with `Signals`.
///
/// # Examples
///
/// The easiest way to create a `Rom`, is using the [`FromIterator`] trait.
/// ```rust
/// use logical::models::rtlib::memory::Rom1kx8;
///
/// let rom: Rom1kx8 = (0..=255).cycle().collect();
/// ```
///
/// The `FromIterator` implementation takes exactly 1024 bytes out of the stream and panics if there
/// are less bytes available.
pub struct Rom1kx8 {
    /// The memory that holds the values stored inside this Rom.
    pub memory: [u8; 1024],
    /// Determines the position inside the `Rom` where the data to read from.
    pub addr: Port<LogicVector, Input>,
    /// Data port which contains the data addressed by the `addr` port.
    pub data: Port<LogicVector, Output>,
    /// Active-low chip-select pin. If pulled high, the output will be [`Ieee1164::_Z`].
    pub n_chip_select: Port<Ieee1164, Input>,
    /// Active-low output enable pin. If pulled high, the output will be [`Ieee1164::_Z`].
    pub n_output_enable: Port<Ieee1164, Input>,
    _private: (),
}

impl FromIterator<u8> for Rom1kx8 {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut mem = [0; 1024];
        let mut bytes_read = 0;
        for (m, v) in mem.iter_mut().zip(iter.into_iter()).take(1024) {
            bytes_read += 1;
            *m = v;
        }
        assert_eq!(1024, bytes_read);

        Self {
            memory: mem,
            addr: Port::new(LogicVector::with_width(10)),
            data: Port::new(LogicVector::with_width(8)),
            n_chip_select: Port::default(),
            n_output_enable: Port::default(),
            _private: (),
        }
    }
}

impl Default for Rom1kx8 {
    fn default() -> Self {
        Self {
            memory: [0; 1024],
            addr: Port::new(LogicVector::with_width(10)),
            data: Port::new(LogicVector::with_width(8)),
            n_chip_select: Port::default(),
            n_output_enable: Port::default(),
            _private: (),
        }
    }
}

impl fmt::Debug for Rom1kx8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rom1kx8 {{ addr: {:?}, data: {:?}, n_chip_select {:?}, n_output_enable {:?} }}",
            self.addr, self.data, self.n_chip_select, self.n_output_enable
        )
    }
}

impl Updateable for Rom1kx8 {
    fn update(&mut self) {
        println!("ROM Update");
        let ncs = self.n_chip_select.value();
        let noe = self.n_output_enable.value();
        let data = if let Some(addr) = self.addr.value().as_u128() {
            Some(u128::from(self.memory[addr as usize]))
        } else {
            None
        };

        println!("{} {} {:?}", ncs, noe, data);

        self.data.with_value_mut(|f| {
            if ncs.is_UXZ() || noe.is_UXZ() {
                f.set_all_to(Ieee1164::_X);
            } else if ncs.is_1H() || noe.is_1H() {
                f.set_all_to(Ieee1164::_Z);
            } else if let Some(data) = data {
                f.replace_with_int(data).unwrap();
            } else {
                f.set_all_to(Ieee1164::_X);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Signal;

    #[test]
    fn default_all_zero() {
        let rom = Rom1kx8::default();
        for mem in rom.memory.iter() {
            assert_eq!(0, *mem);
        }
    }

    #[test]
    fn read_out_all_data() {
        let mut rom: Rom1kx8 = (0..=255).cycle().collect();
        let mut addr = Port::<LogicVector, Output>::new(LogicVector::from_ieee(Ieee1164::_0, 10));
        let data = Port::<LogicVector, Input>::new(LogicVector::with_width(8));
        let noe = Port::<Ieee1164, Output>::new(Ieee1164::_0);

        let mut sig_noe_ncs = Signal::new();
        sig_noe_ncs.connect(&noe).unwrap();
        sig_noe_ncs.connect(&rom.n_output_enable).unwrap();
        sig_noe_ncs.connect(&rom.n_chip_select).unwrap();

        sig_noe_ncs.update();

        let mut sig_addr = Signal::new();
        sig_addr.connect(&rom.addr).unwrap();
        sig_addr.connect(&addr).unwrap();

        let mut sig_data = Signal::new();
        sig_data.connect(&rom.data).unwrap();
        sig_data.connect(&data).unwrap();

        for i in 0..1024 {
            addr.with_value_mut(|f| f.replace_with_int(i).unwrap());
            sig_addr.update();
            rom.update();
            sig_data.update();

            assert_eq!(data.value(), i & 0xFF);
        }
    }

    #[test]
    fn output() {
        let mut rom = Rom1kx8::default();
        for (i, m) in rom.memory.iter_mut().enumerate() {
            *m = i as u8;
        }
        let mut addr = Port::<LogicVector, Output>::new(LogicVector::from_ieee(Ieee1164::_0, 10));
        let data = Port::<LogicVector, Input>::new(LogicVector::with_width(8));
        let noe = Port::<Ieee1164, Output>::new(Ieee1164::_0);

        let mut sig_noe_ncs = Signal::new();
        sig_noe_ncs.connect(&noe).unwrap();
        sig_noe_ncs.connect(&rom.n_output_enable).unwrap();
        sig_noe_ncs.connect(&rom.n_chip_select).unwrap();

        sig_noe_ncs.update();

        let mut sig_addr = Signal::new();
        sig_addr.connect(&rom.addr).unwrap();
        sig_addr.connect(&addr).unwrap();

        let mut sig_data = Signal::new();
        sig_data.connect(&rom.data).unwrap();
        sig_data.connect(&data).unwrap();

        for i in 0..1024 {
            addr.with_value_mut(|f| f.replace_with_int(i).unwrap());
            sig_addr.update();
            rom.update();
            sig_data.update();

            assert_eq!(data.value(), i & 0xFF);
        }
    }
}
