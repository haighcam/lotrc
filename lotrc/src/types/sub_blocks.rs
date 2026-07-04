use crate::types::GetNative;
use crate::types::{get_default_ref, Crc, DumpData, DumpSlice, RefFromData, OrderedData, OrderedDataStrict, ref_slice, align_offset};
use crate::level::pak::block2::PFields;
#[make_platforms]
use crate::{
    types::{CrcVER, u16VER, u32VER},
    level::pak::block2::{PFieldsRefVER, PFieldsVER}
};
use anyhow::{Context, Result};
use lotrc_proc::{make_platforms};
use enum_dispatch::enum_dispatch;

#[derive(Debug, Default, Clone, OrderedData)]
pub struct SubBlocksHeader {
    pub z0: u32,
    pub block_num: u32,
    pub z2: u32,
    pub z3: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct SubBlocksBlockHeader {
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct SubBlocksInfoRefVER<'a> {
    pub header: &'a SubBlocksHeaderVER,
    pub block_headers: ref_slice<'a, SubBlocksBlockHeaderVER>,
}

#[make_platforms]
impl Default for SubBlocksInfoRefVER<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            block_headers: ref_slice::default(),
        }
    }
}

#[make_platforms]
impl<'a> SubBlocksInfoRefVER<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = SubBlocksHeaderVER::from_data(&src[offset..]).context("header")?;
        offset += header.size();
        let block_headers = SubBlocksBlockHeaderVER::slice_from_data(
            &src[offset..],
            header.block_num.get() as usize,
        )
        .context("block_headers")?;

        Ok(Self {
            header: header,
            block_headers: block_headers.into(),
        })
    }

    pub fn size(num: usize) -> usize {
        align_offset(
            std::mem::size_of::<SubBlocksBlockHeaderVER>() +
            (std::mem::size_of::<SubBlocksBlockHeaderVER>() * num),
            16
        )
    }

    pub fn dump<'d>(dst: &mut DumpSlice<'d>, num: usize) -> Result<&'d mut [SubBlocksBlockHeaderVER]> {
        let header = SubBlocksHeaderVER::mut_from_data(dst).context("header")?;
        let block_headers = SubBlocksBlockHeaderVER::mut_slice_from_data(dst, num).context("block_headers")?;
        header.block_num = block_headers.len().conv();
        dst.align(16)?; 
        Ok(block_headers)
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct DataRefVER<'a> {
    pub data: ref_slice<'a, u8>
}

#[make_platforms]
impl<'a> DataRefVER<'a> {
    pub fn from_data(src: &'a [u8]) -> Self {
        Self { data: src.into() }
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    pub data: Vec<u8>,
}

#[make_platforms]
impl From<&DataRefVER<'_>> for Data {
    fn from(val: &DataRefVER) -> Self {
        Self {
            data: val.data.to_vec(),
        }
    }
}

#[make_platforms]
#[enum_dispatch(DumpDataVER)]
pub enum DataVER<'a> {
    Ref(DataRefVER<'a>),
    Owned(Data)
}

#[make_platforms]
#[enum_dispatch]
pub trait DumpDataVER {
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
    fn size(&self) -> usize; 
}

#[make_platforms]
impl DumpDataVER for DataRefVER<'_> {
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        self.data.dump_into(dst)
    }
    fn size(&self) -> usize {
        self.data.len()
    }  
}

#[make_platforms]
impl DumpDataVER for Data {
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        self.data.as_slice().dump_into(dst)
    }
    fn size(&self) -> usize {
        self.data.len()
    } 
}

#[make_platforms]
pub trait DumpStringVER {
    fn string_size(&self) -> usize;
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()>;
}

#[make_platforms]
impl DumpStringVER for ref_slice<'_, u16VER> {
    fn string_size(&self) -> usize {
        self.size()
    }
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()> {
        self.dump_into(dst)
    }
}

#[make_platforms]
impl DumpStringVER for String {
    fn string_size(&self) -> usize {
        self.encode_utf16().count() * u16VER::size_of()
    }
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()> {
        let val = self.encode_utf16().collect::<Vec<_>>();
        let string = u16VER::mut_slice_from_data(dst, val.len()).context("string")?;
        for (src, dst) in val.into_iter().zip(string) {
            *dst = src.into();
        }
        Ok(())
    }
}
