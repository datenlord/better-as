#![allow(clippy::use_self)]

#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
)))]
compile_error!("unsupported pointer width");

use core::convert::Infallible;
use core::fmt::{self, Display};

pub fn checked_cast<T, U>(src: T) -> Result<U, T::Error>
where
    T: CheckedCast<U>,
{
    src.checked_cast()
}

pub fn wrapping_cast<T, U>(src: T) -> U
where
    T: WrappingCast<U>,
{
    src.wrapping_cast()
}

pub fn extending_cast<T, U>(src: T) -> U
where
    T: ExtendingCast<U>,
{
    src.extending_cast()
}

pub fn truncating_cast<T, U>(src: T) -> U
where
    T: TruncatingCast<U>,
{
    src.truncating_cast()
}

pub trait CheckedCast<T> {
    type Error;
    fn checked_cast(self) -> Result<T, Self::Error>;
}

pub trait WrappingCast<T> {
    fn wrapping_cast(self) -> T;
}

pub trait ExtendingCast<T> {
    fn extending_cast(self) -> T;
}

pub trait TruncatingCast<T> {
    fn truncating_cast(self) -> T;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumCastError {
    Overflow,
    Underflow,
    Infinite,
    NaN,
    Fractional,
}

impl Display for NumCastError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Self::Overflow => "Overflow during numeric conversion",
            Self::Underflow => "Underflow during numeric conversion",
            Self::Fractional => "Cannot store fractional value without loss",
            Self::Infinite => "Cannot store infinite value in finite type",
            Self::NaN => "Cannot store NaN in type which does not support it",
        };

        f.write_str(msg)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NumCastError {}

macro_rules! check {
    (upper $val: expr, $rhs: ty) => {
        if $val > <$rhs>::MAX as Self {
            return Err(NumCastError::Overflow);
        }
    };
    (lower $val: expr, $rhs: ty) => {
        if $val < <$rhs>::MIN as Self {
            return Err(NumCastError::Underflow);
        }
    };
    (both $val: expr, $rhs: ty) => {
        check!(upper $val, $rhs);
        check!(lower $val, $rhs);
    };
    (infallible $val: expr, $rhs: ty) => {}
}

macro_rules! checked_cast{
    ($lhs:ty => $rhs:ty: upper) => {
        impl CheckedCast<$rhs> for $lhs {
            type Error = NumCastError;
            fn checked_cast(self) -> Result<$rhs, Self::Error> {
                check!(upper self, $rhs);
                Ok(self as $rhs)
            }
        }
    };
    ($lhs:ty => $rhs:ty: lower) => {
        impl CheckedCast<$rhs> for $lhs {
            type Error = NumCastError;
            fn checked_cast(self) -> Result<$rhs, Self::Error> {
                check!(lower self, $rhs);
                Ok(self as $rhs)
            }
        }
    };
    ($lhs:ty => $rhs:ty: both) => {
        impl CheckedCast<$rhs> for $lhs {
            type Error = NumCastError;
            fn checked_cast(self) -> Result<$rhs, Self::Error> {
                check!(both self, $rhs);
                Ok(self as $rhs)
            }
        }
    };
    ($lhs:ty => $rhs:ty: infallible ) => {
        impl CheckedCast<$rhs> for $lhs {
            type Error = Infallible;
            fn checked_cast(self) -> Result<$rhs, Self::Error> {
                Ok(self as $rhs)
            }
        }
    };

    ($lhs: ty => $rhs: ty: 16:$c16:tt, 32:$c32:tt, 64:$c64:tt) => {
        impl CheckedCast<$rhs> for $lhs {
            type Error = NumCastError;

            fn checked_cast(self) -> Result<$rhs, Self::Error> {
                #[cfg(target_pointer_width = "16")]
                {
                    check!($c16 self, $rhs);
                }
                #[cfg(target_pointer_width = "32")]
                {
                    check!($c32 self, $rhs);
                }
                #[cfg(target_pointer_width = "64")]
                {
                    check!($c64 self, $rhs);
                }
                Ok(self as $rhs)
            }
        }
    };
    (f32 => $rhs: ty) => {
        impl CheckedCast<$rhs> for f32 {
            type Error = NumCastError;

            fn checked_cast(self) -> Result<$rhs, Self::Error> {
                if self.is_nan() {
                    return Err(NumCastError::NaN);
                }
                if self.is_infinite() {
                    return Err(NumCastError::Infinite);
                }
                if self.trunc().to_bits() != self.to_bits() {
                    return Err(NumCastError::Fractional);
                }
                check!(both self, $rhs);
                Ok(self as $rhs)
            }
        }
    };
    (f64 => $rhs: ty) => {
        impl CheckedCast<$rhs> for f64 {
            type Error = NumCastError;

            fn checked_cast(self) -> Result<$rhs, Self::Error> {
                if self.is_nan() {
                    return Err(NumCastError::NaN);
                }
                if self.is_infinite() {
                    return Err(NumCastError::Infinite);
                }
                if self.trunc().to_bits() != self.to_bits() {
                    return Err(NumCastError::Fractional);
                }
                check!(both self, $rhs);
                Ok(self as $rhs)
            }
        }
    }
}

