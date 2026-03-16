#![no_std]
#![forbid(unsafe_code)]
#![allow(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]
// All casts are validated at compile time via `assert_infallible_cast!`
// and fallible conversions are guarded against being built on incompatible
// targets.
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

#[cfg(any(feature = "min-usize-32", feature = "from_usize"))]
macro_rules! assert_infallible_cast {
    ($src:tt => $dst:tt) => {
        const _: () = {
            const SRC_BITS: u32 = $src::BITS;
            const DST_BITS: u32 = $dst::BITS;
            #[allow(clippy::cast_sign_loss)]
            const SRC_SIGNED: bool = (-1_i8 as $src) < (0 as $src);
            #[allow(clippy::cast_sign_loss)]
            const DST_SIGNED: bool = (-1_i8 as $dst) < (0 as $dst);
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
/// Infallibly convert a value into `usize`.
///
/// Implementations exist only when the conversion is guaranteed not
/// to lose information under the selected `min-usize-*` portability
/// contract.
///
/// ```
/// use usize_conv::ToUsize;
///
/// let x: u16 = 5;
/// assert_eq!(x.to_usize(), 5usize);
/// ```
pub trait ToUsize {
    fn to_usize(self) -> usize;
}

#[cfg(feature = "min-usize-32")]
/// Infallibly convert a value into `isize`.
///
/// Implementations exist only when the conversion is guaranteed not
/// to lose information under the selected `min-usize-*` portability
/// contract.
///
/// ```
/// use usize_conv::ToIsize;
///
/// let x: i16 = -3;
/// assert_eq!(x.to_isize(), -3isize);
/// ```
pub trait ToIsize {
    fn to_isize(self) -> isize;
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
                self as usize
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
                self as isize
            }
        }
    };
}

#[cfg(feature = "min-usize-32")]
macro_rules! impl_nonzero_usize {
    ($($src:tt),* $(,)?) => {
        $(
            impl ToUsize for $src {
                #[inline]
                fn to_usize(self) -> usize {
                    self.get() as usize
                }
            }
        )*
    };
}

#[cfg(feature = "min-usize-32")]
macro_rules! impl_nonzero_isize {
    ($($src:ty),* $(,)?) => {
        $(
            impl ToIsize for $src {
                #[inline]
                fn to_isize(self) -> isize {
                    self.get() as isize
                }
            }
        )*
    };
}

#[cfg(feature = "min-usize-32")]
impl ToUsize for usize {
    #[inline]
    fn to_usize(self) -> usize {
        self
    }
}

#[cfg(feature = "min-usize-32")]
impl ToIsize for isize {
    #[inline]
    fn to_isize(self) -> isize {
        self
    }
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
    use core::num::{NonZeroI8, NonZeroI16, NonZeroI32, NonZeroU8, NonZeroU16, NonZeroU32};

    impl_to_usize!(u8);
    impl_to_usize!(u16);
    impl_to_usize!(u32);

    impl_to_isize!(u8);
    impl_to_isize!(i8);
    impl_to_isize!(u16);
    impl_to_isize!(i16);
    impl_to_isize!(i32);

    impl_nonzero_usize!(NonZeroU8);
    impl_nonzero_usize!(NonZeroU16);
    impl_nonzero_usize!(NonZeroU32);

    impl_nonzero_isize!(NonZeroU8);
    impl_nonzero_isize!(NonZeroI8);
    impl_nonzero_isize!(NonZeroU16);
    impl_nonzero_isize!(NonZeroI16);
    impl_nonzero_isize!(NonZeroI32);
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
    use core::num::{NonZeroI64, NonZeroU32, NonZeroU64};

    impl_to_isize!(u32);
    impl_to_usize!(u64);
    impl_to_isize!(i64);

    impl_nonzero_usize!(NonZeroU64);
    impl_nonzero_isize!(NonZeroU32);
    impl_nonzero_isize!(NonZeroI64);
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
    mod ge32 {
        #[allow(clippy::wildcard_imports)]
        use super::*;
        use core::num::{NonZeroI8, NonZeroI16, NonZeroI32, NonZeroU8, NonZeroU16, NonZeroU32};

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
            let x = NonZeroU8::new(5).unwrap();
            assert_eq!(x.to_usize(), 5);

            let x = NonZeroU16::new(5).unwrap();
            assert_eq!(x.to_usize(), 5);

            let x = NonZeroU32::new(7).unwrap();
            assert_eq!(x.to_usize(), 7);

            // to isize (signed)
            let y = NonZeroI8::new(-5).unwrap();
            assert_eq!(y.to_isize(), -5);

            let y = NonZeroI16::new(-5).unwrap();
            assert_eq!(y.to_isize(), -5);

            let y = NonZeroI32::new(-7).unwrap();
            assert_eq!(y.to_isize(), -7);

            // unsigned to isize (widening cross-sign)
            let x = NonZeroU8::new(5).unwrap();
            assert_eq!(x.to_isize(), 5);

            let x = NonZeroU16::new(5).unwrap();
            assert_eq!(x.to_isize(), 5);
        }
    }

    #[cfg(feature = "min-usize-64")]
    mod ge64 {
        #[allow(clippy::wildcard_imports)]
        use super::*;
        use core::num::{NonZeroI64, NonZeroU32, NonZeroU64};

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
            let x = NonZeroU64::new(9).unwrap();
            assert_eq!(x.to_usize(), 9);

            let y = NonZeroI64::new(-9).unwrap();
            assert_eq!(y.to_isize(), -9);

            // u32 -> isize: only valid on 64-bit
            let x = NonZeroU32::new(7).unwrap();
            assert_eq!(x.to_isize(), 7);
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
