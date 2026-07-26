#![allow(non_camel_case_types)]
use lotrc_proc::make_endian;

pub type ref_slice<'a, T> = &'a [T];
pub type mut_slice<'a, T> = &'a mut [T];
pub type slice<T> = Box<[T]>;
pub type string<'a> = &'a str;

pub trait OrderedData<T> {
    fn conv(&self) -> T;
}
pub trait OrderedDataTrivial<T: From<Self>>: Copy {}
impl<T: OrderedDataTrivial<U>, U: From<T>> OrderedData<U> for T {
    fn conv(&self) -> U {
        (*self).into()
    }
}

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

#[make_endian]
mod endian_wrappers_xe_ {
    use super::{OrderedData, OrderedDataTrivial};

    pub type f32_XE_ = rend::f32_xe_;
    pub type u16_XE_ = rend::u16_xe_;
    pub type u32_XE_ = rend::u32_xe_;
    pub type u64_XE_ = rend::u64_xe_;
    pub type i16_XE_ = rend::i16_xe_;
    pub type i32_XE_ = rend::i32_xe_;

    pub type U16_XE_ = zerocopy::U16<zerocopy::_XE_>;
    pub type U32_XE_ = zerocopy::U32<zerocopy::_XE_>;
    pub type I32_XE_ = zerocopy::I32<zerocopy::_XE_>;

    impl OrderedDataTrivial<f32_XE_> for f32 {}
    impl OrderedDataTrivial<f32> for f32_XE_ {}

    impl OrderedDataTrivial<u16_XE_> for u16 {}
    impl OrderedDataTrivial<u16> for u16_XE_ {}

    impl OrderedDataTrivial<u32_XE_> for u32 {}
    impl OrderedDataTrivial<u32> for u32_XE_ {}

    impl OrderedDataTrivial<u64_XE_> for u64 {}
    impl OrderedDataTrivial<u64> for u64_XE_ {}

    impl OrderedDataTrivial<i16_XE_> for i16 {}
    impl OrderedDataTrivial<i16> for i16_XE_ {}

    impl OrderedDataTrivial<i32_XE_> for i32 {}
    impl OrderedDataTrivial<i32> for i32_XE_ {}
    
    // unaligned versions
    impl OrderedDataTrivial<u16> for U16_XE_ {}
    impl OrderedDataTrivial<U16_XE_> for u16 {}
    impl OrderedDataTrivial<u32> for U32_XE_ {}
    impl OrderedDataTrivial<U32_XE_> for u32 {}
    impl OrderedDataTrivial<i32> for I32_XE_ {}
    impl OrderedDataTrivial<I32_XE_> for i32 {}

    impl OrderedData<U16_XE_> for usize {
        #[inline(always)]
        fn conv(&self) -> U16_XE_ {
            (*self as u16).into()
        }
    }
    impl OrderedData<usize> for U16_XE_ {
        #[inline(always)]
        fn conv(&self) -> usize {
            self.get() as usize
        }
    }
    impl OrderedData<u16_XE_> for usize {
        #[inline(always)]
        fn conv(&self) -> u16_XE_ {
            (*self as u16).into()
        }
    }
    impl OrderedData<usize> for u16_XE_ {
        #[inline(always)]
        fn conv(&self) -> usize {
            self.to_native() as usize
        }
    }
    impl OrderedData<u32_XE_> for usize {
        #[inline(always)]
        fn conv(&self) -> u32_XE_ {
            (*self as u32).into()
        }
    }
    impl OrderedData<usize> for u32_XE_ {
        #[inline(always)]
        fn conv(&self) -> usize {
            self.to_native() as usize
        }
    }
    impl OrderedData<u16> for u32_XE_ {
        #[inline(always)]
        fn conv(&self) -> u16 {
            self.to_native() as u16 
        }
    }
    impl OrderedData<u64_XE_> for usize {
        #[inline(always)]
        fn conv(&self) -> u64_XE_ {
            (*self as u64).into()
        }
    }
    impl OrderedData<usize> for u64_XE_ {
        #[inline(always)]
        fn conv(&self) -> usize {
            self.to_native() as usize
        }
    }
    impl OrderedData<i32_XE_> for usize {
        #[inline(always)]
        fn conv(&self) -> i32_XE_ {
            (*self as i32).into()
        }
    }
    impl OrderedData<usize> for i32_XE_ {
        #[inline(always)]
        fn conv(&self) -> usize {
            self.to_native() as usize
        }
    }
}

#[make_endian]
pub use endian_wrappers_xe_::*;

