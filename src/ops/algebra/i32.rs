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

use crate::ops::algebra::Addition;

impl Addition for i32 {
    type Output = i32;

    fn checked_addition(self, rhs: Self) -> Option<Self::Output> {
        self.checked_add(rhs)
    }

    fn overflowing_addition(self, rhs: Self) -> (Self::Output, bool) {
        self.overflowing_add(rhs)
    }

    fn saturating_addition(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }

    fn wrapping_addition(self, rhs: Self) -> Self::Output {
        self.wrapping_add(rhs)
    }
}