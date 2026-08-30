use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use indexmap::IndexMap;
use std::collections::HashMap;

use crate::{
    types::{Crc, Matrix4x4, ReadData, Vector3, DumpSlice, align_offset, ref_slice, CompressedDataRef, slice, BaseTypes, NE, Vector4, Vector2}, 
    level::{
        model::{
            //data::ModelData,
            shape::{
                ShapeInfo, HkConstraintData, DumpShape, DumpHkConstraint, HkConstraintInfo, TRS, HkConstraint,
                BoxShape, CapsuleShape, CylinderShape, SphereShape, BVTreeMeshInfo, ConvexVerticesInfo, ShapeExtraInfo, HkShapeInfo 
            },
            data::{BufferInfo, IBuffInfo, VBuffInfo, DumpModelData},
            mat::{DumpMat, Mat1, Mat2, Mat3, Mat4, MatExtra},
        },
        pak::block1::infos::{InfoCounts, DumpInfos, DumpInfoData},
    },
};
use lotrc_proc::{derive_pod};

pub mod data;
pub mod mat;
pub mod shape;

#[derive_pod]
pub struct LodInfo<T: BaseTypes> {
    pub start: T::u32,
    pub static_end: T::u32,
    pub skinned_end: T::u32,
    pub physics_end: T::u32,
    pub breakable_end: T::u32,
}

impl<T: BaseTypes> LodInfo<T> {
    pub const UNKNOWN: u32 = 1;
    pub const STATIC: u32 = 2;
    pub const SKINNED: u32 = 4;
    pub const PHYSICS: u32 = 8;
    pub const BREAKABLE: u32 = 16;
    pub const LOD0: u32 = 32;
    pub const LOD1: u32 = 64;
    pub const LOD2: u32 = 128;
    pub const LOD3: u32 = 256;
}

#[derive_pod]
pub struct BoundingBox<T: BaseTypes> {
    center: Vector3<T>,
    unk_3: T::f32,
    half_width: Vector3<T>,
    unk_7: T::f32,
}

#[derive_pod]
pub struct ModelInfoPS3<T: BaseTypes> {
    pub key: Crc<T>,
    pub gamemodemask: T::i32,
    pub mat_offset: T::u32,
    pub buffer_info_offset: T::u32, // pointer to obj2, uses mat_num of sequential objects
    pub bounding_box: BoundingBox<T>, // (center x, y, z, ?, half_width x, y, z, ?) default vals of 1.0 for the ? vals seems to work
    pub mesh_order_offset: T::u32, // ints (c & 0x3fffffff is an index into the obj2s referenced by this object) (1 for each mesh)
    pub lod0: LodInfo<T>,
    pub lod1: LodInfo<T>,
    pub lod2: LodInfo<T>,
    pub lod3: LodInfo<T>,
    pub mat_num: T::u32,
    pub bones_offset: T::u32, // ints
    pub bone_parents_offset: T::u32,
    pub bone_transforms_offset: T::u32, // 16 ints (matrix?) for keys_num
    pub bones_num: T::u32,
    pub skin_binds_offset: T::u32,
    pub skin_binds_num: T::u32,
    pub skin_order_offset: T::u32,
    pub vbuff_offset: T::u32,
    pub vbuff_num: T::u32,
    pub ibuff_offset: T::u32,
    pub ibuff_num: T::u32,
    pub mesh_bounding_boxes_offset: T::u32,
    pub unk_46: T::f32,           // maybe something to do with size?
    pub variation_counts: T::u32, // maybe something to do with variation
    pub vals_j_num: T::u32,
    pub vals_j_offset: T::u32,
    pub block_offset: T::u32,
    pub vals_k_offset: T::u32, // not sure on the size, seems to be 36 ints
    pub asset_key: Crc<T>,     // data in bin that is vertex & index buffer values
    pub asset_type: T::u32,
    pub unk_54: T::u32, // 1 for occuluder otherwise 0 ??
    pub shape_offset: T::u32, 
    pub shape_num: T::u32,
    pub hk_constraint_data_offset: T::u32,
    pub hk_constraint_data_num: T::u32, 
    pub hk_constraint_offset: T::u32,
    pub slots_offset: T::u32, 
    pub slot_map_offset: T::u32,
    pub bone_bounding_boxes_offset: T::u32,
    pub unk_55: T::u32, 
}

#[derive_pod]
pub struct ModelInfo<T: BaseTypes> {
    pub key: Crc<T>,
    pub gamemodemask: T::i32,
    pub mat_offset: T::u32,
    pub buffer_info_offset: T::u32, // pointer to obj2, uses mat_num of sequential objects
    pub bounding_box: BoundingBox<T>, // (center x, y, z, ?, half_width x, y, z, ?) default vals of 1.0 for the ? vals seems to work
    pub mesh_order_offset: T::u32, // ints (c & 0x3fffffff is an index into the obj2s referenced by this object) (1 for each mesh)
    pub lod0: LodInfo<T>,
    pub lod1: LodInfo<T>,
    pub lod2: LodInfo<T>,
    pub lod3: LodInfo<T>,
    pub mat_num: T::u32,
    pub bones_offset: T::u32, // ints
    pub bone_parents_offset: T::u32,
    pub bone_transforms_offset: T::u32, // 16 ints (matrix?) for keys_num
    pub bones_num: T::u32,
    pub skin_binds_offset: T::u32,
    pub skin_binds_num: T::u32,
    pub skin_order_offset: T::u32,
    pub vbuff_offset: T::u32,
    pub vbuff_num: T::u32,
    pub ibuff_offset: T::u32,
    pub ibuff_num: T::u32,
    pub mesh_bounding_boxes_offset: T::u32,
    pub unk_46: T::f32,           // maybe something to do with size?
    pub variation_counts: T::u32, // maybe something to do with variation
    pub vals_j_num: T::u32,
    pub vals_j_offset: T::u32,
    pub block_offset: T::u32,
    pub vals_k_offset: T::u32, // not sure on the size, seems to be 36 ints
    pub asset_key: Crc<T>,     // data in bin that is vertex & index buffer values
    pub asset_type: T::u32,
    pub unk_54: T::u32, // 1 for occuluder otherwise 0 ??
    pub unk_55: T::u32, // always 0 ??
    pub shape_offset: T::u32,
    pub shape_num: T::u32,
    pub hk_constraint_data_offset: T::u32, // optional pointer to obje
    pub hk_constraint_data_num: T::u32,
    pub hk_constraint_offset: T::u32, // optional pointer to hkConstraint
    pub slots_offset: T::u32,
    pub slot_map_offset: T::u32,
    pub bone_bounding_boxes_offset: T::u32, // 8 ints
}

#[derive_pod]
pub struct BlockHeader<T: BaseTypes> {
    pub a: T::u32,
    pub b: T::u32,
    pub unk_2: T::u32,
    pub unk_3: T::u32,
    pub unk_4: T::u32,
}

