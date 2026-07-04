#![allow(non_camel_case_types)]
use lotrc::macros::{make_platforms, export};
use lotrc::types::AlignedBuf;
use std::ptr::NonNull;

crate::make_owned_opaque!(OwnedAlignedBuf, AlignedBuf);
#[export(mod_name=AlignedBuf)]
mod aligned_buf {
    use super::*;
    fn with_capacity(size: usize) -> NonNull<OwnedAlignedBuf> {
        OwnedAlignedBuf::leak(AlignedBuf::with_capacity(size))
    }
}

pub mod wrappers {
    // Aliases used for underlying types disregarding endianess

    pub type u64_le = u64;
    pub type u64_be = u64;
    pub type u32_le = u32;
    pub type u32_be = u32;
    pub type u16_le = u16;
    pub type u16_be = u16;
    pub type i32_le = i32;
    pub type i32_be = i32;
    pub type i16_le = i16;
    pub type i16_be = i16;
    pub type f32_le = f32;
    pub type f32_be = f32;

    // Should really be structs with align(1) that map to c

    pub type U16LE = u16;
    pub type U16BE = u16;
    pub type U32LE = u32;
    pub type U32BE = u32;
    pub type I32LE = i32;
    pub type I32BE = i32;

    // placeholder to force it to generate opaque bindings
    pub struct IndexMap<A, B>(A, B);
}

macro_rules! assert_layout {
    ($t1:ty, $t2:ty) => {
        /// cbindgen:ignore
        const _: () = assert!(std::mem::size_of::<$t1>() == std::mem::size_of::<$t2>());
        /// cbindgen:ignore
        const _: () = assert!(std::mem::align_of::<$t1>() == std::mem::align_of::<$t2>());
    }
}

/// This is a proxy struct for correct size and alignment only
/// don't construct this directly and use the provided methods to access
#[repr(C)]
pub struct string {
    align: u64,
    pad: [u8; 8]
}
assert_layout!(lotrc::types::string, string);
#[lotrc::macros::export(base_only)]
impl string {
    fn get<'a>(string: Option<&lotrc::types::string>) -> *const u8 {
        string.map(|x| x.as_ptr()).unwrap_or(std::ptr::null())
    }
    fn len(string: Option<&lotrc::types::string>) -> usize {
        string.map(|x| x.len()).unwrap_or_default()
    }
}

#[macro_export]
macro_rules! make_owned_opaque {
    ($name:ident, $t:ty) => {
        /// Owned
        pub struct $name {
            pub val: $t
        }

        #[lotrc::macros::export(base_only)]
        impl $name {
            fn get(val: Option<NonNull<$name>>) -> Option<NonNull<$t>> {
                Some((&mut unsafe { val?.as_mut() }.val).into())
            }
            fn free(val: Option<NonNull<$name>>) {
                if let Some(val) = val {
                    drop(Box::from(val))
                }
            }
        }

        impl $name {
            pub fn leak(val: $t) -> NonNull<$name> {
                Box::leak(Box::new($name { val })).into()
            }
        }
    };
    ($name:ident, $t:ty, $l:lifetime) => {
        /// Owned
        pub struct $name<$l> {
            pub val: $t
        }

        #[lotrc::macros::export(impl_name=$name)]
        impl <$l> $name<$l> {
            fn get(val: Option<NonNull<Self>>) -> Option<NonNull<$t>> {
                Some((&mut unsafe { val?.as_mut() }.val).into())
            }
            fn free(val: Option<NonNull<Self>>) {
                if let Some(val) = val {
                    drop(Box::from(val))
                }
            }
        }

        impl <$l> $name<$l> {
            pub fn leak(val: $t) -> NonNull<Self> {
                Box::leak(Box::new($name { val })).into()
            }
        }
    };
}

