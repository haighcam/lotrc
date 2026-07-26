use anyhow::{Context, Result};
use log::warn;
use zerocopy::transmute_ref;
use enum_dispatch::enum_dispatch;

use crate::{
    types::{Crc, DumpData, RefFromData, Vector3, Vector4, DumpSlice, align_offset, hash_string, OrderedData, get_default_ref, ref_slice, string, slice},
    level::{
        pak::block1::infos::InfoCounts,
        model::Key2
    }
};
#[make_endian]
use crate::{
    types::{Vector3_XE_, Vector4_XE_, f32_XE_, i16_XE_, u16_XE_, u32_XE_, Crc_XE_},
    level::{
        model::Key2_XE_,
        pak::block1::infos::DumpInfos_XE_
    }
};
use lotrc_proc::{make_endian, derive_ordered_data};

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ShapeInfo_XE_ {
    pub offset: u32_XE_, // sometimes a pointer to something, otherwise the number of strings from the obj1 pointing to this
    pub kind: u32_XE_,   // 0, 1, 2, 3, 4, 5
    pub unk_2: u32_XE_,
    pub unk_3: f32_XE_,
    pub unk_4: f32_XE_,
    pub unk_5: f32_XE_,
    pub translation: Vector3_XE_,
    pub rotation: Vector4_XE_,
    pub unk_13: f32_XE_,
    pub unk_14: f32_XE_,
    pub unk_15: f32_XE_,
    pub unk_16: f32_XE_,
    pub unk_17: f32_XE_,
    pub unk_18: f32_XE_,
    pub unk_19: f32_XE_,
    pub unk_20: f32_XE_,
    pub unk_21: f32_XE_,
    pub unk_22: f32_XE_,
    pub unk_23: f32_XE_,
    pub unk_24: f32_XE_,
    pub unk_25: f32_XE_,
    pub unk_26: f32_XE_,
    pub hk_shape_num: u32_XE_,
    pub hk_shape_offset: u32_XE_, // pointer to objd
    pub unk_29a: u8,
    pub unk_29b: u8,
    pub unk_29c: u8,
    pub unk_29d: u8,
    pub unk_30: f32_XE_,
}

#[derive_ordered_data]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ShapeExtraInfo_XE_ {
    pub size: u32_XE_,
    pub scale: f32_XE_,
    pub a: f32_XE_,
    pub b: f32_XE_,
}

#[make_endian]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct ShapeExtraRef_XE_<'a> {
    info: &'a ShapeExtraInfo_XE_,
    offs: ref_slice<'a, u32_XE_>,
    data: ref_slice<'a, u8>,
}

#[make_endian]
impl<'a> ShapeExtraRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a ShapeInfo_XE_) -> Result<Self> {
        let mut offset = info.offset.conv();
        let info = ShapeExtraInfo_XE_::from_data(&src[offset..]).context("info")?;
        offset += info.size();
        let offs = u32_XE_::slice_from_data(&src[offset..], info.size.conv())
            .context("vals")?;
        offset += offs.size();
        let mut off = offset + offs.last().unwrap().to_native() as usize;
        // sketchy stuff to account for missing data
        while (src[off] != 0) || (src[off + 1] != 0) {
            off += 1;
        }
        let data = &src[offset..off];
        Ok(Self {
            info: info,
            offs: offs.into(),
            data: data.into(),
        })
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ShapeRef_XE_<'a> {
    info: &'a ShapeInfo_XE_,
    extra: Option<ShapeExtraRef_XE_<'a>>,
    hk_shapes: slice<HkShapeRef_XE_<'a>>
}

