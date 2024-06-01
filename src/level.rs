use std::{collections::HashMap, fs::{self, File}, path::Path, any::TypeId};
use zerocopy::{ByteOrder, LE, BE};
use log::warn;
use serde::{Serialize, Deserialize};
// use rmp_serde::Serializer;
use serde_cbor::{Serializer, Deserializer, ser::IoWrite, de::IoRead};
use std::time::Instant;
use std::iter::zip;
use lotrc_rs_proc::OrderedData;

use super::{
    pak, bin, lua_stuff,
    types::{self, hash_string, GameObjs, OrderedData, OrderedDataVec, SubBlock}
};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Level {
    pub bin_header: bin::Header,
    pub bin_strings: types::Strings,
    pub asset_handles: Vec<bin::AssetHandle>,
    pub asset_handle_lookup: HashMap<(u32, u32), usize>,
    pub asset_data: HashMap<(u32, u32), types::CompressedBlock>,
    pub pak_header: pak::Header,
    pub pak_strings: types::Strings,

    pub block1: types::CompressedBlock,
    pub block2: types::CompressedBlock,
    pub dump_block1: Option<types::CompressedBlock>,
    pub dump_block2: Option<types::CompressedBlock>,

    pub objas: Vec<pak::ObjA>,
    pub obj0s: Vec<pak::Obj0>,
    pub mesh_infos: Vec<pak::MeshInfo>,
    pub buffer_infos: Vec<pak::BufferInfo>,
    pub mat1s: Vec<pak::Mat1>,
    pub mat2s: Vec<pak::Mat2>,
    pub mat3s: Vec<pak::Mat3>,
    pub mat4s: Vec<pak::Mat4>,
    pub mat_extras: Vec<pak::MatExtra>,
    pub shape_infos: Vec<pak::ShapeInfo>,
    pub hk_shape_infos: Vec<pak::HkShapeInfo>,
    pub hk_constraint_datas: Vec<pak::HkConstraintData>,
    pub vbuff_infos: Vec<pak::VBuffInfo>,
    pub ibuff_infos: Vec<pak::IBuffInfo>,
    pub texture_infos: Vec<pak::TextureInfo>,
    pub animation_infos: Vec<pak::AnimationInfo>,
    pub hk_constraint_infos: Vec<pak::HkConstraintInfo>,
    pub gameobj_block_infos: Vec<pak::LevelBlockInfo>,
    pub foliage_infos: Vec<pak::FoliageInfo>,
    pub pfield_infos: Vec<pak::PFieldInfo>,
    pub gfx_block_infos: Vec<pak::GFXBlockInfo>,
    pub obj14_infos: Vec<pak::Obj14Info>,
    pub animation_block_infos: Vec<pak::AnimationBlockInfo>,
    pub dump_animation_block_infos: Vec<pak::AnimationBlockInfo>,

    pub meshes: Vec<pak::Mesh>,
    pub shapes: Vec<pak::Shape>,
    pub hk_shapes: Vec<pak::HkShape>,
    pub hk_constraints: Vec<pak::HkConstraint>,
    pub gameobj_blocks: Vec<types::GameObjs>,
    pub gfx_blocks: Vec<types::Data>,
    pub obj14s: Vec<pak::Obj14>,
    pub foliages: Vec<pak::Foliage>,

    pub animation_blocks: Vec<types::CompressedBlock>,
    pub dump_animation_blocks: Vec<types::CompressedBlock>,
    pub animations: Vec<pak::Animation>,

    pub string_keys: types::StringKeys,
    pub sub_blocks1: types::SubBlocks,
    pub sub_blocks2: types::SubBlocks,
    pub block2_offsets: Vec<u32>,

    pub radiosity: HashMap<(u32, u32), bin::Radiosity>,
    pub textures: HashMap<types::Crc, bin::Tex>,

    pub ibuff_info_map: HashMap<u32, usize>,
    pub vbuff_info_map: HashMap<u32, usize>,
    pub vertex_formats: HashMap<(u32, u32), (Vec<(u32, pak::VertexUsage)>, usize)>,
    pub vbuffs: Vec<Vec<pak::VertexBuffer>>,
    pub ibuffs: Vec<Vec<pak::IndexBuffer>>,

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

    pub fn dump<O: ByteOrder + 'static, P: AsRef<Path>>(&mut self, path: P, compress: bool) {
        let path = path.as_ref();
        let (pak, bin) = self.to_data::<O>(compress);
        fs::write(path.with_extension("PAK"), pak).unwrap();
        fs::write(path.with_extension("BIN"), bin).unwrap();
    }

    pub fn from_data<O: ByteOrder + 'static>(bin_data: &[u8], pak_data: &[u8]) -> Self {
        let time = Instant::now();
        println!("extracting level");

        let mut val = Self::default();
        let lua = lua_stuff::LuaCompiler::new().unwrap();
        val.bin_header = OrderedData::from_bytes::<O>(bin_data);
        val.bin_strings = types::Strings::from_data::<O>(bin_data, val.bin_header.strings_offset as usize, val.bin_header.strings_num as usize);
        val.pak_header = OrderedData::from_bytes::<O>(pak_data);
        val.pak_strings = types::Strings::from_data::<O>(pak_data, val.pak_header.strings_offset as usize, val.pak_header.strings_num as usize);
        types::update_strings(&val.bin_strings.strings);
        types::update_strings(&val.pak_strings.strings);
        println!("headers in {:?}", time.elapsed());

        val.asset_handles = OrderedDataVec::from_bytes::<O>(&bin_data[val.bin_header.asset_handle_offset as usize..], val.bin_header.asset_handle_num as usize);
        val.asset_handle_lookup = val.asset_handles.iter().enumerate().map(|(i, info)| ((info.key.key(), info.kind), i)).collect();
        val.asset_data = val.asset_handles.iter().map(|info| (
            (info.key.key(), info.kind), 
            types::CompressedBlock::from_data(bin_data, info.size as usize, info.size_comp as usize, info.offset as usize))
        ).collect();
        println!("assets extracted in {:?}", time.elapsed());

        val.block1 = types::CompressedBlock::from_data(pak_data, val.pak_header.block1_size as usize, val.pak_header.block1_size_comp as usize, val.pak_header.block1_offset as usize);
        val.block2 = types::CompressedBlock::from_data(pak_data, val.pak_header.block2_size as usize, val.pak_header.block2_size_comp as usize, val.pak_header.block2_offset as usize);
        println!("main blocks extracted in {:?}", time.elapsed());

        val.objas = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.obja_offset as usize..], val.pak_header.obja_num as usize);
        val.obj0s = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.obj0_offset as usize..], val.pak_header.obj0_num as usize);
        val.mesh_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.mesh_info_offset as usize..], val.pak_header.mesh_info_num as usize);
        val.buffer_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.buffer_info_offset as usize..], val.pak_header.buffer_info_num as usize);
        val.mat1s = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.mat1_offset as usize..], val.pak_header.mat1_num as usize);
        val.mat2s = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.mat2_offset as usize..], val.pak_header.mat2_num as usize);
        val.mat3s = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.mat3_offset as usize..], val.pak_header.mat3_num as usize);
        val.mat4s = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.mat4_offset as usize..], val.pak_header.mat4_num as usize);
        val.mat_extras = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.mat_extra_offset as usize..], val.pak_header.mat_extra_num as usize);
        val.shape_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.shape_info_offset as usize..], val.pak_header.shape_info_num as usize);
        val.hk_shape_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.hk_shape_info_offset as usize..], val.pak_header.hk_shape_info_num as usize);
        val.hk_constraint_datas = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.hk_constraint_data_offset as usize..], val.pak_header.hk_constraint_data_num as usize);
        val.vbuff_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.vbuff_info_offset as usize..], val.pak_header.vbuff_info_num as usize);
        val.ibuff_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.ibuff_info_offset as usize..], val.pak_header.ibuff_info_num as usize);
        val.texture_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.texture_info_offset as usize..], val.pak_header.texture_info_num as usize);
        val.animation_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.animation_info_offset as usize..], val.pak_header.animation_info_num as usize);
        val.hk_constraint_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.hk_constraint_info_offset as usize..], val.pak_header.hk_constraint_info_num as usize);
        val.gameobj_block_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.gameobj_block_info_offset as usize..], val.pak_header.gameobj_block_info_num as usize);
        val.foliage_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.foliage_info_offset as usize..], val.pak_header.foliage_info_num as usize);
        val.pfield_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.pfield_info_offset as usize..], val.pak_header.pfield_info_num as usize);
        val.gfx_block_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.gfx_block_info_offset as usize..], val.pak_header.gfx_block_info_num as usize);
        val.obj14_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.obj14_info_offset as usize..], val.pak_header.obj14_info_num as usize);
        val.animation_block_infos = OrderedDataVec::from_bytes::<O>(&val.block1.data[val.pak_header.animation_block_info_offset as usize..], val.pak_header.animation_block_info_num as usize);
        println!("packed items extracted in {:?}", time.elapsed());

        val.meshes = val.mesh_infos.iter().map(|info| pak::Mesh::from_data::<O>(val.block1.data.as_slice(), info)).collect();
        val.shapes = val.shape_infos.iter().map(|info| pak::Shape::from_data::<O>(val.block1.data.as_slice(), info)).collect();
        val.hk_shapes = val.hk_shape_infos.iter().map(|info| pak::HkShape::from_data::<O>(val.block1.data.as_slice(), info)).collect();
        val.hk_constraints = val.hk_constraint_infos.iter().map(|info| pak::HkConstraint::from_data::<O>(val.block1.data.as_slice(), info)).collect();
        val.gameobj_blocks = val.gameobj_block_infos.iter().map(|info| types::GameObjs::from_data::<O>(val.block1.data.as_slice(), info.offset as usize, info.size as usize)).collect();
        val.gfx_blocks = val.gfx_block_infos.iter().map(|info| types::Data::from_data(val.block1.data.as_slice(), info.offset as usize, info.size as usize)).collect();
        val.obj14s = val.obj14_infos.iter().map(|info| pak::Obj14::from_data::<O>(val.block1.data.as_slice(), info)).collect();
        val.foliages = val.foliage_infos.iter().map(|info| pak::Foliage::from_data::<O>(val.block1.data.as_slice(), info)).collect();
        println!("item extra extracted in {:?}", time.elapsed());

        val.animation_blocks = val.animation_block_infos.iter().map(|info| types::CompressedBlock::from_data(pak_data, info.size as usize, info.size_comp as usize,info.offset as usize)).collect();
        val.animations = val.animation_infos.iter().map(|_| pak::Animation::default()).collect();
        for (i, block) in val.animation_blocks.iter().enumerate() {
            pak::Animation::unpack_block::<O>(&mut val.animations[..], &val.animation_infos[..], &block.data[..], 0, i);
        }
        println!("animations extracted in {:?}", time.elapsed());

        val.string_keys = types::StringKeys::from_data::<O>(&val.block1.data[..], val.pak_header.string_keys_offset as usize);
        val.sub_blocks1 = types::SubBlocks::from_data::<O>(&val.block1.data[..], val.pak_header.sub_blocks1_offset as usize, &lua);
        val.sub_blocks2 = types::SubBlocks::from_data::<O>(&val.block2.data[..], val.pak_header.sub_blocks2_offset as usize, &lua);
        val.block2_offsets = OrderedDataVec::from_bytes::<O>(&val.block2.data[val.pak_header.block2_offsets_offset as usize..], val.pak_header.block2_offsets_num as usize);
        println!("sub blocks extracted in {:?}", time.elapsed());

        val.radiosity = val.asset_handles.iter().filter(|info| info.key.str().map(|x| x.ends_with("_radiosity")).unwrap_or(false)).map(|info| (
            (info.key.key(), info.kind),
            bin::Radiosity::from_data::<O>(&val.asset_data.get(&(info.key.key(), info.kind)).unwrap().data[..])
        )).collect();

        val.textures = val.texture_infos.iter_mut().filter_map(|info| {
            let data0 = &val.asset_data.get(&(info.asset_key.key(), info.asset_type)).unwrap().data;
            let data1 = &val.asset_data.get(&(hash_string("*".as_bytes(), Some(info.asset_key.key())), info.asset_type)).unwrap().data;
            match info.kind {
                0 | 7 | 8 => Some((info.asset_key.clone(), bin::Tex::Texture(bin::Texture::from_data::<O>(&data0[..], &data1[..], info)))),
                1 | 9 => Some((info.asset_key.clone(), bin::Tex::CubeTexture(bin::CubeTexture::from_data::<O>(&data0[..], &data1[..], info)))),
                _ => {
                    warn!("Unsupported Texture Type {}", info.kind);
                    None
                }
            }
        }).collect();
        println!("textures extracted in {:?}", time.elapsed());

        val.ibuff_info_map = (0..val.pak_header.ibuff_info_num).map(|i| (val.pak_header.ibuff_info_offset + pak::IBuffInfo::size::<O>() as u32 * i, i as usize)).collect();
        val.vbuff_info_map = (0..val.pak_header.vbuff_info_num).map(|i| (val.pak_header.vbuff_info_offset + pak::VBuffInfo::size::<O>() as u32 * i, i as usize)).collect();

        for (mesh, info) in std::iter::zip(&val.meshes, &val.mesh_infos) {
            if info.vbuff_num == 0 && info.ibuff_num == 0 { // no buffer
                val.vbuffs.push(vec![]);
                val.ibuffs.push(vec![]);
            } else {
                let buffer = &val.asset_data.get(&(info.asset_key, info.asset_type)).unwrap().data;
                val.vbuffs.push(mesh.vbuffs.iter().map(|info| pak::VertexBuffer::from_data::<O>(&buffer[..], &val.vbuff_infos[*val.vbuff_info_map.get(&info).unwrap()], &mut val.vertex_formats)).collect());
                val.ibuffs.push(mesh.ibuffs.iter().map(|info| pak::IndexBuffer::from_data::<O>(&buffer[..], &val.ibuff_infos[*val.ibuff_info_map.get(&info).unwrap()])).collect());    
            }
        }

        val.pak_block_a = OrderedDataVec::from_bytes::<O>(&pak_data[val.pak_header.block_a_offset as usize..], val.pak_header.block_a_num as usize);
        println!("buffers extracted in {:?}", time.elapsed());

        val
    }
    
    pub fn to_data<O: ByteOrder + 'static>(&mut self, compress: bool) -> (Vec<u8>, Vec<u8>) {
        let time: Instant = Instant::now();
        println!("compressing level");

        let lua = lua_stuff::LuaCompiler::new().unwrap();

        let mut bin_data = vec![0u8; bin::Header::size::<O>()];
        let mut dump_bin_header = self.bin_header.clone();
        dump_bin_header.version = if TypeId::of::<O>() == TypeId::of::<LE>() { 1 } else { 2 };
        let mut dump_asset_handles = self.asset_handles.clone();
        for asset_handle in &mut dump_asset_handles {
            asset_handle.size = 0;
            asset_handle.size_comp = 0;
            asset_handle.offset = 0;
        }

        let off = (bin_data.len() + 2047) & 0xfffff800;
        bin_data.extend(vec![0u8; off-bin_data.len()]);
        for ((mesh, info), (vbuffs, ibuffs)) in zip(zip(&self.meshes, &self.mesh_infos), zip(&self.vbuffs, &self.ibuffs)) {
            let key = (info.asset_key, info.asset_type);
            if (info.vbuff_num != 0) || (info.ibuff_num != 0) {
                let i = *self.asset_handle_lookup.get(&key).unwrap();
                let asset_handle = dump_asset_handles.get_mut(i).unwrap();
                let mut data = vec![0u8; self.asset_handles[i].size as usize];
                for (vbuff, info) in zip(vbuffs, &mesh.vbuffs) {
                    vbuff.into_data::<O>(&mut data, &self.vbuff_infos[*self.vbuff_info_map.get(&info).unwrap()]);
                }
                for (ibuff, info) in zip(ibuffs, &mesh.ibuffs) {
                    ibuff.into_data::<O>(&mut data[self.ibuff_infos[*self.ibuff_info_map.get(&info).unwrap()].offset as usize..]);
                }
                asset_handle.size = data.len() as u32;
                if compress {
                    data = types::CompressedBlock { data }.dump();
                    asset_handle.size_comp = data.len() as u32;
                }
                asset_handle.offset = bin_data.len() as u32;
                bin_data.extend(data);
                let off = (bin_data.len() + 2047) & 0xfffff800;
                bin_data.extend(vec![0u8; off-bin_data.len()]);            
            }
        }

        for (key, texture) in self.textures.iter() {
            let (data0, data1) = texture.dump::<O>();
            let i = *self.asset_handle_lookup.get(&(key.key(), texture.kind())).unwrap();
            let j = *self.asset_handle_lookup.get(&(hash_string("*".as_bytes(), Some(key.key())), texture.kind())).unwrap();

            let asset_handle = dump_asset_handles.get_mut(i).unwrap();
            let mut data = data0;
            asset_handle.size = data.len() as u32;
            if compress {
                data = types::CompressedBlock { data }.dump();
                asset_handle.size_comp = data.len() as u32;
            }
            asset_handle.offset = bin_data.len() as u32;
            bin_data.extend(data);
            let off = (bin_data.len() + 2047) & 0xfffff800;
            bin_data.extend(vec![0u8; off-bin_data.len()]);

            let asset_handle = dump_asset_handles.get_mut(j).unwrap();
            let mut data = data1;
            asset_handle.size = data.len() as u32;
            if compress {
                data = types::CompressedBlock { data }.dump();
                asset_handle.size_comp = data.len() as u32;
            }
            asset_handle.offset = bin_data.len() as u32;
            bin_data.extend(data);
            let off = (bin_data.len() + 2047) & 0xfffff800;
            bin_data.extend(vec![0u8; off-bin_data.len()]);
        }

        for (key, radiosity) in self.radiosity.iter() {
            let i = *self.asset_handle_lookup.get(key).unwrap();
            let mut data = radiosity.dump::<O>();

            let asset_handle = dump_asset_handles.get_mut(i).unwrap();
            asset_handle.size = data.len() as u32;
            if compress {
                data = types::CompressedBlock { data }.dump();
                asset_handle.size_comp = data.len() as u32;
            }
            asset_handle.offset = bin_data.len() as u32;
            bin_data.extend(data);
            let off = (bin_data.len() + 2047) & 0xfffff800;
            bin_data.extend(vec![0u8; off-bin_data.len()]);
        }

        println!("assets in {:?}", time.elapsed());

        dump_bin_header.asset_handle_offset = bin_data.len() as u32;
        dump_bin_header.asset_handle_num = dump_asset_handles.len() as u32;
        let data = dump_asset_handles.dump_bytes::<O>();
        bin_data.extend(data);

        dump_bin_header.strings_offset = bin_data.len() as u32;
        dump_bin_header.strings_num = self.bin_strings.strings.len() as u32;
        let data = self.bin_strings.dump::<O>();
        dump_bin_header.strings_size = data.len() as u32;
        bin_data.extend(data);
        
        let off = (bin_data.len() + 2047) & 0xfffff800;
        bin_data.extend(vec![0u8; off-bin_data.len()]);
        dump_bin_header.to_bytes::<O>(&mut bin_data[..]);

        println!("bin in {:?}", time.elapsed());

        let mut pak_data = vec![0u8; pak::Header::size::<O>()];
        let mut dump_pak_header = self.pak_header.clone();
        dump_pak_header.version = if TypeId::of::<O>() == TypeId::of::<LE>() { 1 } else { 2 };

        self.dump_animation_blocks.clear();
        self.dump_animation_block_infos = self.animation_block_infos.clone();
        for (i, info) in self.dump_animation_block_infos.iter_mut().enumerate() {
            let mut data = vec![0u8; info.size as usize];
            pak::Animation::pack_block::<O>(&self.animations[..], &self.animation_infos[..], &mut data[..], 0, i);
            let off = (pak_data.len() + 4095) & 0xfffff000;
            pak_data.extend(vec![0u8; off-pak_data.len()]);
            info.size = data.len() as u32;
            let block = types::CompressedBlock { data }; 
            let data_comp = block.dump();
            info.size_comp = data_comp.len() as u32;
            info.offset = pak_data.len() as u32;
            pak_data.extend(data_comp);
            self.dump_animation_blocks.push(block);

        }
        println!("animations in {:?}", time.elapsed());

        let mut dump_block1 = vec![0u8; dump_pak_header.sub_blocks1_offset as usize];

        for (foliage, info) in zip(&self.foliages, &self.foliage_infos) {
            foliage.into_data::<O>(&mut dump_block1, info);
        }
        for (obj14, info) in zip(&self.obj14s, &self.obj14_infos) {
            obj14.into_data::<O>(&mut dump_block1, info);
        }
        for (gfx_block, info) in zip(&self.gfx_blocks, &self.gfx_block_infos) {
            gfx_block.into_data(&mut dump_block1, info.offset as usize);
        }

        for (mesh, info) in zip(&self.meshes, &self.mesh_infos) {
            mesh.into_data::<O>(&mut dump_block1, info);
        }
        for (shape, info) in zip(&self.shapes, &self.shape_infos) {
            shape.into_data::<O>(&mut dump_block1, info);
        }
        for (hk_shape, info) in zip(&self.hk_shapes, &self.hk_shape_infos) {
            hk_shape.into_data::<O>(&mut dump_block1, info);
        }
        for (hk_constraint, info) in zip(&self.hk_constraints, &self.hk_constraint_infos) {
            hk_constraint.into_data::<O>(&mut dump_block1, info);
        }
        for (gameobj_block, info) in zip(&self.gameobj_blocks, &self.gameobj_block_infos) {
            gameobj_block.into_data::<O>(&mut dump_block1, info.offset as usize);
        }
        println!("item extras in {:?}", time.elapsed());

        self.objas.to_bytes::<O>(&mut dump_block1[dump_pak_header.obja_offset as usize..]);
        self.obj0s.to_bytes::<O>(&mut dump_block1[dump_pak_header.obj0_offset as usize..]);
        self.mesh_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.mesh_info_offset as usize..]);
        self.buffer_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.buffer_info_offset as usize..]);
        self.mat1s.to_bytes::<O>(&mut dump_block1[dump_pak_header.mat1_offset as usize..]);
        self.mat2s.to_bytes::<O>(&mut dump_block1[dump_pak_header.mat2_offset as usize..]);
        self.mat3s.to_bytes::<O>(&mut dump_block1[dump_pak_header.mat3_offset as usize..]);
        self.mat4s.to_bytes::<O>(&mut dump_block1[dump_pak_header.mat4_offset as usize..]);
        self.mat_extras.to_bytes::<O>(&mut dump_block1[dump_pak_header.mat_extra_offset as usize..]);
        self.shape_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.shape_info_offset as usize..]);
        self.hk_shape_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.hk_shape_info_offset as usize..]);
        self.hk_constraint_datas.to_bytes::<O>(&mut dump_block1[dump_pak_header.hk_constraint_data_offset as usize..]);
        self.vbuff_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.vbuff_info_offset as usize..]);
        self.ibuff_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.ibuff_info_offset as usize..]);
        self.texture_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.texture_info_offset as usize..]);
        self.animation_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.animation_info_offset as usize..]);
        self.hk_constraint_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.hk_constraint_info_offset as usize..]);
        self.gameobj_block_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.gameobj_block_info_offset as usize..]);
        self.foliage_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.foliage_info_offset as usize..]);
        self.pfield_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.pfield_info_offset as usize..]);
        self.gfx_block_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.gfx_block_info_offset as usize..]);
        self.obj14_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.obj14_info_offset as usize..]);
        self.dump_animation_block_infos.to_bytes::<O>(&mut dump_block1[dump_pak_header.animation_block_info_offset as usize..]);
        println!("packed items in {:?}", time.elapsed());

        let obj_map: HashMap<_,_> = self.ibuff_info_map.iter().map(
            |(off, i)| (*off, dump_pak_header.ibuff_info_offset+(pak::IBuffInfo::size::<O>()*i) as u32)
        ).chain(self.vbuff_info_map.iter().map(
            |(off, i)| (*off, dump_pak_header.vbuff_info_offset+(pak::VBuffInfo::size::<O>()*i) as u32)
        )).collect();

        for offset in &self.block2_offsets {
            if let Some(new_val) = obj_map.get(&u32::from_bytes::<O>(&dump_block1[*offset as usize..])) {
                new_val.to_bytes::<O>(&mut dump_block1[*offset as usize..]);
            }
        }

        let off = (dump_block1.len() + 15) & 0xfffffff0;
        dump_block1.extend(vec![0u8; off-dump_block1.len()]);
        dump_pak_header.sub_blocks1_offset = dump_block1.len() as u32;
        dump_block1.extend(self.sub_blocks1.dump::<O>(&lua));

        let off = (dump_block1.len() + 31) & 0xffffffe0;
        dump_block1.extend(vec![0u8; off-dump_block1.len()]);
        dump_pak_header.string_keys_offset = dump_block1.len() as u32;
        dump_block1.extend(self.string_keys.dump::<O>());

        dump_pak_header.sub_blocks2_offset = 0;
        let mut dump_block2 = self.sub_blocks2.dump::<O>(&lua);
        dump_pak_header.block2_offsets_offset = dump_block2.len() as u32;
        dump_pak_header.block2_offsets_num = self.block2_offsets.len() as u32;
        dump_block2.extend(self.block2_offsets.dump_bytes::<O>());
        println!("sub blocks in {:?}", time.elapsed());

        let off = (pak_data.len() + 4095) & 0xfffff000;
        pak_data.extend(vec![0u8; off-pak_data.len()]);
        dump_pak_header.block1_size = dump_block1.len() as u32;
        self.dump_block1.replace(types::CompressedBlock { data: dump_block1 });
        let data = self.dump_block1.as_ref().unwrap().dump();
        dump_pak_header.block1_offset = pak_data.len() as u32;
        dump_pak_header.block1_size_comp = data.len() as u32;
        pak_data.extend(data);

        let off = (pak_data.len() + 4095) & 0xfffff000;
        pak_data.extend(vec![0u8; off-pak_data.len()]);
        dump_pak_header.block2_size = dump_block2.len() as u32;
        self.dump_block2.replace(types::CompressedBlock { data: dump_block2 });
        let data = self.dump_block2.as_ref().unwrap().dump();
        dump_pak_header.block2_offset = pak_data.len() as u32;
        dump_pak_header.block2_size_comp = data.len() as u32;
        pak_data.extend(data);

        let off = (pak_data.len() + 4095) & 0xfffff000;
        pak_data.extend(vec![0u8; off-pak_data.len()]);
        let data = self.pak_strings.dump::<O>();
        dump_pak_header.strings_offset = pak_data.len() as u32;
        dump_pak_header.strings_num = self.pak_strings.strings.len() as u32;
        dump_pak_header.strings_size = data.len() as u32;
        pak_data.extend(data);

        dump_pak_header.block_a_offset = pak_data.len() as u32;
        dump_pak_header.block_a_num = self.pak_block_a.len() as u32;
        pak_data.extend(self.pak_block_a.dump_bytes::<O>());

        dump_pak_header.to_bytes::<O>(&mut pak_data[..]);
        println!("pak in {:?}", time.elapsed());

        (pak_data, bin_data)
    }


    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        let time: Instant = Instant::now();
        println!("storing level");

        let path = path.as_ref();
        std::fs::create_dir(path).ok();
        std::fs::create_dir(path.join("assets")).ok();
        std::fs::create_dir(path.join("assets").join("raw")).ok();
        self.bin_header.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("bin_header")).unwrap()))).unwrap();
        self.bin_header.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("bin_header")).unwrap()))).unwrap();
        self.bin_strings.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("bin_strings")).unwrap()))).unwrap();
        self.asset_handles.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("asset_handles")).unwrap()))).unwrap();

        self.pak_header.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("pak_header")).unwrap()))).unwrap();
        self.pak_strings.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("pak_strings")).unwrap()))).unwrap();
        println!("headers in {:?}", time.elapsed());

        self.block1.to_file(path.join("block1"));
        self.block2.to_file(path.join("block2"));
        if let Some(block1) = &self.dump_block1 {
            block1.to_file(path.join("dump_block1"));
        }
        if let Some(block2) = &self.dump_block2 {
            block2.to_file(path.join("dump_block2"));
        }
        println!("blocks in {:?}", time.elapsed());

        self.objas.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("objas")).unwrap()))).unwrap();
        self.obj0s.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("obj0s")).unwrap()))).unwrap();
        self.mesh_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("mesh_infos")).unwrap()))).unwrap();
        self.buffer_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("buffer_infos")).unwrap()))).unwrap();
        self.mat1s.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("mat1s")).unwrap()))).unwrap();
        self.mat2s.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("mat2s")).unwrap()))).unwrap();
        self.mat3s.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("mat3s")).unwrap()))).unwrap();
        self.mat4s.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("mat4s")).unwrap()))).unwrap();
        self.mat_extras.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("mat_extras")).unwrap()))).unwrap();
        self.shape_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("shape_infos")).unwrap()))).unwrap();
        self.hk_shape_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("hk_shape_infos")).unwrap()))).unwrap();
        self.hk_constraint_datas.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("hk_constraint_datas")).unwrap()))).unwrap();
        self.vbuff_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("vbuff_infos")).unwrap()))).unwrap();
        self.ibuff_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("ibuff_infos")).unwrap()))).unwrap();
        self.texture_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("texture_infos")).unwrap()))).unwrap();
        self.animation_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("animation_infos")).unwrap()))).unwrap();
        self.hk_constraint_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("hk_constraint_infos")).unwrap()))).unwrap();
        self.gameobj_block_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("gameobj_block_infos")).unwrap()))).unwrap();
        self.foliage_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("foliage_infos")).unwrap()))).unwrap();
        self.pfield_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("pfield_infos")).unwrap()))).unwrap();
        self.gfx_block_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("gfx_block_infos")).unwrap()))).unwrap();
        self.obj14_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("obj14_infos")).unwrap()))).unwrap();
        self.animation_block_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("animation_block_infos")).unwrap()))).unwrap();
        self.dump_animation_block_infos.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("dump_animation_block_infos")).unwrap()))).unwrap();
        println!("packed items in {:?}", time.elapsed());

        // self.meshes.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("meshes")).unwrap()))).unwrap();
        fs::write(path.join("meshes.json"), serde_json::to_string_pretty(&self.meshes).unwrap()).unwrap();
        println!("meshes in {:?}", time.elapsed());
        self.hk_shapes.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("hk_shapes")).unwrap()))).unwrap();
        println!("shapes in {:?}", time.elapsed());
        self.hk_constraints.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("hk_constraints")).unwrap()))).unwrap();
        println!("constriaints in {:?}", time.elapsed());
        self.gameobj_blocks.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("gameobj_blocks")).unwrap()))).unwrap();
        println!("gameobjs in {:?}", time.elapsed());
        self.gfx_blocks.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("gfx_blocks")).unwrap()))).unwrap();
        println!("gfx_blocks in {:?}", time.elapsed());
        self.obj14s.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("obj14s")).unwrap()))).unwrap();
        println!("obj14s in {:?}", time.elapsed());
        println!("packed item extras in {:?}", time.elapsed());

        // for (i, anim) in self.animation_blocks.iter().enumerate() {
        //     anim.to_file(path.join(format!("animation_block_{}", i)));
        // }
        // for (i, anim) in self.dump_animation_blocks.iter().enumerate() {
        //     anim.to_file(path.join(format!("dump_animation_block_{}", i)));
        // }
        // self.animations.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("animations")).unwrap()))).unwrap();
        // println!("animations in {:?}", time.elapsed());

        self.string_keys.to_file(path.join("string_keys"));
        self.sub_blocks1.to_file(path.join("sub_blocks1"), &self.string_keys);
        self.sub_blocks2.to_file(path.join("sub_blocks2"), &self.string_keys);
        self.block2_offsets.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("block2_offsets")).unwrap()))).unwrap();
        println!("sub blocks in {:?}", time.elapsed());

        // for (i, info) in self.asset_handles.iter().enumerate() {
        //     self.asset_data.get(&(info.key.key(), info.kind)).unwrap().to_file(path.join("assets").join("raw").join(format!("data_{}", i)));
        // }

        // for info in self.asset_handles.iter().filter(|info| info.key.str().map(|x| x.ends_with("_radiosity")).unwrap_or(false)){
        //     self.radiosity.get(&(info.key.key(), info.kind)).unwrap().serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("assets").join(info.key.str().unwrap())).unwrap()))).unwrap();
        // }
        // for (k, val) in self.textures.iter() {
        //     val.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("assets").join(k.str().unwrap())).unwrap()))).unwrap();
        // }

        // self.vertex_formats.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("vertex_formats")).unwrap()))).unwrap();
    
        // for (info, (vbuff, ibuff)) in std::iter::zip(&self.mesh_infos, std::iter::zip(&self.vbuffs, &self.ibuffs)) {
        //     if info.vbuff_num != 0 || info.ibuff_num != 0 {
        //         vbuff.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("assets").join(format!("{}_vbuff", info.key.str().unwrap()))).unwrap()))).unwrap();
        //         ibuff.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("assets").join(format!("{}_ibuff", info.key.str().unwrap()))).unwrap()))).unwrap();
        //     }
        // }
        // println!("assets in {:?}", time.elapsed());
    
        self.pak_block_a.serialize(&mut Serializer::new(IoWrite::new(File::create(path.join("pak_block_a")).unwrap()))).unwrap();
    }
}