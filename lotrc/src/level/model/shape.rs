use anyhow::{Context, Result};
use log::warn;
use enum_dispatch::enum_dispatch;

use crate::{
    types::{Crc, ReadData, Vector3, Vector4, DumpSlice, align_offset, hash_string, get_default_ref, ref_slice, string, slice, BaseTypes, NE},
    level::{
        pak::block1::infos::{InfoCounts, DumpInfos},
        model::Key2
    }
};
use lotrc_proc::{derive_pod};

#[derive_pod]
pub struct ShapeInfo<T: BaseTypes> {
    pub offset: T::u32, // sometimes a pointer to something, otherwise the number of strings from the obj1 pointing to this
    pub kind: T::u32,   // 0, 1, 2, 3, 4, 5
    pub unk_2: T::u32,
    pub unk_3: T::f32,
    pub unk_4: T::f32,
    pub unk_5: T::f32,
    pub translation: Vector3<T>,
    pub rotation: Vector4<T>,
    pub unk_13: T::f32,
    pub unk_14: T::f32,
    pub unk_15: T::f32,
    pub unk_16: T::f32,
    pub unk_17: T::f32,
    pub unk_18: T::f32,
    pub unk_19: T::f32,
    pub unk_20: T::f32,
    pub unk_21: T::f32,
    pub unk_22: T::f32,
    pub unk_23: T::f32,
    pub unk_24: T::f32,
    pub unk_25: T::f32,
    pub unk_26: T::f32,
    pub hk_shape_num: T::u32,
    pub hk_shape_offset: T::u32, // pointer to objd
    pub unk_29a: u8,
    pub unk_29b: u8,
    pub unk_29c: u8,
    pub unk_29d: u8,
    pub unk_30: T::f32,
}

#[derive_pod]
pub struct ShapeExtraInfo<T: BaseTypes> {
    pub size: T::u32,
    pub scale: T::f32,
    pub a: T::f32,
    pub b: T::f32,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct ShapeExtraRef<'a, T: BaseTypes> {
    info: &'a ShapeExtraInfo<T>,
    offs: ref_slice<'a, T::u32>,
    data: ref_slice<'a, u8>,
}

impl<'a, T: BaseTypes> ShapeExtraRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a ShapeInfo<T>) -> Result<Self> {
        let mut offset = info.offset.into() as usize;
        let info = ShapeExtraInfo::<T>::from_data(&src[offset..]).context("info")?;
        offset += std::mem::size_of_val(info);
        let offs = T::u32::slice_from_data(&src[offset..], info.size.into() as usize)
            .context("vals")?;
        offset += std::mem::size_of_val(offs);
        let mut off = offset + offs.last().unwrap().clone().into() as usize;
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

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ShapeRef<'a, T: BaseTypes> {
    info: &'a ShapeInfo<T>,
    extra: Option<ShapeExtraRef<'a, T>>,
    hk_shapes: slice<HkShapeRef<'a, T>>
}

