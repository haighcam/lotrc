#[cfg(feature = "python")]
use crate::pyobj_ref;
use anyhow::{anyhow, Context, Result};
#[cfg(not(feature = "python"))]
use lotrc_proc::getter;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::ptr::NonNull;
use std::sync::Arc;
use indexmap::IndexMap;

#[make_platforms]
use crate::{
    level::{
        model::{
            data::{BufferInfoVER, IBuffInfoVER, VBuffInfoVER},
            mat::{Mat1VER, Mat2VER, Mat3VER, Mat4VER, MatExtraVER},
            shape::{
                HkConstraintDataVER, HkConstraintInfoVER, HkConstraintVER, HkShapeInfoVER,
                HkShapeRefVER, HkShapeVER, ShapeInfoVER, ShapeVER,
            },
            ModelInfoVER, ModelVER,
        },
        pak::{
            animation::{AnimationBlockInfoVER, AnimationInfoVER},
            PakHeaderVER,
        },
        bin::BinVER,
        radiosity::{RadiosityValsInfoVER, RadiosityValsVER, RadiosityVER},
        texture::{TextureInfoVER, TextureVER},
    },
    sub_blocks::{
        gameobjs::GameObjsVER,
        SubBlocksVER,
    },
    types::U32VER,
};
use crate::{
    level::{
        model::{
            shape::{HkConstraint, HkShape, Shape},
            Model,
        },
        pak::{animation::AnimationBlockInfo, PakHeader},
        radiosity::RadiosityValsInfo,
    },
    sub_blocks::gameobjs::GameObjs,
    types::{Crc, DumpData, DumpSlice, RefFromData, Vector4, hash_string},
};
use lotrc_proc::{make_platforms, OrderedData};

