use anyhow::{anyhow, Context, Result};
use log::debug;
use itertools::Itertools;
use indexmap::IndexMap;
use std::collections::HashMap;

use crate::{
    types::{Crc, DumpData, Matrix4x4, RefFromData, Vector3, DumpSlice, align_offset, OrderedData, ref_slice, CompressedDataRef, slice}, 
    level::{
        LevelPc,
        model::data::ModelData,
        pak::block1::infos::InfoCounts,
    },
};
#[make_endian]
use crate::{
    level::{
        model::{
            mat::{DumpMat_XE_, MatRef_XE_},
            shape::{DumpShape_XE_, DumpHkConstraint_XE_, ShapeInfo_XE_, ShapeRef_XE_, HkConstraintData_XE_, HkConstraintRef_XE_},
            data::{BufferInfo_XE_, IBuffInfo_XE_, VBuffInfo_XE_, ModelDataRef_XE_, DumpModelData_XE_},
        },
        pak::{
            block1::infos::{DumpInfos_XE_, DumpInfoData_XE_},
        },
    },
    types::{Crc_XE_, Matrix4x4_XE_, i32_XE_, u16_XE_, u32_XE_, Vector3_XE_, f32_XE_},
};
use lotrc_proc::{make_endian, derive_ordered_data};

pub mod data;
pub mod mat;
pub mod shape;

use data::DataTypes;
use mat::MatTypes;
use shape::ShapeTypes;

pub trait ModelTypes: where Self: DataTypes + MatTypes + ShapeTypes {
    type BoundingBox: PartialEq + RefFromData + DumpData + BoundingBoxTypeTrait;
    type BlockHeader1: PartialEq + RefFromData + DumpData + BlockHeader1TypeTrait;
    type BlockHeader2: PartialEq + RefFromData + DumpData + BlockHeader2TypeTrait;
    type BlockValA: PartialEq + RefFromData + DumpData + BlockValATypeTrait;
    type BlockValB: PartialEq + RefFromData + DumpData + BlockValBTypeTrait;
}

