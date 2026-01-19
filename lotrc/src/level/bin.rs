use anyhow::{Context, Result};
use lotrc_proc::{make_platforms, OrderedData};
use rayon::prelude::*;
use std::ptr::NonNull;
use indexmap::IndexMap;

#[cfg(not(feature = "ffi"))]
use crate::types::GetNative;
use crate::types::{self, update_crc, Crc, RefFromData, CompressedDataRef, OrderedDataStrict, OrderedData, BufType, AlignedBuf, CompressedDataRefAlt, Map, MapImpl, slice, get_default_ref};
#[make_platforms]
use crate::types::{u32VER, CrcVER, StringsRefVER};

pub struct BinData<'a> {
    pub data: AlignedBuf,
    pub model_data: Map<u32, CompressedDataRefAlt<'a>>,
    pub texture_data: Map<u32, CompressedDataRefAlt<'a>>
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct BinHeader {
    pub constx06: u32,
    pub version: u32,
    pub strings_offset: u32,
    pub strings_size: u32,
    pub strings_num: u32,
    pub asset_handle_num: u32,
    pub asset_handle_offset: u32,
    pub unk_7: u32,
    pub vdata_num: u32,
    pub vdata_num_alt: u32,
    pub texdata_num: u32,
    pub unk_11: u32,
    pub unk_12: u32,
    pub unk_13: u32,
    pub unk_14: u32,
    pub unk_15: u32,
    pub unk_16: u32,
    pub unk_17: u32,
    pub unk_18: u32,
    pub unk_19: u32,
    pub unk_20: u32,
    pub unk_21: u32,
    pub unk_22: u32,
    pub unk_23: u32,
    pub unk_24: u32,
    pub unk_25: u32,
    pub unk_26: u32,
    pub unk_27: u32,
    pub unk_28: u32,
    pub unk_29: u32,
    pub unk_30: u32,
    pub unk_31: u32,
    pub unk_32: u32,
    pub unk_33: u32,
    pub unk_34: u32,
    pub unk_35: u32,
    pub unk_36: u32,
    pub unk_37: u32,
    pub unk_38: u32,
    pub unk_39: u32,
    pub unk_40: u32,
    pub unk_41: u32,
    pub unk_42: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct AssetHandle {
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
    pub size_comp: u32,
    pub kind: u32,
}

pub struct BinCompressedData<'a> {
    pub model_data: Map<u32, CompressedDataRefAlt<'a>>,
    pub texture_data: Map<u32, CompressedDataRefAlt<'a>>
}

impl Default for BinCompressedData<'_> {
    fn default() -> Self {
        Self {
            model_data: MapImpl::default().into(),
            texture_data: MapImpl::default().into()
        }
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct BinRefVER<'a> {
    pub header: &'a BinHeaderVER,
    pub strings: StringsRefVER<'a>,
    pub asset_handles: slice<'a, AssetHandleVER>,
    pub model_data: Map<u32, &'a CompressedDataRefAlt<'a>>,
    pub texture_data: Map<u32, &'a CompressedDataRefAlt<'a>>
}

#[make_platforms]
impl Default for BinRefVER<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            strings: StringsRefVER::default(),
            asset_handles: slice::default(),
            model_data: MapImpl::default().into(),
            texture_data: MapImpl::default().into(),
        }
    }
}

#[make_platforms]
impl<'a> BinRefVER<'a> {
    pub fn from_data<'b: 'a, 'c: 'b>(src: &'c [u8], data: &'a mut BinCompressedData<'b>) -> Result<Self> {
        let t = std::time::Instant::now();
        let header = BinHeaderVER::from_data(src).context("header")?;
        let strings = StringsRefVER::from_data(
            &src[header.strings_offset.get() as usize..],
            header.strings_num.get() as usize,
        )
        .context("strings")?;
        update_crc(strings.strings());
        let asset_handles = AssetHandleVER::slice_from_data(
            &src[header.asset_handle_offset.get() as usize..],
            header.asset_handle_num.get() as usize,
        )
        .context("asset_handles")?;
        println!("Bin headers in {}", t.elapsed().as_secs_f32());
        
        let t = std::time::Instant::now();
        let split = header.vdata_num.get() as usize;
        data.model_data = asset_handles.iter().take(split).map(|info| (
            info.key.get(),
            CompressedDataRefAlt::from_data(&src[info.offset.get() as usize..], info.size_comp.get() as usize, info.size.get() as usize)
        )).collect::<MapImpl<_, _>>().into();
        data.texture_data = asset_handles.iter().skip(split).map(|info| (
            info.key.get(),
            CompressedDataRefAlt::from_data(&src[info.offset.get() as usize..], info.size_comp.get() as usize, info.size.get() as usize)
        )).collect::<MapImpl<_, _>>().into();

        data.model_data.par_values_mut()
            .chain(data.texture_data.par_values_mut())
            .try_for_each(|data| data.decompress())
            .context("compressed data")?;
        println!("Bin compressed data parsed in {}", t.elapsed().as_secs_f32());
        Ok(Self {
            header,
            asset_handles: asset_handles.into(),
            strings,
            model_data: data.model_data.iter().map(|(k,v)| (*k, v)).collect::<MapImpl<_, _>>().into(),
            texture_data: data.texture_data.iter().map(|(k,v)| (*k, v)).collect::<MapImpl<_, _>>().into(),
        })
    }
}

