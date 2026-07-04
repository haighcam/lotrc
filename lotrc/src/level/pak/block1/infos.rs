use log::debug;
use anyhow::{Context, Result};

use crate::types::GetNative;
use crate::types::{CompressedData, OrderedData, DumpSlice, ref_slice, align_offset, mut_slice, DumpData, RefFromData};


use lotrc_proc::{make_platforms};
#[make_platforms]
use crate::{
    level::{
        model::{
            data::{BufferInfoVER, IBuffInfoVER, VBuffInfoVER},
            mat::{Mat1VER, Mat2VER, Mat3VER, Mat4VER, MatExtraVER},
            shape::{
                HkConstraintDataVER, HkConstraintInfoVER, HkShapeInfoVER,
                ShapeInfoVER,
            },
            ModelInfoVER  
        },
        pak::{
            PakHeaderVER,
            animation::{AnimationBlockInfoVER, AnimationInfoVER},
            block1::{
                objs::{ObjAVER, Obj0VER, EffectInfoVER, PFieldInfoVER, GFXBlockInfoVER, FoliageInfoVER},
            },
        },
        radiosity::{RadiosityValsInfoVER},
        texture::{TextureInfoVER},
    },
    types::u32VER
};

#[make_platforms]
#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct InfosRefVER<'a> {
    pub objas: ref_slice<'a, ObjAVER>,
    pub obj0s: ref_slice<'a, Obj0VER>,
    pub models: ref_slice<'a, ModelInfoVER>,
    pub buffers: ref_slice<'a, BufferInfoVER>,
    pub mat1s: ref_slice<'a, Mat1VER>,
    pub mat2s: ref_slice<'a, Mat2VER>,
    pub mat3s: ref_slice<'a, Mat3VER>,
    pub mat4s: ref_slice<'a, Mat4VER>,
    pub mat_extras: ref_slice<'a, MatExtraVER>,
    pub shapes: ref_slice<'a, ShapeInfoVER>,
    pub hk_shapes: ref_slice<'a, HkShapeInfoVER>,
    pub hk_constraint_datas: ref_slice<'a, HkConstraintDataVER>,
    pub vbuffs: ref_slice<'a, VBuffInfoVER>,
    pub ibuffs: ref_slice<'a, IBuffInfoVER>,
    pub textures: ref_slice<'a, TextureInfoVER>,
    pub animations: ref_slice<'a, AnimationInfoVER>,
    pub hk_constraints: ref_slice<'a, HkConstraintInfoVER>,
    pub effects: ref_slice<'a, EffectInfoVER>,
    pub pfields: ref_slice<'a, PFieldInfoVER>,
    pub gfxs: ref_slice<'a, GFXBlockInfoVER>,
    pub animation_blocks: ref_slice<'a, AnimationBlockInfoVER>,
    pub foliages: ref_slice<'a, FoliageInfoVER>,
    pub radiosity_vals: ref_slice<'a, RadiosityValsInfoVER>,
}

