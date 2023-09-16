//! Additional functionality for numerics.
//!
//! This module provides some extra types that are useful when doing numerical
//! work. See the individual documentation for each piece for more information.

#![allow(missing_docs)]

pub use core::num::Saturating;
pub use core::num::Wrapping;
pub use core::num::{FpCategory, ParseFloatError, ParseIntError, TryFromIntError};

pub use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
pub use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

pub use core::num::IntErrorKind;

#[cfg(test)]
use crate::std::fmt;
#[cfg(test)]
use crate::std::ops::{Add, Div, Mul, Rem, Sub};

/// Helper function for testing numeric operations
#[cfg(test)]
pub fn test_num<T>(ten: T, two: T)
where
    T: PartialEq
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + fmt::Debug
        + Copy,
{
    assert_eq!(ten.add(two), ten + two);
    assert_eq!(ten.sub(two), ten - two);
    assert_eq!(ten.mul(two), ten * two);
    assert_eq!(ten.div(two), ten / two);
    assert_eq!(ten.rem(two), ten % two);
}