macro_rules! make_indexmap_wrapper {
    ($name:ident, $key:ty, $val:ty) => {
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; 64]
        }
        assert_layout!(lotrc::re_export::IndexMap<$key, $val>, $name);
        #[lotrc::macros::export(base_only)]
        impl $name {
            fn get<'a>(map: Option<&'a lotrc::re_export::IndexMap<$key, $val>>, key: Option<&$key>) -> Option<&'a $val> {
                map?.get(key?)
            }
            fn len(map: Option<&lotrc::re_export::IndexMap<$key, $val>>) -> usize {
                map.map(|x| x.len()).unwrap_or_default()
            }
            fn keys(map: Option<&lotrc::re_export::IndexMap<$key, $val>>, keys: Option<&mut lotrc::types::mut_slice<$key>>) {
                if let (Some(map), Some(keys)) = (map, keys) {
                    for (src, dst) in map.keys().zip(keys.iter_mut()) {
                        *dst = *src;
                    }
                }
            }
        }
    };
    ($name:ident, $key:ty, $val:ty, $l:ty) => {
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; 64]
        }
        assert_layout!(lotrc::re_export::IndexMap<$key, $val>, $name);
        #[lotrc::macros::export(base_only)]
        impl $name {
            fn get<'a>(map: Option<&'a lotrc::re_export::IndexMap<$key, $l>>, key: Option<&$key>) -> Option<&'a $l> {
                map?.get(key?)
            }
            fn len(map: Option<&lotrc::re_export::IndexMap<$key, $val>>) -> usize {
                map.map(|x| x.len()).unwrap_or_default()
            }
            fn keys(map: Option<&lotrc::re_export::IndexMap<$key, $val>>, keys: Option<&mut lotrc::types::mut_slice<$key>>) {
                if let (Some(map), Some(keys)) = (map, keys) {
                    for (src, dst) in map.keys().zip(keys.iter_mut()) {
                        *dst = *src;
                    }
                }
            }
        }
    }
}
macro_rules! make_vec_wrapper {
    ($name:ident, $t:ty) => {
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; 16]
        }
        assert_layout!(Vec<$t>, $name);
    };
    ($name:ident, $t:ty, $lt:ty) => {
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; 16]
        }
        assert_layout!(Vec<$t>, $name);
    };
}


macro_rules! make_option_wrapper {
    ($name:ident, $t:ty, $n:tt) => {
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; $n] 
        }
        assert_layout!(Option<$t>, $name);
        #[lotrc::macros::export(base_only)]
        impl $name {
            fn get<'a>(slice: Option<&'a Option<$t>>) -> Option<&'a $t> {
                slice.and_then(|x| x.as_ref())
            }
        }
    };
    ($name:ident, $t:ty, $l:ty, $n:tt) => {
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; $n]
        }
        assert_layout!(Option<$t>, $name);
        #[lotrc::macros::export(base_only)]
        impl $name {
            fn get<'a>(slice: Option<&'a Option<$l>>) -> Option<&'a $l> {
                slice.and_then(|x| x.as_ref())
            }
        }
    }
}
macro_rules! make_ref_slice_wrapper {
    ($name:ident, $t:ty) => {
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; 8]
        }
        assert_layout!(lotrc::types::ref_slice<'_, $t>, $name);
        #[lotrc::macros::export(base_only)]
        impl $name {
            fn get<'a>(slice: Option<&lotrc::types::ref_slice<'a, $t>>, idx: usize) -> Option<&'a $t> {
                slice.and_then(|x| x.get(idx))
            }
            fn len(slice: Option<&lotrc::types::ref_slice<$t>>) -> usize {
                slice.map(|x| x.len()).unwrap_or_default()
            }
        }
    }
}
macro_rules! make_mut_slice_wrapper {
    ($name:ident, $t:ty) => {
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; 8]
        }
        assert_layout!(lotrc::types::mut_slice<'_, $t>, $name);
        #[lotrc::macros::export(base_only)]
        impl $name {
            fn get<'a>(slice: Option<&'a mut lotrc::types::mut_slice<'a, $t>>, idx: usize) -> Option<&'a mut $t> {
                slice.and_then(|x| x.get_mut(idx))
            }
            fn len(slice: Option<&lotrc::types::slice<$t>>) -> usize {
                slice.map(|x| x.len()).unwrap_or_default()
            }
        }
    }
}
macro_rules! make_slice_wrapper {
    ($name:ident, $t:ty) => {
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; 8]
        }
        assert_layout!(lotrc::types::slice<$t>, $name);
        #[lotrc::macros::export(base_only)]
        impl $name {
            fn get<'a>(slice: Option<&'a lotrc::types::slice<$t>>, idx: usize) -> Option<&'a $t> {
                slice.and_then(|x| x.get(idx))
            }
            fn len(slice: Option<&lotrc::types::slice<$t>>) -> usize {
                slice.map(|x| x.len()).unwrap_or_default()
            }
        }
    };
    ($name:ident, $t:ty, $l:ty) => {
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; 8]
        }
        assert_layout!(lotrc::types::slice<$t>, $name);
        #[lotrc::macros::export(base_only)]
        impl $name {
            fn get<'a>(slice: Option<&'a lotrc::types::slice<$l>>, idx: usize) -> Option<&'a $l> {
                slice.and_then(|x| x.get(idx))
            }
            fn len(slice: Option<&lotrc::types::slice<$t>>) -> usize {
                slice.map(|x| x.len()).unwrap_or_default()
            }
        }
    }
}

