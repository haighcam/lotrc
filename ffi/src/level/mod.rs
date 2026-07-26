use std::ptr::NonNull;
use std::ffi::c_char;
use lotrc::macros::{make_platforms, export, make_owned_opaque};

use lotrc::{
    level::{
        pak::block1::infos::InfoCounts,
        LevelData, LevelCompressedData, Version
    },
    types::{CompressedDataRef, ref_slice, CompressedData, DumpSlice, mut_slice}
};
#[make_platforms]
use lotrc::{
    level::{
        LevelRefVER, DumpLevelVER,
        pak::block1::infos::DumpInfosVER
    },
    types::u32VER
};
use crate::{c_str_ptr};

make_owned_opaque!(OwnedLevelData, LevelData);
#[export(mod_name=OwnedLevelData)]
mod owned_data {
    use super::*;
    fn read_data(path: Option<&c_char>) -> Option<NonNull<OwnedLevelData>> {
        LevelData::read(unsafe { c_str_ptr(path) }).ok().map(OwnedLevelData::leak)
    }
}
#[export(mod_name=LevelData)]
mod data {
    use super::*;
    fn version<'a>(src: &'a LevelData) -> Version {
        src.version()
    }
}

make_owned_opaque!(OwnedLevelCompressedData, LevelCompressedData<'a>, <'a>);
#[export(mod_name=OwnedLevelCompressedData)]
mod owned_compressed_data {
    use super::*;
    fn new<'a>() -> NonNull<OwnedLevelCompressedData<'a>> {
        OwnedLevelCompressedData::leak(LevelCompressedData::default())
    }
}

#[make_platforms]
make_owned_opaque!(OwnedLevelRefVER, LevelRefVER<'a>, <'a>);
#[make_platforms]
#[export(mod_name=OwnedLevelRefVER)]
mod owned_ref_ver {
    use super::*;
    fn from_data<'a>(src: Option<&'a LevelData>, data: Option<&'a mut LevelCompressedData<'a>>) -> Option<NonNull<OwnedLevelRefVER<'a>>> {
        LevelRefVER::from_data(src?, data?).ok().map(OwnedLevelRefVER::leak)
    }
}

#[make_platforms]
#[export(mod_name=LevelRefVER)]
mod ref_ver {
    use super::*;
    fn dump(src: Option<&LevelRefVER>, compression: u32) -> Option<NonNull<OwnedLevelData>> {
        src.and_then(|x| x.dump(lotrc::re_export::Compression::new(compression)).ok().map(OwnedLevelData::leak))
    }
}

make_owned_opaque!(OwnedInfoCounts, InfoCounts);
#[export(mod_name=InfoCounts)]
mod info_counts {
    use super::*;
    fn new() -> NonNull<OwnedInfoCounts> {
        OwnedInfoCounts::leak(InfoCounts::default())
    }
    #[make_platforms]
    fn size_ver(counts: Option<&InfoCounts>) -> usize {
        counts.map(|x| x.size_ver(0)).unwrap_or_default()
    }
}

#[make_platforms]
make_owned_opaque!(OwnedDumpInfosVER, DumpInfosVER<'a, 'a>, <'a>);
#[make_platforms]
#[export(mod_name=DumpInfosVER)]
mod dump_infos_ver {
    use super::*;
    fn from_data<'a>(dst: Option<&'a mut DumpSlice<'a>>, counts: Option<&InfoCounts>, offsets: Option<&'a mut mut_slice<'a, u32VER>>) -> Option<NonNull<OwnedDumpInfosVER<'a>>> {
        DumpInfosVER::from_data(dst?, counts?, *offsets?).ok().map(OwnedDumpInfosVER::leak)
    }
}

make_owned_opaque!(OwnedVecCompressedData, Vec<&'a CompressedDataRef<'a>>, <'a>);
#[make_platforms]
#[export(mod_name=AnimationsRefVER)]
mod animations_ref_ver {
    use super::*;
    use lotrc::level::pak::animation::{DumpAnimationsVER, AnimationsRefVER, AnimationBlockInfoVER, AnimationInfoVER};
    fn dump<'a>(anims: Option<&'a AnimationsRefVER<'a>>, infos: Option<&mut DumpInfosVER>) -> Option<NonNull<OwnedVecCompressedData<'a>>> {
        anims?.dump(infos?).ok().map(|x| OwnedVecCompressedData::leak(x.into_iter().filter_map(|x| match x {
            CompressedData::Ref(x) => Some(x),
            _ => None
        }).collect()))
    }
    /*
    fn from_data<'a>(anim_infos: Option<&'a ref_slice<'a, AnimationInfoVER>>, blocks: Option<&'a ref_slice<'a, CompressedDataRef<'a>>>, block_infos: Option<&'a ref_slice<'a, AnimationBlockInfoVER>>) -> Option<NonNull<AnimationsRefVER>> {
        AnimationsRefVER::from_data(*(anim_infos?), *(blocks?), *(block_infos?))
    }
    */
}
