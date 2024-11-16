use std::{any::TypeId, collections::{HashMap, HashSet}, iter::zip};
use itertools::Itertools;
use log::warn;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use gltf::{
    json::{Root, Index, validation::Checked, buffer::{View, Target, Buffer, Stride}, Accessor, accessor::GenericComponentType, Node},
    accessor::{DataType, Dimensions},
};
use anyhow::{anyhow, Result};
use pyo3::prelude::*;

use lotrc_proc::{OrderedData, basicpymethods, PyMethods};
use crate::{
    types::{Crc, OrderedData, Vector4, Matrix4x4, OrderedDataVec, OrderedDataImpl, Version, PS3, PC},
    pak::{
        ModelInfo, ValA, model, MatExtra, VBuffInfo, IBuffInfo, BufferInfo, HkConstraintInfo, HkConstraintData,
        VertexBuffer, IndexBuffer, ShapeInfo, VertexUsage, LodMeshes, Header, AnimationInfo, animation, 
        Mat1, Mat2, Mat3, Mat4, MatBase, get_vertex_format
    },
};

mod shape;
pub use shape::*;

#[basicpymethods]
#[pyclass(module="pak_alt", name="Model", get_all, set_all)]
#[derive(Default, Debug, Clone, Serialize, Deserialize, PyMethods)]
pub struct Model {
    pub info: ModelInfo,
    pub bone_parents: Vec<i32>, // parent bone
    pub bones: Vec<Crc>, // bone names
    pub bone_transforms: Vec<Matrix4x4>, // relative bone transforms, used for static meshes 
    pub vals_a: Vec<ValA>, // probably f32
    pub mat_order: Vec<u32>,
    pub mesh_order: Vec<u32>, // order of models (mapped to lod0, lod1, lod2, lod3)
    pub vals_d: Vec<ValA>, // 1 per contained model, stores result of some absolute position calculation?
    pub vbuff_order: Vec<u32>,
    pub ibuff_order: Vec<u32>,
    pub skin_binds: Vec<Matrix4x4>, // mat4, bind matrices or something??
    pub vals_j: Vec<u32>, // bows & banners, maybe for strings?
    pub val_k_header: Vec<u16>,
    pub vals_k: Vec<u32>, // has to do with trees
    pub skin_order: Vec<u32>, // bone mapping for vals_g, seems to be the mapping used for skinning
    pub slots: Vec<Key2>, // attachment points
    pub slot_map: Vec<u32>, // attachment bone mapping
    pub block_header: Option<u32>, // has to do with havok cloth / hair stuff
    pub block_offsets: Vec<u32>,
    pub blocks: Vec<(model::BlockHeader, Vec<u32>, Vec<model::BlockVal>, Vec<u32>)>,
    pub mats: Vec<Mat>,
    pub mat_extras: Vec<Option<MatExtra>>,
    pub vbuffs: Vec<VBuffInfo>,
    pub ibuffs: Vec<IBuffInfo>,
    pub buffer_infos: Vec<BufferInfo>,
    pub hk_constraint: Option<HkConstraint>, // stores bone transforms used for ragdoll ??
    pub hk_constraint_datas: Vec<HkConstraintData>,
    pub shapes: Vec<Shape>,
    pub vertex_data: Vec<VertexBuffer>,
    pub index_data: Vec<IndexBuffer>,
}

