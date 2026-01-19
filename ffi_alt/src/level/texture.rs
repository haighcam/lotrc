use safer_ffi::prelude::*;
use std::ptr::{null, slice_from_raw_parts_mut};
use lotrc::macros::make_platforms;
use lotrc::types::{slice, Map, str_ref};


#[make_platforms]
mod impl_ver {
    use super::*;
    use lotrc::{
        level::texture::{TextureInfoVER, TextureRefVER},
        types::CompressedDataRefAlt
    };
    #[ffi_export]
    pub fn lotrc_texture_from_data_ver<'a>(info: &'a TextureInfoVER, texture_data: &'a Map<u32, &'a CompressedDataRefAlt<'a>>, err: Out<'_, bool>) -> TextureRefVER<'a> {
        let val = TextureRefVER::from_data(info, texture_data);
        err.write(val.is_err());
        val.unwrap_or_default()
    }
}