// u8
checked_cast!(u8 => u8     : infallible  );
checked_cast!(u8 => u16    : infallible  );
checked_cast!(u8 => u32    : infallible  );
checked_cast!(u8 => u64    : infallible  );
checked_cast!(u8 => u128   : infallible  );
checked_cast!(u8 => i8     : upper       );
checked_cast!(u8 => i16    : infallible  );
checked_cast!(u8 => i32    : infallible  );
checked_cast!(u8 => i64    : infallible  );
checked_cast!(u8 => i128   : infallible  );
checked_cast!(u8 => usize  : infallible  );
checked_cast!(u8 => isize  : infallible  );
checked_cast!(u8 => f32    : infallible  );
checked_cast!(u8 => f64    : infallible  );

// u16
checked_cast!(u16 => u8    : upper       );
checked_cast!(u16 => u16   : infallible  );
checked_cast!(u16 => u32   : infallible  );
checked_cast!(u16 => u64   : infallible  );
checked_cast!(u16 => u128  : infallible  );
checked_cast!(u16 => i8    : upper       );
checked_cast!(u16 => i16   : upper       );
checked_cast!(u16 => i32   : infallible  );
checked_cast!(u16 => i64   : infallible  );
checked_cast!(u16 => i128  : infallible  );
checked_cast!(u16 => usize : infallible  );
checked_cast!(u16 => isize : 16: upper, 32: infallible, 64: infallible       );
checked_cast!(u16 => f32   : infallible  );
checked_cast!(u16 => f64   : infallible  );

// u32
checked_cast!(u32 => u8    : upper       );
checked_cast!(u32 => u16   : upper       );
checked_cast!(u32 => u32   : infallible  );
checked_cast!(u32 => u64   : infallible  );
checked_cast!(u32 => u128  : infallible  );
checked_cast!(u32 => i8    : upper       );
checked_cast!(u32 => i16   : upper       );
checked_cast!(u32 => i32   : upper       );
checked_cast!(u32 => i64   : infallible  );
checked_cast!(u32 => i128  : infallible  );
checked_cast!(u32 => usize : 16: upper, 32: infallible, 64: infallible       );
checked_cast!(u32 => isize : 16: upper, 32: upper     , 64: infallible       );

// u64
checked_cast!(u64 => u8    : upper       );
checked_cast!(u64 => u16   : upper       );
checked_cast!(u64 => u32   : upper       );
checked_cast!(u64 => u64   : infallible  );
checked_cast!(u64 => u128  : infallible  );
checked_cast!(u64 => i8    : upper       );
checked_cast!(u64 => i16   : upper       );
checked_cast!(u64 => i32   : upper       );
checked_cast!(u64 => i64   : upper       );
checked_cast!(u64 => i128  : infallible  );
checked_cast!(u64 => usize : 16: upper, 32: upper     , 64: infallible       );
checked_cast!(u64 => isize : upper       );

// u128
checked_cast!(u128 => u8    : upper       );
checked_cast!(u128 => u16   : upper       );
checked_cast!(u128 => u32   : upper       );
checked_cast!(u128 => u64   : upper       );
checked_cast!(u128 => u128  : infallible  );
checked_cast!(u128 => i8    : upper       );
checked_cast!(u128 => i16   : upper       );
checked_cast!(u128 => i32   : upper       );
checked_cast!(u128 => i64   : upper       );
checked_cast!(u128 => i128  : upper       );
checked_cast!(u128 => usize : upper       );
checked_cast!(u128 => isize : upper       );