#[make_platforms]
impl<'a> InfosRefVER<'a> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeaderVER) -> Result<Self> {
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
        let models = ModelInfoVER::slice_from_data(
            &src[pak_header.model_info_offset.get() as usize..],
            pak_header.model_info_num.get() as usize,
        )
        .context("models")?;
        let buffers = BufferInfoVER::slice_from_data(
            &src[pak_header.buffer_info_offset.get() as usize..],
            pak_header.buffer_info_num.get() as usize,
        )
        .context("buffers")?;
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
        let shapes = ShapeInfoVER::slice_from_data(
            &src[pak_header.shape_info_offset.get() as usize..],
            pak_header.shape_info_num.get() as usize,
        )
        .context("shapes")?;
        let hk_shapes = HkShapeInfoVER::slice_from_data(
            &src[pak_header.hk_shape_info_offset.get() as usize..],
            pak_header.hk_shape_info_num.get() as usize,
        )
        .context("hk_shapes")?;
        let hk_constraint_datas = HkConstraintDataVER::slice_from_data(
            &src[pak_header.hk_constraint_data_offset.get() as usize..],
            pak_header.hk_constraint_data_num.get() as usize,
        )
        .context("hk_constraint_datas")?;
        let vbuffs = VBuffInfoVER::slice_from_data(
            &src[pak_header.vbuff_info_offset.get() as usize..],
            pak_header.vbuff_info_num.get() as usize,
        )
        .context("vbuffs")?;
        let ibuffs = IBuffInfoVER::slice_from_data(
            &src[pak_header.ibuff_info_offset.get() as usize..],
            pak_header.ibuff_info_num.get() as usize,
        )
        .context("ibuffs")?;
        let textures = TextureInfoVER::slice_from_data(
            &src[pak_header.texture_info_offset.get() as usize..],
            pak_header.texture_info_num.get() as usize,
        )
        .context("textures")?;
        let animations = AnimationInfoVER::slice_from_data(
            &src[pak_header.animation_info_offset.get() as usize..],
            pak_header.animation_info_num.get() as usize,
        )
        .context("animations")?;
        let hk_constraints = HkConstraintInfoVER::slice_from_data(
            &src[pak_header.hk_constraint_info_offset.get() as usize..],
            pak_header.hk_constraint_info_num.get() as usize,
        )
        .context("hk_constraints")?;
        let effects = EffectInfoVER::slice_from_data(
            &src[pak_header.effect_info_offset.get() as usize..],
            pak_header.effect_info_num.get() as usize,
        )
        .context("effects")?;
        let pfields = PFieldInfoVER::slice_from_data(
            &src[pak_header.pfield_info_offset.get() as usize..],
            pak_header.pfield_info_num.get() as usize,
        )
        .context("pfields")?;
        let gfxs = GFXBlockInfoVER::slice_from_data(
            &src[pak_header.gfx_block_info_offset.get() as usize..],
            pak_header.gfx_block_info_num.get() as usize,
        )
        .context("gfxs")?;
        let animation_blocks = AnimationBlockInfoVER::slice_from_data(
            &src[pak_header.animation_block_info_offset.get() as usize..],
            pak_header.animation_block_info_num.get() as usize,
        )
        .context("animation_blocks")?;
        debug!("animation block infos {:#?}", animation_blocks);
        let foliages = FoliageInfoVER::slice_from_data(
            &src[pak_header.foliage_info_offset.get() as usize..],
            pak_header.foliage_info_num.get() as usize,
        )
        .context("foliages")?;
        let radiosity_vals = RadiosityValsInfoVER::slice_from_data(
            &src[pak_header.radiosity_vals_info_offset.get() as usize..],
            pak_header.radiosity_vals_info_num.get() as usize,
        )
        .context("radiosity_vals")?;
        Ok(Self {
            objas,
            obj0s,
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
            effects,
            pfields,
            gfxs,
            animation_blocks,
            foliages,
            radiosity_vals,
        })
    }
}

#[make_platforms]
pub trait DumpExtraInfosVER {
    fn obja_num(&self) -> usize;
    fn obj0_num(&self) -> usize;
    // TODO proper pfield dumping
    fn pfield_num(&self) -> usize;
    fn write_objas(&self, objas: &mut [ObjAVER]) -> Result<()>;
    fn write_obj0s(&self, obj0s: &mut [Obj0VER]) -> Result<()>;

    fn dump_into<'a, 'b>(&'b self, dst: &mut DumpSlice<'a>, offsets: &'a mut [u32VER], counts: &InfoCounts, pak_header: &mut PakHeaderVER) -> Result<DumpInfosVER<'a, 'b>> {
        let mut dump_infos = DumpInfosVER::from_data(dst, counts, offsets).context("dump infos")?;
        dump_infos.update_header(pak_header);
        debug!("updated header {:#?}", pak_header);
        self.write_objas(dump_infos.objas.take()).context("write objas")?;
        self.write_obj0s(dump_infos.obj0s.take()).context("write obj0s")?;
        Ok(dump_infos)
    }

    fn add_size(&self, offset: usize, counts: &mut InfoCounts) -> usize {
        counts.objas = self.obja_num();
        counts.obj0s = self.obj0_num();
        counts.pfields = self.pfield_num();
        counts.size_ver(offset)
    }
}