impl<'a> BinData<'a> {
    #[make_platforms]
    pub fn parse_ver(&'a mut self) -> Result<BinRefVER<'a>> {
        let t = std::time::Instant::now();
        let header = BinHeaderVER::from_data(&self.data[..]).context("header")?;
        let strings = StringsRefVER::from_data(
            &self.data[header.strings_offset.get() as usize..],
            header.strings_num.get() as usize,
        )
        .context("strings")?;
        update_crc(strings.strings());
        let asset_handles = AssetHandleVER::slice_from_data(
            &self.data[header.asset_handle_offset.get() as usize..],
            header.asset_handle_num.get() as usize,
        )
        .context("asset_handles")?;
        println!("Bin headers in {}", t.elapsed().as_secs_f32());
        
        let mut raw_data = asset_handles.iter().map(|info| (
            info.key.get(),
            CompressedDataRefAlt::from_data(&self.data[info.offset.get() as usize..], info.size.get() as usize, info.size_comp.get() as usize)
        ));
        let split = header.vdata_num.get() as usize;
        let mut model_data = MapImpl::with_capacity(split);
        let mut texture_data = MapImpl::with_capacity(asset_handles.len() - split);
        for _ in 0..split {
            let (k, val) = raw_data.next().unwrap();
            model_data.insert(k, val);
        }
        for _ in split..asset_handles.len() {
            let (k, val) = raw_data.next().unwrap();
            texture_data.insert(k, val);
        }
        self.model_data = model_data.into();
        self.texture_data = texture_data.into();

        Ok(BinRefVER {
            header,
            asset_handles: asset_handles.into(),
            strings,
            model_data: self.model_data.iter().map(|(k,v)| (*k, v)).collect::<MapImpl<_, _>>().into(),
            texture_data: self.texture_data.iter().map(|(k,v)| (*k, v)).collect::<MapImpl<_, _>>().into(),
        })
    }
    pub fn parse(&mut self) -> Result<()> {
        self.model_data.par_values_mut().chain(self.texture_data.par_values_mut()).try_for_each(|x| x.decompress())
    }
}

#[make_platforms]
impl<'a> BinRefVER<'a> {
    pub fn model_handles(&self) -> &[AssetHandleVER] {
        &self.asset_handles[..self.header.vdata_num.get() as usize]
    }
    pub fn texture_handles(&self) -> &[AssetHandleVER] {
        &self.asset_handles[self.header.vdata_num.get() as usize..]
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct BinVER {
    _data: BufType,
    header: NonNull<BinHeaderVER>,
    strings: types::StringsVER,
    asset_handles: NonNull<[AssetHandleVER]>,
    model_data: IndexMap<u32, CompressedDataRef>,
    texture_data: IndexMap<u32, CompressedDataRef>,
}

#[make_platforms]
unsafe impl Sync for BinVER {}
#[make_platforms]
unsafe impl Send for BinVER {}

#[make_platforms]
impl BinVER {
    pub fn from_bytes(data: BufType) -> Result<Self> {
        let t = std::time::Instant::now();
        let header = BinHeaderVER::from_data(&data).context("header")?;
        let strings = types::StringsVER::from_bytes(
            &data,
            header.strings_offset.get() as usize,
            header.strings_num.get() as usize,
        )
        .context("strings")?;
        update_crc(strings.strings());
        let asset_handles = AssetHandleVER::slice_from_data(
            &data[header.asset_handle_offset.get() as usize..],
            header.asset_handle_num.get() as usize,
        )
        .context("asset_handles")?;
        println!("Bin headers in {}", t.elapsed().as_secs_f32());
        
        let mut raw_data = asset_handles.iter().map(|info| (
            info.key.get(),
            CompressedDataRef::from_bytes(&data, info.offset.get() as usize, info.size.get() as usize, info.size_comp.get() as usize)
        ));
        let split = header.vdata_num.get() as usize;
        let mut model_data = IndexMap::with_capacity(split);
        let mut texture_data = IndexMap::with_capacity(asset_handles.len() - split);
        for _ in 0..split {
            let (k, val) = raw_data.next().unwrap();
            model_data.insert(k, val);
        }
        for _ in split..asset_handles.len() {
            let (k, val) = raw_data.next().unwrap();
            texture_data.insert(k, val);
        }

        let header = NonNull::from_ref(header);
        let asset_handles = NonNull::from_ref(asset_handles);
        Ok(Self {
            _data: data,
            header,
            asset_handles,
            strings,
            model_data,
            texture_data,
        })
    }
}

#[make_platforms]
impl BinVER {
    pub fn parse(&mut self) -> Result<()> {
        self.model_data.par_values().chain(self.texture_data.par_values()).try_for_each(|d| {
            d.get()?;
            Ok::<_,anyhow::Error>(())
        })?;
        Ok(())
    }
    pub fn header(&self) -> &BinHeaderVER {
        unsafe { self.header.as_ref() }
    }
    pub fn strings(&self) -> &types::StringsVER {
        &self.strings
    }
    pub fn asset_handles(&self) -> &[AssetHandleVER] {
        unsafe { self.asset_handles.as_ref() }
    }
    pub fn model_handles(&self) -> &[AssetHandleVER] {
        &self.asset_handles()[..self.header().vdata_num.get() as usize]
    }
    pub fn texture_handles(&self) -> &[AssetHandleVER] {
        &self.asset_handles()[self.header().vdata_num.get() as usize..]
    }
}

#[make_platforms]
impl BinVER {
    pub fn model_data(&self) -> &IndexMap<u32, CompressedDataRef> {
        &self.model_data
    }
    pub fn texture_data(&self) -> &IndexMap<u32, CompressedDataRef> {
        &self.texture_data
    }
}
