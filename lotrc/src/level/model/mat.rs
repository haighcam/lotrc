use anyhow::{Context, Result};

use crate::types::{Crc, ReadData, BaseTypes, NE};
use crate::level::{
    pak::block1::infos::{InfoCounts, DumpInfos}
};
use lotrc_proc::{derive_pod};

pub trait MatTypes: BaseTypes {

}

#[derive_pod]
pub struct MatPs3<T: BaseTypes> {
    pub unk_0: T::u32,
    pub tex0: Crc<T>,
    pub tex1: Crc<T>,
    pub tex2: Crc<T>,
    pub tex3: Crc<T>,
    pub tex4: Crc<T>,
    pub tex5: Crc<T>,
    pub key_guid: Crc<T>,
    pub mask0: Crc<T>,
    pub mask1: Crc<T>,
    pub mask2: Crc<T>,
    pub unk_12: T::u32,
    pub unk_13: T::u32,
    pub unk_14: T::u32,
    pub unk_15: T::u32,
    pub unk_16: T::u32,
    pub unk_17: T::u32,
    pub unk_18: T::u32,
    pub unk_19: T::u32,
    pub unk_20: T::u32,
    pub unk_21: T::u32,
    pub unk_22: T::u32,
    pub unk_23: T::u32,
    pub unk_24: T::u32,
    pub unk_25: T::u32,
    pub unk_26: T::u32,
    pub unk_27: T::u32,
    pub unk_28: T::u32,
    pub unk_29: T::u32,
    pub unk_30: T::u32,
    pub unk_31: T::u32,
    pub unk_32: T::u32,
    pub unk_33: T::u32,
    pub z_35: T::u32,
    pub z_36: T::u32,
    pub z_37: T::u32,
    pub z_38: T::u32,
    pub z_39: T::u32,
    pub unk_40: T::u32,
    pub unk_41: T::u32,
    pub unk_42: T::u32,
    pub unk_43: T::u32,
    pub unk_44: T::u32,
    pub unk_45: T::u32,
    pub unk_46: T::u32,
    pub unk_47: T::u32,
    pub unk_48: T::u32,
    pub unk_49: T::u32,
    pub flags: T::u64, //Q', #(flags1, flags2)
    pub kind: T::u32,
    pub unk_53: T::u32,
    pub unk_54a: u8,
    pub unk_54b: u8,
    pub side_flags: T::u16,
    pub unk_55: T::u32,
    pub unk_56: T::u32,
    pub unk_57: T::u32,
    pub unk_58: T::f32,
    pub unk_59: T::f32,
    pub unk_60: T::f32,
    pub unk_61: T::f32,
    pub unk_62: T::f32,
    pub unk_63: T::f32,
    pub unk_64: T::f32,
    pub unk_65: T::f32,
    pub unk_66: T::f32,
    pub unk_67: T::f32,
    pub unk_68: T::f32,
    pub unk_69: T::f32,
    pub unk_70: T::u32,
    pub unk_71: T::u32,
    pub unk_72: T::u32,
    pub unk_73: T::f32,
    pub unk_74: T::f32,
    pub unk_75: T::f32,
    pub unk_76: T::f32,
    pub unk_77: T::u32,
    pub unk_78: T::f32,
    pub unk_79: T::f32,
    pub unk_80: T::f32,
    pub unk_81: T::f32,
    pub unk_82: T::u32,
    pub unk_83: T::u32,
    pub unk_84: T::u32,
    pub unk_85: T::f32,
    pub mat_extra_offset: T::u32,
    pub key: Crc<T>,
    pub unk_88: T::u32,
    pub z_89: T::u32,
}

