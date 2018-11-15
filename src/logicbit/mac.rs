macro_rules! expand_resolve_op {
    ($func_name:ident, $for_type:ty, $output_type:ty, $rhs:ty) => {
        impl Resolve<$rhs> for $for_type {
            type Output = $output_type;
            fn resolve(self, rhs: $rhs) -> Self::Output {
                $func_name(self, rhs)
            }
        }
        impl<'a> Resolve<$rhs> for &'a $for_type {
            type Output = $output_type;
            fn resolve(self, rhs: $rhs) -> Self::Output {
                $func_name(*self, rhs)
            }
        }
        impl<'b> Resolve<&'b $rhs> for $for_type {
            type Output = $output_type;
            fn resolve(self, rhs: &'b $rhs) -> Self::Output {
                $func_name(self, *rhs)
            }
        }
        impl<'a, 'b> Resolve<&'b $rhs> for &'a $for_type {
            type Output = $output_type;
            fn resolve(self, rhs: &'b $rhs) -> Self::Output {
                $func_name(*self, *rhs)
            }
        }
    };
}
