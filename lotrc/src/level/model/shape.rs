#[cfg(feature = "python")]
use crate::pyobj_ref;
use anyhow::{Context, Result};
use log::warn;
#[cfg(not(feature = "python"))]
use lotrc_proc::getter;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::ptr::NonNull;
use std::sync::Arc;
use zerocopy::transmute_ref;

use crate::{
    types::{Crc, DumpData, RefFromData, Vector3, Vector4, DumpSlice, align_offset, hash_string},
    level::{
        pak::objs::InfoCounts,
        model::Key2
    }
};
#[make_platforms]
use crate::{
    types::{Vector3VER, Vector4VER, F32VER, I16VER, U16VER, U32VER},
    level::{
        model::Key2VER,
        pak::objs::DumpInfosVER
    }
};
use lotrc_proc::{make_platforms, OrderedData};

#[cfg(feature = "python")]
pub fn init(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "shape")?;
    m.add_class::<HkConstraintData>()?;
    m.add_class::<HkConstraintInfo>()?;
    m.add_class::<HkShape>()?;
    m.add_class::<HkShapeInfo>()?;
    m.add_class::<Shape>()?;
    m.add_class::<ShapeExtraInfo>()?;
    m.add_class::<ShapeInfo>()?;
    m.add_class::<TRS>()?;
    init_pc(&m)?;
    init_xbox(&m)?;
    init_ps3(&m)?;
    Ok(m)
}
#[cfg(feature = "python")]
#[make_platforms]
pub fn init_ver(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ShapeVER>()?;
    m.add_class::<HkConstraintVER>()
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
pub struct ShapeInfo {
    pub offset: u32, // sometimes a pointer to something, otherwise the number of strings from the obj1 pointing to this
    pub kind: u32,   // 0, 1, 2, 3, 4, 5
    pub unk_2: u32,
    pub unk_3: f32,
    pub unk_4: f32,
    pub unk_5: f32,
    pub translation: Vector3,
    pub rotation: Vector4,
    pub unk_13: f32,
    pub unk_14: f32,
    pub unk_15: f32,
    pub unk_16: f32,
    pub unk_17: f32,
    pub unk_18: f32,
    pub unk_19: f32,
    pub unk_20: f32,
    pub unk_21: f32,
    pub unk_22: f32,
    pub unk_23: f32,
    pub unk_24: f32,
    pub unk_25: f32,
    pub unk_26: f32,
    pub hk_shape_num: u32,
    pub hk_shape_offset: u32, // pointer to objd
    pub unk_29a: u8,
    pub unk_29b: u8,
    pub unk_29c: u8,
    pub unk_29d: u8,
    pub unk_30: f32,
}

#[derive(Default, Debug, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
#[repr(C)]
pub struct ShapeExtraInfo {
    pub size: u32,
    pub scale: f32,
    pub a: f32,
    pub b: f32,
}

#[make_platforms]
#[derive(Debug, Clone)]
struct ShapeExtraVER {
    info: NonNull<ShapeExtraInfoVER>,
    offs: NonNull<[U32VER]>,
    data: NonNull<[u8]>,
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct ShapeExtraRefVER<'a> {
    info: &'a ShapeExtraInfoVER,
    offs: &'a [U32VER],
    data: &'a [u8],
}

#[make_platforms]
unsafe impl Sync for ShapeExtraVER {}
#[make_platforms]
unsafe impl Send for ShapeExtraVER {}

#[make_platforms]
impl ShapeExtraVER {
    fn from_bytes(src: &Arc<[u8]>, info: &ShapeInfoVER) -> Result<Self> {
        let mut offset = info.offset.get() as usize;
        let info = ShapeExtraInfoVER::from_data(&src[offset..]).context("info")?;
        offset += info.size();
        let offs = U32VER::slice_from_data(&src[offset..], info.size.get() as usize)
            .context("vals")?;
        offset += offs.size();
        let mut off = offset + offs.last().unwrap().get() as usize;
        // sketchy stuff to account for missing data
        while (src[off] != 0) || (src[off + 1] != 0) {
            off += 1;
        }
        let data = &src[offset..off];
        Ok(ShapeExtraVER {
            info: info.into(),
            offs: offs.into(),
            data: data.into(),
        })
    }
    pub unsafe fn as_ref(&self) -> ShapeExtraRefVER<'_> {
        ShapeExtraRefVER {
            info: self.info.as_ref(),
            offs: self.offs.as_ref(),
            data: self.data.as_ref(),
        }
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "python", pyclass(module = "model.shape"))]
pub struct ShapeVER {
    _ptr: Arc<[u8]>,
    info: NonNull<ShapeInfoVER>,
    extra: Option<ShapeExtraVER>,
    hk_shapes: Box<[HkShapeVER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(ShapeVER);

#[make_platforms]
unsafe impl Sync for ShapeVER {}
#[make_platforms]
unsafe impl Send for ShapeVER {}

#[make_platforms]
impl ShapeVER {
    pub fn from_bytes(src: &Arc<[u8]>, offset: usize) -> Result<Self> {
        let info = ShapeInfoVER::from_data(&src[offset..]).context("info")?;
        let extra = if info.kind.get() == 0 {
            Some(ShapeExtraVER::from_bytes(src, info).context("extra")?)
        } else {
            None
        };
        let off = info.hk_shape_offset.get() as usize;
        let hk_shapes = (0..info.hk_shape_num.get() as usize)
            .map(|i| {
                HkShapeVER::from_bytes(src, off + i * HkShapeInfoVER::size_of())
                    .with_context(|| format!("hk_shape {}", i))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            _ptr: src.clone(),
            info: info.into(),
            extra,
            hk_shapes: hk_shapes.into(),
        })
    }
    pub fn get(&self, index: usize) -> Option<HkShapeRefVER<'_>> {
        self.hk_shapes.get(index).map(|x| unsafe { x.as_ref() })
    }
    pub fn len(&self) -> usize {
        self.hk_shapes.len()
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl ShapeVER {
    #[getter]
    pub fn info(&self) -> &ShapeInfoVER {
        unsafe { self.info.as_ref() }
    }
    #[getter]
    pub fn extra(&self) -> Option<ShapeExtraRefVER<'_>> {
        self.extra.as_ref().map(|x| unsafe { x.as_ref() })
    }
    #[cfg(feature = "python")]
    fn __getitem__(&self, index: usize) -> Option<HkShapeRefVER<'_>> {
        self.get(index)
    }
    #[cfg(feature = "python")]
    fn __len__(&self) -> usize {
        self.len()
    }
}

#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct ShapeExtra {
    info: ShapeExtraInfo,
    offs: Vec<u32>,
    data: Vec<u8>,
}

