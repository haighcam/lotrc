use anyhow::{Context, Result};

#[make_platforms]
use crate::{
    level::pak::block1::infos::DumpInfosVER,
    types::{u32VER, CrcVER, u8VER, u16VER, u64VER, f32VER}
};

use crate::types::GetNative;
use crate::types::{Crc, RefFromData, OrderedData, OrderedDataStrict, DumpData};
use crate::level::pak::block1::infos::InfoCounts;
use lotrc_proc::{make_platforms, OrderedData};

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct MatBase {
    pub unk_0: u32,
    #[ordered_data(skip_ps3)]
    pub unk_1: u32,
    pub tex0: Crc,
    pub tex1: Crc,
    pub tex2: Crc,
    pub tex3: Crc,
    pub tex4: Crc,
    pub tex5: Crc,
    pub key_guid: Crc,
    pub mask0: Crc,
    pub mask1: Crc,
    pub mask2: Crc,
    pub unk_12: u32,
    pub unk_13: u32,
    pub unk_14: u32,
    pub unk_15: u32,
    pub unk_16: u32,
    pub unk_17: u32,
    pub unk_18: u32,
    pub unk_19: u32,
    pub unk_20: u32,
    pub unk_21: u32,
    pub unk_22: u32,
    pub unk_23: u32,
    pub unk_24: u32,
    pub unk_25: u32,
    pub unk_26: u32,
    pub unk_27: u32,
    pub unk_28: u32,
    pub unk_29: u32,
    pub unk_30: u32,
    pub unk_31: u32,
    pub unk_32: u32,
    pub unk_33: u32,
    #[ordered_data(skip_ps3)]
    pub z_34: u32,
    pub z_35: u32,
    pub z_36: u32,
    pub z_37: u32,
    pub z_38: u32,
    pub z_39: u32,
    pub unk_40: u32,
    pub unk_41: u32,
    pub unk_42: u32,
    pub unk_43: u32,
    pub unk_44: u32,
    pub unk_45: u32,
    pub unk_46: u32,
    pub unk_47: u32,
    pub unk_48: u32,
    pub unk_49: u32,
    pub flags: u64, //Q', #(flags1, flags2)
    pub kind: u32,
    pub unk_53: u32,
    pub unk_54a: u8,
    pub unk_54b: u8,
    pub side_flags: u16,
    pub unk_55: u32,
    pub unk_56: u32,
    pub unk_57: u32,
    pub unk_58: f32,
    pub unk_59: f32,
    pub unk_60: f32,
    pub unk_61: f32,
    pub unk_62: f32,
    pub unk_63: f32,
    pub unk_64: f32,
    pub unk_65: f32,
    pub unk_66: f32,
    pub unk_67: f32,
    pub unk_68: f32,
    pub unk_69: f32,
    pub unk_70: u32,
    pub unk_71: u32,
    pub unk_72: u32,
    pub unk_73: f32,
    pub unk_74: f32,
    pub unk_75: f32,
    pub unk_76: f32,
    pub unk_77: u32,
    pub unk_78: f32,
    pub unk_79: f32,
    pub unk_80: f32,
    pub unk_81: f32,
    pub unk_82: u32,
    pub unk_83: u32,
    pub unk_84: u32,
    pub unk_85: f32,
    pub mat_extra_offset: u32,
    pub key: Crc,
    pub unk_88: u32,
    pub z_89: u32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct Mat1 {
    pub base: MatBase,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct Mat2 {
    pub base: MatBase,
    pub unk_90: u32,
    pub unk_91: u32,
    pub unk_92: u32,
    pub unk_93: u32,
    pub unk_94: u32,
    pub unk_95: u32,
    pub unk_96: u32,
    pub unk_97: u32,
    pub unk_98: u32,
    pub unk_99: u32,
    pub unk_100: u32,
    pub unk_101: u32,
    pub unk_102: f32,
    pub unk_103: f32,
    pub unk_104: f32,
    pub unk_105: f32,
    pub unk_106: f32,
    pub unk_107: f32,
    pub unk_108: f32,
    pub unk_109: f32,
    pub unk_110: f32,
    pub unk_111: f32,
    pub unk_112: f32,
    pub unk_113: f32,
    pub unk_114: u32,
    pub unk_115: u32,
    pub unk_116: u32,
    pub unk_117: Crc,
    pub unk_118: Crc,
    pub unk_119: Crc,
    pub unk_120a: u8,
    pub unk_120b: u8,
    pub unk_120c: u8,
    pub unk_120d: u8,
    pub unk_121: u32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct Mat3 {
    pub base: MatBase,
    pub unk_90: f32,
    pub unk_91: f32,
    pub unk_92: f32,
    pub unk_93: f32,
    pub unk_94: f32,
    pub unk_95: f32,
    pub unk_96: f32,
    pub unk_97: f32,
    pub unk_98: f32,
    pub unk_99: f32,
    pub unk_100: f32,
    pub unk_101: f32,
    pub unk_102: f32,
    pub unk_103: f32,
    pub unk_104: f32,
    pub unk_105: f32,
    pub unk_106: f32,
    pub unk_107: f32,
    pub unk_108: f32,
    pub unk_109: f32,
    pub unk_110: f32,
    pub unk_111: f32,
    pub unk_112: f32,
    pub unk_113: f32,
    pub variation_id_color: u8,
    pub variation_id_texture: u8,
    pub variation_id_specular: u8,
    pub unk_114d: u8,
    pub unk_115: u32,
    #[ordered_data(skip_pc, skip_xbox)]
    pub unk_116: u32,
    #[ordered_data(skip_pc, skip_xbox)]
    pub unk_117: u32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct Mat4 {
    pub base: MatBase,
    pub unk_90: f32,
    pub unk_91: u32,
    pub unk_92: u32,
    pub unk_93: u32,
    pub unk_94: u32,
    pub unk_95: u32,
    pub unk_96: f32,
    pub unk_97: u32,
    pub unk_98: f32,
    pub unk_99: u32,
    pub unk_100: u32,
    pub unk_101: u32,
    pub unk_102: u32,
    pub unk_103: u32,
    pub unk_104: f32,
    pub unk_105: u32,
    pub unk_106: f32,
    pub unk_107: u32,
    pub unk_108: u32,
    pub unk_109: f32,
    pub unk_110: u32,
    pub unk_111: u32,
    pub unk_112: f32,
    pub unk_113: f32,
    pub unk_114: f32,
    pub unk_115: u32,
    pub unk_116: u32,
    pub unk_117: f32,
    pub unk_118: u32,
    pub unk_119: u32,
    pub unk_120: f32,
    pub unk_121: f32,
    pub unk_122: f32,
    pub unk_123: u32,
    pub unk_124: u32,
    pub unk_125: f32,
    pub unk_126: u32,
    pub unk_127: u32,
    pub unk_128: f32,
    pub unk_129: f32,
    pub unk_130: f32,
    pub unk_131: u32,
    pub unk_132: u32,
    pub unk_133: f32,
    pub unk_134: u32,
    pub unk_135: u32,
    pub unk_136: f32,
    pub unk_137: f32,
    pub unk_138: f32,
    pub unk_139: u32,
    pub unk_140: u32,
    pub unk_141: f32,
    pub unk_142: u32,
    pub unk_143: u32,
    pub unk_144: f32,
    pub unk_145: f32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct MatExtra {
    pub unk_0: u32,
    pub unk_1: u32,
    pub unk_2: u32,
    pub unk_3: u32,
    pub unk_4: u32,
    pub unk_5: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: f32,
    pub unk_12: f32,
    pub unk_13: f32,
    pub unk_14: f32,
    pub unk_15: f32,
    pub unk_16: u32,
    pub unk_17: f32,
    pub unk_18: f32,
    pub unk_19: f32,
    pub unk_20: f32,
    pub unk_21: f32,
    pub unk_22: u32,
    pub unk_23: u32,
    pub unk_24: u32,
    pub unk_25: u32,
    pub unk_26: u32,
    pub unk_27: u32,
    pub unk_28: u32,
    pub unk_29: u32,
    pub unk_30: u32,
    pub unk_31: u32,
    pub unk_32: u32,
    pub unk_33: u32,
    pub unk_34: u32,
    pub unk_35: u32,
    pub unk_36: u32,
    pub unk_37: u32,
    pub unk_38: u32,
    pub unk_39: u32,
    pub unk_40: u32,
    pub unk_41: u32,
    pub unk_42: u32,
    pub unk_43: u32,
    pub unk_44: u32,
    pub unk_45: u32,
    pub unk_46: u32,
    pub unk_47: u32,
    pub unk_48: u32,
    pub unk_49: u32,
}

#[make_platforms]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat1RefVER<'a> {
    pub info: &'a Mat1VER,
    pub extra: Option<&'a MatExtraVER>
}

#[make_platforms]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat2RefVER<'a> {
    pub info: &'a Mat2VER,
    pub extra: Option<&'a MatExtraVER>
}

#[make_platforms]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat3RefVER<'a> {
    pub info: &'a Mat3VER,
    pub extra: Option<&'a MatExtraVER>
}

#[make_platforms]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat4RefVER<'a> {
    pub info: &'a Mat4VER,
    pub extra: Option<&'a MatExtraVER>
}

#[make_platforms]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum MatRefVER<'a> {
    Mat1(Mat1RefVER<'a>),
    Mat2(Mat2RefVER<'a>),
    Mat3(Mat3RefVER<'a>),
    Mat4(Mat4RefVER<'a>),
}

#[make_platforms]
impl<'a> MatRefVER<'a> {
    pub fn from_data(src: &'a [u8], offset: usize) -> Result<Self> {
        let base = MatBaseVER::from_data(&src[offset..]).context("base")?;
        let extra = if base.mat_extra_offset.get() != 0 {
            Some(MatExtraVER::from_data(&src[base.mat_extra_offset.get() as usize..]).context("extra")?)
        } else {
            None
        };
        Ok(match base.kind.get() {
            0 => Self::Mat1(Mat1RefVER {
                info: Mat1VER::from_data(&src[offset..]).context("mat1")?,
                extra,
            }),
            1 => Self::Mat4(Mat4RefVER {
                info: Mat4VER::from_data(&src[offset..]).context("mat4")?,
                extra,
            }),
            2 => Self::Mat2(Mat2RefVER {
                info: Mat2VER::from_data(&src[offset..]).context("mat2")?,
                extra,
            }),
            3 => Self::Mat3(Mat3RefVER {
                info: Mat3VER::from_data(&src[offset..]).context("mat3")?,
                extra,
            }),
            _ => return Err(anyhow::anyhow!("Unknown Mat Type {}", base.kind.get())),
        })
    }
}


#[make_platforms]
pub trait DumpMatVER {
    fn dump_infos(&self, infos: &mut DumpInfosVER) -> Result<u32>;
    fn add_counts(&self, counts: &mut InfoCounts);
}

#[make_platforms]
impl DumpMatVER for MatRefVER<'_> {
    fn dump_infos(&self, infos: &mut DumpInfosVER) -> Result<u32> {
        match self {
            Self::Mat1(Mat1RefVER { info, extra}) => {
                let val = infos.mat1s.offset;
                let mat = infos.mat1s.next().context("mat1s")?;
                mat.write_from(info)?;
                mat.base.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().conv();
                if let Some(extra) = extra {
                    infos.mat_extras.next().context("mat_extras")?.write_from(extra)?;
                    *infos.offsets.next().context("offsets")? = (val + std::mem::offset_of!(Mat1VER, base) + std::mem::offset_of!(MatBaseVER, mat_extra_offset)).conv();
                }
                Ok(val as u32)
            }
            Self::Mat2(Mat2RefVER { info, extra }) => {
                let val = infos.mat2s.offset;
                let mat = infos.mat2s.next().context("mat2s")?;
                mat.write_from(info)?;
                mat.base.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().conv();
                if let Some(extra) = extra {
                    infos.mat_extras.next().context("mat_extras")?.write_from(extra)?;
                    *infos.offsets.next().context("offsets")? = (val + std::mem::offset_of!(Mat2VER, base) + std::mem::offset_of!(MatBaseVER, mat_extra_offset)).conv();
                }
                Ok(val as u32)
            }
            Self::Mat3(Mat3RefVER { info, extra }) => {
                let val = infos.mat3s.offset;
                let mat = infos.mat3s.next().context("mat3s")?;
                mat.write_from(info)?;
                mat.base.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().conv();
                if let Some(extra) = extra {
                    infos.mat_extras.next().context("mat_extras")?.write_from(extra)?;
                    *infos.offsets.next().context("offsets")? = (val + std::mem::offset_of!(Mat3VER, base) + std::mem::offset_of!(MatBaseVER, mat_extra_offset)).conv();
                }
                Ok(val as u32)
            }
            Self::Mat4(Mat4RefVER { info, extra }) => {
                let val = infos.mat4s.offset;
                let mat = infos.mat4s.next().context("mat4s")?;
                mat.write_from(info)?;
                mat.base.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().conv();
                if let Some(extra) = extra {
                    infos.mat_extras.next().context("mat_extras")?.write_from(extra)?;
                    *infos.offsets.next().context("offsets")? = (val + std::mem::offset_of!(Mat4VER, base) + std::mem::offset_of!(MatBaseVER, mat_extra_offset)).conv();
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
    Normal(Mat1, Option<MatExtra>),
    Variation(Mat2, Option<MatExtra>),
    CharacterVariation(Mat3, Option<MatExtra>),
    Terrain(Mat4, Option<MatExtra>),
}

#[make_platforms]
impl From<&MatRefVER<'_>> for Mat {
    fn from(val: &MatRefVER) -> Self {
        match val {
            MatRefVER::Mat1(Mat1RefVER { info, extra }) => Self::Normal(info.conv(), extra.map(|x| x.conv())),
            MatRefVER::Mat2(Mat2RefVER { info, extra }) => Self::Variation(info.conv(), extra.map(|x| x.conv())),
            MatRefVER::Mat3(Mat3RefVER { info, extra }) => {
                Self::CharacterVariation(info.conv(), extra.map(|x| x.conv()))
            }
            MatRefVER::Mat4(Mat4RefVER { info, extra }) => Self::Terrain(info.conv(), extra.map(|x| x.conv())),
        }
    }
}

/*
impl Mat {
    #[make_platforms]
    pub fn get_raw_ver(&self) -> (MatVER, Option<MatExtraVER>) {
        match self {
            Self::Normal(mat, extra) => (
                MatVER::Mat1(Mat1VER::from(mat.clone())),
                extra.as_ref().map(|x| MatExtraVER::from(x.clone())),
            ),
            Self::Variation(mat, extra) => (
                MatVER::Mat2(Mat2VER::from(mat.clone())),
                extra.as_ref().map(|x| MatExtraVER::from(x.clone())),
            ),
            Self::CharacterVariation(mat, extra) => (
                MatVER::Mat3(Mat3VER::from(mat.clone())),
                extra.as_ref().map(|x| MatExtraVER::from(x.clone())),
            ),
            Self::Terrain(mat, extra) => (
                MatVER::Mat4(Mat4VER::from(mat.clone())),
                extra.as_ref().map(|x| MatExtraVER::from(x.clone())),
            ),
        }
    }
}
*/
