use safer_ffi::prelude::*;
use std::ptr::{null, slice_from_raw_parts_mut, NonNull};
use lotrc::macros::make_platforms;
use lotrc::types::{slice, Map, str_ref};

pub mod pak;
pub mod bin;
pub mod radiosity;
pub mod texture;


#[make_platforms]
mod impl_ver {
    use super::*;
    use lotrc::{
        level::{LevelData, LevelRefVER, LevelCompressedData},
        types::CompressedDataRefAlt
    };
    #[ffi_export]
    pub fn lotrc_read_level_data_ver<'a>(path: char_p::Ref<'a>) -> Option<repr_c::Box<LevelData>> {
        LevelData::read(path.to_str()).ok().map(|x| Box::new(x).into())
    }
    #[ffi_export]
    pub fn lotrc_level_data_free_ver(level_data: repr_c::Box<LevelData>) {
        drop(level_data)
    }
    #[ffi_export]
    pub fn lotrc_level_compressed_data_ver<'a>() -> repr_c::Box<LevelCompressedData<'a>> {
        Box::new(LevelCompressedData::default()).into()
    }
    #[ffi_export]
    pub fn lotrc_level_compressed_data_free_ver(data: repr_c::Box<LevelCompressedData<'_>>) {
        drop(data)
    }
    #[ffi_export]
    pub fn lotrc_level_from_data_ver<'a>(src: &'a LevelData, data: &'a mut LevelCompressedData<'a>, err: Out<'_, bool>) -> LevelRefVER<'a> {
        let val = LevelRefVER::from_data(src, data);
        err.write(val.is_err());
        val.unwrap_or_default()
    }
}

