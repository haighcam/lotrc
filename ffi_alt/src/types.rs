use safer_ffi::prelude::*;
use lotrc::macros::make_platforms;

#[macro_export]
macro_rules! map_impl {
    ($name:ident, $key:ty, $val:ty, $suffix:ident) => {
        paste::paste! {
            #[safer_ffi::ffi_export]
            pub fn [<lotrc_ $name _map_get $suffix>]<'a>(map: &'a lotrc::types::Map<$key, $val>, key: $key) -> *const $val {
                let val = map.get(&key);
                val.map(|x| x as *const _).unwrap_or(std::ptr::null())
            }
            #[safer_ffi::ffi_export]
            pub fn [<lotrc_ $name _map_len $suffix>]<'a>(map: &'a lotrc::types::Map<$key, $val>) -> usize {
                map.len()
            }
            #[safer_ffi::ffi_export]
            /// keys is a caller allocated array for returning keys
            pub fn [<lotrc_ $name _map_keys $suffix>]<'a>(map: &'a lotrc::types::Map<$key, $val>, keys: *mut $key) {
                if keys.is_null() { return; }
                let keys = unsafe { &mut*std::ptr::slice_from_raw_parts_mut(keys, map.len()) };
                for (src, dst) in map.keys().zip(keys) {
                    *dst = *src;
                }
            }
        }
    }
}

#[make_platforms]
mod impl_ver {
    use super::*;
    use lotrc::{
        types::{slice, StringKeysRefVER}
    };
    map_impl!(anim, u32, lotrc::level::pak::animation::AnimationRefVER<'a>, _ver);
    map_impl!(type, u32, lotrc::sub_blocks::gameobjs::TypeRefVER<'a>, _ver);
    map_impl!(obj, u32, lotrc::sub_blocks::gameobjs::ObjRefVER<'a>, _ver);
    map_impl!(basetype, u32, lotrc::sub_blocks::gameobjs::BaseTypeRefVER<'a>, _ver);
    map_impl!(sub_block, u32, lotrc::sub_blocks::SubBlockRefVER<'a>, _ver);
    map_impl!(data, u32, lotrc::types::CompressedDataRefAlt<'a>, _ver);
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

#[ffi_export]
pub fn hash_string<'a>(string: c_slice::Ref<'a, u8>, mask: Option<&'a u32>) -> u32 {
    lotrc::types::hash_string(&string, mask.copied())
}
