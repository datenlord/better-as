use core::fmt::{self, Display};

pub trait CheckedCast<U> {
    type Output;

    fn checked_cast(self) -> Self::Output;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumCastError {
    Infinite,
    NaN,
    Overflow,
    Underflow,
}

impl Display for NumCastError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Self::Infinite => "Cannot store infinite value in finite type",
            Self::NaN => "Cannot store NaN in type which does not support it",
            Self::Overflow => "Overflow during numeric conversion",
            Self::Underflow => "Underflow during numeric conversion",
        };

        f.write_str(msg)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NumCastError {}

macro_rules! impl_cast {
    (@trivial $($t:ty,)+) => {
        $(
            impl CheckedCast<Self> for $t {
                type Output = Self;
                fn checked_cast(self) -> Self::Output {
                    self
                }
            }
        )+
    };

    (@extending $($lhs:ty => ($($rhs:ty),+),)+) => {
        $(
            $(
                impl CheckedCast<$rhs> for $lhs {
                    type Output = $rhs;
                    fn checked_cast(self) -> Self::Output {
                        self as $rhs
                    }
                }
            )+
        )+

        #[test]
        fn size_lt(){
            use core::mem;
            $(
                $(
                    assert!(mem::size_of::<$lhs>() < mem::size_of::<$rhs>());
                )+
            )+
        }
    };

    (@truncating @unsigned $($lhs:ty => ($($rhs:ty),+),)+) => {
        $(
            $(
                impl CheckedCast<$rhs> for $lhs {
                    type Output = Result<$rhs, NumCastError>;
                    fn checked_cast(self) -> Self::Output {
                        if self <= <$rhs>::max_value() as Self {
                            Ok(self as $rhs)
                        }else{
                            Err(NumCastError::Overflow)
                        }
                    }
                }
            )+
        )+

        #[test]
        fn size_gt_unsigned(){
            use core::mem;
            $(
                $(
                    assert!(mem::size_of::<$lhs>() > mem::size_of::<$rhs>());
                )+
            )+
        }
    };

    (@truncating @signed $($lhs:ty => ($($rhs:ty),+),)+) => {
        $(
            $(
                impl CheckedCast<$rhs> for $lhs {
                    type Output = Result<$rhs, NumCastError>;
                    fn checked_cast(self) -> Self::Output {
                        if self <= <$rhs>::max_value() as Self {
                            if self >= <$rhs>::min_value() as Self {
                                Ok(self as $rhs)
                            }else{
                                Err(NumCastError::Underflow)
                            }
                        }else{
                            Err(NumCastError::Overflow)
                        }
                    }
                }
            )+
        )+

        #[test]
        fn size_gt_signed(){
            use core::mem;
            $(
                $(
                    assert!(mem::size_of::<$lhs>() > mem::size_of::<$rhs>());
                )+
            )+
        }
    };

    (@wrapping @u2i $($lhs:ty => $rhs:ty,)+) => {
        $(
            impl CheckedCast<$rhs> for $lhs {
                type Output = Result<$rhs, NumCastError>;
                fn checked_cast(self) -> Self::Output {
                    if self <= <$rhs>::max_value() as Self {
                        Ok(self as $rhs)
                    }else{
                        Err(NumCastError::Overflow)
                    }
                }
            }
        )+
    };

    (@wrapping @i2u $($lhs:ty => $rhs:ty,)+) => {
        $(
            impl CheckedCast<$rhs> for $lhs {
                type Output = Result<$rhs, NumCastError>;
                fn checked_cast(self) -> Self::Output {
                    if self >= 0 {
                        Ok(self as $rhs)
                    }else{
                        Err(NumCastError::Underflow)
                    }
                }
            }
        )+
    };
}

impl_cast!(@trivial
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
);

impl_cast!(@extending
    u8  => (u16, u32, u64, u128, usize, f32, f64),
    u16 => (u32, u64, u128, f32, f64),
    u32 => (u64, u128),
    u64 => (u128),
    i8  => (i16, i32, i64, i128, isize, f32, f64),
    i16 => (i32, i64, i128, f32, f64),
    i32 => (i64, i128),
    i64 => (i128),
    f32 => (f64),
);

impl_cast!(@truncating @unsigned
    u128  => (u8, u16, u32, u64),
    u64   => (u8, u16, u32),
    u32   => (u8, u16),
    u16   => (u8),
    usize => (u8),
);

impl_cast!(@truncating @signed
    i128  => (i8, i16, i32, i64),
    i64   => (i8, i16, i32),
    i32   => (i8, i16),
    i16   => (i8),
    isize => (i8),
);

impl_cast!(@wrapping @u2i
    usize => isize,
    u128  => i128,
    u64   => i64,
    u32   => i32,
    u16   => i16,
    u8    => i8,
);

impl_cast!(@wrapping @i2u
    isize => usize,
    i128  => u128,
    i64   => u64,
    i32   => u32,
    i16   => u16,
    i8    => u8,
);

impl CheckedCast<f32> for f64 {
    type Output = Result<f32, NumCastError>;

    fn checked_cast(self) -> Self::Output {
        if self.is_nan() || self.is_infinite() {
            Ok(self as f32)
        } else if self < f32::MIN as Self {
            Err(NumCastError::Underflow)
        } else if self > f32::MAX as Self {
            Err(NumCastError::Overflow)
        } else {
            Ok(self as f32)
        }
    }
}