#[make_platforms]
impl From<ShapeExtraRefVER<'_>> for ShapeExtra {
    fn from(val: ShapeExtraRefVER) -> Self {
        ShapeExtra {
            info: val.info.into(),
            offs: val.offs.iter().map(|x| x.into()).collect(),
            data: val.data.to_vec(),
        }
    }
}

#[cfg(feature = "python")]
#[make_platforms]
impl<'a, 'py> IntoPyObject<'py> for ShapeExtraRefVER<'a> {
    type Target = <ShapeExtra as IntoPyObject<'py>>::Target;
    type Output = <ShapeExtra as IntoPyObject<'py>>::Output;
    type Error = <ShapeExtra as IntoPyObject<'py>>::Error;
    #[inline(always)]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        ShapeExtra::from(self).into_pyobject(py)
    }
}

#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct Shape {
    pub info: ShapeInfo,
    pub extra: Option<ShapeExtra>,
    pub hk_shapes: Vec<HkShape>,
}

#[make_platforms]
impl From<&ShapeVER> for Shape {
    fn from(val: &ShapeVER) -> Self {
        Shape {
            info: val.info().into(),
            extra: val.extra().map(|x| x.into()),
            hk_shapes: val
                .hk_shapes
                .iter()
                .map(|x| unsafe { x.as_ref() }.into())
                .collect(),
        }
    }
}

#[make_platforms]
pub struct ShapeExtraDumpVER<'a> {
    info: &'a mut ShapeExtraInfoVER,
    offs: &'a mut [U32VER],
    data: &'a mut [u8],
}

#[make_platforms]
impl<'a> ShapeExtraDumpVER<'a> {
    fn from_bytes(dst: &mut DumpSlice<'a>, sizes: (usize, usize)) -> Result<Self> {
        let info = ShapeExtraInfoVER::mut_from_data(dst).context("info")?;
        let offs = U32VER::mut_slice_from_data(dst, sizes.0).context("offs")?;
        let data = u8::mut_slice_from_data(dst, sizes.1).context("data")?;
        Ok(Self { info, offs, data })
    }
}

#[make_platforms]
pub trait DumpShapeExtraVER {
    fn get_sizes(&self) -> (usize, usize);
    fn write_vals(&self, vals: ShapeExtraDumpVER) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let vals = ShapeExtraDumpVER::from_bytes(dst, self.get_sizes()).context("vals")?;
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
    fn add_size(&self, mut offset: usize) -> usize {
        let sizes = self.get_sizes();
        offset += ShapeExtraInfoVER::size_of() + sizes.0 * U32VER::size_of() + sizes.1;
        offset
    }
}

#[make_platforms]
impl DumpShapeExtraVER for ShapeExtraRefVER<'_> {
    fn get_sizes(&self) -> (usize, usize) {
        (self.offs.len(), self.data.len())
    }
    fn write_vals(&self, vals: ShapeExtraDumpVER) -> Result<()> {
        vals.info.write_from(self.info).context("info")?;
        vals.offs.write_from(self.offs).context("offs")?;
        vals.data.write_from(self.data).context("data")?;
        Ok(())
    }
}

#[make_platforms]
impl DumpShapeExtraVER for &ShapeExtra {
    fn get_sizes(&self) -> (usize, usize) {
        (self.offs.len(), self.data.len())
    }
    fn write_vals(&self, vals: ShapeExtraDumpVER) -> Result<()> {
        *vals.info = (&self.info).into();
        for (src, dst) in self.offs.iter().zip(vals.offs) {
            *dst = src.into();
        }
        vals.data.write_from(&self.data).context("data")?;
        Ok(())
    }
}

#[make_platforms]
pub trait DumpShapeVER {
    fn extra(&self) -> Option<impl DumpShapeExtraVER>;
    fn hk_shapes(&self) -> impl Iterator<Item=impl DumpHkShapeVER>;
    fn write_info(&self, info: &mut ShapeInfoVER) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER, extra_off: Option<usize>) -> Result<()> {
        let info = infos.shapes.next();
        self.write_info(info).context("info")?;
        if let Some(off) = extra_off {
            info.offset = off.into();
            *infos.offsets.next() = (infos.shapes.offset - info.size()).into();
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
        for shape in self.hk_shapes() {
            offset = shape.add_size(offset, infos);
        }
        offset
    }
}

