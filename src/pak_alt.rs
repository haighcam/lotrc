use std::{collections::{HashMap, HashSet}, iter::zip};
use itertools::Itertools;
use log::warn;
use zerocopy::ByteOrder;
use serde::{Serialize, Deserialize};
use crate::types::Crc;

use lotrc_rs_proc::OrderedData;
use super::types::{OrderedData, Vector4, Matrix4x4, OrderedDataVec};
use super::pak::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    pub info: MeshInfo,
    pub indices: Vec<u32>,
    pub keys: Vec<Crc>,
    pub matrices: Vec<Matrix4x4>,
    pub vals_a: Vec<u32>,
    pub mat_order: Vec<u32>,
    pub vals_c: Vec<u32>,
    pub vals_d: Vec<u32>,
    pub vbuff_order: Vec<u32>,
    pub ibuff_order: Vec<u32>,
    pub vals_g: Vec<u32>,
    pub vals_j: Vec<u32>,
    pub val_k_header: Vec<u16>,
    pub vals_k: Vec<u32>,
    pub vals_i: Vec<u32>,
    pub keys2: Vec<Key2>,
    pub keys2_order: Vec<u32>,
    pub block_header: Option<u32>,
    pub block_offsets: Vec<u32>,
    pub blocks: Vec<(mesh::BlockHeader, Vec<u32>, Vec<mesh::BlockVal>, Vec<u32>)>,
    pub mats: Vec<Mat>,
    pub mat_extras: Vec<Option<MatExtra>>,
    pub vbuffs: Vec<VBuffInfo>,
    pub ibuffs: Vec<IBuffInfo>,
    pub buffer_infos: Vec<BufferInfo>,
    pub hk_constraint: Option<HkConstraint>,
    pub hk_constraint_datas: Vec<HkConstraintData>,
    pub shapes: Vec<Shape>,
    pub vertex_data: Vec<VertexBuffer>,
    pub index_data: Vec<IndexBuffer>,
}