#[derive_pod]
pub struct BlockVal<T: BaseTypes> {
    pub unk_0: T::u32,
    pub unk_1: T::u32,
    pub unk_2: T::u32,
    pub unk_3: T::u32,
    pub unk_4: T::u16,
    pub unk_5: T::u16,
}

#[derive_pod]
pub struct BlockHeader1<T: BaseTypes> {
    pub a: T::u32,
    pub b: T::u32,
    pub unk_2: T::u32,
    pub unk_3: T::u32,
}

#[derive_pod]
pub struct BlockHeader2<T: BaseTypes> {
    pub n: T::u32,
    pub unk_1: T::f32,
    pub unk_2: T::f32,
    pub unk_3: T::u32,
    pub unk_4: T::u32,
}

#[derive_pod]
pub struct BlockValA<T: BaseTypes> {
    pub unk_0: T::f32,
    pub unk_1: T::f32,
    pub unk_2: T::f32,
    pub unk_3: T::f32,
    pub unk_4: T::f32,
    pub unk_5: T::f32,
    pub unk_6: T::f32,
    pub unk_7: T::f32,
    pub unk_8: T::f32,
    pub unk_9: T::u32,
    pub unk_10: T::u32,
    pub unk_11: T::u32,
}

#[derive_pod]
pub struct BlockValB<T: BaseTypes> {
    pub unk_0: T::u16,
    pub unk_1: T::u16,
    pub unk_2: T::f32,
    pub unk_3: T::f32,
    pub unk_4: T::f32,
    pub unk_5: T::f32,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct BlockRef<'a, T: BaseTypes> {
    info1: &'a BlockHeader1<T>,
    info2: &'a BlockHeader2<T>,
    vals_a: ref_slice<'a, BlockValA<T>>,
    vals_b: ref_slice<'a, BlockValA<T>>,
    vals_c: ref_slice<'a, BlockValB<T>>,
    pad: ref_slice<'a, u8>,
}

impl<'a, T: BaseTypes> BlockRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let info1 = BlockHeader1::<T>::from_data(&src[offset..]).context("info1")?;
        offset += std::mem::size_of_val(info1);
        let vals_a = BlockValA::slice_from_data(&src[offset..], info1.a.into() as usize)
            .context("vals_a")?;
        offset += std::mem::size_of_val(vals_a);
        let vals_b = BlockValA::slice_from_data(&src[offset..], info1.b.into() as usize)
            .context("vals_b")?;
        offset += std::mem::size_of_val(vals_b);
        let info2 = BlockHeader2::<T>::from_data(&src[offset..]).context("info2")?;
        offset += std::mem::size_of_val(info2);
        let vals_c = BlockValB::slice_from_data(&src[offset..], info2.n.into() as usize)
            .context("vals_c")?;
        offset += std::mem::size_of_val(vals_c);
        let pad = &src[offset..];
        Ok(Self { info1, info2, vals_a, vals_b, vals_c, pad })
    }
}

#[derive(Default, Debug, Clone)]
pub struct Block {
    pub info1: BlockHeader1<NE>,
    pub info2: BlockHeader2<NE>,
    pub vals_a: Vec<BlockValA<NE>>,
    pub vals_b: Vec<BlockValA<NE>>,
    pub vals_c: Vec<BlockValB<NE>>,
    pub pad: Vec<u8>,
}

impl<T: BaseTypes> From<&BlockRef<'_, T>> for Block
where
    BlockHeader1<NE>: From<BlockHeader1<T>>,
    BlockHeader2<NE>: From<BlockHeader2<T>>,
    BlockValA<NE>: From<BlockValA<T>>,
    BlockValB<NE>: From<BlockValB<T>>,
{
    fn from(val: &BlockRef<T>) -> Self {
        Self {
            info1: (*val.info1).into(),
            info2: (*val.info2).into(),
            vals_a: val.vals_a.iter().map(|&x| x.into()).collect(),
            vals_b: val.vals_b.iter().map(|&x| x.into()).collect(),
            vals_c: val.vals_c.iter().map(|&x| x.into()).collect(),
            pad: val.pad.to_vec(),
        }
    }
}

pub trait DumpBlock<T: BaseTypes> {
    fn vals_a_num(&self) -> usize;
    fn vals_b_num(&self) -> usize;
    fn vals_c_num(&self) -> usize;
    fn pad_num(&self) -> usize;
    fn write_info1(&self, info1: &mut BlockHeader1<T>) -> Result<()>;
    fn write_info2(&self, info2: &mut BlockHeader2<T>) -> Result<()>;
    fn write_vals_a(&self, vals_a: &mut [BlockValA<T>]) -> Result<()>;
    fn write_vals_b(&self, vals_b: &mut [BlockValA<T>]) -> Result<()>;
    fn write_vals_c(&self, vals_c: &mut [BlockValB<T>]) -> Result<()>;
    fn write_pad(&self, pad: &mut [u8]) -> Result<()>;

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let info1 = BlockHeader1::mut_from_data(dst).context("info1")?;
        self.write_info1(info1).context("write info1")?;

        let vals_a = BlockValA::mut_slice_from_data(dst, self.vals_a_num()).context("vals_a")?;
        self.write_vals_a(vals_a).context("write vals_a")?;

        let vals_b = BlockValA::mut_slice_from_data(dst, self.vals_b_num()).context("vals_b")?;
        self.write_vals_b(vals_b).context("write vals_b")?;

        let info2 = BlockHeader2::mut_from_data(dst).context("info2")?;
        self.write_info2(info2).context("write info2")?;

        let vals_c = BlockValB::mut_slice_from_data(dst, self.vals_c_num()).context("vals_c")?;
        self.write_vals_c(vals_c).context("write vals_c")?;

        let pad = u8::mut_slice_from_data(dst, self.pad_num()).context("pad")?;
        self.write_pad(pad).context("write pad")?;
        Ok(())
    }
    fn add_size(&self, offset: usize) -> usize {
        offset 
            + std::mem::size_of::<BlockHeader1<T>>()
            + std::mem::size_of::<BlockValA<T>>() * self.vals_a_num() 
            + std::mem::size_of::<BlockValA<T>>() * self.vals_b_num()
            + std::mem::size_of::<BlockHeader2<T>>()
            + std::mem::size_of::<BlockValB<T>>() * self.vals_c_num()
            + self.pad_num()
    }
}

impl<T: BaseTypes> DumpBlock<T> for BlockRef<'_, T> {
    fn vals_a_num(&self) -> usize {
        self.vals_a.len()
    }
    fn vals_b_num(&self) -> usize {
        self.vals_b.len()
    }
    fn vals_c_num(&self) -> usize {
        self.vals_c.len()
    }
    fn pad_num(&self) -> usize {
        self.pad.len()
    }
    fn write_info1(&self, info1: &mut BlockHeader1<T>) -> Result<()> {
        *info1 = *self.info1;
        Ok(())
    }
    fn write_info2(&self, info2: &mut BlockHeader2<T>) -> Result<()> {
        *info2 = *self.info2;
        Ok(())
    }
    fn write_vals_a(&self, vals_a: &mut [BlockValA<T>]) -> Result<()> {
        vals_a.copy_from_slice(&self.vals_a[..]);
        Ok(())
    }
    fn write_vals_b(&self, vals_b: &mut [BlockValA<T>]) -> Result<()> {
        vals_b.copy_from_slice(&self.vals_b[..]);
        Ok(())
    }
    fn write_vals_c(&self, vals_c: &mut [BlockValB<T>]) -> Result<()> {
        vals_c.copy_from_slice(&self.vals_c[..]);
        Ok(())
    }
    fn write_pad(&self, pad: &mut [u8]) -> Result<()> {
        pad.copy_from_slice(&self.pad[..]);
        Ok(())
    }
}

