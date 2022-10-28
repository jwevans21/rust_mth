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

/// Basic constants used in the library.
/// 
/// These are defined for integers, but those are not accurate. For more accurate
/// constants, use a floating-point type.
pub trait Consts {
    /// Euler's number. $e$
    const E: Self;
    /// $\frac{1}{\pi}$
    const FRAC_1_PI: Self;
    /// $\frac{1}{\sqrt{2}}$
    const FRAC_1_SQRT_2: Self;
    /// $\frac{2}{\pi}$
    const FRAC_2_PI: Self;
    /// $\frac{2}{\sqrt{\pi}}$
    const FRAC_2_SQRT_PI: Self;
    /// $\frac{\pi}{2}$
    const FRAC_PI_2: Self;
    /// $\frac{\pi}{3}$
    const FRAC_PI_3: Self;
    /// $\frac{\pi}{4}$
    const FRAC_PI_4: Self;
    /// $\frac{\pi}{6}$
    const FRAC_PI_6: Self;
    /// $\frac{\pi}{8}$
    const FRAC_PI_8: Self;
    /// $ln(2)$
    const LN_2: Self;
    /// $ln(10)$
    const LN_10: Self;
    /// $\log_2(10)$
    const LOG2_10: Self;
    /// $\log_2(e)$
    const LOG2_E: Self;
    /// $\log_{10}(2)$
    const LOG10_2: Self;
    /// $\log_{10}(e)$
    const LOG10_E: Self;
    /// Archimedes' constant. $\pi$
    const PI: Self;
    /// $\sqrt{2}$
    const SQRT_2: Self;
    /// The full circle constant. $\tau$
    const TAU: Self;
}

// These are not very meaningful, but they are here to make sure that the
// operations that require them can be used on all the primitive types.
macro_rules! impl_consts_ints {
    ($($t:ty)*) => ($(
        /// These are not very meaningful, but they are here to make sure that the
        /// operations that require them can be used on all the primitive types.
        /// Most are very heavily rounded resulting in many of them being the
        /// same value. If you need more accurate values, use floating point
        /// numbers.
        impl Consts for $t {
            const E: Self = 2 as $t;
            const FRAC_1_PI: Self = 1 as $t;
            const FRAC_1_SQRT_2: Self = 1 as $t;
            const FRAC_2_PI: Self = 2 as $t;
            const FRAC_2_SQRT_PI: Self = 2 as $t;
            const FRAC_PI_2: Self = 1 as $t;
            const FRAC_PI_3: Self = 1 as $t;
            const FRAC_PI_4: Self = 1 as $t;
            const FRAC_PI_6: Self = 1 as $t;
            const FRAC_PI_8: Self = 1 as $t;
            const LN_2: Self = 1 as $t;
            const LN_10: Self = 1 as $t;
            const LOG2_10: Self = 1 as $t;
            const LOG2_E: Self = 1 as $t;
            const LOG10_2: Self = 1 as $t;
            const LOG10_E: Self = 1 as $t;
            const PI: Self = 3 as $t;
            const SQRT_2: Self = 1 as $t;
            const TAU: Self = 6 as $t;
        }
    )*);
}

use core::f64::consts;

macro_rules! impl_consts_floats {
    ($($t:ty)*) => ($(
        /// These values are more accurate than the ones for integers. However,
        /// floating point numbers are not exact, so they are still not completely
        /// accurate. If you need more accurate values, use a crate like [`rug`]
        /// that provides arbitrary precision numbers.
        /// 
        /// [`rug`]: https://crates.io/crates/rug
        impl Consts for $t {
            const E: Self = consts::E as $t;
            const FRAC_1_PI: Self = consts::FRAC_1_PI as $t;
            const FRAC_1_SQRT_2: Self =  consts::FRAC_1_SQRT_2 as $t;
            const FRAC_2_PI: Self =  consts::FRAC_2_PI as $t;
            const FRAC_2_SQRT_PI: Self =  consts::FRAC_2_SQRT_PI as $t;
            const FRAC_PI_2: Self =  consts::FRAC_PI_2 as $t;
            const FRAC_PI_3: Self =  consts::FRAC_PI_3 as $t;
            const FRAC_PI_4: Self =  consts::FRAC_PI_4 as $t;
            const FRAC_PI_6: Self =  consts::FRAC_PI_6 as $t;
            const FRAC_PI_8: Self =  consts::FRAC_PI_8 as $t;
            const LN_2: Self =  consts::LN_2 as $t;
            const LN_10: Self =  consts::LN_10 as $t;
            const LOG2_10: Self =  consts::LOG2_10 as $t;
            const LOG2_E: Self =  consts::LOG2_E as $t;
            const LOG10_2: Self =  consts::LOG10_2 as $t;
            const LOG10_E: Self =  consts::LOG10_E as $t;
            const PI: Self =  consts::PI as $t;
            const SQRT_2: Self =  consts::SQRT_2 as $t;
            const TAU: Self =  consts::TAU as $t;
        }
    )*);
}

impl_consts_ints!( u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize );
impl_consts_floats!( f32 f64 );
