use anyhow::{Context, Result};
use enum_dispatch::enum_dispatch;
use lotrc_proc::{make_endian, derive_ordered_data};
#[make_endian]
use crate::{
    types::{Crc_XE_, u16_XE_, u32_XE_},
    level::pak::block2::{PFieldsRef_XE_, PFields_XE_}
};

use crate::types::{get_default_ref, Crc, DumpData, DumpSlice, RefFromData, OrderedData, ref_slice, align_offset, EndianTypes};
use crate::level::pak::block2::PFields;

pub trait SubBlockTypes: EndianTypes {
    type StringKeysHeader:  std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + StringKeysHeaderTypeTrait;
    type StringKeysVal:  std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + StringKeysValTypeTrait;
    type SubBlocksHeader:  std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + SubBlocksHeaderTypeTrait;
    type SubBlocksBlockHeader:  std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + SubBlocksBlockHeaderTypeTrait;
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StringKeysHeader_XE_ {
    pub num_a: u16_XE_,
    pub num_b: u16_XE_,
    pub z2: u32_XE_,
    pub z3: u32_XE_,
    pub z4: u32_XE_,
    pub z5: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StringKeysVal_XE_ {
    pub key: Crc_XE_,
    pub offset: u32_XE_,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct StringKeysRef<'a, T: SubBlockTypes> {
    pub header: &'a T::StringKeysHeader,
    pub vals: ref_slice<'a, T::StringKeysVal>,
    pub pad: ref_slice<'a, T::u32>,
}

#[make_endian]
impl<T: SubBlockTypes> Default for StringKeysRef<'_, T> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            vals: ref_slice::default(),
            pad: ref_slice::default()
        }
    }
}

#[make_endian]
impl<'a, T: SubBlockTypes> StringKeysRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = T::StringKeysHeader::from_data(&src[offset..]).context("header")?;
        assert!(header.num_a() == header.num_b(), "Seems to be true");
        offset += header.size_of_val();
        let vals = T::StringKeysVal::slice_from_data(&src[offset..], header.num_a() as usize)
            .context("vals")?;
        offset += vals.size_of_val();
        let pad = T::u32::slice_from_data(&src[offset..], vals.len()).context("pad")?;
        Ok(Self { header, vals: vals.into(), pad: pad.into() })
    }
}

#[make_endian]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct StringKeysRef_XE_<'a> {
    pub header: &'a StringKeysHeader_XE_,
    pub vals: ref_slice<'a, StringKeysVal_XE_>,
    pub pad: ref_slice<'a, u32_XE_>,
}

#[make_endian]
impl Default for StringKeysRef_XE_<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            vals: ref_slice::default(),
            pad: ref_slice::default()
        }
    }
}

#[make_endian]
impl<'a> StringKeysRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = StringKeysHeader_XE_::from_data(&src[offset..]).context("header")?;
        assert!(header.num_a == header.num_b, "Seems to be true");
        offset += header.size_of_val();
        let vals = StringKeysVal_XE_::slice_from_data(&src[offset..], header.num_a.conv())
            .context("vals")?;
        offset += vals.size_of_val();
        let pad = u32_XE_::slice_from_data(&src[offset..], vals.len()).context("pad")?;
        Ok(Self { header, vals: vals.into(), pad: pad.into() })
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct StringKeys {
    pub vals: Vec<Crc>,
}

#[make_endian]
impl From<&StringKeysRef_XE_<'_>> for StringKeys {
    fn from(val: &StringKeysRef_XE_) -> Self {
        Self {
            vals: val.vals.iter().map(|x| x.key.conv()).collect(),
        }
    }
}

#[make_endian]
pub trait DumpStringKeys_XE_ {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc_XE_>);
    fn num(&self) -> usize;
    fn size(&self) -> usize {
        StringKeysHeader_XE_::size_of()
            + self.num() * (u32_XE_::size_of() + StringKeysVal_XE_::size_of())
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let num = self.num();
        let header = StringKeysHeader_XE_::mut_from_data(dst).context("header")?;
        let vals = StringKeysVal_XE_::mut_slice_from_data(dst, num).context("vals")?;
        u32_XE_::mut_slice_from_data(dst, num).context("pad")?;

        header.num_a = num.conv();
        header.num_b = num.conv();
        let mut off = std::mem::size_of::<StringKeysHeader_XE_>() + num * std::mem::size_of::<StringKeysVal_XE_>();
        self.write_keys(vals.iter_mut().map(|x| &mut x.key));
        for val in vals {
            val.offset = off.conv();
            off += size_of::<u32_XE_>();
        }
        Ok(())
    }
}

#[make_endian]
impl DumpStringKeys_XE_ for StringKeysRef_XE_<'_> {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc_XE_>) {
        for (key, val) in keys.zip(&self.vals[..]) {
            *key = val.key;
        }
    }
    fn num(&self) -> usize {
        self.vals.len()
    }
}

#[make_endian]
impl DumpStringKeys_XE_ for [u32] {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc_XE_>) {
        for (key, val) in keys.zip(self.iter()) {
            *key = val.conv();
        }
    }
    fn num(&self) -> usize {
        self.len()
    }
    
}

