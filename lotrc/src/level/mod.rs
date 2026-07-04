use anyhow::{Context, Result};
use lotrc_proc::make_platforms;
use std::path::Path;
use std::io::Read;
use log::debug;

use crate::{
    types::{AlignedBuf, DumpSlice},
};

#[make_platforms]
use crate::{
    level::{
        bin::{DumpBinVER},
        pak::{DumpPakVER}
    }
};

pub mod bin;
pub mod model;
pub mod pak;
pub mod radiosity;
pub mod texture;

#[cfg_attr(feature = "ffi", repr(C))]
pub struct LevelData {
   pub pak: AlignedBuf,
   pub bin: AlignedBuf
}

#[cfg_attr(feature = "ffi", repr(u8))]
pub enum Version {
    Pc = 0,
    Xbox,
    Ps3, Err
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

#[make_platforms]
#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct LevelRefVER<'a> {
    pub pak: pak::PakRefVER<'a>,
    pub bin: bin::BinRefVER<'a>,
}

#[make_platforms]
impl<'a> LevelRefVER<'a> {
    pub fn from_data<'b: 'a, 'c: 'b>(src: &'c LevelData, data: &'a mut LevelCompressedData<'b>) -> Result<Self> {
        let bin = bin::BinRefVER::from_data(&src.bin[..], &mut data.bin).context("bin")?;
        Ok(Self {
            pak: pak::PakRefVER::from_data(&src.pak[..], &mut data.pak, &bin).context("pak")?,
            bin,
        })
    }
}

#[make_platforms]
pub trait DumpLevelVER {
    fn pak(&self) -> &impl DumpPakVER;
    fn bin(&self) -> &impl DumpBinVER;
    fn dump(&self, c: flate2::Compression) -> Result<LevelData> {
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
        bin.dump(&mut dump_slice, &model_data, &texture_data, rad_data.as_ref()).context("dump bin")?;
        debug!("bin dumped in {}", t.elapsed().as_secs_f32());

        Ok(LevelData {
            pak: pak_data,
            bin: bin_data
        })
    }
}

#[make_platforms]
impl DumpLevelVER for LevelRefVER<'_> {
    fn pak(&self) -> &impl DumpPakVER {
        &self.pak
    }
    fn bin(&self) -> &impl DumpBinVER {
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
#[make_platforms]
#[export(mod_name=LevelRefVER)]
mod ref_ver {
    use super::*;
    fn from_data<'a>(src: Option<&'a LevelData>, data: Option<&'a mut LevelCompressedData<'a>>) -> Option<NonNull<LevelRefVER<'a>>> {
        if src.is_none() || data.is_none() {
            return None;
        }
        LevelRefVER::from_data(src.unwrap(), data.unwrap()).ok().map(|x| Box::leak(Box::new(x)).into())
    }
    fn dump(src: Option<&LevelRefVER>, compression: u32) -> Option<NonNull<LevelData>> {
        src.and_then(|x| x.dump(flate2::Compression::new(compression)).ok().map(|x| Box::leak(Box::new(x)).into()))
    }
    fn free(val: Option<std::ptr::NonNull<LevelRefVER>>) {
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
#[make_platforms]
fn level_parse_ver(level: &LevelVER) -> Result<Level> {
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