impl ModelTypes for LevelPc {
    type BoundingBox = BoundingBoxLE;
    type BlockHeader1 = BlockHeader1LE;
    type BlockHeader2 = BlockHeader2LE;
    type BlockValA = BlockValALE;
    type BlockValB = BlockValBLE;
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LodInfo_XE_ {
    pub start: u32_XE_,
    pub static_end: u32_XE_,
    pub skinned_end: u32_XE_,
    pub physics_end: u32_XE_,
    pub breakable_end: u32_XE_,
}

impl LodInfo {
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

#[derive_ordered_data]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BoundingBox_XE_ {
    center: Vector3_XE_,
    unk_3: f32_XE_,
    half_width: Vector3_XE_,
    unk_7: f32_XE_,
}

#[make_endian]
pub struct ModelInfoPS3_XE_ {
    pub key: Crc_XE_,
    pub gamemodemask: i32_XE_,
    pub mat_offset: u32_XE_,
    pub buffer_info_offset: u32_XE_, // pointer to obj2, uses mat_num of sequential objects
    pub bounding_box: BoundingBox_XE_, // (center x, y, z, ?, half_width x, y, z, ?) default vals of 1.0 for the ? vals seems to work
    pub mesh_order_offset: u32_XE_, // ints (c & 0x3fffffff is an index into the obj2s referenced by this object) (1 for each mesh)
    pub lod0: LodInfo_XE_,
    pub lod1: LodInfo_XE_,
    pub lod2: LodInfo_XE_,
    pub lod3: LodInfo_XE_,
    pub mat_num: u32_XE_,
    pub bones_offset: u32_XE_, // ints
    pub bone_parents_offset: u32_XE_,
    pub bone_transforms_offset: u32_XE_, // 16 ints (matrix?) for keys_num
    pub bones_num: u32_XE_,
    pub skin_binds_offset: u32_XE_,
    pub skin_binds_num: u32_XE_,
    pub skin_order_offset: u32_XE_,
    pub vbuff_offset: u32_XE_,
    pub vbuff_num: u32_XE_,
    pub ibuff_offset: u32_XE_,
    pub ibuff_num: u32_XE_,
    pub mesh_bounding_boxes_offset: u32_XE_,
    pub unk_46: f32_XE_,           // maybe something to do with size?
    pub variation_counts: u32_XE_, // maybe something to do with variation
    pub vals_j_num: u32_XE_,
    pub vals_j_offset: u32_XE_,
    pub block_offset: u32_XE_,
    pub vals_k_offset: u32_XE_, // not sure on the size, seems to be 36 ints
    pub asset_key: Crc_XE_,     // data in bin that is vertex & index buffer values
    pub asset_type: u32_XE_,
    pub unk_54: u32_XE_, // 1 for occuluder otherwise 0 ??
    pub shape_offset: u32_XE_, 
    pub shape_num: u32_XE_,
    pub hk_constraint_data_offset: u32_XE_,
    pub hk_constraint_data_num: u32_XE_, 
    pub hk_constraint_offset: u32_XE_,
    pub slots_offset: u32_XE_, 
    pub slot_map_offset: u32_XE_,
    pub bone_bounding_boxes_offset: u32_XE_,
    pub unk_55: u32_XE_, 
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ModelInfo_XE_ {
    pub key: Crc_XE_,
    pub gamemodemask: i32_XE_,
    pub mat_offset: u32_XE_,
    pub buffer_info_offset: u32_XE_, // pointer to obj2, uses mat_num of sequential objects
    pub bounding_box: BoundingBox_XE_, // (center x, y, z, ?, half_width x, y, z, ?) default vals of 1.0 for the ? vals seems to work
    pub mesh_order_offset: u32_XE_, // ints (c & 0x3fffffff is an index into the obj2s referenced by this object) (1 for each mesh)
    pub lod0: LodInfo_XE_,
    pub lod1: LodInfo_XE_,
    pub lod2: LodInfo_XE_,
    pub lod3: LodInfo_XE_,
    pub mat_num: u32_XE_,
    pub bones_offset: u32_XE_, // ints
    pub bone_parents_offset: u32_XE_,
    pub bone_transforms_offset: u32_XE_, // 16 ints (matrix?) for keys_num
    pub bones_num: u32_XE_,
    pub skin_binds_offset: u32_XE_,
    pub skin_binds_num: u32_XE_,
    pub skin_order_offset: u32_XE_,
    pub vbuff_offset: u32_XE_,
    pub vbuff_num: u32_XE_,
    pub ibuff_offset: u32_XE_,
    pub ibuff_num: u32_XE_,
    pub mesh_bounding_boxes_offset: u32_XE_,
    pub unk_46: f32_XE_,           // maybe something to do with size?
    pub variation_counts: u32_XE_, // maybe something to do with variation
    pub vals_j_num: u32_XE_,
    pub vals_j_offset: u32_XE_,
    pub block_offset: u32_XE_,
    pub vals_k_offset: u32_XE_, // not sure on the size, seems to be 36 ints
    pub asset_key: Crc_XE_,     // data in bin that is vertex & index buffer values
    pub asset_type: u32_XE_,
    pub unk_54: u32_XE_, // 1 for occuluder otherwise 0 ??
    pub unk_55: u32_XE_, // always 0 ??
    pub shape_offset: u32_XE_,
    pub shape_num: u32_XE_,
    pub hk_constraint_data_offset: u32_XE_, // optional pointer to obje
    pub hk_constraint_data_num: u32_XE_,
    pub hk_constraint_offset: u32_XE_, // optional pointer to hkConstraint
    pub slots_offset: u32_XE_,
    pub slot_map_offset: u32_XE_,
    pub bone_bounding_boxes_offset: u32_XE_, // 8 ints
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockHeader_XE_ {
    pub a: u32_XE_,
    pub b: u32_XE_,
    pub unk_2: u32_XE_,
    pub unk_3: u32_XE_,
    pub unk_4: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockVal_XE_ {
    pub unk_0: u32_XE_,
    pub unk_1: u32_XE_,
    pub unk_2: u32_XE_,
    pub unk_3: u32_XE_,
    pub unk_4: u16_XE_,
    pub unk_5: u16_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockHeader1_XE_ {
    pub a: u32_XE_,
    pub b: u32_XE_,
    pub unk_2: u32_XE_,
    pub unk_3: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockHeader2_XE_ {
    pub n: u32_XE_,
    pub unk_1: f32_XE_,
    pub unk_2: f32_XE_,
    pub unk_3: u32_XE_,
    pub unk_4: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockValA_XE_ {
    pub unk_0: f32_XE_,
    pub unk_1: f32_XE_,
    pub unk_2: f32_XE_,
    pub unk_3: f32_XE_,
    pub unk_4: f32_XE_,
    pub unk_5: f32_XE_,
    pub unk_6: f32_XE_,
    pub unk_7: f32_XE_,
    pub unk_8: f32_XE_,
    pub unk_9: u32_XE_,
    pub unk_10: u32_XE_,
    pub unk_11: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockValB_XE_ {
    pub unk_0: u16_XE_,
    pub unk_1: u16_XE_,
    pub unk_2: f32_XE_,
    pub unk_3: f32_XE_,
    pub unk_4: f32_XE_,
    pub unk_5: f32_XE_,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct BlockRef<'a, T: ModelTypes> {
    info1: &'a T::BlockHeader1,
    info2: &'a T::BlockHeader2,
    vals_a: ref_slice<'a, T::BlockValA>,
    vals_b: ref_slice<'a, T::BlockValA>,
    vals_c: ref_slice<'a, T::BlockValB>,
    pad: ref_slice<'a, u8>,
}

impl<'a, T: ModelTypes> BlockRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let info1 = T::BlockHeader1::from_data(&src[offset..]).context("info1")?;
        offset += info1.size_of_val();
        let vals_a = T::BlockValA::slice_from_data(&src[offset..], info1.a() as usize)
            .context("vals_a")?;
        offset += vals_a.size_of_val();
        let vals_b = T::BlockValA::slice_from_data(&src[offset..], info1.b() as usize)
            .context("vals_b")?;
        offset += vals_b.size_of_val();
        let info2 = T::BlockHeader2::from_data(&src[offset..]).context("info2")?;
        offset += info2.size_of_val();
        let vals_c = T::BlockValB::slice_from_data(&src[offset..], info2.n() as usize)
            .context("vals_c")?;
        offset += vals_c.size_of_val();
        let pad = &src[offset..];
        Ok(Self { info1, info2, vals_a, vals_b, vals_c, pad })
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct BlockRef_XE_<'a> {
    info1: &'a BlockHeader1_XE_,
    info2: &'a BlockHeader2_XE_,
    vals_a: ref_slice<'a, BlockValA_XE_>,
    vals_b: ref_slice<'a, BlockValA_XE_>,
    vals_c: ref_slice<'a, BlockValB_XE_>,
    pad: ref_slice<'a, u8>,
}

#[make_endian]
impl<'a> BlockRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let info1 = BlockHeader1_XE_::from_data(&src[offset..]).context("info1")?;
        offset += info1.size_of_val();
        let vals_a = BlockValA_XE_::slice_from_data(&src[offset..], info1.a.conv())
            .context("vals_a")?;
        offset += vals_a.size_of_val();
        let vals_b = BlockValA_XE_::slice_from_data(&src[offset..], info1.b.conv())
            .context("vals_b")?;
        offset += vals_b.size_of_val();
        let info2 = BlockHeader2_XE_::from_data(&src[offset..]).context("info2")?;
        offset += info2.size_of_val();
        let vals_c = BlockValB_XE_::slice_from_data(&src[offset..], info2.n.conv())
            .context("vals_c")?;
        offset += vals_c.size_of_val();
        let pad = &src[offset..];
        Ok(Self { info1, info2, vals_a, vals_b, vals_c, pad })
    }
}

#[derive(Default, Debug, Clone)]
pub struct Block {
    pub info1: BlockHeader1,
    pub info2: BlockHeader2,
    pub vals_a: Vec<BlockValA>,
    pub vals_b: Vec<BlockValA>,
    pub vals_c: Vec<BlockValB>,
    pub pad: Vec<u8>,
}

#[make_endian]
impl From<&BlockRef_XE_<'_>> for Block {
    fn from(val: &BlockRef_XE_) -> Self {
        Self {
            info1: val.info1.conv(),
            info2: val.info2.conv(),
            vals_a: val.vals_a.iter().map(|x| x.conv()).collect(),
            vals_b: val.vals_b.iter().map(|x| x.conv()).collect(),
            vals_c: val.vals_c.iter().map(|x| x.conv()).collect(),
            pad: val.pad.to_vec(),
        }
    }
}

#[make_endian]
pub trait DumpBlock_XE_ {
    fn vals_a_num(&self) -> usize;
    fn vals_b_num(&self) -> usize;
    fn vals_c_num(&self) -> usize;
    fn pad_num(&self) -> usize;
    fn write_info1(&self, info1: &mut BlockHeader1_XE_) -> Result<()>;
    fn write_info2(&self, info2: &mut BlockHeader2_XE_) -> Result<()>;
    fn write_vals_a(&self, vals_a: &mut [BlockValA_XE_]) -> Result<()>;
    fn write_vals_b(&self, vals_b: &mut [BlockValA_XE_]) -> Result<()>;
    fn write_vals_c(&self, vals_c: &mut [BlockValB_XE_]) -> Result<()>;
    fn write_pad(&self, pad: &mut [u8]) -> Result<()>;

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let info1 = BlockHeader1_XE_::mut_from_data(dst).context("info1")?;
        self.write_info1(info1).context("write info1")?;

        let vals_a = BlockValA_XE_::mut_slice_from_data(dst, self.vals_a_num()).context("vals_a")?;
        self.write_vals_a(vals_a).context("write vals_a")?;

        let vals_b = BlockValA_XE_::mut_slice_from_data(dst, self.vals_b_num()).context("vals_b")?;
        self.write_vals_b(vals_b).context("write vals_b")?;

        let info2 = BlockHeader2_XE_::mut_from_data(dst).context("info2")?;
        self.write_info2(info2).context("write info2")?;

        let vals_c = BlockValB_XE_::mut_slice_from_data(dst, self.vals_c_num()).context("vals_c")?;
        self.write_vals_c(vals_c).context("write vals_c")?;

        let pad = u8::mut_slice_from_data(dst, self.pad_num()).context("pad")?;
        self.write_pad(pad).context("write pad")?;
        Ok(())
    }
    fn add_size(&self, offset: usize) -> usize {
        offset 
            + BlockHeader1_XE_::size_of()
            + BlockValA_XE_::size_of() * self.vals_a_num() 
            + BlockValA_XE_::size_of() * self.vals_b_num()
            + BlockHeader2_XE_::size_of()
            + BlockValB_XE_::size_of() * self.vals_c_num()
            + self.pad_num()
    }
}

#[make_endian]
impl DumpBlock_XE_ for BlockRef_XE_<'_> {
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
    fn write_info1(&self, info1: &mut BlockHeader1_XE_) -> Result<()> {
        info1.write_from(self.info1)
    }
    fn write_info2(&self, info2: &mut BlockHeader2_XE_) -> Result<()> {
        info2.write_from(self.info2)
    }
    fn write_vals_a(&self, vals_a: &mut [BlockValA_XE_]) -> Result<()> {
        vals_a.write_from(&self.vals_a[..])
    }
    fn write_vals_b(&self, vals_b: &mut [BlockValA_XE_]) -> Result<()> {
        vals_b.write_from(&self.vals_b[..])
    }
    fn write_vals_c(&self, vals_c: &mut [BlockValB_XE_]) -> Result<()> {
        vals_c.write_from(&self.vals_c[..])
    }
    fn write_pad(&self, pad: &mut [u8]) -> Result<()> {
        pad.write_from(&self.pad[..])
    }

}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Key2_XE_ {
    pub key: Crc_XE_,
    pub val: u32_XE_,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct BonesRef<'a, T: ModelTypes> {
    pub names:  ref_slice<'a, T::Crc>,
    pub parents: ref_slice<'a, T::i32>,
    pub transforms: ref_slice<'a, T::Matrix4x4>,
    pub bounding_boxes: ref_slice<'a, T::BoundingBox>,
}

impl<'a, T: ModelTypes> BonesRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a T::ModelInfo) -> Result<Self> {
        let size = info.bones_num() as usize;
        let parents = T::i32::slice_from_data(
            &src[info.bone_parents_offset() as usize..],
            size,
        ).context("parents")?;
        if parents[0].conv() != -1 {
            return Err(anyhow!("first bone should be the root, but parent != -1"));
        }
        let names_off = info.bones_offset() as usize;
        