#[derive_pod]
pub struct Mat1<T: BaseTypes> {
    pub unk_0: T::u32,
    pub unk_1: T::u32,
    pub tex0: Crc<T>,
    pub tex1: Crc<T>,
    pub tex2: Crc<T>,
    pub tex3: Crc<T>,
    pub tex4: Crc<T>,
    pub tex5: Crc<T>,
    pub key_guid: Crc<T>,
    pub mask0: Crc<T>,
    pub mask1: Crc<T>,
    pub mask2: Crc<T>,
    pub unk_12: T::u32,
    pub unk_13: T::u32,
    pub unk_14: T::u32,
    pub unk_15: T::u32,
    pub unk_16: T::u32,
    pub unk_17: T::u32,
    pub unk_18: T::u32,
    pub unk_19: T::u32,
    pub unk_20: T::u32,
    pub unk_21: T::u32,
    pub unk_22: T::u32,
    pub unk_23: T::u32,
    pub unk_24: T::u32,
    pub unk_25: T::u32,
    pub unk_26: T::u32,
    pub unk_27: T::u32,
    pub unk_28: T::u32,
    pub unk_29: T::u32,
    pub unk_30: T::u32,
    pub unk_31: T::u32,
    pub unk_32: T::u32,
    pub unk_33: T::u32,
    pub z_34: T::u32,
    pub z_35: T::u32,
    pub z_36: T::u32,
    pub z_37: T::u32,
    pub z_38: T::u32,
    pub z_39: T::u32,
    pub unk_40: T::u32,
    pub unk_41: T::u32,
    pub unk_42: T::u32,
    pub unk_43: T::u32,
    pub unk_44: T::u32,
    pub unk_45: T::u32,
    pub unk_46: T::u32,
    pub unk_47: T::u32,
    pub unk_48: T::u32,
    pub unk_49: T::u32,
    pub flags: T::u64, //Q', #(flags1, flags2)
    pub kind: T::u32,
    pub unk_53: T::u32,
    pub unk_54a: u8,
    pub unk_54b: u8,
    pub side_flags: T::u16,
    pub unk_55: T::u32,
    pub unk_56: T::u32,
    pub unk_57: T::u32,
    pub unk_58: T::f32,
    pub unk_59: T::f32,
    pub unk_60: T::f32,
    pub unk_61: T::f32,
    pub unk_62: T::f32,
    pub unk_63: T::f32,
    pub unk_64: T::f32,
    pub unk_65: T::f32,
    pub unk_66: T::f32,
    pub unk_67: T::f32,
    pub unk_68: T::f32,
    pub unk_69: T::f32,
    pub unk_70: T::u32,
    pub unk_71: T::u32,
    pub unk_72: T::u32,
    pub unk_73: T::f32,
    pub unk_74: T::f32,
    pub unk_75: T::f32,
    pub unk_76: T::f32,
    pub unk_77: T::u32,
    pub unk_78: T::f32,
    pub unk_79: T::f32,
    pub unk_80: T::f32,
    pub unk_81: T::f32,
    pub unk_82: T::u32,
    pub unk_83: T::u32,
    pub unk_84: T::u32,
    pub unk_85: T::f32,
    pub mat_extra_offset: T::u32,
    pub key: Crc<T>,
    pub unk_88: T::u32,
    pub z_89: T::u32,
}

#[derive_pod]
pub struct Mat2Extra<T: BaseTypes> {
    pub unk_90: T::u32,
    pub unk_91: T::u32,
    pub unk_92: T::u32,
    pub unk_93: T::u32,
    pub unk_94: T::u32,
    pub unk_95: T::u32,
    pub unk_96: T::u32,
    pub unk_97: T::u32,
    pub unk_98: T::u32,
    pub unk_99: T::u32,
    pub unk_100: T::u32,
    pub unk_101: T::u32,
    pub unk_102: T::f32,
    pub unk_103: T::f32,
    pub unk_104: T::f32,
    pub unk_105: T::f32,
    pub unk_106: T::f32,
    pub unk_107: T::f32,
    pub unk_108: T::f32,
    pub unk_109: T::f32,
    pub unk_110: T::f32,
    pub unk_111: T::f32,
    pub unk_112: T::f32,
    pub unk_113: T::f32,
    pub unk_114: T::u32,
    pub unk_115: T::u32,
    pub unk_116: T::u32,
    pub unk_117: Crc<T>,
    pub unk_118: Crc<T>,
    pub unk_119: Crc<T>,
    pub unk_120a: u8,
    pub unk_120b: u8,
    pub unk_120c: u8,
    pub unk_120d: u8,
    pub unk_121: T::u32,
}

#[derive_pod]
pub struct Mat2<T: BaseTypes> {
    base: Mat1<T>,
    extra: Mat2Extra<T>,
}

