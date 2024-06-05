use std::{any::TypeId, collections::HashMap, fs::{self, File}, path::Path};
use itertools::Itertools;
use zerocopy::{ByteOrder, LE, BE};
use log::warn;
use serde::{Serialize, Deserialize};
// use rmp_serde::Serializer;
// use serde_cbor::{Serializer, Deserializer, ser::IoWrite, de::IoRead};
use serde_json::{map::Iter, to_string_pretty};
use std::time::Instant;
use std::iter::zip;
use lotrc_rs_proc::OrderedData;

use crate::{pak::{AnimationBlockInfo, AnimationInfo, BufferInfo, EffectInfo, FoliageInfo, GFXBlockInfo, HkConstraintData, HkConstraintInfo, IBuffInfo, IlluminationInfo, Mat2, MatBase, MatExtra, MeshInfo, PFieldInfo, ShapeInfo, TextureInfo, VBuffInfo}, types::{CompressedBlock, Data}};

use super::{
    pak, bin, lua_stuff, pak_alt::*,
    types::{self, hash_string, GameObjs, OrderedData, OrderedDataVec, SubBlock, Crc}
};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Level {
    pub bin_header: bin::Header,
    pub bin_strings: types::Strings,
    pub pak_header: pak::Header,
    pub pak_strings: types::Strings,

    pub objas: Vec<pak::ObjA>,
    pub obj0s: Vec<pak::Obj0>,
    pub meshes: HashMap<Crc, Mesh>,
    pub textures: HashMap<Crc, Option<bin::Tex>>,
    pub animations: HashMap<Crc, Animation>,
    pub foliages: HashMap<Crc, Vec<(pak::FoliageInfo, Vec<u32>)>>,
    pub light_blocks: HashMap<u32, Vec<u32>>,
    pub effects: HashMap<Crc, GameObjs>,
    pub pfield_infos: Vec<pak::PFieldInfo>,
    pub animation_block_infos: Vec<pak::AnimationBlockInfo>,
    pub gfx_blocks: HashMap<Crc, Vec<u8>>,

    pub string_keys: types::StringKeys,
    pub sub_blocks1: types::SubBlocks,
    pub sub_blocks2: types::SubBlocks,
    pub block2_offsets: Vec<u32>,

    pub radiosity: HashMap<(Crc, u32), bin::Radiosity>,

    pub vertex_formats: HashMap<(u32, u32), (Vec<(u32, pak::VertexUsage)>, usize)>,

    pub pak_block_a: Vec<pak::BlockAVal>,
}

