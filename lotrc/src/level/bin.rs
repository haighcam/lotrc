use anyhow::{Context, Result};
use lotrc_proc::{make_platforms, OrderedData};
use rayon::prelude::*;
use indexmap::IndexMap;
use log::debug;

use crate::types::GetNative;
use crate::types::{update_crc, Crc, RefFromData, OrderedDataStrict, OrderedData, AlignedBuf, CompressedDataRef, get_default_ref, DumpCompressedData, align_offset, DumpData, DumpSlice, ref_slice};
#[make_platforms]
use crate::{
    types::{u32VER, CrcVER, StringsRefVER, DumpStringsVER},
    level::pak::block1::infos::DumpInfoDataVER
};

#[cfg_attr(feature = "ffi", repr(C))]
pub struct BinData<'a> {
    pub data: AlignedBuf,
    pub model_data: IndexMap<u32, CompressedDataRef<'a>>,
    pub texture_data: IndexMap<u32, CompressedDataRef<'a>>
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

#[cfg_attr(feature = "ffi", repr(C))]
pub struct BinCompressedData<'a> {
    pub model_data: IndexMap<u32, CompressedDataRef<'a>>,
    pub texture_data: IndexMap<u32, CompressedDataRef<'a>>
}

impl Default for BinCompressedData<'_> {
    fn default() -> Self {
        Self {
            model_data: IndexMap::default().into(),
            texture_data: IndexMap::default().into()
        }
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct BinRefVER<'a> {
    pub header: &'a BinHeaderVER,
    pub strings: StringsRefVER<'a>,
    pub asset_handles: ref_slice<'a, AssetHandleVER>,
    pub model_data: IndexMap<u32, &'a CompressedDataRef<'a>>,
    pub texture_data: IndexMap<u32, &'a CompressedDataRef<'a>>,
}

