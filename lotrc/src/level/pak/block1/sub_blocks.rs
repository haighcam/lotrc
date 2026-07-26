use crate::types::{get_str, hash_string, Crc, DumpData, DumpSlice, RefFromData, Vector4, OrderedData, align_offset, ref_slice, slice};
#[make_endian]
use crate::{
    level::pak::block1::{
        gameobjs::{DumpGameObjs_XE_, GameObjsRef_XE_, GameObjs_XE_},
    },
    types::{
        Crc_XE_, u16_XE_, u32_XE_, f32_XE_, Vector4_XE_,
        sub_blocks::{
            SubBlocksInfoRef_XE_, DataRef_XE_, Data_XE_, DumpString_XE_, DumpData_XE_, SubBlocksBlockHeader_XE_,
            SubBlocksHeader_XE_,
        },
    },
};
use anyhow::{anyhow, Context, Result};
use crate::types::sub_blocks::{
    Data,
};
use crate::level::pak::block1::gameobjs::TypeInfos;
use indexmap::IndexMap;
use log::warn;
use lotrc_proc::{make_endian, derive_ordered_data};
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

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SubBlocks1Ref_XE_<'a> {
    pub info: SubBlocksInfoRef_XE_<'a>,
    pub files: IndexMap<u32, DataRef_XE_<'a>>,
    pub lua: IndexMap<u32, LuaRef_XE_<'a>>,
    pub subtitles: IndexMap<u32, SSARef_XE_<'a>>,
    pub atlas1: Option<AtlasUVRef_XE_<'a>>,
    pub atlas2: Option<AtlasUVRef_XE_<'a>>,
    pub level: GameObjsRef_XE_<'a>
}

#[make_endian]
impl Default for SubBlocks1Ref_XE_<'_> {
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