#[make_platforms]
impl DumpShapeVER for ShapeVER {
    fn extra(&self) -> Option<impl DumpShapeExtraVER> {
        ShapeVER::extra(self)
    }
    fn hk_shapes(&self) -> impl Iterator<Item=impl DumpHkShapeVER> {
        self.hk_shapes.iter().map(|x| unsafe { x.as_ref() })
    }
    fn write_info(&self, info: &mut ShapeInfoVER) -> Result<()> {
        info.write_from(self.info())
    }
}

#[make_platforms]
impl DumpShapeVER for Shape {
    fn extra(&self) -> Option<impl DumpShapeExtraVER> {
        self.extra.as_ref()
    }
    fn hk_shapes(&self) -> impl Iterator<Item=impl DumpHkShapeVER> {
        self.hk_shapes.iter()
    }
    fn write_info(&self, info: &mut ShapeInfoVER) -> Result<()> {
        *info = (&self.info).into();
        Ok(())
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
pub struct HkShapeInfo {
    pub unk_0: Vector4,
    pub unk_4: Vector4,
    pub kind: u32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: u32,
    pub unk_12: u32,
    pub unk_13: u32,
    pub unk_14: u32,
    pub unk_15: u32,
    pub unk_16: u32,
    pub unk_17: u32,
    pub unk_18: u32,
    pub unk_19: u32,
}

// BoxShape / ConvexTransformShape / ConvexTranslateShape
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData)]
pub struct BoxShape {
    pub translation: Vector4,
    pub rotation: Vector4,
    pub kind: u32,
    pub key: Crc,
    pub half_extents: Vector3,
    pub unk_13: u32,
    pub unk_14: u32,
    pub unk_15: f32,
    pub unk_16: f32,
    pub unk_17: f32,
    pub unk_18: f32,
    pub unk_19: f32,
}

// SphereShape / ConvexTranslateShape
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData)]
pub struct SphereShape {
    pub translation: Vector4,
    pub rotation: Vector4,
    pub kind: u32,
    pub key: Crc,
    pub radius: f32,
    pub unk_11: u32,
    pub unk_12: f32,
    pub unk_13: f32,
    pub unk_14: f32,
    pub unk_15: f32,
    pub unk_16: f32,
    pub unk_17: f32,
    pub unk_18: f32,
    pub unk_19: f32,
}

// CapsuleShape
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData)]
pub struct CapsuleShape {
    pub translation: Vector4,
    pub rotation: Vector4,
    pub kind: u32,
    pub key: Crc,
    pub point1: Vector3,
    pub point2: Vector3,
    pub radius: f32,
    pub unk_17: f32,
    pub unk_18: f32,
    pub unk_19: f32,
}

// CylinderShape
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData)]
pub struct CylinderShape {
    pub translation: Vector4,
    pub rotation: Vector4,
    pub kind: u32,
    pub key: Crc,
    pub point1: Vector3,
    pub point2: Vector3,
    pub radius: f32,
    pub unk_17: f32,
    pub unk_18: f32,
    pub unk_19: f32,
}

// ConvexVerticesShape
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData)]
pub struct ConvexVerticesInfo {
    pub translation: Vector4,
    pub rotation: Vector4,
    pub kind: u32,
    pub key: Crc,
    pub norm_num: u32,
    pub norms_offset: u32,
    pub vert_num: u32,
    pub verts_offset: u32,
    pub unk_14: f32,
    pub unk_15: f32,
    pub unk_16: f32,
    pub unk_17: f32,
    pub unk_18: f32,
    pub unk_19: f32,
}

#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData)]
pub struct BVTreeMeshInfo {
    pub translation: Vector4,
    pub rotation: Vector4,
    pub kind: u32,
    pub key: Crc,
    /// triangles min bound - 0.05
    pub offset: Vector3,
    /// 254*256*256 / (max triangle bound + 0.1)
    pub tree_scale: f32,
    pub tree_size: u32,
    pub tree_offset: u32,  // u8
    pub vert_num: u32,     // vert_num
    pub verts_offset: u32, // vec3 f32 verts offset
    pub tri_num: u32,      // tri_num
    pub inds_offset: u32,  // vec3 u16 , inds offset
}

#[make_platforms]
#[derive(Debug, Clone)]
pub enum HkShapeVER {
    Box(NonNull<BoxShapeVER>),
    Sphere(NonNull<SphereShapeVER>),
    Capsule(NonNull<CapsuleShapeVER>),
    Cylinder(NonNull<CylinderShapeVER>),
    ConvexVertices {
        info: NonNull<ConvexVerticesInfoVER>,
        norms: NonNull<[Vector4VER]>,
        verts: NonNull<[Vector3VER]>,
    },
    BVTreeMesh {
        info: NonNull<BVTreeMeshInfoVER>,
        tree: NonNull<[u8]>,
        verts: NonNull<[Vector3VER]>,
        inds: NonNull<[U16VER]>,
    },
    Unknown(NonNull<HkShapeInfoVER>),
}

