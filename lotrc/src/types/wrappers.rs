#[cfg(feature = "python")]
use pyo3::prelude::*;
use zerocopy::{ByteOrder, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, BE, LE};

use super::{Crc, OrderedDataStrict};

mod wrapper {
    use super::*;
    use zerocopy::{F32, I16, I32, U16, U32, U64};
    macro_rules! make_wrapper {
        ($name:ident, $wrapper:ident, $inner:ident) => {
            #[derive(
                Copy, Clone, Debug, Immutable, KnownLayout, FromBytes, IntoBytes, Unaligned,
            )]
            #[repr(transparent)]
            pub struct $name<T: ByteOrder>($wrapper<T>);

            impl<T: ByteOrder> Default for $name<T> {
                fn default() -> Self {
                    Self($wrapper::default())
                }
            }

            impl<T: ByteOrder> $name<T> {
                #[inline(always)]
                pub const fn new(val: $inner) -> Self {
                    Self($wrapper::new(val))
                }
                #[inline(always)]
                pub const fn get(&self) -> $inner {
                    self.0.get()
                }
            }
            impl<T: ByteOrder> From<$inner> for $name<T> {
                #[inline(always)]
                fn from(value: $inner) -> Self {
                    Self(value.into())
                }
            }
            impl<T: ByteOrder> From<$name<T>> for $inner {
                #[inline(always)]
                fn from(value: $name<T>) -> Self {
                    value.0.get()
                }
            }
            impl<T: ByteOrder> From<&$inner> for $name<T> {
                #[inline(always)]
                fn from(value: &$inner) -> Self {
                    Self((*value).into())
                }
            }
            impl<T: ByteOrder> From<&$name<T>> for $inner {
                #[inline(always)]
                fn from(value: &$name<T>) -> Self {
                    value.0.get()
                }
            }
            impl<T: ByteOrder> From<&$name<T>> for $name<T> {
                #[inline(always)]
                fn from(value: &$name<T>) -> Self {
                    Self(value.0)
                }
            }

            impl<T: ByteOrder> std::fmt::Display for $name<T> {
                #[inline(always)]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }
            #[cfg(feature = "python")]
            impl<'py, T: ByteOrder> FromPyObject<'_, 'py> for $name<T> {
                type Error = PyErr;
                #[inline(always)]
                fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
                    Ok(Self($wrapper::new($inner::extract(obj)?)))
                }
            }
            #[cfg(feature = "python")]
            impl<'py, T: ByteOrder> IntoPyObject<'py> for $name<T> {
                type Target = <$inner as IntoPyObject<'py>>::Target;
                type Output = <$inner as IntoPyObject<'py>>::Output;
                type Error = <$inner as IntoPyObject<'py>>::Error;
                #[inline(always)]
                fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                    self.0.get().into_pyobject(py)
                }
            }
            #[cfg(feature = "python")]
            impl<'a, 'py, T: ByteOrder> IntoPyObject<'py> for &'a $name<T> {
                type Target = <$inner as IntoPyObject<'py>>::Target;
                type Output = <$inner as IntoPyObject<'py>>::Output;
                type Error = <$inner as IntoPyObject<'py>>::Error;
                #[inline(always)]
                fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                    self.0.get().into_pyobject(py)
                }
            }
        };
    }
    make_wrapper!(WrappedU64, U64, u64);
    make_wrapper!(WrappedU32, U32, u32);
    make_wrapper!(WrappedI32, I32, i32);
    make_wrapper!(WrappedF32, F32, f32);
    make_wrapper!(WrappedU16, U16, u16);
    make_wrapper!(WrappedI16, I16, i16);

    impl<T: ByteOrder> std::hash::Hash for WrappedU32<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state)
        }
    }

    impl<T: ByteOrder> std::cmp::PartialEq for WrappedU32<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<T: ByteOrder> std::cmp::Eq for WrappedU32<T> {}
}

use wrapper::*;

