pub trait TruncatingCast<U> {
    fn truncating_cast(self) -> U;
}

macro_rules! impl_cast {
    ($($lhs:ty => $rhs:ty, $f:ident;)+) => {
        $(
            impl TruncatingCast<$rhs> for $lhs {
                #[inline]
                fn truncating_cast(self) -> $rhs {
                    self as $rhs
                }
            }
        )+

        #[test]
        fn size_gt(){
            use core::mem;
            $(
               assert!(mem::size_of::<$lhs>() > mem::size_of::<$rhs>());
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
    u16   => u8,  u16_to_u8  ;
    u32   => u8,  u32_to_u8  ;
    u32   => u16, u32_to_u16 ;
    u64   => u8,  u64_to_u8  ;
    u64   => u16, u64_to_u16 ;
    u64   => u32, u64_to_u32 ;
    u128  => u8,  u128_to_u8 ;
    u128  => u16, u128_to_u16;
    u128  => u32, u128_to_u32;
    u128  => u64, u128_to_u64;
    usize => u8,  usize_to_u8;

    i16   => i8,  i16_to_i8  ;
    i32   => i8,  i32_to_i8  ;
    i32   => i16, i32_to_i16 ;
    i64   => i8,  i64_to_i8  ;
    i64   => i16, i64_to_i16 ;
    i64   => i32, i64_to_i32 ;
    i128  => i8,  i128_to_i8 ;
    i128  => i16, i128_to_i16;
    i128  => i32, i128_to_i32;
    i128  => i64, i128_to_i64;
    isize => i8,  isize_to_i8;
);
