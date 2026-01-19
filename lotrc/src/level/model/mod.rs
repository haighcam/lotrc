use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use std::collections::HashMap;
use std::ptr::NonNull;

#[cfg(not(feature = "ffi"))]
use crate::types::GetNative;
use crate::{
    types::{Crc, DumpData, Matrix4x4, RefFromData, Vector3, DumpSlice, align_offset, CompressedDataRef, OrderedData, OrderedDataStrict, BufType, slice, CompressedDataRefAlt, box_slice, Map, MapImpl}, 
    level::{
        model::data::ModelData,
        pak::objs::InfoCounts,
    },
};
#[make_platforms]
use crate::{
    level::{
        model::{
            mat::{DumpMatVER, MatVER, MatRefVER},
            shape::{DumpShapeVER, DumpHkConstraintVER, ShapeVER, ShapeInfoVER, ShapeRefVER, HkConstraintDataVER, DumpShapeExtraVER, HkConstraintVER, HkConstraintRefVER},
            data::{ModelDataVER, BufferInfoVER, IBuffInfoVER, VBuffInfoVER, ModelDataRefVER},
        },
        pak::{
            objs::{ObjsVER, DumpInfosVER},
            PakHeaderVER
        },
    },
    types::{CrcVER, Matrix4x4VER, i32VER, u16VER, u32VER, Vector3VER, f32VER},
};
use lotrc_proc::{make_platforms, OrderedData};

pub mod data;
pub mod mat;
pub mod shape;