// i8
checked_cast!(i8 => u8     : lower       );
checked_cast!(i8 => u16    : lower       );
checked_cast!(i8 => u32    : lower       );
checked_cast!(i8 => u64    : lower       );
checked_cast!(i8 => u128   : lower       );
checked_cast!(i8 => i8     : infallible  );
checked_cast!(i8 => i16    : infallible  );
checked_cast!(i8 => i32    : infallible  );
checked_cast!(i8 => i64    : infallible  );
checked_cast!(i8 => i128   : infallible  );
checked_cast!(i8 => usize  : lower       );
checked_cast!(i8 => isize  : infallible  );
checked_cast!(i8 => f32   : infallible  );
checked_cast!(i8 => f64   : infallible  );

// i16
checked_cast!(i16 => u8    : both        );
checked_cast!(i16 => u16   : lower       );
checked_cast!(i16 => u32   : lower       );
checked_cast!(i16 => u64   : lower       );
checked_cast!(i16 => u128  : lower       );
checked_cast!(i16 => i8    : both        );
checked_cast!(i16 => i16   : infallible  );
checked_cast!(i16 => i32   : infallible  );
checked_cast!(i16 => i64   : infallible  );
checked_cast!(i16 => i128  : infallible  );
checked_cast!(i16 => usize : lower       );
checked_cast!(i16 => isize : infallible  );
checked_cast!(i16 => f32   : infallible  );
checked_cast!(i16 => f64   : infallible  );

// i32
checked_cast!(i32 => u8    : both        );
checked_cast!(i32 => u16   : both        );
checked_cast!(i32 => u32   : lower       );
checked_cast!(i32 => u64   : lower       );
checked_cast!(i32 => u128  : lower       );
checked_cast!(i32 => i8    : both        );
checked_cast!(i32 => i16   : both        );
checked_cast!(i32 => i32   : infallible  );
checked_cast!(i32 => i64   : infallible  );
checked_cast!(i32 => i128  : infallible  );
checked_cast!(i32 => usize : 16: both,   32: lower      , 64: lower             );
checked_cast!(i32 => isize : 16: both,   32: infallible , 64: infallible        );

// i64
checked_cast!(i64 => u8    : both        );
checked_cast!(i64 => u16   : both        );
checked_cast!(i64 => u32   : both        );
checked_cast!(i64 => u64   : lower       );
checked_cast!(i64 => u128  : lower       );
checked_cast!(i64 => i8    : both        );
checked_cast!(i64 => i16   : both        );
checked_cast!(i64 => i32   : both        );
checked_cast!(i64 => i64   : infallible  );
checked_cast!(i64 => i128  : infallible  );
checked_cast!(i64 => usize : 16: both,   32: both      , 64: lower             );
checked_cast!(i64 => isize : 16: both,   32: both      , 64: infallible        );

// i128
checked_cast!(i128 => u8   : both        );
checked_cast!(i128 => u16  : both        );
checked_cast!(i128 => u32  : both        );
checked_cast!(i128 => u64  : both        );
checked_cast!(i128 => u128 : lower       );
checked_cast!(i128 => i8   : both        );
checked_cast!(i128 => i16  : both        );
checked_cast!(i128 => i32  : both        );
checked_cast!(i128 => i64  : both        );
checked_cast!(i128 => i128 : infallible  );
checked_cast!(i128 => usize: both        );
checked_cast!(i128 => isize: both        );

// usize
checked_cast!(usize => u8   : upper                                              );
checked_cast!(usize => u16  : 16: infallible, 32: upper     , 64: upper          );
checked_cast!(usize => u32  : 16: infallible, 32: infallible, 64: upper          );
checked_cast!(usize => u64  : 16: infallible, 32: infallible, 64: infallible     );
checked_cast!(usize => u128 : 16: infallible, 32: infallible, 64: infallible     );
checked_cast!(usize => i8   : upper                                              );
checked_cast!(usize => i16  : upper                                              );
checked_cast!(usize => i32  : 16: infallible, 32: upper     , 64: upper          );
checked_cast!(usize => i64  : 16: infallible, 32: infallible, 64: upper          );
checked_cast!(usize => i128 : 16: infallible, 32: infallible, 64: infallible     );
checked_cast!(usize => usize: infallible                                         );
checked_cast!(usize => isize: upper                                              );

