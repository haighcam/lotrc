use anyhow::{Context, Result};
use rayon::prelude::*;
use log::debug;

use crate::types::{OwnedCompressedData, CompressedData, DumpCompressedData, update_crc, Crc, RefFromData, OrderedData, slice, hash_string, ref_slice, get_default_ref, CompressedDataRef, DumpData, DumpSlice, align_offset};
use crate::level::pak::block1::infos::InfoCounts;
#[make_endian]
use crate::{
    level::{
        pak::{
            block1::{
                Block1Ref_XE_, DumpBlock1_XE_,
                infos::DumpInfoData_XE_,
                objs::DumpObjs_XE_,
                sub_blocks::DumpSubBlocks1_XE_,
                gameobjs::DumpGameObjs_XE_,
            },
            block2::{Block2Ref_XE_, DumpBlock2_XE_, DumpSubBlocks2_XE_},
            animation::{AnimationsRef_XE_, DumpAnimations_XE_}
        },
        bin::{BinRef_XE_},
        radiosity::DumpRadiosity_XE_,
    },
    types::{u32_XE_, U32_XE_, I32_XE_, StringsRef_XE_, DumpStrings_XE_}
};
use lotrc_proc::{make_endian, derive_ordered_data};

pub mod animation;
pub mod block1;
pub mod block2;

#[derive(Debug, Default, Clone, lotrc_proc::FromConvImpl)]
#[create_conv_trait]
pub struct PakHeader {
    pub block_a_num: u32,
    pub block_a_offset: u32,
    pub constx13: u32,
    pub version: u32,
    pub strings_offset: u32,
    pub strings_size: u32,
    pub strings_num: u32,
    pub block1_offset: u32,
    pub block1_size: u32,
    pub block1_size_comp: u32,
    pub sub_blocks1_offset: u32,
    pub block2_offset: u32,
    pub block2_size: u32,
    pub block2_size_comp: u32,
    pub sub_blocks2_offset: u32,
    pub string_keys_offset: u32,
    pub unk_16: u32,
    pub obja_size: u32,
    pub obj0_size: u32,
    pub model_info_size: u32,
    pub buffer_info_size: u32,
    pub mat1_size: u32,
    pub mat2_size: u32,
    pub mat3_size: u32,
    pub mat4_size: u32,
    pub mat_extra_size: u32,
    pub unk_26: u32,
    pub shape_info_size: u32,
    pub hk_shape_info_size: u32,
    pub hk_constraint_data_size: u32,
    pub vbuff_info_size: u32,
    pub ibuff_info_size: u32,
    pub texture_info_size: u32,
    pub animation_info_size: u32,
    pub hk_constraint_info_size: u32,
    pub effect_info_size: u32,
    pub pfield_info_size: u32,
    pub gfx_block_info_size: u32,
    pub animation_block_info_size: u32,
    pub foliage_info_size: u32,
    pub radiosity_vals_info_size: u32,
    pub unk_41: u32,
    pub obja_num: u32,
    pub obj0_num: u32,
    pub model_info_num: u32,
    pub buffer_info_num: u32,
    pub mat1_num: u32,
    pub mat2_num: u32,
    pub mat3_num: u32,
    pub mat4_num: u32,
    pub mat_extra_num: u32,
    pub unk_51: u32,
    pub shape_info_num: u32,
    pub hk_shape_info_num: u32,
    pub hk_constraint_data_num: u32,
    pub vbuff_info_num: u32,
    pub ibuff_info_num: u32,
    pub texture_info_num: u32,
    pub animation_info_num: u32,
    pub hk_constraint_info_num: u32,
    pub effect_info_num: u32,
    pub pfield_info_num: u32,
    pub gfx_block_info_num: u32,
    pub animation_block_info_num: u32,
    pub foliage_info_num: u32,
    pub radiosity_vals_info_num: u32,
    pub unk_66: u32,
    pub obja_offset: u32,
    pub obj0_offset: u32,
    pub model_info_offset: u32, // max loaded is 0x400
    pub buffer_info_offset: u32,
    pub mat1_offset: u32,
    pub mat2_offset: u32,
    pub mat3_offset: u32,
    pub mat4_offset: u32,
    pub mat_extra_offset: u32,
    pub unk_76: u32,
    pub shape_info_offset: u32,
    pub hk_shape_info_offset: u32,
    pub hk_constraint_data_offset: u32,
    pub vbuff_info_offset: u32,
    pub ibuff_info_offset: u32,
    pub texture_info_offset: u32, // max loaded is 0x800
    pub animation_info_offset: u32,
    pub hk_constraint_info_offset: u32,
    pub effect_info_offset: u32,
    pub pfield_info_offset: u32,
    pub gfx_block_info_offset: u32, // max loaded is 0x40
    pub animation_block_info_offset: u32,
    pub foliage_info_offset: u32,
    pub radiosity_vals_info_offset: u32,
    pub unk_91: u32,
    pub unk_92: u32,
    pub unk_93: u32,
    pub unk_94: u32,
    pub unk_95: u32,
    pub unk_96: u32,
    pub unk_97: u32,
    pub unk_98: u32,
    pub unk_99: u32,
    pub unk_100: u32,
    pub unk_101: u32,
    pub unk_102: u32,
    pub unk_103: u32,
    pub unk_104: u32,
    pub unk_105: u32,
    pub unk_106: u32,
    pub unk_107: u32,
    pub unk_108: u32,
    pub unk_109: u32,
    pub unk_110: u32,
    pub unk_111: u32,
    pub unk_112: u32,
    pub unk_113: u32,
    pub unk_114: u32,
    pub unk_115: u32,
    pub block2_offsets_num: u32,
    pub block2_offsets_offset: u32,
}

