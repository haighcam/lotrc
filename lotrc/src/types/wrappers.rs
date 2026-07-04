#![allow(non_camel_case_types)]
use lotrc_proc::{make_platforms};
use crate::types::Crc;

pub type ref_slice<'a, T> = &'a [T];
pub type mut_slice<'a, T> = &'a mut [T];
pub type slice<T> = Box<[T]>;
pub type string<'a> = &'a str;

pub use rend::{
    f32_le, u16_le, u32_le, u64_le, i16_le, i32_le,
    f32_be, u16_be, u32_be, u64_be, i16_be, i32_be,
};

pub type U16LE = zerocopy::U16<zerocopy::LE>;
pub type U16BE = zerocopy::U16<zerocopy::BE>;
pub type U32LE = zerocopy::U32<zerocopy::LE>;
pub type U32BE = zerocopy::U32<zerocopy::BE>;
pub type I32LE = zerocopy::I32<zerocopy::LE>;
pub type I32BE = zerocopy::I32<zerocopy::BE>;

pub trait GetNative {
    type Native;
    fn get(&self) -> Self::Native;
}
macro_rules! impl_get_native {
    ($name:ident, $alt:ident) => {
        impl GetNative for $name {
            type Native = $alt;
            #[inline(always)]
            fn get(&self) -> Self::Native {
                self.to_native()
            }
        }
    }
}
impl_get_native!(f32_le, f32);
impl_get_native!(f32_be, f32);
impl_get_native!(u16_le, u16);
impl_get_native!(u16_be, u16);
impl_get_native!(u32_le, u32);
impl_get_native!(u32_be, u32);
impl_get_native!(u64_le, u64);
impl_get_native!(u64_be, u64);
impl_get_native!(i16_le, i16);
impl_get_native!(i16_be, i16);
impl_get_native!(i32_le, i32);
impl_get_native!(i32_be, i32);

pub trait OrderedData<T> {
    fn conv(&self) -> T;
}
pub trait OrderedDataTrivial<T: From<Self>>: Copy {}
impl<T: OrderedDataTrivial<U>, U: From<T>> OrderedData<U> for T {
    fn conv(&self) -> U {
        (*self).into()
    }
}
impl OrderedDataTrivial<f32_le> for f32 {}
impl OrderedDataTrivial<f32_be> for f32 {}
impl OrderedDataTrivial<f32> for f32_le {}
impl OrderedDataTrivial<f32> for f32_be {}

impl OrderedDataTrivial<u16_le> for u16 {}
impl OrderedDataTrivial<u16_be> for u16 {}
impl OrderedDataTrivial<u16> for u16_le {}
impl OrderedDataTrivial<u16> for u16_be {}

impl OrderedDataTrivial<u32_le> for u32 {}
impl OrderedDataTrivial<u32_be> for u32 {}
impl OrderedDataTrivial<u32> for u32_le {}
impl OrderedDataTrivial<u32> for u32_be {}

impl OrderedDataTrivial<u64_le> for u64 {}
impl OrderedDataTrivial<u64_be> for u64 {}
impl OrderedDataTrivial<u64> for u64_le {}
impl OrderedDataTrivial<u64> for u64_be {}

impl OrderedDataTrivial<i16_le> for i16 {}
impl OrderedDataTrivial<i16_be> for i16 {}
impl OrderedDataTrivial<i16> for i16_le {}
impl OrderedDataTrivial<i16> for i16_be {}

impl OrderedDataTrivial<i32_le> for i32 {}
impl OrderedDataTrivial<i32_be> for i32 {}
impl OrderedDataTrivial<i32> for i32_le {}
impl OrderedDataTrivial<i32> for i32_be {}

impl OrderedData<u8> for u8 {
    fn conv(&self) -> u8 {
        *self
    }
}
impl OrderedData<i8> for i8 {
    fn conv(&self) -> i8 {
        *self
    }
}

pub trait OrderedDataStrict
where
    Self: Sized + Clone + Default + OrderedData<Self::Pc> + OrderedData<Self::Xbox> + OrderedData<Self::Ps3>,
    Self::Pc: zerocopy::Immutable
        + zerocopy::KnownLayout
        + zerocopy::FromBytes
        + zerocopy::IntoBytes
        + Clone
        + std::fmt::Debug
        + Default
        + OrderedData<Self>,
    Self::Xbox: zerocopy::Immutable
        + zerocopy::KnownLayout
        + zerocopy::FromBytes
        + zerocopy::IntoBytes
        + Clone
        + std::fmt::Debug
        + Default
        + OrderedData<Self>,
    Self::Ps3: zerocopy::Immutable
        + zerocopy::KnownLayout
        + zerocopy::FromBytes
        + zerocopy::IntoBytes
        + Clone
        + std::fmt::Debug
        + Default
        + OrderedData<Self>,
{
    type Pc;
    type Xbox;
    type Ps3;
    const SIZE_PC: usize = std::mem::size_of::<Self::Pc>();
    const SIZE_XBOX: usize = std::mem::size_of::<Self::Xbox>();
    const SIZE_PS3: usize = std::mem::size_of::<Self::Ps3>();
}

