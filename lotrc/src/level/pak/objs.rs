use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use itertools::Itertools;
use std::ptr::NonNull;
use std::collections::BinaryHeap;

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
            ModelInfoVER, ModelVER, ModelRawVER, ModelRefVER, DumpModelVER 
        },
        pak::{
            animation::{AnimationBlockInfoVER, AnimationInfoVER},
            PakHeaderVER,
        },
        bin::{BinVER, BinRefVER},
        radiosity::{RadiosityValsInfoVER, RadiosityValsVER, RadiosityVER, RadiosityRawVER, RadiosityRefVER, DumpRadiosityVER},
        texture::{TextureInfoVER, TextureVER, TextureRawVER, TextureRefVER, DumpTextureVER},
    },
    sub_blocks::{
        gameobjs::{GameObjsVER, GameObjsRefVER, DumpGameObjsVER},
        SubBlocksVER, SubBlocksRefVER
    },
    types::{u32VER, CrcVER, i32VER, f32VER, i16VER, u16VER, Vector4VER},
};
#[cfg(not(feature = "ffi"))]
use crate::types::GetNative;
use crate::{
    level::{
        model::{
            shape::{HkConstraint, HkShape, Shape},
            Model,
        },
        pak::{animation::AnimationBlockInfo, PakHeader},
        radiosity::{RadiosityValsInfo, Radiosity},
        texture::Texture,
    },
    sub_blocks::gameobjs::{GameObjs, TypeInfos},
    types::{Crc, DumpData, DumpSlice, RefFromData, Vector4, hash_string, OrderedData, OrderedDataStrict, BufType, slice, Map, MapImpl, box_slice, get_str, align_offset, CompressedDataRefAlt, DumpCompressedData},
};
use lotrc_proc::{make_platforms, OrderedData};

