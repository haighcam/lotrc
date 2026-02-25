use anyhow::{anyhow, Context, Result};
use std::ptr::NonNull;
use rayon::prelude::*;
use std::cell::OnceCell;

#[cfg(not(feature = "ffi"))]
use crate::types::{GetNative, AsSlice};
use crate::sub_blocks;
use crate::types::{self, decompress_block, update_crc, Crc, RefFromData, OrderedData, OrderedDataStrict, BufType, AlignedBuf, box_slice, hash_string, slice, decompress_block_into, get_default_ref, CompressedDataRefAlt, DumpData, DumpSlice, CompressedBlock, align_offset, DumpCompressedData};
use crate::level::pak::objs::InfoCounts;
use crate::sub_blocks::gameobjs::TypeInfos;
#[make_platforms]
use crate::{
    level::{
        pak::{
            objs::{ObjsRefVER, DumpObjsVER, DumpInfosVER},
            animation::{AnimationsVER, AnimationsRawVER, AnimationsRefVER, AnimationInfoVER, DumpAnimationsVER}
        },
        bin::{BinVER, BinRefVER},
        radiosity::DumpRadiosityVER,
    },
    sub_blocks::{SubBlocksRefVER, SubBlockRefVER, DumpSubBlocksVER},
    types::{u32VER, U32VER, I32VER, StringKeysRefVER, StringsRefVER, DumpStringsVER, DumpStringKeysVER}
};
use lotrc_proc::{make_platforms, OrderedData};

pub mod animation;
pub mod objs;

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
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
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

#[make_platforms]
#[derive(Default)]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct Block1RefVER<'a> {
    pub objs: ObjsRefVER<'a>,
    pub sub_blocks: SubBlocksRefVER<'a>,
    pub string_keys: StringKeysRefVER<'a>
}