#[derive_pod]
pub struct Key2<T: BaseTypes> {
    pub key: Crc<T>,
    pub val: T::u32,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct BonesRef<'a, T: BaseTypes> {
    pub names:  ref_slice<'a, Crc<T>>,
    pub parents: ref_slice<'a, T::i32>,
    pub transforms: ref_slice<'a, Matrix4x4<T>>,
    pub bounding_boxes: ref_slice<'a, BoundingBox<T>>,
}

impl<'a, T: BaseTypes> BonesRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a ModelInfo<T>) -> Result<Self> {
        let size = info.bones_num.into() as usize;
        let parents = T::i32::slice_from_data(
            &src[info.bone_parents_offset.into() as usize..],
            size,
        ).context("parents")?;
        if parents[0].into() != -1 {
            return Err(anyhow!("first bone should be the root, but parent != -1"));
        }
        let names_off = info.bones_offset.into() as usize;
        
        let (names, bounding_boxes) = if names_off != 0 {
            (
                Crc::<T>::slice_from_data(&src[names_off..], size).context("names")?,
                BoundingBox::slice_from_data(
                    &src[info.bone_bounding_boxes_offset.into() as usize..],
                    info.bones_num.into() as usize,
                ).context("bounding_boxes")?
            )
        } else {
            (&[] as _, &[] as _)
        };
        let transforms = Matrix4x4::slice_from_data(
            &src[info.bone_transforms_offset.into() as usize..],
            size,
        ).context("transforms")?;
        Ok(Self { names, parents, transforms, bounding_boxes })
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.transforms.len()
    }
}

#[derive(Debug, Clone)]
pub struct Bones {
    pub names: Vec<Crc<NE>>,
    pub parents: Vec<i32>,
    pub transforms: Vec<Matrix4x4<NE>>,
    pub bounding_boxes: Vec<BoundingBox<NE>>
}

impl<T: BaseTypes> From<&BonesRef<'_, T>> for Bones
where
    Crc<NE>: From<Crc<T>>,
    Matrix4x4<NE>: From<Matrix4x4<T>>,
    BoundingBox<NE>: From<BoundingBox<T>>,
{
    fn from(val: &BonesRef<T>) -> Self {
        Self {
            names: val.names.iter().map(|&x| x.into()).collect(),
            parents: val.parents.iter().map(|&x| x.into()).collect(),
            transforms: val.transforms.iter().map(|&x| x.into()).collect(),
            bounding_boxes: val.bounding_boxes.iter().map(|&x| x.into()).collect(),
        }
    }
}

pub struct BonesDump<'a, T: BaseTypes> {
    names:  &'a mut [Crc<T>],
    parents: &'a mut [T::i32],
    transforms: &'a mut [Matrix4x4<T>],
    bounding_boxes: &'a mut [BoundingBox<T>],
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ModelRef<'a, T: BaseTypes> {
    pub info: &'a ModelInfo<T>,
    pub bones: BonesRef<'a, T>,
    pub mat_order: ref_slice<'a, T::u32>,
    pub mesh_order: ref_slice<'a, T::u32>,
    pub mesh_bounding_boxes: ref_slice<'a, BoundingBox<T>>,
    pub skin_binds: ref_slice<'a, Matrix4x4<T>>,
    pub vals_j: ref_slice<'a, T::u32>,
    pub val_k_header: ref_slice<'a, T::u16>,
    pub vals_k: ref_slice<'a, T::u32>,
    pub skin_order: ref_slice<'a, T::u32>,
    pub slots: ref_slice<'a, Key2<T>>,
    pub slot_map: ref_slice<'a, T::u32>,
    pub block_header: Option<&'a T::u32>,
    pub block_offsets: ref_slice<'a, T::u32>,
    pub blocks: slice<BlockRef<'a, T>>,
    pub buffer_infos: ref_slice<'a, BufferInfo<T>>,
    pub vbuff_order: ref_slice<'a, T::u32>,
    pub ibuff_order: ref_slice<'a, T::u32>,
    pub vbuffs: IndexMap<u32, &'a VBuffInfo<T>>,
    pub ibuffs: IndexMap<u32, &'a IBuffInfo<T>>,
    pub mats: IndexMap<u32, mat::MatRef<'a, T>>,
    pub hk_constraint: Option<shape::HkConstraintRef<'a, T>>, // stores bone transforms used for ragdoll
    pub hk_constraint_datas: ref_slice<'a, HkConstraintData<T>>,
    pub shapes: slice<shape::ShapeRef<'a, T>>,
    pub data: data::ModelDataRef<'a, T>,
}