#[make_endian]
#[derive(Debug, Default, Clone, lotrc_proc::IntoConvImpl, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::FromBytes)]
#[conv_base(PakHeader)]
pub struct PakHeader_XE_ {
    pub block_a_num: u32LE,
    pub block_a_offset: u32LE,
    pub constx13: u32_XE_,
    pub version: u32_XE_,
    pub strings_offset: u32_XE_,
    pub strings_size: u32_XE_,
    pub strings_num: u32_XE_,
    pub block1_offset: u32_XE_,
    pub block1_size: u32_XE_,
    pub block1_size_comp: u32_XE_,
    pub sub_blocks1_offset: u32_XE_,
    pub block2_offset: u32_XE_,
    pub block2_size: u32_XE_,
    pub block2_size_comp: u32_XE_,
    pub sub_blocks2_offset: u32_XE_,
    pub string_keys_offset: u32_XE_,
    pub unk_16: u32_XE_,
    pub obja_size: u32_XE_,
    pub obj0_size: u32_XE_,
    pub model_info_size: u32_XE_,
    pub buffer_info_size: u32_XE_,
    pub mat1_size: u32_XE_,
    pub mat2_size: u32_XE_,
    pub mat3_size: u32_XE_,
    pub mat4_size: u32_XE_,
    pub mat_extra_size: u32_XE_,
    pub unk_26: u32_XE_,
    pub shape_info_size: u32_XE_,
    pub hk_shape_info_size: u32_XE_,
    pub hk_constraint_data_size: u32_XE_,
    pub vbuff_info_size: u32_XE_,
    pub ibuff_info_size: u32_XE_,
    pub texture_info_size: u32_XE_,
    pub animation_info_size: u32_XE_,
    pub hk_constraint_info_size: u32_XE_,
    pub effect_info_size: u32_XE_,
    pub pfield_info_size: u32_XE_,
    pub gfx_block_info_size: u32_XE_,
    pub animation_block_info_size: u32_XE_,
    pub foliage_info_size: u32_XE_,
    pub radiosity_vals_info_size: u32_XE_,
    pub unk_41: u32_XE_,
    pub obja_num: u32_XE_,
    pub obj0_num: u32_XE_,
    pub model_info_num: u32_XE_,
    pub buffer_info_num: u32_XE_,
    pub mat1_num: u32_XE_,
    pub mat2_num: u32_XE_,
    pub mat3_num: u32_XE_,
    pub mat4_num: u32_XE_,
    pub mat_extra_num: u32_XE_,
    pub unk_51: u32_XE_,
    pub shape_info_num: u32_XE_,
    pub hk_shape_info_num: u32_XE_,
    pub hk_constraint_data_num: u32_XE_,
    pub vbuff_info_num: u32_XE_,
    pub ibuff_info_num: u32_XE_,
    pub texture_info_num: u32_XE_,
    pub animation_info_num: u32_XE_,
    pub hk_constraint_info_num: u32_XE_,
    pub effect_info_num: u32_XE_,
    pub pfield_info_num: u32_XE_,
    pub gfx_block_info_num: u32_XE_,
    pub animation_block_info_num: u32_XE_,
    pub foliage_info_num: u32_XE_,
    pub radiosity_vals_info_num: u32_XE_,
    pub unk_66: u32_XE_,
    pub obja_offset: u32_XE_,
    pub obj0_offset: u32_XE_,
    pub model_info_offset: u32_XE_, // max loaded is 0x400
    pub buffer_info_offset: u32_XE_,
    pub mat1_offset: u32_XE_,
    pub mat2_offset: u32_XE_,
    pub mat3_offset: u32_XE_,
    pub mat4_offset: u32_XE_,
    pub mat_extra_offset: u32_XE_,
    pub unk_76: u32_XE_,
    pub shape_info_offset: u32_XE_,
    pub hk_shape_info_offset: u32_XE_,
    pub hk_constraint_data_offset: u32_XE_,
    pub vbuff_info_offset: u32_XE_,
    pub ibuff_info_offset: u32_XE_,
    pub texture_info_offset: u32_XE_, // max loaded is 0x800
    pub animation_info_offset: u32_XE_,
    pub hk_constraint_info_offset: u32_XE_,
    pub effect_info_offset: u32_XE_,
    pub pfield_info_offset: u32_XE_,
    pub gfx_block_info_offset: u32_XE_, // max loaded is 0x40
    pub animation_block_info_offset: u32_XE_,
    pub foliage_info_offset: u32_XE_,
    pub radiosity_vals_info_offset: u32_XE_,
    pub unk_91: u32_XE_,
    pub unk_92: u32_XE_,
    pub unk_93: u32_XE_,
    pub unk_94: u32_XE_,
    pub unk_95: u32_XE_,
    pub unk_96: u32_XE_,
    pub unk_97: u32_XE_,
    pub unk_98: u32_XE_,
    pub unk_99: u32_XE_,
    pub unk_100: u32_XE_,
    pub unk_101: u32_XE_,
    pub unk_102: u32_XE_,
    pub unk_103: u32_XE_,
    pub unk_104: u32_XE_,
    pub unk_105: u32_XE_,
    pub unk_106: u32_XE_,
    pub unk_107: u32_XE_,
    pub unk_108: u32_XE_,
    pub unk_109: u32_XE_,
    pub unk_110: u32_XE_,
    pub unk_111: u32_XE_,
    pub unk_112: u32_XE_,
    pub unk_113: u32_XE_,
    pub unk_114: u32_XE_,
    pub unk_115: u32_XE_,
    pub block2_offsets_num: u32_XE_,
    pub block2_offsets_offset: u32_XE_,
}