impl OrderedDataStrict for f32 {
    type Pc = f32_le;
    type Xbox = f32_be;
    type Ps3 = f32_be;
}
pub type f32Pc = f32_le;
pub type f32Xbox = f32_be;
pub type f32Ps3 = f32_be;
impl OrderedDataStrict for i16 {
    type Pc = i16_le;
    type Xbox = i16_be;
    type Ps3 = i16_be;
}
pub type i16Pc = i16_le;
pub type i16Xbox = i16_be;
pub type i16Ps3 = i16_be;
impl OrderedDataStrict for i32 {
    type Pc = i32_le;
    type Xbox = i32_be;
    type Ps3 = i32_be;
}
pub type i32Pc = i32_le;
pub type i32Xbox = i32_be;
pub type i32Ps3 = i32_be;
impl OrderedDataStrict for u16 {
    type Pc = u16_le;
    type Xbox = u16_be;
    type Ps3 = u16_be;
}
pub type u16Pc = u16_le;
pub type u16Xbox = u16_be;
pub type u16Ps3 = u16_be;
impl OrderedDataStrict for u32 {
    type Pc = u32_le;
    type Xbox = u32_be;
    type Ps3 = u32_be;
}
pub type u32Pc = u32_le;
pub type u32Xbox = u32_be;
pub type u32Ps3 = u32_be;
impl OrderedDataStrict for u64 {
    type Pc = u64_le;
    type Xbox = u64_be;
    type Ps3 = u64_be;
}
pub type u64Pc = u64_le;
pub type u64Xbox = u64_be;
pub type u64Ps3 = u64_be;

// unaligned versions
pub type U16Pc = U16LE;
pub type U16Xbox = U16BE;
pub type U16Ps3 = U16BE;
impl OrderedDataTrivial<u16> for U16LE {}
impl OrderedDataTrivial<u16> for U16BE {}
impl OrderedDataTrivial<U16LE> for u16 {}
impl OrderedDataTrivial<U16BE> for u16 {}
pub type U32Pc = U32LE;
pub type U32Xbox = U32LE;
pub type U32Ps3 = U32BE;
impl OrderedDataTrivial<u32> for U32LE {}
impl OrderedDataTrivial<u32> for U32BE {}
impl OrderedDataTrivial<U32LE> for u32 {}
impl OrderedDataTrivial<U32BE> for u32 {}
pub type I32Pc = I32LE;
pub type I32Xbox = I32BE;
pub type I32Ps3 = I32BE;
impl OrderedDataTrivial<i32> for I32LE {}
impl OrderedDataTrivial<i32> for I32BE {}
impl OrderedDataTrivial<I32LE> for i32 {}
impl OrderedDataTrivial<I32BE> for i32 {}

#[make_platforms]
pub type u8VER = u8;
impl OrderedDataStrict for u8 {
    type Pc = u8;
    type Xbox = u8;
    type Ps3 = u8;
}
impl OrderedDataStrict for i8 {
    type Pc = i8;
    type Xbox = i8;
    type Ps3 = i8;
}

impl OrderedData<U16LE> for usize {
    fn conv(&self) -> U16LE {
        (*self as u16).into()
    }
}
impl OrderedData<U16BE> for usize {
    fn conv(&self) -> U16BE {
        (*self as u16).into()
    }
}
impl OrderedData<u16_le> for usize {
    fn conv(&self) -> u16_le {
        (*self as u16).into()
    }
}
impl OrderedData<u16_be> for usize {
    fn conv(&self) -> u16_be {
        (*self as u16).into()
    }
}
impl OrderedData<u32_le> for usize {
    fn conv(&self) -> u32_le {
        (*self as u32).into()
    }
}
impl OrderedData<u32_be> for usize {
    fn conv(&self) -> u32_be {
        (*self as u32).into()
    }
}
impl OrderedData<u64_le> for usize {
    fn conv(&self) -> u64_le {
        (*self as u64).into()
    }
}
impl OrderedData<u64_be> for usize {
    fn conv(&self) -> u64_be {
        (*self as u64).into()
    }
}

#[make_platforms]
pub type CrcVER = u32VER;
impl OrderedData<Crc> for U32LE {
    fn conv(&self) -> Crc {
        self.get().into()
    }
}
impl OrderedData<Crc> for U32BE {
    fn conv(&self) -> Crc {
        self.get().into()
    }
}
impl OrderedData<U32LE> for Crc {
    fn conv(&self) -> U32LE {
        self.get().into()
    }
}
impl OrderedData<U32BE> for Crc {
    fn conv(&self) -> U32BE {
        self.get().into()
    }
}
impl OrderedData<Crc> for u32_le {
    fn conv(&self) -> Crc {
        self.get().into()
    }
}
impl OrderedData<Crc> for u32_be {
    fn conv(&self) -> Crc {
        self.get().into()
    }
}
impl OrderedData<u32_le> for Crc {
    fn conv(&self) -> u32_le {
        self.get().into()
    }
}
impl OrderedData<u32_be> for Crc {
    fn conv(&self) -> u32_be {
        self.get().into()
    }
}
impl OrderedDataStrict for Crc {
    type Pc = CrcPc;
    type Xbox = CrcXbox;
    type Ps3 = CrcPs3;
}