#[make_platforms]
pub enum HkShapeRefVER<'a> {
    Box(&'a BoxShapeVER),
    Sphere(&'a SphereShapeVER),
    Capsule(&'a CapsuleShapeVER),
    Cylinder(&'a CylinderShapeVER),
    ConvexVertices {
        info: &'a ConvexVerticesInfoVER,
        norms: &'a [Vector4VER],
        verts: &'a [Vector3VER],
    },
    BVTreeMesh {
        info: &'a BVTreeMeshInfoVER,
        tree: &'a [u8],
        verts: &'a [Vector3VER],
        inds: &'a [U16VER],
    },
    Unknown(&'a HkShapeInfoVER),
}

#[make_platforms]
unsafe impl Sync for HkShapeVER {}
#[make_platforms]
unsafe impl Send for HkShapeVER {}

#[make_platforms]
impl HkShapeVER {
    pub fn from_bytes(src: &Arc<[u8]>, offset: usize) -> Result<Self> {
        let info = HkShapeInfoVER::from_data(&src[offset..]).context("info")?;
        Ok(match info.kind.get() {
            1 => HkShapeVER::Box(<&BoxShapeVER>::into(transmute_ref!(info))),
            2 => HkShapeVER::Sphere(<&SphereShapeVER>::into(transmute_ref!(info))),
            3 => HkShapeVER::Capsule(<&CapsuleShapeVER>::into(transmute_ref!(info))),
            4 => HkShapeVER::Cylinder(<&CylinderShapeVER>::into(transmute_ref!(info))),
            5 => {
                let info: &ConvexVerticesInfoVER = transmute_ref!(info);

                let norms = Vector4VER::slice_from_data(
                    &src[info.norms_offset.get() as usize..],
                    info.norm_num.get() as usize,
                )
                .context("vals")?;
                let mut vert_num = info.vert_num.get() as usize; // sketchy stuff to account for data that was not otherwise captured, is it needed?
                while (info.verts_offset.get() as usize + vert_num * 12) % 16 != 0 {
                    vert_num += 1;
                }
                let verts = Vector3VER::slice_from_data(
                    &src[info.verts_offset.get() as usize..],
                    vert_num * 3,
                )
                .context("vals")?;
                HkShapeVER::ConvexVertices {
                    info: info.into(),
                    norms: norms.into(),
                    verts: verts.into(),
                }
            }
            6 => {
                let info: &BVTreeMeshInfoVER = transmute_ref!(info);

                let tree = u8::slice_from_data(
                    &src[info.tree_offset.get() as usize..],
                    info.tree_size.get() as usize,
                )
                .context("vals")?;
                let verts = Vector3VER::slice_from_data(
                    &src[info.verts_offset.get() as usize..],
                    info.vert_num.get() as usize,
                )
                .context("vals")?;
                let inds = U16VER::slice_from_data(
                    &src[info.inds_offset.get() as usize..],
                    info.tri_num.get() as usize * 3,
                )
                .context("vals")?;

                HkShapeVER::BVTreeMesh {
                    info: info.into(),
                    tree: tree.into(),
                    verts: verts.into(),
                    inds: inds.into(),
                }
            }
            _ => {
                warn!("Unknown & Unhandled HkShape type {}", info.kind);
                HkShapeVER::Unknown(info.into())
            }
        })
    }
    pub unsafe fn as_ref(&self) -> HkShapeRefVER<'_> {
        match self {
            Self::Box(info) => HkShapeRefVER::Box(info.as_ref()),
            Self::Sphere(info) => HkShapeRefVER::Sphere(info.as_ref()),
            Self::Capsule(info) => HkShapeRefVER::Capsule(info.as_ref()),
            Self::Cylinder(info) => HkShapeRefVER::Cylinder(info.as_ref()),
            Self::ConvexVertices { info, norms, verts } => HkShapeRefVER::ConvexVertices {
                info: info.as_ref(),
                norms: norms.as_ref(),
                verts: verts.as_ref(),
            },
            Self::BVTreeMesh {
                info,
                tree,
                verts,
                inds,
            } => HkShapeRefVER::BVTreeMesh {
                info: info.as_ref(),
                tree: tree.as_ref(),
                verts: verts.as_ref(),
                inds: inds.as_ref(),
            },
            Self::Unknown(info) => HkShapeRefVER::Unknown(info.as_ref()),
        }
    }
}

/*
#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl HkShapeVER {
    #[getter]
    pub fn norms(&self) -> Option<&[Vector4VER]> {
        match self {
            HkShapeVER::ConvexVertices { norms, .. } => Some(unsafe { norms }),
            _ => None,
        }
    }
    #[getter]
    pub fn verts(&self) -> Option<&[Vector3VER]> {
        match self {
            HkShapeVER::ConvexVertices { verts, .. } => Some(unsafe { verts }),
            HkShapeVER::BVTreeMesh { verts, .. } => Some(unsafe { verts }),
            _ => None,
        }
    }
    #[getter]
    pub fn tree(&self) -> Option<&[u8]> {
        match self {
            HkShapeVER::BVTreeMesh { tree, .. } => Some(unsafe { tree }),
            _ => None,
        }
    }
    #[getter]
    pub fn inds(&self) -> Option<&[U16VER]> {
        match self {
            HkShapeVER::BVTreeMesh { inds, .. } => Some(unsafe { inds }),
            _ => None,
        }
    }
}
*/

#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
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

#[make_platforms]
impl From<HkShapeRefVER<'_>> for HkShape {
    fn from(val: HkShapeRefVER) -> Self {
        match val {
            HkShapeRefVER::Box(info) => HkShape::Box(info.into()),
            HkShapeRefVER::Sphere(info) => HkShape::Sphere(info.into()),
            HkShapeRefVER::Capsule(info) => HkShape::Capsule(info.into()),
            HkShapeRefVER::Cylinder(info) => HkShape::Cylinder(info.into()),
            HkShapeRefVER::ConvexVertices { info, norms, verts } => HkShape::ConvexVertices {
                info: info.into(),
                norms: norms.iter().map(|x| x.into()).collect(),
                verts: verts.iter().map(|x| x.into()).collect(),
            },
            HkShapeRefVER::BVTreeMesh {
                info,
                tree,
                verts,
                inds,
            } => HkShape::BVTreeMesh {
                info: info.into(),
                tree: tree.iter().cloned().collect(),
                verts: verts.iter().map(|x| x.into()).collect(),
                inds: inds.iter().map(|x| x.into()).collect(),
            },
            HkShapeRefVER::Unknown(info) => HkShape::Unknown(info.into()),
        }
    }
}

#[make_platforms]
pub enum HkShapeDumpVER<'a> {
    ConvexVertices {
        norms: &'a mut [Vector4VER],
        verts: &'a mut [Vector3VER],
    },
    BVTreeMesh {
        tree: &'a mut [u8],
        verts: &'a mut [Vector3VER],
        inds: &'a mut [U16VER],
    },
    None
}

#[make_platforms]
impl<'a> HkShapeDumpVER<'a> {
    fn from_bytes(dst: &mut DumpSlice<'a>, sizes: (usize, usize, usize), info: &mut HkShapeInfoVER) -> Result<Self> {
        Ok(match info.kind.get() {
            5 => {
                let info: &mut ConvexVerticesInfoVER = zerocopy::transmute_mut!(info);

                dst.align(16);
                info.norms_offset = dst.offset.into();
                let norms = Vector4VER::mut_slice_from_data(dst, sizes.0).context("norms")?;
                info.norm_num = norms.len().into();

                info.verts_offset = dst.offset.into();
                let verts = Vector3VER::mut_slice_from_data(dst, sizes.1).context("verts")?;
                info.vert_num = (verts.len() - sizes.2).into();
                Self::ConvexVertices { norms, verts }
            }
            6 => {
                let info: &mut BVTreeMeshInfoVER = zerocopy::transmute_mut!(info);

                info.verts_offset = dst.offset.into();
                let verts = Vector3VER::mut_slice_from_data(dst, sizes.0).context("verts")?;
                info.vert_num = verts.len().into();

                info.inds_offset = dst.offset.into();
                let inds = U16VER::mut_slice_from_data(dst, sizes.1).context("inds")?;
                info.tri_num = (inds.len() / 3).into();

                dst.align(4);
                info.tree_offset = dst.offset.into();
                let tree = u8::mut_slice_from_data(dst, sizes.2).context("tree")?;
                info.tree_size = tree.len().into();
                dst.align(4);
                Self::BVTreeMesh { tree, verts, inds }
            }
            _ => Self::None
        })
    }
}

#[make_platforms]
pub trait DumpHkShapeVER {
    fn kind(&self) -> u32;
    fn get_sizes(&self) -> (usize, usize, usize);
    fn write_info(&self, info: &mut HkShapeInfoVER) -> Result<()>;
    fn write_vals(&self, vals: HkShapeDumpVER) -> Result<()>;
    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        counts.hk_shapes += 1;
        match self.kind() {
            5 => {
                let sizes = self.get_sizes();
                offset = align_offset(offset, 16);
                offset += sizes.0 * Vector4VER::size_of();
                offset += sizes.1 * Vector3VER::size_of();
            }
            6 => {
                let sizes = self.get_sizes();
                offset += sizes.0 * Vector3VER::size_of();
                offset += sizes.1 * U16VER::size_of();
                offset = align_offset(offset, 4);
                offset += sizes.2;
                offset = align_offset(offset, 4);
            }
            _ => ()
        }
        offset
    }
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER) -> Result<()> {
        let info = infos.hk_shapes.next();
        self.write_info(info).context("write info")?;
        let vals = HkShapeDumpVER::from_bytes(dst, self.get_sizes(), info).context("vals")?;
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
}

