use std::{any::TypeId, collections::{hash_map::Entry, HashMap, HashSet}, ffi::OsStr, fs, path::Path, sync::Arc};
use itertools::Itertools;
use log::{warn, info};
use serde::{Serialize, Deserialize};
use serde_json::to_vec_pretty;
use std::time::Instant;
use std::iter::zip;
use anyhow::{Result, Context};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use pyo3::prelude::*;
use lotrc_proc::{basicpymethods, PyMethods};

use crate::types::BaseTypes;

use super::{
    pak, bin, lua_stuff, pak_alt::*,
    types::{self, hash_string, GameObjs, OrderedData, OrderedDataVec, CompressedBlock, Crc, Version, PC, XBOX, PS3, SubBlock},
    read_write::{Reader, Writer, PathStuff},
};

const BAR_FMT: &str = "{wide_msg:>} {pos}/{len}";
const SUB_BAR_FMT: &str = "{wide_msg:>} {pos}/{len}";

#[pyclass(module="level_alt", set_all, get_all)]
#[derive(Debug, Default, Serialize, Deserialize, PyMethods)]
pub struct Level {
    pub bin_header: bin::Header,
    pub bin_strings: types::Strings,
    pub pak_header: pak::Header,
    pub pak_strings: types::Strings,

    pub objas: Vec<pak::ObjA>,
    pub obj0s: Vec<pak::Obj0>,
    pub models: HashMap<Crc, Model>,
    pub textures: HashMap<Crc, bin::Tex>,
    pub animations: HashMap<Crc, Animation>,
    pub foliages: HashMap<Crc, Vec<(pak::FoliageInfo, pak::Foliage)>>,
    pub radiosity_vals: HashMap<u32, Vec<u32>>,
    pub effects: HashMap<Crc, GameObjs>,
    pub pfield_infos: Vec<pak::PFieldInfo>,
    pub animation_block_infos: Vec<pak::AnimationBlockInfo>,
    pub gfx_blocks: HashMap<Crc, Vec<u8>>,

    pub string_keys: types::StringKeys,
    pub sub_blocks1: types::SubBlocks,
    pub sub_blocks2: types::SubBlocks,
    pub block2_offsets: Vec<u32>,

    pub radiosity: HashMap<Crc, bin::Radiosity>,

    //pub vertex_formats: HashMap<(u32, u32), (Vec<pak::VertexData>, usize)>,

    pub pak_vals_a: Vec<pak::BlockAVal>,
}

#[basicpymethods]
#[pymethods]
impl Level {
    #[staticmethod]
    fn load(path: String) -> Result<Self> {
        let path = std::path::PathBuf::from(path);
        if path.with_extension("zip").is_file() {
            Self::from_file(Reader::new(path, true)?, None)
        } else if path.is_dir() {
            Self::from_file(Reader::new(path, false)?, None)
        } else {
            Self::parse(path, None)
        }
    }

    fn dump_pc(&self, path: String) -> Result<()> {
        self.dump::<PC, _>(path, None)
    }
}

impl Level {
    pub fn parse<P: AsRef<Path>>(path: P, mp: Option<&MultiProgress>) -> Result<Self> {
        let path = path.as_ref();
        info!("Parsing level data {:?}", path);   
        let pak_data = fs::read(path.with_extension("PAK")).context(path.with_extension("PAK").display().to_string())?;
        let bin_data = fs::read(path.with_extension("BIN")).context(path.with_extension("BIN").display().to_string())?;
        Ok(if bin_data[0] == 6 {
            Self::from_data::<PC>(&bin_data[..], &pak_data[..], mp)?
        } else if bin_data[3] == 6 && bin_data[7] == 2 {
            Self::from_data::<XBOX>(&bin_data[..], &pak_data[..], mp)?
        } else if bin_data[3] == 6 && bin_data[7] == 3 {
            Self::from_data::<PS3>(&bin_data[..], &pak_data[..], mp)?
        } else {
            warn!("Invalid level data");
            Default::default()
        })
    }