        let (names, bounding_boxes) = if names_off != 0 {
            (
                T::Crc::slice_from_data(&src[names_off..], size).context("names")?,
                T::BoundingBox::slice_from_data(
                    &src[info.bone_bounding_boxes_offset() as usize..],
                    info.bones_num() as usize,
                ).context("bounding_boxes")?
            )
        } else {
            (&[] as _, &[] as _)
        };
        let transforms = T::Matrix4x4::slice_from_data(
            &src[info.bone_transforms_offset() as usize..],
            size,
        ).context("transforms")?;
        Ok(Self { names, parents, transforms, bounding_boxes })
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.transforms.len()
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct BonesRef_XE_<'a> {
    pub names:  ref_slice<'a, Crc_XE_>,
    pub parents: ref_slice<'a, i32_XE_>,
    pub transforms: ref_slice<'a, Matrix4x4_XE_>,
    pub bounding_boxes: ref_slice<'a, BoundingBox_XE_>,
}

#[make_endian]
impl<'a> BonesRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a ModelInfo_XE_) -> Result<Self> {
        let size: usize = info.bones_num.conv();
        let parents = i32_XE_::slice_from_data(
            &src[info.bone_parents_offset.conv()..],
            size,
        ).context("parents")?;
        if parents[0] != -1 {
            return Err(anyhow!("first bone should be the root, but parent != -1"));
        }
        let names_off: usize = info.bones_offset.conv();
        
        let (names, bounding_boxes) = if names_off != 0 {
            (
                u32_XE_::slice_from_data(&src[names_off..], size).context("names")?,
                BoundingBox_XE_::slice_from_data(
                    &src[info.bone_bounding_boxes_offset.conv()..],
                    info.bones_num.conv(),
                ).context("bounding_boxes")?
            )
        } else {
            (&[] as _, &[] as _)
        };
        let transforms = Matrix4x4_XE_::slice_from_data(
            &src[info.bone_transforms_offset.conv()..],
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
    pub names: Vec<Crc>,
    pub parents: Vec<i32>,
    pub transforms: Vec<Matrix4x4>,
    pub bounding_boxes: Vec<BoundingBox>
}

#[make_endian]
impl From<&BonesRef_XE_<'_>> for Bones {
    fn from(val: &BonesRef_XE_<'_>) -> Self {
        Self {
            names: val.names.iter().map(|x| x.conv()).collect(),
            parents: val.parents.iter().map(|x| x.conv()).collect(),
            transforms: val.transforms.iter().map(|x| x.conv()).collect(),
            bounding_boxes: val.bounding_boxes.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_endian]
pub struct BonesDump_XE_<'a> {
    names:  &'a mut [Crc_XE_],
    parents: &'a mut [i32_XE_],
    transforms: &'a mut [Matrix4x4_XE_],
    bounding_boxes: &'a mut [BoundingBox_XE_],
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ModelRef<'a, T: ModelTypes> {
    pub info: &'a T::ModelInfo,
    pub bones: BonesRef<'a, T>,
    pub mat_order: ref_slice<'a, T::u32>,
    pub mesh_order: ref_slice<'a, T::u32>,
    pub mesh_bounding_boxes: ref_slice<'a, T::BoundingBox>,
    pub skin_binds: ref_slice<'a, T::Matrix4x4>,
    pub vals_j: ref_slice<'a, T::u32>,
    pub val_k_header: ref_slice<'a, T::u16>,
    pub vals_k: ref_slice<'a, T::u32>,
    pub skin_order: ref_slice<'a, T::u32>,
    pub slots: ref_slice<'a, T::Key2>,
    pub slot_map: ref_slice<'a, T::u32>,
    pub block_header: Option<&'a T::u32>,
    pub block_offsets: ref_slice<'a, T::u32>,
    pub blocks: slice<BlockRef<'a, T>>,
    pub buffer_infos: ref_slice<'a, T::BufferInfo>,
    pub vbuff_order: ref_slice<'a, T::u32>,
    pub ibuff_order: ref_slice<'a, T::u32>,
    pub vbuffs: IndexMap<u32, &'a T::VBuffInfo>,
    pub ibuffs: IndexMap<u32, &'a T::IBuffInfo>,
    pub mats: IndexMap<u32, mat::MatRef<'a, T>>,
    pub hk_constraint: Option<shape::HkConstraintRef<'a, T>>, // stores bone transforms used for ragdoll
    pub hk_constraint_datas: ref_slice<'a, T::HkConstraintData>,
    pub shapes: slice<shape::ShapeRef<'a, T>>,
    pub data: data::ModelDataRef<'a, T>,
}

impl<'a, T: ModelTypes> ModelRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a T::ModelInfo, model_data: &IndexMap<u32, &'a CompressedDataRef<'a>>) -> Result<Self> {
        let bones = BonesRef::from_data(src, info).context("bones")?;
        let mat_order = T::u32::slice_from_data(
            &src[info.mat_offset() as usize..],
            info.mat_num() as usize,
        )
        .context("mat order")?;
        let mesh_order = T::u32::slice_from_data(
            &src[info.mesh_order_offset() as usize..],
            info.lod3().breakable_end() as usize,
        )
        .context("mesh order")?;
        let mesh_bounding_boxes = T::BoundingBox::slice_from_data(
            &src[info.mesh_bounding_boxes_offset() as usize..],
            info.lod3().breakable_end() as usize,
        )
        .context("mesh bounding boxes")?;
        let skin_binds = T::Matrix4x4::slice_from_data(
            &src[info.skin_binds_offset() as usize..],
            info.skin_binds_num() as usize,
        )
        .context("skin binds")?;
        let vals_j = T::u32::slice_from_data(
            &src[info.vals_j_offset() as usize..],
            info.vals_j_num() as usize,
        )
        .context("vals j")?;
        let (val_k_header, vals_k) = if info.vals_k_offset() != 0 {
            (
                T::u16::slice_from_data(&src[info.vals_k_offset() as usize..], 2)
                    .context("vals k header")?,
                T::u32::slice_from_data(&src[info.vals_k_offset() as usize + 4..], 35)
                    .context("vals k")?,
            )
        } else {
            (&[] as _, &[] as _)
        };
        let skin_order = if info.skin_order_offset() != 0 {
            T::u32::slice_from_data(
                &src[info.skin_order_offset() as usize..],
                info.skin_binds_num() as usize,
            )
            .context("skin order")?
        } else {
            &[] as _
        };
        let (slots, slot_map) = if info.slots_offset() != 0 {
            if info.slot_map_offset() == 0 {
                return Err(anyhow!("expected non zero slot_map_offset"));
            }
            let mut i = 0;
            {
                while {
                    let val = T::u32::from_data(&src[info.slots_offset() as usize + i * 8..])
                        .context("slot end")?;
                    val.conv() != 0
                } {
                    i += 1;
                }
                i += 1;
            }
            let slots = T::Key2::slice_from_data(&src[info.slots_offset() as usize..], i)
                .context("slot")?;
            (
                slots,
                T::u32::slice_from_data(
                    &src[info.slot_map_offset() as usize..],
                    slots.last().unwrap().val() as usize,
                )
                .context("slot map")?,
            )
        } else {
            (&[] as _, &[] as _)
        };
        let (block_header, block_offsets, blocks) = if info.block_offset() != 0 {
            let block_header =
                T::u32::from_data(&src[info.block_offset() as usize..]).context("block header")?;
            let n = ((info.lod0().physics_end() - info.lod0().skinned_end()) as usize).max(0);
            let block_offsets =
                T::u32::slice_from_data(&src[info.block_offset() as usize + 4..], n + 1)
                    .context("block offsets")?;
            let mut blocks = Vec::with_capacity(n);
            for i in 0..n {
                let size = (block_offsets[i + 1].conv() - block_offsets[i].conv()) as usize;
                let offset = (block_offsets[i].conv() + info.block_offset()) as usize;
                let block = BlockRef::from_data(&src[offset..offset+size]).with_context(|| format!("block {}", i))?;
                blocks.push(block);
            }
            (Some(block_header), block_offsets, blocks.into_boxed_slice())
        } else {
            (None, &[] as _, Box::default())
        };
        let hk_constraint = if info.hk_constraint_offset() != 0 {
            Some(
                shape::HkConstraintRef::from_data(src, info.hk_constraint_offset() as usize)
                    .context("hk_constarint")?,
            )
        } else {
            None
        };
        let hk_constraint_datas = T::HkConstraintData::slice_from_data(
            &src[info.hk_constraint_data_offset() as usize..],
            info.hk_constraint_data_num() as usize,
        )
        .context("hk_constraint_datas")?;
        let shape_infos = T::ShapeInfo::slice_from_data(&src[info.shape_offset() as usize..], info.shape_num() as usize).context("shape_infos")?;
        
