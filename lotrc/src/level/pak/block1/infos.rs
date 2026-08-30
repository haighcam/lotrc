use log::debug;
use anyhow::{Context, Result};
use indexmap::IndexMap;

use crate::{
    level::{
        model::{
            data::{BufferInfo, IBuffInfo, VBuffInfo},
            mat::{Mat1, Mat2, Mat3, Mat4, MatExtra},
            shape::{
                HkConstraintData, HkConstraintInfo, HkShapeInfo,
                ShapeInfo,
            },
            ModelInfo  
        },
    },
    level::{
        pak::{
            PakHeader,
            animation::{AnimationBlockInfo, AnimationInfo},
            block1::{
                objs::{ObjA, Obj0, EffectInfo, PFieldInfo, GFXBlockInfo, FoliageInfo, TextureInfo},
            },
        },
        radiosity::{RadiosityValsInfo},
    },
    types::{CompressedData, DumpSlice, ref_slice, align_offset, mut_slice, ReadData, BaseTypes}
};


#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct InfosRef<'a,  T: BaseTypes> {
    pub objas: ref_slice<'a, ObjA<T>>,
    pub obj0s: ref_slice<'a, Obj0<T>>,
    pub models: ref_slice<'a, ModelInfo<T>>,
    pub buffers: ref_slice<'a, BufferInfo<T>>,
    pub mat1s: ref_slice<'a, Mat1<T>>,
    pub mat2s: ref_slice<'a, Mat2<T>>,
    pub mat3s: ref_slice<'a, Mat3<T>>,
    pub mat4s: ref_slice<'a, Mat4<T>>,
    pub mat_extras: ref_slice<'a, MatExtra<T>>,
    pub shapes: ref_slice<'a, ShapeInfo<T>>,
    pub hk_shapes: ref_slice<'a, HkShapeInfo<T>>,
    pub hk_constraint_datas: ref_slice<'a, HkConstraintData<T>>,
    pub vbuffs: ref_slice<'a, VBuffInfo<T>>,
    pub ibuffs: ref_slice<'a, IBuffInfo<T>>,
    pub textures: ref_slice<'a, TextureInfo<T>>,
    pub animations: ref_slice<'a, AnimationInfo<T>>,
    pub hk_constraints: ref_slice<'a, HkConstraintInfo<T>>,
    pub effects: ref_slice<'a, EffectInfo<T>>,
    pub pfields: ref_slice<'a, PFieldInfo<T>>,
    pub gfxs: ref_slice<'a, GFXBlockInfo<T>>,
    pub animation_blocks: ref_slice<'a, AnimationBlockInfo<T>>,
    pub foliages: ref_slice<'a, FoliageInfo<T>>,
    pub radiosity_vals: ref_slice<'a, RadiosityValsInfo<T>>,
}

