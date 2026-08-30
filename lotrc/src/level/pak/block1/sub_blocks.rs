use crate::types::{get_str, hash_string, Crc, DumpSlice, ReadData, Vector4, align_offset, ref_slice, slice, BaseTypes, NE};
use anyhow::{anyhow, Context, Result};
use crate::types::sub_blocks::{
    DataRef, SubBlocksBlockHeader, SubBlocksInfoRef, SubBlocksHeader, Data, DumpString, DumpDataImpl
};
use crate::level::pak::{
    block1::gameobjs::{
        GameObjsRef, TypeInfos, DumpGameObjs
    }
};
use indexmap::IndexMap;
use log::{warn, debug};
use lotrc_proc::{derive_pod};
use enum_dispatch::enum_dispatch;

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

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SubBlocks1Ref<'a, T: BaseTypes> {
    pub info: SubBlocksInfoRef<'a, T>,
    pub files: IndexMap<u32, DataRef<'a>>,
    pub lua: IndexMap<u32, LuaRef<'a>>,
    pub subtitles: IndexMap<u32, SSARef<'a, T>>,
    pub atlas1: Option<AtlasUVRef<'a, T>>,
    pub atlas2: Option<AtlasUVRef<'a, T>>,
    pub level: GameObjsRef<'a, T>
}

impl<T: BaseTypes> Default for SubBlocks1Ref<'_, T> {
    fn default() -> Self {
        Self {
            info: Default::default(),
            files: IndexMap::default().into(),
            lua: IndexMap::default().into(),
            subtitles: IndexMap::default().into(),
            atlas1: None.into(),
            atlas2: None.into(),
            level: Default::default()
        }
    }
}

impl<'a, T: BaseTypes> SubBlocks1Ref<'a, T> {
    // TODO should use the pak string to get key names
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        use keys::*;
        let info = SubBlocksInfoRef::<T>::from_data(src)?;
        let mut files = IndexMap::new();
        let mut lua = IndexMap::new();
        let mut subtitles = IndexMap::new();
        let mut atlas1 = None;
        let mut atlas2 = None;
        let mut level = None;
        for info in info.block_headers.iter(){
            let data = &src[info.offset.into() as usize.. (info.offset.into() + info.size.into()) as usize];
            match info.key.val.into() {
                KEY_LEVEL => {
                    level.replace(GameObjsRef::from_data(data)?);
                },
                KEY_ATLAS1 => {
                    atlas1.replace(AtlasUVRef::from_data(data)?);
                },
                KEY_ATLAS2 => {
                    atlas2.replace(AtlasUVRef::from_data(data)?);
                },
                key => match get_str(&key) {
                    Some(x) if x.ends_with(".lua") => {lua.insert(key, LuaRef::from_data(data));},
                    Some(x) if x.ends_with(".ssa") => {subtitles.insert(key, SSARef::from_data(data)?);},
                    Some(x) if x.ends_with(".csv") || x.ends_with(".txt") || x.ends_with(".dat") => {
                        files.insert(key, DataRef::from_data(data));
                    }
                    x => {
                        warn!("Unknown block type {:?}, {:?}", key, x.map(|x| x.to_string()).unwrap_or_default());
                        files.insert(key, DataRef::from_data(data));
                    }
                }
            }
        }
        let level = level.ok_or(anyhow!("sub_blocks1 missing level block"))?;
        Ok(Self {
            info,
            files: files.into(),
            lua: lua.into(),
            subtitles: subtitles.into(),
            atlas1: atlas1.into(),
            atlas2: atlas2.into(),
            level
        })
    }
}

/*
pub struct SubBlocks1_XE_<'a> {
    pub files: IndexMap<Crc, Data_XE_<'a>>,
    pub lua: IndexMap<Crc, Lua_XE_<'a>>,
    pub subtitles: IndexMap<Crc, SSA_XE_<'a>>,
    pub atlas1: Option<AtlasUV_XE_<'a>>,
    pub atlas2: Option<AtlasUV_XE_<'a>>,
    pub level: GameObjs_XE_<'a>,
}

impl DumpSubBlocks1_XE_ for SubBlocks1_XE_<'_> {
    fn files_num(&self) -> usize {
        self.files.len()
    }
    fn lua_num(&self) -> usize {
        self.lua.len()
    }
    fn subtitles_num(&self) -> usize {
        self.subtitles.len()
    }
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpData_XE_)> {
        self.files.iter().map(|(k, v)| (k.get(), v))
    }
    fn lua(&self) -> impl Iterator<Item = (u32, &impl DumpData_XE_)> {
        self.lua.iter().map(|(k, v)| (k.get(), v))
    }
    fn subtitles(&self) -> impl Iterator<Item = (u32, &impl DumpSSA_XE_)> {
        self.subtitles.iter().map(|(k, v)| (k.get(), v))
    }
    fn atlas1(&self) -> Option<&impl DumpAtlasUV_XE_> {
        self.atlas1.as_ref()
    }
    fn atlas2(&self) -> Option<&impl DumpAtlasUV_XE_> {
        self.atlas2.as_ref()
    }
    fn level(&self) -> &impl DumpGameObjs_XE_ {
        &self.level
    }
}
*/

