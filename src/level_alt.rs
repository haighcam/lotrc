use std::{any::TypeId, collections::HashMap, ffi::OsStr, fs, path::Path, sync::Arc};
use itertools::Itertools;
use zerocopy::{ByteOrder, LE, BE};
use log::{warn, info};
use serde::{Serialize, Deserialize};
use serde_json::to_vec_pretty;
use std::time::Instant;
use std::iter::zip;

use super::{
    pak, bin, lua_stuff, pak_alt::*,
    types::{self, hash_string, GameObjs, OrderedData, OrderedDataVec, CompressedBlock, Crc},
    read_write::{Reader, Writer, PathStuff},
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
    pub textures: HashMap<Crc, bin::Tex>,
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

    pub radiosity: HashMap<Crc, bin::Radiosity>,

    pub vertex_formats: HashMap<(u32, u32), (Vec<(u32, pak::VertexUsage)>, usize)>,

    pub pak_vals_a: Vec<pak::BlockAVal>,
}

impl Level {
    pub fn parse<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        info!("Parsing level data {:?}", path);   
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

    pub fn dump<O: ByteOrder + 'static, P: AsRef<Path>>(&self, path: P) {
        let path = path.as_ref();
        info!("Dumping level data {:?}", path);
        let (pak, bin, _infos) = self.to_data::<O>();
        fs::write(path.with_extension("PAK"), pak).unwrap();
        fs::write(path.with_extension("BIN"), bin).unwrap();
        // fs::write(path.with_extension("json"), serde_json::to_vec_pretty(&_infos).unwrap()).unwrap();
    }

    pub fn from_data<O: ByteOrder + 'static>(bin_data: &[u8], pak_data: &[u8]) -> Self {
        let time = Instant::now();
        info!("extracting level");

        let lua = lua_stuff::LuaCompiler::new().unwrap();
        let bin_header: bin::Header = OrderedData::from_bytes::<O>(bin_data);
        let bin_strings = types::Strings::from_data::<O>(bin_data, bin_header.strings_offset as usize, bin_header.strings_num as usize);
        types::update_strings(&bin_strings.strings);

        let asset_handles: Vec<bin::AssetHandle> = OrderedDataVec::from_bytes::<O>(&bin_data[bin_header.asset_handle_offset as usize..], bin_header.asset_handle_num as usize);
        let asset_data = HashMap::<(Crc, u32), Vec<u8>>::from_iter(
            asset_handles.iter().map(|info| ((info.key.clone(), info.kind), types::CompressedBlock::from_data(bin_data, info.size as usize, info.size_comp as usize, info.offset as usize).data))
        );

        let radiosity: HashMap<Crc, bin::Radiosity> = asset_data.iter().filter(|(key, _)| key.0.str().map(|x| x.ends_with("_radiosity")).unwrap_or(false)).map(|(key, data)| (
            key.0.clone(), bin::Radiosity::from_data::<O>(&data[..], key.1)
        )).collect();

        info!("bin parsed in {:?}", time.elapsed());
        
        let pak_header: pak::Header = OrderedData::from_bytes::<O>(pak_data);
        let pak_strings = types::Strings::from_data::<O>(pak_data, pak_header.strings_offset as usize, pak_header.strings_num as usize);
        types::update_strings(&pak_strings.strings);
        info!("headers in {:?}", time.elapsed());

        let block2 = types::CompressedBlock::from_data(pak_data, pak_header.block2_size as usize, pak_header.block2_size_comp as usize, pak_header.block2_offset as usize).data;
        let sub_blocks2 = types::SubBlocks::from_data::<O>(&block2[..], pak_header.sub_blocks2_offset as usize, &lua);
        let block2_offsets = OrderedDataVec::from_bytes::<O>(&block2[pak_header.block2_offsets_offset as usize..], pak_header.block2_offsets_num as usize);
        info!("block2 parsed in {:?}", time.elapsed());

        let block1 = types::CompressedBlock::from_data(pak_data, pak_header.block1_size as usize, pak_header.block1_size_comp as usize, pak_header.block1_offset as usize).data;
        info!("main blocks extracted in {:?}", time.elapsed());

        let objas = OrderedDataVec::from_bytes::<O>(&block1[pak_header.obja_offset as usize..], pak_header.obja_num as usize);
        let obj0s = OrderedDataVec::from_bytes::<O>(&block1[pak_header.obj0_offset as usize..], pak_header.obj0_num as usize);
        let animation_block_infos: Vec<pak::AnimationBlockInfo> = OrderedDataVec::from_bytes::<O>(&block1[pak_header.animation_block_info_offset as usize..], pak_header.animation_block_info_num as usize);
        let pfield_infos = OrderedDataVec::from_bytes::<O>(&block1[pak_header.pfield_info_offset as usize..], pak_header.pfield_info_num as usize);

        let mut vertex_formats = HashMap::new();
        let meshes = (0..pak_header.mesh_info_num as usize).map(|i| {
            let mut mesh = Mesh::from_data::<O>(&block1[..], pak_header.mesh_info_offset as usize + i * pak::MeshInfo::size::<O>());
            if mesh.info.vbuff_num != 0 || mesh.info.ibuff_num != 0 {
                let buffer = asset_data.get(&(mesh.info.asset_key.clone(), mesh.info.asset_type)).unwrap();
                mesh.vertex_data.extend(mesh.vbuffs.iter_mut().map(|info| pak::VertexBuffer::from_data::<O>(&buffer[..], info, &mut vertex_formats)));
                mesh.index_data.extend(mesh.ibuffs.iter().map(|info| pak::IndexBuffer::from_data::<O>(&buffer[..], info)));    
            }
            (mesh.info.key.clone(), mesh)
        }).collect::<HashMap<_, _>>();

        let effects = <Vec<pak::EffectInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.effect_info_offset as usize..], pak_header.effect_info_num as usize).into_iter().map(|info| (
            info.key, GameObjs::from_data::<O>(&block1[..], info.offset as usize, info.size as usize, info.gamemodemask)
        )).collect::<HashMap<_, _>>();

        let gfx_blocks = <Vec<pak::GFXBlockInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.gfx_block_info_offset as usize..], pak_header.gfx_block_info_num as usize).into_iter().map(|info| (
            info.key, block1[info.offset as usize..(info.offset + info.size) as usize].to_vec()
        )).collect::<HashMap<_, _>>();

        let light_blocks = <Vec<pak::IlluminationInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.illumination_info_offset as usize..], pak_header.illumination_info_num as usize).into_iter().map(|info| (
            info.guid, <Vec<u32> as OrderedDataVec>::from_bytes::<O>(&block1[info.offset as usize..], info.num as usize)
        )).collect::<HashMap<_, _>>();

        let mut foliages: HashMap<Crc, Vec<(pak::FoliageInfo, Vec<u32>)>> = HashMap::new();
        for info in <Vec<pak::FoliageInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.foliage_info_offset as usize..], pak_header.foliage_info_num as usize) {
            foliages.entry(info.key.clone()).or_default().push((
                info.clone(), OrderedDataVec::from_bytes::<O>(&block1[info.offset as usize..], ((info.s1b - info.s1a) * (info.s2b - info.s2a)) as usize * 2)
            ))
        }

        let textures = <Vec<pak::TextureInfo> as OrderedDataVec>::from_bytes::<O>(&block1[pak_header.texture_info_offset as usize..], pak_header.texture_info_num as usize).into_iter().map(|mut info| {
            let data0 = &asset_data.get(&(info.asset_key.clone(), info.asset_type)).unwrap();
            let data1 = &asset_data.get(&(Crc::Key(hash_string("*".as_bytes(), Some(info.asset_key.key()))), info.asset_type)).unwrap();
            let key = info.key.clone();
            (key, bin::Tex::from_data::<O>(&data0, &data1, &mut info))
        }).collect::<HashMap<_, _>>();

        let blocks = animation_block_infos.iter().map(|info| 
            types::CompressedBlock::from_data(&pak_data[..], info.size as usize, info.size_comp as usize, info.offset as usize).data
        ).collect::<Vec<_>>();
        let mut offsets = blocks.iter().map(|_| 0usize).collect::<Vec<_>>();
        let animations = (0..pak_header.animation_info_num as usize).map(|i| {
            let anim = Animation::from_data::<O>(&block1[pak_header.animation_info_offset as usize + i * pak::AnimationInfo::size::<O>()..], &mut offsets, &blocks);
            (anim.info.key.clone(), anim)
        }).collect::<HashMap<_, _>>();

        info!("items extracted in {:?}", time.elapsed());

        let sub_blocks1 = types::SubBlocks::from_data::<O>(&block1[..], pak_header.sub_blocks1_offset as usize, &lua);
        let string_keys = types::StringKeys::from_data::<O>(&block1[..], pak_header.string_keys_offset as usize);
        info!("sub blocks extracted in {:?}", time.elapsed());

        let pak_vals_a = OrderedDataVec::from_bytes::<O>(&pak_data[pak_header.block_a_offset as usize..], pak_header.block_a_num as usize);
        info!("buffers extracted in {:?}", time.elapsed());

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
            pak_vals_a,
            gfx_blocks,
        }
    }
    
    pub fn to_data<O: ByteOrder + 'static>(&self) -> (Vec<u8>, Vec<u8>, DumpInfos) {
        fn dump_vertex_data<O: ByteOrder + 'static>(mesh: &mut Mesh) -> Option<((Crc, u32), Vec<u8>)> {
            if mesh.vertex_data.len() != 0 || mesh.index_data.len() != 0 {
                let size = mesh.vbuffs.iter().map(|x| x.size + x.offset).chain(mesh.ibuffs.iter().map(|x| x.size + x.offset)).max().unwrap();
                let mut data = vec![0u8; size as usize];
                for (vbuff, buff) in zip(&mesh.vbuffs, &mesh.vertex_data) {
                    buff.into_data::<O>(&mut data, vbuff);
                }
                for (ibuff, buff) in zip(&mesh.ibuffs, &mesh.index_data) {
                    buff.into_data::<O>(&mut data[ibuff.offset as usize..]);
                }
                let mut data = Vec::with_capacity(size as usize);
                for i in 0..(mesh.vbuff_order.len().max(mesh.ibuff_order.len())) {
                    if i < mesh.vbuff_order.len() {
                        let info = &mut mesh.vbuffs[i];
                        let vals = mesh.vertex_data[i].dump::<O>();
                        info.offset = data.len() as u32;
                        info.size = vals.len() as u32;
                        data.extend(vals);
                        for buffer_info in &mut mesh.buffer_infos {
                            if buffer_info.vbuff_info_offset == mesh.vbuff_order[i] {
                                buffer_info.v_size = mesh.vertex_data[i].vals.iter().map(|(_, x)| x.size()).sum::<usize>() as u32;
                                buffer_info.vbuff_size = info.size;
                            }
                            if buffer_info.vbuff_info_offset_2 == mesh.vbuff_order[i] {
                                buffer_info.v_size_2 = mesh.vertex_data[i].vals.iter().map(|(_, x)| x.size()).sum::<usize>() as u32;
                                buffer_info.vbuff_size_2 = info.size;
                            }
                            if buffer_info.vbuff_info_offset_3 == mesh.vbuff_order[i] {
                                buffer_info.v_size_3 = mesh.vertex_data[i].vals.iter().map(|(_, x)| x.size()).sum::<usize>() as u32;
                                buffer_info.vbuff_size_3 = info.size;
                            }
                        }
                    }
                    if i < mesh.ibuff_order.len() {
                        let info = &mut mesh.ibuffs[i];
                        let vals = mesh.index_data[i].dump::<O>();
                        info.offset = data.len() as u32;
                        info.size = vals.len() as u32;
                        data.extend(vals);
                    }
                }        
                Some(((mesh.info.asset_key.clone(), mesh.info.asset_type), data))
            } else {
                None
            }
        }
        let time = Instant::now();
        info!("compressing level");
        let lua: lua_stuff::LuaCompiler = lua_stuff::LuaCompiler::new().unwrap();
        
        let mut texture_data = vec![];
        fn sort_texture(tex: &&bin::Tex) -> u32 {
            let k = tex.info().asset_key.key();
            if k == 3804089404 {
                0
            } else if k == 4026460901 {
                1
            } else {
                k
            }
        }
        let texture_infos = self.textures.values().sorted_by_key(sort_texture).map(|tex| {
            let (data0, data1) = tex.dump::<O>();
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
            tex.info().clone()
        }).collect::<Vec<_>>();

        info!("textures in {:?}", time.elapsed());

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

        ) = self.meshes.values().map(|mesh| mesh.infos_count()).fold(
            (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0), 
            |mut a,b| {
                a.0 += b.0; a.1 += b.1; a.2 += b.2; a.3 += b.3; a.4 += b.4; a.5 += b.5; a.6 += b.6; a.7 += b.7; a.8 += b.8; a.9 += b.9; a.10 += b.10; a.11 += b.11;
                a
            }
        );
        pak_header.mesh_info_num = self.meshes.len() as u32;
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
        let animation_vals: Vec<_> = self.animations.iter().sorted_by(|a, b| a.0.key().cmp(&b.0.key())).map(|(_, anim)| {
            let vals = anim.dump::<O>(offset, &mut infos);
            offset += vals.len();
            (vals, anim.info.gamemodemask)
        }).collect();
        let animations_blocks = (0..self.animation_block_infos.len() as u32).map(|i| {
            let gamemodemask = 1i32 << i;
            animation_vals.iter().filter(|(_, k)| k & gamemodemask != 0).flat_map(|(x, _)| x).cloned().collect::<Vec<_>>()
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
        info!("animations in {:?}", time.elapsed());

        let effects = self.effects.iter().sorted_by(|a, b| a.0.key().cmp(&b.0.key())).map(|(key, effect)| {
            let vals = effect.dump::<O>();
            let effect = pak::EffectInfo { key: key.clone(), gamemodemask: effect.gamemodemask, offset: block1.len() as u32, size: vals.len() as u32 };
            block1.extend(vals);
            effect
        }).collect::<Vec<_>>();
        info!("effects in {:?}", time.elapsed());

        let key_occluder = hash_string(b"occluder", None);
        let mut normal = vec![];
        let mut collision_road = vec![];
        let mut terrain = vec![];
        let mut mesh_data = vec![];
        for k in self.meshes.keys() {
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
            let mut mesh = self.meshes.get(key).unwrap().clone();
            if let Some(val) = dump_vertex_data::<O>(&mut mesh) {
                mesh_data.push(val);
            }
            block1.extend(mesh.dump::<O>(block1.len(), &mut infos));
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        }
        let terrain_start_offset = block1.len() as u32;
        block1.extend(vec![0xFFu8; 16]);
        for key in terrain {
            let mut mesh = self.meshes.get(key).unwrap().clone();
            if let Some(val) = dump_vertex_data::<O>(&mut mesh) {
                mesh_data.push(val);
            }
            block1.extend(mesh.dump_terrain::<O>(block1.len(), terrain_start_offset, &mut infos));
        }
        block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);

        let foliages = self.foliages.iter().flat_map(|(_, x)| x).map(|(info, val)| {
            let mut info = info.clone();
            info.offset = block1.len() as u32;
            block1.extend(val.dump_bytes::<O>());
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
            info
        }).collect::<Vec<_>>();

        if let Some(mesh) = self.meshes.get(&Crc::Key(key_occluder)) {
            let mut mesh = mesh.clone();
            if let Some(val) = dump_vertex_data::<O>(&mut mesh) {
                mesh_data.push(val);
            }
            block1.extend(mesh.dump::<O>(block1.len(), &mut infos));
            block1.extend(vec![0u8; ((block1.len() + 15) & 0xFFFFFFF0) - block1.len()]);
        }
        info!("mesh & foliage in {:?}", time.elapsed());

        // maybe assert that all the lengths are as they should be ?

        let gfx_blocks = self.gfx_blocks.iter().sorted_by(|a,b| a.0.cmp(&b.0)).map(|(key, val)| {
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
        info!("block1 objs in {:?}", time.elapsed());


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
        info!("block1 in {:?}", time.elapsed());

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
                infos.block2_offsets.push(pak_header.hk_constraint_info_offset + (i * pak::HkConstraintInfo::size::<O>()) as u32 + 64);
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
        info!("block2 in {:?}", time.elapsed());

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
        pak_header.block_a_num = self.pak_vals_a.len() as u32;
        pak_data.extend(self.pak_vals_a.dump_bytes::<O>());

        pak_header.to_bytes::<O>(&mut pak_data);
        info!("pak in {:?}", time.elapsed());

        // bin_data
        let mut bin_header = self.bin_header.clone();
        let mut bin_data = vec![0u8; bin::Header::size::<O>()];
        bin_header.version = if TypeId::of::<O>() == TypeId::of::<LE>() { 2 } else { 1 };

        bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
        let mut mesh_asset_handles = mesh_data.into_iter().map(|((key, kind), data)| {
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

        mesh_asset_handles.extend(self.radiosity.iter().map(|(key, data)| {
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

        mesh_asset_handles.sort_by_key(|x| x.key.key());
        texture_asset_handles.sort_by_key(|x| x.key.key());

        bin_header.vdata_num = mesh_asset_handles.len() as u32;
        bin_header.vdata_num_ = mesh_asset_handles.len() as u32;
        bin_header.texdata_num = texture_asset_handles.len() as u32;

        let mut asset_handles = mesh_asset_handles;
        asset_handles.extend(texture_asset_handles);
        
        bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
        bin_header.asset_handle_offset = bin_data.len() as u32;
        bin_header.asset_handle_num = asset_handles.len() as u32;
        bin_data.extend(asset_handles.dump_bytes::<O>());

        let data = self.bin_strings.dump::<O>();
        bin_header.strings_offset = bin_data.len() as u32;
        bin_header.strings_size = data.len() as u32;
        bin_header.strings_num = self.bin_strings.strings.len() as u32;
        bin_data.extend(data);
        
        bin_data.extend(vec![0u8; ((bin_data.len() + 2047) & 0xfffff800)-bin_data.len()]);
        bin_header.to_bytes::<O>(&mut bin_data);
        info!("bin in {:?}", time.elapsed());

        // bin done
            
        (pak_data, bin_data, infos)
    }

    pub fn to_file(&self, writer: Writer) {
        let time: Instant = Instant::now();
        info!("storing level");

        // std::fs::create_dir_all(path.join("assets").join("raw")).ok();
    
        writer.join("bin_header.json").write(&to_vec_pretty(&self.bin_header).unwrap());
        self.bin_strings.to_file(writer.join("bin_strings"));

        writer.join("pak_header.json").write(&to_vec_pretty(&self.pak_header).unwrap());
        self.pak_strings.to_file(writer.join("pak_strings"));
        info!("headers in {:?}", time.elapsed());

        writer.join("objas.json").write(&to_vec_pretty(&self.objas).unwrap());
        writer.join("obj0s.json").write(&to_vec_pretty(&self.obj0s).unwrap());
        writer.join("pak_vals_a.json").write(&to_vec_pretty(&self.pak_vals_a).unwrap());
        info!("unused objs in {:?}", time.elapsed());

        for (key, data) in &self.meshes {
            writer.join("meshes").join(key.to_string()).with_extension("json").write(&to_vec_pretty(&data).unwrap());
        }
        info!("meshes in {:?}", time.elapsed());
        for (key, data) in &self.effects {
            data.to_file(writer.join("effects").join(key.to_string()));
        }
        info!("effects in {:?}", time.elapsed());
        for (key, data) in &self.foliages {
            let (info, data): (Vec<_>, Vec<_>) = Iterator::unzip(data.iter().map(|(a,b)| (a,b)));
            writer.join("foliage").join(key.to_string()).with_extension("json").write(&to_vec_pretty(&info).unwrap());
            for (i, data) in data.iter().enumerate() {
                writer.join("foliage").join(format!("{}-{}", key.to_string(), i)).with_extension("bin").write(&data.dump_bytes::<LE>());
            }
        }
        info!("foliage objs in {:?}", time.elapsed());
        for (key, data) in &self.light_blocks {
            writer.join("illumination").join(format!("{}", key)).with_extension("bin").write(&data.dump_bytes::<LE>());
        }
        info!("illumination objs in {:?}", time.elapsed());
        for (key, data) in &self.gfx_blocks {
            writer.join("gfxs").join(key.to_string()).with_extension("gfx").write(data);
        }
        info!("gfxs in {:?}", time.elapsed());

        writer.join("animation_block_infos.json").write(&to_vec_pretty(&self.animation_block_infos).unwrap());
        for (key, data) in &self.animations {
            writer.join("animations").join(key.to_string()).with_extension("json").write(&to_vec_pretty(&data).unwrap());
        }
        info!("animations in {:?}", time.elapsed());

        for (key, tex) in &self.textures {
            tex.to_file(writer.join("textures").join(key.to_string()));
        }
        info!("textures in {:?}", time.elapsed());

        for (key, data) in &self.radiosity {
            writer.join("radiosity").join(key.to_string()).with_extension("json").write(&to_vec_pretty(&data).unwrap());
        }
        info!("radiosity in {:?}", time.elapsed());

        writer.join("pfield_infos.json").write(&to_vec_pretty(&self.pfield_infos).unwrap());

        info!("packed items in {:?}", time.elapsed());

        self.string_keys.to_file(writer.join("string_keys"));
        self.sub_blocks1.to_file(writer.join("sub_blocks1"), &self.string_keys);
        self.sub_blocks2.to_file(writer.join("sub_blocks2"), &self.string_keys);
        info!("sub blocks in {:?}", time.elapsed());

        if *types::ANIM_TABLES.lock().unwrap() {
            let mut script_manager = HashMap::new();
            {
                let lua = lua_stuff::LuaCompiler::new().unwrap();
                for block in &self.sub_blocks1.blocks {
                    if let types::SubBlock::Lua(val) = block {
                        let mut name = val.name.clone();
                        name.truncate(name.len()-4);
                        script_manager.insert(Crc::from_string(&name), lua.convert(&val.data, "L4808").unwrap());
                    }
                }
            }
            
            let anim_scripts = script_manager.keys().filter_map(|x| x.str().and_then(|x| x.starts_with("ANM_").then_some(x.to_string()))).collect::<Vec<_>>();
            let script_manager = Arc::new(script_manager);
            for anim in anim_scripts {
                let val = lua_stuff::load_anim(script_manager.clone(), anim.clone());
                writer.join("animation_tables").join(anim).with_extension("json").write(&to_vec_pretty(&val).unwrap());
            }
            info!("animation tables in {:?}", time.elapsed());
        }
    }

    pub fn from_file(reader: Reader) -> Self {
        let time: Instant = Instant::now();
        info!("reading level");        
        
        let lua: lua_stuff::LuaCompiler = lua_stuff::LuaCompiler::new().unwrap();

        let bin_header = serde_json::from_slice::<bin::Header>(&reader.join("bin_header.json").read()).unwrap();
        let bin_strings = types::Strings::from_file(reader.join("bin_strings"));

        let pak_header = serde_json::from_slice::<pak::Header>(&reader.join("pak_header.json").read()).unwrap();
        let pak_strings = types::Strings::from_file(reader.join("pak_strings"));
        info!("headers in {:?}", time.elapsed());

        let objas = serde_json::from_slice::<Vec<pak::ObjA>>(&reader.join("objas.json").read()).unwrap();
        let obj0s = serde_json::from_slice::<Vec<pak::Obj0>>(&reader.join("obj0s.json").read()).unwrap();
        let pak_vals_a = serde_json::from_slice::<Vec<pak::BlockAVal>>(&reader.join("pak_vals_a.json").read()).unwrap();
        info!("unused objs in {:?}", time.elapsed());

        let mut meshes = HashMap::new();
        for path in reader.join("meshes") {
            let key = Crc::from_string(path.name());
            let data = serde_json::from_slice::<Mesh>(&path.read()).unwrap();
            meshes.insert(key, data);
        }
        info!("meshes in {:?}", time.elapsed());

        let mut effects = HashMap::new();
        for path in reader.join("effects") {
            let key = Crc::from_string(path.name());
            let data = GameObjs::from_file(path);
            effects.insert(key, data);
        }
        info!("effects in {:?}", time.elapsed());

        let mut foliages = HashMap::new();
        for path in reader.join("foliage").into_iter().filter(|x| x.path().extension().unwrap_or(OsStr::new("")).to_str() == Some("json")) {
            let key = Crc::from_string(path.name());
            let info = serde_json::from_slice::<Vec<pak::FoliageInfo>>(&path.read()).unwrap();
            let mut data = Vec::with_capacity(info.len());
            for i in 0..info.len() {
                let dat = path.with_file_name(&format!("{}-{}.bin", key.to_string(), i)).read();
                data.push(<Vec<u32> as OrderedDataVec>::from_bytes::<LE>(&dat, dat.len()/4));
            }
            foliages.insert(key, zip(info, data).collect::<Vec<_>>());
        }
        info!("foliage objs in {:?}", time.elapsed());

        let mut light_blocks = HashMap::new();
        for path in reader.join("illumination") {
            let key: u32 = path.name().parse().unwrap();
            let dat = path.read();
            let data = <Vec<u32> as OrderedDataVec>::from_bytes::<LE>(&dat, dat.len()/4);
            light_blocks.insert(key, data);
        }
        info!("illumination objs in {:?}", time.elapsed());

        let mut gfx_blocks = HashMap::new();
        for path in reader.join("gfxs") {
            let key = Crc::from_string(path.name());
            let data = path.read();
            gfx_blocks.insert(key, data);
        }
        info!("gfxs in {:?}", time.elapsed());

        let animation_block_infos = serde_json::from_slice::<Vec<pak::AnimationBlockInfo>>(&reader.join("animation_block_infos.json").read()).unwrap();
        let mut animations = HashMap::new();
        for path in reader.join("animations") {
            let key = Crc::from_string(path.name());
            let data = serde_json::from_slice::<Animation>(&path.read()).unwrap();
            animations.insert(key, data);
        }
        info!("animations in {:?}", time.elapsed());

        let mut textures = HashMap::new();
        for path in reader.join("textures").into_iter().filter(|x| x.path().extension().unwrap_or(OsStr::new("")).to_str() == Some("json")) {
            let key = Crc::from_string(path.name());
            let data = bin::Tex::from_file(path);
            textures.insert(key, data);
        }
        info!("textures in {:?}", time.elapsed());

        let mut radiosity = HashMap::new();
        for path in reader.join("radiosity") {
            let key = Crc::from_string(path.name());
            let data = serde_json::from_slice::<bin::Radiosity>(&path.read()).unwrap();
            radiosity.insert(key, data);
        }
        info!("radiosity in {:?}", time.elapsed());

        let pfield_infos = serde_json::from_slice::<Vec<pak::PFieldInfo>>(&reader.join("pfield_infos.json").read()).unwrap();

        info!("packed items in {:?}", time.elapsed());

        let string_keys = types::StringKeys::from_file(reader.join("string_keys"));
        let sub_blocks1 = types::SubBlocks::from_file(reader.join("sub_blocks1"), &lua);
        let sub_blocks2 = types::SubBlocks::from_file(reader.join("sub_blocks2"), &lua);
        info!("sub blocks in {:?}", time.elapsed());

        let vertex_formats = HashMap::new();
        let block2_offsets = Vec::new();

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
            pak_vals_a,
            gfx_blocks,
        }

    }
}