#[make_platforms]
impl<'a> Block1RefVER<'a> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeaderVER, bin: &BinRefVER<'a>) -> Result<Self> {
        let t = std::time::Instant::now();
        let sub_blocks = SubBlocksRefVER::from_data(
            &src[pak_header.sub_blocks1_offset.get() as usize..]
        )
        .context("sub_blocks")?;
        println!("Block1 sub_blocks parsed in {}", t.elapsed().as_secs_f32());

        let level = sub_blocks.blocks
            .get(&hash_string(b"level", None))
            .ok_or(anyhow!("level block missing"))
            .and_then(|x| match x {
                SubBlockRefVER::Level(val) => Ok(val),
                _ => Err(anyhow!("level block wrong format"))
            })?;
        let field = level.objs.values()
            .find(|v| v.header.key.get() == hash_string(b"templateLevel", None))
            .ok_or(anyhow!("templateLevel not found"))?
            .fields.get(&hash_string(b"name", None))
            .ok_or(anyhow!("templateLevel missing name field"))?;
        let name = field
            .crc()
            .ok_or(anyhow!("templateObject name field is not a crc"))?.get();

        let t = std::time::Instant::now();
        let objs = ObjsRefVER::from_data(src, pak_header, bin, name).context("objs")?;
        println!("Block1 objs parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let string_keys =
            StringKeysRefVER::from_data(&src[pak_header.string_keys_offset.get() as usize..])
                .context("string_keys")?;
        println!("Block1 string_keys parsed in {}", t.elapsed().as_secs_f32());
        Ok(Self {
            objs,
            sub_blocks,
            string_keys,
        })
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct Block1VER {
    _ptr: BufType,
    objs: objs::ObjsVER,
    sub_blocks: sub_blocks::SubBlocksVER,
    string_keys: types::StringKeysVER,
}

#[make_platforms]
unsafe impl Sync for Block1VER {}
#[make_platforms]
unsafe impl Send for Block1VER {}

#[make_platforms]
impl Block1VER {
    pub fn from_bytes(src: &BufType, bin: &BinVER, pak_header: &PakHeaderVER) -> Result<Self> {
        let t = std::time::Instant::now();
        let data = decompress_block(
            &src[pak_header.block1_offset.get() as usize..],
            pak_header.block1_size_comp.get() as usize,
            pak_header.block1_size.get() as usize,
        )
        .context("compressed_data")?;
        println!("Block1 data parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let sub_blocks = sub_blocks::SubBlocksVER::from_bytes(
            &data,
            pak_header.sub_blocks1_offset.get() as usize,
        )
        .context("sub_blocks")?;
        println!("Block1 sub_blocks parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let objs = objs::ObjsRawVER::from_bytes(&data, bin, &sub_blocks, pak_header).and_then(|x| objs::ObjsVER::try_from(x)).context("objs")?;
        println!("Block1 objs parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let string_keys =
            types::StringKeysVER::from_bytes(&data, pak_header.string_keys_offset.get() as usize)
                .context("string_keys")?;
        println!("Block1 string_keys parsed in {}", t.elapsed().as_secs_f32());
        Ok(Self {
            _ptr: data,
            objs,
            sub_blocks,
            string_keys,
        })
    }
}

#[make_platforms]
impl Block1VER {
    pub fn objs(&self) -> &objs::ObjsVER {
        &self.objs
    }
    pub fn sub_blocks(&self) -> &sub_blocks::SubBlocksVER {
        &self.sub_blocks
    }
    pub fn string_keys(&self) -> &types::StringKeysVER {
        &self.string_keys
    }
}

pub struct Block1 {
    pub objs: objs::Objs,
    pub sub_blocks: sub_blocks::SubBlocks,
    pub strings_keys: types::StringKeys,
}

#[make_platforms]
pub trait DumpBlock1VER {
    type Data;
    fn objs(&self) -> &impl DumpObjsVER<Data=Self::Data>; 
    fn sub_blocks(&self) -> &impl DumpSubBlocksVER;
    fn string_keys(&self) -> &impl DumpStringKeysVER;
    fn dump<'a>(&self, dst: &mut DumpSlice<'a>, offsets: &'a mut [u32VER], counts: &InfoCounts, pak_header: &mut PakHeaderVER, type_infos: (&TypeInfos, &[TypeInfos])) -> Result<DumpInfosVER<'a, Self::Data>> {
        let t = std::time::Instant::now();
        let infos = self.objs().dump_into(dst, offsets, counts, pak_header, type_infos.1).context("objs")?; 
        println!("objs dumped in {}", t.elapsed().as_secs_f32());
        pak_header.sub_blocks1_offset = dst.offset.conv();
        self.sub_blocks().dump_into(dst, type_infos.0).context("sub_blocks")?;
        println!("sub_blocks1 dumped in {}", t.elapsed().as_secs_f32());
        // TODO: generate string keys based solely on the LangString sub_blocks to make those
        // easier to deal with
        pak_header.string_keys_offset = dst.offset.conv();
        self.string_keys().dump_into(dst).context("string_keys")?;
        println!("string keys dumped in {}", t.elapsed().as_secs_f32());
        Ok(infos)
    }
    fn size(&self, counts: &mut InfoCounts) -> (usize, (TypeInfos, Vec<TypeInfos>)) {
        let t = std::time::Instant::now();
        let (mut size, obj_infos) = self.objs().add_size(0, counts);
        println!("objs size in {}", t.elapsed().as_secs_f32());
        let mut infos = None;
        size += self.sub_blocks().size(&mut infos);
        println!("sub_blocks1 size in {}", t.elapsed().as_secs_f32());
        size += self.string_keys().size();
        println!("string keys size in {}", t.elapsed().as_secs_f32());
        (size, (infos.expect("sub_blocks1 level block missing"), obj_infos))
    }
}

#[make_platforms]
impl<'a> DumpBlock1VER for Block1RefVER<'a> {
    type Data = &'a CompressedDataRefAlt<'a>;
    fn objs(&self) -> &impl DumpObjsVER<Data=Self::Data> {
        &self.objs
    }
    fn sub_blocks(&self) -> &impl DumpSubBlocksVER {
        &self.sub_blocks
    }
    fn string_keys(&self) -> &impl DumpStringKeysVER {
        &self.string_keys
    }
}

#[make_platforms]
#[derive(Default)]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct Block2RefVER<'a> {
    pub sub_blocks: SubBlocksRefVER<'a>,
    pub offsets: slice<'a, u32VER>
}

#[make_platforms]
impl<'a> Block2RefVER<'a> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeaderVER) -> Result<Self> {
        let t = std::time::Instant::now();
        let sub_blocks = SubBlocksRefVER::from_data(
            &src[pak_header.sub_blocks2_offset.get() as usize..]
        )
        .context("sub_blocks")?;
        println!("Block2 sub_blocks parsed in {}", t.elapsed().as_secs_f32());
        let t = std::time::Instant::now();
        let offsets = u32VER::slice_from_data(
            &src[pak_header.block2_offsets_offset.get() as usize..],
            pak_header.block2_offsets_num.get() as usize,
        )
        .context("offsets")?;
        println!("Block2 offsets parsed in {}", t.elapsed().as_secs_f32());
        Ok(Self {
            sub_blocks,
            offsets: offsets.into(),
        })
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct Block2VER {
    _ptr: BufType,
    sub_blocks: sub_blocks::SubBlocksVER,
    offsets: NonNull<[u32VER]>,
}

#[make_platforms]
unsafe impl Sync for Block2VER {}
#[make_platforms]
unsafe impl Send for Block2VER {}

#[make_platforms]
impl Block2VER {
    pub fn from_bytes(src: &BufType, pak_header: &PakHeaderVER) -> Result<Self> {
        let t = std::time::Instant::now();
        let data = decompress_block(
            &src[pak_header.block2_offset.get() as usize..],
            pak_header.block2_size_comp.get() as usize,
            pak_header.block2_size.get() as usize,
        )
        .context("compressed_data")?;
        println!("Block2 data parsed in {}", t.elapsed().as_secs_f32());
        let t = std::time::Instant::now();
        let sub_blocks = sub_blocks::SubBlocksVER::from_bytes(
            &data,
            pak_header.sub_blocks2_offset.get() as usize,
        )
        .context("sub_blocks")?;
        println!("Block2 sub_blocks parsed in {}", t.elapsed().as_secs_f32());
        let t = std::time::Instant::now();
        let offsets = NonNull::from_ref(
            u32VER::slice_from_data(
                &data[pak_header.block2_offsets_offset.get() as usize..],
                pak_header.block2_offsets_num.get() as usize,
            )
            .context("offsets")?,
        );
        println!("Block2 offsets parsed in {}", t.elapsed().as_secs_f32());
        Ok(Self {
            _ptr: data,
            sub_blocks,
            offsets,
        })
    }
}

#[make_platforms]
impl Block2VER {
    pub fn sub_blocks(&self) -> &sub_blocks::SubBlocksVER {
        &self.sub_blocks
    }
    pub fn offsets(&self) -> &[u32VER] {
        unsafe { self.offsets.as_ref() }
    }
}

pub struct Block2 {
    pub sub_blocks: sub_blocks::SubBlocks,
    pub offsets: Vec<u32>,
}

#[make_platforms]
pub trait DumpBlock2VER {
    fn sub_blocks(&self) -> &impl DumpSubBlocksVER;
    fn dump<'a>(&self, dst: &mut DumpSlice<'a>, offset_num: usize) -> Result<&'a mut [u32VER]> {
        let type_infos = Default::default();
        self.sub_blocks().dump_into(dst, &type_infos).context("sub_blocks")?;
        let offsets = u32VER::mut_slice_from_data(dst, offset_num).context("offsets")?;
        Ok(offsets)
    }
    fn size(&self, offset_num: usize) -> usize {
        let mut type_infos = None;
        self.sub_blocks().size(&mut type_infos) + offset_num * std::mem::size_of::<u32VER>()
    }
}