impl<'a, T: BaseTypes> ModelRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a ModelInfo<T>, model_data: &IndexMap<u32, &'a CompressedDataRef<'a>>) -> Result<Self> {
        let bones = BonesRef::from_data(src, info).context("bones")?;
        let mat_order = T::u32::slice_from_data(
            &src[info.mat_offset.into() as usize..],
            info.mat_num.into() as usize,
        )
        .context("mat order")?;
        let mesh_order = T::u32::slice_from_data(
            &src[info.mesh_order_offset.into() as usize..],
            info.lod3.breakable_end.into() as usize,
        )
        .context("mesh order")?;
        let mesh_bounding_boxes = BoundingBox::slice_from_data(
            &src[info.mesh_bounding_boxes_offset.into() as usize..],
            info.lod3.breakable_end.into() as usize,
        )
        .context("mesh bounding boxes")?;
        let skin_binds = Matrix4x4::slice_from_data(
            &src[info.skin_binds_offset.into() as usize..],
            info.skin_binds_num.into() as usize,
        )
        .context("skin binds")?;
        let vals_j = T::u32::slice_from_data(
            &src[info.vals_j_offset.into() as usize..],
            info.vals_j_num.into() as usize,
        )
        .context("vals j")?;
        let (val_k_header, vals_k) = if info.vals_k_offset.into() != 0 {
            (
                T::u16::slice_from_data(&src[info.vals_k_offset.into() as usize..], 2)
                    .context("vals k header")?,
                T::u32::slice_from_data(&src[info.vals_k_offset.into() as usize + 4..], 35)
                    .context("vals k")?,
            )
        } else {
            (&[] as &[T::u16], &[] as &[T::u32])
        };
        let skin_order = if info.skin_order_offset.into() != 0 {
            T::u32::slice_from_data(
                &src[info.skin_order_offset.into() as usize..],
                info.skin_binds_num.into() as usize,
            )
            .context("skin order")?
        } else {
            &[] as &[T::u32]
        };
        let (slots, slot_map) = if info.slots_offset.into() != 0 {
            if info.slot_map_offset.into() == 0 {
                return Err(anyhow!("expected non zero slot_map_offset"));
            }
            let mut i = 0;
            {
                while {
                    let &val = T::u32::from_data(&src[info.slots_offset.into() as usize + i * 8..])
                        .context("slot end")?;
                    val.into() != 0
                } {
                    i += 1;
                }
                i += 1;
            }
            let slots = Key2::<T>::slice_from_data(&src[info.slots_offset.into() as usize..], i)
                .context("slot")?;
            (
                slots,
                T::u32::slice_from_data(
                    &src[info.slot_map_offset.into() as usize..],
                    slots.last().unwrap().val.into() as usize,
                )
                .context("slot map")?,
            )
        } else {
            (&[] as &[Key2<T>], &[] as &[T::u32])
        };
        let (block_header, block_offsets, blocks) = if info.block_offset.into() != 0 {
            let block_header =
                T::u32::from_data(&src[info.block_offset.into() as usize..]).context("block header")?;
            let n = ((info.lod0.physics_end.into() - info.lod0.skinned_end.into()) as usize).max(0);
            let block_offsets =
                T::u32::slice_from_data(&src[info.block_offset.into() as usize + 4..], n + 1)
                    .context("block offsets")?;
            let mut blocks = Vec::with_capacity(n);
            for i in 0..n {
                let size = (block_offsets[i + 1].into() - block_offsets[i].into()) as usize;
                let offset = (block_offsets[i].into() + info.block_offset.into()) as usize;
                let block = BlockRef::from_data(&src[offset..offset+size]).with_context(|| format!("block {}", i))?;
                blocks.push(block);
            }
            (Some(block_header), block_offsets, blocks.into_boxed_slice())
        } else {
            (None, &[] as &[T::u32], Box::default())
        };
        let hk_constraint = if info.hk_constraint_offset.into() != 0 {
            Some(
                shape::HkConstraintRef::from_data(src, info.hk_constraint_offset.into() as usize)
                    .context("hk_constarint")?,
            )
        } else {
            None
        };
        let hk_constraint_datas = HkConstraintData::slice_from_data(
            &src[info.hk_constraint_data_offset.into() as usize..],
            info.hk_constraint_data_num.into() as usize,
        )
        .context("hk_constraint_datas")?;
        let shape_infos = ShapeInfo::slice_from_data(&src[info.shape_offset.into() as usize..], info.shape_num.into() as usize).context("shape_infos")?;
        
        let shapes = shape_infos.into_iter().enumerate()
            .map(|(i, info)| {
                shape::ShapeRef::from_data(src, info)
                .with_context(|| format!("shape {}", i))
            })
            .collect::<Result<Vec<_>>>()?.into_boxed_slice();
        let mats = mat_order
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                Ok((
                    x.into(),
                    mat::MatRef::from_data(src, x.into() as usize)
                        .with_context(|| format!("mat {}", i))?,
                ))
            })
            .collect::<Result<IndexMap<_, _>>>()?;
        let buffer_infos = BufferInfo::slice_from_data(&src[info.buffer_info_offset.into() as usize..], info.mat_num.into() as usize).context("buffer infos")?;
        let vbuff_order = T::u32::slice_from_data(&src[info.vbuff_offset.into() as usize..], info.vbuff_num.into() as usize).context("vbuff order")?;
        let ibuff_order = T::u32::slice_from_data(&src[info.ibuff_offset.into() as usize..], info.ibuff_num.into() as usize).context("vbuff order")?;
        let vbuffs = vbuff_order.iter().map(|&x| Ok((x.into(), VBuffInfo::from_data(&src[x.into() as usize..]).with_context(|| format!("vbuff info {}", x.into()))?))).collect::<Result<IndexMap<_, _>>>()?;
        let ibuffs = ibuff_order.iter().map(|&x| Ok((x.into(), IBuffInfo::from_data(&src[x.into() as usize..]).with_context(|| format!("ibuff info {}", x.into()))?))).collect::<Result<IndexMap<_, _>>>()?;
        let data = data::ModelDataRef::from_data(src, info, model_data).context("model data")?;
        Ok(Self {
            info: info,
            bones,
            mat_order: mat_order.into(),
            mesh_order: mesh_order.into(),
            mesh_bounding_boxes: mesh_bounding_boxes.into(),
            skin_binds: skin_binds.into(),
            vals_j: vals_j.into(),
            val_k_header: val_k_header.into(),
            vals_k: vals_k.into(),
            skin_order: skin_order.into(),
            slots: slots.into(),
            slot_map: slot_map.into(),
            block_header: block_header.into(),
            block_offsets: block_offsets.into(),
            blocks: blocks.into(),
            mats: mats.into(),
            buffer_infos: buffer_infos.into(),
            vbuff_order: vbuff_order.into(),
            ibuff_order: ibuff_order.into(),
            vbuffs: vbuffs.into(),
            ibuffs: ibuffs.into(),
            hk_constraint: hk_constraint.into(),
            hk_constraint_datas: hk_constraint_datas.into(),
            shapes: shapes.into(),
            data
        })
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    pub info: ModelInfo<NE>,
    pub bones: Bones,
    pub mat_order: Vec<u32>,
    pub mesh_order: Vec<u32>, // order of models (mapped to lod0, lod1, lod2, lod3)
    pub mesh_bounding_boxes: Vec<BoundingBox<NE>>,
    pub skin_binds: Vec<Matrix4x4<NE>>, // mat4, bind matrices or something??
    pub vals_j: Vec<u32>,           // bows & banners, maybe for strings?
    pub val_k_header: Vec<u16>,
    pub vals_k: Vec<u32>,          // has to do with trees
    pub skin_order: Vec<u32>, // bone mapping for vals_g, seems to be the mapping used for skinning
    pub slots: Vec<Key2<NE>>,     // attachment points
    pub slot_map: Vec<u32>,   // attachment bone mapping
    pub block_header: Option<u32>, // has to do with havok cloth / hair stuff
    pub blocks: Vec<Block>,
    pub mats: Vec<mat::Mat>,
    pub hk_constraint: Option<HkConstraint>, // stores bone transforms used for ragdoll ??
    pub hk_constraint_datas: Vec<HkConstraintData<NE>>,
    pub shapes: Vec<shape::Shape>,
    pub data: data::ModelData
}