impl Model {
    pub fn from_data<O: Version + 'static>(info: ModelInfo, data: &[u8]) -> Result<Self> {
        let bone_parents: Vec<i32> = OrderedDataVec::from_bytes::<O>(&data[info.bone_parents_offset as usize..], info.bones_num as usize)?;
        let bones: Vec<Crc> = if info.bones_offset != 0 {
            OrderedDataVec::from_bytes::<O>(&data[info.bones_offset as usize..], info.bones_num as usize)?
        } else { vec![Crc::Key(0); info.bones_num as usize] };
        let bone_transforms: Vec<Matrix4x4> = OrderedDataVec::from_bytes::<O>(&data[info.bone_transforms_offset as usize..], info.bones_num as usize)?;
        let vals_a: Vec<ValA> = OrderedDataVec::from_bytes::<O>(&data[info.vals_a_offset as usize..], info.bones_num as usize)?;
        let mut mat_order: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.mat_offset as usize..], info.mat_num as usize)?;
        let mesh_order: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.mesh_order_offset as usize..], info.lod3.breakable_end as usize)?;
        let vals_d: Vec<ValA> = OrderedDataVec::from_bytes::<O>(&data[info.vals_d_offset as usize..], info.lod3.breakable_end as usize)?;
        let mut vbuff_order: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.vbuff_offset as usize..], info.vbuff_num as usize)?;
        let mut ibuff_order: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.ibuff_offset as usize..], info.ibuff_num as usize)?;
        let skin_binds: Vec<Matrix4x4> = OrderedDataVec::from_bytes::<O>(&data[info.skin_binds_offset as usize..], info.skin_binds_num as usize)?;
        let vals_j: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.vals_j_offset as usize..], info.vals_j_num as usize)?;
        let (val_k_header, vals_k) = if info.vals_k_offset != 0 {(
            OrderedDataVec::from_bytes::<O>(&data[info.vals_k_offset as usize..], 2)?,
            OrderedDataVec::from_bytes::<O>(&data[info.vals_k_offset as usize + 4..], 35)?
        )} else {(
            Vec::new(), Vec::new()
        )};
        let skin_order = if info.skin_order_offset != 0 {
            OrderedDataVec::from_bytes::<O>(&data[info.skin_order_offset as usize..], info.skin_binds_num as usize)?
        } else {
            Vec::new()
        };
        let (slots, slot_map) = if info.slots_offset != 0 {
            assert!(info.slot_map_offset != 0);
            let mut i = 0;
            {
                while u32::from_bytes::<O>(&data[info.slots_offset as usize + i * 8..])? != 0 {
                    i += 1;
                }
                i += 1;
            }
            let keys2: Vec<Key2> = OrderedDataVec::from_bytes::<O>(&data[info.slots_offset as usize..], i)?;
            let keys2_order = OrderedDataVec::from_bytes::<O>(&data[info.slot_map_offset as usize..], keys2.last().unwrap().val as usize)?;
            (keys2, keys2_order)
        } else {(
            Vec::new(), Vec::new()
        )};
        let (block_header, block_offsets, blocks) = if info.block_offset != 0 {
            let block_header = OrderedData::from_bytes::<O>(&data[info.block_offset as usize..])?;
            let n = (info.lod0.physics_end - info.lod0.skinned_end) as usize;
            let block_offsets: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.block_offset as usize + 4..], n+1)?;
            let mut blocks = Vec::with_capacity(n);
            for i in 0..n {
                let size = (block_offsets[i+1] - block_offsets[i]) as usize;
                let offset = (block_offsets[i] + info.block_offset) as usize;
                let header: model::BlockHeader = OrderedData::from_bytes::<O>(&data[offset..])?;
                let mut s = model::BlockHeader::size::<O>();
                let vals_a: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (header.a + header.b) as usize * 12)?;
                s += vals_a.size::<O>();
                let vals_b: Vec<model::BlockVal> = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (size - s)/model::BlockVal::size::<O>())?;
                s += vals_b.size::<O>();
                let extra = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (size - s)/4)?;
                blocks.push((header, vals_a, vals_b, extra));
            }
            (Some(block_header), block_offsets, blocks)
        } else {(
            None, Vec::new(), Vec::new()
        )};

        assert!(bone_parents[0] == -1);

        let shapes = (0..info.shape_num as usize).map(|i| 
            Shape::from_data::<O>(data, info.shape_offset as usize + i * ShapeInfo::size::<O>())
        ).collect::<Result<Vec<_>>>()?;

        let hk_constraint = if info.hk_constraint_offset != 0 {
            Some(HkConstraint::from_data::<O>(data, info.hk_constraint_offset as usize)?) 
        } else { None };
        let hk_constraint_datas: Vec<HkConstraintData> = OrderedDataVec::from_bytes::<O>(&data[info.hk_constraint_data_offset as usize..], info.hk_constraint_data_num as usize)?;
        
        let mats = HashSet::<u32>::from_iter(mat_order.iter().cloned()).into_iter().sorted().collect::<Vec<_>>();
        let mat_map: HashMap<_, _> = mats.iter().enumerate().map(|(i, x)| (*x, i as u32)).collect();
        mat_order.iter_mut().for_each(|x| *x = *mat_map.get(x).unwrap());
        let mats: Vec<Mat> = mats.into_iter().map(|off| Mat::from_data::<O>(data, off as usize)).collect::<Result<Vec<_>>>()?;
        let mat_extras: Vec<_> = mats.iter().map(|x| 
            Ok(if x.base().mat_extra_offset != 0 { Some(OrderedData::from_bytes::<O>(&data[x.base().mat_extra_offset as usize..])?) } else { None })
        ).collect::<Result<Vec<_>>>()?;

        let vbuffs = HashSet::<u32>::from_iter(vbuff_order.iter().cloned()).into_iter().sorted().collect::<Vec<_>>();
        let mut vbuff_map: HashMap<_, _> = vbuffs.iter().enumerate().map(|(i, x)| (*x, i as u32)).collect();
        vbuff_order.iter_mut().for_each(|x| *x = *vbuff_map.get(x).unwrap());
        let vbuffs: Vec<VBuffInfo> = vbuffs.into_iter().map(|off| OrderedData::from_bytes::<O>(&data[off as usize..])).collect::<Result<Vec<_>>>()?;
        let vertex_data = Vec::with_capacity(vbuffs.len());

        let ibuffs = HashSet::<u32>::from_iter(ibuff_order.iter().cloned()).into_iter().sorted().collect::<Vec<_>>();
        let mut ibuff_map: HashMap<_, _> = ibuffs.iter().enumerate().map(|(i, x)| (*x, i as u32)).collect();
        ibuff_order.iter_mut().for_each(|x| *x = *ibuff_map.get(x).unwrap());
        let ibuffs: Vec<IBuffInfo> = ibuffs.into_iter().map(|off| OrderedData::from_bytes::<O>(&data[off as usize..])).collect::<Result<Vec<_>>>()?;
        let index_data = Vec::with_capacity(ibuffs.len());

        ibuff_map.insert(0, 0xFFFFFFFF);
        vbuff_map.insert(0, 0xFFFFFFFF);
        let mut buffer_infos: Vec<BufferInfo> = OrderedDataVec::from_bytes::<O>(&data[info.buffer_info_offset as usize..], info.mat_num as usize)?;
        buffer_infos.iter_mut().for_each(|buff| {
            buff.vbuff_info_offset = *vbuff_map.get(&buff.vbuff_info_offset).unwrap_or(&buff.vbuff_info_offset);
            buff.vbuff_info_offset_2 = *vbuff_map.get(&buff.vbuff_info_offset_2).unwrap_or(&buff.vbuff_info_offset_2);
            buff.vbuff_info_offset_3 = *vbuff_map.get(&buff.vbuff_info_offset_3).unwrap_or(&buff.vbuff_info_offset_3);
            buff.ibuff_info_offset = *ibuff_map.get(&buff.ibuff_info_offset).unwrap_or(&buff.ibuff_info_offset);
        });

        Ok(Self {
            info,
            bone_parents,
            bones,
            bone_transforms,
            mat_order,
            vals_a,
            mesh_order,
            vals_d,
            vbuff_order,
            ibuff_order,
            skin_binds,
            vals_j,
            val_k_header,
            vals_k,
            skin_order,
            slots,
            slot_map,
            block_header,
            block_offsets,
            blocks,
            mats,
            mat_extras,
            vbuffs,
            ibuffs,
            buffer_infos,
            shapes,
            hk_constraint,
            hk_constraint_datas,
            vertex_data,
            index_data,
        })
    }

    pub fn dump<O: Version + 'static>(&self, mut offset: usize, infos: &mut DumpInfos) -> Vec<u8> {
        let mut info = self.info.clone();
        let mut data = vec![];

        let mut mat_order = self.mat_order.clone();
        let mut mat_map = HashMap::with_capacity(self.mats.len());
        for (i, (mat, mat_extra)) in zip(&self.mats, &self.mat_extras).enumerate() {
            let mut mat = mat.clone();
            if let Some(mat_extra) = mat_extra {
                mat.base_mut().mat_extra_offset = infos.header.mat_extra_offset + (MatExtra::size::<O>() * infos.mat_extra.len()) as u32;
                infos.mat_extra.push(mat_extra.clone())
            }
            match mat {
                Mat::Mat1(mat) => {
                    mat_map.insert(i as u32, infos.header.mat1_offset + (Mat1::size::<O>() * infos.mat1.len()) as u32);
                    infos.mat1.push(mat);
                },
                Mat::Mat2(mat) => {
                    mat_map.insert(i as u32, infos.header.mat2_offset + (Mat2::size::<O>() * infos.mat2.len()) as u32);
                    infos.mat2.push(mat);
                },
                Mat::Mat3(mat) => {
                    mat_map.insert(i as u32, infos.header.mat3_offset + (Mat3::size::<O>() * infos.mat3.len()) as u32);
                    infos.mat3.push(mat);
                },
                Mat::Mat4(mat) => {
                    mat_map.insert(i as u32, infos.header.mat4_offset + (Mat4::size::<O>() * infos.mat4.len()) as u32);
                    infos.mat4.push(mat);
                },
            }
        }
        mat_order.iter_mut().for_each(|x| *x = *mat_map.get(x).unwrap());

        let mut vbuff_map: HashMap<_, _> = (0..self.vbuffs.len()).map(|x| (x as u32, infos.header.vbuff_info_offset + (VBuffInfo::size::<O>() * (infos.vbuff.len() + x)) as u32)).collect();
        let vbuff_order: Vec<u32> = self.vbuff_order.iter().map(|x| *vbuff_map.get(x).unwrap()).collect();
        infos.vbuff.extend(self.vbuffs.clone());

        let mut ibuff_map: HashMap<_, _> = (0..self.ibuffs.len()).map(|x| (x as u32, infos.header.ibuff_info_offset + (IBuffInfo::size::<O>() * (infos.ibuff.len() + x)) as u32)).collect();
        let ibuff_order: Vec<u32> = self.ibuff_order.iter().map(|x| *ibuff_map.get(x).unwrap()).collect();
        infos.ibuff.extend(self.ibuffs.clone());

        ibuff_map.insert(0xFFFFFFFF, 0);
        vbuff_map.insert(0xFFFFFFFF, 0);
        let buffer_infos: Vec<_> = self.buffer_infos.iter().map(|buff| {
            let mut buff = buff.clone();
            buff.vbuff_info_offset = *vbuff_map.get(&buff.vbuff_info_offset).unwrap();
            buff.vbuff_info_offset_2 = *vbuff_map.get(&buff.vbuff_info_offset_2).unwrap();
            buff.vbuff_info_offset_3 = *vbuff_map.get(&buff.vbuff_info_offset_3).unwrap();
            buff.ibuff_info_offset = *ibuff_map.get(&buff.ibuff_info_offset).unwrap();
            buff
        }).collect();
        info.buffer_info_offset = infos.header.buffer_info_offset + (BufferInfo::size::<O>() * infos.buffer.len()) as u32;
        infos.buffer.extend(buffer_infos);

        info.hk_constraint_data_num = self.hk_constraint_datas.len() as u32;
        info.hk_constraint_data_offset = if self.hk_constraint_datas.len() != 0 {
            infos.header.hk_constraint_data_offset + (HkConstraintData::size::<O>() * infos.hk_constraint_data.len()) as u32
        } else { 0 };
        infos.hk_constraint_data.extend(self.hk_constraint_datas.clone());

        info.bones_offset = offset as u32;
        info.bones_num = self.bones.len() as u32;
        let vals = self.bones.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        let off = (offset+ 15) & 0xFFFFFFF0;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        info.vals_a_offset = offset as u32;
        let vals = self.vals_a.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.vals_j_offset = offset as u32;
        info.vals_j_num = self.vals_j.len() as u32;
        let vals = self.vals_j.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        if let Some(hk_constraint) = &self.hk_constraint {
            info.hk_constraint_offset = infos.header.hk_constraint_info_offset + (HkConstraintInfo::size::<O>() * infos.hk_constraint.len()) as u32;
            let vals = hk_constraint.dump::<O>(offset, info.bones_offset, info.bones_num, infos);
            offset += vals.len();
            data.extend(vals);
        } else {
            info.hk_constraint_offset = 0;
        }

        let off = (offset+ 15) & 0xFFFFFFF0;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        info.skin_binds_offset = offset as u32;
        info.skin_binds_num = self.skin_binds.len() as u32;
        let vals = self.skin_binds.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        if self.skin_order.len() != 0 {
            info.skin_order_offset = offset as u32;
            let vals = self.skin_order.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
        } else {
            info.skin_order_offset = 0;
        }

        info.bone_parents_offset = offset as u32;
        let vals = self.bone_parents.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        let off = (offset+ 15) & 0xFFFFFFF0;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        info.bone_transforms_offset = offset as u32;
        let vals = self.bone_transforms.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.mat_offset = offset as u32;
        info.mat_num = mat_order.len() as u32;
        infos.block2_offsets.extend((0..mat_order.len() as u32).map(|x| offset as u32 + x * 4));
        let vals = mat_order.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.shape_num = self.shapes.len() as u32;
        info.shape_offset = if self.shapes.len() != 0 {
            infos.header.shape_info_offset + (ShapeInfo::size::<O>() * infos.shape.len()) as u32
        } else { 0 };
        for shape in &self.shapes {
            let vals = shape.dump::<O>(offset, None, infos);
            offset += vals.len();
            data.extend(vals);    
        }

        info.mesh_order_offset = offset as u32;
        //info.lod3.breakable_end = self.mesh_order.len() as u32;
        let vals = self.mesh_order.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        let off = (offset+ 15) & 0xFFFFFFF0;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        info.vals_d_offset = offset as u32;
        let vals = self.vals_d.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);    

        info.vbuff_offset = offset as u32;
        info.vbuff_num = vbuff_order.len() as u32;
        infos.block2_offsets.extend((0..vbuff_order.len() as u32).map(|x|  offset as u32 + x * 4));
        let vals = vbuff_order.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.ibuff_offset = offset as u32;
        info.ibuff_num = ibuff_order.len() as u32;
        infos.block2_offsets.extend((0..ibuff_order.len() as u32).map(|x|  offset as u32 + x * 4));
        let vals = ibuff_order.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        if self.val_k_header.len() != 0 {
            let off = (offset+ 15) & 0xFFFFFFF0;
            data.extend(vec![0u8; off-offset]);
            offset = off;

            info.vals_k_offset = offset as u32;
            let vals = self.val_k_header.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
            let vals = self.vals_k.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
        }

        if self.slots.len() != 0 {
            info.slots_offset = offset as u32;
            let vals = self.slots.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
            info.slot_map_offset = offset as u32;
            let vals = self.slot_map.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
        }

        if let Some(block_header) = self.block_header {
            let off = (offset+ 15) & 0xFFFFFFF0;
            data.extend(vec![0u8; off-offset]);
            offset = off;

            info.block_offset = offset as u32;
            let vals = block_header.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
            let vals = self.block_offsets.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);

            for (i, (header, vals_a, vals_b, extra)) in self.blocks.iter().enumerate() {
                let off = (self.block_offsets[i] + info.block_offset) as usize;
                data.extend(vec![0u8; off-offset]);
                offset = off;
                let vals = header.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);
                let vals = vals_a.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);
                let vals = vals_b.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);
                let vals = extra.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);
            }
        }

        infos.model.push(info);
        data
    }

    pub fn dump_terrain<O: Version + 'static>(&self, mut offset: usize, indices_offset: u32, infos: &mut DumpInfos) -> Vec<u8> {
        let mut info = self.info.clone();
        let mut data = vec![];

        let mut mat_order = self.mat_order.clone();
        let mut mat_map = HashMap::with_capacity(self.mats.len());
        for (i, (mat, mat_extra)) in zip(&self.mats, &self.mat_extras).enumerate() {
            let mut mat = mat.clone();
            if let Some(mat_extra) = mat_extra {
                mat.base_mut().mat_extra_offset = infos.header.mat_extra_offset + (MatExtra::size::<O>() * infos.mat_extra.len()) as u32;
                infos.mat_extra.push(mat_extra.clone())
            }
            match mat {
                Mat::Mat1(mat) => {
                    mat_map.insert(i as u32, infos.header.mat1_offset + (Mat1::size::<O>() * infos.mat1.len()) as u32);
                    infos.mat1.push(mat);
                },
                Mat::Mat2(mat) => {
                    mat_map.insert(i as u32, infos.header.mat2_offset + (Mat2::size::<O>() * infos.mat2.len()) as u32);
                    infos.mat2.push(mat);
                },
                Mat::Mat3(mat) => {
                    mat_map.insert(i as u32, infos.header.mat3_offset + (Mat3::size::<O>() * infos.mat3.len()) as u32);
                    infos.mat3.push(mat);
                },
                Mat::Mat4(mat) => {
                    mat_map.insert(i as u32, infos.header.mat4_offset + (Mat4::size::<O>() * infos.mat4.len()) as u32);
                    infos.mat4.push(mat);
                },
            }
        }
        mat_order.iter_mut().for_each(|x| *x = *mat_map.get(x).unwrap());

        let mut vbuff_map: HashMap<_, _> = (0..self.vbuffs.len()).map(|x| (x as u32, infos.header.vbuff_info_offset + (VBuffInfo::size::<O>() * (infos.vbuff.len() + x)) as u32)).collect();
        let vbuff_order: Vec<u32> = self.vbuff_order.iter().map(|x| *vbuff_map.get(x).unwrap()).collect();
        infos.vbuff.extend(self.vbuffs.clone());

        let mut ibuff_map: HashMap<_, _> = (0..self.ibuffs.len()).map(|x| (x as u32, infos.header.ibuff_info_offset + (IBuffInfo::size::<O>() * (infos.ibuff.len() + x)) as u32)).collect();
        let ibuff_order: Vec<u32> = self.ibuff_order.iter().map(|x| *ibuff_map.get(x).unwrap()).collect();
        infos.ibuff.extend(self.ibuffs.clone());

        ibuff_map.insert(0xFFFFFFFF, 0);
        vbuff_map.insert(0xFFFFFFFF, 0);
        let buffer_infos: Vec<_> = self.buffer_infos.iter().map(|buff| {
            let mut buff = buff.clone();
            buff.vbuff_info_offset = *vbuff_map.get(&buff.vbuff_info_offset).unwrap();
            buff.vbuff_info_offset_2 = *vbuff_map.get(&buff.vbuff_info_offset_2).unwrap();
            buff.vbuff_info_offset_3 = *vbuff_map.get(&buff.vbuff_info_offset_3).unwrap();
            buff.ibuff_info_offset = *ibuff_map.get(&buff.ibuff_info_offset).unwrap();
            buff
        }).collect();
        info.buffer_info_offset = infos.header.buffer_info_offset + (BufferInfo::size::<O>() * infos.buffer.len()) as u32;
        infos.buffer.extend(buffer_infos);

        info.hk_constraint_data_num = self.hk_constraint_datas.len() as u32;
        info.hk_constraint_data_offset = if self.hk_constraint_datas.len() != 0 {
            infos.header.hk_constraint_data_offset + (HkConstraintData::size::<O>() * infos.hk_constraint_data.len()) as u32
        } else { 0 };
        infos.hk_constraint_data.extend(self.hk_constraint_datas.clone());

        info.bones_offset = 0;
        info.bones_num = self.vals_a.len() as u32;

        info.vals_a_offset = infos.header.model_info_offset + (infos.model.len() * ModelInfo::size::<O>()) as u32 + 16;

        info.skin_binds_offset = indices_offset as u32;
        info.skin_binds_num = 0;
        info.skin_order_offset = 0;
        info.bone_parents_offset = indices_offset as u32;

        if let Some(hk_constraint) = &self.hk_constraint {
            info.hk_constraint_offset = infos.header.hk_constraint_info_offset + (HkConstraintInfo::size::<O>() * infos.hk_constraint.len()) as u32;
            let vals = hk_constraint.dump::<O>(offset, info.bones_offset, info.bones_num, infos);
            offset += vals.len();
            data.extend(vals);    
        } else {
            info.hk_constraint_offset = 0;
        }

        let mut shape_offsets = Vec::with_capacity(self.shapes.len());
        for shape in &self.shapes {
            if let Some(extra) = &shape.extra {
                shape_offsets.push(Some(offset as u32));
                let vals = extra.dump::<O>();
                offset += vals.len();
                data.extend(vals);
            } else {
                shape_offsets.push(None);
            }
        }

        let off = (offset+ 15) & 0xFFFFFFF0;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        info.vals_d_offset = offset as u32;
        let vals = self.vals_d.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);    

        info.bone_transforms_offset = offset as u32;
        let vals = self.bone_transforms.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.mat_offset = offset as u32;
        info.mat_num = mat_order.len() as u32;
        infos.block2_offsets.extend((0..mat_order.len() as u32).map(|x| offset as u32 + x * 4));
        let vals = mat_order.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);
        
        info.mesh_order_offset = offset as u32;
        //info.lod3.breakable_end = self.mesh_order.len() as u32;
        let vals = self.mesh_order.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.vbuff_offset = offset as u32;
        info.vbuff_num = vbuff_order.len() as u32;
        infos.block2_offsets.extend((0..vbuff_order.len() as u32).map(|x|  offset as u32 + x * 4));
        let vals = vbuff_order.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        let off_dest = offset + 320;
        info.ibuff_offset = offset as u32;
        info.ibuff_num = ibuff_order.len() as u32;
        infos.block2_offsets.extend((0..ibuff_order.len() as u32).map(|x|  offset as u32 + x * 4));
        let vals = ibuff_order.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        if self.val_k_header.len() != 0 {
            let off = (offset+ 15) & 0xFFFFFFF0;
            data.extend(vec![0u8; off-offset]);
            offset = off;

            info.vals_k_offset = offset as u32;
            let vals = self.val_k_header.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
            let vals = self.vals_k.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
        }

        if self.slots.len() != 0 {
            info.slots_offset = offset as u32;
            let vals = self.slots.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
            info.slot_map_offset = offset as u32;
            let vals = self.slot_map.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
        }

        if let Some(block_header) = self.block_header {
            let off = (offset+ 15) & 0xFFFFFFF0;
            data.extend(vec![0u8; off-offset]);
            offset = off;

            info.block_offset = offset as u32;
            let vals = block_header.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
            let vals = self.block_offsets.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);

            for (i, (header, vals_a, vals_b, extra)) in self.blocks.iter().enumerate() {
                let off = (self.block_offsets[i] + info.block_offset) as usize;
                data.extend(vec![0u8; off-offset]);
                offset = off;
                let vals = header.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);
                let vals = vals_a.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);
                let vals = vals_b.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);
                let vals = extra.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);
            }
        }

        data.extend(vec![0u8; off_dest - offset]);
        offset = off_dest;

        info.shape_num = self.shapes.len() as u32;
        info.shape_offset = if self.shapes.len() != 0 {
            infos.header.shape_info_offset + (ShapeInfo::size::<O>() * infos.shape.len()) as u32
        } else { 0 };
        for (shape, off) in zip(&self.shapes, shape_offsets) {
            let vals = shape.dump::<O>(offset, off, infos);
            offset += vals.len();
            data.extend(vals);    
        }


        infos.model.push(info);
        data
    }

    pub fn infos_count(&self) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) {
        let hk_shapes = self.shapes.iter().map(|shape| shape.hk_shapes.len()).sum::<usize>();
        let (mut mat1_num, mut mat2_num, mut mat3_num, mut mat4_num) = (0u32, 0u32, 0u32, 0u32);
        for mat in &self.mats {
            match mat {
                Mat::Mat1(_) => { mat1_num += 1; },
                Mat::Mat2(_) => { mat2_num += 1; },
                Mat::Mat3(_) => { mat3_num += 1; },
                Mat::Mat4(_) => { mat4_num += 1; },
            }
        }
        return (
            self.shapes.len() as u32,
            hk_shapes as u32,
            if self.hk_constraint.is_some() { 1u32 } else { 0 },
            self.hk_constraint_datas.len() as u32,
            mat1_num, mat2_num, mat3_num, mat4_num,
            self.mat_extras.iter().map(|x| if x.is_some() { 1 } else { 0 }).sum::<u32>(),
            self.buffer_infos.len() as u32,
            self.vbuffs.len() as u32,
            self.ibuffs.len() as u32,
        )
    }

    pub fn to_gltf(&self) -> Result<gltf::Glb> {
        let mut root = gltf::json::root::Root::default();
        let mut bin = Vec::new();

        // add index data
        let ibuffs: Vec<_> = self.index_data.iter().map(|ibuff| match ibuff {
            IndexBuffer::U16 { vals } => GltfAsset { data: vals.dump_bytes::<PC>(), count: vals.len(), ty: DataType::U16, dim: Dimensions::Scalar,  target: Some(Target::ElementArrayBuffer), ..Default::default() },
            IndexBuffer::U32 { vals } => GltfAsset { data: vals.dump_bytes::<PC>(), count: vals.len(), ty: DataType::U32, dim: Dimensions::Scalar,  target: Some(Target::ElementArrayBuffer), ..Default::default() },
        }.to_gltf(&mut root, &mut bin)).collect();

        let vbuffs: Vec<HashMap<VertexUsage, Index<Accessor>>> = self.vertex_data.iter().map(|vbuff| 
            vbuff.vals.iter().map(|val| (val.usage.clone(), GltfAsset { 
                data: val.gltf_data(),
                count: val.val.len(),
                stride: val.stride(),
                target: Some(Target::ArrayBuffer),
                ty: val.component_type(),
                dim: val.dimensions(),
                min: val.min(),
                max: val.max(),
                normalized: val.normalized(),
                extras: Some(json!(val.usage.clone())),
            }.to_gltf(&mut root, &mut bin))).collect()
        ).collect();

        // add skeleton data if relevant
        let mut children = self.bones.iter().map(|_| Vec::new()).collect::<Vec<_>>();
        for (i, &parent) in self.bone_parents.iter().enumerate() {
            if parent == -1 { continue }
            children[parent as usize].push(Index::<Node>::new((root.nodes.len() + i) as u32));
        }

        let bones: Vec<_> = self.bones.iter().zip(self.bone_transforms.iter().zip(children)).map(|(bone, (mat, children))| {
            let children = if children.is_empty() {
                None 
            } else {
                Some(children)
            };
            root.push(Node {
                name: Some(bone.to_string()),
                children,
                matrix: Some(mat.into()),
                ..Default::default()
            })
        }).collect();

        // add skin if relevant
        let skin = if self.skin_order.len() != 0 {
            let joints: Vec<_> = self.skin_order.iter().map(|&x| bones[x as usize]).collect();
            let accessor = GltfAsset { 
                data: self.skin_binds.dump_bytes::<PC>(), 
                count: joints.len(), 
                ty: DataType::F32,
                dim: Dimensions::Mat4, 
                ..Default::default()
            }.to_gltf(&mut root, &mut bin);
            let skeleton = bones.first().copied();

            Some(root.push(gltf::json::Skin {
                name: None,
                extensions: None,
                extras: Default::default(),
                inverse_bind_matrices: Some(accessor),
                joints,
                skeleton,
            }))
        } else {
            None
        };

        // add meshesand nodes

        let mut i = 0;
        let mut uses = vec![0u32; self.buffer_infos.len()];
        for (lod, f) in [&self.info.lod0, &self.info.lod1, &self.info.lod2, &self.info.lod3].iter().zip(
            [LodMeshes::LOD0, LodMeshes::LOD1, LodMeshes::LOD2, LodMeshes::LOD3]
        ) {
            for i in i..lod.start { 
                uses[(self.mesh_order[i as usize] & 0x3FFFFFFF) as usize] |= f | LodMeshes::UNKNOWN; 
            }
            for i in lod.start..lod.static_end {
                uses[(self.mesh_order[i as usize] & 0x3FFFFFFF) as usize] |= f | LodMeshes::STATIC;
            }
            for i in lod.static_end..lod.skinned_end {
                uses[(self.mesh_order[i as usize] & 0x3FFFFFFF) as usize] |= f | LodMeshes::SKINNED;
            }
            for i in lod.skinned_end..lod.physics_end {
                uses[(self.mesh_order[i as usize] & 0x3FFFFFFF) as usize] |= f | LodMeshes::PHYSICS;
            }
            for i in lod.physics_end..lod.breakable_end {
                uses[(self.mesh_order[i as usize] & 0x3FFFFFFF) as usize] |= f | LodMeshes::BREAKABLE;
            }
            i = lod.breakable_end;
        }

        let mut nodes: Vec<_> = self.buffer_infos.iter().zip(uses).enumerate().map(|(i, (info, usage))| {
            let vbuff = &vbuffs[info.vbuff_info_offset as usize];
            let indices = ibuffs.get(info.ibuff_info_offset as usize).copied();
            //let i_size = root.accessors[indices.unwrap().value()].max.as_ref().unwrap().as_u64().unwrap();
            let mut attributes = std::collections::BTreeMap::new();
            let skinned = usage & LodMeshes::SKINNED != 0;
            //let skinned = vbuff.contains_key(&VertexUsage::BlendWeight) && vbuff.contains_key(&VertexUsage::BlendIndices);
            for (k, v) in vbuff.iter() {
                match k {
                    VertexUsage::Position() => {
                        attributes.insert(Checked::Valid(gltf::mesh::Semantic::Positions), v.clone());
                    },
                    VertexUsage::Normal() => {
                        attributes.insert(Checked::Valid(gltf::mesh::Semantic::Normals), v.clone());
                    },
                    VertexUsage::TextureCoord(i) => {
                        attributes.insert(Checked::Valid(gltf::mesh::Semantic::TexCoords(*i as u32)), v.clone());
                    },
                    VertexUsage::BlendIndices() => if skinned { 
                        attributes.insert(Checked::Valid(gltf::mesh::Semantic::Joints(0)), v.clone());
                    } else {
                        attributes.insert(Checked::Valid(gltf::mesh::Semantic::Extras(k.to_string())), v.clone());
                    },
                    VertexUsage::BlendWeight() => if skinned {
                        attributes.insert(Checked::Valid(gltf::mesh::Semantic::Weights(0)), v.clone());
                    } else {
                        attributes.insert(Checked::Valid(gltf::mesh::Semantic::Extras(k.to_string())), v.clone());
                    },
                    VertexUsage::Tangent() => {
                        attributes.insert(Checked::Valid(gltf::mesh::Semantic::Tangents), v.clone());
                    },
                    _ => {
                        attributes.insert(Checked::Valid(gltf::mesh::Semantic::Extras(k.to_string())), v.clone());
                    }
                }
            }
            let skin = if skinned { skin } else { None };

            let mut name = String::new() + "(";
            if usage & LodMeshes::LOD0 != 0 { name += "Lod0, "; }
            if usage & LodMeshes::LOD1 != 0 { name += "Lod1, "; }
            if usage & LodMeshes::LOD2 != 0 { name += "Lod2, "; }
            if usage & LodMeshes::LOD3 != 0 { name += "Lod3, "; }
            name.truncate(name.len() - 2);
            name += format!(") Mesh {} [", i).as_str();
            if usage & LodMeshes::UNKNOWN != 0 { name += "Unknown, "; }
            if usage & LodMeshes::STATIC != 0 { name += "Static, "; }
            if usage & LodMeshes::SKINNED != 0 { name += "Skinned, "; }
            if usage & LodMeshes::PHYSICS != 0 { name += "Physics, "; }
            if usage & LodMeshes::BREAKABLE != 0 { name += "Breakable, "; }
            name.truncate(name.len() - 2);
            name += "]";

            let mesh = root.push(gltf::json::Mesh {
                name: None,
                extensions: None,
                extras: Default::default(),
                primitives: vec![gltf::json::mesh::Primitive {
                    attributes,
                    extensions: None,
                    extras: Default::default(),
                    indices, 
                    material: None,
                    mode: Checked::Valid(gltf::json::mesh::Mode::Triangles),
                    targets: None,
                }],
                weights: None,
            });

            let skin = if let Some(skin) = skin {
                let skin_start = info.skin_offset as usize;
                let skin_end = (info.skin_offset + info.skin_size) as usize;
                let skin = root.get(skin).unwrap();
                let joints = skin.joints[skin_start..skin_end].to_vec();
                let skeleton = skin.skeleton;
                let inverse_bind_matrices = if let Some(accessor) = skin.inverse_bind_matrices {
                    let mut accessor = root.get(accessor).unwrap().clone();
                    accessor.count = (info.skin_size as usize).into();
                    accessor.byte_offset = Some((info.skin_offset as usize * 64).into());
                    Some(root.push(accessor))
                } else {
                    None
                };

                Some(root.push(gltf::json::Skin {
                    name: None,
                    extensions: None,
                    extras: Default::default(),
                    inverse_bind_matrices,
                    joints,
                    skeleton,
                }))
            } else { None };

            root.push(Node {
                name: Some(name),
                skin,
                mesh: Some(mesh),
                ..Default::default()
            })
        }).collect();

        // construct scene
    
        if let Some(node) = bones.first() {
            nodes.push(node.clone());
        }
        
        root.push(gltf::json::Scene {
            name: None,
            extensions: None,
            extras: Default::default(),
            nodes
        });

        root.push(gltf::json::Buffer {
            name: None,
            byte_length: bin.len().into(),
            extensions: None,
            extras: Default::default(),
            uri: None,
        });
            
        root.extensions_used.push("KHR_mesh_quantization".into());
        root.extensions_required.push("KHR_mesh_quantization".into());
        root.extras = Some(serde_json::value::to_raw_value(&json!(ModelGltf {
            info: self.info.clone(),
            vals_a: self.vals_a.clone(),
            mat_order: self.mat_order.clone(),
            mesh_order: self.mesh_order.clone(),
            vals_d: self.vals_d.clone(),
            vbuff_order: self.vbuff_order.clone(),
            ibuff_order: self.ibuff_order.clone(),
            vals_j: self.vals_j.clone(),
            val_k_header: self.val_k_header.clone(),
            vals_k: self.vals_k.clone(),
            slots: self.slots.clone(),
            slot_map: self.slot_map.clone(),
            block_header: self.block_header.clone(),
            block_offsets: self.block_offsets.clone(),
            blocks: self.blocks.clone(),
            mats: self.mats.clone(),
            mat_extras: self.mat_extras.clone(),
            vbuffs: self.vbuffs.clone(),
            ibuffs: self.ibuffs.clone(),
            buffer_infos: self.buffer_infos.clone(),
            hk_constraint: self.hk_constraint.clone(),
            hk_constraint_datas: self.hk_constraint_datas.clone(),
            shapes: self.shapes.iter().map(|x| x.to_gltf(&mut root, &mut bin)).collect(),
            index_data: ibuffs,
            vertex_data: vbuffs,
            bones,
        }))?);

        // decode and reformat so that extras are properly formatted in the result
        let json = serde_json::to_vec_pretty(&serde_json::from_slice::<serde_json::Value>(root.to_vec()?.as_slice())?)?;
        let length = json.len();
        //json.extend(vec![' ' as u8; ((json.len() + 3) & 0xFFFFFFFC) - json.len()]);
    
        let header = gltf::binary::Header {
            magic: *b"GLTF",
            version: 2,
            length: (length + bin.len()) as u32,
        };

        Ok(gltf::Glb {
            header,
            bin: Some(bin.into()),
            json: json.into(),
        })
    }

    pub fn from_gltf(root: &Root, bin: &[u8]) -> Result<Self> {
        let model: ModelGltf = serde_json::from_str(root.extras.as_ref().unwrap().get())?;

        // get vertex / index data
        let index_data = model.ibuffs.iter().zip(model.index_data).map(|(info, i)| match info.format {
            0x10 => IndexBuffer::U16 { vals: GltfData::from_buffer(i, root, bin).u16().unwrap() },
            _ => IndexBuffer::U32 { vals: GltfData::from_buffer(i, root, bin).u32().unwrap() },
        }).collect();

        let mut formats = HashMap::new();
        let vertex_data = model.vbuffs.iter().zip(model.vertex_data).map(|(info, data)| {
            let fmt = formats.entry((info.fmt1, info.fmt2)).or_insert_with(|| {
                get_vertex_format::<PC>(info.fmt1, info.fmt2).0
            });

            let mut vals = fmt.clone();
            for val in &mut vals {
                let i = data.get(&val.usage).unwrap();
                val.from_gltf(GltfData::from_buffer(*i, root, bin)).unwrap();
            }
            VertexBuffer { vals }
        }).collect();

        let mut bones = Vec::with_capacity(model.bones.len());
        let mut bone_parents = vec![-1; model.bones.len()];
        let mut bone_transforms = Vec::with_capacity(model.bones.len());
        let bone_map: HashMap<Index<Node>, usize> = model.bones.iter().cloned().enumerate().map(|(x,y)| (y,x)).collect();
        for (i, node) in model.bones.into_iter().map(|x| root.get(x).unwrap()).enumerate() {
            if let Some(children) = &node.children {
                for j in children {
                    bone_parents[*bone_map.get(j).unwrap()] = i as i32;
                }
            }
            bones.push(Crc::from_string(node.name.as_ref().unwrap().as_str()));
            bone_transforms.push(node.matrix.as_ref().unwrap().into());
        }
        
        let mut skin_order = Vec::new();
        let mut skin_binds = Vec::new();
        if let Some(skin) = root.skins.first() {
            skin_order.extend(skin.joints.iter().map(|x| *bone_map.get(x).unwrap() as u32));
            skin_binds.extend(GltfData::from_buffer(skin.inverse_bind_matrices.unwrap(), root, bin).f32().unwrap().as_slice().chunks_exact(16).map(|x| x.try_into().unwrap()));
        }

        Ok(Self {
            bones,
            bone_parents,
            bone_transforms,
            skin_order,
            skin_binds,
            vertex_data,
            index_data,
            info: model.info,
            vals_a: model.vals_a,
            mat_order: model.mat_order,
            mesh_order: model.mesh_order,
            vals_d: model.vals_d,
            vbuff_order: model.vbuff_order,
            ibuff_order: model.ibuff_order,
            vals_j: model.vals_j,
            val_k_header: model.val_k_header,
            vals_k: model.vals_k,
            slots: model.slots,
            slot_map: model.slot_map,
            block_header: model.block_header,
            block_offsets: model.block_offsets,
            blocks: model.blocks,
            mats: model.mats,
            mat_extras: model.mat_extras,
            vbuffs: model.vbuffs,
            ibuffs: model.ibuffs,
            buffer_infos: model.buffer_infos,
            hk_constraint: model.hk_constraint,
            hk_constraint_datas: model.hk_constraint_datas,
            shapes: model.shapes.into_iter().map(|x| x.parse(root, bin)).collect(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModelGltf {
    pub info: ModelInfo,
    pub vals_a: Vec<ValA>,
    pub mat_order: Vec<u32>,
    pub mesh_order: Vec<u32>, // order of meshes (mapped to lod0, lod1, lod2, lod3)
    pub vals_d: Vec<ValA>, // 1 per contained mesh, stores result of some absolute position calculation?
    pub vbuff_order: Vec<u32>,
    pub ibuff_order: Vec<u32>,
    pub vals_j: Vec<u32>,
    pub val_k_header: Vec<u16>,
    pub vals_k: Vec<u32>,
    pub slots: Vec<Key2>, // attachment points
    pub slot_map: Vec<u32>, // attachment bone mapping
    pub block_header: Option<u32>, // has to do with havok cloth / hair stuff
    pub block_offsets: Vec<u32>,
    pub blocks: Vec<(model::BlockHeader, Vec<u32>, Vec<model::BlockVal>, Vec<u32>)>,
    pub mats: Vec<Mat>,
    pub mat_extras: Vec<Option<MatExtra>>,
    pub vbuffs: Vec<VBuffInfo>,
    pub ibuffs: Vec<IBuffInfo>,
    pub buffer_infos: Vec<BufferInfo>,
    pub hk_constraint: Option<HkConstraint>,
    pub hk_constraint_datas: Vec<HkConstraintData>,
    pub shapes: Vec<ShapeGltf>,
    pub index_data: Vec<Index<Accessor>>,
    pub vertex_data: Vec<HashMap<VertexUsage, Index<Accessor>>>,
    pub bones: Vec<Index<Node>>,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Key2 {
    pub key: Crc,
    pub val: u32,
}

impl IntoPy<PyObject> for Key2 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        (self.key, self.val).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Key2 {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let (key, val) = <(Crc, u32)>::extract_bound(ob)?;
        Ok(Self { key, val })
    }
}

#[basicpymethods]
#[pyclass(module="pak_alt", get_all, set_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, OrderedData, PyMethods)]
pub struct TRS {
    pub translation: Vector4,
    pub rotation: Vector4,
    pub scale: Vector4,
}

#[basicpymethods]
#[pyclass(module="pak_alt", name="HkConstraint", get_all, set_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
pub struct HkConstraint {
    pub info: HkConstraintInfo,
    pub bone_parents: Vec<i16>,
    pub bone_names: Vec<(String, u32)>, // bones ?
    pub bone_transforms: Vec<TRS>, // probably f32
    pub vals2: Vec<f32>, // probably f32
    pub bone_order: Vec<Key2>, // bones in order of increasing crc, number is index unsorted order
}

impl HkConstraint {
    pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize) -> Result<Self> {
        let info: HkConstraintInfo = OrderedData::from_bytes::<O>(&data[offset..])?;
        if info.kind != 0 { panic!("Unknown & Unhandled HkConstraint type {} at offset {}", info.kind, offset); }

        let bone_parents: Vec<i16> = OrderedDataVec::from_bytes::<O>(&data[info.bone_parents_offset as usize..], info.bone_parents_num as usize)?;
        assert!(bone_parents[0] == -1);

        
        let string_offsets: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.bone_names_offset as usize..], info.bone_names_num as usize)?;
        let mut bone_names = Vec::with_capacity(string_offsets.len());
        for offset_ in string_offsets.iter() {
            let (mut offset, val) = { 
                let vals: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[*offset_ as usize..], 2)?;
                (vals[0], vals[1])
            };
            let start = offset;
            while data[offset as usize] != 0 { offset += 1; }
            let string = String::from_utf8(data[start as usize..offset as usize].to_vec()).unwrap();
            bone_names.push((string, val));
        }
        let bone_transforms = OrderedDataVec::from_bytes::<O>(&data[info.bone_transforms_offset as usize..], info.bone_transforms_num as usize)?;
        let vals2 = OrderedDataVec::from_bytes::<O>(&data[info.vals2_offset as usize..], info.vals2_num as usize * 42)?;
        let bone_order = OrderedDataVec::from_bytes::<O>(&data[info.bone_order_offset as usize..], info.bone_order_num as usize)?;
        Ok(Self {
            info, bone_parents, bone_names, bone_transforms, vals2, bone_order
        })
    }

    pub fn dump<O: Version + 'static>(&self, mut offset: usize, bones_offset: u32, bones_num: u32, infos: &mut DumpInfos) -> Vec<u8> {
        let mut info = self.info.clone();
        info.bones_num = bones_num as u16;
        info.bones_offset = bones_offset;
        let mut data = vec![];

        info.bone_names_offset = offset as u32;
        info.bone_names_num = self.bone_names.len() as u32;
        offset += 12 * self.bone_names.len();
        let off = (offset + 15) & 0xFFFFFFF0;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        info.bone_transforms_offset = offset as u32;
        info.bone_transforms_num = self.bone_transforms.len() as u32;
        let vals = self.bone_transforms.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.bone_order_offset = offset as u32;
        info.bone_order_num = self.bone_order.len() as u16;
        let vals = self.bone_order.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.bone_parents_offset = offset as u32;
        info.bone_parents_num = self.bone_parents.len() as u32;
        let vals = self.bone_parents.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        let off: usize = (offset + 3) & 0xFFFFFFFC;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        let mut offsets = vec![];
        let mut string_offsets = vec![];
        let mut block2_off = info.bone_names_offset;
        let mut offset_off = info.bone_names_offset + 4 * self.bone_names.len() as u32;
        for (string, val) in &self.bone_names {
            let string = string.as_bytes();
            string_offsets.push([offset as u32, *val]);
            offset += string.len();
            data.extend(string);
            let off: usize = (offset + 4) & 0xFFFFFFFC;
            data.extend(vec![0u8; off-offset]);
            offset = off;

            offsets.push(offset_off);
            infos.block2_offsets.push(block2_off);
            infos.block2_offsets.push(offset_off);
            offset_off += 8;
            block2_off += 4;
        }
        let string_offsets = string_offsets.into_iter().flat_map(|x| x).collect::<Vec<_>>();

        if self.vals2.len() != 0 {
            info.vals2_offset = offset as u32;
            info.vals2_num = self.vals2.len() as u32 / 42;
            let vals = self.vals2.dump_bytes::<O>();
            data.extend(vals);
        } else {
            info.vals2_num = 0;
            info.vals2_offset = 0;
        }

        infos.hk_constraint.push(info);
        offsets.dump_bytes::<O>().into_iter().chain(string_offsets.dump_bytes::<O>()).chain(data).collect()
    }
}