impl <'a,  T: BaseTypes> InfosRef<'a, T> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeader<T>) -> Result<Self> {
        let objas = ObjA::slice_from_data(
            &src[pak_header.obja_offset.into() as usize..],
            pak_header.obja_num.into() as usize,
        )
        .context("objas")?;
        let obj0s = Obj0::slice_from_data(
            &src[pak_header.obj0_offset.into() as usize..],
            pak_header.obj0_num.into() as usize,
        )
        .context("obj0s")?;
        let models = ModelInfo::slice_from_data(
            &src[pak_header.model_info_offset.into() as usize..],
            pak_header.model_info_num.into() as usize,
        )
        .context("models")?;
        let buffers = BufferInfo::slice_from_data(
            &src[pak_header.buffer_info_offset.into() as usize..],
            pak_header.buffer_info_num.into() as usize,
        )
        .context("buffers")?;
        let mat1s = Mat1::slice_from_data(
            &src[pak_header.mat1_offset.into() as usize..],
            pak_header.mat1_num.into() as usize,
        )
        .context("mat1s")?;
        let mat2s = Mat2::slice_from_data(
            &src[pak_header.mat2_offset.into() as usize..],
            pak_header.mat2_num.into() as usize,
        )
        .context("mat2s")?;
        let mat3s = Mat3::slice_from_data(
            &src[pak_header.mat3_offset.into() as usize..],
            pak_header.mat3_num.into() as usize,
        )
        .context("mat3s")?;
        let mat4s = Mat4::slice_from_data(
            &src[pak_header.mat4_offset.into() as usize..],
            pak_header.mat4_num.into() as usize,
        )
        .context("mat4s")?;
        let mat_extras = MatExtra::slice_from_data(
            &src[pak_header.mat_extra_offset.into() as usize..],
            pak_header.mat_extra_num.into() as usize,
        )
        .context("mat_extras")?;
        let shapes = ShapeInfo::slice_from_data(
            &src[pak_header.shape_info_offset.into() as usize..],
            pak_header.shape_info_num.into() as usize,
        )
        .context("shapes")?;
        let hk_shapes = HkShapeInfo::slice_from_data(
            &src[pak_header.hk_shape_info_offset.into() as usize..],
            pak_header.hk_shape_info_num.into() as usize,
        )
        .context("hk_shapes")?;
        let hk_constraint_datas = HkConstraintData::slice_from_data(
            &src[pak_header.hk_constraint_data_offset.into() as usize..],
            pak_header.hk_constraint_data_num.into() as usize,
        )
        .context("hk_constraint_datas")?;
        let vbuffs = VBuffInfo::slice_from_data(
            &src[pak_header.vbuff_info_offset.into() as usize..],
            pak_header.vbuff_info_num.into() as usize,
        )
        .context("vbuffs")?;
        let ibuffs = IBuffInfo::slice_from_data(
            &src[pak_header.ibuff_info_offset.into() as usize..],
            pak_header.ibuff_info_num.into() as usize,
        )
        .context("ibuffs")?;
        let textures = TextureInfo::slice_from_data(
            &src[pak_header.texture_info_offset.into() as usize..],
            pak_header.texture_info_num.into() as usize,
        )
        .context("textures")?;
        let animations = AnimationInfo::slice_from_data(
            &src[pak_header.animation_info_offset.into() as usize..],
            pak_header.animation_info_num.into() as usize,
        )
        .context("animations")?;
        let hk_constraints = HkConstraintInfo::slice_from_data(
            &src[pak_header.hk_constraint_info_offset.into() as usize..],
            pak_header.hk_constraint_info_num.into() as usize,
        )
        .context("hk_constraints")?;
        let effects = EffectInfo::slice_from_data(
            &src[pak_header.effect_info_offset.into() as usize..],
            pak_header.effect_info_num.into() as usize,
        )
        .context("effects")?;
        let pfields = PFieldInfo::slice_from_data(
            &src[pak_header.pfield_info_offset.into() as usize..],
            pak_header.pfield_info_num.into() as usize,
        )
        .context("pfields")?;
        let gfxs = GFXBlockInfo::slice_from_data(
            &src[pak_header.gfx_block_info_offset.into() as usize..],
            pak_header.gfx_block_info_num.into() as usize,
        )
        .context("gfxs")?;
        let animation_blocks = AnimationBlockInfo::slice_from_data(
            &src[pak_header.animation_block_info_offset.into() as usize..],
            pak_header.animation_block_info_num.into() as usize,
        )
        .context("animation_blocks")?;
        let foliages = FoliageInfo::slice_from_data(
            &src[pak_header.foliage_info_offset.into() as usize..],
            pak_header.foliage_info_num.into() as usize,
        )
        .context("foliages")?;
        let radiosity_vals = RadiosityValsInfo::slice_from_data(
            &src[pak_header.radiosity_vals_info_offset.into() as usize..],
            pak_header.radiosity_vals_info_num.into() as usize,
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

#[cfg_attr(feature = "ffi", repr(C))]
pub struct DumpInfo<'a, T: ReadData> {
    pub vals: mut_slice<'a, T>,
    pub ind: usize,
    pub offset: usize,
}

impl<'a, T: ReadData> Default for DumpInfo<'a, T> {
    fn default() -> Self {
        Self {
            vals: &mut [],
            ind: 0,
            offset: 0,
        }
    }
}

impl<'a, T: ReadData> DumpInfo<'a, T> {
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
        offset += std::mem::size_of_val(val);
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
        offset += std::mem::size_of_val(val);
        *self = Self { vals, ind, offset };
        val
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct DumpInfoData<'a, T: BaseTypes> {
    pub key: T::u32,
    pub kind: T::u32,
    pub data: CompressedData<'a>
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct DumpInfos<'a, 'd, T: BaseTypes> {
    pub objas: DumpInfo<'a, ObjA<T>>,
    pub obj0s: DumpInfo<'a, Obj0<T>>,
    pub models: DumpInfo<'a, ModelInfo<T>>,
    pub buffers: DumpInfo<'a, BufferInfo<T>>,
    pub mat1s: DumpInfo<'a, Mat1<T>>,
    pub mat2s: DumpInfo<'a, Mat2<T>>,
    pub mat3s: DumpInfo<'a, Mat3<T>>,
    pub mat4s: DumpInfo<'a, Mat4<T>>,
    pub mat_extras: DumpInfo<'a, MatExtra<T>>,
    pub shapes: DumpInfo<'a, ShapeInfo<T>>,
    pub hk_shapes: DumpInfo<'a, HkShapeInfo<T>>,
    pub hk_constraint_datas: DumpInfo<'a, HkConstraintData<T>>,
    pub vbuffs: DumpInfo<'a, VBuffInfo<T>>,
    pub ibuffs: DumpInfo<'a, IBuffInfo<T>>,
    pub textures: DumpInfo<'a, TextureInfo<T>>,
    pub animations: DumpInfo<'a, AnimationInfo<T>>,
    pub hk_constraints: DumpInfo<'a, HkConstraintInfo<T>>,
    pub effects: DumpInfo<'a, EffectInfo<T>>,
    pub pfields: DumpInfo<'a, PFieldInfo<T>>,
    pub gfxs: DumpInfo<'a, GFXBlockInfo<T>>,
    pub animation_blocks: DumpInfo<'a, AnimationBlockInfo<T>>,
    pub foliages: DumpInfo<'a, FoliageInfo<T>>,
    pub radiosity_vals: DumpInfo<'a, RadiosityValsInfo<T>>,
    pub offsets: DumpInfo<'a, T::u32>,
    pub model_data: Vec<DumpInfoData<'d, T>>,
    pub texture_data: IndexMap<u32, DumpInfoData<'d, T>>,
}

impl<'a, T: BaseTypes> DumpInfos<'a, '_, T> {
    pub fn from_data(dst: &mut DumpSlice<'a>, counts: &InfoCounts, offsets: &'a mut [T::u32]) -> Result<Self> {
        dst.align(16)?;
        let objas = DumpInfo::<ObjA<T>>::ref_from(dst, counts.objas).context("objas")?;
        
        dst.align(16)?;
        let obj0s = DumpInfo::<Obj0<T>>::ref_from(dst, counts.obj0s).context("obj0s")?;

        dst.align(16)?;
        let models = DumpInfo::<ModelInfo<T>>::ref_from(dst, counts.models).context("model_infos")?;

        dst.align(16)?;
        let buffers = DumpInfo::<BufferInfo<T>>::ref_from(dst, counts.buffers).context("buffer_infos")?;

        dst.align(16)?;
        let mat1s = DumpInfo::<Mat1<T>>::ref_from(dst, counts.mat1s).context("mat1s")?;

        dst.align(16)?;
        let mat2s = DumpInfo::<Mat2<T>>::ref_from(dst, counts.mat2s).context("mat2s")?;

        dst.align(16)?;
        let mat3s = DumpInfo::<Mat3<T>>::ref_from(dst, counts.mat3s).context("mat3s")?;

        dst.align(16)?;
        let mat4s = DumpInfo::<Mat4<T>>::ref_from(dst, counts.mat4s).context("mat4s")?;

        dst.align(16)?;
        let mat_extras = DumpInfo::<MatExtra<T>>::ref_from(dst, counts.mat_extras).context("mat_extras")?;

        dst.align(16)?;
        let shapes = DumpInfo::<ShapeInfo<T>>::ref_from(dst, counts.shapes).context("shape_infos")?;

        dst.align(16)?;
        let hk_shapes = DumpInfo::<HkShapeInfo<T>>::ref_from(dst, counts.hk_shapes).context("hk_shape_infos")?;

        dst.align(16)?;
        let hk_constraint_datas = DumpInfo::<HkConstraintData<T>>::ref_from(dst, counts.hk_constraint_datas).context("hk_constraint_datas")?;

        dst.align(16)?;
        let vbuffs = DumpInfo::<VBuffInfo<T>>::ref_from(dst, counts.vbuffs).context("vbuff_infos")?;

        dst.align(16)?;
        let ibuffs = DumpInfo::<IBuffInfo<T>>::ref_from(dst, counts.ibuffs).context("ibuff_infos")?;

        dst.align(16)?;
        let textures = DumpInfo::<TextureInfo<T>>::ref_from(dst, counts.textures).context("texture_infos")?;

        dst.align(16)?;
        let animations = DumpInfo::<AnimationInfo<T>>::ref_from(dst, counts.animations).context("animation_infos")?;

        dst.align(16)?;
        let hk_constraints = DumpInfo::<HkConstraintInfo<T>>::ref_from(dst, counts.hk_constraints).context("hk_constraint_infos")?;

        dst.align(16)?;
        let effects = DumpInfo::<EffectInfo<T>>::ref_from(dst, counts.effects).context("effect_infos")?;

        dst.align(16)?;
        let pfields = DumpInfo::<PFieldInfo<T>>::ref_from(dst, counts.pfields).context("pfield_infos")?;

        dst.align(16)?;
        let gfxs = DumpInfo::<GFXBlockInfo<T>>::ref_from(dst, counts.gfxs).context("gfx_block_infos")?;

        dst.align(16)?;
        let animation_blocks = DumpInfo::<AnimationBlockInfo<T>>::ref_from(dst, counts.animation_blocks).context("animation_block_infos")?;

        dst.align(16)?;
        let foliages = DumpInfo::<FoliageInfo<T>>::ref_from(dst, counts.foliages).context("foliage_infos")?;

        dst.align(16)?;
        let radiosity_vals = DumpInfo::<RadiosityValsInfo<T>>::ref_from(dst, counts.radiosity_vals).context("radiosity_vals_infos")?;
        dst.align(16)?;

        let texture_data = IndexMap::with_capacity(textures.len() * 2);
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

    pub fn update_header(&self, pak_header: &mut PakHeader<T>) {
        pak_header.obja_offset = (self.objas.offset as u32).into();
        pak_header.obja_num = (self.objas.len() as u32).into();
        pak_header.obj0_offset = (self.obj0s.offset as u32).into();
        pak_header.obj0_num = (self.obj0s.len() as u32).into();
        pak_header.model_info_offset = (self.models.offset as u32).into();
        pak_header.model_info_num = (self.models.len() as u32).into();
        pak_header.buffer_info_offset = (self.buffers.offset as u32).into();
        pak_header.buffer_info_num = (self.buffers.len() as u32).into();
        pak_header.mat1_offset = (self.mat1s.offset as u32).into();
        pak_header.mat1_num = (self.mat1s.len() as u32).into();
        pak_header.mat2_offset = (self.mat2s.offset as u32).into();
        pak_header.mat2_num = (self.mat2s.len() as u32).into();
        pak_header.mat3_offset = (self.mat3s.offset as u32).into();
        pak_header.mat3_num = (self.mat3s.len() as u32).into();
        pak_header.mat4_offset = (self.mat4s.offset as u32).into();
        pak_header.mat4_num = (self.mat4s.len() as u32).into();
        pak_header.mat_extra_offset = (self.mat_extras.offset as u32).into();
        pak_header.mat_extra_num = (self.mat_extras.len() as u32).into();
        pak_header.unk_76 = (self.shapes.offset as u32).into();
        pak_header.shape_info_offset = (self.shapes.offset as u32).into();
        pak_header.shape_info_num = (self.shapes.len() as u32).into();
        pak_header.hk_shape_info_offset = (self.hk_shapes.offset as u32).into();
        pak_header.hk_shape_info_num = (self.hk_shapes.len() as u32).into();
        pak_header.hk_constraint_data_offset = (self.hk_constraint_datas.offset as u32).into();
        pak_header.hk_constraint_data_num = (self.hk_constraint_datas.len() as u32).into();
        pak_header.vbuff_info_offset = (self.vbuffs.offset as u32).into();
        pak_header.vbuff_info_num = (self.vbuffs.len() as u32).into();
        pak_header.ibuff_info_offset = (self.ibuffs.offset as u32).into();
        pak_header.ibuff_info_num = (self.ibuffs.len() as u32).into();
        pak_header.texture_info_offset = (self.textures.offset as u32).into();
        pak_header.texture_info_num = (self.textures.len() as u32).into();
        pak_header.animation_info_offset = (self.animations.offset as u32).into();
        pak_header.animation_info_num = (self.animations.len() as u32).into();
        pak_header.hk_constraint_info_offset = (self.hk_constraints.offset as u32).into();
        pak_header.hk_constraint_info_num = (self.hk_constraints.len() as u32).into();
        pak_header.effect_info_offset = (self.effects.offset as u32).into();
        pak_header.effect_info_num = (self.effects.len() as u32).into();
        pak_header.foliage_info_offset = (self.foliages.offset as u32).into();
        pak_header.foliage_info_num = (self.foliages.len() as u32).into();
        pak_header.pfield_info_offset = (self.pfields.offset as u32).into();
        pak_header.pfield_info_num = (self.pfields.len() as u32).into();
        pak_header.gfx_block_info_offset = (self.gfxs.offset as u32).into();
        pak_header.gfx_block_info_num = (self.gfxs.len() as u32).into();
        pak_header.radiosity_vals_info_offset = (self.radiosity_vals.offset as u32).into();
        pak_header.radiosity_vals_info_num = (self.radiosity_vals.len() as u32).into();
        pak_header.animation_block_info_offset = (self.animation_blocks.offset as u32).into();
        pak_header.animation_block_info_num = (self.animation_blocks.len() as u32).into();
    }
}

pub trait DumpExtraInfos<T: BaseTypes> {
    fn obja_num(&self) -> usize;
    fn obj0_num(&self) -> usize;
    // TODO proper pfield dumping
    fn pfield_num(&self) -> usize;
    fn write_objas(&self, objas: &mut [ObjA<T>]) -> Result<()>;
    fn write_obj0s(&self, obj0s: &mut [Obj0<T>]) -> Result<()>;