make_indexmap_wrapper!(
    IndexMap_u32__CompressedDataRef,
    u32,
    lotrc::types::CompressedDataRef,
    lotrc::types::CompressedDataRef<'a>
);
make_indexmap_wrapper!(
    IndexMap_u32__ref_slice_u8,
    u32,
    lotrc::types::ref_slice<'_, u8>,
    lotrc::types::ref_slice<'a, u8>
);
make_indexmap_wrapper!(
    IndexMap_u32_______CompressedDataRef,
    u32,
    &lotrc::types::CompressedDataRef,
    &'a lotrc::types::CompressedDataRef<'a>
);
make_indexmap_wrapper!(
    IndexMap_VertexUsage__VertexDataIndex,
    lotrc::level::model::data::VertexUsage, 
    lotrc::level::model::data::VertexDataIndex
);

make_ref_slice_wrapper!(ref_slice_u8, u8);
make_ref_slice_wrapper!(ref_slice_u32, u32);
make_ref_slice_wrapper!(ref_slice_VertexUsage, lotrc::level::model::data::VertexUsage);
make_mut_slice_wrapper!(mut_slice_u8, u8);
make_mut_slice_wrapper!(mut_slice_VertexUsage, lotrc::level::model::data::VertexUsage);
make_mut_slice_wrapper!(mut_slice_u32, u32);
make_slice_wrapper!(
    slice_string, 
    lotrc::types::string,
    lotrc::types::string<'a>
);
make_slice_wrapper!(
    slice_CompressedDataRef,
    lotrc::types::CompressedDataRef,
    lotrc::types::CompressedDataRef<'a>
);
make_slice_wrapper!(
    slice_AlignmentHelper,
    lotrc::types::AlignmentHelper
);
make_slice_wrapper!(
    slice______CompressedDataRef,
    &lotrc::types::CompressedDataRef,
    &'a lotrc::types::CompressedDataRef<'a>
);
make_slice_wrapper!(
    ref_slice_CompressedDataRef,
    lotrc::types::CompressedDataRef,
    lotrc::types::CompressedDataRef<'a>
);