#[basicpymethods]
#[pyclass(module="pak_alt", get_all, set_all)]
#[derive(Debug, Clone, Serialize, Deserialize, PyMethods)]
pub enum Mat {
    Mat1(Mat1),
    Mat2(Mat2),
    Mat3(Mat3),
    Mat4(Mat4),
}

impl Mat {
    pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize) -> Result<Self> {
        let ty: u32 = if TypeId::of::<O>() == TypeId::of::<PS3>() {
            OrderedData::from_bytes::<O>(&data[offset + 200..])?
        } else {
            OrderedData::from_bytes::<O>(&data[offset + 208..])?
        };
        Ok(match ty {
            0 => Self::Mat1(OrderedData::from_bytes::<O>(&data[offset..])?),
            1 => Self::Mat4(OrderedData::from_bytes::<O>(&data[offset..])?),
            2 => Self::Mat2(OrderedData::from_bytes::<O>(&data[offset..])?),
            3 => Self::Mat3(OrderedData::from_bytes::<O>(&data[offset..])?),
            _ => panic!("Unknown Mat Type {} at offset {}", ty, offset)
        })
    }

    pub fn base(&self) -> &MatBase {
        match self {
            Self::Mat1(mat) => &mat.base,
            Self::Mat2(mat) => &mat.base,
            Self::Mat3(mat) => &mat.base,
            Self::Mat4(mat) => &mat.base,
        }
    }
    
    pub fn base_mut(&mut self) -> &mut MatBase {
        match self {
            Self::Mat1(mat) => &mut mat.base,
            Self::Mat2(mat) => &mut mat.base,
            Self::Mat3(mat) => &mut mat.base,
            Self::Mat4(mat) => &mut mat.base,
        }
    }
}

