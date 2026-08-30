use anyhow::{Context, Result};
use enum_dispatch::enum_dispatch;
use lotrc_proc::{derive_pod};

use crate::types::{get_default_ref, Crc, DumpSlice, ReadData, ref_slice, align_offset, BaseTypes, NE};
//use crate::level::pak::block2::PFields;

#[derive_pod]
pub struct StringKeysHeader<T: BaseTypes> {
    pub num_a: T::u16,
    pub num_b: T::u16,
    pub z2: T::u32,
    pub z3: T::u32,
    pub z4: T::u32,
    pub z5: T::u32,
}

#[derive_pod]
pub struct StringKeysVal<T: BaseTypes> {
    pub key: Crc<T>,
    pub offset: T::u32,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct StringKeysRef<'a, T: BaseTypes> {
    pub header: &'a StringKeysHeader<T>,
    pub vals: ref_slice<'a, StringKeysVal<T>>,
    pub pad: ref_slice<'a, T::u32>,
}

impl<T: BaseTypes> Default for StringKeysRef<'_, T> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            vals: ref_slice::default(),
            pad: ref_slice::default()
        }
    }
}

impl<'a, T: BaseTypes> StringKeysRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = StringKeysHeader::<T>::from_data(&src[offset..]).context("header")?;
        assert!(header.num_a.into() == header.num_b.into(), "Seems to be true");
        offset += std::mem::size_of::<StringKeysHeader<T>>();
        let vals = StringKeysVal::slice_from_data(&src[offset..], header.num_a.into() as usize)
            .context("vals")?;
        offset += std::mem::size_of_val(vals);
        let pad = T::u32::slice_from_data(&src[offset..], vals.len()).context("pad")?;
        Ok(Self { header, vals: vals.into(), pad: pad.into() })
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct StringKeys {
    pub vals: Vec<Crc<NE>>,
}

impl<T: BaseTypes> From<&StringKeysRef<'_, T>> for StringKeys
where
    Crc<NE>: From<Crc<T>>,
{
    fn from(val: &StringKeysRef<T>) -> Self {
        Self {
            vals: val.vals.iter().map(|x| x.key.into()).collect(),
        }
    }
}

pub trait DumpStringKeys<T: BaseTypes> {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc<T>>);
    fn num(&self) -> usize;
    fn size(&self) -> usize {
        std::mem::size_of::<StringKeysHeader::<T>>()
            + self.num() * (std::mem::size_of::<T::u32>() + std::mem::size_of::<StringKeysVal<T>>())
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let num = self.num();
        let header = StringKeysHeader::<T>::mut_from_data(dst).context("header")?;
        let vals = StringKeysVal::mut_slice_from_data(dst, num).context("vals")?;
        T::u32::mut_slice_from_data(dst, num).context("pad")?;

        header.num_a = (num as u16).into();
        header.num_b = (num as u16).into();
        let mut off = std::mem::size_of::<StringKeysHeader<T>>() + num * std::mem::size_of::<StringKeysVal<T>>();
        self.write_keys(vals.iter_mut().map(|x| &mut x.key));
        for val in vals {
            val.offset = (off as u32).into();
            off += size_of::<T::u32>();
        }
        Ok(())
    }
}

impl<T: BaseTypes> DumpStringKeys<T> for StringKeysRef<'_, T> {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc<T>>) {
        for (key, val) in keys.zip(&self.vals[..]) {
            *key = val.key;
        }
    }
    fn num(&self) -> usize {
        self.vals.len()
    }
}

impl<T: BaseTypes> DumpStringKeys<T> for [u32] {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc<T>>) {
        for (key, &val) in keys.zip(self.iter()) {
            key.val = val.into();
        }
    }
    fn num(&self) -> usize {
        self.len()
    }
    
}

impl<T: BaseTypes> DumpStringKeys<T> for StringKeys
where
    Crc<T>: From<Crc<NE>>,
{
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc<T>>) {
        for (key, &val) in keys.zip(&self.vals) {
            *key = val.into();
        }
    }
    fn num(&self) -> usize {
        self.vals.len()
    }
}

