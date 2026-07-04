use anyhow::{Context, Result};
use log::warn;
use zerocopy::transmute_ref;
use enum_dispatch::enum_dispatch;

use crate::types::GetNative;
use crate::{
    types::{Crc, DumpData, RefFromData, Vector3, Vector4, DumpSlice, align_offset, hash_string, OrderedData, OrderedDataStrict, get_default_ref, ref_slice, string, slice},
    level::{
        pak::block1::infos::InfoCounts,
        model::Key2
    }
};
#[make_platforms]
use crate::{
    types::{Vector3VER, Vector4VER, f32VER, i16VER, u16VER, u32VER, CrcVER, u8VER},
    level::{
        model::Key2VER,
        pak::block1::infos::DumpInfosVER
    }
};
use lotrc_proc::{make_platforms, OrderedData};

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
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

///gen_ffi:export
#[derive(Default, Debug, Clone, OrderedData)]
pub struct ShapeExtraInfo {
    pub size: u32,
    pub scale: f32,
    pub a: f32,
    pub b: f32,
}

#[make_platforms]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct ShapeExtraRefVER<'a> {
    info: &'a ShapeExtraInfoVER,
    offs: ref_slice<'a, u32VER>,
    data: ref_slice<'a, u8>,
}

#[make_platforms]
impl<'a> ShapeExtraRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a ShapeInfoVER) -> Result<Self> {
        let mut offset = info.offset.get() as usize;
        let info = ShapeExtraInfoVER::from_data(&src[offset..]).context("info")?;
        offset += info.size();
        let offs = u32VER::slice_from_data(&src[offset..], info.size.get() as usize)
            .context("vals")?;
        offset += offs.size();
        let mut off = offset + offs.last().unwrap().get() as usize;
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

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ShapeRefVER<'a> {
    info: &'a ShapeInfoVER,
    extra: Option<ShapeExtraRefVER<'a>>,
    hk_shapes: slice<HkShapeRefVER<'a>>
}

#[make_platforms]
impl<'a> ShapeRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a ShapeInfoVER) -> Result<Self> {
        let extra = if info.kind.get() == 0 {
            Some(ShapeExtraRefVER::from_data(src, info).context("extra")?)
        } else {
            None
        };
        let infos = HkShapeInfoVER::slice_from_data(&src[info.hk_shape_offset.get() as usize..], info.hk_shape_num.get() as usize).context("hk_shape infos")?;
        let hk_shapes = infos.into_iter().enumerate().map(|(i, info)| HkShapeRefVER::from_data(src, info).with_context(|| format!("hk_shape {}", i))).collect::<Result<Vec<_>>>()?.into_boxed_slice();
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

#[make_platforms]
impl From<&ShapeExtraRefVER<'_>> for ShapeExtra {
    fn from(val: &ShapeExtraRefVER) -> Self {
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

#[make_platforms]
impl From<&ShapeRefVER<'_>> for Shape {
    fn from(val: &ShapeRefVER) -> Self {
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

#[make_platforms]
pub struct ShapeExtraDumpVER<'a> {
    info: &'a mut ShapeExtraInfoVER,
    offs: &'a mut [u32VER],
    data: &'a mut [u8],
}

#[make_platforms]
impl<'a> ShapeExtraDumpVER<'a> {
    fn from_bytes(dst: &mut DumpSlice<'a>, sizes: (usize, usize)) -> Result<Self> {
        let info = ShapeExtraInfoVER::mut_from_data(dst).context("info")?;
        let offs = u32VER::mut_slice_from_data(dst, sizes.0).context("offs")?;
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
        offset += ShapeExtraInfoVER::size_of() + sizes.0 * u32VER::size_of() + sizes.1;
        offset
    }
}

#[make_platforms]
#[enum_dispatch(DumpShapeVER)]
pub enum ShapeVER<'a> {
    Ref(ShapeRefVER<'a>),
    Owned(Shape)
}

#[make_platforms]
impl DumpShapeExtraVER for ShapeExtraRefVER<'_> {
    fn get_sizes(&self) -> (usize, usize) {
        (self.offs.len(), self.data.len())
    }
    fn write_vals(&self, vals: ShapeExtraDumpVER) -> Result<()> {
        vals.info.write_from(self.info).context("info")?;
        vals.offs.write_from(&self.offs[..]).context("offs")?;
        vals.data.write_from(&self.data[..]).context("data")?;
        Ok(())
    }
}

