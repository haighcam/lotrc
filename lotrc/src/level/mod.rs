#[cfg(feature = "python")]
use crate::pyobj_ref;
use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use itertools::Itertools;
use lotrc_proc::make_platforms;
#[cfg(not(feature = "python"))]
use lotrc_proc::staticmethod;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;
use std::ptr::NonNull;

use crate::sub_blocks;
use crate::{
    level::{
        bin::BinHeader,
        pak::{PakHeader, PakHeaderPc, objs::InfoCounts},
    },
    types::{hash_string, Crc, RefFromData, StringKeys, Strings},
};

#[make_platforms]
use crate::level::{bin::BinVER, pak::PakVER};

pub mod bin;
pub mod model;
pub mod pak;
pub mod radiosity;
pub mod texture;

#[cfg(feature = "python")]
pub fn init(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "level")?;
    m.add_function(wrap_pyfunction!(parse_level, &m)?)?;
    m.add_class::<Level>()?;
    m.add_submodule(&bin::init(py)?)?;
    m.add_submodule(&model::init(py)?)?;
    m.add_submodule(&pak::init(py)?)?;
    m.add_submodule(&radiosity::init(py)?)?;
    m.add_submodule(&texture::init(py)?)?;
    init_pc(&m)?;
    init_xbox(&m)?;
    init_ps3(&m)?;
    Ok(m)
}
#[cfg(feature = "python")]
#[make_platforms]
pub fn init_ver(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LevelVER>()
}

pub struct CompressedData {
    _ptr: Arc<[u8]>,
    offset: usize,
    size: usize,
    size_decomp: usize
}

#[make_platforms]
pub enum ModelVER {
    Raw(model::ModelVER, CompressedData),
    Parsed(model::Model)
}

#[make_platforms]
pub enum TextureVER {
    Raw {
        _ptr: Arc<[u8]>,
        info: NonNull<texture::TextureInfoVER>, 
        data0: CompressedData, 
        daat1: CompressedData
    },
    Parsed(texture::Texture)
}

#[make_platforms]
pub enum EffectVER {
    Raw(sub_blocks::gameobjs::GameObjsVER),
    Parsed(sub_blocks::gameobjs::GameObjs),
}

#[make_platforms]
pub enum FoliagesVER {
    Raw(Box<[pak::objs::FoliageVER]>),
    Parsed(Vec<pak::objs::Foliage>)
}

#[make_platforms]
pub enum AnimationVER {
    Raw(pak::animation::AnimationVER),
    Parsed(pak::animation::Animation)
}

// store the gamemode + unparsed animation block for raw
// otherwise just the gamemode + guid
#[make_platforms]
pub enum GamemodeVER {
    Raw(Arc<[u8]>, NonNull<pak::animation::AnimationBlockInfoVER>, CompressedData),
    Parsed(pak::animation::AnimationBlockInfo),
}

#[make_platforms]
pub enum GFXVER {
    Raw(Arc<[u8]>, usize, usize),
    Parsed(Vec<u8>)
}

#[make_platforms]
pub enum SubBlockVER {
    Raw(sub_blocks::SubBlockVER),
    Parsed(sub_blocks::SubBlock)
}

#[make_platforms]
pub enum Obj0VER {
    Raw(Arc<[u8]>, NonNull<[pak::objs::Obj0VER]>),
    Parsed(pak::objs::Obj0)
}

#[make_platforms]
pub enum ObjAVER {
    Raw(Arc<[u8]>, NonNull<[pak::objs::ObjAVER]>),
    Parsed(pak::objs::ObjA)
}

#[make_platforms]
pub enum RadiosityVER {
    Raw(radiosity::RadiosityVER),
    Parsed(radiosity::Radiosity),
}

#[make_platforms]
pub enum ValsAVER {
    Raw(),
    Parsed()
}

// raw version has sufficient info to get the parsed version
// to serialize, convert everything to the parsed version
//
// both versions impl the same dump traits to make dumping easier

#[make_platforms]
pub struct LevelRawVER {
    block1: Option<Arc<[u8]>>,
    block2: Option<Arc<[u8]>>,
    pub obj0s: IndexMap<u32, Obj0VER>,
    pub objas: IndexMap<u32, ObjAVER>,
    pub models: IndexMap<u32, ModelVER>,
    pub textures: IndexMap<u32, TextureVER>,
    pub effects: IndexMap<u32, EffectVER>,
    pub foliages: IndexMap<u32, FoliagesVER>,
    pub animations: IndexMap<u32, AnimationVER>,
    pub gamemodes: Vec<GamemodeVER>,
    pub gfxs: IndexMap<u32, GFXVER>,
    pub sub_blocks1: IndexMap<u32, SubBlockVER>,
    pub sub_blocks2: IndexMap<u32, SubBlockVER>,
    pub radiosity: RadiosityVER,
    pub vals_a: IndexMap<u32, ValsAVER>,
}

#[make_platforms]
impl LevelRawVER {
    pub fn parse_full(pak_data: Arc<[u8]>, bin_data: Arc<[u8]>) -> Result<Self> {
        // parse pak skeleton
        // parse bin if needed
        // parse block1 / block2 / animations if needed
        Ok(Self {
            
        })
    }
}