#[make_endian]
impl<'a> ShapeRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a ShapeInfo_XE_) -> Result<Self> {
        let extra = if info.kind == 0 {
            Some(ShapeExtraRef_XE_::from_data(src, info).context("extra")?)
        } else {
            None
        };
        let infos = HkShapeInfo_XE_::slice_from_data(&src[info.hk_shape_offset.conv()..], info.hk_shape_num.conv()).context("hk_shape infos")?;
        let hk_shapes = infos.into_iter().enumerate().map(|(i, info)| HkShapeRef_XE_::from_data(src, info).with_context(|| format!("hk_shape {}", i))).collect::<Result<Vec<_>>>()?.into_boxed_slice();
        Ok(Self {
            info,
            extra: extra.into(),
            hk_shapes: hk_shapes.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShapeExtra {
    info: ShapeExtraInfo,
    offs: Vec<u32>,
    data: Vec<u8>,
}

#[make_endian]
impl From<&ShapeExtraRef_XE_<'_>> for ShapeExtra {
    fn from(val: &ShapeExtraRef_XE_) -> Self {
        ShapeExtra {
            info: val.info.conv(),
            offs: val.offs.iter().map(|x| x.conv()).collect(),
            data: val.data.to_vec(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub info: ShapeInfo,
    pub extra: Option<ShapeExtra>,
    pub hk_shapes: Vec<HkShape>,
}

#[make_endian]
impl From<&ShapeRef_XE_<'_>> for Shape {
    fn from(val: &ShapeRef_XE_) -> Self {
        Shape {
            info: val.info.conv(),
            extra: val.extra.as_ref().map(|x| x.into()),
            hk_shapes: val
                .hk_shapes
                .iter()
                .map(|x| x.into())
                .collect(),
        }
    }
}

#[make_endian]
pub struct ShapeExtraDump_XE_<'a> {
    info: &'a mut ShapeExtraInfo_XE_,
    offs: &'a mut [u32_XE_],
    data: &'a mut [u8],
}

#[make_endian]
impl<'a> ShapeExtraDump_XE_<'a> {
    fn from_bytes(dst: &mut DumpSlice<'a>, sizes: (usize, usize)) -> Result<Self> {
        let info = ShapeExtraInfo_XE_::mut_from_data(dst).context("info")?;
        let offs = u32_XE_::mut_slice_from_data(dst, sizes.0).context("offs")?;
        let data = u8::mut_slice_from_data(dst, sizes.1).context("data")?;
        Ok(Self { info, offs, data })
    }
}

#[make_endian]
pub trait DumpShapeExtra_XE_ {
    fn get_sizes(&self) -> (usize, usize);
    fn write_vals(&self, vals: ShapeExtraDump_XE_) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let vals = ShapeExtraDump_XE_::from_bytes(dst, self.get_sizes()).context("vals")?;
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
    fn add_size(&self, mut offset: usize) -> usize {
        let sizes = self.get_sizes();
        offset += ShapeExtraInfo_XE_::size_of() + sizes.0 * u32_XE_::size_of() + sizes.1;
        offset
    }
}

#[make_endian]
#[enum_dispatch(DumpShape_XE_)]
pub enum Shape_XE_<'a> {
    Ref(ShapeRef_XE_<'a>),
    Owned(Shape)
}

#[make_endian]
impl DumpShapeExtra_XE_ for ShapeExtraRef_XE_<'_> {
    fn get_sizes(&self) -> (usize, usize) {
        (self.offs.len(), self.data.len())
    }
    fn write_vals(&self, vals: ShapeExtraDump_XE_) -> Result<()> {
        vals.info.write_from(self.info).context("info")?;
        vals.offs.write_from(&self.offs[..]).context("offs")?;
        vals.data.write_from(&self.data[..]).context("data")?;
        Ok(())
    }
}

#[make_endian]
impl DumpShapeExtra_XE_ for ShapeExtra {
    fn get_sizes(&self) -> (usize, usize) {
        (self.offs.len(), self.data.len())
    }
    fn write_vals(&self, vals: ShapeExtraDump_XE_) -> Result<()> {
        *vals.info = (&self.info).conv();
        for (src, dst) in self.offs.iter().zip(vals.offs) {
            *dst = src.conv();
        }
        vals.data.write_from(&self.data).context("data")?;
        Ok(())
    }
} 

#[make_endian]
pub trait DumpShapeImpl_XE_ {
    fn extra(&self) -> Option<&impl DumpShapeExtra_XE_>;
    fn has_hk_shapes(&self) -> bool;
    fn hk_shapes(&self) -> impl Iterator<Item=&impl DumpHkShape_XE_>;
    fn write_info(&self, info: &mut ShapeInfo_XE_) -> Result<()>;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpShape_XE_ {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_, extra_off: Option<usize>) -> Result<()>;
    fn add_size(&self, offset: usize, infos: &mut InfoCounts) -> usize;
    fn add_extra_size(&self, offset: usize) -> usize;
    fn dump_extra_into(&self, dst: &mut DumpSlice) -> Result<Option<usize>>;
}
#[make_endian]
impl<T: DumpShapeImpl_XE_> DumpShape_XE_ for T {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_, extra_off: Option<usize>) -> Result<()> {
        let info_offset = infos.shapes.offset;
        let info = infos.shapes.next().context("shapes")?;
        self.write_info(info).context("info")?;
        if let Some(off) = extra_off {
            info.offset = off.conv();
            *infos.offsets.next().context("offsets")? = (info_offset + std::mem::offset_of!(ShapeInfo_XE_, offset)).conv();
        }
        if self.has_hk_shapes() {
            info.hk_shape_offset = infos.hk_shapes.offset.conv();
            *infos.offsets.next().context("offsets")? = (info_offset + std::mem::offset_of!(ShapeInfo_XE_, hk_shape_offset)).conv();
        }
        for (i, shape) in self.hk_shapes().enumerate() {
            shape.dump_into(dst, infos).with_context(|| format!("hk_shape {}", i))?;
        }
        Ok(())
    }
    fn add_size(&self, mut offset: usize, infos: &mut InfoCounts) -> usize {
        infos.shapes += 1;
        if let Some(extra) = self.extra() {
            offset = extra.add_size(offset);
            infos.offsets += 1;
        }
        if self.has_hk_shapes() {
            infos.offsets += 1;
        }
        for shape in self.hk_shapes() {
            offset = shape.add_size(offset, infos);
        }
        offset
    }
    fn add_extra_size(&self, offset: usize) -> usize {
        self.extra().map(|x| x.add_size(offset)).unwrap_or(offset)
    }
    fn dump_extra_into(&self, dst: &mut DumpSlice) -> Result<Option<usize>> {
        Ok(if let Some(x) = self.extra() {
            let offset = dst.offset;
            x.dump_into(dst)?;
            Some(offset)
        } else {
            None
        })
    }
}

#[make_endian]
impl DumpShapeImpl_XE_ for ShapeRef_XE_<'_> {
    fn extra(&self) -> Option<&impl DumpShapeExtra_XE_> {
        self.extra.as_ref()
    }
    fn has_hk_shapes(&self) -> bool {
        !self.hk_shapes.is_empty()    
    }
    fn hk_shapes(&self) -> impl Iterator<Item=&impl DumpHkShape_XE_> {
        self.hk_shapes.iter()
    }
    fn write_info(&self, info: &mut ShapeInfo_XE_) -> Result<()> {
        info.write_from(self.info)
    }
}