impl<'a, T: BaseTypes> ShapeRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a ShapeInfo<T>) -> Result<Self> {
        let extra = if info.kind.into() == 0 {
            Some(ShapeExtraRef::from_data(src, info).context("extra")?)
        } else {
            None
        };
        let infos = HkShapeInfo::slice_from_data(&src[info.hk_shape_offset.into() as usize..], info.hk_shape_num.into() as usize).context("hk_shape infos")?;
        let hk_shapes = infos.into_iter().enumerate().map(|(i, info)| HkShapeRef::from_data(src, info).with_context(|| format!("hk_shape {}", i))).collect::<Result<Vec<_>>>()?.into_boxed_slice();
        Ok(Self {
            info,
            extra: extra.into(),
            hk_shapes: hk_shapes.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShapeExtra {
    info: ShapeExtraInfo<NE>,
    offs: Vec<u32>,
    data: Vec<u8>,
}

impl<T: BaseTypes> From<&ShapeExtraRef<'_, T>> for ShapeExtra
where
    ShapeExtraInfo<NE>: From<ShapeExtraInfo<T>>,
{
    fn from(val: &ShapeExtraRef<T>) -> Self {
        ShapeExtra {
            info: (*val.info).into(),
            offs: val.offs.iter().map(|&x| x.into()).collect(),
            data: val.data.to_vec(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub info: ShapeInfo<NE>,
    pub extra: Option<ShapeExtra>,
    pub hk_shapes: Vec<HkShape>,
}

impl<T: BaseTypes> From<&ShapeRef<'_, T>> for Shape
where
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
{
    fn from(val: &ShapeRef<T>) -> Self {
        Shape {
            info: (*val.info).into(),
            extra: val.extra.as_ref().map(|x| x.into()),
            hk_shapes: val
                .hk_shapes
                .iter()
                .map(|x| x.into())
                .collect(),
        }
    }
}

pub struct ShapeExtraDump<'a, T: BaseTypes> {
    info: &'a mut ShapeExtraInfo<T>,
    offs: &'a mut [T::u32],
    data: &'a mut [u8],
}

impl<'a, T: BaseTypes> ShapeExtraDump<'a, T> {
    fn from_bytes(dst: &mut DumpSlice<'a>, sizes: (usize, usize)) -> Result<Self> {
        let info = ShapeExtraInfo::mut_from_data(dst).context("info")?;
        let offs = T::u32::mut_slice_from_data(dst, sizes.0).context("offs")?;
        let data = u8::mut_slice_from_data(dst, sizes.1).context("data")?;
        Ok(Self { info, offs, data })
    }
}

pub trait DumpShapeExtra<T: BaseTypes> {
    fn get_sizes(&self) -> (usize, usize);
    fn write_vals(&self, vals: ShapeExtraDump<T>) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let vals = ShapeExtraDump::from_bytes(dst, self.get_sizes()).context("vals")?;
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
    fn add_size(&self, mut offset: usize) -> usize {
        let sizes = self.get_sizes();
        offset += std::mem::size_of::<ShapeExtraInfo<T>>() + sizes.0 * std::mem::size_of::<T::u32>() + sizes.1;
        offset
    }
}

/*
#[enum_dispatch(DumpShape_XE_)]
pub enum Shape_XE_<'a> {
    Ref(ShapeRef_XE_<'a>),
    Owned(Shape)
}
*/

impl<T: BaseTypes> DumpShapeExtra<T> for ShapeExtraRef<'_, T> {
    fn get_sizes(&self) -> (usize, usize) {
        (self.offs.len(), self.data.len())
    }
    fn write_vals(&self, vals: ShapeExtraDump<T>) -> Result<()> {
        *vals.info = *self.info;
        vals.offs.copy_from_slice(self.offs);
        vals.data.copy_from_slice(self.data);
        Ok(())
    }
}

impl<T: BaseTypes> DumpShapeExtra<T> for ShapeExtra
where
    ShapeExtraInfo<T>: From<ShapeExtraInfo<NE>>,
{
    fn get_sizes(&self) -> (usize, usize) {
        (self.offs.len(), self.data.len())
    }
    fn write_vals(&self, vals: ShapeExtraDump<T>) -> Result<()> {
        *vals.info = self.info.into();
        for (&src, dst) in self.offs.iter().zip(vals.offs) {
            *dst = src.into();
        }
        vals.data.copy_from_slice(&self.data);
        Ok(())
    }
} 

pub trait DumpShapeImpl<T: BaseTypes> {
    fn extra(&self) -> Option<&impl DumpShapeExtra<T>>;
    fn has_hk_shapes(&self) -> bool;
    fn hk_shapes(&self) -> impl Iterator<Item=&impl DumpHkShape<T>>;
    fn write_info(&self, info: &mut ShapeInfo<T>) -> Result<()>;
}

#[enum_dispatch]
pub trait DumpShape<T: BaseTypes> {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>, extra_off: Option<usize>) -> Result<()>;
    fn add_size(&self, offset: usize, infos: &mut InfoCounts) -> usize;
    fn add_extra_size(&self, offset: usize) -> usize;
    fn dump_extra_into(&self, dst: &mut DumpSlice) -> Result<Option<usize>>;
}
impl<T: BaseTypes, I: DumpShapeImpl<T>> DumpShape<T> for I {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>, extra_off: Option<usize>) -> Result<()> {
        let info_offset = infos.shapes.offset;
        let info = infos.shapes.next().context("shapes")?;
        self.write_info(info).context("info")?;
        if let Some(off) = extra_off {
            info.offset = (off as u32).into();
            *infos.offsets.next().context("offsets")? = ((info_offset + std::mem::offset_of!(ShapeInfo<T>, offset)) as u32).into();
        }
        if self.has_hk_shapes() {
            info.hk_shape_offset = (infos.hk_shapes.offset as u32).into();
            *infos.offsets.next().context("offsets")? = ((info_offset + std::mem::offset_of!(ShapeInfo<T>, hk_shape_offset)) as u32).into();
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

impl<T: BaseTypes> DumpShapeImpl<T> for ShapeRef<'_, T> {
    fn extra(&self) -> Option<&impl DumpShapeExtra<T>> {
        self.extra.as_ref()
    }
    fn has_hk_shapes(&self) -> bool {
        !self.hk_shapes.is_empty()    
    }
    fn hk_shapes(&self) -> impl Iterator<Item=&impl DumpHkShape<T>> {
        self.hk_shapes.iter()
    }
    fn write_info(&self, info: &mut ShapeInfo<T>) -> Result<()> {
        *info = *self.info;
        Ok(())
    }
}

impl<T: BaseTypes> DumpShapeImpl<T> for Shape
where
    ShapeExtraInfo<T>: From<ShapeExtraInfo<NE>>,
    ShapeInfo<T>: From<ShapeInfo<NE>>,
    BoxShape<T>: From<BoxShape<NE>>,
    SphereShape<T>: From<SphereShape<NE>>,
    CapsuleShape<T>: From<CapsuleShape<NE>>,
    CylinderShape<T>: From<CylinderShape<NE>>,
    ConvexVerticesInfo<T>: From<ConvexVerticesInfo<NE>>,
    BVTreeMeshInfo<T>: From<BVTreeMeshInfo<NE>>,
    HkShapeInfo<T>: From<HkShapeInfo<NE>>,
    Vector3<T>: From<Vector3<NE>>,
    Vector4<T>: From<Vector4<NE>>,
{
    fn extra(&self) -> Option<&impl DumpShapeExtra<T>> {
        self.extra.as_ref()
    }
    fn has_hk_shapes(&self) -> bool {
        !self.hk_shapes.is_empty()    
    }
    fn hk_shapes(&self) -> impl Iterator<Item=&impl DumpHkShape<T>> {
        self.hk_shapes.iter()
    }
    fn write_info(&self, info: &mut ShapeInfo<T>) -> Result<()> {
        *info = self.info.into();
        Ok(())
    }
}

#[derive_pod]
pub struct HkShapeInfo<T: BaseTypes> {
    pub unk_0: Vector4<T>,
    pub unk_4: Vector4<T>,
    pub kind: T::u32,
    pub unk_9: T::u32,
    pub unk_10: T::u32,
    pub unk_11: T::u32,
    pub unk_12: T::u32,
    pub unk_13: T::u32,
    pub unk_14: T::u32,
    pub unk_15: T::u32,
    pub unk_16: T::u32,
    pub unk_17: T::u32,
    pub unk_18: T::u32,
    pub unk_19: T::u32,
}

#[derive_pod]
pub struct BoxShape<T: BaseTypes> {
    pub translation: Vector4<T>,
    pub rotation: Vector4<T>,
    pub kind: T::u32,
    pub key: Crc<T>,
    pub half_extents: Vector3<T>,
    pub unk_13: T::u32,
    pub unk_14: T::u32,
    pub unk_15: T::f32,
    pub unk_16: T::f32,
    pub unk_17: T::f32,
    pub unk_18: T::f32,
    pub unk_19: T::f32,
}

#[derive_pod]
pub struct SphereShape<T: BaseTypes> {
    pub translation: Vector4<T>,
    pub rotation: Vector4<T>,
    pub kind: T::u32,
    pub key: Crc<T>,
    pub radius: T::f32,
    pub unk_11: T::u32,
    pub unk_12: T::f32,
    pub unk_13: T::f32,
    pub unk_14: T::f32,
    pub unk_15: T::f32,
    pub unk_16: T::f32,
    pub unk_17: T::f32,
    pub unk_18: T::f32,
    pub unk_19: T::f32,
}

#[derive_pod]
pub struct CapsuleShape<T: BaseTypes> {
    pub translation: Vector4<T>,
    pub rotation: Vector4<T>,
    pub kind: T::u32,
    pub key: Crc<T>,
    pub point1: Vector3<T>,
    pub point2: Vector3<T>,
    pub radius: T::f32,
    pub unk_17: T::f32,
    pub unk_18: T::f32,
    pub unk_19: T::f32,
}

#[derive_pod]
pub struct CylinderShape<T: BaseTypes> {
    pub translation: Vector4<T>,
    pub rotation: Vector4<T>,
    pub kind: T::u32,
    pub key: Crc<T>,
    pub point1: Vector3<T>,
    pub point2: Vector3<T>,
    pub radius: T::f32,
    pub unk_17: T::f32,
    pub unk_18: T::f32,
    pub unk_19: T::f32,
}

#[derive_pod]
pub struct ConvexVerticesInfo<T: BaseTypes> {
    pub translation: Vector4<T>,
    pub rotation: Vector4<T>,
    pub kind: T::u32,
    pub key: Crc<T>,
    pub norm_num: T::u32,
    pub norms_offset: T::u32,
    pub vert_num: T::u32,
    pub verts_offset: T::u32,
    pub unk_14: T::f32,
    pub unk_15: T::f32,
    pub unk_16: T::f32,
    pub unk_17: T::f32,
    pub unk_18: T::f32,
    pub unk_19: T::f32,
}

#[derive_pod]
pub struct BVTreeMeshInfo<T: BaseTypes> {
    pub translation: Vector4<T>,
    pub rotation: Vector4<T>,
    pub kind: T::u32,
    pub key: Crc<T>,
    /// triangles min bound - 0.05
    pub offset: Vector3<T>,
    /// 254*256*256 / (max triangle bound + 0.1)
    pub tree_scale: T::f32,
    pub tree_size: T::u32,
    pub tree_offset: T::u32,  // u8
    pub vert_num: T::u32,     // vert_num
    pub verts_offset: T::u32, // vec3 f32 verts offset
    pub tri_num: T::u32,      // tri_num
    pub inds_offset: T::u32,  // vec3 u16 , inds offset
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ConvexVerticesRef<'a, T: BaseTypes> {
    pub info: &'a ConvexVerticesInfo<T>,
    pub norms: ref_slice<'a, Vector4<T>>,
    pub verts: ref_slice<'a, Vector3<T>>,
}

impl<'a, T: BaseTypes> ConvexVerticesRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a HkShapeInfo<T>) -> Result<Self> {
        let info: &ConvexVerticesInfo<T> = bytemuck::try_cast_ref(info)?;

        let norms = Vector4::slice_from_data(
            &src[info.norms_offset.into() as usize..],
            info.norm_num.into() as usize,
        )
        .context("vals")?;
        let mut vert_num = info.vert_num.into() as usize; // sketchy stuff to account for data that was not otherwise captured, is it needed?
        while (info.verts_offset.into() as usize + vert_num * 12) % 16 != 0 {
            vert_num += 1;
        }
        let verts = Vector3::slice_from_data(
            &src[info.verts_offset.into() as usize..],
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

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct BVTreeMeshRef<'a, T: BaseTypes> {
    info: &'a BVTreeMeshInfo<T>,
    tree: ref_slice<'a, u8>,
    verts: ref_slice<'a, Vector3<T>>,
    inds: ref_slice<'a, T::u16>
}

impl<'a, T: BaseTypes> BVTreeMeshRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a HkShapeInfo<T>) -> Result<Self> {
        let info: &BVTreeMeshInfo<T> = bytemuck::try_cast_ref(info)?;

        let tree = u8::slice_from_data(
            &src[info.tree_offset.into() as usize..],
            info.tree_size.into() as usize,
        )
        .context("vals")?;
        let verts = Vector3::slice_from_data(
            &src[info.verts_offset.into() as usize..],
            info.vert_num.into() as usize,
        )
        .context("vals")?;
        let inds = T::u16::slice_from_data(
            &src[info.inds_offset.into() as usize..],
            info.tri_num.into() as usize * 3,
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

#[cfg_attr(feature = "ffi", repr(C, u8))]
#[derive(PartialEq)]
pub enum HkShapeRef<'a, T: BaseTypes> {
    Box(&'a BoxShape<T>),
    Sphere(&'a SphereShape<T>),
    Capsule(&'a CapsuleShape<T>),
    Cylinder(&'a CylinderShape<T>),
    ConvexVertices(ConvexVerticesRef<'a, T>),
    BVTreeMesh(BVTreeMeshRef<'a, T>),
    Unknown(&'a HkShapeInfo<T>),
}

impl<'a, T: BaseTypes> HkShapeRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a HkShapeInfo<T>) -> Result<Self> {
        Ok(match info.kind.into() {
            1 => Self::Box(bytemuck::try_cast_ref(info)?),
            2 => Self::Sphere(bytemuck::try_cast_ref(info)?),
            3 => Self::Capsule(bytemuck::try_cast_ref(info)?),
            4 => Self::Cylinder(bytemuck::try_cast_ref(info)?),
            5 => Self::ConvexVertices(ConvexVerticesRef::from_data(src, info)?),
            6 => Self::BVTreeMesh(BVTreeMeshRef::from_data(src, info)?),
            x => {
                warn!("Unknown & Unhandled HkShape type {}", x);
                Self::Unknown(info)
            }
        })
    }
}

#[derive(Debug, Clone)]
pub enum HkShape {
    Box(BoxShape<NE>),
    Sphere(SphereShape<NE>),
    Capsule(CapsuleShape<NE>),
    Cylinder(CylinderShape<NE>),
    ConvexVertices {
        info: ConvexVerticesInfo<NE>,
        norms: Vec<Vector4<NE>>,
        verts: Vec<Vector3<NE>>,
    },
    BVTreeMesh {
        info: BVTreeMeshInfo<NE>,
        tree: Vec<u8>,
        verts: Vec<Vector3<NE>>,
        inds: Vec<u16>,
    },
    Unknown(HkShapeInfo<NE>),
}

impl<T: BaseTypes> From<&HkShapeRef<'_, T>> for HkShape
where
    BoxShape<NE>: From<BoxShape<T>>,
    SphereShape<NE>: From<SphereShape<T>>,
    CapsuleShape<NE>: From<CapsuleShape<T>>,
    CylinderShape<NE>: From<CylinderShape<T>>,
    ConvexVerticesInfo<NE>: From<ConvexVerticesInfo<T>>,
    BVTreeMeshInfo<NE>: From<BVTreeMeshInfo<T>>,
    HkShapeInfo<NE>: From<HkShapeInfo<T>>,
    Vector3<NE>: From<Vector3<T>>,
    Vector4<NE>: From<Vector4<T>>,
{ 
    fn from(val: &HkShapeRef<T>) -> Self {
        match val {
            HkShapeRef::Box(info) => HkShape::Box((**info).into()),
            HkShapeRef::Sphere(info) => HkShape::Sphere((**info).into()),
            HkShapeRef::Capsule(info) => HkShape::Capsule((**info).into()),
            HkShapeRef::Cylinder(info) => HkShape::Cylinder((**info).into()),
            HkShapeRef::ConvexVertices(ConvexVerticesRef { info, norms, verts }) => HkShape::ConvexVertices {
                info: (**info).into(),
                norms: norms.iter().map(|&x| x.into()).collect(),
                verts: verts.iter().map(|&x| x.into()).collect(),
            },
            HkShapeRef::BVTreeMesh(BVTreeMeshRef {
                info,
                tree,
                verts,
                inds,
            }) => HkShape::BVTreeMesh {
                info: (**info).into(),
                tree: tree.iter().cloned().collect(),
                verts: verts.iter().map(|&x| x.into()).collect(),
                inds: inds.iter().map(|&x| x.into()).collect(),
            },
            HkShapeRef::Unknown(info) => HkShape::Unknown((**info).into()),
        }
    }
}

/*
#[enum_dispatch(DumpHkShape_XE_)]
pub enum HkShape_XE_<'a> {
    Ref(HkShapeRef_XE_<'a>),
    Owned(HkShape)
}
*/

pub enum HkShapeDump<'a, T: BaseTypes> {
    ConvexVertices {
        norms: &'a mut [Vector4<T>],
        verts: &'a mut [Vector3<T>],
    },
    BVTreeMesh {
        tree: &'a mut [u8],
        verts: &'a mut [Vector3<T>],
        inds: &'a mut [T::u16],
    },
    None
}

impl<'a, T: BaseTypes> HkShapeDump<'a, T> {

    fn from_bytes(dst: &mut DumpSlice<'a>, sizes: (usize, usize, usize), info: &mut HkShapeInfo<T>) -> Result<Self> {
        Ok(match info.kind.into() {
            5u32 => {
                let info: &mut ConvexVerticesInfo<T> = bytemuck::cast_mut(info);

                dst.align(16)?;
                info.norms_offset = (dst.offset as u32).into();
                let norms = Vector4::mut_slice_from_data(dst, sizes.0).context("norms")?;
                info.norm_num = (norms.len() as u32).into();

                info.verts_offset = (dst.offset as u32).into();
                let verts = Vector3::mut_slice_from_data(dst, sizes.1).context("verts")?;
                info.vert_num = ((verts.len() - sizes.2) as u32).into();
                Self::ConvexVertices { norms, verts }
            }
            6 => {
                let info: &mut BVTreeMeshInfo<T> = bytemuck::cast_mut(info);

                info.verts_offset = (dst.offset as u32).into();
                let verts = Vector3::mut_slice_from_data(dst, sizes.0).context("verts")?;
                info.vert_num = (verts.len() as u32).into();

                info.inds_offset = (dst.offset as u32).into();
                let inds = T::u16::mut_slice_from_data(dst, sizes.1).context("inds")?;
                info.tri_num = ((inds.len() / 3) as u32).into();

                dst.align(4)?;
                info.tree_offset = (dst.offset as u32).into();
                let tree = u8::mut_slice_from_data(dst, sizes.2).context("tree")?;
                info.tree_size = (tree.len() as u32).into();
                dst.align(4)?;
                Self::BVTreeMesh { tree, verts, inds }
            }
            _ => Self::None
        })
    }
}

#[enum_dispatch]
pub trait DumpHkShape<T: BaseTypes> {
    fn kind(&self) -> u32;
    fn get_sizes(&self) -> (usize, usize, usize);
    fn write_info(&self, info: &mut HkShapeInfo<T>) -> Result<()>;
    fn write_vals(&self, vals: HkShapeDump<T>) -> Result<()>;
    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        counts.hk_shapes += 1;
        match self.kind() {
            5 => {
                counts.offsets += 2;
                let sizes = self.get_sizes();
                offset = align_offset(offset, 16);
                offset += sizes.0 * size_of::<Vector4<T>>();
                offset += sizes.1 * size_of::<Vector3<T>>();
            }
            6 => {
                counts.offsets += 3;
                let sizes = self.get_sizes();
                offset += sizes.0 * size_of::<Vector3<T>>();
                offset += sizes.1 * size_of::<T::u16>();
                offset = align_offset(offset, 4);
                offset += sizes.2;
                offset = align_offset(offset, 4);
            }
            _ => ()
        }
        offset
    }
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>) -> Result<()> {
        let info_off = infos.hk_shapes.offset;
        let info = infos.hk_shapes.next().context("hk_shapes")?;
        self.write_info(info).context("write info")?;
        let vals = HkShapeDump::from_bytes(dst, self.get_sizes(), info).context("vals")?;
        self.write_vals(vals).context("write vals")?;
        match self.kind() {
            5 => {
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ConvexVerticesInfo<T>, norms_offset)) as u32).into();
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ConvexVerticesInfo<T>, verts_offset)) as u32).into();
            }
            6 => {
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(BVTreeMeshInfo<T>, verts_offset)) as u32).into();
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(BVTreeMeshInfo<T>, inds_offset)) as u32).into();
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(BVTreeMeshInfo<T>, tree_offset)) as u32).into();

            }
            _ => ()
        }
        Ok(())
    }
}