#[derive_pod]
pub struct Mat3ExtraPS3<T: BaseTypes> {
    pub unk_90: T::f32,
    pub unk_91: T::f32,
    pub unk_92: T::f32,
    pub unk_93: T::f32,
    pub unk_94: T::f32,
    pub unk_95: T::f32,
    pub unk_96: T::f32,
    pub unk_97: T::f32,
    pub unk_98: T::f32,
    pub unk_99: T::f32,
    pub unk_100: T::f32,
    pub unk_101: T::f32,
    pub unk_102: T::f32,
    pub unk_103: T::f32,
    pub unk_104: T::f32,
    pub unk_105: T::f32,
    pub unk_106: T::f32,
    pub unk_107: T::f32,
    pub unk_108: T::f32,
    pub unk_109: T::f32,
    pub unk_110: T::f32,
    pub unk_111: T::f32,
    pub unk_112: T::f32,
    pub unk_113: T::f32,
    pub variation_id_color: u8,
    pub variation_id_texture: u8,
    pub variation_id_specular: u8,
    pub unk_114d: u8,
    pub unk_115: T::u32,
    pub unk_116: T::u32,
    pub unk_117: T::u32,
}

#[derive_pod]
pub struct Mat3Extra<T: BaseTypes> {
    pub unk_90: T::f32,
    pub unk_91: T::f32,
    pub unk_92: T::f32,
    pub unk_93: T::f32,
    pub unk_94: T::f32,
    pub unk_95: T::f32,
    pub unk_96: T::f32,
    pub unk_97: T::f32,
    pub unk_98: T::f32,
    pub unk_99: T::f32,
    pub unk_100: T::f32,
    pub unk_101: T::f32,
    pub unk_102: T::f32,
    pub unk_103: T::f32,
    pub unk_104: T::f32,
    pub unk_105: T::f32,
    pub unk_106: T::f32,
    pub unk_107: T::f32,
    pub unk_108: T::f32,
    pub unk_109: T::f32,
    pub unk_110: T::f32,
    pub unk_111: T::f32,
    pub unk_112: T::f32,
    pub unk_113: T::f32,
    pub variation_id_color: u8,
    pub variation_id_texture: u8,
    pub variation_id_specular: u8,
    pub unk_114d: u8,
    pub unk_115: T::u32,
}

#[derive_pod]
pub struct Mat3<T: BaseTypes> {
    base: Mat1<T>,
    extra: Mat3Extra<T>,
}

#[derive_pod]
pub struct Mat4Extra<T: BaseTypes> {
    pub unk_90: T::f32,
    pub unk_91: T::u32,
    pub unk_92: T::u32,
    pub unk_93: T::u32,
    pub unk_94: T::u32,
    pub unk_95: T::u32,
    pub unk_96: T::f32,
    pub unk_97: T::u32,
    pub unk_98: T::f32,
    pub unk_99: T::u32,
    pub unk_100: T::u32,
    pub unk_101: T::u32,
    pub unk_102: T::u32,
    pub unk_103: T::u32,
    pub unk_104: T::f32,
    pub unk_105: T::u32,
    pub unk_106: T::f32,
    pub unk_107: T::u32,
    pub unk_108: T::u32,
    pub unk_109: T::f32,
    pub unk_110: T::u32,
    pub unk_111: T::u32,
    pub unk_112: T::f32,
    pub unk_113: T::f32,
    pub unk_114: T::f32,
    pub unk_115: T::u32,
    pub unk_116: T::u32,
    pub unk_117: T::f32,
    pub unk_118: T::u32,
    pub unk_119: T::u32,
    pub unk_120: T::f32,
    pub unk_121: T::f32,
    pub unk_122: T::f32,
    pub unk_123: T::u32,
    pub unk_124: T::u32,
    pub unk_125: T::f32,
    pub unk_126: T::u32,
    pub unk_127: T::u32,
    pub unk_128: T::f32,
    pub unk_129: T::f32,
    pub unk_130: T::f32,
    pub unk_131: T::u32,
    pub unk_132: T::u32,
    pub unk_133: T::f32,
    pub unk_134: T::u32,
    pub unk_135: T::u32,
    pub unk_136: T::f32,
    pub unk_137: T::f32,
    pub unk_138: T::f32,
    pub unk_139: T::u32,
    pub unk_140: T::u32,
    pub unk_141: T::f32,
    pub unk_142: T::u32,
    pub unk_143: T::u32,
    pub unk_144: T::f32,
    pub unk_145: T::f32,
}

#[derive_pod]
pub struct Mat4<T: BaseTypes> {
    base: Mat1<T>,
    extra: Mat4Extra<T>,
}