#[make_endian]
impl<'a> SubBlocks1Ref_XE_<'a> {
    // TODO should use the pak string to get key names
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        use keys::*;
        let info = SubBlocksInfoRef_XE_::from_data(src)?;
        let mut files = IndexMap::new();
        let mut lua = IndexMap::new();
        let mut subtitles = IndexMap::new();
        let mut atlas1 = None;
        let mut atlas2 = None;
        let mut level = None;
        for info in info.block_headers.iter(){
            let data = &src[info.offset.conv() .. (info.offset + info.size) as usize];
            match info.key.conv() {
                KEY_LEVEL => {
                    level.replace(GameObjsRef_XE_::from_data(data)?);
                },
                KEY_ATLAS1 => {
                    atlas1.replace(AtlasUVRef_XE_::from_data(data)?);
                },
                KEY_ATLAS2 => {
                    atlas2.replace(AtlasUVRef_XE_::from_data(data)?);
                },
                key => match get_str(&key) {
                    Some(x) if x.ends_with(".lua") => {lua.insert(key, LuaRef_XE_::from_data(data));},
                    Some(x) if x.ends_with(".ssa") => {subtitles.insert(key, SSARef_XE_::from_data(data)?);},
                    Some(x) if x.ends_with(".csv") || x.ends_with(".txt") || x.ends_with(".dat") => {
                        files.insert(key, DataRef_XE_::from_data(data));
                    }
                    x => {
                        warn!("Unknown block type {:?}, {:?}", key, x.map(|x| x.to_string()).unwrap_or_default());
                        files.insert(key, DataRef_XE_::from_data(data));
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

#[make_endian]
pub struct SubBlocks1_XE_<'a> {
    pub files: IndexMap<Crc, Data_XE_<'a>>,
    pub lua: IndexMap<Crc, Lua_XE_<'a>>,
    pub subtitles: IndexMap<Crc, SSA_XE_<'a>>,
    pub atlas1: Option<AtlasUV_XE_<'a>>,
    pub atlas2: Option<AtlasUV_XE_<'a>>,
    pub level: GameObjs_XE_<'a>,
}

#[make_endian]
pub trait DumpSubBlocks1_XE_ {
    fn files_num(&self) -> usize;
    fn lua_num(&self) -> usize;
    fn subtitles_num(&self) -> usize;
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpData_XE_)>;
    fn lua(&self) -> impl Iterator<Item = (u32, &impl DumpData_XE_)>;
    fn subtitles(&self) -> impl Iterator<Item = (u32, &impl DumpSSA_XE_)>;
    fn atlas1(&self) -> Option<&impl DumpAtlasUV_XE_>;
    fn atlas2(&self) -> Option<&impl DumpAtlasUV_XE_>;
    fn level(&self) -> &impl DumpGameObjs_XE_;

    fn blocks_num(&self) -> usize {
        self.files_num() + self.lua_num() + self.subtitles_num() + 1
    }

    fn size(&self) -> (usize, TypeInfos) {
        let atlas1 = self.atlas1();
        let atlas2 = self.atlas2();
        let mut size = SubBlocksHeader_XE_::size_of()
            + SubBlocksBlockHeader_XE_::size_of() * (self.blocks_num() + atlas1.is_some().then_some(1).unwrap_or_default() + atlas2.is_some().then_some(1).unwrap_or_default());
        size = align_offset(size, 16);
        for (_, file) in self.files() {
            size = align_offset(size + file.size() + 1, 16)
        }
        for (_, lua) in self.lua() {
            size = align_offset(size + lua.size() + 1, 16)
        }
        for (_, subtitle) in self.subtitles() {
            size = align_offset(size + subtitle.size() + 1, 16)
        }
        if let Some(atlas) = atlas1 {
            size = align_offset(size + atlas.size() + 1, 16)
        }
        if let Some(atlas) = atlas2 {
            size = align_offset(size + atlas.size() + 1, 16)
        }
        let (s, ty) = self.level().size();
        size = align_offset(size + s, 16);
        (size, ty)
    }
    fn dump_into(&self, dst: &mut DumpSlice, type_infos: &TypeInfos) -> Result<()> {
        let start = dst.offset;
        let atlas1 = self.atlas1();
        let atlas2 = self.atlas2();
        let header = SubBlocksHeader_XE_::mut_from_data(dst).context("header")?;
        let block_headers = SubBlocksBlockHeader_XE_::mut_slice_from_data(dst, self.blocks_num() + atlas1.is_some().then_some(1).unwrap_or_default() + atlas2.is_some().then_some(1).unwrap_or_default()).context("block_headers")?;
        header.block_num = block_headers.len().conv();
        dst.align(16)?;
        let mut i = 0;
        for (key, file) in self.files() {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = (dst.offset - start).conv();
            info.key = key.conv();
            let start = dst.offset;
            file.dump_into(dst)?;
            info.size = (dst.offset - start).conv();
            dst.split(1)?;
            dst.align(16)?;
        }
        for (key, lua) in self.lua() {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = (dst.offset - start).conv();
            info.key = key.conv();
            let start = dst.offset;
            lua.dump_into(dst)?;
            info.size = (dst.offset - start).conv();
            dst.split(1)?;
            dst.align(16)?;
        }
        for (key, subtitle) in self.subtitles() {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = (dst.offset - start).conv();
            info.key = key.conv();
            let start = dst.offset;
            subtitle.dump_into(dst)?;
            info.size = (dst.offset - start).conv();
            dst.split(1)?;
            dst.align(16)?;
        }
        if let Some(atlas) = atlas1 {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = (dst.offset - start).conv();
            info.key = keys::KEY_ATLAS1.conv();
            let start = dst.offset;
            atlas.dump_into(dst)?;
            info.size = (dst.offset - start).conv();
            dst.split(1)?;
            dst.align(16)?;
        }
        if let Some(atlas) = atlas2 {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = (dst.offset - start).conv();
            info.key = keys::KEY_ATLAS2.conv();
            let start = dst.offset;
            atlas.dump_into(dst)?;
            info.size = (dst.offset - start).conv();
            dst.split(1)?;
            dst.align(16)?;
        }
        let info = &mut block_headers[i];
        info.offset = (dst.offset - start).conv();
        info.key = keys::KEY_LEVEL.conv();
        let start = dst.offset;
        self.level().dump_into(dst, type_infos)?;
        info.size = (dst.offset - start).conv();
        dst.align(16)?;
        
        Ok(())
    }
}

#[make_endian]
impl DumpSubBlocks1_XE_ for SubBlocks1Ref_XE_<'_> {
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
        self.files.iter().map(|(&k, v)| (k, v))
    }
    fn lua(&self) -> impl Iterator<Item = (u32, &impl DumpData_XE_)> {
        self.lua.iter().map(|(&k, v)| (k, v))
    }
    fn subtitles(&self) -> impl Iterator<Item = (u32, &impl DumpSSA_XE_)> {
        self.subtitles.iter().map(|(&k, v)| (k, v))
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

#[make_endian]
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

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AtlasUVVal_XE_ {
    pub key: Crc_XE_,
    pub vals: Vector4_XE_,
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct AtlasUVRef_XE_<'a> {
    pub vals: ref_slice<'a, AtlasUVVal_XE_>
}
#[make_endian]
impl<'a> AtlasUVRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        if src.len() % std::mem::size_of::<AtlasUVVal_XE_>() != 0 {
            return Err(anyhow!("Invalid UV Atlas size {}", src.len()));
        }
        let num = src.len() / std::mem::size_of::<AtlasUVVal_XE_>();
        let vals = AtlasUVVal_XE_::slice_from_data(&src, num).context("vals")?.into();
        Ok(Self { vals })
    }
}

#[make_endian]
#[derive(Debug, Clone)]
pub struct AtlasUV {
    pub vals: Vec<AtlasUVVal>,
}

#[make_endian]
#[enum_dispatch(DumpAtlasUV_XE_)]
pub enum AtlasUV_XE_<'a> {
    Ref(AtlasUVRef_XE_<'a>),
    Owned(AtlasUV)
}