#[basicpymethods]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
#[pyclass(module="pak_alt", name="Animation", get_all, set_all)]
pub struct Animation {
    pub info: AnimationInfo,
    pub obj1: Vec<u32>,
    pub obj2: Vec<u32>,
    pub obj3: Vec<animation::Obj3>,
    pub keys: Vec<Crc>,
    pub obj5_header: Option<animation::Obj5Header>,
    pub obj5_a: Vec<u32>,
    pub obj5_b: Vec<u32>,
    pub obj_c: Option<animation::HkaSplineSkeletalAnimation>,
}

impl Animation {
    pub fn from_data<O: Version + 'static>(info: AnimationInfo, offsets: &mut Vec<usize>, blocks: & Vec<Vec<u8>>) -> Result<Self> {
        let (_, (offset, block)) = zip(offsets.iter().cloned(), blocks.iter()).enumerate().find(|(i, _)| {
            info.gamemodemask & (1 << i) != 0
        }).unwrap();
        let obj1 = OrderedDataVec::from_bytes::<O>(&block[offset + info.obj1_offset as usize..], info.obj1_num as usize * 2)?;
        let obj2 = OrderedDataVec::from_bytes::<O>(&block[offset + info.obj2_offset as usize..], info.obj2_num as usize * 4)?;
        let obj3 = OrderedDataVec::from_bytes::<O>(&block[offset + info.obj3_offset as usize..], info.obj3_num as usize)?;
        let keys = OrderedDataVec::from_bytes::<O>(&block[offset + info.keys_offset as usize..], (info.keys_num + info.obj1_num) as usize)?;
        let (obj5_header, obj5_a, obj5_b) = if info.obj5_offset != 0 {
            let obj5_header: animation::Obj5Header = OrderedData::from_bytes::<O>(&block[offset + info.obj5_offset as usize..])?;
            let obj5_a = OrderedDataVec::from_bytes::<O>(&block[offset + obj5_header.obj_a_offset as usize..], obj5_header.obj_a_num as usize * 7)?;
            let obj5_b = OrderedDataVec::from_bytes::<O>(&block[offset + obj5_header.obj_b_offset as usize..], obj5_header.obj_b_num as usize * 7)?;
            (Some(obj5_header), obj5_a, obj5_b)
        } else {(
            None, vec![], vec![]
        )};
        let obj_c = if info.kind == 3 {
            Some(animation::HkaSplineSkeletalAnimation::from_data::<O>(&block[..], offset, &info)?)
        } else if info.kind < 3 {
            warn!("Unhandled animation type {} at offset {}", info.kind, offset);
            None
        } else {
            warn!("Unknown animation type {} at offset {}", info.kind, offset);
            None
        };
        offsets.iter_mut().enumerate().filter(|(i, _)| info.gamemodemask & (1 << i) != 0).for_each(|(_, x)| *x += info.size as usize );
        Ok(Self { info, obj1, obj2, obj3, keys, obj5_a, obj5_b, obj5_header, obj_c })
    }

    pub fn dump<O: Version + 'static>(&self, offset: usize, infos: &mut DumpInfos) -> Result<Vec<u8>> {
        let mut info = self.info.clone();
        info.offset = offset as u32;
        let mut data = vec![0u8; info.size as usize];
        self.obj1.to_bytes::<O>(&mut data[info.obj1_offset as usize..])?;
        self.obj2.to_bytes::<O>(&mut data[info.obj2_offset as usize..])?;
        self.obj3.to_bytes::<O>(&mut data[info.obj3_offset as usize..])?;
        self.keys.to_bytes::<O>(&mut data[info.keys_offset as usize..])?;
        if let Some(obj5_header) = &self.obj5_header {
            obj5_header.to_bytes::<O>(&mut data[info.obj5_offset as usize..])?;
            self.obj5_a.to_bytes::<O>(&mut data[obj5_header.obj_a_offset as usize..])?;
            self.obj5_b.to_bytes::<O>(&mut data[obj5_header.obj_b_offset as usize..])?;
        }
        if let Some(obj_c) = &self.obj_c {
            obj_c.into_data::<O>(&mut data, 0, &info)?;
        }
        infos.animation.push(info);
        Ok(data)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DumpInfos {
    pub header: Header,
    pub animation: Vec<AnimationInfo>,
    pub hk_shape: Vec<HkShapeInfo>,
    pub shape: Vec<ShapeInfo>,
    pub model: Vec<ModelInfo>,
    pub mat1: Vec<Mat1>,
    pub mat2: Vec<Mat2>,
    pub mat3: Vec<Mat3>,
    pub mat4: Vec<Mat4>,
    pub mat_extra: Vec<MatExtra>,
    pub hk_constraint: Vec<HkConstraintInfo>,
    pub hk_constraint_data: Vec<HkConstraintData>,
    pub vbuff: Vec<VBuffInfo>,
    pub ibuff: Vec<IBuffInfo>,
    pub buffer: Vec<BufferInfo>,
    pub block2_offsets: Vec<u32>,
}