impl<T: BaseTypes> DumpHkShape<T> for HkShapeRef<'_, T> { 
    fn kind(&self) -> u32 {
        match self {
            Self::Box(info) => info.kind.into(),
            Self::Sphere(info) => info.kind.into(),
            Self::Capsule(info) => info.kind.into(),
            Self::Cylinder(info) => info.kind.into(),
            Self::ConvexVertices(val) => val.info.kind.into(),
            Self::BVTreeMesh(val) => val.info.kind.into(),
            Self::Unknown(info) => info.kind.into(),
        }
    }
    fn get_sizes(&self) -> (usize, usize, usize) {
        match self {
            Self::ConvexVertices(ConvexVerticesRef { info, verts, norms }) => (norms.len(), verts.len(), verts.len() - info.vert_num.into() as usize),
            Self::BVTreeMesh(BVTreeMeshRef { tree, verts, inds, .. }) => (verts.len(), inds.len(), tree.len()),
            _ => (0,0,0)
        }
    }
    fn write_info(&self, info: &mut HkShapeInfo<T>) -> Result<()> {
        match self {
            Self::Box(src) => *info = *bytemuck::cast_ref(*src),
            Self::Sphere(src) => *info = *bytemuck::cast_ref(*src),
            Self::Capsule(src) => *info = *bytemuck::cast_ref(*src),
            Self::Cylinder(src) => *info = *bytemuck::cast_ref(*src),
            Self::ConvexVertices(val) => *info = *bytemuck::cast_ref(val.info),
            Self::BVTreeMesh(val) => *info = *bytemuck::cast_ref(val.info),
            Self::Unknown(src) => *info = **src,
        };
        Ok(())
    }
    fn write_vals(&self, vals: HkShapeDump<T>) -> Result<()> {
        match (self, vals) {
            (Self::ConvexVertices(ConvexVerticesRef { verts: src1, norms: src2, .. }), HkShapeDump::ConvexVertices { verts: dst1, norms: dst2 }) => {
                dst1.copy_from_slice(src1);
                dst2.copy_from_slice(src2);
            },
            (Self::BVTreeMesh(BVTreeMeshRef { verts: src1, inds: src2, tree: src3, .. }), HkShapeDump::BVTreeMesh { verts: dst1, inds: dst2, tree: dst3 }) => {
                dst1.copy_from_slice(src1);
                dst2.copy_from_slice(src2);
                dst3.copy_from_slice(src3);
            },
            _ => ()
        }
        Ok(())
    }
}

