use crate::types::GetNative;
use crate::types::{get_str, hash_string, Crc, DumpData, DumpSlice, RefFromData, Vector4, OrderedData, OrderedDataStrict, align_offset, ref_slice, slice};
#[make_platforms]
use crate::{
    level::pak::block1::{
        gameobjs::{DumpGameObjsVER, GameObjsRefVER, GameObjsVER},
    },
    types::{
        CrcVER, u16VER, u32VER, f32VER, Vector4VER,
        sub_blocks::{
            SubBlocksInfoRefVER, DataRefVER, DataVER, DumpStringVER, DumpDataVER, SubBlocksBlockHeaderVER,
            SubBlocksHeaderVER,
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
use lotrc_proc::{make_platforms, OrderedData};
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

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SubBlocks1RefVER<'a> {
    pub info: SubBlocksInfoRefVER<'a>,
    pub files: IndexMap<u32, DataRefVER<'a>>,
    pub lua: IndexMap<u32, LuaRefVER<'a>>,
    pub subtitles: IndexMap<u32, SSARefVER<'a>>,
    pub atlas1: Option<AtlasUVRefVER<'a>>,
    pub atlas2: Option<AtlasUVRefVER<'a>>,
    pub level: GameObjsRefVER<'a>
}

#[make_platforms]
impl Default for SubBlocks1RefVER<'_> {
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

#[make_platforms]
impl<'a> SubBlocks1RefVER<'a> {
    // TODO should use the pak string to get key names
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        use keys::*;
        let info = SubBlocksInfoRefVER::from_data(src)?;
        let mut files = IndexMap::new();
        let mut lua = IndexMap::new();
        let mut subtitles = IndexMap::new();
        let mut atlas1 = None;
        let mut atlas2 = None;
        let mut level = None;
        for info in info.block_headers.iter(){
            let data = &src[info.offset.get() as usize .. (info.offset + info.size) as usize];
            match info.key.get() {
                KEY_LEVEL => {
                    level.replace(GameObjsRefVER::from_data(data)?);
                },
                KEY_ATLAS1 => {
                    atlas1.replace(AtlasUVRefVER::from_data(data)?);
                },
                KEY_ATLAS2 => {
                    atlas2.replace(AtlasUVRefVER::from_data(data)?);
                },
                key => match get_str(&key) {
                    Some(x) if x.ends_with(".lua") => {lua.insert(key, LuaRefVER::from_data(data));},
                    Some(x) if x.ends_with(".ssa") => {subtitles.insert(key, SSARefVER::from_data(data)?);},
                    Some(x) if x.ends_with(".csv") || x.ends_with(".txt") || x.ends_with(".dat") => {
                        files.insert(key, DataRefVER::from_data(data));
                    }
                    x => {
                        warn!("Unknown block type {:?}, {:?}", key, x.map(|x| x.to_string()).unwrap_or_default());
                        files.insert(key, DataRefVER::from_data(data));
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

#[make_platforms]
pub struct SubBlocks1VER<'a> {
    pub files: IndexMap<Crc, DataVER<'a>>,
    pub lua: IndexMap<Crc, LuaVER<'a>>,
    pub subtitles: IndexMap<Crc, SSAVER<'a>>,
    pub atlas1: Option<AtlasUVVER<'a>>,
    pub atlas2: Option<AtlasUVVER<'a>>,
    pub level: GameObjsVER<'a>,
}

#[make_platforms]
pub trait DumpSubBlocks1VER {
    fn files_num(&self) -> usize;
    fn lua_num(&self) -> usize;
    fn subtitles_num(&self) -> usize;
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpDataVER)>;
    fn lua(&self) -> impl Iterator<Item = (u32, &impl DumpDataVER)>;
    fn subtitles(&self) -> impl Iterator<Item = (u32, &impl DumpSSAVER)>;
    fn atlas1(&self) -> Option<&impl DumpAtlasUVVER>;
    fn atlas2(&self) -> Option<&impl DumpAtlasUVVER>;
    fn level(&self) -> &impl DumpGameObjsVER;

    fn blocks_num(&self) -> usize {
        self.files_num() + self.lua_num() + self.subtitles_num() + 1
    }

    fn size(&self) -> (usize, TypeInfos) {
        let atlas1 = self.atlas1();
        let atlas2 = self.atlas2();
        let mut size = SubBlocksHeaderVER::size_of()
            + SubBlocksBlockHeaderVER::size_of() * (self.blocks_num() + atlas1.is_some().then_some(1).unwrap_or_default() + atlas2.is_some().then_some(1).unwrap_or_default());
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
        let header = SubBlocksHeaderVER::mut_from_data(dst).context("header")?;
        let block_headers = SubBlocksBlockHeaderVER::mut_slice_from_data(dst, self.blocks_num() + atlas1.is_some().then_some(1).unwrap_or_default() + atlas2.is_some().then_some(1).unwrap_or_default()).context("block_headers")?;
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

#[make_platforms]
impl DumpSubBlocks1VER for SubBlocks1RefVER<'_> {
    fn files_num(&self) -> usize {
        self.files.len()
    }
    fn lua_num(&self) -> usize {
        self.lua.len()
    }
    fn subtitles_num(&self) -> usize {
        self.subtitles.len()
    }
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpDataVER)> {
        self.files.iter().map(|(&k, v)| (k, v))
    }
    fn lua(&self) -> impl Iterator<Item = (u32, &impl DumpDataVER)> {
        self.lua.iter().map(|(&k, v)| (k, v))
    }
    fn subtitles(&self) -> impl Iterator<Item = (u32, &impl DumpSSAVER)> {
        self.subtitles.iter().map(|(&k, v)| (k, v))
    }
    fn atlas1(&self) -> Option<&impl DumpAtlasUVVER> {
        self.atlas1.as_ref()
    }
    fn atlas2(&self) -> Option<&impl DumpAtlasUVVER> {
        self.atlas2.as_ref()
    }
    fn level(&self) -> &impl DumpGameObjsVER {
        &self.level
    }
}

#[make_platforms]
impl DumpSubBlocks1VER for SubBlocks1VER<'_> {
    fn files_num(&self) -> usize {
        self.files.len()
    }
    fn lua_num(&self) -> usize {
        self.lua.len()
    }
    fn subtitles_num(&self) -> usize {
        self.subtitles.len()
    }
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpDataVER)> {
        self.files.iter().map(|(k, v)| (k.get(), v))
    }
    fn lua(&self) -> impl Iterator<Item = (u32, &impl DumpDataVER)> {
        self.lua.iter().map(|(k, v)| (k.get(), v))
    }
    fn subtitles(&self) -> impl Iterator<Item = (u32, &impl DumpSSAVER)> {
        self.subtitles.iter().map(|(k, v)| (k.get(), v))
    }
    fn atlas1(&self) -> Option<&impl DumpAtlasUVVER> {
        self.atlas1.as_ref()
    }
    fn atlas2(&self) -> Option<&impl DumpAtlasUVVER> {
        self.atlas2.as_ref()
    }
    fn level(&self) -> &impl DumpGameObjsVER {
        &self.level
    }
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct AtlasUVVal {
    pub key: Crc,
    pub vals: Vector4,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct AtlasUVRefVER<'a> {
    pub vals: ref_slice<'a, AtlasUVValVER>
}
#[make_platforms]
impl<'a> AtlasUVRefVER<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        if src.len() % std::mem::size_of::<AtlasUVValVER>() != 0 {
            return Err(anyhow!("Invalid UV Atlas size {}", src.len()));
        }
        let num = src.len() / std::mem::size_of::<AtlasUVValVER>();
        let vals = AtlasUVValVER::slice_from_data(&src, num).context("vals")?.into();
        Ok(Self { vals })
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct AtlasUV {
    pub vals: Vec<AtlasUVVal>,
}

#[make_platforms]
#[enum_dispatch(DumpAtlasUVVER)]
pub enum AtlasUVVER<'a> {
    Ref(AtlasUVRefVER<'a>),
    Owned(AtlasUV)
}

#[make_platforms]
impl From<&AtlasUVRefVER<'_>> for AtlasUV {
    fn from(val: &AtlasUVRefVER) -> Self {
        Self {
            vals: val.vals.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_platforms]
#[enum_dispatch]
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
impl DumpAtlasUVVER for AtlasUVRefVER<'_> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [AtlasUVValVER]) -> Result<()> {
        vals.write_from(&self.vals[..])
    }
}

#[make_platforms]
impl DumpAtlasUVVER for AtlasUV {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [AtlasUVValVER]) -> Result<()> {
        for (src, dst) in self.vals.iter().zip(vals) {
            *dst = src.conv();
        }
        Ok(())
    }
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct SSAVal {
    pub t_start: f32,
    pub t_end: f32,
    pub unk_2: u32,
    pub unk_3: u32,
    pub off: u32,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SSARefVER<'a> {
    pub vals: ref_slice<'a, SSAValVER>,
    pub strings: slice<ref_slice<'a, u16VER>>,
}

#[make_platforms]
impl<'a> SSARefVER<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let n = u32VER::from_data(src).context("n")?;
        let vals =
            SSAValVER::slice_from_data(&src[4..], n.get() as usize).context("vals")?;
        let strings = (0..n.get() as usize)
            .map(|i| {
                let start = vals[i].off.get() as usize;
                let end = if i == n.get() as usize - 1 {
                    src.len()
                } else {
                    vals[i + 1].off.get() as usize
                };
                Ok(
                    u16VER::slice_from_data(&src[start..], (end - start) / 2)
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

#[make_platforms]
impl From<&SSARefVER<'_>> for SSA {
    fn from(val: &SSARefVER) -> Self {
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

#[make_platforms]
#[enum_dispatch(DumpSSAVER)]
pub enum SSAVER<'a> {
    Ref(SSARefVER<'a>),
    Owned(SSA)
}

#[make_platforms]
pub trait DumpSSAImplVER {
    fn vals_len(&self) -> usize;
    fn strings(&self) -> impl Iterator<Item = &impl DumpStringVER>;
    fn write_vals(&self, vals: &mut [SSAValVER]) -> Result<()>;
}

#[make_platforms]
#[enum_dispatch]
pub trait DumpSSAVER {
    fn size(&self) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
}

#[make_platforms]
impl<T: DumpSSAImplVER> DumpSSAVER for T {
    fn size(&self) -> usize {
        u32VER::size_of()
            + SSAValVER::size_of() * self.vals_len()
            + self.strings().map(|x| x.string_size()).sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let start = dst.offset;
        let n = u32VER::mut_from_data(dst).context("n")?;
        let vals = SSAValVER::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
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

#[make_platforms]
impl DumpSSAImplVER for SSARefVER<'_> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn strings(&self) -> impl Iterator<Item = &impl DumpStringVER> {
        self.strings.iter()
    }
    fn write_vals(&self, vals: &mut [SSAValVER]) -> Result<()> {
        vals.write_from(&self.vals[..])
    }
}

#[make_platforms]
impl DumpSSAImplVER for SSA {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn strings(&self) -> impl Iterator<Item = &impl DumpStringVER> {
        self.strings.iter()
    }
    fn write_vals(&self, vals: &mut [SSAValVER]) -> Result<()> {
        for (src, dst) in self.vals.iter().zip(vals) {
            *dst = src.conv()
        }
        Ok(())
    }
}

#[make_platforms]
pub type LuaRefVER<'a> = DataRefVER<'a>;
pub type Lua = Data;
#[make_platforms]
pub type LuaVER<'a> = DataVER<'a>;