pub struct GltfAsset {
    pub data: Vec<u8>,
    pub count: usize,
    pub stride: Option<usize>,
    pub target: Option<Target>,
    pub ty: DataType,
    pub dim: Dimensions,
    pub min: Option<Value>,
    pub max: Option<Value>,
    pub normalized: bool,
    pub extras: Option<Value>,
}

impl Default for GltfAsset {
    fn default() -> Self {
        Self {
            data: vec![],
            count: 0,
            stride: None,
            target: None,
            ty: DataType::U32,
            dim: Dimensions::Scalar,
            min: None,
            max: None,
            normalized: false,
            extras: None,
        }
    }
}

impl GltfAsset {
    pub fn to_gltf(self, root: &mut Root, bin: &mut Vec<u8>) -> Index<Accessor> {
        assert!(self.data.len() % self.count == 0);
        let s = match self.ty {
            DataType::I8 | DataType::U8 => 1,
            DataType::I16 | DataType::U16 => 2,
            _ => 4
        };
        bin.extend(vec![0u8; ((bin.len() + s - 1) & (0xFFFFFFFF - s + 1)) - bin.len()]);

        let buffer_view = root.push(View {
            name: None,
            buffer: Index::<Buffer>::new(0),
            byte_length: self.data.len().into(),
            byte_offset: Some(bin.len().into()),
            byte_stride: self.stride.map(|x| Stride(x)),
            target: self.target.map(|x| Checked::Valid(x)),
            extensions: None,
            extras: Default::default(),
        });
        bin.extend(self.data);

        root.push(gltf::json::Accessor {
            name: None,
            buffer_view: Some(buffer_view),
            byte_offset: None,
            count: self.count.into(),
            component_type: Checked::Valid(GenericComponentType(self.ty)),
            extensions: None,
            extras: self.extras.map(|x| serde_json::value::to_raw_value(&x).unwrap()),
            type_: Checked::Valid(self.dim),
            min: self.min,
            max: self.max,
            normalized: self.normalized,
            sparse: None,
        })
    }
}