        let shapes = shape_infos.into_iter().enumerate()
            .map(|(i, info)| {
                shape::ShapeRef::from_data(src, info)
                .with_context(|| format!("shape {}", i))
            })
            .collect::<Result<Vec<_>>>()?.into_boxed_slice();
        let mats = mat_order
            .iter()
            .enumerate()
            .map(|(i, x)| {
                Ok((
                    x.conv(),
                    mat::MatRef::from_data(src, x.conv() as usize)
                        .with_context(|| format!("mat {}", i))?,
                ))
            })
            .collect::<Result<IndexMap<_, _>>>()?;
        let buffer_infos = T::BufferInfo::slice_from_data(&src[info.buffer_info_offset() as usize..], info.mat_num() as usize).context("buffer infos")?;
        let vbuff_order = T::u32::slice_from_data(&src[info.vbuff_offset() as usize..], info.vbuff_num() as usize).context("vbuff order")?;
        let ibuff_order = T::u32::slice_from_data(&src[info.ibuff_offset() as usize..], info.ibuff_num() as usize).context("vbuff order")?;
        let vbuffs = vbuff_order.iter().map(|x| Ok((x.conv(), T::VBuffInfo::from_data(&src[x.conv() as usize..]).with_context(|| format!("vbuff info {}", x.conv()))?))).collect::<Result<IndexMap<_, _>>>()?;
        let ibuffs = ibuff_order.iter().map(|x| Ok((x.conv(), T::IBuffInfo::from_data(&src[x.conv() as usize..]).with_context(|| format!("ibuff info {}", x.conv()))?))).collect::<Result<IndexMap<_, _>>>()?;
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

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ModelRef_XE_<'a> {
    pub info: &'a ModelInfo_XE_,
    pub bones: BonesRef_XE_<'a>,
    pub mat_order: ref_slice<'a, u32_XE_>,
    pub mesh_order: ref_slice<'a, u32_XE_>,
    pub mesh_bounding_boxes: ref_slice<'a, BoundingBox_XE_>,
    pub skin_binds: ref_slice<'a, Matrix4x4_XE_>,
    pub vals_j: ref_slice<'a, u32_XE_>,
    pub val_k_header: ref_slice<'a, u16_XE_>,
    pub vals_k: ref_slice<'a, u32_XE_>,
    pub skin_order: ref_slice<'a, u32_XE_>,
    pub slots: ref_slice<'a, Key2_XE_>,
    pub slot_map: ref_slice<'a, u32_XE_>,
    pub block_header: Option<&'a u32_XE_>,
    pub block_offsets: ref_slice<'a, u32_XE_>,
    pub blocks: slice<BlockRef_XE_<'a>>,
    pub buffer_infos: ref_slice<'a, BufferInfo_XE_>,
    pub vbuff_order: ref_slice<'a, u32_XE_>,
    pub ibuff_order: ref_slice<'a, u32_XE_>,
    pub vbuffs: IndexMap<u32, &'a VBuffInfo_XE_>,
    pub ibuffs: IndexMap<u32, &'a IBuffInfo_XE_>,
    pub mats: IndexMap<u32, MatRef_XE_<'a>>,
    pub hk_constraint: Option<HkConstraintRef_XE_<'a>>, // stores bone transforms used for ragdoll
    pub hk_constraint_datas: ref_slice<'a, HkConstraintData_XE_>,
    pub shapes: slice<ShapeRef_XE_<'a>>,
    pub data: ModelDataRef_XE_<'a>,
}