pub trait DumpSubBlocks1<T: BaseTypes> {
    fn files_num(&self) -> usize;
    fn lua_num(&self) -> usize;
    fn subtitles_num(&self) -> usize;
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpDataImpl)>;
    fn lua(&self) -> impl Iterator<Item = (u32, &impl DumpDataImpl)>;
    fn subtitles(&self) -> impl Iterator<Item = (u32, &impl DumpSSA<T>)>;
    fn atlas1(&self) -> Option<&impl DumpAtlasUV<T>>;
    fn atlas2(&self) -> Option<&impl DumpAtlasUV<T>>;
    fn level(&self) -> &impl DumpGameObjs<T>;

    fn blocks_num(&self) -> usize {
        self.files_num() + self.lua_num() + self.subtitles_num() + 1
    }

    fn size(&self) -> (usize, TypeInfos) {
        let atlas1 = self.atlas1();
        let atlas2 = self.atlas2();
        let mut size = std::mem::size_of::<SubBlocksHeader<T>>()
            + std::mem::size_of::<SubBlocksBlockHeader<T>>() * (self.blocks_num() + atlas1.is_some().then_some(1).unwrap_or_default() + atlas2.is_some().then_some(1).unwrap_or_default());
        size = align_offset(size, 16);
        for (key, file) in self.files() {
            size = align_offset(size + file.size() + 1, 16);
            debug!("size after file {} {}", key, size);
        }
        for (key, lua) in self.lua() {
            size = align_offset(size + lua.size() + 1, 16);
            debug!("size after lua {} {}", key, size);
        }
        for (key, subtitle) in self.subtitles() {
            size = align_offset(size + subtitle.size() + 1, 16);
            debug!("size after ssa {} {}", key, size);
        }
        if let Some(atlas) = atlas1 {
            size = align_offset(size + atlas.size() + 1, 16);
            debug!("size after atlas1 {}", size);
        }
        if let Some(atlas) = atlas2 {
            size = align_offset(size + atlas.size() + 1, 16);
            debug!("size after atlas2 {}", size);
        }
        let (s, ty) = self.level().size();
        size = align_offset(size + s, 16);
        (size, ty)
    }
    fn dump_into(&self, dst: &mut DumpSlice, type_infos: &TypeInfos) -> Result<()> {
        let start = dst.offset;
        let atlas1 = self.atlas1();
        let atlas2 = self.atlas2();
        let header = SubBlocksHeader::<T>::mut_from_data(dst).context("header")?;
        let block_headers = SubBlocksBlockHeader::<T>::mut_slice_from_data(dst, self.blocks_num() + atlas1.is_some().then_some(1).unwrap_or_default() + atlas2.is_some().then_some(1).unwrap_or_default()).context("block_headers")?;
        header.block_num = (block_headers.len() as u32).into();
        dst.align(16)?;
        let mut i = 0;
        let start_ = dst.offset;
        for (key, file) in self.files() {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = ((dst.offset - start) as u32).into();
            info.key = Crc::new(key.into());
            let start = dst.offset;
            file.dump_into(dst)?;
            info.size = ((dst.offset - start) as u32).into();
            dst.split(1)?;
            dst.align(16)?;
            debug!("offset after file {} {}", key, dst.offset - start_);
        }
        for (key, lua) in self.lua() {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = ((dst.offset - start) as u32).into();
            info.key = Crc::new(key.into());
            let start = dst.offset;
            lua.dump_into(dst)?;
            info.size = ((dst.offset - start) as u32).into();
            dst.split(1)?;
            dst.align(16)?;
            debug!("offset after lua {} {}", key, dst.offset - start_);
        }
        for (key, subtitle) in self.subtitles() {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = ((dst.offset - start) as u32).into();
            info.key = Crc::new(key.into());
            let start = dst.offset;
            subtitle.dump_into(dst)?;
            info.size = ((dst.offset - start) as u32).into();
            dst.split(1)?;
            dst.align(16)?;
            debug!("offset after ssa {} {}", key, dst.offset - start_);
        }
        if let Some(atlas) = atlas1 {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = ((dst.offset - start) as u32).into();
            info.key = Crc::new(keys::KEY_ATLAS1.into());
            let start = dst.offset;
            atlas.dump_into(dst)?;
            info.size = ((dst.offset - start) as u32).into();
            dst.split(1)?;
            dst.align(16)?;
            debug!("offset after atlas1 {}", dst.offset - start_);
        }
        if let Some(atlas) = atlas2 {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = ((dst.offset - start) as u32).into();
            info.key = Crc::new(keys::KEY_ATLAS2.into());
            let start = dst.offset;
            atlas.dump_into(dst)?;
            info.size = ((dst.offset - start) as u32).into();
            dst.split(1)?;
            dst.align(16)?;
            debug!("offset after atlas2 {}", dst.offset - start_);
        }
        let info = &mut block_headers[i];
        info.offset = ((dst.offset - start) as u32).into();
        info.key = Crc::new(keys::KEY_LEVEL.into());
        let start = dst.offset;
        self.level().dump_into(dst, type_infos)?;
        info.size = ((dst.offset - start) as u32).into();
        dst.align(16)?;
        
        Ok(())
    }
}

