#![no_std]
#![forbid(unsafe_code)]
#![allow(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]
// All casts are validated at compile time via `assert_infallible_cast!`
// and fallible conversions are guarded against being built on incompatible
// targets.
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

#[cfg(feature = "min-usize-32")]
use core::num::{NonZero, NonZeroIsize, NonZeroUsize};

#[cfg(any(feature = "min-usize-32", feature = "from_usize"))]
macro_rules! assert_infallible_cast {
    ($src:tt => $dst:tt) => {
        const _: () = {
            const SRC_BITS: u32 = $src::BITS;
            const DST_BITS: u32 = $dst::BITS;
            #[allow(unused_comparisons)] // silence warning on unsigned types
            const SRC_SIGNED: bool = $src::MIN < 0;
            #[allow(unused_comparisons)] // silence warning on unsigned types
            const DST_SIGNED: bool = $dst::MIN < 0;
            assert!(match (SRC_SIGNED, DST_SIGNED) {
                (false, false) => SRC_BITS <= DST_BITS,
                (true, true) => SRC_BITS <= DST_BITS,
                (false, true) => SRC_BITS < DST_BITS,
                (true, false) => false,
            });
        };
    };
}

#[cfg(feature = "min-usize-32")]
/// Infallibly convert a value into [`usize`].
///
/// Implementations exist only when the conversion is guaranteed not
/// to lose information under the selected `min-usize-*` portability
/// contract.
///
/// ```
/// use usize_conv::ToUsize;
///
/// let x: u16 = 5;
/// assert_eq!(x.to_usize(), 5_usize);
/// ```
pub trait ToUsize {
    fn to_usize(self) -> usize;
}

#[cfg(feature = "min-usize-32")]
/// Infallibly convert a non-zero value into [`NonZeroUsize`].
///
/// Implementations exist only when the conversion is guaranteed not
/// to lose information under the selected `min-usize-*` portability
/// contract.
///
/// ```
/// # use core::num::{NonZeroU16, NonZeroUsize};
/// use usize_conv::ToNonZeroUsize;
///
/// let x: NonZeroU16 = 5.try_into().unwrap();
/// assert_eq!(x.to_nonzero_usize(), NonZeroUsize::try_from(5).unwrap());
/// ```
pub trait ToNonZeroUsize {
    fn to_nonzero_usize(self) -> NonZeroUsize;
}

#[cfg(feature = "min-usize-32")]
/// Infallibly convert a value into [`isize`].
///
/// Implementations exist only when the conversion is guaranteed not
/// to lose information under the selected `min-usize-*` portability
/// contract.
///
/// ```
/// use usize_conv::ToIsize;
///
/// let x: i16 = -3;
/// assert_eq!(x.to_isize(), -3_isize);
/// ```
pub trait ToIsize {
    fn to_isize(self) -> isize;
}

#[cfg(feature = "min-usize-32")]
/// Infallibly convert a non-zero value into [`NonZeroIsize`].
///
/// Implementations exist only when the conversion is guaranteed not
/// to lose information under the selected `min-usize-*` portability
/// contract.
///
/// ```
/// # use core::num::{NonZeroI16, NonZeroIsize};
/// use usize_conv::ToNonZeroIsize;
///
/// let x = NonZeroI16::try_from(-3).unwrap();
/// assert_eq!(x.to_nonzero_isize(), NonZeroIsize::try_from(-3).unwrap());
/// ```
pub trait ToNonZeroIsize {
    fn to_nonzero_isize(self) -> NonZeroIsize;
}

#[cfg(feature = "from_usize")]
/// Infallibly convert `usize` to `u64`.
pub trait ToU64 {
    fn to_u64(self) -> u64;
}

#[cfg(feature = "from_usize")]
/// Infallibly convert `isize` to `i64`.
pub trait ToI64 {
    fn to_i64(self) -> i64;
}

#[cfg(feature = "from_usize")]
/// Infallibly convert `usize` to `u128`.
pub trait ToU128 {
    fn to_u128(self) -> u128;
}

#[cfg(feature = "from_usize")]
/// Infallibly convert `usize` or `isize` to `i128`.
pub trait ToI128 {
    fn to_i128(self) -> i128;
}