#[make_platforms]
impl DumpShapeExtraVER for ShapeExtra {
    fn get_sizes(&self) -> (usize, usize) {
        (self.offs.len(), self.data.len())
    }
    fn write_vals(&self, vals: ShapeExtraDumpVER) -> Result<()> {
        *vals.info = (&self.info).conv();
        for (src, dst) in self.offs.iter().zip(vals.offs) {
            *dst = src.conv();
        }
        vals.data.write_from(&self.data).context("data")?;
        Ok(())
    }
} 

#[make_platforms]
pub trait DumpShapeImplVER {
    fn extra(&self) -> Option<&impl DumpShapeExtraVER>;
    fn has_hk_shapes(&self) -> bool;
    fn hk_shapes(&self) -> impl Iterator<Item=&impl DumpHkShapeVER>;
    fn write_info(&self, info: &mut ShapeInfoVER) -> Result<()>;
}

#[make_platforms]
#[enum_dispatch]
pub trait DumpShapeVER {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER, extra_off: Option<usize>) -> Result<()>;
    fn add_size(&self, offset: usize, infos: &mut InfoCounts) -> usize;
    fn add_extra_size(&self, offset: usize) -> usize;
    fn dump_extra_into(&self, dst: &mut DumpSlice) -> Result<Option<usize>>;
}
#[make_platforms]
impl<T: DumpShapeImplVER> DumpShapeVER for T {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER, extra_off: Option<usize>) -> Result<()> {
        let info_offset = infos.shapes.offset;
        let info = infos.shapes.next().context("shapes")?;
        self.write_info(info).context("info")?;
        if let Some(off) = extra_off {
            info.offset = off.conv();
            *infos.offsets.next().context("offsets")? = (info_offset + std::mem::offset_of!(ShapeInfoVER, offset)).conv();
        }
        if self.has_hk_shapes() {
            info.hk_shape_offset = infos.hk_shapes.offset.conv();
            *infos.offsets.next().context("offsets")? = (info_offset + std::mem::offset_of!(ShapeInfoVER, hk_shape_offset)).conv();
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

#[make_platforms]
impl DumpShapeImplVER for ShapeRefVER<'_> {
    fn extra(&self) -> Option<&impl DumpShapeExtraVER> {
        self.extra.as_ref()
    }
    fn has_hk_shapes(&self) -> bool {
        !self.hk_shapes.is_empty()    
    }
    fn hk_shapes(&self) -> impl Iterator<Item=&impl DumpHkShapeVER> {
        self.hk_shapes.iter()
    }
    fn write_info(&self, info: &mut ShapeInfoVER) -> Result<()> {
        info.write_from(self.info)
    }
}

#[make_platforms]
impl DumpShapeImplVER for Shape {
    fn extra(&self) -> Option<&impl DumpShapeExtraVER> {
        self.extra.as_ref()
    }
    fn has_hk_shapes(&self) -> bool {
        !self.hk_shapes.is_empty()    
    }
    fn hk_shapes(&self) -> impl Iterator<Item=&impl DumpHkShapeVER> {
        self.hk_shapes.iter()
    }
    fn write_info(&self, info: &mut ShapeInfoVER) -> Result<()> {
        *info = (&self.info).conv();
        Ok(())
    }
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
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

///gen_ffi:export
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

///gen_ffi:export
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

///gen_ffi:export
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

///gen_ffi:export
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

///gen_ffi:export
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

///gen_ffi:export
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
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ConvexVerticesRefVER<'a> {
    pub info: &'a ConvexVerticesInfoVER,
    pub norms: ref_slice<'a, Vector4VER>,
    pub verts: ref_slice<'a, Vector3VER>,
}

#[make_platforms]
impl<'a> ConvexVerticesRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a HkShapeInfoVER) -> Result<Self> {
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

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct BVTreeMeshRefVER<'a> {
    info: &'a BVTreeMeshInfoVER,
    tree: ref_slice<'a, u8>,
    verts: ref_slice<'a, Vector3VER>,
    inds: ref_slice<'a, u16VER>
}