impl<T: BaseTypes> DumpHkShape<T> for HkShape
where
    BoxShape<T>: From<BoxShape<NE>>,
    SphereShape<T>: From<SphereShape<NE>>,
    CapsuleShape<T>: From<CapsuleShape<NE>>,
    CylinderShape<T>: From<CylinderShape<NE>>,
    ConvexVerticesInfo<T>: From<ConvexVerticesInfo<NE>>,
    BVTreeMeshInfo<T>: From<BVTreeMeshInfo<NE>>,
    HkShapeInfo<T>: From<HkShapeInfo<NE>>,
    Vector3<T>: From<Vector3<NE>>,
    Vector4<T>: From<Vector4<NE>>,
{ 
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
    fn write_info(&self, info: &mut HkShapeInfo<T>) -> Result<()> {
        match self {
            HkShape::Box(src) => *bytemuck::cast_mut(info) = BoxShape::<T>::from(*src),
            HkShape::Sphere(src) => *bytemuck::cast_mut(info) = SphereShape::<T>::from(*src),
            HkShape::Capsule(src) => *bytemuck::cast_mut(info) = CapsuleShape::<T>::from(*src),
            HkShape::Cylinder(src) => *bytemuck::cast_mut(info) = CylinderShape::<T>::from(*src),
            HkShape::ConvexVertices { info: src, .. } => *bytemuck::cast_mut(info) = ConvexVerticesInfo::<T>::from(*src),
            HkShape::BVTreeMesh { info: src, .. } => *bytemuck::cast_mut(info) = BVTreeMeshInfo::<T>::from(*src),
            HkShape::Unknown(src) => *info = (*src).into(),
        }
        Ok(())
    }
    fn write_vals(&self, vals: HkShapeDump<T>) -> Result<()> {
        match (self, vals) {
            (HkShape::ConvexVertices { verts: src1, norms: src2, .. }, HkShapeDump::ConvexVertices { verts: dst1, norms: dst2 }) => {
                for (&src, dst) in src1.iter().zip(dst1) {
                    *dst = src.into();
                }
                for (&src, dst) in src2.iter().zip(dst2) {
                    *dst = src.into();
                }
            },
            (HkShape::BVTreeMesh { verts: src1, inds: src2, tree: src3, .. }, HkShapeDump::BVTreeMesh { verts: dst1, inds: dst2, tree: dst3 }) => {
                for (&src, dst) in src1.iter().zip(dst1) {
                    *dst = src.into();
                }
                for (&src, dst) in src2.iter().zip(dst2) {
                    *dst = src.into();
                }
                dst3.copy_from_slice(src3);
            },
            _ => ()
        }
        Ok(())
    }
}

