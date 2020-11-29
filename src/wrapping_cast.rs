pub trait WrappingCast {
    type Target;
    fn wrapping_cast(self) -> Self::Target;
}

macro_rules! impl_cast {
    ($($lhs:ty => $rhs:ty, $f:ident;)+) => {
        $(
        impl WrappingCast for $lhs {
            type Target = $rhs;

            #[inline]
            fn wrapping_cast(self) -> Self::Target {
                $f(self)
            }
        }
        )+

        #[test]
        fn size_eq(){
            use core::mem;
            $(
                assert_eq!(mem::size_of::<$lhs>(), mem::size_of::<$rhs>());
            )+
        }

        $(
            #[inline]
            #[must_use]
            pub const fn $f(x: $lhs)->$rhs{
                x as $rhs
            }
        )+
    };
}

impl_cast!(
    u8 => i8,       u8_to_i8;
    u16 => i16,     u16_to_i16;
    u32 => i32,     u32_to_i32;
    u64 => i64,     u64_to_i64;
    u128 => i128,   u128_to_i128;
    usize => isize, usize_to_isize;

    i8 => u8,       i8_to_u8;
    i16 => u16,     i16_to_u16;
    i32 => u32,     i32_to_u32;
    i64 => u64,     i64_to_u64;
    i128 => u128,   i128_to_u128;
    isize => usize, isize_to_usize;
);
