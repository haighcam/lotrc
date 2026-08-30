use crate::{
    types::{Crc, ReadData, align_offset, DumpSlice, CompressedDataRef, ref_slice, slice, OwnedCompressedData, BaseTypes, NE, CompressedData},
    level::pak::block1::infos::{InfoCounts, DumpInfos}
};
use anyhow::{anyhow, Context, Result};
use log::{warn};
use lotrc_proc::{derive_pod};
use indexmap::IndexMap;
use std::ptr::NonNull;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum AnimVals1Ref<'a, T: BaseTypes> {
    Type1(ref_slice<'a, u8>),
    Type2(ref_slice<'a, T::u16>),
    Type3(ref_slice<'a, T::u16>),
    Type4(ref_slice<'a, T::u16>),
}

impl<'a, T: BaseTypes> AnimVals1Ref<'a, T> {
    pub fn from_data(src: &'a [u8], num: usize, kind: u8) -> Result<Self> {
        Ok(match kind {
            0 => Self::Type1(u8::slice_from_data(src, num).context("type1")?.into()),
            1 => Self::Type2(T::u16::slice_from_data(src, num).context("type2")?.into()),
            2 => Self::Type3(T::u16::slice_from_data(src, num).context("type3")?.into()),
            3 => Self::Type4(T::u16::slice_from_data(src, num).context("type4")?.into()),
            _ => return Err(anyhow!("Illegal Type {} for spline data", kind)),
        })
    }
    pub fn empty(kind: u8) -> Result<Self> {
        Ok(match kind {
            0 => Self::Type1(Default::default()),
            1 => Self::Type2(Default::default()),
            2 => Self::Type3(Default::default()),
            3 => Self::Type4(Default::default()),
            _ => return Err(anyhow!("Illegal Type {} for spline data", kind)),
        })
    }
    pub fn kind(&self) -> u8 {
        match self {
            Self::Type1(_) => 0,
            Self::Type2(_) => 1,
            Self::Type3(_) => 2,
            Self::Type4(_) => 3,
        }
    }
    pub fn size(&self) -> usize {
        match self {
            Self::Type1(vals) => vals.len() * std::mem::size_of::<u8>(),
            Self::Type2(vals) => vals.len() * std::mem::size_of::<T::u16>(),
            Self::Type3(vals) => vals.len() * std::mem::size_of::<T::u16>(),
            Self::Type4(vals) => vals.len() * std::mem::size_of::<T::u16>(),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            Self::Type1(vals) => vals.len(),
            Self::Type2(vals) => vals.len(),
            Self::Type3(vals) => vals.len(),
            Self::Type4(vals) => vals.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnimVals1 {
    Type1(Vec<u8>),
    Type2(Vec<u16>),
    Type3(Vec<u16>),
    Type4(Vec<u16>),
}

impl AnimVals1 {
    pub fn kind(&self) -> u8 {
        match self {
            Self::Type1(_) => 0,
            Self::Type2(_) => 1,
            Self::Type3(_) => 2,
            Self::Type4(_) => 3,
        }
    }
    pub fn len(&self) -> usize {
        match self {
            Self::Type1(vals) => vals.len(),
            Self::Type2(vals) => vals.len(),
            Self::Type3(vals) => vals.len(),
            Self::Type4(vals) => vals.len(),
        }
    }
}

impl<T: BaseTypes> From<&AnimVals1Ref<'_, T>> for AnimVals1 {
    fn from(val: &AnimVals1Ref<T>) -> Self {
        match val {
            AnimVals1Ref::Type1(vals) => Self::Type1(vals.to_vec()),
            AnimVals1Ref::Type2(vals) => Self::Type2(vals.iter().map(|&x| x.into()).collect()),
            AnimVals1Ref::Type3(vals) => Self::Type3(vals.iter().map(|&x| x.into()).collect()),
            AnimVals1Ref::Type4(vals) => Self::Type4(vals.iter().map(|&x| x.into()).collect()),
        }
    }
}

pub enum AnimVals1Dump<'a, T: BaseTypes> {
    Type1(&'a mut [u8]),
    Type2(&'a mut [T::u16]),
    Type3(&'a mut [T::u16]),
    Type4(&'a mut [T::u16]),
}

impl<'a, T: BaseTypes> AnimVals1Dump<'a, T> {
    fn mut_from_data(
        dst: &mut DumpSlice<'a>,
        kind: u8,
        count: usize,
    ) -> Result<Self> {
        match kind {
            0 => Ok(Self::Type1(ReadData::mut_slice_from_data(dst, count)?)),
            1 => Ok(Self::Type2(ReadData::mut_slice_from_data(dst, count)?)),
            2 => Ok(Self::Type3(ReadData::mut_slice_from_data(dst, count)?)),
            3 => Ok(Self::Type4(ReadData::mut_slice_from_data(dst, count)?)),
            _ => return Err(anyhow!("Illegal Type for spline thingy")),
        }
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct Obj1Ref<'a, T: BaseTypes> {
    pub flags: u8,
    pub s2: u8,
    pub s1: T::u16,
    pub data: ref_slice<'a, u8>,
    pub vals_a: ref_slice<'a, T::f32>,
    pub vals: AnimVals1Ref<'a, T>,
    pub size: usize,
}

impl<'a, T: BaseTypes> Obj1Ref<'a, T> {
    const COUNTS: [usize; 8] = [0, 1, 1, 2, 1, 2, 2, 3];
    pub fn from_data(src: &'a [u8], flags: u8, kind: u8) -> Result<Self> {
        let mut offset = 0;
        let start = offset;
        let (s1, s2, data) = if flags & 0xf0 != 0 {
            let &s1 = T::u16::from_data(&src[offset..]).context("s1")?;
            offset += std::mem::size_of::<T::u16>();
            let &s2 = u8::from_data(&src[offset..]).context("s2")?;
            offset += std::mem::size_of::<u8>();
            let data = u8::slice_from_data(&src[offset..], s1.into() as usize + s2 as usize + 2).context("data")?;
            offset += std::mem::size_of_val(data);
            (s1, s2, data.into())
        } else {
            (0u16.into(), 0, ref_slice::default())       
        };
        let vals_a = if flags != 0 {
            offset = align_offset(offset, 4);
            let num = Self::COUNTS[(flags & 7) as usize]
                + 2 * Self::COUNTS[(((flags >> 4) & !flags) & 7) as usize];
            let vals_a = T::f32::slice_from_data(&src[offset..], num).context("vals_a")?;
            offset += std::mem::size_of_val(vals_a);
            vals_a.into()
        } else {
            ref_slice::default() 
        };
        let vals = if flags & 0xf0 != 0 {
            offset = align_offset(offset, 2);
            let num = Self::COUNTS[((flags >> 4) & 7) as usize] * (s1.into() as usize + 1);
            let vals = AnimVals1Ref::from_data(&src[offset..], num, kind).context("vals")?;
            offset += vals.size();
            vals
        } else {
           AnimVals1Ref::empty(kind).context("vals")?
        };
        Ok(Self {
            flags,
            data,
            s1,
            s2,
            vals_a,
            vals,
            size: offset - start
        })
    }
}

#[derive(Debug, Clone)]
pub struct Obj1 {
    pub flags: u8,
    pub s2: u8,
    pub s1: u16,
    pub data: Vec<u8>,
    pub vals_a: Vec<f32>,
    pub vals: AnimVals1,
}

impl<T: BaseTypes> From<&Obj1Ref<'_, T>> for Obj1 {
    fn from(val: &Obj1Ref<T>) -> Self {
        Self {
            flags: val.flags,
            s1: val.s1.into(),
            s2: val.s2,
            data: (&val.data[..]).into(),
            vals_a: val
                .vals_a
                .iter()
                .map(|&x| x.into())
                .collect(),
            vals: (&val.vals).into(),
        }
    }
}

pub trait DumpObj1<T: BaseTypes> {
    fn flags(&self) -> u8;
    fn write_s1(&self, s1: &mut T::u16) -> Result<()>;
    fn write_s2(&self, s2: &mut u8) -> Result<()>;
    fn data_len(&self) -> usize;
    fn vals_a_len(&self) -> usize;
    fn write_data(&self, data: &mut [u8]) -> Result<()>;
    fn write_vals_a(&self, vals_a: &mut [T::f32]) -> Result<()>;
    fn write_vals(&self, vals: AnimVals1Dump<T>) -> Result<()>;
    fn vals_kind(&self) -> u8;
    fn vals_len(&self) -> usize;

    fn vals_size(&self) -> usize {
        match self.vals_kind() {
            0 => self.vals_len(),
            1 | 2 | 3 => self.vals_len() * std::mem::size_of::<T::u16>(),
            _ => panic!("Invalid vals kind")
        }
    }

    fn dump_into(
        &self,
        dst: &mut DumpSlice
    ) -> Result<()> {
        if self.flags() & 0xf0 != 0 {
            let s1 = T::u16::mut_from_data(dst).context("s1")?;
            self.write_s1(s1).context("write s1")?;
            let s2 = u8::mut_from_data(dst).context("s2")?;
            self.write_s2(s2).context("write s2")?;
            let data = u8::mut_slice_from_data(dst, self.data_len()).context("data")?;
            self.write_data(data).context("write data")?;
        }
        if self.flags() != 0 {
            dst.align(4)?;
            let vals_a = T::f32::mut_slice_from_data(dst, self.vals_a_len()).context("vals_a")?;
            self.write_vals_a(vals_a).context("write vals_a")?;
        }
        if self.flags() & 0xf0 != 0 {
            dst.align(2)?;
            let vals = AnimVals1Dump::mut_from_data(dst, self.vals_kind(), self.vals_len()).context("vals")?;
            self.write_vals(vals).context("write vals")?;
        }
        Ok(())
    }

    fn size(&self) -> usize {
        let mut size = 0;
        if self.flags() & 0xf0 != 0 {
            size += std::mem::size_of::<T::u16>();
            size += std::mem::size_of::<u8>();
            size += self.data_len();
        }
        if self.flags() != 0 {
            size = align_offset(size, 4);
            size += std::mem::size_of::<T::f32>() * self.vals_a_len();
        }
        if self.flags() != 0xf0 {
            size = align_offset(size, 2);
            size += self.vals_size();
        }
        size
    }
}

impl<T: BaseTypes> DumpObj1<T> for Obj1Ref<'_, T> {
    fn data_len(&self) -> usize {
        self.data.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals_kind(&self) -> u8 {
        self.vals.kind()
    }
    fn vals_a_len(&self) -> usize {
        self.vals_a.len()
    }
    fn flags(&self) -> u8 {
        self.flags
    }
    fn write_s1(&self, s1: &mut T::u16) -> Result<()> {
        *s1 = self.s1;
        Ok(())
    }
    fn write_s2(&self, s2: &mut u8) -> Result<()> {
        *s2 = self.s2;
        Ok(())
    }
    fn write_data(&self, data: &mut [u8]) -> Result<()> {
        data.copy_from_slice(self.data);
        Ok(())
    }
    fn write_vals(&self, vals: AnimVals1Dump<T>) -> Result<()> {
        match (&self.vals, vals) {
            (AnimVals1Ref::Type1(src), AnimVals1Dump::Type1(dst)) => dst.copy_from_slice(src),
            (AnimVals1Ref::Type2(src), AnimVals1Dump::Type2(dst)) => dst.copy_from_slice(src),
            (AnimVals1Ref::Type3(src), AnimVals1Dump::Type3(dst)) => dst.copy_from_slice(src),
            (AnimVals1Ref::Type4(src), AnimVals1Dump::Type4(dst)) => dst.copy_from_slice(src),
            _ => return Err(anyhow!("missmatched obj1type and dump obj1type"))
        };
        Ok(())
    }
    fn write_vals_a(&self, vals_a: &mut [T::f32]) -> Result<()> {
        vals_a.copy_from_slice(self.vals_a);
        Ok(())
    }
}

impl<T: BaseTypes> DumpObj1<T> for Obj1 {
    fn data_len(&self) -> usize {
        self.data.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals_kind(&self) -> u8 {
        self.vals.kind()
    }
    fn vals_a_len(&self) -> usize {
        self.vals_a.len()
    }
    fn flags(&self) -> u8 {
        self.flags
    }
    fn write_s1(&self, s1: &mut T::u16) -> Result<()> {
        *s1 = self.s1.into();
        Ok(())
    }
    fn write_s2(&self, s2: &mut u8) -> Result<()> {
        *s2 = self.s2;
        Ok(())
    }
    fn write_data(&self, data: &mut [u8]) -> Result<()> {
        data.copy_from_slice(self.data.as_slice());
        Ok(())
    }
    fn write_vals(&self, vals: AnimVals1Dump<T>) -> Result<()> {
        match (&self.vals, vals) {
            (AnimVals1::Type1(src), AnimVals1Dump::Type1(dst)) => dst.copy_from_slice(src),
            (AnimVals1::Type2(src), AnimVals1Dump::Type2(dst)) => for (&src, dst) in src.iter().zip(dst) { *dst = src.into() },
            (AnimVals1::Type3(src), AnimVals1Dump::Type3(dst)) => for (&src, dst) in src.iter().zip(dst) { *dst = src.into() },
            (AnimVals1::Type4(src), AnimVals1Dump::Type4(dst)) => for (&src, dst) in src.iter().zip(dst) { *dst = src.into() },
            _ => return Err(anyhow!("missmatched obj1type and dump obj1type"))
        };
        Ok(())
    }
    fn write_vals_a(&self, vals_a: &mut [T::f32]) -> Result<()> {
        for (&src, dst) in self.vals_a.iter().zip(vals_a) {
            *dst = src.into();
        }
        Ok(())
    }
}

#[derive_pod]
pub struct RotationPolar32<T: BaseTypes> {
    a: T::u32,
}

#[derive_pod]
// should be (u8, u8, u8, u16) but for xbox into it is (u8, u8, u8, u8, u8)
pub struct RotationThreeComp40 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
}

#[derive_pod]
pub struct RotationThreeComp48<T: BaseTypes> {
    a: T::u16,
    b: T::u16,
    c: T::u16,
}

#[derive_pod]
pub struct RotationThreeComp24 {
    a: u8,
    b: u8,
    c: u8,
}

#[derive_pod]
pub struct RotationStraight16 {
    a: u8,
    b: u8,
}

#[derive_pod]
pub struct RotationUncompressed<T: BaseTypes> {
    a: T::f32,
    b: T::f32,
    c: T::f32,
    d: T::f32,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum RotationQuantizationRef<'a, T: BaseTypes> {
    Polar32(ref_slice<'a, RotationPolar32<T>>),
    ThreeComp40(ref_slice<'a, RotationThreeComp40>),
    ThreeComp48(ref_slice<'a, RotationThreeComp48<T>>),
    ThreeComp24(ref_slice<'a, RotationThreeComp24>),
    Straight16(ref_slice<'a, RotationStraight16>),
    Uncompressed(ref_slice<'a, RotationUncompressed<T>>),
}

impl<'a, T: BaseTypes> RotationQuantizationRef<'a, T> {
    pub fn from_data(src: &'a [u8], num: usize, kind: u8) -> Result<Self> {
        Ok(match kind {
            0 => Self::Polar32(RotationPolar32::slice_from_data(src, num).context("Polar32")?.into()),
            1 => Self::ThreeComp40(RotationThreeComp40::slice_from_data(src, num).context("ThreeComp40")?.into()),
            2 => Self::ThreeComp48(RotationThreeComp48::slice_from_data(src, num).context("ThreeComp48")?.into()),
            3 => Self::ThreeComp24(RotationThreeComp24::slice_from_data(src, num).context("ThreeComp24")?.into()),
            4 => Self::Straight16(RotationStraight16::slice_from_data(src, num).context("Straight16")?.into()),
            5 => Self::Uncompressed(RotationUncompressed::slice_from_data(src, num).context("Uncompressed")?.into()),
            _ => return Err(anyhow!("Invalid Rotation Compression method {}", kind))?,
        })
    }
    pub fn empty(kind: u8) -> Result<Self> {
        Ok(match kind {
            0 => Self::Polar32(Default::default()),
            1 => Self::ThreeComp40(Default::default()),
            2 => Self::ThreeComp48(Default::default()),
            3 => Self::ThreeComp24(Default::default()),
            4 => Self::Straight16(Default::default()),
            5 => Self::Uncompressed(Default::default()),
            _ => return Err(anyhow!("Invalid Rotation Compression method {}", kind))?,
        })
    }
    pub fn align(kind: u8) -> usize {
        match kind {
            0 => 4,
            1 => 1,
            2 => 2,
            3 => 1,
            4 => 2,
            5 => 4,
            _ => 0,
        }
    }
    pub fn kind(&self) -> u8 {
        match self{
            Self::Polar32(_) => 0,
            Self::ThreeComp40(_) => 1,
            Self::ThreeComp48(_) => 2,
            Self::ThreeComp24(_) => 3,
            Self::Straight16(_) => 4,
            Self::Uncompressed(_) => 5,
        }
    }
    pub fn size(&self) -> usize {
        match self{
            Self::Polar32(vals) => vals.len() * std::mem::size_of::<RotationPolar32::<T>>(),
            Self::ThreeComp40(vals) => vals.len() * std::mem::size_of::<RotationThreeComp40>(),
            Self::ThreeComp48(vals) => vals.len() * std::mem::size_of::<RotationThreeComp48::<T>>(),
            Self::ThreeComp24(vals) => vals.len() * std::mem::size_of::<RotationThreeComp24>(),
            Self::Straight16(vals) => vals.len() * std::mem::size_of::<RotationStraight16>(),
            Self::Uncompressed(vals) => vals.len() * std::mem::size_of::<RotationUncompressed::<T>>(),
        }
    }
    pub fn len(&self) -> usize {
        match self{
            Self::Polar32(vals) => vals.len(),
            Self::ThreeComp40(vals) => vals.len(),
            Self::ThreeComp48(vals) => vals.len(),
            Self::ThreeComp24(vals) => vals.len(),
            Self::Straight16(vals) => vals.len(),
            Self::Uncompressed(vals) => vals.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RotationQuantization {
    Polar32(Vec<RotationPolar32<NE>>),
    ThreeComp40(Vec<RotationThreeComp40>),
    ThreeComp48(Vec<RotationThreeComp48<NE>>),
    ThreeComp24(Vec<RotationThreeComp24>),
    Straight16(Vec<RotationStraight16>),
    Uncompressed(Vec<RotationUncompressed<NE>>),
}

impl RotationQuantization {
    pub fn kind(&self) -> u8 {
        match self{
            Self::Polar32(_) => 0,
            Self::ThreeComp40(_) => 1,
            Self::ThreeComp48(_) => 2,
            Self::ThreeComp24(_) => 3,
            Self::Straight16(_) => 4,
            Self::Uncompressed(_) => 5,
        }
    }
    pub fn len(&self) -> usize {
        match self{
            Self::Polar32(vals) => vals.len(),
            Self::ThreeComp40(vals) => vals.len(),
            Self::ThreeComp48(vals) => vals.len(),
            Self::ThreeComp24(vals) => vals.len(),
            Self::Straight16(vals) => vals.len(),
            Self::Uncompressed(vals) => vals.len(),
        }
    }
}

impl<T: BaseTypes> From<&RotationQuantizationRef<'_, T>> for RotationQuantization
where
    RotationPolar32<NE>: From<RotationPolar32<T>>,
    RotationThreeComp48<NE>: From<RotationThreeComp48<T>>,
    RotationUncompressed<NE>: From<RotationUncompressed<T>>,
{
    fn from(val: &RotationQuantizationRef<T>) -> Self {
        match val {
            RotationQuantizationRef::Polar32(vals) => Self::Polar32(vals.iter().map(|&x| x.into()).collect()),
            RotationQuantizationRef::ThreeComp40(vals) => Self::ThreeComp40(vals.iter().copied().collect()),
            RotationQuantizationRef::ThreeComp48(vals) => Self::ThreeComp48(vals.iter().map(|&x| x.into()).collect()),
            RotationQuantizationRef::ThreeComp24(vals) => Self::ThreeComp24(vals.iter().copied().collect()),
            RotationQuantizationRef::Straight16(vals) => Self::Straight16(vals.iter().copied().collect()),
            RotationQuantizationRef::Uncompressed(vals) => Self::Uncompressed(vals.iter().map(|&x| x.into()).collect()),
        }
    }
}

#[derive(Debug)]
pub enum RotationQuantizationDump<'a, T: BaseTypes> {
    Polar32(&'a mut [RotationPolar32<T>]),
    ThreeComp40(&'a mut [RotationThreeComp40]),
    ThreeComp48(&'a mut [RotationThreeComp48<T>]),
    ThreeComp24(&'a mut [RotationThreeComp24]),
    Straight16(&'a mut [RotationStraight16]),
    Uncompressed(&'a mut [RotationUncompressed<T>]),
}

impl<'a, T: BaseTypes> RotationQuantizationDump<'a, T> {
    pub fn mut_from_data(
        dst: &mut DumpSlice<'a>,
        count: usize,
        kind: u8,
    ) -> Result<Self> {
        match kind {
            0 => Ok(Self::Polar32(ReadData::mut_slice_from_data(dst, count)?)),
            1 => Ok(Self::ThreeComp40(ReadData::mut_slice_from_data(dst, count)?)),
            2 => Ok(Self::ThreeComp48(ReadData::mut_slice_from_data(dst, count)?)),
            3 => Ok(Self::ThreeComp24(ReadData::mut_slice_from_data(dst, count)?)),
            4 => Ok(Self::Straight16(ReadData::mut_slice_from_data(dst, count)?)),
            5 => Ok(Self::Uncompressed(ReadData::mut_slice_from_data(dst, count)?)),
            _ => return Err(anyhow!("Invalid Rotation Compression method")),
        }
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct Obj2Ref<'a, T: BaseTypes> {
    pub flags: u8,
    pub s2: u8,
    pub s1: T::u16,
    pub data: ref_slice<'a, u8>,
    pub vals: RotationQuantizationRef<'a, T>,
    pub size: usize
}

impl<'a, T: BaseTypes> Obj2Ref<'a, T> {
    pub fn from_data(src: &'a [u8], flags: u8, kind: u8) -> Result<Self> {
        let mut offset = 0;
        let start = offset;
        let (s1, s2, data) = if flags & 0xf0 != 0 {
            let &s1 = T::u16::from_data(&src[offset..]).context("s1")?;
            offset += std::mem::size_of::<T::u16>();
            let &s2 = u8::from_data(&src[offset..]).context("s2")?;
            offset += std::mem::size_of::<u8>();
            let data = u8::slice_from_data(&src[offset..], s1.into() as usize + s2 as usize + 2).context("data")?;
            offset += std::mem::size_of_val(data);
            (s1, s2, data.into())
        } else {
            (0u16.into(), 0, ref_slice::default())
        };
        let vals = if flags != 0 {
            let align = RotationQuantizationRef::<T>::align(kind);
            offset = ((offset + align - 1) & !(align - 1)) as usize;
            let vals = RotationQuantizationRef::from_data(&src[offset..], s1.into() as usize + 1, kind).context("vals")?;
            offset += vals.size();
            vals
        } else {
            RotationQuantizationRef::empty(kind).context("vals")?
        };
        Ok(Self {
            flags,
            s1,
            s2,
            data,
            vals,
            size: offset - start 
        })
    }
}

#[derive(Debug, Clone)]
pub struct Obj2 {
    pub flags: u8,
    pub s1: u16,
    pub s2: u8,
    pub data: Vec<u8>,
    pub vals: RotationQuantization,
}

impl<T: BaseTypes> From<&Obj2Ref<'_, T>> for Obj2
where
    RotationPolar32<NE>: From<RotationPolar32<T>>,
    RotationThreeComp48<NE>: From<RotationThreeComp48<T>>,
    RotationUncompressed<NE>: From<RotationUncompressed<T>>,
{
    fn from(val: &Obj2Ref<T>) -> Self {
        Self {
            flags: val.flags,
            s1: val.s1.into(),
            s2: val.s2,
            data: (&val.data[..]).into(),
            vals: (&val.vals).into(),
        }
    }
}
pub trait DumpObj2<T: BaseTypes> {
    fn flags(&self) -> u8;
    fn data_len(&self) -> usize;
    fn vals_len(&self) -> usize;
    fn vals_kind(&self) -> u8;
    fn write_s1(&self, s1: &mut T::u16) -> Result<()>;
    fn write_s2(&self, s2: &mut u8) -> Result<()>;
    fn write_data(&self, data: &mut [u8]) -> Result<()>;
    fn write_vals(&self, vals: RotationQuantizationDump<T>) -> Result<()>;

    fn vals_size(&self) -> usize {
        match self.vals_kind() {
            0 => self.vals_len() * std::mem::size_of::<RotationPolar32<T>>(),
            1 => self.vals_len() * std::mem::size_of::<RotationThreeComp40>(),
            2 => self.vals_len() * std::mem::size_of::<RotationThreeComp48<T>>(),
            3 => self.vals_len() * std::mem::size_of::<RotationThreeComp24>(),
            4 => self.vals_len() * std::mem::size_of::<RotationStraight16>(),
            5 => self.vals_len() * std::mem::size_of::<RotationUncompressed<T>>(),
            _ => panic!("Invalid vals kind")
        }
    }

    fn size(&self) -> usize {
        let mut size = 0;
        if self.flags() & 0xf0 != 0 {
            size += std::mem::size_of::<T::u16>();
            size += std::mem::size_of::<u8>();
            size += self.data_len();
        }
        if self.flags() != 0 {
            size = align_offset(size, RotationQuantizationRef::<T>::align(self.vals_kind()));

            size += self.vals_size();
        }
        size
    }

    fn dump_into(
        &self,
        dst: &mut DumpSlice
    ) -> Result<()> {
        if self.flags() & 0xf0 != 0 {
            let s1= T::u16::mut_from_data(dst).context("s1")?;
            self.write_s1(s1).context("write s1")?;
            let s2 = u8::mut_from_data(dst).context("s2")?;
            self.write_s2(s2).context("write s2")?;
            let data = u8::mut_slice_from_data(dst, self.data_len()).context("data")?;
            self.write_data(data).context("write data")?;
        }
        if self.flags() != 0 {
            dst.align(RotationQuantizationRef::<T>::align(self.vals_kind()))?;
            let vals = RotationQuantizationDump::mut_from_data(dst, self.vals_len(), self.vals_kind()).context("vals")?;
            self.write_vals(vals).context("write vals")?;
        }
        Ok(())
    }
}

impl<T: BaseTypes> DumpObj2<T> for Obj2Ref<'_, T> {
    fn flags(&self) -> u8 {
        self.flags
    }
    fn data_len(&self) -> usize {
        self.data.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals_kind(&self) -> u8 {
        self.vals.kind()
    }
    fn write_s1(&self, s1: &mut T::u16) -> Result<()> {
        *s1 = self.s1;
        Ok(())
    }
    fn write_s2(&self, s2: &mut u8) -> Result<()> {
        *s2 = self.s2;
        Ok(())
    }
    fn write_data(&self, data: &mut [u8]) -> Result<()> {
        data.copy_from_slice(&self.data[..]);
        Ok(())
    }
    fn write_vals(&self, vals: RotationQuantizationDump<T>) -> Result<()> {
        match (&self.vals, vals) {
            (RotationQuantizationRef::Polar32(src), RotationQuantizationDump::Polar32(dst)) => dst.copy_from_slice(src),
            (RotationQuantizationRef::ThreeComp40(src), RotationQuantizationDump::ThreeComp40(dst)) => dst.copy_from_slice(src),
            (RotationQuantizationRef::ThreeComp48(src), RotationQuantizationDump::ThreeComp48(dst)) => dst.copy_from_slice(src),
            (RotationQuantizationRef::ThreeComp24(src), RotationQuantizationDump::ThreeComp24(dst)) => dst.copy_from_slice(src),
            (RotationQuantizationRef::Straight16(src), RotationQuantizationDump::Straight16(dst)) => dst.copy_from_slice(src),
            (RotationQuantizationRef::Uncompressed(src), RotationQuantizationDump::Uncompressed(dst)) => dst.copy_from_slice(src),
            _ => return Err(anyhow!("missmatched rotation quantization and dump rotation quantization"))
        };
        Ok(())
    }
}

impl<T: BaseTypes> DumpObj2<T> for Obj2
where
    RotationPolar32<T>: From<RotationPolar32<NE>>,
    RotationThreeComp48<T>: From<RotationThreeComp48<NE>>,
    RotationUncompressed<T>: From<RotationUncompressed<NE>>,
{
    fn flags(&self) -> u8 {
        self.flags
    }
    fn data_len(&self) -> usize {
        self.data.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals_kind(&self) -> u8 {
        self.vals.kind()
    }
    fn write_s1(&self, s1: &mut T::u16) -> Result<()> {
        *s1 = self.s1.into();
        Ok(())
    }
    fn write_s2(&self, s2: &mut u8) -> Result<()> {
        *s2 = self.s2;
        Ok(())
    }
    fn write_data(&self, data: &mut [u8]) -> Result<()> {
        data.copy_from_slice(self.data.as_slice());
        Ok(())
    }
    fn write_vals(&self, vals: RotationQuantizationDump<T>) -> Result<()> {
        match (&self.vals, vals) {
            (RotationQuantization::Polar32(src), RotationQuantizationDump::Polar32(dst)) => for (&src, dst) in src.iter().zip(dst) { *dst = src.into() },
            (RotationQuantization::ThreeComp40(src), RotationQuantizationDump::ThreeComp40(dst)) => dst.copy_from_slice(src),
            (RotationQuantization::ThreeComp48(src), RotationQuantizationDump::ThreeComp48(dst)) => for (&src, dst) in src.iter().zip(dst) { *dst = src.into() },
            (RotationQuantization::ThreeComp24(src), RotationQuantizationDump::ThreeComp24(dst)) => dst.copy_from_slice(src),
            (RotationQuantization::Straight16(src), RotationQuantizationDump::Straight16(dst)) => dst.copy_from_slice(src),
            (RotationQuantization::Uncompressed(src), RotationQuantizationDump::Uncompressed(dst)) => for (&src, dst) in src.iter().zip(dst) { *dst = src.into() },
            _ => return Err(anyhow!("missmatched rotation quantization and dump rotation quantization"))
        };
        Ok(())
    }
}

#[derive_pod]
pub struct Flags {
    pub f: u8,
    pub a: u8,
    pub b: u8,
    pub c: u8,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct BlockValARef<'a, T: BaseTypes> {
    pub a: Obj1Ref<'a, T>,
    pub b: Obj2Ref<'a, T>,
    pub c: Obj1Ref<'a, T>,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct BlockValRef<'a, T: BaseTypes> {
    pub vals_a: slice<BlockValARef<'a, T>>,
    pub vals_b: slice<Obj1Ref<'a, T>>
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct BlocksRef<'a, T: BaseTypes> {
    pub block_starts: ref_slice<'a, T::u32>,
    pub block_starts2: ref_slice<'a, T::u32>,
    pub obj_c3: ref_slice<'a, T::u32>,
    pub obj_c4: ref_slice<'a, T::u32>,
    pub blocks: slice<BlockValRef<'a, T>>
}

impl<'a, T: BaseTypes> BlocksRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &AnimationInfo<T>) -> Result<Self> {
        let block_starts = T::u32::slice_from_data(&src[info.block_starts_offset.into() as usize..], info.block_starts_num.into() as usize).context("block_starts")?;
        let block_starts2 = T::u32::slice_from_data(&src[info.block_starts2_offset.into() as usize..], info.block_starts2_num.into() as usize).context("block_starts2")?;
        let obj_c3 = T::u32::slice_from_data(&src[info.obj_c3_offset.into() as usize..], info.obj_c3_num.into() as usize).context("obj_c3")?;
        let obj_c4 = T::u32::slice_from_data(&src[info.obj_c4_offset.into() as usize..], info.obj_c4_num.into() as usize).context("obj_c4")?;
        let mut blocks = Vec::with_capacity(block_starts.len());
        for (&start, &start2) in block_starts.iter().zip(block_starts2) {
            let off = (start.into() + info.block_offset.into()) as usize;
            let flags = Flags::slice_from_data(&src[off..], info.vals_num.into() as usize).context("flags")?;
            let flags2 = u8::slice_from_data(&src[off + std::mem::size_of_val(flags)..], info.vals2_num.into() as usize).context("flags2")?; 
            let mut off =
                (info.block_offset.into() + start.into() + info.data_offset.into()) as usize;
            let mut vals_a = Vec::with_capacity(flags.len());
            let mut vals_b = Vec::with_capacity(flags2.len());
            for flag in flags {
                let a = Obj1Ref::from_data(&src[off..], flag.a, flag.f & 3)?;
                off = align_offset(off + a.size, 4);
                let b = Obj2Ref::from_data(&src[off..], flag.b, (flag.f >> 2) & 0xf)?;
                off = align_offset(off + b.size, 4);
                let c = Obj1Ref::from_data(&src[off..], flag.c, (flag.f >> 6) & 3)?;
                off = align_offset(off + c.size, 4);
                vals_a.push(BlockValARef { a, b, c });
            }
            off = (info.block_offset.into() + start.into() + start2.into()) as usize;
            for flag in flags2 {
                let d = Obj1Ref::from_data(&src[off..], flag & 0xf9, (flag >> 1) & 3)?;
                off = align_offset(off + d.size, 4);
                vals_b.push(d);
            }
            blocks.push(BlockValRef {
                vals_a: vals_a.into_boxed_slice().into(),
                vals_b: vals_b.into_boxed_slice().into()
            });
        }
        Ok(Self {
            block_starts: block_starts.into(),
            block_starts2: block_starts2.into(),
            obj_c3: obj_c3.into(),
            obj_c4: obj_c4.into(),
            blocks: blocks.into_boxed_slice().into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Blocks {
    pub obj_c3: Vec<u32>,
    pub obj_c4: Vec<u32>,
    pub blocks: Vec<(Vec<(Obj1, Obj2, Obj1)>, Vec<Obj1>)>,
}

impl<T: BaseTypes> From<&BlocksRef<'_, T>> for Blocks
where
    RotationPolar32<NE>: From<RotationPolar32<T>>,
    RotationThreeComp48<NE>: From<RotationThreeComp48<T>>,
    RotationUncompressed<NE>: From<RotationUncompressed<T>>,
{
    fn from(val: &BlocksRef<T>) -> Self {
        Self {
            obj_c3: val
                .obj_c3
                .iter()
                .map(|&x| x.into()) 
                .collect(),
            obj_c4: val
                .obj_c4
                .iter()
                .map(|&x| x.into())
                .collect(),
            blocks: val.blocks.iter()
                .map(|BlockValRef { vals_a, vals_b }| (
                    vals_a.iter().map(|BlockValARef { a, b, c }| (a.into(), b.into(), c.into())).collect(),
                    vals_b.iter().map(|x| x.into()).collect()
                ))
                .collect()
        }
    }
}

pub trait DumpBlocks<T: BaseTypes> {
    fn blocks_len(&self) -> usize;
    fn block1_len(&self) -> usize;
    fn block2_len(&self) -> usize;
    fn obj_c3_len(&self) -> usize;
    fn obj_c4_len(&self) -> usize;

    fn blocks(
        &self,
    ) -> impl Iterator<
        Item = (
            impl Iterator<Item = (&impl DumpObj1<T>, &impl DumpObj2<T>, &impl DumpObj1<T>)>,
            impl Iterator<Item = &impl DumpObj1<T>>,
        ),
    >;
    fn write_obj_c3(&self, obj_c3: &mut [T::u32]) -> Result<()>;
    fn write_obj_c4(&self, obj_c4: &mut [T::u32]) -> Result<()>;

    fn dump_into(
        &self,
        dst: &mut DumpSlice,
        info: &mut AnimationInfo<T>,
        start: usize
    ) -> Result<()> {
        info.vals_num = (self.block1_len() as u32).into();
        info.vals2_num = (self.block2_len() as u32).into();
        info.data_offset = ((self.block1_len() * std::mem::size_of::<Flags>() + self.block2_len()) as u32).into();

        info.block_starts_offset = ((dst.offset - start) as u32).into();
        let block_starts = T::u32::mut_slice_from_data(dst, self.blocks_len()).context("block_starts")?;
        info.block_starts_num = (block_starts.len() as u32).into();

        info.block_starts2_offset = ((dst.offset - start) as u32).into();
        let block_starts2 = T::u32::mut_slice_from_data(dst, self.blocks_len()).context("block_starts2")?;
        info.block_starts2_num = (block_starts2.len() as u32).into();

        info.obj_c3_offset = ((dst.offset - start) as u32).into();
        let obj_c3 = T::u32::mut_slice_from_data(dst, self.obj_c3_len()).context("obj_c3")?;
        self.write_obj_c3(obj_c3).context("write obj_c3")?;
        info.obj_c3_num = (obj_c3.len() as u32).into();

        info.obj_c4_offset = ((dst.offset - start) as u32).into();
        let obj_c4 = T::u32::mut_slice_from_data(dst, self.obj_c4_len()).context("obj_c4")?;
        self.write_obj_c4(obj_c4).context("write obj_c4")?;
        info.obj_c4_num = (obj_c4.len() as u32).into();

        let start_off = dst.offset;
        info.block_offset = ((dst.offset - start) as u32).into();
        for (i, ((vals, vals2), (block_start, block_start2))) in self
            .blocks()
            .zip(block_starts.iter_mut().zip(block_starts2))
            .enumerate()
        {
            let start = dst.offset - start_off;
            *block_start = (start as u32).into();

            let flags = Flags::mut_slice_from_data(dst, self.block1_len()).with_context(|| format!("block {} flags", i))?;

            let flags2 = u8::mut_slice_from_data(dst, self.block2_len()).with_context(|| format!("block {} flags2", i))?;

            dst.adjusted_align(4, start_off)?;
            for (j, ((a, b, c), flag)) in vals.zip(flags).enumerate() {
                flag.f = a.vals_kind() | (b.vals_kind() << 2) | (c.vals_kind() << 6);
                flag.a = a.flags();
                flag.b = b.flags();
                flag.c = c.flags();
                a.dump_into(dst).with_context(|| format!("block {} a {}", i, j))?;
                dst.adjusted_align(4, start_off)?;
                b.dump_into(dst).with_context(|| format!("block {} b {}", i, j))?;
                dst.adjusted_align(4, start_off)?;
                c.dump_into(dst).with_context(|| format!("block {} c {}", i, j))?;
                dst.adjusted_align(4, start_off)?;
            }

            *block_start2 = ((dst.offset - start_off - start) as u32).into();
            for (j, (d, flag)) in vals2.zip(flags2).enumerate() {
                *flag = d.flags() | (d.vals_kind() << 1);
                d.dump_into(dst).with_context(|| format!("block {} d {}", i, j))?;
                dst.adjusted_align(4, start_off)?;
            }
            dst.adjusted_align(16, start_off)?;
        }
        info.block_size = ((dst.offset - start_off) as u32).into();
        Ok(())
    }

    fn size(&self) -> usize {
        let mut size = 0;
        size += std::mem::size_of::<T::u32>() * self.blocks_len() * 2;
        size += std::mem::size_of::<T::u32>() * (self.obj_c3_len() + self.obj_c4_len());
        let mut blocks_size = 0;
        for (vals, vals2) in self.blocks() {
            blocks_size += std::mem::size_of::<Flags>() * self.block1_len();
            blocks_size += std::mem::size_of::<u8>() * self.block2_len();
            blocks_size = align_offset(blocks_size, 4);
            for (a, b, c) in vals {
                blocks_size = align_offset(blocks_size + a.size(), 4);
                blocks_size = align_offset(blocks_size + b.size(), 4);
                blocks_size = align_offset(blocks_size + c.size(), 4);
            }
            for d in vals2 {
                blocks_size = align_offset(blocks_size + d.size(), 4);
            }
            blocks_size = align_offset(blocks_size, 16);
        }
        size + blocks_size
    }
}

impl<T: BaseTypes> DumpBlocks<T> for BlocksRef<'_, T> {
    fn blocks_len(&self) -> usize {
        self.blocks.len()
    }
    fn block1_len(&self) -> usize {
        self.blocks[0].vals_a.len()
    }
    fn block2_len(&self) -> usize {
        self.blocks[0].vals_b.len()
    }
    fn obj_c3_len(&self) -> usize {
        self.obj_c3.len()
    }
    fn obj_c4_len(&self) -> usize {
        self.obj_c4.len()
    }

    fn blocks(
        &self,
    ) -> impl Iterator<
        Item = (
            impl Iterator<Item = (&impl DumpObj1<T>, &impl DumpObj2<T>, &impl DumpObj1<T>)>,
            impl Iterator<Item = &impl DumpObj1<T>>,
        ),
    > {
        self.blocks
            .iter()
            .map(|vals| (vals.vals_a.iter().map(|BlockValARef { a, b, c }| (a, b, c)), vals.vals_b.iter()))
    }
    fn write_obj_c3(&self, obj_c3: &mut [T::u32]) -> Result<()> {
        obj_c3.copy_from_slice(self.obj_c3);
        Ok(())
    }
    fn write_obj_c4(&self, obj_c4: &mut [T::u32]) -> Result<()> {
        obj_c4.copy_from_slice(self.obj_c4);
        Ok(())
    }
}

impl<T: BaseTypes> DumpBlocks<T> for Blocks
where
    RotationPolar32<T>: From<RotationPolar32<NE>>,
    RotationThreeComp48<T>: From<RotationThreeComp48<NE>>,
    RotationUncompressed<T>: From<RotationUncompressed<NE>>,
{
    fn blocks_len(&self) -> usize {
        self.blocks.len()
    }
    fn block1_len(&self) -> usize {
        self.blocks[0].0.len()
    }
    fn block2_len(&self) -> usize {
        self.blocks[0].1.len()
    }
    fn obj_c3_len(&self) -> usize {
        self.obj_c3.len()
    }
    fn obj_c4_len(&self) -> usize {
        self.obj_c4.len()
    }

    fn blocks(
        &self,
    ) -> impl Iterator<
        Item = (
            impl Iterator<Item = (&impl DumpObj1<T>, &impl DumpObj2<T>, &impl DumpObj1<T>)>,
            impl Iterator<Item = &impl DumpObj1<T>>,
        ),
    > {
        self.blocks
            .iter()
            .map(|(vals, vals2)| (vals.iter().map(|(a,b,c)|(a,b,c)), vals2.iter()))
    }
    fn write_obj_c3(&self, obj_c3: &mut [T::u32]) -> Result<()> {
        for (&src, dst) in self.obj_c3.iter().zip(obj_c3) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_obj_c4(&self, obj_c4: &mut [T::u32]) -> Result<()> {
        for (&src, dst) in self.obj_c4.iter().zip(obj_c4) {
            *dst = src.into();
        }
        Ok(())
    }
}

#[derive_pod]
pub struct Obj5Header<T: BaseTypes> {
    pub obj_a_num: T::u32,
    pub obj_a_offset: T::u32,
    pub obj_b_num: T::u32,
    pub obj_b_offset: T::u32,
}

#[derive_pod]
pub struct Obj5Val<T: BaseTypes> {
    pub unk_0: T::f32,
    pub unk_1: T::f32,
    pub unk_2: T::f32,
    pub unk_3: T::f32,
    pub unk_4: T::f32,
    pub unk_5: T::f32,
    pub unk_6: T::f32,
}

#[derive_pod]
pub struct Obj3<T: BaseTypes> {
    pub t: T::f32,
    pub event: Crc<T>,
    pub dat_2: T::u32,
    pub dat_3: T::u32,
    pub dat_4: T::u32,
    pub dat_5: T::u32,
    pub dat_6: T::u32,
    pub dat_7: T::u32,
    pub dat_8: T::u32,
    pub dat_9: T::u32,
    pub dat_10: T::u32,
}

#[derive_pod]
pub struct AnimationInfo<T: BaseTypes> {
    pub key: Crc<T>,
    pub gamemodemask: T::i32,
    pub offset: T::u32,
    pub size: T::u32,
    pub kind: T::u32, // always 3
    pub unk_5: T::f32,
    pub vals_num: T::u32,
    pub vals2_num: T::u32,
    pub unk_8: T::u32,
    pub vala: T::u32,                 // numFrames
    pub unk_10: T::u32,               // numBlocks
    pub unk_11: T::u32,               // maxFramesPerBlock
    pub data_offset: T::u32,          // relative to block_starts, maskAndQuantizationSize
    pub unk_13: T::f32,               // blockDuration
    pub unk_14: T::f32,               // blockInverseDuration
    pub t_scale: T::f32,              // frameDuration
    pub block_starts_offset: T::u32,  //relative to block_offset, blockOffsets
    pub block_starts_num: T::u32,     // relative to block_starts, nunBlocks
    pub block_starts2_offset: T::u32, // floatBlockOffsets
    pub block_starts2_num: T::u32,    // numFloatBlocks
    pub obj_c3_offset: T::u32,        // unused, equal to block_start, transformOffsets
    pub obj_c3_num: T::u32,           // unused, equal to block_start, numTransforms
    pub obj_c4_offset: T::u32,        // floatOffsets
    pub obj_c4_num: T::u32,           // numFloats
    pub block_offset: T::u32,         // data
    pub block_size: T::u32,           // dataSize
    pub obj3_num: T::u32,             // numEvents
    pub obj3_offset: T::u32,          // eventOffsets
    pub bones_num1: T::u32,           // bones is at least this + obj1_num long
    pub unk_29: T::u32,
    pub obj1_num: T::u32,
    pub bones_offset: T::u32,
    pub unk_32: T::u32,
    pub obj1_offset: T::u32,
    pub obj2_offset: T::u32,
    pub obj2_num: T::u32,
    pub obj5_offset: T::u32, // to some object that contains offsets in pos 1 and 2 and a value in pos 0
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct AnimationRef<'a, T: BaseTypes> {
    pub info: &'a AnimationInfo<T>,
    pub obj1: ref_slice<'a, T::u32>,
    pub obj2: ref_slice<'a, T::u32>,
    pub obj3: ref_slice<'a, Obj3<T>>,
    pub bones: ref_slice<'a, Crc<T>>,
    pub obj5_header: Option<&'a Obj5Header<T>>,
    pub obj5_a: ref_slice<'a, Obj5Val<T>>,
    pub obj5_b: ref_slice<'a, Obj5Val<T>>,
    pub blocks: Option<BlocksRef<'a, T>>,
    pub size: usize
}

impl<'a, T: BaseTypes> AnimationRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a AnimationInfo<T>) -> Result<Self> {
        let obj1 = T::u32::slice_from_data(&src[info.obj1_offset.into() as usize..],
            info.obj1_num.into() as usize * 2)
            .context("obj1")?;
        let obj2 = T::u32::slice_from_data(&src[info.obj2_offset.into() as usize..],
            info.obj2_num.into() as usize * 4)
            .context("obj2")?;
        let obj3 = Obj3::slice_from_data(&src[info.obj3_offset.into() as usize..],
            info.obj3_num.into() as usize)
            .context("Obj3")?;
        let bones = Crc::<T>::slice_from_data(&src[info.bones_offset.into() as usize..],
            (info.vals_num.into() + info.obj1_num.into()) as usize)
            .context("bones")?;
        let (obj5_header, obj5_a, obj5_b) = if info.obj5_offset.into() != 0 {
            let obj5_header = Obj5Header::<T>::from_data(&src[info.obj5_offset.into() as usize..])
                .context("obj5_header")?;
            let obj5_a = Obj5Val::slice_from_data(&src[obj5_header.obj_a_offset.into() as usize..],
                obj5_header.obj_a_num.into() as usize)
                .context("obj5_a")?;
            let obj5_b = Obj5Val::slice_from_data(&src[obj5_header.obj_b_offset.into() as usize..],
                obj5_header.obj_b_num.into() as usize)
                .context("obj5_b")?;
            (Some(obj5_header.into()), obj5_a, obj5_b)
        } else {
            (None, &[] as &_, &[] as &_)
        };
        let blocks = if info.kind.into() == 3 {
            Some(BlocksRef::from_data(src, info).context("blocks")?)
        } else if info.kind.into() < 3 {
            warn!("Unhandled animation type {}", info.kind.into());
            None
        } else {
            warn!("Unknown animation type {}", info.kind.into());
            None
        };
        Ok(Self {
            info,
            obj1: obj1.into(),
            obj2: obj2.into(),
            obj3: obj3.into(),
            bones: bones.into(),
            obj5_header,
            obj5_a: obj5_a.into(),
            obj5_b: obj5_b.into(),
            blocks: blocks.into(),
            size: info.size.into() as usize
        })
    }
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub info: AnimationInfo<NE>,
    pub obj1: Vec<u32>,
    pub obj2: Vec<u32>,
    pub obj3: Vec<Obj3<NE>>,
    pub bones: Vec<Crc<NE>>,
    pub obj5_a: Vec<Obj5Val<NE>>,
    pub obj5_b: Vec<Obj5Val<NE>>,
    pub blocks: Option<Blocks>,
}

/*
#[enum_dispatch(DumpAnimation)]
pub enum Animation_XE_<'a> {
    Ref(AnimationRef_XE_<'a>),
    Owned(Animation)
}
*/

impl<T: BaseTypes> From<&AnimationRef<'_, T>> for Animation
where
    AnimationInfo<NE>: From<AnimationInfo<T>>,
    Crc<NE>: From<Crc<T>>,
    Obj3<NE>: From<Obj3<T>>,
    Obj5Val<NE>: From<Obj5Val<T>>,
    RotationPolar32<NE>: From<RotationPolar32<T>>,
    RotationThreeComp48<NE>: From<RotationThreeComp48<T>>,
    RotationUncompressed<NE>: From<RotationUncompressed<T>>,
{
    fn from(val: &AnimationRef<T>) -> Self {
        Self {
            info: val.info.clone().into(),
            obj1: val
                .obj1
                .iter()
                .map(|&x| x.into())
                .collect(),
            obj2: val
                .obj2
                .iter()
                .map(|&x| x.into())
                .collect(),
            obj3: val
                .obj3
                .iter()
                .map(|&x| x.into())
                .collect(),
            bones: val
                .bones
                .iter()
                .map(|&x| x.into())
                .collect(),
            obj5_a: val
                .obj5_a
                .iter()
                .map(|&x| x.into())
                .collect(),
            obj5_b: val
                .obj5_b
                .iter()
                .map(|&x| x.into())
                .collect(),
            blocks: val.blocks.as_ref().map(|x| x.into()),
        }
    }
}

#[enum_dispatch]
pub trait DumpAnimationImpl<T: BaseTypes> {
    fn key(&self) -> u32;
    fn gamemodemask(&self) -> i32;
    fn obj1_len(&self) -> usize;
    fn obj2_len(&self) -> usize;
    fn obj3_len(&self) -> usize;
    fn bones_len(&self) -> usize;
    fn obj5_a_len(&self) -> usize;
    fn obj5_b_len(&self) -> usize;
    fn write_obj1(&self, obj1: &mut [T::u32]) -> Result<()>;
    fn write_obj2(&self, obj2: &mut [T::u32]) -> Result<()>;
    fn write_obj3(&self, obj3: &mut [Obj3<T>]) -> Result<()>;
    fn write_bones(&self, bones: &mut [Crc<T>]) -> Result<()>;
    fn write_obj5_a(&self, obj5_a: &mut [Obj5Val<T>]) -> Result<()>;
    fn write_obj5_b(&self, obj5_b: &mut [Obj5Val<T>]) -> Result<()>;
    fn blocks(&self) -> Option<&impl DumpBlocks<T>>;
    fn write_info(&self, info: &mut AnimationInfo<T>) -> Result<()>;
}

#[enum_dispatch]
pub trait DumpAnimation<T: BaseTypes> {
    fn key(&self) -> u32;
    fn gamemodemask(&self) -> i32;
    fn dump_into(
        &self,
        dst: &mut DumpSlice,
        info: &mut AnimationInfo<T>,
    ) -> Result<()>;

    fn size(&self) -> usize;
}

impl<T: BaseTypes, I: DumpAnimationImpl<T>> DumpAnimation<T> for I {
    #[inline(always)]
    fn key(&self) -> u32 {
        DumpAnimationImpl::key(self)
    }
    #[inline(always)]
    fn gamemodemask(&self) -> i32 {
        DumpAnimationImpl::gamemodemask(self)
    }
    fn dump_into(
        &self,
        dst: &mut DumpSlice,
        info: &mut AnimationInfo<T>,
    ) -> Result<()> {
        self.write_info(info).context("write_info")?;
        let start = dst.offset;
        info.offset = (dst.offset as u32).into();

        info.obj1_offset = ((dst.offset - start) as u32).into();
        let obj1 = T::u32::mut_slice_from_data(dst, self.obj1_len()).context("obj1")?;
        info.obj1_num = ((obj1.len() / 2) as u32).into();
        self.write_obj1(obj1).context("write obj1")?;

        info.obj2_offset = (if self.obj2_len() == 0 { 0 } else { dst.offset - start } as u32).into();
        let obj2 = T::u32::mut_slice_from_data(dst, self.obj2_len()).context("obj2")?;
        info.obj2_num = ((obj2.len() / 4) as u32).into();
        self.write_obj2(obj2).context("write obj2")?;

        if let Some(blocks) = self.blocks() {
            blocks.dump_into(dst, info, start).context("blocks")?;
        }

        info.obj3_offset = (if self.obj3_len() == 0 { 0 } else { dst.offset - start } as u32).into();
        let obj3 = Obj3::mut_slice_from_data(dst, self.obj3_len()).context("obj3")?;
        info.obj3_num = (obj3.len() as u32).into();
        self.write_obj3(obj3).context("write obj3")?;

        info.bones_offset = ((dst.offset - start) as u32).into();
        let bones = Crc::mut_slice_from_data(dst, self.bones_len()).context("bones")?;
        self.write_bones(bones).context("write bones")?;

        if self.obj5_a_len() != 0 || self.obj5_b_len() != 0 {
            info.obj5_offset = ((dst.offset - start) as u32).into();
            let obj5_header = Obj5Header::<T>::mut_from_data(dst).context("obj5_header")?;

            obj5_header.obj_a_offset = (if self.obj5_a_len() == 0 { 0 } else { dst.offset - start } as u32).into();
            let obj5_a = Obj5Val::mut_slice_from_data(dst, self.obj5_a_len()).context("obj5_a")?;
            obj5_header.obj_a_num = (obj5_a.len() as u32).into();
            self.write_obj5_a(obj5_a).context("write obj5_a")?;

            obj5_header.obj_b_offset = (if self.obj5_b_len() == 0 { 0 } else { dst.offset - start } as u32).into();
            let obj5_b = Obj5Val::mut_slice_from_data(dst, self.obj5_b_len()).context("obj5_b")?;
            obj5_header.obj_b_num = (obj5_b.len() as u32).into();
            self.write_obj5_b(obj5_b).context("write obj5_b")?;
        } else {
            info.obj5_offset = 0u32.into();
        }
        dst.align(16)?;
        info.size = ((dst.offset - start) as u32).into();
        Ok(())
    }

    fn size(&self) -> usize {
        let mut size = 0;

        size += (self.obj1_len() + self.obj2_len()) * std::mem::size_of::<T::u32>();

        if let Some(blocks) = self.blocks() {
            size += blocks.size();
        }

        size += std::mem::size_of::<Obj3<T>>() * self.obj3_len();

        size += std::mem::size_of::<Crc<T>>() * self.bones_len();

        if self.obj5_a_len() != 0 || self.obj5_b_len() != 0 {
            size += std::mem::size_of::<Obj5Header<T>>();
            size += (self.obj5_a_len() + self.obj5_b_len()) * std::mem::size_of::<Obj5Val<T>>()
        }

        align_offset(size, 16)
    }
}

impl<T: BaseTypes> DumpAnimationImpl<T> for AnimationRef<'_, T> {
    fn key(&self) -> u32 {
        self.info.key.val.into()
    }
    fn gamemodemask(&self) -> i32 {
        self.info.gamemodemask.into()
    }
    fn obj1_len(&self) -> usize {
        self.obj1.len()
    }
    fn obj2_len(&self) -> usize {
        self.obj2.len()
    }
    fn obj3_len(&self) -> usize {
        self.obj3.len()
    }
    fn bones_len(&self) -> usize {
        self.bones.len()
    }
    fn obj5_a_len(&self) -> usize {
        self.obj5_a.len()
    }
    fn obj5_b_len(&self) -> usize {
        self.obj5_b.len()
    }
    fn write_obj1(&self, obj1: &mut [T::u32]) -> Result<()> {
        obj1.copy_from_slice(&self.obj1[..]);
        Ok(())
    }
    fn write_obj2(&self, obj2: &mut [T::u32]) -> Result<()> {
        obj2.copy_from_slice(&self.obj2[..]);
        Ok(())
    }
    fn write_obj3(&self, obj3: &mut [Obj3<T>]) -> Result<()> {
        obj3.copy_from_slice(&self.obj3[..]);
        Ok(())
    }
    fn write_bones(&self, bones: &mut [Crc<T>]) -> Result<()> {
        bones.copy_from_slice(&self.bones[..]);
        Ok(())
    }
    fn write_obj5_a(&self, obj5_a: &mut [Obj5Val<T>]) -> Result<()> {
        obj5_a.copy_from_slice(&self.obj5_a[..]);
        Ok(())
    }
    fn write_obj5_b(&self, obj5_b: &mut [Obj5Val<T>]) -> Result<()> {
        obj5_b.copy_from_slice(&self.obj5_b[..]);
        Ok(())
    }
    fn blocks(&self) -> Option<&impl DumpBlocks<T>> {
        self.blocks.as_ref()
    }
    fn write_info(&self, info: &mut AnimationInfo<T>) -> Result<()> {
        *info = *self.info;
        Ok(())
    }
}

impl<T: BaseTypes> DumpAnimationImpl<T> for Animation
where
    AnimationInfo<T>: From<AnimationInfo<NE>>,
    Crc<T>: From<Crc<NE>>,
    Obj3<T>: From<Obj3<NE>>,
    Obj5Val<T>: From<Obj5Val<NE>>,
    RotationPolar32<T>: From<RotationPolar32<NE>>,
    RotationThreeComp48<T>: From<RotationThreeComp48<NE>>,
    RotationUncompressed<T>: From<RotationUncompressed<NE>>,
{
    fn key(&self) -> u32 {
        self.info.key.val
    }
    fn gamemodemask(&self) -> i32 {
        self.info.gamemodemask
    }
    fn obj1_len(&self) -> usize {
        self.obj1.len()
    }
    fn obj2_len(&self) -> usize {
        self.obj2.len()
    }
    fn obj3_len(&self) -> usize {
        self.obj3.len()
    }
    fn bones_len(&self) -> usize {
        self.bones.len()
    }
    fn obj5_a_len(&self) -> usize {
        self.obj5_a.len()
    }
    fn obj5_b_len(&self) -> usize {
        self.obj5_b.len()
    }
    fn write_obj1(&self, obj1: &mut [T::u32]) -> Result<()> {
        for (&src, dst) in self.obj1.iter().zip(obj1) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_obj2(&self, obj2: &mut [T::u32]) -> Result<()> {
        for (&src, dst) in self.obj2.iter().zip(obj2) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_obj3(&self, obj3: &mut [Obj3<T>]) -> Result<()> {
        for (&src, dst) in self.obj3.iter().zip(obj3) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_bones(&self, bones: &mut [Crc<T>]) -> Result<()> {
        for (&src, dst) in self.bones.iter().zip(bones) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_obj5_a(&self, obj5_a: &mut [Obj5Val<T>]) -> Result<()> {
        for (&src, dst) in self.obj5_a.iter().zip(obj5_a) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_obj5_b(&self, obj5_b: &mut [Obj5Val<T>]) -> Result<()> {
        for (&src, dst) in self.obj5_b.iter().zip(obj5_b) {
            *dst = src.into();
        }
        Ok(())
    }
    fn blocks(&self) -> Option<&impl DumpBlocks<T>> {
        self.blocks.as_ref()
    }
    fn write_info(&self, info: &mut AnimationInfo<T>) -> Result<()> {
        *info = self.info.into();
        Ok(())
    }
}

#[derive_pod]
pub struct AnimationBlockInfo<T: BaseTypes> {
    pub key: Crc<T>,
    pub guid: T::u32,
    pub key_name: Crc<T>,
    pub offset: T::u32,
    pub size: T::u32,
    pub size_comp: T::u32,
    pub unk_6: T::u32,
    pub unk_7: T::u32,
    pub unk_8: T::u32,
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct AnimationsRef<'a, T: BaseTypes> {
    pub animations: IndexMap<u32, AnimationRef<'a, T>>,
    pub block_infos: ref_slice<'a, AnimationBlockInfo<T>>,
}

impl<T: BaseTypes> Default for AnimationsRef<'_, T> {
    fn default() -> Self {
        Self {
            animations: IndexMap::default().into(),
            block_infos: ref_slice::default()
        }
    }
}

impl<'a, T: BaseTypes> AnimationsRef<'a, T> {
    pub fn from_data(anim_infos: &'a [AnimationInfo<T>], blocks: &'a [CompressedDataRef<'_>], block_infos: &'a [AnimationBlockInfo<T>]) -> Result<Self> {
        let mut offsets = vec![0; blocks.len()];
        let mut animations = IndexMap::with_capacity(anim_infos.len());
        for info in anim_infos {
            for (i, (data, offset)) in blocks.iter().zip(offsets.iter()).enumerate() {
                if info.gamemodemask.into() >> i & 1 != 0 {
                    let val = AnimationRef::from_data(&data.get()[*offset..], info).with_context(|| format!("animation {}", i))?;
                    animations.insert(info.key.val.into(), val);
                    break;
                }
            }
            for (i, offset) in offsets.iter_mut().enumerate() {
                if info.gamemodemask.into() >> i & 1 != 0 {
                    *offset += animations.last().unwrap().1.size;
                }
            }
        }
        Ok(Self { animations: animations.into(), block_infos: block_infos.into() })
    }
}

#[derive(Debug, Clone)]
pub struct Animations {
    pub animations: IndexMap<Crc<NE>, Animation>,
    pub block_infos: Vec<AnimationBlockInfo<NE>>
}

impl<T: BaseTypes> From<&AnimationsRef<'_, T>> for Animations
where
    AnimationInfo<NE>: From<AnimationInfo<T>>,
    Crc<NE>: From<Crc<T>>,
    Obj3<NE>: From<Obj3<T>>,
    Obj5Val<NE>: From<Obj5Val<T>>,
    RotationPolar32<NE>: From<RotationPolar32<T>>,
    RotationThreeComp48<NE>: From<RotationThreeComp48<T>>,
    RotationUncompressed<NE>: From<RotationUncompressed<T>>,
    AnimationBlockInfo<NE>: From<AnimationBlockInfo<T>>,
{
    fn from(val: &AnimationsRef<T>) -> Self {
        Self {
            animations: val.animations.values().map(|x| (x.info.key.into(), x.into())).collect(),
            block_infos: val.block_infos.iter().map(|&x| x.into()).collect()
        }
    }
}

/*
pub struct AnimationsImpl_XE_<'a> {
    pub animations: IndexMap<Crc, Animation_XE_<'a>>,
    pub block_infos: Vec<AnimationBlockInfo>
}

pub struct AnimationsRaw_XE_<'a, 'b> {
    pub anim_infos: &'a [AnimationInfo_XE_], 
    pub blocks: &'a [CompressedDataRef<'b>], 
    pub block_infos: &'a [AnimationBlockInfo_XE_]
}

#[enum_dispatch(DumpAnimations_XE_)]
pub enum Animations_XE_<'a, 'b> {
    Raw(AnimationsRaw_XE_<'a, 'b>),
    Owned(AnimationsImpl_XE_<'a>)
}

impl DumpAnimations_XE_ for AnimationsRaw_XE_<'_, '_> {
    fn animation_num(&self) -> usize {
        self.anim_infos.len()
    }
    fn block_num(&self) -> usize {
        self.blocks.len()
    }

    fn info_counts(&self, counts: &mut InfoCounts) {
        counts.animations = self.animation_num();
        counts.animation_blocks = self.block_num()
    }

    fn dump<'d>(&'d self, infos: &mut DumpInfos_XE_) -> Result<Vec<CompressedData<'d>>> {
        infos.animations.take().write_from(self.anim_infos).context("anim infos")?;
        infos.animation_blocks.take().write_from(self.block_infos).context("anim block infos")?;
        Ok(self.blocks.iter().map(|x| x.into()).collect())
    }
}

impl DumpAnimationsImpl_XE_ for AnimationsImpl_XE_<'_> {
    fn animation_num(&self) -> usize {
        self.animations.len()
    }
    fn block_num(&self) -> usize {
        self.block_infos.len()
    }
    fn animations(&self) -> impl Iterator<Item=&impl DumpAnimation_XE_> {
        self.animations.values()
    }
    fn write_block_infos(&self, infos: &mut [AnimationBlockInfo_XE_]) -> Result<()> {
        for (src, dst) in self.block_infos.iter().zip(infos) {
            *dst = src.into()
        }
        Ok(())
    }
}
*/

pub trait DumpAnimationsImpl<T: BaseTypes> {
    fn animation_num(&self) -> usize;
    fn block_num(&self) -> usize;
    fn animations(&self) -> impl Iterator<Item=&impl DumpAnimation<T>>;
    fn write_block_infos(&self, infos: &mut [AnimationBlockInfo<T>]) -> Result<()>;
}

#[enum_dispatch]
pub trait DumpAnimations<T: BaseTypes> {
    fn animation_num(&self) -> usize;
    fn block_num(&self) -> usize;

    fn info_counts(&self, counts: &mut InfoCounts) {
        counts.animations = self.animation_num();
        counts.animation_blocks = self.block_num()
    }

    fn dump<'d>(&'d self, infos: &mut DumpInfos<T>) -> Result<Vec<CompressedData<'d>>>;
}

impl<T: BaseTypes, I: DumpAnimationsImpl<T>> DumpAnimations<T> for I {
    #[inline(always)]
    fn animation_num(&self) -> usize {
        DumpAnimationsImpl::animation_num(self)
    }
    #[inline(always)]
    fn block_num(&self) -> usize {
        DumpAnimationsImpl::block_num(self)
    }

    fn dump<'d>(&'d self, infos: &mut DumpInfos<T>) -> Result<Vec<CompressedData<'d>>> {
        let mut animations = self.animations().collect::<Vec<_>>();
        animations.sort_by_key(|x| x.key());
        
        let animation_block_infos = infos.animation_blocks.as_ref();
        let mut block_sizes = vec![0usize; animation_block_infos.len()];
        for animation in animations.iter() {
            let gamemodemask = animation.gamemodemask();
            let anim_size = animation.size();
            for (i, size) in block_sizes.iter_mut().enumerate() {
                if gamemodemask >> i & 1 == 0 {
                    continue;
                }
                *size += anim_size;
            }
        }

        self.write_block_infos(animation_block_infos).context("write block_infos")?;
        for (size, block_info) in block_sizes.iter().zip(animation_block_infos) {
            block_info.size = (*size as u32).into();
        }

        let animation_infos = infos.animations.as_ref();
        let mut blocks = block_sizes.into_iter().map(|size| OwnedCompressedData::with_capacity(size)).collect::<Vec<_>>();
        let mut data = blocks.iter_mut().map(|x| x.dump_slice()).collect::<Vec<_>>();
        for (j, (info, animation)) in animation_infos.iter_mut().zip(animations).enumerate() {
            let gamemodemask = animation.gamemodemask();
            let mut anim_dump: Option<NonNull<[u8]>> = None;
            for (i, dst) in data.iter_mut().enumerate() {
                if gamemodemask >> i & 1 == 0 {
                    continue;
                }
                if let Some(val) = anim_dump {
                    let src = unsafe { val.as_ref() };
                    let dst = u8::mut_slice_from_data(dst, src.len()).with_context(|| format!("pre-dumped animation {}", j))?;
                    dst.copy_from_slice(src);
                } else {
                    let old_off = dst.offset;
                    let old_ptr = NonNull::from_ref(&dst.vals[0]);
                    animation.dump_into(dst, info).with_context(|| format!("animation {}", j))?;
                    anim_dump.replace(NonNull::slice_from_raw_parts(old_ptr, dst.offset - old_off));
                }
            }
        }
        Ok(blocks.into_iter().map(|x| x.into()).collect())
    }
}

impl<T: BaseTypes> DumpAnimationsImpl<T> for AnimationsRef<'_, T> {
    fn animation_num(&self) -> usize {
        self.animations.len()
    }
    fn block_num(&self) -> usize {
        self.block_infos.len()
    }
    fn animations(&self) -> impl Iterator<Item=&impl DumpAnimation<T>> {
        self.animations.values()
    }
    fn write_block_infos(&self, infos: &mut [AnimationBlockInfo<T>]) -> Result<()> {
        infos.copy_from_slice(&self.block_infos[..]);
        Ok(())
    }
}

impl<T: BaseTypes> DumpAnimationsImpl<T> for Animations
where
    AnimationInfo<T>: From<AnimationInfo<NE>>,
    Crc<T>: From<Crc<NE>>,
    Obj3<T>: From<Obj3<NE>>,
    Obj5Val<T>: From<Obj5Val<NE>>,
    RotationPolar32<T>: From<RotationPolar32<NE>>,
    RotationThreeComp48<T>: From<RotationThreeComp48<NE>>,
    RotationUncompressed<T>: From<RotationUncompressed<NE>>,
    AnimationBlockInfo<T>: From<AnimationBlockInfo<NE>>,
{
    fn animation_num(&self) -> usize {
        self.animations.len()
    }
    fn block_num(&self) -> usize {
        self.block_infos.len()
    }
    fn animations(&self) -> impl Iterator<Item=&impl DumpAnimation<T>> {
        self.animations.values()
    }
    fn write_block_infos(&self, infos: &mut [AnimationBlockInfo<T>]) -> Result<()> {
        for (&src, dst) in self.block_infos.iter().zip(infos) {
            *dst = src.into()
        }
        Ok(())
    }
}
