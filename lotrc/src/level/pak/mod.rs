use anyhow::{Context, Result};
use rayon::prelude::*;
use log::debug;
use indexmap::IndexMap;

use crate::types::{OwnedCompressedData, update_crc, ReadData, slice, hash_string, ref_slice, get_default_ref, CompressedDataRef, DumpSlice, align_offset, BaseTypes, DumpCompressedData};
use crate::level::pak::block1::infos::InfoCounts;
use lotrc_proc::{derive_pod};

pub mod animation;
pub mod block1;
pub mod block2;

use crate::{
    level::{
        pak::{
            block1::{
                Block1Ref, DumpBlock1,
                infos::DumpInfoData,
                objs::DumpObjs,
                sub_blocks::DumpSubBlocks1,
                gameobjs::DumpGameObjs,
            },
            block2::{Block2Ref, DumpBlock2, DumpSubBlocks2},
            animation::{AnimationsRef, DumpAnimations},
        },
        bin::BinRef,
        radiosity::DumpRadiosity,

    },
    types::{
        StringsRef, CompressedData, DumpStrings 
    }
};

#[derive_pod]
pub struct PakHeader<T: BaseTypes> {
    pub block_a_num: T::u32LE,
    pub block_a_offset: T::u32LE,
    pub constx13: T::u32,
    pub version: T::u32,
    pub strings_offset: T::u32,
    pub strings_size: T::u32,
    pub strings_num: T::u32,
    pub block1_offset: T::u32,
    pub block1_size: T::u32,
    pub block1_size_comp: T::u32,
    pub sub_blocks1_offset: T::u32,
    pub block2_offset: T::u32,
    pub block2_size: T::u32,
    pub block2_size_comp: T::u32,
    pub sub_blocks2_offset: T::u32,
    pub string_keys_offset: T::u32,
    pub unk_16: T::u32,
    pub obja_size: T::u32,
    pub obj0_size: T::u32,
    pub model_info_size: T::u32,
    pub buffer_info_size: T::u32,
    pub mat1_size: T::u32,
    pub mat2_size: T::u32,
    pub mat3_size: T::u32,
    pub mat4_size: T::u32,
    pub mat_extra_size: T::u32,
    pub unk_26: T::u32,
    pub shape_info_size: T::u32,
    pub hk_shape_info_size: T::u32,
    pub hk_constraint_data_size: T::u32,
    pub vbuff_info_size: T::u32,
    pub ibuff_info_size: T::u32,
    pub texture_info_size: T::u32,
    pub animation_info_size: T::u32,
    pub hk_constraint_info_size: T::u32,
    pub effect_info_size: T::u32,
    pub pfield_info_size: T::u32,
    pub gfx_block_info_size: T::u32,
    pub animation_block_info_size: T::u32,
    pub foliage_info_size: T::u32,
    pub radiosity_vals_info_size: T::u32,
    pub unk_41: T::u32,
    pub obja_num: T::u32,
    pub obj0_num: T::u32,
    pub model_info_num: T::u32,
    pub buffer_info_num: T::u32,
    pub mat1_num: T::u32,
    pub mat2_num: T::u32,
    pub mat3_num: T::u32,
    pub mat4_num: T::u32,
    pub mat_extra_num: T::u32,
    pub unk_51: T::u32,
    pub shape_info_num: T::u32,
    pub hk_shape_info_num: T::u32,
    pub hk_constraint_data_num: T::u32,
    pub vbuff_info_num: T::u32,
    pub ibuff_info_num: T::u32,
    pub texture_info_num: T::u32,
    pub animation_info_num: T::u32,
    pub hk_constraint_info_num: T::u32,
    pub effect_info_num: T::u32,
    pub pfield_info_num: T::u32,
    pub gfx_block_info_num: T::u32,
    pub animation_block_info_num: T::u32,
    pub foliage_info_num: T::u32,
    pub radiosity_vals_info_num: T::u32,
    pub unk_66: T::u32,
    pub obja_offset: T::u32,
    pub obj0_offset: T::u32,
    pub model_info_offset: T::u32, // max loaded is 0x400
    pub buffer_info_offset: T::u32,
    pub mat1_offset: T::u32,
    pub mat2_offset: T::u32,
    pub mat3_offset: T::u32,
    pub mat4_offset: T::u32,
    pub mat_extra_offset: T::u32,
    pub unk_76: T::u32,
    pub shape_info_offset: T::u32,
    pub hk_shape_info_offset: T::u32,
    pub hk_constraint_data_offset: T::u32,
    pub vbuff_info_offset: T::u32,
    pub ibuff_info_offset: T::u32,
    pub texture_info_offset: T::u32, // max loaded is 0x800
    pub animation_info_offset: T::u32,
    pub hk_constraint_info_offset: T::u32,
    pub effect_info_offset: T::u32,
    pub pfield_info_offset: T::u32,
    pub gfx_block_info_offset: T::u32, // max loaded is 0x40
    pub animation_block_info_offset: T::u32,
    pub foliage_info_offset: T::u32,
    pub radiosity_vals_info_offset: T::u32,
    pub unk_91: T::u32,
    pub unk_92: T::u32,
    pub unk_93: T::u32,
    pub unk_94: T::u32,
    pub unk_95: T::u32,
    pub unk_96: T::u32,
    pub unk_97: T::u32,
    pub unk_98: T::u32,
    pub unk_99: T::u32,
    pub unk_100: T::u32,
    pub unk_101: T::u32,
    pub unk_102: T::u32,
    pub unk_103: T::u32,
    pub unk_104: T::u32,
    pub unk_105: T::u32,
    pub unk_106: T::u32,
    pub unk_107: T::u32,
    pub unk_108: T::u32,
    pub unk_109: T::u32,
    pub unk_110: T::u32,
    pub unk_111: T::u32,
    pub unk_112: T::u32,
    pub unk_113: T::u32,
    pub unk_114: T::u32,
    pub unk_115: T::u32,
    pub block2_offsets_num: T::u32,
    pub block2_offsets_offset: T::u32,
}