#[make_endian]
impl DumpShapeImpl_XE_ for Shape {
    fn extra(&self) -> Option<&impl DumpShapeExtra_XE_> {
        self.extra.as_ref()
    }
    fn has_hk_shapes(&self) -> bool {
        !self.hk_shapes.is_empty()    
    }
    fn hk_shapes(&self) -> impl Iterator<Item=&impl DumpHkShape_XE_> {
        self.hk_shapes.iter()
    }
    fn write_info(&self, info: &mut ShapeInfo_XE_) -> Result<()> {
        *info = (&self.info).conv();
        Ok(())
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct HkShapeInfo_XE_ {
    pub unk_0: Vector4_XE_,
    pub unk_4: Vector4_XE_,
    pub kind: u32_XE_,
    pub unk_9: u32_XE_,
    pub unk_10: u32_XE_,
    pub unk_11: u32_XE_,
    pub unk_12: u32_XE_,
    pub unk_13: u32_XE_,
    pub unk_14: u32_XE_,
    pub unk_15: u32_XE_,
    pub unk_16: u32_XE_,
    pub unk_17: u32_XE_,
    pub unk_18: u32_XE_,
    pub unk_19: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BoxShape_XE_ {
    pub translation: Vector4_XE_,
    pub rotation: Vector4_XE_,
    pub kind: u32_XE_,
    pub key: Crc_XE_,
    pub half_extents: Vector3_XE_,
    pub unk_13: u32_XE_,
    pub unk_14: u32_XE_,
    pub unk_15: f32_XE_,
    pub unk_16: f32_XE_,
    pub unk_17: f32_XE_,
    pub unk_18: f32_XE_,
    pub unk_19: f32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SphereShape_XE_ {
    pub translation: Vector4_XE_,
    pub rotation: Vector4_XE_,
    pub kind: u32_XE_,
    pub key: Crc_XE_,
    pub radius: f32_XE_,
    pub unk_11: u32_XE_,
    pub unk_12: f32_XE_,
    pub unk_13: f32_XE_,
    pub unk_14: f32_XE_,
    pub unk_15: f32_XE_,
    pub unk_16: f32_XE_,
    pub unk_17: f32_XE_,
    pub unk_18: f32_XE_,
    pub unk_19: f32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CapsuleShape_XE_ {
    pub translation: Vector4_XE_,
    pub rotation: Vector4_XE_,
    pub kind: u32_XE_,
    pub key: Crc_XE_,
    pub point1: Vector3_XE_,
    pub point2: Vector3_XE_,
    pub radius: f32_XE_,
    pub unk_17: f32_XE_,
    pub unk_18: f32_XE_,
    pub unk_19: f32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CylinderShape_XE_ {
    pub translation: Vector4_XE_,
    pub rotation: Vector4_XE_,
    pub kind: u32_XE_,
    pub key: Crc_XE_,
    pub point1: Vector3_XE_,
    pub point2: Vector3_XE_,
    pub radius: f32_XE_,
    pub unk_17: f32_XE_,
    pub unk_18: f32_XE_,
    pub unk_19: f32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ConvexVerticesInfo_XE_ {
    pub translation: Vector4_XE_,
    pub rotation: Vector4_XE_,
    pub kind: u32_XE_,
    pub key: Crc_XE_,
    pub norm_num: u32_XE_,
    pub norms_offset: u32_XE_,
    pub vert_num: u32_XE_,
    pub verts_offset: u32_XE_,
    pub unk_14: f32_XE_,
    pub unk_15: f32_XE_,
    pub unk_16: f32_XE_,
    pub unk_17: f32_XE_,
    pub unk_18: f32_XE_,
    pub unk_19: f32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BVTreeMeshInfo_XE_ {
    pub translation: Vector4_XE_,
    pub rotation: Vector4_XE_,
    pub kind: u32_XE_,
    pub key: Crc_XE_,
    /// triangles min bound - 0.05
    pub offset: Vector3_XE_,
    /// 254*256*256 / (max triangle bound + 0.1)
    pub tree_scale: f32_XE_,
    pub tree_size: u32_XE_,
    pub tree_offset: u32_XE_,  // u8
    pub vert_num: u32_XE_,     // vert_num
    pub verts_offset: u32_XE_, // vec3 f32 verts offset
    pub tri_num: u32_XE_,      // tri_num
    pub inds_offset: u32_XE_,  // vec3 u16 , inds offset
}
#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ConvexVerticesRef_XE_<'a> {
    pub info: &'a ConvexVerticesInfo_XE_,
    pub norms: ref_slice<'a, Vector4_XE_>,
    pub verts: ref_slice<'a, Vector3_XE_>,
}

#[make_endian]
impl<'a> ConvexVerticesRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a HkShapeInfo_XE_) -> Result<Self> {
        let info: &ConvexVerticesInfo_XE_ = transmute_ref!(info);

        let norms = Vector4_XE_::slice_from_data(
            &src[info.norms_offset.conv()..],
            info.norm_num.conv(),
        )
        .context("vals")?;
        let mut vert_num = info.vert_num.conv(); // sketchy stuff to account for data that was not otherwise captured, is it needed?
        while (info.verts_offset.to_native() as usize + vert_num * 12) % 16 != 0 {
            vert_num += 1;
        }
        let verts = Vector3_XE_::slice_from_data(
            &src[info.verts_offset.conv()..],
            vert_num,
        )
        .context("vals")?;
        Ok(Self {
            info,
            norms: norms.into(),
            verts: verts.into(),
        })
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct BVTreeMeshRef_XE_<'a> {
    info: &'a BVTreeMeshInfo_XE_,
    tree: ref_slice<'a, u8>,
    verts: ref_slice<'a, Vector3_XE_>,
    inds: ref_slice<'a, u16_XE_>
}

#[make_endian]
impl<'a> BVTreeMeshRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a HkShapeInfo_XE_) -> Result<Self> {
        let info: &BVTreeMeshInfo_XE_ = transmute_ref!(info);

        let tree = u8::slice_from_data(
            &src[info.tree_offset.conv()..],
            info.tree_size.conv(),
        )
        .context("vals")?;
        let verts = Vector3_XE_::slice_from_data(
            &src[info.verts_offset.conv()..],
            info.vert_num.conv(),
        )
        .context("vals")?;
        let inds = u16_XE_::slice_from_data(
            &src[info.inds_offset.conv()..],
            info.tri_num.to_native() as usize * 3,
        )
        .context("vals")?;

        Ok(Self {
            info: info,
            tree: tree.into(),
            verts: verts.into(),
            inds: inds.into(),
        })
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C, u8))]
#[derive(PartialEq)]
pub enum HkShapeRef_XE_<'a> {
    Box(&'a BoxShape_XE_),
    Sphere(&'a SphereShape_XE_),
    Capsule(&'a CapsuleShape_XE_),
    Cylinder(&'a CylinderShape_XE_),
    ConvexVertices(ConvexVerticesRef_XE_<'a>),
    BVTreeMesh(BVTreeMeshRef_XE_<'a>),
    Unknown(&'a HkShapeInfo_XE_),
}

#[make_endian]
impl<'a> HkShapeRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a HkShapeInfo_XE_) -> Result<Self> {
        Ok(match info.kind.conv() {
            1u32 => Self::Box(transmute_ref!(info)),
            2 => Self::Sphere(transmute_ref!(info)),
            3 => Self::Capsule(transmute_ref!(info)),
            4 => Self::Cylinder(transmute_ref!(info)),
            5 => Self::ConvexVertices(ConvexVerticesRef_XE_::from_data(src, info)?),
            6 => Self::BVTreeMesh(BVTreeMeshRef_XE_::from_data(src, info)?),
            _ => {
                warn!("Unknown & Unhandled HkShape type {}", info.kind);
                Self::Unknown(info)
            }
        })
    }
}

#[derive(Debug, Clone)]
pub enum HkShape {
    Box(BoxShape),
    Sphere(SphereShape),
    Capsule(CapsuleShape),
    Cylinder(CylinderShape),
    ConvexVertices {
        info: ConvexVerticesInfo,
        norms: Vec<Vector4>,
        verts: Vec<Vector3>,
    },
    BVTreeMesh {
        info: BVTreeMeshInfo,
        tree: Vec<u8>,
        verts: Vec<Vector3>,
        inds: Vec<u16>,
    },
    Unknown(HkShapeInfo),
}

#[make_endian]
impl From<&HkShapeRef_XE_<'_>> for HkShape {
    fn from(val: &HkShapeRef_XE_) -> Self {
        match val {
            HkShapeRef_XE_::Box(info) => HkShape::Box(info.conv()),
            HkShapeRef_XE_::Sphere(info) => HkShape::Sphere(info.conv()),
            HkShapeRef_XE_::Capsule(info) => HkShape::Capsule(info.conv()),
            HkShapeRef_XE_::Cylinder(info) => HkShape::Cylinder(info.conv()),
            HkShapeRef_XE_::ConvexVertices(ConvexVerticesRef_XE_ { info, norms, verts }) => HkShape::ConvexVertices {
                info: info.conv(),
                norms: norms.iter().map(|x| x.conv()).collect(),
                verts: verts.iter().map(|x| x.conv()).collect(),
            },
            HkShapeRef_XE_::BVTreeMesh(BVTreeMeshRef_XE_ {
                info,
                tree,
                verts,
                inds,
            }) => HkShape::BVTreeMesh {
                info: info.conv(),
                tree: tree.iter().cloned().collect(),
                verts: verts.iter().map(|x| x.conv()).collect(),
                inds: inds.iter().map(|x| x.conv()).collect(),
            },
            HkShapeRef_XE_::Unknown(info) => HkShape::Unknown(info.conv()),
        }
    }
}

#[make_endian]
#[enum_dispatch(DumpHkShape_XE_)]
pub enum HkShape_XE_<'a> {
    Ref(HkShapeRef_XE_<'a>),
    Owned(HkShape)
}

#[make_endian]
pub enum HkShapeDump_XE_<'a> {
    ConvexVertices {
        norms: &'a mut [Vector4_XE_],
        verts: &'a mut [Vector3_XE_],
    },
    BVTreeMesh {
        tree: &'a mut [u8],
        verts: &'a mut [Vector3_XE_],
        inds: &'a mut [u16_XE_],
    },
    None
}

#[make_endian]
impl<'a> HkShapeDump_XE_<'a> {
    fn from_bytes(dst: &mut DumpSlice<'a>, sizes: (usize, usize, usize), info: &mut HkShapeInfo_XE_) -> Result<Self> {
        Ok(match info.kind.conv() {
            5u32 => {
                let info: &mut ConvexVerticesInfo_XE_ = zerocopy::transmute_mut!(info);

                dst.align(16)?;
                info.norms_offset = dst.offset.conv();
                let norms = Vector4_XE_::mut_slice_from_data(dst, sizes.0).context("norms")?;
                info.norm_num = norms.len().conv();

                info.verts_offset = dst.offset.conv();
                let verts = Vector3_XE_::mut_slice_from_data(dst, sizes.1).context("verts")?;
                info.vert_num = (verts.len() - sizes.2).conv();
                Self::ConvexVertices { norms, verts }
            }
            6 => {
                let info: &mut BVTreeMeshInfo_XE_ = zerocopy::transmute_mut!(info);

                info.verts_offset = dst.offset.conv();
                let verts = Vector3_XE_::mut_slice_from_data(dst, sizes.0).context("verts")?;
                info.vert_num = verts.len().conv();

                info.inds_offset = dst.offset.conv();
                let inds = u16_XE_::mut_slice_from_data(dst, sizes.1).context("inds")?;
                info.tri_num = (inds.len() / 3).conv();

                dst.align(4)?;
                info.tree_offset = dst.offset.conv();
                let tree = u8::mut_slice_from_data(dst, sizes.2).context("tree")?;
                info.tree_size = tree.len().conv();
                dst.align(4)?;
                Self::BVTreeMesh { tree, verts, inds }
            }
            _ => Self::None
        })
    }
}

#[make_endian]
#[enum_dispatch]
pub trait DumpHkShape_XE_ {
    fn kind(&self) -> u32;
    fn get_sizes(&self) -> (usize, usize, usize);
    fn write_info(&self, info: &mut HkShapeInfo_XE_) -> Result<()>;
    fn write_vals(&self, vals: HkShapeDump_XE_) -> Result<()>;
    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        counts.hk_shapes += 1;
        match self.kind() {
            5 => {
                counts.offsets += 2;
                let sizes = self.get_sizes();
                offset = align_offset(offset, 16);
                offset += sizes.0 * Vector4_XE_::size_of();
                offset += sizes.1 * Vector3_XE_::size_of();
            }
            6 => {
                counts.offsets += 3;
                let sizes = self.get_sizes();
                offset += sizes.0 * Vector3_XE_::size_of();
                offset += sizes.1 * u16_XE_::size_of();
                offset = align_offset(offset, 4);
                offset += sizes.2;
                offset = align_offset(offset, 4);
            }
            _ => ()
        }
        offset
    }
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_) -> Result<()> {
        let info_off = infos.hk_shapes.offset;
        let info = infos.hk_shapes.next().context("hk_shapes")?;
        self.write_info(info).context("write info")?;
        let vals = HkShapeDump_XE_::from_bytes(dst, self.get_sizes(), info).context("vals")?;
        self.write_vals(vals).context("write vals")?;
        match self.kind() {
            5 => {
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ConvexVerticesInfo_XE_, norms_offset)) as u32).conv();
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ConvexVerticesInfo_XE_, verts_offset)) as u32).conv();
            }
            6 => {
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(BVTreeMeshInfo_XE_, verts_offset)) as u32).conv();
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(BVTreeMeshInfo_XE_, inds_offset)) as u32).conv();
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(BVTreeMeshInfo_XE_, tree_offset)) as u32).conv();

            }
            _ => ()
        }
        Ok(())
    }
}

