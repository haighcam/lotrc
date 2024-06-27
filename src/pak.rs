use std::{any::TypeId, collections::HashMap, fmt::Display, iter::zip, num::ParseIntError, ops::Div, str::FromStr};
use log::warn;
use zerocopy::{ByteOrder, BE};
use serde::{Serialize, Deserialize};

use lotrc_rs_proc::OrderedData;
use super::types::{BaseTypes, OrderedData, Vector4, Matrix4x4, OrderedDataVec, Vector2, Crc, Vector3};

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Header {
    #[ordered_data(LE)]
    pub block_a_num: u32, 
    #[ordered_data(LE)]
    pub block_a_offset: u32, 
    pub constx13: u32, 
    pub version: u32, 
    pub strings_offset: u32, 
    pub strings_size: u32, 
    pub strings_num: u32, 
    pub block1_offset: u32, 
    pub block1_size: u32, 
    pub block1_size_comp: u32, 
    pub sub_blocks1_offset: u32, 
    pub block2_offset: u32, 
    pub block2_size: u32, 
    pub block2_size_comp: u32, 
    pub sub_blocks2_offset: u32, 
    pub string_keys_offset: u32, 
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
    pub unk_34: u32, 
    pub unk_35: u32, 
    pub unk_36: u32, 
    pub unk_37: u32, 
    pub unk_38: u32, 
    pub unk_39: u32, 
    pub unk_40: u32, 
    pub unk_41: u32, 
    pub obja_num: u32, 
    pub obj0_num: u32, 
    pub mesh_info_num: u32,
    pub buffer_info_num: u32,
    pub mat1_num: u32, 
    pub mat2_num: u32, 
    pub mat3_num: u32, 
    pub mat4_num: u32, 
    pub mat_extra_num: u32, 
    pub unk_51: u32, 
    pub shape_info_num: u32, 
    pub hk_shape_info_num: u32,
    pub hk_constraint_data_num: u32,
    pub vbuff_info_num: u32,
    pub ibuff_info_num: u32,
    pub texture_info_num: u32,
    pub animation_info_num: u32,
    pub hk_constraint_info_num: u32,
    pub effect_info_num: u32,
    pub pfield_info_num: u32,
    pub gfx_block_info_num: u32, 
    pub animation_block_info_num: u32, 
    pub foliage_info_num: u32, 
    pub illumination_info_num: u32, 
    pub unk_66: u32, 
    pub obja_offset: u32,
    pub obj0_offset: u32, 
    pub mesh_info_offset: u32,  // max loaded is 0x400
    pub buffer_info_offset: u32, 
    pub mat1_offset: u32, 
    pub mat2_offset: u32, 
    pub mat3_offset: u32, 
    pub mat4_offset: u32, 
    pub mat_extra_offset: u32, 
    pub unk_76: u32, 
    pub shape_info_offset: u32, 
    pub hk_shape_info_offset: u32, 
    pub hk_constraint_data_offset: u32, 
    pub vbuff_info_offset: u32, 
    pub ibuff_info_offset: u32, 
    pub texture_info_offset: u32,  // max loaded is 0x800
    pub animation_info_offset: u32, 
    pub hk_constraint_info_offset: u32, 
    pub effect_info_offset: u32, 
    pub pfield_info_offset: u32, 
    pub gfx_block_info_offset: u32,  // max loaded is 0x40
    pub animation_block_info_offset: u32,
    pub foliage_info_offset: u32, 
    pub illumination_info_offset: u32, 
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
    pub unk_102: u32, 
    pub unk_103: u32, 
    pub unk_104: u32, 
    pub unk_105: u32, 
    pub unk_106: u32, 
    pub unk_107: u32, 
    pub unk_108: u32, 
    pub unk_109: u32, 
    pub unk_110: u32, 
    pub unk_111: u32, 
    pub unk_112: u32, 
    pub unk_113: u32, 
    pub unk_114: u32, 
    pub unk_115: u32, 
    pub block2_offsets_num: u32, 
    pub block2_offsets_offset: u32, 
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct ObjA {
    #[ordered_data(LE)]
    pub key: Crc,
    #[ordered_data(LE)]
    pub unk_1: u32,
    #[ordered_data(LE)]
    pub size: u32,
    #[ordered_data(LE)]
    pub size_comp: u32,
    #[ordered_data(LE)]
    pub unk_4: u32,
    #[ordered_data(LE)]
    pub kind: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Obj0{
    #[ordered_data(LE)]
    pub unk_0: u32,
    #[ordered_data(LE)]
    pub key: Crc,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct MeshInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub mat_offset: u32,
    pub buffer_info_offset: u32, // pointer to obj2, uses mat_num of sequential objects
    pub unk_4: u32,
    pub unk_5: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: u32,
    pub vals_c_offset: u32, // ints (c & 0x3fffffff is an index into the obj2s referenced by this object)
    pub unk_13: u32, // (v1, v2, v3, v4, v5) * 4 (up to unk_23), v1 is a starting offset to obj2s, v2 is the end offset
    pub unk_14: u32,
    pub block_start: u32,
    pub block_end: u32,
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
    pub vals_c_num: u32,
    pub mat_num: u32,
    pub keys_offset: u32, // ints
    pub indices_offset: u32,
    pub matrices_offset: u32, // 16 ints (matrix?) for keys_num
    pub keys_num: u32,
    pub vals_g_offset: u32,
    pub vals_g_num: u32,
    pub vals_i_offset: u32,
    pub vbuff_offset: u32,
    pub vbuff_num: u32,
    pub ibuff_offset: u32,
    pub ibuff_num: u32,
    pub vals_d_offset: u32, // f_num * 8 ints
    pub unk_46: u32,
    pub unk_47: u32,
    pub vals_j_num: u32,
    pub vals_j_offset: u32,
    pub block_offset: u32,
    pub vals_k_offset: u32, // not sure on the size, seems to be 36 ints
    pub asset_key: Crc, // data in bin that is vertex & index buffer values
    pub asset_type: u32,
    pub unk_54: u32,
    pub unk_55: u32,
    pub shape_offset: u32,
    pub shape_num: u32,
    pub hk_constraint_data_offset: u32, // optional pointer to obje
    pub hk_constraint_data_num: u32,
    pub hk_constraint_offset: u32, // optional pointer to hkConstraint
    pub keys2_offset: u32,
    pub keys2_order_offset: u32,
    pub vals_a_offset: u32, // 8 ints
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct BufferInfo {
    pub vbuff_info_offset: u32, // pointer to objf
    pub vbuff_info_offset_2: u32, // optional pointer to objf
    pub vbuff_info_offset_3: u32, // optional pointer to objf
    pub unk_3: u32,
    pub unk_4: u32,
    pub unk_5: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: u32,
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
    pub v_size: u32,
    pub v_size_2: u32,
    pub v_size_3: u32,
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
    pub vbuff_size: u32,
    pub vbuff_size_2: u32,
    pub vbuff_size_3: u32,
    pub unk_51: u32,
    pub unk_52: u32,
    pub unk_53: u32,
    pub unk_54: u32,
    pub unk_55: u32,
    pub unk_56: u32,
    pub unk_57: u32,
    pub unk_58: u32,
    pub unk_59: u32,
    pub unk_60: u32,
    pub unk_61: u32,
    pub unk_62: u32,
    pub unk_63: u32,
    pub unk_64: u32,
    pub ibuff_info_offset: u32, // poiner to objg
    pub i_num: u32, // number of indeices in ibuffer
    pub unk_67: u32,
    pub unk_68: u32,
    pub unk_69: u32,
    pub unk_70: u32,
    pub tri_num: u32, // number of objects(triangles) in ibufffer
    pub unk_72: u32,
    pub unk_73: u32,
    pub unk_74: u32,
    pub unk_75: u32,
    pub unk_76: u32,
    pub unk_77: u32,
    pub unk_78: u32,
    pub unk_79: u32,
    pub unk_80: u32,
    pub unk_81: u32,
    pub unk_82: u32,
    pub unk_83: u32,
    pub unk_84: u32,
    pub unk_85: u32,
    pub unk_86: u32,
    pub unk_87: u32,
    pub unk_88a: u8,
    pub unk_88b: u8,
    pub unk_88c: u8,
    pub unk_88d: u8,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct MatBase {
    pub unk_0: u32,
    pub unk_1: u32,
    pub tex_2: Crc,
    pub tex_3: Crc,
    pub tex_4: Crc,
    pub tex_5: Crc,
    pub tex_6: Crc,
    pub tex_7: Crc,
    pub tex_8: Crc,
    pub tex_9: Crc,
    pub tex_10: Crc,
    pub tex_11: Crc,
    pub tex_12: Crc,
    pub tex_13: Crc,
    pub tex_14: Crc,
    pub tex_15: Crc,
    pub tex_16: Crc,
    pub tex_17: Crc,
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
    pub unk_58: u32,
    pub unk_59: u32,
    pub unk_60: u32,
    pub unk_61: u32,
    pub unk_62: u32,
    pub unk_63: u32,
    pub unk_64: u32,
    pub unk_65: u32,
    pub unk_66: u32,
    pub unk_67: u32,
    pub unk_68: u32,
    pub unk_69: u32,
    pub unk_70: u32,
    pub unk_71: u32,
    pub unk_72: u32,
    pub unk_73: u32,
    pub unk_74: u32,
    pub unk_75: u32,
    pub unk_76: u32,
    pub unk_77: u32,
    pub unk_78: u32,
    pub unk_79: u32,
    pub unk_80: u32,
    pub unk_81: u32,
    pub unk_82: u32,
    pub unk_83: u32,
    pub unk_84: u32,
    pub unk_85: u32,
    pub mat_extra_offset: u32,
    pub key: u32,
    pub unk_88: u32,
    pub z_89: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Mat1 {
    pub base: MatBase,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
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
    pub unk_102: u32,
    pub unk_103: u32,
    pub unk_104: u32,
    pub unk_105: u32,
    pub unk_106: u32,
    pub unk_107: u32,
    pub unk_108: u32,
    pub unk_109: u32,
    pub unk_110: u32,
    pub unk_111: u32,
    pub unk_112: u32,
    pub unk_113: u32,
    pub unk_114: u32,
    pub unk_115: u32,
    pub unk_116: u32,
    pub unk_117: u32,
    pub unk_118: u32,
    pub unk_119: u32,
    pub unk_120a: u8,
    pub unk_120b: u8,
    pub unk_120c: u8,
    pub unk_120d: u8,
    pub unk_121: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Mat3 {
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
    pub unk_102: u32,
    pub unk_103: u32,
    pub unk_104: u32,
    pub unk_105: u32,
    pub unk_106: u32,
    pub unk_107: u32,
    pub unk_108: u32,
    pub unk_109: u32,
    pub unk_110: u32,
    pub unk_111: u32,
    pub unk_112: u32,
    pub unk_113: u32,
    pub unk_114a: u8,
    pub unk_114b: u8,
    pub unk_114c: u8,
    pub unk_114d: u8,
    pub unk_115: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Mat4 {
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
    pub unk_102: u32,
    pub unk_103: u32,
    pub unk_104: u32,
    pub unk_105: u32,
    pub unk_106: u32,
    pub unk_107: u32,
    pub unk_108: u32,
    pub unk_109: u32,
    pub unk_110: u32,
    pub unk_111: u32,
    pub unk_112: u32,
    pub unk_113: u32,
    pub unk_114: u32,
    pub unk_115: u32,
    pub unk_116: u32,
    pub unk_117: u32,
    pub unk_118: u32,
    pub unk_119: u32,
    pub unk_120: u32,
    pub unk_121: u32,
    pub unk_122: u32,
    pub unk_123: u32,
    pub unk_124: u32,
    pub unk_125: u32,
    pub unk_126: u32,
    pub unk_127: u32,
    pub unk_128: u32,
    pub unk_129: u32,
    pub unk_130: u32,
    pub unk_131: u32,
    pub unk_132: u32,
    pub unk_133: u32,
    pub unk_134: u32,
    pub unk_135: u32,
    pub unk_136: u32,
    pub unk_137: u32,
    pub unk_138: u32,
    pub unk_139: u32,
    pub unk_140: u32,
    pub unk_141: u32,
    pub unk_142: u32,
    pub unk_143: u32,
    pub unk_144: u32,
    pub unk_145: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
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
    pub unk_11: u32,
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

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct ShapeInfo {
    pub offset: u32, // sometimes a pointer to something, otherwise the number of strings from the obj1 pointing to this
    pub kind: u32, // 0, 1, 2, 3, 4, 5
    pub unk_2: u32,
    pub unk_3: u32,
    pub unk_4: u32,
    pub unk_5: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: u32,
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
    pub hk_shape_num: u32,
    pub hk_shape_offset: u32, // pointer to objd
    pub unk_29a: u8,
    pub unk_29b: u8,
    pub unk_29c: u8,
    pub unk_29d: u8,
    pub unk_30: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct HkShapeInfo {
    pub unk_0: Vector4,
    pub unk_4: Vector4,
    pub kind: u32,
    pub unk_9: u32,
    pub a_num: u32,
    pub a_offset: u32,
    pub b_num: u32,
    pub b_offset: u32,
    pub c_num: u32,
    pub c_offset: u32,
    pub d_num: u32,
    pub d_offset: u32,
    pub e_num: u32,
    pub e_offset: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct HkConstraintData {
    pub kind: u32,
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
    pub unk_11: u32,
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
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct VBuffInfo {
    pub unk_0: u32,
    pub size: u32,
    pub unk_3: u32,
    pub offset: u32,
    #[name_be(fmt2)]
    pub fmt1: u32,
    #[name_be(fmt1)]
    pub fmt2: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    #[ordered_data(skipLE)]
    pub unk_8: u32,
    #[ordered_data(skipLE)]
    pub unk_9: u32,
    #[ordered_data(skipLE)]
    pub unk_10: u32,
    #[ordered_data(skipLE)]
    pub unk_11: u32,
    #[ordered_data(skipLE)]
    pub unk_12: u32,
    #[ordered_data(skipLE)]
    pub unk_13: u32,

}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct IBuffInfo {
    pub unk_0: u32,
    pub size: u32,
    pub format: u32,
    pub unk_3: u32,
    pub offset: u32,
    pub unk_5: u32,
    #[ordered_data(skipLE)]
    pub unk_6: u32,
    #[ordered_data(skipLE)]
    pub unk_7: u32,
    #[ordered_data(skipLE)]
    pub unk_8: u32,
    #[ordered_data(skipLE)]
    pub unk_9: u32,
    #[ordered_data(skipLE)]
    pub unk_10: u32,
    #[ordered_data(skipLE)]
    pub unk_11: u32,
    #[ordered_data(skipLE)]
    pub unk_12: u32,

}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct TextureInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub asset_key: Crc,
    pub asset_type: u32,
    pub kind: u32,
    pub format: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: u32,
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub levels: u16,
    pub unk_16_1: u8,
    pub unk_16_2: u8,
    pub unk_16_3: u8,
    pub unk_16_4: u8,
    pub unk_16_5: u8,
    pub unk_16_6: u8,
    pub unk_16_7: u8,
    pub unk_16_8: u8,
    pub unk_16_9: u8,
    pub unk_16_10: u8,
    pub unk_16_11: u8,
    pub unk_16_12: u8,
    pub unk_16_13: u8,
    pub unk_16_14: u8,
    pub unk_16_15: u8,
    pub unk_16_16: u8,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct AnimationInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub offset: u32,
    pub size: u32,
    pub kind: u32,
    pub unk_5: u32,
    pub keys_num: u32,
    pub keys2_num: u32,
    pub unk_8: u32,
    pub vala: u32,
    pub unk_10: u32,
    pub unk_11: u32,
    pub data_offset: u32,
    pub unk_13: u32,
    pub unk_14: u32,
    pub unk_15: u32,
    pub block_starts_offset: u32,
    pub block_starts_num: u32,
    pub block_starts2_offset: u32,
    pub block_starts2_num: u32,
    pub obj_c3_offset: u32,
    pub obj_c3_num: u32,
    pub obj_c4_offset: u32,
    pub obj_c4_num: u32,
    pub block_offset: u32,
    pub block_size: u32,
    pub obj3_num: u32,
    pub obj3_offset: u32,
    pub unk_28: u32,
    pub unk_29: u32,
    pub obj1_num: u32,
    pub keys_offset: u32,
    pub unk_32: u32,
    pub obj1_offset: u32,
    pub obj2_offset: u32,
    pub obj2_num: u32,
    pub obj5_offset: u32, // to some object that contains offsets in pos 1 and 2 and a value in pos 0
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct HkConstraintInfo {
    pub kind: u32,
    pub shorts_offset: u32, 
    pub shorts_num: u32,
    pub strings_offset: u32,
    pub strings_num: u32,
    pub vals_offset: u32,
    pub vals_num: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub keys_offset: u32,
    pub keys_num: u16,
    pub keys2_num: u16,
    pub keys2_offset: u32,
    pub unk_13: u32,
    pub unk_14: f32, //f',
    pub vals2_num: u32,
    pub vals2_offset: u32,
    pub unk_17: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct EffectInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct PFieldInfo {
    pub key1: Crc, 
    pub key2: Crc, 
    pub width: u32, 
    pub height: u32, 
    pub offset: u32, 
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct GFXBlockInfo {
    // // GFX blocks?, unchanged by encoding, model as data
    pub key: Crc,
    pub offset: u32, // offset pointing to something in block1
    pub size: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct AnimationBlockInfo {
    pub key: Crc,
    pub unk_1: u32,
    pub key_name: u32,
    pub offset: u32,
    pub size: u32,
    pub size_comp: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct FoliageInfo {
    pub key: Crc, 
    pub unk_1: u32, 
    pub s1a: i32, 
    pub s2a: i32, 
    pub s1b: i32, 
    pub s2b: i32, 
    pub unk_6: u32, 
    pub offset: u32, 
    pub key_mesh: Crc, 
    pub key_mesh_lod1: Crc, 
    pub key_mesh_lod2: Crc, 
    pub unk_11: u32, 
    pub unk_12: u32, 
    pub unk_13: u32, 
    pub unk_14: u32, 
    pub unk_15: u32, 
    pub unk_16: u32, 
    pub unk_17: u32, 
    pub unk_18: u32, 
    pub unk_19: u32, 
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct IlluminationInfo {
    // points to list of ints in block1, maybe something to do with radiosity
    pub guid: u32,
    pub num: u32,
    pub offset: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct BlockAVal {
    pub unk_0: u32,
    pub gamemodemask: i32,
    pub key: Crc,
    pub unk_3: u32,
    pub unk_4: u32,
    pub unk_5: u32,
    pub unk_6: u32,
}

pub mod mesh {
    use super::*;
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct BlockHeader {
        pub a: u32,
        pub b: u32,
        pub unk_2: u32,
        pub unk_3: u32,
        pub unk_4: u32,
    }
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct BlockVal {
        pub unk_0: u32,
        pub unk_1: u32,
        pub unk_2: u32,
        pub unk_3: u32,
        pub unk_4: u16,
        pub unk_5: u16,
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Mesh {
    pub indices: Vec<u32>,
    pub keys: Vec<u32>,
    pub matrices: Vec<Matrix4x4>,
    pub vals_a: Vec<u32>,
    pub mats: Vec<u32>,
    pub vals_c: Vec<u32>,
    pub vals_d: Vec<u32>,
    pub vbuffs: Vec<u32>,
    pub ibuffs: Vec<u32>,
    pub vals_g: Vec<u32>,
    pub vals_j: Vec<u32>,
    pub string_offsets: Vec<u32>,
    pub strings: Vec<String>,
    pub val_k_header: Vec<u16>,
    pub vals_k: Vec<u32>,
    pub vals_i: Vec<u32>,
    pub keys2: Vec<u32>,
    pub keys2_order: Vec<u32>,
    pub block_header: u32,
    pub block_offsets: Vec<u32>,
    pub blocks: Vec<(mesh::BlockHeader, Vec<u32>, Vec<mesh::BlockVal>, Vec<u32>)>,
    pub val: Vec<u32>,
}

impl Mesh {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &MeshInfo) -> Self {
        let mut val = Self::default();

        val.indices = OrderedDataVec::from_bytes::<O>(&data[info.indices_offset as usize..], info.keys_num.max(4) as usize);
        assert!(val.indices[0] == 0xffffffff);
        val.keys = OrderedDataVec::from_bytes::<O>(&data[info.keys_offset as usize..], info.keys_num as usize);
        val.matrices = OrderedDataVec::from_bytes::<O>(&data[info.matrices_offset as usize..], info.keys_num as usize);
        val.vals_a = OrderedDataVec::from_bytes::<O>(&data[info.vals_a_offset as usize..], info.keys_num as usize * 8);
        val.mats = OrderedDataVec::from_bytes::<O>(&data[info.mat_offset as usize..], info.mat_num as usize);
        val.vals_c = OrderedDataVec::from_bytes::<O>(&data[info.vals_c_offset as usize..], info.vals_c_num as usize);
        val.vals_d = OrderedDataVec::from_bytes::<O>(&data[info.vals_d_offset as usize..], info.vals_c_num as usize * 8);
        val.vbuffs = OrderedDataVec::from_bytes::<O>(&data[info.vbuff_offset as usize..], info.vbuff_num as usize);
        val.ibuffs = OrderedDataVec::from_bytes::<O>(&data[info.ibuff_offset as usize..], info.ibuff_num as usize);
        val.vals_g = OrderedDataVec::from_bytes::<O>(&data[info.vals_g_offset as usize..], info.vals_g_num as usize * 16);
        if (info.vals_j_num == 0) && (info.vals_j_offset != 0) && (info.vals_j_offset != info.vals_g_offset) {
            // val.vals_j = OrderedDataVec::from_bytes::<O>(&data[info.vals_j_offset as usize..], info.keys_num as usize);
            // for v in &val.vals_j {
            //     let mut offset: u32 = OrderedData::from_bytes::<O>(&data[*v as usize..]);
            //     let start = offset;
            //     while data[offset as usize] != 0 { offset += 1; }
            //     let string = String::from_utf8(data[start as usize..offset as usize].to_vec()).unwrap();
            //     val.string_offsets.push(start);
            //     val.strings.push(string);
            // }
        } else {
            val.vals_j = OrderedDataVec::from_bytes::<O>(&data[info.vals_j_offset as usize..], info.vals_j_num as usize);
        }
        if info.vals_k_offset != 0 {
            val.val_k_header = OrderedDataVec::from_bytes::<O>(&data[info.vals_k_offset as usize..], 2);
            if (val.val_k_header[0] != 3) || (val.val_k_header[0] != 6) {
                warn!("unexpected valsK data {:?}", info.key);
            }
            val.vals_k = OrderedDataVec::from_bytes::<O>(&data[info.vals_k_offset as usize + 4..], 35);
        }
        if info.vals_i_offset != 0 {
            val.vals_i = OrderedDataVec::from_bytes::<O>(&data[info.vals_i_offset as usize..], info.vals_g_num as usize);
        }
        if info.keys2_offset != 0 {
            assert!(info.keys2_order_offset != 0);
            let mut i = 0;
            {
                while u32::from_bytes::<O>(&data[info.keys2_offset as usize + i * 8..]) != 0 {
                    i += 1;
                }
                i += 1;
            }
            val.keys2 = OrderedDataVec::from_bytes::<O>(&data[info.keys2_offset as usize..], i * 2);
            val.keys2_order = OrderedDataVec::from_bytes::<O>(&data[info.keys2_order_offset as usize..], *val.keys2.last().unwrap() as usize);
        }
        if info.block_offset != 0 {
            val.block_header = OrderedData::from_bytes::<O>(&data[info.block_offset as usize..]);
            let n = (info.block_end - info.block_start) as usize;
            val.block_offsets = OrderedDataVec::from_bytes::<O>(&data[info.block_offset as usize + 4..], n+1);
            for i in 0..n {
                let size = (val.block_offsets[i+1] - val.block_offsets[i]) as usize;
                let offset = (val.block_offsets[i] + info.block_offset) as usize;
                let header: mesh::BlockHeader = OrderedData::from_bytes::<O>(&data[offset..]);
                let mut s = mesh::BlockHeader::size::<O>();
                let vals_a: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (header.a + header.b) as usize * 12);
                s += vals_a.size::<O>();
                let vals_b: Vec<mesh::BlockVal> = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (size - s)/mesh::BlockVal::size::<O>());
                s += vals_b.size::<O>();
                let extra = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (size - s)/4);
                val.blocks.push((header, vals_a, vals_b, extra));
            }
        }
        // not sure why this pops up once, maybe it is padding between items?
        if (info.vals_c_offset == info.vbuff_offset) && (info.vals_c_offset == info.ibuff_offset) && (info.vals_c_offset == info.vals_d_offset) {
            val.val = OrderedDataVec::from_bytes::<O>(&data[info.vals_c_offset as usize..], 4);
        }
        val
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], info: &MeshInfo) {
        self.indices.to_bytes::<O>(&mut data[info.indices_offset as usize..]);
        self.keys.to_bytes::<O>(&mut data[info.keys_offset as usize..]);
        self.matrices.to_bytes::<O>(&mut data[info.matrices_offset as usize..]);
        self.vals_a.to_bytes::<O>(&mut data[info.vals_a_offset as usize..]);
        self.mats.to_bytes::<O>(&mut data[info.mat_offset as usize..]);
        self.vals_c.to_bytes::<O>(&mut data[info.vals_c_offset as usize..]);
        self.vals_d.to_bytes::<O>(&mut data[info.vals_d_offset as usize..]);
        self.vbuffs.to_bytes::<O>(&mut data[info.vbuff_offset as usize..]);
        self.ibuffs.to_bytes::<O>(&mut data[info.ibuff_offset as usize..]);
        self.vals_g.to_bytes::<O>(&mut data[info.vals_g_offset as usize..]);
        if (info.vals_j_num) == 0 && (info.vals_j_offset != 0) && (info.vals_j_offset != info.vals_g_offset) {
            self.vals_j.to_bytes::<O>(&mut data[info.vals_j_offset as usize..]);
            for (v, (off, string)) in zip(&self.vals_j, zip(&self.string_offsets,& self.strings)) {
                off.to_bytes::<O>(&mut data[*v as usize..]);
                data[*off as usize..*off as usize+string.len()].copy_from_slice(string.as_bytes());
            }
        } else {
            self.vals_j.to_bytes::<O>(&mut data[info.vals_j_offset as usize..]);
        }
        if info.vals_k_offset != 0 {
            self.val_k_header.to_bytes::<O>(&mut data[info.vals_k_offset as usize..]);
            self.vals_k.to_bytes::<O>(&mut data[info.vals_k_offset as usize + 4..])
        }
        if info.vals_i_offset != 0 {
            self.vals_i.to_bytes::<O>(&mut data[info.vals_i_offset as usize..]);
        }
        if info.keys2_offset != 0 {
            self.keys2.to_bytes::<O>(&mut data[info.keys2_offset as usize..]);
            self.keys2_order.to_bytes::<O>(&mut data[info.keys2_order_offset as usize..]);
        }
        if info.block_offset != 0 {
            self.block_header.to_bytes::<O>(&mut data[info.block_offset as usize..]);
            self.block_offsets.to_bytes::<O>(&mut data[info.block_offset as usize + 4..]);
            for (i, (header, vals_a, vals_b, extra)) in self.blocks.iter().enumerate() {
                let offset = (self.block_offsets[i] + info.block_offset) as usize;
                header.to_bytes::<O>(&mut data[offset..]);
                let mut s = mesh::BlockHeader::size::<O>();
                vals_a.to_bytes::<O>(&mut data[offset + s..]);
                s += vals_a.size::<O>();
                vals_b.to_bytes::<O>(&mut data[offset + s..]);
                s += vals_b.size::<O>();
                extra.to_bytes::<O>(&mut data[offset + s..]);
            }
        }
        if (info.vals_c_offset == info.vbuff_offset) && (info.vals_c_offset == info.ibuff_offset) && (info.vals_c_offset == info.vals_d_offset) {
            self.val.to_bytes::<O>(&mut data[info.vals_c_offset as usize..]);
        }
    }
}

pub mod shape {
    use super::*;
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct Header {
        pub num: u32,
        pub unk_1: u32,
        pub unk_2: u32,
        pub unk_3: u32,
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub header: shape::Header,
    pub vals: Vec<u32>,
    pub data: Vec<u8>,
}

impl Shape {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &ShapeInfo) -> Self {
        let mut val = Self::default();
        if info.kind == 0 {
            let mut offset = info.offset as usize;
            val.header = OrderedData::from_bytes::<O>(&data[offset..]);
            offset += shape::Header::size::<O>();
            val.vals = OrderedDataVec::from_bytes::<O>(&data[offset..], val.header.num as usize);
            offset += val.vals.size::<O>();
            val.data = OrderedDataVec::from_bytes::<O>(&data[offset..], *val.vals.last().unwrap() as usize + 2) // might need to be more than +2, not sure    
        }
        val
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], info: &ShapeInfo) {
        if info.kind == 0 {
            let mut offset = info.offset as usize;
            self.header.to_bytes::<O>(&mut data[offset..]);
            offset += shape::Header::size::<O>();
            self.vals.to_bytes::<O>(&mut data[offset..]);
            offset += self.vals.size::<O>();
            self.data.to_bytes::<O>(&mut data[offset..]);
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HkShape {
    pub a: Vec<u32>,
    pub b: Vec<u32>,
    pub c: Vec<u8>,
    pub d: Vec<u32>,
    pub e: Vec<u16>,
}

impl HkShape {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &HkShapeInfo) -> Self {
        let mut val = Self::default();
        if info.kind == 5 {
            val.a = OrderedDataVec::from_bytes::<O>(&data[info.a_offset as usize..], info.a_num as usize * 4);
            let mut b_num = info.b_num as usize; // sketchy stuff to account for data that was not otherwise captured, is it needed?
            while (info.b_offset as usize + b_num * 12) % 16 != 0 { b_num += 1; }
            val.b = OrderedDataVec::from_bytes::<O>(&data[info.b_offset as usize..], b_num * 3);
        } else if info.kind == 6 {
            val.c = OrderedDataVec::from_bytes::<O>(&data[info.c_offset as usize..], info.c_num as usize);
            val.d = OrderedDataVec::from_bytes::<O>(&data[info.d_offset as usize..], info.d_num as usize * 3);
            val.e = OrderedDataVec::from_bytes::<O>(&data[info.e_offset as usize..], info.e_num as usize * 3);
        } else if info.kind > 6 {
            warn!("Unknown & Unhandled HkShape type {}", info.kind);
        }
        val
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], info: &HkShapeInfo) {
        if info.kind == 5 {
            self.a.to_bytes::<O>(&mut data[info.a_offset as usize..]);
            self.b.to_bytes::<O>(&mut data[info.b_offset as usize..]);
        } else if info.kind == 6 {
            self.c.to_bytes::<O>(&mut data[info.c_offset as usize..]);
            self.d.to_bytes::<O>(&mut data[info.d_offset as usize..]);
            self.e.to_bytes::<O>(&mut data[info.e_offset as usize..]);
        } else {
            // Unknown/Unhadled ObjD type 
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HkConstraint {
    pub shorts: Vec<u16>,
    pub strings: Vec<(String, u32, u32)>,
    pub string_offsets: Vec<u32>,
    pub vals: Vec<u32>,
    pub keys: Vec<u32>,
    pub keys2: Vec<u32>,
}

impl HkConstraint {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &HkConstraintInfo) -> Self {
        let mut val = Self::default();
        if info.kind != 0 { warn!("Unknown & Unhandled HkConstraint type {}", info.kind); }

        val.shorts = OrderedDataVec::from_bytes::<O>(&data[info.shorts_offset as usize..], info.shorts_num as usize);
        assert!(val.shorts[0] == 0xFFFF);

        val.string_offsets = OrderedDataVec::from_bytes::<O>(&data[info.strings_offset as usize..], info.strings_num as usize);
        for offset_ in val.string_offsets.iter() {
            let (mut offset, val_) = { 
                let vals: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[*offset_ as usize..], 2);

                (vals[0], vals[1]) 
            };
            let start = offset;
            while data[offset as usize] != 0 { offset += 1; }
            let string = String::from_utf8(data[start as usize..offset as usize].to_vec()).unwrap();
            val.strings.push((string, start, val_));
        }
        val.vals = OrderedDataVec::from_bytes::<O>(&data[info.vals_offset as usize..], info.vals_num as usize * 12);
        val.keys = OrderedDataVec::from_bytes::<O>(&data[info.keys_offset as usize..], info.keys_num as usize);
        val.keys2 = OrderedDataVec::from_bytes::<O>(&data[info.keys2_offset as usize..], info.keys2_num as usize * 2);
        val
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], info: &HkConstraintInfo) {
        self.shorts.to_bytes::<O>(&mut data[info.shorts_offset as usize..]);
        self.string_offsets.to_bytes::<O>(&mut data[info.strings_offset as usize..]);
        for (offset_, (string, offset, val)) in zip(&self.string_offsets, &self.strings) {
            offset.to_bytes::<O>(&mut data[*offset_ as usize..]);
            val.to_bytes::<O>(&mut data[*offset_ as usize + u32::size::<O>()..]);
            data[*offset as usize..*offset as usize+string.len()].copy_from_slice(string.as_bytes());
        }
        self.vals.to_bytes::<O>(&mut data[info.vals_offset as usize..]);
        self.keys.to_bytes::<O>(&mut data[info.keys_offset as usize..]);
        self.keys2.to_bytes::<O>(&mut data[info.keys2_offset as usize..]);
    }
}

pub mod animation {
    use super::*;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub enum HkaSplineSkeletalAnimationObj1Types {
        #[default]
        Empty,
        Type1(Vec<u8>),
        Type2(Vec<u16>),
    }
    
    impl HkaSplineSkeletalAnimationObj1Types {
        pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, num: usize, kind: u8) -> Self {
            match kind {
                0 | 2 =>  Self::Type1(OrderedDataVec::from_bytes::<O>(&data[offset..], num)),
                1 | 3 =>  Self::Type2(OrderedDataVec::from_bytes::<O>(&data[offset..], num)),
                _ => panic!("Illegal Type for spline thingy")
            }
        }
    
        pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize) {
            match self {
                Self::Type1(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type2(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                _ => (),
            };
        }
    
        pub fn size<O: ByteOrder + 'static>(&self) -> usize {
            match self {
                Self::Type1(vals) => vals.size::<O>(),
                Self::Type2(vals) => vals.size::<O>(),
                _ => 0,
            }
        }
    }
    
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct HkaSplineSkeletalAnimationObj1 {
        pub nbytes: usize,
        pub s1: u16,
        pub s2: u8,
        pub data: Vec<u8>,
        pub vals_a: Vec<f32>,
        pub vals: HkaSplineSkeletalAnimationObj1Types,
    }
    
    impl HkaSplineSkeletalAnimationObj1 {
        // const ITEM_SIZES: [usize; 4] = [1,2,1,2];
        const COUNTS: [usize; 8] = [0,1,1,2,1,2,2,3];
    
        pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset_: usize, flags: u8, kind: u8) -> Self {
            let mut val = Self::default();
            let mut offset = offset_;
            if flags != 0 {        
                if flags & 0xf0 == 0 {
                    val.s1 = 0;
                    val.s2 = 0;
                } else {
                    val.s1 = OrderedData::from_bytes::<O>(&data[offset..]);
                    offset += u16::size::<O>();
                    val.s2 = OrderedData::from_bytes::<O>(&data[offset..]);
                    offset += u8::size::<O>();
                    val.data = OrderedDataVec::from_bytes::<O>(&data[offset..], val.s1 as usize + val.s2 as usize + 2);
                    offset += val.data.size::<O>();
                }
                offset = (offset + 3) & 0xfffffffc;
    
                let num = Self::COUNTS[(flags & 7) as usize] + 2 * Self::COUNTS[(((flags >> 4) & !flags) & 7) as usize];
                val.vals_a = OrderedDataVec::from_bytes::<O>(&data[offset..], num);
                offset += val.vals_a.size::<O>();
    
                if flags & 0xf0 == 0 {
                    offset = (offset + 3) & 0xfffffffc;
                    val.nbytes = offset - offset_;
                    return val;
                }
    
                offset = (offset + 1) & 0xfffffffe;
    
                let num = Self::COUNTS[((flags >> 4) & 7) as usize] * (val.s1 as usize + 1);
                val.vals = HkaSplineSkeletalAnimationObj1Types::from_data::<O>(data, offset, num, kind);
                offset += val.vals.size::<O>();
            }
            offset = (offset + 3) & 0xfffffffc;
            val.nbytes = offset - offset_;
            val
        }
    
        pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize, flags: u8) {
            let mut offset = offset;
            if flags == 0 { return; }
            if flags & 0xf0 != 0 {
                self.s1.to_bytes::<O>(&mut data[offset..]);
                offset += u16::size::<O>();
                self.s2.to_bytes::<O>(&mut data[offset..]);
                offset += u8::size::<O>();
                self.data.to_bytes::<O>(&mut data[offset..]);
                offset += self.data.size::<O>();
            }
            offset = (offset + 3) & 0xfffffffc;
    
            self.vals_a.to_bytes::<O>(&mut data[offset..]);
            offset += self.vals_a.size::<O>();
    
            if flags & 0xf0 == 0 { return; }
    
            offset = (offset + 3) & 0xfffffffc;
            self.vals.into_data::<O>(data, offset);
        }
    }
    
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct HkaSplineSkeletalAnimationObj2Type1(u32);
    
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    // should be (u8, u8, u8, u16) but for xbox conv it is (u8, u8, u8, u8, u8)
    pub struct HkaSplineSkeletalAnimationObj2Type2(u8, u8, u8, u8, u8);
    
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct HkaSplineSkeletalAnimationObj2Type3(u16, u16, u16);
    
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct HkaSplineSkeletalAnimationObj2Type4(u8, u8, u8);
    
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct HkaSplineSkeletalAnimationObj2Type5(u8, u8);
    
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct HkaSplineSkeletalAnimationObj2Type6(u32, u32, u32, u32);
    
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub enum HkaSplineSkeletalAnimationObj2Types{
        #[default]
        Empty,
        Type1(Vec<HkaSplineSkeletalAnimationObj2Type1>),
        Type2(Vec<HkaSplineSkeletalAnimationObj2Type2>),
        Type3(Vec<HkaSplineSkeletalAnimationObj2Type3>),
        Type4(Vec<HkaSplineSkeletalAnimationObj2Type4>),
        Type5(Vec<HkaSplineSkeletalAnimationObj2Type5>),
        Type6(Vec<HkaSplineSkeletalAnimationObj2Type6>),
    }
    
    impl HkaSplineSkeletalAnimationObj2Types {
        pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, num: usize, kind: u8) -> Self {
            match kind {
                0 =>  Self::Type1(OrderedDataVec::from_bytes::<O>(&data[offset..], num)),
                1 =>  Self::Type2(OrderedDataVec::from_bytes::<O>(&data[offset..], num)),
                2 =>  Self::Type3(OrderedDataVec::from_bytes::<O>(&data[offset..], num)),
                3 =>  Self::Type4(OrderedDataVec::from_bytes::<O>(&data[offset..], num)),
                4 =>  Self::Type5(OrderedDataVec::from_bytes::<O>(&data[offset..], num)),
                5 =>  Self::Type6(OrderedDataVec::from_bytes::<O>(&data[offset..], num)),
                _ => panic!("Illegal Type for spline thingy")
            }
        }
    
        pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize) {
            match self {
                Self::Type1(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type2(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type3(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type4(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type5(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type6(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                _ => (),
            };
        }
    
        pub fn size<O: ByteOrder + 'static>(&self) -> usize {
            match self {
                Self::Type1(vals) => vals.size::<O>(),
                Self::Type2(vals) => vals.size::<O>(),
                Self::Type3(vals) => vals.size::<O>(),
                Self::Type4(vals) => vals.size::<O>(),
                Self::Type5(vals) => vals.size::<O>(),
                Self::Type6(vals) => vals.size::<O>(),
                _ => 0,
            }
        }
    }
    
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct HkaSplineSkeletalAnimationObj2 {
        pub nbytes: usize,
        pub align: u32,
        pub s1: u16,
        pub s2: u8,
        pub data: Vec<u8>,
        pub vals: HkaSplineSkeletalAnimationObj2Types,
    }
    
    impl HkaSplineSkeletalAnimationObj2 {
        const ALIGNMENTS: [u32; 6] = [4, 1, 2, 1, 2, 4];
    
        pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset_: usize, flags: u8, kind: u8) -> Self {
            let mut val = Self::default();
            let mut offset = offset_;
            if flags != 0 {
                val.align = Self::ALIGNMENTS[kind as usize];
                if flags & 0xf0 == 0 {
                    val.s1 = 0;
                    val.s2 = 0;
                } else {
                    val.s1 = OrderedData::from_bytes::<O>(&data[offset..]);
                    offset += u16::size::<O>();
                    val.s2 = OrderedData::from_bytes::<O>(&data[offset..]);
                    offset += u8::size::<O>();
                    val.data = OrderedDataVec::from_bytes::<O>(&data[offset..], val.s1 as usize + val.s2 as usize + 2);
                    offset += val.data.size::<O>();
                }
    
                offset = ((offset as u32 + val.align - 1) & !(val.align - 1)) as usize;
                val.vals = HkaSplineSkeletalAnimationObj2Types::from_data::<O>(data, offset, val.s1 as usize + 1, kind);
                offset += val.vals.size::<O>();   
            }
            offset = (offset + 3) & 0xfffffffc;
            val.nbytes = offset - offset_;
            val
        }
    
        pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize, flags: u8) {
            let mut offset = offset;
            if flags == 0 { return; }
            if flags & 0xf0 != 0 {
                self.s1.to_bytes::<O>(&mut data[offset..]);
                offset += u16::size::<O>();
                self.s2.to_bytes::<O>(&mut data[offset..]);
                offset += u8::size::<O>();
                self.data.to_bytes::<O>(&mut data[offset..]);
                offset += self.data.size::<O>();
            }
    
            offset = ((offset as u32 + self.align - 1) & !(self.align - 1)) as usize;
            self.vals.into_data::<O>(data, offset);
        }
    }
    
    
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct HkaSplineSkeletalAnimationFlags{
        pub f: u8, 
        pub a: u8, 
        pub b: u8, 
        pub c: u8,
    }
    
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct HkaSplineSkeletalAnimation {
        pub block_starts: Vec<u32>,
        pub block_starts2: Vec<u32>,
        pub obj_c3: Vec<u32>,
        pub obj_c4: Vec<u32>,
        pub flags: Vec<Vec<HkaSplineSkeletalAnimationFlags>>,
        pub flags2: Vec<Vec<u8>>,
        pub vals_a: Vec<Vec<HkaSplineSkeletalAnimationObj1>>,
        pub vals_b: Vec<Vec<HkaSplineSkeletalAnimationObj2>>,
        pub vals_c: Vec<Vec<HkaSplineSkeletalAnimationObj1>>,
        pub vals_d: Vec<Vec<HkaSplineSkeletalAnimationObj1>>,
    }
    
    impl HkaSplineSkeletalAnimation {
        pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, info: &AnimationInfo) -> Self {
            let mut val = Self::default();
            val.block_starts = OrderedDataVec::from_bytes::<O>(&data[offset + info.block_starts_offset as usize..], info.block_starts_num as usize);
            val.block_starts2 = OrderedDataVec::from_bytes::<O>(&data[offset + info.block_starts2_offset as usize..], info.block_starts2_num as usize);
            val.obj_c3 = OrderedDataVec::from_bytes::<O>(&data[offset + info.obj_c3_offset as usize..], info.obj_c3_num as usize);
            val.obj_c4 = OrderedDataVec::from_bytes::<O>(&data[offset + info.obj_c4_offset as usize..], info.obj_c4_num as usize);
            for (start, start2) in zip(&val.block_starts,&val.block_starts2) {
                let off = offset + (start + info.block_offset) as usize;
                let flags: Vec<HkaSplineSkeletalAnimationFlags> = OrderedDataVec::from_bytes::<O>(&data[off..], info.keys_num as usize);
                let flags2: Vec<u8> = OrderedDataVec::from_bytes::<O>(&data[off + flags.size::<O>()..], info.keys2_num as usize);
                let mut off = offset + (info.block_offset + start + info.data_offset) as usize;
                let mut vals_a = Vec::with_capacity(flags.len());
                let mut vals_b = Vec::with_capacity(flags.len());
                let mut vals_c = Vec::with_capacity(flags.len());
                let mut vals_d= Vec::with_capacity(flags2.len());
                for flag in &flags {
                    let a = HkaSplineSkeletalAnimationObj1::from_data::<O>(data, off, flag.a, flag.f & 3);
                    off += a.nbytes;
                    let b = HkaSplineSkeletalAnimationObj2::from_data::<O>(data, off, flag.b, (flag.f >> 2) & 0xf);
                    off += b.nbytes;
                    let c = HkaSplineSkeletalAnimationObj1::from_data::<O>(data, off, flag.c, (flag.f >> 6) & 3);
                    off += c.nbytes;
                    vals_a.push(a);
                    vals_b.push(b);
                    vals_c.push(c);
                }
                off = offset + (info.block_offset + start + start2) as usize;
                for flag in &flags2 {
                    let d: HkaSplineSkeletalAnimationObj1 = HkaSplineSkeletalAnimationObj1::from_data::<O>(data, off, flag & 0xf9, (flag >> 1) & 3);
                    off += d.nbytes;
                    vals_d.push(d);
                }
                val.flags.push(flags);
                val.flags2.push(flags2);
                val.vals_a.push(vals_a);
                val.vals_b.push(vals_b);
                val.vals_c.push(vals_c);
                val.vals_d.push(vals_d);
            }
            val
        }
    
        pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize, info: &AnimationInfo) {
            self.block_starts.to_bytes::<O>(&mut data[offset + info.block_starts_offset as usize..]);
            self.block_starts2.to_bytes::<O>(&mut data[offset + info.block_starts2_offset as usize..]);
            self.obj_c3.to_bytes::<O>(&mut data[offset + info.obj_c3_offset as usize..]);
            self.obj_c4.to_bytes::<O>(&mut data[offset + info.obj_c4_offset as usize..]);
            for (((start, start2), (flags, flags2)), ((vals_a, vals_b), (vals_c, vals_d))) in zip(zip(zip(&self.block_starts, &self.block_starts2), zip(&self.flags, &self.flags2)), zip(zip(&self.vals_a, &self.vals_b), zip(&self.vals_c, &self.vals_d))) {
                flags.to_bytes::<O>(&mut data[offset + (start + info.block_offset) as usize..]);
                let mut off = offset + (info.block_offset + start + info.data_offset) as usize;
                for ((flag, a), (b, c)) in zip(zip(flags, vals_a), zip(vals_b, vals_c)) {
                    a.into_data::<O>(data, off, flag.a);
                    off += a.nbytes;
                    b.into_data::<O>(data, off, flag.b);
                    off += b.nbytes;
                    c.into_data::<O>(data, off, flag.c);
                    off += c.nbytes;
                }
                off = offset + (info.block_offset + start + start2) as usize;
                for (flag, d) in zip(flags2, vals_d) {
                    d.into_data::<O>(data, off, flag & 0xf9);
                    off += d.nbytes;
                }
            }
        }
    }
    
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct Obj5Header {
        pub obj_a_num: u32,
        pub obj_a_offset: u32,
        pub obj_b_num: u32,
        pub obj_b_offset: u32,
    }

    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct Obj3 {
        pub t: f32,
        pub event: Crc,
        pub dat_2: Crc,
        pub dat_3: Crc,
        pub dat_4: Crc,
        pub dat_5: Crc,
        pub dat_6: Crc,
        pub dat_7: Crc,
        pub dat_8: Crc,
        pub dat_9: Crc,
        pub dat_10: Crc,
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub obj1: HashMap<usize, Vec<u32>>,
    pub obj2: HashMap<usize, Vec<u32>>,
    pub obj3: HashMap<usize, Vec<animation::Obj3>>,
    pub keys: HashMap<usize, Vec<u32>>,
    pub obj5_header: HashMap<usize, animation::Obj5Header>,
    pub obj5_a: HashMap<usize, Vec<u32>>,
    pub obj5_b: HashMap<usize, Vec<u32>>,
    pub obj_c: HashMap<usize, animation::HkaSplineSkeletalAnimation>,
}

impl Animation {
    pub fn unpack_from_block<O: ByteOrder + 'static>(&mut self, data: &[u8], offset: usize, index: usize, info: &AnimationInfo) {
        self.obj1.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.obj1_offset as usize..], info.obj1_num as usize * 2));
        self.obj2.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.obj2_offset as usize..], info.obj2_num as usize * 4));
        self.obj3.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.obj3_offset as usize..], info.obj3_num as usize));
        self.keys.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.keys_offset as usize..], (info.keys_num + info.obj1_num) as usize));
        if info.obj5_offset != 0 {
            let obj5_header: animation::Obj5Header = OrderedData::from_bytes::<O>(&data[offset + info.obj5_offset as usize..]);
            self.obj5_a.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + obj5_header.obj_a_offset as usize..], obj5_header.obj_a_num as usize * 7));
            self.obj5_b.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + obj5_header.obj_b_offset as usize..], obj5_header.obj_b_num as usize * 7));
            self.obj5_header.insert(index, obj5_header);
        }
        if info.kind == 3 {
            self.obj_c.insert(index, animation::HkaSplineSkeletalAnimation::from_data::<O>(data, offset, info));
        } else if info.kind < 3 {
            warn!("Unhandled animation type {}", info.kind);
        } else {
            warn!("Unknown animation type {}", info.kind);
        }
    }

    pub fn pack_into_block<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize, index: usize, info: &AnimationInfo) {
        self.obj1.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.obj1_offset as usize..]);
        self.obj2.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.obj2_offset as usize..]);
        self.obj3.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.obj3_offset as usize..]);
        self.keys.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.keys_offset as usize..]);
        if info.obj5_offset != 0 {
            let obj5_header = self.obj5_header.get(&index).unwrap();
            obj5_header.to_bytes::<O>(&mut data[offset + info.obj5_offset as usize..]);
            self.obj5_a.get(&index).unwrap().to_bytes::<O>(&mut data[offset + obj5_header.obj_a_offset as usize..]);
            self.obj5_b.get(&index).unwrap().to_bytes::<O>(&mut data[offset + obj5_header.obj_b_offset as usize..]);
        }
        if info.kind == 3 {
            self.obj_c.get(&index).unwrap().into_data::<O>(data, offset, info);
        }
    }

    pub fn unpack_block<O: ByteOrder + 'static>(anims: &mut [Self], infos: &[AnimationInfo], data: & [u8], offset: usize, index: usize) {
        let mut offset = offset;
        for (anim, info) in zip(anims, infos) {
            let gamemodemask = 1i32 << index;
            if gamemodemask & info.gamemodemask != 0 {
                anim.unpack_from_block::<O>(data, offset, index, info);
                offset += info.size as usize;
            }
        }
    }

    pub fn pack_block<O: ByteOrder + 'static>(anims: & [Self], infos: &[AnimationInfo], data: &mut [u8], offset: usize, index: usize) {
        let mut offset = offset;
        for (anim, info) in zip(anims, infos) {
            let gamemodemask = 1i32 << index;
            if gamemodemask & info.gamemodemask != 0 {
                anim.pack_into_block::<O>(data, offset, index, info);
                offset += info.size as usize;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VertexUsage {
    Position,
    Normal,
    Tangent,
    BiNormal,
    BlendWeight,
    BlendIndices(usize),
    TextureCoord(usize),
    PSize,
    Pad,
}

impl Display for VertexUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Position => write!(f, "Position"),
            Self::Normal => write!(f, "Normal"),
            Self::Tangent => write!(f, "Tangent"),
            Self::BiNormal => write!(f, "BiNormal"),
            Self::BlendWeight => write!(f, "BlendWeight"),
            Self::BlendIndices(i) => write!(f, "BlendIndices({})", i),
            Self::TextureCoord(i) => write!(f, "TextureCoord({})", i),
            Self::PSize => write!(f, "PSize"),
            Self::Pad => write!(f, "Pad"),
        }
    }
}


