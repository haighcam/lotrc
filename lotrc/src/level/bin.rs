use anyhow::{Context, Result};
use lotrc_proc::{make_endian, derive_ordered_data};
use rayon::prelude::*;
use indexmap::IndexMap;
use log::debug;

use crate::{
    level::Version,
    types::{update_crc, Crc, RefFromData, OrderedData, AlignedBuf, CompressedDataRef, get_default_ref, DumpCompressedData, align_offset, DumpData, DumpSlice, ref_slice}
};
#[make_endian]
use crate::{
    types::{u32_XE_, Crc_XE_, StringsRef_XE_, DumpStrings_XE_},
    level::pak::block1::infos::DumpInfoData_XE_
};

#[cfg_attr(feature = "ffi", repr(C))]
pub struct BinData<'a> {
    pub data: AlignedBuf,
    pub model_data: IndexMap<u32, CompressedDataRef<'a>>,
    pub texture_data: IndexMap<u32, CompressedDataRef<'a>>
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BinHeader_XE_ {
    pub constx06: u32_XE_,
    pub version: u32_XE_,
    pub strings_offset: u32_XE_,
    pub strings_size: u32_XE_,
    pub strings_num: u32_XE_,
    pub asset_handle_num: u32_XE_,
    pub asset_handle_offset: u32_XE_,
    pub unk_7: u32_XE_,
    pub vdata_num: u32_XE_,
    pub vdata_num_alt: u32_XE_,
    pub texdata_num: u32_XE_,
    pub unk_11: u32_XE_,
    pub unk_12: u32_XE_,
    pub unk_13: u32_XE_,
    pub unk_14: u32_XE_,
    pub unk_15: u32_XE_,
    pub unk_16: u32_XE_,
    pub unk_17: u32_XE_,
    pub unk_18: u32_XE_,
    pub unk_19: u32_XE_,
    pub unk_20: u32_XE_,
    pub unk_21: u32_XE_,
    pub unk_22: u32_XE_,
    pub unk_23: u32_XE_,
    pub unk_24: u32_XE_,
    pub unk_25: u32_XE_,
    pub unk_26: u32_XE_,
    pub unk_27: u32_XE_,
    pub unk_28: u32_XE_,
    pub unk_29: u32_XE_,
    pub unk_30: u32_XE_,
    pub unk_31: u32_XE_,
    pub unk_32: u32_XE_,
    pub unk_33: u32_XE_,
    pub unk_34: u32_XE_,
    pub unk_35: u32_XE_,
    pub unk_36: u32_XE_,
    pub unk_37: u32_XE_,
    pub unk_38: u32_XE_,
    pub unk_39: u32_XE_,
    pub unk_40: u32_XE_,
    pub unk_41: u32_XE_,
    pub unk_42: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AssetHandle_XE_ {
    pub key: Crc_XE_,
    pub offset: u32_XE_,
    pub size: u32_XE_,
    pub size_comp: u32_XE_,
    pub kind: u32_XE_,
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

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct BinRef_XE_<'a> {
    pub header: &'a BinHeader_XE_,
    pub strings: StringsRef_XE_<'a>,
    pub asset_handles: ref_slice<'a, AssetHandle_XE_>,
    pub model_data: IndexMap<u32, &'a CompressedDataRef<'a>>,
    pub texture_data: IndexMap<u32, &'a CompressedDataRef<'a>>,
}

#[make_endian]
impl Default for BinRef_XE_<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            strings: StringsRef_XE_::default(),
            asset_handles: ref_slice::default(),
            model_data: IndexMap::default().into(),
            texture_data: IndexMap::default().into(),
        }
    }
}

