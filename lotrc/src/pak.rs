use std::{any::TypeId, collections::HashMap, fmt::Display, iter::zip, num::ParseIntError, ops::Div, str::FromStr};
use log::warn;
use serde::{Serialize, Deserialize};
use serde_with::{SerializeDisplay, DeserializeFromStr};
use serde_with::serde_as;
use anyhow::Result;
use pyo3::prelude::*;

use lotrc_proc::{OrderedData, basicpymethods, PyMethods};
use crate::{
    pak_alt::GltfData,
    types::{
        BaseTypes, Color, OrderedData, Vector4, Matrix4x4, OrderedDataVec, Vector2, 
        Crc, Vector3, OrderedDataImpl, Version, XBOX, PS3, PC
    }
};

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct Header {
    #[ordered_data(PC)]
    pub block_a_num: u32, 
    #[ordered_data(PC)]
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
    pub obja_size: u32, 
    pub obj0_size: u32, 
    pub model_info_size: u32, 
    pub buffer_info_size: u32, 
    pub mat1_size: u32, 
    pub mat2_size: u32, 
    pub mat3_size: u32, 
    pub mat4_size: u32, 
    pub mat_extra_size: u32, 
    pub unk_26: u32, 
    pub shape_info_size: u32, 
    pub hk_shape_info_size: u32, 
    pub hk_constraint_data_size: u32, 
    pub vbuff_info_size: u32, 
    pub ibuff_info_size: u32, 
    pub texture_info_size: u32, 
    pub animation_info_size: u32,
    pub hk_constraint_info_size: u32,
    pub effect_info_size: u32,
    pub pfield_info_size: u32,
    pub gfx_block_info_size: u32,
    pub animation_block_info_size: u32, 
    pub foliage_info_size: u32, 
    pub radiosity_vals_info_size: u32,
    pub unk_41: u32, 
    pub obja_num: u32, 
    pub obj0_num: u32, 
    pub model_info_num: u32,
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
    pub radiosity_vals_info_num: u32, 
    pub unk_66: u32, 
    pub obja_offset: u32,
    pub obj0_offset: u32, 
    pub model_info_offset: u32,  // max loaded is 0x400
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
    pub radiosity_vals_info_offset: u32, 
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

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct ObjA {
    #[ordered_data(PC)]
    pub key: Crc,
    #[ordered_data(PC)]
    pub unk_1: u32,
    #[ordered_data(PC)]
    pub size: u32,
    #[ordered_data(PC)]
    pub size_comp: u32,
    #[ordered_data(PC)]
    pub unk_4: u32,
    #[ordered_data(PC)]
    pub kind: u32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct Obj0 {
    #[ordered_data(PC)]
    pub unk_0: u32,
    #[ordered_data(PC)]
    pub key: Crc,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct LodMeshes {
    pub start: u32,
    pub static_end: u32,
    pub skinned_end: u32,
    pub physics_end: u32,
    pub breakable_end: u32,
}

impl LodMeshes {
    pub const UNKNOWN: u32 = 1;
    pub const STATIC: u32 = 2;
    pub const SKINNED: u32 = 4;
    pub const PHYSICS: u32 = 8;
    pub const BREAKABLE: u32 = 16;
    pub const LOD0: u32 = 32;
    pub const LOD1: u32 = 64;
    pub const LOD2: u32 = 128;
    pub const LOD3: u32 = 256;
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, OrderedData)]
pub struct ValA(f32, f32, f32, f32, f32, f32, f32, f32);


impl IntoPy<PyObject> for ValA {
    fn into_py(self, py: Python<'_>) -> PyObject {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for ValA {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let [a, b, c, d, e, f, g, h] = <[f32; 8]>::extract_bound(ob)?;
        Ok(Self(a,b,c,d,e,f,g,h))
    }
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct ModelInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub mat_offset: u32,
    pub buffer_info_offset: u32, // pointer to obj2, uses mat_num of sequential objects
    pub unk_4: ValA,
    pub mesh_order_offset: u32, // ints (c & 0x3fffffff is an index into the obj2s referenced by this object) (1 for each mesh)
    pub lod0: LodMeshes,
    pub lod1: LodMeshes,
    pub lod2: LodMeshes,
    pub lod3: LodMeshes,
    pub mat_num: u32,
    pub bones_offset: u32, // ints
    pub bone_parents_offset: u32,
    pub bone_transforms_offset: u32, // 16 ints (matrix?) for keys_num
    pub bones_num: u32,
    pub skin_binds_offset: u32,
    pub skin_binds_num: u32,
    pub skin_order_offset: u32,
    pub vbuff_offset: u32,
    pub vbuff_num: u32,
    pub ibuff_offset: u32,
    pub ibuff_num: u32,
    pub vals_d_offset: u32,
    pub unk_46: u32, // probably a float
    pub unk_47: u32, // maybe something to do with variation
    pub vals_j_num: u32,
    pub vals_j_offset: u32,
    pub block_offset: u32,
    pub vals_k_offset: u32, // not sure on the size, seems to be 36 ints
    pub asset_key: Crc, // data in bin that is vertex & index buffer values
    pub asset_type: u32,
    pub unk_54: u32,
    #[name_ps3(shape_offset)]
    pub unk_55: u32,
    #[name_ps3(shape_num)]
    pub shape_offset: u32,
    #[name_ps3(hk_constraint_data_offset)]
    pub shape_num: u32,
    #[name_ps3(hk_constraint_data_num)]
    pub hk_constraint_data_offset: u32, // optional pointer to obje
    #[name_ps3(hk_constraint_offset)]
    pub hk_constraint_data_num: u32,
    #[name_ps3(slots_offset)]
    pub hk_constraint_offset: u32, // optional pointer to hkConstraint
    #[name_ps3(slot_map_offset)]
    pub slots_offset: u32,
    #[name_ps3(vals_a_offset)]
    pub slot_map_offset: u32,
    #[name_ps3(unk_55)]
    pub vals_a_offset: u32, // 8 ints
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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
    #[ordered_data(skipPS3)]
    pub unk_20: u32,
    #[ordered_data(skipPS3)]
    pub unk_21: u32,
    #[ordered_data(skipPS3)]
    pub unk_22: u32,
    #[ordered_data(skipPS3)]
    pub unk_23: u32,
    #[ordered_data(skipPS3)]
    pub unk_24: u32,
    #[ordered_data(skipPS3)]
    pub unk_25: u32,
    #[ordered_data(skipPS3)]
    pub unk_26: u32,
    #[ordered_data(skipPS3)]
    pub unk_27: u32,
    #[ordered_data(skipPS3)]
    pub unk_28: u32,
    #[ordered_data(skipPS3)]
    pub unk_29: u32,
    #[ordered_data(skipPS3)]
    pub unk_30: u32,
    #[ordered_data(skipPS3)]
    pub unk_31: u32,
    #[ordered_data(skipPS3)]
    pub v_size: u32,
    #[ordered_data(skipPS3)]
    pub v_size_2: u32,
    #[ordered_data(skipPS3)]
    pub v_size_3: u32,
    #[ordered_data(skipPS3)]
    pub unk_35: u32,
    #[ordered_data(skipPS3)]
    pub unk_36: u32,
    #[ordered_data(skipPS3)]
    pub unk_37: u32,
    #[ordered_data(skipPS3)]
    pub unk_38: u32,
    #[ordered_data(skipPS3)]
    pub unk_39: u32,
    #[ordered_data(skipPS3)]
    pub unk_40: u32,
    #[ordered_data(skipPS3)]
    pub unk_41: u32,
    #[ordered_data(skipPS3)]
    pub unk_42: u32,
    #[ordered_data(skipPS3)]
    pub unk_43: u32,
    #[ordered_data(skipPS3)]
    pub unk_44: u32,
    #[ordered_data(skipPS3)]
    pub unk_45: u32,
    #[ordered_data(skipPS3)]
    pub unk_46: u32,
    #[ordered_data(skipPS3)]
    pub unk_47: u32,
    #[ordered_data(skipPS3)]
    pub vbuff_size: u32,
    #[ordered_data(skipPS3)]
    pub vbuff_size_2: u32,
    #[ordered_data(skipPS3)]
    pub vbuff_size_3: u32,
    #[ordered_data(skipPS3)]
    pub unk_51: u32,
    #[ordered_data(skipPS3)]
    pub unk_52: u32,
    #[ordered_data(skipPS3)]
    pub unk_53: u32,
    #[ordered_data(skipPS3)]
    pub unk_54: u32,
    #[ordered_data(skipPS3)]
    pub unk_55: u32,
    #[ordered_data(skipPS3)]
    pub unk_56: u32,
    #[ordered_data(skipPS3)]
    pub unk_57: u32,
    #[ordered_data(skipPS3)]
    pub unk_58: u32,
    #[ordered_data(skipPS3)]
    pub unk_59: u32,
    #[ordered_data(skipPS3)]
    pub unk_60: u32,
    #[ordered_data(skipPS3)]
    pub unk_61: u32,
    #[ordered_data(skipPS3)]
    pub unk_62: u32,
    #[ordered_data(skipPS3)]
    pub unk_63: u32,
    #[ordered_data(skipPS3)]
    pub unk_64: u32,
    #[ordered_data(skipPS3)]
    pub ibuff_info_offset: u32, // poiner to objg
    #[ordered_data(skipPS3)]
    pub i_num: u32, // number of indeices in ibuffer
    #[ordered_data(skipPS3)]
    pub unk_67: u32,
    #[ordered_data(skipPS3)]
    pub skin_offset: u32,
    #[ordered_data(skipPS3)]
    pub skin_size: u32,
    #[ordered_data(skipPS3)]
    pub unk_70: u32,
    #[ordered_data(skipPS3)]
    pub tri_num: u32, // number of objects(triangles) in ibufffer
    #[ordered_data(skipPS3)]
    pub unk_72: u32, // possibly index to bone_transform used for mesh
    #[ordered_data(skipPS3)]
    pub unk_73: u32,
    #[ordered_data(skipPS3)]
    pub unk_74: u32,
    #[ordered_data(skipPS3)]
    pub unk_75: u32,
    #[ordered_data(skipPS3)]
    pub unk_76: u32,
    #[ordered_data(skipPS3)]
    pub unk_77: u32,
    #[ordered_data(skipPS3)]
    pub unk_78: u32,
    #[ordered_data(skipPS3)]
    pub unk_79: u32,
    #[ordered_data(skipPS3)]
    pub unk_80: u32,
    #[ordered_data(skipPS3)]
    pub unk_81: u32,
    #[ordered_data(skipPS3)]
    pub unk_82: u32,
    #[ordered_data(skipPS3)]
    pub unk_83: u32,
    #[ordered_data(skipPS3)]
    pub unk_84: u32,
    #[ordered_data(skipPS3)]
    pub unk_85: u32,
    #[ordered_data(skipPS3)]
    pub unk_86: u32,
    #[ordered_data(skipPS3)]
    pub unk_87: u32,
    #[ordered_data(skipPS3)]
    pub variation_id: u8,
    #[ordered_data(skipPS3)]
    pub variation: u8,
    #[ordered_data(skipPS3)]
    pub unk_88c: u8,
    #[ordered_data(skipPS3)]
    pub unk_88d: u8,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct MatBase {
    pub unk_0: u32,
    #[ordered_data(skipPS3)]
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
    #[ordered_data(skipPS3)]
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
    pub key: Crc,
    pub unk_88: u32,
    pub z_89: u32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct Mat1 {
    pub base: MatBase,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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
    #[ordered_data(skipPC, skipXBOX)]
    #[serde(default)]
    pub unk_116: u32,
    #[ordered_data(skipPC, skipXBOX)]
    #[serde(default)]
    pub unk_117: u32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct ShapeInfo {
    pub offset: u32, // sometimes a pointer to something, otherwise the number of strings from the obj1 pointing to this
    pub kind: u32, // 0, 1, 2, 3, 4, 5
    pub unk_2: u32,
    pub unk_3: f32,
    pub unk_4: f32,
    pub unk_5: f32,
    pub translation: Vector3,
    pub rotation: Vector4,
    pub unk_13: f32,
    pub unk_14: f32,
    pub unk_15: f32,
    pub unk_16: f32,
    pub unk_17: f32,
    pub unk_18: f32,
    pub unk_19: f32,
    pub unk_20: f32,
    pub unk_21: f32,
    pub unk_22: f32,
    pub unk_23: f32,
    pub unk_24: f32,
    pub unk_25: f32,
    pub unk_26: f32,
    pub hk_shape_num: u32,
    pub hk_shape_offset: u32, // pointer to objd
    pub unk_29a: u8,
    pub unk_29b: u8,
    pub unk_29c: u8,
    pub unk_29d: u8,
    pub unk_30: f32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct VBuffInfo {
    pub unk_0: u32,
    #[name_ps3(unk_7)]
    pub size: u32,
    pub unk_3: u32,
    #[name_ps3(unk_8)]
    pub offset: u32,
    #[name_xbox(fmt2)]
    #[name_ps3(unk_9)]
    pub fmt1: u32,
    #[name_xbox(fmt1)]
    #[name_ps3(size)]
    pub fmt2: u32,
    pub unk_6: u32,
    #[name_ps3(offset)]
    pub unk_7: u32,
    #[name_ps3(fmt2)]
    #[ordered_data(skipPC)]
    pub unk_8: u32,
    #[name_ps3(fmt1)]
    #[ordered_data(skipPC)]
    pub unk_9: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_10: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_11: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_12: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_13: u32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct IBuffInfo {
    pub unk_0: u32,
    #[name_ps3(unk_5)]
    pub size: u32,
    #[name_ps3(unk_6)]
    pub format: u32,
    pub unk_3: u32,
    #[name_ps3(unk_8)]
    pub offset: u32,
    #[name_ps3(size)]
    pub unk_5: u32,
    #[name_ps3(format)]
    #[ordered_data(skipPC)]
    pub unk_6: u32,
    #[ordered_data(skipPC)]
    pub unk_7: u32,
    #[name_ps3(offset)]
    #[ordered_data(skipPC)]
    pub unk_8: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_9: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_10: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_11: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_12: u32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct HkConstraintInfo {
    pub kind: u32,
    pub bone_parents_offset: u32, 
    pub bone_parents_num: u32,
    pub bone_names_offset: u32,
    pub bone_names_num: u32,
    pub bone_transforms_offset: u32,
    pub bone_transforms_num: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub bones_offset: u32,
    pub bones_num: u16,
    pub bone_order_num: u16,
    pub bone_order_offset: u32,
    pub unk_13: u32,
    pub unk_14: f32,
    pub vals2_num: u32,
    pub vals2_offset: u32,
    pub unk_17: u32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct EffectInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub offset: u32,
    pub size: u32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct PFieldInfo {
    pub key1: Crc, 
    pub key2: Crc, 
    pub width: u32, 
    pub height: u32, 
    pub offset: u32, 
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct GFXBlockInfo {
    // GFX blocks?, unchanged by encoding, model as data
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct AnimationBlockInfo {
    pub key: Crc,
    #[serde(alias="unk_1")]
    pub guid: u32,
    pub key_name: Crc,
    pub offset: u32,
    pub size: u32,
    pub size_comp: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: u32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct FoliageInfo {
    pub key: Crc, 
    pub kind: u32, 
    pub lb_w: i32, 
    pub lb_h: i32, 
    pub ub_w: i32, 
    pub ub_h: i32, 
    pub scale: f32, 
    pub offset: u32, 
    pub key_mesh: Crc, 
    pub key_mesh_lod1: Crc, 
    pub key_mesh_lod2: Crc, 
    pub color: Vector4, 
    pub lod1a: f32,
    pub lod1b: f32, 
    pub lod2a: f32, 
    pub lod2b: f32, 
    pub lod_max: f32, 
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
// points to list of ints in block1
pub struct RadiosityValsInfo {
    pub guid: u32,
    pub num: u32,
    pub offset: u32,
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct BlockAVal {
    pub unk_0: u32,
    pub gamemodemask: i32,
    pub key: Crc,
    pub unk_3: u32,
    pub unk_4: u32,
    pub unk_5: u32,
    pub unk_6: u32,
}

pub mod model {
    use super::*;
    #[basicpymethods]
    #[pyclass(module="pak.model", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    pub struct BlockHeader {
        pub a: u32,
        pub b: u32,
        pub unk_2: u32,
        pub unk_3: u32,
        pub unk_4: u32,
    }

    #[basicpymethods]
    #[pyclass(module="pak.model", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    pub struct BlockVal {
        pub unk_0: u32,
        pub unk_1: u32,
        pub unk_2: u32,
        pub unk_3: u32,
        pub unk_4: u16,
        pub unk_5: u16,
    }
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
pub struct Model{
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
    pub blocks: Vec<(model::BlockHeader, Vec<u32>, Vec<model::BlockVal>, Vec<u32>)>,
    pub val: Vec<u32>,
}

impl Model {
    pub fn from_data<O: Version + 'static>(data: &[u8], info: &ModelInfo) -> Result<Self> {
        let mut val = Self::default();

        val.indices = OrderedDataVec::from_bytes::<O>(&data[info.bone_parents_offset as usize..], info.bones_num.max(4) as usize)?;
        assert!(val.indices[0] == 0xffffffff);
        val.keys = OrderedDataVec::from_bytes::<O>(&data[info.bones_offset as usize..], info.bones_num as usize)?;
        val.matrices = OrderedDataVec::from_bytes::<O>(&data[info.bone_transforms_offset as usize..], info.bones_num as usize)?;
        val.vals_a = OrderedDataVec::from_bytes::<O>(&data[info.vals_a_offset as usize..], info.bones_num as usize * 8)?;
        val.mats = OrderedDataVec::from_bytes::<O>(&data[info.mat_offset as usize..], info.mat_num as usize)?;
        val.vals_c = OrderedDataVec::from_bytes::<O>(&data[info.mesh_order_offset as usize..], info.lod3.breakable_end as usize)?;
        val.vals_d = OrderedDataVec::from_bytes::<O>(&data[info.vals_d_offset as usize..], info.lod3.breakable_end as usize * 8)?;
        val.vbuffs = OrderedDataVec::from_bytes::<O>(&data[info.vbuff_offset as usize..], info.vbuff_num as usize)?;
        val.ibuffs = OrderedDataVec::from_bytes::<O>(&data[info.ibuff_offset as usize..], info.ibuff_num as usize)?;
        val.vals_g = OrderedDataVec::from_bytes::<O>(&data[info.skin_binds_offset as usize..], info.skin_binds_num as usize * 16)?;
        if (info.vals_j_num == 0) && (info.vals_j_offset != 0) && (info.vals_j_offset != info.skin_binds_offset) {
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
            val.vals_j = OrderedDataVec::from_bytes::<O>(&data[info.vals_j_offset as usize..], info.vals_j_num as usize)?;
        }
        if info.vals_k_offset != 0 {
            val.val_k_header = OrderedDataVec::from_bytes::<O>(&data[info.vals_k_offset as usize..], 2)?;
            // if (val.val_k_header[0] != 3) || (val.val_k_header[0] != 6) {
            //     warn!("unexpected valsK data {:?}", info.key);
            // }
            val.vals_k = OrderedDataVec::from_bytes::<O>(&data[info.vals_k_offset as usize + 4..], 35)?;
        }
        if info.skin_order_offset != 0 {
            val.vals_i = OrderedDataVec::from_bytes::<O>(&data[info.skin_order_offset as usize..], info.skin_binds_num as usize)?;
        }
        if info.slots_offset != 0 {
            assert!(info.slot_map_offset != 0);
            let mut i = 0;
            {
                while u32::from_bytes::<O>(&data[info.slots_offset as usize + i * 8..])? != 0 {
                    i += 1;
                }
                i += 1;
            }
            val.keys2 = OrderedDataVec::from_bytes::<O>(&data[info.slots_offset as usize..], i * 2)?;
            val.keys2_order = OrderedDataVec::from_bytes::<O>(&data[info.slot_map_offset as usize..], *val.keys2.last().unwrap() as usize)?;
        }
        if info.block_offset != 0 {
            val.block_header = OrderedData::from_bytes::<O>(&data[info.block_offset as usize..])?;
            let n = (info.lod0.physics_end - info.lod0.skinned_end) as usize;
            val.block_offsets = OrderedDataVec::from_bytes::<O>(&data[info.block_offset as usize + 4..], n+1)?;
            for i in 0..n {
                let size = (val.block_offsets[i+1] - val.block_offsets[i]) as usize;
                let offset = (val.block_offsets[i] + info.block_offset) as usize;
                let header: model::BlockHeader = OrderedData::from_bytes::<O>(&data[offset..])?;
                let mut s = model::BlockHeader::size::<O>();
                let vals_a: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (header.a + header.b) as usize * 12)?;
                s += vals_a.size::<O>();
                let vals_b: Vec<model::BlockVal> = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (size - s)/model::BlockVal::size::<O>())?;
                s += vals_b.size::<O>();
                let extra = OrderedDataVec::from_bytes::<O>(&data[offset+s..], (size - s)/4)?;
                val.blocks.push((header, vals_a, vals_b, extra));
            }
        }
        // not sure why this pops up once, maybe it is padding between items?
        if (info.mesh_order_offset == info.vbuff_offset) && (info.mesh_order_offset == info.ibuff_offset) && (info.mesh_order_offset == info.vals_d_offset) {
            val.val = OrderedDataVec::from_bytes::<O>(&data[info.mesh_order_offset as usize..], 4)?;
        }
        Ok(val)
    }

    pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], info: &ModelInfo) -> Result<()> {
        self.indices.to_bytes::<O>(&mut data[info.bone_parents_offset as usize..])?;
        self.keys.to_bytes::<O>(&mut data[info.bones_offset as usize..])?;
        self.matrices.to_bytes::<O>(&mut data[info.bone_transforms_offset as usize..])?;
        self.vals_a.to_bytes::<O>(&mut data[info.vals_a_offset as usize..])?;
        self.mats.to_bytes::<O>(&mut data[info.mat_offset as usize..])?;
        self.vals_c.to_bytes::<O>(&mut data[info.mesh_order_offset as usize..])?;
        self.vals_d.to_bytes::<O>(&mut data[info.vals_d_offset as usize..])?;
        self.vbuffs.to_bytes::<O>(&mut data[info.vbuff_offset as usize..])?;
        self.ibuffs.to_bytes::<O>(&mut data[info.ibuff_offset as usize..])?;
        self.vals_g.to_bytes::<O>(&mut data[info.skin_binds_offset as usize..])?;
        if (info.vals_j_num) == 0 && (info.vals_j_offset != 0) && (info.vals_j_offset != info.skin_binds_offset) {
            self.vals_j.to_bytes::<O>(&mut data[info.vals_j_offset as usize..])?;
            for (v, (off, string)) in zip(&self.vals_j, zip(&self.string_offsets,& self.strings)) {
                off.to_bytes::<O>(&mut data[*v as usize..])?;
                data[*off as usize..*off as usize+string.len()].copy_from_slice(string.as_bytes());
            }
        } else {
            self.vals_j.to_bytes::<O>(&mut data[info.vals_j_offset as usize..])?;
        }
        if info.vals_k_offset != 0 {
            self.val_k_header.to_bytes::<O>(&mut data[info.vals_k_offset as usize..])?;
            self.vals_k.to_bytes::<O>(&mut data[info.vals_k_offset as usize + 4..])?;
        }
        if info.skin_order_offset != 0 {
            self.vals_i.to_bytes::<O>(&mut data[info.skin_order_offset as usize..])?;
        }
        if info.slots_offset != 0 {
            self.keys2.to_bytes::<O>(&mut data[info.slots_offset as usize..])?;
            self.keys2_order.to_bytes::<O>(&mut data[info.slot_map_offset as usize..])?;
        }
        if info.block_offset != 0 {
            self.block_header.to_bytes::<O>(&mut data[info.block_offset as usize..])?;
            self.block_offsets.to_bytes::<O>(&mut data[info.block_offset as usize + 4..])?;
            for (i, (header, vals_a, vals_b, extra)) in self.blocks.iter().enumerate() {
                let offset = (self.block_offsets[i] + info.block_offset) as usize;
                header.to_bytes::<O>(&mut data[offset..])?;
                let mut s = model::BlockHeader::size::<O>();
                vals_a.to_bytes::<O>(&mut data[offset + s..])?;
                s += vals_a.size::<O>();
                vals_b.to_bytes::<O>(&mut data[offset + s..])?;
                s += vals_b.size::<O>();
                extra.to_bytes::<O>(&mut data[offset + s..])?;
            }
        }
        if (info.mesh_order_offset == info.vbuff_offset) && (info.mesh_order_offset == info.ibuff_offset) && (info.mesh_order_offset == info.vals_d_offset) {
            self.val.to_bytes::<O>(&mut data[info.mesh_order_offset as usize..])?;
        }
        Ok(())
    }
}

pub mod shape {
    use super::*;
    #[basicpymethods]
    #[pyclass(module="pak.shape", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    pub struct Header {
        pub num: u32,
        pub unk_1: u32,
        pub unk_2: u32,
        pub unk_3: u32,
    }
}

#[pyclass(module="pak", set_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
pub struct Shape {
    #[pyo3(get)]
    pub header: shape::Header,
    #[pyo3(get)]
    pub vals: Vec<u32>,
    pub data: Vec<u8>,
}

#[basicpymethods]
#[pymethods]
impl Shape {
    #[getter]
    fn get_data(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::from(&self.data[..])
    }
}

impl Shape {
    pub fn from_data<O: Version + 'static>(data: &[u8], info: &ShapeInfo) -> Result<Self> {
        let mut val = Self::default();
        if info.kind == 0 {
            let mut offset = info.offset as usize;
            val.header = OrderedData::from_bytes::<O>(&data[offset..])?;
            offset += shape::Header::size::<O>();
            val.vals = OrderedDataVec::from_bytes::<O>(&data[offset..], val.header.num as usize)?;
            offset += val.vals.size::<O>();
            val.data = OrderedDataVec::from_bytes::<O>(&data[offset..], *val.vals.last().unwrap() as usize + 2)?; // might need to be more than +2, not sure    
        }
        Ok(val)
    }

    pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], info: &ShapeInfo) -> Result<()> {
        if info.kind == 0 {
            let mut offset = info.offset as usize;
            self.header.to_bytes::<O>(&mut data[offset..])?;
            offset += shape::Header::size::<O>();
            self.vals.to_bytes::<O>(&mut data[offset..])?;
            offset += self.vals.size::<O>();
            self.data.to_bytes::<O>(&mut data[offset..])?;
        } else {
            warn!("Unknown & Unhandled Shape type {}", info.kind);
        }
        Ok(())
    }
}

#[pyclass(module="pak", set_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
pub struct HkShape {
    #[pyo3(get)]
    pub a: Vec<u32>,
    #[pyo3(get)]
    pub b: Vec<u32>,
    pub c: Vec<u8>,
    #[pyo3(get)]
    pub d: Vec<u32>,
    #[pyo3(get)]
    pub e: Vec<u16>,
}

#[basicpymethods]
#[pymethods]
impl HkShape {
    #[getter]
    fn get_c(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::from(&self.c[..])
    }
}

impl HkShape {
    pub fn from_data<O: Version + 'static>(data: &[u8], info: &HkShapeInfo) -> Result<Self> {
        let mut val = Self::default();
        if info.kind == 5 {
            val.a = OrderedDataVec::from_bytes::<O>(&data[info.a_offset as usize..], info.a_num as usize * 4)?;
            let mut b_num = info.b_num as usize; // sketchy stuff to account for data that was not otherwise captured, is it needed?
            while (info.b_offset as usize + b_num * 12) % 16 != 0 { b_num += 1; }
            val.b = OrderedDataVec::from_bytes::<O>(&data[info.b_offset as usize..], b_num * 3)?;
        } else if info.kind == 6 {
            val.c = OrderedDataVec::from_bytes::<O>(&data[info.c_offset as usize..], info.c_num as usize)?;
            val.d = OrderedDataVec::from_bytes::<O>(&data[info.d_offset as usize..], info.d_num as usize * 3)?;
            val.e = OrderedDataVec::from_bytes::<O>(&data[info.e_offset as usize..], info.e_num as usize * 3)?;
        } else if info.kind > 6 {
            warn!("Unknown & Unhandled HkShape type {}", info.kind);
        }
        Ok(val)
    }

    pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], info: &HkShapeInfo) -> Result<()> {
        if info.kind == 5 {
            self.a.to_bytes::<O>(&mut data[info.a_offset as usize..])?;
            self.b.to_bytes::<O>(&mut data[info.b_offset as usize..])?;
        } else if info.kind == 6 {
            self.c.to_bytes::<O>(&mut data[info.c_offset as usize..])?;
            self.d.to_bytes::<O>(&mut data[info.d_offset as usize..])?;
            self.e.to_bytes::<O>(&mut data[info.e_offset as usize..])?;
        } else {
            warn!("Unknown & Unhandled HkShape type {}", info.kind);
        }
        Ok(())
    }
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
pub struct HkConstraint {
    pub shorts: Vec<u16>,
    pub strings: Vec<(String, u32, u32)>,
    pub string_offsets: Vec<u32>,
    pub vals: Vec<u32>,
    pub keys: Vec<u32>,
    pub keys2: Vec<u32>,
}

impl HkConstraint {
    pub fn from_data<O: Version + 'static>(data: &[u8], info: &HkConstraintInfo) -> Result<Self> {
        let mut val = Self::default();
        if info.kind != 0 { warn!("Unknown & Unhandled HkConstraint type {}", info.kind); }

        val.shorts = OrderedDataVec::from_bytes::<O>(&data[info.bone_parents_offset as usize..], info.bone_parents_num as usize)?;
        assert!(val.shorts[0] == 0xFFFF);

        val.string_offsets = OrderedDataVec::from_bytes::<O>(&data[info.bone_names_offset as usize..], info.bone_names_num as usize)?;
        for offset_ in val.string_offsets.iter() {
            let (mut offset, val_) = { 
                let vals: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[*offset_ as usize..], 2)?;

                (vals[0], vals[1]) 
            };
            let start = offset;
            while data[offset as usize] != 0 { offset += 1; }
            let string = String::from_utf8(data[start as usize..offset as usize].to_vec()).unwrap();
            val.strings.push((string, start, val_));
        }
        val.vals = OrderedDataVec::from_bytes::<O>(&data[info.bone_transforms_offset as usize..], info.bone_transforms_num as usize * 12)?;
        val.keys = OrderedDataVec::from_bytes::<O>(&data[info.bones_offset as usize..], info.bones_num as usize)?;
        val.keys2 = OrderedDataVec::from_bytes::<O>(&data[info.bone_order_offset as usize..], info.bone_order_num as usize * 2)?;
        Ok(val)
    }

    pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], info: &HkConstraintInfo) -> Result<()> {
        self.shorts.to_bytes::<O>(&mut data[info.bone_parents_offset as usize..])?;
        self.string_offsets.to_bytes::<O>(&mut data[info.bone_names_offset as usize..])?;
        for (offset_, (string, offset, val)) in zip(&self.string_offsets, &self.strings) {
            offset.to_bytes::<O>(&mut data[*offset_ as usize..])?;
            val.to_bytes::<O>(&mut data[*offset_ as usize + u32::size::<O>()..])?;
            data[*offset as usize..*offset as usize+string.len()].copy_from_slice(string.as_bytes());
        }
        self.vals.to_bytes::<O>(&mut data[info.bone_transforms_offset as usize..])?;
        self.keys.to_bytes::<O>(&mut data[info.bones_offset as usize..])?;
        self.keys2.to_bytes::<O>(&mut data[info.bone_order_offset as usize..])?;
        Ok(())
    }
}

