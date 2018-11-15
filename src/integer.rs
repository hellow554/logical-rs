use crate::Resolve;

#[derive(Debug, Default, Clone, Copy)]
pub struct Integer {
    value: u128,
    width: u8,
}

impl<'a, 'b> Resolve<&'b Integer> for &'a Integer {
    type Output = Integer;
    fn resolve(self, other: &'b Integer) -> Self::Output {
        if self.width != other.width {
            panic!("Width mismatch!") //TODO: do not panic
        }
        Integer {
            value: self.value | other.value,
            width: self.width,
        }
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Integer) -> bool {
        self.value == other.value
    }
}

impl<T: Into<u128> + Copy> PartialEq<T> for Integer {
    fn eq(&self, other: &T) -> bool {
        self.value == (*other).into()
    }
}

impl Eq for Integer {}

impl Integer {
    pub fn new() -> Self {
        Self { value: 0, width: 128 }
    }

    pub fn new_with_value(value: impl Into<u128>, width: impl Into<Option<u8>>) -> Self {
        Self {
            value: value.into(),
            width: width.into().unwrap_or(128),
        }
    }

    pub fn new_with_width(width: u8) -> Option<Self> {
        if width != 0 && width <= 128 {
            Some(Self { value: 0, width })
        } else {
            None
        }
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn set_width(&mut self, width: u8) {
        if width != 0 && width <= 128 {
            self.value &= (1 << width) - 1
        }
    }
}