#[make_platforms]
mod wrapper_ver {
    make_indexmap_wrapper!(
        IndexMap_u32__IndexBufferRefVER, 
        u32, 
        lotrc::level::model::data::IndexBufferRefVER, 
        lotrc::level::model::data::IndexBufferRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__VertexBufferRefVER, 
        u32, 
        lotrc::level::model::data::VertexBufferRefVER, 
        lotrc::level::model::data::VertexBufferRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__MatRefVER, 
        u32,
        lotrc::level::model::mat::MatRefVER,
        lotrc::level::model::mat::MatRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__ModelRefVER,
        u32,
        lotrc::level::model::ModelRefVER,
        lotrc::level::model::ModelRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32_______IBuffInfoVER,
        u32,
        &lotrc::level::model::data::IBuffInfoVER,
        &'a lotrc::level::model::data::IBuffInfoVER
    );
    make_indexmap_wrapper!(
        IndexMap_u32_______VBuffInfoVER,
        u32,
        &lotrc::level::model::data::VBuffInfoVER,
        &'a lotrc::level::model::data::VBuffInfoVER
    );
    make_indexmap_wrapper!(
        IndexMap_u32__AnimationRefVER,
        u32,
        lotrc::level::pak::animation::AnimationRefVER,
        lotrc::level::pak::animation::AnimationRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__DataRefVER,
        u32,
        lotrc::types::sub_blocks::DataRefVER,
        lotrc::types::sub_blocks::DataRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__EffectRefVER,
        u32,
        lotrc::level::pak::block1::objs::EffectRefVER,
        lotrc::level::pak::block1::objs::EffectRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__LangStringsRefVER,
        u32,
        lotrc::level::pak::block2::LangStringsRefVER,
        lotrc::level::pak::block2::LangStringsRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__LuaRefVER,
        u32,
        lotrc::level::pak::block1::sub_blocks::LuaRefVER,
        lotrc::level::pak::block1::sub_blocks::LuaRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__ObjRefVER,
        u32,
        lotrc::level::pak::block1::gameobjs::ObjRefVER,
        lotrc::level::pak::block1::gameobjs::ObjRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__RadiosityValsRefVER,
        u32,
        lotrc::level::radiosity::RadiosityValsRefVER,
        lotrc::level::radiosity::RadiosityValsRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__SSARefVER,
        u32,
        lotrc::level::pak::block1::sub_blocks::SSARefVER,
        lotrc::level::pak::block1::sub_blocks::SSARefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__TextureRefVER,
        u32,
        lotrc::level::texture::TextureRefVER,
        lotrc::level::texture::TextureRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__TypeRefVER,
        u32,
        lotrc::level::pak::block1::gameobjs::TypeRefVER,
        lotrc::level::pak::block1::gameobjs::TypeRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__slice_FoliageRefVER,
        u32,
        lotrc::types::slice<lotrc::level::pak::block1::objs::FoliageRefVER>,
        lotrc::types::slice<lotrc::level::pak::block1::objs::FoliageRefVER<'a>>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__BaseTypeRefVER,
        u32,
        lotrc::level::pak::block1::gameobjs::BaseTypeRefVER,
        lotrc::level::pak::block1::gameobjs::BaseTypeRefVER<'a>
    );
    make_indexmap_wrapper!(
        IndexMap_u32__ref_slice_u16VER,
        u32,
        lotrc::types::ref_slice<'_, lotrc::types::u16VER>,
        lotrc::types::ref_slice<'a, lotrc::types::u16VER>
    );

    make_slice_wrapper!(
        slice_BlockRefVER,
        lotrc::level::model::BlockRefVER,
        lotrc::level::model::BlockRefVER<'a>
    );
    make_slice_wrapper!(
        slice_ShapeRefVER,
        lotrc::level::model::shape::ShapeRefVER,
        lotrc::level::model::shape::ShapeRefVER<'a>
    );
    make_slice_wrapper!(owned_slice_HkShapeRefVER,
        lotrc::level::model::shape::HkShapeRefVER,
        lotrc::level::model::shape::HkShapeRefVER<'a>
    );
    make_slice_wrapper!(
        slice_FoliageRefVER,
        lotrc::level::pak::block1::objs::FoliageRefVER,
        lotrc::level::pak::block1::objs::FoliageRefVER<'a>
    );
    make_slice_wrapper!(
        slice_ref_slice_u16VER,
        lotrc::types::ref_slice<'_, lotrc::types::u16VER>,
        lotrc::types::ref_slice<'a, lotrc::types::u16VER>
    );
    make_slice_wrapper!(
        slice_BlockValRefVER,
        lotrc::level::pak::animation::BlockValRefVER,
        lotrc::level::pak::animation::BlockValRefVER<'a>
    );
    make_slice_wrapper!(
        slice_CrowdItemRefVER,
        lotrc::level::pak::block2::CrowdItemRefVER,
        lotrc::level::pak::block2::CrowdItemRefVER<'a>
    );
    make_slice_wrapper!(
        slice_HkConstraintBoneRefVER,
        lotrc::level::model::shape::HkConstraintBoneRefVER,
        lotrc::level::model::shape::HkConstraintBoneRefVER<'a>
    );
    make_slice_wrapper!(
        slice_BlockValARefVER,
        lotrc::level::pak::animation::BlockValARefVER,
        lotrc::level::pak::animation::BlockValARefVER<'a>
    );
    make_slice_wrapper!(
        slice_Obj1RefVER,
        lotrc::level::pak::animation::Obj1RefVER,
        lotrc::level::pak::animation::Obj1RefVER<'a>
    );
    make_slice_wrapper!(
        slice_HkShapeRefVER,
        lotrc::level::model::shape::HkShapeRefVER,
        lotrc::level::model::shape::HkShapeRefVER<'a>
    );