#[make_endian]
impl<'a> ModelRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a ModelInfo_XE_, model_data: &IndexMap<u32, &'a CompressedDataRef<'a>>) -> Result<Self> {
        let bones = BonesRef_XE_::from_data(src, info).context("bones")?;
        let mat_order = u32_XE_::slice_from_data(
            &src[info.mat_offset.conv()..],
            info.mat_num.conv(),
        )
        .context("mat order")?;
        let mesh_order = u32_XE_::slice_from_data(
            &src[info.mesh_order_offset.conv()..],
            info.lod3.breakable_end.conv(),
        )
        .context("mesh order")?;
        let mesh_bounding_boxes = BoundingBox_XE_::slice_from_data(
            &src[info.mesh_bounding_boxes_offset.conv()..],
            info.lod3.breakable_end.conv(),
        )
        .context("mesh bounding boxes")?;
        let skin_binds = Matrix4x4_XE_::slice_from_data(
            &src[info.skin_binds_offset.conv()..],
            info.skin_binds_num.conv(),
        )
        .context("skin binds")?;
        let vals_j = u32_XE_::slice_from_data(
            &src[info.vals_j_offset.conv()..],
            info.vals_j_num.conv(),
        )
        .context("vals j")?;
        let (val_k_header, vals_k) = if info.vals_k_offset != 0 {
            (
                u16_XE_::slice_from_data(&src[info.vals_k_offset.conv()..], 2)
                    .context("vals k header")?,
                u32_XE_::slice_from_data(&src[info.vals_k_offset.to_native() as usize + 4..], 35)
                    .context("vals k")?,
            )
        } else {
            (&[] as _, &[] as _)
        };
        let skin_order = if info.skin_order_offset != 0 {
            u32_XE_::slice_from_data(
                &src[info.skin_order_offset.conv()..],
                info.skin_binds_num.conv(),
            )
            .context("skin order")?
        } else {
            &[] as _
        };
        let (slots, slot_map) = if info.slots_offset != 0 {
            if info.slot_map_offset == 0 {
                return Err(anyhow!("expected non zero slot_map_offset"));
            }
            let mut i = 0;
            {
                while {
                    let val = u32_XE_::from_data(&src[info.slots_offset.to_native() as usize + i * 8..])
                        .context("slot end")?;
                    *val != 0
                } {
                    i += 1;
                }
                i += 1;
            }
            let slots = Key2_XE_::slice_from_data(&src[info.slots_offset.conv()..], i)
                .context("slot")?;
            (
                slots,
                u32_XE_::slice_from_data(
                    &src[info.slot_map_offset.conv()..],
                    slots.last().unwrap().val.conv(),
                )
                .context("slot map")?,
            )
        } else {
            (&[] as _, &[] as _)
        };
        let (block_header, block_offsets, blocks) = if info.block_offset != 0 {
            let block_header =
                u32_XE_::from_data(&src[info.block_offset.conv()..]).context("block header")?;
            let n = ((info.lod0.physics_end.to_native() - info.lod0.skinned_end.to_native()) as usize).max(0);
            let block_offsets =
                u32_XE_::slice_from_data(&src[info.block_offset.to_native() as usize + 4..], n + 1)
                    .context("block offsets")?;
            let mut blocks = Vec::with_capacity(n);
            for i in 0..n {
                let size = (block_offsets[i + 1].to_native() - block_offsets[i].to_native()) as usize;
                let offset = (block_offsets[i].to_native() + info.block_offset.to_native()) as usize;
                let block = BlockRef_XE_::from_data(&src[offset..offset+size]).with_context(|| format!("block {}", i))?;
                blocks.push(block);
            }
            (Some(block_header), block_offsets, blocks.into_boxed_slice())
        } else {
            (None, &[] as _, Box::default())
        };
        let hk_constraint = if info.hk_constraint_offset != 0 {
            Some(
                HkConstraintRef_XE_::from_data(src, info.hk_constraint_offset.conv())
                    .context("hk_constarint")?,
            )
        } else {
            None
        };
        let hk_constraint_datas = shape::HkConstraintData_XE_::slice_from_data(
            &src[info.hk_constraint_data_offset.conv()..],
            info.hk_constraint_data_num.conv(),
        )
        .context("hk_constraint_datas")?;
        let shape_infos = ShapeInfo_XE_::slice_from_data(&src[info.shape_offset.conv()..], info.shape_num.conv()).context("shape_infos")?;
        
        let shapes = shape_infos.into_iter().enumerate()
            .map(|(i, info)| {
                ShapeRef_XE_::from_data(src, info)
                .with_context(|| format!("shape {}", i))
            })
            .collect::<Result<Vec<_>>>()?.into_boxed_slice();
        let mats = mat_order
            .iter()
            .enumerate()
            .map(|(i, x)| {
                Ok((
                    x.conv(),
                    MatRef_XE_::from_data(src, x.conv())
                        .with_context(|| format!("mat {}", i))?,
                ))
            })
            .collect::<Result<IndexMap<_, _>>>()?;
        let buffer_infos = data::BufferInfo_XE_::slice_from_data(&src[info.buffer_info_offset.conv()..], info.mat_num.conv()).context("buffer infos")?;
        let vbuff_order = u32_XE_::slice_from_data(&src[info.vbuff_offset.conv()..], info.vbuff_num.conv()).context("vbuff order")?;
        let ibuff_order = u32_XE_::slice_from_data(&src[info.ibuff_offset.conv()..], info.ibuff_num.conv()).context("vbuff order")?;
        let vbuffs = vbuff_order.iter().map(|x| Ok((x.conv(), data::VBuffInfo_XE_::from_data(&src[x.conv()..]).with_context(|| format!("vbuff info {}", x.to_native()))?))).collect::<Result<IndexMap<_, _>>>()?;
        let ibuffs = ibuff_order.iter().map(|x| Ok((x.conv(), data::IBuffInfo_XE_::from_data(&src[x.conv()..]).with_context(|| format!("ibuff info {}", x.to_native()))?))).collect::<Result<IndexMap<_, _>>>()?;
        let data = ModelDataRef_XE_::from_data(src, info, model_data).context("model data")?;
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
    pub info: ModelInfo,
    pub bones: Bones,
    pub mat_order: Vec<u32>,
    pub mesh_order: Vec<u32>, // order of models (mapped to lod0, lod1, lod2, lod3)
    pub mesh_bounding_boxes: Vec<BoundingBox>,
    pub skin_binds: Vec<Matrix4x4>, // mat4, bind matrices or something??
    pub vals_j: Vec<u32>,           // bows & banners, maybe for strings?
    pub val_k_header: Vec<u16>,
    pub vals_k: Vec<u32>,          // has to do with trees
    pub skin_order: Vec<u32>, // bone mapping for vals_g, seems to be the mapping used for skinning
    pub slots: Vec<Key2>,     // attachment points
    pub slot_map: Vec<u32>,   // attachment bone mapping
    pub block_header: Option<u32>, // has to do with havok cloth / hair stuff
    pub blocks: Vec<Block>,
    pub mats: Vec<mat::Mat>,
    pub hk_constraint: Option<shape::HkConstraint>, // stores bone transforms used for ragdoll ??
    pub hk_constraint_datas: Vec<shape::HkConstraintData>,
    pub shapes: Vec<shape::Shape>,
    pub data: ModelData
}