#[make_platforms]
impl Default for BinRefVER<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            strings: StringsRefVER::default(),
            asset_handles: ref_slice::default(),
            model_data: IndexMap::default().into(),
            texture_data: IndexMap::default().into(),
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
        debug!("Bin headers in {}", t.elapsed().as_secs_f32());
        
        let t = std::time::Instant::now();
        let split = header.vdata_num.get() as usize;
        data.model_data = asset_handles.iter().take(split).map(|info| (
            info.key.get(),
            CompressedDataRef::from_data(&src[info.offset.get() as usize..], info.size_comp.get() as usize, info.size.get() as usize)
        )).collect::<IndexMap<_, _>>().into();
        data.texture_data = asset_handles.iter().skip(split).map(|info| (
            info.key.get(),
            CompressedDataRef::from_data(&src[info.offset.get() as usize..], info.size_comp.get() as usize, info.size.get() as usize)
        )).collect::<IndexMap<_, _>>().into();

        data.model_data.par_values_mut()
            .chain(data.texture_data.par_values_mut())
            .try_for_each(|data| data.decompress())
            .context("compressed data")?;
        debug!("Bin compressed data parsed in {}", t.elapsed().as_secs_f32());
        Ok(Self {
            header,
            asset_handles: asset_handles.into(),
            strings,
            model_data: data.model_data.iter().map(|(k,v)| (*k, v)).collect::<IndexMap<_, _>>().into(),
            texture_data: data.texture_data.iter().map(|(k,v)| (*k, v)).collect::<IndexMap<_, _>>().into(),
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
        debug!("Bin headers in {}", t.elapsed().as_secs_f32());
        
        let t = std::time::Instant::now();
        let mut raw_data = asset_handles.iter().map(|info| (
            info.key.get(),
            CompressedDataRef::from_data(&self.data[info.offset.get() as usize..], info.size.get() as usize, info.size_comp.get() as usize)
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
        self.model_data = model_data.into();
        self.texture_data = texture_data.into();
        debug!("Bin raw data in {}", t.elapsed().as_secs_f32());

        Ok(BinRefVER {
            header,
            asset_handles: asset_handles.into(),
            strings,
            model_data: self.model_data.iter().map(|(k,v)| (*k, v)).collect::<IndexMap<_, _>>().into(),
            texture_data: self.texture_data.iter().map(|(k,v)| (*k, v)).collect::<IndexMap<_, _>>().into(),
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
pub trait DumpBinVER {
    fn strings(&self) -> &impl DumpStringsVER;
    fn dump(&self, dst: &mut DumpSlice, model_data: &[DumpInfoDataVER], texture_data: &[DumpInfoDataVER], rad_data: Option<&DumpInfoDataVER>) -> Result<()> {
        let header = BinHeaderVER::mut_from_data(dst).context("header")?;
        dst.align(2048)?;
        header.constx06 = 0x6u32.conv();
        if IS_PC {
            header.version = 1u32.conv();
        }
        if IS_XBOX {
            header.version = 2u32.conv();
        }
        if IS_PS3 {
            header.version = 3u32.conv();
        }

        let mut model_handles = Vec::with_capacity(model_data.len() + 1);
        for val in model_data {
            if !val.data.is_some() { continue; }
            let size = val.data.size();
            if size == 0 { continue; }
            let offset = dst.offset.conv();
            val.data.dump_into(dst).with_context(|| format!("dump model asset {}", val.key))?;
            dst.align(2048)?;
            model_handles.push(AssetHandleVER {
                key: val.key.clone(),
                offset,
                size: size.conv(),
                size_comp: val.data.size_comp().conv(),
                kind: val.kind.clone() 
            });
        }
        let mut texture_handles = Vec::with_capacity(texture_data.len());
        for val in texture_data {
            let offset = dst.offset.conv();
            let (size, size_comp) = if val.data.is_some() {
                val.data.dump_into(dst).with_context(|| format!("dump texture asset {}", val.key))?;
                dst.align(2048)?;
                if (val.data.size() == 0) {
                    (0u32.conv(), 0u32.conv())
                } else {
                    (val.data.size().conv(), val.data.size_comp().conv())
                }
            } else {
                (0u32.conv(), 0u32.conv())
            };
            texture_handles.push(AssetHandleVER {
                key: val.key.clone(),
                offset,
                size,
                size_comp,
                kind: val.kind.clone() 
            })
        }
        if let Some(val) = rad_data {
            let offset = dst.offset.conv();
            val.data.dump_into(dst).with_context(|| format!("dump radiosity asset {}", val.key))?;
            dst.align(2048)?;
            let (size, size_comp) = if (val.data.size() == 0) {
                (0u32.conv(), 0u32.conv())
            } else {
                (val.data.size().conv(), val.data.size_comp().conv())
            };
            model_handles.push(AssetHandleVER {
                key: val.key.clone(),
                offset,
                size,
                size_comp,
                kind: val.kind.clone() 
            })
        }
 
        header.vdata_num = model_handles.len().conv();
        header.vdata_num_alt = header.vdata_num;
        header.texdata_num = texture_data.len().conv();
        header.asset_handle_offset = dst.offset.conv();
        let asset_handles = AssetHandleVER::mut_slice_from_data(dst, model_handles.len() + texture_handles.len()).context("asset_handles")?;
        header.asset_handle_num = asset_handles.len().conv();
        model_handles.sort_by_key(|x| x.key.get());
        texture_handles.sort_by_key(|x| x.key.get());
        (&mut asset_handles[..model_handles.len()]).write_from(&model_handles).context("model asset handles")?;
        (&mut asset_handles[model_handles.len()..]).write_from(&texture_handles).context("texture asset handles")?;

        let strings = self.strings();
        let off = dst.offset;
        header.strings_offset = off.conv();
        header.strings_num = strings.num_strings().conv();
        strings.dump_into(dst).context("strings")?;
        header.strings_size = (dst.offset - off).conv();

        dst.align(2048)?;

        Ok(())
    }
    fn size(&self, model_data: &[DumpInfoDataVER], texture_data: &[DumpInfoDataVER], rad_data: Option<&DumpInfoDataVER>) -> usize {
        let mut size = align_offset(std::mem::size_of::<BinHeaderVER>(), 2048);
        for val in model_data {
            if val.data.is_some() {
                size = align_offset(size + val.data.size_comp(), 2048);
            }
        }
        for val in texture_data {
            if val.data.is_some() {
                size = align_offset(size + val.data.size_comp(), 2048);
            }
        }
        if let Some(val) = rad_data {
            size = align_offset(size + val.data.size_comp(), 2048);
        }

        size += std::mem::size_of::<AssetHandleVER>() * (model_data.len() + texture_data.len() + if rad_data.is_some() { 1 } else { 0 });

        align_offset(size + self.strings().size(), 2048)
    }
}

#[make_platforms]
impl DumpBinVER for BinRefVER<'_> {
    fn strings(&self) -> &impl DumpStringsVER {
        &self.strings
    }
}