#[make_platforms]
impl DumpHkShapeVER for HkShapeRefVER<'_> { 
    fn kind(&self) -> u32 {
        match self {
            Self::Box(info) => info.kind.get(),
            Self::Sphere(info) => info.kind.get(),
            Self::Capsule(info) => info.kind.get(),
            Self::Cylinder(info) => info.kind.get(),
            Self::ConvexVertices { info, .. } => info.kind.get(),
            Self::BVTreeMesh { info, .. } => info.kind.get(),
            Self::Unknown(info) => info.kind.get(),
        }
    }
    fn get_sizes(&self) -> (usize, usize, usize) {
        match self {
            Self::ConvexVertices { info, verts, norms } => (norms.len(), verts.len(), verts.len() - info.vert_num.get() as usize),
            Self::BVTreeMesh { tree, verts, inds, .. } => (verts.len(), inds.len(), tree.len()),
            _ => (0,0,0)
        }
    }
    fn write_info(&self, info: &mut HkShapeInfoVER) -> Result<()> {
        match self {
            Self::Box(src) => info.write_from(zerocopy::transmute_ref!(*src)),
            Self::Sphere(src) => info.write_from(zerocopy::transmute_ref!(*src)),
            Self::Capsule(src) => info.write_from(zerocopy::transmute_ref!(*src)),
            Self::Cylinder(src) => info.write_from(zerocopy::transmute_ref!(*src)),
            Self::ConvexVertices { info: src, .. } => info.write_from(zerocopy::transmute_ref!(*src)),
            Self::BVTreeMesh { info: src, .. } => info.write_from(zerocopy::transmute_ref!(*src)),
            Self::Unknown(src) => info.write_from(src),
        }
    }
    fn write_vals(&self, vals: HkShapeDumpVER) -> Result<()> {
        match (self, vals) {
            (Self::ConvexVertices { verts: src1, norms: src2, .. }, HkShapeDumpVER::ConvexVertices { verts: dst1, norms: dst2 }) => {
                dst1.write_from(src1).context("verts")?;
                dst2.write_from(src2).context("norms")?;
            },
            (Self::BVTreeMesh { verts: src1, inds: src2, tree: src3, .. }, HkShapeDumpVER::BVTreeMesh { verts: dst1, inds: dst2, tree: dst3 }) => {
                dst1.write_from(src1).context("verts")?;
                dst2.write_from(src2).context("inds")?;
                dst3.write_from(src3).context("tree")?;
            },
            _ => ()
        }
        Ok(())
    }
}