    pub fn dump<O: Version + 'static, P: AsRef<Path>>(&self, path: P, mp: Option<&MultiProgress>) -> Result<()> {
        let path = path.as_ref();
        info!("Dumping level data {:?}", path);
        let (pak, bin, _infos) = self.to_data::<O>(mp)?;
        path.parent().map(fs::create_dir_all);
        fs::write(path.with_extension("PAK"), pak).context(path.with_extension("PAK").display().to_string())?;
        fs::write(path.with_extension("BIN"), bin).context(path.with_extension("BIN").display().to_string())?;
        // fs::write(path.with_extension("json"), serde_json::to_vec_pretty(&_infos).unwrap()).unwrap();
        Ok(())
    }

    pub fn from_data<O: Version + 'static>(bin_data: &[u8], pak_data: &[u8], mp: Option<&MultiProgress>) -> Result<Self> {
        let time = Instant::now();
        info!("extracting level");

        let (bar, sub_bar) = if let Some(mp) = mp {
            let bar = ProgressBar::new(8).with_style(ProgressStyle::with_template(BAR_FMT).unwrap());
            let sub_bar = ProgressBar::new(0).with_style(ProgressStyle::with_template(SUB_BAR_FMT).unwrap());
            mp.add(bar.clone());
            mp.add(sub_bar.clone());
            (Some(bar), Some(sub_bar))
        } else { (None, None) };

        bar.as_ref().map(|x| { x.inc(1); x.set_message("bin headers") });
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        let lua = lua_stuff::LuaCompiler::new().unwrap();
        let bin_header: bin::Header = OrderedData::from_bytes::<O>(bin_data)?;
        let bin_strings = types::Strings::from_data::<O>(bin_data, bin_header.strings_offset as usize, bin_header.strings_num as usize)?;
        types::update_strings(&bin_strings.strings);
        info!("bin headers in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("bin") });
        sub_bar.as_ref().map(|x| { x.set_length(bin_header.asset_handle_num as u64); x.reset() });

        let asset_handles: Vec<bin::AssetHandle> = OrderedDataVec::from_bytes::<O>(&bin_data[bin_header.asset_handle_offset as usize..], bin_header.asset_handle_num as usize)?;
        let model_data = HashMap::<Crc, Vec<u8>>::from_iter(
            asset_handles.iter().map(|info| (info.key.clone(), {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
                types::CompressedBlock::from_data::<O>(bin_data, info.size as usize, info.size_comp as usize, info.offset as usize).data
            })).take(bin_header.vdata_num as usize)
        );
        let texture_data = HashMap::<Crc, Vec<u8>>::from_iter(
            asset_handles.iter().skip(bin_header.vdata_num as usize).map(|info| {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
                (info.key.clone(), types::CompressedBlock::from_data::<O>(bin_data, info.size as usize, info.size_comp as usize, info.offset as usize).data)
            })
        );

        let radiosity = model_data.iter().filter(|(key, _)| key.str().map(|x| x.ends_with("_radiosity")).unwrap_or(false)).map(|(key, data)| Ok((
            key.clone(), bin::Radiosity::from_data::<O>(&data[..], 0)?
        ))).collect::<Result<HashMap<Crc, bin::Radiosity>>>()?;
        
        info!("bin parsed in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("pak headers") });
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        let pak_header: pak::Header = OrderedData::from_bytes::<O>(pak_data)?;
        let pak_strings = types::Strings::from_data::<O>(pak_data, pak_header.strings_offset as usize, pak_header.strings_num as usize)?;
        types::update_strings(&pak_strings.strings);
        info!("pak headers in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("block2") });
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        let block2 = types::CompressedBlock::from_data::<O>(pak_data, pak_header.block2_size as usize, pak_header.block2_size_comp as usize, pak_header.block2_offset as usize).data;
        let sub_blocks2 = types::SubBlocks::from_data::<O>(&block2[..], pak_header.sub_blocks2_offset as usize, &lua, sub_bar.as_ref())?;
        let block2_offsets = OrderedDataVec::from_bytes::<O>(&block2[pak_header.block2_offsets_offset as usize..], pak_header.block2_offsets_num as usize)?;
        info!("block2 parsed in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("main blocks") });
        sub_bar.as_ref().map(|x| { x.set_length((
            pak_header.animation_block_info_num +
            pak_header.model_info_num + 
            pak_header.effect_info_num +
            pak_header.gfx_block_info_num +
            pak_header.radiosity_vals_info_num +
            pak_header.foliage_info_num +
            pak_header.texture_info_num +
            pak_header.animation_info_num
        ) as u64); x.reset() });
        let block1 = types::CompressedBlock::from_data::<O>(pak_data, pak_header.block1_size as usize, pak_header.block1_size_comp as usize, pak_header.block1_offset as usize).data;
        info!("main blocks parsed in {:?}", time.elapsed());
        
        bar.as_ref().map(|x| { x.inc(1); x.set_message("items") });
        sub_bar.as_ref().map(|x| { x.set_length(bin_header.asset_handle_num as u64); x.reset() });

        let objas = OrderedDataVec::from_bytes::<O>(&block1[pak_header.obja_offset as usize..], pak_header.obja_num as usize)?;
        let obj0s = OrderedDataVec::from_bytes::<O>(&block1[pak_header.obj0_offset as usize..], pak_header.obj0_num as usize)?;
        let animation_block_infos: Vec<pak::AnimationBlockInfo> = OrderedDataVec::from_bytes::<O>(&block1[pak_header.animation_block_info_offset as usize..], pak_header.animation_block_info_num as usize)?;
        let pfield_infos = OrderedDataVec::from_bytes::<O>(&block1[pak_header.pfield_info_offset as usize..], pak_header.pfield_info_num as usize)?;

        let mut vertex_formats = HashMap::new();
        let models = <Vec<pak::ModelInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.model_info_offset as usize..], pak_header.model_info_num as usize)?.into_iter().map(|info| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
            let mut model = Model::from_data::<O>(info, &block1[..])?;
            if model.info.vbuff_num != 0 || model.info.ibuff_num != 0 {
                let buffer = model_data.get(&model.info.asset_key.clone()).unwrap();
                model.vertex_data.extend(model.vbuffs.iter_mut().map(|info| pak::VertexBuffer::from_data::<O>(&buffer[..], info, &mut vertex_formats)).collect::<Result<Vec<_>>>()?);
                model.index_data.extend(model.ibuffs.iter().map(|info| pak::IndexBuffer::from_data::<O>(&buffer[..], info)).collect::<Result<Vec<_>>>()?);    
            }
            Ok((model.info.key.clone(), model))
        }).collect::<Result<HashMap<_, _>>>()?;

        let effects = <Vec<pak::EffectInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.effect_info_offset as usize..], pak_header.effect_info_num as usize)?.into_iter().map(|info| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
            Ok((info.key, GameObjs::from_data::<O>(&block1[..], info.offset as usize, info.size as usize, info.gamemodemask)?))
        }).collect::<Result<HashMap<_, _>>>()?;

        let gfx_blocks = <Vec<pak::GFXBlockInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.gfx_block_info_offset as usize..], pak_header.gfx_block_info_num as usize)?.into_iter().map(|info| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
            (info.key, block1[info.offset as usize..(info.offset + info.size) as usize].to_vec())
        }).collect::<HashMap<_, _>>();

        let radiosity_vals = <Vec<pak::RadiosityValsInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.radiosity_vals_info_offset as usize..], pak_header.radiosity_vals_info_num as usize)?.into_iter().map(|info| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(format!("{}", info.guid))});
            Ok((info.guid, <Vec<u32> as OrderedDataVec>::from_bytes::<O>(&block1[info.offset as usize..], info.num as usize)?))
        }).collect::<Result<HashMap<_, _>>>()?;

        let mut foliages: HashMap<Crc, Vec<(pak::FoliageInfo, pak::Foliage)>> = HashMap::with_capacity(pak_header.foliage_info_num as usize);
        for info in <Vec<pak::FoliageInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.foliage_info_offset as usize..], pak_header.foliage_info_num as usize)? {        
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
            foliages.entry(info.key.clone()).or_default().push((
                info.clone(), 
                pak::Foliage::from_data::<O>(&block1[..], &info)?
            ))
        }

        let textures = <Vec<pak::TextureInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.texture_info_offset as usize..], pak_header.texture_info_num as usize)?.into_iter().map(|mut info| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
            let data0 = texture_data.get(&info.asset_key).expect(format!("could not find texture data {}", info.asset_key.to_string()).as_str());
            let data1 = texture_data.get(&Crc::Key(hash_string("*".as_bytes(), Some(info.asset_key.key())))).unwrap();
            let tex = bin::Tex::from_data::<O>(data0, data1, &mut info)?;
            let key = info.key.clone();
            Ok((key, tex))
        }).collect::<Result<HashMap<_, _>>>()?;

        let blocks = animation_block_infos.iter().map(|info| { 
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
            types::CompressedBlock::from_data::<O>(&pak_data[..], info.size as usize, info.size_comp as usize, info.offset as usize).data
        }).collect::<Vec<_>>();
        let mut offsets = blocks.iter().map(|_| 0usize).collect::<Vec<_>>();
        let animations = <Vec<pak::AnimationInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.animation_info_offset as usize..], pak_header.animation_info_num as usize)?.into_iter().map(|info| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
            let anim = Animation::from_data::<O>(info, &mut offsets, &blocks)?;
            Ok((anim.info.key.clone(), anim))
        }).collect::<Result<HashMap<_, _>>>()?;

        info!("items parsed in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("sub blocks") });
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        let sub_blocks1 = types::SubBlocks::from_data::<O>(&block1[..], pak_header.sub_blocks1_offset as usize, &lua, sub_bar.as_ref())?;
        let string_keys = types::StringKeys::from_data::<O>(&block1[..], pak_header.string_keys_offset as usize)?;
        info!("sub blocks parsed in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("pak") });
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        let pak_vals_a = OrderedDataVec::from_bytes::<O>(&pak_data[pak_header.block_a_offset as usize..], pak_header.block_a_num as usize)?;
        info!("pak parsed in {:?}", time.elapsed());

        bar.as_ref().map(|x| x.finish_and_clear());
        sub_bar.as_ref().map(|x| x.finish_and_clear());

        Ok(Self {
            bin_header,
            bin_strings,
            pak_header,
            pak_strings,
            objas,
            obj0s,
            models,
            textures,
            animations,
            foliages,
            radiosity_vals,
            effects,
            pfield_infos,
            animation_block_infos,
            string_keys,
            sub_blocks1,
            sub_blocks2,
            block2_offsets,
            radiosity,
            //vertex_formats,
            pak_vals_a,
            gfx_blocks,
        })
    }
    
    pub fn to_data<O: Version + 'static>(&self, mp: Option<&MultiProgress>) -> Result<(Vec<u8>, Vec<u8>, DumpInfos)> {
        fn dump_vertex_data<O: Version + 'static>(model: &mut Model) -> Option<((Crc, u32), Vec<u8>)> {
            fn pack_vbuff<O: Version + 'static>(vbuffs: &mut Vec<pak::VBuffInfo>, vertex_data: &Vec<pak::VertexBuffer>, i: usize, vbuff_map: &mut HashMap<usize, (u32, u32)>, data: &mut Vec<u8>) -> (u32, u32) {
                if i == 0xFFFFFFFF {
                    return (0, 0);
                }
                match vbuff_map.entry(i) {
                    Entry::Occupied(val) => *val.get(),
                    Entry::Vacant(val) => {
                        data.extend(vec![0u8; ((data.len() + 15) & 0xFFFFFFF0) - data.len()]);
                        let info = &mut vbuffs[i];
                        let vals = vertex_data[i].dump::<O>();
                        info.offset = data.len() as u32;
                        info.size = vals.len() as u32;
                        data.extend(vals);
                        *val.insert((vertex_data[i].vals.iter().map(|x| x.val.size()).sum::<usize>() as u32, info.size)) 
                    }
                }
            }

            if model.vertex_data.len() != 0 || model.index_data.len() != 0 {
                let size = model.vbuffs.iter().map(|x| x.size + x.offset).chain(model.ibuffs.iter().map(|x| x.size + x.offset)).max().unwrap();

                let mut data = Vec::with_capacity(size as usize);
                let mut vbuff_map = HashMap::with_capacity(model.vbuffs.len());
                let mut ibuffs = HashSet::with_capacity(model.ibuffs.len());
                for buffer_info in &mut model.buffer_infos {
                    (
                        buffer_info.v_size, 
                        buffer_info.vbuff_size
                    ) = pack_vbuff::<O>(&mut model.vbuffs, &model.vertex_data, buffer_info.vbuff_info_offset as usize, &mut vbuff_map, &mut data);
                    (
                        buffer_info.v_size_2, 
                        buffer_info.vbuff_size_2
                    ) = pack_vbuff::<O>(&mut model.vbuffs, &model.vertex_data, buffer_info.vbuff_info_offset_2 as usize, &mut vbuff_map, &mut data);
                    (
                        buffer_info.v_size_3, 
                        buffer_info.vbuff_size_3
                    ) = pack_vbuff::<O>(&mut model.vbuffs, &model.vertex_data, buffer_info.vbuff_info_offset_3 as usize, &mut vbuff_map, &mut data);
                    let i = buffer_info.ibuff_info_offset as usize;
                    if i != 0xFFFFFFFF && !ibuffs.contains(&i) {
                        ibuffs.insert(i);
                        data.extend(vec![0u8; ((data.len() + 15) & 0xFFFFFFF0) - data.len()]);
                        let info = &mut model.ibuffs[i];
                        let vals = model.index_data[i].dump::<O>();
                        info.offset = data.len() as u32;
                        info.size = vals.len() as u32;
                        data.extend(vals);
                    }
                }

                Some(((model.info.asset_key.clone(), model.info.asset_type), data))
            } else {
                None
            }
        }
        let time = Instant::now();
        info!("compressing level");

        let (bar, sub_bar) = if let Some(mp) = mp {
            let bar = ProgressBar::new(9).with_style(ProgressStyle::with_template(BAR_FMT).unwrap());
            let sub_bar = ProgressBar::new(0).with_style(ProgressStyle::with_template(SUB_BAR_FMT).unwrap());
            mp.add(bar.clone());
            mp.add(sub_bar.clone());
            (Some(bar), Some(sub_bar))
        } else { (None, None) };

        bar.as_ref().map(|x| { x.inc(1); x.set_message("textures") });
        sub_bar.as_ref().map(|x| { x.set_length(self.textures.len() as u64); x.reset() });

        let lua: lua_stuff::LuaCompiler = lua_stuff::LuaCompiler::new().unwrap();
        
        let mut texture_data = vec![];
        let mut seen_textures = HashSet::new();
        fn sort_texture(tex: &&bin::Tex) -> u32 {
            let k = tex.info().key.key();
            if k == 3804089404 {
                0
            } else if k == 4026460901 {
                1
            } else {
                k
            }
        }

        let texture_infos = self.textures.values().sorted_by_key(sort_texture).map(|tex| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(tex.info().key.to_string())});
            let (data0, data1) = tex.dump::<O>();
            if !seen_textures.contains(&tex.info().asset_key.key()) {
                if data1.len() == 0 {
                    texture_data.push((
                        (Crc::Key(hash_string("*".as_bytes(), Some(tex.info().asset_key.key()))), tex.info().asset_type),
                        data1
                    ));
                    texture_data.push(((tex.info().asset_key.clone(), tex.info().asset_type), data0));
                } else {
                    texture_data.push(((tex.info().asset_key.clone(), tex.info().asset_type), data0));
                    texture_data.push((
                        (Crc::Key(hash_string("*".as_bytes(), Some(tex.info().asset_key.key()))), tex.info().asset_type),
                        data1
                    ));
                }
                seen_textures.insert(tex.info().asset_key.key());
            }
            tex.info().clone()
        }).collect::<Vec<_>>();

        info!("textures in {:?}", time.elapsed());
        bar.as_ref().map(|x| { x.inc(1); x.set_message("animations") }); 
        sub_bar.as_ref().map(|x| { x.set_length((self.animation_block_infos.len() + self.animations.len()) as u64); x.reset() });

        // pak stuff
        let mut pak_header = self.pak_header.clone();
        pak_header.version = if TypeId::of::<O>() == TypeId::of::<PC>() { 
            1 
        } else if TypeId::of::<O>() == TypeId::of::<XBOX>() {
            2
        }  else if TypeId::of::<O>() == TypeId::of::<PS3>() {
            3
        } else {
            panic!("Unsupported format")
        };
        let mut pak_data = vec![0u8; pak::Header::size::<O>()];

        // block1 stuff
        
        (
            pak_header.shape_info_num,
            pak_header.hk_shape_info_num,
            pak_header.hk_constraint_info_num,
            pak_header.hk_constraint_data_num,
            pak_header.mat1_num, pak_header.mat2_num, pak_header.mat3_num, pak_header.mat4_num,
            pak_header.mat_extra_num,
            pak_header.buffer_info_num,
            pak_header.vbuff_info_num, pak_header.ibuff_info_num

        ) = self.models.values().map(|model| model.infos_count()).fold(
            (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0), 
            |mut a,b| {
                a.0 += b.0; a.1 += b.1; a.2 += b.2; a.3 += b.3; a.4 += b.4; a.5 += b.5; a.6 += b.6; a.7 += b.7; a.8 += b.8; a.9 += b.9; a.10 += b.10; a.11 += b.11;
                a
            }
        );
        pak_header.model_info_num = self.models.len() as u32;
        pak_header.texture_info_num = self.textures.len() as u32;
        pak_header.effect_info_num = self.effects.len() as u32;
        pak_header.gfx_block_info_num = self.gfx_blocks.len() as u32;
        pak_header.radiosity_vals_info_num = self.radiosity_vals.len() as u32;
        pak_header.foliage_info_num = self.foliages.iter().map(|(_, x)| x.len()).sum::<usize>() as u32;
        pak_header.animation_info_num = self.animations.len() as u32;

        let mut block1: Vec<u8> = vec![];
        pak_header.obja_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.obja_num as usize * pak::ObjA::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.obj0_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.obj0_num as usize * pak::Obj0::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.model_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.model_info_num as usize * pak::ModelInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.buffer_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.buffer_info_num as usize * pak::BufferInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mat1_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.mat1_num as usize * pak::MatBase::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mat2_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.mat2_num as usize * pak::Mat2::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mat3_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.mat3_num as usize * pak::Mat3::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mat4_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.mat4_num as usize * pak::Mat4::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mat_extra_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.mat_extra_num as usize * pak::MatExtra::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.shape_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.shape_info_num as usize * pak::ShapeInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.hk_shape_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.hk_shape_info_num as usize * HkShape0::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.hk_constraint_data_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.hk_constraint_data_num as usize * pak::HkConstraintData::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.vbuff_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.vbuff_info_num as usize * pak::VBuffInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.ibuff_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.ibuff_info_num as usize * pak::IBuffInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.texture_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.texture_info_num as usize * pak::TextureInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.animation_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.animation_info_num as usize * pak::AnimationInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.hk_constraint_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.hk_constraint_info_num as usize * pak::HkConstraintInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.effect_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.effect_info_num as usize * pak::EffectInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.pfield_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.pfield_info_num as usize * pak::PFieldInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.gfx_block_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.gfx_block_info_num as usize * pak::GFXBlockInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.animation_block_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.animation_block_info_num as usize * pak::AnimationBlockInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.foliage_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.foliage_info_num as usize * pak::FoliageInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.radiosity_vals_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.radiosity_vals_info_num as usize * pak::RadiosityValsInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);

        let mut infos = DumpInfos {
            header: pak_header.clone(),
            ..Default::default()
        };

        // infos done
        let mut offset = 0;
        let animation_vals = self.animations.iter().sorted_by(|a, b| a.0.key().cmp(&b.0.key())).map(|(key, anim)| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let vals = anim.dump::<O>(offset, &mut infos)?;
            offset += vals.len();
            Ok((vals, anim.info.gamemodemask))
        }).collect::<Result<Vec<_>>>()?;
        let animations_blocks = (0..self.animation_block_infos.len() as u32).map(|i| {
            let gamemodemask = 1i32 << i;
            animation_vals.iter().filter(|(_, k)| k & gamemodemask != 0).flat_map(|(x, _)| x).cloned().collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        
        let mut animation_block_infos = self.animation_block_infos.clone();
        for (info, data) in zip(&mut animation_block_infos, animations_blocks) {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
            pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
            let size = data.len();
            let data = CompressedBlock { data }.dump();
            info.offset = pak_data.len() as u32;
            info.size = size as u32;
            info.size_comp = data.len() as u32;
            pak_data.extend(data);
        }
        info!("animations in {:?}", time.elapsed());
        bar.as_ref().map(|x| { x.inc(1); x.set_message("effects") }); 
        sub_bar.as_ref().map(|x| { x.set_length(self.effects.len() as u64); x.reset() });

        let effects = self.effects.iter().sorted_by(|a, b| a.0.key().cmp(&b.0.key())).map(|(key, effect)| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let vals = effect.dump::<O>()?;
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
        if let Some(SubBlock::GameObjs(objs)) = self.sub_blocks1.blocks.last() {
            let guid_order: HashMap<usize, usize> = HashMap::from_iter(objs.objs.iter().enumerate().map(|(i, obj)|{
                let ts = objs.types.get(&obj.key).unwrap();
                (ts.iter().zip(&obj.fields).find_map(|(ty, val)| {
                    if let BaseTypes::GUID(guid) = val {
                        (ty.key.key() == 3482846511).then_some(*guid as usize)
                    } else {
                        None
                    }
                }).unwrap(), i)
            }));
            collision_road.sort_unstable_by_key(|x| 
                guid_order.get(&x.str().and_then(|x| x.split('_').last().and_then(|x| x.parse::<usize>().ok())).unwrap_or_default())
            );    

        } else {
            collision_road.sort_unstable_by_key(|x| 
                x.str().and_then(|x| x.split('_').last().and_then(|x| x.parse::<usize>().ok())).unwrap_or_default()
            );    
        }
        // should be sorted in the order that they appear the level block
        for key in normal.into_iter().chain(collision_road) {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let mut model = self.models.get(key).unwrap().clone();
            if let Some(val) = dump_vertex_data::<O>(&mut model) {
                model_data.push(val);
            }
            block1.extend(model.dump::<O>(block1.len(), &mut infos));
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        }
        let terrain_start_offset = block1.len() as u32;
        block1.extend(vec![0xFFu8; 16]);
        for key in terrain {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let mut model = self.models.get(key).unwrap().clone();
            if let Some(val) = dump_vertex_data::<O>(&mut model) {
                model_data.push(val);
            }
            block1.extend(model.dump_terrain::<O>(block1.len(), terrain_start_offset, &mut infos));
        }
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);

        let foliages = self.foliages.iter().flat_map(|(key, x)| { 
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            x
        }).map(|(info, val)| {
            let mut info = info.clone();
            info.offset = block1.len() as u32;
            block1.extend(val.dump::<O>());
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
            info
        }).collect::<Vec<_>>();

        if let Some(model) = self.models.get(&Crc::Key(key_occluder)) {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(model.info.key.to_string())});
            let mut model = model.clone();
            if let Some(val) = dump_vertex_data::<O>(&mut model) {
                model_data.push(val);
            }
            block1.extend(model.dump::<O>(block1.len(), &mut infos));
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        }
        info!("models & foliages in {:?}", time.elapsed());
        bar.as_ref().map(|x| { x.inc(1); x.set_message("block1 objs") }); 
        sub_bar.as_ref().map(|x| { x.set_length((self.gfx_blocks.len() + self.radiosity_vals.len()) as u64); x.reset() });

        let gfx_blocks = self.gfx_blocks.iter().sorted_by(|a,b| a.0.cmp(&b.0)).map(|(key, val)| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let gfx_block = pak::GFXBlockInfo { key: key.clone(), offset: block1.len() as u32, size: val.len() as u32 };
            block1.extend(val.clone());
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
            gfx_block
        }).collect::<Vec<_>>();

        let radiosity_vals = self.radiosity_vals.iter().sorted_by_key(|(_, val)| val.first()).map(|(&guid, val)| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(format!("{}", guid))});
            let radiosity_vals = pak::RadiosityValsInfo { guid, num: val.len() as u32, offset: block1.len() as u32 };
            block1.extend(val.dump_bytes::<O>());
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
            radiosity_vals
        }).collect::<Vec<_>>();
        info!("block1 objs in {:?}", time.elapsed());
        bar.as_ref().map(|x| { x.inc(1); x.set_message("block1") }); 
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });


        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.sub_blocks1_offset = block1.len() as u32;
        block1.extend(self.sub_blocks1.dump::<O>(&lua, sub_bar.as_ref())?);
        pak_header.string_keys_offset = block1.len() as u32;
        block1.extend(self.string_keys.dump::<O>());

        self.objas.to_bytes::<O>(&mut block1[pak_header.obja_offset as usize..])?;
        self.obj0s.to_bytes::<O>(&mut block1[pak_header.obj0_offset as usize..])?;
        infos.model.to_bytes::<O>(&mut block1[pak_header.model_info_offset as usize..])?;
        infos.buffer.to_bytes::<O>(&mut block1[pak_header.buffer_info_offset as usize..])?;
        infos.mat1.to_bytes::<O>(&mut block1[pak_header.mat1_offset as usize..])?;
        infos.mat2.to_bytes::<O>(&mut block1[pak_header.mat2_offset as usize..])?;
        infos.mat3.to_bytes::<O>(&mut block1[pak_header.mat3_offset as usize..])?;
        infos.mat4.to_bytes::<O>(&mut block1[pak_header.mat4_offset as usize..])?;
        infos.mat_extra.to_bytes::<O>(&mut block1[pak_header.mat_extra_offset as usize..])?;
        infos.shape.to_bytes::<O>(&mut block1[pak_header.shape_info_offset as usize..])?;
        for (i, hk_shape) in infos.hk_shape.iter().enumerate() {
            hk_shape.to_bytes::<O>(&mut block1[pak_header.hk_shape_info_offset as usize + i * HkShape0::size::<O>()..])?;
        }
        infos.hk_constraint_data.to_bytes::<O>(&mut block1[pak_header.hk_constraint_data_offset as usize..])?;
        infos.vbuff.to_bytes::<O>(&mut block1[pak_header.vbuff_info_offset as usize..])?;
        infos.ibuff.to_bytes::<O>(&mut block1[pak_header.ibuff_info_offset as usize..])?;
        texture_infos.to_bytes::<O>(&mut block1[pak_header.texture_info_offset as usize..])?;
        infos.animation.to_bytes::<O>(&mut block1[pak_header.animation_info_offset as usize..])?;
        infos.hk_constraint.to_bytes::<O>(&mut block1[pak_header.hk_constraint_info_offset as usize..])?;
        effects.to_bytes::<O>(&mut block1[pak_header.effect_info_offset as usize..])?;
        foliages.to_bytes::<O>(&mut block1[pak_header.foliage_info_offset as usize..])?;
        self.pfield_infos.to_bytes::<O>(&mut block1[pak_header.pfield_info_offset as usize..])?;
        gfx_blocks.to_bytes::<O>(&mut block1[pak_header.gfx_block_info_offset as usize..])?;
        radiosity_vals.to_bytes::<O>(&mut block1[pak_header.radiosity_vals_info_offset as usize..])?;
        animation_block_infos.to_bytes::<O>(&mut block1[pak_header.animation_block_info_offset as usize..])?;
        info!("block1 in {:?}", time.elapsed());
        bar.as_ref().map(|x| { x.inc(1); x.set_message("block2") }); 
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });

        // block2
        for (i, model) in infos.model.iter().enumerate() {
            infos.block2_offsets.extend([
                pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 8,
                pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 12,
                pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 48,
                pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 140,
                pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 144,
                pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 152,
                pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 164,
                pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 172,
                pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 180,
                pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 252,
            ]);
            if model.bones_offset != 0 {
                infos.block2_offsets.push(pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 136);
            }
            if model.skin_order_offset != 0 {
                infos.block2_offsets.push(pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 160);
            }
            if model.vals_j_offset != 0 {
                infos.block2_offsets.push(pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 196);
            }
            if model.block_offset != 0 {
                infos.block2_offsets.push(pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 200);
            }
            if model.vals_k_offset != 0 {
                infos.block2_offsets.push(pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 204);
            }
            if model.shape_offset != 0 {
                infos.block2_offsets.push(pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 224);
            }
            if model.hk_constraint_data_offset != 0 {
                infos.block2_offsets.push(pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 232);
            }
            if model.hk_constraint_offset != 0 {
                infos.block2_offsets.push(pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 240);
            }
            if model.slots_offset != 0 {
                infos.block2_offsets.push(pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 244);
            }
            if model.slot_map_offset != 0 {
                infos.block2_offsets.push(pak_header.model_info_offset + (i * pak::ModelInfo::size::<O>()) as u32 + 248);
            }
        }
        for (i, buffer) in infos.buffer.iter().enumerate() {
            infos.block2_offsets.extend([
                pak_header.buffer_info_offset + (i * pak::BufferInfo::size::<O>()) as u32,
                pak_header.buffer_info_offset + (i * pak::BufferInfo::size::<O>()) as u32 + 260,
            ]);
            if buffer.vbuff_info_offset_2 != 0 {
                infos.block2_offsets.push(pak_header.buffer_info_offset + (i * pak::BufferInfo::size::<O>()) as u32 + 4);
            }
            if buffer.vbuff_info_offset_3 != 0 {
                infos.block2_offsets.push(pak_header.buffer_info_offset + (i * pak::BufferInfo::size::<O>()) as u32 + 8);
            }
        }
        for (i, mat) in infos.mat1.iter().enumerate() {
            if mat.base.mat_extra_offset != 0 {
                infos.block2_offsets.push(pak_header.mat1_offset + (i * pak::MatBase::size::<O>()) as u32 + 344);
            }
        }
        for (i, mat) in infos.mat2.iter().enumerate() {
            if mat.base.mat_extra_offset != 0 {
                infos.block2_offsets.push(pak_header.mat2_offset + (i * pak::Mat2::size::<O>()) as u32 + 344);
            }
        }
        for (i, mat) in infos.mat3.iter().enumerate() {
            if mat.base.mat_extra_offset != 0 {
                infos.block2_offsets.push(pak_header.mat3_offset + (i * pak::Mat3::size::<O>()) as u32 + 344);
            }
        }
        for (i, mat) in infos.mat4.iter().enumerate() {
            if mat.base.mat_extra_offset != 0 {
                infos.block2_offsets.push(pak_header.mat4_offset + (i * pak::Mat4::size::<O>()) as u32 + 344);
            }
        }
        for (i, shape) in infos.shape.iter().enumerate() {
            if shape.hk_shape_offset != 0 {
                infos.block2_offsets.push(pak_header.shape_info_offset + (i * pak::ShapeInfo::size::<O>()) as u32 + 112);
            }
        }
        for (i, hk_shape) in infos.hk_shape.iter().enumerate() {
            infos.block2_offsets.extend(match hk_shape {
                HkShapeInfo::ConvexVertices(_) => vec![
                    pak_header.hk_shape_info_offset + (i * pak::HkShapeInfo::size::<O>()) as u32 + 44,
                    pak_header.hk_shape_info_offset + (i * pak::HkShapeInfo::size::<O>()) as u32 + 52
                ],
                HkShapeInfo::BVTreeMesh(_) => vec![
                    pak_header.hk_shape_info_offset + (i * pak::HkShapeInfo::size::<O>()) as u32 + 60,
                    pak_header.hk_shape_info_offset + (i * pak::HkShapeInfo::size::<O>()) as u32 + 68,
                    pak_header.hk_shape_info_offset + (i * pak::HkShapeInfo::size::<O>()) as u32 + 76
                ],
                _ => vec![]
            });
        }
        for (i, hk_constraint) in infos.hk_constraint.iter().enumerate() {
            infos.block2_offsets.extend([
                pak_header.hk_constraint_info_offset + (i * pak::HkConstraintInfo::size::<O>()) as u32 + 4,
                pak_header.hk_constraint_info_offset + (i * pak::HkConstraintInfo::size::<O>()) as u32 + 12,
                pak_header.hk_constraint_info_offset + (i * pak::HkConstraintInfo::size::<O>()) as u32 + 20,
                pak_header.hk_constraint_info_offset + (i * pak::HkConstraintInfo::size::<O>()) as u32 + 40,
                pak_header.hk_constraint_info_offset + (i * pak::HkConstraintInfo::size::<O>()) as u32 + 48,
            ]);
            if hk_constraint.vals2_offset != 0 {
                infos.block2_offsets.push(pak_header.hk_constraint_info_offset + (i * pak::HkConstraintInfo::size::<O>()) as u32 + 64);
            }
        }
        for i in 0..pak_header.effect_info_num as usize {
            infos.block2_offsets.push(pak_header.effect_info_offset + (i * pak::EffectInfo::size::<O>()) as u32 + 8);
        }
        for i in 0..pak_header.gfx_block_info_num as usize {
            infos.block2_offsets.push(pak_header.gfx_block_info_offset + (i * pak::GFXBlockInfo::size::<O>()) as u32 + 4);
        }
        for i in 0..pak_header.radiosity_vals_info_num as usize {
            infos.block2_offsets.push(pak_header.radiosity_vals_info_offset + (i * pak::RadiosityValsInfo::size::<O>()) as u32 + 8);
        }
        for i in 0..pak_header.foliage_info_num as usize {
            infos.block2_offsets.push(pak_header.foliage_info_offset + (i * pak::FoliageInfo::size::<O>()) as u32 + 28);
        }
        pak_header.sub_blocks2_offset = 0;
        let mut block2 = self.sub_blocks2.dump::<O>(&lua, sub_bar.as_ref())?;
        pak_header.block2_offsets_offset = block2.len() as u32;
        pak_header.block2_offsets_num = infos.block2_offsets.len() as u32;
        block2.extend(infos.block2_offsets.dump_bytes::<O>());
        info!("block2 in {:?}", time.elapsed());
        bar.as_ref().map(|x| { x.inc(1); x.set_message("pak") }); 
        sub_bar.as_ref().map(|x| { x.set_length(4); x.reset() });

        // rest of pak
        sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("block1")});
        pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
        let size = block1.len();
        let data = CompressedBlock { data: block1 }.dump();
        pak_header.block1_size = size as u32;
        pak_header.block1_size_comp = data.len() as u32;
        pak_header.block1_offset = pak_data.len() as u32;
        pak_data.extend(data);

        sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("block2")});
        pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
        let size = block2.len();
        let data = CompressedBlock { data: block2 }.dump();
        pak_header.block2_size = size as u32;
        pak_header.block2_size_comp = data.len() as u32;
        pak_header.block2_offset = pak_data.len() as u32;
        pak_data.extend(data);

        sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("strings")});
        pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
        let data = self.pak_strings.dump::<O>();
        pak_header.strings_offset = pak_data.len() as u32;
        pak_header.strings_num = self.pak_strings.strings.len() as u32;
        pak_header.strings_size = data.len() as u32;
        pak_data.extend(data);

        sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("vals_a")});
        pak_header.block_a_offset = pak_data.len() as u32;
        pak_header.block_a_num = self.pak_vals_a.len() as u32;
        pak_data.extend(self.pak_vals_a.dump_bytes::<O>());

        pak_data.extend(vec![0u8; ((pak_data.len() + 2047) & 0xfffff800)-pak_data.len()]);
        pak_header.to_bytes::<O>(&mut pak_data)?;
        info!("pak in {:?}", time.elapsed());
        bar.as_ref().map(|x| { x.inc(1); x.set_message("bin") }); 
        sub_bar.as_ref().map(|x| { x.set_length((model_data.len() + texture_data.len() + self.radiosity.len() + 2) as u64); x.reset() });

        // bin_data
        let mut bin_header = self.bin_header.clone();
        let mut bin_data = vec![0u8; bin::Header::size::<O>()];
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
                let data = CompressedBlock { data }.dump();
                let size_comp = data.len() as u32;
                bin_data.extend(data);
                bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
                size_comp
            } else { 0 };
            bin::AssetHandle { key, offset, size, size_comp, kind }
        }).collect::<Vec<_>>();

        let mut texture_asset_handles = texture_data.into_iter().map(|((key, kind), data)| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let size = data.len() as u32;
            let offset = bin_data.len() as u32;
            let size_comp = if size != 0 {
                let data = CompressedBlock { data }.dump();
                let size_comp = data.len() as u32;
                bin_data.extend(data);
                bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
                size_comp
            } else { 0 };
            bin::AssetHandle { key, offset, size, size_comp, kind }
        }).collect::<Vec<_>>();

        model_asset_handles.extend(self.radiosity.iter().map(|(key, data)| {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let kind = data.usage;
            let data = data.data.dump_bytes::<O>();
            let size = data.len() as u32;
            let offset = bin_data.len() as u32;
            let size_comp = if size != 0 {
                let data = CompressedBlock { data }.dump();
                let size_comp = data.len() as u32;
                bin_data.extend(data);
                bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
                size_comp
            } else { 0 };
            bin::AssetHandle { key: key.clone(), offset, size, size_comp, kind }
        }));

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
        bin_data.extend(asset_handles.dump_bytes::<O>());

        sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("strings")});
        let data = self.bin_strings.dump::<O>();
        bin_header.strings_offset = bin_data.len() as u32;
        bin_header.strings_size = data.len() as u32;
        bin_header.strings_num = self.bin_strings.strings.len() as u32;
        bin_data.extend(data);
        
        sub_bar.as_ref().map(|x| { x.inc(1); x.set_message("header")});
        bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
        bin_header.to_bytes::<O>(&mut bin_data)?;
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

    pub fn to_file(&self, writer: Writer, mp: Option<MultiProgress>) -> Result<()> {
        let time: Instant = Instant::now();
        info!("storing level");

        let (bar, sub_bar) = if let Some(mp) = mp {
            let bar = ProgressBar::new(13 + types::ANIM_TABLES.lock().unwrap().then_some(1).unwrap_or(0)).with_style(ProgressStyle::with_template(BAR_FMT).unwrap());
            let sub_bar = ProgressBar::new(0).with_style(ProgressStyle::with_template(SUB_BAR_FMT).unwrap());
            mp.add(bar.clone());
            mp.add(sub_bar.clone());
            (Some(bar), Some(sub_bar))
        } else { (None, None) };


        // std::fs::create_dir_all(path.join("assets").join("raw")).ok();
    
        bar.as_ref().map(|x| { x.inc(1); x.set_message("headers") }); 
        writer.join("bin_header.json").write(&to_vec_pretty(&self.bin_header)?)?;
        self.bin_strings.to_file(writer.join("bin_strings"))?;
        writer.join("pak_header.json").write(&to_vec_pretty(&self.pak_header)?)?;
        self.pak_strings.to_file(writer.join("pak_strings"))?;

        self.string_keys.to_file(writer.join("string_keys"))?;
        info!("headers in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("unused objs") }); 
        writer.join("objas.json").write(&to_vec_pretty(&self.objas)?)?;
        writer.join("obj0s.json").write(&to_vec_pretty(&self.obj0s)?)?;
        writer.join("pak_vals_a.json").write(&to_vec_pretty(&self.pak_vals_a)?)?;
        info!("unused objs in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("models") }); 
        sub_bar.as_ref().map(|x| { x.set_length(self.models.len() as u64); x.reset() });
        if *types::GLTF.lock().unwrap() {
            for (key, model) in self.models.iter() {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
                writer.join("models").join(key.to_string()).with_extension("glb").write(&model.to_gltf()?.to_vec()?)?;
            }
        } else {
            for (key, data) in self.models.iter() {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
                writer.join("models").join(key.to_string()).with_extension("json").write(&to_vec_pretty(&data)?)?;
            }
        }
        info!("models in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("effects") }); 
        sub_bar.as_ref().map(|x| { x.set_length(self.effects.len() as u64); x.reset() });
        for (key, data) in self.effects.iter() {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            data.to_file(writer.join("effects").join(key.to_string()))?;
        }
        info!("effects in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("foliage objs") }); 
        sub_bar.as_ref().map(|x| { x.set_length(self.foliages.len() as u64); x.reset() });
        for (key, data) in self.foliages.iter() {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            writer.join("foliage").join(key.to_string()).with_extension("json").write(&to_vec_pretty(&data)?)?;
        }
        info!("foliage objs in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("radiosity_vals") }); 
        sub_bar.as_ref().map(|x| { x.set_length(self.radiosity_vals.len() as u64); x.reset() });
        for (key, data) in self.radiosity_vals.iter() {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            writer.join("radiosity_vals").join(format!("{}", key)).with_extension("bin").write(&data.dump_bytes::<PC>())?;
        }
        info!("radiosity vals in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("gfxs") }); 
        sub_bar.as_ref().map(|x| { x.set_length(self.gfx_blocks.len() as u64); x.reset() });
        for (key, data) in self.gfx_blocks.iter() {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            writer.join("gfxs").join(key.to_string()).with_extension("gfx").write(data)?;
        }
        info!("gfxs in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("animations") }); 
        sub_bar.as_ref().map(|x| { x.set_length(self.animations.len() as u64); x.reset() });

        writer.join("animation_block_infos.json").write(&to_vec_pretty(&self.animation_block_infos)?)?;
        for (key, data) in self.animations.iter() {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            writer.join("animations").join(key.to_string()).with_extension("json").write(&to_vec_pretty(&data)?)?;
        }
        info!("animations in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("textures") }); 
        sub_bar.as_ref().map(|x| { x.set_length(self.textures.len() as u64); x.reset() });
        for (key, tex) in self.textures.iter() {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            tex.to_file(writer.join("textures").join(key.to_string()))?;
        }
        info!("textures in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("radiosity") }); 
        sub_bar.as_ref().map(|x| { x.set_length(self.radiosity.len() as u64); x.reset() });
        for (key, data) in self.radiosity.iter() {
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            writer.join("radiosity").join(key.to_string()).with_extension("json").write(&to_vec_pretty(&data)?)?;
        }
        info!("radiosity in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("paked_items") }); 
        sub_bar.as_ref().map(|x| { x.set_length(1); x.reset() });
        writer.join("pfield_infos.json").write(&to_vec_pretty(&self.pfield_infos)?)?;

        info!("packed items in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("sub_blocks1") }); 
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        self.sub_blocks1.to_file(writer.join("sub_blocks1"), &self.string_keys, sub_bar.as_ref())?;
        info!("sub block1 in {:?}", time.elapsed());
        
        bar.as_ref().map(|x| { x.inc(1); x.set_message("sub_blocks2") }); 
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        self.sub_blocks2.to_file(writer.join("sub_blocks2"), &self.string_keys, sub_bar.as_ref())?;
        info!("sub block2 in {:?}", time.elapsed());

        if *types::ANIM_TABLES.lock().unwrap() {
            bar.as_ref().map(|x| { x.inc(1); x.set_message("animation tables") }); 
            sub_bar.as_ref().map(|x| { x.set_length(self.sub_blocks1.blocks.len() as u64 * 2); x.reset() });
            let mut script_manager = HashMap::new();
            {
                let lua = lua_stuff::LuaCompiler::new().unwrap();
                for (block, info) in self.sub_blocks1.blocks.iter().zip(&self.sub_blocks1.block_headers) {
                    sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(info.key.to_string())});
                    if let types::SubBlock::Lua(val) = block {
                        let mut name = val.name.clone();
                        name.truncate(name.len()-4);
                        //println!("{:?}", val.data);
                        script_manager.insert(Crc::from_string(&name), val.conv("L4808", &lua)?);
                        //script_manager.insert(Crc::from_string(&name), lua.convert(&val.data, "L4808")?);
                    }
                }
            }
            
            let anim_scripts = script_manager.keys().filter_map(|x| x.str().and_then(|x| x.starts_with("ANM_").then_some(x.to_string()))).collect::<Vec<_>>();
            sub_bar.as_ref().map(|x| x.set_length(x.length().unwrap() - self.sub_blocks1.blocks.len() as u64 + anim_scripts.len() as u64));
            let script_manager = Arc::new(script_manager);
            for anim in anim_scripts.into_iter() {
                sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(anim.clone())});
                let val = lua_stuff::load_anim(script_manager.clone(), anim.clone());
                writer.join("animation_tables").join(anim).with_extension("json").write(&to_vec_pretty(&val)?)?;
            }
            info!("animation tables in {:?}", time.elapsed());
        }
        sub_bar.as_ref().map(|x| x.finish_and_clear());
        bar.as_ref().map(|x| x.finish_and_clear());
        Ok(())
    }

    pub fn from_file(reader: Reader, mp: Option<&MultiProgress>) -> Result<Self> {
        let time: Instant = Instant::now();
        info!("Reading level {:?}", reader.full_path());        

        let (bar, sub_bar) = if let Some(mp) = mp {
            let bar = ProgressBar::new(13).with_style(ProgressStyle::with_template(BAR_FMT).unwrap());
            let sub_bar = ProgressBar::new(0).with_style(ProgressStyle::with_template(SUB_BAR_FMT).unwrap());
            mp.add(bar.clone());
            mp.add(sub_bar.clone());
            (Some(bar), Some(sub_bar))
        } else { (None, None) };
        
        let lua: lua_stuff::LuaCompiler = lua_stuff::LuaCompiler::new().unwrap();

        bar.as_ref().map(|x| { x.inc(1); x.set_message("headers") }); 
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        let bin_header = serde_json::from_slice::<bin::Header>(&reader.join("bin_header.json").read()?)?;
        let bin_strings = types::Strings::from_file(reader.join("bin_strings"))?;
        types::update_strings(&bin_strings.strings);

        let pak_header = serde_json::from_slice::<pak::Header>(&reader.join("pak_header.json").read()?)?;
        let pak_strings = types::Strings::from_file(reader.join("pak_strings"))?;
        types::update_strings(&pak_strings.strings);

        let string_keys = types::StringKeys::from_file(reader.join("string_keys"))?;
        info!("headers in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("unused objs") }); 
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        let objas = serde_json::from_slice::<Vec<pak::ObjA>>(&reader.join("objas.json").read()?)?;
        let obj0s = serde_json::from_slice::<Vec<pak::Obj0>>(&reader.join("obj0s.json").read()?)?;
        let pak_vals_a = serde_json::from_slice::<Vec<pak::BlockAVal>>(&reader.join("pak_vals_a.json").read()?)?;
        info!("unused objs in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("models") }); 
        let data = reader.join("models").into_iter();
        sub_bar.as_ref().map(|x| { x.set_length(data.len() as u64); x.reset() });
        let mut models = HashMap::new();
        for path in data {
            let key = Crc::from_string(path.name());
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            if let Some(data) = match path.path().extension().and_then(|x| x.to_str()) {
                Some("glb") => {
                    let data = path.read()?;
                    let glb = gltf::Glb::from_slice(&data)?;
                    let bin = glb.bin.as_ref().unwrap();
                    let root = gltf::json::Root::from_slice(&glb.json)?;
                    Some(Model::from_gltf(&root, bin)?)
                },
                Some("json") => Some(serde_json::from_slice::<Model>(&path.read()?)?),
                _ => None

            } {
                models.insert(key, data);
            }
        }
        info!("models in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("effects") }); 
        let data = reader.join("effects").into_iter();
        sub_bar.as_ref().map(|x| { x.set_length(data.len() as u64); x.reset() });
        let mut effects = HashMap::new();
        for path in data {
            let key = Crc::from_string(path.name());
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let data = GameObjs::from_file(path)?;
            effects.insert(key, data);
        }
        info!("effects in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("foliage objs") }); 
        let data = reader.join("foliage").into_iter();
        sub_bar.as_ref().map(|x| { x.set_length(data.len() as u64); x.reset() });
        let mut foliages = HashMap::new();
        for path in data {
            let key = Crc::from_string(path.name());
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let data = serde_json::from_slice::<Vec<(pak::FoliageInfo, pak::Foliage)>>(&path.read()?)?;
            foliages.insert(key, data);
        }
        info!("foliage objs in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("radiosity vals") }); 
        let data = reader.join("radiosity_vals").into_iter();
        sub_bar.as_ref().map(|x| { x.set_length(data.len() as u64); x.reset() });
        let mut radiosity_vals = HashMap::new();
        for path in data {
            let key: u32 = path.name().parse().unwrap();
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let dat = path.read()?;
            let data = <Vec<u32> as OrderedDataVec>::from_bytes::<PC>(&dat, dat.len()/4)?;
            radiosity_vals.insert(key, data);
        }
        info!("radiosity vals in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("gfxs") }); 
        let data = reader.join("gfxs").into_iter();
        sub_bar.as_ref().map(|x| { x.set_length(data.len() as u64); x.reset() });
        let mut gfx_blocks = HashMap::new();
        for path in data {
            let key = Crc::from_string(path.name());
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let data = path.read()?;
            gfx_blocks.insert(key, data);
        }
        info!("gfxs in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("animations") }); 
        let data = reader.join("animations").into_iter();
        sub_bar.as_ref().map(|x| { x.set_length(data.len() as u64); x.reset() });
        let animation_block_infos = serde_json::from_slice::<Vec<pak::AnimationBlockInfo>>(&reader.join("animation_block_infos.json").read()?)?;
        let mut animations = HashMap::new();
        for path in data {
            let key = Crc::from_string(path.name());
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let data = serde_json::from_slice::<Animation>(&path.read()?)?;
            animations.insert(key, data);
        }
        info!("animations in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("textures") }); 
        let data = reader.join("textures").into_iter();
        sub_bar.as_ref().map(|x| { x.set_length(data.len() as u64); x.reset() });
        let mut textures = HashMap::new();
        for path in data.filter(|x| x.path().extension().unwrap_or(OsStr::new("")).to_str() == Some("json")) {
            let key = Crc::from_string(path.name());
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let data = bin::Tex::from_file(path)?;
            textures.insert(key, data);
        }
        info!("textures in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("radiosity") }); 
        let data = reader.join("radiosity").into_iter();
        sub_bar.as_ref().map(|x| { x.set_length(data.len() as u64); x.reset() });
        let mut radiosity = HashMap::new();
        for path in data {
            let key = Crc::from_string(path.name());
            sub_bar.as_ref().map(|x| { x.inc(1); x.set_message(key.to_string())});
            let data = serde_json::from_slice::<bin::Radiosity>(&path.read()?)?;
            radiosity.insert(key, data);
        }
        info!("radiosity in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("packed items") }); 
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        let pfield_infos = serde_json::from_slice::<Vec<pak::PFieldInfo>>(&reader.join("pfield_infos.json").read()?)?;
        info!("packed items in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("sub_blocks1") }); 
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        let sub_blocks1 = types::SubBlocks::from_file(reader.join("sub_blocks1"), &lua, sub_bar.as_ref())?;
        info!("sub_blocks1 in {:?}", time.elapsed());

        bar.as_ref().map(|x| { x.inc(1); x.set_message("sub_blocks2") }); 
        sub_bar.as_ref().map(|x| { x.set_length(0); x.reset() });
        let sub_blocks2 = types::SubBlocks::from_file(reader.join("sub_blocks2"), &lua, sub_bar.as_ref())?;
        info!("sub_blocks2 in {:?}", time.elapsed());

        bar.as_ref().map(|x| x.finish_and_clear());
        sub_bar.as_ref().map(|x| x.finish_and_clear());

        Ok(Self {
            bin_header,
            bin_strings,
            pak_header,
            pak_strings,
            objas,
            obj0s,
            models,
            textures,
            animations,
            foliages,
            radiosity_vals,
            effects,
            pfield_infos,
            animation_block_infos,
            string_keys,
            sub_blocks1,
            sub_blocks2,
            block2_offsets: Vec::new(),
            radiosity,
            //vertex_formats: HashMap::new(),
            pak_vals_a,
            gfx_blocks,
        })
    }
}