#[derive(Debug, Default, Clone, OrderedData)]
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
pub struct Obj0 {
    #[ordered_data(PC)]
    pub unk_0: u32,
    #[ordered_data(PC)]
    pub key: Crc,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct EffectInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct PFieldInfo {
    pub link_guid: u32,
    pub gamemode_guid: u32,
    pub width: u32,
    pub height: u32,
    pub offset: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct GFXBlockInfo {
    // GFX blocks?, unchanged by encoding, model as data
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
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
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct FoliageRefVER<'a> {
    info: &'a FoliageInfoVER,
    vals: slice<'a, FoliageValVER>
}

#[make_platforms]
impl<'a> FoliageRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a FoliageInfoVER) -> Result<Self> {
        let n =
            ((info.ub_w.get() - info.lb_w.get()) * (info.ub_h.get() - info.lb_h.get())) as usize;
        let vals = FoliageValVER::slice_from_data(&src[info.offset.get() as usize..], n)
            .context("vals")?;
        Ok(Self {
            info: info,
            vals: vals.into(),
        })
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct FoliageVER {
    _ptr: BufType,
    info: NonNull<FoliageInfoVER>,
    vals: NonNull<[FoliageValVER]>,
}

#[make_platforms]
unsafe impl Sync for FoliageVER {}
#[make_platforms]
unsafe impl Send for FoliageVER {}

#[make_platforms]
impl FoliageVER {
    pub fn from_bytes(src: &BufType, offset: usize) -> Result<Self> {
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
impl FoliageVER {
    pub fn info(&self) -> &FoliageInfoVER {
        unsafe { self.info.as_ref() }
    }
    pub fn vals(&self) -> &[FoliageValVER] {
        unsafe { self.vals.as_ref() }
    }
}

#[derive(Debug, Clone)]
pub struct Foliage {
    pub info: FoliageInfo,
    pub vals: Vec<FoliageVal>,
}

#[make_platforms]
impl From<&FoliageVER> for Foliage {
    fn from(val: &FoliageVER) -> Self {
        Self {
            info: val.info().conv(),
            vals: val.vals().iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_platforms]
pub trait DumpFoliageVER {
    fn vals_num(&self) -> usize;
    fn write_vals(&self, vals: &mut [FoliageValVER]) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice, info: &mut FoliageInfoVER) -> Result<()> {
        info.offset = dst.offset.conv();
        let vals = FoliageValVER::mut_slice_from_data(dst, self.vals_num()).context("vals")?;
        self.write_vals(vals).context("write vals")
    }
    fn add_size(&self, offset: usize) -> usize {
        offset + self.vals_num() * std::mem::size_of::<FoliageValVER>()
    }
}

#[make_platforms]
impl DumpFoliageVER for FoliageRefVER<'_> {
    fn vals_num(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [FoliageValVER]) -> Result<()> {
        vals.write_from(&self.vals[..])
    }
}

pub enum Patch<R,P> {
    Raw(R),
    Parsed(P)
}

#[make_platforms]
pub struct ObjsPatchVER {
    _ptr: BufType,
    pub objas: Patch<NonNull<[ObjAVER]>, Vec<ObjA>>,
    pub obj0s: Patch<NonNull<[Obj0VER]>, Vec<Obj0>>,
    pub models: IndexMap<u32, Patch<ModelRawVER, Model>>,
    pub textures: IndexMap<u32, Patch<TextureRawVER, Texture>>,
    pub effects: IndexMap<u32, Patch<GameObjsVER, GameObjs>>,
    pub gfxs: IndexMap<u32, Patch<NonNull<[u8]>, Vec<u8>>>,
    pub foliages: IndexMap<u32, Patch<FoliageVER, Foliage>>,
    pub radiosity: Patch<RadiosityRawVER, Radiosity>,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct ObjsRefVER<'a> {
    pub objas: slice<'a, ObjAVER>,
    pub obj0s: slice<'a, Obj0VER>,
    pub model_infos: slice<'a, ModelInfoVER>,
    pub buffer_infos: slice<'a, BufferInfoVER>,
    pub mat1s: slice<'a, Mat1VER>,
    pub mat2s: slice<'a, Mat2VER>,
    pub mat3s: slice<'a, Mat3VER>,
    pub mat4s: slice<'a, Mat4VER>,
    pub mat_extras: slice<'a, MatExtraVER>,
    pub shape_infos: slice<'a, ShapeInfoVER>,
    pub hk_shape_infos: slice<'a, HkShapeInfoVER>,
    pub hk_constraint_datas: slice<'a, HkConstraintDataVER>,
    pub vbuff_infos: slice<'a, VBuffInfoVER>,
    pub ibuff_infos: slice<'a, IBuffInfoVER>,
    pub texture_infos: slice<'a, TextureInfoVER>,
    pub animation_infos: slice<'a, AnimationInfoVER>,
    pub hk_constraint_infos: slice<'a, HkConstraintInfoVER>,
    pub effect_infos: slice<'a, EffectInfoVER>,
    pub pfield_infos: slice<'a, PFieldInfoVER>,
    pub gfx_block_infos: slice<'a, GFXBlockInfoVER>,
    pub animation_block_infos: slice<'a, AnimationBlockInfoVER>,
    pub foliage_infos: slice<'a, FoliageInfoVER>,
    pub radiosity_vals_infos: slice<'a, RadiosityValsInfoVER>,

    pub textures: Map<u32, TextureRefVER<'a>>,
    pub models: Map<u32, ModelRefVER<'a>>,
    pub effects: Map<u32, GameObjsRefVER<'a>>,
    pub foliages: Map<u32, box_slice<FoliageRefVER<'a>>>,
    pub gfxs: Map<u32, slice<'a, u8>>,
    pub radiosity: RadiosityRefVER<'a>,
}

#[make_platforms]
impl Default for ObjsRefVER<'_> {
    fn default() -> Self {
        Self {
            objas: slice::default(),
            obj0s: slice::default(),
            model_infos: slice::default(),
            buffer_infos: slice::default(),
            mat1s: slice::default(),
            mat2s: slice::default(),
            mat3s: slice::default(),
            mat4s: slice::default(),
            mat_extras: slice::default(),
            shape_infos: slice::default(),
            hk_shape_infos: slice::default(),
            hk_constraint_datas: slice::default(),
            vbuff_infos: slice::default(),
            ibuff_infos: slice::default(),
            texture_infos: slice::default(),
            animation_infos: slice::default(),
            hk_constraint_infos: slice::default(),
            effect_infos: slice::default(),
            pfield_infos: slice::default(),
            gfx_block_infos: slice::default(),
            animation_block_infos: slice::default(),
            foliage_infos: slice::default(),
            radiosity_vals_infos: slice::default(),

            textures: MapImpl::default().into(),
            models: MapImpl::default().into(),
            effects: MapImpl::default().into(),
            foliages: MapImpl::default().into(),
            gfxs: MapImpl::default().into(),
            radiosity: RadiosityRefVER::default(),
        }
    }
}

#[make_platforms]
impl<'a> ObjsRefVER<'a> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeaderVER, bin: &BinRefVER<'a>, name: u32) -> Result<Self> {
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
        let model_infos = ModelInfoVER::slice_from_data(
            &src[pak_header.model_info_offset.get() as usize..],
            pak_header.model_info_num.get() as usize,
        )
        .context("model_infos")?;
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
        let shape_infos = ShapeInfoVER::slice_from_data(
            &src[pak_header.shape_info_offset.get() as usize..],
            pak_header.shape_info_num.get() as usize,
        )
        .context("shape_infos")?;
        let hk_shape_infos = HkShapeInfoVER::slice_from_data(
            &src[pak_header.hk_shape_info_offset.get() as usize..],
            pak_header.hk_shape_info_num.get() as usize,
        )
        .context("hk_shape_infos")?;
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
        let hk_constraint_infos = HkConstraintInfoVER::slice_from_data(
            &src[pak_header.hk_constraint_info_offset.get() as usize..],
            pak_header.hk_constraint_info_num.get() as usize,
        )
        .context("hk_constraint_infos")?;
        let effect_infos = EffectInfoVER::slice_from_data(
            &src[pak_header.effect_info_offset.get() as usize..],
            pak_header.effect_info_num.get() as usize,
        )
        .context("effect_infos")?;
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
        let animation_block_infos = AnimationBlockInfoVER::slice_from_data(
            &src[pak_header.animation_block_info_offset.get() as usize..],
            pak_header.animation_block_info_num.get() as usize,
        )
        .context("animation_block_infos")?;
        let foliage_infos = FoliageInfoVER::slice_from_data(
            &src[pak_header.foliage_info_offset.get() as usize..],
            pak_header.foliage_info_num.get() as usize,
        )
        .context("foliage_infos")?;
        let radiosity_vals_infos = RadiosityValsInfoVER::slice_from_data(
            &src[pak_header.radiosity_vals_info_offset.get() as usize..],
            pak_header.radiosity_vals_info_num.get() as usize,
        )
        .context("radiosity_vals_infos")?;

        let texture_data = &bin.texture_data;
        let textures: MapImpl<_, _> = texture_infos.iter().enumerate()
            .map(|(i, info)| Ok((
                info.key.get(),
                TextureRefVER::from_data(info, texture_data).with_context(|| format!("texture {}", i))?
            )))
            .collect::<Result<_>>()?;
        let model_data = &bin.model_data;
        let models: MapImpl<_, _> = model_infos.iter().enumerate()
            .map(|(i, info)| Ok((
                info.key.get(),
                ModelRefVER::from_data(src, info, model_data).with_context(|| format!("model {}", i))?
            )))
            .collect::<Result<_>>()?;
        let effects: MapImpl<_, _> = effect_infos
            .iter()
            .map(|info| Ok((
                info.key.get(),
                GameObjsRefVER::from_data(
                    &src[info.offset.get() as usize..info.offset.get() as usize + info.size.get() as usize],
                    info.gamemodemask.get(),
                )?
            )))
            .collect::<Result<_>>()?;
        let gfxs: MapImpl<_, _> = gfx_block_infos
            .iter()
            .map(|info| (
                info.key.get(),
                (&src[info.offset.get() as usize..(info.offset.get() + info.size.get()) as usize]).into(),
            ))
            .collect();
        let mut foliages = IndexMap::<u32, Vec<FoliageRefVER>>::with_capacity(foliage_infos.len());
        for (i, info) in foliage_infos.iter().enumerate() {
            let foliage = FoliageRefVER::from_data(src, info).with_context(|| format!("foliage {}", i))?;
            foliages.entry(foliage.info.key.get()).or_default().push(foliage);
        }
        let foliages: MapImpl<_, _> = foliages.into_iter().map(|(k, v)| (k, v.into_boxed_slice().into())).collect();
        let radiosity_name = hash_string(b"_radiosity", Some(name));
        let ind = model_data.get_index_of(&radiosity_name);
        let data = ind.and_then(|x| model_data.get_index(x)).map(|(_, x)| x).copied();
        let usage =  ind.map(|x| bin.model_handles()[x].kind.get()).unwrap_or_default();
        
        let radiosity = RadiosityRefVER::from_data(src, radiosity_vals_infos, data, usage).context("radiosity")?; 
        Ok(Self {
            objas: objas.into(),
            obj0s: obj0s.into(),
            model_infos: model_infos.into(),
            buffer_infos: buffer_infos.into(),
            mat1s: mat1s.into(),
            mat2s: mat2s.into(),
            mat3s: mat3s.into(),
            mat4s: mat4s.into(),
            mat_extras: mat_extras.into(),
            shape_infos: shape_infos.into(),
            hk_shape_infos: hk_shape_infos.into(),
            hk_constraint_datas: hk_constraint_datas.into(),
            vbuff_infos: vbuff_infos.into(),
            ibuff_infos: ibuff_infos.into(),
            texture_infos: texture_infos.into(),
            animation_infos: animation_infos.into(),
            hk_constraint_infos: hk_constraint_infos.into(),
            effect_infos: effect_infos.into(),
            pfield_infos: pfield_infos.into(),
            gfx_block_infos: gfx_block_infos.into(),
            animation_block_infos: animation_block_infos.into(),
            foliage_infos: foliage_infos.into(),
            radiosity_vals_infos: radiosity_vals_infos.into(),

            textures: textures.into(),
            models: models.into(),
            effects: effects.into(),
            gfxs: gfxs.into(),
            foliages: foliages.into(),
            radiosity,
        })
        
    }
}

#[make_platforms]
pub struct ObjsRawVER {
    _ptr: BufType,
    objas: NonNull<[ObjAVER]>,
    obj0s: NonNull<[Obj0VER]>,
    model_infos: NonNull<[ModelInfoVER]>,
    buffer_infos: NonNull<[BufferInfoVER]>,
    mat1s: NonNull<[Mat1VER]>,
    mat2s: NonNull<[Mat2VER]>,
    mat3s: NonNull<[Mat3VER]>,
    mat4s: NonNull<[Mat4VER]>,
    mat_extras: NonNull<[MatExtraVER]>,
    shape_infos: NonNull<[ShapeInfoVER]>,
    hk_shape_infos: NonNull<[HkShapeInfoVER]>,
    hk_constraint_datas: NonNull<[HkConstraintDataVER]>,
    vbuff_infos: NonNull<[VBuffInfoVER]>,
    ibuff_infos: NonNull<[IBuffInfoVER]>,
    texture_infos: NonNull<[TextureInfoVER]>,
    animation_infos: NonNull<[AnimationInfoVER]>,
    hk_constraint_infos: NonNull<[HkConstraintInfoVER]>,
    effect_infos: NonNull<[EffectInfoVER]>,
    pfield_infos: NonNull<[PFieldInfoVER]>,
    gfx_block_infos: NonNull<[GFXBlockInfoVER]>,
    animation_block_infos: NonNull<[AnimationBlockInfoVER]>,
    foliage_infos: NonNull<[FoliageInfoVER]>,
    radiosity_vals_infos: NonNull<[RadiosityValsInfoVER]>,

