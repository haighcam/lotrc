use anyhow::{Context, Result};
use rayon::prelude::*;
use log::debug;

use crate::types::GetNative;
use crate::types::{OwnedCompressedData, CompressedData, DumpCompressedData, update_crc, Crc, RefFromData, OrderedData, OrderedDataStrict, slice, hash_string, ref_slice, get_default_ref, CompressedDataRef, DumpData, DumpSlice, align_offset};
use crate::level::pak::block1::infos::InfoCounts;
#[make_platforms]
use crate::{
    level::{
        pak::{
            block1::{
                Block1RefVER, DumpBlock1VER,
                infos::DumpInfoDataVER,
                objs::DumpObjsVER,
                sub_blocks::DumpSubBlocks1VER,
                gameobjs::DumpGameObjsVER,
            },
            block2::{Block2RefVER, DumpBlock2VER, DumpSubBlocks2VER},
            animation::{AnimationsRefVER, DumpAnimationsVER}
        },
        bin::{BinRefVER},
        radiosity::DumpRadiosityVER,
    },
    types::{u32VER, U32VER, I32VER, StringsRefVER, DumpStringsVER}
};
use lotrc_proc::{make_platforms, OrderedData};

pub mod animation;
pub mod block1;
pub mod block2;

#[derive(Debug, Default, Clone, OrderedData)]
pub struct PakHeader {
    #[ordered_data(Pc)]
    pub block_a_num: u32,
    #[ordered_data(Pc)]
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
#[make_platforms]
#[derive(Debug, Default, Clone, zerocopy::Immutable, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::KnownLayout, zerocopy::Unaligned)]
#[repr(C)]
pub struct BlockAValVER {
    pub unk_0: U32VER,
    pub gamemodemask: I32VER,
    pub key: U32VER,
    pub unk_3: U32VER,
    pub unk_4: U32VER,
    pub unk_5: U32VER,
    pub unk_6: U32VER,
}
#[make_platforms]
impl OrderedData<BlockAVal> for BlockAValVER {
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

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct PakRefVER<'a> {
    pub header: &'a PakHeaderVER,
    pub strings: StringsRefVER<'a>,
    pub block1: Block1RefVER<'a>,
    pub block2: Block2RefVER<'a>,
    pub animations: AnimationsRefVER<'a>,
    pub vals_a: ref_slice<'a, BlockAValVER>,
    pub animation_data: ref_slice<'a, CompressedDataRef<'a>>
}

#[make_platforms]
impl Default for PakRefVER<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            strings: StringsRefVER::default(),
            block1: Block1RefVER::default(),
            block2: Block2RefVER::default(),
            animations: AnimationsRefVER::default(),
            vals_a: ref_slice::default(),
            animation_data: ref_slice::default()
        }
    }
}