#[derive_pod]
pub struct SubBlocksHeader<T: BaseTypes> {
    pub z0: T::u32,
    pub block_num: T::u32,
    pub z2: T::u32,
    pub z3: T::u32,
}

#[derive_pod]
pub struct SubBlocksBlockHeader<T: BaseTypes> {
    pub key: Crc<T>,
    pub offset: T::u32,
    pub size: T::u32,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct SubBlocksInfoRef<'a, T: BaseTypes> {
    pub header: &'a SubBlocksHeader<T>,
    pub block_headers: ref_slice<'a, SubBlocksBlockHeader<T>>,
}

impl<T: BaseTypes> Default for SubBlocksInfoRef<'_, T> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            block_headers: ref_slice::default(),
        }
    }
}

impl<'a, T: BaseTypes> SubBlocksInfoRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = SubBlocksHeader::<T>::from_data(&src[offset..]).context("header")?;
        offset += std::mem::size_of::<SubBlocksHeader<T>>();
        let block_headers = SubBlocksBlockHeader::slice_from_data(
            &src[offset..],
            header.block_num.into() as usize,
        )
        .context("block_headers")?;

        Ok(Self {
            header: header,
            block_headers: block_headers.into(),
        })
    }

    pub fn size(num: usize) -> usize {
        align_offset(
            std::mem::size_of::<SubBlocksBlockHeader<T>>() +
            (std::mem::size_of::<SubBlocksBlockHeader<T>>() * num),
            16
        )
    }

    pub fn dump<'d>(dst: &mut DumpSlice<'d>, num: usize) -> Result<&'d mut [SubBlocksBlockHeader<T>]> {
        let header = SubBlocksHeader::<T>::mut_from_data(dst).context("header")?;
        let block_headers = SubBlocksBlockHeader::mut_slice_from_data(dst, num).context("block_headers")?;
        header.block_num = (block_headers.len() as u32).into();
        dst.align(16)?; 
        Ok(block_headers)
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct DataRef<'a> {
    pub data: ref_slice<'a, u8>
}
impl<'a> DataRef<'a> {
    pub fn from_data(src: &'a [u8]) -> Self {
        Self { data: src.into() }
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    pub data: Vec<u8>,
}

impl From<&DataRef<'_>> for Data {
    fn from(val: &DataRef) -> Self {
        Self {
            data: val.data.to_vec(),
        }
    }
}

/*
#[enum_dispatch(DumpData_XE_)]
pub enum Data_XE_<'a> {
    Ref(DataRef_XE_<'a>),
    Owned(Data)
}
*/

#[enum_dispatch]
pub trait DumpDataImpl {
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
    fn size(&self) -> usize; 
}

impl DumpDataImpl for DataRef<'_> {
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let dst = u8::mut_slice_from_data(dst, self.data.len())?;
        dst.copy_from_slice(self.data);
        Ok(())
    }
    fn size(&self) -> usize {
        self.data.len()
    }  
}

impl DumpDataImpl for Data {
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let dst = u8::mut_slice_from_data(dst, self.data.len())?;
        dst.copy_from_slice(&self.data);
        Ok(())
    }
    fn size(&self) -> usize {
        self.data.len()
    } 
}

pub trait DumpString<T: BaseTypes> {
    fn string_size(&self) -> usize;
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()>;
}

impl<T: BaseTypes> DumpString<T> for ref_slice<'_, T::u16> {
    fn string_size(&self) -> usize {
        std::mem::size_of_val(*self)
    }
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()> {
        T::u16::mut_slice_from_data(dst, self.len())?.copy_from_slice(self);
        Ok(())
    }
}

impl<T: BaseTypes> DumpString<T> for String {
    fn string_size(&self) -> usize {
        self.encode_utf16().count() * std::mem::size_of::<T::u16>()
    }
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()> {
        let val = self.encode_utf16().collect::<Vec<_>>();
        let string = T::u16::mut_slice_from_data(dst, val.len()).context("string")?;
        for (src, dst) in val.into_iter().zip(string) {
            *dst = src.into();
        }
        Ok(())
    }
}