pub mod animation {
    use super::*;
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Clone, Serialize, Deserialize, PyMethods)]
    pub enum HkaSplineSkeletalAnimationObj1Types {
        Empty(),
        Type1(Vec<u8>),
        Type2(Vec<u16>),
    }

    impl Default for HkaSplineSkeletalAnimationObj1Types {
        fn default() -> Self { 
            Self::Empty()
        }
    }
    
    impl HkaSplineSkeletalAnimationObj1Types {
        pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize, num: usize, kind: u8) -> Result<Self> {
            Ok(match kind {
                0 | 2 =>  Self::Type1(OrderedDataVec::from_bytes::<O>(&data[offset..], num)?),
                1 | 3 =>  Self::Type2(OrderedDataVec::from_bytes::<O>(&data[offset..], num)?),
                _ => panic!("Illegal Type for spline thingy")
            })
        }
    
        pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], offset: usize) -> Result<()> {
            Ok(match self {
                Self::Type1(vals) => vals.to_bytes::<O>(&mut data[offset..])?,
                Self::Type2(vals) => vals.to_bytes::<O>(&mut data[offset..])?,
                _ => (),
            })
        }
    
        pub fn size<O: Version + 'static>(&self) -> usize {
            match self {
                Self::Type1(vals) => vals.size::<O>(),
                Self::Type2(vals) => vals.size::<O>(),
                _ => 0,
            }
        }
    }
    
    #[pyclass(module="pak.animation", set_all)]
    #[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
    pub struct HkaSplineSkeletalAnimationObj1 {
        #[pyo3(get)]
        pub nbytes: usize,
        #[pyo3(get)]
        pub s1: u16,
        #[pyo3(get)]
        pub s2: u8,
        pub data: Vec<u8>,
        #[pyo3(get)]
        pub vals_a: Vec<f32>,
        #[pyo3(get)]
        pub vals: HkaSplineSkeletalAnimationObj1Types,
    }

    #[basicpymethods]
    #[pymethods]
    impl HkaSplineSkeletalAnimationObj1 {
        #[getter]
        fn get_data(&self) -> std::borrow::Cow<[u8]> {
            std::borrow::Cow::from(&self.data[..])
        }
    }
    
    impl HkaSplineSkeletalAnimationObj1 {
        // const ITEM_SIZES: [usize; 4] = [1,2,1,2];
        const COUNTS: [usize; 8] = [0,1,1,2,1,2,2,3];
    
        pub fn from_data<O: Version + 'static>(data: &[u8], offset_: usize, flags: u8, kind: u8) -> Result<Self> {
            let mut val = Self::default();
            let mut offset = offset_;
            if flags != 0 {        
                if flags & 0xf0 == 0 {
                    val.s1 = 0;
                    val.s2 = 0;
                } else {
                    val.s1 = OrderedData::from_bytes::<O>(&data[offset..])?;
                    offset += u16::size::<O>();
                    val.s2 = OrderedData::from_bytes::<O>(&data[offset..])?;
                    offset += u8::size::<O>();
                    val.data = OrderedDataVec::from_bytes::<O>(&data[offset..], val.s1 as usize + val.s2 as usize + 2)?;
                    offset += val.data.size::<O>();
                }
                offset = (offset + 3) & 0xfffffffc;
    
                let num = Self::COUNTS[(flags & 7) as usize] + 2 * Self::COUNTS[(((flags >> 4) & !flags) & 7) as usize];
                val.vals_a = OrderedDataVec::from_bytes::<O>(&data[offset..], num)?;
                offset += val.vals_a.size::<O>();
    
                if flags & 0xf0 == 0 {
                    offset = (offset + 3) & 0xfffffffc;
                    val.nbytes = offset - offset_;
                    return Ok(val);
                }
    
                offset = (offset + 1) & 0xfffffffe;
    
                let num = Self::COUNTS[((flags >> 4) & 7) as usize] * (val.s1 as usize + 1);
                val.vals = HkaSplineSkeletalAnimationObj1Types::from_data::<O>(data, offset, num, kind)?;
                offset += val.vals.size::<O>();
            }
            offset = (offset + 3) & 0xfffffffc;
            val.nbytes = offset - offset_;
            Ok(val)
        }
    
        pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], offset: usize, flags: u8) -> Result<()> {
            let mut offset = offset;
            if flags == 0 { return Ok(()); }
            if flags & 0xf0 != 0 {
                self.s1.to_bytes::<O>(&mut data[offset..])?;
                offset += u16::size::<O>();
                self.s2.to_bytes::<O>(&mut data[offset..])?;
                offset += u8::size::<O>();
                self.data.to_bytes::<O>(&mut data[offset..])?;
                offset += self.data.size::<O>();
            }
            offset = (offset + 3) & 0xfffffffc;
    
            self.vals_a.to_bytes::<O>(&mut data[offset..])?;
            offset += self.vals_a.size::<O>();
    
            if flags & 0xf0 == 0 { return Ok(()); }
    
            offset = (offset + 3) & 0xfffffffc;
            self.vals.into_data::<O>(data, offset)
        }
    }
    
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    pub struct HkaSplineSkeletalAnimationObj2Type1 { a: u32 }
    
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    // should be (u8, u8, u8, u16) but for xbox conv it is (u8, u8, u8, u8, u8)
    pub struct HkaSplineSkeletalAnimationObj2Type2 { a: u8, b: u8, c: u8, d: u8, e: u8 }
    
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    pub struct HkaSplineSkeletalAnimationObj2Type3 { a: u16, b: u16, c: u16 }
    
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    pub struct HkaSplineSkeletalAnimationObj2Type4 { a: u8, b: u8, c: u8}
    
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    pub struct HkaSplineSkeletalAnimationObj2Type5 { a: u8, b: u8 }
    
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    pub struct HkaSplineSkeletalAnimationObj2Type6 { a: u32, b: u32, c: u32, d: u32 }
    
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Clone, Serialize, Deserialize, PyMethods)]
    pub enum HkaSplineSkeletalAnimationObj2Types{
        Empty(),
        Type1(Vec<HkaSplineSkeletalAnimationObj2Type1>),
        Type2(Vec<HkaSplineSkeletalAnimationObj2Type2>),
        Type3(Vec<HkaSplineSkeletalAnimationObj2Type3>),
        Type4(Vec<HkaSplineSkeletalAnimationObj2Type4>),
        Type5(Vec<HkaSplineSkeletalAnimationObj2Type5>),
        Type6(Vec<HkaSplineSkeletalAnimationObj2Type6>),
    }

    impl Default for HkaSplineSkeletalAnimationObj2Types {
        fn default() -> Self { 
            Self::Empty()
        }
    }
    
    impl HkaSplineSkeletalAnimationObj2Types {
        pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize, num: usize, kind: u8) -> Result<Self> {
            Ok(match kind {
                0 =>  Self::Type1(OrderedDataVec::from_bytes::<O>(&data[offset..], num)?),
                1 =>  Self::Type2(OrderedDataVec::from_bytes::<O>(&data[offset..], num)?),
                2 =>  Self::Type3(OrderedDataVec::from_bytes::<O>(&data[offset..], num)?),
                3 =>  Self::Type4(OrderedDataVec::from_bytes::<O>(&data[offset..], num)?),
                4 =>  Self::Type5(OrderedDataVec::from_bytes::<O>(&data[offset..], num)?),
                5 =>  Self::Type6(OrderedDataVec::from_bytes::<O>(&data[offset..], num)?),
                _ => panic!("Illegal Type for spline thingy")
            })
        }
    
        pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], offset: usize) -> Result<()> {
            match self {
                Self::Type1(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type2(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type3(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type4(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type5(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                Self::Type6(vals) => vals.to_bytes::<O>(&mut data[offset..]),
                _ => Ok(()),
            }
        }
    
        pub fn size<O: Version + 'static>(&self) -> usize {
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
    
    #[pyclass(module="pak.animation", set_all)]
    #[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
    pub struct HkaSplineSkeletalAnimationObj2 {
        #[pyo3(get)]
        pub nbytes: usize,
        #[pyo3(get)]
        pub align: u32,
        #[pyo3(get)]
        pub s1: u16,
        #[pyo3(get)]
        pub s2: u8,
        pub data: Vec<u8>,
        #[pyo3(get)]
        pub vals: HkaSplineSkeletalAnimationObj2Types,
    }

    #[basicpymethods]
    #[pymethods]
    impl HkaSplineSkeletalAnimationObj2 {
        #[getter]
        fn get_data(&self) -> std::borrow::Cow<[u8]> {
            std::borrow::Cow::from(&self.data[..])
        }
    }
    
    impl HkaSplineSkeletalAnimationObj2 {
        const ALIGNMENTS: [u32; 6] = [4, 1, 2, 1, 2, 4];
    
        pub fn from_data<O: Version + 'static>(data: &[u8], offset_: usize, flags: u8, kind: u8) -> Result<Self> {
            let mut val = Self::default();
            let mut offset = offset_;
            if flags != 0 {
                val.align = Self::ALIGNMENTS[kind as usize];
                if flags & 0xf0 == 0 {
                    val.s1 = 0;
                    val.s2 = 0;
                } else {
                    val.s1 = OrderedData::from_bytes::<O>(&data[offset..])?;
                    offset += u16::size::<O>();
                    val.s2 = OrderedData::from_bytes::<O>(&data[offset..])?;
                    offset += u8::size::<O>();
                    val.data = OrderedDataVec::from_bytes::<O>(&data[offset..], val.s1 as usize + val.s2 as usize + 2)?;
                    offset += val.data.size::<O>();
                }
    
                offset = ((offset as u32 + val.align - 1) & !(val.align - 1)) as usize;
                val.vals = HkaSplineSkeletalAnimationObj2Types::from_data::<O>(data, offset, val.s1 as usize + 1, kind)?;
                offset += val.vals.size::<O>();   
            }
            offset = (offset + 3) & 0xfffffffc;
            val.nbytes = offset - offset_;
            Ok(val)
        }
    
        pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], offset: usize, flags: u8) -> Result<()> {
            let mut offset = offset;
            if flags == 0 { return Ok(()); }
            if flags & 0xf0 != 0 {
                self.s1.to_bytes::<O>(&mut data[offset..])?;
                offset += u16::size::<O>();
                self.s2.to_bytes::<O>(&mut data[offset..])?;
                offset += u8::size::<O>();
                self.data.to_bytes::<O>(&mut data[offset..])?;
                offset += self.data.size::<O>();
            }
    
            offset = ((offset as u32 + self.align - 1) & !(self.align - 1)) as usize;
            self.vals.into_data::<O>(data, offset)
        }
    }
    
    
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    pub struct HkaSplineSkeletalAnimationFlags{
        pub f: u8, 
        pub a: u8, 
        pub b: u8, 
        pub c: u8,
    }
    
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
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
        pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize, info: &AnimationInfo) -> Result<Self> {
            let mut val = Self::default();
            val.block_starts = OrderedDataVec::from_bytes::<O>(&data[offset + info.block_starts_offset as usize..], info.block_starts_num as usize)?;
            val.block_starts2 = OrderedDataVec::from_bytes::<O>(&data[offset + info.block_starts2_offset as usize..], info.block_starts2_num as usize)?;
            val.obj_c3 = OrderedDataVec::from_bytes::<O>(&data[offset + info.obj_c3_offset as usize..], info.obj_c3_num as usize)?;
            val.obj_c4 = OrderedDataVec::from_bytes::<O>(&data[offset + info.obj_c4_offset as usize..], info.obj_c4_num as usize)?;
            for (start, start2) in zip(&val.block_starts,&val.block_starts2) {
                let off = offset + (start + info.block_offset) as usize;
                let flags: Vec<HkaSplineSkeletalAnimationFlags> = OrderedDataVec::from_bytes::<O>(&data[off..], info.keys_num as usize)?;
                let flags2: Vec<u8> = OrderedDataVec::from_bytes::<O>(&data[off + flags.size::<O>()..], info.keys2_num as usize)?;
                let mut off = offset + (info.block_offset + start + info.data_offset) as usize;
                let mut vals_a = Vec::with_capacity(flags.len());
                let mut vals_b = Vec::with_capacity(flags.len());
                let mut vals_c = Vec::with_capacity(flags.len());
                let mut vals_d = Vec::with_capacity(flags2.len());
                for flag in &flags {
                    let a = HkaSplineSkeletalAnimationObj1::from_data::<O>(data, off, flag.a, flag.f & 3)?;
                    off += a.nbytes;
                    let b = HkaSplineSkeletalAnimationObj2::from_data::<O>(data, off, flag.b, (flag.f >> 2) & 0xf)?;
                    off += b.nbytes;
                    let c = HkaSplineSkeletalAnimationObj1::from_data::<O>(data, off, flag.c, (flag.f >> 6) & 3)?;
                    off += c.nbytes;
                    vals_a.push(a);
                    vals_b.push(b);
                    vals_c.push(c);
                }
                off = offset + (info.block_offset + start + start2) as usize;
                for flag in &flags2 {
                    let d: HkaSplineSkeletalAnimationObj1 = HkaSplineSkeletalAnimationObj1::from_data::<O>(data, off, flag & 0xf9, (flag >> 1) & 3)?;
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
            Ok(val)
        }
    
        pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], offset: usize, info: &AnimationInfo) -> Result<()> {
            self.block_starts.to_bytes::<O>(&mut data[offset + info.block_starts_offset as usize..])?;
            self.block_starts2.to_bytes::<O>(&mut data[offset + info.block_starts2_offset as usize..])?;
            self.obj_c3.to_bytes::<O>(&mut data[offset + info.obj_c3_offset as usize..])?;
            self.obj_c4.to_bytes::<O>(&mut data[offset + info.obj_c4_offset as usize..])?;
            for (((start, start2), (flags, flags2)), ((vals_a, vals_b), (vals_c, vals_d))) in zip(zip(zip(&self.block_starts, &self.block_starts2), zip(&self.flags, &self.flags2)), zip(zip(&self.vals_a, &self.vals_b), zip(&self.vals_c, &self.vals_d))) {
                flags.to_bytes::<O>(&mut data[offset + (start + info.block_offset) as usize..])?;
                flags2.to_bytes::<O>(&mut data[offset + (start + info.block_offset) as usize + flags.size::<O>()..])?;
                let mut off = offset + (info.block_offset + start + info.data_offset) as usize;
                for ((flag, a), (b, c)) in zip(zip(flags, vals_a), zip(vals_b, vals_c)) {
                    a.into_data::<O>(data, off, flag.a)?;
                    off += a.nbytes;
                    b.into_data::<O>(data, off, flag.b)?;
                    off += b.nbytes;
                    c.into_data::<O>(data, off, flag.c)?;
                    off += c.nbytes;
                }
                off = offset + (info.block_offset + start + start2) as usize;
                for (flag, d) in zip(flags2, vals_d) {
                    d.into_data::<O>(data, off, flag & 0xf9)?;
                    off += d.nbytes;
                }
            }
            Ok(())
        }
    }
    
    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
    pub struct Obj5Header {
        pub obj_a_num: u32,
        pub obj_a_offset: u32,
        pub obj_b_num: u32,
        pub obj_b_offset: u32,
    }

    #[basicpymethods]
    #[pyclass(module="pak.animation", get_all, set_all)]
    #[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
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
    pub fn unpack_from_block<O: Version + 'static>(&mut self, data: &[u8], offset: usize, index: usize, info: &AnimationInfo) -> Result<()> {
        self.obj1.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.obj1_offset as usize..], info.obj1_num as usize * 2)?);
        self.obj2.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.obj2_offset as usize..], info.obj2_num as usize * 4)?);
        self.obj3.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.obj3_offset as usize..], info.obj3_num as usize)?);
        self.keys.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + info.keys_offset as usize..], (info.keys_num + info.obj1_num) as usize)?);
        if info.obj5_offset != 0 {
            let obj5_header: animation::Obj5Header = OrderedData::from_bytes::<O>(&data[offset + info.obj5_offset as usize..])?;
            self.obj5_a.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + obj5_header.obj_a_offset as usize..], obj5_header.obj_a_num as usize * 7)?);
            self.obj5_b.insert(index, OrderedDataVec::from_bytes::<O>(&data[offset + obj5_header.obj_b_offset as usize..], obj5_header.obj_b_num as usize * 7)?);
            self.obj5_header.insert(index, obj5_header);
        }
        if info.kind == 3 {
            self.obj_c.insert(index, animation::HkaSplineSkeletalAnimation::from_data::<O>(data, offset, info)?);
        } else if info.kind < 3 {
            warn!("Unhandled animation type {}", info.kind);
        } else {
            warn!("Unknown animation type {}", info.kind);
        }
        Ok(())
    }

    pub fn pack_into_block<O: Version + 'static>(&self, data: &mut [u8], offset: usize, index: usize, info: &AnimationInfo) -> Result<()> {
        self.obj1.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.obj1_offset as usize..])?;
        self.obj2.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.obj2_offset as usize..])?;
        self.obj3.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.obj3_offset as usize..])?;
        self.keys.get(&index).unwrap().to_bytes::<O>(&mut data[offset + info.keys_offset as usize..])?;
        if info.obj5_offset != 0 {
            let obj5_header = self.obj5_header.get(&index).unwrap();
            obj5_header.to_bytes::<O>(&mut data[offset + info.obj5_offset as usize..])?;
            self.obj5_a.get(&index).unwrap().to_bytes::<O>(&mut data[offset + obj5_header.obj_a_offset as usize..])?;
            self.obj5_b.get(&index).unwrap().to_bytes::<O>(&mut data[offset + obj5_header.obj_b_offset as usize..])?;
        }
        if info.kind == 3 {
            self.obj_c.get(&index).unwrap().into_data::<O>(data, offset, info)?;
        }
        Ok(())
    }

    pub fn unpack_block<O: Version + 'static>(anims: &mut [Self], infos: &[AnimationInfo], data: & [u8], offset: usize, index: usize) -> Result<()> {
        let mut offset = offset;
        for (anim, info) in zip(anims, infos) {
            let gamemodemask = 1i32 << index;
            if gamemodemask & info.gamemodemask != 0 {
                anim.unpack_from_block::<O>(data, offset, index, info)?;
                offset += info.size as usize;
            }
        }
        Ok(())
    }

    pub fn pack_block<O: Version + 'static>(anims: & [Self], infos: &[AnimationInfo], data: &mut [u8], offset: usize, index: usize) -> Result<()> {
        let mut offset = offset;
        for (anim, info) in zip(anims, infos) {
            let gamemodemask = 1i32 << index;
            if gamemodemask & info.gamemodemask != 0 {
                anim.pack_into_block::<O>(data, offset, index, info)?;
                offset += info.size as usize;
            }
        }
        Ok(())
    }
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all, eq, hash)]
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Hash, PyMethods)]
pub enum VertexUsage {
    Position(),
    Normal(),
    Tangent(),
    BiNormal(),
    BlendWeight(),
    BlendIndices(),
    TextureCoord(usize),
    Unknown(usize),
    PSize(),
    Pad(usize),
}

