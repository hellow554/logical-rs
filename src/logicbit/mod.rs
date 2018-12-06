mod ieee1164;
mod logicvector;
mod tvlogic;

pub use self::ieee1164::Ieee1164;
pub use self::logicvector::LogicVector;
pub use self::tvlogic::Ieee1164Value;

/// This trait is similar to `Add`, `Sub`, `Mul`, ... and is used to describe how values on the
/// same line should be resolved to one `T`.
///
/// It is required to be commutative! (e.g. `A.resolve(B) == B.resolve(A)`)
pub trait Resolve<RHS = Self> {
    /// The output of this trait, e.g. `T` itself. Just look at the [`std::ops::Add`] trait for
    /// an example
    type Output: ?Sized;

    /// The actual resolve function. This takes lhs, rhs and produces an output from it.
    /// The type is not restricted and can be freely chosen.
    fn resolve(self, rhs: RHS) -> Self::Output;
}
