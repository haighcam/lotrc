use anyhow::{Context, Result};

#[make_endian]
use crate::{
    level::pak::block1::infos::DumpInfos_XE_,
    types::{u32_XE_, Crc_XE_, u16_XE_, u64_XE_, f32_XE_}
};

use crate::types::{Crc, RefFromData, OrderedData, DumpData};
use crate::level::pak::block1::infos::InfoCounts;
use lotrc_proc::{make_endian, derive_ordered_data};

#[make_endian]
#[derive(Debug, Default, Clone)]
pub struct MatPs3_XE_ {
    pub unk_0: u32_XE_,
    pub tex0: Crc_XE_,
    pub tex1: Crc_XE_,
    pub tex2: Crc_XE_,
    pub tex3: Crc_XE_,
    pub tex4: Crc_XE_,
    pub tex5: Crc_XE_,
    pub key_guid: Crc_XE_,
    pub mask0: Crc_XE_,
    pub mask1: Crc_XE_,
    pub mask2: Crc_XE_,
    pub unk_12: u32_XE_,
    pub unk_13: u32_XE_,
    pub unk_14: u32_XE_,
    pub unk_15: u32_XE_,
    pub unk_16: u32_XE_,
    pub unk_17: u32_XE_,
    pub unk_18: u32_XE_,
    pub unk_19: u32_XE_,
    pub unk_20: u32_XE_,
    pub unk_21: u32_XE_,
    pub unk_22: u32_XE_,
    pub unk_23: u32_XE_,
    pub unk_24: u32_XE_,
    pub unk_25: u32_XE_,
    pub unk_26: u32_XE_,
    pub unk_27: u32_XE_,
    pub unk_28: u32_XE_,
    pub unk_29: u32_XE_,
    pub unk_30: u32_XE_,
    pub unk_31: u32_XE_,
    pub unk_32: u32_XE_,
    pub unk_33: u32_XE_,
    pub z_35: u32_XE_,
    pub z_36: u32_XE_,
    pub z_37: u32_XE_,
    pub z_38: u32_XE_,
    pub z_39: u32_XE_,
    pub unk_40: u32_XE_,
    pub unk_41: u32_XE_,
    pub unk_42: u32_XE_,
    pub unk_43: u32_XE_,
    pub unk_44: u32_XE_,
    pub unk_45: u32_XE_,
    pub unk_46: u32_XE_,
    pub unk_47: u32_XE_,
    pub unk_48: u32_XE_,
    pub unk_49: u32_XE_,
    pub flags: u64_XE_, //Q', #(flags1, flags2)
    pub kind: u32_XE_,
    pub unk_53: u32_XE_,
    pub unk_54a: u8,
    pub unk_54b: u8,
    pub side_flags: u16_XE_,
    pub unk_55: u32_XE_,
    pub unk_56: u32_XE_,
    pub unk_57: u32_XE_,
    pub unk_58: f32_XE_,
    pub unk_59: f32_XE_,
    pub unk_60: f32_XE_,
    pub unk_61: f32_XE_,
    pub unk_62: f32_XE_,
    pub unk_63: f32_XE_,
    pub unk_64: f32_XE_,
    pub unk_65: f32_XE_,
    pub unk_66: f32_XE_,
    pub unk_67: f32_XE_,
    pub unk_68: f32_XE_,
    pub unk_69: f32_XE_,
    pub unk_70: u32_XE_,
    pub unk_71: u32_XE_,
    pub unk_72: u32_XE_,
    pub unk_73: f32_XE_,
    pub unk_74: f32_XE_,
    pub unk_75: f32_XE_,
    pub unk_76: f32_XE_,
    pub unk_77: u32_XE_,
    pub unk_78: f32_XE_,
    pub unk_79: f32_XE_,
    pub unk_80: f32_XE_,
    pub unk_81: f32_XE_,
    pub unk_82: u32_XE_,
    pub unk_83: u32_XE_,
    pub unk_84: u32_XE_,
    pub unk_85: f32_XE_,
    pub mat_extra_offset: u32_XE_,
    pub key: Crc_XE_,
    pub unk_88: u32_XE_,
    pub z_89: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat1_XE_ {
    pub unk_0: u32_XE_,
    pub unk_1: u32_XE_,
    pub tex0: Crc_XE_,
    pub tex1: Crc_XE_,
    pub tex2: Crc_XE_,
    pub tex3: Crc_XE_,
    pub tex4: Crc_XE_,
    pub tex5: Crc_XE_,
    pub key_guid: Crc_XE_,
    pub mask0: Crc_XE_,
    pub mask1: Crc_XE_,
    pub mask2: Crc_XE_,
    pub unk_12: u32_XE_,
    pub unk_13: u32_XE_,
    pub unk_14: u32_XE_,
    pub unk_15: u32_XE_,
    pub unk_16: u32_XE_,
    pub unk_17: u32_XE_,
    pub unk_18: u32_XE_,
    pub unk_19: u32_XE_,
    pub unk_20: u32_XE_,
    pub unk_21: u32_XE_,
    pub unk_22: u32_XE_,
    pub unk_23: u32_XE_,
    pub unk_24: u32_XE_,
    pub unk_25: u32_XE_,
    pub unk_26: u32_XE_,
    pub unk_27: u32_XE_,
    pub unk_28: u32_XE_,
    pub unk_29: u32_XE_,
    pub unk_30: u32_XE_,
    pub unk_31: u32_XE_,
    pub unk_32: u32_XE_,
    pub unk_33: u32_XE_,
    pub z_34: u32_XE_,
    pub z_35: u32_XE_,
    pub z_36: u32_XE_,
    pub z_37: u32_XE_,
    pub z_38: u32_XE_,
    pub z_39: u32_XE_,
    pub unk_40: u32_XE_,
    pub unk_41: u32_XE_,
    pub unk_42: u32_XE_,
    pub unk_43: u32_XE_,
    pub unk_44: u32_XE_,
    pub unk_45: u32_XE_,
    pub unk_46: u32_XE_,
    pub unk_47: u32_XE_,
    pub unk_48: u32_XE_,
    pub unk_49: u32_XE_,
    pub flags: u64_XE_, //Q', #(flags1, flags2)
    pub kind: u32_XE_,
    pub unk_53: u32_XE_,
    pub unk_54a: u8,
    pub unk_54b: u8,
    pub side_flags: u16_XE_,
    pub unk_55: u32_XE_,
    pub unk_56: u32_XE_,
    pub unk_57: u32_XE_,
    pub unk_58: f32_XE_,
    pub unk_59: f32_XE_,
    pub unk_60: f32_XE_,
    pub unk_61: f32_XE_,
    pub unk_62: f32_XE_,
    pub unk_63: f32_XE_,
    pub unk_64: f32_XE_,
    pub unk_65: f32_XE_,
    pub unk_66: f32_XE_,
    pub unk_67: f32_XE_,
    pub unk_68: f32_XE_,
    pub unk_69: f32_XE_,
    pub unk_70: u32_XE_,
    pub unk_71: u32_XE_,
    pub unk_72: u32_XE_,
    pub unk_73: f32_XE_,
    pub unk_74: f32_XE_,
    pub unk_75: f32_XE_,
    pub unk_76: f32_XE_,
    pub unk_77: u32_XE_,
    pub unk_78: f32_XE_,
    pub unk_79: f32_XE_,
    pub unk_80: f32_XE_,
    pub unk_81: f32_XE_,
    pub unk_82: u32_XE_,
    pub unk_83: u32_XE_,
    pub unk_84: u32_XE_,
    pub unk_85: f32_XE_,
    pub mat_extra_offset: u32_XE_,
    pub key: Crc_XE_,
    pub unk_88: u32_XE_,
    pub z_89: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat2Extra_XE_ {
    pub unk_90: u32_XE_,
    pub unk_91: u32_XE_,
    pub unk_92: u32_XE_,
    pub unk_93: u32_XE_,
    pub unk_94: u32_XE_,
    pub unk_95: u32_XE_,
    pub unk_96: u32_XE_,
    pub unk_97: u32_XE_,
    pub unk_98: u32_XE_,
    pub unk_99: u32_XE_,
    pub unk_100: u32_XE_,
    pub unk_101: u32_XE_,
    pub unk_102: f32_XE_,
    pub unk_103: f32_XE_,
    pub unk_104: f32_XE_,
    pub unk_105: f32_XE_,
    pub unk_106: f32_XE_,
    pub unk_107: f32_XE_,
    pub unk_108: f32_XE_,
    pub unk_109: f32_XE_,
    pub unk_110: f32_XE_,
    pub unk_111: f32_XE_,
    pub unk_112: f32_XE_,
    pub unk_113: f32_XE_,
    pub unk_114: u32_XE_,
    pub unk_115: u32_XE_,
    pub unk_116: u32_XE_,
    pub unk_117: Crc_XE_,
    pub unk_118: Crc_XE_,
    pub unk_119: Crc_XE_,
    pub unk_120a: u8,
    pub unk_120b: u8,
    pub unk_120c: u8,
    pub unk_120d: u8,
    pub unk_121: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat2_XE_ {
    base: Mat1_XE_,
    extra: Mat2Extra_XE_
}

#[make_endian]
pub struct Mat3ExtraPS3_XE_ {
    pub unk_90: f32_XE_,
    pub unk_91: f32_XE_,
    pub unk_92: f32_XE_,
    pub unk_93: f32_XE_,
    pub unk_94: f32_XE_,
    pub unk_95: f32_XE_,
    pub unk_96: f32_XE_,
    pub unk_97: f32_XE_,
    pub unk_98: f32_XE_,
    pub unk_99: f32_XE_,
    pub unk_100: f32_XE_,
    pub unk_101: f32_XE_,
    pub unk_102: f32_XE_,
    pub unk_103: f32_XE_,
    pub unk_104: f32_XE_,
    pub unk_105: f32_XE_,
    pub unk_106: f32_XE_,
    pub unk_107: f32_XE_,
    pub unk_108: f32_XE_,
    pub unk_109: f32_XE_,
    pub unk_110: f32_XE_,
    pub unk_111: f32_XE_,
    pub unk_112: f32_XE_,
    pub unk_113: f32_XE_,
    pub variation_id_color: u8,
    pub variation_id_texture: u8,
    pub variation_id_specular: u8,
    pub unk_114d: u8,
    pub unk_115: u32_XE_,
    pub unk_116: u32_XE_,
    pub unk_117: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat3Extra_XE_ {
    pub unk_90: f32_XE_,
    pub unk_91: f32_XE_,
    pub unk_92: f32_XE_,
    pub unk_93: f32_XE_,
    pub unk_94: f32_XE_,
    pub unk_95: f32_XE_,
    pub unk_96: f32_XE_,
    pub unk_97: f32_XE_,
    pub unk_98: f32_XE_,
    pub unk_99: f32_XE_,
    pub unk_100: f32_XE_,
    pub unk_101: f32_XE_,
    pub unk_102: f32_XE_,
    pub unk_103: f32_XE_,
    pub unk_104: f32_XE_,
    pub unk_105: f32_XE_,
    pub unk_106: f32_XE_,
    pub unk_107: f32_XE_,
    pub unk_108: f32_XE_,
    pub unk_109: f32_XE_,
    pub unk_110: f32_XE_,
    pub unk_111: f32_XE_,
    pub unk_112: f32_XE_,
    pub unk_113: f32_XE_,
    pub variation_id_color: u8,
    pub variation_id_texture: u8,
    pub variation_id_specular: u8,
    pub unk_114d: u8,
    pub unk_115: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat3_XE_ {
    base: Mat1_XE_,
    extra: Mat3Extra_XE_
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat4Extra_XE_ {
    pub unk_90: f32_XE_,
    pub unk_91: u32_XE_,
    pub unk_92: u32_XE_,
    pub unk_93: u32_XE_,
    pub unk_94: u32_XE_,
    pub unk_95: u32_XE_,
    pub unk_96: f32_XE_,
    pub unk_97: u32_XE_,
    pub unk_98: f32_XE_,
    pub unk_99: u32_XE_,
    pub unk_100: u32_XE_,
    pub unk_101: u32_XE_,
    pub unk_102: u32_XE_,
    pub unk_103: u32_XE_,
    pub unk_104: f32_XE_,
    pub unk_105: u32_XE_,
    pub unk_106: f32_XE_,
    pub unk_107: u32_XE_,
    pub unk_108: u32_XE_,
    pub unk_109: f32_XE_,
    pub unk_110: u32_XE_,
    pub unk_111: u32_XE_,
    pub unk_112: f32_XE_,
    pub unk_113: f32_XE_,
    pub unk_114: f32_XE_,
    pub unk_115: u32_XE_,
    pub unk_116: u32_XE_,
    pub unk_117: f32_XE_,
    pub unk_118: u32_XE_,
    pub unk_119: u32_XE_,
    pub unk_120: f32_XE_,
    pub unk_121: f32_XE_,
    pub unk_122: f32_XE_,
    pub unk_123: u32_XE_,
    pub unk_124: u32_XE_,
    pub unk_125: f32_XE_,
    pub unk_126: u32_XE_,
    pub unk_127: u32_XE_,
    pub unk_128: f32_XE_,
    pub unk_129: f32_XE_,
    pub unk_130: f32_XE_,
    pub unk_131: u32_XE_,
    pub unk_132: u32_XE_,
    pub unk_133: f32_XE_,
    pub unk_134: u32_XE_,
    pub unk_135: u32_XE_,
    pub unk_136: f32_XE_,
    pub unk_137: f32_XE_,
    pub unk_138: f32_XE_,
    pub unk_139: u32_XE_,
    pub unk_140: u32_XE_,
    pub unk_141: f32_XE_,
    pub unk_142: u32_XE_,
    pub unk_143: u32_XE_,
    pub unk_144: f32_XE_,
    pub unk_145: f32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat4_XE_ {
    base: Mat1_XE_,
    extra: Mat4Extra_XE_
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MatExtra_XE_ {
    pub unk_0: u32_XE_,
    pub unk_1: u32_XE_,
    pub unk_2: u32_XE_,
    pub unk_3: u32_XE_,
    pub unk_4: u32_XE_,
    pub unk_5: u32_XE_,
    pub unk_6: u32_XE_,
    pub unk_7: u32_XE_,
    pub unk_8: u32_XE_,
    pub unk_9: u32_XE_,
    pub unk_10: u32_XE_,
    pub unk_11: f32_XE_,
    pub unk_12: f32_XE_,
    pub unk_13: f32_XE_,
    pub unk_14: f32_XE_,
    pub unk_15: f32_XE_,
    pub unk_16: u32_XE_,
    pub unk_17: f32_XE_,
    pub unk_18: f32_XE_,
    pub unk_19: f32_XE_,
    pub unk_20: f32_XE_,
    pub unk_21: f32_XE_,
    pub unk_22: u32_XE_,
    pub unk_23: u32_XE_,
    pub unk_24: u32_XE_,
    pub unk_25: u32_XE_,
    pub unk_26: u32_XE_,
    pub unk_27: u32_XE_,
    pub unk_28: u32_XE_,
    pub unk_29: u32_XE_,
    pub unk_30: u32_XE_,
    pub unk_31: u32_XE_,
    pub unk_32: u32_XE_,
    pub unk_33: u32_XE_,
    pub unk_34: u32_XE_,
    pub unk_35: u32_XE_,
    pub unk_36: u32_XE_,
    pub unk_37: u32_XE_,
    pub unk_38: u32_XE_,
    pub unk_39: u32_XE_,
    pub unk_40: u32_XE_,
    pub unk_41: u32_XE_,
    pub unk_42: u32_XE_,
    pub unk_43: u32_XE_,
    pub unk_44: u32_XE_,
    pub unk_45: u32_XE_,
    pub unk_46: u32_XE_,
    pub unk_47: u32_XE_,
    pub unk_48: u32_XE_,
    pub unk_49: u32_XE_,
}

#[make_endian]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat1Ref_XE_<'a> {
    pub info: &'a Mat1_XE_,
    pub extra: Option<&'a MatExtra_XE_>
}

#[make_endian]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat2Ref_XE_<'a> {
    pub info: &'a Mat2_XE_,
    pub extra: Option<&'a MatExtra_XE_>
}

#[make_endian]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat3Ref_XE_<'a> {
    pub info: &'a Mat3_XE_,
    pub extra: Option<&'a MatExtra_XE_>
}

#[make_endian]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Mat4Ref_XE_<'a> {
    pub info: &'a Mat4_XE_,
    pub extra: Option<&'a MatExtra_XE_>
}

#[make_endian]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum MatRef_XE_<'a> {
    Mat1(Mat1Ref_XE_<'a>),
    Mat2(Mat2Ref_XE_<'a>),
    Mat3(Mat3Ref_XE_<'a>),
    Mat4(Mat4Ref_XE_<'a>),
}

#[make_endian]
impl<'a> MatRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], offset: usize) -> Result<Self> {
        let base = Mat1_XE_::from_data(&src[offset..]).context("base")?;
        let extra = if base.mat_extra_offset != 0 {
            Some(MatExtra_XE_::from_data(&src[base.mat_extra_offset.conv()..]).context("extra")?)
        } else {
            None
        };
        Ok(match base.kind.conv() {
            0u32 => Self::Mat1(Mat1Ref_XE_ {
                info: Mat1_XE_::from_data(&src[offset..]).context("mat1")?,
                extra,
            }),
            1 => Self::Mat4(Mat4Ref_XE_ {
                info: Mat4_XE_::from_data(&src[offset..]).context("mat4")?,
                extra,
            }),
            2 => Self::Mat2(Mat2Ref_XE_ {
                info: Mat2_XE_::from_data(&src[offset..]).context("mat2")?,
                extra,
            }),
            3 => Self::Mat3(Mat3Ref_XE_ {
                info: Mat3_XE_::from_data(&src[offset..]).context("mat3")?,
                extra,
            }),
            _ => return Err(anyhow::anyhow!("Unknown Mat Type {}", base.kind.to_native())),
        })
    }
}


#[make_endian]
pub trait DumpMat_XE_ {
    fn dump_infos(&self, infos: &mut DumpInfos_XE_) -> Result<u32>;
    fn add_counts(&self, counts: &mut InfoCounts);
}

#[make_endian]
impl DumpMat_XE_ for MatRef_XE_<'_> {
    fn dump_infos(&self, infos: &mut DumpInfos_XE_) -> Result<u32> {
        match self {
            Self::Mat1(Mat1Ref_XE_ { info, extra}) => {
                let val = infos.mat1s.offset;
                let mat = infos.mat1s.next().context("mat1s")?;
                mat.write_from(info)?;
                mat.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().conv();
                if let Some(extra) = extra {
                    infos.mat_extras.next().context("mat_extras")?.write_from(extra)?;
                    *infos.offsets.next().context("offsets")? = (val + std::mem::offset_of!(Mat1_XE_, mat_extra_offset)).conv();
                }
                Ok(val as u32)
            }
            Self::Mat2(Mat2Ref_XE_ { info, extra }) => {
                let val = infos.mat2s.offset;
                let mat = infos.mat2s.next().context("mat2s")?;
                mat.write_from(info)?;
                mat.base.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().conv();
                if let Some(extra) = extra {
                    infos.mat_extras.next().context("mat_extras")?.write_from(extra)?;
                    *infos.offsets.next().context("offsets")? = (val + std::mem::offset_of!(Mat2_XE_, base) + std::mem::offset_of!(Mat1_XE_, mat_extra_offset)).conv();
                }
                Ok(val as u32)
            }
            Self::Mat3(Mat3Ref_XE_ { info, extra }) => {
                let val = infos.mat3s.offset;
                let mat = infos.mat3s.next().context("mat3s")?;
                mat.write_from(info)?;
                mat.base.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().conv();
                if let Some(extra) = extra {
                    infos.mat_extras.next().context("mat_extras")?.write_from(extra)?;
                    *infos.offsets.next().context("offsets")? = (val + std::mem::offset_of!(Mat3_XE_, base) + std::mem::offset_of!(Mat1_XE_, mat_extra_offset)).conv();
                }
                Ok(val as u32)
            }
            Self::Mat4(Mat4Ref_XE_ { info, extra }) => {
                let val = infos.mat4s.offset;
                let mat = infos.mat4s.next().context("mat4s")?;
                mat.write_from(info)?;
                mat.base.mat_extra_offset = extra.map(|_| infos.mat_extras.offset as u32).unwrap_or_default().conv();
                if let Some(extra) = extra {
                    infos.mat_extras.next().context("mat_extras")?.write_from(extra)?;
                    *infos.offsets.next().context("offsets")? = (val + std::mem::offset_of!(Mat4_XE_, base) + std::mem::offset_of!(Mat1_XE_, mat_extra_offset)).conv();
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

#[make_endian]
impl From<&MatRef_XE_<'_>> for Mat {
    fn from(val: &MatRef_XE_) -> Self {
        match val {
            MatRef_XE_::Mat1(Mat1Ref_XE_ { info, extra }) => Self::Normal(info.conv(), extra.map(|x| x.conv())),
            MatRef_XE_::Mat2(Mat2Ref_XE_ { info, extra }) => Self::Variation(info.conv(), extra.map(|x| x.conv())),
            MatRef_XE_::Mat3(Mat3Ref_XE_ { info, extra }) => {
                Self::CharacterVariation(info.conv(), extra.map(|x| x.conv()))
            }
            MatRef_XE_::Mat4(Mat4Ref_XE_ { info, extra }) => Self::Terrain(info.conv(), extra.map(|x| x.conv())),
        }
    }
}

/*
impl Mat {
    #[make_endian]
    pub fn get_raw_ver(&self) -> (Mat_XE_, Option<MatExtra_XE_>) {
        match self {
            Self::Normal(mat, extra) => (
                Mat_XE_::Mat1(Mat1_XE_::from(mat.clone())),
                extra.as_ref().map(|x| MatExtra_XE_::from(x.clone())),
            ),
            Self::Variation(mat, extra) => (
                Mat_XE_::Mat2(Mat2_XE_::from(mat.clone())),
                extra.as_ref().map(|x| MatExtra_XE_::from(x.clone())),
            ),
            Self::CharacterVariation(mat, extra) => (
                Mat_XE_::Mat3(Mat3_XE_::from(mat.clone())),
                extra.as_ref().map(|x| MatExtra_XE_::from(x.clone())),
            ),
            Self::Terrain(mat, extra) => (
                Mat_XE_::Mat4(Mat4_XE_::from(mat.clone())),
                extra.as_ref().map(|x| MatExtra_XE_::from(x.clone())),
            ),
        }
    }
}
*/