#[derive_pod]
pub struct MatExtra<T: BaseTypes> {
    pub unk_0: T::u32,
    pub unk_1: T::u32,
    pub unk_2: T::u32,
    pub unk_3: T::u32,
    pub unk_4: T::u32,
    pub unk_5: T::u32,
    pub unk_6: T::u32,
    pub unk_7: T::u32,
    pub unk_8: T::u32,
    pub unk_9: T::u32,
    pub unk_10: T::u32,
    pub unk_11: T::f32,
    pub unk_12: T::f32,
    pub unk_13: T::f32,
    pub unk_14: T::f32,
    pub unk_15: T::f32,
    pub unk_16: T::u32,
    pub unk_17: T::f32,
    pub unk_18: T::f32,
    pub unk_19: T::f32,
    pub unk_20: T::f32,
    pub unk_21: T::f32,
    pub unk_22: T::u32,
    pub unk_23: T::u32,
    pub unk_24: T::u32,
    pub unk_25: T::u32,
    pub unk_26: T::u32,
    pub unk_27: T::u32,
    pub unk_28: T::u32,
    pub unk_29: T::u32,
    pub unk_30: T::u32,
    pub unk_31: T::u32,
    pub unk_32: T::u32,
    pub unk_33: T::u32,
    pub unk_34: T::u32,
    pub unk_35: T::u32,
    pub unk_36: T::u32,
    pub unk_37: T::u32,
    pub unk_38: T::u32,
    pub unk_39: T::u32,
    pub unk_40: T::u32,
    pub unk_41: T::u32,
    pub unk_42: T::u32,
    pub unk_43: T::u32,
    pub unk_44: T::u32,
    pub unk_45: T::u32,
    pub unk_46: T::u32,
    pub unk_47: T::u32,
    pub unk_48: T::u32,
    pub unk_49: T::u32,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat1Ref<'a, T: BaseTypes> {
    pub info: &'a Mat1<T>,
    pub extra: Option<&'a MatExtra<T>>
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat2Ref<'a, T: BaseTypes> {
    pub info: &'a Mat2<T>,
    pub extra: Option<&'a MatExtra<T>>
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat3Ref<'a, T: BaseTypes> {
    pub info: &'a Mat3<T>,
    pub extra: Option<&'a MatExtra<T>>
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat4Ref<'a, T: BaseTypes> {
    pub info: &'a Mat4<T>,
    pub extra: Option<&'a MatExtra<T>>
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum MatRef<'a, T: BaseTypes> {
    Mat1(Mat1Ref<'a, T>),
    Mat2(Mat2Ref<'a, T>),
    Mat3(Mat3Ref<'a, T>),
    Mat4(Mat4Ref<'a, T>),
}

impl<'a, T: BaseTypes> MatRef<'a, T> {
    pub fn from_data(src: &'a [u8], offset: usize) -> Result<Self> {
        let base = Mat1::<T>::from_data(&src[offset..]).context("base")?;
        let extra = if base.mat_extra_offset.into() != 0 {
            Some(MatExtra::from_data(&src[base.mat_extra_offset.into() as usize..]).context("extra")?)
        } else {
            None
        };
        Ok(match base.kind.into() {
            0 => Self::Mat1(Mat1Ref {
                info: Mat1::from_data(&src[offset..]).context("mat1")?,
                extra,
            }),
            1 => Self::Mat4(Mat4Ref {
                info: Mat4::from_data(&src[offset..]).context("mat4")?,
                extra,
            }),
            2 => Self::Mat2(Mat2Ref {
                info: Mat2::from_data(&src[offset..]).context("mat2")?,
                extra,
            }),
            3 => Self::Mat3(Mat3Ref {
                info: Mat3::from_data(&src[offset..]).context("mat3")?,
                extra,
            }),
            _ => return Err(anyhow::anyhow!("Unknown Mat Type {}", base.kind.into())),
        })
    }
}

pub trait DumpMat<T: BaseTypes> {
    fn dump_infos(&self, infos: &mut DumpInfos<T>) -> Result<u32>;
    fn add_counts(&self, counts: &mut InfoCounts);
}

impl<T: BaseTypes> DumpMat<T> for MatRef<'_, T> {
    fn dump_infos(&self, infos: &mut DumpInfos<T>) -> Result<u32> {
        match self {
            Self::Mat1(Mat1Ref { info, extra}) => {
                let val = infos.mat1s.offset;
                let mat = infos.mat1s.next().context("mat1s")?;
                *mat = **info;
                mat.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().into();
                if let Some(extra) = extra {
                    *infos.mat_extras.next().context("mat_extras")? = **extra;
                    *infos.offsets.next().context("offsets")? = ((val + std::mem::offset_of!(Mat1<T>, mat_extra_offset)) as u32).into();
                }
                Ok(val as u32)
            }
            Self::Mat2(Mat2Ref { info, extra }) => {
                let val = infos.mat2s.offset;
                let mat = infos.mat2s.next().context("mat2s")?;
                *mat = **info;
                mat.base.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().into();
                if let Some(extra) = extra {
                    *infos.mat_extras.next().context("mat_extras")? = **extra;
                    *infos.offsets.next().context("offsets")? = ((val + std::mem::offset_of!(Mat2<T>, base) + std::mem::offset_of!(Mat1<T>, mat_extra_offset)) as u32).into();
                }
                Ok(val as u32)
            }
            Self::Mat3(Mat3Ref { info, extra }) => {
                let val = infos.mat3s.offset;
                let mat = infos.mat3s.next().context("mat3s")?;
                *mat = **info;
                mat.base.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().into();
                if let Some(extra) = extra {
                    *infos.mat_extras.next().context("mat_extras")? = **extra;
                    *infos.offsets.next().context("offsets")? = ((val + std::mem::offset_of!(Mat3<T>, base) + std::mem::offset_of!(Mat1<T>, mat_extra_offset)) as u32).into();
                }
                Ok(val as u32)
            }
            Self::Mat4(Mat4Ref { info, extra }) => {
                let val = infos.mat4s.offset;
                let mat = infos.mat4s.next().context("mat4s")?;
                *mat = **info;
                mat.base.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().into();
                if let Some(extra) = extra {
                    *infos.mat_extras.next().context("mat_extras")? = **extra;
                    *infos.offsets.next().context("offsets")? = ((val + std::mem::offset_of!(Mat4<T>, base) + std::mem::offset_of!(Mat1<T>, mat_extra_offset)) as u32).into();
                }
                Ok(val as u32)
            }
        }
    }
    fn add_counts(&self, counts: &mut InfoCounts) {
        match self {
            Self::Mat1(val) => {
                counts.mat1s += 1;
                if val.extra.is_some() {
                    counts.mat_extras += 1;
                    counts.offsets += 1;
                }
            }
            Self::Mat2(val) => {
                counts.mat2s += 1;
                if val.extra.is_some() {
                    counts.mat_extras += 1;
                    counts.offsets += 1;
                }
            }
            Self::Mat3(val) => {
                counts.mat3s += 1;
                if val.extra.is_some() {
                    counts.mat_extras += 1;
                    counts.offsets += 1;
                }
            }
            Self::Mat4(val) => {
                counts.mat4s += 1;
                if val.extra.is_some() {
                    counts.mat_extras += 1;
                    counts.offsets += 1;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum MatInd {
    Mat1(usize),
    Mat2(usize),
    Mat3(usize),
    Mat4(usize),
}

#[derive(Debug, Clone)]
pub enum Mat {
    Normal(Mat1<NE>, Option<MatExtra<NE>>),
    Variation(Mat2<NE>, Option<MatExtra<NE>>),
    CharacterVariation(Mat3<NE>, Option<MatExtra<NE>>),
    Terrain(Mat4<NE>, Option<MatExtra<NE>>),
}

impl<T: BaseTypes> From<&MatRef<'_, T>> for Mat
where
    Mat1<NE>: From<Mat1<T>>,
    Mat2<NE>: From<Mat2<T>>,
    Mat3<NE>: From<Mat3<T>>,
    Mat4<NE>: From<Mat4<T>>,
    MatExtra<NE>: From<MatExtra<T>>,
{
    fn from(val: &MatRef<T>) -> Self {
        match val {
            MatRef::Mat1(Mat1Ref { info, extra }) => Self::Normal((**info).into(), extra.map(|&x| x.into())),
            MatRef::Mat2(Mat2Ref { info, extra }) => Self::Variation((**info).into(), extra.map(|&x| x.into())),
            MatRef::Mat3(Mat3Ref { info, extra }) => {
                Self::CharacterVariation((**info).into(), extra.map(|&x| x.into()))
            }
            MatRef::Mat4(Mat4Ref { info, extra }) => Self::Terrain((**info).into(), extra.map(|&x| x.into())),
        }
    }
}
