use anyhow::{Context, Result};
use lotrc_proc::make_endian;
use std::path::Path;
use std::io::Read;
use log::debug;

use crate::{
    types::{AlignedBuf, DumpSlice, RefFromData, EndianTypes},
};

#[make_endian]
use crate::{
    level::{
        bin::{DumpBin_XE_},
        pak::{DumpPak_XE_}
    }
};

pub mod bin;
pub mod model;
pub mod pak;
pub mod radiosity;
pub mod texture;

pub trait LevelFormat: model::ModelTypes + pak::block1::gameobjs::GameObjTypes {
    type PakHeader: RefFromData + pak::PakHeaderTypeTrait;
    type BinHeader: RefFromData + bin::BinHeaderTypeTrait;
    type AssetHandle: RefFromData + bin::AssetHandleTypeTrait;

    type ObjA: PartialEq + RefFromData + pak::block1::objs::ObjATypeTrait;
    type Obj0: PartialEq + RefFromData + pak::block1::objs::Obj0TypeTrait;
    type TextureInfo: PartialEq + RefFromData + texture::TextureInfoTypeTrait;
    type AnimationInfo: PartialEq + RefFromData + pak::animation::AnimationInfoTypeTrait;
    type EffectInfo: PartialEq + RefFromData + pak::block1::objs::EffectInfoTypeTrait;
    type PFieldInfo: PartialEq + RefFromData + pak::block1::objs::PFieldInfoTypeTrait;
    type GFXBlockInfo: PartialEq + RefFromData + pak::block1::objs::GFXBlockInfoTypeTrait;
    type AnimationBlockInfo: PartialEq + RefFromData + pak::animation::AnimationBlockInfoTypeTrait;
    type FoliageInfo: PartialEq + RefFromData + pak::block1::objs::FoliageInfoTypeTrait;
    type FoliageVal: PartialEq + RefFromData + pak::block1::objs::FoliageValTypeTrait;
    type RadiosityValsInfo: PartialEq + RefFromData + radiosity::RadiosityValsInfoTypeTrait;
    

}

pub struct LevelPc;

impl EndianTypes for LevelPc {
    type u16 = crate::types::u16LE;
    type u32 = crate::types::u32LE;
    type u64 = crate::types::u64LE;
    type i16 = crate::types::i16LE;
    type i32 = crate::types::i32LE;
    type f32 = crate::types::f32LE;

    // should be unaligned
    type U16 = crate::types::U16LE;
    type U32 = crate::types::U32LE;
    type I32 = crate::types::I32LE;

    type Crc = crate::types::CrcLE;
    type Vector2 = crate::types::Vector2LE;
    type Vector3 = crate::types::Vector3LE;
    type Vector4 = crate::types::Vector4LE;
    type Matrix4x4 = crate::types::Matrix4x4LE;
}

impl LevelFormat for LevelPc {
    type PakHeader = pak::PakHeaderLE;
    type BinHeader = bin::BinHeaderLE;
    type AssetHandle = bin::AssetHandleLE;

    type ObjA = pak::block1::objs::ObjALE;
    type Obj0 = pak::block1::objs::Obj0LE;
    type TextureInfo = texture::TextureInfoLE;
    type AnimationInfo = pak::animation::AnimationInfoLE;
    type EffectInfo = pak::block1::objs::EffectInfoLE;
    type PFieldInfo = pak::block1::objs::PFieldInfoLE;
    type GFXBlockInfo = pak::block1::objs::GFXBlockInfoLE;
    type AnimationBlockInfo = pak::animation::AnimationBlockInfoLE;
    type FoliageInfo = pak::block1::objs::FoliageInfoLE;
    type FoliageVal = pak::block1::objs::FoliageValLE;
    type RadiosityValsInfo = radiosity::RadiosityValsInfoLE;

    /*
    type List = pak::block1::gameobjs::ListLE; 
    type Weight = crate::types::WeightLE;

    type GameObjsHeader = pak::block1::gameobjs::GameObjsHeaderLE;
    type TypeHeader = pak::block1::gameobjs::TypeHeaderLE;
    type TypeField = pak::block1::gameobjs::TypeFieldLE;
    type ObjHeader = pak::block1::gameobjs::ObjHeaderLE;
    */
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct LevelData {
   pub pak: AlignedBuf,
   pub bin: AlignedBuf
}

#[derive(Copy, Clone)]
#[cfg_attr(feature = "ffi", repr(u8))]
pub enum Version {
    Pc = 0,
    Xbox,
    Ps3, Err
}

impl Version {
    #[inline(always)]
    pub const fn is_xbox(&self) -> bool {
        match self {
            Self::Xbox => true,
            _ => false
        }
    }
}

impl LevelData {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let pak_len = std::fs::metadata(path.with_extension("PAK")).context("pak file")?.len() as usize;
        let bin_len = std::fs::metadata(path.with_extension("BIN")).context("bin file")?.len() as usize;