impl Default for VertexUsage {
    fn default() -> Self {
        Self::Unknown(0xFFFFFFFF)
    }
}

impl Eq for VertexUsage {}

impl Display for VertexUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Position() => write!(f, "Position"),
            Self::Normal() => write!(f, "Normal"),
            Self::Tangent() => write!(f, "Tangent"),
            Self::BiNormal() => write!(f, "BiNormal"),
            Self::BlendWeight() => write!(f, "BlendWeight"),
            Self::BlendIndices() => write!(f, "BlendIndices"),
            Self::TextureCoord(i) => write!(f, "TextureCoord({})", i),
            Self::Unknown(i) => write!(f, "Unknown({})", i),
            Self::PSize() => write!(f, "PSize"),
            Self::Pad(i) => write!(f, "Pad({})", i),
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
            "Position" => Ok(Self::Position()),
            "Normal" => Ok(Self::Normal()),
            "Tangent" => Ok(Self::Tangent()),
            "BiNormal" => Ok(Self::BiNormal()),
            "BlendWeight" => Ok(Self::BlendWeight()),
            "BlendIndices" => Ok(Self::BlendIndices()),
            "PSize" => Ok(Self::PSize()),
            s => {
                if s.starts_with("TextureCoord(") {
                    Ok(s[13..].split(')').next().unwrap().parse::<usize>().map(|i| Self::TextureCoord(i))?)
                } else if s.starts_with("Unknown(") {
                    Ok(s[8..].split(')').next().unwrap().parse::<usize>().map(|i| Self::Unknown(i))?)
                } else if s.starts_with("Pad(") {
                    Ok(s[4..].split(')').next().unwrap().parse::<usize>().map(|i| Self::Pad(i))?)
                } else {
                    Err(VertexUsageParseError)
                }
            }
        }
    }
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Clone, Serialize, Deserialize, PyMethods)]
pub enum VertexTypes {
    Vector2(Vec<f32>,Vec<f32>),
    Vector3(Vec<f32>,Vec<f32>,Vec<f32>),
    Vector4(Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>),
    Unorm4x8(Vec<u32>),
    Pad(Vec<u32>),
    None(),
}

