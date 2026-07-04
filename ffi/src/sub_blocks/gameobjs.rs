use std::ptr::{slice_from_raw_parts_mut, null};
use std::ffi::{c_char};
use lotrc::macros::make_platforms;
use stdint::size_t;
use zerocopy::IntoBytes; 

#[repr(u8)]
pub enum BaseType {
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
    Err = 255
}

#[make_platforms]
pub mod ref_impl_ver {
    use super::*; 
    use lotrc::{
        sub_blocks::gameobjs::{
            GameObjsVER, ObjVER, TypeVER, GameObjsHeaderVER, BaseTypeVER, ObjHeaderVER,
            TypeHeaderVER, TypeFieldVER, BaseTypeRefVER
        },
        types::{CrcVER, u32VER, ColorVER, Vector2VER, Vector3VER, Vector4VER, Matrix4x4VER, f32VER, i32VER, WeightVER}
    };
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetypes_test_ver(basetypes: &BaseTypeRefVER) {
        
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_gameobjs_get_header_ver(gameobjs: *const GameObjsVER) -> *const GameObjsHeaderVER {
        if gameobjs.is_null() { return null(); }
        unsafe { &*gameobjs }.header() as _
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_gameobjs_get_types_num_ver(gameobjs: *const GameObjsVER) -> size_t  {
        if gameobjs.is_null() { return 0; }
        unsafe { &*gameobjs }.types().len()
    }
    #[unsafe(no_mangle)]
    /// types is a caller allocated array for returning type keys
    pub extern "C" fn lotrc_gameobjs_get_types_ver(gameobjs: *const GameObjsVER, types: *mut u32) {
        if gameobjs.is_null() || types.is_null() { return; }
        let gameobjs = unsafe { &*gameobjs };
        let types = unsafe { &mut*slice_from_raw_parts_mut(types, gameobjs.types().len()) };
        for (src, dst) in gameobjs.types().keys().zip(types) {
            *dst = *src;
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_gameobjs_get_type_ver(gameobjs: *const GameObjsVER, key: u32) -> *const TypeVER {
        if gameobjs.is_null() { return null(); }
        unsafe { &*gameobjs }.types().get(&key).map(|x| x as _).unwrap_or(null())
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_gameobjs_get_objs_num_ver(gameobjs: *const GameObjsVER) -> size_t {
        if gameobjs.is_null() { return 0; }
        unsafe { &*gameobjs }.objs().len()
    }
    #[unsafe(no_mangle)]
    /// objs is a caller allocated array for returning obj keys
    pub extern "C" fn lotrc_gameobjs_get_objs_ver(gameobjs: *const GameObjsVER, objs: *mut u32) {
        if gameobjs.is_null() || objs.is_null() { return; }
        let gameobjs = unsafe { &*gameobjs };
        let types = unsafe { &mut*slice_from_raw_parts_mut(objs, gameobjs.objs().len()) };
        for (src, dst) in gameobjs.objs().keys().zip(types) {
            *dst = *src;
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_gameobjs_get_obj_ver(gameobjs: *const GameObjsVER, key: u32) -> *const ObjVER {
        if gameobjs.is_null() { return null() };
        unsafe { &*gameobjs }.objs().get(&key).map(|x| x as _).unwrap_or(null())
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_obj_get_header_ver(obj: *const ObjVER) -> *const ObjHeaderVER {
        if obj.is_null() { return null(); }
        unsafe { &*obj }.header() as _
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_obj_get_fields_num_ver(obj: *const ObjVER) -> size_t {
        if obj.is_null() { return 0; }
        unsafe { &*obj }.fields().len()
    }
    #[unsafe(no_mangle)]
    /// fields is a caller allocated array for returning field keys
    pub extern "C" fn lotrc_obj_get_fields_ver(obj: *const ObjVER, fields: *mut u32) {
        if obj.is_null() || fields.is_null() { return; }
        let obj = unsafe { &*obj };
        let fields = unsafe { &mut*slice_from_raw_parts_mut(fields, obj.fields().len()) };
        for (src, dst) in obj.fields().keys().zip(fields) {
            *dst = *src;
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_obj_get_field_ver(obj: *const ObjVER, key: u32) -> *const BaseTypeVER {
        if obj.is_null() { return null(); }
        unsafe { &*obj }.fields().get(&key).map(|x| x as _).unwrap_or(null())
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_type_get_header_ver(ty: *const TypeVER) -> *const TypeHeaderVER {
        if ty.is_null() { return null(); }
        unsafe { &*ty }.header() as _
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_type_get_fields_num_ver(ty: *const TypeVER) -> size_t {
        if ty.is_null() { return 0; }
        unsafe { &*ty }.fields().len()
    }
    #[unsafe(no_mangle)]
    /// fields is a caller allocated array for returning fields
    pub extern "C" fn lotrc_type_get_fields_ver(ty: *const TypeVER, fields: *mut TypeFieldVER) {
        if ty.is_null() || fields.is_null() { return; }
        let ty = unsafe { &*ty };
        let fields = unsafe { &mut*slice_from_raw_parts_mut(fields, ty.fields().len()) };
        ty.fields().write_to(fields.as_mut_bytes()).unwrap();
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_type_ver(basetype: *const BaseTypeVER) -> BaseType {
        if basetype.is_null() { return BaseType::Err; }
        match unsafe { &*basetype } {
            BaseTypeVER::Crc(_) => BaseType::Crc,
            BaseTypeVER::GUID(_) => BaseType::GUID,
            BaseTypeVER::Color(_) => BaseType::Color,
            BaseTypeVER::Vector2(_) => BaseType::Vector2,
            BaseTypeVER::Vector3(_) => BaseType::Vector3,
            BaseTypeVER::Vector4(_) => BaseType::Vector4,
            BaseTypeVER::Matrix4x4(_) => BaseType::Matrix4x4,
            BaseTypeVER::Float(_) => BaseType::Float,
            BaseTypeVER::Int(_) => BaseType::Int,
            BaseTypeVER::Bool(_) => BaseType::Bool,
            BaseTypeVER::String(_) => BaseType::String,
            BaseTypeVER::StringList(_) => BaseType::StringList,
            BaseTypeVER::ObjectList(_) => BaseType::ObjectList,
            BaseTypeVER::NodeList(_) => BaseType::NodeList,
            BaseTypeVER::IntList(_) => BaseType::IntList,
            BaseTypeVER::CrcList(_) => BaseType::CrcList,
            BaseTypeVER::WeightList(_) => BaseType::WeightList,
            BaseTypeVER::MatrixList(_) => BaseType::MatrixList,
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_len_ver(basetype: *const BaseTypeVER) -> size_t {
        if basetype.is_null() { return 0; }
        match unsafe { &*basetype } {
            BaseTypeVER::String(val) => unsafe { val.as_ref() }.len(),
            BaseTypeVER::StringList(vals) => vals.len(),
            BaseTypeVER::ObjectList(vals) => unsafe { vals.as_ref() }.len(),
            BaseTypeVER::NodeList(vals) => unsafe { vals.as_ref() }.len(),
            BaseTypeVER::IntList(vals) => unsafe { vals.as_ref() }.len(),
            BaseTypeVER::CrcList(vals) => unsafe { vals.as_ref() }.len(),
            BaseTypeVER::WeightList(vals) => unsafe { vals.as_ref() }.len(),
            BaseTypeVER::MatrixList(vals) => unsafe { vals.as_ref() }.len(),
            _ => 0
        }
    }
    #[unsafe(no_mangle)]
    /// sizes is a caller allocated array for returning string sizes
    pub extern "C" fn lotrc_basetype_get_stringlist_sizes_ver(basetype: *const BaseTypeVER, sizes: *mut size_t) {
        if basetype.is_null() || sizes.is_null() { return; }
        match unsafe { &*basetype } {
            BaseTypeVER::StringList(vals) => {
                let sizes = unsafe { &mut*slice_from_raw_parts_mut(sizes, vals.len()) };
                for (s, dst) in vals.iter().zip(sizes) {
                    *dst = unsafe { s.as_ref() }.len();
                }
            },
            _ => ()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_crc_ver(basetype: *const BaseTypeVER) -> CrcVER {
        if basetype.is_null() { return CrcVER::default(); }
        match unsafe { &*basetype } {
            BaseTypeVER::Crc(val) => *unsafe { val.as_ref() },
            _ => CrcVER::default()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_guid_ver(basetype: *const BaseTypeVER) -> u32VER {
        if basetype.is_null() { return u32VER::default(); }
        match unsafe { &*basetype } {
            BaseTypeVER::GUID(val) => *unsafe { val.as_ref() },
            _ => u32VER::default()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_color_ver(basetype: *const BaseTypeVER) -> ColorVER {
        if basetype.is_null() { return ColorVER::default(); }
        match unsafe { &*basetype } {
            BaseTypeVER::Color(val) => *unsafe { val.as_ref() },
            _ => ColorVER::default()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_vector2_ver(basetype: *const BaseTypeVER) -> *const Vector2VER {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::Vector2(val) => val.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_vector3_ver(basetype: *const BaseTypeVER) -> *const Vector3VER {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::Vector2(val) => val.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_vector4_ver(basetype: *const BaseTypeVER) -> *const Vector4VER {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::Vector2(val) => val.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_matrix4x4_ver(basetype: *const BaseTypeVER) -> *const Matrix4x4VER {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::Matrix4x4(val) => val.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_float_ver(basetype: *const BaseTypeVER) -> f32VER {
        if basetype.is_null() { return f32VER::default(); }
        match unsafe { &*basetype } {
            BaseTypeVER::Float(val) => *unsafe { val.as_ref() },
            _ => f32VER::default()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_int_ver(basetype: *const BaseTypeVER) -> i32VER {
        if basetype.is_null() { return i32VER::default(); }
        match unsafe { &*basetype } {
            BaseTypeVER::Int(val) => *unsafe { val.as_ref() },
            _ => i32VER::default()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_bool_ver(basetype: *const BaseTypeVER) -> bool {
        if basetype.is_null() { return bool::default(); }
        match unsafe { &*basetype } {
            BaseTypeVER::Bool(val) => *unsafe { val.as_ref() } != 0,
            _ => bool::default()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_string_ver(basetype: *const BaseTypeVER) -> *const c_char {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::String(val) => val.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    // probably fine, but might need to pin the underlying if it misbehaves
    pub extern "C" fn lotrc_basetype_get_stringlist_ver(basetype: *const BaseTypeVER) -> *const *const c_char {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::StringList(vals) => vals.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_objectlist_ver(basetype: *const BaseTypeVER) -> *const u32VER {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::ObjectList(val) => val.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_nodelist_ver(basetype: *const BaseTypeVER) -> *const Vector4VER {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::NodeList(val) => val.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_intlist_ver(basetype: *const BaseTypeVER) -> *const i32VER {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::IntList(val) => val.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_crclist_ver(basetype: *const BaseTypeVER) -> *const CrcVER {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::CrcList(val) => val.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_weightlist_ver(basetype: *const BaseTypeVER) -> *const WeightVER {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::WeightList(val) => val.as_ptr() as _,
            _ => null()
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn lotrc_basetype_get_matrixlist_ver(basetype: *const BaseTypeVER) -> *const Matrix4x4VER {
        if basetype.is_null() { return null(); }
        match unsafe { &*basetype } {
            BaseTypeVER::MatrixList(val) => val.as_ptr() as _,
            _ => null()
        }
    }
}
