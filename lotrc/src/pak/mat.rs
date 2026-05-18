use super::*;

#[cfg_attr(feature = "python", basicpymethods)]
#[cfg_attr(feature = "python", pyclass(module = "pak", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
#[cfg_attr(feature = "python", derive(PyMethods))]
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
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_0: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_1: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_2: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_3: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_4: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_5: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_6: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_7: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_8: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_9: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_10: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_11: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_12: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_13: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_14: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_15: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_16: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_17: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_18: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_19: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_20: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_21: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_22: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_23: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_24: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_25: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_26: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_27: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_28: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_29: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_30: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_31: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_32: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_33: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_34: u32,
    #[ordered_data(skipPC, skipXBOX, skipPS3, skipXBOXPROTO2)]
    pub p_35: u32,
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

#[cfg_attr(feature = "python", basicpymethods)]
#[cfg_attr(feature = "python", pyclass(module = "pak", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
#[cfg_attr(feature = "python", derive(PyMethods))]
pub struct Mat1 {
    pub base: MatBase,
}

#[cfg_attr(feature = "python", basicpymethods)]
#[cfg_attr(feature = "python", pyclass(module = "pak", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
#[cfg_attr(feature = "python", derive(PyMethods))]
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

#[cfg_attr(feature = "python", basicpymethods)]
#[cfg_attr(feature = "python", pyclass(module = "pak", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
#[cfg_attr(feature = "python", derive(PyMethods))]
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
    pub variation_id_color: u8,
    pub variation_id_texture: u8,
    pub variation_id_specular: u8,
    pub unk_114d: u8,
    pub unk_115: u32,
    #[ordered_data(skipPC, skipXBOX, skipXBOXPROTO, skipXBOXPROTO2)]
    #[serde(default)]
    pub unk_116: u32,
    #[ordered_data(skipPC, skipXBOX, skipXBOXPROTO, skipXBOXPROTO2)]
    #[serde(default)]
    pub unk_117: u32,
}

#[cfg_attr(feature = "python", basicpymethods)]
#[cfg_attr(feature = "python", pyclass(module = "pak", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
#[cfg_attr(feature = "python", derive(PyMethods))]
pub struct Mat4 {
    pub base: MatBase,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_90: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_91: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_92: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_93: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_94: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_95: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_96: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_97: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_98: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_99: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_100: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_101: u32,
    #[ordered_data(skipXBOXPROTO)]
    pub unk_102: u32,
    #[ordered_data(skipXBOXPROTO)]
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

#[cfg_attr(feature = "python", basicpymethods)]
#[cfg_attr(feature = "python", pyclass(module = "pak", get_all, set_all))]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
#[cfg_attr(feature = "python", derive(PyMethods))]
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
