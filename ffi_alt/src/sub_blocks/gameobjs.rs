use safer_ffi::prelude::*;
use std::ptr::{null, slice_from_raw_parts_mut};
use lotrc::macros::make_platforms;
use lotrc::types::{slice, Map, str_ref};

#[derive_ReprC]
#[repr(u8)]
enum BaseType {
    Crc = 0,
    GUID,
    Color,
    Vector2,
    Vector3,
    Vector4,
    Matrix4x4,
    Float,
    Int,
    Bool,
    String,
    StringList,
    ObjectList,
    NodeList,
    IntList,
    CrcList,
    WeightList,
    MatrixList,
}

#[make_platforms]
mod impl_ver {
    use super::*;
    use lotrc::{
        sub_blocks::gameobjs::{BaseTypeRefVER, ObjRefVER, TypeRefVER, GameObjsRefVER},
        types::{u32VER, CrcVER, ColorVER, Vector2VER, Vector3VER, Vector4VER, Matrix4x4VER, f32VER, i32VER, U32VER, WeightVER},
    };
    #[ffi_export]
    pub fn lotrc_basetype_get_crc_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> CrcVER {
        basetype.crc().cloned().unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_guid_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> u32VER {
        basetype.guid().cloned().unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_color_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> ColorVER {
        basetype.color().cloned().unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_vector2_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> Vector2VER {
        basetype.vector2().cloned().unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_vector3_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> Vector3VER {
        basetype.vector3().cloned().unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_vector4_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> Vector4VER {
        basetype.vector4().cloned().unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_matrix4x4_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> Matrix4x4VER {
        basetype.matrix4x4().cloned().unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_float_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> f32VER {
        basetype.float().cloned().unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_int_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> i32VER {
        basetype.int().cloned().unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_bool_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> bool {
        basetype.bool().map(|x| x.get() != 0).unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_string_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> str_ref<'a> {
        basetype.string().unwrap_or_default().into()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_string_list_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> slice<'a, str_ref<'a>> {
        basetype.string_list().unwrap_or_default().into()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_object_list_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> slice<'a, U32VER> {
        basetype.object_list().unwrap_or_default().into()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_node_list_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> slice<'a, Vector4VER> {
        basetype.node_list().unwrap_or_default().into()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_int_list_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> slice<'a, i32VER> {
        basetype.int_list().unwrap_or_default().into()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_crc_list_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> slice<'a, U32VER> {
        basetype.crc_list().unwrap_or_default().into()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_weight_list_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> slice<'a, WeightVER> {
        basetype.weight_list().unwrap_or_default().into()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_matrix_list_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> slice<'a, Matrix4x4VER> {
        basetype.matrix_list().unwrap_or_default().into()
    }
    #[ffi_export]
    pub fn lotrc_basetype_get_type_ver<'a>(basetype: &'a BaseTypeRefVER<'a>) -> BaseType {
        match basetype {
            BaseTypeRefVER::Crc(_) => BaseType::Crc,
            BaseTypeRefVER::GUID(_) => BaseType::GUID,
            BaseTypeRefVER::Color(_) => BaseType::Color,
            BaseTypeRefVER::Vector2(_) => BaseType::Vector2,
            BaseTypeRefVER::Vector3(_) => BaseType::Vector3,
            BaseTypeRefVER::Vector4(_) => BaseType::Vector4,
            BaseTypeRefVER::Matrix4x4(_) => BaseType::Matrix4x4,
            BaseTypeRefVER::Float(_) => BaseType::Float,
            BaseTypeRefVER::Int(_) => BaseType::Int,
            BaseTypeRefVER::Bool(_) => BaseType::Bool,
            BaseTypeRefVER::String(_) => BaseType::String,
            BaseTypeRefVER::StringList(_) => BaseType::StringList,
            BaseTypeRefVER::ObjectList(_) => BaseType::ObjectList,
            BaseTypeRefVER::NodeList(_) => BaseType::NodeList,
            BaseTypeRefVER::IntList(_) => BaseType::IntList,
            BaseTypeRefVER::CrcList(_) => BaseType::CrcList,
            BaseTypeRefVER::WeightList(_) => BaseType::WeightList,
            BaseTypeRefVER::MatrixList(_) => BaseType::MatrixList,
        }
    }

    #[ffi_export]
    pub fn lotrc_obj_from_data_ver<'a, 'b>(src: slice<'a, u8>, types: &'b Map<u32, TypeRefVER<'b>>, err: Out<'_, bool>) -> ObjRefVER<'a> {
        let val = ObjRefVER::from_data(src.as_slice(), types);
        err.write(val.is_err());
        val.unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_type_from_data_ver<'a>(src: slice<'a, u8>, err: Out<'_, bool>) -> TypeRefVER<'a> {
        let val = TypeRefVER::from_data(src.as_slice());
        err.write(val.is_err());
        val.unwrap_or_default()
    }
    #[ffi_export]
    pub fn lotrc_gameobjs_from_data_ver<'a>(src: slice<'a, u8>, gamemodemask: i32, err: Out<'_, bool>) -> GameObjsRefVER<'a> {
        let val = GameObjsRefVER::from_data(src.as_slice(), gamemodemask);
        err.write(val.is_err());
        val.unwrap_or_default()
    }


    #[ffi_export]
    pub fn lotrc_type_map_get_ver<'a>(map: &'a Map<u32, TypeRefVER<'a>>, key: u32) -> *const TypeRefVER<'a> {
        let val = map.get(&key);
        val.map(|x| x as *const _).unwrap_or(null())
    }
    #[ffi_export]
    pub fn lotrc_type_map_len_ver<'a>(map: &'a Map<u32, TypeRefVER<'a>>) -> usize {
        map.len()
    }
    #[ffi_export]
    /// keys is a caller allocated array for returning keys
    pub fn lotrc_type_map_keys_ver<'a>(map: &'a Map<u32, TypeRefVER<'a>>, keys: *mut u32) {
        if keys.is_null() { return; }
        let keys = unsafe { &mut*slice_from_raw_parts_mut(keys, map.len()) };
        for (src, dst) in map.keys().zip(keys) {
            *dst = *src;
        }
    }
    #[ffi_export]
    pub fn lotrc_obj_map_get_ver<'a>(map: &'a Map<u32, ObjRefVER<'a>>, key: u32) -> *const ObjRefVER<'a> {
        let val = map.get(&key);
        val.map(|x| x as *const _).unwrap_or(null())
    }
    #[ffi_export]
    pub fn lotrc_obj_map_len_ver<'a>(map: &'a Map<u32, ObjRefVER<'a>>) -> usize {
        map.len()
    }
    #[ffi_export]
    /// keys is a caller allocated array for returning keys
    pub fn lotrc_obj_map_keys_ver<'a>(map: &'a Map<u32, ObjRefVER<'a>>, keys: *mut u32) {
        if keys.is_null() { return; }
        let keys = unsafe { &mut*slice_from_raw_parts_mut(keys, map.len()) };
        for (src, dst) in map.keys().zip(keys) {
            *dst = *src;
        }
    }
    #[ffi_export]
    pub fn lotrc_basetype_map_get_ver<'a>(map: &'a Map<u32, BaseTypeRefVER<'a>>, key: u32) -> *const BaseTypeRefVER<'a> {
        let val = map.get(&key);
        val.map(|x| x as *const _).unwrap_or(null())
    }
    #[ffi_export]
    pub fn lotrc_basetype_map_len_ver<'a>(map: &'a Map<u32, BaseTypeRefVER<'a>>) -> usize {
        map.len()
    }
    #[ffi_export]
    /// keys is a caller allocated array for returning keys
    pub fn lotrc_basetype_map_keys_ver<'a>(map: &'a Map<u32, BaseTypeRefVER<'a>>, keys: *mut u32) {
        if keys.is_null() { return; }
        let keys = unsafe { &mut*slice_from_raw_parts_mut(keys, map.len()) };
        for (src, dst) in map.keys().zip(keys) {
            *dst = *src;
        }
    }
}