#[make_platforms]
impl<'a> DumpExtraInfosVER for InfosRefVER<'a> {
    fn obja_num(&self) -> usize {
        self.objas.len()
    }
    fn obj0_num(&self) -> usize {
        self.obj0s.len()
    }
    fn pfield_num(&self) -> usize {
        self.pfields.len()
    }
    fn write_objas(&self, objas: &mut [ObjAVER]) -> Result<()> {
        objas.write_from(&self.objas[..])
    }
    fn write_obj0s(&self, obj0s: &mut [Obj0VER]) -> Result<()> {
        obj0s.write_from(&self.obj0s[..])
    }
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "ffi", repr(C))] 
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
    pub pfields: usize,
    pub gfxs: usize,
    pub animation_blocks: usize,
    pub foliages: usize,
    pub radiosity_vals: usize,
    pub offsets: usize,
}

impl InfoCounts {
    #[make_platforms]
    pub fn size_ver(&self, mut offset: usize) -> usize {
        offset = align_offset(offset, 16);
        offset = align_offset(offset + self.objas * std::mem::size_of::<ObjAVER>(), 16);
        offset = align_offset(offset + self.obj0s * std::mem::size_of::<Obj0VER>(), 16);
        offset = align_offset(offset + self.models * std::mem::size_of::<ModelInfoVER>(), 16);
        offset = align_offset(offset + self.buffers * std::mem::size_of::<BufferInfoVER>(), 16);
        offset = align_offset(offset + self.mat1s * std::mem::size_of::<Mat1VER>(), 16);
        offset = align_offset(offset + self.mat2s * std::mem::size_of::<Mat2VER>(), 16);
        offset = align_offset(offset + self.mat3s * std::mem::size_of::<Mat3VER>(), 16);
        offset = align_offset(offset + self.mat4s * std::mem::size_of::<Mat4VER>(), 16);
        offset = align_offset(offset + self.mat_extras * std::mem::size_of::<MatExtraVER>(), 16);
        offset = align_offset(offset + self.shapes * std::mem::size_of::<ShapeInfoVER>(), 16);
        offset = align_offset(offset + self.hk_shapes * std::mem::size_of::<HkShapeInfoVER>(), 16);
        offset = align_offset(offset + self.hk_constraint_datas * std::mem::size_of::<HkConstraintDataVER>(), 16);
        offset = align_offset(offset + self.vbuffs * std::mem::size_of::<VBuffInfoVER>(), 16);
        offset = align_offset(offset + self.ibuffs * std::mem::size_of::<IBuffInfoVER>(), 16);
        offset = align_offset(offset + self.textures * std::mem::size_of::<TextureInfoVER>(), 16);
        offset = align_offset(offset + self.animations * std::mem::size_of::<AnimationInfoVER>(), 16);
        offset = align_offset(offset + self.hk_constraints * std::mem::size_of::<HkConstraintInfoVER>(), 16);
        offset = align_offset(offset + self.effects * std::mem::size_of::<EffectInfoVER>(), 16);
        offset = align_offset(offset + self.pfields * std::mem::size_of::<PFieldInfoVER>(), 16);
        offset = align_offset(offset + self.gfxs * std::mem::size_of::<GFXBlockInfoVER>(), 16);
        offset = align_offset(offset + self.animation_blocks * std::mem::size_of::<AnimationBlockInfoVER>(), 16);
        offset = align_offset(offset + self.foliages * std::mem::size_of::<FoliageInfoVER>(), 16);
        offset = align_offset(offset + self.radiosity_vals * std::mem::size_of::<RadiosityValsInfoVER>(), 16);
        debug!("infos size {}", offset);
        offset
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct DumpInfo<'a, T: RefFromData> {
    pub vals: mut_slice<'a, T>,
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
    pub fn next(&mut self) -> Result<&'a mut T> {
        let Self {
            vals,
            mut ind,
            mut offset,
        } = std::mem::take(self);
        let (val, vals) = vals.split_first_mut().ok_or(anyhow::anyhow!("ran out of info"))?;
        ind += 1;
        offset += val.size();
        *self = Self { vals, ind, offset };
        Ok(val)
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
#[cfg_attr(feature = "ffi", repr(C))]
pub struct DumpInfoDataVER<'a> {
    pub key: u32VER,
    pub kind: u32VER,
    pub data: CompressedData<'a>
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct DumpInfosVER<'a, 'd> {
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
    pub pfields: DumpInfo<'a, PFieldInfoVER>,
    pub gfxs: DumpInfo<'a, GFXBlockInfoVER>,
    pub animation_blocks: DumpInfo<'a, AnimationBlockInfoVER>,
    pub foliages: DumpInfo<'a, FoliageInfoVER>,
    pub radiosity_vals: DumpInfo<'a, RadiosityValsInfoVER>,
    pub offsets: DumpInfo<'a, u32VER>,
    pub model_data: Vec<DumpInfoDataVER<'d>>,
    pub texture_data: Vec<DumpInfoDataVER<'d>>,
}

#[make_platforms]
impl<'a> DumpInfosVER<'a, '_> {
    pub fn from_data(dst: &mut DumpSlice<'a>, counts: &InfoCounts, offsets: &'a mut [u32VER]) -> Result<Self> {
        dst.align(16)?;
        let objas = DumpInfo::<ObjAVER>::ref_from(dst, counts.objas).context("objas")?;
        
        dst.align(16)?;
        let obj0s = DumpInfo::<Obj0VER>::ref_from(dst, counts.obj0s).context("obj0s")?;

        dst.align(16)?;
        let models = DumpInfo::<ModelInfoVER>::ref_from(dst, counts.models).context("model_infos")?;

        dst.align(16)?;
        let buffers = DumpInfo::<BufferInfoVER>::ref_from(dst, counts.buffers).context("buffer_infos")?;

        dst.align(16)?;
        let mat1s = DumpInfo::<Mat1VER>::ref_from(dst, counts.mat1s).context("mat1s")?;

        dst.align(16)?;
        let mat2s = DumpInfo::<Mat2VER>::ref_from(dst, counts.mat2s).context("mat2s")?;

        dst.align(16)?;
        let mat3s = DumpInfo::<Mat3VER>::ref_from(dst, counts.mat3s).context("mat3s")?;

        dst.align(16)?;
        let mat4s = DumpInfo::<Mat4VER>::ref_from(dst, counts.mat4s).context("mat4s")?;

        dst.align(16)?;
        let mat_extras = DumpInfo::<MatExtraVER>::ref_from(dst, counts.mat_extras).context("mat_extras")?;

        dst.align(16)?;
        let shapes = DumpInfo::<ShapeInfoVER>::ref_from(dst, counts.shapes).context("shape_infos")?;

        dst.align(16)?;
        let hk_shapes = DumpInfo::<HkShapeInfoVER>::ref_from(dst, counts.hk_shapes).context("hk_shape_infos")?;

        dst.align(16)?;
        let hk_constraint_datas = DumpInfo::<HkConstraintDataVER>::ref_from(dst, counts.hk_constraint_datas).context("hk_constraint_datas")?;

        dst.align(16)?;
        let vbuffs = DumpInfo::<VBuffInfoVER>::ref_from(dst, counts.vbuffs).context("vbuff_infos")?;

        dst.align(16)?;
        let ibuffs = DumpInfo::<IBuffInfoVER>::ref_from(dst, counts.ibuffs).context("ibuff_infos")?;

        dst.align(16)?;
        let textures = DumpInfo::<TextureInfoVER>::ref_from(dst, counts.textures).context("texture_infos")?;

        dst.align(16)?;
        let animations = DumpInfo::<AnimationInfoVER>::ref_from(dst, counts.animations).context("animation_infos")?;

        dst.align(16)?;
        let hk_constraints = DumpInfo::<HkConstraintInfoVER>::ref_from(dst, counts.hk_constraints).context("hk_constraint_infos")?;

        dst.align(16)?;
        let effects = DumpInfo::<EffectInfoVER>::ref_from(dst, counts.effects).context("effect_infos")?;

        dst.align(16)?;
        let pfields = DumpInfo::<PFieldInfoVER>::ref_from(dst, counts.pfields).context("pfield_infos")?;

        dst.align(16)?;
        let gfxs = DumpInfo::<GFXBlockInfoVER>::ref_from(dst, counts.gfxs).context("gfx_block_infos")?;

        dst.align(16)?;
        let animation_blocks = DumpInfo::<AnimationBlockInfoVER>::ref_from(dst, counts.animation_blocks).context("animation_block_infos")?;

        dst.align(16)?;
        let foliages = DumpInfo::<FoliageInfoVER>::ref_from(dst, counts.foliages).context("foliage_infos")?;

        dst.align(16)?;
        let radiosity_vals = DumpInfo::<RadiosityValsInfoVER>::ref_from(dst, counts.radiosity_vals).context("radiosity_vals_infos")?;
        dst.align(16)?;

        let texture_data = Vec::with_capacity(textures.len() * 2);
        let model_data = Vec::with_capacity(models.len() + 1);

        Ok(Self {
            objas,
            obj0s,
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
            effects,
            pfields,
            gfxs,
            animation_blocks,
            foliages,
            radiosity_vals,
            texture_data,
            model_data,
            offsets: DumpInfo { vals: offsets, ind: 0, offset: 0 },
        })
    }

    pub fn update_header(&self, pak_header: &mut PakHeaderVER) {
        pak_header.obja_offset = self.objas.offset.conv();
        pak_header.obja_num = self.objas.len().conv();
        pak_header.obj0_offset = self.obj0s.offset.conv();
        pak_header.obj0_num = self.obj0s.len().conv();
        pak_header.model_info_offset = self.models.offset.conv();
        pak_header.model_info_num = self.models.len().conv();
        pak_header.buffer_info_offset = self.buffers.offset.conv();
        pak_header.buffer_info_num = self.buffers.len().conv();
        pak_header.mat1_offset = self.mat1s.offset.conv();
        pak_header.mat1_num = self.mat1s.len().conv();
        pak_header.mat2_offset = self.mat2s.offset.conv();
        pak_header.mat2_num = self.mat2s.len().conv();
        pak_header.mat3_offset = self.mat3s.offset.conv();
        pak_header.mat3_num = self.mat3s.len().conv();
        pak_header.mat4_offset = self.mat4s.offset.conv();
        pak_header.mat4_num = self.mat4s.len().conv();
        pak_header.mat_extra_offset = self.mat_extras.offset.conv();
        pak_header.mat_extra_num = self.mat_extras.len().conv();
        pak_header.shape_info_offset = self.shapes.offset.conv();
        pak_header.shape_info_num = self.shapes.len().conv();
        pak_header.hk_shape_info_offset = self.hk_shapes.offset.conv();
        pak_header.hk_shape_info_num = self.hk_shapes.len().conv();
        pak_header.hk_constraint_data_offset = self.hk_constraint_datas.offset.conv();
        pak_header.hk_constraint_data_num = self.hk_constraint_datas.len().conv();
        pak_header.vbuff_info_offset = self.vbuffs.offset.conv();
        pak_header.vbuff_info_num = self.vbuffs.len().conv();
        pak_header.ibuff_info_offset = self.ibuffs.offset.conv();
        pak_header.ibuff_info_num = self.ibuffs.len().conv();
        pak_header.texture_info_offset = self.textures.offset.conv();
        pak_header.texture_info_num = self.textures.len().conv();
        pak_header.animation_info_offset = self.animations.offset.conv();
        pak_header.animation_info_num = self.animations.len().conv();
        pak_header.hk_constraint_info_offset = self.hk_constraints.offset.conv();
        pak_header.hk_constraint_info_num = self.hk_constraints.len().conv();
        pak_header.effect_info_offset = self.effects.offset.conv();
        pak_header.effect_info_num = self.effects.len().conv();
        pak_header.foliage_info_offset = self.foliages.offset.conv();
        pak_header.foliage_info_num = self.foliages.len().conv();
        pak_header.pfield_info_offset = self.pfields.offset.conv();
        pak_header.pfield_info_num = self.pfields.len().conv();
        pak_header.gfx_block_info_offset = self.gfxs.offset.conv();
        pak_header.gfx_block_info_num = self.gfxs.len().conv();
        pak_header.radiosity_vals_info_offset = self.radiosity_vals.offset.conv();
        pak_header.radiosity_vals_info_num = self.radiosity_vals.len().conv();
        pak_header.animation_block_info_offset = self.animation_blocks.offset.conv();
        pak_header.animation_block_info_num = self.animation_blocks.len().conv();
    }
}