impl<T: BaseTypes> From<&ModelRef<'_, T>> for Model
where
    ModelInfo<NE>: From<ModelInfo<T>>,
    Crc<NE>: From<Crc<T>>,
    Matrix4x4<NE>: From<Matrix4x4<T>>,
    BoundingBox<NE>: From<BoundingBox<T>>,
    BlockHeader1<NE>: From<BlockHeader1<T>>,
    BlockHeader2<NE>: From<BlockHeader2<T>>,
    BlockValA<NE>: From<BlockValA<T>>,
    BlockValB<NE>: From<BlockValB<T>>,
    Key2<NE>: From<Key2<T>>,
    Mat1<NE>: From<Mat1<T>>,
    Mat2<NE>: From<Mat2<T>>,
    Mat3<NE>: From<Mat3<T>>,
    Mat4<NE>: From<Mat4<T>>,
    MatExtra<NE>: From<MatExtra<T>>,
    HkConstraintInfo<NE>: From<HkConstraintInfo<T>>,
    TRS<NE>: From<TRS<T>>,
    TRS<NE>: From<TRS<T>>,
    HkConstraintData<NE>: From<HkConstraintData<T>>,
    ShapeInfo<NE>: From<ShapeInfo<T>>,
    ShapeExtraInfo<NE>: From<ShapeExtraInfo<T>>,
    BoxShape<NE>: From<BoxShape<T>>,
    SphereShape<NE>: From<SphereShape<T>>,
    CapsuleShape<NE>: From<CapsuleShape<T>>,
    CylinderShape<NE>: From<CylinderShape<T>>,
    ConvexVerticesInfo<NE>: From<ConvexVerticesInfo<T>>,
    BVTreeMeshInfo<NE>: From<BVTreeMeshInfo<T>>,
    HkShapeInfo<NE>: From<HkShapeInfo<T>>,
    Vector3<NE>: From<Vector3<T>>,
    Vector4<NE>: From<Vector4<T>>,
    BufferInfo<NE>: From<BufferInfo<T>>,
    Vector2<NE>: From<Vector2<T>>,
    VBuffInfo<NE>: From<VBuffInfo<T>>,
{
    fn from(val: &ModelRef<T>) -> Self {
        Self {
            info: (*val.info).into(),
            bones: (&val.bones).into(),
            mat_order: val
                .mat_order
                .iter()
                .map(|&x| val.mats.get_index_of(&x.into()).unwrap() as u32)
                .collect(),
            mesh_order: val.mesh_order.iter().map(|&x| x.into()).collect(),
            mesh_bounding_boxes: val.mesh_bounding_boxes.iter().map(|&x| x.into()).collect(),
            skin_binds: val.skin_binds.iter().map(|&x| x.into()).collect(),
            vals_j: val.vals_j.iter().map(|&x| x.into()).collect(),
            val_k_header: val.val_k_header.iter().map(|&x| x.into()).collect(),
            vals_k: val.vals_k.iter().map(|&x| x.into()).collect(),
            skin_order: val.skin_order.iter().map(|&x| x.into()).collect(),
            slots: val.slots.iter().map(|&x| x.into()).collect(),
            slot_map: val.slot_map.iter().map(|&x| x.into()).collect(),
            block_header: val.block_header.map(|&x| x.into()),
            blocks: val.blocks.iter().map(|x| x.into()).collect(),
            mats: val
                .mats
                .values()
                .map(|x| x.into())
                .collect(),
            hk_constraint: val.hk_constraint.as_ref().map(|x| x.into()),
            hk_constraint_datas: val.hk_constraint_datas.iter().map(|&x| x.into()).collect(),
            shapes: val.shapes.iter().map(|x| x.into()).collect(),
            data: (&val.data).into()
        }
    }
}