        let mut pak = AlignedBuf::with_capacity(pak_len);
        {
            let mut file = std::fs::File::open(path.with_extension("PAK")).context("pak file")?;
            let mut off = 0;
            while off < pak_len {
                off += file.read(&mut pak[off..]).context("pak read")?;
            }
        }
        let mut bin = AlignedBuf::with_capacity(bin_len);
        {
            let mut file = std::fs::File::open(path.with_extension("BIN")).context("bin file")?;
            let mut off = 0;
            while off < bin_len {
                off += file.read(&mut bin[off..]).context("bin read")?;
            }
        }
        Ok(Self { pak, bin })
    }
    pub fn version(&self) -> Version {
        if self.bin[0] == 6 {
            Version::Pc
        } else if self.bin[3] == 6 && self.bin[7] == 2 {
            Version::Xbox
        } else if self.bin[3] == 6 && self.bin[7] == 3 {
            Version::Ps3
        } else {
            Version::Err
        }
    }
}

#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct LevelCompressedData<'a> {
    pub pak: pak::PakCompressedData<'a>,
    pub bin: bin::BinCompressedData<'a>,
}

#[make_endian]
#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct LevelRef_XE_<'a> {
    pub pak: pak::PakRef_XE_<'a>,
    pub bin: bin::BinRef_XE_<'a>,
}

#[make_endian]
impl<'a> LevelRef_XE_<'a> {
    pub fn from_data<'b: 'a, 'c: 'b>(src: &'c LevelData, data: &'a mut LevelCompressedData<'b>) -> Result<Self> {
        let bin = bin::BinRef_XE_::from_data(&src.bin[..], &mut data.bin).context("bin")?;
        Ok(Self {
            pak: pak::PakRef_XE_::from_data(&src.pak[..], &mut data.pak, &bin).context("pak")?,
            bin,
        })
    }
}

#[make_endian]
pub trait DumpLevel_XE_ {
    fn pak(&self) -> &impl DumpPak_XE_;
    fn bin(&self) -> &impl DumpBin_XE_;
    fn dump(&self, c: flate2::Compression, version: Version) -> Result<LevelData> {
        let t = std::time::Instant::now();
        let pak = self.pak();
        let bin = self.bin();
        let (pak_size, pak_header, block1, block2, animation_data, model_data, texture_data, rad_data)  = pak.size(c).context("pak preproces")?;
        debug!("pak size in {}", t.elapsed().as_secs_f32());
        let t = std::time::Instant::now();
        let mut pak_data = AlignedBuf::with_capacity(pak_size);
        let mut dump_slice = DumpSlice::from(&mut pak_data[..]);
        pak.dump(&mut dump_slice, pak_header, block1, block2, animation_data).context("dump pak")?;
        debug!("pak dumped in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let bin_size = bin.size(&model_data, &texture_data, rad_data.as_ref());
        let mut bin_data = AlignedBuf::with_capacity(bin_size);
        let mut dump_slice = DumpSlice::from(&mut bin_data[..]);
        bin.dump(&mut dump_slice, &model_data, &texture_data, rad_data.as_ref(), version).context("dump bin")?;
        debug!("bin dumped in {}", t.elapsed().as_secs_f32());

        Ok(LevelData {
            pak: pak_data,
            bin: bin_data
        })
    }
}

#[make_endian]
impl DumpLevel_XE_ for LevelRef_XE_<'_> {
    fn pak(&self) -> &impl DumpPak_XE_ {
        &self.pak
    }
    fn bin(&self) -> &impl DumpBin_XE_ {
        &self.bin
    }
}