#[derive_pod]
pub struct HkConstraintInfo<T: BaseTypes> {
    pub kind: T::u32,
    pub bone_parents_offset: T::u32,
    pub bone_parents_num: T::u32,
    pub bone_names_offset: T::u32,
    pub bone_names_num: T::u32,
    pub bone_transforms_offset: T::u32,
    pub bone_transforms_num: T::u32,
    pub unk_7: T::u32,
    pub unk_8: T::u32,
    pub unk_9: T::u32,
    pub bones_offset: T::u32,
    pub bones_num: T::u16,
    pub bone_order_num: T::u16,
    pub bone_order_offset: T::u32,
    pub unk_13: T::u32,
    pub unk_14: T::f32,
    pub vals2_num: T::u32,
    pub vals2_offset: T::u32,
    pub unk_17: T::u32,
}

#[derive_pod]
pub struct TRS<T: BaseTypes> {
    pub translation: Vector4<T>,
    pub rotation: Vector4<T>,
    pub scale: Vector4<T>,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct HkConstraintBoneRef<'a, T: BaseTypes> {
    pub name: string<'a>,
    pub start: T::u32,
    pub val: T::u32
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct HkConstraintRef<'a, T: BaseTypes> {
    pub info: &'a HkConstraintInfo<T>,
    pub bone_parents: ref_slice<'a, T::i16>,
    pub bone_names: slice<HkConstraintBoneRef<'a, T>>,
    pub name_offsets: ref_slice<'a, T::u32>,
    pub bone_transforms: ref_slice<'a, TRS<T>>,
    pub bones: ref_slice<'a, T::u32>,
    pub bones_order: ref_slice<'a, Key2<T>>,
    pub vals2: ref_slice<'a, T::f32>,
}

