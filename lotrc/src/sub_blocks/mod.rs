#[cfg(feature = "python")]
use crate::pyobj_ref;
use crate::types::{get_str, hash_string, Crc, DumpData, DumpSlice, RefFromData, Vector3, Vector4};
#[make_platforms]
use crate::{
    level::pak::objs::PFieldInfoVER,
    sub_blocks::gameobjs::{DumpGameObjsVER, GameObjsVER},
    types::{CrcVER, U16VER, U32VER},
};
use anyhow::{anyhow, Context, Result};
use gameobjs::GameObjs;
use indexmap::IndexMap;
use log::warn;
#[cfg(not(feature = "python"))]
use lotrc_proc::getter;
use lotrc_proc::{make_platforms, OrderedData};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;
use std::sync::Arc;

pub mod gameobjs;

#[cfg(feature = "python")]
pub fn init(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "sub_blocks")?;
    m.add_class::<SubBlock>()?;
    m.add_class::<SubBlocksHeader>()?;
    m.add_class::<SubBlocksBlockHeader>()?;
    m.add_class::<SubBlocks>()?;
    m.add_class::<LangStrings>()?;
    m.add_class::<SSAVal>()?;
    m.add_class::<SSA>()?;
    m.add_class::<SprayInstance>()?;
    m.add_class::<SprayVal>()?;
    m.add_class::<Spray>()?;
    m.add_class::<CrowdItemHeader>()?;
    m.add_class::<CrowdVal>()?;
    m.add_class::<CrowdItem>()?;
    m.add_class::<CrowdHeader>()?;
    m.add_class::<Crowd>()?;
    m.add_class::<AtlasUVVal>()?;
    m.add_class::<AtlasUV>()?;
    m.add_class::<Data>()?;
    m.add_class::<PField>()?;
    m.add_class::<PFields>()?;
    init_pc(&m)?;
    init_xbox(&m)?;
    init_ps3(&m)?;
    Ok(m)
}
#[cfg(feature = "python")]
#[make_platforms]
pub fn init_ver(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SubBlockVER>()?;
    m.add_class::<SubBlocksVER>()?;
    m.add_class::<LangStringsVER>()?;
    m.add_class::<SSAVER>()?;
    m.add_class::<SprayVER>()?;
    m.add_class::<CrowdItemVER>()?;
    m.add_class::<CrowdVER>()?;
    m.add_class::<AtlasUVVER>()?;
    m.add_class::<DataVER>()
}