impl<T: BaseTypes> DumpSubBlocks1<T> for SubBlocks1Ref<'_, T> {
    fn files_num(&self) -> usize {
        self.files.len()
    }
    fn lua_num(&self) -> usize {
        self.lua.len()
    }
    fn subtitles_num(&self) -> usize {
        self.subtitles.len()
    }
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpDataImpl)> {
        self.files.iter().map(|(&k, v)| (k, v))
    }
    fn lua(&self) -> impl Iterator<Item = (u32, &impl DumpDataImpl)> {
        self.lua.iter().map(|(&k, v)| (k, v))
    }
    fn subtitles(&self) -> impl Iterator<Item = (u32, &impl DumpSSA<T>)> {
        self.subtitles.iter().map(|(&k, v)| (k, v))
    }
    fn atlas1(&self) -> Option<&impl DumpAtlasUV<T>> {
        self.atlas1.as_ref()
    }
    fn atlas2(&self) -> Option<&impl DumpAtlasUV<T>> {
        self.atlas2.as_ref()
    }
    fn level(&self) -> &impl DumpGameObjs<T> {
        &self.level
    }
}

#[derive_pod]
pub struct AtlasUVVal<T: BaseTypes> {
    pub key: Crc<T>,
    pub vals: Vector4<T>,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct AtlasUVRef<'a, T: BaseTypes> {
    pub vals: ref_slice<'a, AtlasUVVal<T>>
}
impl<'a, T: BaseTypes> AtlasUVRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        if src.len() % std::mem::size_of::<AtlasUVVal<T>>() != 0 {
            return Err(anyhow!("Invalid UV Atlas size {}", src.len()));
        }
        let num = src.len() / std::mem::size_of::<AtlasUVVal<T>>();
        let vals = AtlasUVVal::slice_from_data(&src, num).context("vals")?.into();
        Ok(Self { vals })
    }
}

#[derive(Debug, Clone)]
pub struct AtlasUV {
    pub vals: Vec<AtlasUVVal<NE>>,
}

/*
#[enum_dispatch(DumpAtlasUV_XE_)]
pub enum AtlasUV_XE_<'a> {
    Ref(AtlasUVRef_XE_<'a>),
    Owned(AtlasUV)
}
*/

impl<T: BaseTypes> From<&AtlasUVRef<'_, T>> for AtlasUV
where
    AtlasUVVal<NE>: From<AtlasUVVal<T>>,
{
    fn from(val: &AtlasUVRef<T>) -> Self {
        Self {
            vals: val.vals.iter().map(|&x| x.into()).collect(),
        }
    }
}

#[enum_dispatch]
pub trait DumpAtlasUV<T: BaseTypes> {
    fn vals_len(&self) -> usize;
    fn write_vals(&self, vals: &mut [AtlasUVVal<T>]) -> Result<()>;

    fn size(&self) -> usize {
        self.vals_len() * std::mem::size_of::<AtlasUVVal<T>>()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let vals = AtlasUVVal::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
}

impl<T: BaseTypes> DumpAtlasUV<T> for AtlasUVRef<'_, T> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [AtlasUVVal<T>]) -> Result<()> {
        vals.copy_from_slice(&self.vals[..]);
        Ok(())
    }
}