pub trait DumpModel<T: BaseTypes> {
    fn info(&self) -> &ModelInfo<T>;
    fn key(&self) -> u32;
    fn bone_num(&self) -> usize;
    fn vals_j_num(&self) -> usize;
    fn skin_bind_num(&self) -> usize;
    fn has_skin_order(&self) -> bool;
    fn mat_num(&self) -> usize;
    fn shape_num(&self) -> usize;
    fn mesh_order_num(&self) -> usize;
    fn buffer_num(&self) -> usize;
    fn vbuff_num(&self) -> usize;
    fn ibuff_num(&self) -> usize;
    fn has_vals_k(&self) -> bool;
    fn slot_num(&self) -> usize;
    fn slot_map_num(&self) -> usize;
    fn block_num(&self) -> usize;
    fn hk_constraint_data_num(&self) -> usize;
    fn write_bones(&self, bones: BonesDump<T>) -> Result<()>;
    fn write_vals_j(&self, vals_j: &mut [T::u32]) -> Result<()>;
    fn write_skin_binds(&self, skin_binds: &mut [Matrix4x4<T>]) -> Result<()>;
    fn write_skin_order(&self, skin_order: &mut [T::u32]) -> Result<()>;
    fn write_mesh_order(&self, mesh_order: &mut [T::u32]) -> Result<()>;
    fn write_mesh_bounding_boxes(&self, mesh_bboxes: &mut [BoundingBox<T>]) -> Result<()>;
    fn vbuff_order(&self) -> impl Iterator<Item=u32>;
    fn ibuff_order(&self) -> impl Iterator<Item=u32>;
    fn write_buffer_infos(&self, buffer_infos: &mut [BufferInfo<T>]) -> Result<()>;
    fn write_vbuff_infos(&self, vbuff_infos: &mut [VBuffInfo<T>]) -> Result<()>;
    fn write_ibuff_infos(&self, ibuff_infos: &mut [IBuffInfo<T>]) -> Result<()>;
    fn write_vals_k(&self, header: &mut [T::u16], vals_k: &mut [T::u32]) -> Result<()>;
    fn write_slots(&self, slots: &mut [Key2<T>]) -> Result<()>;
    fn write_slot_map(&self, slot_map: &mut [T::u32]) -> Result<()>;
    fn write_hk_constraint_datas(&self, hk_constraint_datas: &mut [HkConstraintData<T>]) -> Result<()>;
    fn mat_order(&self) -> impl Iterator<Item=u32>;
    fn mats<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpMat<T> + 'a))>;
    fn shapes<'a>(&'a self) -> impl Iterator<Item=&'a (impl DumpShape<T> + 'a)>;
    fn hk_constraint<'a>(&'a self) -> Option<&'a (impl DumpHkConstraint<T> + 'a)>;
    fn block_header(&self) -> Option<impl Into<T::u32>>;
    fn blocks<'a>(&'a self) -> impl Iterator<Item=&'a (impl DumpBlock<T> + 'a)>;
    fn data(&self) -> &impl DumpModelData<T>;
    fn asset_info(&self) -> (T::u32, T::u32);
    fn write_info(&self, info: &mut ModelInfo<T>);

    fn dump_misc(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>, info: &mut ModelInfo<T>, info_off: usize) -> Result<()> {
        if self.has_vals_k() {
            dst.align(16)?;
            info.vals_k_offset = (dst.offset as u32).into();
            let vals_k_header = T::u16::mut_slice_from_data(dst, 2).context("val_k_header")?;
            let vals_k = T::u32::mut_slice_from_data(dst, 35).context("vals_k")?;
            self.write_vals_k(vals_k_header, vals_k).context("vals_k")?;
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, vals_k_offset)) as u32).into();
        } else {
            info.vals_k_offset = 0u32.into();
        }

        if self.slot_num() != 0 {
            info.slots_offset =( dst.offset as u32).into();
            let slots = Key2::mut_slice_from_data(dst, self.slot_num()).context("slots")?;
            self.write_slots(slots).context("write slots")?;
            info.slot_map_offset = (dst.offset as u32).into();
            let slot_map = T::u32::mut_slice_from_data(dst, self.slot_map_num()).context("slot_map")?;
            self.write_slot_map(slot_map).context("write slot_map")?;
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, slots_offset)) as u32).into();
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, slot_map_offset)) as u32).into();
        } else {
            info.slots_offset = 0u32.into();
        }

        if let Some(block_header) = self.block_header() {
            dst.align(16)?;
            let start = dst.offset;
            info.block_offset = (start as u32).into();
            *T::u32::mut_from_data(dst).context("block_header")? = block_header.into();
            let block_offsets = T::u32::mut_slice_from_data(dst, self.block_num() + 1).context("block_offsets")?;
            for (i, (block, off)) in self.blocks().zip(block_offsets.iter_mut()).enumerate() {
                dst.align(16)?;
                *off = ((dst.offset - start) as u32).into();
                block.dump_into(dst).with_context(|| format!("block {}", i))?;
            }
            if let Some(off) = block_offsets.last_mut() {
                *off = ((dst.offset - start) as u32).into();
            }
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, block_offset)) as u32).into();
        }

        info.hk_constraint_data_num = (self.hk_constraint_data_num() as u32).into();
        if info.hk_constraint_data_num.into() != 0 {
            info.hk_constraint_data_offset = (infos.hk_constraint_datas.offset as u32).into();
            self.write_hk_constraint_datas(infos.hk_constraint_datas.next_slice(self.hk_constraint_data_num())).context("hk_constraint_datas")?;
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, hk_constraint_data_offset)) as u32).into();
        }

        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, mat_offset)) as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, buffer_info_offset)) as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, mesh_order_offset)) as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, bone_parents_offset)) as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, bone_transforms_offset)) as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, skin_binds_offset)) as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, vbuff_offset)) as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, ibuff_offset)) as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, mesh_bounding_boxes_offset)) as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, bone_bounding_boxes_offset)) as u32).into();

        Ok(())
    }

    fn add_misc_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        if self.has_vals_k() {
            offset = align_offset(offset, 16) + 2 * std::mem::size_of::<T::u16>() + 35 * std::mem::size_of::<T::u32>();
            counts.offsets += 1;
        }

        offset += std::mem::size_of::<Key2<T>>() * self.slot_num() + std::mem::size_of::<T::u32>() * self.slot_map_num();

        if self.slot_num() != 0 {
            counts.offsets += 2;
        }

        if self.block_header().is_some() {
            offset = align_offset(offset, 16) + (2 + self.block_num()) * std::mem::size_of::<T::u32>();
            for block in self.blocks() {
                offset = block.add_size(align_offset(offset, 16));
            }
            counts.offsets += 1;
        }

        counts.hk_constraint_datas += self.hk_constraint_data_num();
        if self.hk_constraint_data_num() != 0 {
            counts.offsets += 1;
        }

        counts.offsets += 10;

        offset
    }

    fn dump_mats(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>, info: &mut ModelInfo<T>) -> Result<()> {
        let mut off = dst.offset;
        info.mat_offset = (off as u32).into();
        let mat_map = self.mats().sorted_by_key(|(k, _)| k.clone()).map(|(i, mat)| Ok((i, mat.dump_infos(infos)?))).collect::<Result<HashMap<_,_>>>().context("mats")?;
        let mat_order = T::u32::mut_slice_from_data(dst, self.mat_num()).context("mat_order")?;
        info.mat_num = (mat_order.len() as u32).into();
        let mut j = 0;
        for (i, mat) in self.mat_order().zip(mat_order) {
            *mat = (*mat_map.get(&i).ok_or(anyhow!("mats is missing item {}", i))?).into(); 
            *infos.offsets.next().context("offsets")? = (off as u32).into();
            off += 4;
            j += 1;
        }
        assert!(j == self.mat_num());
        Ok(())
    }

    fn add_mat_counts(&self, offset: usize, counts: &mut InfoCounts) -> usize {
        for mat in self.mats() {
            mat.1.add_counts(counts);
        }
        counts.offsets += self.mat_num();
        offset + self.mat_num() * std::mem::size_of::<T::u32>()
    }

    fn dump_into<'a>(&'a self, dst: &mut DumpSlice, infos: &mut DumpInfos<'_, 'a, T>) -> Result<()> {
        let info_off = infos.models.offset;
        let info = infos.models.next().context("models")?;
        self.write_info(info);
        
        info.bones_offset = (dst.offset as u32).into();
        let bones = Crc::mut_slice_from_data(dst, self.bone_num()).context("bones")?;
        info.bones_num = (bones.len() as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, bones_offset)) as u32).into();

        dst.align(16)?;
        info.bone_bounding_boxes_offset = (dst.offset as u32).into();
        let bone_bboxes = BoundingBox::mut_slice_from_data(dst, self.bone_num()).context("bone_bounding_boxes")?;

        info.vals_j_offset = (dst.offset as u32).into();
        let vals_j = T::u32::mut_slice_from_data(dst, self.vals_j_num()).context("vals_j")?;
        info.vals_j_num = (vals_j.len() as u32).into();
        self.write_vals_j(vals_j).context("write vals_j")?;
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, vals_j_offset)) as u32).into();

        if let Some(hk_constraint) = self.hk_constraint() {
            info.hk_constraint_offset = (infos.hk_constraints.offset as u32).into();
            hk_constraint.dump_into(dst, infos, info.bones_num.into() as u16, info.bones_offset.into()).context("hk_constraint")?;
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, hk_constraint_offset)) as u32).into();
        } else {
            info.hk_constraint_offset = 0u32.into();
        }

        dst.align(16)?;
        info.skin_binds_offset = (dst.offset as u32).into();
        let skin_binds = Matrix4x4::mut_slice_from_data(dst, self.skin_bind_num()).context("skin_binds")?;
        info.skin_binds_num = (skin_binds.len() as u32).into();
        self.write_skin_binds(skin_binds).context("write skin_binds")?;

        if self.has_skin_order() {
            info.skin_order_offset = (dst.offset as u32).into();
            let skin_order = T::u32::mut_slice_from_data(dst, skin_binds.len()).context("skin_order")?;
            self.write_skin_order(skin_order).context("write skin_order")?;
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, skin_order_offset)) as u32).into();
        } else {
            info.skin_order_offset = 0u32.into();
        }

        info.bone_parents_offset = (dst.offset as u32).into();
        let bone_parents = T::i32::mut_slice_from_data(dst, bones.len()).context("bone_parents")?;

        dst.align(16)?;
        info.bone_transforms_offset = (dst.offset as u32).into();
        let bone_transforms = Matrix4x4::mut_slice_from_data(dst, bones.len()).context("bone_transforms")?;
        self.write_bones(BonesDump {
            names: bones,
            parents: bone_parents,
            bounding_boxes: bone_bboxes,
            transforms: bone_transforms,
        }).context("write bones")?;

        self.dump_mats(dst, infos, info)?;

        info.shape_num = (self.shape_num() as u32).into();
        if info.shape_num.into() != 0 {
            info.shape_offset = (infos.shapes.offset as u32).into();
            for (i, shape) in self.shapes().enumerate() {
                shape.dump_into(dst, infos, None).with_context(|| format!("shape {}", i))?;
            }
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, shape_offset)) as u32).into();
        } else {
            info.shape_offset = 0u32.into();
        }

        info.mesh_order_offset = (dst.offset as u32).into();
        let mesh_order = T::u32::mut_slice_from_data(dst, self.mesh_order_num()).context("mesh_order")?;
        // TODO need to rework how lod information is stored / presented
        // info.lod3.breakable_end = mesh_order.len();
        self.write_mesh_order(mesh_order).context("write mesh_order")?;

        dst.align(16)?;
        info.mesh_bounding_boxes_offset = (dst.offset as u32).into();
        let mesh_bboxes = BoundingBox::mut_slice_from_data(dst, mesh_order.len()).context("mesh_bounding_boxes")?;
        self.write_mesh_bounding_boxes(mesh_bboxes).context("write mesh_bounding_boxes")?;

        let data = self.data().dump_into(dst, infos, info)?;
        let (asset_key, asset_type) = self.asset_info();
        infos.model_data.push(DumpInfoData {
            key: asset_key,
            kind: asset_type,
            data
        });

        self.dump_misc(dst, infos, info, info_off)?;
        Ok(())
    }

    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        counts.models += 1;
        
        offset += std::mem::size_of::<Crc<T>>() * self.bone_num();
        offset = align_offset(offset, 16) + std::mem::size_of::<BoundingBox<T>>() * self.bone_num();

        offset += std::mem::size_of::<T::u32>() * self.vals_j_num();

        counts.offsets += 2;

        if let Some(hk_constraint) = self.hk_constraint() {
            offset = hk_constraint.add_size(offset, counts);
            counts.offsets += 1;
        }

        offset = align_offset(offset, 16) + std::mem::size_of::<Matrix4x4<T>>() * self.skin_bind_num();

        if self.has_skin_order() {
            offset += std::mem::size_of::<T::u32>() * self.skin_bind_num();
            counts.offsets += 1;
        }

        offset += std::mem::size_of::<T::i32>() * self.bone_num();
        offset = align_offset(offset, 16) + std::mem::size_of::<Matrix4x4<T>>() * self.bone_num();

        offset = self.add_mat_counts(offset, counts);

        let mut has_shape = false;
        for shape in self.shapes() {
            offset = shape.add_size(offset, counts);
            has_shape = true;
        }
        if has_shape {
            counts.offsets += 1;
        }

        offset += std::mem::size_of::<T::u32>() * self.mesh_order_num();
        offset = align_offset(offset, 16) + std::mem::size_of::<BoundingBox<T>>() * self.mesh_order_num();


        offset = self.data().add_size(offset, counts).0;

        offset = self.add_misc_size(offset, counts);
        offset
    }

    fn dump_terrain_into<'a>(&'a self, dst: &mut DumpSlice, infos: &mut DumpInfos<'_, 'a, T>, indices_offset: usize) -> Result<()> {
        let bone_bbox_offset = infos.models.offset + 16;
        let info_off = infos.models.offset;
        let info = infos.models.next().context("models")?;
        self.write_info(info);

        info.bones_offset = 0u32.into();
        info.bones_num = (self.bone_num() as u32).into();
        info.bone_bounding_boxes_offset = (bone_bbox_offset as u32).into();
        info.skin_binds_offset = (indices_offset as u32).into();
        info.skin_binds_num = 0u32.into();
        info.skin_order_offset = 0u32.into();
        info.vals_j_offset = 0u32.into();
        info.bone_parents_offset = (indices_offset as u32).into();

        if let Some(hk_constraint) = self.hk_constraint() {
            info.hk_constraint_offset = (infos.hk_constraints.offset as u32).into();
            hk_constraint.dump_into(dst, infos, info.bones_num.into() as u16, info.bones_offset.into()).context("hk_constraint")?;
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, hk_constraint_offset)) as u32).into();
        } else {
            info.hk_constraint_offset = 0u32.into();
        }

        let mut shape_offsets = Vec::with_capacity(self.shape_num());
        for (i, shape) in self.shapes().enumerate() {
            shape_offsets.push(shape.dump_extra_into(dst).with_context(|| format!("shape {} extra", i))?);
        }
        
        dst.align(16)?;
        info.mesh_bounding_boxes_offset = (dst.offset as u32).into();
        let mesh_bboxes = BoundingBox::mut_slice_from_data(dst, self.mesh_order_num()).context("mesh_bounding_boxes")?;
        self.write_mesh_bounding_boxes(mesh_bboxes).context("write mesh_bounding_boxes")?;

        info.bone_transforms_offset = (dst.offset as u32).into();
        let bone_transforms = Matrix4x4::mut_slice_from_data(dst, self.bone_num()).context("bone_transforms")?;
        self.write_bones(BonesDump {
            names: &mut [],
            parents: &mut [],
            bounding_boxes: &mut [],
            transforms: bone_transforms,
        }).context("write bones")?;

        self.dump_mats(dst, infos, info)?;

        info.mesh_order_offset = (dst.offset as u32).into();
        let mesh_order = T::u32::mut_slice_from_data(dst, self.mesh_order_num()).context("mesh_order")?;
        // TODO need to rework how lod information is stored / presented
        // info.lod3.breakable_end = mesh_order.len();
        self.write_mesh_order(mesh_order).context("write mesh_order")?;

        let data = self.data().dump_into(dst, infos, info)?;
        let (asset_key, asset_type) = self.asset_info();
        infos.model_data.push(DumpInfoData {
            key: asset_key,
            kind: asset_type,
            data
        });
        let off_dest = info.ibuff_offset.into() as usize + 320;

        self.dump_misc(dst, infos, info, info_off)?;

        if dst.offset < off_dest {
            dst.split(off_dest - dst.offset).context("model pad")?;
            // *dst = dst.split(off_dest - dst.offset)?;
        }

        info.shape_num = (self.shape_num() as u32).into();
        if info.shape_num .into()!= 0 {
            info.shape_offset = (infos.shapes.offset as u32).into();
            for ((i, shape), off) in self.shapes().enumerate().zip(shape_offsets) {
                shape.dump_into(dst, infos, off).with_context(|| format!("shape {}", i))?;
            }
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ModelInfo<T>, shape_offset)) as u32).into();
        } else {
            info.shape_offset = 0u32.into();
        }
        Ok(())
    }

    fn add_terrain_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        counts.models += 1;

        if let Some(hk_constraint) = self.hk_constraint() {
            offset = hk_constraint.add_size(offset, counts);
            counts.offsets += 1;
        }

        let mut has_shape = false;
        for shape in self.shapes() {
            offset = shape.add_extra_size(offset);
            has_shape = true;
        }
        if has_shape {
            counts.offsets += 1;
        }
        
        offset = align_offset(offset, 16) + std::mem::size_of::<BoundingBox<T>>() * self.mesh_order_num();
        offset += std::mem::size_of::<Matrix4x4<T>>() * self.bone_num();

        offset = self.add_mat_counts(offset, counts);

        offset += std::mem::size_of::<T::u32>() * self.mesh_order_num();

        let (mut offset, off_dest) = self.data().add_size(offset, counts);


        offset = self.add_misc_size(offset, counts);

        offset = (off_dest + 320).max(offset);

        for shape in self.shapes() {
            offset = shape.add_size(offset, counts);
        }
        offset
    }
}