#[make_endian]
impl DumpHkShape_XE_ for HkShapeRef_XE_<'_> { 
    fn kind(&self) -> u32 {
        match self {
            Self::Box(info) => info.kind.conv(),
            Self::Sphere(info) => info.kind.conv(),
            Self::Capsule(info) => info.kind.conv(),
            Self::Cylinder(info) => info.kind.conv(),
            Self::ConvexVertices(val) => val.info.kind.conv(),
            Self::BVTreeMesh(val) => val.info.kind.conv(),
            Self::Unknown(info) => info.kind.conv(),
        }
    }
    fn get_sizes(&self) -> (usize, usize, usize) {
        match self {
            Self::ConvexVertices(ConvexVerticesRef_XE_ { info, verts, norms }) => (norms.len(), verts.len(), verts.len() - info.vert_num.to_native() as usize),
            Self::BVTreeMesh(BVTreeMeshRef_XE_ { tree, verts, inds, .. }) => (verts.len(), inds.len(), tree.len()),
            _ => (0,0,0)
        }
    }
    fn write_info(&self, info: &mut HkShapeInfo_XE_) -> Result<()> {
        match self {
            Self::Box(src) => info.write_from(zerocopy::transmute_ref!(*src)),
            Self::Sphere(src) => info.write_from(zerocopy::transmute_ref!(*src)),
            Self::Capsule(src) => info.write_from(zerocopy::transmute_ref!(*src)),
            Self::Cylinder(src) => info.write_from(zerocopy::transmute_ref!(*src)),
            Self::ConvexVertices(val) => info.write_from(zerocopy::transmute_ref!(val.info)),
            Self::BVTreeMesh(val) => info.write_from(zerocopy::transmute_ref!(val.info)),
            Self::Unknown(src) => info.write_from(src),
        }
    }
    fn write_vals(&self, vals: HkShapeDump_XE_) -> Result<()> {
        match (self, vals) {
            (Self::ConvexVertices(ConvexVerticesRef_XE_ { verts: src1, norms: src2, .. }), HkShapeDump_XE_::ConvexVertices { verts: dst1, norms: dst2 }) => {
                dst1.write_from(src1).context("verts")?;
                dst2.write_from(src2).context("norms")?;
            },
            (Self::BVTreeMesh(BVTreeMeshRef_XE_ { verts: src1, inds: src2, tree: src3, .. }), HkShapeDump_XE_::BVTreeMesh { verts: dst1, inds: dst2, tree: dst3 }) => {
                dst1.write_from(src1).context("verts")?;
                dst2.write_from(src2).context("inds")?;
                dst3.write_from(src3).context("tree")?;
            },
            _ => ()
        }
        Ok(())
    }
}