impl<T: BaseTypes> DumpAtlasUV<T> for AtlasUV
where
    AtlasUVVal<T>: From<AtlasUVVal<NE>>,
{
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [AtlasUVVal<T>]) -> Result<()> {
        for (&src, dst) in self.vals.iter().zip(vals) {
            *dst = src.into();
        }
        Ok(())
    }
}

#[derive_pod]
pub struct SSAVal<T: BaseTypes> {
    pub t_start: T::f32,
    pub t_end: T::f32,
    pub unk_2: T::u32,
    pub unk_3: T::u32,
    pub off: T::u32,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SSARef<'a, T: BaseTypes> {
    pub vals: ref_slice<'a, SSAVal<T>>,
    pub strings: slice<ref_slice<'a, T::u16>>,
}

impl<'a, T: BaseTypes> SSARef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let &n = T::u32::from_data(src).context("n")?;
        let vals = SSAVal::<T>::slice_from_data(&src[4..], n.into() as usize).context("vals")?;
        let strings = (0..n.into() as usize)
            .map(|i| {
                let start = vals[i].off.into() as usize;
                let end = if i == n.into() as usize - 1 {
                    src.len()
                } else {
                    vals[i + 1].off.into() as usize
                };
                Ok(
                    T::u16::slice_from_data(&src[start..], (end - start) / 2)
                        .with_context(|| format!("string {}", i))?
                        .into(),
                )
            })
            .collect::<Result<Vec<_>>>()?.into_boxed_slice();

        Ok(Self {
            vals: vals.into(),
            strings: strings.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SSA {
    pub vals: Vec<SSAVal<NE>>,
    pub strings: Vec<String>,
}

impl<T: BaseTypes> From<&SSARef<'_, T>> for SSA
where
    SSAVal<NE>: From<SSAVal<T>>,
{
    fn from(val: &SSARef<T>) -> Self {
        Self {
            vals: val.vals.iter().map(|&x| x.into()).collect(),
            strings: val
                .strings
                .iter()
                .map(|x| {
                    String::from_utf16(
                        x.as_ref()
                            .iter()
                            .map(|&y| y.into())
                            .collect::<Vec<_>>()
                            .as_ref(),
                    )
                    .unwrap()
                })
                .collect(),
        }
    }
}

/*
#[enum_dispatch(DumpSSA_XE_)]
pub enum SSA_XE_<'a> {
    Ref(SSARef_XE_<'a>),
    Owned(SSA)
}
*/

pub trait DumpSSAImpl<T: BaseTypes> {
    fn vals_len(&self) -> usize;
    fn strings(&self) -> impl Iterator<Item = &impl DumpString<T>>;
    fn write_vals(&self, vals: &mut [SSAVal<T>]) -> Result<()>;
}

#[enum_dispatch]
pub trait DumpSSA<T: BaseTypes> {
    fn size(&self) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
}

impl<T: BaseTypes, I: DumpSSAImpl<T>> DumpSSA<T> for I {
    fn size(&self) -> usize {
        std::mem::size_of::<T::u32>()
            + std::mem::size_of::<SSAVal<T>>() * self.vals_len()
            + self.strings().map(|x| x.string_size()).sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let start = dst.offset;
        let n = T::u32::mut_from_data(dst).context("n")?;
        let vals = SSAVal::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
        *n = (vals.len() as u32).into();
        self.write_vals(vals).context("write vals")?;

        for (i, (string, val)) in self.strings().zip(vals).enumerate() {
            val.off = ((dst.offset - start) as u32).into();
            string
                .dump_string(dst)
                .with_context(|| format!("string {}", i))?;
        }
        Ok(())
    }
}

impl<T: BaseTypes> DumpSSAImpl<T> for SSARef<'_, T> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn strings(&self) -> impl Iterator<Item = &impl DumpString<T>> {
        self.strings.iter()
    }
    fn write_vals(&self, vals: &mut [SSAVal<T>]) -> Result<()> {
        vals.copy_from_slice(&self.vals[..]);
        Ok(())
    }
}

impl<T: BaseTypes> DumpSSAImpl<T> for SSA
where
    SSAVal<T>: From<SSAVal<NE>>,
{
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn strings(&self) -> impl Iterator<Item = &impl DumpString<T>> {
        self.strings.iter()
    }
    fn write_vals(&self, vals: &mut [SSAVal<T>]) -> Result<()> {
        for (&src, dst) in self.vals.iter().zip(vals) {
            *dst = src.into()
        }
        Ok(())
    }
}

pub type LuaRef<'a> = DataRef<'a>;
pub type Lua = Data;
