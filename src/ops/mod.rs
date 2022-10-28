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

//! 
//! 
//! ## Operations
//! 
//! The `ops` module contains the definition of many traits for mathematical operations.
//! 
//! Documentation for each trait is provided in the trait's definition.

mod algebra;
mod trigonometry;

pub use algebra::*;
pub use trigonometry::*;