#[derive(Debug)]
pub struct VertexUsageParseError;
impl Display for VertexUsageParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VertexUsageParseError")
    }
}

impl From<ParseIntError> for VertexUsageParseError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}

impl FromStr for VertexUsage {
    type Err = VertexUsageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Position" => Ok(Self::Position),
            "Normal" => Ok(Self::Normal),
            "Tangent" => Ok(Self::Tangent),
            "BiNormal" => Ok(Self::BiNormal),
            "BlendWeight" => Ok(Self::BlendWeight),
            "PSize" => Ok(Self::PSize),
            "Pad" => Ok(Self::Pad),
            s => {
                if s.starts_with("BlendIndices(") {
                    Ok(s[13..].split(')').next().unwrap().parse::<usize>().map(|i| Self::BlendIndices(i))?)
                } else if s.starts_with("TextureCoord(") {
                    Ok(s[13..].split(')').next().unwrap().parse::<usize>().map(|i| Self::TextureCoord(i))?)
                } else {
                    Err(VertexUsageParseError)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VertexTypes {
    Vector2(Vec<f32>,Vec<f32>),
    Vector3(Vec<f32>,Vec<f32>,Vec<f32>),
    Vector4(Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>),
    Unorm4x8 (Vec<u32>),
    Pad (Vec<u32>),
    None,
}

impl VertexTypes {
    pub fn new(format: u32) -> Self {
        match format {
            BaseTypes::INT_KEY => Self::Pad(vec![]),
            BaseTypes::COLOR_KEY => Self::Unorm4x8(vec![]),
            BaseTypes::VECTOR2_KEY => Self::Vector2(vec![], vec![]),
            BaseTypes::VECTOR3_KEY => Self::Vector3(vec![], vec![], vec![]),
            BaseTypes::VECTOR4_KEY => Self::Vector4(vec![], vec![], vec![], vec![]),
            _ => Self::None
        }
    }

    pub fn get(&self, i: usize) -> BaseTypes {
        match self {
            Self::Pad(vals) => BaseTypes::Color(vals[i]),
            Self::Unorm4x8(vals) => BaseTypes::Color(vals[i]),
            Self::Vector2(x, y) => BaseTypes::Vector2(Vector2 {x: x[i], y: y[i]}),
            Self::Vector3(x, y, z) => BaseTypes::Vector3(Vector3 {x: x[i], y: y[i], z: z[i]}),
            Self::Vector4(x, y, z, w) => BaseTypes::Vector4(Vector4 {x: x[i], y: y[i], z: z[i], w: w[i]}),
            Self::None => BaseTypes::Int(0)
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Pad(vals) => vals.len(),
            Self::Unorm4x8(vals) => vals.len(),
            Self::Vector2(x, y) => x.len().min(y.len()),
            Self::Vector3(x, y, z) => x.len().min(y.len()).min(z.len()),
            Self::Vector4(x, y, z, w) => x.len().min(y.len()).min(z.len()).min(w.len()),
            Self::None => 0
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Pad(..) => 4,
            Self::Unorm4x8(..) => 4,
            Self::Vector2(..) => 8,
            Self::Vector3(..) => 12,
            Self::Vector4(..) => 16,
            Self::None => 0
        }
    }

    pub fn push(&mut self, val: BaseTypes) {
        match self {
            Self::Pad(vals) => if let BaseTypes::Color(val) = val {
                vals.push(val);
            },
            Self::Unorm4x8(vals) => if let BaseTypes::Color(val) = val {
                vals.push(val);
            },
            Self::Vector2(x, y)=> if let BaseTypes::Vector2(val) = val {
                x.push(val.x); y.push(val.y);
            },
            Self::Vector3(x, y, z)=> if let BaseTypes::Vector3(val) = val {
                x.push(val.x); y.push(val.y); z.push(val.z);
            },
            Self::Vector4(x, y, z, w) => if let BaseTypes::Vector4(val) = val {
                x.push(val.x); y.push(val.y); z.push(val.z); w.push(val.w);
            },
            Self::None => ()
        }
    }
}

fn get_vertex_format<O: ByteOrder + 'static>(fmt1: u32, fmt2: u32) -> (Vec<(u32, VertexUsage)>, usize) {
    let mut fmt = Vec::new();
    let mut s = 0;
    if fmt2 == 0 {
        let b1: bool = (fmt1 & 0x40000) != 0;
        if fmt1 & 1 != 0 {
            fmt.push((if b1 {BaseTypes::VECTOR4_KEY} else {BaseTypes::VECTOR3_KEY}, VertexUsage::Position));
            s += if b1 {16} else {12};
        }
        if (fmt1 & 0x400) != 0 {
            fmt.push((BaseTypes::COLOR_KEY, VertexUsage::Tangent));
            s += 4;
        }
        if (fmt1 & 0x800) != 0 {
            fmt.push((BaseTypes::COLOR_KEY, VertexUsage::BiNormal));
            s += 4;
        }
        if (fmt1 & 2) != 0 {
            if b1 {
                for _ in (0..(((s + 15) & 0xFFFF0) - s)).step_by(4) {
                    fmt.push((BaseTypes::COLOR_KEY, VertexUsage::Pad));
                    s += 4;
                }
                fmt.push((BaseTypes::VECTOR4_KEY, VertexUsage::BlendWeight));
                s += 16;
            } else {
                fmt.push((BaseTypes::COLOR_KEY, VertexUsage::BlendWeight));
                s += 4;
            }
        }
        if fmt1 & 0x100 != 0 {
            fmt.push((BaseTypes::COLOR_KEY, VertexUsage::TextureCoord(0)));
            s += 4;
        }
        if fmt1 & 0x200 != 0 {
            fmt.push((BaseTypes::COLOR_KEY, VertexUsage::TextureCoord(1)));
            s += 4;
        }
        for i in 0..((fmt1 >> 2) & 0xF) {
            fmt.push((BaseTypes::VECTOR2_KEY, VertexUsage::BlendIndices(i as usize)));
            s += 8;
        }
        if fmt1 & 0x40 != 0 {
            if b1 {
                for _ in (0..(((s + 15) & 0xFFFF0) - s)).step_by(4) {
                    fmt.push((BaseTypes::COLOR_KEY, VertexUsage::Pad));
                    s += 4;
                }
                fmt.push((BaseTypes::VECTOR4_KEY, VertexUsage::Normal));
                s += 16;
            } else {
                fmt.push((BaseTypes::COLOR_KEY, VertexUsage::Normal));
                s += 4;
            }
        }
        if fmt1 & 0x80 != 0 {
            fmt.push((BaseTypes::VECTOR3_KEY, VertexUsage::PSize));
            s += 12;
        }
        if b1 {
            for _ in (0..(((s + 15) & 0xFFFF0) - s)).step_by(4) {
                fmt.push((BaseTypes::COLOR_KEY, VertexUsage::Pad));
                s += 4;
            }
        }
    } else {
        if fmt1 & 1 != 0 {
            fmt.push((BaseTypes::VECTOR3_KEY, VertexUsage::Position));
            s += 12;
        }
        if fmt1 & 0x400 != 0 {
            fmt.push((BaseTypes::COLOR_KEY, VertexUsage::Tangent));
            s += 4;
        }
        if fmt1 & 0x800 != 0 {
            fmt.push((BaseTypes::COLOR_KEY, VertexUsage::BiNormal));
            s += 4;
        }
        if fmt1 & 2 != 0{
            fmt.push((BaseTypes::COLOR_KEY, VertexUsage::BlendWeight));
            s += 4;
        }
        if fmt1 & 0x100 != 0 {
            fmt.push((BaseTypes::COLOR_KEY, VertexUsage::TextureCoord(0)));
            s += 4;
        }
        if fmt1 & 0x200 != 0 {
            fmt.push((BaseTypes::COLOR_KEY, VertexUsage::TextureCoord(1)));
            s += 4;
        }
        let n = (fmt1 >> 2) & 0xf;
        if n <= 2 {
            for i in 0..n {
                fmt.push((BaseTypes::VECTOR2_KEY, VertexUsage::BlendIndices(i as usize)));
                s += 8;
            }
        }
        if fmt1 & 0x40 != 0 {
            fmt.push((BaseTypes::COLOR_KEY, VertexUsage::Normal));
            s += 4;
        }
        if fmt1 & 0x80 != 0 {
            fmt.push((BaseTypes::VECTOR3_KEY, VertexUsage::PSize));
            s += 12;
        }
    }
    (fmt, s)
}

use serde_with::serde_as;
#[serde_as]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VertexBuffer {
    #[serde_as(as = "serde_with::Map<serde_with::DisplayFromStr, _>")]
    pub vals: Vec<(VertexUsage, VertexTypes)>
}

impl VertexBuffer {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &mut VBuffInfo, formats: &mut HashMap<(u32, u32), (Vec<(u32, VertexUsage)>, usize)>) -> Self {
        let (fmt, size) = formats.entry((info.fmt1, info.fmt2)).or_insert_with(|| {
            get_vertex_format::<O>(info.fmt1, info.fmt2)
        });
        assert!(info.size as usize % *size == 0);
        let n = info.size as usize / *size;
        let mut offset = info.offset as usize;
        let mut vals = fmt.iter().map(|(t, u)| (u.clone(), VertexTypes::new(*t))).collect::<Vec<_>>();
        for _ in 0..n {
            // let mut val = Vec::with_capacity(fmt.len());
            for (kind, (_, val)) in zip(fmt.iter().map(|(x, _)| x), &mut vals) {
                let v = BaseTypes::from_data::<O>(&data[offset..], *kind);
                offset += v.size::<O>();
                val.push(v);
            }
        }
        if TypeId::of::<O>() == TypeId::of::<BE>() {
            if (info.fmt1 & 0x80000 != 0) & (info.fmt1 & 0x400 == 0) {
                info.fmt1 |= 0x400;
                let (fmt, _) = formats.entry((info.fmt1, info.fmt2)).or_insert_with(|| {
                    get_vertex_format::<O>(info.fmt1, info.fmt2)
                });
                let mut vals_new = fmt.iter().map(|(t, u)| (u.clone(), VertexTypes::new(*t))).collect::<Vec<_>>();
                let mut binorm = Vec::with_capacity(n);
                let mut tan = Vec::with_capacity(n);
                for (usage, val) in &mut vals {
                    if *usage == VertexUsage::BiNormal {
                        match val {
                            VertexTypes::Unorm4x8(val) => {
                                for v in val {
                                    let a = *v & 0xFF;
                                    let b = (*v >> 8) & 0xFF;
                                    let c = (*v >> 16) & 0xFF;
                                    let d = (*v >> 24) & 0xFF;
                                    // println!("{}, {:?}",*v, (a,b,c,d));
                                    binorm.push((d << 24) | (d << 16) | (d << 8) | c);
                                    tan.push((a << 16) | (b << 8));
                                }
                            },
                            _ => ()
                        }
                    }
                }
                for (usage, val) in &mut vals_new {
                    if *usage == VertexUsage::BiNormal {
                        *val = VertexTypes::Unorm4x8(binorm.clone());
                    } else if *usage == VertexUsage::Tangent {
                        *val = VertexTypes::Unorm4x8(tan.clone());
                    } else {
                        for (usage2, val2) in &vals {
                            if *usage == *usage2 {
                                *val = val2.clone();
                            }
                        }
                    }
                }
                vals = vals_new;
            }
            for (usage, val) in &mut vals {
                if *usage == VertexUsage::BlendWeight {
                    match val {
                        VertexTypes::Vector4(x, y, z, ..) => {
                            x.iter_mut().for_each(|x| *x = *x/2.0 + 0.5);
                            y.iter_mut().for_each(|x| *x = *x/2.0 + 0.5);
                            z.iter_mut().for_each(|x| *x = *x/2.0 + 0.5);
                        },
                        VertexTypes::Unorm4x8(v) => v.iter_mut().for_each(|val| {
                            let z_ = ((*val) & 0x3FF) ^ 0x200;
                            let y_ = (((*val) >> 10) & 0x3FF) ^ 0x200;
                            let x_ = (((*val) >> 20) & 0x3FF) ^ 0x200;
                            // let x: u32 = ((x as f32)/4.0).round().max(0.0).min(255.0) as u32;
                            // let y: u32 = ((y as f32)/4.0).round().max(0.0).min(255.0) as u32;
                            // let z: u32 = ((z as f32)/4.0).round().max(0.0).min(255.0) as u32;
                            // let x: u32 = (x_ as f64 - 4.0).div(4.0).round_ties_even().clamp(0.0, 255.0) as u32;
                            // let y: u32 = (y_ as f64 - 4.0).div(4.0).round_ties_even().clamp(0.0, 255.0) as u32;
                            // let z: u32 = (z_ as f64 - 4.0).div(4.0).round_ties_even().clamp(0.0, 255.0) as u32;
                            let x: u32 = (x_ as f32 - 4.0f32).div(4.0f32).round_ties_even().clamp(0.0, 255.0) as u32;
                            let y: u32 = (y_ as f32 - 4.0f32).div(4.0f32).round_ties_even().clamp(0.0, 255.0) as u32;
                            let z: u32 = (z_ as f32 - 4.0f32).div(4.0f32).round_ties_even().clamp(0.0, 255.0) as u32;
                            // println!("{:?}, {:?}", (
                            //     (x_ as f32 - 4.0).div(4.0),
                            //     (y_ as f32 - 4.0).div(4.0),
                            //     (z_ as f32 - 4.0).div(4.0)
                            // ), (x,y,z));

                            // let z = *val & 0x3FF;
                            // let y = (*val >> 10) & 0x3FF;
                            // let x = (*val >> 20) & 0x3FF;
                            // let x = (((if x & 0x200 != 0 { x - 512 } else { x + 512 }) as f32 - 4.0)/4.0).round().max(0.1).min(255.0) as u32;
                            // let y = (((if y & 0x200 != 0 { y - 512 } else { y + 512 }) as f32 - 4.0)/4.0).round().max(0.1).min(255.0) as u32;
                            // let z = (((if z & 0x200 != 0 { z - 512 } else { z + 512 }) as f32 - 4.0)/4.0).round().max(0.1).min(255.0) as u32;
                            // let x = if x & 0x200 != 0 { (x as f32 - 512.0) / 512.0 * 127.0 } else { x as f32 / 511.0 * 128.0  + 127.0 } as u32;
                            // let y = if y & 0x200 != 0 { (y as f32 - 512.0) / 512.0 * 127.0 } else { y as f32 / 511.0 * 128.0  + 127.0 } as u32;
                            // let z = if z & 0x200 != 0 { (z as f32 - 512.0) / 512.0 * 127.0 } else { z as f32 / 511.0 * 128.0  + 127.0 } as u32;
                            *val = (127 << 24) | (z << 16) | (y << 8) | x;
                        }),
                        _ => panic!("Unexpected vertex type for weight")
                    }
                }
            }
            // vals.push(val);
        }
        Self { vals }
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut[u8], info: &VBuffInfo) {
        let mut offset = info.offset as usize;
        let mut off_ = 0;
        let i = self.vals.iter().map(|(_, x)| x.len()).min().unwrap();
        for (_, val) in &self.vals {
            assert!(val.len() == i);
        }
        for i in 0..i {
            for (_, val) in &self.vals {
                let v = val.get(i);
                v.into_data::<O>(&mut data[offset..], &mut off_);
                offset += v.size::<O>();
            }
        }
        // for val in &self.vals {
        //     for v in val {
        //         v.into_data::<O>(&mut data[offset..], &mut off_);
        //         offset += v.size::<O>();
        //     }
        // }
    }
    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        // self.vals.iter().flat_map(|x| x.iter().flat_map(|x| x.dump_bytes::<O>())).collect()
        let i = self.vals.iter().map(|(_, x)| x.len()).min().unwrap();
        (0..i).flat_map(|i| self.vals.iter().flat_map(move |(_, val)| val.get(i).dump_bytes::<O>())).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexBuffer {
    U16 { vals: Vec<u16> },
    U32 { vals: Vec<u32> },
}

impl IndexBuffer {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &IBuffInfo) -> Self {
        let size = match info.format {
            0x10 => u16::size::<O>(),
            _ => u32::size::<O>(),
        };
        assert!(info.size as usize % size == 0);
        let n = info.size as usize / size;
        match info.format {
            0x10 => Self::U16 { vals: OrderedDataVec::from_bytes::<O>(&data[info.offset as usize..], n) },
            _ => Self::U32 { vals: OrderedDataVec::from_bytes::<O>(&data[info.offset as usize..], n) },
        }
    }
    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8]) {
        match self {
            Self::U16 { vals } => vals.to_bytes::<O>(data),
            Self::U32 { vals } => vals.to_bytes::<O>(data)
        };
    }
    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        match self {
            Self::U16 { vals } => vals.dump_bytes::<O>(),
            Self::U32 { vals } => vals.dump_bytes::<O>()
        }
    }

}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Illumination {
    pub vals: Vec<u32>
}

impl Illumination {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &IlluminationInfo) -> Self {
        Self { vals: OrderedDataVec::from_bytes::<O>(&data[info.offset as usize..], info.num as usize) }
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], info: &IlluminationInfo) {
        self.vals.to_bytes::<O>(&mut data[info.offset as usize..]);
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Foliage {
    pub vals: Vec<u32>
}

impl Foliage {
    // holds vertex data of some sort
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &FoliageInfo) -> Self {
        let n = (info.s1b - info.s1a) * (info.s2b - info.s2a) * 2;
        Self { vals: OrderedDataVec::from_bytes::<O>(&data[info.offset as usize..], n as usize) }
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], info: &FoliageInfo) {
        self.vals.to_bytes::<O>(&mut data[info.offset as usize..]);
    }
}
