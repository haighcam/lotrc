use log::debug;
use anyhow::{Context, Result};

use crate::types::{CompressedData, OrderedData, DumpSlice, ref_slice, align_offset, mut_slice, DumpData, RefFromData};

use lotrc_proc::{make_endian};
#[make_endian]
use crate::{
    level::{
        model::{
            data::{BufferInfo_XE_, IBuffInfo_XE_, VBuffInfo_XE_},
            mat::{Mat1_XE_, Mat2_XE_, Mat3_XE_, Mat4_XE_, MatExtra_XE_},
            shape::{
                HkConstraintData_XE_, HkConstraintInfo_XE_, HkShapeInfo_XE_,
                ShapeInfo_XE_,
            },
            ModelInfo_XE_  
        },
        pak::{
            PakHeader_XE_,
            animation::{AnimationBlockInfo_XE_, AnimationInfo_XE_},
            block1::{
                objs::{ObjA_XE_, Obj0_XE_, EffectInfo_XE_, PFieldInfo_XE_, GFXBlockInfo_XE_, FoliageInfo_XE_},
            },
        },
        radiosity::{RadiosityValsInfo_XE_},
        texture::{TextureInfo_XE_},
    },
    types::u32_XE_
};

#[make_endian]
#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct InfosRef_XE_<'a> {
    pub objas: ref_slice<'a, ObjA_XE_>,
    pub obj0s: ref_slice<'a, Obj0_XE_>,
    pub models: ref_slice<'a, ModelInfo_XE_>,
    pub buffers: ref_slice<'a, BufferInfo_XE_>,
    pub mat1s: ref_slice<'a, Mat1_XE_>,
    pub mat2s: ref_slice<'a, Mat2_XE_>,
    pub mat3s: ref_slice<'a, Mat3_XE_>,
    pub mat4s: ref_slice<'a, Mat4_XE_>,
    pub mat_extras: ref_slice<'a, MatExtra_XE_>,
    pub shapes: ref_slice<'a, ShapeInfo_XE_>,
    pub hk_shapes: ref_slice<'a, HkShapeInfo_XE_>,
    pub hk_constraint_datas: ref_slice<'a, HkConstraintData_XE_>,
    pub vbuffs: ref_slice<'a, VBuffInfo_XE_>,
    pub ibuffs: ref_slice<'a, IBuffInfo_XE_>,
    pub textures: ref_slice<'a, TextureInfo_XE_>,
    pub animations: ref_slice<'a, AnimationInfo_XE_>,
    pub hk_constraints: ref_slice<'a, HkConstraintInfo_XE_>,
    pub effects: ref_slice<'a, EffectInfo_XE_>,
    pub pfields: ref_slice<'a, PFieldInfo_XE_>,
    pub gfxs: ref_slice<'a, GFXBlockInfo_XE_>,
    pub animation_blocks: ref_slice<'a, AnimationBlockInfo_XE_>,
    pub foliages: ref_slice<'a, FoliageInfo_XE_>,
    pub radiosity_vals: ref_slice<'a, RadiosityValsInfo_XE_>,
}