#[derive_pod]
// this is unaligned, likely due to having a packed c repr and being unused
pub struct BlockAVal<T: BaseTypes> {
    pub unk_0: T::U32,
    pub gamemodemask: T::I32,
    pub key: T::U32,
    pub unk_3: T::U32,
    pub unk_4: T::U32,
    pub unk_5: T::U32,
    pub unk_6: T::U32,
}

#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct PakCompressedData<'a> {
    pub block1: CompressedDataRef<'a>,
    pub block2: CompressedDataRef<'a>,
    pub animations: slice<CompressedDataRef<'a>>
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct PakRef<'a, T: BaseTypes> {
    pub header: &'a PakHeader<T>,
    pub strings: StringsRef<'a>,
    pub block1: Block1Ref<'a, T>,
    pub block2: Block2Ref<'a, T>,
    pub animations: AnimationsRef<'a, T>,
    pub vals_a: ref_slice<'a, BlockAVal<T>>,
    pub animation_data: ref_slice<'a, CompressedDataRef<'a>>
}

impl<T: BaseTypes> Default for PakRef<'_, T> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            strings: StringsRef::default(),
            block1: Block1Ref::default(),
            block2: Block2Ref::default(),
            animations: AnimationsRef::default(),
            vals_a: ref_slice::default(),
            animation_data: ref_slice::default()
        }
    }
}

