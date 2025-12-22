#[cfg(feature = "python")]
use crate::pyobj_ref;
use anyhow::{Context, Result};
#[cfg(not(feature = "python"))]
use lotrc_proc::getter;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::ptr::NonNull;
use std::sync::Arc;
use indexmap::IndexMap;

use crate::sub_blocks;
use crate::types::{self, decompress_block, update_crc, Crc, RefFromData};
#[make_platforms]
use crate::{
    level::{
        pak::animation::AnimationsVER,
        bin::BinVER,
    }, types::U32VER
};
use lotrc_proc::{make_platforms, OrderedData};

pub mod animation;
pub mod objs;

#[cfg(feature = "python")]
pub fn init(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "pak")?;
    m.add_class::<PakHeader>()?;
    m.add_class::<BlockAVal>()?;
    //m.add_class::<Block1>()?;
    //m.add_class::<Block2>()?;
    m.add_submodule(&animation::init(py)?)?;
    m.add_submodule(&objs::init(py)?)?;
    init_pc(&m)?;
    init_xbox(&m)?;
    init_ps3(&m)?;
    Ok(m)
}
#[cfg(feature = "python")]
#[make_platforms]
pub fn init_ver(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Block1VER>()?;
    m.add_class::<Block2VER>()?;
    m.add_class::<PakVER>()
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "pak", get_all, set_all))]
pub struct PakHeader {
    #[ordered_data(PC)]
    pub block_a_num: u32,
    #[ordered_data(PC)]
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

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "pak", get_all, set_all))]
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
#[cfg_attr(feature = "python", pyclass(module = "pak"))]
#[derive(Debug, Clone)]
pub struct Block1VER {
    _ptr: Arc<[u8]>,
    objs: objs::ObjsVER,
    sub_blocks: sub_blocks::SubBlocksVER,
    string_keys: types::StringKeysVER,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(Block1VER);

#[make_platforms]
unsafe impl Sync for Block1VER {}
#[make_platforms]
unsafe impl Send for Block1VER {}

#[make_platforms]
impl Block1VER {
    pub fn from_bytes(src: &Arc<[u8]>, bin: &BinVER, pak_header: &PakHeaderVER) -> Result<Self> {
        let t = std::time::Instant::now();
        let data: Arc<[u8]> = decompress_block(
            &src[pak_header.block1_offset.get() as usize..],
            pak_header.block1_size_comp.get() as usize,
            pak_header.block1_size.get() as usize,
        )
        .context("compressed_data")?
        .into();
        println!("Block1 data parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let sub_blocks = sub_blocks::SubBlocksVER::from_bytes(
            &data,
            pak_header.sub_blocks1_offset.get() as usize,
        )
        .context("sub_blocks")?;

        let t = std::time::Instant::now();
        let objs = objs::ObjsVER::from_bytes(&data, bin, &sub_blocks, pak_header).context("objs")?;
        println!("Block1 objs parsed in {}", t.elapsed().as_secs_f32());

        println!("Block1 sub_blocks parsed in {}", t.elapsed().as_secs_f32());
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
#[cfg_attr(feature = "python", pymethods)]
impl Block1VER {
    #[getter]
    pub fn objs(&self) -> &objs::ObjsVER {
        &self.objs
    }
    #[getter]
    pub fn sub_blocks(&self) -> &sub_blocks::SubBlocksVER {
        &self.sub_blocks
    }
    #[getter]
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
#[cfg_attr(feature = "python", pyclass(module = "pak"))]
#[derive(Debug, Clone)]
pub struct Block2VER {
    _ptr: Arc<[u8]>,
    sub_blocks: sub_blocks::SubBlocksVER,
    offsets: NonNull<[U32VER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(Block2VER);

#[make_platforms]
unsafe impl Sync for Block2VER {}
#[make_platforms]
unsafe impl Send for Block2VER {}

#[make_platforms]
impl Block2VER {
    pub fn from_bytes(src: &Arc<[u8]>, pak_header: &PakHeaderVER) -> Result<Self> {
        let t = std::time::Instant::now();
        let data: Arc<[u8]> = decompress_block(
            &src[pak_header.block2_offset.get() as usize..],
            pak_header.block2_size_comp.get() as usize,
            pak_header.block2_size.get() as usize,
        )
        .context("compressed_data")?
        .into();
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
            U32VER::slice_from_data(
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
#[cfg_attr(feature = "python", pymethods)]
impl Block2VER {
    #[getter]
    pub fn sub_blocks(&self) -> &sub_blocks::SubBlocksVER {
        &self.sub_blocks
    }
    #[getter]
    pub fn offsets(&self) -> &[U32VER] {
        unsafe { self.offsets.as_ref() }
    }
}

pub struct Block2 {
    pub sub_blocks: sub_blocks::SubBlocks,
    pub offsets: Vec<u32>,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "pak"))]
#[derive(Debug, Clone)]
pub struct PakVER {
    data: Arc<[u8]>,
    header: NonNull<PakHeaderVER>,
    strings: types::StringsVER,
    block1: Option<Block1VER>,
    block2: Option<Block2VER>,
    animation_blocks: Option<AnimationsVER>,
    vals_a: NonNull<[BlockAValVER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(PakVER);

#[make_platforms]
unsafe impl Sync for PakVER {}
#[make_platforms]
unsafe impl Send for PakVER {}

#[make_platforms]
impl PakVER {
    pub fn from_bytes(data: Arc<[u8]>) -> Result<Self> {
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
#[cfg_attr(feature = "python", pymethods)]
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
                AnimationsVER::from_bytes(&self.data, self.header(), &block1._ptr)
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
    #[getter]
    pub fn header(&self) -> &PakHeaderVER {
        unsafe { self.header.as_ref() }
    }
    #[getter]
    pub fn strings(&self) -> &types::StringsVER {
        &self.strings
    }
    #[getter]
    pub fn block1(&self) -> Option<&Block1VER> {
        self.block1.as_ref()
    }
    #[getter]
    pub fn block2(&self) -> Option<&Block2VER> {
        self.block2.as_ref()
    }
    #[getter]
    pub fn animation_blocks(&self) -> Option<&AnimationsVER> {
        self.animation_blocks.as_ref()
    }
    #[getter]
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