#[make_endian]
impl DumpHkShape_XE_ for HkShape { 
    fn kind(&self) -> u32 {
        match self {
            HkShape::Box(info) => info.kind,
            HkShape::Sphere(info) => info.kind,
            HkShape::Capsule(info) => info.kind,
            HkShape::Cylinder(info) => info.kind,
            HkShape::ConvexVertices { info, .. } => info.kind,
            HkShape::BVTreeMesh { info, .. } => info.kind,
            HkShape::Unknown(info) => info.kind,
        }
    }
    fn get_sizes(&self) -> (usize, usize, usize) {
        match self {
            HkShape::ConvexVertices { info, verts, norms } => (norms.len(), verts.len(), verts.len() - info.vert_num as usize),
            HkShape::BVTreeMesh { tree, verts, inds, .. } => (verts.len(), inds.len(), tree.len()),
            _ => (0,0,0)
        }
    }
    fn write_info(&self, info: &mut HkShapeInfo_XE_) -> Result<()> {
        match self {
            HkShape::Box(src) => *zerocopy::transmute_mut!(info) = OrderedData::<BoxShape_XE_>::conv(src),
            HkShape::Sphere(src) => *zerocopy::transmute_mut!(info) = OrderedData::<SphereShape_XE_>::conv(src),
            HkShape::Capsule(src) => *zerocopy::transmute_mut!(info) = OrderedData::<CapsuleShape_XE_>::conv(src),
            HkShape::Cylinder(src) => *zerocopy::transmute_mut!(info) = OrderedData::<CylinderShape_XE_>::conv(src),
            HkShape::ConvexVertices { info: src, .. } => *zerocopy::transmute_mut!(info) = OrderedData::<ConvexVerticesInfo_XE_>::conv(src),
            HkShape::BVTreeMesh { info: src, .. } => *zerocopy::transmute_mut!(info) = OrderedData::<BVTreeMeshInfo_XE_>::conv(src),
            HkShape::Unknown(src) => *info = src.conv(),
        }
        Ok(())
    }
    fn write_vals(&self, vals: HkShapeDump_XE_) -> Result<()> {
        match (self, vals) {
            (HkShape::ConvexVertices { verts: src1, norms: src2, .. }, HkShapeDump_XE_::ConvexVertices { verts: dst1, norms: dst2 }) => {
                for (src, dst) in src1.iter().zip(dst1) {
                    *dst = src.conv();
                }
                for (src, dst) in src2.iter().zip(dst2) {
                    *dst = src.conv();
                }
            },
            (HkShape::BVTreeMesh { verts: src1, inds: src2, tree: src3, .. }, HkShapeDump_XE_::BVTreeMesh { verts: dst1, inds: dst2, tree: dst3 }) => {
                for (src, dst) in src1.iter().zip(dst1) {
                    *dst = src.conv();
                }
                for (src, dst) in src2.iter().zip(dst2) {
                    *dst = src.conv();
                }
                dst3.write_from(src3).context("tree")?;
            },
            _ => ()
        }
        Ok(())
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct HkConstraintInfo_XE_ {
    pub kind: u32_XE_,
    pub bone_parents_offset: u32_XE_,
    pub bone_parents_num: u32_XE_,
    pub bone_names_offset: u32_XE_,
    pub bone_names_num: u32_XE_,
    pub bone_transforms_offset: u32_XE_,
    pub bone_transforms_num: u32_XE_,
    pub unk_7: u32_XE_,
    pub unk_8: u32_XE_,
    pub unk_9: u32_XE_,
    pub bones_offset: u32_XE_,
    pub bones_num: u16_XE_,
    pub bone_order_num: u16_XE_,
    pub bone_order_offset: u32_XE_,
    pub unk_13: u32_XE_,
    pub unk_14: f32_XE_,
    pub vals2_num: u32_XE_,
    pub vals2_offset: u32_XE_,
    pub unk_17: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TRS_XE_ {
    pub translation: Vector4_XE_,
    pub rotation: Vector4_XE_,
    pub scale: Vector4_XE_,
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct HkConstraintBoneRef_XE_<'a> {
    pub name: string<'a>,
    pub start: u32_XE_,
    pub val: u32_XE_
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct HkConstraintRef_XE_<'a> {
    pub info: &'a HkConstraintInfo_XE_,
    pub bone_parents: ref_slice<'a, i16_XE_>,
    pub bone_names: slice<HkConstraintBoneRef_XE_<'a>>,
    pub name_offsets: ref_slice<'a, u32_XE_>,
    pub bone_transforms: ref_slice<'a, TRS_XE_>,
    pub bones: ref_slice<'a, u32_XE_>,
    pub bones_order: ref_slice<'a, Key2_XE_>,
    pub vals2: ref_slice<'a, f32_XE_>,
}

#[make_endian]
impl Default for HkConstraintRef_XE_<'_> {
    fn default() -> Self {
        Self {
            info: get_default_ref(),
            bone_parents: ref_slice::default(),
            bone_names: slice::default(),
            name_offsets: ref_slice::default(),
            bone_transforms: ref_slice::default(),
            bones: ref_slice::default(),
            bones_order: ref_slice::default(),
            vals2: ref_slice::default(),
        }
    }
}

#[make_endian]
impl<'a> HkConstraintRef_XE_<'a> {
    pub fn from_data(src: &'a[u8], offset: usize) -> Result<Self> {
        let info = HkConstraintInfo_XE_::from_data(&src[offset..]).context("info")?;
        if info.kind != 0 {
            warn!("Unknown & Unhandled HkConstraint type {}", info.kind);
        }

        let bone_parents = i16_XE_::slice_from_data(
            &src[info.bone_parents_offset.conv()..],
            info.bone_parents_num.conv(),
        )
        .context("bone_parents")?;
        assert!(bone_parents[0] == -1, "first bone should be root node with no parent");

        let name_offsets = u32_XE_::slice_from_data(
            &src[info.bone_names_offset.conv()..],
            info.bone_names_num.conv(),
        )
        .context("name_offsets")?;
        let mut bone_names = Vec::with_capacity(name_offsets.len());
        for offset_ in name_offsets.iter() {
            let start = u32_XE_::from_data(&src[offset_.conv()..]).context("start")?;
            let val_ = u32_XE_::from_data(&src[offset_.to_native() as usize + 4..]).context("val_")?;
            let mut offset = start.conv();
            while src[offset] != 0 {
                offset += 1;
            }
            bone_names.push(HkConstraintBoneRef_XE_ {
                name: str::from_utf8(&src[start.conv()..offset]).context("string")?.into(),
                start: *start,
                val: *val_,
            });
        }
        let bone_transforms = TRS_XE_::slice_from_data(
            &src[info.bone_transforms_offset.conv()..],
            info.bone_transforms_num.conv(),
        )
        .context("bone_tranforms")?;
        let bones = u32_XE_::slice_from_data(
            &src[info.bones_offset.conv()..],
            info.bones_num.conv(),
        )
        .context("bones")?;
        let bones_order = Key2_XE_::slice_from_data(
            &src[info.bone_order_offset.conv()..],
            info.bone_order_num.conv(),
        )
        .context("bone_order")?;

        // TODO should probably figure out what this data is
        let vals2 = f32_XE_::slice_from_data(
            &src[info.vals2_offset.conv()..],
            info.vals2_num.to_native() as usize * 42, 
        )
        .context("vals2")?;

        Ok(Self {
            info: info,
            bone_parents: bone_parents.into(),
            bone_names: bone_names.into_boxed_slice().into(),
            name_offsets: name_offsets.into(),
            bone_transforms: bone_transforms.into(),
            bones: bones.into(),
            bones_order: bones_order.into(),
            vals2: vals2.into(),
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct HkConstraint {
    pub info: HkConstraintInfo,
    pub bone_parents: Vec<i16>,
    pub bone_names: Vec<String>,
    pub bone_transforms: Vec<TRS>,
    pub vals2: Vec<f32>,           // probably f32
}

#[make_endian]
impl From<&HkConstraintRef_XE_<'_>> for HkConstraint {
    fn from(val: &HkConstraintRef_XE_) -> Self {
        Self {
            info: val.info.conv(),
            bone_parents: val.bone_parents.iter().map(|x| x.conv()).collect(),
            bone_names: val
                .bone_names
                .iter()
                .map(|x| x.name.to_string())
                .collect(),
            bone_transforms: val.bone_transforms.iter().map(|x| x.conv()).collect(),
            vals2: val.vals2.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_endian]
#[enum_dispatch(DumpHkConstraint_XE_)]
pub enum HkConstraint_XE_<'a> {
    Ref(HkConstraintRef_XE_<'a>),
    Owned(HkConstraint)
}

#[make_endian]
pub trait DumpHkConstraintImpl_XE_ {
    fn bone_names_num(&self) -> usize;
    fn bone_transforms_num(&self) -> usize;
    fn bone_order_num(&self) -> usize;
    fn bone_parents_num(&self) -> usize;
    fn vals2_num(&self) -> usize;
    fn bone_names(&self) -> impl Iterator<Item=&str>;
    fn write_info(&self, info: &mut HkConstraintInfo_XE_) -> Result<()>;
    fn write_bone_transforms(&self, bone_transforms: &mut [TRS_XE_]) -> Result<()>;
    fn write_bone_order(&self, bone_order: &mut [Key2_XE_]) -> Result<()>;
    fn write_bone_parents(&self, bone_parents: &mut [i16_XE_]) -> Result<()>;
    fn write_bone_name_vals<'a>(&self, vals: impl Iterator<Item = &'a mut u32_XE_>);
    fn write_vals2(&self, vals2: &mut [f32_XE_]) -> Result<()>;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpHkConstraint_XE_ {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_, bones_num: u16, bones_offset: u32) -> Result<()>;
    fn add_size(&self, offset: usize, infos: &mut InfoCounts) -> usize;
}
#[make_endian]
impl<T: DumpHkConstraintImpl_XE_> DumpHkConstraint_XE_ for T {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_, bones_num: u16, bones_offset: u32) -> Result<()> {
        let info_off = infos.hk_constraints.offset;
        let info = infos.hk_constraints.next().context("hk_constraints")?;
        self.write_info(info).context("write info")?;
        info.bones_num = bones_num.conv();
        info.bones_offset = bones_offset.conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfo_XE_, bones_offset)).conv();

        info.bone_names_offset = dst.offset.conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfo_XE_, bone_names_offset)).conv();
        let mut name_off = dst.offset;
        let name_offsets = u32_XE_::mut_slice_from_data(dst, self.bone_names_num()).context("name_offsets")?;
        let mut string_off = dst.offset;
        let string_offs = u32_XE_::mut_slice_from_data(dst, name_offsets.len()*2).context("string_offs")?;
        info.bone_names_num = name_offsets.len().conv();

        dst.align(16)?;
        info.bone_transforms_offset = dst.offset.conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfo_XE_, bone_transforms_offset)).conv();
        let bone_transforms = TRS_XE_::mut_slice_from_data(dst, self.bone_transforms_num()).context("bone_transforms")?;
        info.bone_transforms_num = bone_transforms.len().conv();
        self.write_bone_transforms(bone_transforms).context("write bone_transforms")?;