// this is unaligned, likely due to having a packed c repr and being unused
#[derive(Debug, Default, Clone)]
pub struct BlockAVal {
    pub unk_0: u32,
    pub gamemodemask: i32,
    pub key: Crc,
    pub unk_3: u32,
    pub unk_4: u32,
    pub unk_5: u32,
    pub unk_6: u32,
}
#[make_endian]
#[derive(Debug, Default, Clone, zerocopy::Immutable, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::KnownLayout, zerocopy::Unaligned)]
#[repr(C)]
pub struct BlockAVal_XE_ {
    pub unk_0: U32_XE_,
    pub gamemodemask: I32_XE_,
    pub key: U32_XE_,
    pub unk_3: U32_XE_,
    pub unk_4: U32_XE_,
    pub unk_5: U32_XE_,
    pub unk_6: U32_XE_,
}
#[make_endian]
impl OrderedData<BlockAVal> for BlockAVal_XE_ {
    fn conv(&self) -> BlockAVal {
        BlockAVal {
            unk_0: self.unk_0.conv(),
            gamemodemask: self.gamemodemask.conv(),
            key: self.key.conv(),
            unk_3: self.unk_3.conv(),
            unk_4: self.unk_4.conv(),
            unk_5: self.unk_5.conv(),
            unk_6: self.unk_6.conv(),
        }
    }
}

#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct PakCompressedData<'a> {
    pub block1: CompressedDataRef<'a>,
    pub block2: CompressedDataRef<'a>,
    pub animations: slice<CompressedDataRef<'a>>
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct PakRef_XE_<'a> {
    pub header: &'a PakHeader_XE_,
    pub strings: StringsRef_XE_<'a>,
    pub block1: Block1Ref_XE_<'a>,
    pub block2: Block2Ref_XE_<'a>,
    pub animations: AnimationsRef_XE_<'a>,
    pub vals_a: ref_slice<'a, BlockAVal_XE_>,
    pub animation_data: ref_slice<'a, CompressedDataRef<'a>>
}