impl<'a, T: BaseTypes> DumpModel<T> for ModelRef<'a, T> {
    fn info(&self) -> &ModelInfo<T> {
        self.info
    }
    fn key(&self) -> u32 {
        self.info.key.val.into()
    }
    fn bone_num(&self) -> usize {
        self.bones.len()
    }
    fn vals_j_num(&self) -> usize {
        self.vals_j.len()
    }
    fn skin_bind_num(&self) -> usize {
        self.skin_binds.len()
    }
    fn has_skin_order(&self) -> bool {
        !self.skin_order.is_empty()
    }
    fn mat_num(&self) -> usize {
        self.mat_order.len()
    }
    fn shape_num(&self) -> usize {
        self.shapes.len()
    }
    fn mesh_order_num(&self) -> usize {
        self.mesh_order.len()
    }
    fn buffer_num(&self) -> usize {
        self.buffer_infos.len()
    }
    fn vbuff_num(&self) -> usize {
        self.vbuff_order.len()
    }
    fn ibuff_num(&self) -> usize {
        self.ibuff_order.len()
    }
    fn has_vals_k(&self) -> bool {
        !self.val_k_header.is_empty()
    }
    fn slot_num(&self) -> usize {
        self.slots.len()
    }
    fn slot_map_num(&self) -> usize {
        self.slot_map.len()
    }
    fn block_num(&self) -> usize {
        self.blocks.len()
    }
    fn hk_constraint_data_num(&self) -> usize {
        self.hk_constraint_datas.len()
    }
    fn write_bones(&self, BonesDump { names, parents, transforms, bounding_boxes }: BonesDump<T>) -> Result<()> {
        if names.len() != 0 {
            names.copy_from_slice(&self.bones.names[..]);
            parents.copy_from_slice(&self.bones.parents[..]);
            bounding_boxes.copy_from_slice(&self.bones.bounding_boxes[..]);
        }
        transforms.copy_from_slice(&self.bones.transforms[..]);
        Ok(())
    }
    fn write_vals_j(&self, vals_j: &mut [T::u32]) -> Result<()> {
        vals_j.copy_from_slice(&self.vals_j[..]);
        Ok(())
    }
    fn write_skin_binds(&self, skin_binds: &mut [Matrix4x4<T>]) -> Result<()> {
        skin_binds.copy_from_slice(&self.skin_binds[..]);
        Ok(())
    }
    fn write_skin_order(&self, skin_order: &mut [T::u32]) -> Result<()> {
        skin_order.copy_from_slice(&self.skin_order[..]);
        Ok(())
    }
    fn write_mesh_order(&self, mesh_order: &mut [T::u32]) -> Result<()> {
        mesh_order.copy_from_slice(&self.mesh_order[..]);
        Ok(())
    }
    fn write_mesh_bounding_boxes(&self, mesh_bboxes: &mut [BoundingBox<T>]) -> Result<()> {
        mesh_bboxes.copy_from_slice(&self.mesh_bounding_boxes[..]);
        Ok(())
    }
    fn vbuff_order(&self) -> impl Iterator<Item=u32> {
        self.vbuff_order.iter().map(|&x| x.into())
    }
    fn ibuff_order(&self) -> impl Iterator<Item=u32> {
        self.ibuff_order.iter().map(|&x| x.into())
    }
    fn write_buffer_infos(&self, buffer_infos: &mut [BufferInfo<T>]) -> Result<()> {
        buffer_infos.copy_from_slice(&self.buffer_infos[..]);
        Ok(())
    }
    fn write_vbuff_infos(&self, vbuff_infos: &mut [VBuffInfo<T>]) -> Result<()> {
        for (&&src, dst) in self.vbuffs.values().zip(vbuff_infos) {
            *dst = src;
        }
        Ok(())
    }
    fn write_ibuff_infos(&self, ibuff_infos: &mut [IBuffInfo<T>]) -> Result<()> {
        for (&&src, dst) in self.ibuffs.values().zip(ibuff_infos) {
            *dst = src;
        }
        Ok(())
    }
    fn write_vals_k(&self, header: &mut [T::u16], vals_k: &mut [T::u32]) -> Result<()>  {
        header.copy_from_slice(&self.val_k_header[..]);
        vals_k.copy_from_slice(&self.vals_k[..]);
        Ok(())

    }
    fn write_slots(&self, slots: &mut [Key2<T>]) -> Result<()> {
        slots.copy_from_slice(&self.slots[..]);
        Ok(())

    }
    fn write_slot_map(&self, slot_map: &mut [T::u32]) -> Result<()> {
        slot_map.copy_from_slice(&self.slot_map[..]);
        Ok(())

    }
    fn write_hk_constraint_datas(&self, hk_constraint_datas: &mut [HkConstraintData<T>]) -> Result<()> {
        hk_constraint_datas.copy_from_slice(&self.hk_constraint_datas[..]);
        Ok(())

    }
    fn mat_order(&self) -> impl Iterator<Item=u32> {
        self.mat_order.iter().map(|&x| x.into())
    }
    fn mats(&self) -> impl Iterator<Item=(u32, &impl DumpMat<T>)> {
        self.mats.iter().map(|(k, v)| (*k, v))
    }
    fn shapes(&self) -> impl Iterator<Item=&impl DumpShape<T>> {
        self.shapes.iter()
    }
    fn hk_constraint(&self) -> Option<&impl DumpHkConstraint<T>> {
        self.hk_constraint.as_ref()
    }
    fn block_header(&self) -> Option<impl Into<T::u32>> {
        self.block_header.cloned()
    }
    fn blocks(&self) -> impl Iterator<Item=&impl DumpBlock<T>> {
        self.blocks.iter()
    }
    fn data(&self) -> &impl DumpModelData<T> {
        &self.data
    }
    fn asset_info(&self) -> (T::u32, T::u32) {
        (self.info.asset_key.val, self.info.asset_type)
    }
    fn write_info(&self, info: &mut ModelInfo<T>) {
        *info = self.info.clone();
    }
}
