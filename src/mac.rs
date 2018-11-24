macro_rules! expand_op {
    ($func_name:ident, $trait_name:ident, $fn_name:ident, $for_type:ty, $rhs:ty, $output_type:ty) => {
        impl $trait_name<$rhs> for $for_type {
            type Output = $output_type;
            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                $func_name(&self, &rhs)
            }
        }
        impl<'a> $trait_name<$rhs> for &'a $for_type {
            type Output = $output_type;
            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                $func_name(self, &rhs)
            }
        }
        impl<'b> $trait_name<&'b $rhs> for $for_type {
            type Output = $output_type;
            fn $fn_name(self, rhs: &'b $rhs) -> Self::Output {
                $func_name(&self, rhs)
            }
        }
        impl<'a, 'b> $trait_name<&'b $rhs> for &'a $for_type {
            type Output = $output_type;
            fn $fn_name(self, rhs: &'b $rhs) -> Self::Output {
                $func_name(self, rhs)
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! from_prim {
    ($t:expr, $u:ident, $s:ident) => {
        impl From<$u> for LogicVector {
            fn from(v: $u) -> LogicVector {
                LogicVector::new_from_value($t, u64::from(v))
            }
        }
        impl From<$s> for LogicVector {
            fn from(v: $s) -> LogicVector {
                LogicVector::new_from_value($t, v as u64)
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! into_prim {
    ($t:expr, $u:ident, $s:ident) => {
        impl TryFrom<LogicVector> for $u {
            type Error = VectorError;
            fn try_from(lv: LogicVector) -> Result<Self, Self::Error> {
                //TODO maybe without string, could be improved here
                if lv.bit_width() > $t {
                    return Err(VectorError::Overflow);
                }
                let mut str = String::from_iter(lv.data.iter().take($t).map(char::from));
                if !lv.is_01() {
                    return Err(VectorError::HasUXZ);
                }
                match $u::from_str_radix(str.deref_mut(), 2) {
                    Ok(e) => Ok(e),
                    _ => Err(VectorError::Other),
                } //TODO hae?!
            }
        }
        impl<'a> TryFrom<&'a LogicVector> for $u {
            type Error = VectorError;
            fn try_from(lv: &'a LogicVector) -> Result<Self, Self::Error> {
                if lv.bit_width() > $t {
                    return Err(VectorError::Overflow);
                }
                let mut str = String::from_iter(lv.data.iter().take($t).map(char::from));
                if !lv.is_01() {
                    return Err(VectorError::HasUXZ);
                }
                match $u::from_str_radix(str.deref_mut(), 2) {
                    Ok(e) => Ok(e),
                    _ => Err(VectorError::Other),
                } //TODO hae?!
            }
        }
        impl TryFrom<LogicVector> for $s {
            type Error = VectorError;
            fn try_from(lv: LogicVector) -> Result<Self, Self::Error> {
                if lv.bit_width() > $t {
                    return Err(VectorError::Overflow);
                }
                let mut str = String::from_iter(lv.data.iter().take($t).map(char::from));
                if !lv.is_01() {
                    return Err(VectorError::HasUXZ);
                }
                match $u::from_str_radix(str.deref_mut(), 2) {
                    Ok(e) => Ok(e as $s),
                    _ => Err(VectorError::Other),
                } //TODO hae?!
            }
        }

        impl<'a> TryFrom<&'a LogicVector> for $s {
            type Error = VectorError;
            fn try_from(lv: &'a LogicVector) -> Result<Self, Self::Error> {
                if lv.bit_width() > $t {
                    return Err(VectorError::Overflow);
                }
                let mut str = String::from_iter(lv.data.iter().take($t).map(char::from));
                if !lv.is_01() {
                    return Err(VectorError::HasUXZ);
                }
                match $u::from_str_radix(str.deref_mut(), 2) {
                    Ok(e) => Ok(e as $s),
                    _ => Err(VectorError::Other),
                } //TODO hae?!
            }
        }
    };
}