#[make_platforms]
impl DumpHkShapeVER for &HkShape { 
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
    fn write_info(&self, info: &mut HkShapeInfoVER) -> Result<()> {
        match self {
            HkShape::Box(src) => *zerocopy::transmute_mut!(info) = BoxShapeVER::from(src),
            HkShape::Sphere(src) => *zerocopy::transmute_mut!(info) = SphereShapeVER::from(src),
            HkShape::Capsule(src) => *zerocopy::transmute_mut!(info) = CapsuleShapeVER::from(src),
            HkShape::Cylinder(src) => *zerocopy::transmute_mut!(info) = CylinderShapeVER::from(src),
            HkShape::ConvexVertices { info: src, .. } => *zerocopy::transmute_mut!(info) = ConvexVerticesInfoVER::from(src),
            HkShape::BVTreeMesh { info: src, .. } => *zerocopy::transmute_mut!(info) = BVTreeMeshInfoVER::from(src),
            HkShape::Unknown(src) => *info = src.into(),
        }
        Ok(())
    }
    fn write_vals(&self, vals: HkShapeDumpVER) -> Result<()> {
        match (self, vals) {
            (HkShape::ConvexVertices { verts: src1, norms: src2, .. }, HkShapeDumpVER::ConvexVertices { verts: dst1, norms: dst2 }) => {
                for (src, dst) in src1.iter().zip(dst1) {
                    *dst = src.into();
                }
                for (src, dst) in src2.iter().zip(dst2) {
                    *dst = src.into();
                }
            },
            (HkShape::BVTreeMesh { verts: src1, inds: src2, tree: src3, .. }, HkShapeDumpVER::BVTreeMesh { verts: dst1, inds: dst2, tree: dst3 }) => {
                for (src, dst) in src1.iter().zip(dst1) {
                    *dst = src.into();
                }
                for (src, dst) in src2.iter().zip(dst2) {
                    *dst = src.into();
                }
                dst3.write_from(src3).context("tree")?;
            },
            _ => ()
        }
        Ok(())
    }
}

#[cfg(feature = "python")]
#[make_platforms]
impl<'a, 'py> IntoPyObject<'py> for HkShapeRefVER<'a> {
    type Target = <HkShape as IntoPyObject<'py>>::Target;
    type Output = <HkShape as IntoPyObject<'py>>::Output;
    type Error = <HkShape as IntoPyObject<'py>>::Error;
    #[inline(always)]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        HkShape::from(self).into_pyobject(py)
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
pub struct HkConstraintInfo {
    pub kind: u32,
    pub bone_parents_offset: u32,
    pub bone_parents_num: u32,
    pub bone_names_offset: u32,
    pub bone_names_num: u32,
    pub bone_transforms_offset: u32,
    pub bone_transforms_num: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub bones_offset: u32,
    pub bones_num: u16,
    pub bone_order_num: u16,
    pub bone_order_offset: u32,
    pub unk_13: u32,
    pub unk_14: f32,
    pub vals2_num: u32,
    pub vals2_offset: u32,
    pub unk_17: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
pub struct TRS {
    pub translation: Vector4,
    pub rotation: Vector4,
    pub scale: Vector4,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "model.shape"))]