#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "level", get_all))]
#[derive(Debug, Clone)]
pub struct LevelVER {
    pak: PakVER,
    bin: BinVER,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(LevelVER);

#[make_platforms]
unsafe impl Sync for LevelVER {}
#[make_platforms]
unsafe impl Send for LevelVER {}

#[cfg_attr(feature = "python", pyclass(module = "level", get_all))]
pub enum LevelRef {
    PC(LevelPc),
    XBOX(LevelXbox),
    PS3(LevelPs3),
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn parse_level(path: String) -> Result<LevelRef> {
    LevelRef::from_data(path)
}

impl LevelRef {
    pub fn from_data<P: AsRef<Path>>(path: P) -> Result<LevelRef> {
        let t = std::time::Instant::now();
        let path = path.as_ref();
        let pak_data = std::fs::read(path.with_extension("PAK")).context("pak_data")?;
        let bin_data = std::fs::read(path.with_extension("BIN")).context("bin_data")?;

        println!("Files read in {}", t.elapsed().as_secs_f32());

        if bin_data[0] == 6 {
            Ok(Self::PC(
                LevelPc::from_bytes(pak_data, bin_data).context("level_pc")?,
            ))
        } else if bin_data[3] == 6 && bin_data[7] == 2 {
            Ok(Self::XBOX(
                LevelXbox::from_bytes(pak_data, bin_data).context("level_xbox")?,
            ))
        } else if bin_data[3] == 6 && bin_data[7] == 3 {
            Ok(Self::PS3(
                LevelPs3::from_bytes(pak_data, bin_data).context("level_ps3")?,
            ))
        } else {
            Err(anyhow!("unknown level version"))
        }
    }
}

#[make_platforms]
impl LevelVER {
    pub fn new(pak_data: impl Into<Arc<[u8]>>, bin_data: impl Into<Arc<[u8]>>) -> Result<Self> {
        Ok(Self {
            pak: PakVER::from_bytes(pak_data.into()).context("pak")?,
            bin: BinVER::from_bytes(bin_data.into()).context("bin")?,
        })
    }
    pub fn from_bytes(
        pak_data: impl Into<Arc<[u8]>>,
        bin_data: impl Into<Arc<[u8]>>,
    ) -> Result<Self> {
        let t = std::time::Instant::now();
        let mut bin = BinVER::from_bytes(bin_data.into()).context("pak")?;
        bin.parse().context("parse bin")?;
        println!("Bin parsed in {}", t.elapsed().as_secs_f32());
        let t = std::time::Instant::now();
        let mut pak = PakVER::from_bytes(pak_data.into()).context("pak")?;
        pak.parse(&bin).context("parse pak")?;
        println!("Pak parsed in {}", t.elapsed().as_secs_f32());

        Ok(Self { pak, bin })
    }
    pub fn pak(&self) -> &pak::PakVER {
        &self.pak
    }
    pub fn bin(&self) -> &bin::BinVER {
        &self.bin
    }
}

#[cfg_attr(feature = "python", pyclass(module = "level", get_all, set_all))]
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
    pub gfx_blocks: IndexMap<Crc, Vec<u8>>,

    pub string_keys: StringKeys,
    pub sub_blocks1: sub_blocks::SubBlocks,
    pub sub_blocks2: sub_blocks::SubBlocks,
    pub block2_offsets: Vec<u32>,

    pub radiosity: Option<radiosity::Radiosity>,

    pub pak_vals_a: Vec<pak::BlockAVal>,
}

#[make_platforms]
fn level_parse_ver(level: &LevelVER) -> Result<Level> {
    let block1 = level.pak().block1().unwrap();
    let block2 = level.pak().block2().unwrap();

    let objs = block1.objs();
    //types::update_strings(&bin_strings.strings);
    //types::update_strings(&pak_strings.strings);

    let mut foliages: IndexMap<Crc, Vec<_>> =
        IndexMap::with_capacity(level.pak().header().foliage_info_num.get() as usize);
    for foliage in objs.foliages() {
        foliages
            .entry(foliage.info().key.into())
            .or_default()
            .push(foliage.into());
    }
    Ok(Level {
        bin_header: level.bin().header().into(),
        bin_strings: level.bin().strings().into(),

        pak_header: level.pak().header().into(),
        pak_strings: level.pak().strings().into(),

        sub_blocks1: sub_blocks::SubBlocks::from_ver(block1.sub_blocks(), objs.pfield_infos()),
        string_keys: block1.string_keys().into(),
        sub_blocks2: sub_blocks::SubBlocks::from_ver(block2.sub_blocks(), objs.pfield_infos()),
        block2_offsets: block2.offsets().iter().map(|x| x.into()).collect(),

        objas: objs.objas().iter().map(|x| x.into()).collect(),
        obj0s: objs.obj0s().iter().map(|x| x.into()).collect(),

        gfx_blocks: objs
            .gfx_block_infos()
            .iter()
            .zip(block1.objs().gfx_blocks())
            .map(|(k, v)| (k.key.into(), v.to_vec()))
            .collect(),
        effects: objs
            .effect_infos()
            .iter()
            .zip(objs.effects())
            .map(|(k, v)| (k.key.into(), v.into()))
            .collect(),

        models: objs
            .models()
            .iter()
            .map(|model| {
                (
                    model.info().key.into(),
                    model::Model::from_ver(model, level.pak().header(), objs),
                )
            })
            .collect(),
        textures: objs
            .texture_infos()
            .iter()
            .zip(level.bin().textures().unwrap())
            .map(|(info, texture)| (info.key.into(), texture::Texture::from_ver(texture, info)))
            .collect(),
        animations: level.pak().animation_blocks().unwrap().into(),
        foliages,
        radiosity: Some(radiosity::Radiosity::from_ver(
            level.bin().radiosity().unwrap(),
            block1,
        )),

        pak_vals_a: level.pak().vals_a().iter().map(|x| x.into()).collect(),
    })
}

#[make_platforms]
fn level_dump_ver(_level: &Level, _level_raw: &LevelVER) -> (Vec<u8>, Vec<u8>) {
    let pak_data = vec![];
    let bin_data = vec![];

    // dump bin

    // dump pak
    //  dump block1
    //  dump block2
    //

    (pak_data, bin_data)
}

#[cfg_attr(feature = "python", pymethods)]
impl Level {
    #[staticmethod]
    pub fn parse(level: &LevelRef) -> Result<Self> {
        match level {
            LevelRef::PC(val) => level_parse_pc(val),
            LevelRef::XBOX(val) => level_parse_xbox(val),
            LevelRef::PS3(val) => level_parse_ps3(val),
        }
    }