impl Level {
    pub fn parse<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let pak_data = fs::read(path.with_extension("PAK")).unwrap();
        let bin_data = fs::read(path.with_extension("BIN")).unwrap();    
        if bin_data[0] == 6 {
            Self::from_data::<LE>(&bin_data[..], &pak_data[..])
        } else if bin_data[3] == 6 {
            Self::from_data::<BE>(&bin_data[..], &pak_data[..])
        } else {
            warn!("Invalid level data");
            Default::default()
        }
    }

    pub fn dump<O: ByteOrder + 'static, P: AsRef<Path>>(&self, path: P, compress: bool) {
        let path = path.as_ref();
        let (pak, bin, infos) = self.to_data::<O>(compress);
        fs::write(path.with_extension("PAK"), pak).unwrap();
        fs::write(path.with_extension("BIN"), bin).unwrap();
        fs::write(path.with_extension("json"), serde_json::to_vec_pretty(&infos).unwrap()).unwrap();
    }

    pub fn from_data<O: ByteOrder + 'static>(bin_data: &[u8], pak_data: &[u8]) -> Self {
        let time = Instant::now();
        println!("extracting level");

        let lua = lua_stuff::LuaCompiler::new().unwrap();
        let bin_header: bin::Header = OrderedData::from_bytes::<O>(bin_data);
        let bin_strings = types::Strings::from_data::<O>(bin_data, bin_header.strings_offset as usize, bin_header.strings_num as usize);
        types::update_strings(&bin_strings.strings);

        let asset_handles: Vec<bin::AssetHandle> = OrderedDataVec::from_bytes::<O>(&bin_data[bin_header.asset_handle_offset as usize..], bin_header.asset_handle_num as usize);
        let asset_data = HashMap::<(Crc, u32), Vec<u8>>::from_iter(
            asset_handles.iter().map(|info| ((info.key.clone(), info.kind), types::CompressedBlock::from_data(bin_data, info.size as usize, info.size_comp as usize, info.offset as usize).data))
        );

        let radiosity: HashMap<(Crc, u32), bin::Radiosity> = asset_data.iter().filter(|(key, data)| key.0.str().map(|x| x.ends_with("_radiosity")).unwrap_or(false)).map(|(key, data)| (
            key.clone(), bin::Radiosity::from_data::<O>(&data[..])
        )).collect();

        println!("bin parsed in {:?}", time.elapsed());
        
        let pak_header: pak::Header = OrderedData::from_bytes::<O>(pak_data);
        let pak_strings = types::Strings::from_data::<O>(pak_data, pak_header.strings_offset as usize, pak_header.strings_num as usize);
        types::update_strings(&pak_strings.strings);
        println!("headers in {:?}", time.elapsed());

        let block2 = types::CompressedBlock::from_data(pak_data, pak_header.block2_size as usize, pak_header.block2_size_comp as usize, pak_header.block2_offset as usize).data;
        let sub_blocks2 = types::SubBlocks::from_data::<O>(&block2[..], pak_header.sub_blocks2_offset as usize, &lua);
        let block2_offsets = OrderedDataVec::from_bytes::<O>(&block2[pak_header.block2_offsets_offset as usize..], pak_header.block2_offsets_num as usize);
        println!("block2 parsed in {:?}", time.elapsed());

        let block1 = types::CompressedBlock::from_data(pak_data, pak_header.block1_size as usize, pak_header.block1_size_comp as usize, pak_header.block1_offset as usize).data;
        println!("main blocks extracted in {:?}", time.elapsed());

        let objas = OrderedDataVec::from_bytes::<O>(&block1[pak_header.obja_offset as usize..], pak_header.obja_num as usize);
        let obj0s = OrderedDataVec::from_bytes::<O>(&block1[pak_header.obj0_offset as usize..], pak_header.obj0_num as usize);
        let animation_block_infos: Vec<pak::AnimationBlockInfo> = OrderedDataVec::from_bytes::<O>(&block1[pak_header.animation_block_info_offset as usize..], pak_header.animation_block_info_num as usize);
        let pfield_infos = OrderedDataVec::from_bytes::<O>(&block1[pak_header.pfield_info_offset as usize..], pak_header.pfield_info_num as usize);

        let mut vertex_formats = HashMap::new();
        let meshes = (0..pak_header.mesh_info_num as usize).map(|i| {
            let mut mesh = Mesh::from_data::<O>(&block1[..], pak_header.mesh_info_offset as usize + i * pak::MeshInfo::size::<O>());
            if mesh.info.vbuff_num != 0 || mesh.info.ibuff_num != 0 {
                let buffer = asset_data.get(&(mesh.info.asset_key.clone(), mesh.info.asset_type)).unwrap();
                mesh.vertex_data.extend(mesh.vbuffs.iter().map(|info| pak::VertexBuffer::from_data::<O>(&buffer[..], info, &mut vertex_formats)));
                mesh.index_data.extend(mesh.ibuffs.iter().map(|info| pak::IndexBuffer::from_data::<O>(&buffer[..], info)));    
            }
            (mesh.info.key.clone(), mesh)
        }).collect::<HashMap<_, _>>();

        let effects = <Vec<pak::EffectInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.effect_info_offset as usize..], pak_header.effect_info_num as usize).into_iter().map(|info| (
            info.key, GameObjs::from_data::<O>(&block1[..], info.offset as usize, info.size as usize, info.level_flags)
        )).collect::<HashMap<_, _>>();

        let gfx_blocks = <Vec<pak::GFXBlockInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.gfx_block_info_offset as usize..], pak_header.gfx_block_info_num as usize).into_iter().map(|info| (
            info.key, block1[info.offset as usize..(info.offset + info.size) as usize].to_vec()
        )).collect::<HashMap<_, _>>();

        let light_blocks = <Vec<pak::IlluminationInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.illumination_info_offset as usize..], pak_header.illumination_info_num as usize).into_iter().map(|info| (
            info.guid, <Vec<u32> as OrderedDataVec>::from_bytes::<O>(&block1[info.offset as usize..], info.num as usize)
        )).collect::<HashMap<_, _>>();

        let mut foliages: HashMap<Crc, Vec<(FoliageInfo, Vec<u32>)>> = HashMap::new();
        for info in <Vec<pak::FoliageInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.foliage_info_offset as usize..], pak_header.foliage_info_num as usize) {
            foliages.entry(info.key.clone()).or_default().push((
                info.clone(), OrderedDataVec::from_bytes::<O>(&block1[info.offset as usize..], ((info.s1b - info.s1a) * (info.s2b - info.s2a)) as usize * 2)
            ))
        }

        let textures = <Vec<pak::TextureInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.texture_info_offset as usize..], pak_header.texture_info_num as usize).into_iter().map(|info| {
            let data0 = &asset_data.get(&(info.asset_key.clone(), info.asset_type)).unwrap();
            let data1 = &asset_data.get(&(Crc::Key(hash_string("*".as_bytes(), Some(info.asset_key.key()))), info.asset_type)).unwrap();
            let key = info.key.clone();
            let tex = match info.kind {
                0 | 7 | 8 => Some(bin::Tex::Texture(bin::Texture::from_data::<O>(&data0[..], &data1[..], info.clone()))),
                1 | 9 => Some(bin::Tex::CubeTexture(bin::CubeTexture::from_data::<O>(&data0[..], &data1[..], info))),
                _ => {
                    warn!("Unsupported Texture Type {}", info.kind);
                    None
                }
            };
            (key, tex)
        }).collect::<HashMap<_, _>>();

        let blocks = animation_block_infos.iter().map(|info| 
            types::CompressedBlock::from_data(&pak_data[..], info.size as usize, info.size_comp as usize, info.offset as usize).data
        ).collect::<Vec<_>>();
        let mut offsets = blocks.iter().map(|_| 0usize).collect::<Vec<_>>();
        let animations = (0..pak_header.animation_info_num as usize).map(|i| {
            let anim = Animation::from_data::<O>(&block1[pak_header.animation_info_offset as usize + i * pak::AnimationInfo::size::<O>()..], &mut offsets, &blocks);
            (anim.info.key.clone(), anim)
        }).collect::<HashMap<_, _>>();

        println!("items extracted in {:?}", time.elapsed());

        let sub_blocks1 = types::SubBlocks::from_data::<O>(&block1[..], pak_header.sub_blocks1_offset as usize, &lua);
        let string_keys = types::StringKeys::from_data::<O>(&block1[..], pak_header.string_keys_offset as usize);
        println!("sub blocks extracted in {:?}", time.elapsed());

        let pak_block_a = OrderedDataVec::from_bytes::<O>(&pak_data[pak_header.block_a_offset as usize..], pak_header.block_a_num as usize);
        println!("buffers extracted in {:?}", time.elapsed());

        Self {
            bin_header,
            bin_strings,
            pak_header,
            pak_strings,
            objas,
            obj0s,
            meshes,
            textures,
            animations,
            foliages,
            light_blocks,
            effects,
            pfield_infos,
            animation_block_infos,
            string_keys,
            sub_blocks1,
            sub_blocks2,
            block2_offsets,
            radiosity,
            vertex_formats,
            pak_block_a,
            gfx_blocks,
        }
    }
    
    pub fn to_data<O: ByteOrder + 'static>(&self, compress: bool) -> (Vec<u8>, Vec<u8>, DumpInfos) {
        let lua: lua_stuff::LuaCompiler = lua_stuff::LuaCompiler::new().unwrap();
        
        // bin_data
        let mut asset_data = indexmap::IndexMap::new();
        let mut bin_header = self.bin_header.clone();
        let mut bin_data = vec![0u8; bin::Header::size::<O>()];
        bin_header.version = if TypeId::of::<O>() == TypeId::of::<LE>() { 2 } else { 1 };

        let mut meshes = self.meshes.clone();
        asset_data.extend(
            self.radiosity.iter().map(|(a, b)| (a.clone(), b.dump::<O>())).chain(
                meshes.values_mut().filter_map(|mesh| {
                    if mesh.vertex_data.len() != 0 || mesh.index_data.len() != 0 {
                        let size = mesh.vbuffs.iter().map(|x| x.size + x.offset).chain(mesh.ibuffs.iter().map(|x| x.size + x.offset)).max().unwrap();
                        let mut data = vec![0u8; size as usize];
                        for (vbuff, buff) in zip(&mesh.vbuffs, &mesh.vertex_data) {
                            buff.into_data::<O>(&mut data, vbuff);
                        }
                        for (ibuff, buff) in zip(&mesh.ibuffs, &mesh.index_data) {
                            buff.into_data::<O>(&mut data[ibuff.offset as usize..]);
                        }
                        // let vbuffs = mesh.vbuffs.iter_mut().map(|x| (&mut x.offset, x.size)).collect::<Vec<_>>();
                        // let ibuffs = mesh.ibuffs.iter_mut().map(|x| (&mut x.offset, x.size)).collect::<Vec<_>>();
                        // let vertex_data = mesh.vertex_data.iter().map(|x| x.dump::<O>()).collect::<Vec<_>>();
                        // let index_data = mesh.index_data.iter().map(|x| x.dump::<O>()).collect::<Vec<_>>();
                        // for ((off, size), dat) in zip(vbuffs, vertex_data).interleave(zip(ibuffs, index_data)) {
                        //     *off = data.len() as u32;
                        //     assert!(size == dat.len() as u32);
                        //     data.extend(dat);
                        //     data.extend(vec![0u8; ((data.len() + 32) & 0xFFFFFFE0)-data.len()]);
                        // }
                        Some(((mesh.info.asset_key.clone(), mesh.info.asset_type), data))
                    } else {
                        None
                    }
                })
            ).sorted_by(|a,b| a.0.0.key().cmp(&b.0.0.key()))
        );
        bin_header.vdata_num = asset_data.len() as u32;
        bin_header.vdata_num_ = asset_data.len() as u32;

        let mut texture_infos_map = HashMap::new();
        asset_data.extend(
            self.textures.values().filter_map(|x| x.as_ref()).flat_map(|tex| {
                let (data0, data1) = tex.dump::<O>();
                texture_infos_map.insert(tex.info().key.clone(), tex.info().clone());
                [
                    ((tex.info().asset_key.clone(), tex.info().asset_type), data0),
                    ((Crc::Key(hash_string("*".as_bytes(), Some(tex.info().asset_key.key()))), tex.info().asset_type), data1)
                ]
            }).sorted_by(|a,b| a.0.0.key().cmp(&b.0.0.key()))
        );
        bin_header.texdata_num = asset_data.len() as u32 - bin_header.vdata_num;
        let mut texture_infos = Vec::with_capacity(texture_infos_map.len());
        // println!("{:?}", texture_infos_map);
        // println!("{}", Crc::Key(3804089404) == Crc::from_string("atlas_1"));
        // println!("{}", Crc::from_string("atlas_1") == Crc::Key(3804089404));
        // println!("{}", texture_infos_map.contains_key(&Crc::Key(3804089404)));
        texture_infos.push(texture_infos_map.remove(&Crc::Key(3804089404)).unwrap());
        texture_infos.push(texture_infos_map.remove(&Crc::Key(4026460901)).unwrap());
        texture_infos.extend(texture_infos_map.into_iter().sorted_by(|a,b| a.0.key().cmp(&b.0.key())).map(|x| x.1));

        bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
        let asset_handles = asset_data.into_iter().map(|((key, kind), data)| {
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

        bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
        bin_header.asset_handle_offset = bin_data.len() as u32;
        bin_header.asset_handle_num = asset_handles.len() as u32;
        bin_data.extend(asset_handles.dump_bytes::<O>());

        let data = self.bin_strings.dump::<O>();
        bin_header.strings_offset = bin_data.len() as u32;
        bin_header.strings_size = data.len() as u32;
        bin_header.strings_size = self.bin_strings.strings.len() as u32;
        bin_data.extend(data);
        
        bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
        bin_header.to_bytes::<O>(&mut bin_data);

        // bin done

        // pak stuff
        let mut pak_header = self.pak_header.clone();
        pak_header.version = if TypeId::of::<O>() == TypeId::of::<LE>() { 2 } else { 1 };
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

        ) = meshes.values().map(|mesh| mesh.infos_count()).fold(
            (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0), 
            |mut a,b| {
                a.0 += b.0; a.1 += b.1; a.2 += b.2; a.3 += b.3; a.4 += b.4; a.5 += b.5; a.6 += b.6; a.7 += b.7; a.8 += b.8; a.9 += b.9; a.10 += b.10; a.11 += b.11;
                a
            }
        );
        pak_header.mesh_info_num = meshes.len() as u32;
        pak_header.texture_info_num = self.textures.len() as u32;
        pak_header.effect_info_num = self.effects.len() as u32;
        pak_header.gfx_block_info_num = self.gfx_blocks.len() as u32;
        pak_header.illumination_info_num = self.light_blocks.len() as u32;
        pak_header.foliage_info_num = self.foliages.iter().map(|(_, x)| x.len()).sum::<usize>() as u32;
        pak_header.animation_info_num = self.animations.len() as u32;

        let mut block1: Vec<u8> = vec![];
        pak_header.obja_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.obja_num as usize * pak::ObjA::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.obj0_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.obj0_num as usize * pak::Obj0::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.mesh_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.mesh_info_num as usize * pak::MeshInfo::size::<O>()]);
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
        pak_header.illumination_info_offset = block1.len() as u32;
        block1.extend(vec![0u8; pak_header.illumination_info_num as usize * pak::IlluminationInfo::size::<O>()]);
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);

        let mut infos = DumpInfos {
            header: pak_header.clone(),
            ..Default::default()
        };

        // infos done
        let mut offset = 0;
        let animation_vals: Vec<_> = self.animations.iter().sorted_by(|a, b| a.0.key().cmp(&b.0.key())).map(|(k, anim)| {
            let vals = anim.dump::<O>(offset, &mut infos);
            offset += vals.len();
            (vals, anim.info.level_flag)
        }).collect();
        let animations_blocks = (0..self.animation_block_infos.len() as u32).map(|i| {
            let level_flag = 1u32 << i;
            animation_vals.iter().filter(|(_, k)| k & level_flag != 0).flat_map(|(x, _)| x).cloned().collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        
        let mut animation_block_infos = self.animation_block_infos.clone();
        for (info, data) in zip(&mut animation_block_infos, animations_blocks) {
            pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
            let size = data.len();
            let data = CompressedBlock { data }.dump();
            info.offset = pak_data.len() as u32;
            info.size = size as u32;
            info.size_comp = data.len() as u32;
            pak_data.extend(data);
        }

        let effects = self.effects.iter().sorted_by(|a, b| a.0.key().cmp(&b.0.key())).map(|(key, effect)| {
            let vals = effect.dump::<O>();
            let effect = pak::EffectInfo { key: key.clone(), level_flags: effect.level_flags, offset: block1.len() as u32, size: vals.len() as u32 };
            block1.extend(vals);
            effect
        }).collect::<Vec<_>>();


        let key_occluder = hash_string(b"occluder", None);
        let mut normal = vec![];
        let mut collision_road = vec![];
        let mut terrain = vec![];
        for k in meshes.keys() {
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
        collision_road.sort_unstable_by_key(|x| 
            x.str().and_then(|x| x.split('_').last().and_then(|x| x.parse::<usize>().ok())).unwrap_or_default()
        );
        for key in normal.into_iter().chain(collision_road) {
            block1.extend(meshes.get(key).unwrap().dump::<O>(block1.len(), &mut infos));
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        }
        let terrain_start_offset = block1.len() as u32;
        block1.extend(vec![0xFFu8; 16]);
        for key in terrain {
            block1.extend(meshes.get(key).unwrap().dump_terrain::<O>(block1.len(), terrain_start_offset, &mut infos));
        }
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);

        let foliages = self.foliages.iter().flat_map(|(_, x)| x).map(|(info, val)| {
            let mut info = info.clone();
            info.offset = block1.len() as u32;
            block1.extend(val.dump_bytes::<O>());
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
            info
        }).collect::<Vec<_>>();

        block1.extend(meshes.get(&Crc::Key(key_occluder)).unwrap().dump::<O>(block1.len(), &mut infos));
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);

        // maybe assert that all the lengths are as they should be ?

        let gfx_blocks = self.gfx_blocks.iter().map(|(key, val)| {
            let gfx_block = pak::GFXBlockInfo { key: key.clone(), offset: block1.len() as u32, size: val.len() as u32 };
            block1.extend(val.clone());
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
            gfx_block
        }).collect::<Vec<_>>();

        let light_blocks = self.light_blocks.iter().map(|(&guid, val)| {
            let light_block = pak::IlluminationInfo { guid, num: val.len() as u32, offset: block1.len() as u32 };
            block1.extend(val.dump_bytes::<O>());
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
            light_block
        }).collect::<Vec<_>>();

        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        pak_header.sub_blocks1_offset = block1.len() as u32;
        block1.extend(self.sub_blocks1.dump::<O>(&lua));
        pak_header.string_keys_offset = block1.len() as u32;
        block1.extend(self.string_keys.dump::<O>());

        self.objas.to_bytes::<O>(&mut block1[pak_header.obja_offset as usize..]);
        self.obj0s.to_bytes::<O>(&mut block1[pak_header.obj0_offset as usize..]);
        infos.mesh.to_bytes::<O>(&mut block1[pak_header.mesh_info_offset as usize..]);
        infos.buffer.to_bytes::<O>(&mut block1[pak_header.buffer_info_offset as usize..]);
        infos.mat1.to_bytes::<O>(&mut block1[pak_header.mat1_offset as usize..]);
        infos.mat2.to_bytes::<O>(&mut block1[pak_header.mat2_offset as usize..]);
        infos.mat3.to_bytes::<O>(&mut block1[pak_header.mat3_offset as usize..]);
        infos.mat4.to_bytes::<O>(&mut block1[pak_header.mat4_offset as usize..]);
        infos.mat_extra.to_bytes::<O>(&mut block1[pak_header.mat_extra_offset as usize..]);
        infos.shape.to_bytes::<O>(&mut block1[pak_header.shape_info_offset as usize..]);
        for (i, hk_shape) in infos.hk_shape.iter().enumerate() {
            hk_shape.to_bytes::<O>(&mut block1[pak_header.hk_shape_info_offset as usize + i * HkShape0::size::<O>()..]);
        }
        infos.hk_constraint_data.to_bytes::<O>(&mut block1[pak_header.hk_constraint_data_offset as usize..]);
        infos.vbuff.to_bytes::<O>(&mut block1[pak_header.vbuff_info_offset as usize..]);
        infos.ibuff.to_bytes::<O>(&mut block1[pak_header.ibuff_info_offset as usize..]);
        texture_infos.to_bytes::<O>(&mut block1[pak_header.texture_info_offset as usize..]);
        infos.animation.to_bytes::<O>(&mut block1[pak_header.animation_info_offset as usize..]);
        infos.hk_constraint.to_bytes::<O>(&mut block1[pak_header.hk_constraint_info_offset as usize..]);
        effects.to_bytes::<O>(&mut block1[pak_header.effect_info_offset as usize..]);
        foliages.to_bytes::<O>(&mut block1[pak_header.foliage_info_offset as usize..]);
        self.pfield_infos.to_bytes::<O>(&mut block1[pak_header.pfield_info_offset as usize..]);
        gfx_blocks.to_bytes::<O>(&mut block1[pak_header.gfx_block_info_offset as usize..]);
        light_blocks.to_bytes::<O>(&mut block1[pak_header.illumination_info_offset as usize..]);
        animation_block_infos.to_bytes::<O>(&mut block1[pak_header.animation_block_info_offset as usize..]);

        // block2
        for (i, mesh) in infos.mesh.iter().enumerate() {
            infos.block2_offsets.extend([
                pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 8,
                pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 12,
                pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 48,
                pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 140,
                pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 144,
                pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 152,
                pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 164,
                pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 172,
                pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 180,
                pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 252,
            ]);
            if mesh.keys_offset != 0 {
                infos.block2_offsets.push(pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 136);
            }
            if mesh.vals_i_offset != 0 {
                infos.block2_offsets.push(pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 160);
            }
            if mesh.vals_j_offset != 0 {
                infos.block2_offsets.push(pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 196);
            }
            if mesh.block_offset != 0 {
                infos.block2_offsets.push(pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 200);
            }
            if mesh.vals_k_offset != 0 {
                infos.block2_offsets.push(pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 204);
            }
            if mesh.shape_offset != 0 {
                infos.block2_offsets.push(pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 224);
            }
            if mesh.hk_constraint_data_offset != 0 {
                infos.block2_offsets.push(pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 232);
            }
            if mesh.hk_constraint_offset != 0 {
                infos.block2_offsets.push(pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 240);
            }
            if mesh.keys2_offset != 0 {
                infos.block2_offsets.push(pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 244);
            }
            if mesh.keys2_order_offset != 0 {
                infos.block2_offsets.push(pak_header.mesh_info_offset + (i * pak::MeshInfo::size::<O>()) as u32 + 248);
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
            if mat.mat_extra_offset != 0 {
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
                HkShapeInfo::HkShape5(_) => vec![
                    pak_header.hk_shape_info_offset + (i * pak::HkShapeInfo::size::<O>()) as u32 + 44,
                    pak_header.hk_shape_info_offset + (i * pak::HkShapeInfo::size::<O>()) as u32 + 52
                ],
                HkShapeInfo::HkShape6(_) => vec![
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
                infos.block2_offsets.push(pak_header.hk_constraint_info_offset + (i * pak::HkConstraintInfo::size::<O>()) as u32 + 60);
            }
        }
        for i in 0..pak_header.effect_info_num as usize {
            infos.block2_offsets.push(pak_header.effect_info_offset + (i * pak::EffectInfo::size::<O>()) as u32 + 8);
        }
        for i in 0..pak_header.gfx_block_info_num as usize {
            infos.block2_offsets.push(pak_header.gfx_block_info_offset + (i * pak::GFXBlockInfo::size::<O>()) as u32 + 4);
        }
        for i in 0..pak_header.illumination_info_num as usize {
            infos.block2_offsets.push(pak_header.illumination_info_offset + (i * pak::IlluminationInfo::size::<O>()) as u32 + 8);
        }
        for i in 0..pak_header.foliage_info_num as usize {
            infos.block2_offsets.push(pak_header.foliage_info_offset + (i * pak::FoliageInfo::size::<O>()) as u32 + 28);
        }
        pak_header.sub_blocks2_offset = 0;
        let mut block2 = self.sub_blocks2.dump::<O>(&lua);
        pak_header.block2_offsets_offset = block2.len() as u32;
        pak_header.block2_offsets_num = infos.block2_offsets.len() as u32;
        block2.extend(infos.block2_offsets.dump_bytes::<O>());

        // rest of pak
        pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
        let size = block1.len();
        let data = CompressedBlock { data: block1 }.dump();
        pak_header.block1_size = size as u32;
        pak_header.block1_size_comp = data.len() as u32;
        pak_header.block1_offset = pak_data.len() as u32;
        pak_data.extend(data);

        pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
        let size = block2.len();
        let data = CompressedBlock { data: block2 }.dump();
        pak_header.block2_size = size as u32;
        pak_header.block2_size_comp = data.len() as u32;
        pak_header.block2_offset = pak_data.len() as u32;
        pak_data.extend(data);

        pak_data.extend(vec![0u8; ((pak_data.len() + 4095) & 0xfffff000)-pak_data.len()]);
        let data = self.pak_strings.dump::<O>();
        pak_header.strings_offset = pak_data.len() as u32;
        pak_header.strings_num = self.pak_strings.strings.len() as u32;
        pak_header.strings_size = data.len() as u32;
        pak_data.extend(data);

        pak_header.block_a_offset = pak_data.len() as u32;
        pak_header.block_a_num = self.pak_block_a.len() as u32;
        pak_data.extend(self.pak_block_a.dump_bytes::<O>());

        pak_header.to_bytes::<O>(&mut pak_data);

        (pak_data, bin_data, infos)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        let time: Instant = Instant::now();
        println!("storing level");

        let path = path.as_ref();
        std::fs::create_dir(path).ok();
        // std::fs::create_dir(path.join("assets").join("raw")).ok();
        std::fs::write(path.join("bin_header.json"), to_string_pretty(&self.bin_header).unwrap()).unwrap();
        self.bin_strings.to_file(path.join("bin_strings"));

        std::fs::write(path.join("pak_header.json"), to_string_pretty(&self.pak_header).unwrap()).unwrap();
        self.pak_strings.to_file(path.join("pak_strings"));
        println!("headers in {:?}", time.elapsed());

        std::fs::write(path.join("objas.json"), to_string_pretty(&self.objas).unwrap()).unwrap();
        std::fs::write(path.join("obj0s.json"), to_string_pretty(&self.obj0s).unwrap()).unwrap();
        std::fs::write(path.join("pak_vals_a.json"), to_string_pretty(&self.pak_block_a).unwrap()).unwrap();
        println!("unused objs in {:?}", time.elapsed());

        std::fs::create_dir(path.join("meshes")).ok();
        for (key, data) in &self.meshes {
            std::fs::write(path.join("meshes").join(key.to_string()).with_extension("json"), to_string_pretty(&data).unwrap()).unwrap();
        }
        println!("meshes in {:?}", time.elapsed());
        std::fs::create_dir(path.join("effects")).ok();
        for (key, data) in &self.effects {
            data.to_file(path.join("effects").join(key.to_string()));
        }
        println!("effects in {:?}", time.elapsed());
        std::fs::create_dir(path.join("foliage")).ok();
        for (key, data) in &self.foliages {
            let (info, data): (Vec<_>, Vec<_>) = Iterator::unzip(data.iter().map(|(a,b)| (a,b)));
            std::fs::write(path.join("foliage").join(key.to_string()).with_extension("json"), to_string_pretty(&info).unwrap()).unwrap();
            for (i, data) in data.iter().enumerate() {
                std::fs::write(path.join("foliage").join(format!("{}-{}", key.to_string(), i)).with_extension("bin"), data.dump_bytes::<LE>()).unwrap();
            }
        }
        println!("foliage objs in {:?}", time.elapsed());
        std::fs::create_dir(path.join("illumination")).ok();
        for (key, data) in &self.light_blocks {
            std::fs::write(path.join("illumination").join(format!("{}", key)).with_extension("bin"), data.dump_bytes::<LE>()).unwrap();
        }
        println!("illumination objs in {:?}", time.elapsed());
        std::fs::create_dir(path.join("gfxs")).ok();
        for (key, data) in &self.gfx_blocks {
            std::fs::write(path.join("gfxs").join(key.to_string()).with_extension("gfx"), data).unwrap();
        }
        println!("gfxs in {:?}", time.elapsed());

        std::fs::write(path.join("animation_block_infos.json"), to_string_pretty(&self.animation_block_infos).unwrap()).unwrap();
        std::fs::create_dir(path.join("animations")).ok();
        for (key, data) in &self.animations {
            std::fs::write(path.join("animations").join(key.to_string()).with_extension("json"), to_string_pretty(&data).unwrap()).unwrap();
        }
        println!("animations in {:?}", time.elapsed());

        std::fs::create_dir(path.join("textures")).ok();
        for (key, data) in &self.textures {
            if let Some(tex) = data {
                tex.to_file(path.join("textures").join(key.to_string()));
            }
        }
        println!("textures in {:?}", time.elapsed());

        std::fs::write(path.join("pfield_infos.json"), to_string_pretty(&self.pfield_infos).unwrap()).unwrap();

        println!("packed items in {:?}", time.elapsed());

        self.string_keys.to_file(path.join("string_keys"));
        self.sub_blocks1.to_file(path.join("sub_blocks1"), &self.string_keys);
        self.sub_blocks2.to_file(path.join("sub_blocks2"), &self.string_keys);
        println!("sub blocks in {:?}", time.elapsed());

    }
}