#![allow(non_camel_case_types)]
use bytemuck::{Pod, Zeroable};

pub type ref_slice<'a, T> = &'a [T];
pub type mut_slice<'a, T> = &'a mut [T];
pub type slice<T> = Box<[T]>;
pub type string<'a> = &'a str;

pub trait BaseTypes: std::fmt::Debug + Copy + Clone + Default + PartialEq + std::hash::Hash + Zeroable  + 'static {
    #[allow(non_camel_case_types)]
    type u16: Pod + std::fmt::Debug + Copy + Default + PartialEq + From<u16> + Into<u16>;
    #[allow(non_camel_case_types)]
    type u32: Pod + std::fmt::Debug + Copy + Default + PartialEq + From<u32> + Into<u32>;
    #[allow(non_camel_case_types)]
    type u64: Pod + std::fmt::Debug + Copy + Default + PartialEq + From<u64> + Into<u64>;
    #[allow(non_camel_case_types)]
    type i16: Pod + std::fmt::Debug + Copy + Default + PartialEq + From<i16> + Into<i16>;
    #[allow(non_camel_case_types)]
    type i32: Pod + std::fmt::Debug + Copy + Default + PartialEq + From<i32> + Into<i32>;
    #[allow(non_camel_case_types)]
    type f32: Pod + std::fmt::Debug + Copy + Default + PartialEq + From<f32> + Into<f32>;

    // should be unaligned
    type U16: Pod + std::fmt::Debug + Copy + Default + PartialEq + From<u16> + Into<u16>;
    type U32: Pod + std::fmt::Debug + Copy + Default + PartialEq + From<u32> + Into<u32>;
    type I32: Pod + std::fmt::Debug + Copy + Default + PartialEq + From<i32> + Into<i32>;

    // some things are always LE
    #[allow(non_camel_case_types)]
    type u32LE: Pod + std::fmt::Debug + Copy + Default + PartialEq + From<u32> + Into<u32>;
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Hash, Zeroable)]
pub struct LE;

#[cfg(target_endian = "big")]
impl BaseTypes for LE {
    type u16 = rend::u16_le;
    type u32 = rend::u32_le;
    type u64 = rend::u64_le;
    type i16 = rend::i16_le;
    type i32 = rend::i32_le;
    type f32 = rend::f32_le;
    type U16 = U16LE;
    type U32 = U32LE;
    type I32 = I32LE;
    type u32LE = rend::u32_le;
}

#[cfg(target_endian = "little")]
impl BaseTypes for LE {
    type u16 = u16;
    type u32 = u32;
    type u64 = u64;
    type i16 = i16;
    type i32 = i32;
    type f32 = f32;
    type U16 = U16LE;
    type U32 = U32LE;
    type I32 = I32LE;
    type u32LE = u32;
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Hash, Zeroable)]
pub struct BE;

#[cfg(target_endian = "little")]
impl BaseTypes for BE {
    type u16 = rend::u16_be;
    type u32 = rend::u32_be;
    type u64 = rend::u64_be;
    type i16 = rend::i16_be;
    type i32 = rend::i32_be;
    type f32 = rend::f32_be;
    type U16 = U16BE;
    type U32 = U32BE;
    type I32 = I32BE;
    type u32LE = u32;
}
#[cfg(target_endian = "big")]
impl BaseTypes for BE {
    type u16 = u16;
    type u32 = u32;
    type u64 = u64;
    type i16 = i16;
    type i32 = i32;
    type f32 = f32;
    type U16 = U16BE;
    type U32 = U32BE;
    type I32 = I32BE;
    type u32LE = rend::u32_le;
}

#[cfg(target_endian = "little")]
pub type NE = LE;
#[cfg(target_endian = "big")]
pub type NE = BE;

macro_rules! unaligned_ty {
    ($name:ident, $n:literal, $ty:ty, $c_to:ident, $c_from:ident) => {
        #[derive(Copy, Clone, Default, PartialEq, Zeroable, Pod)]
        #[repr(transparent)]
        pub struct $name([u8; $n]);

        impl From<$ty> for $name {
            #[inline]
            fn from(val: $ty) -> Self {
                Self(val.$c_to())
            }
        }
        impl From<$name> for $ty {
            #[inline]
            fn from(val: $name) -> Self {
                Self::$c_from(val.0)
            }
        }
        impl std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                <$ty>::from(*self).fmt(fmt)
            }
        }
    }
}

macro_rules! unaligned_pair {
    ($name_le:ident, $name_be:ident, $n:literal, $ty:ty) => {
        unaligned_ty!($name_le, $n, $ty, to_le_bytes, from_le_bytes);
        unaligned_ty!($name_be, $n, $ty, to_be_bytes, from_be_bytes);

        impl From<$name_le> for $name_be {
            #[inline]
            fn from(val: $name_le) -> Self {
                <$ty>::from(val).into()
            }
        }
        impl From<$name_be> for $name_le {
            #[inline]
            fn from(val: $name_be) -> Self {
                <$ty>::from(val).into()
            }
        }
    }
}

unaligned_pair!(U16LE, U16BE, 2, u16);
unaligned_pair!(U32LE, U32BE, 4, u32);
unaligned_pair!(I32LE, I32BE, 4, i32);

