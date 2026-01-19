use std::ptr::{null, slice_from_raw_parts_mut};
use stdint::size_t;
use lotrc::macros::make_platforms;

pub mod gameobjs;

#[repr(u8)]
pub enum SubBlockType {
    LangStrings,
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
pub mod ref_impl_ver {
    use super::*;
    use lotrc::{
        types::{u32VER, CrcVER, u16VER},
        sub_blocks::{AtlasUVVER, AtlasUVValVER, CrowdVER, CrowdHeaderVER, CrowdItemVER, CrowdValVER, CrowdItemHeaderVER, SprayVER, SprayInstanceVER, SprayValVER, SSAVER, SSAValVER}
    };

    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_ssa_get_vals_ver(ssa: *const SSAVER) -> *const SSAValVER {
        if ssa.is_null() { return null(); }
        unsafe { &*ssa }.vals().as_ptr()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_ssa_get_vals_num_ver(ssa: *const SSAVER) -> size_t {
        if ssa.is_null() { return 0; }
        unsafe { &*ssa }.vals().len()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_ssa_get_strings_ver(ssa: *const SSAVER) -> *const *const u16VER {
        if ssa.is_null() { return null(); }
        unsafe { &*ssa }.strings_raw().as_ptr() as _
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_ssa_get_strings_num_ver(ssa: *const SSAVER) -> size_t {
        if ssa.is_null() { return 0; }
        unsafe { &*ssa }.strings_raw().len()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_ssa_get_strings_sizes_ver(ssa: *const SSAVER, sizes: *mut size_t) {
        if ssa.is_null() || sizes.is_null() { return; }
        let ssa = unsafe { &*ssa };
        let sizes = unsafe { &mut*slice_from_raw_parts_mut(sizes, ssa.strings_raw().len()) };
        for (src, dst) in ssa.strings_raw().iter().zip(sizes) {
            *dst = unsafe { src.as_ref() }.len();
        }
    }
    
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_spray_get_instances_ver(spray: *const SprayVER) -> *const SprayInstanceVER {
        if spray.is_null() { return null(); }
        unsafe { &*spray }.instances().as_ptr()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_spray_get_instances_num_ver(spray: *const SprayVER) -> size_t {
        if spray.is_null() { return 0; }
        unsafe { &*spray }.instances().len()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_spray_get_vals_ver(spray: *const SprayVER) -> *const SprayValVER {
        if spray.is_null() { return null(); }
        unsafe { &*spray }.vals().as_ptr()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_spray_get_vals_num_ver(spray: *const SprayVER) -> size_t {
        if spray.is_null() { return 0; }
        unsafe { &*spray }.vals().len()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_crowditem_get_header_ver(crowditem: *const CrowdItemVER) -> *const CrowdItemHeaderVER {
        if crowditem.is_null() { return null(); }
        unsafe { &*crowditem }.header() as _
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_crowditem_get_animations_ver(crowditem: *const CrowdItemVER) -> *const CrcVER {
        if crowditem.is_null() { return null(); }
        unsafe { &*crowditem }.animations().as_ptr()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_crowditem_get_animations_num_ver(crowditem: *const CrowdItemVER) -> size_t {
        if crowditem.is_null() { return 0; }
        unsafe { &*crowditem }.animations().len()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_crowditem_get_instances_ver(crowditem: *const CrowdItemVER) -> *const CrowdValVER {
        if crowditem.is_null() { return null(); }
        unsafe { &*crowditem }.instances().as_ptr()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_crowditem_get_instances_num_ver(crowditem: *const CrowdItemVER) -> size_t {
        if crowditem.is_null() { return 0; }
        unsafe { &*crowditem }.instances().len()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_crowd_get_header_ver(crowd: *const CrowdVER) -> *const CrowdHeaderVER {
        if crowd.is_null() { return null(); }
        unsafe { &*crowd }.header() as _
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_crowd_get_offs_ver(crowd: *const CrowdVER) -> *const u32VER {
        if crowd.is_null() { return null(); }
        unsafe { &*crowd }.offs().as_ptr()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_crowd_get_offs_num_ver(crowd: *const CrowdVER) -> size_t {
        if crowd.is_null() { return 0; }
        unsafe { &*crowd }.offs().len()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_crowd_get_vals_ver(crowd: *const CrowdVER) -> *const CrowdItemVER {
        if crowd.is_null() { return null(); }
        unsafe { &*crowd }.vals().as_ptr()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_crowd_get_vals_num_ver(crowd: *const CrowdVER) -> size_t {
        if crowd.is_null() { return 0; }
        unsafe { &*crowd }.vals().len()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_atlasuv_get_vals_ver(atlasuv: *const AtlasUVVER) -> *const AtlasUVValVER {
        if atlasuv.is_null() { return null(); }
        unsafe { &*atlasuv }.vals().as_ptr()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_atlasuv_get_vals_num_ver(atlasuv: *const AtlasUVVER) -> size_t {
        if atlasuv.is_null() { return 0; }
        unsafe { &*atlasuv }.vals().len()
    }
}