#[derive(
    Debug, Copy, Clone, PartialEq, KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned,
)]
#[repr(transparent)]
pub struct WrappedU8(u8);
impl WrappedU8 {
    pub fn new<T: Into<WrappedU8>>(value: T) -> Self {
        value.into()
    }
    pub fn get(&self) -> u8 {
        self.0
    }
}
impl Default for WrappedU8 {
    fn default() -> Self {
        Self(u8::default())
    }
}
impl From<u8> for WrappedU8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<&u8> for WrappedU8 {
    fn from(value: &u8) -> Self {
        Self(*value)
    }
}
impl From<&WrappedU8> for WrappedU8 {
    fn from(value: &WrappedU8) -> Self {
        Self(value.0)
    }
}
impl From<WrappedU8> for u8 {
    fn from(value: WrappedU8) -> Self {
        value.0
    }
}
impl From<&WrappedU8> for u8 {
    fn from(value: &WrappedU8) -> Self {
        value.0
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, KnownLayout, Immutable, FromBytes, IntoBytes, Unaligned,
)]
#[repr(transparent)]
pub struct WrappedI8(i8);
impl WrappedI8 {
    pub fn new<T: Into<WrappedI8>>(value: T) -> Self {
        value.into()
    }
    pub fn get(&self) -> i8 {
        self.0
    }
}
impl Default for WrappedI8 {
    fn default() -> Self {
        Self(i8::default())
    }
}
impl From<i8> for WrappedI8 {
    fn from(value: i8) -> Self {
        Self(value)
    }
}
impl From<&i8> for WrappedI8 {
    fn from(value: &i8) -> Self {
        Self(*value)
    }
}
impl From<&WrappedI8> for WrappedI8 {
    fn from(value: &WrappedI8) -> Self {
        Self(value.0)
    }
}
impl From<WrappedI8> for i8 {
    fn from(value: WrappedI8) -> Self {
        value.0
    }
}
impl From<&WrappedI8> for i8 {
    fn from(value: &WrappedI8) -> Self {
        value.0
    }
}

impl<T: ByteOrder> From<WrappedU32<T>> for Crc {
    fn from(value: WrappedU32<T>) -> Self {
        Self { val: value.get() }
    }
}
impl<T: ByteOrder> From<&WrappedU32<T>> for Crc {
    fn from(value: &WrappedU32<T>) -> Self {
        Self { val: value.get() }
    }
}

impl<T: ByteOrder> From<Crc> for WrappedU32<T> {
    fn from(value: Crc) -> Self {
        Self::new(value.val)
    }
}
impl<T: ByteOrder> From<&Crc> for WrappedU32<T> {
    fn from(value: &Crc) -> Self {
        Self::new(value.val)
    }
}

impl<T: ByteOrder> From<usize> for WrappedU16<T> {
    fn from(value: usize) -> Self {
        (value as u16).into()
    }
}

impl<T: ByteOrder> From<usize> for WrappedU32<T> {
    fn from(value: usize) -> Self {
        (value as u32).into()
    }
}

macro_rules! make_ordered {
    ($name:ident, $x:ident) => {
        impl OrderedDataStrict for $x {
            type PC = $name<LE>;
            type XBOX = $name<BE>;
            type PS3 = $name<BE>;
        }
    };
}

make_ordered!(WrappedU64, u64);
make_ordered!(WrappedU32, u32);
make_ordered!(WrappedU32, Crc);
make_ordered!(WrappedI32, i32);
make_ordered!(WrappedF32, f32);
make_ordered!(WrappedU16, u16);
make_ordered!(WrappedI16, i16);

pub type F32Pc = WrappedF32<LE>;
pub type F32Xbox = WrappedF32<BE>;
pub type F32Ps3 = WrappedF32<BE>;

pub type CrcPc = WrappedU32<LE>;
pub type CrcXbox = WrappedU32<BE>;
pub type CrcPs3 = WrappedU32<BE>;

pub type U16Pc = WrappedU16<LE>;
pub type U16Xbox = WrappedU16<BE>;
pub type U16Ps3 = WrappedU16<BE>;

pub type U32Pc = WrappedU32<LE>;
pub type U32Xbox = WrappedU32<BE>;
pub type U32Ps3 = WrappedU32<BE>;

pub type U64Pc = WrappedU64<LE>;
pub type U64Xbox = WrappedU64<BE>;
pub type U64Ps3 = WrappedU64<BE>;

pub type I16Pc = WrappedI16<LE>;
pub type I16Xbox = WrappedI16<BE>;
pub type I16Ps3 = WrappedI16<BE>;

pub type I32Pc = WrappedI32<LE>;
pub type I32Xbox = WrappedI32<BE>;
pub type I32Ps3 = WrappedI32<BE>;

impl OrderedDataStrict for u8 {
    type PC = WrappedU8;
    type XBOX = WrappedU8;
    type PS3 = WrappedU8;
}
impl OrderedDataStrict for i8 {
    type PC = WrappedI8;
    type XBOX = WrappedI8;
    type PS3 = WrappedI8;
}