// isize
checked_cast!(isize => u8   : both                                               );
checked_cast!(isize => u16  : 16: lower     , 32: both       , 64: both          );
checked_cast!(isize => u32  : 16: lower     , 32: lower      , 64: both          );
checked_cast!(isize => u64  : lower                                              );
checked_cast!(isize => u128 : lower                                              );
checked_cast!(isize => i8   : both                                               );
checked_cast!(isize => i16  : 16: infallible, 32: both      , 64: both           );
checked_cast!(isize => i32  : 16: infallible, 32: infallible, 64: both           );
checked_cast!(isize => i64  : 16: infallible, 32: infallible, 64: infallible     );
checked_cast!(isize => i128 : 16: infallible, 32: infallible, 64: infallible     );
checked_cast!(isize => usize: lower                                              );
checked_cast!(isize => isize: infallible                                         );

// f32
checked_cast!(f32 => u8);
checked_cast!(f32 => u16);
checked_cast!(f32 => i8);
checked_cast!(f32 => i16);
checked_cast!(f32 => f32: infallible);
checked_cast!(f32 => f64: infallible);

// f64
checked_cast!(f64 => u8);
checked_cast!(f64 => u16);
checked_cast!(f64 => i8);
checked_cast!(f64 => i16);
checked_cast!(f64 => f64: infallible);

macro_rules! wrapping_cast {
    ($lhs: ty=>$rhs:ty) => {
        impl WrappingCast<$rhs> for $lhs {
            fn wrapping_cast(self) -> $rhs {
                self as $rhs
            }
        }
    };
}

wrapping_cast!(u8    => i8   );
wrapping_cast!(u16   => i16  );
wrapping_cast!(u32   => i32  );
wrapping_cast!(u64   => i64  );
wrapping_cast!(u128  => i128 );
wrapping_cast!(usize => isize);
wrapping_cast!(i8    => u8   );
wrapping_cast!(i16   => u16  );
wrapping_cast!(i32   => u32  );
wrapping_cast!(i64   => u64  );
wrapping_cast!(i128  => u128 );
wrapping_cast!(isize => usize);

macro_rules! extending_cast {
    ($lhs: ty=>$rhs:ty) => {
        impl ExtendingCast<$rhs> for $lhs {
            fn extending_cast(self) -> $rhs {
                self as $rhs
            }
        }
    };
}

extending_cast!(u8  => u16  );
extending_cast!(u8  => u32  );
extending_cast!(u8  => u64  );
extending_cast!(u8  => u128 );
extending_cast!(u16 => u32  );
extending_cast!(u16 => u64  );
extending_cast!(u16 => u128 );
extending_cast!(u32 => u64  );
extending_cast!(u32 => u128 );
extending_cast!(u64 => u128 );
extending_cast!(i8  => i16  );
extending_cast!(i8  => i32  );
extending_cast!(i8  => i64  );
extending_cast!(i8  => i128 );
extending_cast!(i16 => i32  );
extending_cast!(i16 => i64  );
extending_cast!(i16 => i128 );
extending_cast!(i32 => i64  );
extending_cast!(i32 => i128 );
extending_cast!(i64 => i128 );

macro_rules! truncating_cast {
    ($lhs: ty=>$rhs:ty) => {
        impl TruncatingCast<$rhs> for $lhs {
            fn truncating_cast(self) -> $rhs {
                self as $rhs
            }
        }
    };
}

truncating_cast!(u16  => u8  );
truncating_cast!(u32  => u8  );
truncating_cast!(u64  => u8  );
truncating_cast!(u128 => u8  );
truncating_cast!(u32  => u16 );
truncating_cast!(u64  => u16 );
truncating_cast!(u128 => u16 );
truncating_cast!(u64  => u32 );
truncating_cast!(u128 => u32 );
truncating_cast!(u128 => u64 );
truncating_cast!(i16  => i8  );
truncating_cast!(i32  => i8  );
truncating_cast!(i64  => i8  );
truncating_cast!(i128 => i8  );
truncating_cast!(i32  => i16 );
truncating_cast!(i64  => i16 );
truncating_cast!(i128 => i16 );
truncating_cast!(i64  => i32 );
truncating_cast!(i128 => i32 );
truncating_cast!(i128 => i64 );
