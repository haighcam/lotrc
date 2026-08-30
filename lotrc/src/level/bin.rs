use anyhow::{Context, Result};
use lotrc_proc::{derive_pod};
use rayon::prelude::*;
use indexmap::IndexMap;
use log::debug;
use tracing::info;

use crate::{
    level::{
        Version,
        pak::block1::infos::{DumpInfoData}
        
    },
    types::{
        update_crc, Crc, ReadData, AlignedBuf, CompressedDataRef, get_default_ref, align_offset, DumpSlice, ref_slice, BaseTypes, DumpStrings, StringsRef, DumpCompressedData
    }
};

#[cfg_attr(feature = "ffi", repr(C))]
pub struct BinData<'a> {
    pub data: AlignedBuf,
    pub model_data: IndexMap<u32, CompressedDataRef<'a>>,
    pub texture_data: IndexMap<u32, CompressedDataRef<'a>>
}

#[derive_pod]
pub struct BinHeader<T: BaseTypes> {
    pub constx06: T::u32,
    pub version: T::u32,
    pub strings_offset: T::u32,
    pub strings_size: T::u32,
    pub strings_num: T::u32,
    pub asset_handle_num: T::u32,
    pub asset_handle_offset: T::u32,
    pub unk_7: T::u32,
    pub vdata_num: T::u32,
    pub vdata_num_alt: T::u32,
    pub texdata_num: T::u32,
    pub unk_11: T::u32,
    pub unk_12: T::u32,
    pub unk_13: T::u32,
    pub unk_14: T::u32,
    pub unk_15: T::u32,
    pub unk_16: T::u32,
    pub unk_17: T::u32,
    pub unk_18: T::u32,
    pub unk_19: T::u32,
    pub unk_20: T::u32,
    pub unk_21: T::u32,
    pub unk_22: T::u32,
    pub unk_23: T::u32,
    pub unk_24: T::u32,
    pub unk_25: T::u32,
    pub unk_26: T::u32,
    pub unk_27: T::u32,
    pub unk_28: T::u32,
    pub unk_29: T::u32,
    pub unk_30: T::u32,
    pub unk_31: T::u32,
    pub unk_32: T::u32,
    pub unk_33: T::u32,
    pub unk_34: T::u32,
    pub unk_35: T::u32,
    pub unk_36: T::u32,
    pub unk_37: T::u32,
    pub unk_38: T::u32,
    pub unk_39: T::u32,
    pub unk_40: T::u32,
    pub unk_41: T::u32,
    pub unk_42: T::u32,
}

#[derive_pod]
pub struct AssetHandle<T: BaseTypes> {
    pub key: Crc<T>,
    pub offset: T::u32,
    pub size: T::u32,
    pub size_comp: T::u32,
    pub kind: T::u32,
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


#[cfg_attr(feature = "ffi", repr(C))]
pub struct BinRef<'a,  T: BaseTypes> {
    pub header: &'a BinHeader<T>,
    pub strings: crate::types::StringsRef<'a>,
    pub asset_handles: ref_slice<'a, AssetHandle<T>>,
    pub model_data: IndexMap<u32, &'a CompressedDataRef<'a>>,
    pub texture_data: IndexMap<u32, &'a CompressedDataRef<'a>>,
}

impl<'a, T: BaseTypes> Default for BinRef<'a, T> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            strings: Default::default(),
            asset_handles: Default::default(),
            model_data: Default::default(),
            texture_data: Default::default(),
        }
    }
}