        info.bone_order_offset = dst.offset.conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfo_XE_, bone_order_offset)).conv();
        let bone_order = Key2_XE_::mut_slice_from_data(dst, self.bone_order_num()).context("bone_order")?;
        info.bone_order_num = bone_order.len().conv();
        self.write_bone_order(bone_order).context("write bone_order")?;

        info.bone_parents_offset = dst.offset.conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfo_XE_, bone_parents_offset)).conv();
        let bone_parents = i16_XE_::mut_slice_from_data(dst, self.bone_parents_num()).context("bone_parents")?;
        info.bone_parents_num = bone_parents.len().conv();
        self.write_bone_parents(bone_parents).context("write bone_parents")?;
        dst.align(4)?;

        self.write_bone_name_vals(string_offs.iter_mut().skip(1).step_by(2));

        for (i, string) in self.bone_names().enumerate() {
            let s = string.as_bytes(); 
            name_offsets[i] = string_off.conv();
            string_offs[i*2] = dst.offset.conv();
            s.dump_into(dst).with_context(|| format!("string: {}", string))?;
            dst.split(1)?;
            dst.align(4)?;
            *infos.offsets.next().context("offsets")? = name_off.conv();
            *infos.offsets.next().context("offsets")? = string_off.conv();
            name_off += size_of::<u32_XE_>();
            string_off += size_of::<u32_XE_>() * 2;
        }

        info.vals2_offset = dst.offset.conv();
        let vals2 = f32_XE_::mut_slice_from_data(dst, self.vals2_num()).context("vals2")?;
        self.write_vals2(vals2).context("write vals2")?;
        if vals2.len() == 0 {
            info.vals2_offset = 0u32.conv();
        } else {
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfo_XE_, vals2_offset)).conv();
        }
        Ok(())
    }
    fn add_size(&self, mut offset: usize, infos: &mut InfoCounts) -> usize {
        infos.hk_constraints += 1;
        infos.offsets += self.bone_names_num() * 2 + 5;
        if self.vals2_num() != 0 {
            infos.offsets += 1;
        }
        offset += size_of::<u32_XE_>() * self.bone_names_num() * 3;
        offset = align_offset(offset, 16) + size_of::<TRS_XE_>() * self.bone_transforms_num();
        offset += size_of::<Key2_XE_>() * self.bone_order_num();
        offset += size_of::<i16_XE_>() * self.bone_parents_num();
        offset = align_offset(offset, 4);

        for string in self.bone_names() {
            let s = string.as_bytes(); 
            offset += s.len() + 1;
            offset = align_offset(offset, 4);
        }

        offset += size_of::<f32_XE_>() * self.vals2_num();
        offset
    }
}