impl Default for VertexTypes {
    fn default() -> Self {
        Self::None()
    }
}

impl VertexTypes {
    pub fn new(format: u32) -> Self {
        match format {
            BaseTypes::INT_KEY => Self::Pad(vec![]),
            BaseTypes::COLOR_KEY => Self::Unorm4x8(vec![]),
            BaseTypes::VECTOR2_KEY => Self::Vector2(vec![], vec![]),
            BaseTypes::VECTOR3_KEY => Self::Vector3(vec![], vec![], vec![]),
            BaseTypes::VECTOR4_KEY => Self::Vector4(vec![], vec![], vec![], vec![]),
            _ => Self::None()
        }
    }

    pub fn ty(&self) -> u32 {
        match self {
            Self::Pad(..) => BaseTypes::INT_KEY,
            Self::Unorm4x8(..) => BaseTypes::COLOR_KEY,
            Self::Vector2(..) => BaseTypes::VECTOR2_KEY,
            Self::Vector3(..) => BaseTypes::VECTOR3_KEY,
            Self::Vector4(..) => BaseTypes::VECTOR4_KEY,
            Self::None() => 0,
        }
    }

    pub fn get(&self, i: usize) -> BaseTypes {
        match self {
            Self::Pad(vals) => BaseTypes::Color(Color(vals[i])),
            Self::Unorm4x8(vals) => BaseTypes::Color(Color(vals[i])),
            Self::Vector2(x, y) => BaseTypes::Vector2(Vector2 {x: x[i], y: y[i]}),
            Self::Vector3(x, y, z) => BaseTypes::Vector3(Vector3 {x: x[i], y: y[i], z: z[i]}),
            Self::Vector4(x, y, z, w) => BaseTypes::Vector4(Vector4 {x: x[i], y: y[i], z: z[i], w: w[i]}),
            Self::None() => BaseTypes::Int(0)
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Pad(vals) => vals.len(),
            Self::Unorm4x8(vals) => vals.len(),
            Self::Vector2(x, y) => x.len().min(y.len()),
            Self::Vector3(x, y, z) => x.len().min(y.len()).min(z.len()),
            Self::Vector4(x, y, z, w) => x.len().min(y.len()).min(z.len()).min(w.len()),
            Self::None() => 0
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Pad(..) => 4,
            Self::Unorm4x8(..) => 4,
            Self::Vector2(..) => 8,
            Self::Vector3(..) => 12,
            Self::Vector4(..) => 16,
            Self::None() => 0
        }
    }

