#[cfg(feature = "python")]
use crate::pyobj_ref;
#[make_platforms]
use crate::types::CrcVER;
use crate::types::{self, decompress_block, hash_string, update_crc, Crc, RefFromData};
use anyhow::anyhow;
use anyhow::{Context, Result};
#[cfg(not(feature = "python"))]
use lotrc_proc::getter;
use lotrc_proc::{make_platforms, OrderedData};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::Arc;
use indexmap::IndexMap;

#[make_platforms]
use crate::level::{
    model::data::ModelDataVER, pak::PakVER, radiosity::RadiosityVER, texture::TextureVER,
};

#[cfg(feature = "python")]
pub fn init(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "bin")?;
    m.add_class::<AssetHandle>()?;
    m.add_class::<BinHeader>()?;
    init_pc(&m)?;
    init_xbox(&m)?;
    init_ps3(&m)?;
    Ok(m)
}
#[cfg(feature = "python")]
#[make_platforms]
pub fn init_ver(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BinVER>()
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "bin", get_all, set_all))]
#[repr(C)]
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
    pub vdata_num_: u32,
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
#[cfg_attr(feature = "python", pyclass(module = "bin", get_all, set_all))]
#[repr(C)]
pub struct AssetHandle {
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
    pub size_comp: u32,
    pub kind: u32,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "bin"))]
#[derive(Debug, Clone)]
pub struct BinVER {
    data: Arc<[u8]>,

    header: NonNull<BinHeaderVER>,
    strings: types::StringsVER,
    asset_handles: NonNull<[AssetHandleVER]>,
    model_data: Option<IndexMap<u32, Arc<[u8]>>>,
    texture_data: Option<IndexMap<u32, Arc<[u8]>>>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(BinVER);

#[make_platforms]
unsafe impl Sync for BinVER {}
#[make_platforms]
unsafe impl Send for BinVER {}

#[make_platforms]
impl BinVER {
    pub fn from_bytes(data: Arc<[u8]>) -> Result<Self> {
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

        let header = NonNull::from_ref(header);
        let asset_handles = NonNull::from_ref(asset_handles);
        Ok(Self {
            data,
            header,
            asset_handles,
            strings,
            model_data: None,
            texture_data: None,
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl BinVER {
    pub fn parse(&mut self) -> Result<()> {
        if self.model_data.is_some() && self.texture_data.is_some() {
            return Ok(());
        }

        let t = std::time::Instant::now();
        let mut raw_data = self
            .asset_handles()
            .par_iter()
            .map(|info| {
                Ok((
                    info.key.get().into(),
                    decompress_block(
                        &self.data[info.offset.get() as usize..],
                        info.size_comp.get() as usize,
                        info.size.get() as usize,
                    )?
                    .into(),
                ))
            })
            .collect::<Result<Vec<(u32, Arc<[u8]>)>>>()?.into_iter();
        let split = self.header().vdata_num.get() as usize;
        let mut model_data = IndexMap::with_capacity(split);
        let mut texture_data = IndexMap::with_capacity(self.asset_handles.len() - split);
        for _ in 0..split {
            let (k, val) = raw_data.next().unwrap();
            model_data.insert(k, val);
        }
        for _ in split..self.asset_handles.len() {
            let (k, val) = raw_data.next().unwrap();
            texture_data.insert(k, val);
        }
        println!("Bin data parsed in {}", t.elapsed().as_secs_f32());
        self.model_data.replace(model_data);
        self.texture_data.replace(texture_data);

        Ok(())
        /*
        let level = block1
            .sub_blocks()
            .get(&CrcVER::from(hash_string(b"level", None)))
            .ok_or(anyhow!("level block missing"))?
            .level()
            .ok_or(anyhow!("level block is wrong type"))?;
        let field = level
            .objs()
            .iter()
            .find(|(_, v)| v.header().key == CrcVER::from(hash_string(b"templateLevel", None)))
            .ok_or(anyhow!("templateLevel not found"))?
            .1
            .get(&CrcVER::from(hash_string(b"name", None)))
            .ok_or(anyhow!("templateLevel missing name field"))?;
        let name = field
            .crc()
            .ok_or(anyhow!("templateObject name field is not a crc"))?;
        */


    }
    #[getter]
    pub fn header(&self) -> &BinHeaderVER {
        unsafe { self.header.as_ref() }
    }
    #[getter]
    pub fn strings(&self) -> &types::StringsVER {
        &self.strings
    }
    #[getter]
    pub fn asset_handles(&self) -> &[AssetHandleVER] {
        unsafe { self.asset_handles.as_ref() }
    }
    #[getter]
    pub fn model_handles(&self) -> &[AssetHandleVER] {
        &self.asset_handles()[..self.header().vdata_num.get() as usize]
    }
    #[getter]
    pub fn texture_handles(&self) -> &[AssetHandleVER] {
        &self.asset_handles()[self.header().vdata_num.get() as usize..]
    }
}

#[make_platforms]
impl BinVER {
    pub fn model_data(&self) -> Option<&IndexMap<u32, Arc<[u8]>>> {
        self.model_data.as_ref()
    }
    pub fn texture_data(&self) -> Option<&IndexMap<u32, Arc<[u8]>>> {
        self.texture_data.as_ref()
    }
}
