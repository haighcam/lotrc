use anyhow::{Context, Result};
use log::debug;

use crate::types::GetNative;
use crate::types::{CompressedData, StringKeys, OrderedData, DumpSlice};
use crate::level::pak::{
    block1::{
        infos::InfoCounts,
        gameobjs::TypeInfos
    }
};

#[make_platforms]
use crate::{
    level::{
        pak::{
            PakHeaderVER,
            block1::{
                infos::{InfosRefVER, DumpInfosVER, DumpExtraInfosVER},
                objs::{ObjsRefVER, DumpObjsVER},
                sub_blocks::{SubBlocks1RefVER, DumpSubBlocks1VER},
                gameobjs::DumpGameObjsVER,
            },
        },
        bin::{BinRefVER},
    },
    types::{u32VER, StringKeysRefVER, DumpStringKeysVER}
};
use lotrc_proc::{make_platforms};

pub mod gameobjs;
pub mod infos;
pub mod objs;
pub mod sub_blocks;

#[make_platforms]
#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Block1RefVER<'a> {
    pub infos: InfosRefVER<'a>,
    pub objs: ObjsRefVER<'a>,
    pub sub_blocks: SubBlocks1RefVER<'a>,
    pub string_keys: StringKeysRefVER<'a>
}

#[make_platforms]
impl<'a> Block1RefVER<'a> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeaderVER, bin: &BinRefVER<'a>) -> Result<Self> {
        let t = std::time::Instant::now();
        let sub_blocks = SubBlocks1RefVER::from_data(
            &src[pak_header.sub_blocks1_offset.get() as usize..]
        )
        .context("sub_blocks")?;
        debug!("Block1 sub_blocks parsed in {}", t.elapsed().as_secs_f32());

        let name = sub_blocks.level.level_name()?;

        let t = std::time::Instant::now();
        let infos = InfosRefVER::from_data(src, pak_header).context("infos")?;
        let objs = ObjsRefVER::from_data(src, &infos, bin, name).context("objs")?;
        debug!("Block1 objs parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let string_keys =
            StringKeysRefVER::from_data(&src[pak_header.string_keys_offset.get() as usize..])
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

#[make_platforms]
pub trait DumpBlock1VER {
    fn infos(&self) -> &impl DumpExtraInfosVER;
    fn objs(&self) -> &impl DumpObjsVER; 
    fn sub_blocks(&self) -> &impl DumpSubBlocks1VER;
    fn dump<'a, 'b>(&'b self, dst: &mut DumpSlice<'a>, offsets: &'a mut [u32VER], counts: &InfoCounts, pak_header: &mut PakHeaderVER, type_infos: (&TypeInfos, &[TypeInfos]), string_keys: &[u32]) -> Result<(DumpInfosVER<'a, 'b>, CompressedData<'b>)> {
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
        DumpStringKeysVER::dump_into(string_keys, dst).context("string_keys")?;
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
        size += DumpStringKeysVER::size(string_keys);
        debug!("string keys size in {}, size {}", t.elapsed().as_secs_f32(), size);
        (size, (ty, obj_infos))
    }
}

#[make_platforms]
impl<'a> DumpBlock1VER for Block1RefVER<'a> {
    fn infos(&self) -> &impl DumpExtraInfosVER {
        &self.infos
    }
    fn objs(&self) -> &impl DumpObjsVER {
        &self.objs
    }
    fn sub_blocks(&self) -> &impl DumpSubBlocks1VER {
        &self.sub_blocks
    }
}

