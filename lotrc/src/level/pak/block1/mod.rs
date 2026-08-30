use anyhow::{Context, Result};
use log::debug;

use crate::{
    level::pak::{
        PakHeader,
        block1::{
            infos::{InfoCounts, InfosRef, DumpInfos, DumpExtraInfos},
            objs::{ObjsRef, DumpObjs},
            sub_blocks::{SubBlocks1Ref, DumpSubBlocks1},
            gameobjs::{TypeInfos}
        }
    },
    level::bin::BinRef,
    types::{
        DumpSlice, BaseTypes, CompressedData,
        sub_blocks::{StringKeysRef, DumpStringKeys, StringKeys}
    }
};

pub mod gameobjs;
pub mod infos;
pub mod objs;
pub mod sub_blocks;

#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Block1Ref<'a, T: BaseTypes> {
    pub infos: InfosRef<'a, T>,
    pub objs: ObjsRef<'a, T>,
    pub sub_blocks: SubBlocks1Ref<'a, T>,
    pub string_keys: StringKeysRef<'a, T>
}

impl<'a, T: BaseTypes> Block1Ref<'a, T> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeader<T>, bin: &BinRef<'a, T>) -> Result<Self> {
        let t = std::time::Instant::now();
        let sub_blocks = SubBlocks1Ref::from_data(
            &src[pak_header.sub_blocks1_offset.into() as usize..]
        )
        .context("sub_blocks")?;
        debug!("Block1 sub_blocks parsed in {}", t.elapsed().as_secs_f32());

        let name = sub_blocks.level.level_name()?;

        let t = std::time::Instant::now();
        let infos = InfosRef::from_data(src, pak_header).context("infos")?;
        let objs = ObjsRef::from_data(src, &infos, bin, name).context("objs")?;
        debug!("Block1 objs parsed in {}", t.elapsed().as_secs_f32());

        let t = std::time::Instant::now();
        let string_keys =
            StringKeysRef::from_data(&src[pak_header.string_keys_offset.into() as usize..])
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

pub trait DumpBlock1<T: BaseTypes> {
    fn infos(&self) -> &impl DumpExtraInfos<T>;
    fn objs(&self) -> &impl DumpObjs<T>; 
    fn sub_blocks(&self) -> &impl DumpSubBlocks1<T>;
    fn dump<'a, 'b>(&'b self, dst: &mut DumpSlice<'a>, offsets: &'a mut [T::u32], counts: &InfoCounts, pak_header: &mut PakHeader<T>, type_infos: (&TypeInfos, &[TypeInfos]), string_keys: &[u32]) -> Result<(DumpInfos<'a, 'b, T>, CompressedData<'b>)> {
        debug!("Dumping block1 into buffer of size {}", dst.vals.len());
        let t = std::time::Instant::now();
        let mut infos = self.infos().dump_into(dst, offsets, counts, pak_header).context("infos")?;
        let rad_data = self.objs().dump_into(dst, pak_header, type_infos.1, self.sub_blocks().level(), &mut infos).context("objs")?; 
        debug!("objs dumped in {}, size {}", t.elapsed().as_secs_f32(), dst.offset);
        pak_header.sub_blocks1_offset = (dst.offset as u32).into();
        self.sub_blocks().dump_into(dst, type_infos.0).context("sub_blocks")?;
        debug!("sub_blocks1 dumped in {}, size {}", t.elapsed().as_secs_f32(), dst.offset);
        // TODO: generate string keys based solely on the LangString sub_blocks to make those
        // easier to deal with
        pak_header.string_keys_offset = (dst.offset as u32).into();
        DumpStringKeys::<T>::dump_into(string_keys, dst).context("string_keys")?;
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
        size += DumpStringKeys::<T>::size(string_keys);
        debug!("string keys size in {}, size {}", t.elapsed().as_secs_f32(), size);
        (size, (ty, obj_infos))
    }
}

impl<'a, T: BaseTypes> DumpBlock1<T> for Block1Ref<'a, T> {
    fn infos(&self) -> &impl DumpExtraInfos<T> {
        &self.infos
    }
    fn objs(&self) -> &impl DumpObjs<T> {
        &self.objs
    }
    fn sub_blocks(&self) -> &impl DumpSubBlocks1<T> {
        &self.sub_blocks
    }
}