pub struct GltfData<'a> {
    pub ty: DataType,
    pub m: usize,
    pub n: usize,
    pub stride: usize,
    pub bin: &'a [u8],
}

impl <'a> GltfData<'a> {
    pub fn from_buffer(i: Index<Accessor>, root: &Root, bin: &'a [u8]) -> Self {
        let accessor = root.get(i).unwrap();
        let buffer_view = root.get(accessor.buffer_view.unwrap()).unwrap();

        let ty = accessor.component_type.unwrap().0;
        let dim = accessor.type_.unwrap();
        let n = accessor.count.0 as usize;
        let m = dim.multiplicity();
        let stride = buffer_view.byte_stride.map(|x| x.0).unwrap_or_else(|| ty.size() * m);
        let off = (buffer_view.byte_offset.map(|x| x.0).unwrap_or(0) + accessor.byte_offset.map(|x| x.0).unwrap_or(0)) as usize;
        Self {
            ty, m, n, stride, bin: &bin[off..]
        }
    }

    pub fn u8(self) -> Result<Vec<u8>> {
        match self.ty {
            DataType::U8 => (0..self.n).map(|x| Vec::<u8>::from_bytes::<PC>(&self.bin[x * self.stride..], self.m)).flatten_ok().collect::<Result<Vec<_>>>(),
            _ => Result::Err(anyhow!("Invalid Data Type {:?} for U8", self.ty)) 
        }
    }