#[make_platforms]
impl<'a> BVTreeMeshRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a HkShapeInfoVER) -> Result<Self> {
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
        let inds = u16VER::slice_from_data(
            &src[info.inds_offset.get() as usize..],
            info.tri_num.get() as usize * 3,
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

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C, u8))]
#[derive(PartialEq)]
pub enum HkShapeRefVER<'a> {
    Box(&'a BoxShapeVER),
    Sphere(&'a SphereShapeVER),
    Capsule(&'a CapsuleShapeVER),
    Cylinder(&'a CylinderShapeVER),
    ConvexVertices(ConvexVerticesRefVER<'a>),
    BVTreeMesh(BVTreeMeshRefVER<'a>),
    Unknown(&'a HkShapeInfoVER),
}

#[make_platforms]
impl<'a> HkShapeRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a HkShapeInfoVER) -> Result<Self> {
        Ok(match info.kind.get() {
            1 => Self::Box(transmute_ref!(info)),
            2 => Self::Sphere(transmute_ref!(info)),
            3 => Self::Capsule(transmute_ref!(info)),
            4 => Self::Cylinder(transmute_ref!(info)),
            5 => Self::ConvexVertices(ConvexVerticesRefVER::from_data(src, info)?),
            6 => Self::BVTreeMesh(BVTreeMeshRefVER::from_data(src, info)?),
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

#[make_platforms]
impl From<&HkShapeRefVER<'_>> for HkShape {
    fn from(val: &HkShapeRefVER) -> Self {
        match val {
            HkShapeRefVER::Box(info) => HkShape::Box(info.conv()),
            HkShapeRefVER::Sphere(info) => HkShape::Sphere(info.conv()),
            HkShapeRefVER::Capsule(info) => HkShape::Capsule(info.conv()),
            HkShapeRefVER::Cylinder(info) => HkShape::Cylinder(info.conv()),
            HkShapeRefVER::ConvexVertices(ConvexVerticesRefVER { info, norms, verts }) => HkShape::ConvexVertices {
                info: info.conv(),
                norms: norms.iter().map(|x| x.conv()).collect(),
                verts: verts.iter().map(|x| x.conv()).collect(),
            },
            HkShapeRefVER::BVTreeMesh(BVTreeMeshRefVER {
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
            HkShapeRefVER::Unknown(info) => HkShape::Unknown(info.conv()),
        }
    }
}

#[make_platforms]
#[enum_dispatch(DumpHkShapeVER)]
pub enum HkShapeVER<'a> {
    Ref(HkShapeRefVER<'a>),
    Owned(HkShape)
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
        inds: &'a mut [u16VER],
    },
    None
}

#[make_platforms]
impl<'a> HkShapeDumpVER<'a> {
    fn from_bytes(dst: &mut DumpSlice<'a>, sizes: (usize, usize, usize), info: &mut HkShapeInfoVER) -> Result<Self> {
        Ok(match info.kind.get() {
            5 => {
                let info: &mut ConvexVerticesInfoVER = zerocopy::transmute_mut!(info);

                dst.align(16)?;
                info.norms_offset = dst.offset.conv();
                let norms = Vector4VER::mut_slice_from_data(dst, sizes.0).context("norms")?;
                info.norm_num = norms.len().conv();

                info.verts_offset = dst.offset.conv();
                let verts = Vector3VER::mut_slice_from_data(dst, sizes.1).context("verts")?;
                info.vert_num = (verts.len() - sizes.2).conv();
                Self::ConvexVertices { norms, verts }
            }
            6 => {
                let info: &mut BVTreeMeshInfoVER = zerocopy::transmute_mut!(info);

                info.verts_offset = dst.offset.conv();
                let verts = Vector3VER::mut_slice_from_data(dst, sizes.0).context("verts")?;
                info.vert_num = verts.len().conv();

                info.inds_offset = dst.offset.conv();
                let inds = u16VER::mut_slice_from_data(dst, sizes.1).context("inds")?;
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

#[make_platforms]
#[enum_dispatch]
pub trait DumpHkShapeVER {
    fn kind(&self) -> u32;
    fn get_sizes(&self) -> (usize, usize, usize);
    fn write_info(&self, info: &mut HkShapeInfoVER) -> Result<()>;
    fn write_vals(&self, vals: HkShapeDumpVER) -> Result<()>;
    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        counts.hk_shapes += 1;
        match self.kind() {
            5 => {
                counts.offsets += 2;
                let sizes = self.get_sizes();
                offset = align_offset(offset, 16);
                offset += sizes.0 * Vector4VER::size_of();
                offset += sizes.1 * Vector3VER::size_of();
            }
            6 => {
                counts.offsets += 3;
                let sizes = self.get_sizes();
                offset += sizes.0 * Vector3VER::size_of();
                offset += sizes.1 * u16VER::size_of();
                offset = align_offset(offset, 4);
                offset += sizes.2;
                offset = align_offset(offset, 4);
            }
            _ => ()
        }
        offset
    }
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER) -> Result<()> {
        let info_off = infos.hk_shapes.offset;
        let info = infos.hk_shapes.next().context("hk_shapes")?;
        self.write_info(info).context("write info")?;
        let vals = HkShapeDumpVER::from_bytes(dst, self.get_sizes(), info).context("vals")?;
        self.write_vals(vals).context("write vals")?;
        match self.kind() {
            5 => {
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ConvexVerticesInfoVER, norms_offset)) as u32).conv();
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(ConvexVerticesInfoVER, verts_offset)) as u32).conv();
            }
            6 => {
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(BVTreeMeshInfoVER, verts_offset)) as u32).conv();
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(BVTreeMeshInfoVER, inds_offset)) as u32).conv();
                *infos.offsets.next().context("offsets")? = ((info_off + std::mem::offset_of!(BVTreeMeshInfoVER, tree_offset)) as u32).conv();

            }
            _ => ()
        }
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
            Self::ConvexVertices(val) => val.info.kind.get(),
            Self::BVTreeMesh(val) => val.info.kind.get(),
            Self::Unknown(info) => info.kind.get(),
        }
    }
    fn get_sizes(&self) -> (usize, usize, usize) {
        match self {
            Self::ConvexVertices(ConvexVerticesRefVER { info, verts, norms }) => (norms.len(), verts.len(), verts.len() - info.vert_num.get() as usize),
            Self::BVTreeMesh(BVTreeMeshRefVER { tree, verts, inds, .. }) => (verts.len(), inds.len(), tree.len()),
            _ => (0,0,0)
        }
    }
    fn write_info(&self, info: &mut HkShapeInfoVER) -> Result<()> {
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
    fn write_vals(&self, vals: HkShapeDumpVER) -> Result<()> {
        match (self, vals) {
            (Self::ConvexVertices(ConvexVerticesRefVER { verts: src1, norms: src2, .. }), HkShapeDumpVER::ConvexVertices { verts: dst1, norms: dst2 }) => {
                dst1.write_from(src1).context("verts")?;
                dst2.write_from(src2).context("norms")?;
            },
            (Self::BVTreeMesh(BVTreeMeshRefVER { verts: src1, inds: src2, tree: src3, .. }), HkShapeDumpVER::BVTreeMesh { verts: dst1, inds: dst2, tree: dst3 }) => {
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
impl DumpHkShapeVER for HkShape { 
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
            HkShape::Box(src) => *zerocopy::transmute_mut!(info) = OrderedData::<BoxShapeVER>::conv(src),
            HkShape::Sphere(src) => *zerocopy::transmute_mut!(info) = OrderedData::<SphereShapeVER>::conv(src),
            HkShape::Capsule(src) => *zerocopy::transmute_mut!(info) = OrderedData::<CapsuleShapeVER>::conv(src),
            HkShape::Cylinder(src) => *zerocopy::transmute_mut!(info) = OrderedData::<CylinderShapeVER>::conv(src),
            HkShape::ConvexVertices { info: src, .. } => *zerocopy::transmute_mut!(info) = OrderedData::<ConvexVerticesInfoVER>::conv(src),
            HkShape::BVTreeMesh { info: src, .. } => *zerocopy::transmute_mut!(info) = OrderedData::<BVTreeMeshInfoVER>::conv(src),
            HkShape::Unknown(src) => *info = src.conv(),
        }
        Ok(())
    }
    fn write_vals(&self, vals: HkShapeDumpVER) -> Result<()> {
        match (self, vals) {
            (HkShape::ConvexVertices { verts: src1, norms: src2, .. }, HkShapeDumpVER::ConvexVertices { verts: dst1, norms: dst2 }) => {
                for (src, dst) in src1.iter().zip(dst1) {
                    *dst = src.conv();
                }
                for (src, dst) in src2.iter().zip(dst2) {
                    *dst = src.conv();
                }
            },
            (HkShape::BVTreeMesh { verts: src1, inds: src2, tree: src3, .. }, HkShapeDumpVER::BVTreeMesh { verts: dst1, inds: dst2, tree: dst3 }) => {
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

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
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

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct TRS {
    pub translation: Vector4,
    pub rotation: Vector4,
    pub scale: Vector4,
}

///gen_ffi:export
#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct HkConstraintBoneRefVER<'a> {
    pub name: string<'a>,
    pub start: u32VER,
    pub val: u32VER
}

///gen_ffi:export
#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct HkConstraintRefVER<'a> {
    pub info: &'a HkConstraintInfoVER,
    pub bone_parents: ref_slice<'a, i16VER>,
    pub bone_names: slice<HkConstraintBoneRefVER<'a>>,
    pub name_offsets: ref_slice<'a, u32VER>,
    pub bone_transforms: ref_slice<'a, TRSVER>,
    pub bones: ref_slice<'a, u32VER>,
    pub bones_order: ref_slice<'a, Key2VER>,
    pub vals2: ref_slice<'a, f32VER>,
}

#[make_platforms]
impl Default for HkConstraintRefVER<'_> {
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

#[make_platforms]
impl<'a> HkConstraintRefVER<'a> {
    pub fn from_data(src: &'a[u8], offset: usize) -> Result<Self> {
        let info = HkConstraintInfoVER::from_data(&src[offset..]).context("info")?;
        if info.kind.get() != 0 {
            warn!("Unknown & Unhandled HkConstraint type {}", info.kind);
        }

        let bone_parents = i16VER::slice_from_data(
            &src[info.bone_parents_offset.get() as usize..],
            info.bone_parents_num.get() as usize,
        )
        .context("bone_parents")?;
        assert!(bone_parents[0].get() == -1, "first bone should be root node with no parent");

        let name_offsets = u32VER::slice_from_data(
            &src[info.bone_names_offset.get() as usize..],
            info.bone_names_num.get() as usize,
        )
        .context("name_offsets")?;
        let mut bone_names = Vec::with_capacity(name_offsets.len());
        for offset_ in name_offsets.iter() {
            let start = u32VER::from_data(&src[offset_.get() as usize..]).context("start")?;
            let val_ = u32VER::from_data(&src[offset_.get() as usize + 4..]).context("val_")?;
            let mut offset = start.get() as usize;
            while src[offset] != 0 {
                offset += 1;
            }
            bone_names.push(HkConstraintBoneRefVER {
                name: str::from_utf8(&src[start.get() as usize..offset]).context("string")?.into(),
                start: *start,
                val: *val_,
            });
        }
        let bone_transforms = TRSVER::slice_from_data(
            &src[info.bone_transforms_offset.get() as usize..],
            info.bone_transforms_num.get() as usize,
        )
        .context("bone_tranforms")?;
        let bones = u32VER::slice_from_data(
            &src[info.bones_offset.get() as usize..],
            info.bones_num.get() as usize,
        )
        .context("bones")?;
        let bones_order = Key2VER::slice_from_data(
            &src[info.bone_order_offset.get() as usize..],
            info.bone_order_num.get() as usize,
        )
        .context("bone_order")?;

        // TODO should probably figure out what this data is
        let vals2 = f32VER::slice_from_data(
            &src[info.vals2_offset.get() as usize..],
            info.vals2_num.get() as usize * 42, 
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

#[make_platforms]
impl From<&HkConstraintRefVER<'_>> for HkConstraint {
    fn from(val: &HkConstraintRefVER) -> Self {
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

#[make_platforms]
#[enum_dispatch(DumpHkConstraintVER)]
pub enum HkConstraintVER<'a> {
    Ref(HkConstraintRefVER<'a>),
    Owned(HkConstraint)
}

#[make_platforms]
pub trait DumpHkConstraintImplVER {
    fn bone_names_num(&self) -> usize;
    fn bone_transforms_num(&self) -> usize;
    fn bone_order_num(&self) -> usize;
    fn bone_parents_num(&self) -> usize;
    fn vals2_num(&self) -> usize;
    fn bone_names(&self) -> impl Iterator<Item=&str>;
    fn write_info(&self, info: &mut HkConstraintInfoVER) -> Result<()>;
    fn write_bone_transforms(&self, bone_transforms: &mut [TRSVER]) -> Result<()>;
    fn write_bone_order(&self, bone_order: &mut [Key2VER]) -> Result<()>;
    fn write_bone_parents(&self, bone_parents: &mut [i16VER]) -> Result<()>;
    fn write_bone_name_vals<'a>(&self, vals: impl Iterator<Item = &'a mut u32VER>);
    fn write_vals2(&self, vals2: &mut [f32VER]) -> Result<()>;
}

#[make_platforms]
#[enum_dispatch]
pub trait DumpHkConstraintVER {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER, bones_num: u16, bones_offset: u32) -> Result<()>;
    fn add_size(&self, offset: usize, infos: &mut InfoCounts) -> usize;
}
#[make_platforms]
impl<T: DumpHkConstraintImplVER> DumpHkConstraintVER for T {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER, bones_num: u16, bones_offset: u32) -> Result<()> {
        let info_off = infos.hk_constraints.offset;
        let info = infos.hk_constraints.next().context("hk_constraints")?;
        self.write_info(info).context("write info")?;
        info.bones_num = bones_num.conv();
        info.bones_offset = bones_offset.conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfoVER, bones_offset)).conv();

        info.bone_names_offset = dst.offset.conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfoVER, bone_names_offset)).conv();
        let mut name_off = dst.offset;
        let name_offsets = u32VER::mut_slice_from_data(dst, self.bone_names_num()).context("name_offsets")?;
        let mut string_off = dst.offset;
        let string_offs = u32VER::mut_slice_from_data(dst, name_offsets.len()*2).context("string_offs")?;
        info.bone_names_num = name_offsets.len().conv();

        dst.align(16)?;
        info.bone_transforms_offset = dst.offset.conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfoVER, bone_transforms_offset)).conv();
        let bone_transforms = TRSVER::mut_slice_from_data(dst, self.bone_transforms_num()).context("bone_transforms")?;
        info.bone_transforms_num = bone_transforms.len().conv();
        self.write_bone_transforms(bone_transforms).context("write bone_transforms")?;

        info.bone_order_offset = dst.offset.conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfoVER, bone_order_offset)).conv();
        let bone_order = Key2VER::mut_slice_from_data(dst, self.bone_order_num()).context("bone_order")?;
        info.bone_order_num = bone_order.len().conv();
        self.write_bone_order(bone_order).context("write bone_order")?;

        info.bone_parents_offset = dst.offset.conv();
        *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfoVER, bone_parents_offset)).conv();
        let bone_parents = i16VER::mut_slice_from_data(dst, self.bone_parents_num()).context("bone_parents")?;
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
            name_off += size_of::<u32VER>();
            string_off += size_of::<u32VER>() * 2;
        }

        info.vals2_offset = dst.offset.conv();
        let vals2 = f32VER::mut_slice_from_data(dst, self.vals2_num()).context("vals2")?;
        self.write_vals2(vals2).context("write vals2")?;
        if vals2.len() == 0 {
            info.vals2_offset = 0u32.conv();
        } else {
            *infos.offsets.next().context("offsets")? = (info_off + std::mem::offset_of!(HkConstraintInfoVER, vals2_offset)).conv();
        }
        Ok(())
    }
    fn add_size(&self, mut offset: usize, infos: &mut InfoCounts) -> usize {
        infos.hk_constraints += 1;
        infos.offsets += self.bone_names_num() * 2 + 5;
        if self.vals2_num() != 0 {
            infos.offsets += 1;
        }
        offset += size_of::<u32VER>() * self.bone_names_num() * 3;
        offset = align_offset(offset, 16) + size_of::<TRSVER>() * self.bone_transforms_num();
        offset += size_of::<Key2VER>() * self.bone_order_num();
        offset += size_of::<i16VER>() * self.bone_parents_num();
        offset = align_offset(offset, 4);

        for string in self.bone_names() {
            let s = string.as_bytes(); 
            offset += s.len() + 1;
            offset = align_offset(offset, 4);
        }

        offset += size_of::<f32VER>() * self.vals2_num();
        offset
    }
}