    make_ref_slice_wrapper!(ref_slice_BoundingBoxVER, lotrc::level::model::BoundingBoxVER);
    make_ref_slice_wrapper!(ref_slice_BufferInfoVER, lotrc::level::model::data::BufferInfoVER);
    make_ref_slice_wrapper!(ref_slice_CrcVER, lotrc::types::CrcVER);
    make_ref_slice_wrapper!(ref_slice_HkConstraintDataVER, lotrc::level::model::shape::HkConstraintDataVER);
    make_ref_slice_wrapper!(ref_slice_Key2VER, lotrc::level::model::Key2VER);
    make_ref_slice_wrapper!(ref_slice_Matrix4x4VER, lotrc::types::Matrix4x4VER);
    make_ref_slice_wrapper!(ref_slice_Vector3VER, lotrc::types::Vector3VER);
    make_ref_slice_wrapper!(ref_slice_Vector4VER, lotrc::types::Vector4VER);
    make_ref_slice_wrapper!(ref_slice_i32VER, lotrc::types::i32VER);
    make_ref_slice_wrapper!(ref_slice_u16VER, lotrc::types::u16VER);
    make_ref_slice_wrapper!(ref_slice_u32VER, lotrc::types::u32VER);
    make_ref_slice_wrapper!(ref_slice_BlockValAVER, lotrc::level::model::BlockValAVER);
    make_ref_slice_wrapper!(ref_slice_BlockValBVER, lotrc::level::model::BlockValBVER);
    make_ref_slice_wrapper!(ref_slice_AnimationBlockInfoVER, lotrc::level::pak::animation::AnimationBlockInfoVER);
    make_ref_slice_wrapper!(ref_slice_AnimationInfoVER, lotrc::level::pak::animation::AnimationInfoVER);
    make_ref_slice_wrapper!(ref_slice_AssetHandleVER, lotrc::level::bin::AssetHandleVER);
    make_ref_slice_wrapper!(ref_slice_BlockAValVER, lotrc::level::pak::BlockAValVER);
    make_ref_slice_wrapper!(ref_slice_EffectInfoVER, lotrc::level::pak::block1::objs::EffectInfoVER);
    make_ref_slice_wrapper!(ref_slice_FoliageInfoVER, lotrc::level::pak::block1::objs::FoliageInfoVER);
    make_ref_slice_wrapper!(ref_slice_GFXBlockInfoVER, lotrc::level::pak::block1::objs::GFXBlockInfoVER);
    make_ref_slice_wrapper!(ref_slice_HkConstraintInfoVER, lotrc::level::model::shape::HkConstraintInfoVER);
    make_ref_slice_wrapper!(ref_slice_HkShapeInfoVER, lotrc::level::model::shape::HkShapeInfoVER);
    make_ref_slice_wrapper!(ref_slice_IBuffInfoVER, lotrc::level::model::data::IBuffInfoVER);
    make_ref_slice_wrapper!(ref_slice_Mat1VER, lotrc::level::model::mat::Mat1VER);
    make_ref_slice_wrapper!(ref_slice_Mat2VER, lotrc::level::model::mat::Mat2VER);
    make_ref_slice_wrapper!(ref_slice_Mat3VER, lotrc::level::model::mat::Mat3VER);
    make_ref_slice_wrapper!(ref_slice_Mat4VER, lotrc::level::model::mat::Mat4VER);
    make_ref_slice_wrapper!(ref_slice_MatExtraVER, lotrc::level::model::mat::MatExtraVER);
    make_ref_slice_wrapper!(ref_slice_ModelInfoVER, lotrc::level::model::ModelInfoVER);
    make_ref_slice_wrapper!(ref_slice_Obj0VER, lotrc::level::pak::block1::objs::Obj0VER);
    make_ref_slice_wrapper!(ref_slice_ObjAVER, lotrc::level::pak::block1::objs::ObjAVER);
    make_ref_slice_wrapper!(ref_slice_PFieldInfoVER, lotrc::level::pak::block1::objs::PFieldInfoVER);
    make_ref_slice_wrapper!(ref_slice_RadiosityValsInfoVER, lotrc::level::radiosity::RadiosityValsInfoVER);
    make_ref_slice_wrapper!(ref_slice_ShapeInfoVER, lotrc::level::model::shape::ShapeInfoVER);
    make_ref_slice_wrapper!(ref_slice_StringKeysValVER, lotrc::types::StringKeysValVER);
    make_ref_slice_wrapper!(ref_slice_SubBlocksBlockHeaderVER, lotrc::types::sub_blocks::SubBlocksBlockHeaderVER);
    make_ref_slice_wrapper!(ref_slice_TextureInfoVER, lotrc::level::texture::TextureInfoVER);
    make_ref_slice_wrapper!(ref_slice_VBuffInfoVER, lotrc::level::model::data::VBuffInfoVER);
    make_ref_slice_wrapper!(ref_slice_Obj3VER, lotrc::level::pak::animation::Obj3VER);
    make_ref_slice_wrapper!(ref_slice_Obj5ValVER, lotrc::level::pak::animation::Obj5ValVER);
    make_ref_slice_wrapper!(ref_slice_SSAValVER, lotrc::level::pak::block1::sub_blocks::SSAValVER);
    make_ref_slice_wrapper!(ref_slice_TypeFieldVER, lotrc::level::pak::block1::gameobjs::TypeFieldVER);
    make_ref_slice_wrapper!(ref_slice_FoliageValVER, lotrc::level::pak::block1::objs::FoliageValVER);
    make_ref_slice_wrapper!(ref_slice_U32VER, lotrc::types::U32VER);
    make_ref_slice_wrapper!(ref_slice_WeightVER, lotrc::types::WeightVER);
    make_ref_slice_wrapper!(ref_slice_AtlasUVValVER, lotrc::level::pak::block1::sub_blocks::AtlasUVValVER);
    make_ref_slice_wrapper!(ref_slice_SprayInstanceVER, lotrc::level::pak::block2::SprayInstanceVER);
    make_ref_slice_wrapper!(ref_slice_SprayValVER, lotrc::level::pak::block2::SprayValVER);
    make_ref_slice_wrapper!(ref_slice_TRSVER, lotrc::level::model::shape::TRSVER);
    make_ref_slice_wrapper!(ref_slice_f32VER, lotrc::types::f32VER);
    make_ref_slice_wrapper!(ref_slice_i16VER, lotrc::types::i16VER);
    make_ref_slice_wrapper!(ref_slice_CrowdValVER, lotrc::level::pak::block2::CrowdValVER);
    make_ref_slice_wrapper!(ref_slice_RotationPolar32VER, lotrc::level::pak::animation::RotationPolar32VER);
    make_ref_slice_wrapper!(ref_slice_RotationStraight16VER, lotrc::level::pak::animation::RotationStraight16VER);
    make_ref_slice_wrapper!(ref_slice_RotationThreeComp24VER, lotrc::level::pak::animation::RotationThreeComp24VER);
    make_ref_slice_wrapper!(ref_slice_RotationThreeComp40VER, lotrc::level::pak::animation::RotationThreeComp40VER);
    make_ref_slice_wrapper!(ref_slice_RotationThreeComp48VER, lotrc::level::pak::animation::RotationThreeComp48VER);
    make_ref_slice_wrapper!(ref_slice_RotationUncompressedVER, lotrc::level::pak::animation::RotationUncompressedVER);

