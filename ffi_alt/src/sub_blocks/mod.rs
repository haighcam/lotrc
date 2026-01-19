use safer_ffi::prelude::*;
use std::ptr::{null, slice_from_raw_parts_mut};
use lotrc::macros::make_platforms;
use lotrc::types::Map;

pub mod gameobjs;

#[make_platforms]
#[derive_ReprC]
#[repr(u8)]
enum SubBlockType {
    LangStrings = 0,
    Data,
    Spray,
    PFields,
    Crowd,
    Level,
    AtlasUV,
    Lua,
    SSA,
}

#[make_platforms]
mod impl_ver {
    use super::*;
    use lotrc::{
        sub_blocks::{
            SubBlockRefVER, LangStringsRefVER, DataRefVER, SprayRefVER, CrowdRefVER,
            AtlasUVRefVER, PFieldsRefVER, LuaRefVER, SSARefVER,
            gameobjs::GameObjsRefVER,
        },
    };
    #[ffi_export]
    pub fn lotrc_sub_block_map_get_ver<'a>(map: &'a Map<u32, SubBlockRefVER<'a>>, key: u32) -> *const SubBlockRefVER<'a> {
        let val = map.get(&key);
        val.map(|x| x as *const _).unwrap_or(null())
    }
    #[ffi_export]
    pub fn lotrc_sub_block_map_len_ver<'a>(map: &'a Map<u32, SubBlockRefVER<'a>>) -> usize {
        map.len()
    }
    #[ffi_export]
    /// keys is a caller allocated array for returning keys
    pub fn lotrc_sub_block_map_keys_ver<'a>(map: &'a Map<u32, SubBlockRefVER<'a>>, keys: *mut u32) {
        if keys.is_null() { return; }
        let keys = unsafe { &mut*slice_from_raw_parts_mut(keys, map.len()) };
        for (src, dst) in map.keys().zip(keys) {
            *dst = *src;
        }
    }

    #[ffi_export]
    pub fn lotrc_sub_block_get_type_ver<'a>(sub_block: &'a SubBlockRefVER<'a>) -> SubBlockType {
        match sub_block {
            SubBlockRefVER::LangStrings(_) => SubBlockType::LangStrings,
            SubBlockRefVER::Data(_) => SubBlockType::Data,
            SubBlockRefVER::Spray(_) => SubBlockType::Spray,
            SubBlockRefVER::PFields(_) => SubBlockType::PFields,
            SubBlockRefVER::Crowd(_) => SubBlockType::Crowd,
            SubBlockRefVER::Level(_) => SubBlockType::Level,
            SubBlockRefVER::AtlasUV(_) => SubBlockType::AtlasUV,
            SubBlockRefVER::Lua(_) => SubBlockType::Lua,
            SubBlockRefVER::SSA(_) => SubBlockType::SSA,
        }
    }
    #[ffi_export]
    pub fn lotrc_sub_block_get_lang_strings_ver<'a>(sub_block: &'a SubBlockRefVER<'a>) -> *const LangStringsRefVER<'a> {
        match sub_block {
            SubBlockRefVER::LangStrings(val) => val as *const _,
            _ => null()
        }
    }
    #[ffi_export]
    pub fn lotrc_sub_block_get_data_ver<'a>(sub_block: &'a SubBlockRefVER<'a>) -> *const DataRefVER<'a> {
        match sub_block {
            SubBlockRefVER::Data(val) => val as *const _,
            _ => null()
        }
    }
    #[ffi_export]
    pub fn lotrc_sub_block_get_spray_ver<'a>(sub_block: &'a SubBlockRefVER<'a>) -> *const SprayRefVER<'a> {
        match sub_block {
            SubBlockRefVER::Spray(val) => val as *const _,
            _ => null()
        }
    }
    #[ffi_export]
    pub fn lotrc_sub_block_get_pfields_ver<'a>(sub_block: &'a SubBlockRefVER<'a>) -> *const PFieldsRefVER<'a> {
        match sub_block {
            SubBlockRefVER::PFields(val) => val as *const _,
            _ => null()
        }
    }
    #[ffi_export]
    pub fn lotrc_sub_block_get_crowd_ver<'a>(sub_block: &'a SubBlockRefVER<'a>) -> *const CrowdRefVER<'a> {
        match sub_block {
            SubBlockRefVER::Crowd(val) => val as *const _,
            _ => null()
        }
    }
    #[ffi_export]
    pub fn lotrc_sub_block_get_level_ver<'a>(sub_block: &'a SubBlockRefVER<'a>) -> *const GameObjsRefVER<'a> {
        match sub_block {
            SubBlockRefVER::Level(val) => val as *const _,
            _ => null()
        }
    }
    #[ffi_export]
    pub fn lotrc_sub_block_get_atlas_uv_ver<'a>(sub_block: &'a SubBlockRefVER<'a>) -> *const AtlasUVRefVER<'a> {
        match sub_block {
            SubBlockRefVER::AtlasUV(val) => val as *const _,
            _ => null()
        }
    }
    #[ffi_export]
    pub fn lotrc_sub_block_get_lua_ver<'a>(sub_block: &'a SubBlockRefVER<'a>) -> *const LuaRefVER<'a> {
        match sub_block {
            SubBlockRefVER::Lua(val) => val as *const _,
            _ => null()
        }
    }
    #[ffi_export]
    pub fn lotrc_sub_block_get_ssa_ver<'a>(sub_block: &'a SubBlockRefVER<'a>) -> *const SSARefVER<'a> {
        match sub_block {
            SubBlockRefVER::SSA(val) => val as *const _,
            _ => null()
        }
    }
}