#[make_endian]
impl<'a> InfosRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeader_XE_) -> Result<Self> {
        let objas = ObjA_XE_::slice_from_data(
            &src[pak_header.obja_offset.conv()..],
            pak_header.obja_num.conv(),
        )
        .context("objas")?;
        let obj0s = Obj0_XE_::slice_from_data(
            &src[pak_header.obj0_offset.conv()..],
            pak_header.obj0_num.conv(),
        )
        .context("obj0s")?;
        let models = ModelInfo_XE_::slice_from_data(
            &src[pak_header.model_info_offset.conv()..],
            pak_header.model_info_num.conv(),
        )
        .context("models")?;
        let buffers = BufferInfo_XE_::slice_from_data(
            &src[pak_header.buffer_info_offset.conv()..],
            pak_header.buffer_info_num.conv(),
        )
        .context("buffers")?;
        let mat1s = Mat1_XE_::slice_from_data(
            &src[pak_header.mat1_offset.conv()..],
            pak_header.mat1_num.conv(),
        )
        .context("mat1s")?;
        let mat2s = Mat2_XE_::slice_from_data(
            &src[pak_header.mat2_offset.conv()..],
            pak_header.mat2_num.conv(),
        )
        .context("mat2s")?;
        let mat3s = Mat3_XE_::slice_from_data(
            &src[pak_header.mat3_offset.conv()..],
            pak_header.mat3_num.conv(),
        )
        .context("mat3s")?;
        let mat4s = Mat4_XE_::slice_from_data(
            &src[pak_header.mat4_offset.conv()..],
            pak_header.mat4_num.conv(),
        )
        .context("mat4s")?;
        let mat_extras = MatExtra_XE_::slice_from_data(
            &src[pak_header.mat_extra_offset.conv()..],
            pak_header.mat_extra_num.conv(),
        )
        .context("mat_extras")?;
        let shapes = ShapeInfo_XE_::slice_from_data(
            &src[pak_header.shape_info_offset.conv()..],
            pak_header.shape_info_num.conv(),
        )
        .context("shapes")?;
        let hk_shapes = HkShapeInfo_XE_::slice_from_data(
            &src[pak_header.hk_shape_info_offset.conv()..],
            pak_header.hk_shape_info_num.conv(),
        )
        .context("hk_shapes")?;
        let hk_constraint_datas = HkConstraintData_XE_::slice_from_data(
            &src[pak_header.hk_constraint_data_offset.conv()..],
            pak_header.hk_constraint_data_num.conv(),
        )
        .context("hk_constraint_datas")?;
        let vbuffs = VBuffInfo_XE_::slice_from_data(
            &src[pak_header.vbuff_info_offset.conv()..],
            pak_header.vbuff_info_num.conv(),
        )
        .context("vbuffs")?;
        let ibuffs = IBuffInfo_XE_::slice_from_data(
            &src[pak_header.ibuff_info_offset.conv()..],
            pak_header.ibuff_info_num.conv(),
        )
        .context("ibuffs")?;
        let textures = TextureInfo_XE_::slice_from_data(
            &src[pak_header.texture_info_offset.conv()..],
            pak_header.texture_info_num.conv(),
        )
        .context("textures")?;
        let animations = AnimationInfo_XE_::slice_from_data(
            &src[pak_header.animation_info_offset.conv()..],
            pak_header.animation_info_num.conv(),
        )
        .context("animations")?;
        let hk_constraints = HkConstraintInfo_XE_::slice_from_data(
            &src[pak_header.hk_constraint_info_offset.conv()..],
            pak_header.hk_constraint_info_num.conv(),
        )
        .context("hk_constraints")?;
        let effects = EffectInfo_XE_::slice_from_data(
            &src[pak_header.effect_info_offset.conv()..],
            pak_header.effect_info_num.conv(),
        )
        .context("effects")?;
        let pfields = PFieldInfo_XE_::slice_from_data(
            &src[pak_header.pfield_info_offset.conv()..],
            pak_header.pfield_info_num.conv(),
        )
        .context("pfields")?;
        let gfxs = GFXBlockInfo_XE_::slice_from_data(
            &src[pak_header.gfx_block_info_offset.conv()..],
            pak_header.gfx_block_info_num.conv(),
        )
        .context("gfxs")?;
        let animation_blocks = AnimationBlockInfo_XE_::slice_from_data(
            &src[pak_header.animation_block_info_offset.conv()..],
            pak_header.animation_block_info_num.conv(),
        )
        .context("animation_blocks")?;
        debug!("animation block infos {:#?}", animation_blocks);
        let foliages = FoliageInfo_XE_::slice_from_data(
            &src[pak_header.foliage_info_offset.conv()..],
            pak_header.foliage_info_num.conv(),
        )
        .context("foliages")?;
        let radiosity_vals = RadiosityValsInfo_XE_::slice_from_data(
            &src[pak_header.radiosity_vals_info_offset.conv()..],
            pak_header.radiosity_vals_info_num.conv(),
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

#[make_endian]
pub trait DumpExtraInfos_XE_ {
    fn obja_num(&self) -> usize;
    fn obj0_num(&self) -> usize;
    // TODO proper pfield dumping
    fn pfield_num(&self) -> usize;
    fn write_objas(&self, objas: &mut [ObjA_XE_]) -> Result<()>;
    fn write_obj0s(&self, obj0s: &mut [Obj0_XE_]) -> Result<()>;

    fn dump_into<'a, 'b>(&'b self, dst: &mut DumpSlice<'a>, offsets: &'a mut [u32_XE_], counts: &InfoCounts, pak_header: &mut PakHeader_XE_) -> Result<DumpInfos_XE_<'a, 'b>> {
        let mut dump_infos = DumpInfos_XE_::from_data(dst, counts, offsets).context("dump infos")?;
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
        counts.size_xe_(offset)
    }
}

#[make_endian]
impl<'a> DumpExtraInfos_XE_ for InfosRef_XE_<'a> {
    fn obja_num(&self) -> usize {
        self.objas.len()
    }
    fn obj0_num(&self) -> usize {
        self.obj0s.len()
    }
    fn pfield_num(&self) -> usize {
        self.pfields.len()
    }
    fn write_objas(&self, objas: &mut [ObjA_XE_]) -> Result<()> {
        objas.write_from(&self.objas[..])
    }
    fn write_obj0s(&self, obj0s: &mut [Obj0_XE_]) -> Result<()> {
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
    #[make_endian]
    pub fn size_xe_(&self, mut offset: usize) -> usize {
        offset = align_offset(offset, 16);
        offset = align_offset(offset + self.objas * std::mem::size_of::<ObjA_XE_>(), 16);
        offset = align_offset(offset + self.obj0s * std::mem::size_of::<Obj0_XE_>(), 16);
        offset = align_offset(offset + self.models * std::mem::size_of::<ModelInfo_XE_>(), 16);
        offset = align_offset(offset + self.buffers * std::mem::size_of::<BufferInfo_XE_>(), 16);
        offset = align_offset(offset + self.mat1s * std::mem::size_of::<Mat1_XE_>(), 16);
        offset = align_offset(offset + self.mat2s * std::mem::size_of::<Mat2_XE_>(), 16);
        offset = align_offset(offset + self.mat3s * std::mem::size_of::<Mat3_XE_>(), 16);
        offset = align_offset(offset + self.mat4s * std::mem::size_of::<Mat4_XE_>(), 16);
        offset = align_offset(offset + self.mat_extras * std::mem::size_of::<MatExtra_XE_>(), 16);
        offset = align_offset(offset + self.shapes * std::mem::size_of::<ShapeInfo_XE_>(), 16);
        offset = align_offset(offset + self.hk_shapes * std::mem::size_of::<HkShapeInfo_XE_>(), 16);
        offset = align_offset(offset + self.hk_constraint_datas * std::mem::size_of::<HkConstraintData_XE_>(), 16);
        offset = align_offset(offset + self.vbuffs * std::mem::size_of::<VBuffInfo_XE_>(), 16);
        offset = align_offset(offset + self.ibuffs * std::mem::size_of::<IBuffInfo_XE_>(), 16);
        offset = align_offset(offset + self.textures * std::mem::size_of::<TextureInfo_XE_>(), 16);
        offset = align_offset(offset + self.animations * std::mem::size_of::<AnimationInfo_XE_>(), 16);
        offset = align_offset(offset + self.hk_constraints * std::mem::size_of::<HkConstraintInfo_XE_>(), 16);
        offset = align_offset(offset + self.effects * std::mem::size_of::<EffectInfo_XE_>(), 16);
        offset = align_offset(offset + self.pfields * std::mem::size_of::<PFieldInfo_XE_>(), 16);
        offset = align_offset(offset + self.gfxs * std::mem::size_of::<GFXBlockInfo_XE_>(), 16);
        offset = align_offset(offset + self.animation_blocks * std::mem::size_of::<AnimationBlockInfo_XE_>(), 16);
        offset = align_offset(offset + self.foliages * std::mem::size_of::<FoliageInfo_XE_>(), 16);
        offset = align_offset(offset + self.radiosity_vals * std::mem::size_of::<RadiosityValsInfo_XE_>(), 16);
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

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct DumpInfoData_XE_<'a> {
    pub key: u32_XE_,
    pub kind: u32_XE_,
    pub data: CompressedData<'a>
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct DumpInfos_XE_<'a, 'd> {
    pub objas: DumpInfo<'a, ObjA_XE_>,
    pub obj0s: DumpInfo<'a, Obj0_XE_>,
    pub models: DumpInfo<'a, ModelInfo_XE_>,
    pub buffers: DumpInfo<'a, BufferInfo_XE_>,
    pub mat1s: DumpInfo<'a, Mat1_XE_>,
    pub mat2s: DumpInfo<'a, Mat2_XE_>,
    pub mat3s: DumpInfo<'a, Mat3_XE_>,
    pub mat4s: DumpInfo<'a, Mat4_XE_>,
    pub mat_extras: DumpInfo<'a, MatExtra_XE_>,
    pub shapes: DumpInfo<'a, ShapeInfo_XE_>,
    pub hk_shapes: DumpInfo<'a, HkShapeInfo_XE_>,
    pub hk_constraint_datas: DumpInfo<'a, HkConstraintData_XE_>,
    pub vbuffs: DumpInfo<'a, VBuffInfo_XE_>,
    pub ibuffs: DumpInfo<'a, IBuffInfo_XE_>,
    pub textures: DumpInfo<'a, TextureInfo_XE_>,
    pub animations: DumpInfo<'a, AnimationInfo_XE_>,
    pub hk_constraints: DumpInfo<'a, HkConstraintInfo_XE_>,
    pub effects: DumpInfo<'a, EffectInfo_XE_>,
    pub pfields: DumpInfo<'a, PFieldInfo_XE_>,
    pub gfxs: DumpInfo<'a, GFXBlockInfo_XE_>,
    pub animation_blocks: DumpInfo<'a, AnimationBlockInfo_XE_>,
    pub foliages: DumpInfo<'a, FoliageInfo_XE_>,
    pub radiosity_vals: DumpInfo<'a, RadiosityValsInfo_XE_>,
    pub offsets: DumpInfo<'a, u32_XE_>,
    pub model_data: Vec<DumpInfoData_XE_<'d>>,
    pub texture_data: Vec<DumpInfoData_XE_<'d>>,
}

#[make_endian]
impl<'a> DumpInfos_XE_<'a, '_> {
    pub fn from_data(dst: &mut DumpSlice<'a>, counts: &InfoCounts, offsets: &'a mut [u32_XE_]) -> Result<Self> {
        dst.align(16)?;
        let objas = DumpInfo::<ObjA_XE_>::ref_from(dst, counts.objas).context("objas")?;
        
        dst.align(16)?;
        let obj0s = DumpInfo::<Obj0_XE_>::ref_from(dst, counts.obj0s).context("obj0s")?;

        dst.align(16)?;
        let models = DumpInfo::<ModelInfo_XE_>::ref_from(dst, counts.models).context("model_infos")?;

        dst.align(16)?;
        let buffers = DumpInfo::<BufferInfo_XE_>::ref_from(dst, counts.buffers).context("buffer_infos")?;

        dst.align(16)?;
        let mat1s = DumpInfo::<Mat1_XE_>::ref_from(dst, counts.mat1s).context("mat1s")?;

        dst.align(16)?;
        let mat2s = DumpInfo::<Mat2_XE_>::ref_from(dst, counts.mat2s).context("mat2s")?;

        dst.align(16)?;
        let mat3s = DumpInfo::<Mat3_XE_>::ref_from(dst, counts.mat3s).context("mat3s")?;

        dst.align(16)?;
        let mat4s = DumpInfo::<Mat4_XE_>::ref_from(dst, counts.mat4s).context("mat4s")?;

        dst.align(16)?;
        let mat_extras = DumpInfo::<MatExtra_XE_>::ref_from(dst, counts.mat_extras).context("mat_extras")?;

        dst.align(16)?;
        let shapes = DumpInfo::<ShapeInfo_XE_>::ref_from(dst, counts.shapes).context("shape_infos")?;

        dst.align(16)?;
        let hk_shapes = DumpInfo::<HkShapeInfo_XE_>::ref_from(dst, counts.hk_shapes).context("hk_shape_infos")?;

        dst.align(16)?;
        let hk_constraint_datas = DumpInfo::<HkConstraintData_XE_>::ref_from(dst, counts.hk_constraint_datas).context("hk_constraint_datas")?;

        dst.align(16)?;
        let vbuffs = DumpInfo::<VBuffInfo_XE_>::ref_from(dst, counts.vbuffs).context("vbuff_infos")?;

        dst.align(16)?;
        let ibuffs = DumpInfo::<IBuffInfo_XE_>::ref_from(dst, counts.ibuffs).context("ibuff_infos")?;

        dst.align(16)?;
        let textures = DumpInfo::<TextureInfo_XE_>::ref_from(dst, counts.textures).context("texture_infos")?;

        dst.align(16)?;
        let animations = DumpInfo::<AnimationInfo_XE_>::ref_from(dst, counts.animations).context("animation_infos")?;

        dst.align(16)?;
        let hk_constraints = DumpInfo::<HkConstraintInfo_XE_>::ref_from(dst, counts.hk_constraints).context("hk_constraint_infos")?;

        dst.align(16)?;
        let effects = DumpInfo::<EffectInfo_XE_>::ref_from(dst, counts.effects).context("effect_infos")?;

        dst.align(16)?;
        let pfields = DumpInfo::<PFieldInfo_XE_>::ref_from(dst, counts.pfields).context("pfield_infos")?;

        dst.align(16)?;
        let gfxs = DumpInfo::<GFXBlockInfo_XE_>::ref_from(dst, counts.gfxs).context("gfx_block_infos")?;

        dst.align(16)?;
        let animation_blocks = DumpInfo::<AnimationBlockInfo_XE_>::ref_from(dst, counts.animation_blocks).context("animation_block_infos")?;

        dst.align(16)?;
        let foliages = DumpInfo::<FoliageInfo_XE_>::ref_from(dst, counts.foliages).context("foliage_infos")?;

        dst.align(16)?;
        let radiosity_vals = DumpInfo::<RadiosityValsInfo_XE_>::ref_from(dst, counts.radiosity_vals).context("radiosity_vals_infos")?;
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

    pub fn update_header(&self, pak_header: &mut PakHeader_XE_) {
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
