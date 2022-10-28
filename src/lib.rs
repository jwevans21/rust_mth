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

#![no_std]
#![deny(clippy::pedantic)]
#![deny(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    unused_doc_comments
)]

//! # `rust_mth`
//!
//! `rust_mth` is a library for doing math in Rust.
//! 
//! ## Main Features
//! 
//! ## Crate Features
//! 
//! ## Versioning
//! 
//! This library is versioned according to [Semantic Versioning](https://semver.org/).
//! 
//! Trait members where a default implementation is provided that is tested against
//! the primitive types are not considered breaking changes. These changes can occur
//! in minor version updates.
//! 
//! The addition of new trait requirements or the addition of new trait members that
//! do not have a default implementation are considered breaking changes. These changes
//! can occur only in major version updates.
//! 
//! When an enum is marked as `#[non_exhaustive]`, the addition of new variants is
//! not considered a breaking change. These changes can occur in minor version updates.
//! 
//! ## License
//! 
//! This project is licensed under either of
//! 
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE)
//!   or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or
//!   <http://opensource.org/licenses/MIT>)
//! 
//! at your option.
//! 
//! ### Contribution
//! 
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in `rust_mth` by you, as defined in the Apache-2.0 license, shall
//! be dual licensed as above, without any additional terms or conditions.

mod consts;
pub mod ops;

pub use consts::*;