    pub fn push(&mut self, val: BaseTypes) {
        match self {
            Self::Pad(vals) => if let BaseTypes::Color(val) = val {
                vals.push(val.0);
            } else { panic!("VertexTypes Mismatch") },
            Self::Unorm4x8(vals) => if let BaseTypes::Color(val) = val {
                vals.push(val.0);
            } else { panic!("VertexTypes Mismatch") },
            Self::Vector2(x, y)=> if let BaseTypes::Vector2(val) = val {
                x.push(val.x); y.push(val.y);
            } else { panic!("VertexTypes Mismatch") },
            Self::Vector3(x, y, z)=> if let BaseTypes::Vector3(val) = val {
                x.push(val.x); y.push(val.y); z.push(val.z);
            } else { panic!("VertexTypes Mismatch") },
            Self::Vector4(x, y, z, w) => if let BaseTypes::Vector4(val) = val {
                x.push(val.x); y.push(val.y); z.push(val.z); w.push(val.w);
            } else { panic!("VertexTypes Mismatch") },
            Self::None() => ()
        }
    }
}

//#[serde_as]
#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PyMethods)]
pub struct VertexData {
    #[serde(rename="$key$")]
    //#[serde_as(as="serde_with::DisplayFromStr")]
    pub usage: VertexUsage,
    pub val: VertexTypes,
}