#[make_endian]
impl From<&ModelRef_XE_<'_>> for Model {
    fn from(val: &ModelRef_XE_) -> Self {
        Self {
            info: val.info.conv(),
            bones: (&val.bones).into(),
            mat_order: val
                .mat_order
                .iter()
                .map(|x| val.mats.get_index_of(&x.to_native()).unwrap() as u32)
                .collect(),
            mesh_order: val.mesh_order.iter().map(|x| x.conv()).collect(),
            mesh_bounding_boxes: val.mesh_bounding_boxes.iter().map(|x| x.conv()).collect(),
            skin_binds: val.skin_binds.iter().map(|x| x.conv()).collect(),
            vals_j: val.vals_j.iter().map(|x| x.conv()).collect(),
            val_k_header: val.val_k_header.iter().map(|x| x.conv()).collect(),
            vals_k: val.vals_k.iter().map(|x| x.conv()).collect(),
            skin_order: val.skin_order.iter().map(|x| x.conv()).collect(),
            slots: val.slots.iter().map(|x| x.conv()).collect(),
            slot_map: val.slot_map.iter().map(|x| x.conv()).collect(),
            block_header: val.block_header.map(|x| x.conv()),
            blocks: val.blocks.iter().map(|x| x.into()).collect(),
            mats: val
                .mats
                .values()
                .map(|x| x.into())
                .collect(),
            hk_constraint: val.hk_constraint.as_ref().map(|x| x.into()),
            hk_constraint_datas: val.hk_constraint_datas.iter().map(|x| x.conv()).collect(),
            shapes: val.shapes.iter().map(|x| x.into()).collect(),
            data: (&val.data).into()
        }
    }
}