pub mod keys {
    use super::hash_string;
    pub const KEY_POLISH: u32 = hash_string("Polish".as_bytes(), None);
    pub const KEY_GERMAN: u32 = hash_string("German".as_bytes(), None);
    pub const KEY_FRENCH: u32 = hash_string("French".as_bytes(), None);
    pub const KEY_SPANISH: u32 = hash_string("Spanish".as_bytes(), None);
    pub const KEY_RUSSIAN: u32 = hash_string("Russian".as_bytes(), None);
    pub const KEY_SWEDISH: u32 = hash_string("Swedish".as_bytes(), None);
    pub const KEY_ENGLISH: u32 = hash_string("English".as_bytes(), None);
    pub const KEY_ITALIAN: u32 = hash_string("Italian".as_bytes(), None);
    pub const KEY_NORWEGIAN: u32 = hash_string("Norwegian".as_bytes(), None);
    pub const KEY_ATLAS1: u32 = hash_string("atlas_1.uv".as_bytes(), None);
    pub const KEY_ATLAS2: u32 = hash_string("atlas_2.uv".as_bytes(), None);
    pub const KEY_CROWD: u32 = hash_string("3dCrowd".as_bytes(), None);
    pub const KEY_SPRAY: u32 = hash_string("Spray".as_bytes(), None);
    pub const KEY_PFIELDS: u32 = hash_string("PFields".as_bytes(), None);
    pub const KEY_LEVEL: u32 = hash_string("Level".as_bytes(), None);
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks"))]
#[derive(Debug, Clone)]
pub enum SubBlockVER {
    LangStrings(LangStringsVER),
    Data(DataVER),
    Spray(SprayVER),
    PFields(PFieldsVER),
    Crowd(CrowdVER),
    Level(GameObjsVER),
    AtlasUV(AtlasUVVER),
    Lua(LuaVER),
    SSA(SSAVER),
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(SubBlockVER);

#[make_platforms]
unsafe impl Sync for SubBlockVER {}
#[make_platforms]
unsafe impl Send for SubBlockVER {}

#[make_platforms]
impl SubBlockVER {
    pub fn from_bytes(src: &Arc<[u8]>, offset: usize, size: usize, key: u32) -> Result<Self> {
        use keys::*;
        Ok(match key {
            KEY_POLISH | KEY_GERMAN | KEY_FRENCH | KEY_SPANISH | KEY_RUSSIAN | KEY_SWEDISH
            | KEY_ENGLISH | KEY_ITALIAN | KEY_NORWEGIAN => {
                Self::LangStrings(LangStringsVER::from_bytes(src, offset, size)?)
            }
            KEY_SPRAY => Self::Spray(SprayVER::from_bytes(src, offset, size)?),
            KEY_CROWD => Self::Crowd(CrowdVER::from_bytes(src, offset, size)?),
            KEY_PFIELDS => Self::PFields(PFieldsVER::from_bytes(src, offset, size)?),
            KEY_LEVEL => Self::Level(GameObjsVER::from_bytes(&src, offset, size, -1)?),
            KEY_ATLAS1 | KEY_ATLAS2 => Self::AtlasUV(AtlasUVVER::from_bytes(src, offset, size)?),
            _ => match get_str(&key) {
                Some(x) if x.ends_with(".lua") => Self::Lua(LuaVER::from_bytes(src, offset, size)?),
                Some(x) if x.ends_with(".ssa") => Self::SSA(SSAVER::from_bytes(src, offset, size)?),
                Some(x) if x.ends_with(".csv") || x.ends_with(".txt") || x.ends_with(".dat") => {
                    Self::Data(DataVER::from_bytes(src, offset, size)?)
                }
                name => {
                    warn!("Unknown block type {:?}, {:?}", key, name);
                    Self::Data(DataVER::from_bytes(src, offset, size)?)
                }
            },
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl SubBlockVER {
    #[getter]
    pub fn langstrings(&self) -> Option<&LangStringsVER> {
        match self {
            Self::LangStrings(val) => Some(val),
            _ => None,
        }
    }
    #[getter]
    pub fn data(&self) -> Option<&DataVER> {
        match self {
            Self::Data(val) => Some(val),
            _ => None,
        }
    }
    #[getter]
    pub fn spray(&self) -> Option<&SprayVER> {
        match self {
            Self::Spray(val) => Some(val),
            _ => None,
        }
    }
    #[getter]
    pub fn pfields(&self) -> Option<&PFieldsVER> {
        match self {
            Self::PFields(val) => Some(val),
            _ => None,
        }
    }
    #[getter]
    pub fn crowd(&self) -> Option<&CrowdVER> {
        match self {
            Self::Crowd(val) => Some(val),
            _ => None,
        }
    }
    #[getter]
    pub fn level(&self) -> Option<&GameObjsVER> {
        match self {
            Self::Level(val) => Some(val),
            _ => None,
        }
    }
    #[getter]
    pub fn atlasuv(&self) -> Option<&AtlasUVVER> {
        match self {
            Self::AtlasUV(val) => Some(val),
            _ => None,
        }
    }
    #[getter]
    pub fn lua(&self) -> Option<&LuaVER> {
        match self {
            Self::Lua(val) => Some(val),
            _ => None,
        }
    }
    #[getter]
    pub fn ssa(&self) -> Option<&SSAVER> {
        match self {
            Self::SSA(val) => Some(val),
            _ => None,
        }
    }
}

#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[derive(Debug, Clone)]
pub enum SubBlock {
    LangStrings(LangStrings),
    Data(Data),
    Spray(Spray),
    PFields(PFields),
    Crowd(Crowd),
    Level(GameObjs),
    AtlasUV(AtlasUV),
    Lua(Lua),
    SSA(SSA),
}

#[make_platforms]
impl SubBlock {
    fn from_ver(val: &SubBlockVER, pfield_infos: &[PFieldInfoVER]) -> Self {
        match val {
            SubBlockVER::LangStrings(val) => Self::LangStrings(val.into()),
            SubBlockVER::Data(val) => Self::Data(val.into()),
            SubBlockVER::Spray(val) => Self::Spray(val.into()),
            SubBlockVER::Crowd(val) => Self::Crowd(val.into()),
            SubBlockVER::PFields(val) => Self::PFields(PFields::from_ver(val, pfield_infos)),
            SubBlockVER::Level(val) => Self::Level(val.into()),
            SubBlockVER::AtlasUV(val) => Self::AtlasUV(val.into()),
            SubBlockVER::Lua(val) => Self::Lua(val.into()),
            SubBlockVER::SSA(val) => Self::SSA(val.into()),
        }
    }
}

#[make_platforms]
pub trait DumpSubBlockVER {
    fn size(&self) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
}

#[make_platforms]
impl DumpSubBlockVER for SubBlockVER {
    fn size(&self) -> usize {
        match self {
            Self::LangStrings(val) => DumpLangStringsVER::size(val),
            Self::Data(val) => val.size(),
            Self::Spray(val) => DumpSprayVER::size(val),
            Self::PFields(val) => val.size(),
            Self::Crowd(val) => DumpCrowdVER::size(val),
            Self::Level(val) => DumpGameObjsVER::size(val),
            Self::AtlasUV(val) => DumpAtlasUVVER::size(val),
            Self::Lua(val) => val.size(),
            Self::SSA(val) => DumpSSAVER::size(val),
        }
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        match self {
            Self::LangStrings(val) => DumpLangStringsVER::dump_into(val, dst),
            Self::Data(val) => val.dump_into(dst),
            Self::Spray(val) => DumpSprayVER::dump_into(val, dst),
            Self::PFields(val) => val.dump_into(dst),
            Self::Crowd(val) => DumpCrowdVER::dump_into(val, dst),
            Self::Level(val) => DumpGameObjsVER::dump_into(val, dst),
            Self::AtlasUV(val) => DumpAtlasUVVER::dump_into(val, dst),
            Self::Lua(val) => val.dump_into(dst),
            Self::SSA(val) => DumpSSAVER::dump_into(val, dst),
        }
    }
}
#[make_platforms]
impl DumpSubBlockVER for SubBlock {
    fn size(&self) -> usize {
        match self {
            Self::LangStrings(val) => DumpLangStringsVER::size(val),
            Self::Data(val) => val.size(),
            Self::Spray(val) => DumpSprayVER::size(val),
            Self::PFields(val) => val.size_ver(),
            Self::Crowd(val) => DumpCrowdVER::size(val),
            Self::Level(val) => DumpGameObjsVER::size(val),
            Self::AtlasUV(val) => DumpAtlasUVVER::size(val),
            Self::Lua(val) => val.size(),
            Self::SSA(val) => DumpSSAVER::size(val),
        }
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        match self {
            Self::LangStrings(val) => DumpLangStringsVER::dump_into(val, dst),
            Self::Data(val) => val.dump_into(dst),
            Self::Spray(val) => DumpSprayVER::dump_into(val, dst),
            Self::PFields(val) => val.dump_into_ver(dst),
            Self::Crowd(val) => DumpCrowdVER::dump_into(val, dst),
            Self::Level(val) => DumpGameObjsVER::dump_into(val, dst),
            Self::AtlasUV(val) => DumpAtlasUVVER::dump_into(val, dst),
            Self::Lua(val) => val.dump_into(dst),
            Self::SSA(val) => DumpSSAVER::dump_into(val, dst),
        }
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[repr(C)]
pub struct SubBlocksHeader {
    pub z0: u32,
    pub block_num: u32,
    pub z2: u32,
    pub z3: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[repr(C)]
pub struct SubBlocksBlockHeader {
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks"))]
#[derive(Debug, Clone)]
pub struct SubBlocksVER {
    _ptr: Arc<[u8]>,
    header: NonNull<SubBlocksHeaderVER>,
    block_headers: NonNull<[SubBlocksBlockHeaderVER]>,
    blocks: IndexMap<CrcVER, SubBlockVER>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(SubBlocksVER);

#[make_platforms]
unsafe impl Sync for SubBlocksVER {}
#[make_platforms]
unsafe impl Send for SubBlocksVER {}

#[make_platforms]
impl SubBlocksVER {
    pub fn from_bytes(src: &Arc<[u8]>, mut offset: usize) -> Result<Self> {
        let start = offset;
        let header = SubBlocksHeaderVER::from_data(&src[offset..]).context("header")?;
        offset += header.size();
        let block_headers = SubBlocksBlockHeaderVER::slice_from_data(
            &src[offset..],
            header.block_num.get() as usize,
        )
        .context("block_headers")?;
        let blocks: IndexMap<CrcVER, SubBlockVER> = block_headers
            .into_iter()
            .map(|info| {
                Ok((
                    info.key.clone(),
                    SubBlockVER::from_bytes(
                        src,
                        start + info.offset.get() as usize,
                        info.size.get() as usize,
                        info.key.get(),
                    )
                    .with_context(|| {
                        format!(
                            "sub_blocks {}",
                            get_str(&info.key.get())
                                .map(|x| x.to_string())
                                .unwrap_or_else(|| format!("id: {}", info.key.get()))
                        )
                    })?,
                ))
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            _ptr: src.clone(),
            header: header.into(),
            block_headers: block_headers.into(),
            blocks: blocks.into(),
        })
    }
    pub fn get(&self, key: &CrcVER) -> Option<&SubBlockVER> {
        self.blocks.get(key)
    }
    pub fn get_full(&self, key: &CrcVER) -> Option<(&SubBlocksBlockHeaderVER, &SubBlockVER)> {
        self.blocks.get_full(key).map(|(i, _, val)| {
            let info = &self.block_headers()[i];
            (info, val)
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl SubBlocksVER {
    #[getter]
    pub fn header(&self) -> &SubBlocksHeaderVER {
        unsafe { self.header.as_ref() }
    }
    #[getter]
    pub fn block_headers(&self) -> &[SubBlocksBlockHeaderVER] {
        unsafe { self.block_headers.as_ref() }
    }
}

#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct SubBlocks {
    pub blocks: IndexMap<Crc, SubBlock>,
}

impl SubBlocks {
    #[make_platforms]
    pub fn from_ver(val: &SubBlocksVER, pfield_infos: &[PFieldInfoVER]) -> Self {
        Self {
            blocks: val
                .blocks
                .iter()
                .map(|(k, block)| (k.into(), SubBlock::from_ver(block, pfield_infos)))
                .collect(),
        }
    }
}

#[make_platforms]
pub trait DumpSubBlocksVER {
    fn blocks_num(&self) -> usize;
    fn blocks(&self) -> impl Iterator<Item = &impl DumpSubBlockVER>;
    fn write_names<'a>(&self, names: impl Iterator<Item = &'a mut CrcVER>);

    fn size(&self) -> usize {
        SubBlocksHeaderVER::size_of()
            + SubBlocksBlockHeaderVER::size_of() * self.blocks_num()
            + self.blocks().map(|x| x.size()).sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let start = dst.offset;
        let header = SubBlocksHeaderVER::mut_from_data(dst).context("header")?;
        let block_headers = SubBlocksBlockHeaderVER::mut_slice_from_data(dst, self.blocks_num())
            .context("block_headers")?;
        header.block_num = block_headers.len().into();
        dst.align(16);
        self.write_names(block_headers.iter_mut().map(|x| &mut x.key));
        for (i, (block, info)) in self.blocks().zip(block_headers).enumerate() {
            info.offset = (dst.offset - start).into();
            let start = dst.offset;
            block
                .dump_into(dst)
                .with_context(|| format!("block {}", i))?;
            info.size = (dst.offset - start).into();
            dst.align(16);
        }

        Ok(())
    }
}

#[make_platforms]
impl DumpSubBlocksVER for SubBlocksVER {
    fn blocks_num(&self) -> usize {
        self.blocks.len()
    }
    fn blocks(&self) -> impl Iterator<Item = &impl DumpSubBlockVER> {
        self.blocks.values()
    }
    fn write_names<'a>(&self, names: impl Iterator<Item = &'a mut CrcVER>) {
        for (src, dst) in self.blocks.keys().zip(names) {
            *dst = *src;
        }
    }
}

#[make_platforms]
impl DumpSubBlocksVER for SubBlocks {
    fn blocks_num(&self) -> usize {
        self.blocks.len()
    }
    fn blocks(&self) -> impl Iterator<Item = &impl DumpSubBlockVER> {
        self.blocks.values()
    }
    fn write_names<'a>(&self, names: impl Iterator<Item = &'a mut CrcVER>) {
        for (src, dst) in self.blocks.keys().zip(names) {
            *dst = src.into();
        }
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks"))]
#[derive(Debug, Clone)]
pub struct LangStringsVER {
    _ptr: Arc<[u8]>,
    strings: Box<[NonNull<[U16VER]>]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(LangStringsVER);

#[make_platforms]
unsafe impl Sync for LangStringsVER {}
#[make_platforms]
unsafe impl Send for LangStringsVER {}

#[make_platforms]
impl LangStringsVER {
    pub fn from_bytes(src: &Arc<[u8]>, mut offset: usize, size: usize) -> Result<Self> {
        let mut strings = vec![];
        while offset < size {
            let start = offset;
            while src[offset] != 0 || src[offset + 1] != 0 {
                offset += 2;
            }
            let s = U16VER::slice_from_data(&src[start..offset], (offset - start) / 2)
                .with_context(|| format!("string {}", strings.len()))?;
            strings.push(NonNull::from(s));
            offset += 2;
        }
        Ok(Self {
            _ptr: src.clone(),
            strings: strings.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl LangStringsVER {
    #[getter]
    pub fn strings(&self) -> Vec<&[U16VER]> {
        self.strings.iter().map(|x| unsafe { x.as_ref() }).collect()
    }
}

#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct LangStrings {
    pub strings: Vec<String>,
}

#[make_platforms]
impl From<&LangStringsVER> for LangStrings {
    fn from(val: &LangStringsVER) -> Self {
        Self {
            strings: val
                .strings
                .iter()
                .map(|x| {
                    String::from_utf16(
                        unsafe { x.as_ref() }
                            .iter()
                            .map(|y| y.into())
                            .collect::<Vec<_>>()
                            .as_ref(),
                    )
                    .unwrap()
                })
                .collect(),
        }
    }
}

#[make_platforms]
pub trait DumpStringVER {
    fn string_size(&self) -> usize;
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()>;
}

#[make_platforms]
impl DumpStringVER for &[U16VER] {
    fn string_size(&self) -> usize {
        self.size()
    }
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()> {
        self.dump_into(dst)
    }
}

#[make_platforms]
impl DumpStringVER for &String {
    fn string_size(&self) -> usize {
        self.encode_utf16().count() * U16VER::size_of()
    }
    fn dump_string(&self, dst: &mut DumpSlice) -> Result<()> {
        let val = self.encode_utf16().collect::<Vec<_>>();
        let string = U16VER::mut_slice_from_data(dst, val.len()).context("string")?;
        for (src, dst) in val.into_iter().zip(string) {
            *dst = src.into();
        }
        Ok(())
    }
}

#[make_platforms]
pub trait DumpLangStringsVER {
    fn strings(&self) -> impl Iterator<Item = impl DumpStringVER>;
    fn size(&self) -> usize {
        self.strings()
            .map(|x| x.string_size() + U16VER::size_of())
            .sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        for (i, string) in self.strings().enumerate() {
            string
                .dump_string(dst)
                .with_context(|| format!("string {}", i))?;
            U16VER::from(0u16)
                .dump_into(dst)
                .with_context(|| format!("string pad {}", i))?;
        }
        Ok(())
    }
}

#[make_platforms]
impl DumpLangStringsVER for LangStringsVER {
    fn strings(&self) -> impl Iterator<Item = impl DumpStringVER> {
        self.strings.iter().map(|x| unsafe { x.as_ref() })
    }
}

#[make_platforms]
impl DumpLangStringsVER for LangStrings {
    fn strings(&self) -> impl Iterator<Item = impl DumpStringVER> {
        self.strings.iter()
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[repr(C)]
pub struct SSAVal {
    pub t_start: f32,
    pub t_end: f32,
    pub unk_2: u32,
    pub unk_3: u32,
    pub off: u32,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks"))]
#[derive(Debug, Clone)]
pub struct SSAVER {
    _ptr: Arc<[u8]>,
    vals: NonNull<[SSAValVER]>,
    strings: Box<[NonNull<[U16VER]>]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(SSAVER);

#[make_platforms]
unsafe impl Sync for SSAVER {}
#[make_platforms]
unsafe impl Send for SSAVER {}

#[make_platforms]
impl SSAVER {
    pub fn from_bytes(src: &Arc<[u8]>, offset: usize, size: usize) -> Result<Self> {
        let n = U32VER::from_data(&src[offset..]).context("n")?;
        let vals =
            SSAValVER::slice_from_data(&src[offset + 4..], n.get() as usize).context("vals")?;
        let strings = (0..n.get() as usize)
            .map(|i| {
                let start = vals[i].off.get() as usize + offset;
                let end = if i == n.get() as usize - 1 {
                    size
                } else {
                    vals[i + 1].off.get() as usize
                } + offset;
                Ok(NonNull::from_ref(
                    U16VER::slice_from_data(&src[start..], (end - start) / 2)
                        .with_context(|| format!("string {}", i))?,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            _ptr: src.clone(),
            vals: vals.into(),
            strings: strings.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl SSAVER {
    #[getter]
    pub fn vals(&self) -> &[SSAValVER] {
        unsafe { self.vals.as_ref() }
    }
    #[getter]
    pub fn strings(&self) -> Vec<&[U16VER]> {
        self.strings.iter().map(|x| unsafe { x.as_ref() }).collect()
    }
}

#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct SSA {
    pub vals: Vec<SSAVal>,
    pub strings: Vec<String>,
}

#[make_platforms]
impl From<&SSAVER> for SSA {
    fn from(val: &SSAVER) -> Self {
        Self {
            vals: val.vals().iter().map(|x| x.into()).collect(),
            strings: val
                .strings
                .iter()
                .map(|x| {
                    String::from_utf16(
                        unsafe { x.as_ref() }
                            .iter()
                            .map(|y| y.into())
                            .collect::<Vec<_>>()
                            .as_ref(),
                    )
                    .unwrap()
                })
                .collect(),
        }
    }
}

#[make_platforms]
pub trait DumpSSAVER {
    fn vals_len(&self) -> usize;
    fn strings(&self) -> impl Iterator<Item = impl DumpStringVER>;
    fn write_vals(&self, vals: &mut [SSAValVER]) -> Result<()>;

    fn size(&self) -> usize {
        U32VER::size_of()
            + SSAValVER::size_of() * self.vals_len()
            + self.strings().map(|x| x.string_size()).sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let start = dst.offset;
        let n = U32VER::mut_from_data(dst).context("n")?;
        let vals = SSAValVER::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
        *n = vals.len().into();
        self.write_vals(vals).context("write vals")?;

        for (i, (string, val)) in self.strings().zip(vals).enumerate() {
            val.off = (dst.offset - start).into();
            string
                .dump_string(dst)
                .with_context(|| format!("string {}", i))?;
        }
        Ok(())
    }
}

#[make_platforms]
impl DumpSSAVER for SSAVER {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn strings(&self) -> impl Iterator<Item = impl DumpStringVER> {
        self.strings.iter().map(|x| unsafe { x.as_ref() })
    }
    fn write_vals(&self, vals: &mut [SSAValVER]) -> Result<()> {
        vals.write_from(self.vals())
    }
}

#[make_platforms]
impl DumpSSAVER for SSA {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn strings(&self) -> impl Iterator<Item = impl DumpStringVER> {
        self.strings.iter()
    }
    fn write_vals(&self, vals: &mut [SSAValVER]) -> Result<()> {
        for (src, dst) in self.vals.iter().zip(vals) {
            *dst = src.into()
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[repr(C)]
pub struct SprayInstance {
    pub key: Crc,
    pub tex1: Crc,
    pub tex2: Crc,
    pub unk_3: u32,
    pub width: u32,
    pub height: u32,
    pub unk_6: f32,
    pub unk_7: f32,
    pub size_w: u32,
    pub size_h: u32,
    pub scale_w: f32,
    pub scale_h: f32,
    pub delay: u32,
    pub stride_x: f32,
    pub stride_y: f32,
    pub unk_15: u32,
    pub unk_16: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[repr(C)]
pub struct SprayVal {
    pub position: Vector3,
    pub scale: f32,
    pub instance: u16,
    pub rotation: u16,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks"))]
#[derive(Debug, Clone)]
pub struct SprayVER {
    _ptr: Arc<[u8]>,
    instances: NonNull<[SprayInstanceVER]>,
    vals: NonNull<[SprayValVER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(SprayVER);

#[make_platforms]
unsafe impl Sync for SprayVER {}
#[make_platforms]
unsafe impl Send for SprayVER {}

#[make_platforms]
impl SprayVER {
    pub fn from_bytes(src: &Arc<[u8]>, mut offset: usize, _size: usize) -> Result<Self> {
        let n = U32VER::from_data(&src[offset..]).context("n1")?;
        offset += n.size();
        let instances = SprayInstanceVER::slice_from_data(&src[offset..], n.get() as usize)
            .context("instances")?;
        offset += instances.size();
        let n = U32VER::from_data(&src[offset..]).context("n2")?;
        offset += n.size();
        let vals =
            SprayValVER::slice_from_data(&src[offset..], n.get() as usize).context("vals")?;

        Ok(Self {
            _ptr: src.clone(),
            instances: instances.into(),
            vals: vals.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl SprayVER {
    #[getter]
    pub fn instances(&self) -> &[SprayInstanceVER] {
        unsafe { self.instances.as_ref() }
    }
    #[getter]
    pub fn vals(&self) -> &[SprayValVER] {
        unsafe { self.vals.as_ref() }
    }
}

#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct Spray {
    pub instances: Vec<SprayInstance>,
    pub vals: Vec<SprayVal>,
}

#[make_platforms]
impl From<&SprayVER> for Spray {
    fn from(val: &SprayVER) -> Self {
        Self {
            instances: val.instances().iter().map(|x| x.into()).collect(),
            vals: val.vals().iter().map(|x| x.into()).collect(),
        }
    }
}

#[make_platforms]
pub trait DumpSprayVER {
    fn instances_len(&self) -> usize;
    fn vals_len(&self) -> usize;
    fn write_instances(&self, instances: &mut [SprayInstanceVER]) -> Result<()>;
    fn write_vals(&self, vals: &mut [SprayValVER]) -> Result<()>;

    fn size(&self) -> usize {
        U32VER::size_of() * 2
            + SprayInstanceVER::size_of() * self.instances_len()
            + SprayValVER::size_of() * self.vals_len()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let n = U32VER::mut_from_data(dst).context("n1")?;
        let instances = SprayInstanceVER::mut_slice_from_data(dst, self.instances_len())
            .context("instances")?;
        *n = instances.len().into();
        self.write_instances(instances).context("write instances")?;
        let n = U32VER::mut_from_data(dst).context("n1")?;
        let vals = SprayValVER::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
        *n = vals.len().into();
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
}

#[make_platforms]
impl DumpSprayVER for SprayVER {
    fn instances_len(&self) -> usize {
        self.instances().len()
    }
    fn vals_len(&self) -> usize {
        self.vals().len()
    }
    fn write_instances(&self, instances: &mut [SprayInstanceVER]) -> Result<()> {
        instances.write_from(self.instances())
    }
    fn write_vals(&self, vals: &mut [SprayValVER]) -> Result<()> {
        vals.write_from(self.vals())
    }
}

#[make_platforms]
impl DumpSprayVER for Spray {
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_instances(&self, instances: &mut [SprayInstanceVER]) -> Result<()> {
        for (src, dst) in self.instances.iter().zip(instances) {
            *dst = src.into()
        }
        Ok(())
    }
    fn write_vals(&self, vals: &mut [SprayValVER]) -> Result<()> {
        for (src, dst) in self.vals.iter().zip(vals) {
            *dst = src.into()
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[repr(C)]
pub struct CrowdItemHeader {
    pub key: Crc,
    pub key_main: Crc,
    pub key_right: Crc,
    pub key_left: Crc,
    pub unk_4: f32,
    pub animation_num: u32,
    pub instance_num: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[repr(C)]
pub struct CrowdVal {
    pub position: Vector3,
    pub rotation: f32,
    pub lod: f32,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks"))]
#[derive(Debug, Clone)]
pub struct CrowdItemVER {
    _ptr: Arc<[u8]>,
    header: NonNull<CrowdItemHeaderVER>,
    animations: NonNull<[CrcVER]>,
    instances: NonNull<[CrowdValVER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(CrowdItemVER);

#[make_platforms]
unsafe impl Sync for CrowdItemVER {}
#[make_platforms]
unsafe impl Send for CrowdItemVER {}

#[make_platforms]
impl CrowdItemVER {
    pub fn from_bytes(src: &Arc<[u8]>, mut offset: usize) -> Result<Self> {
        let header = CrowdItemHeaderVER::from_data(&src[offset..]).context("header")?;
        offset += header.size();
        let animations =
            CrcVER::slice_from_data(&src[offset..], header.animation_num.get() as usize)
                .context("animations")?;
        offset += animations.size();
        let instances =
            CrowdValVER::slice_from_data(&src[offset..], header.instance_num.get() as usize)
                .context("instances")?;
        Ok(Self {
            _ptr: src.clone(),
            header: header.into(),
            animations: animations.into(),
            instances: instances.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl CrowdItemVER {
    #[getter]
    pub fn header(&self) -> &CrowdItemHeaderVER {
        unsafe { self.header.as_ref() }
    }
    #[getter]
    pub fn animations(&self) -> &[CrcVER] {
        unsafe { self.animations.as_ref() }
    }
    #[getter]
    pub fn instances(&self) -> &[CrowdValVER] {
        unsafe { self.instances.as_ref() }
    }
}

#[cfg_attr(
    feature = "python",
    pyclass(module = "sub_blocks.gameobjs", get_all, set_all)
)]
#[derive(Debug, Clone)]
pub struct CrowdItem {
    pub header: CrowdItemHeader,
    pub animations: Vec<Crc>,
    pub instances: Vec<CrowdVal>,
}

#[make_platforms]
impl From<&CrowdItemVER> for CrowdItem {
    fn from(val: &CrowdItemVER) -> Self {
        Self {
            header: val.header().into(),
            animations: val.animations().iter().map(|x| x.into()).collect(),
            instances: val.instances().iter().map(|x| x.into()).collect(),
        }
    }
}

#[make_platforms]
pub trait DumpCrowdItemVER {
    fn animations_len(&self) -> usize;
    fn instances_len(&self) -> usize;
    fn write_header(&self, header: &mut CrowdItemHeaderVER) -> Result<()>;
    fn write_animations(&self, animations: &mut [CrcVER]) -> Result<()>;
    fn write_instances(&self, instances: &mut [CrowdValVER]) -> Result<()>;

    fn size(&self) -> usize {
        CrowdItemHeaderVER::size_of()
            + CrcVER::size_of() * self.animations_len()
            + CrowdValVER::size_of() * self.instances_len()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let header = CrowdItemHeaderVER::mut_from_data(dst).context("header")?;
        self.write_header(header).context("write header")?;
        header.animation_num = self.animations_len().into();
        header.instance_num = self.instances_len().into();
        let animations =
            CrcVER::mut_slice_from_data(dst, self.animations_len()).context("animations")?;
        self.write_animations(animations)
            .context("write animations")?;
        let instances =
            CrowdValVER::mut_slice_from_data(dst, self.instances_len()).context("instances")?;
        self.write_instances(instances).context("write instances")?;
        Ok(())
    }
}

#[make_platforms]
impl DumpCrowdItemVER for CrowdItemVER {
    fn animations_len(&self) -> usize {
        self.animations.len()
    }
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn write_header(&self, header: &mut CrowdItemHeaderVER) -> Result<()> {
        header.write_from(self.header())
    }
    fn write_instances(&self, instances: &mut [CrowdValVER]) -> Result<()> {
        instances.write_from(self.instances())
    }
    fn write_animations(&self, animations: &mut [CrcVER]) -> Result<()> {
        animations.write_from(self.animations())
    }
}

#[make_platforms]
impl DumpCrowdItemVER for CrowdItem {
    fn animations_len(&self) -> usize {
        self.animations.len()
    }
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn write_header(&self, header: &mut CrowdItemHeaderVER) -> Result<()> {
        *header = (&self.header).into();
        Ok(())
    }
    fn write_instances(&self, instances: &mut [CrowdValVER]) -> Result<()> {
        for (src, dst) in self.instances.iter().zip(instances) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_animations(&self, animations: &mut [CrcVER]) -> Result<()> {
        for (src, dst) in self.animations.iter().zip(animations) {
            *dst = src.into();
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[repr(C)]
pub struct CrowdHeader {
    pub const0x65: u32,
    pub n: u32,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks"))]
#[derive(Debug, Clone)]
pub struct CrowdVER {
    _ptr: Arc<[u8]>,
    header: NonNull<CrowdHeaderVER>,
    offs: NonNull<[U32VER]>,
    vals: Box<[CrowdItemVER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(CrowdVER);

#[make_platforms]
unsafe impl Sync for CrowdVER {}
#[make_platforms]
unsafe impl Send for CrowdVER {}

#[make_platforms]
impl CrowdVER {
    pub fn from_bytes(src: &Arc<[u8]>, offset: usize, _size: usize) -> Result<Self> {
        let header = CrowdHeaderVER::from_data(&src[offset..]).context("header")?;
        if header.const0x65.get() != 0x65 {
            return Err(anyhow!("Invalid Block Data for Crowd Block"));
        }
        let offs = U32VER::slice_from_data(&src[offset + header.size()..], header.n.get() as usize)
            .context("offs")?;
        let vals = offs
            .into_iter()
            .enumerate()
            .map(|(i, off)| {
                CrowdItemVER::from_bytes(src, off.get() as usize + offset)
                    .with_context(|| format!("item {}", i))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            _ptr: src.clone(),
            header: header.into(),
            offs: offs.into(),
            vals: vals.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl CrowdVER {
    #[getter]
    pub fn header(&self) -> &CrowdHeaderVER {
        unsafe { self.header.as_ref() }
    }
    #[getter]
    pub fn offs(&self) -> &[U32VER] {
        unsafe { self.offs.as_ref() }
    }
    #[getter]
    pub fn vals(&self) -> &[CrowdItemVER] {
        &self.vals
    }
}

#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct Crowd {
    pub vals: Vec<CrowdItem>,
}

#[make_platforms]
impl From<&CrowdVER> for Crowd {
    fn from(val: &CrowdVER) -> Self {
        Self {
            vals: val.vals().iter().map(|x| x.into()).collect(),
        }
    }
}

#[make_platforms]
pub trait DumpCrowdVER {
    fn vals_len(&self) -> usize;
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItemVER>;

    fn size(&self) -> usize {
        CrowdHeaderVER::size_of()
            + U32VER::size_of() * self.vals_len()
            + self.vals().map(|x| x.size()).sum::<usize>()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let start = dst.offset;
        let header = CrowdHeaderVER::mut_from_data(dst).context("header")?;
        let offs = U32VER::mut_slice_from_data(dst, self.vals_len()).context("offs")?;
        header.n = offs.len().into();
        header.const0x65 = 0x65u32.into();

        for (i, (val, off)) in self.vals().zip(offs).enumerate() {
            *off = (dst.offset - start).into();
            val.dump_into(dst).with_context(|| format!("val {}", i))?;
        }

        Ok(())
    }
}

#[make_platforms]
impl DumpCrowdVER for CrowdVER {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItemVER> {
        self.vals.iter()
    }
}

#[make_platforms]
impl DumpCrowdVER for Crowd {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItemVER> {
        self.vals.iter()
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[repr(C)]
pub struct AtlasUVVal {
    pub key: Crc,
    pub vals: Vector4,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks"))]
#[derive(Debug, Clone)]
pub struct AtlasUVVER {
    _ptr: Arc<[u8]>,
    vals: NonNull<[AtlasUVValVER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(AtlasUVVER);

#[make_platforms]
unsafe impl Sync for AtlasUVVER {}
#[make_platforms]
unsafe impl Send for AtlasUVVER {}

#[make_platforms]
impl AtlasUVVER {
    pub fn from_bytes(src: &Arc<[u8]>, offset: usize, size: usize) -> Result<Self> {
        if size % std::mem::size_of::<AtlasUVValVER>() != 0 {
            return Err(anyhow!("Invalid UV Atlas size {}", size));
        }
        let num = size / std::mem::size_of::<AtlasUVValVER>();
        let vals = AtlasUVValVER::slice_from_data(&src[offset..], num).context("vals")?;
        Ok(Self {
            _ptr: src.clone(),
            vals: vals.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl AtlasUVVER {
    #[getter]
    pub fn vals(&self) -> &[AtlasUVValVER] {
        unsafe { self.vals.as_ref() }
    }
}

#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct AtlasUV {
    pub vals: Vec<AtlasUVVal>,
}

#[make_platforms]
impl From<&AtlasUVVER> for AtlasUV {
    fn from(val: &AtlasUVVER) -> Self {
        Self {
            vals: val.vals().iter().map(|x| x.into()).collect(),
        }
    }
}

#[make_platforms]
pub trait DumpAtlasUVVER {
    fn vals_len(&self) -> usize;
    fn write_vals(&self, vals: &mut [AtlasUVValVER]) -> Result<()>;

    fn size(&self) -> usize {
        self.vals_len() * AtlasUVValVER::size_of()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let vals = AtlasUVValVER::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
}

#[make_platforms]
impl DumpAtlasUVVER for AtlasUVVER {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [AtlasUVValVER]) -> Result<()> {
        vals.write_from(self.vals())
    }
}

#[make_platforms]
impl DumpAtlasUVVER for AtlasUV {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [AtlasUVValVER]) -> Result<()> {
        for (src, dst) in self.vals.iter().zip(vals) {
            *dst = src.into();
        }
        Ok(())
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks"))]
pub struct DataVER {
    _ptr: Arc<[u8]>,
    data: NonNull<[u8]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(DataVER);

#[make_platforms]
unsafe impl Sync for DataVER {}
#[make_platforms]
unsafe impl Send for DataVER {}

#[make_platforms]
impl DataVER {
    pub fn from_bytes(src: &Arc<[u8]>, offset: usize, size: usize) -> Result<Self> {
        Ok(Self {
            _ptr: src.clone(),
            data: NonNull::from_ref(&src[offset..offset + size]),
        })
    }
    pub fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        self.data().dump_into(dst)
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl DataVER {
    #[getter]
    pub fn data(&self) -> &[u8] {
        unsafe { self.data.as_ref() }
    }
}

#[make_platforms]
type LuaVER = DataVER;
#[make_platforms]
type PFieldsVER = DataVER;

#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct Data {
    pub data: Vec<u8>,
}

impl Data {
    pub fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        self.data.as_slice().dump_into(dst)
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[make_platforms]
impl From<&DataVER> for Data {
    fn from(val: &DataVER) -> Self {
        Self {
            data: val.data().to_vec(),
        }
    }
}

type Lua = Data;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
pub struct PField {
    pub link_guid: u32,
    pub vals: Vec<(HashSet<u32>, Vec<u8>)>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "python", pyclass(module = "sub_blocks", get_all, set_all))]
pub struct PFields {
    pub vals: IndexMap<u32, PField>,
}

impl PFields {
    #[make_platforms]
    pub fn from_ver(val: &PFieldsVER, infos: &[PFieldInfoVER]) -> Self {
        let mut offset_maps: HashMap<u32, HashMap<u32, usize>> = HashMap::new();
        let mut pfields = IndexMap::new();
        let data = val.data();
        for info in infos {
            let link_guid = info.link_guid.get();
            let offset = info.offset.get();
            let gamemode_guid = info.gamemode_guid.get();
            let width = info.width.get();
            let height = info.height.get();
            if !pfields.contains_key(&link_guid) {
                pfields.insert(
                    link_guid,
                    PField {
                        link_guid,
                        width,
                        height,
                        ..Default::default()
                    },
                );
                offset_maps.insert(link_guid, HashMap::new());
            }
            let val = pfields.get_mut(&link_guid).unwrap();
            let offset_map = offset_maps.get_mut(&link_guid).unwrap();
            if let Some(&index) = offset_map.get(&offset) {
                val.vals[index].0.insert(gamemode_guid);
            } else {
                offset_map.insert(offset, val.vals.len());
                let mut gamemodes = HashSet::new();
                gamemodes.insert(gamemode_guid);
                let offset = offset as usize;
                let size = (width * height) as usize;
                let vals = data[offset..offset + size].to_vec();
                val.vals.push((gamemodes, vals));
            }
        }
        Self { vals: pfields }
    }

    #[make_platforms]
    pub fn infos_ver(&self) -> Vec<PFieldInfoVER> {
        let mut infos = vec![];
        let mut offset = 0u32;
        for pfield in self.vals.values() {
            for (gamemodes, _) in &pfield.vals {
                for &gamemode_guid in gamemodes {
                    infos.push(PFieldInfoVER {
                        link_guid: pfield.link_guid.into(),
                        width: pfield.width.into(),
                        height: pfield.height.into(),
                        gamemode_guid: gamemode_guid.into(),
                        offset: offset.into(),
                    });
                }
                offset += pfield.width * pfield.height;
            }
        }
        infos
    }
    #[make_platforms]
    pub fn size_ver(&self) -> usize {
        self.vals
            .values()
            .flat_map(|x| x.vals.iter())
            .map(|(_, vals)| vals.size())
            .sum::<usize>()
    }
    #[make_platforms]
    pub fn dump_into_ver(&self, dst: &mut DumpSlice) -> Result<()> {
        for (i, pfield) in self.vals.values().enumerate() {
            for (j, (_, vals)) in pfield.vals.iter().enumerate() {
                vals.dump_into(dst)
                    .with_context(|| format!("pfield {} val {}", i, j))?;
            }
        }
        Ok(())
    }
}