#[derive(Debug)]
pub struct VertexDataParseError;
impl Display for VertexDataParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VertexDataParseError")
    }
}

impl VertexData {
    pub fn new(format: u32, usage: VertexUsage) -> Self {
        Self {
            usage,
            val: VertexTypes::new(format)
        }
    }

    pub fn dump_bytes<O: Version + 'static>(&self) -> Vec<u8> {
        match self {
            Self { usage: VertexUsage::BlendWeight(), val: VertexTypes::Unorm4x8(vals) } => vals.iter().map(|x| ((x & 0xFF0000) >> 16) | ((x & 0xFF) << 16) | (x & 0xFF00FF00)).collect::<Vec<_>>().dump_bytes::<O>(),
            Self { val: VertexTypes::Pad(vals), .. } => vals.dump_bytes::<O>(),
            Self { val: VertexTypes::Unorm4x8(vals), .. } => vals.dump_bytes::<O>(),
            Self { val: VertexTypes::Vector2(x, y), .. } => x.iter().zip(y).map(|(&x, &y)| Vector2{x,y}).collect::<Vec<_>>().dump_bytes::<O>(),
            Self { val: VertexTypes::Vector3(x, y, z), .. } => x.iter().zip(y.iter().zip(z)).map(|(&x, (&y, &z))| Vector3{x,y,z}).collect::<Vec<_>>().dump_bytes::<O>(),
            Self { val: VertexTypes::Vector4(x, y, z, w), .. } => x.iter().zip(y.iter().zip(z.iter().zip(w))).map(|(&x, (&y, (&z, &w)))| Vector4{x,y,z,w}).collect::<Vec<_>>().dump_bytes::<O>(),
            Self { val: VertexTypes::None(), .. } => vec![],
        }
    }

    pub fn gltf_data(&self) -> Vec<u8> {
        fn conv_val(val: u32) -> u32 {
            if val < 127 {
                129 + val
            } else {
                val - 127
            }
        }
        match self {
            Self { usage: VertexUsage::Normal(), val: VertexTypes::Unorm4x8(vals) } => {
                vals.iter().map(|x|
                    conv_val((x >> 24) & 0xff) << 24 | 
                    conv_val((x >> 16) & 0xff) << 16 | 
                    conv_val((x >> 8) & 0xff) << 8 | 
                    conv_val(x & 0xff)
                ).collect::<Vec<_>>().dump_bytes::<PC>()
            },
            _ => self.dump_bytes::<PC>()
        }
    }

    
    pub fn from_gltf(&mut self, mut data: GltfData) -> Result<(), VertexDataParseError> {
        use gltf::accessor::DataType;
        fn conv_val(val: u32) -> u32 {
            if val > 128 {
                val - 129
            } else if val < 128 {
                val + 127
            } else {
                0
            }
        }

        match self {
            Self { usage: VertexUsage::BlendWeight(), val: VertexTypes::Unorm4x8(vals) } => {
                assert!(data.ty == DataType::U8);
                assert!(data.m == 4);
                data.ty = DataType::U32;
                data.m = 1;
                vals.extend(data.u32().unwrap()
                    .into_iter().map(|x| ((x & 0xFF0000) >> 16) | ((x & 0xFF) << 16) | (x & 0xFF00FF00))
                );
            },
            Self { usage: VertexUsage::Normal(), val: VertexTypes::Unorm4x8(vals) } => {
                assert!(data.ty == DataType::I8);
                assert!(data.m == 3);
                data.ty = DataType::U32;
                data.m = 1;
                vals.extend(data.u32().unwrap().iter().map(|x|
                    conv_val((x >> 24) & 0xff) << 24 | 
                    conv_val((x >> 16) & 0xff) << 16 | 
                    conv_val((x >> 8) & 0xff) << 8 | 
                    conv_val(x & 0xff)
                ));
            },
            Self { usage: VertexUsage::Normal() | VertexUsage::Position(), val: VertexTypes::Vector4(x,y,z,w) } => {
                assert!(data.ty == DataType::F32);
                assert!(data.m == 3);
                data.m = 4;
                data.f32().unwrap().as_slice().chunks_exact(4).for_each(|val| {
                    x.push(val[0]); y.push(val[1]); z.push(val[2]); w.push(val[3]); 
                });
            },
            Self { val: VertexTypes::Pad(vals), .. } => vals.extend(data.u32().unwrap()),
            Self { val: VertexTypes::Unorm4x8(vals), .. } => {
                assert!(data.ty == DataType::U8);
                assert!(data.m == 4);
                data.ty = DataType::U32;
                data.m = 1;
                vals.extend(data.u32().unwrap());
            }
            Self { val: VertexTypes::Vector2(x,y), .. } => data.f32().unwrap().as_slice().chunks_exact(2)
                .for_each(|val| { x.push(val[0]); y.push(val[1]); }),
            Self { val: VertexTypes::Vector3(x,y,z), .. } => data.f32().unwrap().as_slice().chunks_exact(3)
                .for_each(|val| { x.push(val[0]); y.push(val[1]); z.push(val[2]); }),
            Self { val: VertexTypes::Vector4(x,y,z,w), .. } => data.f32().unwrap().as_slice().chunks_exact(4)
                .for_each(|val| { x.push(val[0]); y.push(val[1]); z.push(val[2]); w.push(val[3]); }),
            _ => Err(VertexDataParseError)?
        }
        Ok(())
    }

    pub fn min(&self) -> Option<serde_json::value::Value> {
        use serde_json::json;
        match self {
            Self { usage: VertexUsage::Position(), val: VertexTypes::Vector3(x,y,z) } => Some(json!(vec![
                    x.iter().copied().reduce(f32::min), 
                    y.iter().copied().reduce(f32::min), 
                    z.iter().copied().reduce(f32::min)
            ])),
            Self { usage: VertexUsage::Position(), val: VertexTypes::Vector4(x,y,z,..) } => Some(json!(vec![
                    x.iter().copied().reduce(f32::min), 
                    y.iter().copied().reduce(f32::min), 
                    z.iter().copied().reduce(f32::min),
            ])),
            _ => None
        }
    }

    pub fn max(&self) -> Option<serde_json::value::Value> {
        use serde_json::json;
        match self {
            Self { usage: VertexUsage::Position(), val: VertexTypes::Vector3(x,y,z) } => Some(json!(vec![
                    x.iter().copied().reduce(f32::max), 
                    y.iter().copied().reduce(f32::max), 
                    z.iter().copied().reduce(f32::max)
            ])),
            Self { usage: VertexUsage::Position(), val: VertexTypes::Vector4(x,y,z,..) } => Some(json!(vec![
                    x.iter().copied().reduce(f32::max), 
                    y.iter().copied().reduce(f32::max), 
                    z.iter().copied().reduce(f32::max),
            ])),
            _ => None
        }
    }

    pub fn component_type(&self) -> gltf::accessor::DataType {
        use gltf::accessor::DataType;
        match self {
            Self { usage: VertexUsage::Normal(), val: VertexTypes::Unorm4x8(..) } => DataType::I8,
            Self { val: VertexTypes::Unorm4x8(..), .. } => DataType::U8,
            Self { val: VertexTypes::Pad(..), .. } => DataType::U32,
            Self { val: VertexTypes::Vector2(..), .. } => DataType::F32,
            Self { val: VertexTypes::Vector3(..), .. } => DataType::F32,
            Self { val: VertexTypes::Vector4(..), .. } => DataType::F32,
            _ => DataType::U32,
        }
    }

    pub fn dimensions(&self) -> gltf::accessor::Dimensions {
        use gltf::accessor::Dimensions;
        match self {
            Self { usage: VertexUsage::Normal(), val: VertexTypes::Unorm4x8(..) } => Dimensions::Vec3,
            Self { usage: VertexUsage::Normal() | VertexUsage::Position(), val: VertexTypes::Vector4(..) } => Dimensions::Vec3,
            Self { val: VertexTypes::Unorm4x8(..), .. } => Dimensions::Vec4,
            Self { val: VertexTypes::Pad(..), .. } => Dimensions::Scalar,
            Self { val: VertexTypes::Vector2(..), .. } => Dimensions::Vec2,
            Self { val: VertexTypes::Vector3(..), .. } => Dimensions::Vec3,
            Self { val: VertexTypes::Vector4(..), .. } => Dimensions::Vec4,
            _ => Dimensions::Scalar,
        }
    }

    pub fn stride(&self) -> Option<usize> {
        match self {
            Self { usage: VertexUsage::Normal(), val: VertexTypes::Unorm4x8(..) } => Some(4),
            Self { usage: VertexUsage::Normal() | VertexUsage::Position(), val: VertexTypes::Vector4(..) } => Some(16),
            _ => None,
        }
    }
    
    pub fn normalized(&self) -> bool {
        match self {
            Self { usage: VertexUsage::Normal() | VertexUsage::BlendWeight(), val: VertexTypes::Unorm4x8(..) } => true,
            _ => false
        }
    }
}

