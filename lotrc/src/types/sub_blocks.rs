use anyhow::{Context, Result};
use enum_dispatch::enum_dispatch;
use lotrc_proc::{make_endian, derive_ordered_data};
#[make_endian]
use crate::{
    types::{Crc_XE_, u16_XE_, u32_XE_},
    level::pak::block2::{PFieldsRef_XE_, PFields_XE_}
};

use crate::types::{get_default_ref, Crc, DumpData, DumpSlice, RefFromData, OrderedData, ref_slice, align_offset};
use crate::level::pak::block2::PFields;

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
        offset += header.size();
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
        self.size()
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
