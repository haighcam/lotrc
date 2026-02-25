use safer_ffi::prelude::*;
use std::ptr::{null, slice_from_raw_parts_mut};
use lotrc::macros::make_platforms;
use lotrc::types::{slice, Map, str_ref};

#[derive_ReprC]
#[repr(u8)]
enum AnimVals1Type {
    Type1 = 0,
    Type2,
    Type3,
    Type4,
}

#[derive_ReprC]
#[repr(u8)]
enum RotationQuantizationType {
    Polar32 = 0,
    ThreeComp40,
    ThreeComp48,
    ThreeComp24,
    Straight16,
    Uncompressed,
}

#[make_platforms]
mod impl_ver {
    use super::*;
    use lotrc::{
        level::pak::animation::{AnimVals1RefVER, RotationQuantizationRefVER, RotationPolar32VER, RotationThreeComp40VER, RotationThreeComp48VER, RotationThreeComp24VER, RotationStraight16VER, RotationUncompressedVER, AnimationRefVER},
        types::u16VER,
    };
    #[ffi_export]
    pub fn lotrc_anim_vals1_get_type_ver<'a>(vals: &'a AnimVals1RefVER<'a>) -> AnimVals1Type {
        match vals {
            AnimVals1RefVER::Type1(..) => AnimVals1Type::Type1,
            AnimVals1RefVER::Type2(..) => AnimVals1Type::Type2,
            AnimVals1RefVER::Type3(..) => AnimVals1Type::Type3,
            AnimVals1RefVER::Type4(..) => AnimVals1Type::Type4,
        }
    }
    #[ffi_export]
    pub fn lotrc_anim_vals1_get_type1_ver<'a>(vals: &'a AnimVals1RefVER<'a>) -> slice<'a, u8> {
        match vals {
            AnimVals1RefVER::Type1(vals) => (*vals).into(),
            _ => Default::default()
        }
    }
    #[ffi_export]
    pub fn lotrc_anim_vals1_get_type2_ver<'a>(vals: &'a AnimVals1RefVER<'a>) -> slice<'a, u16VER> {
        match vals {
            AnimVals1RefVER::Type2(vals) => (*vals).into(),
            _ => Default::default()
        }
    }
    #[ffi_export]
    pub fn lotrc_anim_vals1_get_type3_ver<'a>(vals: &'a AnimVals1RefVER<'a>) -> slice<'a, u16VER> {
        match vals {
            AnimVals1RefVER::Type3(vals) => (*vals).into(),
            _ => Default::default()
        }
    }
    #[ffi_export]
    pub fn lotrc_anim_vals1_get_type4_ver<'a>(vals: &'a AnimVals1RefVER<'a>) -> slice<'a, u16VER> {
        match vals {
            AnimVals1RefVER::Type4(vals) => (*vals).into(),
            _ => Default::default()
        }
    }
    #[ffi_export]
    pub fn lotrc_rotation_quantization_get_type_ver<'a>(vals: &'a RotationQuantizationRefVER<'a>) -> RotationQuantizationType {
        match vals {
            RotationQuantizationRefVER::Polar32(..) => RotationQuantizationType::Polar32,
            RotationQuantizationRefVER::ThreeComp40(..) => RotationQuantizationType::ThreeComp40,
            RotationQuantizationRefVER::ThreeComp48(..) => RotationQuantizationType::ThreeComp48,
            RotationQuantizationRefVER::ThreeComp24(..) => RotationQuantizationType::ThreeComp24,
            RotationQuantizationRefVER::Straight16(..) => RotationQuantizationType::Straight16,
            RotationQuantizationRefVER::Uncompressed(..) => RotationQuantizationType::Uncompressed,
        }
    }
    #[ffi_export]
    pub fn lotrc_rotation_quantization_get_polar32_ver<'a>(vals: &'a RotationQuantizationRefVER<'a>) -> slice<'a, RotationPolar32VER> {
        match vals {
            RotationQuantizationRefVER::Polar32(vals) => (*vals).into(),
            _ => Default::default()
        }
    }
    #[ffi_export]
    pub fn lotrc_rotation_quantization_get_threecomp40_ver<'a>(vals: &'a RotationQuantizationRefVER<'a>) -> slice<'a, RotationThreeComp40VER> {
        match vals {
            RotationQuantizationRefVER::ThreeComp40(vals) => (*vals).into(),
            _ => Default::default()
        }
    }
    #[ffi_export]
    pub fn lotrc_rotation_quantization_get_threecomp48_ver<'a>(vals: &'a RotationQuantizationRefVER<'a>) -> slice<'a, RotationThreeComp48VER> {
        match vals {
            RotationQuantizationRefVER::ThreeComp48(vals) => (*vals).into(),
            _ => Default::default()
        }
    }
    #[ffi_export]
    pub fn lotrc_rotation_quantization_get_threecomp24_ver<'a>(vals: &'a RotationQuantizationRefVER<'a>) -> slice<'a, RotationThreeComp24VER> {
        match vals {
            RotationQuantizationRefVER::ThreeComp24(vals) => (*vals).into(),
            _ => Default::default()
        }
    }
    #[ffi_export]
    pub fn lotrc_rotation_quantization_get_straight16_ver<'a>(vals: &'a RotationQuantizationRefVER<'a>) -> slice<'a, RotationStraight16VER> {
        match vals {
            RotationQuantizationRefVER::Straight16(vals) => (*vals).into(),
            _ => Default::default()
        }
    }
    #[ffi_export]
    pub fn lotrc_rotation_quantization_get_uncompressed_ver<'a>(vals: &'a RotationQuantizationRefVER<'a>) -> slice<'a, RotationUncompressedVER> {
        match vals {
            RotationQuantizationRefVER::Uncompressed(vals) => (*vals).into(),
            _ => Default::default()
        }
    }
}