#[cfg(feature = "min-usize-32")]
macro_rules! impl_to_usize {
    ($src:tt) => {
        assert_infallible_cast!($src => usize);

        impl ToUsize for $src {
            #[inline]
            fn to_usize(self) -> usize {
                // CAST: Validated by `assert_infallible_cast!($src => usize)`
                // at the start of this macro expansion.
                self as usize
            }
        }

        impl ToUsize for NonZero<$src> {
            #[inline]
            fn to_usize(self) -> usize {
                // CAST: Validated by `assert_infallible_cast!($src => usize)`
                // at the start of this macro expansion.
                self.get() as usize
            }
        }

        impl ToNonZeroUsize for NonZero<$src> {
            #[inline]
            fn to_nonzero_usize(self) -> NonZeroUsize {
                // CAST: Validated by `assert_infallible_cast!($src => usize)`
                // at the start of this macro expansion.
                let val = self.get() as usize;

                // The source is non-zero and the cast is lossless, so the non-zero
                // property is preserved.
                NonZeroUsize::new(val).unwrap()
            }
        }
    };
}

#[cfg(feature = "min-usize-32")]
macro_rules! impl_to_isize {
    ($src:tt) => {
        assert_infallible_cast!($src => isize);

        impl ToIsize for $src {
            #[inline]
            fn to_isize(self) -> isize {
                // CAST: Validated by `assert_infallible_cast!($src => isize)`
                // at the start of this macro expansion.
                self as isize
            }
        }

        impl ToIsize for NonZero<$src> {
            #[inline]
            fn to_isize(self) -> isize {
                // CAST: Validated by `assert_infallible_cast!($src => isize)`
                // at the start of this macro expansion.
                self.get() as isize
            }
        }

        impl ToNonZeroIsize for NonZero<$src> {
            #[inline]
            fn to_nonzero_isize(self) -> NonZeroIsize {
                // CAST: Validated by `assert_infallible_cast!($src => isize)`
                // at the start of this macro expansion.
                let val = self.get() as isize;

                // The source is non-zero and the cast is lossless, so the non-zero
                // property is preserved.
                NonZeroIsize::new(val).unwrap()
            }
        }
    };
}

/// Define infallible conversions to `usize`/`isize` that are guaranteed
/// on 32-bit or larger architectures
///
/// This module is enabled for `min-usize-32` and `min-usize-64`.
#[cfg(feature = "min-usize-32")]
mod ge32 {
    #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
    compile_error!("`min-usize-32` requires a target with at least 32-bit `usize`");

    #[allow(clippy::wildcard_imports)]
    use super::*;

    impl_to_usize!(u8);
    impl_to_usize!(u16);
    impl_to_usize!(u32);
    impl_to_usize!(usize);

    impl_to_isize!(u8);
    impl_to_isize!(i8);
    impl_to_isize!(u16);
    impl_to_isize!(i16);
    impl_to_isize!(i32);
    impl_to_isize!(isize);
}

/// Define infallible conversions to `usize`/`isize` that are guaranteed on
/// 64-bit architectures
///
/// This module is enabled only for `min-usize-64`.
#[cfg(feature = "min-usize-64")]
mod ge64 {
    #[cfg(not(target_pointer_width = "64"))]
    compile_error!("`min-usize-64` requires a target with 64-bit `usize`");

    #[allow(clippy::wildcard_imports)]
    use super::*;

    impl_to_isize!(u32);
    impl_to_usize!(u64);
    impl_to_isize!(i64);
}

/// Define widening conversions whose source is `usize`/`isize`.
///
/// This module is enabled by the `from_usize` feature.
#[cfg(feature = "from_usize")]
mod from_usize_mod {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    assert_infallible_cast!(usize => u64);

    impl ToU64 for usize {
        #[inline]
        fn to_u64(self) -> u64 {
            self as u64
        }
    }

    assert_infallible_cast!(usize => u128);

    impl ToU128 for usize {
        #[inline]
        fn to_u128(self) -> u128 {
            self as u128
        }
    }

    assert_infallible_cast!(usize => i128);

    impl ToI128 for usize {
        #[inline]
        fn to_i128(self) -> i128 {
            self as i128
        }
    }

    assert_infallible_cast!(isize => i64);

    impl ToI64 for isize {
        #[inline]
        fn to_i64(self) -> i64 {
            self as i64
        }
    }

    assert_infallible_cast!(isize => i128);