#[make_endian]
impl From<&AtlasUVRef_XE_<'_>> for AtlasUV {
    fn from(val: &AtlasUVRef_XE_) -> Self {
        Self {
            vals: val.vals.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_endian]
#[enum_dispatch]
pub trait DumpAtlasUV_XE_ {
    fn vals_len(&self) -> usize;
    fn write_vals(&self, vals: &mut [AtlasUVVal_XE_]) -> Result<()>;

    fn size(&self) -> usize {
        self.vals_len() * AtlasUVVal_XE_::size_of()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let vals = AtlasUVVal_XE_::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
}

#[make_endian]
impl DumpAtlasUV_XE_ for AtlasUVRef_XE_<'_> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [AtlasUVVal_XE_]) -> Result<()> {
        vals.write_from(&self.vals[..])
    }
}

#[make_endian]
impl DumpAtlasUV_XE_ for AtlasUV {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [AtlasUVVal_XE_]) -> Result<()> {
        for (src, dst) in self.vals.iter().zip(vals) {
            *dst = src.conv();
        }
        Ok(())
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SSAVal_XE_ {
    pub t_start: f32_XE_,
    pub t_end: f32_XE_,
    pub unk_2: u32_XE_,
    pub unk_3: u32_XE_,
    pub off: u32_XE_,
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SSARef_XE_<'a> {
    pub vals: ref_slice<'a, SSAVal_XE_>,
    pub strings: slice<ref_slice<'a, u16_XE_>>,
}

#[make_endian]
impl<'a> SSARef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let n = u32_XE_::from_data(src).context("n")?;
        let vals =
            SSAVal_XE_::slice_from_data(&src[4..], n.conv()).context("vals")?;
        let strings = (0usize..n.conv())
            .map(|i| {
                let start = vals[i].off.conv();
                let end = if i == n.to_native() as usize - 1 {
                    src.len()
                } else {
                    vals[i + 1].off.conv()
                };
                Ok(
                    u16_XE_::slice_from_data(&src[start..], (end - start) / 2)
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
    pub vals: Vec<SSAVal>,
    pub strings: Vec<String>,
}

#[make_endian]
impl From<&SSARef_XE_<'_>> for SSA {
    fn from(val: &SSARef_XE_) -> Self {
        Self {
            vals: val.vals.iter().map(|x| x.conv()).collect(),
            strings: val
                .strings
                .iter()
                .map(|x| {
                    String::from_utf16(
                        unsafe { x.as_ref() }
                            .iter()
                            .map(|y| y.conv())
                            .collect::<Vec<_>>()
                            .as_ref(),
                    )
                    .unwrap()
                })
                .collect(),
        }
    }
}

#[make_endian]
#[enum_dispatch(DumpSSA_XE_)]
pub enum SSA_XE_<'a> {
    Ref(SSARef_XE_<'a>),
    Owned(SSA)
}

#[make_endian]
pub trait DumpSSAImpl_XE_ {
    fn vals_len(&self) -> usize;
    fn strings(&self) -> impl Iterator<Item = &impl DumpString_XE_>;
    fn write_vals(&self, vals: &mut [SSAVal_XE_]) -> Result<()>;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpSSA_XE_ {
    fn size(&self) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
}

#[make_endian]
impl<T: DumpSSAImpl_XE_> DumpSSA_XE_ for T {
    fn size(&self) -> usize {
        u32_XE_::size_of()
            + SSAVal_XE_::size_of() * self.vals_len()
            + self.strings().map(|x| x.string_size()).sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let start = dst.offset;
        let n = u32_XE_::mut_from_data(dst).context("n")?;
        let vals = SSAVal_XE_::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
        *n = vals.len().conv();
        self.write_vals(vals).context("write vals")?;

        for (i, (string, val)) in self.strings().zip(vals).enumerate() {
            val.off = (dst.offset - start).conv();
            string
                .dump_string(dst)
                .with_context(|| format!("string {}", i))?;
        }
        Ok(())
    }
}

#[make_endian]
impl DumpSSAImpl_XE_ for SSARef_XE_<'_> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn strings(&self) -> impl Iterator<Item = &impl DumpString_XE_> {
        self.strings.iter()
    }
    fn write_vals(&self, vals: &mut [SSAVal_XE_]) -> Result<()> {
        vals.write_from(&self.vals[..])
    }
}

#[make_endian]
impl DumpSSAImpl_XE_ for SSA {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn strings(&self) -> impl Iterator<Item = &impl DumpString_XE_> {
        self.strings.iter()
    }
    fn write_vals(&self, vals: &mut [SSAVal_XE_]) -> Result<()> {
        for (src, dst) in self.vals.iter().zip(vals) {
            *dst = src.conv()
        }
        Ok(())
    }
}

#[make_endian]
pub type LuaRef_XE_<'a> = DataRef_XE_<'a>;
pub type Lua = Data;
#[make_endian]
pub type Lua_XE_<'a> = Data_XE_<'a>;