#[make_endian]
impl Default for PakRef_XE_<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            strings: StringsRef_XE_::default(),
            block1: Block1Ref_XE_::default(),
            block2: Block2Ref_XE_::default(),
            animations: AnimationsRef_XE_::default(),
            vals_a: ref_slice::default(),
            animation_data: ref_slice::default()
        }
    }
}

#[make_endian]
impl<'a> PakRef_XE_<'a> {
    pub fn from_data<'b: 'a, 'c: 'b>(src: &'c [u8], data: &'a mut PakCompressedData<'b>, bin: &BinRef_XE_<'a>) -> Result<Self> {
        let t = std::time::Instant::now();
        let header = PakHeader_XE_::from_data(&src[..]).context("header")?;
        debug!("{:#?}", header);
        let strings = StringsRef_XE_::from_data(
            &src[header.strings_offset.conv()..],
            header.strings_num.conv(),
        )
        .context("strings")?;
        update_crc(strings.strings());
        debug!("Pak headers parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let vals_a = BlockAVal_XE_::slice_from_data(
            &src[header.block_a_offset.conv()..],
            header.block_a_num.conv(),
        )
        .context("vals_a")?;
        debug!("Pak vals_a parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();

        data.block1 = CompressedDataRef::from_data(&src[header.block1_offset.conv()..], header.block1_size_comp.conv(), header.block1_size.conv());
        data.block2 = CompressedDataRef::from_data(&src[header.block2_offset.conv()..], header.block2_size_comp.conv(), header.block2_size.conv());
        
        rayon::iter::once(&mut data.block1)
            .chain(rayon::iter::once(&mut data.block2))
            .try_for_each(|data| data.decompress()).context("blocks")?;
        debug!("decompressed blocks");

        let block1 = Block1Ref_XE_::from_data(&data.block1.get(), header, bin).context("block1")?;
        let block2 = Block2Ref_XE_::from_data(&data.block2.get(), header, &block1.string_keys).context("block2")?;
        debug!("blocks in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        println!("animation_block_infos {:#?}", block1.infos.animation_blocks);
        data.animations = block1.infos.animation_blocks.iter().map(|info| CompressedDataRef::from_data(
            &src[info.offset.conv()..],
            info.size_comp.conv(),
            info.size.conv()
        )).collect::<Vec<_>>().into_boxed_slice().into();
        data.animations.par_iter_mut().try_for_each(|data| data.decompress()).context("animation blocks")?;
        debug!("animation_sizes");
        for (info, data) in block1.infos.animation_blocks.iter().zip(data.animations.iter()) {
            debug!("{} {} {} {} {}", info.offset, info.size, info.size_comp, data.data.len(), data.data_decomp.len());
        }
        debug!("Pak animation_data parsed in {}", t.elapsed().as_secs_f32());

        //let animations = AnimationsRef_XE_::from_data(animation_infos, &self.animations[..]).context("animations")?;
        let t = std::time::Instant::now();
        let animations = AnimationsRef_XE_::from_data(block1.infos.animations.clone().into(), &data.animations[..], block1.infos.animation_blocks.clone().into()).context("animations")?;
        
        debug!("Pak animations parsed in {}", t.elapsed().as_secs_f32());
        Ok(PakRef_XE_ {
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

#[make_endian]
pub trait DumpPak_XE_ {
    fn vals_a_num(&self) -> usize;
    fn write_vals_a(&self, vals_a: &mut [BlockAVal_XE_]) -> Result<()>;
    fn block1(&self) -> &impl DumpBlock1_XE_;
    fn block2(&self) -> &impl DumpBlock2_XE_;
    fn animations(&self) -> &impl DumpAnimations_XE_;
    fn strings(&self) -> &impl DumpStrings_XE_;
    fn write_header(&self, header: &mut PakHeader_XE_) -> Result<()>;
    fn dump(&self, dst: &mut DumpSlice, in_header: PakHeader_XE_, block1: OwnedCompressedData, block2: OwnedCompressedData, animations: Vec<CompressedData>) -> Result<()> {
        let header = PakHeader_XE_::mut_from_data(dst).context("header")?;
        header.write_from(&in_header).context("write header")?;

        for (i, animation) in animations.into_iter().enumerate() {
            dst.align(4096)?;
            debug!("animation block {} offset {}", i, dst.offset);
            animation.dump_into(dst).with_context(|| format!("animation block {}", i))?;
        }

        dst.align(4096)?;
        debug!("block1 offset {}", dst.offset);
        header.block1_offset = dst.offset.conv();
        header.block1_size = block1.size().conv();
        header.block1_size_comp = block1.size_comp().conv();
        block1.dump_into(dst).context("block1")?;
        dst.align(4096)?;
        debug!("block2 offset {}", dst.offset);
        header.block2_offset = dst.offset.conv();
        header.block2_size = block2.size().conv();
        header.block2_size_comp = block2.size_comp().conv();
        block2.dump_into(dst).context("block2")?;

        dst.align(4096)?;
        debug!("strings offset {}", dst.offset);
        let start = dst.offset;
        let strings = self.strings();
        header.strings_offset = dst.offset.conv();
        header.strings_num = strings.num_strings().conv();
        // TODO make sure that strings contains all relevant dumped crc values
        strings.dump_into(dst).context("strings")?;
        header.strings_size = (dst.offset - start).conv();

        header.block_a_offset = dst.offset.conv();
        let vals_a = BlockAVal_XE_::mut_slice_from_data(dst, self.vals_a_num()).context("vals_a")?;
        header.block_a_num = vals_a.len().conv();
        self.write_vals_a(vals_a).context("write vals_a")?;
        dst.align(2048)?;
        debug!("pak size {}", dst.offset);
        Ok(())
    }
    fn size(&self, c: flate2::Compression) -> Result<(usize, PakHeader_XE_, OwnedCompressedData, OwnedCompressedData, Vec<CompressedData<'_>>, Vec<DumpInfoData_XE_<'_>>, Vec<DumpInfoData_XE_<'_>>, Option<DumpInfoData_XE_<'_>>)> {
        let t = std::time::Instant::now();
        let mut size = align_offset(std::mem::size_of::<PakHeader_XE_>(), 4096);

        let mut header = PakHeader_XE_::default();
        self.write_header(&mut header).context("write header")?;

        let block1 = self.block1();
        let block2 = self.block2();
        let animations = self.animations();
        let mut counts = InfoCounts::default();

        animations.info_counts(&mut counts);
        debug!("animation counts in {}", t.elapsed().as_secs_f32());
        let (mut block2_size, string_keys) = block2.sub_blocks().size();
        let (block1_size, type_infos) = block1.size(&mut counts, &string_keys);
        block2_size += counts.offsets * std::mem::size_of::<u32_XE_>();
        let mut block1_data = OwnedCompressedData::with_capacity(block1_size);
        let mut block2_data = OwnedCompressedData::with_capacity(block2_size);
        debug!("blocks size in {}", t.elapsed().as_secs_f32());
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
            debug!("animation block {} offset {}", info.key, size);
            info.offset = size.conv();
            info.size_comp = block.size_comp().conv(); 
            info.size = block.size().conv();
            size = align_offset(size + block.size_comp(), 4096);
        }
        
        let model_data = infos.model_data;
        let texture_data = infos.texture_data;
        let radiosity = block1.objs().radiosity();
        let radiosity_name = hash_string(b"_radiosity", Some(block1.sub_blocks().level().level_name()?));
        let rad_data = Some(DumpInfoData_XE_ {
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
        let strings_size = self.strings().size() + self.vals_a_num() * std::mem::size_of::<BlockAVal_XE_>();
        size = align_offset(size + strings_size, 2048);
        debug!("misc sizes in {}", t.elapsed().as_secs_f32());
        debug!("pak size {}", size); 
        Ok((size, header, block1_data, block2_data, animation_blocks, model_data, texture_data, rad_data))
    }
}

#[make_endian]
impl<'a> DumpPak_XE_ for PakRef_XE_<'a> {
    fn vals_a_num(&self) -> usize {
        self.vals_a.len()
    }
    fn write_vals_a(&self, vals_a: &mut [BlockAVal_XE_]) -> Result<()> {
        vals_a.write_from(&self.vals_a[..])
    }
    fn block1(&self) -> &impl DumpBlock1_XE_ {
        &self.block1
    }
    fn block2(&self) -> &impl DumpBlock2_XE_ {
        &self.block2
    }
    fn animations(&self) -> &impl DumpAnimations_XE_ {
        &self.animations
    }
    fn strings(&self) -> &impl DumpStrings_XE_ {
        &self.strings
    }
    fn write_header(&self, header: &mut PakHeader_XE_) -> Result<()> {
        header.write_from(self.header)
    }
}