    textures: IndexMap<u32, TextureRawVER>,
    models: IndexMap<u32, ModelRawVER>,
    effects: IndexMap<u32, GameObjsVER>,
    foliages: IndexMap<u32, Box<[FoliageVER]>>,
    gfxs: IndexMap<u32, NonNull<[u8]>>,
    radiosity: RadiosityRawVER,
}

#[make_platforms]
impl ObjsRawVER {
    pub fn from_bytes(src: &BufType, bin: &BinVER, sub_blocks: &SubBlocksVER, pak_header: &PakHeaderVER) -> Result<Self> {
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
        let model_infos = ModelInfoVER::slice_from_data(
            &src[pak_header.model_info_offset.get() as usize..],
            pak_header.model_info_num.get() as usize,
        )
        .context("model_infos")?;
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
        let shape_infos = ShapeInfoVER::slice_from_data(
            &src[pak_header.shape_info_offset.get() as usize..],
            pak_header.shape_info_num.get() as usize,
        )
        .context("shape_infos")?;
        let hk_shape_infos = HkShapeInfoVER::slice_from_data(
            &src[pak_header.hk_shape_info_offset.get() as usize..],
            pak_header.hk_shape_info_num.get() as usize,
        )
        .context("hk_shape_infos")?;
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
        let hk_constraint_infos = HkConstraintInfoVER::slice_from_data(
            &src[pak_header.hk_constraint_info_offset.get() as usize..],
            pak_header.hk_constraint_info_num.get() as usize,
        )
        .context("hk_constraint_infos")?;
        let effect_infos = EffectInfoVER::slice_from_data(
            &src[pak_header.effect_info_offset.get() as usize..],
            pak_header.effect_info_num.get() as usize,
        )
        .context("effect_infos")?;
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
        let animation_block_infos = AnimationBlockInfoVER::slice_from_data(
            &src[pak_header.animation_block_info_offset.get() as usize..],
            pak_header.animation_block_info_num.get() as usize,
        )
        .context("animation_block_infos")?;
        let foliage_infos = FoliageInfoVER::slice_from_data(
            &src[pak_header.foliage_info_offset.get() as usize..],
            pak_header.foliage_info_num.get() as usize,
        )
        .context("foliage_infos")?;
        let radiosity_vals_infos = RadiosityValsInfoVER::slice_from_data(
            &src[pak_header.radiosity_vals_info_offset.get() as usize..],
            pak_header.radiosity_vals_info_num.get() as usize,
        )
        .context("radiosity_vals_infos")?;

        let texture_data = bin.texture_data();
        let texture_off = pak_header.texture_info_offset.get() as usize;
        let textures = (0..pak_header.texture_info_num.get() as usize)
            .map(|i| TextureRawVER::from_bytes(src, texture_data, texture_off + i * TextureInfoVER::size_of())
                .map(|x| (x.info().key.get(), x))
                .with_context(|| format!("texture {}", i))
            )
            .collect::<Result<_>>()?;
        let model_data = bin.model_data();
        let model_off = pak_header.model_info_offset.get() as usize;
        let models = (0..pak_header.model_info_num.get() as usize)
            .map(|i| ModelRawVER::from_bytes(src, model_data, model_off + i * ModelInfoVER::size_of())
                .map(|x| (x.info().key.get(), x))
                .with_context(|| format!("model {}", i))
            )
            .collect::<Result<_>>()?;
        let effects = effect_infos
            .iter()
            .map(|info| Ok((
                info.key.get(),
                GameObjsVER::from_bytes(
                    src,
                    info.offset.get() as usize,
                    info.size.get() as usize,
                    info.gamemodemask.get(),
                )?
            )))
            .collect::<Result<_>>()?;
        let gfxs = gfx_block_infos
            .iter()
            .map(|info| (
                info.key.get(),
                NonNull::from_ref(
                    &src[info.offset.get() as usize
                        ..(info.offset.get() + info.size.get()) as usize],
                )
            ))
            .collect();
        let mut foliages = IndexMap::<u32, Vec<FoliageVER>>::new();
        let foliage_off = pak_header.foliage_info_offset.get() as usize;
        for i in 0..foliage_infos.len() {
            let foliage = FoliageVER::from_bytes(src, foliage_off).with_context(|| format!("foliage {}", i))?;
            foliages.entry(foliage.info().key.get()).or_default().push(foliage);
        }
        let foliages = foliages.into_iter().map(|(k, v)| (k, v.into())).collect();
        let level = sub_blocks.get(&hash_string(b"level", None)).ok_or(anyhow!("missing level sub_block"))?.level().ok_or(anyhow!("incorrect level type"))?;
        let radiosity = RadiosityRawVER::from_bytes(src, bin, level, pak_header).context("radiosity")?; 
        Ok(Self {
            _ptr: src.clone(),
            objas: objas.into(),
            obj0s: obj0s.into(),
            model_infos: model_infos.into(),
            buffer_infos: buffer_infos.into(),
            mat1s: mat1s.into(),
            mat2s: mat2s.into(),
            mat3s: mat3s.into(),
            mat4s: mat4s.into(),
            mat_extras: mat_extras.into(),
            shape_infos: shape_infos.into(),
            hk_shape_infos: hk_shape_infos.into(),
            hk_constraint_datas: hk_constraint_datas.into(),
            vbuff_infos: vbuff_infos.into(),
            ibuff_infos: ibuff_infos.into(),
            texture_infos: texture_infos.into(),
            animation_infos: animation_infos.into(),
            hk_constraint_infos: hk_constraint_infos.into(),
            effect_infos: effect_infos.into(),
            pfield_infos: pfield_infos.into(),
            gfx_block_infos: gfx_block_infos.into(),
            animation_block_infos: animation_block_infos.into(),
            foliage_infos: foliage_infos.into(),
            radiosity_vals_infos: radiosity_vals_infos.into(),

            textures,
            models,
            effects,
            gfxs,
            foliages,
            radiosity,
        })
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct ObjsVER {
    _ptr: BufType,
    objas: NonNull<[ObjAVER]>,
    obj0s: NonNull<[Obj0VER]>,
    model_infos: NonNull<[ModelInfoVER]>,
    buffer_infos: NonNull<[BufferInfoVER]>,
    mat1s: NonNull<[Mat1VER]>,
    mat2s: NonNull<[Mat2VER]>,
    mat3s: NonNull<[Mat3VER]>,
    mat4s: NonNull<[Mat4VER]>,
    mat_extras: NonNull<[MatExtraVER]>,
    shape_infos: NonNull<[ShapeInfoVER]>,
    hk_shape_infos: NonNull<[HkShapeInfoVER]>,
    hk_constraint_datas: NonNull<[HkConstraintDataVER]>,
    vbuff_infos: NonNull<[VBuffInfoVER]>,
    ibuff_infos: NonNull<[IBuffInfoVER]>,
    texture_infos: NonNull<[TextureInfoVER]>,
    animation_infos: NonNull<[AnimationInfoVER]>,
    hk_constraint_infos: NonNull<[HkConstraintInfoVER]>,
    effect_infos: NonNull<[EffectInfoVER]>,
    pfield_infos: NonNull<[PFieldInfoVER]>,
    gfx_block_infos: NonNull<[GFXBlockInfoVER]>,
    animation_block_infos: NonNull<[AnimationBlockInfoVER]>,
    foliage_infos: NonNull<[FoliageInfoVER]>,
    radiosity_vals_infos: NonNull<[RadiosityValsInfoVER]>,

    textures: IndexMap<u32, TextureVER>,
    models: IndexMap<u32, ModelVER>,
    shapes: Box<[ShapeVER]>,
    hk_shapes: Box<[HkShapeVER]>,
    hk_constraints: Box<[HkConstraintVER]>,
    effects: IndexMap<u32, GameObjsVER>,
    gfxs: IndexMap<u32, NonNull<[u8]>>,
    radiosity_vals: IndexMap<u32, RadiosityValsVER>,
    foliages: IndexMap<u32, Box<[FoliageVER]>>,
    radiosity: RadiosityVER,
}

#[make_platforms]
unsafe impl Sync for ObjsVER {}
#[make_platforms]
unsafe impl Send for ObjsVER {}

#[make_platforms]
impl TryFrom<ObjsRawVER> for ObjsVER {
    type Error = anyhow::Error;
    fn try_from(ObjsRawVER {
        _ptr,
        objas,
        obj0s,
        model_infos,
        buffer_infos,
        mat1s,
        mat2s,
        mat3s,
        mat4s,
        mat_extras,
        shape_infos,
        hk_shape_infos,
        hk_constraint_datas,
        vbuff_infos,
        ibuff_infos,
        texture_infos,
        animation_infos,
        hk_constraint_infos,
        effect_infos,
        pfield_infos,
        gfx_block_infos,
        animation_block_infos,
        foliage_infos,
        radiosity_vals_infos,

        textures,
        models,
        effects,
        gfxs,
        foliages,
        radiosity,
    }: ObjsRawVER) -> Result<Self> {
        let shape_off = shape_infos.addr().get() - _ptr.as_ptr().addr();
        let shapes = (0..shape_infos.len())
            .map(|i| {
                ShapeVER::from_bytes(&_ptr, shape_off + i * ShapeInfoVER::size_of())
                    .with_context(|| format!("shape {}", i))
            })
            .collect::<Result<Vec<_>>>()?;
        let hk_shape_off = hk_shape_infos.addr().get() - _ptr.as_ptr().addr();
        let hk_shapes = (0..hk_shape_infos.len())
            .map(|i| {
                HkShapeVER::from_bytes(&_ptr, hk_shape_off + i * HkShapeInfoVER::size_of())
                    .with_context(|| format!("hk_shape {}", i))
            })
            .collect::<Result<Vec<_>>>()?;
        let hk_constraint_off = hk_constraint_infos.addr().get() - _ptr.as_ptr().addr();
        let hk_constraints = (0..hk_constraint_infos.len())
            .map(|i| {
                HkConstraintVER::from_bytes(
                    &_ptr,
                    hk_constraint_off + i * HkConstraintInfoVER::size_of(),
                )
                .with_context(|| format!("hk_constraint {}", i))
            })
            .collect::<Result<Vec<_>>>()?;
        let radiosity_val_off = radiosity_vals_infos.addr().get() - _ptr.as_ptr().addr();
        let radiosity_vals = (0..radiosity_vals_infos.len())
            .map(|i| {
                RadiosityValsVER::from_bytes(
                    &_ptr,
                    radiosity_val_off + i * RadiosityValsInfoVER::size_of(),
                )
                .map(|x| (x.info().guid.get(), x))
                .with_context(|| format!("radiosity_vals {}", i))
            })
            .collect::<Result<_>>()?;
        let textures = textures.into_iter().map(|(k,v)| Ok((k, v.try_into()?))).collect::<Result<_>>().context("textures")?;
        let models = models.into_iter().map(|(k,v)| Ok((k, v.try_into()?))).collect::<Result<_>>().context("models")?;
        let radiosity = radiosity.try_into().context("radiosity")?;

        Ok(Self {
            _ptr,
            objas,
            obj0s,
            model_infos,
            buffer_infos,
            mat1s,
            mat2s,
            mat3s,
            mat4s,
            mat_extras,
            shape_infos,
            hk_shape_infos,
            hk_constraint_datas,
            vbuff_infos,
            ibuff_infos,
            texture_infos,
            animation_infos,
            hk_constraint_infos,
            effect_infos,
            pfield_infos,
            gfx_block_infos,
            animation_block_infos,
            foliage_infos,
            radiosity_vals_infos,
            
            textures,
            models,
            shapes: shapes.into(),
            hk_shapes: hk_shapes.into(),
            hk_constraints: hk_constraints.into(),
            effects,
            gfxs,
            radiosity_vals,
            foliages,
            radiosity,
        })
    }
}

#[make_platforms]
impl ObjsVER {
    pub fn objas(&self) -> &[ObjAVER] {
        unsafe { self.objas.as_ref() }
    }
    pub fn obj0s(&self) -> &[Obj0VER] {
        unsafe { self.obj0s.as_ref() }
    }
    pub fn model_infos(&self) -> &[ModelInfoVER] {
        unsafe { self.model_infos.as_ref() }
    }
    pub fn buffer_infos(&self) -> &[BufferInfoVER] {
        unsafe { self.buffer_infos.as_ref() }
    }
    pub fn mat1s(&self) -> &[Mat1VER] {
        unsafe { self.mat1s.as_ref() }
    }
    pub fn mat2s(&self) -> &[Mat2VER] {
        unsafe { self.mat2s.as_ref() }
    }
    pub fn mat3s(&self) -> &[Mat3VER] {
        unsafe { self.mat3s.as_ref() }
    }
    pub fn mat4s(&self) -> &[Mat4VER] {
        unsafe { self.mat4s.as_ref() }
    }
    pub fn mat_extras(&self) -> &[MatExtraVER] {
        unsafe { self.mat_extras.as_ref() }
    }
    pub fn shape_infos(&self) -> &[ShapeInfoVER] {
        unsafe { self.shape_infos.as_ref() }
    }
    pub fn hk_shape_infos(&self) -> &[HkShapeInfoVER] {
        unsafe { self.hk_shape_infos.as_ref() }
    }
    pub fn hk_constraint_datas(&self) -> &[HkConstraintDataVER] {
        unsafe { self.hk_constraint_datas.as_ref() }
    }
    pub fn vbuff_infos(&self) -> &[VBuffInfoVER] {
        unsafe { self.vbuff_infos.as_ref() }
    }
    pub fn ibuff_infos(&self) -> &[IBuffInfoVER] {
        unsafe { self.ibuff_infos.as_ref() }
    }
    pub fn texture_infos(&self) -> &[TextureInfoVER] {
        unsafe { self.texture_infos.as_ref() }
    }
    pub fn animation_infos(&self) -> &[AnimationInfoVER] {
        unsafe { self.animation_infos.as_ref() }
    }
    pub fn hk_constraint_infos(&self) -> &[HkConstraintInfoVER] {
        unsafe { self.hk_constraint_infos.as_ref() }
    }
    pub fn effect_infos(&self) -> &[EffectInfoVER] {
        unsafe { self.effect_infos.as_ref() }
    }
    pub fn pfield_infos(&self) -> &[PFieldInfoVER] {
        unsafe { self.pfield_infos.as_ref() }
    }
    pub fn gfx_block_infos(&self) -> &[GFXBlockInfoVER] {
        unsafe { self.gfx_block_infos.as_ref() }
    }
    pub fn animation_block_infos(&self) -> &[AnimationBlockInfoVER] {
        unsafe { self.animation_block_infos.as_ref() }
    }
    pub fn foliage_infos(&self) -> &[FoliageInfoVER] {
        unsafe { self.foliage_infos.as_ref() }
    }
    pub fn radiosity_vals_infos(&self) -> &[RadiosityValsInfoVER] {
        unsafe { self.radiosity_vals_infos.as_ref() }
    }
    pub fn textures(&self) -> &IndexMap<u32, TextureVER> {
        &self.textures
    }
    pub fn models(&self) -> &IndexMap<u32, ModelVER> {
        &self.models
    }
    pub fn hk_constraints(&self) -> &[HkConstraintVER] {
        &self.hk_constraints
    }
    pub fn effects(&self) -> &IndexMap<u32, GameObjsVER> {
        &self.effects
    }
    pub fn gfxs(&self) -> &IndexMap<u32, NonNull<[u8]>> {
        &self.gfxs
    }
    pub fn radiosity_vals(&self) -> &IndexMap<u32, RadiosityValsVER> {
        &self.radiosity_vals
    }
    pub fn foliages(&self) -> &IndexMap<u32, Box<[FoliageVER]>> {
        &self.foliages
    }
    pub fn shapes(&self) -> &[ShapeVER] {
        &self.shapes
    }
    pub fn hk_shapes(&self) -> Vec<HkShapeRefVER<'_>> {
        self.hk_shapes
            .iter()
            .map(|x| unsafe { x.as_ref() })
            .collect()
    }
    pub fn radiosity(&self) -> &RadiosityVER {
        &self.radiosity
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
    pub animation_blocks: usize,
    pub radiosity_vals: usize,
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
    pub fn len(&self) -> usize {
        self.vals.len()
    }
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
    pub fn next_slice(&mut self, num: usize) -> &'a mut [T] {
        let Self {
            vals,
            mut ind,
            mut offset,
        } = std::mem::take(self);
        let (val, vals) = vals.split_at_mut(num);
        ind += 1;
        offset += val.size();
        *self = Self { vals, ind, offset };
        val
    }
}

#[make_platforms]
pub struct DumpInfosVER<'a, D> {
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
    pub animation_blocks: DumpInfo<'a, AnimationBlockInfoVER>,
    pub radiosity_vals: DumpInfo<'a, RadiosityValsInfoVER>,
    pub offsets: DumpInfo<'a, u32VER>,
    pub model_data: Vec<(u32VER, u32VER, Option<D>)>,
    pub texture_data: Vec<(u32VER, u32VER, Option<D>)>,
}

struct Key<T>(u32, T);

impl<T> Ord for Key<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> PartialOrd for Key<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Key<T> {}

#[make_platforms]
pub trait DumpObjsVER {
    type Data;
    fn obja_num(&self) -> usize;
    fn obj0_num(&self) -> usize;
    fn effect_num(&self) -> usize;
    fn foliage_num(&self) -> usize;
    // TODO proper pfield dumping
    fn pfield_num(&self) -> usize;
    fn gfx_num(&self) -> usize;
    fn write_objas(&self, objas: &mut [ObjAVER]) -> Result<()>;
    fn write_obj0s(&self, obj0s: &mut [Obj0VER]) -> Result<()>;
    fn effects<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpGameObjsVER + 'a))>;
    fn models<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpModelVER<Data=Self::Data> + 'a))>;
    fn foliages<'a>(&'a self) -> impl Iterator<Item=&'a (impl DumpFoliageVER + 'a)>;
    fn gfxs<'a>(&'a self) -> impl Iterator<Item=(u32, &'a [u8])>;
    fn radiosity(&self) -> &impl DumpRadiosityVER<Data=Self::Data>;
    fn textures<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpTextureVER<Data=Self::Data> + 'a))>;

    fn group_models<'a>(&'a self) -> (impl Iterator<Item=&'a (impl DumpModelVER<Data=Self::Data> + 'a)>, impl ExactSizeIterator<Item=&'a (impl DumpModelVER<Data=Self::Data> + 'a)>, Option<&'a (impl DumpModelVER<Data=Self::Data> + 'a)>) {
        let mut normal = BinaryHeap::new();
        let mut collision_road = BinaryHeap::new();
        let mut terrain = BinaryHeap::new();
        let mut occluder = None;
        for (key, model) in self.models() {
            if key == hash_string(b"occluder", None) {
                occluder.replace(model);
            } else if let Some(name) = get_str(&key) {
                if name.starts_with("Terrain") {
                    terrain.push(std::cmp::Reverse(Key(name.split("_").last().and_then(|x| x.parse::<u32>().ok()).unwrap_or_default(), model)));
                } else if name.contains("_Road_") || name.contains("_Collision_") {
                    collision_road.push(std::cmp::Reverse(Key(name.split("_").last().and_then(|x| x.parse::<u32>().ok()).unwrap_or_default(), model)));
                } else {
                    normal.push(std::cmp::Reverse(Key(key, model)));
            }
            } else {
                normal.push(std::cmp::Reverse(Key(key, model)));
            }
        }
        (
            normal.into_iter().chain(collision_road).map(|x| x.0.1),
            terrain.into_iter().map(|x| x.0.1),
            occluder
        )
    }

    fn dump_into<'a>(&self, dst: &mut DumpSlice<'a>, offsets: &'a mut [u32VER], counts: &InfoCounts, pak_header: &mut PakHeaderVER, type_infos: &[TypeInfos]) -> Result<DumpInfosVER<'a, Self::Data>> {
        dst.align(16);
        pak_header.obja_offset = dst.offset.conv();
        let objas = ObjAVER::mut_slice_from_data(dst, self.obja_num()).context("objas")?;
        pak_header.obja_num = objas.len().conv();
        self.write_objas(objas).context("write objas")?;
        
        dst.align(16);
        pak_header.obj0_offset = dst.offset.conv();
        let obj0s = Obj0VER::mut_slice_from_data(dst, self.obj0_num()).context("obj0s")?;
        pak_header.obj0_num = obj0s.len().conv();
        self.write_obj0s(obj0s).context("write obj0s")?;

        dst.align(16);
        pak_header.model_info_offset = dst.offset.conv();
        pak_header.model_info_num = counts.models.conv();
        let models = DumpInfo::<ModelInfoVER>::ref_from(dst, counts.models).context("model_infos")?;

        dst.align(16);
        pak_header.buffer_info_offset = dst.offset.conv();
        pak_header.buffer_info_num = counts.buffers.conv();
        let buffers = DumpInfo::<BufferInfoVER>::ref_from(dst, counts.buffers).context("buffer_infos")?;

        dst.align(16);
        pak_header.mat1_offset = dst.offset.conv();
        pak_header.mat1_num = counts.mat1s.conv();
        let mat1s = DumpInfo::<Mat1VER>::ref_from(dst, counts.mat1s).context("mat1s")?;

        dst.align(16);
        pak_header.mat2_offset = dst.offset.conv();
        pak_header.mat2_num = counts.mat2s.conv();
        let mat2s = DumpInfo::<Mat2VER>::ref_from(dst, counts.mat2s).context("mat2s")?;

        dst.align(16);
        pak_header.mat3_offset = dst.offset.conv();
        pak_header.mat3_num = counts.mat3s.conv();
        let mat3s = DumpInfo::<Mat3VER>::ref_from(dst, counts.mat3s).context("mat3s")?;

        dst.align(16);
        pak_header.mat4_offset = dst.offset.conv();
        pak_header.mat4_num = counts.mat4s.conv();
        let mat4s = DumpInfo::<Mat4VER>::ref_from(dst, counts.mat4s).context("mat4s")?;

        dst.align(16);
        pak_header.mat_extra_offset = dst.offset.conv();
        pak_header.mat_extra_num = counts.mat_extras.conv();
        let mat_extras = DumpInfo::<MatExtraVER>::ref_from(dst, counts.mat_extras).context("mat_extras")?;

        dst.align(16);
        pak_header.shape_info_offset = dst.offset.conv();
        pak_header.shape_info_num = counts.shapes.conv();
        let shapes = DumpInfo::<ShapeInfoVER>::ref_from(dst, counts.shapes).context("shape_infos")?;

        dst.align(16);
        pak_header.hk_shape_info_offset = dst.offset.conv();
        pak_header.hk_shape_info_num = counts.hk_shapes.conv();
        let hk_shapes = DumpInfo::<HkShapeInfoVER>::ref_from(dst, counts.hk_shapes).context("hk_shape_infos")?;

        dst.align(16);
        pak_header.hk_constraint_data_offset = dst.offset.conv();
        pak_header.hk_constraint_data_num = counts.hk_constraint_datas.conv();
        let hk_constraint_datas = DumpInfo::<HkConstraintDataVER>::ref_from(dst, counts.hk_constraint_datas).context("hk_constraint_datas")?;

        dst.align(16);
        pak_header.vbuff_info_offset = dst.offset.conv();
        pak_header.vbuff_info_num = counts.vbuffs.conv();
        let vbuffs = DumpInfo::<VBuffInfoVER>::ref_from(dst, counts.vbuffs).context("vbuff_infos")?;

        dst.align(16);
        pak_header.ibuff_info_offset = dst.offset.conv();
        pak_header.ibuff_info_num = counts.ibuffs.conv();
        let ibuffs = DumpInfo::<IBuffInfoVER>::ref_from(dst, counts.ibuffs).context("ibuff_infos")?;

        dst.align(16);
        pak_header.texture_info_offset = dst.offset.conv();
        pak_header.texture_info_num = counts.textures.conv();
        let textures = DumpInfo::<TextureInfoVER>::ref_from(dst, counts.textures).context("texture_infos")?;

        dst.align(16);
        pak_header.animation_info_offset = dst.offset.conv();
        pak_header.animation_info_num = counts.animations.conv();
        let animations = DumpInfo::<AnimationInfoVER>::ref_from(dst, counts.animations).context("animation_infos")?;

        dst.align(16);
        pak_header.hk_constraint_info_offset = dst.offset.conv();
        pak_header.hk_constraint_info_num = counts.hk_constraints.conv();
        let hk_constraints = DumpInfo::<HkConstraintInfoVER>::ref_from(dst, counts.hk_constraints).context("hk_constraint_infos")?;

        dst.align(16);
        let mut effect_info_offset = dst.offset;
        pak_header.effect_info_offset = effect_info_offset.conv();
        let effect_infos = EffectInfoVER::mut_slice_from_data(dst, self.effect_num()).context("effect_infos")?;
        pak_header.effect_info_num = effect_infos.len().conv();

        dst.align(16);
        let mut foliage_info_offset = dst.offset;
        pak_header.foliage_info_offset = foliage_info_offset.conv();
        let foliage_infos = FoliageInfoVER::mut_slice_from_data(dst, self.foliage_num()).context("foliage_infos")?;
        pak_header.foliage_info_num = foliage_infos.len().conv();

        dst.align(16);
        pak_header.pfield_info_offset = dst.offset.conv();
        let pfield_infos = PFieldInfoVER::mut_slice_from_data(dst, self.pfield_num()).context("pfield_infos")?;
        pak_header.pfield_info_num = pfield_infos.len().conv();

        dst.align(16);
        let mut gfx_block_info_offset = dst.offset;
        pak_header.gfx_block_info_offset = gfx_block_info_offset.conv();
        let gfx_block_infos = GFXBlockInfoVER::mut_slice_from_data(dst, self.gfx_num()).context("gfx_block_infos")?;
        pak_header.gfx_block_info_num = gfx_block_infos.len().conv();

        dst.align(16);
        let radiosity = self.radiosity();
        pak_header.radiosity_vals_info_offset = dst.offset.conv();
        pak_header.radiosity_vals_info_num = counts.radiosity_vals.conv();
        let radiosity_vals = DumpInfo::<RadiosityValsInfoVER>::ref_from(dst, counts.radiosity_vals).context("radiosity_vals_infos")?;

        dst.align(16);
        pak_header.animation_block_info_offset = dst.offset.conv();
        pak_header.animation_block_info_num = counts.animation_blocks.conv();
        let animation_blocks = DumpInfo::<AnimationBlockInfoVER>::ref_from(dst, counts.animation_blocks).context("animation_block_infos")?;

        let texture_data = Vec::with_capacity(textures.len() * 2);
        let model_data = Vec::with_capacity(models.len() + 1);

        let mut dump_infos = DumpInfosVER {
            models,
            buffers,
            mat1s,
            mat2s,
            mat3s,
            mat4s,
            mat_extras,
            shapes,
            hk_shapes,
            hk_constraint_datas,
            vbuffs,
            ibuffs,
            textures,
            animations,
            hk_constraints,
            animation_blocks,
            radiosity_vals,
            texture_data,
            model_data,
            offsets: DumpInfo { vals: offsets, ind: 0, offset: 0 },
        };

        dst.align(16);
        for (((key, effect), info), type_infos) in self.effects().sorted_by_key(|x| x.0).zip(effect_infos).zip(type_infos) {
            info.key = key.conv();
            info.gamemodemask = effect.gamemodemask().conv();
            let off = dst.offset;
            effect.dump_into(dst, type_infos).with_context(|| format!("effect {}", key))?;
            info.size = (dst.offset - off).conv();
            info.offset = off.conv();
            *dump_infos.offsets.next() = (effect_info_offset + std::mem::offset_of!(EffectInfoVER, offset)).conv();
            effect_info_offset += std::mem::size_of::<EffectInfoVER>();
        }

        let (normal, terrain, occluder) = self.group_models();
        let mut model_count = 0;
        for model in normal {
            // TODO (better context)
            model.dump_into(dst, &mut dump_infos).with_context(|| format!("model({}) {}", model_count, model.key()))?;
            dst.align(16);
            model_count += 1;
        }
        let terrain_offset = dst.offset;
        if terrain.len() != 0 {
            *<[u8; 16]>::mut_from_data(dst).context("terrain indices")? = [0xFFu8; 16];
        }
        for model in terrain {
            // TODO (better context)
            model.dump_terrain_into(dst, &mut dump_infos, terrain_offset).with_context(|| format!("terrain model({}) {}", model_count, model.key()))?; 
            model_count += 1;
        }

        dst.align(16);
        for (foliage, info) in self.foliages().zip(foliage_infos) {
            foliage.dump_into(dst, info).with_context(|| format!("foliage {}", info.key))?;
            dst.align(16);
            *dump_infos.offsets.next() = (foliage_info_offset + std::mem::offset_of!(FoliageInfoVER, offset)).conv();
            foliage_info_offset += std::mem::size_of::<FoliageInfoVER>();
        }

        if let Some(model) = occluder {
            model.dump_into(dst, &mut dump_infos).context("model occluder")?;
            dst.align(16);
        }

        for ((key, gfx), info) in self.gfxs().zip(gfx_block_infos) {
            info.key = key.conv();
            info.offset = dst.offset.conv();
            gfx.dump_into(dst).with_context(|| format!("gfx {}", key))?;
            dst.align(16);
            *dump_infos.offsets.next() = (gfx_block_info_offset + std::mem::offset_of!(GFXBlockInfoVER, offset)).conv();
            gfx_block_info_offset += std::mem::size_of::<GFXBlockInfoVER>();
        }

        radiosity.dump_into(dst, &mut dump_infos).context("radiosity")?;
        dst.align(16);


        // populate offsets

        Ok(dump_infos)
    }

    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> (usize, Vec<TypeInfos>) {
        let mut objs_size = 0;

        let mut type_infos = self.effects().sorted_by_key(|x| x.0).map(|(_, effect)| {
            let (size, type_infos) = effect.size();
            objs_size += size;
            type_infos
        }).collect();

        let (normal, terrain, occluder) = self.group_models();

        for model in normal {
            objs_size = align_offset(model.add_size(objs_size, counts), 16);
        }
        if terrain.len() != 0 {
            objs_size += 16;
        }
        for model in terrain {
            objs_size = model.add_terrain_size(objs_size, counts); 
        }

        objs_size = align_offset(objs_size, 16);
        for foliage in self.foliages() {
            objs_size += align_offset(foliage.add_size(objs_size), 16);
        }

        if let Some(model) = occluder {
            objs_size = align_offset(model.add_size(objs_size, counts), 16);
        }

        for (_, gfx) in self.gfxs() {
            objs_size = align_offset(objs_size + gfx.len(), 16);
        }

        objs_size = align_offset(self.radiosity().add_size(objs_size, counts), 16);

        offset = align_offset(offset, 16);
        offset = align_offset(offset + self.obja_num() * std::mem::size_of::<ObjAVER>(), 16);
        offset = align_offset(offset + self.obja_num() * std::mem::size_of::<ObjAVER>(), 16);
        offset = align_offset(offset + self.obj0_num() * std::mem::size_of::<Obj0VER>(), 16);
        offset = align_offset(offset + counts.models * std::mem::size_of::<DumpInfo::<ModelInfoVER>>(), 16);
        offset = align_offset(offset + counts.buffers * std::mem::size_of::<DumpInfo::<BufferInfoVER>>(), 16);
        offset = align_offset(offset + counts.mat1s * std::mem::size_of::<DumpInfo::<Mat1VER>>(), 16);
        offset = align_offset(offset + counts.mat2s * std::mem::size_of::<DumpInfo::<Mat2VER>>(), 16);
        offset = align_offset(offset + counts.mat3s * std::mem::size_of::<DumpInfo::<Mat3VER>>(), 16);
        offset = align_offset(offset + counts.mat4s * std::mem::size_of::<DumpInfo::<Mat4VER>>(), 16);
        offset = align_offset(offset + counts.mat_extras * std::mem::size_of::<DumpInfo::<MatExtraVER>>(), 16);
        offset = align_offset(offset + counts.shapes * std::mem::size_of::<DumpInfo::<ShapeInfoVER>>(), 16);
        offset = align_offset(offset + counts.hk_shapes * std::mem::size_of::<DumpInfo::<HkShapeInfoVER>>(), 16);
        offset = align_offset(offset + counts.hk_constraint_datas * std::mem::size_of::<DumpInfo::<HkConstraintDataVER>>(), 16);
        offset = align_offset(offset + counts.vbuffs * std::mem::size_of::<DumpInfo::<VBuffInfoVER>>(), 16);
        offset = align_offset(offset + counts.ibuffs * std::mem::size_of::<DumpInfo::<IBuffInfoVER>>(), 16);
        offset = align_offset(offset + counts.textures * std::mem::size_of::<DumpInfo::<TextureInfoVER>>(), 16);
        offset = align_offset(offset + counts.animations * std::mem::size_of::<DumpInfo::<AnimationInfoVER>>(), 16);
        offset = align_offset(offset + counts.hk_constraints * std::mem::size_of::<DumpInfo::<HkConstraintInfoVER>>(), 16);
        offset = align_offset(offset + self.effect_num() * std::mem::size_of::<EffectInfoVER>(), 16);
        offset = align_offset(offset + self.foliage_num() * std::mem::size_of::<FoliageInfoVER>(), 16);
        offset = align_offset(offset + self.pfield_num() * std::mem::size_of::<PFieldInfoVER>(), 16);
        offset = align_offset(offset + self.gfx_num() * std::mem::size_of::<GFXBlockInfoVER>(), 16);
        offset = align_offset(offset + counts.radiosity_vals * std::mem::size_of::<RadiosityValsInfoVER>(), 16);
        offset = align_offset(offset + counts.animation_blocks * std::mem::size_of::<DumpInfo::<AnimationBlockInfoVER>>(), 16);
        counts.offsets += self.effect_num() + self.gfx_num() + self.foliage_num();
        (offset + objs_size, type_infos)
    }
}