impl<T: BaseTypes> Default for HkConstraintRef<'_, T> {
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

impl<'a, T: BaseTypes> HkConstraintRef<'a, T> {
    pub fn from_data(src: &'a[u8], offset: usize) -> Result<Self> {
        let info = HkConstraintInfo::<T>::from_data(&src[offset..]).context("info")?;
        if info.kind.into() != 0 {
            warn!("Unknown & Unhandled HkConstraint type {}", info.kind.into());
        }

        let bone_parents = T::i16::slice_from_data(
            &src[info.bone_parents_offset.into() as usize..],
            info.bone_parents_num.into() as usize,
        )
        .context("bone_parents")?;
        assert!(bone_parents[0].into() == -1, "first bone should be root node with no parent");

        let name_offsets = T::u32::slice_from_data(
            &src[info.bone_names_offset.into() as usize..],
            info.bone_names_num.into() as usize,
        )
        .context("name_offsets")?;
        let mut bone_names = Vec::with_capacity(name_offsets.len());
        for &offset_ in name_offsets.iter() {
            let &start = T::u32::from_data(&src[offset_.into() as usize..]).context("start")?;
            let val_ = T::u32::from_data(&src[offset_.into() as usize + 4..]).context("val_")?;
            let mut offset = start.into() as usize;
            while src[offset] != 0 {
                offset += 1;
            }
            bone_names.push(HkConstraintBoneRef {
                name: str::from_utf8(&src[start.into() as usize..offset]).context("string")?.into(),
                start: start,
                val: *val_,
            });
        }
        let bone_transforms = TRS::slice_from_data(
            &src[info.bone_transforms_offset.into() as usize..],
            info.bone_transforms_num.into() as usize,
        )
        .context("bone_tranforms")?;
        let bones = T::u32::slice_from_data(
            &src[info.bones_offset.into() as usize..],
            info.bones_num.into() as usize,
        )
        .context("bones")?;
        let bones_order = Key2::slice_from_data(
            &src[info.bone_order_offset.into() as usize..],
            info.bone_order_num.into() as usize,
        )
        .context("bone_order")?;

        // TODO should probably figure out what this data is
        let vals2 = T::f32::slice_from_data(
            &src[info.vals2_offset.into() as usize..],
            info.vals2_num.into() as usize * 42, 
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
    pub info: HkConstraintInfo<NE>,
    pub bone_parents: Vec<i16>,
    pub bone_names: Vec<String>,
    pub bone_transforms: Vec<TRS<NE>>,
    pub vals2: Vec<f32>,
}

impl<T: BaseTypes> From<&HkConstraintRef<'_, T>> for HkConstraint
where
    HkConstraintInfo<NE>: From<HkConstraintInfo<T>>,
    TRS<NE>: From<TRS<T>>,
{
    fn from(val: &HkConstraintRef<T>) -> Self {
        Self {
            info: (*val.info).into(),
            bone_parents: val.bone_parents.iter().map(|&x| x.into()).collect(),
            bone_names: val
                .bone_names
                .iter()
                .map(|x| x.name.to_string())
                .collect(),
            bone_transforms: val.bone_transforms.iter().map(|&x| x.into()).collect(),
            vals2: val.vals2.iter().map(|&x| x.into()).collect(),
        }
    }
}

/*
#[enum_dispatch(DumpHkConstraint_XE_)]
pub enum HkConstraint_XE_<'a> {
    Ref(HkConstraintRef_XE_<'a>),
    Owned(HkConstraint)
}
*/

pub trait DumpHkConstraintImpl<T: BaseTypes> {
    fn bone_names_num(&self) -> usize;
    fn bone_transforms_num(&self) -> usize;
    fn bone_order_num(&self) -> usize;
    fn bone_parents_num(&self) -> usize;
    fn vals2_num(&self) -> usize;
    fn bone_names(&self) -> impl Iterator<Item=&str>;
    fn write_info(&self, info: &mut HkConstraintInfo<T>) -> Result<()>;
    fn write_bone_transforms(&self, bone_transforms: &mut [TRS<T>]) -> Result<()>;
    fn write_bone_order(&self, bone_order: &mut [Key2<T>]) -> Result<()>;
    fn write_bone_parents(&self, bone_parents: &mut [T::i16]) -> Result<()>;
    fn write_bone_name_vals<'a>(&self, vals: impl Iterator<Item = &'a mut T::u32>);
    fn write_vals2(&self, vals2: &mut [T::f32]) -> Result<()>;
}

#[enum_dispatch]
pub trait DumpHkConstraint<T: BaseTypes> {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>, bones_num: u16, bones_offset: u32) -> Result<()>;
    fn add_size(&self, offset: usize, infos: &mut InfoCounts) -> usize;
}
impl<T: BaseTypes, I: DumpHkConstraintImpl<T>> DumpHkConstraint<T> for I {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>, bones_num: u16, bones_offset: u32) -> Result<()> {
        let info_off = infos.hk_constraints.offset;
        let info = infos.hk_constraints.next().context("hk_constraints")?;
        self.write_info(info).context("write info")?;
        info.bones_num = bones_num.into();
        info.bones_offset = bones_offset.into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(HkConstraintInfo<T>, bones_offset)) as u32).into();

        info.bone_names_offset = (dst.offset as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(HkConstraintInfo<T>, bone_names_offset)) as u32).into();
        let mut name_off = dst.offset;
        let name_offsets = T::u32::mut_slice_from_data(dst, self.bone_names_num()).context("name_offsets")?;
        let mut string_off = dst.offset;
        let string_offs = T::u32::mut_slice_from_data(dst, name_offsets.len()*2).context("string_offs")?;
        info.bone_names_num = (name_offsets.len() as u32).into();

        dst.align(16)?;
        info.bone_transforms_offset = (dst.offset as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(HkConstraintInfo<T>, bone_transforms_offset)) as u32).into();
        let bone_transforms = TRS::mut_slice_from_data(dst, self.bone_transforms_num()).context("bone_transforms")?;
        info.bone_transforms_num = (bone_transforms.len() as u32).into();
        self.write_bone_transforms(bone_transforms).context("write bone_transforms")?;

        info.bone_order_offset = (dst.offset as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(HkConstraintInfo<T>, bone_order_offset)) as u32).into();
        let bone_order = Key2::mut_slice_from_data(dst, self.bone_order_num()).context("bone_order")?;
        info.bone_order_num = (bone_order.len() as u16).into();
        self.write_bone_order(bone_order).context("write bone_order")?;

        info.bone_parents_offset = (dst.offset as u32).into();
        *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(HkConstraintInfo<T>, bone_parents_offset)) as u32).into();
        let bone_parents = T::i16::mut_slice_from_data(dst, self.bone_parents_num()).context("bone_parents")?;
        info.bone_parents_num = (bone_parents.len() as u32).into();
        self.write_bone_parents(bone_parents).context("write bone_parents")?;
        dst.align(4)?;

        self.write_bone_name_vals(string_offs.iter_mut().skip(1).step_by(2));

        for (i, string) in self.bone_names().enumerate() {
            let s = string.as_bytes(); 
            name_offsets[i] = (string_off as u32).into();
            string_offs[i*2] = (dst.offset as u32).into();
            u8::mut_slice_from_data(dst, s.len()).with_context(|| format!("string: {}", string))?.copy_from_slice(s);
            dst.split(1)?;
            dst.align(4)?;
            *infos.offsets.next().context("offsets")? = (name_off as u32).into();
            *infos.offsets.next().context("offsets")? = (string_off as u32).into();
            name_off += size_of::<T::u32>();
            string_off += size_of::<T::u32>() * 2;
        }

        info.vals2_offset = (dst.offset as u32).into();
        let vals2 = T::f32::mut_slice_from_data(dst, self.vals2_num()).context("vals2")?;
        self.write_vals2(vals2).context("write vals2")?;
        if vals2.len() == 0 {
            info.vals2_offset = 0u32.into();
        } else {
            *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(HkConstraintInfo<T>, vals2_offset)) as u32).into();
        }
        Ok(())
    }
    fn add_size(&self, mut offset: usize, infos: &mut InfoCounts) -> usize {
        infos.hk_constraints += 1;
        infos.offsets += self.bone_names_num() * 2 + 5;
        if self.vals2_num() != 0 {
            infos.offsets += 1;
        }
        offset += size_of::<T::u32>() * self.bone_names_num() * 3;
        offset = align_offset(offset, 16) + size_of::<TRS<T>>() * self.bone_transforms_num();
        offset += size_of::<Key2<T>>() * self.bone_order_num();
        offset += size_of::<T::i16>() * self.bone_parents_num();
        offset = align_offset(offset, 4);

        for string in self.bone_names() {
            let s = string.as_bytes(); 
            offset += s.len() + 1;
            offset = align_offset(offset, 4);
        }

        offset += size_of::<T::f32>() * self.vals2_num();
        offset
    }
}

