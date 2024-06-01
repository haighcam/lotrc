use std::{collections::HashMap, iter::zip, any::TypeId};
use log::warn;
use zerocopy::{ByteOrder, BE};
use serde::{Serialize, Deserialize};
use crate::types::Crc;

use super::types::BaseTypes;

use lotrc_rs_proc::OrderedData;
use super::types::{OrderedData, Vector4, Matrix4x4, OrderedDataVec};

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
    pub mesh_info_num: u32,  // 1
    pub buffer_info_num: u32,  // 2
    pub mat1_num: u32, 
    pub mat2_num: u32, 
    pub mat3_num: u32, 
    pub mat4_num: u32, 
    pub mat_extra_num: u32, 
    pub unk_51: u32, 
    pub shape_info_num: u32, 
    pub hk_shape_info_num: u32,  // d
    pub hk_constraint_data_num: u32,  // e
    pub vbuff_info_num: u32,  // f
    pub ibuff_info_num: u32,  // g
    pub texture_info_num: u32,  // 7
    pub animation_info_num: u32,  // 8
    pub hk_constraint_info_num: u32,  // 9
    pub gameobj_block_info_num: u32,  // 10
    pub pfield_info_num: u32,  // 12
    pub gfx_block_info_num: u32, 
    pub animation_block_info_num: u32, 
    pub foliage_info_num: u32, 
    pub obj14_info_num: u32, 
    pub unk_66: u32, 
    pub obja_offset: u32,  // 24 bytes
    pub obj0_offset: u32, 
    pub mesh_info_offset: u32,  //256 bytes, max loaded is 0x400
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
    pub texture_info_offset: u32,  //0x12 bytes, max loaded is 0x800, related to MgSurfaceWin32
    pub animation_info_offset: u32, 
    pub hk_constraint_info_offset: u32, 
    pub gameobj_block_info_offset: u32, 
    pub pfield_info_offset: u32, 
    pub gfx_block_info_offset: u32,  // 0xc bytes, max loaded is 0x40
    pub animation_block_info_offset: u32,  // 36 bytes
    pub foliage_info_offset: u32, 
    pub obj14_info_offset: u32, 
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
    pub key: u32, //<I',
    #[ordered_data(LE)]
    pub unk_1: u32, //<I',
    #[ordered_data(LE)]
    pub size: u32, //<I',
    #[ordered_data(LE)]
    pub size_comp: u32, //<I',
    #[ordered_data(LE)]
    pub unk_4: u32, //<I',
    #[ordered_data(LE)]
    pub kind: u32, //<I',
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Obj0{
    #[ordered_data(LE)]
    pub unk_0: u32, //<I',
    #[ordered_data(LE)]
    pub key: u32, //<I',
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct MeshInfo {
    pub key: Crc,
    pub block_flag: u32,
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
    pub valCs_offset: u32, // ints (c & 0x3fffffff is an index into the obj2s referenced by this object)
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
    pub valCs_num: u32,
    pub mat_num: u32,
    pub keys_offset: u32, // ints
    pub indices_offset: u32,
    pub matrices_offset: u32, // 16 ints (matrix?) for keys_num
    pub keys_num: u32,
    pub valGs_offset: u32,
    pub valGs_num: u32,
    pub valIs_offset: u32,
    pub vbuff_offset: u32,
    pub vbuff_num: u32,
    pub ibuff_offset: u32,
    pub ibuff_num: u32,
    pub valDs_offset: u32, // f_num * 8 ints
    pub unk_46: u32,
    pub unk_47: u32,
    pub valJs_num: u32,
    pub valJs_offset: u32,
    pub block_offset: u32,
    pub valKs_offset: u32, // not sure on the size, seems to be 36 ints
    pub asset_key: u32, // data in bin that is vertex & index buffer values
    pub asset_type: u32,
    pub unk_54: u32,
    pub unk_55: u32,
    pub shape_info_offset: u32,
    pub shape_info_num: u32,
    pub hk_constraint_data_offset: u32, // optional pointer to obje
    pub unk_59: u32,
    pub hk_constraint_offset: u32, // optional pointer to hkConstraint
    pub keys2_offset: u32,
    pub keys2_order_offset: u32,
    pub valAs_offset: u32, // 8 ints
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
    pub tex_2: u32,
    pub tex_3: u32,
    pub tex_4: u32,
    pub tex_5: u32,
    pub tex_6: u32,
    pub tex_7: u32,
    pub tex_8: u32,
    pub tex_9: u32,
    pub tex_10: u32,
    pub tex_11: u32,
    pub tex_12: u32,
    pub tex_13: u32,
    pub tex_14: u32,
    pub tex_15: u32,
    pub tex_16: u32,
    pub tex_17: u32,
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
    pub side_flags: u16, //H',
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
    base: MatBase,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Mat2 {
    base: MatBase,
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
    pub unk_120: u16, //H',
    pub unk_120_: u16, //H',
    pub unk_121: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Mat3 {
    base: MatBase,
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
    base: MatBase,
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
    pub block_flag: u32,
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
    pub width: u16, //H',
    pub height: u16, //H',
    pub depth: u16, //H',
    pub levels: u16, //H',
    pub unk_16_1: u8, //16S',
    pub unk_16_2: u8, //16S',
    pub unk_16_3: u8, //16S',
    pub unk_16_4: u8, //16S',
    pub unk_16_5: u8, //16S',
    pub unk_16_6: u8, //16S',
    pub unk_16_7: u8, //16S',
    pub unk_16_8: u8, //16S',
    pub unk_16_9: u8, //16S',
    pub unk_16_10: u8, //16S',
    pub unk_16_11: u8, //16S',
    pub unk_16_12: u8, //16S',
    pub unk_16_13: u8, //16S',
    pub unk_16_14: u8, //16S',
    pub unk_16_15: u8, //16S',
    pub unk_16_16: u8, //16S',
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct AnimationInfo {
    pub key: Crc,
    pub block_flag: u32,
    pub offset: u32,
    pub size: u32,
    pub kind: u32,
    pub unk_5: u32,
    pub keys_num: u32,
    pub something_num: u32,
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
    pub block_ends_offset: u32,
    pub block_ends_num: u32,
    pub objC3_offset: u32,
    pub objC3_num: u32,
    pub objC4_offset: u32,
    pub objC4_num: u32,
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
    pub keys_num: u16, //H',
    pub keys2_num: u16, //H',
    pub keys2_offset: u32,
    pub unk_13: u32,
    pub unk_14: f32, //f',
    pub unk_15: u32,
    pub unk_16: u32,
    pub unk_17: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct LevelBlockInfo {
    pub key: u32,
    pub unk_1: u32,
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
    pub s1a: u32, 
    pub s2a: u32, 
    pub s1b: u32, 
    pub s2b: u32, 
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
pub struct Obj14Info {
    // points to list of ints in block1, maybe something to do with radiosity
    pub guid: u32,
    pub num: u32,
    pub offset: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct BlockAVal {
    pub unk_0: u32,
    pub block_flag: u32,
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
    pub valAs: Vec<u32>,
    pub mats: Vec<u32>,
    pub valCs: Vec<u32>,
    pub valDs: Vec<u32>,
    pub vbuffs: Vec<u32>,
    pub ibuffs: Vec<u32>,
    pub valGs: Vec<u32>,
    pub valJs: Vec<u32>,
    pub string_offsets: Vec<u32>,
    pub strings: Vec<String>,
    pub valK_header: Vec<u16>,
    pub valKs: Vec<u32>,
    pub valIs: Vec<u32>,
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
        val.valAs = OrderedDataVec::from_bytes::<O>(&data[info.valAs_offset as usize..], info.keys_num as usize * 8);
        val.mats = OrderedDataVec::from_bytes::<O>(&data[info.mat_offset as usize..], info.mat_num as usize);
        val.valCs = OrderedDataVec::from_bytes::<O>(&data[info.valCs_offset as usize..], info.valCs_num as usize);
        val.valDs = OrderedDataVec::from_bytes::<O>(&data[info.valDs_offset as usize..], info.valCs_num as usize * 8);
        val.vbuffs = OrderedDataVec::from_bytes::<O>(&data[info.vbuff_offset as usize..], info.vbuff_num as usize);
        val.ibuffs = OrderedDataVec::from_bytes::<O>(&data[info.ibuff_offset as usize..], info.ibuff_num as usize);
        val.valGs = OrderedDataVec::from_bytes::<O>(&data[info.valGs_offset as usize..], info.valGs_num as usize * 16);
        if (info.valJs_num == 0) && (info.valJs_offset != 0) && (info.valJs_offset != info.valGs_offset) {
            // val.valJs = OrderedDataVec::from_bytes::<O>(&data[info.valJs_offset as usize..], info.keys_num as usize);
            // for v in &val.valJs {
            //     let mut offset: u32 = OrderedData::from_bytes::<O>(&data[*v as usize..]);
            //     let start = offset;
            //     while data[offset as usize] != 0 { offset += 1; }
            //     let string = String::from_utf8(data[start as usize..offset as usize].to_vec()).unwrap();
            //     val.string_offsets.push(start);
            //     val.strings.push(string);
            // }
        } else {
            val.valJs = OrderedDataVec::from_bytes::<O>(&data[info.valJs_offset as usize..], info.valJs_num as usize);
        }
        if info.valKs_offset != 0 {
            val.valK_header = OrderedDataVec::from_bytes::<O>(&data[info.valKs_offset as usize..], 2);
            if (val.valK_header[0] != 3) || (val.valK_header[0] != 6) {
                warn!("unexpected valsK data {:?}", info.key);
            }
            val.valKs = OrderedDataVec::from_bytes::<O>(&data[info.valKs_offset as usize + 4..], 35);
        }
        if info.valIs_offset != 0 {
            val.valIs = OrderedDataVec::from_bytes::<O>(&data[info.valIs_offset as usize..], info.valGs_num as usize);
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
        if (info.valCs_offset == info.vbuff_offset) && (info.valCs_offset == info.ibuff_offset) && (info.valCs_offset == info.valDs_offset) {
            val.val = OrderedDataVec::from_bytes::<O>(&data[info.valCs_offset as usize..], 4);
        }
        val
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], info: &MeshInfo) {
        self.indices.to_bytes::<O>(&mut data[info.indices_offset as usize..]);
        self.keys.to_bytes::<O>(&mut data[info.keys_offset as usize..]);
        self.matrices.to_bytes::<O>(&mut data[info.matrices_offset as usize..]);
        self.valAs.to_bytes::<O>(&mut data[info.valAs_offset as usize..]);
        self.mats.to_bytes::<O>(&mut data[info.mat_offset as usize..]);
        self.valCs.to_bytes::<O>(&mut data[info.valCs_offset as usize..]);
        self.valDs.to_bytes::<O>(&mut data[info.valDs_offset as usize..]);
        self.vbuffs.to_bytes::<O>(&mut data[info.vbuff_offset as usize..]);
        self.ibuffs.to_bytes::<O>(&mut data[info.ibuff_offset as usize..]);
        self.valGs.to_bytes::<O>(&mut data[info.valGs_offset as usize..]);
        if (info.valJs_num) == 0 && (info.valJs_offset != 0) && (info.valJs_offset != info.valGs_offset) {
            self.valJs.to_bytes::<O>(&mut data[info.valJs_offset as usize..]);
            for (v, (off, string)) in zip(&self.valJs, zip(&self.string_offsets,& self.strings)) {
                off.to_bytes::<O>(&mut data[*v as usize..]);
                data[*off as usize..*off as usize+string.len()].copy_from_slice(string.as_bytes());
            }
        } else {
            self.valJs.to_bytes::<O>(&mut data[info.valJs_offset as usize..]);
        }
        if info.valKs_offset != 0 {
            self.valK_header.to_bytes::<O>(&mut data[info.valKs_offset as usize..]);
            self.valKs.to_bytes::<O>(&mut data[info.valKs_offset as usize + 4..])
        }
        if info.valIs_offset != 0 {
            self.valIs.to_bytes::<O>(&mut data[info.valIs_offset as usize..]);
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
        if (info.valCs_offset == info.vbuff_offset) && (info.valCs_offset == info.ibuff_offset) && (info.valCs_offset == info.valDs_offset) {
            self.val.to_bytes::<O>(&mut data[info.valCs_offset as usize..]);
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
        const ITEM_SIZES: [usize; 4] = [1,2,1,2];
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
        pub block_ends: Vec<u32>,
        pub objC3: Vec<u32>,
        pub objC4: Vec<u32>,
        pub flags: Vec<Vec<HkaSplineSkeletalAnimationFlags>>,
        pub vals_a: Vec<Vec<HkaSplineSkeletalAnimationObj1>>,
        pub vals_b: Vec<Vec<HkaSplineSkeletalAnimationObj2>>,
        pub vals_c: Vec<Vec<HkaSplineSkeletalAnimationObj1>>,
    }
    
    impl HkaSplineSkeletalAnimation {
        pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, info: &AnimationInfo) -> Self {
            let mut val = Self::default();
            val.block_starts = OrderedDataVec::from_bytes::<O>(&data[offset + info.block_starts_offset as usize..], info.block_starts_num as usize);
            val.block_ends = OrderedDataVec::from_bytes::<O>(&data[offset + info.block_ends_offset as usize..], info.block_ends_num as usize);
            val.objC3 = OrderedDataVec::from_bytes::<O>(&data[offset + info.objC3_offset as usize..], info.objC3_num as usize);
            val.objC4 = OrderedDataVec::from_bytes::<O>(&data[offset + info.objC4_offset as usize..], info.objC4_num as usize);
            for start in val.block_starts.iter() {
                let flags: Vec<HkaSplineSkeletalAnimationFlags> = OrderedDataVec::from_bytes::<O>(&data[offset + (start + info.block_offset) as usize..], info.keys_num as usize);
                let mut offset = offset + (info.block_offset + start + info.data_offset) as usize;
                let mut vals_a = Vec::with_capacity(flags.len());
                let mut vals_b = Vec::with_capacity(flags.len());
                let mut vals_c = Vec::with_capacity(flags.len());
                for flag in flags.iter() {
                    let a = HkaSplineSkeletalAnimationObj1::from_data::<O>(data, offset, flag.a, flag.f & 3);
                    offset += a.nbytes;
                    let b = HkaSplineSkeletalAnimationObj2::from_data::<O>(data, offset, flag.b, (flag.f >> 2) & 0xf);
                    offset += b.nbytes;
                    let c = HkaSplineSkeletalAnimationObj1::from_data::<O>(data, offset, flag.c, (flag.f >> 6) & 3);
                    offset += c.nbytes;
                    vals_a.push(a);
                    vals_b.push(b);
                    vals_c.push(c);
                }
                val.flags.push(flags);
                val.vals_a.push(vals_a);
                val.vals_b.push(vals_b);
                val.vals_c.push(vals_c);
            }
            val
        }
    
        pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize, info: &AnimationInfo) {
            self.block_starts.to_bytes::<O>(&mut data[offset + info.block_starts_offset as usize..]);
            self.block_ends.to_bytes::<O>(&mut data[offset + info.block_ends_offset as usize..]);
            self.objC3.to_bytes::<O>(&mut data[offset + info.objC3_offset as usize..]);
            self.objC4.to_bytes::<O>(&mut data[offset + info.objC4_offset as usize..]);
            self.block_starts.to_bytes::<O>(&mut data[offset + info.block_starts_offset as usize..]);
            for (start, ((flags, vals_a), (vals_b, vals_c))) in zip(self.block_starts.iter(), zip(zip(self.flags.iter(), self.vals_a.iter()), zip(self.vals_b.iter(), self.vals_c.iter()))) {
                flags.to_bytes::<O>(&mut data[offset + (start + info.block_offset) as usize..]);
                let mut offset = offset + (info.block_offset + start + info.data_offset) as usize;
                for ((flag, a), (b, c)) in zip(zip(flags, vals_a), zip(vals_b, vals_c)) {
                    a.into_data::<O>(data, offset, flag.a);
                    offset += a.nbytes;
                    b.into_data::<O>(data, offset, flag.b);
                    offset += b.nbytes;
                    c.into_data::<O>(data, offset, flag.c);
                    offset += c.nbytes;
                }
            }
        }
    }
    
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
    pub struct Obj5Header {
        pub objA_num: u32,
        pub objA_offset: u32,
        pub objB_num: u32,
        pub objB_offset: u32,
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub obj2: HashMap<usize, Vec<u32>>,
    pub obj3: HashMap<usize, Vec<u32>>,
    pub keys: HashMap<usize, Vec<u32>>,
    pub obj5_header: HashMap<usize, animation::Obj5Header>,
    pub obj5A: HashMap<usize, Vec<u32>>,
    pub obj5B: HashMap<usize, Vec<u32>>,
    pub objC: HashMap<usize, animation::HkaSplineSkeletalAnimation>,
}

impl Animation {
    pub fn unpack_from_block<O: ByteOrder + 'static>(&mut self, data: &[u8], offset: usize, index: usize, info: &AnimationInfo) {
        self.obj2.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.obj2_offset as usize..], info.obj2_num as usize * 4));
        self.obj3.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.obj3_offset as usize..], info.obj3_num as usize * 11));
        self.keys.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.keys_offset as usize..], info.keys_num as usize));
        if info.obj5_offset != 0 {
            let obj5_header: animation::Obj5Header = OrderedData::from_bytes::<O>(&data[offset + info.obj5_offset as usize..]);
            self.obj5A.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + obj5_header.objA_offset as usize..], obj5_header.objA_num as usize * 7));
            self.obj5B.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + obj5_header.objB_offset as usize..], obj5_header.objB_num as usize * 7));
            self.obj5_header.insert(index, obj5_header);
        }
        if info.kind == 3 {
            self.objC.insert(index, animation::HkaSplineSkeletalAnimation::from_data::<O>(data, offset, info));
        } else if info.kind < 3 {
            warn!("Unhandled animation type {}", info.kind);
        } else {
            warn!("Unknown animation type {}", info.kind);
        }
    }

    pub fn pack_into_block<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize, index: usize, info: &AnimationInfo) {
        self.obj2.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.obj2_offset as usize..]);
        self.obj3.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.obj3_offset as usize..]);
        self.keys.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.keys_offset as usize..]);
        if info.obj5_offset != 0 {
            let obj5_header = self.obj5_header.get(&index).unwrap();
            obj5_header.to_bytes::<O>(&mut data[offset + info.obj5_offset as usize..]);
            self.obj5A.get(&index).unwrap().to_bytes::<O>(&mut data[offset + obj5_header.objA_offset as usize..]);
            self.obj5B.get(&index).unwrap().to_bytes::<O>(&mut data[offset + obj5_header.objB_offset as usize..]);
        }
        if info.kind == 3 {
            self.objC.get(&index).unwrap().into_data::<O>(data, offset, info);
        }
    }

    pub fn unpack_block<O: ByteOrder + 'static>(anims: &mut [Self], infos: &[AnimationInfo], data: & [u8], offset: usize, index: usize) {
        let mut offset = offset;
        for (anim, info) in zip(anims, infos) {
            let block_flag = 1u32 << index;
            if block_flag & info.block_flag != 0 {
                anim.unpack_from_block::<O>(data, offset, index, info);
                offset += info.size as usize;
            }
        }
    }

    pub fn pack_block<O: ByteOrder + 'static>(anims: & [Self], infos: &[AnimationInfo], data: &mut [u8], offset: usize, index: usize) {
        let mut offset = offset;
        for (anim, info) in zip(anims, infos) {
            let block_flag = 1u32 << index;
            if block_flag & info.block_flag != 0 {
                anim.pack_into_block::<O>(data, offset, index, info);
                offset += info.size as usize;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
                    fmt.push((BaseTypes::INT_KEY, VertexUsage::Pad));
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
                    fmt.push((BaseTypes::INT_KEY, VertexUsage::Pad));
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
                fmt.push((BaseTypes::INT_KEY, VertexUsage::Pad));
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VertexBuffer {
    pub vals: Vec<Vec<BaseTypes>>
}

impl VertexBuffer {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &VBuffInfo, formats: &mut HashMap<(u32, u32), (Vec<(u32, VertexUsage)>, usize)>) -> Self {
        let (fmt, size) = formats.entry((info.fmt1, info.fmt2)).or_insert_with(|| {
            get_vertex_format::<O>(info.fmt1, info.fmt2)
        });
        assert!(info.size as usize % *size == 0);
        let n = info.size as usize / *size;
        let mut vals = Vec::with_capacity(n);
        let mut offset = info.offset as usize;
        for _ in 0..n {
            let mut val = Vec::with_capacity(fmt.len());
            for kind in fmt.iter() {
                let mut v = BaseTypes::from_data::<O>(&data[offset..], kind.0);
                if (TypeId::of::<O>() == TypeId::of::<BE>()) && (kind.1 == VertexUsage::BlendWeight) {
                    match &mut v {
                        BaseTypes::Vector4(val) => {
                            val.x = val.x/2.0 + 0.5;
                            val.y = val.y/2.0 + 0.5;
                            val.z = val.z/2.0 + 0.5;    
                        },
                        BaseTypes::Color(val) => {
                            let z = *val & 0x3FF;
                            let y = (*val >> 10) & 0x3FF;
                            let x = (*val >> 20) & 0x3FF;
                            let x = if x & 0x200 != 0 { (x as f32 - 512.0) / 512.0 * 127.0 } else { x as f32 / 511.0 * 128.0  + 127.0 } as u32;
                            let y = if y & 0x200 != 0 { (y as f32 - 512.0) / 512.0 * 127.0 } else { y as f32 / 511.0 * 128.0  + 127.0 } as u32;
                            let z = if z & 0x200 != 0 { (z as f32 - 512.0) / 512.0 * 127.0 } else { z as f32 / 511.0 * 128.0  + 127.0 } as u32;
                            *val = (127 << 24) | (z << 16) | (y << 8) | x;
                        },
                        _ => ()
                    }
                }
                offset += v.size::<O>();
                val.push(v);
            }
            vals.push(val);
        }
        Self { vals }
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut[u8], info: &VBuffInfo) {
        let mut offset = info.offset as usize;
        let mut off_ = 0;
        for val in &self.vals {
            for v in val {
                v.into_data::<O>(&mut data[offset..], &mut off_);
                offset += v.size::<O>();
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Obj14 {
    pub vals: Vec<u32>
}

impl Obj14 {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &Obj14Info) -> Self {
        Self { vals: OrderedDataVec::from_bytes::<O>(&data[info.offset as usize..], info.num as usize) }
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], info: &Obj14Info) {
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