    fn dump_into<'a, 'b>(&'b self, dst: &mut DumpSlice<'a>, offsets: &'a mut [T::u32], counts: &InfoCounts, pak_header: &mut PakHeader<T>) -> Result<DumpInfos<'a, 'b, T>> {
        let mut dump_infos = DumpInfos::from_data(dst, counts, offsets).context("dump infos")?;
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
        counts.size::<T>(offset)
    }
}

impl<'a, T: BaseTypes> DumpExtraInfos<T> for InfosRef<'a, T> {
    fn obja_num(&self) -> usize {
        self.objas.len()
    }
    fn obj0_num(&self) -> usize {
        self.obj0s.len()
    }
    fn pfield_num(&self) -> usize {
        self.pfields.len()
    }
    fn write_objas(&self, objas: &mut [ObjA<T>]) -> Result<()> {
        objas.copy_from_slice(self.objas);
        Ok(())
    }
    fn write_obj0s(&self, obj0s: &mut [Obj0<T>]) -> Result<()> {
        obj0s.copy_from_slice(self.obj0s);
        Ok(())
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
    pub fn size<T: BaseTypes>(&self, mut offset: usize) -> usize {
        offset = align_offset(offset, 16);
        offset = align_offset(offset + self.objas * std::mem::size_of::<ObjA<T>>(), 16);
        offset = align_offset(offset + self.obj0s * std::mem::size_of::<Obj0<T>>(), 16);
        offset = align_offset(offset + self.models * std::mem::size_of::<ModelInfo<T>>(), 16);
        offset = align_offset(offset + self.buffers * std::mem::size_of::<BufferInfo<T>>(), 16);
        offset = align_offset(offset + self.mat1s * std::mem::size_of::<Mat1<T>>(), 16);
        offset = align_offset(offset + self.mat2s * std::mem::size_of::<Mat2<T>>(), 16);
        offset = align_offset(offset + self.mat3s * std::mem::size_of::<Mat3<T>>(), 16);
        offset = align_offset(offset + self.mat4s * std::mem::size_of::<Mat4<T>>(), 16);
        offset = align_offset(offset + self.mat_extras * std::mem::size_of::<MatExtra<T>>(), 16);
        offset = align_offset(offset + self.shapes * std::mem::size_of::<ShapeInfo<T>>(), 16);
        offset = align_offset(offset + self.hk_shapes * std::mem::size_of::<HkShapeInfo<T>>(), 16);
        offset = align_offset(offset + self.hk_constraint_datas * std::mem::size_of::<HkConstraintData<T>>(), 16);
        offset = align_offset(offset + self.vbuffs * std::mem::size_of::<VBuffInfo<T>>(), 16);
        offset = align_offset(offset + self.ibuffs * std::mem::size_of::<IBuffInfo<T>>(), 16);
        offset = align_offset(offset + self.textures * std::mem::size_of::<TextureInfo<T>>(), 16);
        offset = align_offset(offset + self.animations * std::mem::size_of::<AnimationInfo<T>>(), 16);
        offset = align_offset(offset + self.hk_constraints * std::mem::size_of::<HkConstraintInfo<T>>(), 16);
        offset = align_offset(offset + self.effects * std::mem::size_of::<EffectInfo<T>>(), 16);
        offset = align_offset(offset + self.pfields * std::mem::size_of::<PFieldInfo<T>>(), 16);
        offset = align_offset(offset + self.gfxs * std::mem::size_of::<GFXBlockInfo<T>>(), 16);
        offset = align_offset(offset + self.animation_blocks * std::mem::size_of::<AnimationBlockInfo<T>>(), 16);
        offset = align_offset(offset + self.foliages * std::mem::size_of::<FoliageInfo<T>>(), 16);
        offset = align_offset(offset + self.radiosity_vals * std::mem::size_of::<RadiosityValsInfo<T>>(), 16);
        debug!("infos size {}", offset);
        offset
    }
}