    make_mut_slice_wrapper!(mut_slice_u32VER, lotrc::types::u32VER);
	make_mut_slice_wrapper!(mut_slice_AnimationBlockInfoVER, lotrc::level::pak::animation::AnimationBlockInfoVER);
	make_mut_slice_wrapper!(mut_slice_AnimationInfoVER, lotrc::level::pak::animation::AnimationInfoVER);
	make_mut_slice_wrapper!(mut_slice_BufferInfoVER, lotrc::level::model::data::BufferInfoVER);
	make_mut_slice_wrapper!(mut_slice_EffectInfoVER, lotrc::level::pak::block1::objs::EffectInfoVER);
	make_mut_slice_wrapper!(mut_slice_FoliageInfoVER, lotrc::level::pak::block1::objs::FoliageInfoVER);
	make_mut_slice_wrapper!(mut_slice_GFXBlockInfoVER, lotrc::level::pak::block1::objs::GFXBlockInfoVER);
	make_mut_slice_wrapper!(mut_slice_HkConstraintDataVER, lotrc::level::model::shape::HkConstraintDataVER);
	make_mut_slice_wrapper!(mut_slice_HkConstraintInfoVER, lotrc::level::model::shape::HkConstraintInfoVER);
	make_mut_slice_wrapper!(mut_slice_HkShapeInfoVER, lotrc::level::model::shape::HkShapeInfoVER);
	make_mut_slice_wrapper!(mut_slice_IBuffInfoVER, lotrc::level::model::data::IBuffInfoVER);
	make_mut_slice_wrapper!(mut_slice_Mat1VER, lotrc::level::model::mat::Mat1VER);
	make_mut_slice_wrapper!(mut_slice_Mat2VER, lotrc::level::model::mat::Mat2VER);
	make_mut_slice_wrapper!(mut_slice_Mat3VER, lotrc::level::model::mat::Mat3VER);
	make_mut_slice_wrapper!(mut_slice_Mat4VER, lotrc::level::model::mat::Mat4VER);
	make_mut_slice_wrapper!(mut_slice_MatExtraVER, lotrc::level::model::mat::MatExtraVER);
	make_mut_slice_wrapper!(mut_slice_ModelInfoVER, lotrc::level::model::ModelInfoVER);
	make_mut_slice_wrapper!(mut_slice_Obj0VER, lotrc::level::pak::block1::objs::Obj0VER);
	make_mut_slice_wrapper!(mut_slice_ObjAVER, lotrc::level::pak::block1::objs::ObjAVER);
	make_mut_slice_wrapper!(mut_slice_PFieldInfoVER, lotrc::level::pak::block1::objs::PFieldInfoVER);
	make_mut_slice_wrapper!(mut_slice_RadiosityValsInfoVER, lotrc::level::radiosity::RadiosityValsInfoVER);
	make_mut_slice_wrapper!(mut_slice_ShapeInfoVER, lotrc::level::model::shape::ShapeInfoVER);
	make_mut_slice_wrapper!(mut_slice_TextureInfoVER, lotrc::level::texture::TextureInfoVER);
	make_mut_slice_wrapper!(mut_slice_VBuffInfoVER, lotrc::level::model::data::VBuffInfoVER);