#[derive(Debug, Default, Clone, OrderedData)]
pub struct LodInfo {
    pub start: u32,
    pub static_end: u32,
    pub skinned_end: u32,
    pub physics_end: u32,
    pub breakable_end: u32,
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

#[derive(Default, Debug, Clone, OrderedData)]
pub struct BoundingBox {
    center: Vector3,
    unk_3: f32,
    half_width: Vector3,
    unk_7: f32,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct ModelInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub mat_offset: u32,
    pub buffer_info_offset: u32, // pointer to obj2, uses mat_num of sequential objects
    pub bounding_box: BoundingBox, // (center x, y, z, ?, half_width x, y, z, ?) default vals of 1.0 for the ? vals seems to work
    pub mesh_order_offset: u32, // ints (c & 0x3fffffff is an index into the obj2s referenced by this object) (1 for each mesh)
    pub lod0: LodInfo,
    pub lod1: LodInfo,
    pub lod2: LodInfo,
    pub lod3: LodInfo,
    pub mat_num: u32,
    pub bones_offset: u32, // ints
    pub bone_parents_offset: u32,
    pub bone_transforms_offset: u32, // 16 ints (matrix?) for keys_num
    pub bones_num: u32,
    pub skin_binds_offset: u32,
    pub skin_binds_num: u32,
    pub skin_order_offset: u32,
    pub vbuff_offset: u32,
    pub vbuff_num: u32,
    pub ibuff_offset: u32,
    pub ibuff_num: u32,
    pub mesh_bounding_boxes_offset: u32,
    pub unk_46: f32,           // maybe something to do with size?
    pub variation_counts: u32, // maybe something to do with variation
    pub vals_j_num: u32,
    pub vals_j_offset: u32,
    pub block_offset: u32,
    pub vals_k_offset: u32, // not sure on the size, seems to be 36 ints
    pub asset_key: Crc,     // data in bin that is vertex & index buffer values
    pub asset_type: u32,
    pub unk_54: u32, // 1 for occuluder otherwise 0 ??
    #[name_ps3(shape_offset)]
    pub unk_55: u32, // always 0 ??
    #[name_ps3(shape_num)]
    pub shape_offset: u32,
    #[name_ps3(hk_constraint_data_offset)]
    pub shape_num: u32,
    #[name_ps3(hk_constraint_data_num)]
    pub hk_constraint_data_offset: u32, // optional pointer to obje
    #[name_ps3(hk_constraint_offset)]
    pub hk_constraint_data_num: u32,
    #[name_ps3(slots_offset)]
    pub hk_constraint_offset: u32, // optional pointer to hkConstraint
    #[name_ps3(slot_map_offset)]
    pub slots_offset: u32,
    #[name_ps3(bone_bounding_boxes_offset)]
    pub slot_map_offset: u32,
    #[name_ps3(unk_55)]
    pub bone_bounding_boxes_offset: u32, // 8 ints
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct BlockHeader {
    pub a: u32,
    pub b: u32,
    pub unk_2: u32,
    pub unk_3: u32,
    pub unk_4: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct BlockVal {
    pub unk_0: u32,
    pub unk_1: u32,
    pub unk_2: u32,
    pub unk_3: u32,
    pub unk_4: u16,
    pub unk_5: u16,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct BlockHeader1 {
    pub a: u32,
    pub b: u32,
    pub unk_2: u32,
    pub unk_3: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct BlockHeader2 {
    pub n: u32,
    pub unk_1: f32,
    pub unk_2: f32,
    pub unk_3: u32,
    pub unk_4: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct BlockValA {
    pub unk_0: f32,
    pub unk_1: f32,
    pub unk_2: f32,
    pub unk_3: f32,
    pub unk_4: f32,
    pub unk_5: f32,
    pub unk_6: f32,
    pub unk_7: f32,
    pub unk_8: f32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct BlockValB {
    pub unk_0: u16,
    pub unk_1: u16,
    pub unk_2: f32,
    pub unk_3: f32,
    pub unk_4: f32,
    pub unk_5: f32,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct BlockRefVER<'a> {
    info1: &'a BlockHeader1VER,
    info2: &'a BlockHeader2VER,
    vals_a: slice<'a, BlockValAVER>,
    vals_b: slice<'a, BlockValAVER>,
    vals_c: slice<'a, BlockValBVER>,
    pad: slice<'a, u8>,
}

#[make_platforms]
impl<'a> BlockRefVER<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let info1 = BlockHeader1VER::from_data(&src[offset..]).context("info1")?;
        offset += info1.size();
        let vals_a = BlockValAVER::slice_from_data(&src[offset..], info1.a.get() as usize)
            .context("vals_a")?;
        offset += vals_a.size();
        let vals_b = BlockValAVER::slice_from_data(&src[offset..], info1.b.get() as usize)
            .context("vals_b")?;
        offset += vals_b.size();
        let info2 = BlockHeader2VER::from_data(&src[offset..]).context("info2")?;
        offset += info2.size();
        let vals_c = BlockValBVER::slice_from_data(&src[offset..], info2.n.get() as usize)
            .context("vals_c")?;
        offset += vals_c.size();
        let pad = &src[offset..];
        Ok(Self {
            info1: info1,
            info2: info2,
            vals_a: vals_a.into(),
            vals_b: vals_b.into(),
            vals_c: vals_c.into(),
            pad: pad.into(),
        })
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct BlockVER {
    _ptr: BufType,
    info1: NonNull<BlockHeader1VER>,
    info2: NonNull<BlockHeader2VER>,
    vals_a: NonNull<[BlockValAVER]>,
    vals_b: NonNull<[BlockValAVER]>,
    vals_c: NonNull<[BlockValBVER]>,
    pad: NonNull<[u8]>,
}

#[make_platforms]
unsafe impl Sync for BlockVER {}
#[make_platforms]
unsafe impl Send for BlockVER {}

#[make_platforms]
impl BlockVER {
    pub fn from_bytes(src: &BufType, mut offset: usize, size: usize) -> Result<Self> {
        let info1 = BlockHeader1VER::from_data(&src[offset..]).context("info1")?;
        offset += info1.size();
        let vals_a = BlockValAVER::slice_from_data(&src[offset..], info1.a.get() as usize)
            .context("vals_a")?;
        offset += vals_a.size();
        let vals_b = BlockValAVER::slice_from_data(&src[offset..], info1.b.get() as usize)
            .context("vals_b")?;
        offset += vals_b.size();
        let info2 = BlockHeader2VER::from_data(&src[offset..]).context("info2")?;
        offset += info2.size();
        let vals_c = BlockValBVER::slice_from_data(&src[offset..], info2.n.get() as usize)
            .context("vals_c")?;
        offset += vals_c.size();
        let pad = &src[offset..offset + size];
        Ok(Self {
            _ptr: src.clone(),
            info1: info1.into(),
            info2: info2.into(),
            vals_a: vals_a.into(),
            vals_b: vals_b.into(),
            vals_c: vals_c.into(),
            pad: pad.into(),
        })
    }
}

#[make_platforms]
impl BlockVER {
    pub fn info1(&self) -> &BlockHeader1VER {
        unsafe { self.info1.as_ref() }
    }
    pub fn info2(&self) -> &BlockHeader2VER {
        unsafe { self.info2.as_ref() }
    }
    pub fn vals_a(&self) -> &[BlockValAVER] {
        unsafe { self.vals_a.as_ref() }
    }
    pub fn vals_b(&self) -> &[BlockValAVER] {
        unsafe { self.vals_b.as_ref() }
    }
    pub fn vals_c(&self) -> &[BlockValBVER] {
        unsafe { self.vals_c.as_ref() }
    }
    pub fn pad(&self) -> &[u8] {
        unsafe { self.pad.as_ref() }
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

#[make_platforms]
impl From<&BlockVER> for Block {
    fn from(val: &BlockVER) -> Self {
        Self {
            info1: val.info1().conv(),
            info2: val.info2().conv(),
            vals_a: val.vals_a().iter().map(|x| x.conv()).collect(),
            vals_b: val.vals_b().iter().map(|x| x.conv()).collect(),
            vals_c: val.vals_c().iter().map(|x| x.conv()).collect(),
            pad: val.pad().to_vec(),
        }
    }
}

#[make_platforms]
pub trait DumpBlockVER {
    fn vals_a_num(&self) -> usize;
    fn vals_b_num(&self) -> usize;
    fn vals_c_num(&self) -> usize;
    fn pad_num(&self) -> usize;
    fn write_info1(&self, info1: &mut BlockHeader1VER) -> Result<()>;
    fn write_info2(&self, info2: &mut BlockHeader2VER) -> Result<()>;
    fn write_vals_a(&self, vals_a: &mut [BlockValAVER]) -> Result<()>;
    fn write_vals_b(&self, vals_b: &mut [BlockValAVER]) -> Result<()>;
    fn write_vals_c(&self, vals_c: &mut [BlockValBVER]) -> Result<()>;
    fn write_pad(&self, pad: &mut [u8]) -> Result<()>;

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let info1 = BlockHeader1VER::mut_from_data(dst).context("info1")?;
        self.write_info1(info1).context("write info1")?;

        let info2 = BlockHeader2VER::mut_from_data(dst).context("info2")?;
        self.write_info2(info2).context("write info2")?;

        let vals_a = BlockValAVER::mut_slice_from_data(dst, self.vals_a_num()).context("vals_a")?;
        self.write_vals_a(vals_a).context("write vals_a")?;

        let vals_b = BlockValAVER::mut_slice_from_data(dst, self.vals_b_num()).context("vals_b")?;
        self.write_vals_b(vals_b).context("write vals_b")?;

        let vals_c = BlockValBVER::mut_slice_from_data(dst, self.vals_c_num()).context("vals_c")?;
        self.write_vals_c(vals_c).context("write vals_c")?;

        let pad = u8::mut_slice_from_data(dst, self.pad_num()).context("pad")?;
        self.write_pad(pad).context("write pad")?;
        Ok(())
    }
    fn add_size(&self, offset: usize) -> usize {
        offset 
            + BlockHeader1VER::size_of()
            + BlockHeader2VER::size_of()
            + BlockValAVER::size_of() * self.vals_a_num() 
            + BlockValAVER::size_of() * self.vals_b_num()
            + BlockValBVER::size_of() * self.vals_c_num()
            + self.pad_num()
    }
}

#[make_platforms]
impl DumpBlockVER for BlockRefVER<'_> {
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
    fn write_info1(&self, info1: &mut BlockHeader1VER) -> Result<()> {
        info1.write_from(self.info1)
    }
    fn write_info2(&self, info2: &mut BlockHeader2VER) -> Result<()> {
        info2.write_from(self.info2)
    }
    fn write_vals_a(&self, vals_a: &mut [BlockValAVER]) -> Result<()> {
        vals_a.write_from(&self.vals_a[..])
    }
    fn write_vals_b(&self, vals_b: &mut [BlockValAVER]) -> Result<()> {
        vals_b.write_from(&self.vals_b[..])
    }
    fn write_vals_c(&self, vals_c: &mut [BlockValBVER]) -> Result<()> {
        vals_c.write_from(&self.vals_c[..])
    }
    fn write_pad(&self, pad: &mut [u8]) -> Result<()> {
        pad.write_from(&self.pad[..])
    }

}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct Key2 {
    pub key: Crc,
    pub val: u32,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct BonesRefVER<'a> {
    pub names:  slice<'a, CrcVER>,
    pub parents: slice<'a, i32VER>,
    pub transforms: slice<'a, Matrix4x4VER>,
    pub bounding_boxes: slice<'a, BoundingBoxVER>,
}

#[make_platforms]
impl<'a> BonesRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a ModelInfoVER) -> Result<Self> {
        let size = info.bones_num.get() as usize;
        let parents = i32VER::slice_from_data(
            &src[info.bone_parents_offset.get() as usize..],
            size.max(4),
        ).context("parents")?;
        if parents[0].get() != -1 {
            return Err(anyhow!("first bone should be the root, but parent != -1"));
        }
        let names_off = info.bones_offset.get() as usize;
        
        let names = if names_off != 0 {
            u32VER::slice_from_data(&src[names_off..], size).context("names")?
        } else {
            &[] as _
        };
        let transforms = Matrix4x4VER::slice_from_data(
            &src[info.bone_transforms_offset.get() as usize..],
            size,
        ).context("transforms")?;
        let bounding_boxes = BoundingBoxVER::slice_from_data(
            &src[info.bone_bounding_boxes_offset.get() as usize..],
            info.bones_num.get() as usize,
        ).context("bounding_boxes")?;
        Ok(Self {
            names: names.into(),
            parents: parents.into(),
            transforms: transforms.into(),
            bounding_boxes: bounding_boxes.into(),
        })
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.names.len()
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct BonesVER {
    names: Option<NonNull<CrcVER>>,
    parents: Option<NonNull<i32VER>>,
    transforms: Option<NonNull<Matrix4x4VER>>,
    bounding_boxes: Option<NonNull<BoundingBoxVER>>,
    size: usize
}

#[make_platforms]
impl BonesVER {
    pub fn from_bytes(src: &BufType, info: &ModelInfoVER) -> Result<Self> {
        let size = info.bones_num.get() as usize;
        let parents = i32VER::slice_from_data(
            &src[info.bone_parents_offset.get() as usize..],
            size.max(4),
        ).context("parents")?;
        if parents[0].get() != -1 {
            return Err(anyhow!("first bone should be the root, but parent != -1"));
        }
        let names_off = info.bones_offset.get() as usize;
        
        let names = if names_off != 0 {
            Some(u32VER::slice_from_data(&src[names_off..], size).context("names")?)
        } else {
            None
        };
        let transforms = Matrix4x4VER::slice_from_data(
            &src[info.bone_transforms_offset.get() as usize..],
            size,
        ).context("transforms")?;
        let bounding_boxes = BoundingBoxVER::slice_from_data(
            &src[info.bone_bounding_boxes_offset.get() as usize..],
            info.bones_num.get() as usize,
        ).context("bounding_boxes")?;
        Ok(Self {
            names: names.and_then(|x| x.first().map(|x| x.into())),
            parents: parents.first().map(|x| x.into()),
            transforms: transforms.first().map(|x| x.into()),
            bounding_boxes: bounding_boxes.first().map(|x| x.into()),
            size
        })
    }

    pub unsafe fn as_ref(&self) -> BonesRefVER<'_> {
        BonesRefVER {
            names: self.names.map(|x| unsafe { NonNull::slice_from_raw_parts(x, self.size).as_ref() }).unwrap_or(&[]).into(),
            parents: self.parents.map(|x| unsafe { NonNull::slice_from_raw_parts(x, self.size).as_ref() }).unwrap_or(&[]).into(),
            transforms: self.transforms.map(|x| unsafe { NonNull::slice_from_raw_parts(x, self.size).as_ref() }).unwrap_or(&[]).into(),
            bounding_boxes: self.bounding_boxes.map(|x| unsafe { NonNull::slice_from_raw_parts(x, self.size).as_ref() }).unwrap_or(&[]).into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bones {
    pub names: Vec<Crc>,
    pub parents: Vec<i32>,
    pub transforms: Vec<Matrix4x4>,
    pub bounding_boxes: Vec<BoundingBox>
}

#[make_platforms]
impl From<BonesRefVER<'_>> for Bones {
    fn from(val: BonesRefVER<'_>) -> Self {
        Self {
            names: val.names.iter().map(|x| x.conv()).collect(),
            parents: val.parents.iter().map(|x| x.conv()).collect(),
            transforms: val.transforms.iter().map(|x| x.conv()).collect(),
            bounding_boxes: val.bounding_boxes.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_platforms]
pub struct BonesDumpVER<'a> {
    names:  &'a mut [CrcVER],
    parents: &'a mut [i32VER],
    transforms: &'a mut [Matrix4x4VER],
    bounding_boxes: &'a mut [BoundingBoxVER],
}

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct ModelRefVER<'a> {
    pub info: &'a ModelInfoVER,
    pub bones: BonesRefVER<'a>,
    pub mat_order: slice<'a, u32VER>,
    pub mesh_order: slice<'a, u32VER>,
    pub mesh_bounding_boxes: slice<'a, BoundingBoxVER>,
    pub skin_binds: slice<'a, Matrix4x4VER>,
    pub vals_j: slice<'a, u32VER>,
    pub val_k_header: slice<'a, u16VER>,
    pub vals_k: slice<'a, u32VER>,
    pub skin_order: slice<'a, u32VER>,
    pub slots: slice<'a, Key2VER>,
    pub slot_map: slice<'a, u32VER>,
    pub block_header: Option<&'a u32VER>,
    pub block_offsets: slice<'a, u32VER>,
    pub blocks: box_slice<BlockRefVER<'a>>,
    pub val: slice<'a, u32VER>,
    pub buffer_infos: slice<'a, BufferInfoVER>,
    pub vbuff_order: slice<'a, u32VER>,
    pub ibuff_order: slice<'a, u32VER>,
    pub vbuffs: Map<u32, &'a VBuffInfoVER>,
    pub ibuffs: Map<u32, &'a IBuffInfoVER>,
    pub mats: Map<u32, MatRefVER<'a>>,
    pub hk_constraint: Option<HkConstraintRefVER<'a>>, // stores bone transforms used for ragdoll ??
    pub hk_constraint_datas: slice<'a, HkConstraintDataVER>,
    pub shapes: box_slice<ShapeRefVER<'a>>,
    pub data: Option<&'a CompressedDataRefAlt<'a>>,
}

#[make_platforms]
impl<'a> ModelRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a ModelInfoVER, model_data: &Map<u32, &'a CompressedDataRefAlt<'a>>) -> Result<Self> {
        let bones = BonesRefVER::from_data(src, info).context("bones")?;
        let mat_order = u32VER::slice_from_data(
            &src[info.mat_offset.get() as usize..],
            info.mat_num.get() as usize,
        )
        .context("vals")?;
        let mesh_order = u32VER::slice_from_data(
            &src[info.mesh_order_offset.get() as usize..],
            info.lod3.breakable_end.get() as usize,
        )
        .context("vals")?;
        let mesh_bounding_boxes = BoundingBoxVER::slice_from_data(
            &src[info.mesh_bounding_boxes_offset.get() as usize..],
            info.lod3.breakable_end.get() as usize,
        )
        .context("vals")?;
        let skin_binds = Matrix4x4VER::slice_from_data(
            &src[info.skin_binds_offset.get() as usize..],
            info.skin_binds_num.get() as usize,
        )
        .context("vals")?;
        let vals_j = u32VER::slice_from_data(
            &src[info.vals_j_offset.get() as usize..],
            info.vals_j_num.get() as usize,
        )
        .context("vals")?;
        let (val_k_header, vals_k) = if info.vals_k_offset.get() != 0 {
            (
                u16VER::slice_from_data(&src[info.vals_k_offset.get() as usize..], 2)
                    .context("vals")?,
                u32VER::slice_from_data(&src[info.vals_k_offset.get() as usize + 4..], 35)
                    .context("vals")?,
            )
        } else {
            (&[] as _, &[] as _)
        };
        let skin_order = if info.skin_order_offset.get() != 0 {
            u32VER::slice_from_data(
                &src[info.skin_order_offset.get() as usize..],
                info.skin_binds_num.get() as usize,
            )
            .context("vals")?
        } else {
            &[] as _
        };
        let (slots, slot_map) = if info.slots_offset.get() != 0 {
            if info.slot_map_offset.get() == 0 {
                return Err(anyhow!("expected non zero slot_map_offset"));
            }
            let mut i = 0;
            {
                while {
                    let val = u32VER::from_data(&src[info.slots_offset.get() as usize + i * 8..])
                        .context("vals")?;
                    val.get() != 0
                } {
                    i += 1;
                }
                i += 1;
            }
            let slots = Key2VER::slice_from_data(&src[info.slots_offset.get() as usize..], i)
                .context("vals")?;
            (
                slots,
                u32VER::slice_from_data(
                    &src[info.slot_map_offset.get() as usize..],
                    slots.last().unwrap().val.get() as usize,
                )
                .context("vals")?,
            )
        } else {
            (&[] as _, &[] as _)
        };
        let (block_header, block_offsets, blocks) = if info.block_offset.get() != 0 {
            let block_header =
                u32VER::from_data(&src[info.block_offset.get() as usize..]).context("vals")?;
            let n = ((info.lod0.physics_end.get() - info.lod0.skinned_end.get()) as usize).max(0);
            let block_offsets =
                u32VER::slice_from_data(&src[info.block_offset.get() as usize + 4..], n + 1)
                    .context("vals")?;
            let mut blocks = Vec::with_capacity(n);
            for i in 0..n {
                let size = (block_offsets[i + 1].get() - block_offsets[i].get()) as usize;
                let offset = (block_offsets[i].get() + info.block_offset.get()) as usize;
                let block = BlockRefVER::from_data(&src[offset..offset+size]).with_context(|| format!("block {}", i))?;
                blocks.push(block);
            }
            (Some(block_header), block_offsets, blocks.into_boxed_slice())
        } else {
            (None, &[] as _, Box::default())
        };
        let val = u32VER::slice_from_data(&src[info.mesh_order_offset.get() as usize..], 4)
            .context("vals")?;
        let hk_constraint = if info.hk_constraint_offset.get() != 0 {
            Some(
                HkConstraintRefVER::from_data(src, info.hk_constraint_offset.get() as usize)
                    .context("hk_constarint")?,
            )
        } else {
            None
        };
        let hk_constraint_datas = shape::HkConstraintDataVER::slice_from_data(
            &src[info.hk_constraint_data_offset.get() as usize..],
            info.hk_constraint_data_num.get() as usize,
        )
        .context("hk_constraint_datas")?;
        let shape_infos = ShapeInfoVER::slice_from_data(&src[info.shape_offset.get() as usize..], info.shape_num.get() as usize).context("shape_infos")?;
        
        let shapes = shape_infos.into_iter().enumerate()
            .map(|(i, info)| {
                ShapeRefVER::from_data(src, info)
                .with_context(|| format!("shape {}", i))
            })
            .collect::<Result<Vec<_>>>()?.into_boxed_slice();
        let mats = mat_order
            .iter()
            .enumerate()
            .map(|(i, x)| {
                Ok((
                    x.get(),
                    MatRefVER::from_data(src, x.get() as usize)
                        .with_context(|| format!("mat {}", i))?,
                ))
            })
            .collect::<Result<MapImpl<_, _>>>()?;
        let buffer_infos = data::BufferInfoVER::slice_from_data(&src[info.buffer_info_offset.get() as usize..], info.mat_num.get() as usize).context("buffer infos")?;
        let vbuff_order = u32VER::slice_from_data(&src[info.vbuff_offset.get() as usize..], info.vbuff_num.get() as usize).context("vbuff order")?;
        let ibuff_order = u32VER::slice_from_data(&src[info.ibuff_offset.get() as usize..], info.ibuff_num.get() as usize).context("vbuff order")?;
        let vbuffs = vbuff_order.iter().map(|x| Ok((x.get(), data::VBuffInfoVER::from_data(&src[x.get() as usize..]).with_context(|| format!("vbuff info {}", x.get()))?))).collect::<Result<MapImpl<_, _>>>()?;
        let ibuffs = ibuff_order.iter().map(|x| Ok((x.get(), data::IBuffInfoVER::from_data(&src[x.get() as usize..]).with_context(|| format!("ibuff info {}", x.get()))?))).collect::<Result<MapImpl<_, _>>>()?;
        let data = model_data.get(&info.asset_key.get()).copied();
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
            val: val.into(),
            hk_constraint,
            hk_constraint_datas: hk_constraint_datas.into(),
            shapes: shapes.into(),
            data
        })
    }
    pub fn parse_data(&self) -> Result<ModelDataRefVER<'a>> {
        self.try_into()
    }
}

#[make_platforms]
pub struct ModelRawVER {
    _ptr: BufType,
    info: NonNull<ModelInfoVER>,
    bones: BonesVER,
    mat_order: NonNull<[u32VER]>,
    mesh_order: NonNull<[u32VER]>,
    mesh_bounding_boxes: NonNull<[BoundingBoxVER]>,
    skin_binds: NonNull<[Matrix4x4VER]>,
    vals_j: NonNull<[u32VER]>,
    val_k_header: NonNull<[u16VER]>,
    vals_k: NonNull<[u32VER]>,
    skin_order: NonNull<[u32VER]>,
    slots: NonNull<[Key2VER]>,
    slot_map: NonNull<[u32VER]>,
    block_header: Option<u32VER>,
    block_offsets: NonNull<[u32VER]>,
    blocks: Box<[BlockVER]>,
    val: NonNull<[u32VER]>,
    buffer_infos: NonNull<[BufferInfoVER]>,
    vbuff_order: NonNull<[u32VER]>,
    ibuff_order: NonNull<[u32VER]>,
    _vbuffs: IndexMap<u32, NonNull<VBuffInfoVER>>,
    _ibuffs: IndexMap<u32, NonNull<IBuffInfoVER>>,
    mats: IndexMap<u32, MatVER>,
    hk_constraint: Option<HkConstraintVER>, // stores bone transforms used for ragdoll ??
    hk_constraint_datas: NonNull<[HkConstraintDataVER]>,
    shapes: Box<[ShapeVER]>,
    data: CompressedDataRef,
}

#[make_platforms]
impl ModelRawVER {
    pub fn from_bytes(src: &BufType, model_data: &IndexMap<u32, CompressedDataRef>, offset: usize) -> Result<Self> {
        let info = ModelInfoVER::from_data(&src[offset..]).context("info")?;
        let bones = BonesVER::from_bytes(src, info).context("bones")?;
        let mat_order = u32VER::slice_from_data(
            &src[info.mat_offset.get() as usize..],
            info.mat_num.get() as usize,
        )
        .context("vals")?;
        let mesh_order = u32VER::slice_from_data(
            &src[info.mesh_order_offset.get() as usize..],
            info.lod3.breakable_end.get() as usize,
        )
        .context("vals")?;
        let mesh_bounding_boxes = BoundingBoxVER::slice_from_data(
            &src[info.mesh_bounding_boxes_offset.get() as usize..],
            info.lod3.breakable_end.get() as usize,
        )
        .context("vals")?;
        let skin_binds = Matrix4x4VER::slice_from_data(
            &src[info.skin_binds_offset.get() as usize..],
            info.skin_binds_num.get() as usize,
        )
        .context("vals")?;
        let vals_j = u32VER::slice_from_data(
            &src[info.vals_j_offset.get() as usize..],
            info.vals_j_num.get() as usize,
        )
        .context("vals")?;
        let (val_k_header, vals_k) = if info.vals_k_offset.get() != 0 {
            (
                u16VER::slice_from_data(&src[info.vals_k_offset.get() as usize..], 2)
                    .context("vals")?,
                u32VER::slice_from_data(&src[info.vals_k_offset.get() as usize + 4..], 35)
                    .context("vals")?,
            )
        } else {
            (&[] as _, &[] as _)
        };
        let skin_order = if info.skin_order_offset.get() != 0 {
            u32VER::slice_from_data(
                &src[info.skin_order_offset.get() as usize..],
                info.skin_binds_num.get() as usize,
            )
            .context("vals")?
        } else {
            &[] as _
        };
        let (slots, slot_map) = if info.slots_offset.get() != 0 {
            if info.slot_map_offset.get() == 0 {
                return Err(anyhow!("expected non zero slot_map_offset"));
            }
            let mut i = 0;
            {
                while {
                    let val = u32VER::from_data(&src[info.slots_offset.get() as usize + i * 8..])
                        .context("vals")?;
                    val.get() != 0
                } {
                    i += 1;
                }
                i += 1;
            }
            let slots = Key2VER::slice_from_data(&src[info.slots_offset.get() as usize..], i)
                .context("vals")?;
            (
                slots,
                u32VER::slice_from_data(
                    &src[info.slot_map_offset.get() as usize..],
                    slots.last().unwrap().val.get() as usize,
                )
                .context("vals")?,
            )
        } else {
            (&[] as _, &[] as _)
        };
        let (block_header, block_offsets, blocks) = if info.block_offset.get() != 0 {
            let block_header =
                u32VER::from_data(&src[info.block_offset.get() as usize..]).context("vals")?;
            let n = ((info.lod0.physics_end.get() - info.lod0.skinned_end.get()) as usize).max(0);
            let block_offsets =
                u32VER::slice_from_data(&src[info.block_offset.get() as usize + 4..], n + 1)
                    .context("vals")?;
            let mut blocks = Vec::with_capacity(n);
            for i in 0..n {
                let size = (block_offsets[i + 1].get() - block_offsets[i].get()) as usize;
                let offset = (block_offsets[i].get() + info.block_offset.get()) as usize;
                let block = BlockVER::from_bytes(src, offset, size)?;
                blocks.push(block);
            }
            (Some(*block_header), block_offsets, blocks)
        } else {
            (None, &[] as _, vec![])
        };
        let val = u32VER::slice_from_data(&src[info.mesh_order_offset.get() as usize..], 4)
            .context("vals")?;
        let hk_constraint = if info.hk_constraint_offset.get() != 0 {
            Some(
                shape::HkConstraintVER::from_bytes(src, info.hk_constraint_offset.get() as usize)
                    .context("hk_constarint")?,
            )
        } else {
            None
        };
        let hk_constraint_datas = shape::HkConstraintDataVER::slice_from_data(
            &src[info.hk_constraint_data_offset.get() as usize..],
            info.hk_constraint_data_num.get() as usize,
        )
        .context("hk_constraint_datas")?;
        let shapes = (0..info.shape_num.get() as usize)
            .map(|i| {
                shape::ShapeVER::from_bytes(
                    src,
                    info.shape_offset.get() as usize + i * shape::ShapeInfoVER::size_of(),
                )
                .with_context(|| format!("shape {}", i))
            })
            .collect::<Result<Vec<_>>>()?;
        let mats = mat_order
            .iter()
            .enumerate()
            .map(|(i, x)| {
                Ok((
                    x.get(),
                    mat::MatVER::from_bytes(src, x.get() as usize)
                        .with_context(|| format!("mat {}", i))?,
                ))
            })
            .collect::<Result<_>>()?;
        let buffer_infos = data::BufferInfoVER::slice_from_data(&src[info.buffer_info_offset.get() as usize..], info.mat_num.get() as usize).context("buffer infos")?;
        let vbuff_order = u32VER::slice_from_data(&src[info.vbuff_offset.get() as usize..], info.vbuff_num.get() as usize).context("vbuff order")?;
        let ibuff_order = u32VER::slice_from_data(&src[info.ibuff_offset.get() as usize..], info.ibuff_num.get() as usize).context("vbuff order")?;
        let _vbuffs = vbuff_order.iter().map(|x| Ok((x.get(), NonNull::from_ref(data::VBuffInfoVER::from_data(&src[x.get() as usize..]).with_context(|| format!("vbuff info {}", x.get()))?)))).collect::<Result<_>>()?;
        let _ibuffs = ibuff_order.iter().map(|x| Ok((x.get(), NonNull::from_ref(data::IBuffInfoVER::from_data(&src[x.get() as usize..]).with_context(|| format!("ibuff info {}", x.get()))?)))).collect::<Result<_>>()?;
        let data = model_data.get(&info.asset_key.get()).cloned().unwrap_or_default();
        Ok(Self {
            _ptr: src.clone(),
            info: info.into(),
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
            mats,
            buffer_infos: buffer_infos.into(),
            vbuff_order: vbuff_order.into(),
            ibuff_order: ibuff_order.into(),
            _vbuffs,
            _ibuffs,
            val: val.into(),
            hk_constraint,
            hk_constraint_datas: hk_constraint_datas.into(),
            shapes: shapes.into(),
            data
        })
    }
    pub fn info(&self) -> &ModelInfoVER {
        unsafe { self.info.as_ref() }
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct ModelVER {
    _ptr: BufType,
    info: NonNull<ModelInfoVER>,
    bones: BonesVER,
    mat_order: NonNull<[u32VER]>,
    mesh_order: NonNull<[u32VER]>,
    mesh_bounding_boxes: NonNull<[BoundingBoxVER]>,
    skin_binds: NonNull<[Matrix4x4VER]>,
    vals_j: NonNull<[u32VER]>,
    val_k_header: NonNull<[u16VER]>,
    vals_k: NonNull<[u32VER]>,
    skin_order: NonNull<[u32VER]>,
    slots: NonNull<[Key2VER]>,
    slot_map: NonNull<[u32VER]>,
    block_header: Option<u32VER>,
    block_offsets: NonNull<[u32VER]>,
    blocks: Box<[BlockVER]>,
    val: NonNull<[u32VER]>,
    mats: IndexMap<u32, mat::MatVER>,
    hk_constraint: Option<shape::HkConstraintVER>, // stores bone transforms used for ragdoll ??
    hk_constraint_datas: NonNull<[shape::HkConstraintDataVER]>,
    shapes: Box<[shape::ShapeVER]>,
    data: data::ModelDataVER
}

#[make_platforms]
unsafe impl Sync for ModelVER {}
#[make_platforms]
unsafe impl Send for ModelVER {}

#[make_platforms]
impl TryFrom<ModelRawVER> for ModelVER {
    type Error = anyhow::Error;
    fn try_from(val: ModelRawVER) -> Result<Self> {
        let data = data::ModelDataVER::try_from(&val).context("data")?;
        Ok(Self {
            _ptr: val._ptr,
            info: val.info,
            bones: val.bones,
            mat_order: val.mat_order,
            mesh_order: val.mesh_order,
            mesh_bounding_boxes: val.mesh_bounding_boxes,
            skin_binds: val.skin_binds,
            vals_j: val.vals_j,
            val_k_header: val.val_k_header,
            vals_k: val.vals_k,
            skin_order: val.skin_order,
            slots: val.slots,
            slot_map: val.slot_map,
            block_header: val.block_header,
            block_offsets: val.block_offsets,
            blocks: val.blocks,
            mats: val.mats,
            val: val.val,
            hk_constraint: val.hk_constraint,
            hk_constraint_datas: val.hk_constraint_datas,
            shapes: val.shapes,
            data
        })
    }
}

#[make_platforms]
impl ModelVER {
    pub fn info(&self) -> &ModelInfoVER {
        unsafe { self.info.as_ref() }
    }
    pub fn bones(&self) -> BonesRefVER<'_> {
        unsafe { self.bones.as_ref() }
    }
    pub fn mat_order(&self) -> &[u32VER] {
        unsafe { self.mat_order.as_ref() }
    }
    pub fn mesh_order(&self) -> &[u32VER] {
        unsafe { self.mesh_order.as_ref() }
    }
    pub fn mesh_bounding_boxes(&self) -> &[BoundingBoxVER] {
        unsafe { self.mesh_bounding_boxes.as_ref() }
    }
    pub fn skin_binds(&self) -> &[Matrix4x4VER] {
        unsafe { self.skin_binds.as_ref() }
    }
    pub fn vals_j(&self) -> &[u32VER] {
        unsafe { self.vals_j.as_ref() }
    }
    pub fn val_k_header(&self) -> &[u16VER] {
        unsafe { self.val_k_header.as_ref() }
    }
    pub fn vals_k(&self) -> &[u32VER] {
        unsafe { self.vals_k.as_ref() }
    }
    pub fn skin_order(&self) -> &[u32VER] {
        unsafe { self.skin_order.as_ref() }
    }
    pub fn slots(&self) -> &[Key2VER] {
        unsafe { self.slots.as_ref() }
    }
    pub fn slot_map(&self) -> &[u32VER] {
        unsafe { self.slot_map.as_ref() }
    }
    pub fn block_header(&self) -> Option<&u32VER> {
        self.block_header.as_ref()
    }
    pub fn block_offsets(&self) -> &[u32VER] {
        unsafe { self.block_offsets.as_ref() }
    }
    pub fn blocks(&self) -> &[BlockVER] {
        &self.blocks
    }
    pub fn val(&self) -> &[u32VER] {
        unsafe { self.val.as_ref() }
    }
    pub fn hk_constraint_datas(&self) -> &[shape::HkConstraintDataVER] {
        unsafe { self.hk_constraint_datas.as_ref() }
    }
    pub fn data(&self) -> &ModelDataVER {
        &self.data
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

#[make_platforms]
impl From<&ModelVER> for Model {
    fn from(val: &ModelVER) -> Self {
        Self {
            info: val.info().conv(),
            bones: val.bones().into(),
            mat_order: val
                .mat_order()
                .iter()
                .map(|x| val.mats.get_index_of(&x.get()).unwrap() as u32)
                .collect(),
            mesh_order: val.mesh_order().iter().map(|x| x.conv()).collect(),
            mesh_bounding_boxes: val.mesh_bounding_boxes().iter().map(|x| x.conv()).collect(),
            skin_binds: val.skin_binds().iter().map(|x| x.conv()).collect(),
            vals_j: val.vals_j().iter().map(|x| x.conv()).collect(),
            val_k_header: val.val_k_header().iter().map(|x| x.conv()).collect(),
            vals_k: val.vals_k().iter().map(|x| x.conv()).collect(),
            skin_order: val.skin_order().iter().map(|x| x.conv()).collect(),
            slots: val.slots().iter().map(|x| x.conv()).collect(),
            slot_map: val.slot_map().iter().map(|x| x.conv()).collect(),
            block_header: val.block_header().map(|x| x.conv()),
            blocks: val.blocks.iter().map(|x| x.into()).collect(),
            mats: val
                .mats
                .values()
                .map(|x| unsafe { x.as_ref() }.into())
                .collect(),
            hk_constraint: val.hk_constraint.as_ref().map(|x| x.into()),
            hk_constraint_datas: val.hk_constraint_datas().iter().map(|x| x.conv()).collect(),
            shapes: val.shapes.iter().map(|x| x.into()).collect(),
            data: val.data().into()
        }
    }
}

impl Model {
    #[make_platforms]
    pub fn from_ver(val: &ModelVER, _pak_header: &PakHeaderVER, _objs: &ObjsVER) -> Self {
        Self {
            info: val.info().conv(),
            bones: val.bones().into(),
            mat_order: val
                .mat_order()
                .iter()
                .map(|x| val.mats.get_index_of(&x.get()).unwrap() as u32)
                .collect(),
            mesh_order: val.mesh_order().iter().map(|x| x.conv()).collect(),
            mesh_bounding_boxes: val.mesh_bounding_boxes().iter().map(|x| x.conv()).collect(),
            skin_binds: val.skin_binds().iter().map(|x| x.conv()).collect(),
            vals_j: val.vals_j().iter().map(|x| x.conv()).collect(),
            val_k_header: val.val_k_header().iter().map(|x| x.conv()).collect(),
            vals_k: val.vals_k().iter().map(|x| x.conv()).collect(),
            skin_order: val.skin_order().iter().map(|x| x.conv()).collect(),
            slots: val.slots().iter().map(|x| x.conv()).collect(),
            slot_map: val.slot_map().iter().map(|x| x.conv()).collect(),
            block_header: val.block_header().map(|x| x.conv()),
            blocks: val.blocks.iter().map(|x| x.into()).collect(),
            mats: val
                .mats
                .values()
                .map(|x| unsafe { x.as_ref() }.into())
                .collect(),
            hk_constraint: val.hk_constraint.as_ref().map(|x| x.into()),
            hk_constraint_datas: val.hk_constraint_datas().iter().map(|x| x.conv()).collect(),
            shapes: val.shapes.iter().map(|x| x.into()).collect(),
            data: val.data().into()
        }
    }
}

#[make_platforms]
pub enum ModelPatchVER {
    Raw(ModelRawVER),
    Parsed(Model)
}

#[make_platforms]
pub trait DumpModelVER {
    fn bone_num(&self) -> usize;
    fn vals_j_num(&self) -> usize;
    fn skin_bind_num(&self) -> usize;
    fn has_skin_order(&self) -> bool;
    fn mat_num(&self) -> usize;
    fn shape_num(&self) -> usize;
    fn mesh_order_num(&self) -> usize;
    fn vbuff_num(&self) -> usize;
    fn ibuff_num(&self) -> usize;
    fn has_vals_k(&self) -> bool;
    fn slot_num(&self) -> usize;
    fn slot_map_num(&self) -> usize;
    fn block_num(&self) -> usize;
    fn hk_constraint_data_num(&self) -> usize;
    fn write_bones(&self, bones: BonesDumpVER) -> Result<()>;
    fn write_vals_j(&self, vals_j: &mut [u32VER]) -> Result<()>;
    fn write_skin_binds(&self, skin_binds: &mut [Matrix4x4VER]) -> Result<()>;
    fn write_skin_order(&self, skin_order: &mut [u32VER]) -> Result<()>;
    fn write_mesh_order(&self, mesh_order: &mut [u32VER]) -> Result<()>;
    fn write_mesh_bounding_boxes(&self, mesh_bboxes: &mut [BoundingBoxVER]) -> Result<()>;
    fn write_vbuff_order(&self, vbuff_order: &mut [u32VER]) -> Result<()>;
    fn write_ibuff_order(&self, ibuff_order: &mut [u32VER]) -> Result<()>;
    fn write_vals_k(&self, header: &mut [u16VER], vals_k: &mut [u32VER]) -> Result<()>;
    fn write_slots(&self, slots: &mut [Key2VER]) -> Result<()>;
    fn write_slot_map(&self, slot_map: &mut [u32VER]) -> Result<()>;
    fn write_hk_constraint_datas(&self, hk_constraint_datas: &mut [HkConstraintDataVER]) -> Result<()>;
    fn mat_order(&self) -> impl Iterator<Item=u32>;
    fn mats(&self) -> impl Iterator<Item=(u32, &impl DumpMatVER)>;
    fn shapes(&self) -> impl Iterator<Item=&impl DumpShapeVER>;
    fn hk_constraint(&self) -> Option<&impl DumpHkConstraintVER>;
    fn block_header(&self) -> Option<impl Into<u32VER>>;
    fn blocks(&self) -> impl Iterator<Item=&impl DumpBlockVER>;

    fn dump_misc(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER, info: &mut ModelInfoVER) -> Result<()> {
        if self.has_vals_k() {
            dst.align(16);
            info.vals_k_offset = dst.offset.conv();
            let vals_k_header = u16VER::mut_slice_from_data(dst, 2).context("val_k_header")?;
            let vals_k = u32VER::mut_slice_from_data(dst, 35).context("vals_k")?;
            self.write_vals_k(vals_k_header, vals_k).context("vals_k")?;
        } else {
            info.vals_k_offset = 0u32.conv();
        }

        if self.slot_num() != 0 {
            info.slots_offset = dst.offset.conv();
            let slots = Key2VER::mut_slice_from_data(dst, self.slot_num()).context("slots")?;
            self.write_slots(slots).context("write slots")?;
            info.slot_map_offset = dst.offset.conv();
            let slot_map = u32VER::mut_slice_from_data(dst, self.slot_map_num()).context("slot_map")?;
            self.write_slot_map(slot_map).context("write slot_map")?;
        } else {
            info.slots_offset = 0u32.conv();
        }

        if let Some(block_header) = self.block_header() {
            dst.align(16);
            let start = dst.offset;
            info.block_offset = start.conv();
            *u32VER::mut_from_data(dst).context("block_header")? = block_header.into();
            let block_offsets = u32VER::mut_slice_from_data(dst, self.block_num() + 1).context("block_offsets")?;
            for (i, (block, off)) in self.blocks().zip(block_offsets.iter_mut()).enumerate() {
                dst.align(16);
                *off = (dst.offset - start).conv();
                block.dump_into(dst).with_context(|| format!("block {}", i))?;
            }
            if let Some(off) = block_offsets.last_mut() {
                *off = (dst.offset - start).conv();
            }
        }

        info.hk_constraint_data_num = self.hk_constraint_data_num().conv();
        if info.hk_constraint_data_num.get() != 0 {
            info.hk_constraint_data_offset = infos.hk_constraint_datas.offset.conv();
            self.write_hk_constraint_datas(infos.hk_constraint_datas.next_slice(self.hk_constraint_data_num())).context("hk_constraint_datas")?;
        }
        Ok(())
    }

    fn add_misc_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        if self.has_vals_k() {
            offset = align_offset(offset, 16) + 2 * u16VER::size_of() + 35 * u32VER::size_of();
        }

        offset += Key2VER::size_of() * self.slot_num() + u32VER::size_of() * self.slot_map_num();

        if self.block_header().is_some() {
            offset = align_offset(offset, 16) + (2 + self.block_num()) * u32VER::size_of();
            for block in self.blocks() {
                offset = block.add_size(align_offset(offset, 16));
            }
        }

        counts.hk_constraint_datas += self.hk_constraint_data_num();

        offset
    }

    fn dump_mats(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER, info: &mut ModelInfoVER) -> Result<()> {
        let mut off = dst.offset;
        info.mat_offset = off.conv();
        let mat_map = self.mats().map(|(i, mat)| Ok((i, mat.dump_infos(infos)?))).collect::<Result<HashMap<_,_>>>().context("mats")?;
        let mat_order = u32VER::mut_slice_from_data(dst, self.mat_num()).context("mat_order")?;
        info.mat_num = mat_order.len().conv();
        for (i, mat) in self.mat_order().zip(mat_order) {
            *mat = mat_map.get(&i).ok_or(anyhow!("mats is missing item {}", i))?.conv(); 
            *infos.offsets.next() = off.conv();
            off += 4;
        }
        Ok(())
    }

    fn add_mat_counts(&self, counts: &mut InfoCounts) {
        for mat in self.mats() {
            mat.1.add_counts(counts);
        }
    }

    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER) -> Result<()> {
        let info = infos.models.next();
        
        info.bones_offset = dst.offset.conv();
        let bones = CrcVER::mut_slice_from_data(dst, self.bone_num()).context("bones")?;
        info.bones_num = bones.len().conv();

        dst.align(16);
        info.bone_bounding_boxes_offset = dst.offset.conv();
        let bone_bboxes = BoundingBoxVER::mut_slice_from_data(dst, bones.len()).context("bone_bounding_boxes")?;

        info.vals_j_offset = dst.offset.conv();
        let vals_j = u32VER::mut_slice_from_data(dst, self.vals_j_num()).context("vals_j")?;
        info.vals_j_num = vals_j.len().conv();
        self.write_vals_j(vals_j).context("write vals_j")?;

        if let Some(hk_constraint) = self.hk_constraint() {
            info.hk_constraint_offset = infos.hk_constraints.offset.conv();
            hk_constraint.dump_into(dst, infos, info.bones_offset.get() as u16, info.bones_num.get()).context("hk_constraint")?;
        } else {
            info.hk_constraint_offset = 0u32.conv();
        }

        dst.align(16);
        info.skin_binds_offset = dst.offset.conv();
        let skin_binds = Matrix4x4VER::mut_slice_from_data(dst, self.skin_bind_num()).context("skin_binds")?;
        info.skin_binds_num = skin_binds.len().conv();
        self.write_skin_binds(skin_binds).context("write skin_binds")?;

        if self.has_skin_order() {
            info.skin_order_offset = dst.offset.conv();
            let skin_order = u32VER::mut_slice_from_data(dst, skin_binds.len()).context("skin_order")?;
            self.write_skin_order(skin_order).context("write skin_order")?;
        } else {
            info.skin_order_offset = 0u32.conv();
        }

        info.bone_parents_offset = dst.offset.conv();
        let bone_parents = i32VER::mut_slice_from_data(dst, bones.len()).context("bone_parents")?;

        dst.align(16);
        info.bone_transforms_offset = dst.offset.conv();
        let bone_transforms = Matrix4x4VER::mut_slice_from_data(dst, bones.len()).context("bone_transforms")?;
        self.write_bones(BonesDumpVER {
            names: bones,
            parents: bone_parents,
            bounding_boxes: bone_bboxes,
            transforms: bone_transforms,
        }).context("write bones")?;

        self.dump_mats(dst, infos, info)?;

        info.shape_num = self.shape_num().conv();
        if info.shape_num.get() != 0 {
            info.shape_offset = infos.shapes.offset.conv();
            for (i, shape) in self.shapes().enumerate() {
                shape.dump_into(dst, infos, None).with_context(|| format!("shape {}", i))?;
            }
        } else {
            info.shape_offset = 0u32.conv();
        }

        info.mesh_order_offset = dst.offset.conv();
        let mesh_order = u32VER::mut_slice_from_data(dst, self.mesh_order_num()).context("mesh_order")?;
        // TODO need to rework how lod information is stored / presented
        // info.lod3.breakable_end = mesh_order.len();
        self.write_mesh_order(mesh_order).context("write mesh_order")?;

        dst.align(16);
        info.mesh_bounding_boxes_offset = dst.offset.conv();
        let mesh_bboxes = BoundingBoxVER::mut_slice_from_data(dst, mesh_order.len()).context("mesh_bounding_boxes")?;
        self.write_mesh_bounding_boxes(mesh_bboxes).context("write mesh_bounding_boxes")?;

        info.vbuff_offset = dst.offset.conv();
        let mut off = dst.offset;
        let vbuff_order = u32VER::mut_slice_from_data(dst, self.vbuff_num()).context("vbuff_order")?;
        for _ in 0..vbuff_order.len() {
            *infos.offsets.next() = off.conv();
            off += 4;
        }
        info.vbuff_num = vbuff_order.len().conv();
        self.write_vbuff_order(vbuff_order).context("write vbuff_order")?;

        info.ibuff_offset = dst.offset.conv();
        let mut off = dst.offset;
        let ibuff_order = u32VER::mut_slice_from_data(dst, self.ibuff_num()).context("ibuff_order")?;
        for _ in 0..ibuff_order.len() {
            *infos.offsets.next() = off.conv();
            off += 4;
        }
        info.ibuff_num = ibuff_order.len().conv();
        self.write_ibuff_order(ibuff_order).context("write ibuff_order")?;

        self.dump_misc(dst, infos, info)?;
        Ok(())
    }

    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        counts.models += 1;
        
        offset += CrcVER::size_of() * self.bone_num();
        offset = align_offset(offset, 16) + BoundingBoxVER::size_of() * self.bone_num();

        offset += u32VER::size_of() * self.vals_j_num();

        if let Some(hk_constraint) = self.hk_constraint() {
            offset = hk_constraint.add_size(offset, counts);
        }

        offset = align_offset(offset, 16) + Matrix4x4VER::size_of() * self.skin_bind_num();

        if self.has_skin_order() {
            offset += u32VER::size_of() * self.skin_bind_num();
        }

        offset += i32VER::size_of() * self.bone_num();
        offset = align_offset(offset, 16) + Matrix4x4VER::size_of() * self.bone_num();

        self.add_mat_counts(counts);

        for shape in self.shapes() {
            offset = shape.add_size(offset, counts);
        }

        offset += u32VER::size_of() * self.mesh_order_num();
        offset = align_offset(offset, 16) + BoundingBoxVER::size_of() * self.mesh_order_num();

        // need to handle mesh data properly
        counts.offsets += self.vbuff_num() + self.ibuff_num();
        offset += u32VER::size_of() * (self.vbuff_num() + self.ibuff_num());

        self.add_misc_size(offset, counts)
    }

    fn dump_terrain_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER, indices_offset: usize) -> Result<()> {
        let bone_bbox_offset = infos.models.offset + 16;
        let info = infos.models.next();

        info.bones_offset = 0u32.conv();
        info.bones_num = self.bone_num().conv();
        info.bone_bounding_boxes_offset = bone_bbox_offset.conv();
        info.skin_binds_offset = indices_offset.conv();
        info.skin_binds_num = 0u32.conv();
        info.skin_order_offset = 0u32.conv();
        info.bone_parents_offset = indices_offset.conv();

        if let Some(hk_constraint) = self.hk_constraint() {
            info.hk_constraint_offset = infos.hk_constraints.offset.conv();
            hk_constraint.dump_into(dst, infos, info.bones_offset.get() as u16, info.bones_num.get()).context("hk_constraint")?;
        } else {
            info.hk_constraint_offset = 0u32.conv();
        }

        let mut shape_offsets = Vec::with_capacity(self.shape_num());
        for (i, shape) in self.shapes().enumerate() {
            if let Some(extra) = shape.extra() {
                shape_offsets.push(Some(dst.offset));
                extra.dump_into(dst).with_context(|| format!("shape {} extra", i))?;
            } else {
                shape_offsets.push(None);
            }
        }
        
        dst.align(16);
        info.mesh_bounding_boxes_offset = dst.offset.conv();
        let mesh_bboxes = BoundingBoxVER::mut_slice_from_data(dst, self.mesh_order_num()).context("mesh_bounding_boxes")?;
        self.write_mesh_bounding_boxes(mesh_bboxes).context("write mesh_bounding_boxes")?;

        info.bone_transforms_offset = dst.offset.conv();
        let bone_transforms = Matrix4x4VER::mut_slice_from_data(dst, self.bone_num()).context("bone_transforms")?;
        self.write_bones(BonesDumpVER {
            names: &mut [],
            parents: &mut [],
            bounding_boxes: &mut [],
            transforms: bone_transforms,
        }).context("write bones")?;

        self.dump_mats(dst, infos, info)?;

        info.mesh_order_offset = dst.offset.conv();
        let mesh_order = u32VER::mut_slice_from_data(dst, self.mesh_order_num()).context("mesh_order")?;
        // TODO need to rework how lod information is stored / presented
        // info.lod3.breakable_end = mesh_order.len();
        self.write_mesh_order(mesh_order).context("write mesh_order")?;

        info.vbuff_offset = dst.offset.conv();
        let mut off = dst.offset;
        let vbuff_order = u32VER::mut_slice_from_data(dst, self.vbuff_num()).context("vbuff_order")?;
        for _ in 0..vbuff_order.len() {
            *infos.offsets.next() = off.conv();
            off += 4;
        }
        info.vbuff_num = vbuff_order.len().conv();
        self.write_vbuff_order(vbuff_order).context("write vbuff_order")?;

        let off_dest = dst.offset + 320;
        info.ibuff_offset = dst.offset.conv();
        let mut off = dst.offset;
        let ibuff_order = u32VER::mut_slice_from_data(dst, self.ibuff_num()).context("ibuff_order")?;
        for _ in 0..ibuff_order.len() {
            *infos.offsets.next() = off.conv();
            off += 4;
        }
        info.ibuff_num = ibuff_order.len().conv();
        self.write_ibuff_order(ibuff_order).context("write ibuff_order")?;

        self.dump_misc(dst, infos, info)?;

        if dst.offset < off_dest {
            *dst = dst.split(off_dest - dst.offset);
        }

        info.shape_num = self.shape_num().conv();
        if info.shape_num.get() != 0 {
            info.shape_offset = infos.shapes.offset.conv();
            for ((i, shape), off) in self.shapes().enumerate().zip(shape_offsets) {
                shape.dump_into(dst, infos, off).with_context(|| format!("shape {}", i))?;
            }
        } else {
            info.shape_offset = 0u32.conv();
        }
        Ok(())
    }

    fn add_terrain_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        counts.models += 1;

        if let Some(hk_constraint) = self.hk_constraint() {
            offset = hk_constraint.add_size(offset, counts);
        }

        for shape in self.shapes() {
            if let Some(extra) = shape.extra() {
                offset = extra.add_size(offset);
            }
        }
        
        offset = align_offset(offset, 16) + BoundingBoxVER::size_of() * self.mesh_order_num();
        offset += Matrix4x4VER::size_of() * self.bone_num();

        self.add_mat_counts(counts);

        offset += u32VER::size_of() * self.mesh_order_num();

        counts.offsets += self.vbuff_num() + self.ibuff_num();
        offset += u32VER::size_of() * self.vbuff_num();

        let off_dest = offset + 320;
        offset += u32VER::size_of() * self.ibuff_num();
        offset += self.add_misc_size(offset, counts);
        offset = off_dest.max(offset);

        for shape in self.shapes() {
            offset = shape.add_size(offset, counts);
        }
        offset
    }
}

#[make_platforms]
impl DumpModelVER for ModelRefVER<'_> {
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
        self.mats.len()
    }
    fn shape_num(&self) -> usize {
        self.shapes.len()
    }
    fn mesh_order_num(&self) -> usize {
        self.mesh_order.len()
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
    fn write_bones(&self, BonesDumpVER { names, parents, transforms, bounding_boxes }: BonesDumpVER) -> Result<()> {
        names.write_from(&self.bones.names[..])?;
        parents.write_from(&self.bones.parents[..])?;
        transforms.write_from(&self.bones.transforms[..])?;
        bounding_boxes.write_from(&self.bones.bounding_boxes[..])
    }
    fn write_vals_j(&self, vals_j: &mut [u32VER]) -> Result<()> {
        vals_j.write_from(&self.vals_j[..])
    }
    fn write_skin_binds(&self, skin_binds: &mut [Matrix4x4VER]) -> Result<()> {
        skin_binds.write_from(&self.skin_binds[..])
    }
    fn write_skin_order(&self, skin_order: &mut [u32VER]) -> Result<()> {
        skin_order.write_from(&self.skin_order[..])
    }
    fn write_mesh_order(&self, mesh_order: &mut [u32VER]) -> Result<()> {
        mesh_order.write_from(&self.mesh_order[..])
    }
    fn write_mesh_bounding_boxes(&self, mesh_bboxes: &mut [BoundingBoxVER]) -> Result<()> {
        mesh_bboxes.write_from(&self.mesh_bounding_boxes[..])
    }
    fn write_vbuff_order(&self, vbuff_order: &mut [u32VER]) -> Result<()> {
        vbuff_order.write_from(&self.vbuff_order[..])
    }
    fn write_ibuff_order(&self, ibuff_order: &mut [u32VER]) -> Result<()> {
        ibuff_order.write_from(&self.ibuff_order[..])
    }
    fn write_vals_k(&self, header: &mut [u16VER], vals_k: &mut [u32VER]) -> Result<()>  {
        header.write_from(&self.val_k_header[..])?;
        vals_k.write_from(&self.vals_k[..])
    }
    fn write_slots(&self, slots: &mut [Key2VER]) -> Result<()> {
        slots.write_from(&self.slots[..])
    }
    fn write_slot_map(&self, slot_map: &mut [u32VER]) -> Result<()> {
        slot_map.write_from(&self.slot_map[..])
    }
    fn write_hk_constraint_datas(&self, hk_constraint_datas: &mut [HkConstraintDataVER]) -> Result<()> {
        hk_constraint_datas.write_from(&self.hk_constraint_datas[..])
    }
    fn mat_order(&self) -> impl Iterator<Item=u32> {
        self.mat_order.iter().map(|x: &u32VER| x.get())
    }
    fn mats(&self) -> impl Iterator<Item=(u32, &impl DumpMatVER)> {
        self.mats.iter().map(|(k, v)| (*k, v))
    }
    fn shapes(&self) -> impl Iterator<Item=&impl DumpShapeVER> {
        self.shapes.iter()
    }
    fn hk_constraint(&self) -> Option<&impl DumpHkConstraintVER> {
        self.hk_constraint.as_ref()
    }
    fn block_header(&self) -> Option<impl Into<u32VER>> {
        self.block_header.cloned()
    }
    fn blocks(&self) -> impl Iterator<Item=&impl DumpBlockVER> {
        self.blocks.iter()
    }
}