    impl ToI128 for isize {
        #[inline]
        fn to_i128(self) -> i128 {
            self as i128
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[cfg(feature = "min-usize-32")]
    /// Helper to create NonZero values.
    macro_rules! nz {
        ($value:expr) => {
            const { ::core::num::NonZero::new($value).expect("nz!: value was zero") }
        };
    }

    #[cfg(feature = "min-usize-32")]
    mod ge32 {
        #[allow(clippy::wildcard_imports)]
        use super::*;

        #[test]
        fn identity() {
            assert_eq!(7usize.to_usize(), 7);
            assert_eq!((-7isize).to_isize(), -7);
        }

        #[test]
        fn basic() {
            // u8/i8
            assert_eq!(123u8.to_usize(), 123);
            assert_eq!(123u8.to_isize(), 123);
            assert_eq!((-12i8).to_isize(), -12);

            // u16/i16
            assert_eq!(123u16.to_usize(), 123);
            assert_eq!(123u16.to_isize(), 123);
            assert_eq!((-12i16).to_isize(), -12);

            // u32/i32
            assert_eq!(456u32.to_usize(), 456);
            assert_eq!((-456i32).to_isize(), -456);
        }

        #[test]
        fn nonzero() {
            // to usize
            let x = nz!(5_u8);
            assert_eq!(x.to_nonzero_usize(), nz!(5_usize));
            assert_eq!(x.to_usize(), 5_usize);

            let x = nz!(6_u16);
            assert_eq!(x.to_nonzero_usize(), nz!(6_usize));
            assert_eq!(x.to_usize(), 6_usize);

            let x = nz!(7_u32);
            assert_eq!(x.to_nonzero_usize(), nz!(7_usize));
            assert_eq!(x.to_usize(), 7_usize);

            // to isize (signed)
            let x = nz!(-5_i8);
            assert_eq!(x.to_nonzero_isize(), nz!(-5_isize));
            assert_eq!(x.to_isize(), -5_isize);

            let x = nz!(-6_i16);
            assert_eq!(x.to_nonzero_isize(), nz!(-6_isize));
            assert_eq!(x.to_isize(), -6_isize);

            let x = nz!(-7_i32);
            assert_eq!(x.to_nonzero_isize(), nz!(-7_isize));
            assert_eq!(x.to_isize(), -7_isize);

            // unsigned to isize (widening cross-sign)
            let x = nz!(8_u8);
            assert_eq!(x.to_nonzero_isize(), nz!(8_isize));
            assert_eq!(x.to_isize(), 8_isize);

            let x = nz!(9_u16);
            assert_eq!(x.to_nonzero_isize(), nz!(9_isize));
            assert_eq!(x.to_isize(), 9_isize);

            let x = nz!(10_usize);
            assert_eq!(x.to_nonzero_usize(), nz!(10_usize));
            assert_eq!(x.to_usize(), 10_usize);

            let x = nz!(10_isize);
            assert_eq!(x.to_nonzero_isize(), nz!(10_isize));
            assert_eq!(x.to_isize(), 10_isize);
        }
    }

    #[cfg(feature = "min-usize-64")]
    mod ge64 {
        #[allow(clippy::wildcard_imports)]
        use super::*;

        #[test]
        fn basic() {
            // unsigned to usize
            assert_eq!(789u64.to_usize(), 789);

            // signed to isize
            assert_eq!((-789i64).to_isize(), -789);

            // u32 -> isize: only valid on 64-bit where isize is wider than u32
            assert_eq!(456u32.to_isize(), 456);
        }

        #[test]
        fn nonzero() {
            let x = nz!(9_u64);
            assert_eq!(x.to_nonzero_usize(), nz!(9_usize));
            assert_eq!(x.to_usize(), 9_usize);

            let x = nz!(-9_i64);
            assert_eq!(x.to_nonzero_isize(), nz!(-9_isize));
            assert_eq!(x.to_isize(), -9_isize);

            // u32 -> isize: only valid on 64-bit
            let x = nz!(7_u32);
            assert_eq!(x.to_nonzero_isize(), nz!(7_isize));
            assert_eq!(x.to_isize(), 7_isize);
        }
    }

    #[cfg(feature = "from_usize")]
    #[test]
    fn widening() {
        assert_eq!(42usize.to_u64(), 42);
        assert_eq!(42usize.to_u128(), 42);
        assert_eq!(42usize.to_i128(), 42);

        assert_eq!((-42isize).to_i64(), -42);
        assert_eq!((-42isize).to_i128(), -42);
    }
}