    #[allow(dead_code)]
    pub fn i8(self) -> Result<Vec<i8>> {
        match self.ty {
            DataType::I8 => (0..self.n).map(|x| Vec::<i8>::from_bytes::<PC>(&self.bin[x * self.stride..], self.m)).flatten_ok().collect::<Result<Vec<_>>>(),
            _ => Result::Err(anyhow!("Invalid Data Type {:?} for I8", self.ty)) 
        }
    }

    pub fn u16(self) -> Result<Vec<u16>> {
        match self.ty {
            DataType::U16 => (0..self.n).map(|x| Vec::<u16>::from_bytes::<PC>(&self.bin[x * self.stride..], self.m)).flatten_ok().collect::<Result<Vec<_>>>(),
            _ => Result::Err(anyhow!("Invalid Data Type {:?} for U16", self.ty)) 
        }
    }

    pub fn i16(self) -> Result<Vec<i16>> {
        match self.ty {
            DataType::I16 => (0..self.n).map(|x| Vec::<i16>::from_bytes::<PC>(&self.bin[x * self.stride..], self.m)).flatten_ok().collect::<Result<Vec<_>>>(),
            _ => Result::Err(anyhow!("Invalid Data Type {:?} for I16", self.ty)) 
        }
    }

    pub fn u32(self) -> Result<Vec<u32>> {
        match self.ty {
            DataType::U32 => (0..self.n).map(|x| Vec::<u32>::from_bytes::<PC>(&self.bin[x * self.stride..], self.m)).flatten_ok().collect::<Result<Vec<_>>>(),
            _ => Result::Err(anyhow!("Invalid Data Type {:?} for U32", self.ty)) 
        }
    }

    pub fn f32(self) -> Result<Vec<f32>> {
        match self.ty {
            DataType::F32 => (0..self.n).map(|x| Vec::<f32>::from_bytes::<PC>(&self.bin[x * self.stride..], self.m)).flatten_ok().collect::<Result<Vec<_>>>(),
            _ => Result::Err(anyhow!("Invalid Data Type {:?} for F32", self.ty)) 
        }
    }
}