#[cfg(feature = "python")]
pub fn init(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "objs")?;
    m.add_class::<EffectInfo>()?;
    m.add_class::<Foliage>()?;
    m.add_class::<FoliageInfo>()?;
    m.add_class::<FoliageVal>()?;
    m.add_class::<GFXBlockInfo>()?;
    m.add_class::<Obj0>()?;
    m.add_class::<ObjA>()?;
    m.add_class::<PFieldInfo>()?;
    init_pc(&m)?;
    init_xbox(&m)?;
    init_ps3(&m)?;
    Ok(m)
}
#[cfg(feature = "python")]
#[make_platforms]
pub fn init_ver(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FoliageVER>()
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs", get_all, set_all))]
pub struct ObjA {
    #[ordered_data(PC)]
    pub key: Crc,
    #[ordered_data(PC)]
    pub unk_1: u32,
    #[ordered_data(PC)]
    pub size: u32,
    #[ordered_data(PC)]
    pub size_comp: u32,
    #[ordered_data(PC)]
    pub unk_4: u32,
    #[ordered_data(PC)]
    pub kind: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs", get_all, set_all))]
pub struct Obj0 {
    #[ordered_data(PC)]
    pub unk_0: u32,
    #[ordered_data(PC)]
    pub key: Crc,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs", get_all, set_all))]
pub struct EffectInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs", get_all, set_all))]
pub struct PFieldInfo {
    pub link_guid: u32,
    pub gamemode_guid: u32,
    pub width: u32,
    pub height: u32,
    pub offset: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs", get_all, set_all))]
pub struct GFXBlockInfo {
    // GFX blocks?, unchanged by encoding, model as data
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs", get_all, set_all))]
pub struct FoliageInfo {
    pub key: Crc,
    pub kind: u32,
    pub lb_w: i32,
    pub lb_h: i32,
    pub ub_w: i32,
    pub ub_h: i32,
    pub scale: f32,
    pub offset: u32,
    pub key_mesh: Crc,
    pub key_mesh_lod1: Crc,
    pub key_mesh_lod2: Crc,
    pub color: Vector4,
    pub lod1a: f32,
    pub lod1b: f32,
    pub lod2a: f32,
    pub lod2b: f32,
    pub lod_max: f32,
}

#[derive(Debug, Clone, Default, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs", get_all, set_all))]
#[repr(C)]
// height: u16, var_mask: u16, slope_x: i16, slope_z: i16
// position and orientation are randomized for each instance (within own square)
// var mask only checks high component vs alpha of vertex attr (if it exists)
pub struct FoliageVal {
    pub height: u16,
    pub var_mask: u16,
    pub slope_x: i16,
    pub slope_z: i16,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs"))]
#[derive(Debug, Clone)]
pub struct FoliageVER {
    _ptr: Arc<[u8]>,
    info: NonNull<FoliageInfoVER>,
    vals: NonNull<[FoliageValVER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(FoliageVER);

#[make_platforms]
unsafe impl Sync for FoliageVER {}
#[make_platforms]
unsafe impl Send for FoliageVER {}

#[make_platforms]
impl FoliageVER {
    pub fn from_bytes(src: &Arc<[u8]>, offset: usize) -> Result<Self> {
        let info = FoliageInfoVER::from_data(&src[offset..]).context("info")?;
        let n =
            ((info.ub_w.get() - info.lb_w.get()) * (info.ub_h.get() - info.lb_h.get())) as usize;
        let vals = FoliageValVER::slice_from_data(&src[info.offset.get() as usize..], n)
            .context("vals")?;
        Ok(Self {
            _ptr: src.clone(),
            info: info.into(),
            vals: vals.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl FoliageVER {
    #[getter]
    pub fn info(&self) -> &FoliageInfoVER {
        unsafe { self.info.as_ref() }
    }
    #[getter]
    pub fn vals(&self) -> &[FoliageValVER] {
        unsafe { self.vals.as_ref() }
    }
}

#[cfg_attr(feature = "python", pyclass(module = "pak.objs", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct Foliage {
    pub info: FoliageInfo,
    pub vals: Vec<FoliageVal>,
}

#[make_platforms]
impl From<&FoliageVER> for Foliage {
    fn from(val: &FoliageVER) -> Self {
        Self {
            info: val.info().into(),
            vals: val.vals().iter().map(|x| x.into()).collect(),
        }
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "pak.objs"))]
#[derive(Debug, Clone)]
pub struct ObjsVER {
    _ptr: Arc<[u8]>,
    objas: NonNull<[ObjAVER]>,
    obj0s: NonNull<[Obj0VER]>,
    buffer_infos: NonNull<[BufferInfoVER]>,
    mat1s: NonNull<[Mat1VER]>,
    mat2s: NonNull<[Mat2VER]>,
    mat3s: NonNull<[Mat3VER]>,
    mat4s: NonNull<[Mat4VER]>,
    mat_extras: NonNull<[MatExtraVER]>,
    hk_constraint_datas: NonNull<[HkConstraintDataVER]>,
    vbuff_infos: NonNull<[VBuffInfoVER]>,
    ibuff_infos: NonNull<[IBuffInfoVER]>,
    texture_infos: NonNull<[TextureInfoVER]>,
    animation_infos: NonNull<[AnimationInfoVER]>,
    effect_infos: NonNull<[EffectInfoVER]>,
    pfield_infos: NonNull<[PFieldInfoVER]>,
    gfx_block_infos: NonNull<[GFXBlockInfoVER]>,
    animation_block_infos: NonNull<[AnimationBlockInfoVER]>,

    textures: Box<[TextureVER]>,
    models: Box<[ModelVER]>,
    shapes: Box<[ShapeVER]>,
    hk_shapes: Box<[HkShapeVER]>,
    hk_constraints: Box<[HkConstraintVER]>,
    effects: Box<[GameObjsVER]>,
    gfx_blocks: Box<[NonNull<[u8]>]>,
    radiosity_vals: Box<[RadiosityValsVER]>,
    foliages: Box<[FoliageVER]>,
    radiosity: RadiosityVER,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(ObjsVER);

#[make_platforms]
unsafe impl Sync for ObjsVER {}
#[make_platforms]
unsafe impl Send for ObjsVER {}

#[make_platforms]
impl ObjsVER {
    pub fn from_bytes(src: &Arc<[u8]>, bin: &BinVER, sub_blocks: &SubBlocksVER, pak_header: &PakHeaderVER) -> Result<Self> {
        let objas = ObjAVER::slice_from_data(
            &src[pak_header.obja_offset.get() as usize..],
            pak_header.obja_num.get() as usize,
        )
        .context("objas")?;
        let obj0s = Obj0VER::slice_from_data(
            &src[pak_header.obj0_offset.get() as usize..],
            pak_header.obj0_num.get() as usize,
        )
        .context("obj0s")?;
        let model_data = bin.model_data().unwrap();
        let model_off = pak_header.model_info_offset.get() as usize;
        let models = (0..pak_header.model_info_num.get() as usize)
            .map(|i| ModelVER::from_bytes(src, model_data, model_off + i * ModelInfoVER::size_of())
                .with_context(|| format!("model {}", i))
            )
            .collect::<Result<Vec<_>>>()?;
        let buffer_infos = BufferInfoVER::slice_from_data(
            &src[pak_header.buffer_info_offset.get() as usize..],
            pak_header.buffer_info_num.get() as usize,
        )
        .context("buffer_infos")?;
        let mat1s = Mat1VER::slice_from_data(
            &src[pak_header.mat1_offset.get() as usize..],
            pak_header.mat1_num.get() as usize,
        )
        .context("mat1s")?;
        let mat2s = Mat2VER::slice_from_data(
            &src[pak_header.mat2_offset.get() as usize..],
            pak_header.mat2_num.get() as usize,
        )
        .context("mat2s")?;
        let mat3s = Mat3VER::slice_from_data(
            &src[pak_header.mat3_offset.get() as usize..],
            pak_header.mat3_num.get() as usize,
        )
        .context("mat3s")?;
        let mat4s = Mat4VER::slice_from_data(
            &src[pak_header.mat4_offset.get() as usize..],
            pak_header.mat4_num.get() as usize,
        )
        .context("mat4s")?;
        let mat_extras = MatExtraVER::slice_from_data(
            &src[pak_header.mat_extra_offset.get() as usize..],
            pak_header.mat_extra_num.get() as usize,
        )
        .context("mat_extras")?;
        let shape_off = pak_header.shape_info_offset.get() as usize;
        let shapes = (0..pak_header.shape_info_num.get() as usize)
            .map(|i| {
                ShapeVER::from_bytes(src, shape_off + i * ShapeInfoVER::size_of())
                    .with_context(|| format!("shape {}", i))
            })
            .collect::<Result<Vec<_>>>()?;
        let hk_shape_off = pak_header.hk_shape_info_offset.get() as usize;
        let hk_shapes = (0..pak_header.hk_shape_info_num.get() as usize)
            .map(|i| {
                HkShapeVER::from_bytes(src, hk_shape_off + i * HkShapeInfoVER::size_of())
                    .with_context(|| format!("hk_shape {}", i))
            })
            .collect::<Result<Vec<_>>>()?;
        let hk_constraint_off = pak_header.hk_constraint_info_offset.get() as usize;
        let hk_constraints = (0..pak_header.hk_constraint_info_num.get() as usize)
            .map(|i| {
                HkConstraintVER::from_bytes(
                    src,
                    hk_constraint_off + i * HkConstraintInfoVER::size_of(),
                )
                .with_context(|| format!("hk_constraint {}", i))
            })
            .collect::<Result<Vec<_>>>()?;
        let hk_constraint_datas = HkConstraintDataVER::slice_from_data(
            &src[pak_header.hk_constraint_data_offset.get() as usize..],
            pak_header.hk_constraint_data_num.get() as usize,
        )
        .context("hk_constraint_datas")?;
        let vbuff_infos = VBuffInfoVER::slice_from_data(
            &src[pak_header.vbuff_info_offset.get() as usize..],
            pak_header.vbuff_info_num.get() as usize,
        )
        .context("vbuff_infos")?;
        let ibuff_infos = IBuffInfoVER::slice_from_data(
            &src[pak_header.ibuff_info_offset.get() as usize..],
            pak_header.ibuff_info_num.get() as usize,
        )
        .context("ibuff_infos")?;
        let texture_infos = TextureInfoVER::slice_from_data(
            &src[pak_header.texture_info_offset.get() as usize..],
            pak_header.texture_info_num.get() as usize,
        )
        .context("texture_infos")?;
        let animation_infos = AnimationInfoVER::slice_from_data(
            &src[pak_header.animation_info_offset.get() as usize..],
            pak_header.animation_info_num.get() as usize,
        )
        .context("animation_infos")?;
        let effect_infos = EffectInfoVER::slice_from_data(
            &src[pak_header.effect_info_offset.get() as usize..],
            pak_header.effect_info_num.get() as usize,
        )
        .context("effect_infos")?;
        let foliage_off = pak_header.foliage_info_offset.get() as usize;
        let foliages = (0..pak_header.foliage_info_num.get() as usize)
            .map(|i| {
                FoliageVER::from_bytes(src, foliage_off + i * FoliageInfoVER::size_of())
                    .with_context(|| format!("foliage {}", i))
            })
            .collect::<Result<Vec<_>>>()?;
        let pfield_infos = PFieldInfoVER::slice_from_data(
            &src[pak_header.pfield_info_offset.get() as usize..],
            pak_header.pfield_info_num.get() as usize,
        )
        .context("pfield_infos")?;
        let gfx_block_infos = GFXBlockInfoVER::slice_from_data(
            &src[pak_header.gfx_block_info_offset.get() as usize..],
            pak_header.gfx_block_info_num.get() as usize,
        )
        .context("gfx_block_infos")?;
        let radiosity_val_off = pak_header.radiosity_vals_info_offset.get() as usize;
        let radiosity_vals = (0..pak_header.radiosity_vals_info_num.get() as usize)
            .map(|i| {
                RadiosityValsVER::from_bytes(
                    src,
                    radiosity_val_off + i * RadiosityValsInfoVER::size_of(),
                )
                .with_context(|| format!("radiosity_vals {}", i))
            })
            .collect::<Result<Vec<_>>>()?;
        let animation_block_infos = AnimationBlockInfoVER::slice_from_data(
            &src[pak_header.animation_block_info_offset.get() as usize..],
            pak_header.animation_block_info_num.get() as usize,
        )
        .context("animation_block_infos")?;

        let gfx_blocks = gfx_block_infos
            .iter()
            .map(|info| {
                NonNull::from_ref(
                    &src[info.offset.get() as usize
                        ..(info.offset.get() + info.size.get()) as usize],
                )
            })
            .collect::<Vec<_>>();
        let effects = effect_infos
            .iter()
            .map(|info| {
                GameObjsVER::from_bytes(
                    src,
                    info.offset.get() as usize,
                    info.size.get() as usize,
                    info.gamemodemask.get(),
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let level = sub_blocks.get(&U32VER::new(hash_string(b"level", None))).ok_or(anyhow!("missing level sub_block"))?.level().ok_or(anyhow!("incorrect level type"))?;
        let radiosity = RadiosityVER::from_bytes(src, bin, level, pak_header).context("radiosity")?; 
        let texture_data = bin.texture_data().unwrap();
        let texture_off = pak_header.texture_info_offset.get() as usize;
        let textures = (0..pak_header.texture_info_num.get() as usize)
            .map(|i| TextureVER::from_bytes(src, texture_data, model_off + i * TextureInfoVER::size_of())
                .with_context(|| format!("texture {}", i))
            )
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            _ptr: src.clone(),
            objas: objas.into(),
            obj0s: obj0s.into(),
            buffer_infos: buffer_infos.into(),
            mat1s: mat1s.into(),
            mat2s: mat2s.into(),
            mat3s: mat3s.into(),
            mat4s: mat4s.into(),
            mat_extras: mat_extras.into(),
            hk_constraint_datas: hk_constraint_datas.into(),
            vbuff_infos: vbuff_infos.into(),
            ibuff_infos: ibuff_infos.into(),
            texture_infos: texture_infos.into(),
            animation_infos: animation_infos.into(),
            effect_infos: effect_infos.into(),
            pfield_infos: pfield_infos.into(),
            gfx_block_infos: gfx_block_infos.into(),
            animation_block_infos: animation_block_infos.into(),

            textures: textures.into(),
            models: models.into(),
            shapes: shapes.into(),
            hk_shapes: hk_shapes.into(),
            hk_constraints: hk_constraints.into(),
            effects: effects.into(),
            gfx_blocks: gfx_blocks.into(),
            radiosity_vals: radiosity_vals.into(),
            foliages: foliages.into(),
            radiosity,
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl ObjsVER {
    #[getter]
    pub fn objas(&self) -> &[ObjAVER] {
        unsafe { self.objas.as_ref() }
    }
    #[getter]
    pub fn obj0s(&self) -> &[Obj0VER] {
        unsafe { self.obj0s.as_ref() }
    }
    #[getter]
    pub fn buffer_infos(&self) -> &[BufferInfoVER] {
        unsafe { self.buffer_infos.as_ref() }
    }
    #[getter]
    pub fn mat1s(&self) -> &[Mat1VER] {
        unsafe { self.mat1s.as_ref() }
    }
    #[getter]
    pub fn mat2s(&self) -> &[Mat2VER] {
        unsafe { self.mat2s.as_ref() }
    }
    #[getter]
    pub fn mat3s(&self) -> &[Mat3VER] {
        unsafe { self.mat3s.as_ref() }
    }
    #[getter]
    pub fn mat4s(&self) -> &[Mat4VER] {
        unsafe { self.mat4s.as_ref() }
    }
    #[getter]
    pub fn mat_extras(&self) -> &[MatExtraVER] {
        unsafe { self.mat_extras.as_ref() }
    }
    #[getter]
    pub fn hk_constraint_datas(&self) -> &[HkConstraintDataVER] {
        unsafe { self.hk_constraint_datas.as_ref() }
    }
    #[getter]
    pub fn vbuff_infos(&self) -> &[VBuffInfoVER] {
        unsafe { self.vbuff_infos.as_ref() }
    }
    #[getter]
    pub fn ibuff_infos(&self) -> &[IBuffInfoVER] {
        unsafe { self.ibuff_infos.as_ref() }
    }
    #[getter]
    pub fn texture_infos(&self) -> &[TextureInfoVER] {
        unsafe { self.texture_infos.as_ref() }
    }
    #[getter]
    pub fn animation_infos(&self) -> &[AnimationInfoVER] {
        unsafe { self.animation_infos.as_ref() }
    }
    #[getter]
    pub fn effect_infos(&self) -> &[EffectInfoVER] {
        unsafe { self.effect_infos.as_ref() }
    }
    #[getter]
    pub fn pfield_infos(&self) -> &[PFieldInfoVER] {
        unsafe { self.pfield_infos.as_ref() }
    }
    #[getter]
    pub fn gfx_block_infos(&self) -> &[GFXBlockInfoVER] {
        unsafe { self.gfx_block_infos.as_ref() }
    }
    #[getter]
    pub fn animation_block_infos(&self) -> &[AnimationBlockInfoVER] {
        unsafe { self.animation_block_infos.as_ref() }
    }
    #[getter]
    pub fn models(&self) -> &[ModelVER] {
        &self.models
    }
    #[getter]
    pub fn hk_constraints(&self) -> &[HkConstraintVER] {
        &self.hk_constraints
    }
    #[getter]
    pub fn effects(&self) -> &[GameObjsVER] {
        &self.effects
    }
    #[getter]
    pub fn gfx_blocks(&self) -> Vec<&[u8]> {
        self.gfx_blocks
            .iter()
            .map(|x| unsafe { x.as_ref() })
            .collect()
    }
    #[getter]
    pub fn radiosity_vals(&self) -> &[RadiosityValsVER] {
        &self.radiosity_vals
    }
    #[getter]
    pub fn foliages(&self) -> &[FoliageVER] {
        &self.foliages
    }
    #[getter]
    pub fn shapes(&self) -> &[ShapeVER] {
        &self.shapes
    }
    #[getter]
    pub fn hk_shapes(&self) -> Vec<HkShapeRefVER<'_>> {
        self.hk_shapes
            .iter()
            .map(|x| unsafe { x.as_ref() })
            .collect()
    }
}

pub struct Objs {
    pub objas: Vec<ObjA>,
    pub obj0s: Vec<Obj0>,
    pub effect_infos: Vec<EffectInfo>,
    pub pfield_infos: Vec<PFieldInfo>,
    pub gfx_block_infos: Vec<GFXBlockInfo>,
    pub radiosity_vals_infos: Vec<RadiosityValsInfo>,
    pub animation_block_infos: Vec<AnimationBlockInfo>,

    pub models: Vec<Model>,
    pub shapes: Vec<Shape>,
    pub hk_shapes: Vec<HkShape>,
    pub hk_constraints: Vec<HkConstraint>,
    pub effects: Vec<GameObjs>,
    pub gfx_blocks: Vec<Vec<u8>>,
    pub foliages: Vec<Foliage>,
}

#[derive(Default, Debug, Clone)]
pub struct InfoCounts {
    pub objas: usize,
    pub obj0s: usize,
    pub models: usize,
    pub buffers: usize,
    pub mat1s: usize,
    pub mat2s: usize,
    pub mat3s: usize,
    pub mat4s: usize,
    pub mat_extras: usize,
    pub shapes: usize,
    pub hk_shapes: usize,
    pub hk_constraint_datas: usize,
    pub vbuffs: usize,
    pub ibuffs: usize,
    pub textures: usize,
    pub animations: usize,
    pub hk_constraints: usize,
    pub effects: usize,
    pub foliages: usize,
    pub pfields: usize,
    pub gfx_blocks: usize,
    pub radiosity_vals: usize,
    pub animation_blocks: usize,
    pub offsets: usize,
}

pub struct DumpInfo<'a, T: RefFromData> {
    pub vals: &'a mut [T],
    pub ind: usize,
    pub offset: usize,
}

impl<'a, T: RefFromData> Default for DumpInfo<'a, T> {
    fn default() -> Self {
        Self {
            vals: &mut [],
            ind: 0,
            offset: 0,
        }
    }
}

impl<'a, T: RefFromData> DumpInfo<'a, T> {
    pub fn as_ref(&mut self) -> &mut [T] {
        self.vals
    }
    pub fn take(&mut self) -> &mut [T] {
        let val = std::mem::take(self);
        val.vals
    }
    pub fn ref_from(src: &mut DumpSlice<'a>, size: usize) -> Result<Self> {
        let offset = src.offset;
        let vals = T::mut_slice_from_data(src, size)?;
        let ind = 0;
        Ok(Self { vals, ind, offset })
    }
    pub fn next(&mut self) -> &'a mut T {
        let Self {
            vals,
            mut ind,
            mut offset,
        } = std::mem::take(self);
        let (val, vals) = vals.split_first_mut().unwrap();
        ind += 1;
        offset += val.size();
        *self = Self { vals, ind, offset };
        val
    }
}

#[make_platforms]
pub struct DumpInfosVER<'a> {
    pub objas: DumpInfo<'a, ObjAVER>,
    pub obj0s: DumpInfo<'a, Obj0VER>,
    pub models: DumpInfo<'a, ModelInfoVER>,
    pub buffers: DumpInfo<'a, BufferInfoVER>,
    pub mat1s: DumpInfo<'a, Mat1VER>,
    pub mat2s: DumpInfo<'a, Mat2VER>,
    pub mat3s: DumpInfo<'a, Mat3VER>,
    pub mat4s: DumpInfo<'a, Mat4VER>,
    pub mat_extras: DumpInfo<'a, MatExtraVER>,
    pub shapes: DumpInfo<'a, ShapeInfoVER>,
    pub hk_shapes: DumpInfo<'a, HkShapeInfoVER>,
    pub hk_constraint_datas: DumpInfo<'a, HkConstraintDataVER>,
    pub vbuffs: DumpInfo<'a, VBuffInfoVER>,
    pub ibuffs: DumpInfo<'a, IBuffInfoVER>,
    pub textures: DumpInfo<'a, TextureInfoVER>,
    pub animations: DumpInfo<'a, AnimationInfoVER>,
    pub hk_constraints: DumpInfo<'a, HkConstraintInfoVER>,
    pub effects: DumpInfo<'a, EffectInfoVER>,
    pub foliages: DumpInfo<'a, FoliageInfoVER>,
    pub pfields: DumpInfo<'a, PFieldInfoVER>,
    pub gfx_blocks: DumpInfo<'a, GFXBlockInfoVER>,
    pub radiosity_vals: DumpInfo<'a, RadiosityValsInfoVER>,
    pub animation_blocks: DumpInfo<'a, AnimationBlockInfoVER>,
    pub offsets: DumpInfo<'a, U32VER>,
}

#[make_platforms]
#[derive(Default, Debug, Clone)]
pub struct ObjInfosVER {
    pub header: PakHeader,
    pub objas: Vec<ObjAVER>,
    pub obj0s: Vec<Obj0VER>,
    pub model_infos: Vec<ModelInfoVER>,
    pub buffer_infos: Vec<BufferInfoVER>,
    pub mat1s: Vec<Mat1VER>,
    pub mat2s: Vec<Mat2VER>,
    pub mat3s: Vec<Mat3VER>,
    pub mat4s: Vec<Mat4VER>,
    pub mat_extras: Vec<MatExtraVER>,
    pub shape_infos: Vec<ShapeInfoVER>,
    pub hk_shape_infos: Vec<HkShapeInfoVER>,
    pub hk_constraint_datas: Vec<HkConstraintDataVER>,
    pub vbuff_infos: Vec<VBuffInfoVER>,
    pub ibuff_infos: Vec<IBuffInfoVER>,
    pub texture_infos: Vec<TextureInfoVER>,
    pub animation_infos: Vec<AnimationInfoVER>,
    pub hk_constraint_infos: Vec<HkConstraintInfoVER>,
    pub effect_infos: Vec<EffectInfoVER>,
    pub foliage_infos: Vec<FoliageInfoVER>,
    pub pfield_infos: Vec<PFieldInfoVER>,
    pub gfx_block_infos: Vec<GFXBlockInfoVER>,
    pub radiosity_vals_infos: Vec<RadiosityValsInfoVER>,
    pub animation_block_infos: Vec<AnimationBlockInfoVER>,
}