#[make_platforms]
impl<'a> PakRefVER<'a> {
    pub fn from_data<'b: 'a, 'c: 'b>(src: &'c [u8], data: &'a mut PakCompressedData<'b>, bin: &BinRefVER<'a>) -> Result<Self> {
        let t = std::time::Instant::now();
        let header = PakHeaderVER::from_data(&src[..]).context("header")?;
        debug!("{:#?}", header);
        let strings = StringsRefVER::from_data(
            &src[header.strings_offset.get() as usize..],
            header.strings_num.get() as usize,
        )
        .context("strings")?;
        update_crc(strings.strings());
        debug!("Pak headers parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let vals_a = BlockAValVER::slice_from_data(
            &src[header.block_a_offset.get() as usize..],
            header.block_a_num.get() as usize,
        )
        .context("vals_a")?;
        debug!("Pak vals_a parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();

        data.block1 = CompressedDataRef::from_data(&src[header.block1_offset.get() as usize..], header.block1_size_comp.get() as usize, header.block1_size.get() as usize);
        data.block2 = CompressedDataRef::from_data(&src[header.block2_offset.get() as usize..], header.block2_size_comp.get() as usize, header.block2_size.get() as usize);
        
        rayon::iter::once(&mut data.block1)
            .chain(rayon::iter::once(&mut data.block2))
            .try_for_each(|data| data.decompress()).context("blocks")?;
        debug!("decompressed blocks");

        let block1 = Block1RefVER::from_data(&data.block1.get(), header, bin).context("block1")?;
        let block2 = Block2RefVER::from_data(&data.block2.get(), header, &block1.string_keys).context("block2")?;
        debug!("blocks in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        println!("animation_block_infos {:#?}", block1.infos.animation_blocks);
        data.animations = block1.infos.animation_blocks.iter().map(|info| CompressedDataRef::from_data(
            &src[info.offset.get() as usize..],
            info.size_comp.get() as usize,
            info.size.get() as usize
        )).collect::<Vec<_>>().into_boxed_slice().into();
        data.animations.par_iter_mut().try_for_each(|data| data.decompress()).context("animation blocks")?;
        debug!("animation_sizes");
        for (info, data) in block1.infos.animation_blocks.iter().zip(data.animations.iter()) {
            debug!("{} {} {} {} {}", info.offset, info.size, info.size_comp, data.data.len(), data.data_decomp.len());
        }
        debug!("Pak animation_data parsed in {}", t.elapsed().as_secs_f32());

        //let animations = AnimationsRefVER::from_data(animation_infos, &self.animations[..]).context("animations")?;
        let t = std::time::Instant::now();
        let animations = AnimationsRefVER::from_data(block1.infos.animations.clone().into(), &data.animations[..], block1.infos.animation_blocks.clone().into()).context("animations")?;
        
        debug!("Pak animations parsed in {}", t.elapsed().as_secs_f32());
        Ok(PakRefVER {
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

#[make_platforms]
pub trait DumpPakVER {
    fn vals_a_num(&self) -> usize;
    fn write_vals_a(&self, vals_a: &mut [BlockAValVER]) -> Result<()>;
    fn block1(&self) -> &impl DumpBlock1VER;
    fn block2(&self) -> &impl DumpBlock2VER;
    fn animations(&self) -> &impl DumpAnimationsVER;
    fn strings(&self) -> &impl DumpStringsVER;
    fn write_header(&self, header: &mut PakHeaderVER) -> Result<()>;
    fn dump(&self, dst: &mut DumpSlice, in_header: PakHeaderVER, block1: OwnedCompressedData, block2: OwnedCompressedData, animations: Vec<CompressedData>) -> Result<()> {
        let header = PakHeaderVER::mut_from_data(dst).context("header")?;
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
        let vals_a = BlockAValVER::mut_slice_from_data(dst, self.vals_a_num()).context("vals_a")?;
        header.block_a_num = vals_a.len().conv();
        self.write_vals_a(vals_a).context("write vals_a")?;
        dst.align(2048)?;
        debug!("pak size {}", dst.offset);
        Ok(())
    }
    fn size(&self, c: flate2::Compression) -> Result<(usize, PakHeaderVER, OwnedCompressedData, OwnedCompressedData, Vec<CompressedData<'_>>, Vec<DumpInfoDataVER<'_>>, Vec<DumpInfoDataVER<'_>>, Option<DumpInfoDataVER<'_>>)> {
        let t = std::time::Instant::now();
        let mut size = align_offset(std::mem::size_of::<PakHeaderVER>(), 4096);

        let mut header = PakHeaderVER::default();
        self.write_header(&mut header).context("write header")?;

        let block1 = self.block1();
        let block2 = self.block2();
        let animations = self.animations();
        let mut counts = InfoCounts::default();

        animations.info_counts(&mut counts);
        debug!("animation counts in {}", t.elapsed().as_secs_f32());
        let (mut block2_size, string_keys) = block2.sub_blocks().size();
        let (block1_size, type_infos) = block1.size(&mut counts, &string_keys);
        block2_size += counts.offsets * std::mem::size_of::<u32VER>();
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
        let rad_data = Some(DumpInfoDataVER {
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
        let strings_size = self.strings().size() + self.vals_a_num() * std::mem::size_of::<BlockAValVER>();
        size = align_offset(size + strings_size, 2048);
        debug!("misc sizes in {}", t.elapsed().as_secs_f32());
        debug!("pak size {}", size); 
        Ok((size, header, block1_data, block2_data, animation_blocks, model_data, texture_data, rad_data))
    }
}

#[make_platforms]
impl<'a> DumpPakVER for PakRefVER<'a> {
    fn vals_a_num(&self) -> usize {
        self.vals_a.len()
    }
    fn write_vals_a(&self, vals_a: &mut [BlockAValVER]) -> Result<()> {
        vals_a.write_from(&self.vals_a[..])
    }
    fn block1(&self) -> &impl DumpBlock1VER {
        &self.block1
    }
    fn block2(&self) -> &impl DumpBlock2VER {
        &self.block2
    }
    fn animations(&self) -> &impl DumpAnimationsVER {
        &self.animations
    }
    fn strings(&self) -> &impl DumpStringsVER {
        &self.strings
    }
    fn write_header(&self, header: &mut PakHeaderVER) -> Result<()> {
        header.write_from(self.header)
    }
}
