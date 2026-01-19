use safer_ffi::ffi_export;
use lotrc::macros::make_platforms;
use safer_ffi::prelude::Out;

#[make_platforms]
mod impl_ver {
    use super::*;
    use lotrc::{
        types::{slice, StringKeysRefVER, Map}
    };
    #[ffi_export]
    pub fn lotrc_stringkeys_from_data_ver<'a>(src: slice<'a, u8>, err: Out<'_, bool>) -> StringKeysRefVER<'a> {
        match StringKeysRefVER::from_data(src.as_slice()) {
            Ok(val) => {
                err.write(false);
                val
            },
            Err(_) => {
                err.write(true);
                Default::default()
            }
        }
    }
}