#[make_platforms]
impl DumpHkConstraintImplVER for HkConstraintRefVER<'_> {
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
    fn write_info(&self, info: &mut HkConstraintInfoVER) -> Result<()> {
        info.write_from(self.info)
    }
    fn write_bone_transforms(&self, bone_transforms: &mut [TRSVER]) -> Result<()> {
        bone_transforms.write_from(&self.bone_transforms[..])
    }
    fn write_bone_order(&self, bone_order: &mut [Key2VER]) -> Result<()> {
        bone_order.write_from(&self.bones_order[..])
    }
    fn write_bone_parents(&self, bone_parents: &mut [i16VER]) -> Result<()> {
        bone_parents.write_from(&self.bone_parents[..])
    }
    fn write_bone_name_vals<'a>(&self, vals: impl Iterator<Item = &'a mut u32VER>) {
        for (src, dst) in self.bone_names.iter().map(|x| x.val).zip(vals) {
            *dst = src;
        }
    }
    fn write_vals2(&self, vals2: &mut [f32VER]) -> Result<()> {
        vals2.write_from(&self.vals2[..])
    }
}

#[make_platforms]
impl DumpHkConstraintImplVER for HkConstraint {
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
    fn write_info(&self, info: &mut HkConstraintInfoVER) -> Result<()> {
        *info = (&self.info).conv();
        Ok(())
    }
    fn write_bone_transforms(&self, bone_transforms: &mut [TRSVER]) -> Result<()> {
        for (src, dst) in self.bone_transforms.iter().zip(bone_transforms) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_bone_order(&self, bone_order: &mut [Key2VER]) -> Result<()> {
        let mut new_bone_order = self.bone_names.iter().enumerate().map(|(i,val)| Key2 { key: hash_string(val.as_ref(), None).into(), val: i as u32 }).collect::<Vec<_>>();
        new_bone_order.sort_by_key(|x| x.key.get());
        for (src, dst) in new_bone_order.iter().zip(bone_order) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_bone_parents(&self, bone_parents: &mut [i16VER]) -> Result<()> {
        for (src, dst) in self.bone_parents.iter().zip(bone_parents) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_bone_name_vals<'a>(&self, vals: impl Iterator<Item = &'a mut u32VER>) {
        for val in vals {
            *val = 0u32.conv();
        }
    }
    fn write_vals2(&self, vals2: &mut [f32VER]) -> Result<()> {
        for (src, dst) in self.vals2.iter().zip(vals2) {
            *dst = src.conv();
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
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