    pub fn dump(&self, level: &LevelRef) -> (Vec<u8>, Vec<u8>) {
        match level {
            LevelRef::PC(val) => level_dump_pc(self, val),
            LevelRef::XBOX(val) => level_dump_xbox(self, val),
            LevelRef::PS3(val) => level_dump_ps3(self, val),
        }
    }

    pub fn dump_pc(&self) -> (Vec<u8>, Vec<u8>) {
        let bin_data = vec![];

        let mut texture_data = vec![];
        let mut seen_textures = HashSet::new();

        fn sort_texture(tex: &&texture::Texture) -> u32 {
            let k = tex.info.key.val;
            if k == 3804089404 {
                0
            } else if k == 4026460901 {
                1
            } else {
                k
            }
        }

        let _texture_infos = self
            .textures
            .values()
            .sorted_by_key(sort_texture)
            .map(|tex| {
                let (data0, data1) = tex.dump();
                if !seen_textures.contains(&tex.info.asset_key.val) {
                    let key_alt: Crc =
                        hash_string("*".as_bytes(), Some(tex.info.asset_key.val)).into();
                    if data1.len() == 0 {
                        texture_data.push(((key_alt, tex.info.asset_type), data1));
                        texture_data
                            .push(((tex.info.asset_key.clone(), tex.info.asset_type), data0));
                    } else {
                        texture_data
                            .push(((tex.info.asset_key.clone(), tex.info.asset_type), data0));
                        texture_data.push(((key_alt, tex.info.asset_type), data1));
                    }
                    seen_textures.insert(tex.info.asset_key.val);
                }
                tex.info.clone()
            })
            .collect::<Vec<_>>();

        // pak stuff
        let mut pak_header = self.pak_header.clone();
        let pak_data = vec![0u8; PakHeaderPc::size_of()];
        let mut info_counts = InfoCounts::default();
        for model in self.models.values() {
            //model.infos_count(&mut info_counts);
        }

        let pfield_infos = if let Some(sub_blocks::SubBlock::PFields(val)) = self
            .sub_blocks2
            .blocks
            .get(&Crc::from(sub_blocks::keys::KEY_PFIELDS))
        {
            val.infos_pc()
        } else {
            vec![]
        };
        /*
        pak_header.version = 1;
        pak_header.shape_info_num = info_counts.shapes;
        pak_header.hk_shape_info_num = info_counts.hk_shapes;
        pak_header.hk_constraint_info_num = info_counts.hk_constraints;
        pak_header.hk_constraint_data_num = info_counts.hk_constraint_datas;
        pak_header.mat1_num = info_counts.mat1s;
        pak_header.mat2_num = info_counts.mat2s;
        pak_header.mat3_num = info_counts.mat3s;
        pak_header.mat4_num = info_counts.mat4s;
        pak_header.mat_extra_num = info_counts.mat_extras;
        pak_header.buffer_info_num = info_counts.buffer_infos;
        pak_header.vbuff_info_num = info_counts.vbuff_infos;
        pak_header.ibuff_info_num = info_counts.ibuff_infos;
        pak_header.model_info_num = self.models.len() as u32;
        pak_header.texture_info_num = self.textures.len() as u32;
        pak_header.effect_info_num = self.effects.len() as u32;
        pak_header.gfx_block_info_num = self.gfx_blocks.len() as u32;
        */
        pak_header.radiosity_vals_info_num = self
            .radiosity
            .as_ref()
            .map(|x| x.vals.len() as u32)
            .unwrap_or(0);
        pak_header.foliage_info_num =
            self.foliages.iter().map(|(_, x)| x.len()).sum::<usize>() as u32;
        pak_header.animation_info_num = self.animations.animations.len() as u32;
        pak_header.pfield_info_num = pfield_infos.len() as u32;

        let mut block1: Vec<u8> = vec![];
        pak_header.obja_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.obja_num as usize
                * std::mem::size_of::<pak::objs::ObjAPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.obj0_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.obj0_num as usize
                * std::mem::size_of::<pak::objs::Obj0Pc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.model_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.model_info_num as usize
                * std::mem::size_of::<model::ModelInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.buffer_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.buffer_info_num as usize
                * std::mem::size_of::<model::data::BufferInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mat1_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.mat1_num as usize
                * std::mem::size_of::<model::mat::Mat1Pc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mat2_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.mat2_num as usize
                * std::mem::size_of::<model::mat::Mat2Pc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mat3_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.mat3_num as usize
                * std::mem::size_of::<model::mat::Mat3Pc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mat4_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.mat4_num as usize
                * std::mem::size_of::<model::mat::Mat4Pc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mat_extra_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.mat_extra_num as usize
                * std::mem::size_of::<model::mat::MatExtraPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.shape_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.shape_info_num as usize
                * std::mem::size_of::<model::shape::ShapeInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.hk_shape_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.hk_shape_info_num as usize
                * std::mem::size_of::<model::shape::HkShapeInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.hk_constraint_data_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.hk_constraint_data_num as usize
                * std::mem::size_of::<
                    model::shape::HkConstraintDataPc,
                >()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.vbuff_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.vbuff_info_num as usize
                * std::mem::size_of::<model::data::VBuffInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.ibuff_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.ibuff_info_num as usize
                * std::mem::size_of::<model::data::IBuffInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.texture_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.texture_info_num as usize
                * std::mem::size_of::<texture::TextureInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.animation_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.animation_info_num as usize
                * std::mem::size_of::<pak::animation::AnimationInfoPc>(
                )
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.hk_constraint_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.hk_constraint_info_num as usize
                * std::mem::size_of::<
                    model::shape::HkConstraintInfoPc,
                >()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.effect_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.effect_info_num as usize
                * std::mem::size_of::<pak::objs::EffectInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.pfield_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.pfield_info_num as usize
                * std::mem::size_of::<pak::objs::PFieldInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.gfx_block_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.gfx_block_info_num as usize
                * std::mem::size_of::<pak::objs::GFXBlockInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.animation_block_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.animation_block_info_num as usize
                * std::mem::size_of::<
                    pak::animation::AnimationBlockInfoPc,
                >()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.foliage_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.foliage_info_num as usize
                * std::mem::size_of::<pak::objs::FoliageInfoPc>()
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.radiosity_vals_info_offset = block1.len() as u32;
        block1.extend(vec![
            0u8;
            pak_header.radiosity_vals_info_num as usize
                * std::mem::size_of::<radiosity::RadiosityValsInfoPc>(
                )
        ]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);

        /*

            // block1 stuff

            let mut infos = DumpInfos {
                header: pak_header.clone(),
                ..Default::default()
            };

            // infos done
            let mut offset = 0;
            let animation_vals = self.animations.iter().sorted_by(|a, b| a.0.key().cmp(&b.0.key())).map(|(key, anim)| {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
                let vals = dump_bytes!(O, anim, offset, &mut infos);
                offset += vals.len();
                (vals, anim.info.gamemodemask)
            }).collect::<Vec<_>>();
            let animations_blocks = (0..self.animation_block_infos.len() as u32).map(|i| {
                let gamemodemask = 1i32 << i;
                animation_vals.iter().filter(|(_, k)| k & gamemodemask != 0).flat_map(|(x, _)| x).cloned().collect::<Vec<_>>()
            }).collect::<Vec<_>>();

            let mut animation_block_infos = self.animation_block_infos.clone();
            for (info, data) in zip(&mut animation_block_infos, animations_blocks) {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
                pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
                let size = data.len();
                let data = CompressedBlock { data }.dump(true).with_context(|| format!("{}", info.key.to_string()))?;
                info.offset = pak_data.len() as u32;
                info.size = size as u32;
                info.size_comp = data.len() as u32;
                if info.size_comp == info.size {
                    info.size_comp = 0;
                }
                pak_data.extend(data);
            }
            info!("animations in {:?}", time.elapsed());
            bar.as_ref().map(|x| { x.inc(1); x.set_message("effects") });
            sub_bar.as_ref().map(|x| { x.set_length(self.effects.len() as u64); x.reset() });

            let effects = self.effects.iter().sorted_by(|a, b| a.0.key().cmp(&b.0.key())).map(|(key, effect)| {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
                let vals = dump_bytes!(O, effect);
                let effect = pak::EffectInfo { key: key.clone(), gamemodemask: effect.gamemodemask, offset: block1.len() as u32, size: vals.len() as u32 };
                block1.extend(vals);
                Ok(effect)
            }).collect::<Result<Vec<_>>>()?;
            info!("effects in {:?}", time.elapsed());
            bar.as_ref().map(|x| { x.inc(1); x.set_message("models & foliages") });
            sub_bar.as_ref().map(|x| { x.set_length((self.models.len() + self.foliages.len()) as u64); x.reset() });

            let key_occluder = hash_string(b"occluder", None);
            let mut normal = vec![];
            let mut collision_road = vec![];
            let mut terrain = vec![];
            let mut model_data = vec![];
            for k in self.models.keys() {
                if k.key() == key_occluder {
                    continue
                } else if let Some(s) = k.str() {
                    if s.starts_with("Terrain") {
                        terrain.push(k);
                    } else if s.contains("_Road_") | s.contains("_Collision_") {
                        collision_road.push(k);
                    } else {
                        normal.push(k)
                    }
                } else {
                    normal.push(k);
                }
            }
            normal.sort_unstable();
            terrain.sort_unstable_by_key(|x|
                x.str().and_then(|x| x.split('_').last().and_then(|x| x.parse::<usize>().ok())).unwrap_or_default()
            );
            if let Some(SubBlock::GameObjs(objs)) = self.sub_blocks1.blocks.get(&Crc::Key(2083108783)) {
                collision_road.sort_unstable_by_key(|x|
                    objs.objs.get_index_of(&x.str().and_then(|x| x.split('_').last().and_then(|x| x.parse::<u32>().ok())).unwrap_or_default())
                );
            } else {
                collision_road.sort_unstable_by_key(|x|
                    x.str().and_then(|x| x.split('_').last().and_then(|x| x.parse::<usize>().ok())).unwrap_or_default()
                );
            }
            // should be sorted in the order that they appear the level block
            for model in normal.into_iter().chain(collision_road).map(|key| self.models.get(key).unwrap()) {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(model.info.key.to_string())});
                let (vals, mesh_vals) = model.dump::<O>(block1.len(), &mut infos);
                model_data.push(((model.info.asset_key.clone(), model.info.asset_type), mesh_vals));
                block1.extend(vals);
                block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
            }
            let terrain_start_offset = block1.len() as u32;
            if !terrain.is_empty() {
                block1.extend(vec![0xFFu8; 16]);
            }
            for model in terrain.into_iter().map(|key| self.models.get(key).unwrap()) {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(model.info.key.to_string())});
                let (vals, mesh_vals) = model.dump_terrain::<O>(block1.len(), terrain_start_offset, &mut infos);
                model_data.push(((model.info.asset_key.clone(), model.info.asset_type), mesh_vals));
                block1.extend(vals);
            }
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);

            let foliages = self.foliages.iter().flat_map(|(key, x)| {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
                x
            }).map(|(info, val)| {
                let mut info = info.clone();
                info.offset = block1.len() as u32;
                block1.extend(dump_bytes!(O, val));
                block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
                info
            }).collect::<Vec<_>>();

            if let Some(model) = self.models.get(&Crc::Key(key_occluder)) {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(model.info.key.to_string())});
                let (vals, mesh_vals) = model.dump::<O>(block1.len(), &mut infos);
                model_data.push(((model.info.asset_key.clone(), model.info.asset_type), mesh_vals));
                block1.extend(vals);
                block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
            }
            info!("models & foliages in {:?}", time.elapsed());
            bar.as_ref().map(|x| { x.inc(1); x.set_message("block1 objs") });
            sub_bar.as_ref().map(|x| { x.set_length((self.gfx_blocks.len() + 1) as u64); x.reset() });

            let gfx_blocks = self.gfx_blocks.iter().sorted_by(|a,b| a.0.cmp(&b.0)).map(|(key, val)| {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
                let gfx_block = pak::GFXBlockInfo { key: key.clone(), offset: block1.len() as u32, size: val.len() as u32 };
                block1.extend(val.clone());
                block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
                gfx_block
            }).collect::<Vec<_>>();

            let gameobjs = if let Some(SubBlock::GameObjs(objs)) = self.sub_blocks1.blocks.get(&Crc::Key(2083108783)) {
                Some(objs)
            } else {
                None
            }.unwrap();
            let level_name = if let Some(BaseTypes::CRC(val)) = gameobjs.objs.first().unwrap().1.fields.get(&Crc::Key(2970763744)) {
                Some(val.clone())
            } else { None }.unwrap();
            let rad_name = Crc::Key(hash_string(b"_radiosity", Some(level_name.key())));
            let rad_data = self.radiosity.as_ref().map(|radiosity| {
                let mut rad_data = Vec::with_capacity(radiosity.rad_size::<O>());
                block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
                block1.extend(dump_bytes!(O, radiosity, block1.len(), &mut rad_data, &mut infos));
                block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
                rad_data
            });

            info!("block1 objs in {:?}", time.elapsed());
            bar.as_ref().map(|x| { x.inc(1); x.set_message("block1") });
            sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });

            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
            //block1.extend(vec![0u8; ((block1.len() + 511) & 0xFFFFFE00) - block1.len()]);
            pak_header.sub_blocks1_offset = block1.len() as u32;
            block1.extend(dump_bytes!(O, self.sub_blocks1, sub_bar.clone().into()));
            pak_header.string_keys_offset = block1.len() as u32;
            block1.extend(dump_bytes!(O, self.string_keys));

            to_bytes!(O, self.objas, &mut block1[pak_header.obja_offset as usize..])?;
            to_bytes!(O, self.obj0s, &mut block1[pak_header.obj0_offset as usize..])?;
            to_bytes!(O, infos.model, &mut block1[pak_header.model_info_offset as usize..])?;
            to_bytes!(O, infos.buffer, &mut block1[pak_header.buffer_info_offset as usize..])?;
            to_bytes!(O, infos.mat1, &mut block1[pak_header.mat1_offset as usize..])?;
            to_bytes!(O, infos.mat2, &mut block1[pak_header.mat2_offset as usize..])?;
            to_bytes!(O, infos.mat3, &mut block1[pak_header.mat3_offset as usize..])?;
            to_bytes!(O, infos.mat4, &mut block1[pak_header.mat4_offset as usize..])?;
            to_bytes!(O, infos.mat_extra, &mut block1[pak_header.mat_extra_offset as usize..])?;
            to_bytes!(O, infos.shape, &mut block1[pak_header.shape_info_offset as usize..])?;
            for (i, hk_shape) in infos.hk_shape.iter().enumerate() {
                to_bytes!(O, hk_shape, &mut block1[pak_header.hk_shape_info_offset as usize + i * O::size::<HkShape0>()..])?;
            }
            to_bytes!(O, infos.hk_constraint_data, &mut block1[pak_header.hk_constraint_data_offset as usize..])?;
            to_bytes!(O, infos.vbuff, &mut block1[pak_header.vbuff_info_offset as usize..])?;
            to_bytes!(O, infos.ibuff, &mut block1[pak_header.ibuff_info_offset as usize..])?;
            to_bytes!(O, texture_infos, &mut block1[pak_header.texture_info_offset as usize..])?;
            to_bytes!(O, infos.animation, &mut block1[pak_header.animation_info_offset as usize..])?;
            to_bytes!(O, infos.hk_constraint, &mut block1[pak_header.hk_constraint_info_offset as usize..])?;
            to_bytes!(O, effects, &mut block1[pak_header.effect_info_offset as usize..])?;
            to_bytes!(O, foliages, &mut block1[pak_header.foliage_info_offset as usize..])?;
            to_bytes!(O, pfield_infos, &mut block1[pak_header.pfield_info_offset as usize..])?;
            to_bytes!(O, gfx_blocks, &mut block1[pak_header.gfx_block_info_offset as usize..])?;
            to_bytes!(O, infos.radiosity_vals, &mut block1[pak_header.radiosity_vals_info_offset as usize..])?;
            to_bytes!(O, animation_block_infos, &mut block1[pak_header.animation_block_info_offset as usize..])?;
            info!("block1 in {:?}", time.elapsed());
            bar.as_ref().map(|x| { x.inc(1); x.set_message("block2") });
            sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });

            // block2
            for (i, model) in infos.model.iter().enumerate() {
                infos.block2_offsets.extend([
                    pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 8,
                    pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 12,
                    pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 48,
                    pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 140,
                    pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 144,
                    pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 152,
                    pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 164,
                    pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 172,
                    pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 180,
                    pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 252,
                ]);
                if model.bones_offset != 0 {
                    infos.block2_offsets.push(pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 136);
                }
                if model.skin_order_offset != 0 {
                    infos.block2_offsets.push(pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 160);
                }
                if model.vals_j_offset != 0 {
                    infos.block2_offsets.push(pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 196);
                }
                if model.block_offset != 0 {
                    infos.block2_offsets.push(pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 200);
                }
                if model.vals_k_offset != 0 {
                    infos.block2_offsets.push(pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 204);
                }
                if model.shape_offset != 0 {
                    infos.block2_offsets.push(pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 224);
                }
                if model.hk_constraint_data_offset != 0 {
                    infos.block2_offsets.push(pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 232);
                }
                if model.hk_constraint_offset != 0 {
                    infos.block2_offsets.push(pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 240);
                }
                if model.slots_offset != 0 {
                    infos.block2_offsets.push(pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 244);
                }
                if model.slot_map_offset != 0 {
                    infos.block2_offsets.push(pak_header.model_info_offset + (i * std::mem::size_of<pak::objs::ModelInfo>()) as u32 + 248);
                }
            }
            for (i, buffer) in infos.buffer.iter().enumerate() {
                infos.block2_offsets.extend([
                    pak_header.buffer_info_offset + (i * std::mem::size_of<pak::objs::BufferInfo>()) as u32,
                    pak_header.buffer_info_offset + (i * std::mem::size_of<pak::objs::BufferInfo>()) as u32 + 260,
                ]);
                if buffer.vbuff_info_offset_2 != 0 {
                    infos.block2_offsets.push(pak_header.buffer_info_offset + (i * std::mem::size_of<pak::objs::BufferInfo>()) as u32 + 4);
                }
                if buffer.vbuff_info_offset_3 != 0 {
                    infos.block2_offsets.push(pak_header.buffer_info_offset + (i * std::mem::size_of<pak::objs::BufferInfo>()) as u32 + 8);
                }
            }
            for (i, mat) in infos.mat1.iter().enumerate() {
                if mat.base.mat_extra_offset != 0 {
                    infos.block2_offsets.push(pak_header.mat1_offset + (i * std::mem::size_of<pak::objs::MatBase>()) as u32 + 344);
                }
            }
            for (i, mat) in infos.mat2.iter().enumerate() {
                if mat.base.mat_extra_offset != 0 {
                    infos.block2_offsets.push(pak_header.mat2_offset + (i * std::mem::size_of<pak::objs::Mat2>()) as u32 + 344);
                }
            }
            for (i, mat) in infos.mat3.iter().enumerate() {
                if mat.base.mat_extra_offset != 0 {
                    infos.block2_offsets.push(pak_header.mat3_offset + (i * std::mem::size_of<pak::objs::Mat3>()) as u32 + 344);
                }
            }
            for (i, mat) in infos.mat4.iter().enumerate() {
                if mat.base.mat_extra_offset != 0 {
                    infos.block2_offsets.push(pak_header.mat4_offset + (i * std::mem::size_of<pak::objs::Mat4>()) as u32 + 344);
                }
            }
            for (i, shape) in infos.shape.iter().enumerate() {
                if shape.hk_shape_offset != 0 {
                    infos.block2_offsets.push(pak_header.shape_info_offset + (i * std::mem::size_of<pak::objs::ShapeInfo>()) as u32 + 112);
                }
            }
            for (i, hk_shape) in infos.hk_shape.iter().enumerate() {
                infos.block2_offsets.extend(match hk_shape {
                    HkShapeInfo::ConvexVertices(_) => vec![
                        pak_header.hk_shape_info_offset + (i * std::mem::size_of<pak::objs::HkShapeInfo>()) as u32 + 44,
                        pak_header.hk_shape_info_offset + (i * std::mem::size_of<pak::objs::HkShapeInfo>()) as u32 + 52
                    ],
                    HkShapeInfo::BVTreeMesh(_) => vec![
                        pak_header.hk_shape_info_offset + (i * std::mem::size_of<pak::objs::HkShapeInfo>()) as u32 + 60,
                        pak_header.hk_shape_info_offset + (i * std::mem::size_of<pak::objs::HkShapeInfo>()) as u32 + 68,
                        pak_header.hk_shape_info_offset + (i * std::mem::size_of<pak::objs::HkShapeInfo>()) as u32 + 76
                    ],
                    _ => vec![]
                });
            }
            for (i, hk_constraint) in infos.hk_constraint.iter().enumerate() {
                infos.block2_offsets.extend([
                    pak_header.hk_constraint_info_offset + (i * std::mem::size_of<pak::objs::HkConstraintInfo>()) as u32 + 4,
                    pak_header.hk_constraint_info_offset + (i * std::mem::size_of<pak::objs::HkConstraintInfo>()) as u32 + 12,
                    pak_header.hk_constraint_info_offset + (i * std::mem::size_of<pak::objs::HkConstraintInfo>()) as u32 + 20,
                    pak_header.hk_constraint_info_offset + (i * std::mem::size_of<pak::objs::HkConstraintInfo>()) as u32 + 40,
                    pak_header.hk_constraint_info_offset + (i * std::mem::size_of<pak::objs::HkConstraintInfo>()) as u32 + 48,
                ]);
                if hk_constraint.vals2_offset != 0 {
                    infos.block2_offsets.push(pak_header.hk_constraint_info_offset + (i * std::mem::size_of<pak::objs::HkConstraintInfo>()) as u32 + 64);
                }
            }
            for i in 0..pak_header.effect_info_num as usize {
                infos.block2_offsets.push(pak_header.effect_info_offset + (i * std::mem::size_of<pak::objs::EffectInfo>()) as u32 + 8);
            }
            for i in 0..pak_header.gfx_block_info_num as usize {
                infos.block2_offsets.push(pak_header.gfx_block_info_offset + (i * std::mem::size_of<pak::objs::GFXBlockInfo>()) as u32 + 4);
            }
            for i in 0..pak_header.radiosity_vals_info_num as usize {
                infos.block2_offsets.push(pak_header.radiosity_vals_info_offset + (i * std::mem::size_of<pak::objs::RadiosityValsInfo>()) as u32 + 8);
            }
            for i in 0..pak_header.foliage_info_num as usize {
                infos.block2_offsets.push(pak_header.foliage_info_offset + (i * std::mem::size_of<pak::objs::FoliageInfo>()) as u32 + 28);
            }
            pak_header.sub_blocks2_offset = 0;
            let mut block2 = dump_bytes!(O, self.sub_blocks2, sub_bar.clone().into());
            pak_header.block2_offsets_offset = block2.len() as u32;
            pak_header.block2_offsets_num = infos.block2_offsets.len() as u32;
            block2.extend(dump_bytes!(O, infos.block2_offsets));
            info!("block2 in {:?}", time.elapsed());
            bar.as_ref().map(|x| { x.inc(1); x.set_message("pak") });
            sub_bar.as_ref().map(|x| { x.set_length(4); x.reset() });

            // rest of pak
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("block1")});
            pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
            let size = block1.len();
            let data = CompressedBlock { data: block1 }.dump(true).context("pak_block1")?;
            pak_header.block1_size = size as u32;
            pak_header.block1_size_comp = data.len() as u32;
            pak_header.block1_offset = pak_data.len() as u32;
            if pak_header.block1_size_comp == pak_header.block1_size {
                pak_header.block1_size_comp = 0;
            }
            pak_data.extend(data);

            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("block2")});
            pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
            let size = block2.len();
            let data = CompressedBlock { data: block2 }.dump(true).context("pak_block2")?;
            pak_header.block2_size = size as u32;
            pak_header.block2_size_comp = data.len() as u32;
            pak_header.block2_offset = pak_data.len() as u32;
            if pak_header.block2_size_comp == pak_header.block2_size {
                pak_header.block2_size_comp = 0;
            }
            pak_data.extend(data);

            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("strings")});
            pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
            let data = dump_bytes!(O, self.pak_strings);
            pak_header.strings_offset = pak_data.len() as u32;
            pak_header.strings_num = self.pak_strings.strings.len() as u32;
            pak_header.strings_size = data.len() as u32;
            pak_data.extend(data);

            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("vals_a")});
            pak_header.block_a_offset = pak_data.len() as u32;
            pak_header.block_a_num = self.pak_vals_a.len() as u32;
            pak_data.extend(dump_bytes!(O, self.pak_vals_a));

            pak_data.extend(vec![0u8; ((pak_data.len() + 2047) & 0xfffff800)-pak_data.len()]);
            to_bytes!(O, pak_header, &mut pak_data)?;
            info!("pak in {:?}", time.elapsed());
            bar.as_ref().map(|x| { x.inc(1); x.set_message("bin") });
            sub_bar.as_ref().map(|x| { x.set_length((model_data.len() + texture_data.len() + 2 + rad_data.is_some().then_some(1).unwrap_or(0)) as u64); x.reset() });

            // bin_data
            let mut bin_header = self.bin_header.clone();
            let mut bin_data = vec![0u8; O::size::<bin::Header>()];
            bin_header.version = if TypeId::of::<O>() == TypeId::of::<PC>() {
                1
            } else if TypeId::of::<O>() == TypeId::of::<XBOX>() {
                2
            }  else if TypeId::of::<O>() == TypeId::of::<PS3>() {
                3
            } else {
                panic!("Unsupported format")
            };

            bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
            let mut model_asset_handles = model_data.into_iter().map(|((key, kind), data)| {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
                let size = data.len() as u32;
                let offset = bin_data.len() as u32;
                let size_comp = if size != 0 {
                    let data = CompressedBlock { data }.dump(false).with_context(|| format!("{}", key.to_string()))?;
                    let size_comp = data.len() as u32;
                    bin_data.extend(data);
                    bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
                    size_comp
                } else { 0 };
                Ok(bin::AssetHandle { key, offset, size, size_comp, kind })
            }).collect::<Result<Vec<_>>>()?;

            let mut texture_asset_handles = texture_data.into_iter().map(|((key, kind), data)| {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
                let size = data.len() as u32;
                let offset = bin_data.len() as u32;
                let size_comp = if size != 0 {
                    let data = CompressedBlock { data }.dump(false).with_context(|| format!("{}", key.to_string()))?;
                    let size_comp = data.len() as u32;
                    bin_data.extend(data);
                    bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
                    size_comp
                } else { 0 };
                Ok(bin::AssetHandle { key, offset, size, size_comp, kind })
            }).collect::<Result<Vec<_>>>()?;

            if let Some(rad_data) = rad_data {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(rad_name.to_string())});
                let size = rad_data.len() as u32;
                let offset = bin_data.len() as u32;
                let size_comp = if size != 0 {
                    let data = CompressedBlock { data: rad_data }.dump(false).with_context(|| format!("{}", rad_name.to_string()))?;
                    let size_comp = data.len() as u32;
                    bin_data.extend(data);
                    bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
                    size_comp
                } else { 0 };
                model_asset_handles.push(bin::AssetHandle {
                    key: rad_name,
                    offset, size, size_comp,
                    kind: self.radiosity.as_ref().unwrap().usage
                });
            }

            model_asset_handles.sort_by_key(|x| x.key.key());
            texture_asset_handles.sort_by_key(|x| x.key.key());

            bin_header.vdata_num = model_asset_handles.len() as u32;
            bin_header.vdata_num_ = model_asset_handles.len() as u32;
            bin_header.texdata_num = texture_asset_handles.len() as u32;

            let mut asset_handles = model_asset_handles;
            asset_handles.extend(texture_asset_handles);

            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("asset_handles")});
            bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
            bin_header.asset_handle_offset = bin_data.len() as u32;
            bin_header.asset_handle_num = asset_handles.len() as u32;
            bin_data.extend(dump_bytes!(O, asset_handles));

            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("strings")});
            let data = dump_bytes!(O, self.bin_strings);
            bin_header.strings_offset = bin_data.len() as u32;
            bin_header.strings_size = data.len() as u32;
            bin_header.strings_num = self.bin_strings.strings.len() as u32;
            bin_data.extend(data);

            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("header")});
            bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
            to_bytes!(O, bin_header, &mut bin_data)?;
            info!("bin in {:?}", time.elapsed());
            bar.as_ref().map(|x| x.finish_and_clear());
            sub_bar.as_ref().map(|x| x.finish_and_clear());

            // bin done
            let mut max_vert = 0;
            let mut max_tex = 0;
            for (i, animation_block_info) in animation_block_infos.iter().enumerate() {
                let gamemodemask = 1 << i;
                let mut tex_size = 0;
                let mut vert_size = 0;
                for tex in self.textures.values() {
                    if (tex.info().gamemodemask & gamemodemask) != 0 {
                        tex_size += tex.size();
                    }
                }
                for model in self.models.values() {
                    if (model.info.gamemodemask & gamemodemask) != 0 {
                        for asset_handle in &asset_handles {
                            if asset_handle.key == model.info.asset_key && asset_handle.kind == model.info.asset_type {
                                vert_size += asset_handle.size;
                                break;
                            }
                        }
                    }
                }
                max_vert = max_vert.max(vert_size);
                max_tex = max_tex.max(tex_size);
                info!("Gamemode {} min buffer sizes: texture {}, vertex {}", animation_block_info.key.to_string(), tex_size, vert_size);
            }
            if max_vert > 106954752 {
                warn!("An unmodified conquest executable will not be able to load some gamemodes with min vertex buffer size larger than 106954752")
            }
            if max_tex > 178257920 {
                warn!("An unmodified conquest executable will not be able to load some gamemodes with min texture buffer size larger than 178257920")
            }
            Ok((pak_data, bin_data, infos))
        }
        */

        (pak_data, bin_data)
    }
}

// wrapped level
// - pak
//  - block1: objs + sub_blocks + string_keys
//  - block2: lang_strings + block1_offsets
//  - animation_blocks
// - bin
//  - textures
//  - models
//  - radiosity
