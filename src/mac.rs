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

/// A helper macro to create signals from multiple ports regardless of the port direction;
///
/// # Example
///
/// ```rust
/// # use logical::{Ieee1164, Port, signal, Signal};
/// # use logical::direction::{Input, Output, InOut};
/// let p1 = Port::<Ieee1164, Input>::default();
/// let p2 = Port::<_, Input>::default();
/// let p3 = Port::<_, Output>::default();
/// let p4 = Port::<_, InOut>::default();
/// let p5 = Port::<_, Output>::default();
///
/// let signal = signal!(p1, p2, p3, p4, p5);
/// ```
#[macro_export]
macro_rules! signal {
    ( $( $x:expr ),* ) => {
        {
            let mut signal = Signal::default();
            $(
                signal.connect(&$x).unwrap();
            )*
            signal
        }
    }
}

#[macro_export]
macro_rules! circuit {
    ( $( $x:expr ),* ) => {
        {
            let mut circuit = Circuit::default();
            $(
                circuit.add_updater(&$x);
            )*
            circuit
        }
    }
}