impl<'a, T: BaseTypes> PakRef<'a, T> {
    pub fn from_data<'b: 'a, 'c: 'b>(src: &'c [u8], data: &'a mut PakCompressedData<'b>, bin: &BinRef<'a, T>) -> Result<Self> {
        let t = std::time::Instant::now();
        let header = PakHeader::<T>::from_data(&src[..]).context("header")?;
        debug!("{:#?}", header);
        let strings = StringsRef::from_data::<T>(
            &src[header.strings_offset.into() as usize..],
            header.strings_num.into() as usize,
        )
        .context("strings")?;
        update_crc(strings.strings());
        debug!("Pak headers parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let vals_a = BlockAVal::slice_from_data(
            &src[header.block_a_offset.into() as usize..],
            header.block_a_num.into() as usize,
        )
        .context("vals_a")?;
        debug!("Pak vals_a parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();

        data.block1 = CompressedDataRef::from_data(&src[header.block1_offset.into() as usize..], header.block1_size_comp.into() as usize, header.block1_size.into() as usize).context("block1")?;
        data.block2 = CompressedDataRef::from_data(&src[header.block2_offset.into() as usize..], header.block2_size_comp.into() as usize, header.block2_size.into() as usize).context("block1")?;
        
        rayon::iter::once(&mut data.block1)
            .chain(rayon::iter::once(&mut data.block2))
            .try_for_each(|data| data.decompress()).context("blocks")?;
        debug!("decompressed blocks");

        let block1 = Block1Ref::from_data(&data.block1.get(), header, bin).context("block1")?;
        let block2 = Block2Ref::from_data(&data.block2.get(), header, &block1.string_keys).context("block2")?;
        debug!("blocks in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        println!("animation_block_infos {:#?}", block1.infos.animation_blocks);
        data.animations = block1.infos.animation_blocks.iter().map(|info| CompressedDataRef::from_data(
            &src[info.offset.into() as usize..],
            info.size_comp.into() as usize,
            info.size.into() as usize
        ).with_context(|| format!("animation_block {}", info.key.val.into()))).collect::<Result<Vec<_>, _>>()?.into_boxed_slice().into();
        data.animations.par_iter_mut().try_for_each(|data| data.decompress()).context("animation blocks")?;
        debug!("animation_sizes");
        for (info, data) in block1.infos.animation_blocks.iter().zip(data.animations.iter()) {
            debug!("{} {} {} {} {}", info.offset.into(), info.size.into(), info.size_comp.into(), data.data.len(), data.data_decomp.len());
        }
        debug!("Pak animation_data parsed in {}", t.elapsed().as_secs_f32());

        //let animations = AnimationsRef_XE_::from_data(animation_infos, &self.animations[..]).context("animations")?;
        let t = std::time::Instant::now();
        let animations = AnimationsRef::from_data(block1.infos.animations, &data.animations[..], block1.infos.animation_blocks).context("animations")?;
        
        debug!("Pak animations parsed in {}", t.elapsed().as_secs_f32());
        Ok(Self {
            header,
            strings,
            block1,
            block2,
            animations,
            vals_a: vals_a.into(),
            animation_data: &data.animations[..]
        })

    }
}

pub trait DumpPak<T: BaseTypes> {
    fn vals_a_num(&self) -> usize;
    fn write_vals_a(&self, vals_a: &mut [BlockAVal<T>]) -> Result<()>;
    fn block1(&self) -> &impl DumpBlock1<T>;
    fn block2(&self) -> &impl DumpBlock2<T>;
    fn animations(&self) -> &impl DumpAnimations<T>;
    fn strings(&self) -> &impl DumpStrings<T>;
    fn write_header(&self, header: &mut PakHeader<T>) -> Result<()>;
    fn dump(&self, dst: &mut DumpSlice, in_header: PakHeader<T>, block1: OwnedCompressedData, block2: OwnedCompressedData, animations: Vec<CompressedData>) -> Result<()> {
        let header = PakHeader::<T>::mut_from_data(dst).context("header")?;
        *header = in_header;

        for (i, animation) in animations.into_iter().enumerate() {
            dst.align(4096)?;
            debug!("animation block {} offset {}", i, dst.offset);
            animation.dump_into(dst).with_context(|| format!("animation block {}", i))?;
        }

        dst.align(4096)?;
        debug!("block1 offset {}", dst.offset);
        header.block1_offset = (dst.offset as u32).into();
        header.block1_size = (block1.size() as u32).into();
        header.block1_size_comp = (block1.size_comp() as u32).into();
        block1.dump_into(dst).context("block1")?;
        dst.align(4096)?;
        debug!("block2 offset {}", dst.offset);
        header.block2_offset = (dst.offset as u32).into();
        header.block2_size = (block2.size() as u32).into();
        header.block2_size_comp = (block2.size_comp() as u32).into();
        block2.dump_into(dst).context("block2")?;

        dst.align(4096)?;
        debug!("strings offset {}", dst.offset);
        let start = dst.offset;
        let strings = self.strings();
        header.strings_offset = (dst.offset as u32).into();
        header.strings_num = (strings.num_strings() as u32).into();
        // TODO make sure that strings contains all relevant dumped crc values
        strings.dump_into(dst).context("strings")?;
        header.strings_size = ((dst.offset - start) as u32).into();

        header.block_a_offset = (dst.offset as u32).into();
        let vals_a = BlockAVal::mut_slice_from_data(dst, self.vals_a_num()).context("vals_a")?;
        header.block_a_num = (vals_a.len() as u32).into();
        self.write_vals_a(vals_a).context("write vals_a")?;
        dst.align(2048)?;
        debug!("pak size {}", dst.offset);
        Ok(())
    }
    fn size(&self, c: flate2::Compression) -> Result<(usize, PakHeader<T>, OwnedCompressedData, OwnedCompressedData, Vec<CompressedData<'_>>, Vec<DumpInfoData<'_, T>>, IndexMap<u32, DumpInfoData<'_, T>>, Option<DumpInfoData<'_, T>>)> {
        let t = std::time::Instant::now();
        let mut size = align_offset(std::mem::size_of::<PakHeader<T>>(), 4096);

        let mut header = PakHeader::<T>::default();
        self.write_header(&mut header).context("write header")?;

        let block1 = self.block1();
        let block2 = self.block2();
        let animations = self.animations();
        let mut counts = InfoCounts::default();

        animations.info_counts(&mut counts);
        debug!("animation counts in {}", t.elapsed().as_secs_f32());
        let (mut block2_size, string_keys) = block2.sub_blocks().size();
        let (block1_size, type_infos) = block1.size(&mut counts, &string_keys);
        block2_size += counts.offsets * std::mem::size_of::<T::u32>();
        let mut block1_data = OwnedCompressedData::with_capacity(block1_size);
        let mut block2_data = OwnedCompressedData::with_capacity(block2_size);
        debug!("blocks size in {}", t.elapsed().as_secs_f32());
        debug!("block2 size {}", block2_size);
        let offsets = {
            let mut dst = block2_data.dump_slice();
            block2.dump(&mut dst, counts.offsets, &string_keys, &mut header).context("block2")
        }?;
        debug!("dumped block2 in {}", t.elapsed().as_secs_f32());
 
        let (mut infos, rad_data) = {
            let mut dst = block1_data.dump_slice();
            block1.dump(&mut dst, offsets, &counts, &mut header, (&type_infos.0, &type_infos.1), &string_keys).context("block1")?
        };
        debug!("dumped block1 in {}", t.elapsed().as_secs_f32());

        let mut animation_blocks = animations.dump(&mut infos).context("animations")?;
        debug!("dumped animations in {}", t.elapsed().as_secs_f32());
        
        animation_blocks.par_iter_mut()
            .try_for_each(|x| x.compress(true, c))
            .context("compressed data")?;
        debug!("compressed animations in {}", t.elapsed().as_secs_f32());

        size = align_offset(size, 4096);
        for (block, info) in animation_blocks.iter().zip(infos.animation_blocks.take()) {
            debug!("animation block {} offset {}", info.key.val.into(), size);
            info.offset = (size as u32).into();
            info.size_comp = (block.size_comp() as u32).into(); 
            info.size = (block.size() as u32).into();
            size = align_offset(size + block.size_comp(), 4096);
        }
        
        let model_data = infos.model_data;
        let texture_data = infos.texture_data;
        let radiosity = block1.objs().radiosity();
        let radiosity_name = hash_string(b"_radiosity", Some(block1.sub_blocks().level().level_name()?));
        let rad_data = Some(DumpInfoData {
            key: radiosity_name.into(),
            kind: radiosity.usage(), 
            data: rad_data
        });
        
        rayon::iter::once(&mut block1_data)
            .chain(rayon::iter::once(&mut block2_data))
            .try_for_each(|x| x.compress(false, c))
            .context("compressed blocks")?;
        debug!("compressed blocks in {}", t.elapsed().as_secs_f32());
        debug!("block1 offset {}", size);
        size = align_offset(size + block1_data.size_comp(), 4096);
        debug!("block2 offset {}", size);
        size = align_offset(size + block2_data.size_comp(), 4096);
        debug!("strings offset {}", size);
        let strings_size = self.strings().size() + self.vals_a_num() * std::mem::size_of::<BlockAVal<T>>();
        size = align_offset(size + strings_size, 2048);
        debug!("misc sizes in {}", t.elapsed().as_secs_f32());
        debug!("pak size {}", size); 
        Ok((size, header, block1_data, block2_data, animation_blocks, model_data, texture_data, rad_data))
    }
}

impl<'a, T: BaseTypes> DumpPak<T> for PakRef<'a, T> {
    fn vals_a_num(&self) -> usize {
        self.vals_a.len()
    }
    fn write_vals_a(&self, vals_a: &mut [BlockAVal<T>]) -> Result<()> {
        vals_a.copy_from_slice(&self.vals_a[..]);
        Ok(())
    }
    fn block1(&self) -> &impl DumpBlock1<T> {
        &self.block1
    }
    fn block2(&self) -> &impl DumpBlock2<T> {
        &self.block2
    }
    fn animations(&self) -> &impl DumpAnimations<T> {
        &self.animations
    }
    fn strings(&self) -> &impl DumpStrings<T> {
        &self.strings
    }
    fn write_header(&self, header: &mut PakHeader<T>) -> Result<()> {
        *header = *self.header;
        Ok(())
    }
}