#[make_endian]
impl<'a> BinRef_XE_<'a> {
    pub fn from_data<'b: 'a, 'c: 'b>(src: &'c [u8], data: &'a mut BinCompressedData<'b>) -> Result<Self> {
        let t = std::time::Instant::now();
        let header = BinHeader_XE_::from_data(src).context("header")?;
        let strings = StringsRef_XE_::from_data(
            &src[header.strings_offset.conv()..],
            header.strings_num.conv(),
        )
        .context("strings")?;
        update_crc(strings.strings());
        let asset_handles = AssetHandle_XE_::slice_from_data(
            &src[header.asset_handle_offset.conv()..],
            header.asset_handle_num.conv(),
        )
        .context("asset_handles")?;
        debug!("Bin headers in {}", t.elapsed().as_secs_f32());
        
        let t = std::time::Instant::now();
        let split = header.vdata_num.conv();
        data.model_data = asset_handles.iter().take(split).map(|info| (
            info.key.conv(),
            CompressedDataRef::from_data(&src[info.offset.conv()..], info.size_comp.conv(), info.size.conv())
        )).collect::<IndexMap<_, _>>().into();
        data.texture_data = asset_handles.iter().skip(split).map(|info| (
            info.key.conv(),
            CompressedDataRef::from_data(&src[info.offset.conv()..], info.size_comp.conv(), info.size.conv())
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
    #[make_endian]
    pub fn parse_xe_(&'a mut self) -> Result<BinRef_XE_<'a>> {
        let t = std::time::Instant::now();
        let header = BinHeader_XE_::from_data(&self.data[..]).context("header")?;
        let strings = StringsRef_XE_::from_data(
            &self.data[header.strings_offset.conv()..],
            header.strings_num.conv(),
        )
        .context("strings")?;
        update_crc(strings.strings());
        let asset_handles = AssetHandle_XE_::slice_from_data(
            &self.data[header.asset_handle_offset.conv()..],
            header.asset_handle_num.conv(),
        )
        .context("asset_handles")?;
        debug!("Bin headers in {}", t.elapsed().as_secs_f32());
        
        let t = std::time::Instant::now();
        let mut raw_data = asset_handles.iter().map(|info| (
            info.key.conv(),
            CompressedDataRef::from_data(&self.data[info.offset.conv()..], info.size.conv(), info.size_comp.conv())
        ));
        let split = header.vdata_num.conv();
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

        Ok(BinRef_XE_ {
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

#[make_endian]
impl<'a> BinRef_XE_<'a> {
    pub fn model_handles(&self) -> &[AssetHandle_XE_] {
        &self.asset_handles[..self.header.vdata_num.conv()]
    }
    pub fn texture_handles(&self) -> &[AssetHandle_XE_] {
        &self.asset_handles[self.header.vdata_num.conv()..]
    }
}

#[make_endian]
pub trait DumpBin_XE_ {
    fn strings(&self) -> &impl DumpStrings_XE_;
    fn dump(&self, dst: &mut DumpSlice, model_data: &[DumpInfoData_XE_], texture_data: &[DumpInfoData_XE_], rad_data: Option<&DumpInfoData_XE_>, version: Version) -> Result<()> {
        let header = BinHeader_XE_::mut_from_data(dst).context("header")?;
        dst.align(2048)?;
        header.constx06 = 0x6u32.conv();

        header.version = match version {
            Version::Pc => 1u32,
            Version::Xbox => 2,
            Version::Ps3 => 3,
            _ => 0
        }.conv();

        let mut model_handles = Vec::with_capacity(model_data.len() + 1);
        for val in model_data {
            if !val.data.is_some() { continue; }
            let size = val.data.size();
            if size == 0 { continue; }
            let offset = dst.offset.conv();
            val.data.dump_into(dst).with_context(|| format!("dump model asset {}", val.key))?;
            dst.align(2048)?;
            model_handles.push(AssetHandle_XE_ {
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
            texture_handles.push(AssetHandle_XE_ {
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
            model_handles.push(AssetHandle_XE_ {
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
        let asset_handles = AssetHandle_XE_::mut_slice_from_data(dst, model_handles.len() + texture_handles.len()).context("asset_handles")?;
        header.asset_handle_num = asset_handles.len().conv();
        model_handles.sort_by_key(|x| x.key.to_native());
        texture_handles.sort_by_key(|x| x.key.to_native());
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
    fn size(&self, model_data: &[DumpInfoData_XE_], texture_data: &[DumpInfoData_XE_], rad_data: Option<&DumpInfoData_XE_>) -> usize {
        let mut size = align_offset(std::mem::size_of::<BinHeader_XE_>(), 2048);
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

        size += std::mem::size_of::<AssetHandle_XE_>() * (model_data.len() + texture_data.len() + if rad_data.is_some() { 1 } else { 0 });

        align_offset(size + self.strings().size(), 2048)
    }
}

#[make_endian]
impl DumpBin_XE_ for BinRef_XE_<'_> {
    fn strings(&self) -> &impl DumpStrings_XE_ {
        &self.strings
    }
}
