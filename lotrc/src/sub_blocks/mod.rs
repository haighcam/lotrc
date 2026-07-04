use crate::types::GetNative;
use crate::types::{get_default_ref, hash_string, Crc, DumpData, DumpSlice, RefFromData, OrderedData, OrderedDataStrict, FFISlice};
use crate::level::pak::block2::PFields;
#[make_platforms]
use crate::{
    types::{CrcVER, u16VER, u32VER},
    level::pak::block2::{PFieldsRefVER, PFieldsVER}
};
use anyhow::{Context, Result};
use lotrc_proc::{make_platforms, OrderedData};
use enum_dispatch::enum_dispatch;

/*
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum SubBlockType {
    Polish = hash_string("Polish".as_bytes(), None),
    German = hash_string("German".as_bytes(), None),
    French = hash_string("French".as_bytes(), None),
    Spanish = hash_string("Spanish".as_bytes(), None),
    Russian = hash_string("Russian".as_bytes(), None),
    Swedish = hash_string("Swedish".as_bytes(), None),
    English = hash_string("English".as_bytes(), None),
    Italian = hash_string("Italian".as_bytes(), None),
    Norwegian = hash_string("Norwegian".as_bytes(), None),
    Atlas1 = hash_string("atlas_1.uv".as_bytes(), None),
    Atlas2 = hash_string("atlas_2.uv".as_bytes(), None),
    Crowd = hash_string("3dCrowd".as_bytes(), None),
    Spray = hash_string("Spray".as_bytes(), None),
    PFields = hash_string("PFields".as_bytes(), None),
    Level = hash_string("Level".as_bytes(), None),
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
*/
/*
#[make_platforms]
#[repr(C, u8)]
pub enum SubBlockRefVER<'a> {
    LangStrings(LangStringsRefVER<'a>),
    Data(DataRefVER<'a>),
    Spray(SprayRefVER<'a>),
    PFields(PFieldsRefVER<'a>),
    Crowd(CrowdRefVER<'a>),
    Level(GameObjsRefVER<'a>),
    AtlasUV(AtlasUVRefVER<'a>),
    Lua(LuaRefVER<'a>),
    SSA(SSARefVER<'a>),
}

#[make_platforms]
impl<'a> SubBlockRefVER<'a> {
    pub fn from_data(src: &'a [u8], key: u32) -> Result<Self> {
        use keys::*;
        Ok(match key {
            KEY_POLISH | KEY_GERMAN | KEY_FRENCH | KEY_SPANISH | KEY_RUSSIAN | KEY_SWEDISH
            | KEY_ENGLISH | KEY_ITALIAN | KEY_NORWEGIAN => {
                Self::LangStrings(LangStringsRefVER::from_data(src)?)
            }
            KEY_SPRAY => Self::Spray(SprayRefVER::from_data(src)?),
            KEY_CROWD => Self::Crowd(CrowdRefVER::from_data(src)?),
            KEY_PFIELDS => Self::PFields(PFieldsRefVER::from_data(src)),
            KEY_LEVEL => Self::Level(GameObjsRefVER::from_data(src)?),
            KEY_ATLAS1 | KEY_ATLAS2 => Self::AtlasUV(AtlasUVRefVER::from_data(src)?),
            _ => match get_str(&key) {
                Some(x) if x.ends_with(".lua") => Self::Lua(LuaRefVER::from_data(src)),l
                Some(x) if x.ends_with(".ssa") => Self::SSA(SSARefVER::from_data(src)?),
                Some(x) if x.ends_with(".csv") || x.ends_with(".txt") || x.ends_with(".dat") => {
                    Self::Data(DataRefVER::from_data(src))
                }
                name => {
                    warn!("Unknown block type {:?}, {:?}", key, name);
                    Self::Data(DataRefVER::from_data(src))
                }
            },
        })
    }
}

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
    fn from_ver(val: &SubBlockRefVER, pfield_infos: &[PFieldInfoVER]) -> Result<Self> {
        Ok(match val {
            SubBlockRefVER::LangStrings(val) => Self::LangStrings(val.try_into()?),
            SubBlockRefVER::Data(val) => Self::Data(val.into()),
            SubBlockRefVER::Spray(val) => Self::Spray(val.into()),
            SubBlockRefVER::Crowd(val) => Self::Crowd(val.into()),
            SubBlockRefVER::PFields(val) => Self::PFields(PFields::from_ver(val, pfield_infos)),
            SubBlockRefVER::Level(val) => Self::Level(val.into()),
            SubBlockRefVER::AtlasUV(val) => Self::AtlasUV(val.into()),
            SubBlockRefVER::Lua(val) => Self::Lua(val.into()),
            SubBlockRefVER::SSA(val) => Self::SSA(val.into()),
        })
    }
}

#[make_platforms]
pub enum SubBlockImplVER<'a> {
    LangStrings(LangStringsVER<'a>),
    Data(DataVER<'a>),
    Spray(SprayVER<'a>),
    PFields(PFieldsVER<'a>),
    Crowd(CrowdVER<'a>),
    Level(GameObjsVER<'a>),
    AtlasUV(AtlasUVVER<'a>),
    Lua(LuaVER<'a>),
    SSA(SSAVER<'a>),

}

#[make_platforms]
#[enum_dispatch(DumpSubBlockVER)]
pub enum SubBlockVER<'a> {
    Ref(SubBlockRefVER<'a>),
    Owned(SubBlockImplVER<'a>)
}

#[make_platforms]
#[enum_dispatch]
pub trait DumpSubBlockVER {
    fn size(&self, type_infos: &mut Option<TypeInfos>) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice, type_infos: &TypeInfos) -> Result<()>;
}

#[make_platforms]
impl DumpSubBlockVER for SubBlockRefVER<'_> {
    fn size(&self, type_infos: &mut Option<TypeInfos>) -> usize {
        match self {
            Self::LangStrings(val) => DumpLangStringsVER::size(val),
            Self::Data(val) => DumpDataVER::size(val),
            Self::Spray(val) => DumpSprayVER::size(val),
            Self::PFields(val) => DumpDataVER::size(val),
            Self::Crowd(val) => DumpCrowdVER::size(val),
            Self::Level(val) => {
                let (size, infos) = DumpGameObjsVER::size(val);
                type_infos.replace(infos);
                size
            }
            Self::AtlasUV(val) => DumpAtlasUVVER::size(val),
            Self::Lua(val) => DumpDataVER::size(val),
            Self::SSA(val) => DumpSSAVER::size(val),
        }
    }

    fn dump_into(&self, dst: &mut DumpSlice, type_infos: &TypeInfos) -> Result<()> {
        match self {
            Self::LangStrings(val) => DumpLangStringsVER::dump_into(val, dst),
            Self::Data(val) => DumpDataVER::dump_into(val, dst),
            Self::Spray(val) => DumpSprayVER::dump_into(val, dst),
            Self::PFields(val) => DumpDataVER::dump_into(val, dst),
            Self::Crowd(val) => DumpCrowdVER::dump_into(val, dst),
            Self::Level(val) => DumpGameObjsVER::dump_into(val, dst, type_infos),
            Self::AtlasUV(val) => DumpAtlasUVVER::dump_into(val, dst),
            Self::Lua(val) => DumpDataVER::dump_into(val, dst),
            Self::SSA(val) => DumpSSAVER::dump_into(val, dst),
        }
    }
}

#[make_platforms]
impl DumpSubBlockVER for SubBlock {
    fn size(&self, type_infos: &mut Option<TypeInfos>) -> usize {
        match self {
            Self::LangStrings(val) => DumpLangStringsVER::size(val),
            Self::Data(val) => DumpDataVER::size(val),
            Self::Spray(val) => DumpSprayVER::size(val),
            Self::PFields(val) => DumpDataVER::size(val),
            Self::Crowd(val) => DumpCrowdVER::size(val),
            Self::Level(val) => {
                let (size, infos) = DumpGameObjsVER::size(val);
                type_infos.replace(infos);
                size
            }
            Self::AtlasUV(val) => DumpAtlasUVVER::size(val),
            Self::Lua(val) => DumpDataVER::size(val),
            Self::SSA(val) => DumpSSAVER::size(val),
        }
    }

    fn dump_into(&self, dst: &mut DumpSlice, type_infos: &TypeInfos) -> Result<()> {
        match self {
            Self::LangStrings(val) => DumpLangStringsVER::dump_into(val, dst),
            Self::Data(val) => DumpDataVER::dump_into(val, dst),
            Self::Spray(val) => DumpSprayVER::dump_into(val, dst),
            Self::PFields(val) => DumpDataVER::dump_into(val, dst),
            Self::Crowd(val) => DumpCrowdVER::dump_into(val, dst),
            Self::Level(val) => DumpGameObjsVER::dump_into(val, dst, type_infos),
            Self::AtlasUV(val) => DumpAtlasUVVER::dump_into(val, dst),
            Self::Lua(val) => DumpDataVER::dump_into(val, dst),
            Self::SSA(val) => DumpSSAVER::dump_into(val, dst),
        }
    }
}

#[make_platforms]
impl DumpSubBlockVER for SubBlockImplVER<'_> {
    fn size(&self, type_infos: &mut Option<TypeInfos>) -> usize {
        match self {
            Self::LangStrings(val) => DumpLangStringsVER::size(val),
            Self::Data(val) => DumpDataVER::size(val),
            Self::Spray(val) => DumpSprayVER::size(val),
            Self::PFields(val) => DumpDataVER::size(val),
            Self::Crowd(val) => DumpCrowdVER::size(val),
            Self::Level(val) => {
                let (size, infos) = DumpGameObjsVER::size(val);
                type_infos.replace(infos);
                size
            }
            Self::AtlasUV(val) => DumpAtlasUVVER::size(val),
            Self::Lua(val) => DumpDataVER::size(val),
            Self::SSA(val) => DumpSSAVER::size(val),
        }
    }

    fn dump_into(&self, dst: &mut DumpSlice, type_infos: &TypeInfos) -> Result<()> {
        match self {
            Self::LangStrings(val) => DumpLangStringsVER::dump_into(val, dst),
            Self::Data(val) => DumpDataVER::dump_into(val, dst),
            Self::Spray(val) => DumpSprayVER::dump_into(val, dst),
            Self::PFields(val) => DumpDataVER::dump_into(val, dst),
            Self::Crowd(val) => DumpCrowdVER::dump_into(val, dst),
            Self::Level(val) => DumpGameObjsVER::dump_into(val, dst, type_infos),
            Self::AtlasUV(val) => DumpAtlasUVVER::dump_into(val, dst),
            Self::Lua(val) => DumpDataVER::dump_into(val, dst),
            Self::SSA(val) => DumpSSAVER::dump_into(val, dst),
        }
    }
}
*/