#[make_endian]
impl DumpStringKeys_XE_ for StringKeys {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc_XE_>) {
        for (key, val) in keys.zip(&self.vals) {
            *key = val.conv();
        }
    }
    fn num(&self) -> usize {
        self.vals.len()
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SubBlocksHeader_XE_ {
    pub z0: u32_XE_,
    pub block_num: u32_XE_,
    pub z2: u32_XE_,
    pub z3: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SubBlocksBlockHeader_XE_ {
    pub key: Crc_XE_,
    pub offset: u32_XE_,
    pub size: u32_XE_,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct SubBlocksInfoRef<'a, T: SubBlockTypes> {
    pub header: &'a T::SubBlocksHeader,
    pub block_headers: ref_slice<'a, T::SubBlocksBlockHeader>,
}

impl<T: SubBlockTypes> Default for SubBlocksInfoRef<'_, T> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            block_headers: ref_slice::default(),
        }
    }
}

impl<'a, T: SubBlockTypes> SubBlocksInfoRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = T::SubBlocksHeader::from_data(&src[offset..]).context("header")?;
        offset += header.size_of_val();
        let block_headers = T::SubBlocksBlockHeader::slice_from_data(
            &src[offset..],
            header.block_num() as usize,
        )
        .context("block_headers")?;

        Ok(Self {
            header: header,
            block_headers: block_headers.into(),
        })
    }

    pub fn size(num: usize) -> usize {
        align_offset(
            std::mem::size_of::<T::SubBlocksBlockHeader>() +
            (std::mem::size_of::<T::SubBlocksBlockHeader>() * num),
            16
        )
    }

    /*
    pub fn dump<'d>(dst: &mut DumpSlice<'d>, num: usize) -> Result<&'d mut [T::SubBlocksBlockHeader]> {
        let header = T::SubBlocksHeader::mut_from_data(dst).context("header")?;
        let block_headers = T::SubBlocksBlockHeader::mut_slice_from_data(dst, num).context("block_headers")?;
        header.block_num = block_headers.len().conv();
        dst.align(16)?; 
        Ok(block_headers)
    }
    */
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct SubBlocksInfoRef_XE_<'a> {
    pub header: &'a SubBlocksHeader_XE_,
    pub block_headers: ref_slice<'a, SubBlocksBlockHeader_XE_>,
}

#[make_endian]
impl Default for SubBlocksInfoRef_XE_<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            block_headers: ref_slice::default(),
        }
    }
}

#[make_endian]
impl<'a> SubBlocksInfoRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = SubBlocksHeader_XE_::from_data(&src[offset..]).context("header")?;
        offset += header.size_of_val();
        let block_headers = SubBlocksBlockHeader_XE_::slice_from_data(
            &src[offset..],
            header.block_num.conv(),
        )
        .context("block_headers")?;

        Ok(Self {
            header: header,
            block_headers: block_headers.into(),
        })
    }

    pub fn size(num: usize) -> usize {
        align_offset(
            std::mem::size_of::<SubBlocksBlockHeader_XE_>() +
            (std::mem::size_of::<SubBlocksBlockHeader_XE_>() * num),
            16
        )
    }

    pub fn dump<'d>(dst: &mut DumpSlice<'d>, num: usize) -> Result<&'d mut [SubBlocksBlockHeader_XE_]> {
        let header = SubBlocksHeader_XE_::mut_from_data(dst).context("header")?;
        let block_headers = SubBlocksBlockHeader_XE_::mut_slice_from_data(dst, num).context("block_headers")?;
        header.block_num = block_headers.len().conv();
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

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct DataRef_XE_<'a> {
    pub data: ref_slice<'a, u8>
}

#[make_endian]
impl<'a> DataRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Self {
        Self { data: src.into() }
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    pub data: Vec<u8>,
}

#[make_endian]
impl From<&DataRef_XE_<'_>> for Data {
    fn from(val: &DataRef_XE_) -> Self {
        Self {
            data: val.data.to_vec(),
        }
    }
}

#[make_endian]
#[enum_dispatch(DumpData_XE_)]
pub enum Data_XE_<'a> {
    Ref(DataRef_XE_<'a>),
    Owned(Data)
}

#[make_endian]
#[enum_dispatch]
pub trait DumpData_XE_ {
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
    fn size(&self) -> usize; 
}

#[make_endian]
impl DumpData_XE_ for DataRef_XE_<'_> {
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        self.data.dump_into(dst)
    }
    fn size(&self) -> usize {
        self.data.len()
    }  
}

#[make_endian]
impl DumpData_XE_ for Data {
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        self.data.as_slice().dump_into(dst)
    }
    fn size(&self) -> usize {
        self.data.len()
    } 
}

#[make_endian]
pub trait DumpString_XE_ {
    fn string_size(&self) -> usize;
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()>;
}

#[make_endian]
impl DumpString_XE_ for ref_slice<'_, u16_XE_> {
    fn string_size(&self) -> usize {
        self.size_of_val()
    }
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()> {
        self.dump_into(dst)
    }
}

#[make_endian]
impl DumpString_XE_ for String {
    fn string_size(&self) -> usize {
        self.encode_utf16().count() * u16_XE_::size_of()
    }
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()> {
        let val = self.encode_utf16().collect::<Vec<_>>();
        let string = u16_XE_::mut_slice_from_data(dst, val.len()).context("string")?;
        for (src, dst) in val.into_iter().zip(string) {
            *dst = src.into();
        }
        Ok(())
    }
}
