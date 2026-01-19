#![allow(non_camel_case_types)]
use lotrc_proc::{make_platforms};
use crate::types::Crc;

#[cfg(feature = "ffi")]
mod wrappers {
    use std::hash::Hash;
    use std::cmp::Eq;
    use std::fmt::{Debug, Display};
    use indexmap::{IndexMap, Equivalent, map::{Values, Keys, Iter}};
    macro_rules! make_wrapper_unaligned {
        ($name:ident, $base:path, $alt:path, $conv:ident) => {
            #[derive(Copy, Clone, Default, zerocopy::Immutable, zerocopy::KnownLayout, zerocopy::IntoBytes, zerocopy::FromBytes, zerocopy::Unaligned)]
            #[allow(non_camel_case_types)]
            #[repr(transparent)]
            pub struct $name($base);
            wrapper_impl!($name, $base, $alt, $conv);
        }
    }
    macro_rules! make_wrapper {
        ($name:ident, $base:path, $alt:path, $conv:ident) => {
            #[derive(Copy, Clone, Default, zerocopy::Immutable, zerocopy::KnownLayout, zerocopy::IntoBytes, zerocopy::FromBytes)]
            #[allow(non_camel_case_types)]
            #[repr(transparent)]
            pub struct $name($base);
            wrapper_impl!($name, $base, $alt, $conv);
        }
    }

    macro_rules! wrapper_impl {
        ($name:ident, $base:path, $alt:path, $conv:ident) => {
            impl $name {
                #[inline(always)]
                pub fn get(&self) -> $alt {
                    self.0.$conv()
                }
            }
            impl From<$alt> for $name {
                #[inline(always)]
                fn from(val: $alt) -> Self {
                    Self(val.into())
                }
            }
            impl From<$name> for $alt {
                #[inline(always)]
                fn from(val: $name) -> Self {
                    val.0.into()
                }
            }
            impl Debug for $name {
                #[inline(always)]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Debug::fmt(&self.0, f)
                }
            }
            impl Display for $name {
                #[inline(always)]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Display::fmt(&self.0, f)
                }
            }
            #[cfg(feature = "ffi")]
            unsafe impl safer_ffi::layout::ReprC for $name {
                type CLayout = <$alt as safer_ffi::layout::ReprC>::CLayout;
                #[inline(always)]
                fn is_valid(it: &Self::CLayout) -> bool {
                    <$alt as safer_ffi::layout::ReprC>::is_valid(it)
                }
            }
        }
    }

    make_wrapper!(f32_le, rend::f32_le, f32, to_native);
    make_wrapper!(u16_le, rend::u16_le, u16, to_native);
    make_wrapper!(u32_le, rend::u32_le, u32, to_native);
    make_wrapper!(u64_le, rend::u64_le, u64, to_native);
    make_wrapper!(i16_le, rend::i16_le, i16, to_native);
    make_wrapper!(i32_le, rend::i32_le, i32, to_native);
    make_wrapper!(f32_be, rend::f32_be, f32, to_native);
    make_wrapper!(u16_be, rend::u16_be, u16, to_native);
    make_wrapper!(u32_be, rend::u32_be, u32, to_native);
    make_wrapper!(u64_be, rend::u64_be, u64, to_native);
    make_wrapper!(i16_be, rend::i16_be, i16, to_native);
    make_wrapper!(i32_be, rend::i32_be, i32, to_native);

    make_wrapper_unaligned!(U16LE, zerocopy::U16<zerocopy::LE>, u16, get);
    make_wrapper_unaligned!(U16BE, zerocopy::U16<zerocopy::BE>, u16, get);
    make_wrapper_unaligned!(U32LE, zerocopy::U32<zerocopy::LE>, u32, get);
    make_wrapper_unaligned!(U32BE, zerocopy::U32<zerocopy::BE>, u32, get);
    make_wrapper_unaligned!(I32LE, zerocopy::I32<zerocopy::LE>, i32, get);
    make_wrapper_unaligned!(I32BE, zerocopy::I32<zerocopy::BE>, i32, get);

    pub type slice<'a, T> = safer_ffi::slice::slice_ref<'a, T>;
    pub type str_ref<'a> = safer_ffi::string::str_ref<'a>;
    pub type box_slice<T> = safer_ffi::boxed::slice_boxed<T>;

    #[safer_ffi::derive_ReprC]
    #[repr(opaque)]
    #[repr(transparent)]
    #[derive(Clone)]
    pub struct MapImpl<K, V>(IndexMap<K,V>);

    impl<K, V> From<IndexMap<K, V>> for MapImpl<K, V> {
        fn from(val: IndexMap<K, V>) -> Self {
            Self(val)
        }
    }

    impl<K, V> std::ops::Deref for MapImpl<K, V> {
        type Target = IndexMap<K, V>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<K, V> Default for MapImpl<K, V> {
        fn default() -> Self {
            Self(IndexMap::default())
        }
    }
    
    pub type Map<K,V> = safer_ffi::boxed::Box_<MapImpl<K, V>>;

    impl<K: Hash + Eq, V> MapImpl<K,V> {
        #[inline(always)]
        pub fn get<Q: Hash + Equivalent<K>>(&self, key: &Q) -> Option<&V> {
            self.0.get(key)
        }
        #[inline(always)]
        pub fn insert(&mut self, key: K, val: V) -> Option<V> {
            self.0.insert(key, val)
        }
        #[inline(always)]
        pub fn with_capacity(size: usize) -> Self {
            Self(IndexMap::with_capacity(size))
        }
        #[inline(always)]
        pub fn len(&self) -> usize {
            self.0.len()
        }
        #[inline(always)]
        pub fn values(&self) -> Values<'_, K, V> {
            self.0.values()
        }
        #[inline(always)]
        pub fn keys(&self) -> Keys<'_, K, V> {
            self.0.keys()
        }
        #[inline(always)]
        pub fn iter(&self) -> Iter<'_, K, V> {
            self.0.iter()
        }
        #[inline(always)]
        pub fn last(&self) -> Option<(&K, &V)> {
            self.0.last()
        }
        #[inline(always)]
        pub fn get_index_of<Q: Hash + Equivalent<K>>(&self, key: &Q) -> Option<usize> {
            self.0.get_index_of(key)
        }
        #[inline(always)]
        pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
            self.0.get_index(index)
        }
        #[inline(always)]
        pub fn entry(&mut self, key: K) -> indexmap::map::Entry<'_, K, V> {
            self.0.entry(key)
        }
    }
    impl<K: Send, V: Send> MapImpl<K, V> {
        #[inline(always)]
        pub fn par_values_mut(&mut self) -> indexmap::map::rayon::ParValuesMut<'_, K, V> {
            self.0.par_values_mut()
        }
    }
    impl<K: Debug, V: Debug> Debug for MapImpl<K,V> {
        #[inline(always)]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&self.0, f)
        }
    }
    impl<K: Hash + Eq, V> FromIterator<(K, V)> for MapImpl<K, V> {
        #[inline(always)]
        fn from_iter<I: IntoIterator<Item=(K, V)>>(iter: I) -> Self {
            Self(IndexMap::from_iter(iter))
        }
    }
    impl<K, V> From<MapImpl<K, V>> for Map<K, V> {
        fn from(val: MapImpl<K, V>) -> Self {
            Self::new(val)
        }
    }
}