pub fn get_vertex_format<O: Version + 'static>(fmt1: u32, fmt2: u32) -> (Vec<VertexData>, usize) {
    let mut fmt = Vec::new();
    let mut s = 0;
    if fmt2 == 0 {
        let b1: bool = (fmt1 & 0x40000) != 0;
        if fmt1 & 1 != 0 {
            fmt.push(VertexData::new(if b1 {BaseTypes::VECTOR4_KEY} else {BaseTypes::VECTOR3_KEY}, VertexUsage::Position()));
            s += if b1 {16} else {12};
        }
        if (fmt1 & 0x400) != 0 {
            // blend weights
            if b1 {
                fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Unknown(3)));
            } else {
                fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::BlendWeight()));
            }
            s += 4;
        }
        if (fmt1 & 0x800) != 0 {
            // blend indices
            if b1 {
                fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Unknown(4)));
            } else {
                fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::BlendIndices()));
            }
            s += 4;
            // if b1 then Vec4 ?? 
        }
        if (fmt1 & 2) != 0 {
            // binorm ?
            let usage = VertexUsage::Unknown(0);
            if b1 {
                for _ in (0..(((s + 15) & 0xFFFF0) - s)).step_by(4) {
                    fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Pad(s)));
                    s += 4;
                }
                fmt.push(VertexData::new(BaseTypes::VECTOR4_KEY, usage));
                s += 16;
            } else if TypeId::of::<O>() == TypeId::of::<PS3>() {
                fmt.push(VertexData::new(BaseTypes::VECTOR3_KEY, usage));
                s += 12;
            } else {
                fmt.push(VertexData::new(BaseTypes::COLOR_KEY, usage));
                s += 4;
            }
        }
        if fmt1 & 0x100 != 0 {
            fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Unknown(1)));
            s += 4;
        }
        if fmt1 & 0x200 != 0 {
            fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Unknown(2)));
            s += 4;
        }
        for i in 0..((fmt1 >> 2) & 0xF) {
            // texture coords
            fmt.push(VertexData::new(BaseTypes::VECTOR2_KEY, VertexUsage::TextureCoord(i as usize)));
            s += 8;
        }
        if fmt1 & 0x40 != 0 {
            if b1 {
                for _ in (0..(((s + 15) & 0xFFFF0) - s)).step_by(4) {
                    fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Pad(s)));
                    s += 4;
                }
                fmt.push(VertexData::new(BaseTypes::VECTOR4_KEY, VertexUsage::Normal()));
                s += 16;
            } else {
                fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Normal()));
                s += 4;
            }
        }
        if fmt1 & 0x80 != 0 {
            fmt.push(VertexData::new(BaseTypes::VECTOR3_KEY, VertexUsage::PSize()));
            s += 12;
        }
        if b1 {
            for _ in (0..(((s + 15) & 0xFFFF0) - s)).step_by(4) {
                fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Pad(s)));
                s += 4;
            }
        }
    } else {
        if fmt1 & 1 != 0 {
            fmt.push(VertexData::new(BaseTypes::VECTOR3_KEY, VertexUsage::Position()));
            s += 12;
        }
        if fmt1 & 0x400 != 0 {
            fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::BlendWeight()));
            s += 4;
        }
        if fmt1 & 0x800 != 0 {
            fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::BlendIndices()));
            s += 4;
        }
        if fmt1 & 2 != 0{
            fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Unknown(0)));
            s += 4;
        }
        if fmt1 & 0x100 != 0 {
            fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Unknown(1)));
            s += 4;
        }
        if fmt1 & 0x200 != 0 {
            fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Unknown(2)));
            s += 4;
        }
        let n = (fmt1 >> 2) & 0xf;
        if n <= 2 {
            for i in 0..n {
                fmt.push(VertexData::new(BaseTypes::VECTOR2_KEY, VertexUsage::TextureCoord(i as usize)));
                s += 8;
            }
        }
        if fmt1 & 0x40 != 0 {
            fmt.push(VertexData::new(BaseTypes::COLOR_KEY, VertexUsage::Normal()));
            s += 4;
        }
        if fmt1 & 0x80 != 0 {
            fmt.push(VertexData::new(BaseTypes::VECTOR3_KEY, VertexUsage::PSize()));
            s += 12;
        }
    }
    (fmt, s)
}