impl<'a,  T: BaseTypes> BinRef<'a, T> {
    pub fn from_data<'b: 'a, 'c: 'b>(src: &'c [u8], data: &'a mut BinCompressedData<'b>) -> Result<Self> {
        let t = std::time::Instant::now();
        let header = BinHeader::<T>::from_data(src).context("header")?;
        let strings = crate::types::StringsRef::from_data::<T>(
            &src[header.strings_offset.into() as usize..],
            header.strings_num.into() as usize,
        )
        .context("strings")?;
        update_crc(strings.strings());
        let asset_handles = AssetHandle::<T>::slice_from_data(
            &src[header.asset_handle_offset.into() as usize..],
            header.asset_handle_num.into() as usize,
        )
        .context("asset_handles")?;
        debug!("Bin headers in {}", t.elapsed().as_secs_f32());
        
        let t = std::time::Instant::now();
        let split = header.vdata_num.into() as usize;
        for asset_handle in asset_handles {
            info!(target: "_file", "{} offset {}/{}, size {}", asset_handle.key.val.into(), asset_handle.offset.into(), src.len(), asset_handle.size_comp.into());
            debug!("{} offset {}/{}, size {}", asset_handle.key.val.into(), asset_handle.offset.into(), src.len(), asset_handle.size_comp.into());
        }
        data.model_data = asset_handles.iter().take(split).map(|info| Ok((
            info.key.val.into(),
            CompressedDataRef::from_data(&src[info.offset.into() as usize..], info.size_comp.into() as usize, info.size.into() as usize).with_context(|| format!("model data {}", info.key.val.into()))?
        ))).collect::<Result<IndexMap<_, _>, anyhow::Error>>()?;
        data.texture_data = asset_handles.iter().skip(split).map(|info| Ok((
            info.key.val.into(),
            CompressedDataRef::from_data(&src[info.offset.into() as usize..], info.size_comp.into() as usize, info.size.into() as usize).with_context(|| format!("texture data {}", info.key.val.into()))?
        ))).collect::<Result<IndexMap<_, _>, anyhow::Error>>()?;

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
    pub fn model_handles(&self) -> &[AssetHandle<T>] {
        &self.asset_handles[..self.header.vdata_num.into() as usize]
    }
    pub fn texture_handles(&self) -> &[AssetHandle<T>] {
        &self.asset_handles[self.header.vdata_num.into() as usize..]
    }
}

impl<'a> BinData<'a> {
    pub fn parse<T: BaseTypes>(&'a mut self) -> Result<BinRef<'a, T>> {
        let t = std::time::Instant::now();
        let header = BinHeader::<T>::from_data(&self.data[..]).context("header")?;
        let strings = StringsRef::from_data::<T>(
            &self.data[header.strings_offset.into() as usize..],
            header.strings_num.into() as usize,
        )
        .context("strings")?;
        update_crc(strings.strings());
        let asset_handles = AssetHandle::<T>::slice_from_data(
            &self.data[header.asset_handle_offset.into() as usize..],
            header.asset_handle_num.into() as usize,
        )
        .context("asset_handles")?;
        debug!("Bin headers in {}", t.elapsed().as_secs_f32());
        
        let t = std::time::Instant::now();
        let mut raw_data = asset_handles.iter().map(|info| (
            info.key.val.into(),
            CompressedDataRef::from_data(&self.data[info.offset.into() as usize..], info.size.into() as usize, info.size_comp.into() as usize).with_context(|| format!("bin data {}", info.key.val.into()))
        ));
        let split = header.vdata_num.into() as usize;
        let mut model_data = IndexMap::with_capacity(split);
        let mut texture_data = IndexMap::with_capacity(asset_handles.len() - split);
        for _ in 0..split {
            let (k, val) = raw_data.next().unwrap();
            model_data.insert(k, val?);
        }
        for _ in split..asset_handles.len() {
            let (k, val) = raw_data.next().unwrap();
            texture_data.insert(k, val?);
        }
        self.model_data = model_data;
        self.texture_data = texture_data;
        debug!("Bin raw data in {}", t.elapsed().as_secs_f32());

        Ok(BinRef {
            header,
            asset_handles: asset_handles.into(),
            strings,
            model_data: self.model_data.iter().map(|(k,v)| (*k, v)).collect::<IndexMap<_, _>>().into(),
            texture_data: self.texture_data.iter().map(|(k,v)| (*k, v)).collect::<IndexMap<_, _>>().into(),
        })
    }
    pub fn parse_data(&mut self) -> Result<()> {
        self.model_data.par_values_mut().chain(self.texture_data.par_values_mut()).try_for_each(|x| x.decompress())
    }
}

pub trait DumpBin<T: BaseTypes> {
    fn strings(&self) -> &impl DumpStrings<T>;
    fn dump(&self, dst: &mut DumpSlice, model_data: &[DumpInfoData<T>], texture_data: &IndexMap<u32, DumpInfoData<T>>, rad_data: Option<&DumpInfoData<T>>, version: Version) -> Result<()> {
        let header = BinHeader::<T>::mut_from_data(dst).context("header")?;
        dst.align(2048)?;
        header.constx06 = 0x6u32.into();

        header.version = match version {
            Version::Pc => 1u32,
            Version::Xbox => 2,
            Version::Ps3 => 3,
            _ => 0
        }.into();

        let mut model_handles = Vec::with_capacity(model_data.len() + 1);
        for val in model_data {
            if !val.data.is_some() { continue; }
            let size = val.data.size();
            if size == 0 { continue; }
            let offset = (dst.offset as u32).into();
            val.data.dump_into(dst).with_context(|| format!("dump model asset {}", val.key.into()))?;
            dst.align(2048)?;
            model_handles.push(AssetHandle::<T> {
                key: Crc::new(val.key),
                offset,
                size: (size as u32).into(),
                size_comp: (val.data.size_comp() as u32).into(),
                kind: val.kind.clone() 
            });
        }
        let mut texture_handles = Vec::with_capacity(texture_data.len());
        for val in texture_data.values() {
            let offset = (dst.offset as u32).into();
            let (size, size_comp) = if val.data.is_some() {
                val.data.dump_into(dst).with_context(|| format!("dump texture asset {}", val.key.into()))?;
                dst.align(2048)?;
                if val.data.size() == 0 {
                    (0u32.into(), 0u32.into())
                } else {
                    ((val.data.size() as u32).into(), (val.data.size_comp() as u32).into())
                }
            } else {
                (0u32.into(), 0u32.into())
            };
            texture_handles.push(AssetHandle::<T> {
                key: Crc::new(val.key),
                offset,
                size,
                size_comp,
                kind: val.kind.clone() 
            })
        }
        if let Some(val) = rad_data {
            let offset = (dst.offset as u32).into();
            val.data.dump_into(dst).with_context(|| format!("dump radiosity asset {}", val.key.into()))?;
            dst.align(2048)?;
            let (size, size_comp) = if val.data.size() == 0 {
                (0u32.into(), 0u32.into())
            } else {
                ((val.data.size() as u32).into(), (val.data.size_comp() as u32).into())
            };
            model_handles.push(AssetHandle {
                key: Crc::new(val.key),
                offset,
                size,
                size_comp,
                kind: val.kind.clone() 
            })
        }
 
        header.vdata_num = (model_handles.len() as u32).into();
        header.vdata_num_alt = header.vdata_num;
        header.texdata_num = (texture_data.len() as u32).into();
        header.asset_handle_offset = (dst.offset as u32).into();
        let asset_handles = AssetHandle::<T>::mut_slice_from_data(dst, model_handles.len() + texture_handles.len()).context("asset_handles")?;
        header.asset_handle_num = (asset_handles.len() as u32).into();
        model_handles.sort_by_key(|x| x.key.val.into());
        texture_handles.sort_by_key(|x| x.key.val.into());
        (&mut asset_handles[..model_handles.len()]).copy_from_slice(&model_handles);
        (&mut asset_handles[model_handles.len()..]).copy_from_slice(&texture_handles);

        let strings = self.strings();
        let off = dst.offset;
        header.strings_offset = (off as u32).into();
        header.strings_num = (strings.num_strings() as u32).into();
        strings.dump_into(dst).context("strings")?;
        header.strings_size = ((dst.offset - off) as u32).into();

        dst.align(2048)?;

        Ok(())
    }
    fn size(&self, model_data: &[DumpInfoData<T>], texture_data: &IndexMap<u32, DumpInfoData<T>>, rad_data: Option<&DumpInfoData<T>>) -> usize {
        let mut size = align_offset(std::mem::size_of::<BinHeader<T>>(), 2048);
        for val in model_data {
            if val.data.is_some() {
                size = align_offset(size + val.data.size_comp(), 2048);
            }
        }
        for val in texture_data.values() {
            if val.data.is_some() {
                size = align_offset(size + val.data.size_comp(), 2048);
            }
        }
        if let Some(val) = rad_data {
            size = align_offset(size + val.data.size_comp(), 2048);
        }

        size += std::mem::size_of::<AssetHandle<T>>() * (model_data.len() + texture_data.len() + if rad_data.is_some() { 1 } else { 0 });

        align_offset(size + self.strings().size(), 2048)
    }
}

impl<T: BaseTypes> DumpBin<T> for BinRef<'_, T> {
    fn strings(&self) -> &impl DumpStrings<T> {
        &self.strings
    }
}