    make_option_wrapper!(
        Option_HkConstraintRefVER, 
        lotrc::level::model::shape::HkConstraintRefVER,
        lotrc::level::model::shape::HkConstraintRefVER<'a>,
        112
    );
    make_option_wrapper!(
        Option_ShapeExtraRefVER,
        lotrc::level::model::shape::ShapeExtraRefVER,
        lotrc::level::model::shape::ShapeExtraRefVER<'a>,
        32
    );
    make_option_wrapper!(
        Option_AtlasUVRefVER,
        lotrc::level::pak::block1::sub_blocks::AtlasUVRefVER,
        lotrc::level::pak::block1::sub_blocks::AtlasUVRefVER<'a>,
        8
    );
    make_option_wrapper!(
        Option_BlocksRefVER,
        lotrc::level::pak::animation::BlocksRefVER,
        lotrc::level::pak::animation::BlocksRefVER<'a>,
        72
    );
    make_option_wrapper!(
        Option_CrowdRefVER,
        lotrc::level::pak::block2::CrowdRefVER,
        lotrc::level::pak::block2::CrowdRefVER<'a>,
        32
    );
    make_option_wrapper!(
        Option_PFieldsRefVER,
        lotrc::level::pak::block2::PFieldsRefVER,
        lotrc::level::pak::block2::PFieldsRefVER<'a>,
        8
    );
    make_option_wrapper!(
        Option_SprayRefVER,
        lotrc::level::pak::block2::SprayRefVER,
        lotrc::level::pak::block2::SprayRefVER<'a>,
        24
    );

    make_vec_wrapper!(
        Vec_DumpInfoDataVER,
        lotrc::level::pak::block1::objs::DumpInfoDataVER,
        lotrc::level::pak::block1::objs::DumpInfoDataVER<'a>
    );
}