#[make_platforms]
impl<'a> DumpObjsVER for ObjsRefVER<'a> {
    type Data = &'a CompressedDataRefAlt<'a>;
    fn obja_num(&self) -> usize {
        self.objas.len()
    }
    fn obj0_num(&self) -> usize {
        self.obj0s.len()
    }
    fn effect_num(&self) -> usize {
        self.effects.len()
    }
    fn foliage_num(&self) -> usize {
        self.foliages.iter().map(|(_, x)| x.len()).sum::<usize>()
    }
    fn pfield_num(&self) -> usize {
        self.pfield_infos.len()
    }
    fn gfx_num(&self) -> usize {
        self.gfxs.len()
    }
    fn write_objas(&self, objas: &mut [ObjAVER]) -> Result<()> {
        objas.write_from(&self.objas[..])
    }
    fn write_obj0s(&self, obj0s: &mut [Obj0VER]) -> Result<()> {
        obj0s.write_from(&self.obj0s[..])
    }
    fn effects(&self) -> impl Iterator<Item=(u32, &impl DumpGameObjsVER)> {
        self.effects.iter().map(|(k,v)| (*k, v))
    }
    fn models(&self) -> impl Iterator<Item=(u32, &impl DumpModelVER<Data=Self::Data>)> {
        self.models.iter().map(|(k,v)| (*k, v))
    }
    fn textures(&self) -> impl Iterator<Item=(u32, &impl DumpTextureVER<Data=Self::Data>)> {
        self.textures.iter().map(|(k, v)| (*k, v))
    }
    fn foliages(&self) -> impl Iterator<Item=&impl DumpFoliageVER> {
        self.foliages.iter().flat_map(|(_, x)| x.iter())
    }
    fn gfxs(&self) -> impl Iterator<Item=(u32, &[u8])> {
        self.gfxs.iter().map(|(k,v)| (*k, &v[..]))
    }
    fn radiosity(&self) -> &impl DumpRadiosityVER<Data=Self::Data> {
        &self.radiosity
    }
}
