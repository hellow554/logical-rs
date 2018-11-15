#[macro_use]
mod mac;
mod ieee1164;
mod tvlogic;

pub use self::ieee1164::Ieee1164;
pub use self::tvlogic::Ieee1164Value;

/// This trait is similar to `Add`, `Sub`, `Mul`, ... and is used to describe how values on the
/// same line should be resolved to one `T`.
///
/// It is required to be commutative! (e.g. `A.resolve(B) == B.resolve(A)`)
pub trait Resolve<RHS = Self> {
    type Output: ?Sized;
    fn resolve(self, rhs: RHS) -> Self::Output;
}