#[make_endian]
impl DumpHkConstraintImpl_XE_ for HkConstraintRef_XE_<'_> {
    fn bone_names_num(&self) -> usize {
        self.bone_names.len()
    }
    fn bone_transforms_num(&self) -> usize {
        self.bone_transforms.len()
    }
    fn bone_order_num(&self) -> usize {
        self.bones_order.len()
    }
    fn bone_parents_num(&self) -> usize {
        self.bone_parents.len()
    }
    fn vals2_num(&self) -> usize {
        self.vals2.len()
    }
    fn bone_names(&self) -> impl Iterator<Item=&str> {
        self.bone_names.iter().map(|x| &*x.name)
    }
    fn write_info(&self, info: &mut HkConstraintInfo_XE_) -> Result<()> {
        info.write_from(self.info)
    }
    fn write_bone_transforms(&self, bone_transforms: &mut [TRS_XE_]) -> Result<()> {
        bone_transforms.write_from(&self.bone_transforms[..])
    }
    fn write_bone_order(&self, bone_order: &mut [Key2_XE_]) -> Result<()> {
        bone_order.write_from(&self.bones_order[..])
    }
    fn write_bone_parents(&self, bone_parents: &mut [i16_XE_]) -> Result<()> {
        bone_parents.write_from(&self.bone_parents[..])
    }
    fn write_bone_name_vals<'a>(&self, vals: impl Iterator<Item = &'a mut u32_XE_>) {
        for (src, dst) in self.bone_names.iter().map(|x| x.val).zip(vals) {
            *dst = src;
        }
    }
    fn write_vals2(&self, vals2: &mut [f32_XE_]) -> Result<()> {
        vals2.write_from(&self.vals2[..])
    }
}

