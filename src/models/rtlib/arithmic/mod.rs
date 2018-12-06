//! This module will provide gates that allow doing arithmetical things like adding (either with
//! or without carry bit), Subtracting, Incrementing, Shifting left and right and over things.

mod add;
pub use self::add::Add;
