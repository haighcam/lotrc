use anyhow::{Context, Result};
use log::debug;

use crate::types::{CompressedData, StringKeys, OrderedData, DumpSlice};
use crate::level::pak::{
    block1::{
        infos::InfoCounts,
        gameobjs::TypeInfos
    }
};

#[make_endian]
use crate::{
    level::{
        pak::{
            PakHeader_XE_,
            block1::{
                infos::{InfosRef_XE_, DumpInfos_XE_, DumpExtraInfos_XE_},
                objs::{ObjsRef_XE_, DumpObjs_XE_},
                sub_blocks::{SubBlocks1Ref_XE_, DumpSubBlocks1_XE_},
                gameobjs::DumpGameObjs_XE_,
            },
        },
        bin::{BinRef_XE_},
    },
    types::{u32_XE_, StringKeysRef_XE_, DumpStringKeys_XE_}
};
use lotrc_proc::{make_endian};

pub mod gameobjs;
pub mod infos;
pub mod objs;
pub mod sub_blocks;

#[make_endian]
#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Block1Ref_XE_<'a> {
    pub infos: InfosRef_XE_<'a>,
    pub objs: ObjsRef_XE_<'a>,
    pub sub_blocks: SubBlocks1Ref_XE_<'a>,
    pub string_keys: StringKeysRef_XE_<'a>
}

#[make_endian]
impl<'a> Block1Ref_XE_<'a> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeader_XE_, bin: &BinRef_XE_<'a>) -> Result<Self> {
        let t = std::time::Instant::now();
        let sub_blocks = SubBlocks1Ref_XE_::from_data(
            &src[pak_header.sub_blocks1_offset.conv()..]
        )
        .context("sub_blocks")?;
        debug!("Block1 sub_blocks parsed in {}", t.elapsed().as_secs_f32());

        let name = sub_blocks.level.level_name()?;

        let t = std::time::Instant::now();
        let infos = InfosRef_XE_::from_data(src, pak_header).context("infos")?;
        let objs = ObjsRef_XE_::from_data(src, &infos, bin, name).context("objs")?;
        debug!("Block1 objs parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let string_keys =
            StringKeysRef_XE_::from_data(&src[pak_header.string_keys_offset.conv()..])
                .context("string_keys")?;
        debug!("Block1 string_keys parsed in {}", t.elapsed().as_secs_f32());
        Ok(Self {
            infos,
            objs,
            sub_blocks,
            string_keys,
        })
    }
}

pub struct Block1 {
    pub objs: objs::Objs,
    //pub sub_blocks: sub_blocks::SubBlocks,
    pub strings_keys: StringKeys,
}

#[make_endian]
pub trait DumpBlock1_XE_ {
    fn infos(&self) -> &impl DumpExtraInfos_XE_;
    fn objs(&self) -> &impl DumpObjs_XE_; 
    fn sub_blocks(&self) -> &impl DumpSubBlocks1_XE_;
    fn dump<'a, 'b>(&'b self, dst: &mut DumpSlice<'a>, offsets: &'a mut [u32_XE_], counts: &InfoCounts, pak_header: &mut PakHeader_XE_, type_infos: (&TypeInfos, &[TypeInfos]), string_keys: &[u32]) -> Result<(DumpInfos_XE_<'a, 'b>, CompressedData<'b>)> {
        debug!("Dumping block1 into buffer of size {}", dst.vals.len());
        let t = std::time::Instant::now();
        let mut infos = self.infos().dump_into(dst, offsets, counts, pak_header).context("infos")?;
        let rad_data = self.objs().dump_into(dst, pak_header, type_infos.1, self.sub_blocks().level(), &mut infos).context("objs")?; 
        debug!("objs dumped in {}, size {}", t.elapsed().as_secs_f32(), dst.offset);
        pak_header.sub_blocks1_offset = dst.offset.conv();
        self.sub_blocks().dump_into(dst, type_infos.0).context("sub_blocks")?;
        debug!("sub_blocks1 dumped in {}, size {}", t.elapsed().as_secs_f32(), dst.offset);
        // TODO: generate string keys based solely on the LangString sub_blocks to make those
        // easier to deal with
        pak_header.string_keys_offset = dst.offset.conv();
        DumpStringKeys_XE_::dump_into(string_keys, dst).context("string_keys")?;
        debug!("string keys dumped in {}, size {}", t.elapsed().as_secs_f32(), dst.offset);
        Ok((infos, rad_data))
    }
    fn size(&self, counts: &mut InfoCounts, string_keys: &[u32]) -> (usize, (TypeInfos, Vec<TypeInfos>)) {
        let t = std::time::Instant::now();
        let (mut size, obj_infos) = self.objs().add_size(0, counts, self.sub_blocks().level());
        size = self.infos().add_size(size, counts);
        debug!("objs size in {}, size {}", t.elapsed().as_secs_f32(), size);
        let (s, ty) = self.sub_blocks().size();
        size += s;
        debug!("sub_blocks1 size in {}, size {}", t.elapsed().as_secs_f32(), size);
        size += DumpStringKeys_XE_::size(string_keys);
        debug!("string keys size in {}, size {}", t.elapsed().as_secs_f32(), size);
        (size, (ty, obj_infos))
    }
}

#[make_endian]
impl<'a> DumpBlock1_XE_ for Block1Ref_XE_<'a> {
    fn infos(&self) -> &impl DumpExtraInfos_XE_ {
        &self.infos
    }
    fn objs(&self) -> &impl DumpObjs_XE_ {
        &self.objs
    }
    fn sub_blocks(&self) -> &impl DumpSubBlocks1_XE_ {
        &self.sub_blocks
    }
}