#[make_endian]
impl DumpHkConstraintImpl_XE_ for HkConstraint {
    fn bone_names_num(&self) -> usize {
        self.bone_names.len()
    }
    fn bone_transforms_num(&self) -> usize {
        self.bone_transforms.len()
    }
    fn bone_order_num(&self) -> usize {
        self.bone_names.len()
    }
    fn bone_parents_num(&self) -> usize {
        self.bone_parents.len()
    }
    fn vals2_num(&self) -> usize {
        self.vals2.len()
    }
    fn bone_names(&self) -> impl Iterator<Item=&str> {
        self.bone_names.iter().map(|val| val.as_str())
    }
    fn write_info(&self, info: &mut HkConstraintInfo_XE_) -> Result<()> {
        *info = (&self.info).conv();
        Ok(())
    }
    fn write_bone_transforms(&self, bone_transforms: &mut [TRS_XE_]) -> Result<()> {
        for (src, dst) in self.bone_transforms.iter().zip(bone_transforms) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_bone_order(&self, bone_order: &mut [Key2_XE_]) -> Result<()> {
        let mut new_bone_order = self.bone_names.iter().enumerate().map(|(i,val)| Key2 { key: hash_string(val.as_ref(), None).into(), val: i as u32 }).collect::<Vec<_>>();
        new_bone_order.sort_by_key(|x| x.key.get());
        for (src, dst) in new_bone_order.iter().zip(bone_order) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_bone_parents(&self, bone_parents: &mut [i16_XE_]) -> Result<()> {
        for (src, dst) in self.bone_parents.iter().zip(bone_parents) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_bone_name_vals<'a>(&self, vals: impl Iterator<Item = &'a mut u32_XE_>) {
        for val in vals {
            *val = 0u32.conv();
        }
    }
    fn write_vals2(&self, vals2: &mut [f32_XE_]) -> Result<()> {
        for (src, dst) in self.vals2.iter().zip(vals2) {
            *dst = src.conv();
        }
        Ok(())
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct HkConstraintData_XE_ {
    pub kind: u32_XE_,
    pub unk_1: u32_XE_,
    pub unk_2: u32_XE_,
    pub unk_3: u32_XE_,
    pub unk_4: u32_XE_,
    pub unk_5: u32_XE_,
    pub unk_6: u32_XE_,
    pub unk_7: u32_XE_,
    pub unk_8: u32_XE_,
    pub unk_9: u32_XE_,
    pub unk_10: u32_XE_,
    pub unk_11: u32_XE_,
    pub unk_12: u32_XE_,
    pub unk_13: u32_XE_,
    pub unk_14: u32_XE_,
    pub unk_15: u32_XE_,
    pub unk_16: u32_XE_,
    pub unk_17: u32_XE_,
    pub unk_18: u32_XE_,
    pub unk_19: u32_XE_,
    pub unk_20: u32_XE_,
    pub unk_21: u32_XE_,
    pub unk_22: u32_XE_,
    pub unk_23: u32_XE_,
    pub unk_24: u32_XE_,
    pub unk_25: u32_XE_,
    pub unk_26: u32_XE_,
    pub unk_27: u32_XE_,
    pub unk_28: u32_XE_,
}