/*
#[make_platforms]
#[repr(C)]
pub struct SubBlocksRefVER<'a> {
    pub header: &'a SubBlocksHeaderVER,
    pub block_headers: FFISlice<'a, SubBlocksBlockHeaderVER>,
    pub blocks: Map<u32, SubBlockRefVER<'a>>
}

#[make_platforms]
impl Default for SubBlocksRefVER<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            block_headers: FFISlice::default(),
            blocks: IndexMap::default().into()
        }
    }
}

#[make_platforms]
impl<'a> SubBlocksRefVER<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = SubBlocksHeaderVER::from_data(&src[offset..]).context("header")?;
        offset += header.size();
        let block_headers = SubBlocksBlockHeaderVER::slice_from_data(
            &src[offset..],
            header.block_num.get() as usize,
        )
        .context("block_headers")?;
        let mut blocks = IndexMap::<u32, SubBlockRefVER>::with_capacity(block_headers.len());
        for info in block_headers {
            blocks.insert(
                info.key.get(), 
                SubBlockRefVER::from_data(
                    &src[info.offset.get() as usize..(info.offset.get() + info.size.get()) as usize],
                    info.key.get(),
                )
                .with_context(|| {
                    format!(
                        "sub_blocks {}",
                        get_str(&info.key.get())
                            .map(|x| x.to_string())
                            .unwrap_or_else(|| format!("id: {}", info.key.get()))
                    )
                })?
            );
        }

        Ok(Self {
            header: header,
            block_headers: block_headers.into(),
            blocks: blocks.into(),
        })
        
    }
}

#[derive(Debug, Clone)]
pub struct SubBlocks {
    pub blocks: IndexMap<Crc, SubBlock>,
}

impl SubBlocks {
    #[make_platforms]
    pub fn from_ver(val: &SubBlocksRefVER, pfield_infos: &[PFieldInfoVER]) -> Result<Self> {
        Ok(Self {
            blocks: val
                .blocks
                .iter()
                .map(|(k, block)| Ok(((*k).into(), SubBlock::from_ver(block, pfield_infos)?)))
                .collect::<Result<_>>()?,
        })
    }
}

#[make_platforms]
pub struct SubBlocksImplVER<'a> {
    pub blocks: IndexMap<Crc, SubBlockVER<'a>>
}

#[make_platforms]
#[enum_dispatch(DumpSubBlocksVER)]
pub enum SubBlocksVER<'a> {
    Ref(SubBlocksRefVER<'a>),
    Owned(SubBlocksImplVER<'a>)
}

#[make_platforms]
pub trait DumpSubBlocksVER {
    fn blocks_num(&self) -> usize;
    fn blocks(&self) -> impl Iterator<Item = &impl DumpSubBlockVER>;
    fn write_names<'a>(&self, names: impl Iterator<Item = &'a mut CrcVER>);
    
    fn level_name(&self) -> Result<u32>;

    fn size(&self, type_infos: &mut Option<TypeInfos>) -> usize {
        let mut size = SubBlocksHeaderVER::size_of()
            + SubBlocksBlockHeaderVER::size_of() * self.blocks_num();
        size = align_offset(size, 16);
        for block in self.blocks() {
            size = align_offset(size + block.size(type_infos), 16);
        }
        size
    }
    fn dump_into(&self, dst: &mut DumpSlice, type_infos: &TypeInfos) -> Result<()> {
        let start = dst.offset;
        let header = SubBlocksHeaderVER::mut_from_data(dst).context("header")?;
        let block_headers = SubBlocksBlockHeaderVER::mut_slice_from_data(dst, self.blocks_num())
            .context("block_headers")?;
        header.block_num = block_headers.len().conv();
        dst.align(16);
        self.write_names(block_headers.iter_mut().map(|x| &mut x.key));
        for (i, (block, info)) in self.blocks().zip(block_headers).enumerate() {
            info.offset = (dst.offset - start).conv();
            let start = dst.offset;
            block
                .dump_into(dst, type_infos)
                .with_context(|| format!("block {}", i))?;
            info.size = (dst.offset - start).conv();
            dst.align(16);
        }

        Ok(())
    }
}

#[make_platforms]
impl DumpSubBlocksVER for SubBlocksRefVER<'_> {
    fn blocks_num(&self) -> usize {
        self.blocks.len()
    }
    fn blocks(&self) -> impl Iterator<Item = &impl DumpSubBlockVER> {
        self.blocks.values()
    }
    fn write_names<'a>(&self, names: impl Iterator<Item = &'a mut CrcVER>) {
        for (src, dst) in self.blocks.keys().zip(names) {
            *dst = src.conv();
        }
    }
    fn level_name(&self) -> Result<u32> {
        let level = self.blocks
            .get(&hash_string(b"level", None))
            .ok_or(anyhow!("level block missing"))
            .and_then(|x| match x {
                SubBlockRefVER::Level(val) => Ok(val),
                _ => Err(anyhow!("level block wrong format"))
            })?;
        let name = level.objs.values()
            .find(|v| v.header.key.get() == hash_string(b"templateLevel", None))
            .ok_or(anyhow!("templateLevel not found"))?
            .fields.get(&hash_string(b"name", None))
            .ok_or(anyhow!("templateLevel missing name field"))
            .and_then(|field| if let BaseTypeRefVER::Crc(val) = field {
                Ok(val.get())
            } else {
                Err(anyhow!("templateObject name field is not a crc"))
            })?;
        Ok(name)

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
            *dst = src.conv();
        }
    }
    fn level_name(&self) -> Result<u32> {
        let level = self.blocks
            .get(&Crc::new(hash_string(b"level", None)))
            .ok_or(anyhow!("level block missing"))
            .and_then(|x| match x {
                SubBlock::Level(val) => Ok(val),
                _ => Err(anyhow!("level block wrong format"))
            })?;
        let name = level.objs.iter()
            .find(|(k, _)| k.get() == hash_string(b"templateLevel", None))
            .ok_or(anyhow!("templateLevel not found"))?.1
            .fields.get(&Crc::new(hash_string(b"name", None)))
            .ok_or(anyhow!("templateLevel missing name field"))
            .and_then(|field| if let BaseType::Crc(val) = field {
                Ok(val.get())
            } else {
                Err(anyhow!("templateObject name field is not a crc"))
            })?;
        Ok(name)
    }
}

#[make_platforms]
impl DumpSubBlocksVER for SubBlocksImplVER<'_> {
    fn blocks_num(&self) -> usize {
        self.blocks.len()
    }
    fn blocks(&self) -> impl Iterator<Item = &impl DumpSubBlockVER> {
        self.blocks.values()
    }
    fn write_names<'a>(&self, names: impl Iterator<Item = &'a mut CrcVER>) {
        for (src, dst) in self.blocks.keys().zip(names) {
            *dst = src.conv();
        }
    }
    fn level_name(&self) -> Result<u32> {
        match self.blocks
            .get(&Crc::new(hash_string(b"level", None)))
            .ok_or(anyhow!("level block missing"))? 
        {
            SubBlockVER::Ref(SubBlockRefVER::Level(level)) => level.objs
                .values()
                .find(|v| v.header.key.get() == hash_string(b"templateLevel", None))
                .ok_or(anyhow!("templateLevel not found"))?
                .fields.get(&hash_string(b"name", None))
                .ok_or(anyhow!("templateLevel missing name field"))
                .and_then(|field| if let BaseTypeRefVER::Crc(val) = field {
                    Ok(val.get())
                } else {
                    Err(anyhow!("templateObject name field is not a crc"))
                }),
            SubBlockVER::Owned(SubBlockImplVER::Level(GameObjsVER::Ref(level))) => level.objs
                .values()
                .find(|v| v.header.key.get() == hash_string(b"templateLevel", None))
                .ok_or(anyhow!("templateLevel not found"))?
                .fields.get(&hash_string(b"name", None))
                .ok_or(anyhow!("templateLevel missing name field"))
                .and_then(|field| if let BaseTypeRefVER::Crc(val) = field {
                    Ok(val.get())
                } else {
                    Err(anyhow!("templateObject name field is not a crc"))
                }),
            SubBlockVER::Owned(SubBlockImplVER::Level(GameObjsVER::Owned(level))) => match level.objs
                .iter()
                .find(|(k, _)| **k == Crc::new(hash_string(b"templateLevel", None)))
                .ok_or(anyhow!("templateLevel not found"))?.1 {
                    ObjVER::Ref(obj) => obj.fields.get(&hash_string(b"name", None))
                        .ok_or(anyhow!("templateLevel missing name field"))
                        .and_then(|field| if let BaseTypeRefVER::Crc(val) = field {
                            Ok(val.get())
                        } else {
                            Err(anyhow!("templateObject name field is not a crc"))
                        }),
                    ObjVER::Owned(obj) => match obj.fields.get(&Crc::new(hash_string(b"name", None))) {
                            Some(BaseTypeVER::Ref(BaseTypeRefVER::Crc(val))) => Ok(val.get()),
                            Some(BaseTypeVER::Owned(BaseType::Crc(val))) => Ok(val.get()),
                            None => Err(anyhow!("templateLevel missing name field")),
                            _ => Err(anyhow!("templateObject name field is not a crc"))
                        }
                }
            _ => Err(anyhow!("level block wrong format"))
        }
    }
}
*/