#[make_endian]
pub trait DumpModel_XE_ {
    fn info(&self) -> &ModelInfo_XE_;
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
    fn write_bones(&self, bones: BonesDump_XE_) -> Result<()>;
    fn write_vals_j(&self, vals_j: &mut [u32_XE_]) -> Result<()>;
    fn write_skin_binds(&self, skin_binds: &mut [Matrix4x4_XE_]) -> Result<()>;
    fn write_skin_order(&self, skin_order: &mut [u32_XE_]) -> Result<()>;
    fn write_mesh_order(&self, mesh_order: &mut [u32_XE_]) -> Result<()>;
    fn write_mesh_bounding_boxes(&self, mesh_bboxes: &mut [BoundingBox_XE_]) -> Result<()>;
    fn vbuff_order(&self) -> impl Iterator<Item=u32>;
    fn ibuff_order(&self) -> impl Iterator<Item=u32>;
    fn write_buffer_infos(&self, buffer_infos: &mut [BufferInfo_XE_]) -> Result<()>;
    fn write_vbuff_infos(&self, vbuff_infos: &mut [VBuffInfo_XE_]) -> Result<()>;
    fn write_ibuff_infos(&self, ibuff_infos: &mut [IBuffInfo_XE_]) -> Result<()>;
    fn write_vals_k(&self, header: &mut [u16_XE_], vals_k: &mut [u32_XE_]) -> Result<()>;
    fn write_slots(&self, slots: &mut [Key2_XE_]) -> Result<()>;
    fn write_slot_map(&self, slot_map: &mut [u32_XE_]) -> Result<()>;
    fn write_hk_constraint_datas(&self, hk_constraint_datas: &mut [HkConstraintData_XE_]) -> Result<()>;
    fn mat_order(&self) -> impl Iterator<Item=u32>;
    fn mats<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpMat_XE_ + 'a))>;
    fn shapes<'a>(&'a self) -> impl Iterator<Item=&'a (impl DumpShape_XE_ + 'a)>;
    fn hk_constraint<'a>(&'a self) -> Option<&'a (impl DumpHkConstraint_XE_ + 'a)>;
    fn block_header(&self) -> Option<impl Into<u32_XE_>>;
    fn blocks<'a>(&'a self) -> impl Iterator<Item=&'a (impl DumpBlock_XE_ + 'a)>;
    fn data(&self) -> &impl DumpModelData_XE_;
    fn asset_info(&self) -> (u32_XE_, u32_XE_);
    fn write_info(&self, info: &mut ModelInfo_XE_);

    fn dump_misc(&self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_, info: &mut ModelInfo_XE_, info_off: usize) -> Result<()> {
        if self.has_vals_k() {
            dst.align(16)?;
            info.vals_k_offset = dst.offset.conv();
            let vals_k_header = u16_XE_::mut_slice_from_data(dst, 2).context("val_k_header")?;
            let vals_k = u32_XE_::mut_slice_from_data(dst, 35).context("vals_k")?;
            self.write_vals_k(vals_k_header, vals_k).context("vals_k")?;
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, vals_k_offset)).conv();
        } else {
            info.vals_k_offset = 0u32.conv();
        }

        if self.slot_num() != 0 {
            info.slots_offset = dst.offset.conv();
            let slots = Key2_XE_::mut_slice_from_data(dst, self.slot_num()).context("slots")?;
            self.write_slots(slots).context("write slots")?;
            info.slot_map_offset = dst.offset.conv();
            let slot_map = u32_XE_::mut_slice_from_data(dst, self.slot_map_num()).context("slot_map")?;
            self.write_slot_map(slot_map).context("write slot_map")?;
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, slots_offset)).conv();
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, slot_map_offset)).conv();
        } else {
            info.slots_offset = 0u32.conv();
        }

        if let Some(block_header) = self.block_header() {
            dst.align(16)?;
            let start = dst.offset;
            info.block_offset = start.conv();
            *u32_XE_::mut_from_data(dst).context("block_header")? = block_header.into();
            let block_offsets = u32_XE_::mut_slice_from_data(dst, self.block_num() + 1).context("block_offsets")?;
            for (i, (block, off)) in self.blocks().zip(block_offsets.iter_mut()).enumerate() {
                dst.align(16)?;
                *off = (dst.offset - start).conv();
                block.dump_into(dst).with_context(|| format!("block {}", i))?;
            }
            if let Some(off) = block_offsets.last_mut() {
                *off = (dst.offset - start).conv();
            }
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, block_offset)).conv();
        }

        info.hk_constraint_data_num = self.hk_constraint_data_num().conv();
        if info.hk_constraint_data_num != 0 {
            info.hk_constraint_data_offset = infos.hk_constraint_datas.offset.conv();
            self.write_hk_constraint_datas(infos.hk_constraint_datas.next_slice(self.hk_constraint_data_num())).context("hk_constraint_datas")?;
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, hk_constraint_data_offset)).conv();
        }

        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, mat_offset)).conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, buffer_info_offset)).conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, mesh_order_offset)).conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, bone_parents_offset)).conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, bone_transforms_offset)).conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, skin_binds_offset)).conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, vbuff_offset)).conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, ibuff_offset)).conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, mesh_bounding_boxes_offset)).conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, bone_bounding_boxes_offset)).conv();

        Ok(())
    }

    fn add_misc_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        if self.has_vals_k() {
            offset = align_offset(offset, 16) + 2 * u16_XE_::size_of() + 35 * u32_XE_::size_of();
            counts.offsets += 1;
        }

        offset += Key2_XE_::size_of() * self.slot_num() + u32_XE_::size_of() * self.slot_map_num();

        if self.slot_num() != 0 {
            counts.offsets += 2;
        }

        if self.block_header().is_some() {
            offset = align_offset(offset, 16) + (2 + self.block_num()) * u32_XE_::size_of();
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

    fn dump_mats(&self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_, info: &mut ModelInfo_XE_) -> Result<()> {
        let mut off = dst.offset;
        info.mat_offset = off.conv();
        let mat_map = self.mats().sorted_by_key(|(k, _)| k.clone()).map(|(i, mat)| Ok((i, mat.dump_infos(infos)?))).collect::<Result<HashMap<_,_>>>().context("mats")?;
        let mat_order = u32_XE_::mut_slice_from_data(dst, self.mat_num()).context("mat_order")?;
        info.mat_num = mat_order.len().conv();
        let mut j = 0;
        for (i, mat) in self.mat_order().zip(mat_order) {
            *mat = mat_map.get(&i).ok_or(anyhow!("mats is missing item {}", i))?.conv(); 
            *infos.offsets.next().context("offsets")? = off.conv();
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
        offset + self.mat_num() * std::mem::size_of::<u32_XE_>()
    }

    fn dump_into<'a>(&'a self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_<'_, 'a>) -> Result<()> {
        let info_off = infos.models.offset;
        let info = infos.models.next().context("models")?;
        self.write_info(info);
        
        info.bones_offset = dst.offset.conv();
        let bones = Crc_XE_::mut_slice_from_data(dst, self.bone_num()).context("bones")?;
        info.bones_num = bones.len().conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, bones_offset)).conv();

        dst.align(16)?;
        info.bone_bounding_boxes_offset = dst.offset.conv();
        let bone_bboxes = BoundingBox_XE_::mut_slice_from_data(dst, bones.len()).context("bone_bounding_boxes")?;

        info.vals_j_offset = dst.offset.conv();
        let vals_j = u32_XE_::mut_slice_from_data(dst, self.vals_j_num()).context("vals_j")?;
        info.vals_j_num = vals_j.len().conv();
        self.write_vals_j(vals_j).context("write vals_j")?;
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, vals_j_offset)).conv();

        if let Some(hk_constraint) = self.hk_constraint() {
            info.hk_constraint_offset = infos.hk_constraints.offset.conv();
            hk_constraint.dump_into(dst, infos, info.bones_num.conv(), info.bones_offset.conv()).context("hk_constraint")?;
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, hk_constraint_offset)).conv();
        } else {
            info.hk_constraint_offset = 0u32.conv();
        }

        dst.align(16)?;
        info.skin_binds_offset = dst.offset.conv();
        let skin_binds = Matrix4x4_XE_::mut_slice_from_data(dst, self.skin_bind_num()).context("skin_binds")?;
        info.skin_binds_num = skin_binds.len().conv();
        self.write_skin_binds(skin_binds).context("write skin_binds")?;

        if self.has_skin_order() {
            info.skin_order_offset = dst.offset.conv();
            let skin_order = u32_XE_::mut_slice_from_data(dst, skin_binds.len()).context("skin_order")?;
            self.write_skin_order(skin_order).context("write skin_order")?;
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, skin_order_offset)).conv();
        } else {
            info.skin_order_offset = 0u32.conv();
        }

        info.bone_parents_offset = dst.offset.conv();
        let bone_parents = i32_XE_::mut_slice_from_data(dst, bones.len()).context("bone_parents")?;

        dst.align(16)?;
        info.bone_transforms_offset = dst.offset.conv();
        let bone_transforms = Matrix4x4_XE_::mut_slice_from_data(dst, bones.len()).context("bone_transforms")?;
        self.write_bones(BonesDump_XE_ {
            names: bones,
            parents: bone_parents,
            bounding_boxes: bone_bboxes,
            transforms: bone_transforms,
        }).context("write bones")?;

        self.dump_mats(dst, infos, info)?;

        info.shape_num = self.shape_num().conv();
        if info.shape_num != 0 {
            info.shape_offset = infos.shapes.offset.conv();
            for (i, shape) in self.shapes().enumerate() {
                shape.dump_into(dst, infos, None).with_context(|| format!("shape {}", i))?;
            }
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, shape_offset)).conv();
        } else {
            info.shape_offset = 0u32.conv();
        }

        info.mesh_order_offset = dst.offset.conv();
        let mesh_order = u32_XE_::mut_slice_from_data(dst, self.mesh_order_num()).context("mesh_order")?;
        // TODO need to rework how lod information is stored / presented
        // info.lod3.breakable_end = mesh_order.len();
        self.write_mesh_order(mesh_order).context("write mesh_order")?;

        dst.align(16)?;
        info.mesh_bounding_boxes_offset = dst.offset.conv();
        let mesh_bboxes = BoundingBox_XE_::mut_slice_from_data(dst, mesh_order.len()).context("mesh_bounding_boxes")?;
        self.write_mesh_bounding_boxes(mesh_bboxes).context("write mesh_bounding_boxes")?;

        let data = self.data().dump_into(dst, infos, info)?;
        let (asset_key, asset_type) = self.asset_info();
        infos.model_data.push(DumpInfoData_XE_ {
            key: asset_key,
            kind: asset_type,
            data
        });

        self.dump_misc(dst, infos, info, info_off)?;
        Ok(())
    }

    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        counts.models += 1;
        
        offset += Crc_XE_::size_of() * self.bone_num();
        offset = align_offset(offset, 16) + BoundingBox_XE_::size_of() * self.bone_num();

        offset += u32_XE_::size_of() * self.vals_j_num();

        counts.offsets += 2;

        if let Some(hk_constraint) = self.hk_constraint() {
            offset = hk_constraint.add_size(offset, counts);
            counts.offsets += 1;
        }

        offset = align_offset(offset, 16) + Matrix4x4_XE_::size_of() * self.skin_bind_num();

        if self.has_skin_order() {
            offset += u32_XE_::size_of() * self.skin_bind_num();
            counts.offsets += 1;
        }

        offset += i32_XE_::size_of() * self.bone_num();
        offset = align_offset(offset, 16) + Matrix4x4_XE_::size_of() * self.bone_num();

        offset = self.add_mat_counts(offset, counts);

        let mut has_shape = false;
        for shape in self.shapes() {
            offset = shape.add_size(offset, counts);
            has_shape = true;
        }
        if has_shape {
            counts.offsets += 1;
        }

        offset += u32_XE_::size_of() * self.mesh_order_num();
        offset = align_offset(offset, 16) + BoundingBox_XE_::size_of() * self.mesh_order_num();


        offset = self.data().add_size(offset, counts).0;

        offset = self.add_misc_size(offset, counts);
        offset
    }

    fn dump_terrain_into<'a>(&'a self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_<'_, 'a>, indices_offset: usize) -> Result<()> {
        let bone_bbox_offset = infos.models.offset + 16;
        let info_off = infos.models.offset;
        let info = infos.models.next().context("models")?;
        self.write_info(info);

        info.bones_offset = 0u32.conv();
        info.bones_num = self.bone_num().conv();
        info.bone_bounding_boxes_offset = bone_bbox_offset.conv();
        info.skin_binds_offset = indices_offset.conv();
        info.skin_binds_num = 0u32.conv();
        info.skin_order_offset = 0u32.conv();
        info.vals_j_offset = 0u32.conv();
        info.bone_parents_offset = indices_offset.conv();

        if let Some(hk_constraint) = self.hk_constraint() {
            info.hk_constraint_offset = infos.hk_constraints.offset.conv();
            hk_constraint.dump_into(dst, infos, info.bones_num.conv(), info.bones_offset.conv()).context("hk_constraint")?;
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, hk_constraint_offset)).conv();
        } else {
            info.hk_constraint_offset = 0u32.conv();
        }

        let mut shape_offsets = Vec::with_capacity(self.shape_num());
        for (i, shape) in self.shapes().enumerate() {
            shape_offsets.push(shape.dump_extra_into(dst).with_context(|| format!("shape {} extra", i))?);
        }
        
        dst.align(16)?;
        info.mesh_bounding_boxes_offset = dst.offset.conv();
        let mesh_bboxes = BoundingBox_XE_::mut_slice_from_data(dst, self.mesh_order_num()).context("mesh_bounding_boxes")?;
        self.write_mesh_bounding_boxes(mesh_bboxes).context("write mesh_bounding_boxes")?;

        info.bone_transforms_offset = dst.offset.conv();
        let bone_transforms = Matrix4x4_XE_::mut_slice_from_data(dst, self.bone_num()).context("bone_transforms")?;
        self.write_bones(BonesDump_XE_ {
            names: &mut [],
            parents: &mut [],
            bounding_boxes: &mut [],
            transforms: bone_transforms,
        }).context("write bones")?;

        self.dump_mats(dst, infos, info)?;

        info.mesh_order_offset = dst.offset.conv();
        let mesh_order = u32_XE_::mut_slice_from_data(dst, self.mesh_order_num()).context("mesh_order")?;
        // TODO need to rework how lod information is stored / presented
        // info.lod3.breakable_end = mesh_order.len();
        self.write_mesh_order(mesh_order).context("write mesh_order")?;

        let data = self.data().dump_into(dst, infos, info)?;
        let (asset_key, asset_type) = self.asset_info();
        infos.model_data.push(DumpInfoData_XE_ {
            key: asset_key,
            kind: asset_type,
            data
        });
        let off_dest = info.ibuff_offset.to_native() as usize + 320;

        self.dump_misc(dst, infos, info, info_off)?;

        if dst.offset < off_dest {
            dst.split(off_dest - dst.offset).context("model pad")?;
            //*dst = dst.split(off_dest - dst.offset)?;
        }

        info.shape_num = self.shape_num().conv();
        if info.shape_num != 0 {
            info.shape_offset = infos.shapes.offset.conv();
            for ((i, shape), off) in self.shapes().enumerate().zip(shape_offsets) {
                shape.dump_into(dst, infos, off).with_context(|| format!("shape {}", i))?;
            }
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(ModelInfo_XE_, shape_offset)).conv();
        } else {
            info.shape_offset = 0u32.conv();
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
        
        offset = align_offset(offset, 16) + BoundingBox_XE_::size_of() * self.mesh_order_num();
        offset += Matrix4x4_XE_::size_of() * self.bone_num();

        offset = self.add_mat_counts(offset, counts);

        offset += u32_XE_::size_of() * self.mesh_order_num();

        let (mut offset, off_dest) = self.data().add_size(offset, counts);


        offset = self.add_misc_size(offset, counts);

        offset = (off_dest + 320).max(offset);

        for shape in self.shapes() {
            offset = shape.add_size(offset, counts);
        }
        offset
    }
}