#[make_platforms]
impl DumpBlock2VER for Block2RefVER<'_> {
    fn sub_blocks(&self) -> &impl DumpSubBlocksVER {
        &self.sub_blocks
    }
}

#[derive(Default)]
pub struct PakCompressedData<'a> {
    block1: CompressedDataRefAlt<'a>,
    block2: CompressedDataRefAlt<'a>,
    animations: box_slice<CompressedDataRefAlt<'a>>
}

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct PakRefVER<'a> {
    header: &'a PakHeaderVER,
    strings: StringsRefVER<'a>,
    block1: Block1RefVER<'a>,
    block2: Block2RefVER<'a>,
    animations: AnimationsRefVER<'a>,
    vals_a: slice<'a, BlockAValVER>
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
            vals_a: slice::default()
        }
    }
}

#[make_platforms]
impl<'a> PakRefVER<'a> {
    pub fn from_data<'b: 'a, 'c: 'b>(src: &'c [u8], data: &'a mut PakCompressedData<'b>, bin: &BinRefVER<'a>) -> Result<Self> {
        let t = std::time::Instant::now();
        let header = PakHeaderVER::from_data(&src[..]).context("header")?;
        let strings = StringsRefVER::from_data(
            &src[header.strings_offset.get() as usize..],
            header.strings_num.get() as usize,
        )
        .context("strings")?;
        update_crc(strings.strings());
        println!("Pak headers parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let vals_a = BlockAValVER::slice_from_data(
            &src[header.block_a_offset.get() as usize..],
            header.block_a_num.get() as usize,
        )
        .context("vals_a")?;
        println!("Pak vals_a parsed in {}", t.elapsed().as_secs_f32());

        data.block1 = CompressedDataRefAlt::from_data(&src[header.block1_offset.get() as usize..], header.block1_size_comp.get() as usize, header.block1_size.get() as usize);
        data.block2 = CompressedDataRefAlt::from_data(&src[header.block2_offset.get() as usize..], header.block2_size_comp.get() as usize, header.block2_size.get() as usize);
        
        rayon::iter::once(&mut data.block1)
            .chain(rayon::iter::once(&mut data.block2))
            .try_for_each(|data| data.decompress()).context("blocks")?;

        let block1 = Block1RefVER::from_data(&data.block1.get(), header, bin).context("block1")?;
        let block2 = Block2RefVER::from_data(&data.block2.get(), header).context("block2")?;

        let t = std::time::Instant::now();
        data.animations = block1.objs.animation_block_infos.iter().map(|info| CompressedDataRefAlt::from_data(
            &src[info.offset.get() as usize..],
            info.size_comp.get() as usize,
            info.size.get() as usize
        )).collect::<Vec<_>>().into_boxed_slice().into();
        data.animations.par_iter_mut().try_for_each(|data| data.decompress()).context("animation blocks")?;
        println!("Pak animation_data parsed in {}", t.elapsed().as_secs_f32());

        //let animations = AnimationsRefVER::from_data(animation_infos, &self.animations[..]).context("animations")?;
        let t = std::time::Instant::now();
        let animations = AnimationsRefVER::from_data(block1.objs.animation_infos.as_slice(), &data.animations[..], block1.objs.animation_block_infos.as_slice()).context("animations")?;
        println!("Pak animations parsed in {}", t.elapsed().as_secs_f32());
        Ok(PakRefVER {
            header,
            strings,
            block1,
            block2,
            animations,
            vals_a: vals_a.into()
        })

    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct PakVER {
    data: BufType,
    header: NonNull<PakHeaderVER>,
    strings: types::StringsVER,
    block1: Option<Block1VER>,
    block2: Option<Block2VER>,
    animation_blocks: Option<AnimationsVER>,
    vals_a: NonNull<[BlockAValVER]>,
}

#[make_platforms]
unsafe impl Sync for PakVER {}
#[make_platforms]
unsafe impl Send for PakVER {}

#[make_platforms]
impl PakVER {
    pub fn from_bytes(data: BufType) -> Result<Self> {
        let t = std::time::Instant::now();

        let header = PakHeaderVER::from_data(&data[..]).context("header")?;
        let strings = types::StringsVER::from_bytes(
            &data,
            header.strings_offset.get() as usize,
            header.strings_num.get() as usize,
        )
        .context("strings")?;
        update_crc(strings.strings());
        println!("Pak headers parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let vals_a = BlockAValVER::slice_from_data(
            &data[header.block_a_offset.get() as usize..],
            header.block_a_num.get() as usize,
        )
        .context("vals_a")?;
        println!("Pak vals_a parsed in {}", t.elapsed().as_secs_f32());
        let header = NonNull::from_ref(header);
        let vals_a = NonNull::from_ref(vals_a);
        Ok(Self {
            data,
            header,
            strings,
            block1: None,
            block2: None,
            animation_blocks: None,
            vals_a,
        })
    }
}

#[make_platforms]
impl PakVER {
    pub fn parse_block1(&mut self, bin: &BinVER) -> Result<()> {
        if self.block1.is_none() {
            let t = std::time::Instant::now();
            self.block1
                .replace(Block1VER::from_bytes(&self.data, bin, self.header())?);
            println!("Pak block1 parsed in {}", t.elapsed().as_secs_f32());
        }
        Ok(())
    }
    pub fn parse_block2(&mut self) -> Result<()> {
        if self.block2.is_none() {
            let t = std::time::Instant::now();
            self.block2
                .replace(Block2VER::from_bytes(&self.data, self.header())?);
            println!("Pak block2 parsed in {}", t.elapsed().as_secs_f32());
        }
        Ok(())
    }
    pub fn parse_animation_blocks(&mut self, bin: &BinVER) -> Result<()> {
        self.parse_block1(bin).context("block1")?;
        if self.animation_blocks.is_none() {
            let t = std::time::Instant::now();
            let block1 = self.block1.as_ref().unwrap();
            self.animation_blocks.replace(
                AnimationsRawVER::from_bytes(&self.data, self.header(), &block1._ptr)
                    .and_then(|x| AnimationsVER::try_from(x))
                    .context("animation_blocks")?,
            );
            println!(
                "Pak animation blocks parsed in {}",
                t.elapsed().as_secs_f32()
            );
        }
        Ok(())
    }
    pub fn parse(&mut self, bin: &BinVER) -> Result<()> {
        self.parse_block2().context("block2")?;
        self.parse_animation_blocks(bin)
    }
    pub fn header(&self) -> &PakHeaderVER {
        unsafe { self.header.as_ref() }
    }
    pub fn strings(&self) -> &types::StringsVER {
        &self.strings
    }
    pub fn block1(&self) -> Option<&Block1VER> {
        self.block1.as_ref()
    }
    pub fn block2(&self) -> Option<&Block2VER> {
        self.block2.as_ref()
    }
    pub fn animation_blocks(&self) -> Option<&AnimationsVER> {
        self.animation_blocks.as_ref()
    }
    pub fn vals_a(&self) -> &[BlockAValVER] {
        unsafe { self.vals_a.as_ref() }
    }
    /*
    pub fn vbuff_from_offset(&self, offset: u32) -> Option<&model::data::VBuffInfoVER> {
        offset
            .checked_sub(self.header().vbuff_info_offset.get())
            .and_then(|off| {
                self.block1().and_then(|block1| {
                    block1
                        .objs()
                        .vbuff_infos()
                        .get((off / self.header().vbuff_info_size.get()) as usize)
                })
            })
    }

    pub fn ibuff_from_offset(&self, offset: u32) -> Option<&model::data::IBuffInfoVER> {
        offset
            .checked_sub(self.header().ibuff_info_offset.get())
            .and_then(|off| {
                self.block1().and_then(|block1| {
                    block1
                        .objs()
                        .ibuff_infos()
                        .get((off / self.header().ibuff_info_size.get()) as usize)
                })
            })
    }
    */

    /*
    pub fn hk_shape_from_offset(
        &self,
        offset: u32,
    ) -> Option<(&model::shape::HkShapeInfoVER, &model::shape::HkShapeVER)> {
        offset
            .checked_sub(self.header().hk_shape_info_offset.get())
            .and_then(|off| {
                self.block1().and_then(|block1| {
                    let ind = (off / self.header().hk_shape_info_size.get()) as usize;
                    block1.objs().hk_shape_infos().get(ind).and_then(|info| {
                        block1
                            .objs()
                            .hk_shapes()
                            .get(ind)
                            .map(|shape| (info, shape))
                    })
                })
            })
    }
    */
}


#[make_platforms]
pub trait DumpPakVER {
    type Data;
    fn vals_a_num(&self) -> usize;
    fn write_vals_a(&self, vals_a: &mut [BlockAValVER]) -> Result<()>;
    fn block1(&self) -> &impl DumpBlock1VER<Data=Self::Data>;
    fn block2(&self) -> &impl DumpBlock2VER;
    fn animations(&self) -> &impl DumpAnimationsVER;
    fn strings(&self) -> &impl DumpStringsVER;
    fn dump(&self, dst: &mut DumpSlice, in_header: PakHeaderVER, block1: CompressedBlock, block2: CompressedBlock, animations: Vec<CompressedBlock>) -> Result<()> {
        let header = PakHeaderVER::mut_from_data(dst).context("header")?;
        header.write_from(&in_header).context("write header")?;

        for (i, animation) in animations.into_iter().enumerate() {
            dst.align(4096);
            animation.compressed.dump_into(dst).with_context(|| format!("animation block {}", i))?;
        }

        dst.align(4096);
        block1.compressed.dump_into(dst).context("block1")?;
        dst.align(4096);
        block2.compressed.dump_into(dst).context("block2")?;

        dst.align(4096);
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
        dst.align(2048);
        Ok(())
    }
    fn size(&self) -> Result<(usize, PakHeaderVER, CompressedBlock, CompressedBlock, Vec<CompressedBlock>, Vec<(u32VER, u32VER, Option<Self::Data>)>, Vec<(u32VER, u32VER, Option<Self::Data>)>, Option<(u32VER, u32VER, Self::Data)>)> {
        let t = std::time::Instant::now();
        let mut size = std::mem::size_of::<PakHeaderVER>();

        let mut header = PakHeaderVER::default();

        let block1 = self.block1();
        let block2 = self.block2();
        let animations = self.animations();
        let mut counts = InfoCounts::default();


        animations.info_counts(&mut counts);
        println!("animation counts in {}", t.elapsed().as_secs_f32());
        let (block1_size, type_infos) = block1.size(&mut counts);
        let mut block1_data = CompressedBlock::with_capacity(block1_size);
        println!("block1 size in {}", t.elapsed().as_secs_f32());
        let mut block2_data = CompressedBlock::with_capacity(block2.size(counts.offsets));
        println!("block2 size in {}", t.elapsed().as_secs_f32());
        let offsets = {
            let mut dst = block2_data.dump_slice();
            block2.dump(&mut dst, counts.offsets).context("block2")
        }?;
        println!("dumped block2 in {}", t.elapsed().as_secs_f32());
        
        let mut infos = {
            let mut dst = block1_data.dump_slice();
            block1.dump(&mut dst, offsets, &counts, &mut header, (&type_infos.0, &type_infos.1)).context("block1")
        }?;
        println!("dumped block1 in {}", t.elapsed().as_secs_f32());

        let mut animation_blocks = animations.dump(&mut infos).context("animations")?;
        println!("dumped animations in {}", t.elapsed().as_secs_f32());
        
        animation_blocks.par_iter_mut()
            .try_for_each(|x| x.compress())
            .context("compressed data")?;
        println!("compressed animations in {}", t.elapsed().as_secs_f32());

        
        for (block, info) in animation_blocks.iter().zip(infos.animation_blocks.take()) {
            size = align_offset(size, 4096) + block.compressed.len();
            info.offset = size.conv();
            info.size_comp = block.compressed.len().conv(); 
        }
        
        let model_data = infos.model_data;
        let texture_data = infos.texture_data;
        let radiosity = block1.objs().radiosity();
        let rad_data = if let Some(data) = radiosity.data() {
            let radiosity_name = hash_string(b"_radiosity", Some(block1.sub_blocks().level_name()?));
            Some((radiosity_name.into(), radiosity.usage(), data))
        } else {
            None
        };
        println!("dumped radiosity in {}", t.elapsed().as_secs_f32());
        
        rayon::iter::once(&mut block1_data)
            .chain(rayon::iter::once(&mut block2_data))
            .try_for_each(|x| x.compress())
            .context("compressed blocks")?;
        println!("compressed blocks in {}", t.elapsed().as_secs_f32());
        size = align_offset(size, 4096) + block1_data.compressed.len();
        header.block1_offset = size.conv();
        header.block1_size = block1_data.data.len().conv();
        header.block1_size_comp = block1_data.compressed.len().conv();

        size = align_offset(size, 4096) + block2_data.compressed.len();
        header.block2_offset = size.conv();
        header.block2_size = block2_data.data.len().conv();
        header.block2_size_comp = block2_data.compressed.len().conv();

        size = align_offset(size, 4096) + self.strings().size() + self.vals_a_num() * std::mem::size_of::<BlockAValVER>();
        size = align_offset(size, 2048);
        println!("misc sizes in {}", t.elapsed().as_secs_f32());
         
        Ok((size, header, block1_data, block2_data, animation_blocks, model_data, texture_data, rad_data))
    }
}
#[make_platforms]
impl<'a> DumpPakVER for PakRefVER<'a> {
    type Data = &'a CompressedDataRefAlt<'a>;
    fn vals_a_num(&self) -> usize {
        self.vals_a.len()
    }
    fn write_vals_a(&self, vals_a: &mut [BlockAValVER]) -> Result<()> {
        vals_a.write_from(&self.vals_a[..])
    }
    fn block1(&self) -> &impl DumpBlock1VER<Data=Self::Data> {
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
}