/*
#[cfg(feature="ffi")]
#[export(mod_name=LevelData)]
mod data {
    use super::*;
    fn read_data(path: Option<&std::ffi::c_char>) -> Option<NonNull<LevelData>> {
        LevelData::read(unsafe { crate::types::c_str_ptr(path) }).ok().map(|x| Box::leak(Box::new(x)).into())
    }
    fn version<'a>(src: &'a LevelData) -> Version {
        src.version()
    }
    fn free(val: Option<std::ptr::NonNull<LevelData>>) {
        debug!("freeing level data");
        if let Some(val) = val {
            drop(Box::from(val))
        }
    }
}

#[cfg(feature="ffi")]
#[export(mod_name=LevelCompressedData)]
mod compressed_data {
    use super::*;
    fn new<'a>() -> NonNull<LevelCompressedData<'a>> {
        Box::leak(Box::new(LevelCompressedData::default())).into()
    }
    fn free(val: Option<std::ptr::NonNull<LevelCompressedData>>) {
        if let Some(val) = val {
            drop(Box::from(val))
        }
    }
}

#[cfg(feature="ffi")]
#[make_endian]
#[export(mod_name=LevelRef_XE_)]
mod ref_ver {
    use super::*;
    fn from_data<'a>(src: Option<&'a LevelData>, data: Option<&'a mut LevelCompressedData<'a>>) -> Option<NonNull<LevelRef_XE_<'a>>> {
        if src.is_none() || data.is_none() {
            return None;
        }
        LevelRef_XE_::from_data(src.unwrap(), data.unwrap()).ok().map(|x| Box::leak(Box::new(x)).into())
    }
    fn dump(src: Option<&LevelRef_XE_>, compression: u32) -> Option<NonNull<LevelData>> {
        src.and_then(|x| x.dump(flate2::Compression::new(compression)).ok().map(|x| Box::leak(Box::new(x)).into()))
    }
    fn free(val: Option<std::ptr::NonNull<LevelRef_XE_>>) {
        if let Some(val) = val {
            drop(Box::from(val))
        }
    }
}
*/

/*
#[derive(Debug, Clone)]
pub struct Level {
    pub bin_header: BinHeader,
    pub bin_strings: Strings,
    pub pak_header: PakHeader,
    pub pak_strings: Strings,

    pub objas: Vec<pak::objs::ObjA>,
    pub obj0s: Vec<pak::objs::Obj0>,
    pub models: IndexMap<Crc, model::Model>,
    pub textures: IndexMap<Crc, texture::Texture>,
    pub animations: pak::animation::Animations,
    pub foliages: IndexMap<Crc, Vec<pak::objs::Foliage>>,
    pub effects: IndexMap<Crc, sub_blocks::gameobjs::GameObjs>,
    pub gfxs: IndexMap<Crc, Vec<u8>>,

    pub string_keys: StringKeys,
    pub sub_blocks1: sub_blocks::SubBlocks,
    pub sub_blocks2: sub_blocks::SubBlocks,
    pub block2_offsets: Vec<u32>,

    pub radiosity: radiosity::Radiosity,

    pub pak_vals_a: Vec<pak::BlockAVal>,
}
*/

/*
#[make_endian]
fn level_parse_ver(level: &Level_XE_) -> Result<Level> {
    let block1 = level.pak().block1().unwrap();
    let block2 = level.pak().block2().unwrap();

    let objs = block1.objs();
    //types::update_strings(&bin_strings.strings);
    //types::update_strings(&pak_strings.strings);

    Ok(Level {
        bin_header: level.bin().header().conv(),
        bin_strings: level.bin().strings().into(),

        pak_header: level.pak().header().conv(),
        pak_strings: level.pak().strings().into(),

        sub_blocks1: sub_blocks::SubBlocks::from_ver(block1.sub_blocks(), objs.pfield_infos()),
        string_keys: block1.string_keys().into(),
        sub_blocks2: sub_blocks::SubBlocks::from_ver(block2.sub_blocks(), objs.pfield_infos()),
        block2_offsets: block2.offsets().iter().map(|x| x.conv()).collect(),

        objas: objs.objas().iter().map(|x| x.conv()).collect(),
        obj0s: objs.obj0s().iter().map(|x| x.conv()).collect(),

        gfxs: objs
            .gfxs()
            .iter()
            .map(|(k, v)| ((*k).into(), unsafe { v.as_ref() }.to_vec()))
            .collect(),
        effects: objs
            .effects()
            .iter()
            .map(|(k, v)| ((*k).into(), v.into()))
            .collect(),

        models: objs
            .models()
            .iter()
            .map(|(k, v)| ((*k).into(), v.into()))
            .collect(),
        textures: objs
            .textures()
            .iter()
            .map(|(k, v)| ((*k).into(), v.into()))
            .collect(),
        animations: level.pak().animation_blocks().unwrap().into(),
        foliages: objs.foliages().iter().map(|(k, v)| ((*k).into(), v.iter().map(|x| x.into()).collect())).collect(),
        radiosity: radiosity::Radiosity::from_ver(objs.radiosity(), block1).context("radiosity")?, 
        pak_vals_a: level.pak().vals_a().iter().map(|x| x.conv()).collect(),
    })
}
*/

// wrapped level
// - pak
//  - block1: objs + sub_blocks + string_keys
//  - block2: lang_strings + block1_offsets
//  - animation_blocks
// - bin
//  - textures
//  - models
//  - radiosity
