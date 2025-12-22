#[cfg(feature = "python")]
use crate::pyobj_ref;
use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use log::warn;
#[cfg(not(feature = "python"))]
use lotrc_proc::getter;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::ptr::NonNull;
use std::sync::Arc;

use crate::types::{hash_string, Color, RefFromData};
#[make_platforms]
use crate::{
    level::{
        model::ModelVER,
        pak::{Block1VER, PakHeaderVER},
        bin::BinVER,
    },
    types::{ColorVER, CrcVER, I32VER, U32VER},
    sub_blocks::gameobjs::GameObjsVER,
};
use lotrc_proc::{make_platforms, OrderedData};

#[cfg(feature = "python")]
pub fn init(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "radiosity")?;
    m.add_class::<Radiosity>()?;
    m.add_class::<RadiosityValsInfo>()?;
    init_pc(&m)?;
    init_xbox(&m)?;
    init_ps3(&m)?;
    Ok(m)
}
#[cfg(feature = "python")]
#[make_platforms]
pub fn init_ver(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RadiosityVER>()?;
    m.add_class::<RadiosityValsVER>()
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs", get_all, set_all))]
// points to list of ints in block1
pub struct RadiosityValsInfo {
    pub guid: u32,
    pub num: u32,
    pub offset: u32,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs"))]
#[derive(Debug, Clone)]
// list of ints pointing to the radiosity data
// has a value for each mesh in the model
// corresponding to the object pointed to by the guid
// (value if offset to consecutive values in radiosity, one for each vertex of the mesh)
// a value of -1 means no radiosity for that mesh
pub struct RadiosityValsVER {
    _ptr: Arc<[u8]>,
    info: NonNull<RadiosityValsInfoVER>,
    vals: NonNull<[I32VER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(RadiosityValsVER);

#[make_platforms]
unsafe impl Sync for RadiosityValsVER {}
#[make_platforms]
unsafe impl Send for RadiosityValsVER {}

#[make_platforms]
impl RadiosityValsVER {
    pub fn from_bytes(src: &Arc<[u8]>, offset: usize) -> Result<Self> {
        let info = RadiosityValsInfoVER::from_data(&src[offset..]).context("info")?;
        let vals =
            I32VER::slice_from_data(&src[info.offset.get() as usize..], info.num.get() as usize)
                .context("vals")?;
        Ok(Self {
            _ptr: src.clone(),
            info: info.into(),
            vals: vals.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl RadiosityValsVER {
    #[getter]
    pub fn info(&self) -> &RadiosityValsInfoVER {
        unsafe { self.info.as_ref() }
    }
    #[getter]
    pub fn vals(&self) -> &[I32VER] {
        unsafe { self.vals.as_ref() }
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "bin"))]
#[derive(Clone, Debug)]
pub struct RadiosityVER {
    _ptr: Arc<[u8]>,
    data: Arc<[u8]>,
    infos: NonNull<[RadiosityValsInfoVER]>,
    offs: Box<[NonNull<[I32VER]>]>,
    vals: NonNull<[ColorVER]>,
    usage: u32,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(RadiosityVER);

#[make_platforms]
unsafe impl Sync for RadiosityVER {}
#[make_platforms]
unsafe impl Send for RadiosityVER {}

#[make_platforms]
impl RadiosityVER {
    pub fn from_bytes(src: &Arc<[u8]>, bin: &BinVER, level: &GameObjsVER, pak_header: &PakHeaderVER) -> Result<Self> {
        let infos = RadiosityValsInfoVER::slice_from_data(
            &src[pak_header.radiosity_vals_info_offset.get() as usize..],
            pak_header.radiosity_vals_info_num.get() as usize
        ).context("infos")?;
        let offs = infos.iter().map(|info| Ok(NonNull::from_ref(I32VER::slice_from_data(&src[info.offset.get() as usize..], info.num.get() as usize)?))).collect::<Result<Vec<_>>>().context("offs")?;
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
        let radiosity_name = hash_string(b"_radiosity", Some(name.get()));
        let model_data = bin.model_data().unwrap();
        let ind = model_data.get_index_of(&radiosity_name);
        let data = ind.map(|x| model_data.get_index(x).unwrap().1.clone()).unwrap_or_default();
        let usage =  ind.map(|x| bin.model_handles()[x].kind.get()).unwrap_or_default();
        if data.len() % 4 != 0 {
            warn!("Radiosity length is incorrect?")
        }
        let vals = NonNull::from_ref(ColorVER::slice_from_data(&data[..], data.len() / 4).context("vals")?);
        Ok(Self {
            _ptr: src.clone(),
            data,
            infos: infos.into(),
            offs: offs.into(),
            vals,
            usage
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl RadiosityVER {
    #[getter]
    pub fn vals(&self) -> &[ColorVER] {
        unsafe { self.vals.as_ref() }
    }
    #[getter]
    pub fn usage(&self) -> &u32 {
        &self.usage
    }
    #[getter]
    pub fn infos(&self) -> &[RadiosityValsInfoVER] {
        unsafe { self.infos.as_ref() }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "python", pyclass(module = "bin", get_all, set_all))]
pub enum RadiosityVal {
    Radiosity(Vec<Color>),
    NoRadiosity(u32),
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "python", pyclass(module = "bin", get_all, set_all))]
pub struct Radiosity {
    pub vals: IndexMap<u32, Vec<RadiosityVal>>,
    pub usage: u32,
}

impl Radiosity {
    #[make_platforms]
    pub fn from_ver(val: &RadiosityVER, block1: &Block1VER) -> Self {
        let objs = block1.objs();
        let gameobjs = block1
            .sub_blocks()
            .get(&CrcVER::new(hash_string(b"level", None)))
            .unwrap()
            .level()
            .unwrap();
        let model_inds: IndexMap<_, _> = objs
            .models()
            .iter()
            .map(|model| (model.info().key, model))
            .collect();
        let mut off: usize = 0;
        Radiosity {
            vals: val.offs.iter().zip(val.infos())
                .map(|(vals, info)| {
                    let obj = gameobjs.objs().get(&info.guid).unwrap();
                    let model = Context::<_, anyhow::Error>::with_context(
                        {
                            let field = obj
                                .get(&CrcVER::new(hash_string(b"mesh", None)))
                                .ok_or(anyhow!("missing mesh field"))?;
                            let mesh = field.crc().ok_or(anyhow!("mesh field not crc"))?;
                            Result::Ok(
                                *model_inds
                                    .get(mesh)
                                    .ok_or_else(|| anyhow!("model {} missing", mesh))?,
                            )
                        },
                        || format!("obj {}", info.guid.get()),
                    )?;
                    Ok((
                        info.guid.get(),
                        model.data().infos().iter()
                            .map(|x| (x.vbuff_size.get() / x.v_size.get()) as usize)
                            .zip(unsafe { vals.as_ref() })
                            .map(|(offset, size)| {
                                if offset != off {
                                    RadiosityVal::NoRadiosity(offset as u32)
                                } else {
                                    off += size.get() as usize;
                                    RadiosityVal::Radiosity(
                                        val.vals()[offset..offset + size.get() as usize]
                                            .iter()
                                            .map(|x| x.into())
                                            .collect(),
                                    )
                                }
                            })
                            .collect(),
                    ))
                })
                .collect::<Result<_>>()
                .unwrap(),
            usage: val.usage,
        }
    }
}