impl<T: BaseTypes> DumpHkConstraintImpl<T> for HkConstraintRef<'_, T> {
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
    fn write_info(&self, info: &mut HkConstraintInfo<T>) -> Result<()> {
        *info = *self.info;
        Ok(())
    }
    fn write_bone_transforms(&self, bone_transforms: &mut [TRS<T>]) -> Result<()> {
        bone_transforms.copy_from_slice(self.bone_transforms);
        Ok(())
    }
    fn write_bone_order(&self, bone_order: &mut [Key2<T>]) -> Result<()> {
        bone_order.copy_from_slice(self.bones_order);
        Ok(())
    }
    fn write_bone_parents(&self, bone_parents: &mut [T::i16]) -> Result<()> {
        bone_parents.copy_from_slice(self.bone_parents);
        Ok(())
    }
    fn write_bone_name_vals<'a>(&self, vals: impl Iterator<Item = &'a mut T::u32>) {
        for (src, dst) in self.bone_names.iter().map(|x| x.val).zip(vals) {
            *dst = src;
        }
    }
    fn write_vals2(&self, vals2: &mut [T::f32]) -> Result<()> {
        vals2.copy_from_slice(self.vals2);
        Ok(())
    }
}

impl<T: BaseTypes> DumpHkConstraintImpl<T> for HkConstraint
where
    HkConstraintInfo<T>: From<HkConstraintInfo<NE>>,
    TRS<T>: From<TRS<NE>>,
{
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
    fn write_info(&self, info: &mut HkConstraintInfo<T>) -> Result<()> {
        *info = self.info.into();
        Ok(())
    }
    fn write_bone_transforms(&self, bone_transforms: &mut [TRS<T>]) -> Result<()> {
        for (&src, dst) in self.bone_transforms.iter().zip(bone_transforms) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_bone_order(&self, bone_order: &mut [Key2<T>]) -> Result<()> {
        let mut new_bone_order = self.bone_names.iter().enumerate().map(|(i,val)| Key2 { key: Crc::<T>::new(hash_string(val.as_ref(), None).into()), val: (i as u32).into() }).collect::<Vec<_>>();
        new_bone_order.sort_by_key(|x| x.key.val.into());
        for (&src, dst) in new_bone_order.iter().zip(bone_order) {
            *dst = src;
        }
        Ok(())
    }
    fn write_bone_parents(&self, bone_parents: &mut [T::i16]) -> Result<()> {
        for (&src, dst) in self.bone_parents.iter().zip(bone_parents) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_bone_name_vals<'a>(&self, vals: impl Iterator<Item = &'a mut T::u32>) {
        for val in vals {
            *val = 0u32.into();
        }
    }
    fn write_vals2(&self, vals2: &mut [T::f32]) -> Result<()> {
        for (&src, dst) in self.vals2.iter().zip(vals2) {
            *dst = src.into();
        }
        Ok(())
    }
}

#[derive_pod]
pub struct HkConstraintData<T: BaseTypes>  {
    pub kind: T::u32,
    pub unk_1: T::u32,
    pub unk_2: T::u32,
    pub unk_3: T::u32,
    pub unk_4: T::u32,
    pub unk_5: T::u32,
    pub unk_6: T::u32,
    pub unk_7: T::u32,
    pub unk_8: T::u32,
    pub unk_9: T::u32,
    pub unk_10: T::u32,
    pub unk_11: T::u32,
    pub unk_12: T::u32,
    pub unk_13: T::u32,
    pub unk_14: T::u32,
    pub unk_15: T::u32,
    pub unk_16: T::u32,
    pub unk_17: T::u32,
    pub unk_18: T::u32,
    pub unk_19: T::u32,
    pub unk_20: T::u32,
    pub unk_21: T::u32,
    pub unk_22: T::u32,
    pub unk_23: T::u32,
    pub unk_24: T::u32,
    pub unk_25: T::u32,
    pub unk_26: T::u32,
    pub unk_27: T::u32,
    pub unk_28: T::u32,
}
