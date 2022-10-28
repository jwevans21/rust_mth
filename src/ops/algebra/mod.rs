// Copyright (c) 2022 Jacob Evans
//
// This work is licensed under either of
//
// * Apache License, Version 2.0,
//   http://www.apache.org/licenses/LICENSE-2.0
//
// * MIT license,
//   http://opensource.org/licenses/MIT
//
// at your option.
//
// This file may not be copied, modified, or distributed except
// according to those terms.
//
// See LICENSE-APACHE and LICENSE-MIT for details.

// The implementations for the primitive types
mod i32;

/// A trait for performing basic algebraic operations.
pub trait Algebra: Addition + Subtraction + Multiplication + Division {}

/// A trait for addition operations.
///
/// This is one of the base traits for all operations.
pub trait Addition {
    /// The type of the result of addition.
    type Output;

    /// Adds `self` and `rhs`.
    /// 
    /// If teh result would cause overflow, the `None` variant is returned.
    fn checked_addition(self, rhs: Self) -> Option<Self::Output>;

    /// Adds `self` and `rhs`.
    /// 
    /// If the result would cause overflow, the result is wrapped around
    /// and the returned boolean is set to `true`.
    fn overflowing_addition(self, rhs: Self) -> (Self::Output, bool);

    /// Adds `self` and `rhs`.
    /// 
    /// If the result would cause overflow, the result is saturated to the
    /// maximum or minimum value of the type.
    fn saturating_addition(self, rhs: Self) -> Self::Output;

    /// Adds `self` and `rhs`.
    /// 
    /// If the result would cause overflow, the result is wrapped around.
    fn wrapping_addition(self, rhs: Self) -> Self::Output;
}

/// A trait for subtraction operations.
///
/// This is one of the base traits for all operations.
pub trait Subtraction {
    /// The type of the result of subtraction.
    type Output;

    /// Subtracts `rhs` from `self`.
    /// 
    /// If the result would cause overflow, the `None` variant is returned.
    fn checked_subtraction(self, rhs: Self) -> Option<Self::Output>;

    /// Subtracts `rhs` from `self`.
    /// 
    /// If the result would cause overflow, the result is wrapped around
    /// and the returned boolean is set to `true`.
    fn overflowing_subtraction(self, rhs: Self) -> (Self::Output, bool);

    /// Subtracts `rhs` from `self`.
    /// 
    /// If the result would cause overflow, the result is saturated to the
    /// maximum or minimum value of the type.
    fn saturating_subtraction(self, rhs: Self) -> Self::Output;

    /// Subtracts `rhs` from `self`.
    /// 
    /// If the result would cause overflow, the result is wrapped around.
    fn wrapping_subtraction(self, rhs: Self) -> Self::Output;
}

/// A trait for multiplication operations.
///
/// This is one of the base traits for all operations.
pub trait Multiplication {}

/// A trait for division operations.
///
/// This is one of the base traits for all operations.
pub trait Division {}