impl Mesh {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize) -> Self {
        let info: MeshInfo = OrderedData::from_bytes::<O>(&data[offset..]);
        let indices: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.indices_offset as usize..], info.keys_num as usize);
        let keys: Vec<Crc> = if info.keys_offset != 0 {
            OrderedDataVec::from_bytes::<O>(&data[info.keys_offset as usize..], info.keys_num as usize)
        } else { Vec::new() };
        let matrices: Vec<Matrix4x4> = OrderedDataVec::from_bytes::<O>(&data[info.matrices_offset as usize..], info.keys_num as usize);
        let vals_a: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.vals_a_offset as usize..], info.keys_num as usize * 8);
        let mut mat_order: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.mat_offset as usize..], info.mat_num as usize);
        let vals_c: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.vals_c_offset as usize..], info.vals_c_num as usize);
        let vals_d: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.vals_d_offset as usize..], info.vals_c_num as usize * 8);
        let mut vbuff_order: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.vbuff_offset as usize..], info.vbuff_num as usize);
        let mut ibuff_order: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.ibuff_offset as usize..], info.ibuff_num as usize);
        let vals_g: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.vals_g_offset as usize..], info.vals_g_num as usize * 16);
        let vals_j: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.vals_j_offset as usize..], info.vals_j_num as usize);
        let (val_k_header, vals_k) = if info.vals_k_offset != 0 {(
            OrderedDataVec::from_bytes::<O>(&data[info.vals_k_offset as usize..], 2), 
            OrderedDataVec::from_bytes::<O>(&data[info.vals_k_offset as usize + 4..], 35)
        )} else {(
            Vec::new(), Vec::new()
        )};
        let vals_i = if info.vals_i_offset != 0 {
            OrderedDataVec::from_bytes::<O>(&data[info.vals_i_offset as usize..], info.vals_g_num as usize)
        } else {
            Vec::new()
        };
        let (keys2, keys2_order) = if info.keys2_offset != 0 {
            assert!(info.keys2_order_offset != 0);
            let mut i = 0;
            {
                while u32::from_bytes::<O>(&data[info.keys2_offset as usize + i * 8..]) != 0 {
                    i += 1;
                }
                i += 1;
            }
            let keys2: Vec<Key2> = OrderedDataVec::from_bytes::<O>(&data[info.keys2_offset as usize..], i);
            let keys2_order = OrderedDataVec::from_bytes::<O>(&data[info.keys2_order_offset as usize..], keys2.last().unwrap().val as usize);
            (keys2, keys2_order)
        } else {(
            Vec::new(), Vec::new()
        )};
        let (block_header, block_offsets, blocks) = if info.block_offset != 0 {
            let block_header = OrderedData::from_bytes::<O>(&data[info.block_offset as usize..]);
            let n = (info.block_end - info.block_start) as usize;
            let block_offsets: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.block_offset as usize + 4..], n+1);
            let mut blocks = Vec::with_capacity(n);
            for i in 0..n {
                let size = (block_offsets[i+1] - block_offsets[i]) as usize;
                let offset = (block_offsets[i] + info.block_offset) as usize;
                let header: mesh::BlockHeader = OrderedData::from_bytes::<O>(&data[offset..]);
                let mut s = mesh::BlockHeader::size::<O>();
                let vals_a: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (header.a + header.b) as usize * 12);
                s += vals_a.size::<O>();
                let vals_b: Vec<mesh::BlockVal> = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (size - s)/mesh::BlockVal::size::<O>());
                s += vals_b.size::<O>();
                let extra = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (size - s)/4);
                blocks.push((header, vals_a, vals_b, extra));
            }
            (Some(block_header), block_offsets, blocks)
        } else {(
            None, Vec::new(), Vec::new()
        )};

        assert!(indices[0] == 0xffffffff);

        let shapes = (0..info.shape_num as usize).map(|i| 
            Shape::from_data::<O>(data, info.shape_offset as usize + i * ShapeInfo::size::<O>())
        ).collect();

        let hk_constraint = (info.hk_constraint_offset != 0).then(|| HkConstraint::from_data::<O>(data, info.hk_constraint_offset as usize));
        let hk_constraint_datas: Vec<HkConstraintData> = OrderedDataVec::from_bytes::<O>(&data[info.hk_constraint_data_offset as usize..], info.hk_constraint_data_num as usize);
        
        let mats = HashSet::<u32>::from_iter(mat_order.iter().cloned()).into_iter().sorted().collect::<Vec<_>>();
        let mat_map: HashMap<_, _> = mats.iter().enumerate().map(|(i, x)| (*x, i as u32)).collect();
        mat_order.iter_mut().for_each(|x| *x = *mat_map.get(x).unwrap());
        let mats: Vec<Mat> = mats.into_iter().map(|off| Mat::from_data::<O>(data, off as usize)).collect();
        let mat_extras: Vec<_> = mats.iter().map(|x| 
            (x.base().mat_extra_offset != 0).then(|| OrderedData::from_bytes::<O>(&data[x.base().mat_extra_offset as usize..]))
        ).collect();

        let vbuffs = HashSet::<u32>::from_iter(vbuff_order.iter().cloned()).into_iter().sorted().collect::<Vec<_>>();
        let mut vbuff_map: HashMap<_, _> = vbuffs.iter().enumerate().map(|(i, x)| (*x, i as u32)).collect();
        vbuff_order.iter_mut().for_each(|x| *x = *vbuff_map.get(x).unwrap());
        let vbuffs: Vec<VBuffInfo> = vbuffs.into_iter().map(|off| OrderedData::from_bytes::<O>(&data[off as usize..])).collect();
        let vertex_data = Vec::with_capacity(vbuffs.len());

        let ibuffs = HashSet::<u32>::from_iter(ibuff_order.iter().cloned()).into_iter().sorted().collect::<Vec<_>>();
        let mut ibuff_map: HashMap<_, _> = ibuffs.iter().enumerate().map(|(i, x)| (*x, i as u32)).collect();
        ibuff_order.iter_mut().for_each(|x| *x = *ibuff_map.get(x).unwrap());
        let ibuffs: Vec<IBuffInfo> = ibuffs.into_iter().map(|off| OrderedData::from_bytes::<O>(&data[off as usize..])).collect();
        let index_data = Vec::with_capacity(ibuffs.len());

        ibuff_map.insert(0, 0xFFFFFFFF);
        vbuff_map.insert(0, 0xFFFFFFFF);
        let mut buffer_infos: Vec<BufferInfo> = OrderedDataVec::from_bytes::<O>(&data[info.buffer_info_offset as usize..], info.mat_num as usize);
        buffer_infos.iter_mut().for_each(|buff| {
            buff.vbuff_info_offset = *vbuff_map.get(&buff.vbuff_info_offset).unwrap();
            buff.vbuff_info_offset_2 = *vbuff_map.get(&buff.vbuff_info_offset_2).unwrap();
            buff.vbuff_info_offset_3 = *vbuff_map.get(&buff.vbuff_info_offset_3).unwrap();
            buff.ibuff_info_offset = *ibuff_map.get(&buff.ibuff_info_offset).unwrap();
        });

        Self {
            info,
            indices,
            keys,
            matrices,
            mat_order,
            vals_a,
            vals_c,
            vals_d,
            vbuff_order,
            ibuff_order,
            vals_g,
            vals_j,
            val_k_header,
            vals_k,
            vals_i,
            keys2,
            keys2_order,
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
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self, mut offset: usize, infos: &mut DumpInfos) -> Vec<u8> {
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
                    mat_map.insert(i as u32, infos.header.mat1_offset + (MatBase::size::<O>() * infos.mat1.len()) as u32);
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

        info.keys_offset = offset as u32;
        info.keys_num = self.keys.len() as u32;
        let vals = self.keys.dump_bytes::<O>();
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
            let vals = hk_constraint.dump::<O>(offset, info.keys_offset, info.keys_num, infos);
            offset += vals.len();
            data.extend(vals);
        } else {
            info.hk_constraint_offset = 0;
        }

        let off = (offset+ 15) & 0xFFFFFFF0;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        info.vals_g_offset = offset as u32;
        info.vals_g_num = self.vals_g.len() as u32 / 16;
        let vals = self.vals_g.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        if self.vals_i.len() != 0 {
            info.vals_i_offset = offset as u32;
            let vals = self.vals_i.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
        } else {
            info.vals_i_offset = 0;
        }

        info.indices_offset = offset as u32;
        let vals = self.indices.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        let off = (offset+ 15) & 0xFFFFFFF0;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        info.matrices_offset = offset as u32;
        let vals = self.matrices.dump_bytes::<O>();
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

        info.vals_c_offset = offset as u32;
        info.vals_c_num = self.vals_c.len() as u32;
        let vals = self.vals_c.dump_bytes::<O>();
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

        if self.keys2.len() != 0 {
            info.keys2_offset = offset as u32;
            let vals = self.keys2.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
            info.keys2_order_offset = offset as u32;
            let vals = self.keys2_order.dump_bytes::<O>();
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

        infos.mesh.push(info);
        data
    }

    pub fn dump_terrain<O: ByteOrder + 'static>(&self, mut offset: usize, indices_offset: u32, infos: &mut DumpInfos) -> Vec<u8> {
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
                    mat_map.insert(i as u32, infos.header.mat1_offset + (MatBase::size::<O>() * infos.mat1.len()) as u32);
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

        info.keys_offset = 0;
        info.keys_num = self.vals_a.len() as u32 / 8;

        info.vals_a_offset = infos.header.mesh_info_offset + (infos.mesh.len() * MeshInfo::size::<O>()) as u32 + 16;

        info.vals_g_offset = indices_offset as u32;
        info.vals_g_num = 0;
        info.vals_i_offset = 0;
        info.indices_offset = indices_offset as u32;

        if let Some(hk_constraint) = &self.hk_constraint {
            info.hk_constraint_offset = infos.header.hk_constraint_info_offset + (HkConstraintInfo::size::<O>() * infos.hk_constraint.len()) as u32;
            let vals = hk_constraint.dump::<O>(offset, info.keys_offset, info.keys_num, infos);
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

        info.matrices_offset = offset as u32;
        let vals = self.matrices.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.mat_offset = offset as u32;
        info.mat_num = mat_order.len() as u32;
        infos.block2_offsets.extend((0..mat_order.len() as u32).map(|x| offset as u32 + x * 4));
        let vals = mat_order.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);
        
        info.vals_c_offset = offset as u32;
        info.vals_c_num = self.vals_c.len() as u32;
        let vals = self.vals_c.dump_bytes::<O>();
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

        if self.keys2.len() != 0 {
            info.keys2_offset = offset as u32;
            let vals = self.keys2.dump_bytes::<O>();
            offset += vals.len();
            data.extend(vals);
            info.keys2_order_offset = offset as u32;
            let vals = self.keys2_order.dump_bytes::<O>();
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


        infos.mesh.push(info);
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
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Key2 {
    pub key: Crc,
    pub val: u32,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HkConstraint {
    pub info: HkConstraintInfo,
    pub shorts: Vec<u16>,
    pub strings: Vec<(String, u32)>,
    pub string_offsets: Vec<u32>,
    pub vals: Vec<u32>,
    pub vals2: Vec<u32>,
    pub keys: Vec<Key2>,
}

impl HkConstraint {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize) -> Self {
        let info: HkConstraintInfo = OrderedData::from_bytes::<O>(&data[offset..]);
        if info.kind != 0 { panic!("Unknown & Unhandled HkConstraint type {} at offset {}", info.kind, offset); }

        let shorts: Vec<u16> = OrderedDataVec::from_bytes::<O>(&data[info.shorts_offset as usize..], info.shorts_num as usize);
        assert!(shorts[0] == 0xFFFF);

        
        let string_offsets: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[info.strings_offset as usize..], info.strings_num as usize);
        let mut strings = Vec::with_capacity(string_offsets.len());
        for offset_ in string_offsets.iter() {
            let (mut offset, val) = { 
                let vals: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[*offset_ as usize..], 2);
                (vals[0], vals[1]) 
            };
            let start = offset;
            while data[offset as usize] != 0 { offset += 1; }
            let string = String::from_utf8(data[start as usize..offset as usize].to_vec()).unwrap();
            strings.push((string, val));
        }
        let vals = OrderedDataVec::from_bytes::<O>(&data[info.vals_offset as usize..], info.vals_num as usize * 12);
        let vals2 = OrderedDataVec::from_bytes::<O>(&data[info.vals2_offset as usize..], info.vals2_num as usize * 42);
        let keys = OrderedDataVec::from_bytes::<O>(&data[info.keys2_offset as usize..], info.keys2_num as usize);
        Self {
            info, shorts, strings, string_offsets, vals, vals2, keys
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self, mut offset: usize, keys_offset: u32, keys_num: u32, infos: &mut DumpInfos) -> Vec<u8> {
        let mut info = self.info.clone();
        info.keys_num = keys_num as u16;
        info.keys_offset = keys_offset;
        let mut data = vec![];

        info.strings_offset = offset as u32;
        info.strings_num = self.strings.len() as u32;
        offset += 12 * self.strings.len();
        let off = (offset + 15) & 0xFFFFFFF0;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        info.vals_offset = offset as u32;
        info.vals_num = self.vals.len() as u32 / 12;
        let vals = self.vals.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.keys2_offset = offset as u32;
        info.keys2_num = self.keys.len() as u16;
        let vals = self.keys.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        info.shorts_offset = offset as u32;
        info.shorts_num = self.shorts.len() as u32;
        let vals = self.shorts.dump_bytes::<O>();
        offset += vals.len();
        data.extend(vals);

        let off: usize = (offset + 3) & 0xFFFFFFFC;
        data.extend(vec![0u8; off-offset]);
        offset = off;

        let mut offsets = vec![];
        let mut string_offsets = vec![];
        let mut block2_off = info.strings_offset;
        let mut offset_off = info.strings_offset + 4 * self.strings.len() as u32;
        for (string, val) in &self.strings {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mat {
    Mat1(MatBase),
    Mat2(Mat2),
    Mat3(Mat3),
    Mat4(Mat4),
}

impl Mat {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize) -> Self {
        let ty: u32 = OrderedData::from_bytes::<O>(&data[offset + 208..]);
        match ty {
            0 => Self::Mat1(OrderedData::from_bytes::<O>(&data[offset..])),
            1 => Self::Mat4(OrderedData::from_bytes::<O>(&data[offset..])),
            2 => Self::Mat2(OrderedData::from_bytes::<O>(&data[offset..])),
            3 => Self::Mat3(OrderedData::from_bytes::<O>(&data[offset..])),
            _ => panic!("Unknown Mat Type {} at offset {}", ty, offset)
        }
    }

    pub fn base(&self) -> &MatBase {
        match self {
            Self::Mat1(mat) => &mat,
            Self::Mat2(mat) => &mat.base,
            Self::Mat3(mat) => &mat.base,
            Self::Mat4(mat) => &mat.base,
        }
    }
    
    pub fn base_mut(&mut self) -> &mut MatBase {
        match self {
            Self::Mat1(mat) => mat,
            Self::Mat2(mat) => &mut mat.base,
            Self::Mat3(mat) => &mut mat.base,
            Self::Mat4(mat) => &mut mat.base,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    info: ShapeInfo,
    extra: Option<ShapeExtra>,
    hk_shapes: Vec<HkShape>,
}

impl Shape {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize) -> Self {
        let info: ShapeInfo = OrderedData::from_bytes::<O>(&data[offset..]);
        let extra = (info.kind == 0).then(|| ShapeExtra::from_data::<O>(data, info.offset as usize));
        let hk_shapes = (0..info.hk_shape_num as usize).map(|i| 
            HkShape::from_data::<O>(data, info.hk_shape_offset as usize  + i * HkShape0::size::<O>())
        ).collect();
        Self {
            info,
            extra,
            hk_shapes,
        }
    }
    pub fn dump<O: ByteOrder + 'static>(&self, mut offset: usize, extra_offset: Option<u32>, infos: &mut DumpInfos) -> Vec<u8> {
        let mut info = self.info.clone();
        if let Some(extra_offset) = extra_offset {
            info.offset = extra_offset;
            infos.block2_offsets.push(infos.header.shape_info_offset + (infos.shape.len() * ShapeInfo::size::<O>()) as u32);
        }
        info.hk_shape_offset = infos.header.hk_shape_info_offset + (infos.hk_shape.len() * HkShape0::size::<O>()) as u32;
        let mut data = vec![];
        for hk_shape in &self.hk_shapes {
            let vals = hk_shape.dump::<O>(offset, infos);
            offset += vals.len();
            data.extend(vals);
        }

        infos.shape.push(info);
        data
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
struct ShapeExtraInfo {
    pub num: u32,
    pub unk_1: f32,
    pub unk_2: u32,
    pub unk_3: f32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ShapeExtra {
    info: ShapeExtraInfo,
    offs: Vec<u32>,
    data: Vec<u8>,
}

impl ShapeExtra {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], mut offset: usize) -> Self {
        let info: ShapeExtraInfo = OrderedData::from_bytes::<O>(&data[offset..]);
        offset += ShapeExtraInfo::size::<O>();
        let offs: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[offset..], info.num as usize);
        offset += offs.size::<O>();
        let mut off = offset + *offs.last().unwrap() as usize;
        while (data[off] != 0) || (data[off+1] != 0) { off += 1; }
        Self { info, offs, data: data[offset..off].to_vec() }
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        self.info.dump_bytes::<O>().into_iter().chain(self.offs.dump_bytes::<O>()).chain(self.data.clone()).collect()
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct HkShape0 {
    pub unk_0: Vector4,
    pub unk_4: Vector4,
    pub kind: u32,
    pub unk_9: u32,
    pub a_num: u32,
    pub a_offset: u32,
    pub b_num: u32,
    pub b_offset: u32,
    pub c_num: u32,
    pub c_offset: u32,
    pub d_num: u32,
    pub d_offset: u32,
    pub e_num: u32,
    pub e_offset: u32,
}
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct HkShape1 {
    pub unk_0: Vector4,
    pub unk_4: Vector4,
    pub kind: u32,
    pub unk_9: u32,
    pub a_num: u32,
    pub a_offset: u32,
    pub b_num: u32,
    pub b_offset: u32,
    pub c_num: u32,
    pub c_offset: u32,
    pub d_num: u32,
    pub d_offset: u32,
    pub e_num: u32,
    pub e_offset: u32,
}
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct HkShape2 {
    pub unk_0: Vector4,
    pub unk_4: Vector4,
    pub kind: u32,
    pub unk_9: u32,
    pub a_num: u32,
    pub a_offset: u32,
    pub b_num: u32,
    pub b_offset: u32,
    pub c_num: u32,
    pub c_offset: u32,
    pub d_num: u32,
    pub d_offset: u32,
    pub e_num: u32,
    pub e_offset: u32,
}
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct HkShape3 {
    pub unk_0: Vector4,
    pub unk_4: Vector4,
    pub kind: u32,
    pub unk_9: u32,
    pub a_num: u32,
    pub a_offset: u32,
    pub b_num: u32,
    pub b_offset: u32,
    pub c_num: u32,
    pub c_offset: u32,
    pub d_num: u32,
    pub d_offset: u32,
    pub e_num: u32,
    pub e_offset: u32,
}
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct HkShape4 {
    pub unk_0: Vector4,
    pub unk_4: Vector4,
    pub kind: u32,
    pub unk_9: u32,
    pub a_num: u32,
    pub a_offset: u32,
    pub b_num: u32,
    pub b_offset: u32,
    pub c_num: u32,
    pub c_offset: u32,
    pub d_num: u32,
    pub d_offset: u32,
    pub e_num: u32,
    pub e_offset: u32,
}
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct HkShape5 {
    pub unk_0: Vector4,
    pub unk_4: Vector4,
    pub kind: u32,
    pub unk_9: u32,
    pub a_num: u32,
    pub a_offset: u32,
    pub b_num: u32,
    pub b_offset: u32,
    pub c_num: u32,
    pub c_offset: u32,
    pub d_num: u32,
    pub d_offset: u32,
    pub e_num: u32,
    pub e_offset: u32,
}
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct HkShape6 {
    pub unk_0: Vector4,
    pub unk_4: Vector4,
    pub kind: u32,
    pub unk_9: u32,
    pub a_num: u32,
    pub a_offset: u32,
    pub b_num: u32,
    pub b_offset: u32,
    pub c_num: u32,
    pub c_offset: u32,
    pub d_num: u32,
    pub d_offset: u32,
    pub e_num: u32,
    pub e_offset: u32,
}

use serde_with::serde_as;
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HkShape {
    HkShape0(HkShape0),
    HkShape1(HkShape1),
    HkShape2(HkShape2),
    HkShape3(HkShape3),
    HkShape4(HkShape4),
    HkShape5 {
        info: HkShape5,
        a: Vec<u32>,
        b: Vec<u32>,
        b_extra: usize,
    },
    HkShape6 {
        info: HkShape6,
        #[serde_as(as = "serde_with::hex::Hex")]
        c: Vec<u8>,
        d: Vec<u32>,
        e: Vec<u16>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HkShapeInfo {
    HkShape0(HkShape0),
    HkShape1(HkShape1),
    HkShape2(HkShape2),
    HkShape3(HkShape3),
    HkShape4(HkShape4),
    HkShape5(HkShape5),
    HkShape6(HkShape6)
}

impl HkShapeInfo {
    pub fn to_bytes<O: ByteOrder + 'static>(&self, data: &mut [u8]) {
        match self {
            Self::HkShape0(val) => val.to_bytes::<O>(data),
            Self::HkShape1(val) => val.to_bytes::<O>(data),
            Self::HkShape2(val) => val.to_bytes::<O>(data),
            Self::HkShape3(val) => val.to_bytes::<O>(data),
            Self::HkShape4(val) => val.to_bytes::<O>(data),
            Self::HkShape5(val) => val.to_bytes::<O>(data),
            Self::HkShape6(val) => val.to_bytes::<O>(data),
        }
    }
}

impl HkShape {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize) -> Self {
        let ty: u32 = OrderedData::from_bytes::<O>(&data[offset + 32..]);
        match ty {
            0 => Self::HkShape0(OrderedData::from_bytes::<O>(&data[offset..])),
            1 => Self::HkShape1(OrderedData::from_bytes::<O>(&data[offset..])),
            2 => Self::HkShape2(OrderedData::from_bytes::<O>(&data[offset..])),
            3 => Self::HkShape3(OrderedData::from_bytes::<O>(&data[offset..])),
            4 => Self::HkShape4(OrderedData::from_bytes::<O>(&data[offset..])),
            5 => {
                let info: HkShape5 = OrderedData::from_bytes::<O>(&data[offset..]);
                let a = OrderedDataVec::from_bytes::<O>(&data[info.a_offset as usize..], info.a_num as usize * 4);
                let mut b_num = info.b_num as usize; // sketchy stuff to account for data that was not otherwise captured, is it needed?
                while (info.b_offset as usize + b_num * 12) % 16 != 0 { b_num += 1; }
                let b = OrderedDataVec::from_bytes::<O>(&data[info.b_offset as usize..], b_num * 3);
                let b_extra = b_num - info.b_num as usize;
                Self::HkShape5 { info, a, b, b_extra }
            },
            6 => {
                let info: HkShape6 = OrderedData::from_bytes::<O>(&data[offset..]);
                let c = OrderedDataVec::from_bytes::<O>(&data[info.c_offset as usize..], info.c_num as usize);
                let d = OrderedDataVec::from_bytes::<O>(&data[info.d_offset as usize..], info.d_num as usize * 3);
                let e = OrderedDataVec::from_bytes::<O>(&data[info.e_offset as usize..], info.e_num as usize * 3);
                Self::HkShape6 { info, c, d, e }
            }
            _ => {
                panic!("Unknown HkShape type {} at offset {}", ty, offset)
            }
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self, mut offset: usize, infos: &mut DumpInfos) -> Vec<u8> {
        match self {
            Self::HkShape0(shape) => {
                infos.hk_shape.push(HkShapeInfo::HkShape0(shape.clone()));
                vec![]
            },
            Self::HkShape1(shape) => {
                infos.hk_shape.push(HkShapeInfo::HkShape1(shape.clone()));
                vec![]
            },
            Self::HkShape2(shape) => {
                infos.hk_shape.push(HkShapeInfo::HkShape2(shape.clone()));
                vec![]
            },
            Self::HkShape3(shape) => {
                infos.hk_shape.push(HkShapeInfo::HkShape3(shape.clone()));
                vec![]
            },
            Self::HkShape4(shape) => {
                infos.hk_shape.push(HkShapeInfo::HkShape4(shape.clone()));
                vec![]
            },
            Self::HkShape5{info, a, b,  b_extra} => {
                let mut info = info.clone();
                let mut data = vec![];

                let off: usize = (offset + 15) & 0xFFFFFFF0;
                data.extend(vec![0u8; off-offset]);
                offset = off;

                info.a_num = a.len() as u32 / 4;
                info.a_offset = offset as u32;
                let vals = a.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);

                info.b_num = (b.len() / 3 - *b_extra) as u32;
                info.b_offset = offset as u32;
                let vals = b.dump_bytes::<O>();
                data.extend(vals);

                infos.hk_shape.push(HkShapeInfo::HkShape5(info));
                data
            },
            Self::HkShape6 { info, c, d, e } => {
                let mut info = info.clone();
                let mut data = vec![];

                info.d_num = d.len() as u32 / 3;
                info.d_offset = offset as u32;
                let vals = d.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);

                info.e_num = e.len() as u32 / 3;
                info.e_offset = offset as u32;
                let vals = e.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);

                let off: usize = (offset + 3) & 0xFFFFFFFC;
                data.extend(vec![0u8; off-offset]);
                offset = off;

                info.c_num = c.len() as u32;
                info.c_offset = offset as u32;
                let vals = c.dump_bytes::<O>();
                offset += vals.len();
                data.extend(vals);

                let off: usize = (offset + 3) & 0xFFFFFFFC;
                data.extend(vec![0u8; off-offset]);

                infos.hk_shape.push(HkShapeInfo::HkShape6(info));
                data
            }
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offsets: &mut Vec<usize>, blocks: & Vec<Vec<u8>>) -> Self {
        let info: AnimationInfo = OrderedData::from_bytes::<O>(data);
        let (_, (offset, block)) = zip(offsets.iter().cloned(), blocks.iter()).enumerate().find(|(i, _)| {
            info.gamemodemask & (1 << i) != 0
        }).unwrap();
        let obj1 = OrderedDataVec::from_bytes::<O>(&block[offset + info.obj1_offset as usize..], info.obj1_num as usize * 2);
        let obj2 = OrderedDataVec::from_bytes::<O>(&block[offset + info.obj2_offset as usize..], info.obj2_num as usize * 4);
        let obj3 = OrderedDataVec::from_bytes::<O>(&block[offset + info.obj3_offset as usize..], info.obj3_num as usize);
        let keys = OrderedDataVec::from_bytes::<O>(&block[offset + info.keys_offset as usize..], (info.keys_num + info.obj1_num) as usize);
        let (obj5_header, obj5_a, obj5_b) = if info.obj5_offset != 0 {
            let obj5_header: animation::Obj5Header = OrderedData::from_bytes::<O>(&block[offset + info.obj5_offset as usize..]);
            let obj5_a = OrderedDataVec::from_bytes::<O>(&block[offset + obj5_header.obj_a_offset as usize..], obj5_header.obj_a_num as usize * 7);
            let obj5_b = OrderedDataVec::from_bytes::<O>(&block[offset + obj5_header.obj_b_offset as usize..], obj5_header.obj_b_num as usize * 7);
            (Some(obj5_header), obj5_a, obj5_b)
        } else {(
            None, vec![], vec![]
        )};
        let obj_c = if info.kind == 3 {
            Some(animation::HkaSplineSkeletalAnimation::from_data::<O>(&block[..], offset, &info))
        } else if info.kind < 3 {
            warn!("Unhandled animation type {} at offset {}", info.kind, offset);
            None
        } else {
            warn!("Unknown animation type {} at offset {}", info.kind, offset);
            None
        };
        offsets.iter_mut().enumerate().filter(|(i, _)| info.gamemodemask & (1 << i) != 0).for_each(|(_, x)| *x += info.size as usize );
        Self { info, obj1, obj2, obj3, keys, obj5_a, obj5_b, obj5_header, obj_c }
    }

    pub fn dump<O: ByteOrder + 'static>(&self, offset: usize, infos: &mut DumpInfos) -> Vec<u8> {
        let mut info = self.info.clone();
        info.offset = offset as u32;
        let mut data = vec![0u8; info.size as usize];
        self.obj1.to_bytes::<O>(&mut data[info.obj1_offset as usize..]);
        self.obj2.to_bytes::<O>(&mut data[info.obj2_offset as usize..]);
        self.obj3.to_bytes::<O>(&mut data[info.obj3_offset as usize..]);
        self.keys.to_bytes::<O>(&mut data[info.keys_offset as usize..]);
        if let Some(obj5_header) = &self.obj5_header {
            obj5_header.to_bytes::<O>(&mut data[info.obj5_offset as usize..]);
            self.obj5_a.to_bytes::<O>(&mut data[obj5_header.obj_a_offset as usize..]);
            self.obj5_b.to_bytes::<O>(&mut data[obj5_header.obj_b_offset as usize..]);
        }
        if let Some(obj_c) = &self.obj_c {
            obj_c.into_data::<O>(&mut data, 0, &info);
        }
        infos.animation.push(info);
        data
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DumpInfos {
    pub header: Header,
    pub animation: Vec<AnimationInfo>,
    pub hk_shape: Vec<HkShapeInfo>,
    pub shape: Vec<ShapeInfo>,
    pub mesh: Vec<MeshInfo>,
    pub mat1: Vec<MatBase>,
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