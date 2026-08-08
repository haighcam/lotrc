#[make_endian]
use crate::{
    types::{Crc_XE_, f32_XE_, u16_XE_, u32_XE_, i32_XE_},
    level::pak::block1::infos::DumpInfos_XE_,
};
use crate::{
    types::{Crc, RefFromData, align_offset, DumpData, DumpSlice, OrderedData, CompressedDataRef, ref_slice, slice, OwnedCompressedData, CompressedData, EndianTypes},
    level::pak::block1::infos::InfoCounts
};
use anyhow::{anyhow, Context, Result};
use log::{warn, debug};
use lotrc_proc::{make_endian, derive_ordered_data};
use indexmap::IndexMap;
use std::ptr::NonNull;
use enum_dispatch::enum_dispatch;

pub trait AnimationTypes: EndianTypes {
    type AnimationInfo: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + AnimationInfoTypeTrait; 
    type AnimationBlockInfo: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + AnimationBlockInfoTypeTrait; 
    type RotationPolar32: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + RotationPolar32TypeTrait; 
    type RotationThreeComp48: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + RotationThreeComp48TypeTrait; 
    type RotationUncompressed: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + RotationUncompressedTypeTrait; 
    type Obj5Header: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + Obj5HeaderTypeTrait; 
    type Obj5Val: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + Obj5ValTypeTrait; 
    type Obj3: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + Obj3TypeTrait; 
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum AnimVals1Ref<'a, T: EndianTypes> {
    Type1(ref_slice<'a, u8>),
    Type2(ref_slice<'a, T::u16>),
    Type3(ref_slice<'a, T::u16>),
    Type4(ref_slice<'a, T::u16>),
}

impl<'a, T: EndianTypes> AnimVals1Ref<'a, T> {
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
            Self::Type1(vals) => vals.len() * u8::size_of(),
            Self::Type2(vals) => vals.len() * T::u16::size_of(),
            Self::Type3(vals) => vals.len() * T::u16::size_of(),
            Self::Type4(vals) => vals.len() * T::u16::size_of(),
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

#[make_endian]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum AnimVals1Ref_XE_<'a> {
    Type1(ref_slice<'a, u8>),
    Type2(ref_slice<'a, u16_XE_>),
    Type3(ref_slice<'a, u16_XE_>),
    Type4(ref_slice<'a, u16_XE_>),
}

#[make_endian]
impl<'a> AnimVals1Ref_XE_<'a> {
    pub fn from_data(src: &'a [u8], num: usize, kind: u8) -> Result<Self> {
        Ok(match kind {
            0 => Self::Type1(u8::slice_from_data(src, num).context("type1")?.into()),
            1 => Self::Type2(u16_XE_::slice_from_data(src, num).context("type2")?.into()),
            2 => Self::Type3(u16_XE_::slice_from_data(src, num).context("type3")?.into()),
            3 => Self::Type4(u16_XE_::slice_from_data(src, num).context("type4")?.into()),
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
            Self::Type1(vals) => vals.len() * u8::size_of(),
            Self::Type2(vals) => vals.len() * u16_XE_::size_of(),
            Self::Type3(vals) => vals.len() * u16_XE_::size_of(),
            Self::Type4(vals) => vals.len() * u16_XE_::size_of(),
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

#[make_endian]
impl From<&AnimVals1Ref_XE_<'_>> for AnimVals1 {
    fn from(val: &AnimVals1Ref_XE_) -> Self {
        match val {
            AnimVals1Ref_XE_::Type1(vals) => Self::Type1(vals.to_vec()),
            AnimVals1Ref_XE_::Type2(vals) => Self::Type2(vals.iter().map(|x| x.conv()).collect()),
            AnimVals1Ref_XE_::Type3(vals) => Self::Type3(vals.iter().map(|x| x.conv()).collect()),
            AnimVals1Ref_XE_::Type4(vals) => Self::Type4(vals.iter().map(|x| x.conv()).collect()),
        }
    }
}

#[make_endian]
pub enum AnimVals1Dump_XE_<'a> {
    Type1(&'a mut [u8]),
    Type2(&'a mut [u16_XE_]),
    Type3(&'a mut [u16_XE_]),
    Type4(&'a mut [u16_XE_]),
}

#[make_endian]
impl<'a> AnimVals1Dump_XE_<'a> {
    fn mut_from_data(
        dst: &mut DumpSlice<'a>,
        kind: u8,
        count: usize,
    ) -> Result<Self> {
        match kind {
            0 => Ok((Self::Type1(RefFromData::mut_slice_from_data(dst, count)?))),
            1 => Ok((Self::Type2(RefFromData::mut_slice_from_data(dst, count)?))),
            2 => Ok((Self::Type3(RefFromData::mut_slice_from_data(dst, count)?))),
            3 => Ok((Self::Type4(RefFromData::mut_slice_from_data(dst, count)?))),
            _ => return Err(anyhow!("Illegal Type for spline thingy")),
        }
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct Obj1Ref<'a, T: EndianTypes> {
    pub flags: u8,
    pub s2: u8,
    pub s1: T::u16,
    pub data: ref_slice<'a, u8>,
    pub vals_a: ref_slice<'a, T::f32>,
    pub vals: AnimVals1Ref<'a, T>,
    pub size: usize,
}

#[make_endian]
impl<'a, T: EndianTypes> Obj1Ref<'a, T> {
    const COUNTS: [usize; 8] = [0, 1, 1, 2, 1, 2, 2, 3];
    pub fn from_data(src: &'a [u8], flags: u8, kind: u8) -> Result<Self> {
        let mut offset = 0;
        let start = offset;
        let (s1, s2, data) = if flags & 0xf0 != 0 {
            let s1 = T::u16::from_data(&src[offset..]).context("s1")?;
            offset += s1.size_of_val();
            let s2 = u8::from_data(&src[offset..]).context("s2")?;
            offset += s2.size_of_val();
            let data = u8::slice_from_data(&src[offset..], s1.conv() as usize + *s2 as usize + 2).context("data")?;
            offset += data.size_of_val();
            (*s1, *s2, data.into())
        } else {
            (0u16.into(), 0, ref_slice::default())       
        };
        let vals_a = if flags != 0 {
            offset = align_offset(offset, 4);
            let num = Self::COUNTS[(flags & 7) as usize]
                + 2 * Self::COUNTS[(((flags >> 4) & !flags) & 7) as usize];
            let vals_a = T::f32::slice_from_data(&src[offset..], num).context("vals_a")?;
            offset += vals_a.size_of_val();
            vals_a.into()
        } else {
            ref_slice::default() 
        };
        let vals = if flags & 0xf0 != 0 {
            offset = align_offset(offset, 2);
            let num = Self::COUNTS[((flags >> 4) & 7) as usize] * (s1.conv() as usize + 1);
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

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct Obj1Ref_XE_<'a> {
    pub flags: u8,
    pub s2: u8,
    pub s1: u16_XE_,
    pub data: ref_slice<'a, u8>,
    pub vals_a: ref_slice<'a, f32_XE_>,
    pub vals: AnimVals1Ref_XE_<'a>,
    pub size: usize,
}

#[make_endian]
impl<'a> Obj1Ref_XE_<'a> {
    const COUNTS: [usize; 8] = [0, 1, 1, 2, 1, 2, 2, 3];
    pub fn from_data(src: &'a [u8], flags: u8, kind: u8) -> Result<Self> {
        let mut offset = 0;
        let start = offset;
        let (s1, s2, data) = if flags & 0xf0 != 0 {
            let s1 = u16_XE_::from_data(&src[offset..]).context("s1")?;
            offset += s1.size_of_val();
            let s2 = u8::from_data(&src[offset..]).context("s2")?;
            offset += s2.size_of_val();
            let data = u8::slice_from_data(&src[offset..], s1.to_native() as usize + *s2 as usize + 2).context("data")?;
            offset += data.size_of_val();
            (*s1, *s2, data.into())
        } else {
            (u16_XE_::from(0), 0, ref_slice::default())       
        };
        let vals_a = if flags != 0 {
            offset = align_offset(offset, 4);
            let num = Self::COUNTS[(flags & 7) as usize]
                + 2 * Self::COUNTS[(((flags >> 4) & !flags) & 7) as usize];
            let vals_a = f32_XE_::slice_from_data(&src[offset..], num).context("vals_a")?;
            offset += vals_a.size_of_val();
            vals_a.into()
        } else {
            ref_slice::default() 
        };
        let vals = if flags & 0xf0 != 0 {
            offset = align_offset(offset, 2);
            let num = Self::COUNTS[((flags >> 4) & 7) as usize] * (s1.to_native() as usize + 1);
            let vals = AnimVals1Ref_XE_::from_data(&src[offset..], num, kind).context("vals")?;
            offset += vals.size();
            vals
        } else {
           AnimVals1Ref_XE_::empty(kind).context("vals")?
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

#[make_endian]
impl From<&Obj1Ref_XE_<'_>> for Obj1 {
    fn from(val: &Obj1Ref_XE_) -> Self {
        Self {
            flags: val.flags,
            s1: val.s1.conv(),
            s2: val.s2,
            data: (&val.data[..]).into(),
            vals_a: val
                .vals_a
                .iter()
                .map(|x| x.conv())
                .collect(),
            vals: (&val.vals).into(),
        }
    }
}

#[make_endian]
pub trait DumpObj1_XE_ {
    fn flags(&self) -> u8;
    fn write_s1(&self, s1: &mut u16_XE_) -> Result<()>;
    fn write_s2(&self, s2: &mut u8) -> Result<()>;
    fn data_len(&self) -> usize;
    fn vals_a_len(&self) -> usize;
    fn write_data(&self, data: &mut [u8]) -> Result<()>;
    fn write_vals_a(&self, vals_a: &mut [f32_XE_]) -> Result<()>;
    fn write_vals(&self, vals: AnimVals1Dump_XE_) -> Result<()>;
    fn vals_kind(&self) -> u8;
    fn vals_len(&self) -> usize;

    fn vals_size(&self) -> usize {
        match self.vals_kind() {
            0 => self.vals_len(),
            1 | 2 | 3 => self.vals_len() * u16_XE_::size_of(),
            _ => panic!("Invalid vals kind")
        }
    }

    fn dump_into(
        &self,
        dst: &mut DumpSlice
    ) -> Result<()> {
        if self.flags() & 0xf0 != 0 {
            let s1 = u16_XE_::mut_from_data(dst).context("s1")?;
            self.write_s1(s1).context("write s1")?;
            let s2 = u8::mut_from_data(dst).context("s2")?;
            self.write_s2(s2).context("write s2")?;
            let data = u8::mut_slice_from_data(dst, self.data_len()).context("data")?;
            self.write_data(data).context("write data")?;
        }
        if self.flags() != 0 {
            dst.align(4)?;
            let vals_a = f32_XE_::mut_slice_from_data(dst, self.vals_a_len()).context("vals_a")?;
            self.write_vals_a(vals_a).context("write vals_a")?;
        }
        if self.flags() & 0xf0 != 0 {
            dst.align(2)?;
            let vals = AnimVals1Dump_XE_::mut_from_data(dst, self.vals_kind(), self.vals_len()).context("vals")?;
            self.write_vals(vals).context("write vals")?;
        }
        Ok(())
    }

    fn size(&self) -> usize {
        let mut size = 0;
        if self.flags() & 0xf0 != 0 {
            size += u16_XE_::size_of();
            size += u8::size_of();
            size += self.data_len();
        }
        if self.flags() != 0 {
            size = align_offset(size, 4);
            size += f32_XE_::size_of() * self.vals_a_len();
        }
        if self.flags() != 0xf0 {
            size = align_offset(size, 2);
            size += self.vals_size();
        }
        size
    }
}

#[make_endian]
impl DumpObj1_XE_ for Obj1Ref_XE_<'_> {
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
    fn write_s1(&self, s1: &mut u16_XE_) -> Result<()> {
        s1.write_from(&self.s1)
    }
    fn write_s2(&self, s2: &mut u8) -> Result<()> {
        s2.write_from(&self.s2)
    }
    fn write_data(&self, data: &mut [u8]) -> Result<()> {
        data.write_from(&self.data[..])
    }
    fn write_vals(&self, vals: AnimVals1Dump_XE_) -> Result<()> {
        match (&self.vals, vals) {
            (AnimVals1Ref_XE_::Type1(src), AnimVals1Dump_XE_::Type1(dst)) => dst.write_from(src),
            (AnimVals1Ref_XE_::Type2(src), AnimVals1Dump_XE_::Type2(dst)) => dst.write_from(src),
            (AnimVals1Ref_XE_::Type3(src), AnimVals1Dump_XE_::Type3(dst)) => dst.write_from(src),
            (AnimVals1Ref_XE_::Type4(src), AnimVals1Dump_XE_::Type4(dst)) => dst.write_from(src),
            _ => Err(anyhow!("missmatched obj1type and dump obj1type"))
        }
    }
    fn write_vals_a(&self, vals_a: &mut [f32_XE_]) -> Result<()> {
        vals_a.write_from(&self.vals_a[..])
    }
}

#[make_endian]
impl DumpObj1_XE_ for Obj1 {
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
    fn write_s1(&self, s1: &mut u16_XE_) -> Result<()> {
        *s1 = self.s1.into();
        Ok(())
    }
    fn write_s2(&self, s2: &mut u8) -> Result<()> {
        *s2 = self.s2;
        Ok(())
    }
    fn write_data(&self, data: &mut [u8]) -> Result<()> {
        data.write_from(self.data.as_slice())
    }
    fn write_vals(&self, vals: AnimVals1Dump_XE_) -> Result<()> {
        match (&self.vals, vals) {
            (AnimVals1::Type1(src), AnimVals1Dump_XE_::Type1(dst)) => dst.write_from(src),
            (AnimVals1::Type2(src), AnimVals1Dump_XE_::Type2(dst)) => Ok(for (src, dst) in src.iter().zip(dst) { *dst = src.conv() }),
            (AnimVals1::Type3(src), AnimVals1Dump_XE_::Type3(dst)) => Ok(for (src, dst) in src.iter().zip(dst) { *dst = src.conv() }),
            (AnimVals1::Type4(src), AnimVals1Dump_XE_::Type4(dst)) => Ok(for (src, dst) in src.iter().zip(dst) { *dst = src.conv() }),
            _ => Err(anyhow!("missmatched obj1type and dump obj1type"))
        }
    }
    fn write_vals_a(&self, vals_a: &mut [f32_XE_]) -> Result<()> {
        for (src, dst) in self.vals_a.iter().zip(vals_a) {
            *dst = src.conv();
        }
        Ok(())
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RotationPolar32_XE_ {
    a: u32_XE_,
}

#[derive(Debug, Default, Clone, PartialEq, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::FromBytes)]
// should be (u8, u8, u8, u16) but for xbox conv it is (u8, u8, u8, u8, u8)
pub struct RotationThreeComp40 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RotationThreeComp48_XE_ {
    a: u16_XE_,
    b: u16_XE_,
    c: u16_XE_,
}

#[derive(Debug, Default, Clone, PartialEq, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::FromBytes)]
pub struct RotationThreeComp24 {
    a: u8,
    b: u8,
    c: u8,
}

#[derive(Debug, Default, Clone, PartialEq, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::FromBytes)]
pub struct RotationStraight16 {
    a: u8,
    b: u8,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RotationUncompressed_XE_ {
    a: f32_XE_,
    b: f32_XE_,
    c: f32_XE_,
    d: f32_XE_,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum RotationQuantizationRef<'a, T: AnimationTypes> {
    Polar32(ref_slice<'a, T::RotationPolar32>),
    ThreeComp40(ref_slice<'a, RotationThreeComp40>),
    ThreeComp48(ref_slice<'a, T::RotationThreeComp48>),
    ThreeComp24(ref_slice<'a, RotationThreeComp24>),
    Straight16(ref_slice<'a, RotationStraight16>),
    Uncompressed(ref_slice<'a, T::RotationUncompressed>),
}

#[make_endian]
impl<'a, T: AnimationTypes> RotationQuantizationRef<'a, T> {
    pub fn from_data(src: &'a [u8], num: usize, kind: u8) -> Result<Self> {
        Ok(match kind {
            0 => Self::Polar32(T::RotationPolar32::slice_from_data(src, num).context("Polar32")?.into()),
            1 => Self::ThreeComp40(RotationThreeComp40::slice_from_data(src, num).context("ThreeComp40")?.into()),
            2 => Self::ThreeComp48(T::RotationThreeComp48::slice_from_data(src, num).context("ThreeComp48")?.into()),
            3 => Self::ThreeComp24(RotationThreeComp24::slice_from_data(src, num).context("ThreeComp24")?.into()),
            4 => Self::Straight16(RotationStraight16::slice_from_data(src, num).context("Straight16")?.into()),
            5 => Self::Uncompressed(T::RotationUncompressed::slice_from_data(src, num).context("Uncompressed")?.into()),
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
            Self::Polar32(vals) => vals.len() * T::RotationPolar32::size_of(),
            Self::ThreeComp40(vals) => vals.len() * RotationThreeComp40::size_of(),
            Self::ThreeComp48(vals) => vals.len() * T::RotationThreeComp48::size_of(),
            Self::ThreeComp24(vals) => vals.len() * RotationThreeComp24::size_of(),
            Self::Straight16(vals) => vals.len() * RotationStraight16::size_of(),
            Self::Uncompressed(vals) => vals.len() * T::RotationUncompressed::size_of(),
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

#[make_endian]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum RotationQuantizationRef_XE_<'a> {
    Polar32(ref_slice<'a, RotationPolar32_XE_>),
    ThreeComp40(ref_slice<'a, RotationThreeComp40>),
    ThreeComp48(ref_slice<'a, RotationThreeComp48_XE_>),
    ThreeComp24(ref_slice<'a, RotationThreeComp24>),
    Straight16(ref_slice<'a, RotationStraight16>),
    Uncompressed(ref_slice<'a, RotationUncompressed_XE_>),
}

#[make_endian]
impl<'a> RotationQuantizationRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], num: usize, kind: u8) -> Result<Self> {
        Ok(match kind {
            0 => Self::Polar32(RotationPolar32_XE_::slice_from_data(src, num).context("Polar32")?.into()),
            1 => Self::ThreeComp40(RotationThreeComp40::slice_from_data(src, num).context("ThreeComp40")?.into()),
            2 => Self::ThreeComp48(RotationThreeComp48_XE_::slice_from_data(src, num).context("ThreeComp48")?.into()),
            3 => Self::ThreeComp24(RotationThreeComp24::slice_from_data(src, num).context("ThreeComp24")?.into()),
            4 => Self::Straight16(RotationStraight16::slice_from_data(src, num).context("Straight16")?.into()),
            5 => Self::Uncompressed(RotationUncompressed_XE_::slice_from_data(src, num).context("Uncompressed")?.into()),
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
            Self::Polar32(vals) => vals.len() * RotationPolar32_XE_::size_of(),
            Self::ThreeComp40(vals) => vals.len() * RotationThreeComp40::size_of(),
            Self::ThreeComp48(vals) => vals.len() * RotationThreeComp48_XE_::size_of(),
            Self::ThreeComp24(vals) => vals.len() * RotationThreeComp24::size_of(),
            Self::Straight16(vals) => vals.len() * RotationStraight16::size_of(),
            Self::Uncompressed(vals) => vals.len() * RotationUncompressed_XE_::size_of(),
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
    Polar32(Vec<RotationPolar32>),
    ThreeComp40(Vec<RotationThreeComp40>),
    ThreeComp48(Vec<RotationThreeComp48>),
    ThreeComp24(Vec<RotationThreeComp24>),
    Straight16(Vec<RotationStraight16>),
    Uncompressed(Vec<RotationUncompressed>),
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

#[make_endian]
impl From<&RotationQuantizationRef_XE_<'_>> for RotationQuantization {
    fn from(val: &RotationQuantizationRef_XE_) -> Self {
        match val {
            RotationQuantizationRef_XE_::Polar32(vals) => Self::Polar32(vals.iter().map(|x| OrderedData::<RotationPolar32>::conv(x)).collect()),
            RotationQuantizationRef_XE_::ThreeComp40(vals) => Self::ThreeComp40(vals.iter().map(|x| x.clone()).collect()),
            RotationQuantizationRef_XE_::ThreeComp48(vals) => Self::ThreeComp48(vals.iter().map(|x| OrderedData::<RotationThreeComp48>::conv(x)).collect()),
            RotationQuantizationRef_XE_::ThreeComp24(vals) => Self::ThreeComp24(vals.iter().map(|x| x.clone()).collect()),
            RotationQuantizationRef_XE_::Straight16(vals) => Self::Straight16(vals.iter().map(|x| x.clone()).collect()),
            RotationQuantizationRef_XE_::Uncompressed(vals) => Self::Uncompressed(vals.iter().map(|x| OrderedData::<RotationUncompressed>::conv(x)).collect()),
        }
    }
}

#[make_endian]
#[derive(Debug)]
pub enum RotationQuantizationDump_XE_<'a> {
    Polar32(&'a mut [RotationPolar32_XE_]),
    ThreeComp40(&'a mut [RotationThreeComp40]),
    ThreeComp48(&'a mut [RotationThreeComp48_XE_]),
    ThreeComp24(&'a mut [RotationThreeComp24]),
    Straight16(&'a mut [RotationStraight16]),
    Uncompressed(&'a mut [RotationUncompressed_XE_]),
}

#[make_endian]
impl<'a> RotationQuantizationDump_XE_<'a> {
    pub fn mut_from_data(
        dst: &mut DumpSlice<'a>,
        count: usize,
        kind: u8,
    ) -> Result<Self> {
        match kind {
            0 => Ok((Self::Polar32(RefFromData::mut_slice_from_data(dst, count)?))),
            1 => Ok((Self::ThreeComp40(RefFromData::mut_slice_from_data(dst, count)?))),
            2 => Ok((Self::ThreeComp48(RefFromData::mut_slice_from_data(dst, count)?))),
            3 => Ok((Self::ThreeComp24(RefFromData::mut_slice_from_data(dst, count)?))),
            4 => Ok((Self::Straight16(RefFromData::mut_slice_from_data(dst, count)?))),
            5 => Ok((Self::Uncompressed(RefFromData::mut_slice_from_data(dst, count)?))),
            _ => return Err(anyhow!("Invalid Rotation Compression method")),
        }
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct Obj2Ref<'a, T: AnimationTypes> {
    pub flags: u8,
    pub s2: u8,
    pub s1: T::u16,
    pub data: ref_slice<'a, u8>,
    pub vals: RotationQuantizationRef<'a, T>,
    pub size: usize
}

#[make_endian]
impl<'a, T: AnimationTypes> Obj2Ref<'a, T> {
    pub fn from_data(src: &'a [u8], flags: u8, kind: u8) -> Result<Self> {
        let mut offset = 0;
        let start = offset;
        let (s1, s2, data) = if flags & 0xf0 != 0 {
            let s1 = T::u16::from_data(&src[offset..]).context("s1")?;
            offset += s1.size_of_val();
            let s2 = u8::from_data(&src[offset..]).context("s2")?;
            offset += s2.size_of_val();
            let data = u8::slice_from_data(&src[offset..], s1.conv() as usize + *s2 as usize + 2).context("data")?;
            offset += data.size_of_val();
            (*s1, *s2, data.into())
        } else {
            (0u16.into(), 0, ref_slice::default())
        };
        let vals = if flags != 0 {
            let align = RotationQuantizationRef::<T>::align(kind);
            offset = ((offset + align - 1) & !(align - 1)) as usize;
            let vals = RotationQuantizationRef::from_data(&src[offset..], s1.conv() as usize + 1, kind).context("vals")?;
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

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct Obj2Ref_XE_<'a> {
    pub flags: u8,
    pub s2: u8,
    pub s1: u16_XE_,
    pub data: ref_slice<'a, u8>,
    pub vals: RotationQuantizationRef_XE_<'a>,
    pub size: usize
}

#[make_endian]
impl<'a> Obj2Ref_XE_<'a> {
    pub fn from_data(src: &'a [u8], flags: u8, kind: u8) -> Result<Self> {
        let mut offset = 0;
        let start = offset;
        let (s1, s2, data) = if flags & 0xf0 != 0 {
            let s1 = u16_XE_::from_data(&src[offset..]).context("s1")?;
            offset += s1.size_of_val();
            let s2 = u8::from_data(&src[offset..]).context("s2")?;
            offset += s2.size_of_val();
            let data = u8::slice_from_data(&src[offset..], s1.to_native() as usize + *s2 as usize + 2).context("data")?;
            offset += data.size_of_val();
            (*s1, *s2, data.into())
        } else {
            (u16_XE_::from(0), 0, ref_slice::default())
        };
        let vals = if flags != 0 {
            let align = RotationQuantizationRef_XE_::align(kind);
            offset = ((offset + align - 1) & !(align - 1)) as usize;
            let vals = RotationQuantizationRef_XE_::from_data(&src[offset..], s1.to_native() as usize + 1, kind).context("vals")?;
            offset += vals.size();
            vals
        } else {
            RotationQuantizationRef_XE_::empty(kind).context("vals")?
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

#[make_endian]
impl From<&Obj2Ref_XE_<'_>> for Obj2 {
    fn from(val: &Obj2Ref_XE_) -> Self {
        Self {
            flags: val.flags,
            s1: val.s1.into(),
            s2: val.s2,
            data: (&val.data[..]).into(),
            vals: (&val.vals).into(),
        }
    }
}
#[make_endian]
pub trait DumpObj2_XE_ {
    fn flags(&self) -> u8;
    fn data_len(&self) -> usize;
    fn vals_len(&self) -> usize;
    fn vals_kind(&self) -> u8;
    fn write_s1(&self, s1: &mut u16_XE_) -> Result<()>;
    fn write_s2(&self, s2: &mut u8) -> Result<()>;
    fn write_data(&self, data: &mut [u8]) -> Result<()>;
    fn write_vals(&self, vals: RotationQuantizationDump_XE_) -> Result<()>;

    fn vals_size(&self) -> usize {
        match self.vals_kind() {
            0 => self.vals_len() * RotationPolar32_XE_::size_of(),
            1 => self.vals_len() * RotationThreeComp40::size_of(),
            2 => self.vals_len() * RotationThreeComp48_XE_::size_of(),
            3 => self.vals_len() * RotationThreeComp24::size_of(),
            4 => self.vals_len() * RotationStraight16::size_of(),
            5 => self.vals_len() * RotationUncompressed_XE_::size_of(),
            _ => panic!("Invalid vals kind")
        }
    }

    fn size(&self) -> usize {
        let mut size = 0;
        if self.flags() & 0xf0 != 0 {
            size += u16_XE_::size_of();
            size += u8::size_of();
            size += self.data_len();
        }
        if self.flags() != 0 {
            size = align_offset(size, RotationQuantizationRef_XE_::align(self.vals_kind()));
            size += self.vals_size();
        }
        size
    }

    fn dump_into(
        &self,
        dst: &mut DumpSlice
    ) -> Result<()> {
        if self.flags() & 0xf0 != 0 {
            let s1= u16_XE_::mut_from_data(dst).context("s1")?;
            self.write_s1(s1).context("write s1")?;
            let s2 = u8::mut_from_data(dst).context("s2")?;
            self.write_s2(s2).context("write s2")?;
            let data = u8::mut_slice_from_data(dst, self.data_len()).context("data")?;
            self.write_data(data).context("write data")?;
        }
        if self.flags() != 0 {
            dst.align(RotationQuantizationRef_XE_::align(self.vals_kind()))?;
            let vals = RotationQuantizationDump_XE_::mut_from_data(dst, self.vals_len(), self.vals_kind()).context("vals")?;
            self.write_vals(vals).context("write vals")?;
        }
        Ok(())
    }
}

#[make_endian]
impl DumpObj2_XE_ for Obj2Ref_XE_<'_> {
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
    fn write_s1(&self, s1: &mut u16_XE_) -> Result<()> {
        s1.write_from(&self.s1)
    }
    fn write_s2(&self, s2: &mut u8) -> Result<()> {
        s2.write_from(&self.s2)
    }
    fn write_data(&self, data: &mut [u8]) -> Result<()> {
        data.write_from(&self.data[..])
    }
    fn write_vals(&self, vals: RotationQuantizationDump_XE_) -> Result<()> {
        match (&self.vals, vals) {
            (RotationQuantizationRef_XE_::Polar32(src), RotationQuantizationDump_XE_::Polar32(dst)) => dst.write_from(&src[..]),
            (RotationQuantizationRef_XE_::ThreeComp40(src), RotationQuantizationDump_XE_::ThreeComp40(dst)) => dst.write_from(&src[..]),
            (RotationQuantizationRef_XE_::ThreeComp48(src), RotationQuantizationDump_XE_::ThreeComp48(dst)) => dst.write_from(&src[..]),
            (RotationQuantizationRef_XE_::ThreeComp24(src), RotationQuantizationDump_XE_::ThreeComp24(dst)) => dst.write_from(&src[..]),
            (RotationQuantizationRef_XE_::Straight16(src), RotationQuantizationDump_XE_::Straight16(dst)) => dst.write_from(&src[..]),
            (RotationQuantizationRef_XE_::Uncompressed(src), RotationQuantizationDump_XE_::Uncompressed(dst)) => dst.write_from(&src[..]),
            _ => Err(anyhow!("missmatched rotation quantization and dump rotation quantization"))
        }
    }
}

#[make_endian]
impl DumpObj2_XE_ for Obj2 {
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
    fn write_s1(&self, s1: &mut u16_XE_) -> Result<()> {
        *s1 = self.s1.into();
        Ok(())
    }
    fn write_s2(&self, s2: &mut u8) -> Result<()> {
        *s2 = self.s2;
        Ok(())
    }
    fn write_data(&self, data: &mut [u8]) -> Result<()> {
        data.write_from(self.data.as_slice())
    }
    fn write_vals(&self, vals: RotationQuantizationDump_XE_) -> Result<()> {
        match (&self.vals, vals) {
            (RotationQuantization::Polar32(src), RotationQuantizationDump_XE_::Polar32(dst)) => Ok(for (src, dst) in src.iter().zip(dst) { *dst = src.conv() }),
            (RotationQuantization::ThreeComp40(src), RotationQuantizationDump_XE_::ThreeComp40(dst)) => Ok(for (src, dst) in src.iter().zip(dst) { *dst = src.clone() }),
            (RotationQuantization::ThreeComp48(src), RotationQuantizationDump_XE_::ThreeComp48(dst)) => Ok(for (src, dst) in src.iter().zip(dst) { *dst = src.conv() }),
            (RotationQuantization::ThreeComp24(src), RotationQuantizationDump_XE_::ThreeComp24(dst)) => Ok(for (src, dst) in src.iter().zip(dst) { *dst = src.clone() }),
            (RotationQuantization::Straight16(src), RotationQuantizationDump_XE_::Straight16(dst)) => Ok(for (src, dst) in src.iter().zip(dst) { *dst = src.clone() }),
            (RotationQuantization::Uncompressed(src), RotationQuantizationDump_XE_::Uncompressed(dst)) => Ok(for (src, dst) in src.iter().zip(dst) { *dst = src.conv() }),
            _ => Err(anyhow!("missmatched rotation quantization and dump rotation quantization"))
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::FromBytes)]
pub struct Flags {
    pub f: u8,
    pub a: u8,
    pub b: u8,
    pub c: u8,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct BlockValARef<'a, T: AnimationTypes> {
    pub a: Obj1Ref<'a, T>,
    pub b: Obj2Ref<'a, T>,
    pub c: Obj1Ref<'a, T>,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct BlockValRef<'a, T: AnimationTypes> {
    pub vals_a: slice<BlockValARef<'a, T>>,
    pub vals_b: slice<Obj1Ref<'a, T>>
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct BlocksRef<'a, T: AnimationTypes> {
    pub block_starts: ref_slice<'a, T::u32>,
    pub block_starts2: ref_slice<'a, T::u32>,
    pub obj_c3: ref_slice<'a, T::u32>,
    pub obj_c4: ref_slice<'a, T::u32>,
    pub blocks: slice<BlockValRef<'a, T>>
}

impl<'a, T: AnimationTypes> BlocksRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &T::AnimationInfo) -> Result<Self> {
        let block_starts = T::u32::slice_from_data(&src[info.block_starts_offset() as usize..], info.block_starts_num() as usize).context("block_starts")?;
        let block_starts2 = T::u32::slice_from_data(&src[info.block_starts2_offset() as usize..], info.block_starts2_num() as usize).context("block_starts2")?;
        let obj_c3 = T::u32::slice_from_data(&src[info.obj_c3_offset() as usize..], info.obj_c3_num() as usize).context("obj_c3")?;
        let obj_c4 = T::u32::slice_from_data(&src[info.obj_c4_offset() as usize..], info.obj_c4_num() as usize).context("obj_c4")?;
        let mut blocks = Vec::with_capacity(block_starts.len());
        for (start, start2) in block_starts.iter().zip(block_starts2) {
            let off = (start.conv() + info.block_offset()) as usize;
            let flags = Flags::slice_from_data(&src[off..], info.vals_num() as usize).context("flags")?;
            let flags2 = u8::slice_from_data(&src[off + flags.size_of_val()..], info.vals2_num() as usize).context("flags2")?; 
            let mut off =
                (info.block_offset() + start.conv() + info.data_offset()) as usize;
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
            off = (info.block_offset() + start.conv() + start2.conv()) as usize;
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

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct BlockValARef_XE_<'a> {
    pub a: Obj1Ref_XE_<'a>,
    pub b: Obj2Ref_XE_<'a>,
    pub c: Obj1Ref_XE_<'a>,
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct BlockValRef_XE_<'a> {
    pub vals_a: slice<BlockValARef_XE_<'a>>,
    pub vals_b: slice<Obj1Ref_XE_<'a>>
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct BlocksRef_XE_<'a> {
    pub block_starts: ref_slice<'a, u32_XE_>,
    pub block_starts2: ref_slice<'a, u32_XE_>,
    pub obj_c3: ref_slice<'a, u32_XE_>,
    pub obj_c4: ref_slice<'a, u32_XE_>,
    pub blocks: slice<BlockValRef_XE_<'a>>
}

#[make_endian]
impl<'a> BlocksRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &AnimationInfo_XE_) -> Result<Self> {
        let block_starts = u32_XE_::slice_from_data(&src[info.block_starts_offset.conv()..], info.block_starts_num.conv()).context("block_starts")?;
        let block_starts2 = u32_XE_::slice_from_data(&src[info.block_starts2_offset.conv()..], info.block_starts2_num.conv()).context("block_starts2")?;
        let obj_c3 = u32_XE_::slice_from_data(&src[info.obj_c3_offset.conv()..], info.obj_c3_num.conv()).context("obj_c3")?;
        let obj_c4 = u32_XE_::slice_from_data(&src[info.obj_c4_offset.conv()..], info.obj_c4_num.conv()).context("obj_c4")?;
        let mut blocks = Vec::with_capacity(block_starts.len());
        for (start, start2) in block_starts.iter().zip(block_starts2) {
            let off = (start.to_native() + info.block_offset.to_native()) as usize;
            let flags = Flags::slice_from_data(&src[off..], info.vals_num.conv()).context("flags")?;
            let flags2 = u8::slice_from_data(&src[off + flags.size_of_val()..], info.vals2_num.conv()).context("flags2")?; 
            let mut off =
                (info.block_offset.to_native() + start.to_native() + info.data_offset.to_native()) as usize;
            let mut vals_a = Vec::with_capacity(flags.len());
            let mut vals_b = Vec::with_capacity(flags2.len());
            for flag in flags {
                let a = Obj1Ref_XE_::from_data(&src[off..], flag.a, flag.f & 3)?;
                off = align_offset(off + a.size, 4);
                let b = Obj2Ref_XE_::from_data(&src[off..], flag.b, (flag.f >> 2) & 0xf)?;
                off = align_offset(off + b.size, 4);
                let c = Obj1Ref_XE_::from_data(&src[off..], flag.c, (flag.f >> 6) & 3)?;
                off = align_offset(off + c.size, 4);
                vals_a.push(BlockValARef_XE_ { a, b, c });
            }
            off = (info.block_offset.to_native() + start.to_native() + start2.to_native()) as usize;
            for flag in flags2 {
                let d = Obj1Ref_XE_::from_data(&src[off..], flag & 0xf9, (flag >> 1) & 3)?;
                off = align_offset(off + d.size, 4);
                vals_b.push(d);
            }
            blocks.push(BlockValRef_XE_ {
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

#[make_endian]
impl From<&BlocksRef_XE_<'_>> for Blocks {
    fn from(val: &BlocksRef_XE_) -> Self {
        Self {
            obj_c3: val
                .obj_c3
                .iter()
                .map(|x| x.conv()) 
                .collect(),
            obj_c4: val
                .obj_c4
                .iter()
                .map(|x| x.conv())
                .collect(),
            blocks: val.blocks.iter()
                .map(|BlockValRef_XE_ { vals_a, vals_b }| (
                    vals_a.iter().map(|BlockValARef_XE_ { a, b, c }| (a.into(), b.into(), c.into())).collect(),
                    vals_b.iter().map(|x| x.into()).collect()
                ))
                .collect()
        }
    }
}

#[make_endian]
pub trait DumpBlocks_XE_ {
    fn blocks_len(&self) -> usize;
    fn block1_len(&self) -> usize;
    fn block2_len(&self) -> usize;
    fn obj_c3_len(&self) -> usize;
    fn obj_c4_len(&self) -> usize;

    fn blocks(
        &self,
    ) -> impl Iterator<
        Item = (
            impl Iterator<Item = (&impl DumpObj1_XE_, &impl DumpObj2_XE_, &impl DumpObj1_XE_)>,
            impl Iterator<Item = &impl DumpObj1_XE_>,
        ),
    >;
    fn write_obj_c3(&self, obj_c3: &mut [u32_XE_]) -> Result<()>;
    fn write_obj_c4(&self, obj_c4: &mut [u32_XE_]) -> Result<()>;

    fn dump_into(
        &self,
        dst: &mut DumpSlice,
        info: &mut AnimationInfo_XE_,
        start: usize
    ) -> Result<()> {
        info.vals_num = self.block1_len().conv();
        info.vals2_num = self.block2_len().conv();
        info.data_offset = (self.block1_len() * Flags::size_of() + self.block2_len()).conv();

        info.block_starts_offset = (dst.offset - start).conv();
        let block_starts = u32_XE_::mut_slice_from_data(dst, self.blocks_len()).context("block_starts")?;
        info.block_starts_num = block_starts.len().conv();

        info.block_starts2_offset = (dst.offset - start).conv();
        let block_starts2 = u32_XE_::mut_slice_from_data(dst, self.blocks_len()).context("block_starts2")?;
        info.block_starts2_num = block_starts2.len().conv();

        info.obj_c3_offset = (dst.offset - start).conv();
        let obj_c3 = u32_XE_::mut_slice_from_data(dst, self.obj_c3_len()).context("obj_c3")?;
        self.write_obj_c3(obj_c3).context("write obj_c3")?;
        info.obj_c3_num = obj_c3.len().conv();

        info.obj_c4_offset = (dst.offset - start).conv();
        let obj_c4 = u32_XE_::mut_slice_from_data(dst, self.obj_c4_len()).context("obj_c4")?;
        self.write_obj_c4(obj_c4).context("write obj_c4")?;
        info.obj_c4_num = obj_c4.len().conv();

        let start_off = dst.offset;
        info.block_offset = (dst.offset - start).conv();
        for (i, ((vals, vals2), (block_start, block_start2))) in self
            .blocks()
            .zip(block_starts.iter_mut().zip(block_starts2))
            .enumerate()
        {
            let start = (dst.offset - start_off);
            *block_start = start.conv();

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

            *block_start2 = (dst.offset - start_off - start).conv();
            for (j, (d, flag)) in vals2.zip(flags2).enumerate() {
                *flag = (d.flags() | (d.vals_kind() << 1));
                d.dump_into(dst).with_context(|| format!("block {} d {}", i, j))?;
                dst.adjusted_align(4, start_off)?;
            }
            dst.adjusted_align(16, start_off)?;
        }
        info.block_size = (dst.offset - start_off).conv();
        Ok(())
    }

    fn size(&self) -> usize {
        let mut size = 0;
        size += u32_XE_::size_of() * self.blocks_len() * 2;
        size += u32_XE_::size_of() * (self.obj_c3_len() + self.obj_c4_len());
        let mut blocks_size = 0;
        for (vals, vals2) in self.blocks() {
            blocks_size += Flags::size_of() * self.block1_len();
            blocks_size += u8::size_of() * self.block2_len();
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

#[make_endian]
impl DumpBlocks_XE_ for BlocksRef_XE_<'_> {
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
            impl Iterator<Item = (&impl DumpObj1_XE_, &impl DumpObj2_XE_, &impl DumpObj1_XE_)>,
            impl Iterator<Item = &impl DumpObj1_XE_>,
        ),
    > {
        self.blocks
            .iter()
            .map(|vals| (vals.vals_a.iter().map(|BlockValARef_XE_ { a, b, c }| (a, b, c)), vals.vals_b.iter()))
    }
    fn write_obj_c3(&self, obj_c3: &mut [u32_XE_]) -> Result<()> {
        obj_c3.write_from(&self.obj_c3[..])
    }
    fn write_obj_c4(&self, obj_c4: &mut [u32_XE_]) -> Result<()> {
        obj_c4.write_from(&self.obj_c4[..])
    }
}

#[make_endian]
impl DumpBlocks_XE_ for Blocks {
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
            impl Iterator<Item = (&impl DumpObj1_XE_, &impl DumpObj2_XE_, &impl DumpObj1_XE_)>,
            impl Iterator<Item = &impl DumpObj1_XE_>,
        ),
    > {
        self.blocks
            .iter()
            .map(|(vals, vals2)| (vals.iter().map(|(a,b,c)|(a,b,c)), vals2.iter()))
    }
    fn write_obj_c3(&self, obj_c3: &mut [u32_XE_]) -> Result<()> {
        for (src, dst) in self.obj_c3.iter().zip(obj_c3) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_obj_c4(&self, obj_c4: &mut [u32_XE_]) -> Result<()> {
        for (src, dst) in self.obj_c4.iter().zip(obj_c4) {
            *dst = src.conv();
        }
        Ok(())
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Obj5Header_XE_ {
    pub obj_a_num: u32_XE_,
    pub obj_a_offset: u32_XE_,
    pub obj_b_num: u32_XE_,
    pub obj_b_offset: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Obj5Val_XE_ {
    pub unk_0: f32_XE_,
    pub unk_1: f32_XE_,
    pub unk_2: f32_XE_,
    pub unk_3: f32_XE_,
    pub unk_4: f32_XE_,
    pub unk_5: f32_XE_,
    pub unk_6: f32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Obj3_XE_ {
    pub t: f32_XE_,
    pub event: Crc_XE_,
    pub dat_2: u32_XE_,
    pub dat_3: u32_XE_,
    pub dat_4: u32_XE_,
    pub dat_5: u32_XE_,
    pub dat_6: u32_XE_,
    pub dat_7: u32_XE_,
    pub dat_8: u32_XE_,
    pub dat_9: u32_XE_,
    pub dat_10: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AnimationInfo_XE_ {
    pub key: Crc_XE_,
    pub gamemodemask: i32_XE_,
    pub offset: u32_XE_,
    pub size: u32_XE_,
    pub kind: u32_XE_, // always 3
    pub unk_5: f32_XE_,
    pub vals_num: u32_XE_,
    pub vals2_num: u32_XE_,
    pub unk_8: u32_XE_,
    pub vala: u32_XE_,                 // numFrames
    pub unk_10: u32_XE_,               // numBlocks
    pub unk_11: u32_XE_,               // maxFramesPerBlock
    pub data_offset: u32_XE_,          // relative to block_starts, maskAndQuantizationSize
    pub unk_13: f32_XE_,               // blockDuration
    pub unk_14: f32_XE_,               // blockInverseDuration
    pub t_scale: f32_XE_,              // frameDuration
    pub block_starts_offset: u32_XE_,  //relative to block_offset, blockOffsets
    pub block_starts_num: u32_XE_,     // relative to block_starts, nunBlocks
    pub block_starts2_offset: u32_XE_, // floatBlockOffsets
    pub block_starts2_num: u32_XE_,    // numFloatBlocks
    pub obj_c3_offset: u32_XE_,        // unused, equal to block_start, transformOffsets
    pub obj_c3_num: u32_XE_,           // unused, equal to block_start, numTransforms
    pub obj_c4_offset: u32_XE_,        // floatOffsets
    pub obj_c4_num: u32_XE_,           // numFloats
    pub block_offset: u32_XE_,         // data
    pub block_size: u32_XE_,           // dataSize
    pub obj3_num: u32_XE_,             // numEvents
    pub obj3_offset: u32_XE_,          // eventOffsets
    pub bones_num1: u32_XE_,           // bones is at least this + obj1_num long
    pub unk_29: u32_XE_,
    pub obj1_num: u32_XE_,
    pub bones_offset: u32_XE_,
    pub unk_32: u32_XE_,
    pub obj1_offset: u32_XE_,
    pub obj2_offset: u32_XE_,
    pub obj2_num: u32_XE_,
    pub obj5_offset: u32_XE_, // to some object that contains offsets in pos 1 and 2 and a value in pos 0
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct AnimationRef<'a, T: AnimationTypes> {
    pub info: &'a T::AnimationInfo,
    pub obj1: ref_slice<'a, T::u32>,
    pub obj2: ref_slice<'a, T::u32>,
    pub obj3: ref_slice<'a, T::Obj3>,
    pub bones: ref_slice<'a, T::Crc>,
    pub obj5_header: Option<&'a T::Obj5Header>,
    pub obj5_a: ref_slice<'a, T::Obj5Val>,
    pub obj5_b: ref_slice<'a, T::Obj5Val>,
    pub blocks: Option<BlocksRef<'a, T>>,
    pub size: usize
}

impl<'a, T: AnimationTypes> AnimationRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a T::AnimationInfo) -> Result<Self> {
        let obj1 = T::u32::slice_from_data(&src[info.obj1_offset() as usize..],
            info.obj1_num() as usize * 2)
            .context("obj1")?;
        let obj2 = T::u32::slice_from_data(&src[info.obj2_offset() as usize..],
            info.obj2_num() as usize * 4)
            .context("obj2")?;
        let obj3 = T::Obj3::slice_from_data(&src[info.obj3_offset() as usize..],
            info.obj3_num() as usize)
            .context("Obj3")?;
        let bones = T::Crc::slice_from_data(&src[info.bones_offset() as usize..],
            (info.vals_num() + info.obj1_num()) as usize)
            .context("bones")?;
        let (obj5_header, obj5_a, obj5_b) = if info.obj5_offset() != 0 {
            let obj5_header = T::Obj5Header::from_data(&src[info.obj5_offset() as usize..])
                .context("obj5_header")?;
            let obj5_a = T::Obj5Val::slice_from_data(&src[obj5_header.obj_a_offset() as usize..],
                obj5_header.obj_a_num() as usize)
                .context("obj5_a")?;
            let obj5_b = T::Obj5Val::slice_from_data(&src[obj5_header.obj_b_offset() as usize..],
                obj5_header.obj_b_num() as usize)
                .context("obj5_b")?;
            (Some(obj5_header.into()), obj5_a, obj5_b)
        } else {
            (None, &[] as &_, &[] as &_)
        };
        let blocks = if info.kind() == 3 {
            Some(BlocksRef::from_data(src, info).context("blocks")?)
        } else if info.kind() < 3 {
            warn!("Unhandled animation type {}", info.kind());
            None
        } else {
            warn!("Unknown animation type {}", info.kind());
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
            size: info.size() as usize
        })
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct AnimationRef_XE_<'a> {
    pub info: &'a AnimationInfo_XE_,
    pub obj1: ref_slice<'a, u32_XE_>,
    pub obj2: ref_slice<'a, u32_XE_>,
    pub obj3: ref_slice<'a, Obj3_XE_>,
    pub bones: ref_slice<'a, Crc_XE_>,
    pub obj5_header: Option<&'a Obj5Header_XE_>,
    pub obj5_a: ref_slice<'a, Obj5Val_XE_>,
    pub obj5_b: ref_slice<'a, Obj5Val_XE_>,
    pub blocks: Option<BlocksRef_XE_<'a>>,
    pub size: usize
}

#[make_endian]
impl<'a> AnimationRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a AnimationInfo_XE_) -> Result<Self> {
        let obj1 = u32_XE_::slice_from_data(&src[info.obj1_offset.conv()..],
            info.obj1_num.to_native() as usize * 2)
            .context("obj1")?;
        let obj2 = u32_XE_::slice_from_data(&src[info.obj2_offset.conv()..],
            info.obj2_num.to_native() as usize * 4)
            .context("obj2")?;
        let obj3 = Obj3_XE_::slice_from_data(&src[info.obj3_offset.conv()..],
            info.obj3_num.conv())
            .context("Obj3")?;
        let bones = Crc_XE_::slice_from_data(&src[info.bones_offset.conv()..],
            (info.vals_num.to_native() + info.obj1_num.to_native()) as usize)
            .context("bones")?;
        let (obj5_header, obj5_a, obj5_b) = if info.obj5_offset != 0 {
            let obj5_header = Obj5Header_XE_::from_data(&src[info.obj5_offset.conv()..])
                .context("obj5_header")?;
            let obj5_a = Obj5Val_XE_::slice_from_data(&src[obj5_header.obj_a_offset.conv()..],
                obj5_header.obj_a_num.conv())
                .context("obj5_a")?;
            let obj5_b = Obj5Val_XE_::slice_from_data(&src[obj5_header.obj_b_offset.conv()..],
                obj5_header.obj_b_num.conv())
                .context("obj5_b")?;
            (Some(obj5_header.into()), obj5_a, obj5_b)
        } else {
            (None, &[] as &_, &[] as &_)
        };
        let blocks = if info.kind == 3 {
            Some(BlocksRef_XE_::from_data(src, info).context("blocks")?)
        } else if info.kind < 3 {
            warn!("Unhandled animation type {}", info.kind);
            None
        } else {
            warn!("Unknown animation type {}", info.kind);
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
            size: info.size.conv()
        })
    }
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub info: AnimationInfo,
    pub obj1: Vec<u32>,
    pub obj2: Vec<u32>,
    pub obj3: Vec<Obj3>,
    pub bones: Vec<Crc>,
    pub obj5_a: Vec<Obj5Val>,
    pub obj5_b: Vec<Obj5Val>,
    pub blocks: Option<Blocks>,
}

#[make_endian]
#[enum_dispatch(DumpAnimation_XE_)]
pub enum Animation_XE_<'a> {
    Ref(AnimationRef_XE_<'a>),
    Owned(Animation)
}

#[make_endian]
impl From<&AnimationRef_XE_<'_>> for Animation {
    fn from(val: &AnimationRef_XE_) -> Self {
        Self {
            info: val.info.conv(),
            obj1: val
                .obj1
                .iter()
                .map(|x| x.conv())
                .collect(),
            obj2: val
                .obj2
                .iter()
                .map(|x| x.conv())
                .collect(),
            obj3: val
                .obj3
                .iter()
                .map(|x| x.conv())
                .collect(),
            bones: val
                .bones
                .iter()
                .map(|x| x.conv())
                .collect(),
            obj5_a: val
                .obj5_a
                .iter()
                .map(|x| x.conv())
                .collect(),
            obj5_b: val
                .obj5_b
                .iter()
                .map(|x| x.conv())
                .collect(),
            blocks: val.blocks.as_ref().map(|x| x.into()),
        }
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AnimationBlockInfo_XE_ {
    pub key: Crc_XE_,
    pub guid: u32_XE_,
    pub key_name: Crc_XE_,
    pub offset: u32_XE_,
    pub size: u32_XE_,
    pub size_comp: u32_XE_,
    pub unk_6: u32_XE_,
    pub unk_7: u32_XE_,
    pub unk_8: u32_XE_,
}


#[make_endian]
#[enum_dispatch]
pub trait DumpAnimationImpl_XE_ {
    fn key(&self) -> u32;
    fn gamemodemask(&self) -> i32;
    fn obj1_len(&self) -> usize;
    fn obj2_len(&self) -> usize;
    fn obj3_len(&self) -> usize;
    fn bones_len(&self) -> usize;
    fn obj5_a_len(&self) -> usize;
    fn obj5_b_len(&self) -> usize;
    fn write_obj1(&self, obj1: &mut [u32_XE_]) -> Result<()>;
    fn write_obj2(&self, obj2: &mut [u32_XE_]) -> Result<()>;
    fn write_obj3(&self, obj3: &mut [Obj3_XE_]) -> Result<()>;
    fn write_bones(&self, bones: &mut [Crc_XE_]) -> Result<()>;
    fn write_obj5_a(&self, obj5_a: &mut [Obj5Val_XE_]) -> Result<()>;
    fn write_obj5_b(&self, obj5_b: &mut [Obj5Val_XE_]) -> Result<()>;
    fn blocks(&self) -> Option<&impl DumpBlocks_XE_>;
    fn write_info(&self, info: &mut AnimationInfo_XE_) -> Result<()>;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpAnimation_XE_ {
    fn key(&self) -> u32;
    fn gamemodemask(&self) -> i32;
    fn dump_into(
        &self,
        dst: &mut DumpSlice,
        info: &mut AnimationInfo_XE_,
    ) -> Result<()>;

    fn size(&self) -> usize;
}

#[make_endian]
impl<T: DumpAnimationImpl_XE_> DumpAnimation_XE_ for T {
    #[inline(always)]
    fn key(&self) -> u32 {
        DumpAnimationImpl_XE_::key(self)
    }
    #[inline(always)]
    fn gamemodemask(&self) -> i32 {
        DumpAnimationImpl_XE_::gamemodemask(self)
    }
    fn dump_into(
        &self,
        dst: &mut DumpSlice,
        info: &mut AnimationInfo_XE_,
    ) -> Result<()> {
        self.write_info(info).context("write_info")?;
        let start = dst.offset;
        info.offset = dst.offset.conv();

        info.obj1_offset = (dst.offset - start).conv();
        let obj1 = u32_XE_::mut_slice_from_data(dst, self.obj1_len()).context("obj1")?;
        info.obj1_num = (obj1.len() / 2).conv();
        self.write_obj1(obj1).context("write obj1")?;

        info.obj2_offset = if self.obj2_len() == 0 { 0 } else { dst.offset - start }.conv();
        let obj2 = u32_XE_::mut_slice_from_data(dst, self.obj2_len()).context("obj2")?;
        info.obj2_num = (obj2.len() / 4).conv();
        self.write_obj2(obj2).context("write obj2")?;

        if let Some(blocks) = self.blocks() {
            blocks.dump_into(dst, info, start).context("blocks")?;
        }

        info.obj3_offset = if self.obj3_len() == 0 { 0 } else { dst.offset - start }.conv();
        let obj3 = Obj3_XE_::mut_slice_from_data(dst, self.obj3_len()).context("obj3")?;
        info.obj3_num = obj3.len().conv();
        self.write_obj3(obj3).context("write obj3")?;

        info.bones_offset = (dst.offset - start).conv();
        let bones = Crc_XE_::mut_slice_from_data(dst, self.bones_len()).context("bones")?;
        self.write_bones(bones).context("write bones")?;

        if self.obj5_a_len() != 0 || self.obj5_b_len() != 0 {
            info.obj5_offset = (dst.offset - start).conv();
            let obj5_header = Obj5Header_XE_::mut_from_data(dst).context("obj5_header")?;

            obj5_header.obj_a_offset = if self.obj5_a_len() == 0 { 0 } else { dst.offset - start }.conv();
            let obj5_a = Obj5Val_XE_::mut_slice_from_data(dst, self.obj5_a_len()).context("obj5_a")?;
            obj5_header.obj_a_num = obj5_a.len().conv();
            self.write_obj5_a(obj5_a).context("write obj5_a")?;

            obj5_header.obj_b_offset = if self.obj5_b_len() == 0 { 0 } else { dst.offset - start }.conv();
            let obj5_b = Obj5Val_XE_::mut_slice_from_data(dst, self.obj5_b_len()).context("obj5_b")?;
            obj5_header.obj_b_num = obj5_b.len().conv();
            self.write_obj5_b(obj5_b).context("write obj5_b")?;
        } else {
            info.obj5_offset = 0usize.conv();
        }
        dst.align(16)?;
        info.size = (dst.offset - start).conv();
        Ok(())
    }

    fn size(&self) -> usize {
        let mut size = 0;

        size += (self.obj1_len() + self.obj2_len()) * u32_XE_::size_of();

        if let Some(blocks) = self.blocks() {
            size += blocks.size();
        }

        size += Obj3_XE_::size_of() * self.obj3_len();

        size += Crc_XE_::size_of() * self.bones_len();

        if self.obj5_a_len() != 0 || self.obj5_b_len() != 0 {
            size += Obj5Header_XE_::size_of();
            size += (self.obj5_a_len() + self.obj5_b_len()) * Obj5Val_XE_::size_of()
        }

        align_offset(size, 16)
    }
}

#[make_endian]
impl DumpAnimationImpl_XE_ for AnimationRef_XE_<'_> {
    fn key(&self) -> u32 {
        self.info.key.conv()
    }
    fn gamemodemask(&self) -> i32 {
        self.info.gamemodemask.conv()
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
    fn write_obj1(&self, obj1: &mut [u32_XE_]) -> Result<()> {
        obj1.write_from(&self.obj1[..])
    }
    fn write_obj2(&self, obj2: &mut [u32_XE_]) -> Result<()> {
        obj2.write_from(&self.obj2[..])
    }
    fn write_obj3(&self, obj3: &mut [Obj3_XE_]) -> Result<()> {
        obj3.write_from(&self.obj3[..])
    }
    fn write_bones(&self, bones: &mut [Crc_XE_]) -> Result<()> {
        bones.write_from(&self.bones[..])
    }
    fn write_obj5_a(&self, obj5_a: &mut [Obj5Val_XE_]) -> Result<()> {
        obj5_a.write_from(&self.obj5_a[..])
    }
    fn write_obj5_b(&self, obj5_b: &mut [Obj5Val_XE_]) -> Result<()> {
        obj5_b.write_from(&self.obj5_b[..])
    }
    fn blocks(&self) -> Option<&impl DumpBlocks_XE_> {
        self.blocks.as_ref()
    }
    fn write_info(&self, info: &mut AnimationInfo_XE_) -> Result<()> {
        info.write_from(self.info)
    }
}

#[make_endian]
impl DumpAnimationImpl_XE_ for Animation {
    fn key(&self) -> u32 {
        self.info.key.get()
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
    fn write_obj1(&self, obj1: &mut [u32_XE_]) -> Result<()> {
        for (src, dst) in self.obj1.iter().zip(obj1) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_obj2(&self, obj2: &mut [u32_XE_]) -> Result<()> {
        for (src, dst) in self.obj2.iter().zip(obj2) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_obj3(&self, obj3: &mut [Obj3_XE_]) -> Result<()> {
        for (src, dst) in self.obj3.iter().zip(obj3) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_bones(&self, bones: &mut [Crc_XE_]) -> Result<()> {
        for (src, dst) in self.bones.iter().zip(bones) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_obj5_a(&self, obj5_a: &mut [Obj5Val_XE_]) -> Result<()> {
        for (src, dst) in self.obj5_a.iter().zip(obj5_a) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_obj5_b(&self, obj5_b: &mut [Obj5Val_XE_]) -> Result<()> {
        for (src, dst) in self.obj5_b.iter().zip(obj5_b) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn blocks(&self) -> Option<&impl DumpBlocks_XE_> {
        self.blocks.as_ref()
    }
    fn write_info(&self, info: &mut AnimationInfo_XE_) -> Result<()> {
        *info = self.info.conv();
        Ok(())
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct AnimationsRef<'a, T: AnimationTypes> {
    pub animations: IndexMap<u32, AnimationRef<'a, T>>,
    pub block_infos: ref_slice<'a, T::AnimationBlockInfo>,
}

impl<T: AnimationTypes> Default for AnimationsRef<'_, T> {
    fn default() -> Self {
        Self {
            animations: IndexMap::default().into(),
            block_infos: ref_slice::default()
        }
    }
}

impl<'a, T: AnimationTypes> AnimationsRef<'a, T> {
    pub fn from_data(anim_infos: &'a [T::AnimationInfo], blocks: &'a [CompressedDataRef<'_>], block_infos: &'a [T::AnimationBlockInfo]) -> Result<Self> {
        let mut offsets = vec![0; blocks.len()];
        let mut animations = IndexMap::with_capacity(anim_infos.len());
        for info in anim_infos {
            for (i, (data, offset)) in blocks.iter().zip(offsets.iter()).enumerate() {
                if info.gamemodemask() >> i & 1 != 0 {
                    let val = AnimationRef::from_data(&data.get()[*offset..], info).with_context(|| format!("animation {}", i))?;
                    animations.insert(info.key().get(), val);
                    break;
                }
            }
            for (i, offset) in offsets.iter_mut().enumerate() {
                if info.gamemodemask() >> i & 1 != 0 {
                    *offset += animations.last().unwrap().1.size;
                }
            }
        }
        Ok(Self { animations: animations.into(), block_infos: block_infos.into() })
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct AnimationsRef_XE_<'a> {
    pub animations: IndexMap<u32, AnimationRef_XE_<'a>>,
    pub block_infos: ref_slice<'a, AnimationBlockInfo_XE_>,
}

#[make_endian]
impl Default for AnimationsRef_XE_<'_> {
    fn default() -> Self {
        Self {
            animations: IndexMap::default().into(),
            block_infos: ref_slice::default()
        }
    }
}

#[make_endian]
impl<'a> AnimationsRef_XE_<'a> {
    pub fn from_data(anim_infos: &'a [AnimationInfo_XE_], blocks: &'a [CompressedDataRef<'_>], block_infos: &'a [AnimationBlockInfo_XE_]) -> Result<Self> {
        let mut offsets = vec![0; blocks.len()];
        let mut animations = IndexMap::with_capacity(anim_infos.len());
        for info in anim_infos {
            for (i, (data, offset)) in blocks.iter().zip(offsets.iter()).enumerate() {
                if info.gamemodemask.to_native() >> i & 1 != 0 {
                    let val = AnimationRef_XE_::from_data(&data.get()[*offset..], info).with_context(|| format!("animation {}", i))?;
                    animations.insert(info.key.conv(), val);
                    break;
                }
            }
            for (i, offset) in offsets.iter_mut().enumerate() {
                if info.gamemodemask.to_native() >> i & 1 != 0 {
                    *offset += animations.last().unwrap().1.size();
                }
            }
        }
        Ok(Self { animations: animations.into(), block_infos: block_infos.into() })
    }
}

#[derive(Debug, Clone)]
pub struct Animations {
    pub animations: IndexMap<Crc, Animation>,
    pub block_infos: Vec<AnimationBlockInfo>
}

#[make_endian]
impl From<&AnimationsRef_XE_<'_>> for Animations {
    fn from(val: &AnimationsRef_XE_) -> Self {
        Self {
            animations: val.animations.values().map(|x| (x.info.key.conv(), x.into())).collect(),
            block_infos: val.block_infos.iter().map(|x| x.conv()).collect()
        }
    }
}

#[make_endian]
pub struct AnimationsImpl_XE_<'a> {
    pub animations: IndexMap<Crc, Animation_XE_<'a>>,
    pub block_infos: Vec<AnimationBlockInfo>
}

#[make_endian]
pub struct AnimationsRaw_XE_<'a, 'b> {
    pub anim_infos: &'a [AnimationInfo_XE_], 
    pub blocks: &'a [CompressedDataRef<'b>], 
    pub block_infos: &'a [AnimationBlockInfo_XE_]
}

#[make_endian]
#[enum_dispatch(DumpAnimations_XE_)]
pub enum Animations_XE_<'a, 'b> {
    Raw(AnimationsRaw_XE_<'a, 'b>),
    Owned(AnimationsImpl_XE_<'a>)
}

#[make_endian]
pub trait DumpAnimationsImpl_XE_ {
    fn animation_num(&self) -> usize;
    fn block_num(&self) -> usize;
    fn animations(&self) -> impl Iterator<Item=&impl DumpAnimation_XE_>;
    fn write_block_infos(&self, infos: &mut [AnimationBlockInfo_XE_]) -> Result<()>;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpAnimations_XE_ {
    fn animation_num(&self) -> usize;
    fn block_num(&self) -> usize;

    fn info_counts(&self, counts: &mut InfoCounts) {
        counts.animations = self.animation_num();
        counts.animation_blocks = self.block_num()
    }

    fn dump<'d>(&'d self, infos: &mut DumpInfos_XE_) -> Result<Vec<CompressedData<'d>>>;
}

#[make_endian]
impl<T: DumpAnimationsImpl_XE_> DumpAnimations_XE_ for T {
    #[inline(always)]
    fn animation_num(&self) -> usize {
        DumpAnimationsImpl_XE_::animation_num(self)
    }
    #[inline(always)]
    fn block_num(&self) -> usize {
        DumpAnimationsImpl_XE_::block_num(self)
    }

    fn dump<'d>(&'d self, infos: &mut DumpInfos_XE_) -> Result<Vec<CompressedData<'d>>> {
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
            block_info.size = (*size).conv();
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
                    unsafe { val.as_ref() }
                        .dump_into(dst)
                        .with_context(|| format!("pre-dumped animation {}", j))?;
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

#[make_endian]
impl DumpAnimationsImpl_XE_ for AnimationsRef_XE_<'_> {
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
        infos.write_from(&self.block_infos[..])
    }
}

#[make_endian]
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

#[make_endian]
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
            *dst = src.conv()
        }
        Ok(())
    }
}

#[make_endian]
impl DumpAnimationsImpl_XE_ for Animations {
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
            *dst = src.conv()
        }
        Ok(())
    }
}