#[make_endian]
impl<'a> DumpModel_XE_ for ModelRef_XE_<'a> {
    fn info(&self) -> &ModelInfo_XE_ {
        self.info
    }
    fn key(&self) -> u32 {
        self.info.key.conv()
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
    fn write_bones(&self, BonesDump_XE_ { names, parents, transforms, bounding_boxes }: BonesDump_XE_) -> Result<()> {
        if names.len() != 0 {
            names.write_from(&self.bones.names[..])?;
            parents.write_from(&self.bones.parents[..])?;
            bounding_boxes.write_from(&self.bones.bounding_boxes[..])?;
        }
        transforms.write_from(&self.bones.transforms[..])
    }
    fn write_vals_j(&self, vals_j: &mut [u32_XE_]) -> Result<()> {
        vals_j.write_from(&self.vals_j[..])
    }
    fn write_skin_binds(&self, skin_binds: &mut [Matrix4x4_XE_]) -> Result<()> {
        skin_binds.write_from(&self.skin_binds[..])
    }
    fn write_skin_order(&self, skin_order: &mut [u32_XE_]) -> Result<()> {
        skin_order.write_from(&self.skin_order[..])
    }
    fn write_mesh_order(&self, mesh_order: &mut [u32_XE_]) -> Result<()> {
        mesh_order.write_from(&self.mesh_order[..])
    }
    fn write_mesh_bounding_boxes(&self, mesh_bboxes: &mut [BoundingBox_XE_]) -> Result<()> {
        mesh_bboxes.write_from(&self.mesh_bounding_boxes[..])
    }
    fn vbuff_order(&self) -> impl Iterator<Item=u32> {
        self.vbuff_order.iter().map(|x| x.conv())
    }
    fn ibuff_order(&self) -> impl Iterator<Item=u32> {
        self.ibuff_order.iter().map(|x| x.conv())
    }
    fn write_buffer_infos(&self, buffer_infos: &mut [BufferInfo_XE_]) -> Result<()> {
        buffer_infos.write_from(&self.buffer_infos[..])
    }
    fn write_vbuff_infos(&self, vbuff_infos: &mut [VBuffInfo_XE_]) -> Result<()> {
        for (src, dst) in self.vbuffs.values().zip(vbuff_infos) {
            dst.write_from(src)?;
        }
        Ok(())
    }
    fn write_ibuff_infos(&self, ibuff_infos: &mut [IBuffInfo_XE_]) -> Result<()> {
        for (src, dst) in self.ibuffs.values().zip(ibuff_infos) {
            dst.write_from(src)?;
        }
        Ok(())
    }
    fn write_vals_k(&self, header: &mut [u16_XE_], vals_k: &mut [u32_XE_]) -> Result<()>  {
        header.write_from(&self.val_k_header[..])?;
        vals_k.write_from(&self.vals_k[..])
    }
    fn write_slots(&self, slots: &mut [Key2_XE_]) -> Result<()> {
        slots.write_from(&self.slots[..])
    }
    fn write_slot_map(&self, slot_map: &mut [u32_XE_]) -> Result<()> {
        slot_map.write_from(&self.slot_map[..])
    }
    fn write_hk_constraint_datas(&self, hk_constraint_datas: &mut [HkConstraintData_XE_]) -> Result<()> {
        hk_constraint_datas.write_from(&self.hk_constraint_datas[..])
    }
    fn mat_order(&self) -> impl Iterator<Item=u32> {
        self.mat_order.iter().map(|x: &u32_XE_| x.conv())
    }
    fn mats(&self) -> impl Iterator<Item=(u32, &impl DumpMat_XE_)> {
        self.mats.iter().map(|(k, v)| (*k, v))
    }
    fn shapes(&self) -> impl Iterator<Item=&impl DumpShape_XE_> {
        self.shapes.iter()
    }
    fn hk_constraint(&self) -> Option<&impl DumpHkConstraint_XE_> {
        self.hk_constraint.as_ref()
    }
    fn block_header(&self) -> Option<impl Into<u32_XE_>> {
        self.block_header.cloned()
    }
    fn blocks(&self) -> impl Iterator<Item=&impl DumpBlock_XE_> {
        self.blocks.iter()
    }
    fn data(&self) -> &impl DumpModelData_XE_ {
        &self.data
    }
    fn asset_info(&self) -> (u32_XE_, u32_XE_) {
        (self.info.asset_key, self.info.asset_type)
    }
    fn write_info(&self, info: &mut ModelInfo_XE_) {
        *info = self.info.clone();
    }
}