#[derive(Debug, Clone)]
pub struct HkConstraintVER {
    _ptr: Arc<[u8]>,
    info: NonNull<HkConstraintInfoVER>,
    bone_parents: NonNull<[I16VER]>,
    bone_names: Box<[(NonNull<str>, U32VER, U32VER)]>,
    name_offsets: NonNull<[U32VER]>,
    bone_transforms: NonNull<[TRSVER]>,
    bones: NonNull<[U32VER]>,
    bones_order: NonNull<[Key2VER]>,
    vals2: NonNull<[F32VER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(HkConstraintVER);

#[make_platforms]
unsafe impl Sync for HkConstraintVER {}
#[make_platforms]
unsafe impl Send for HkConstraintVER {}

#[make_platforms]
impl HkConstraintVER {
    pub fn from_bytes(src: &Arc<[u8]>, offset: usize) -> Result<Self> {
        let info = HkConstraintInfoVER::from_data(&src[offset..]).context("info")?;
        if info.kind.get() != 0 {
            warn!("Unknown & Unhandled HkConstraint type {}", info.kind);
        }

        let bone_parents = I16VER::slice_from_data(
            &src[info.bone_parents_offset.get() as usize..],
            info.bone_parents_num.get() as usize,
        )
        .context("bone_parents")?;
        assert!(bone_parents[0].get() == -1);

        let name_offsets = U32VER::slice_from_data(
            &src[info.bone_names_offset.get() as usize..],
            info.bone_names_num.get() as usize,
        )
        .context("name_offsets")?;
        let mut bone_names = Vec::with_capacity(name_offsets.len());
        for offset_ in name_offsets.iter() {
            let start = U32VER::from_data(&src[offset_.get() as usize..]).context("start")?;
            let val_ = U32VER::from_data(&src[offset_.get() as usize + 4..]).context("val_")?;
            let mut offset = start.get() as usize;
            while src[offset] != 0 {
                offset += 1;
            }
            bone_names.push((
                NonNull::from_ref(str::from_utf8(&src[start.get() as usize..offset]).context("string")?),
                *start,
                *val_,
            ));
        }
        let bone_transforms = TRSVER::slice_from_data(
            &src[info.bone_transforms_offset.get() as usize..],
            info.bone_transforms_num.get() as usize,
        )
        .context("bone_tranforms")?;
        let bones = U32VER::slice_from_data(
            &src[info.bones_offset.get() as usize..],
            info.bones_num.get() as usize,
        )
        .context("vals")?;
        let bones_order = Key2VER::slice_from_data(
            &src[info.bone_order_offset.get() as usize..],
            info.bone_order_num.get() as usize,
        )
        .context("vals")?;

        let vals2 = F32VER::slice_from_data(
            &src[info.vals2_offset.get() as usize..],
            info.vals2_num.get() as usize * 42,
        )
        .context("vals2")?;

        Ok(Self {
            _ptr: src.clone(),
            info: info.into(),
            bone_parents: bone_parents.into(),
            bone_names: bone_names.into(),
            name_offsets: name_offsets.into(),
            bone_transforms: bone_transforms.into(),
            bones: bones.into(),
            bones_order: bones_order.into(),
            vals2: vals2.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl HkConstraintVER {
    #[getter]
    pub fn info(&self) -> &HkConstraintInfoVER {
        unsafe { self.info.as_ref() }
    }
    #[getter]
    pub fn bone_parents(&self) -> &[I16VER] {
        unsafe { self.bone_parents.as_ref() }
    }
    #[getter]
    pub fn bone_names(&self) -> Vec<(&str, U32VER, U32VER)> {
        self.bone_names
            .iter()
            .map(|(a, b, c)| (unsafe { a.as_ref() }, b.clone(), c.clone()))
            .collect()
    }
    #[getter]
    pub fn name_offsets(&self) -> &[U32VER] {
        unsafe { self.name_offsets.as_ref() }
    }
    #[getter]
    pub fn bone_transforms(&self) -> &[TRSVER] {
        unsafe { self.bone_transforms.as_ref() }
    }
    #[getter]
    pub fn bones(&self) -> &[U32VER] {
        unsafe { self.bones.as_ref() }
    }
    #[getter]
    pub fn bones_order(&self) -> &[Key2VER] {
        unsafe { self.bones_order.as_ref() }
    }
    #[getter]
    pub fn vals2(&self) -> &[F32VER] {
        unsafe { self.vals2.as_ref() }
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
pub struct HkConstraint {
    pub info: HkConstraintInfo,
    pub bone_parents: Vec<i16>,
    pub bone_names: Vec<String>,
    pub bone_transforms: Vec<TRS>, // probably f32
    pub vals2: Vec<f32>,           // probably f32
}

#[make_platforms]
impl From<&HkConstraintVER> for HkConstraint {
    fn from(val: &HkConstraintVER) -> Self {
        Self {
            info: val.info().into(),
            bone_parents: val.bone_parents().iter().map(|x| x.into()).collect(),
            bone_names: val
                .bone_names()
                .iter()
                .map(|(x, _, _)| x.to_string())
                .collect(),
            bone_transforms: val.bone_transforms().iter().map(|x| x.into()).collect(),
            vals2: val.vals2().iter().map(|x| x.into()).collect(),
        }
    }
}

#[make_platforms]
pub trait DumpHkConstraintVER {
    fn bone_names_num(&self) -> usize;
    fn bone_transforms_num(&self) -> usize;
    fn bone_order_num(&self) -> usize;
    fn bone_parents_num(&self) -> usize;
    fn vals2_num(&self) -> usize;
    fn bone_names(&self) -> impl Iterator<Item=(&str, impl Into<U32VER>)>;
    fn write_info(&self, info: &mut HkConstraintInfoVER) -> Result<()>;
    fn write_bone_transforms(&self, bone_transforms: &mut [TRSVER]) -> Result<()>;
    fn write_bone_order(&self, bone_order: &mut [Key2VER]) -> Result<()>;
    fn write_bone_parents(&self, bone_parents: &mut [I16VER]) -> Result<()>;
    fn write_vals2(&self, vals2: &mut [F32VER]) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER, bones_num: u16, bones_offset: u32) -> Result<()> {
        let info = infos.hk_constraints.next();
        self.write_info(info).context("write info")?;
        info.bones_num = bones_num.into();
        info.bones_offset = bones_offset.into();

        info.bone_names_offset = dst.offset.into();
        let mut name_off = dst.offset;
        let name_offsets= U32VER::mut_slice_from_data(dst, self.bone_names_num()).context("name_offsets")?;
        let mut string_off = dst.offset;
        let string_offs = U32VER::mut_slice_from_data(dst, name_offsets.len()*2).context("string_offs")?;
        info.bone_names_num = name_offsets.len().into();

        dst.align(16);
        info.bone_transforms_offset = dst.offset.into();
        let bone_transforms = TRSVER::mut_slice_from_data(dst, self.bone_transforms_num()).context("bone_transforms")?;
        info.bone_transforms_num = bone_transforms.len().into();
        self.write_bone_transforms(bone_transforms).context("write bone_transforms")?;

        info.bone_order_offset = dst.offset.into();
        let bone_order = Key2VER::mut_slice_from_data(dst, self.bone_order_num()).context("bone_order")?;
        info.bone_order_num = bone_order.len().into();
        self.write_bone_order(bone_order).context("write bone_order")?;

        info.bone_parents_offset = dst.offset.into();
        let bone_parents = I16VER::mut_slice_from_data(dst, self.bone_parents_num()).context("bone_parents")?;
        info.bone_parents_num = bone_parents.len().into();
        self.write_bone_parents(bone_parents).context("write bone_parents")?;
        dst.align(4);

        for (i, (string, val)) in self.bone_names().enumerate() {
            let s = string.as_bytes(); 
            let off = string_off + i * 2 * size_of::<U32VER>();
            name_offsets[i] = off.into();
            string_offs[i*2] = dst.offset.into();
            string_offs[i*2+1] = val.into();
            s.dump_into(dst).with_context(|| format!("string: {}", string))?;
            dst.align(4);
            *infos.offsets.next() = name_off.into();
            *infos.offsets.next() = string_off.into();
            name_off += size_of::<U32VER>();
            string_off += size_of::<U32VER>() * 2;
        }

        info.vals2_offset = dst.offset.into();
        let vals2 = F32VER::mut_slice_from_data(dst, self.vals2_num()).context("vals2")?;
        self.write_vals2(vals2).context("write vals2")?;
        if vals2.len() == 0 {
            info.vals2_offset = 0u32.into();
        }
        Ok(())
    }
    fn add_size(&self, mut offset: usize, infos: &mut InfoCounts) -> usize {
        infos.hk_constraints += 1;
        infos.offsets += self.bone_names_num() * 2;
        offset += size_of::<U32VER>() * self.bone_names_num() * 3;
        offset = align_offset(offset, 16) + size_of::<TRSVER>() * self.bone_transforms_num();
        offset += size_of::<Key2VER>() * self.bone_order_num();
        offset += size_of::<I16VER>() * self.bone_parents_num();
        offset = align_offset(offset, 4);

        for (string, _) in self.bone_names() {
            let s = string.as_bytes(); 
            offset += s.len();
            offset = align_offset(offset, 4);
        }

        offset += size_of::<F32VER>() * self.vals2_num();
        offset
    }
}

#[make_platforms]
impl DumpHkConstraintVER for HkConstraintVER {
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
    fn bone_names(&self) -> impl Iterator<Item=(&str, impl Into<U32VER>)> {
        self.bone_names.iter().map(|(s,_,val)| (unsafe { s.as_ref() }, val))
    }
    fn write_info(&self, info: &mut HkConstraintInfoVER) -> Result<()> {
        info.write_from(self.info())
    }
    fn write_bone_transforms(&self, bone_transforms: &mut [TRSVER]) -> Result<()> {
        bone_transforms.write_from(self.bone_transforms())
    }
    fn write_bone_order(&self, bone_order: &mut [Key2VER]) -> Result<()> {
        bone_order.write_from(self.bones_order())
    }
    fn write_bone_parents(&self, bone_parents: &mut [I16VER]) -> Result<()> {
        bone_parents.write_from(self.bone_parents())
    }
    fn write_vals2(&self, vals2: &mut [F32VER]) -> Result<()> {
        vals2.write_from(self.vals2())
    }
}

#[make_platforms]
impl DumpHkConstraintVER for HkConstraint {
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
    fn bone_names(&self) -> impl Iterator<Item=(&str, impl Into<U32VER>)> {
        self.bone_names.iter().map(|(val)| (val.as_str(), 0u32))
    }
    fn write_info(&self, info: &mut HkConstraintInfoVER) -> Result<()> {
        *info = (&self.info).into();
        Ok(())
    }
    fn write_bone_transforms(&self, bone_transforms: &mut [TRSVER]) -> Result<()> {
        for (src, dst) in self.bone_transforms.iter().zip(bone_transforms) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_bone_order(&self, bone_order: &mut [Key2VER]) -> Result<()> {
        let mut new_bone_order = self.bone_names.iter().enumerate().map(|(i,val)| Key2 { key: hash_string(val.as_ref(), None).into(), val: i as u32 }).collect::<Vec<_>>();
        new_bone_order.sort_by_key(|x| x.key.get());
        for (src, dst) in new_bone_order.iter().zip(bone_order) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_bone_parents(&self, bone_parents: &mut [I16VER]) -> Result<()> {
        for (src, dst) in self.bone_parents.iter().zip(bone_parents) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_vals2(&self, vals2: &mut [F32VER]) -> Result<()> {
        for (src, dst) in self.vals2.iter().zip(vals2) {
            *dst = src.into();
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "model.shape", get_all, set_all))]
pub struct HkConstraintData {
    pub kind: u32,
    pub unk_1: u32,
    pub unk_2: u32,
    pub unk_3: u32,
    pub unk_4: u32,
    pub unk_5: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: u32,
    pub unk_12: u32,
    pub unk_13: u32,
    pub unk_14: u32,
    pub unk_15: u32,
    pub unk_16: u32,
    pub unk_17: u32,
    pub unk_18: u32,
    pub unk_19: u32,
    pub unk_20: u32,
    pub unk_21: u32,
    pub unk_22: u32,
    pub unk_23: u32,
    pub unk_24: u32,
    pub unk_25: u32,
    pub unk_26: u32,
    pub unk_27: u32,
    pub unk_28: u32,
}