#[cfg(not(feature = "ffi"))]
mod wrappers {
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

    pub type slice<'a, T> = &'a [T];
    pub type str_ref<'a> = &'a str;
    pub type box_slice<T> = Box<[T]>;

    pub trait AsSlice<'a> {
        type T;
        fn as_slice(self) -> &'a [Self::T];
    }
    impl<'a, T> AsSlice<'a> for &'a [T] {
        type T = T;
        fn as_slice(self) -> &'a [Self::T] {
            self
        }
    }

    pub type MapImpl<K, V> = indexmap::IndexMap<K, V>;
    pub type Map<K, V> = MapImpl<K, V>;
    pub trait GetNative {
        type Native;
        fn get(&self) -> Self::Native;
    }
    impl GetNative for f32_le {
        type Native = f32;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for f32_be {
        type Native = f32;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for u16_le {
        type Native = u16;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for u16_be {
        type Native = u16;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for u32_le {
        type Native = u32;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for u32_be {
        type Native = u32;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for u64_le {
        type Native = u64;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for u64_be {
        type Native = u64;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for i16_le {
        type Native = i16;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for i16_be {
        type Native = i16;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for i32_le {
        type Native = i32;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
    impl GetNative for i32_be {
        type Native = i32;
        #[inline(always)]
        fn get(&self) -> Self::Native {
            self.to_native()
        }
    }
}
pub use wrappers::*;

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