#[serde_as]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VertexBuffer {
    //#[serde_as(as = "serde_with::Map<serde_with::DisplayFromStr, _>")]
    #[serde_as(as="serde_with::KeyValueMap<_>")]
    pub vals: Vec<VertexData>
}

impl IntoPy<PyObject> for VertexBuffer {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.vals.into_py(py)
    }
}

impl <'py> FromPyObject<'py> for VertexBuffer {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self { vals: Vec::extract_bound(ob)? })
    }
}

impl VertexBuffer {
    pub fn from_data<O: Version + 'static>(data: &[u8], info: &mut VBuffInfo, formats: &mut HashMap<(u32, u32), (Vec<VertexData>, usize)>) -> Result<Self> {
        let (fmt, size) = formats.entry((info.fmt1, info.fmt2)).or_insert_with(|| {
            get_vertex_format::<O>(info.fmt1, info.fmt2)
        });
        if info.size as usize % *size != 0 {
            warn!("Vertex Buffer size is not a multiple of assumed size");
        }
        let n = info.size as usize / *size;
        let mut offset = info.offset as usize;
        let mut vals = fmt.clone();
        for _ in 0..n {
            // let mut val = Vec::with_capacity(fmt.len());
            for val in &mut vals {
                let v = BaseTypes::from_data::<O>(&data[offset..], val.val.ty())?;
                offset += v.size::<O>();
                val.val.push(v);
            }
        }
        if TypeId::of::<O>() == TypeId::of::<XBOX>() {
            if (info.fmt1 & 0x80000 != 0) & (info.fmt1 & 0x400 == 0) {
                info.fmt1 |= 0x400;
                let (fmt, _) = formats.entry((info.fmt1, info.fmt2)).or_insert_with(|| {
                    get_vertex_format::<O>(info.fmt1, info.fmt2)
                });
                let mut vals_new = fmt.clone();
                let mut binorm = Vec::with_capacity(n);
                let mut tan = Vec::with_capacity(n);
                for val in &mut vals {
                    match val {
                        VertexData { 
                            usage: VertexUsage::BlendIndices(), 
                            val: VertexTypes::Unorm4x8(val),
                        } => {
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
                        _ => (),
                    }
                }
                for val in &mut vals_new {
                    match val {
                        VertexData { 
                            usage: VertexUsage::BlendIndices(), val
                        } => *val = VertexTypes::Unorm4x8(binorm.clone()),
                        VertexData { 
                            usage: VertexUsage::BlendWeight(), val
                        } => *val = VertexTypes::Unorm4x8(tan.clone()),
                        _ => for val2 in &vals {
                            if val.usage == val2.usage {
                                val.val = val2.val.clone();
                            }
                        }
                    }
                }
                vals = vals_new;
            }
            for val in &mut vals {
                match val {
                    VertexData {
                        usage: VertexUsage::Unknown(0), val: VertexTypes::Vector4(x, y, z, ..)
                    } => {
                        x.iter_mut().for_each(|x| *x = *x/2.0 + 0.5);
                        y.iter_mut().for_each(|x| *x = *x/2.0 + 0.5);
                        z.iter_mut().for_each(|x| *x = *x/2.0 + 0.5);
                    },
                    VertexData {
                        usage: VertexUsage::Unknown(0), val: VertexTypes::Unorm4x8(v)
                    } => v.iter_mut().for_each(|val| {
                        let z_ = ((*val) & 0x3FF) ^ 0x200;
                        let y_ = (((*val) >> 10) & 0x3FF) ^ 0x200;
                        let x_ = (((*val) >> 20) & 0x3FF) ^ 0x200;
                        let x: u32 = (x_ as f32 - 4.0f32).div(4.0f32).round_ties_even().clamp(0.0, 255.0) as u32;
                        let y: u32 = (y_ as f32 - 4.0f32).div(4.0f32).round_ties_even().clamp(0.0, 255.0) as u32;
                        let z: u32 = (z_ as f32 - 4.0f32).div(4.0f32).round_ties_even().clamp(0.0, 255.0) as u32;
                        *val = (127 << 24) | (z << 16) | (y << 8) | x;
                    }),
                    VertexData { usage: VertexUsage::Unknown(0), .. } => panic!("Unexpected vertex type for Unknown(0)"),
                    _ => (),
                }
            }
        }
        Ok(Self { vals })
    }
    
    #[allow(dead_code)]
    pub fn into_data<O: Version + 'static>(&self, data: &mut[u8], info: &VBuffInfo) -> Result<()> {
        let mut offset = info.offset as usize;
        let mut off_ = 0;
        let i = self.vals.iter().map(|x| x.val.len()).min().unwrap();
        for VertexData { val, .. } in &self.vals {
            assert!(val.len() == i);
        }
        for i in 0..i {
            for VertexData { val, .. } in &self.vals {
                let v = val.get(i);
                v.into_data::<O>(&mut data[offset..], &mut off_)?;
                offset += v.size::<O>();
            }
        }
        Ok(())
    }

    pub fn dump<O: Version + 'static>(&self) -> Vec<u8> {
        // self.vals.iter().flat_map(|x| x.iter().flat_map(|x| x.dump_bytes::<O>())).collect()
        let i = self.vals.iter().map(|x| x.val.len()).min().unwrap();
        (0..i).flat_map(|i| self.vals.iter().flat_map(move |val| val.val.get(i).dump_bytes::<O>())).collect()
    }
}

#[basicpymethods]
#[pyclass(module="pak", get_all, set_all)]
#[derive(Debug, Clone, Serialize, Deserialize, PyMethods)]
pub enum IndexBuffer {
    U16 { vals: Vec<u16> },
    U32 { vals: Vec<u32> },
}

impl IndexBuffer {
    pub fn from_data<O: Version + 'static>(data: &[u8], info: &IBuffInfo) -> Result<Self> {
        let size = match info.format {
            0x10 => u16::size::<O>(),
            _ => u32::size::<O>(),
        };
        assert!(info.size as usize % size == 0);
        let mut n = info.size as usize / size;
        if data.len() < (info.offset + info.size) as usize {
            warn!("Index buffer of size {} only has data of size {} available", info.size, data.len() - info.offset as usize);
            n = (data.len() - info.offset as usize) / size;
        }
        Ok(match info.format {
            0x10 => Self::U16 { vals: OrderedDataVec::from_bytes::<O>(&data[info.offset as usize..], n)? },
            _ => Self::U32 { vals: OrderedDataVec::from_bytes::<O>(&data[info.offset as usize..], n)? },
        })
    }
    #[allow(dead_code)]
    pub fn into_data<O: Version + 'static>(&self, data: &mut [u8]) -> Result<()> {
        match self {
            Self::U16 { vals } => vals.to_bytes::<O>(data),
            Self::U32 { vals } => vals.to_bytes::<O>(data)
        }
    }
    pub fn dump<O: Version + 'static>(&self) -> Vec<u8> {
        match self {
            Self::U16 { vals } => vals.dump_bytes::<O>(),
            Self::U32 { vals } => vals.dump_bytes::<O>()
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
// list of ints pointing to the radiosity data
// has a value for each mesh in the model 
// corresponding to the object pointed to by the guid
// (value if offset to consecutive values in radiosity, one for each vertex of the mesh)
// a value of -1 means no radiosity for that mesh
pub struct RadiosityVals {
    pub vals: Vec<i32>
}

impl IntoPy<PyObject> for RadiosityVals {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.vals.into_py(py)
    }
}

impl <'py> FromPyObject<'py> for RadiosityVals {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self { vals: Vec::extract_bound(ob)? })
    }
}


impl RadiosityVals {
    pub fn from_data<O: Version + 'static>(data: &[u8], info: &RadiosityValsInfo) -> Result<Self> {
        Ok(Self { vals: OrderedDataVec::from_bytes::<O>(&data[info.offset as usize..], info.num as usize)? })
    }

    pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], info: &RadiosityValsInfo) -> Result<()> {
        self.vals.to_bytes::<O>(&mut data[info.offset as usize..])
    }
}

type FoliageValProxy = (u16, u16, i16, i16);
#[derive(Debug, Clone, Default, Serialize, Deserialize, OrderedData)]
#[serde(from = "FoliageValProxy", into = "FoliageValProxy")]
// height: u16, var_mask: u16, slope_x: i16, slope_z: i16
// position and orientation are randomized for each instance (within own square)
// var mask only checks high component vs alpha of vertex attr (if it exists)
pub struct FoliageVal(u16, u16, i16, i16);

impl From<FoliageVal> for FoliageValProxy {
    fn from(value: FoliageVal) -> Self {
        (value.0, value.1, value.2, value.3)
    }
}

impl From<FoliageValProxy> for FoliageVal {
    fn from(value: FoliageValProxy) -> Self {
        Self(value.0, value.1, value.2, value.3)
    }
}

impl IntoPy<PyObject> for FoliageVal {
    fn into_py(self, py: Python<'_>) -> PyObject {
        <FoliageValProxy>::from(self).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for FoliageVal {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(<FoliageValProxy>::extract_bound(ob)?.into())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Foliage {
    pub vals: Vec<FoliageVal>
}

impl IntoPy<PyObject> for Foliage {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.vals.into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Foliage {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self { vals: Vec::extract_bound(ob)? })
    }
}
impl Foliage {
    // holds vertex data of some sort
    pub fn from_data<O: Version + 'static>(data: &[u8], info: &FoliageInfo) -> Result<Self> {
        let n = (info.ub_w - info.lb_w) * (info.ub_h - info.lb_h);
        Ok(Self { vals: OrderedDataVec::from_bytes::<O>(&data[info.offset as usize..], n as usize)? })
    }

    pub fn dump<O: Version + 'static>(&self) -> Vec<u8> {
        self.vals.dump_bytes::<O>() 
    }

    pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], info: &FoliageInfo) -> Result<()> {
        self.vals.to_bytes::<O>(&mut data[info.offset as usize..])
    }
}
