import warnings

import lotrc.types
from lotrc.utils import *
from lotrc.types import *

Header = structtuple("LevelPAK_Header",
    'blockA_num', '<I',
    'blockA_offset', '<I',
    'constx13', 'I',
    'version', 'I',
    'strings_offset', 'I',
    'strings_size', 'I',
    'strings_num', 'I',
    'block1_offset', 'I',
    'block1_size', 'I',
    'block1_size_comp', 'I',
    'sub_blocks1_offset', 'I',
    'block2_offset', 'I',
    'block2_size', 'I',
    'block2_size_comp', 'I',
    'sub_blocks2_offset', 'I',
    'string_keys_offset', 'I',
    'unk_16', 'I',
    'unk_17', 'I',
    'unk_18', 'I',
    'unk_19', 'I',
    'unk_20', 'I',
    'unk_21', 'I',
    'unk_22', 'I',
    'unk_23', 'I',
    'unk_24', 'I',
    'unk_25', 'I',
    'unk_26', 'I',
    'unk_27', 'I',
    'unk_28', 'I',
    'unk_29', 'I',
    'unk_30', 'I',
    'unk_31', 'I',
    'unk_32', 'I',
    'unk_33', 'I',
    'unk_34', 'I',
    'unk_35', 'I',
    'unk_36', 'I',
    'unk_37', 'I',
    'unk_38', 'I',
    'unk_39', 'I',
    'unk_40', 'I',
    'unk_41', 'I',
    'obja_num', 'I',
    'obj0_num', 'I',
    'mesh_info_num', 'I', # 1
    'buffer_info_num', 'I', # 2
    'mat1_num', 'I',
    'mat2_num', 'I',
    'mat3_num', 'I',
    'mat4_num', 'I',
    'mat_extra_num', 'I',
    'unk_51', 'I',
    'shape_info_num', 'I',
    'hk_shape_info_num', 'I', # d
    'hk_constraint_data_num', 'I', # e
    'vbuff_info_num', 'I', # f
    'ibuff_info_num', 'I', # g
    'texture_info_num', 'I', # 7
    'animation_info_num', 'I', # 8
    'hk_constraint_info_num', 'I', # 9
    'game_objs_block_info_num', 'I', # 10
    'pfield_info_num', 'I', # 12
    'gfx_block_info_num', 'I',
    'animation_block_info_num', 'I',
    'obj11_num', 'I',
    'obj14_info_num', 'I',
    'unk_66', 'I',
    'obja_offset', 'I', # 24 bytes
    'obj0_offset', 'I',
    'mesh_info_offset', 'I', # 256 bytes, max loaded is 0x400
    'buffer_info_offset', 'I',
    'mat1_offset', 'I',
    'mat2_offset', 'I',
    'mat3_offset', 'I',
    'mat4_offset', 'I',
    'mat_extra_offset', 'I',
    'unk_76', 'I',
    'shape_info_offset', 'I',
    'hk_shape_info_offset', 'I',
    'hk_constraint_data_offset', 'I',
    'vbuff_info_offset', 'I',
    'ibuff_info_offset', 'I',
    'texture_info_offset', 'I', # 0x12 bytes, max loaded is 0x800, related to MgSurfaceWin32
    'animation_info_offset', 'I',
    'hk_constraint_info_offset', 'I',
    'game_objs_block_info_offset', 'I',
    'pfield_info_offset', 'I',
    'gfx_block_info_offset', 'I', # 0xc bytes, max loaded is 0x40
    'animation_block_info_offset', 'I', # 36 bytes
    'obj11_offset', 'I',
    'obj14_info_offset', 'I',
    'unk_91', 'I',
    'unk_92', 'I',
    'unk_93', 'I',
    'unk_94', 'I',
    'unk_95', 'I',
    'unk_96', 'I',
    'unk_97', 'I',
    'unk_98', 'I',
    'unk_99', 'I',
    'unk_100', 'I',
    'unk_101', 'I',
    'unk_102', 'I',
    'unk_103', 'I',
    'unk_104', 'I',
    'unk_105', 'I',
    'unk_106', 'I',
    'unk_107', 'I',
    'unk_108', 'I',
    'unk_109', 'I',
    'unk_110', 'I',
    'unk_111', 'I',
    'unk_112', 'I',
    'unk_113', 'I',
    'unk_114', 'I',
    'unk_115', 'I',
    'block2_offsets_num', 'I',
    'block2_offsets_offset', 'I',
)

ObjA = structtuple("LevelPAK_ObjA", 
    'key', '<I',
    'unk_1', '<I',
    'size', '<I',
    'size_comp', '<I',
    'unk_4', '<I',
    'type', '<I',
)

Obj0 = structtuple("LevelPAK_Obj0",
    'unk_0', '<I',
    'key', '<I',
)

MeshInfo = structtuple("MeshInfo",
    'key', 'I',
    'block_flag', 'I',
    'mat_offset', 'I',
    'buffer_info_offset', 'I', # pointer to buffer_info, uses mat_num of sequential objects
    'unk_4', 'I',
    'unk_5', 'I',
    'unk_6', 'I',
    'unk_7', 'I',
    'unk_8', 'I',
    'unk_9', 'I',
    'unk_10', 'I',
    'unk_11', 'I',
    'valCs_offset', 'I', # ints (c & 0x3fffffff is an index into the buffer_infos referenced by this object)
    'unk_13', 'I', # (v1, v2, v3, v4, v5) * 4 (up to unk_23), v1 is a starting offset to buffer_infos, v2 is the end offset
    'unk_14', 'I',
    'unk_15', 'I',
    'unk_16', 'I',
    'unk_17', 'I',
    'unk_18', 'I',
    'unk_19', 'I',
    'unk_20', 'I',
    'unk_21', 'I',
    'unk_22', 'I',
    'unk_23', 'I',
    'unk_24', 'I',
    'unk_25', 'I',
    'unk_26', 'I',
    'unk_27', 'I',
    'unk_28', 'I',
    'unk_29', 'I',
    'unk_30', 'I',
    'unk_31', 'I',
    'valCs_num', 'I',
    'mat_num', 'I',
    'keys_offset', 'I', # ints
    'indices_offset', 'I',
    'matrices_offset', 'I', # 16 ints (matrix?) for keys_num
    'keys_num', 'I',
    'valGs_offset', 'I',
    'valGs_num', 'I',
    'valIs_offset', 'I',
    'vbuff_offset', 'I',
    'vbuff_num', 'I',
    'ibuff_offset', 'I',
    'ibuff_num', 'I',
    'valDs_offset', 'I', # f_num * 8 ints
    'unk_46', 'I',
    'unk_47', 'I',
    'valJs_num', 'I',
    'valJs_offset', 'I',
    'block_offset', 'I',
    'valKs_offset', 'I', # not sure on the size, seems to be 36 ints
    'asset_key', 'I', # data in bin that is vertex & index buffer values
    'asset_type', 'I',
    'unk_54', 'I',
    'unk_55', 'I',
    'shape_info_offset', 'I', # optional pointer to shape_info
    'unk_57', 'I',
    'hkConstraintData_offset', 'I', # optional pointer to hkConstraintData
    'unk_59', 'I',
    'hkConstraint_offset', 'I', # optional pointer to hkConstraint
    'keys2_offset', 'I',
    'keys2_order_offset', 'I',
    'valAs_offset', 'I', # 8 ints
)

BufferInfo = structtuple("BufferInfo",
    'vbuff_info_offset', 'I', # pointer to vbuff_info
    'vbuff_info_offset_2', 'I', # optional pointer to vbuff_info
    'vbuff_info_offset_3', 'I', # optional pointer to vbuff_info
    'unk_3', 'I',
    'unk_4', 'I',
    'unk_5', 'I',
    'unk_6', 'I',
    'unk_7', 'I',
    'unk_8', 'I',
    'unk_9', 'I',
    'unk_10', 'I',
    'unk_11', 'I',
    'unk_12', 'I',
    'unk_13', 'I',
    'unk_14', 'I',
    'unk_15', 'I',
    'unk_16', 'I',
    'unk_17', 'I',
    'unk_18', 'I',
    'unk_19', 'I',
    'unk_20', 'I',
    'unk_21', 'I',
    'unk_22', 'I',
    'unk_23', 'I',
    'unk_24', 'I',
    'unk_25', 'I',
    'unk_26', 'I',
    'unk_27', 'I',
    'unk_28', 'I',
    'unk_29', 'I',
    'unk_30', 'I',
    'unk_31', 'I',
    'v_size', 'I',
    'v_size_2', 'I',
    'v_size_3', 'I',
    'unk_35', 'I',
    'unk_36', 'I',
    'unk_37', 'I',
    'unk_38', 'I',
    'unk_39', 'I',
    'unk_40', 'I',
    'unk_41', 'I',
    'unk_42', 'I',
    'unk_43', 'I',
    'unk_44', 'I',
    'unk_45', 'I',
    'unk_46', 'I',
    'unk_47', 'I',
    'vbuff_size', 'I',
    'vbuff_size_2', 'I',
    'vbuff_size_3', 'I',
    'unk_51', 'I',
    'unk_52', 'I',
    'unk_53', 'I',
    'unk_54', 'I',
    'unk_55', 'I',
    'unk_56', 'I',
    'unk_57', 'I',
    'unk_58', 'I',
    'unk_59', 'I',
    'unk_60', 'I',
    'unk_61', 'I',
    'unk_62', 'I',
    'unk_63', 'I',
    'unk_64', 'I',
    'ibuff_info_offset', 'I', # poiner to ibuff_ingo
    'i_num', 'I', # number of indeices in ibuffer
    'unk_67', 'I',
    'unk_68', 'I',
    'unk_69', 'I',
    'unk_70', 'I',
    'tri_num', 'I', # number of objects(triangles) in ibufffer
    'unk_72', 'I',
    'unk_73', 'I',
    'unk_74', 'I',
    'unk_75', 'I',
    'unk_76', 'I',
    'unk_77', 'I',
    'unk_78', 'I',
    'unk_79', 'I',
    'unk_80', 'I',
    'unk_81', 'I',
    'unk_82', 'I',
    'unk_83', 'I',
    'unk_84', 'I',
    'unk_85', 'I',
    'unk_86', 'I',
    'unk_87', 'I',
    'unk_88', '4S',
)

MatBase = [ # normal material
    'unk_0', 'I',
    'unk_1', 'I',
    'tex_2', 'I',
    'tex_3', 'I',
    'tex_4', 'I',
    'tex_5', 'I',
    'tex_6', 'I',
    'tex_7', 'I',
    'tex_8', 'I',
    'tex_9', 'I',
    'tex_10', 'I',
    'tex_11', 'I',
    'tex_12', 'I',
    'tex_13', 'I',
    'tex_14', 'I',
    'tex_15', 'I',
    'tex_16', 'I',
    'tex_17', 'I',
    'unk_18', 'I',
    'unk_19', 'I',
    'unk_20', 'I',
    'unk_21', 'I',
    'unk_22', 'I',
    'unk_23', 'I',
    'unk_24', 'I',
    'unk_25', 'I',
    'unk_26', 'I',
    'unk_27', 'I',
    'unk_28', 'I',
    'unk_29', 'I',
    'unk_30', 'I',
    'unk_31', 'I',
    'unk_32', 'I',
    'unk_33', 'I',
    'z_34', 'I',
    'z_35', 'I',
    'z_36', 'I',
    'z_37', 'I',
    'z_38', 'I',
    'z_39', 'I',
    'unk_40', 'I',
    'unk_41', 'I',
    'unk_42', 'I',
    'unk_43', 'I',
    'unk_44', 'I',
    'unk_45', 'I',
    'unk_46', 'I',
    'unk_47', 'I',
    'unk_48', 'I',
    'unk_49', 'I',
    'flags', 'Q', #(flags1, flags2)
    'type', 'I',
    'unk_53', 'I',
    'unk_54', 'H',
    'side_flags', 'H',
    'unk_55', 'I',
    'unk_56', 'I',
    'unk_57', 'I',
    'unk_58', 'I',
    'unk_59', 'I',
    'unk_60', 'I',
    'unk_61', 'I',
    'unk_62', 'I',
    'unk_63', 'I',
    'unk_64', 'I',
    'unk_65', 'I',
    'unk_66', 'I',
    'unk_67', 'I',
    'unk_68', 'I',
    'unk_69', 'I',
    'unk_70', 'I',
    'unk_71', 'I',
    'unk_72', 'I',
    'unk_73', 'I',
    'unk_74', 'I',
    'unk_75', 'I',
    'unk_76', 'I',
    'unk_77', 'I',
    'unk_78', 'I',
    'unk_79', 'I',
    'unk_80', 'I',
    'unk_81', 'I',
    'unk_82', 'I',
    'unk_83', 'I',
    'unk_84', 'I',
    'unk_85', 'I',
    'mat_extra_offset', 'I', # optional offset to mat_extra
    'key', 'I',
    'unk_88', 'I',
    'z_89', 'I',
]

Mat1 = structtuple("Mat1", # something to do with MgMaterial
    *MatBase,
)

Mat2 = structtuple("Mat2", # something to do with MgMaterial
    *MatBase,
    'unk_90', 'I',
    'unk_91', 'I',
    'unk_92', 'I',
    'unk_93', 'I',
    'unk_94', 'I',
    'unk_95', 'I',
    'unk_96', 'I',
    'unk_97', 'I',
    'unk_98', 'I',
    'unk_99', 'I',
    'unk_100', 'I',
    'unk_101', 'I',
    'unk_102', 'I',
    'unk_103', 'I',
    'unk_104', 'I',
    'unk_105', 'I',
    'unk_106', 'I',
    'unk_107', 'I',
    'unk_108', 'I',
    'unk_109', 'I',
    'unk_110', 'I',
    'unk_111', 'I',
    'unk_112', 'I',
    'unk_113', 'I',
    'unk_114', 'I',
    'unk_115', 'I',
    'unk_116', 'I',
    'unk_117', 'I',
    'unk_118', 'I',
    'unk_119', 'I',
    'unk_120', 'H',
    'unk_120_', 'H',
    'unk_121', 'I',
)

Mat3 = structtuple("Mat3", # something to do with MgMaterial
    *MatBase,
    'unk_90', 'I',
    'unk_91', 'I',
    'unk_92', 'I',
    'unk_93', 'I',
    'unk_94', 'I',
    'unk_95', 'I',
    'unk_96', 'I',
    'unk_97', 'I',
    'unk_98', 'I',
    'unk_99', 'I',
    'unk_100', 'I',
    'unk_101', 'I',
    'unk_102', 'I',
    'unk_103', 'I',
    'unk_104', 'I',
    'unk_105', 'I',
    'unk_106', 'I',
    'unk_107', 'I',
    'unk_108', 'I',
    'unk_109', 'I',
    'unk_110', 'I',
    'unk_111', 'I',
    'unk_112', 'I',
    'unk_113', 'I',
    'unk_114', '4S',
    'unk_115', 'I',
)

Mat4 = structtuple("Mat4", # something to do with MgMaterial
    *MatBase,
    'unk_90', 'I',
    'unk_91', 'I',
    'unk_92', 'I',
    'unk_93', 'I',
    'unk_94', 'I',
    'unk_95', 'I',
    'unk_96', 'I',
    'unk_97', 'I',
    'unk_98', 'I',
    'unk_99', 'I',
    'unk_100', 'I',
    'unk_101', 'I',
    'unk_102', 'I',
    'unk_103', 'I',
    'unk_104', 'I',
    'unk_105', 'I',
    'unk_106', 'I',
    'unk_107', 'I',
    'unk_108', 'I',
    'unk_109', 'I',
    'unk_110', 'I',
    'unk_111', 'I',
    'unk_112', 'I',
    'unk_113', 'I',
    'unk_114', 'I',
    'unk_115', 'I',
    'unk_116', 'I',
    'unk_117', 'I',
    'unk_118', 'I',
    'unk_119', 'I',
    'unk_120', 'I',
    'unk_121', 'I',
    'unk_122', 'I',
    'unk_123', 'I',
    'unk_124', 'I',
    'unk_125', 'I',
    'unk_126', 'I',
    'unk_127', 'I',
    'unk_128', 'I',
    'unk_129', 'I',
    'unk_130', 'I',
    'unk_131', 'I',
    'unk_132', 'I',
    'unk_133', 'I',
    'unk_134', 'I',
    'unk_135', 'I',
    'unk_136', 'I',
    'unk_137', 'I',
    'unk_138', 'I',
    'unk_139', 'I',
    'unk_140', 'I',
    'unk_141', 'I',
    'unk_142', 'I',
    'unk_143', 'I',
    'unk_144', 'I',
    'unk_145', 'I',
)    

MatExtra = structtuple("MatExtra",
    'unk_0', 'I',
    'unk_1', 'I',
    'unk_2', 'I',
    'unk_3', 'I',
    'unk_4', 'I',
    'unk_5', 'I',
    'unk_6', 'I',
    'unk_7', 'I',
    'unk_8', 'I',
    'unk_9', 'I',
    'unk_10', 'I',
    'unk_11', 'I',
    'unk_12', 'I',
    'unk_13', 'I',
    'unk_14', 'I',
    'unk_15', 'I',
    'unk_16', 'I',
    'unk_17', 'I',
    'unk_18', 'I',
    'unk_19', 'I',
    'unk_20', 'I',
    'unk_21', 'I',
    'unk_22', 'I',
    'unk_23', 'I',
    'unk_24', 'I',
    'unk_25', 'I',
    'unk_26', 'I',
    'unk_27', 'I',
    'unk_28', 'I',
    'unk_29', 'I',
    'unk_30', 'I',
    'unk_31', 'I',
    'unk_32', 'I',
    'unk_33', 'I',
    'unk_34', 'I',
    'unk_35', 'I',
    'unk_36', 'I',
    'unk_37', 'I',
    'unk_38', 'I',
    'unk_39', 'I',
    'unk_40', 'I',
    'unk_41', 'I',
    'unk_42', 'I',
    'unk_43', 'I',
    'unk_44', 'I',
    'unk_45', 'I',
    'unk_46', 'I',
    'unk_47', 'I',
    'unk_48', 'I',
    'unk_49', 'I',
)

ShapeInfo = structtuple("ShapeInfo",
    'offset', 'I', # sometimes a pointer to something, otherwise the number of strings from the mesh_info pointing to this
    'type', 'I', # 0, 1, 2, 3, 4, 5
    'unk_2', 'I',
    'unk_3', 'I',
    'unk_4', 'I',
    'unk_5', 'I',
    'unk_6', 'I',
    'unk_7', 'I',
    'unk_8', 'I',
    'unk_9', 'I',
    'unk_10', 'I',
    'unk_11', 'I',
    'unk_12', 'I',
    'unk_13', 'I',
    'unk_14', 'I',
    'unk_15', 'I',
    'unk_16', 'I',
    'unk_17', 'I',
    'unk_18', 'I',
    'unk_19', 'I',
    'unk_20', 'I',
    'unk_21', 'I',
    'unk_22', 'I',
    'unk_23', 'I',
    'unk_24', 'I',
    'unk_25', 'I',
    'unk_26', 'I',
    'hkshape_num', 'I',
    'hkshape_offset', 'I', # pointer to hk_shape_info
    'unk_29', '4S',
    'unk_30', 'I',
)

HkShapeInfo = structtuple("HkShapeInfo", # HkShapeInfo
    'unk_0', Vector4,
    'unk_4', Vector4,
    'type', 'I',
    'unk_9', 'I',
    'a_num', 'I',
    'a_offset', 'I',
    'b_num', 'I',
    'b_offset', 'I',
    'c_num', 'I',
    'c_offset', 'I',
    'd_num', 'I',
    'd_offset', 'I',
    'e_num', 'I',
    'e_offset', 'I',
)

HkConstraintData = structtuple("HkConstraintData", # HkConstraintData
    'type', 'I',
    'unk_1', 'I',
    'unk_2', 'I',
    'unk_3', 'I',
    'unk_4', 'I',
    'unk_5', 'I',
    'unk_6', 'I',
    'unk_7', 'I',
    'unk_8', 'I',
    'unk_9', 'I',
    'unk_10', 'I',
    'unk_11', 'I',
    'unk_12', 'I',
    'unk_13', 'I',
    'unk_14', 'I',
    'unk_15', 'I',
    'unk_16', 'I',
    'unk_17', 'I',
    'unk_18', 'I',
    'unk_19', 'I',
    'unk_20', 'I',
    'unk_21', 'I',
    'unk_22', 'I',
    'unk_23', 'I',
    'unk_24', 'I',
    'unk_25', 'I',
    'unk_26', 'I',
    'unk_27', 'I',
    'unk_28', 'I',
)

VBuffInfo = structtuple("VBuffInfo",
    'unk_0', 'I',
    'size', 'I',
    'unk_3', 'I',
    'offset', 'I',
    'fmt1', 'I',
    'fmt2', 'I',
    'unk_6', 'I',
    'unk_7', 'I',
    alt_fmt = [
        'unk_0', 'I',
        'size', 'I',
        'unk_3', 'I',
        'offset', 'I',
        'fmt2', 'I',
        'fmt1', 'I',
        'unk_6', 'I',
        'unk_7', 'I',
        'unk_8', 'I',
        'unk_9', 'I',
        'unk_10', 'I',
        'unk_11', 'I',
        'unk_12', 'I',
        'unk_13', 'I',
    ]
)

IBuffInfo = structtuple("IBuffInfo",
    'unk_0', 'I',
    'size', 'I',
    'format', 'I',
    'unk_3', 'I',
    'offset', 'I',
    'unk_5', 'I',
    alt_fmt = [
        'unk_0', 'I',
        'size', 'I',
        'format', 'I', # 0x10 -> u16 otherwise u32
        'unk_3', 'I',
        'offset', 'I',
        'unk_5', 'I',
        'unk_6', 'I',
        'unk_7', 'I',
        'unk_8', 'I',
        'unk_9', 'I',
        'unk_10', 'I',
        'unk_11', 'I',
        'unk_12', 'I',
    ]
)

TextureInfo = structtuple("TextureInfo",
    'key', 'I',
    'block_flag', 'I',
    'asset_key', 'I',
    'asset_type', 'I',
    'type', 'I',
    'format', 'I',
    'unk_6', 'I',
    'unk_7', 'I',
    'unk_8', 'I',
    'unk_9', 'I',
    'unk_10', 'I',
    'unk_11', 'I',
    'width', 'H',
    'height', 'H',
    'depth', 'H',
    'levels', 'H',
    'unk_16', '16S',
)

AnimationInfo = structtuple("AnimationInfo",
    'key', 'I',
    'block_flag', 'I',
    'offset', 'I',
    'size', 'I',
    'type', 'I',
    'unk_5', 'I',
    'keys_num', 'I',
    'something_num', 'I',
    'unk_8', 'I',
    'vala', 'I',
    'unk_10', 'I',
    'unk_11', 'I',
    'data_offset', 'I',
    'unk_13', 'I',
    'unk_14', 'I',
    'unk_15', 'I',
    'block_starts_offset', 'I',
    'block_starts_num', 'I',
    'block_ends_offset', 'I',
    'block_ends_num', 'I',
    'objC3_offset', 'I',
    'objC3_num', 'I',
    'objC4_offset', 'I',
    'objC4_num', 'I',
    'block_offset', 'I',
    'block_size', 'I',
    'obj3_num', 'I',
    'obj3_offset', 'I',
    'unk_28', 'I',
    'unk_29', 'I',
    'obj1_num', 'I',
    'keys_offset', 'I',
    'unk_32', 'I', # diff between versions
    'obj1_offset', 'I',
    'obj2_offset', 'I',
    'obj2_num', 'I',
    'obj5_offset', 'I', # to some object that contains offsets in pos 1 and 2 and a value in pos 0
)

HkConstraintInfo = structtuple("HkConstraintInfo",
    'type', 'I',
    'shorts_offset', 'I', 
    'shorts_num', 'I',
    'strings_offset', 'I',
    'strings_num', 'I',
    'vals_offset', 'I',
    'vals_num', 'I',
    'unk_7', 'I',
    'unk_8', 'I',
    'unk_9', 'I',
    'keys_offset', 'I',
    'keys_num', 'H',
    'keys2_num', 'H',
    'keys2_offset', 'I',
    'unk_13', 'I', # diff between versions
    'unk_14', 'f',
    'unk_15', 'I',
    'unk_16', 'I',
    'unk_17', 'I',
)

GameObjBlockInfo = structtuple("GameObjBlockInfo",
    'key', 'I',
    'unk_1', 'I',
    'offset', 'I',
    'size', 'I',
)

PFieldInfo = structtuple("PFieldInfo",
    "key1", "I",
    "key2", "I",
    "width", "I",
    "height", "I",
    "offset", "I",
)

GFXBlockInfo = structtuple("GFXBlockInfo", # GFX blocks?, unchanged by encoding, model as data
    'key', 'I',
    'offset', 'I', # offset pointing to something in block1
    'size', 'I',
)

AnimationBlockInfo = structtuple("AnimationBlockInfo",
    'key', 'I',
    'unk_1', 'I',
    'key_name', 'I',
    'offset', 'I',
    'size', 'I',
    'size_comp', 'I',
    'unk_6', 'I',
    'unk_7', 'I',
    'unk_8', 'I',
)

Obj11 = structtuple("Obj11", # something to do with textures
    "key", "I",
    "unk_1", "I",
    "unk_2", "I",
    "unk_3", "I",
    "unk_4", "I",
    "unk_5", "I",
    "unk_6", "I",
    "offset", "I",
    "key1", "I",
    "key2", "I",
    "key3", "I",
    "unk_11", "I",
    "unk_12", "I",
    "unk_13", "I",
    "unk_14", "I",
    "unk_15", "I",
    "unk_16", "I",
    "unk_17", "I",
    "unk_18", "I",
    "unk_19", "I",
)

Obj14Info = structtuple("Obj14", # points to list of ints in block1
    'guid', 'I',
    'num', 'I',
    'offset', 'I',
)

BlockAVal = structtuple("BlockAVal",
    'unk_0', '<I',
    'block_flags', '<I',
    'key', '<I',
    'unk_3', '<I',
    'unk_4', '<I',
    'unk_5', '<I',
    'unk_6', '<I',
)

class Mesh:
    BlockHeader = structtuple("BlockHeader",
        "unk_0", "I",
        "unk_1", "I",
        "size", "I",
        "unk_3", "I",
        "a", "I",
        "b", "I",
        "unk_6", "I",
        "unk_7", "I",
        "unk_8", "I",
        "unk_9", "I",
        "unk_10", "I",
        "unk_11", "I",
        "unk_12", "I",
    )
    BlockVal = structtuple("BlockVal",
        "unk_0", "H",
        "unk_1", "H",
        "unk_2", "I",
        "unk_3", "I",
        "unk_4", "I",
        "unk_5", "I",
    )

    @classmethod
    def unpack_from(Self, buffer, info, f="<"):
        lotrc.types.MIN_OFFSET = np.inf
        lotrc.types.MAX_OFFSET = 0
        self = Self()
        self.indices = unpack_list_from(Uint[f], buffer, info['indices_offset'], max(info['keys_num'], 4))
        assert self.indices[0]['val'] == 0xFFFFFFFF
        self.keys = unpack_list_from(Uint[f], buffer, info['keys_offset'], info['keys_num'])
        self.matrices = unpack_list_from(Matrix4x4[f], buffer, info['matrices_offset'], info['keys_num'])
        self.valAs = unpack_list_from(Int[f], buffer, info['valAs_offset'], info['keys_num'] * 8)
        self.mats = unpack_list_from(Int[f], buffer, info['mat_offset'], info['mat_num'])
        self.valCs = unpack_list_from(Int[f], buffer, info['valCs_offset'], info['valCs_num'])
        self.valDs = unpack_list_from(Int[f], buffer, info['valDs_offset'], info['valCs_num'] * 8)
        self.vbuffs = unpack_list_from(Int[f], buffer, info['vbuff_offset'], info['vbuff_num'])
        self.ibuffs = unpack_list_from(Int[f], buffer, info['ibuff_offset'], info['ibuff_num'])
        self.valGs = unpack_list_from(Int[f], buffer, info['valGs_offset'], info['valGs_num'] * 16)
        if info['valJs_num'] == 0 and info['valJs_offset'] != 0 and info['valJs_offset'] != info['valGs_offset']:
            # valJs is sometimes a list of offsets of length keys_num, that point to offsets that point to 4 ints ???
            self.valJs = unpack_list_from(Uint[f], buffer, info['valJs_offset'], info['keys_num'])
            self.valJoffs = []
            self.valJvals = []
            for val in self.valJs['val']:
                self.valJoffs.append(unpack_from(Uint[f], buffer, val))
                off = self.valJoffs[-1]['val']
                i = 0
                while buffer[off + i] != 0:
                    i += 1
                self.valJvals.append(buffer[off:off+i])
        else:
            self.valJs = unpack_list_from(Int[f], buffer, info['valJs_offset'], info['valJs_num'])
        if info['valKs_offset'] != 0:
            self.valKs_header = unpack_list_from(Ushort[f], buffer, info['valKs_offset'], 2)
            # if (self.valKs_header[0]['val'] != 3) or (self.valKs_header[1]['val'] != 6):
            #     warnings.warn(f"ValsK error, mesh {info['key']}")
            self.valKs = unpack_list_from(Float[f], buffer, info['valKs_offset'] + 4, 35)
        if info['valIs_offset'] != 0:
            self.valIs = unpack_list_from(Int[f], buffer, info['valIs_offset'], info['valGs_num'])
        if info['keys2_offset'] != 0:
            assert info['keys2_order_offset'] != 0
            i = 0
            while unpack_from(Int[f], buffer, info["keys2_offset"] + 8*i)['val'] != 0:
                i += 1
            i += 1
            self.keys2 = unpack_list_from(Uint[f], buffer, info['keys2_offset'], i * 2)
            self.keys2_order = unpack_list_from(Uint[f], buffer, info['keys2_order_offset'], self.keys2[-1]['val'])
        if info['block_offset'] != 0:
            self.block_header = unpack_from(Self.BlockHeader[f], buffer, info['block_offset'])
            size = self.block_header.nbytes
            self.block_vals_a = unpack_list_from(Uint[f], buffer, info['block_offset'] + size, (self.block_header['a'] + self.block_header['b']) * 12)
            size += self.block_vals_a.nbytes
            self.block_vals_b = unpack_list_from(Self.BlockVal[f], buffer, info['block_offset'] + size, (self.block_header['size'] - size) // Self.BlockVal[f].itemsize)
            size += self.block_vals_b.nbytes
            self.block_extra = unpack_list_from(Byte[f], buffer, info['block_offset'] + size, self.block_header['size'] - size)
            # block_size = unpack_from(Int[f], buffer, info['block_offset'] + 8)['val']
            # assert block_size % 4 == 0
            # self.block = unpack_list_from(Int[f], buffer, info['block_offset'], block_size//4)
        # not sure why this pops up once, maybe it is padding between items?
        if info['valCs_offset'] == info['vbuff_offset'] and info['valCs_offset'] == info['ibuff_offset'] and info['valCs_offset'] == info['valDs_offset']:
            self.val = unpack_list_from(Int[f], buffer, info['valCs_offset'], 4)
        self.max_offset = lotrc.types.MAX_OFFSET
        self.min_offset = lotrc.types.MIN_OFFSET
        return self

    def pack_into(self, buffer, info, f="<"):
        pack_into(self.indices, buffer, info['indices_offset'], f)
        pack_into(self.keys, buffer, info['keys_offset'], f)
        pack_into(self.matrices, buffer, info['matrices_offset'], f)
        pack_into(self.valAs, buffer, info['valAs_offset'], f)
        pack_into(self.mats, buffer, info['mat_offset'], f)
        pack_into(self.valCs, buffer, info['valCs_offset'], f)
        pack_into(self.valDs, buffer, info['valDs_offset'], f)
        pack_into(self.vbuffs, buffer, info['vbuff_offset'], f)
        pack_into(self.ibuffs, buffer, info['ibuff_offset'], f)
        pack_into(self.valGs, buffer, info['valGs_offset'], f)
        if info['valJs_num'] == 0 and info['valJs_offset'] != 0 and info['valJs_offset'] != info['valGs_offset']:
            # valJs is sometimes a list of offsets of length keys_num, that point to offsets that point to 4 ints ???
            pack_into(self.valJs, buffer, info['valJs_offset'], f)
            for val, valoff, valval in zip(self.valJs['val'], self.valJoffs, self.valJvals):
                pack_into(valoff, buffer, val, f)
                buffer[valoff['val']:valoff['val']+len(valval)] = valval
        else:
            pack_into(self.valJs, buffer, info['valJs_offset'], f)

        if info['valKs_offset'] != 0:
            pack_into(self.valKs_header, buffer, info['valKs_offset'], f)
            pack_into(self.valKs, buffer, info['valKs_offset']+4, f)
        if info['valIs_offset'] != 0:
            pack_into(self.valIs, buffer, info['valIs_offset'], f)
        if info['keys2_offset'] != 0:
            pack_into(self.keys2, buffer, info['keys2_offset'], f)
            pack_into(self.keys2_order, buffer, info['keys2_order_offset'], f)
        if info['block_offset'] != 0:
            pack_into(self.block_header, buffer, info['block_offset'], f)
            size = self.block_header.nbytes
            pack_into(self.block_vals_a, buffer, info['block_offset'] + size, f)
            size += self.block_vals_a.nbytes
            pack_into(self.block_vals_b, buffer, info['block_offset'] + size, f)
            size += self.block_vals_b.nbytes
            pack_into(self.block_extra, buffer, info['block_offset'] + size, f)
            # pack_into(self.block, buffer, info['block_offset'], f)
        # not sure why this pops up once, maybe it is padding between items?
        if info['valCs_offset'] == info['vbuff_offset'] and info['valCs_offset'] == info['ibuff_offset'] and info['valCs_offset'] == info['valDs_offset']:
            pack_into(self.val, buffer, info['valCs_offset'], f)

class Shape:
    Header = structtuple("Header",
        "num", "I",
        "unk_1", "I",
        "unk_2", "I",
        "unk_3", "I",
    )
    @classmethod
    def unpack_from(Self, buffer, info, f="<"):
        # the same object is pointed to by multiple infos, so doing it this way isn't the best
        lotrc.types.MIN_OFFSET = np.inf
        lotrc.types.MAX_OFFSET = 0

        self = Self()
        if info['type'] == 0:
            offset = info['offset']
            self.header = unpack_from(Self.Header[f], buffer, offset)
            offset += self.header.nbytes
            self.vals = unpack_list_from(Uint[f], buffer, info['offset'] + self.header.nbytes, self.header['num'])
            offset += self.vals.nbytes
            self.data = buffer[offset:offset+self.vals[-1]['val']+2] # 2 seems to be the correct amount, not sure what the data is so I don't know how much extra is needed
        self.max_offset = lotrc.types.MAX_OFFSET
        self.min_offset = lotrc.types.MIN_OFFSET
        return self

    def pack_into(self, buffer, info, f="<"):
        if info['type'] == 0:
            offset = info['offset']
            pack_into(self.header, buffer, offset, f)
            offset += self.header.nbytes
            pack_into(self.vals, buffer, offset, f)
            offset += self.vals.nbytes
            buffer[offset:offset+self.vals[-1]['val']+2]  = self.data

class HkShape:
    """
        types:
            - 1: BoxShape, no extra data
            - 2: SphereShape, no extra data
            - 3: CapsuleShape, no extra data
            - 4: CylinderShape, no extra data
            - 5: ConvexVerticesShape, extra data
            - 6: MoppBvTreeShape, extra data (containes extended mesh shape and other)
    """
    @classmethod
    def unpack_from(Self, buffer, info, f="<"):
        lotrc.types.MIN_OFFSET = np.inf
        lotrc.types.MAX_OFFSET = 0
        self = Self()
        if info['type'] == 5:
            self.a = unpack_list_from(Uint[f], buffer, info['a_offset'], info['a_num'] * 4)
            b_num = info['b_num']
            while (info['b_offset'] + b_num * 12) % 16 != 0:
                b_num += 1
            self.b = unpack_list_from(Uint[f], buffer, info['b_offset'], b_num * 3) # somethimes seems to be off by 1
        elif info['type'] == 6:
            self.c = buffer[info['c_offset']:info['c_offset']+info['c_num']]
            # self.c = unpack_list_from(Byte[f], buffer, info['c_offset'], info['c_num'] * 4)
            self.d = unpack_list_from(Uint[f], buffer, info['d_offset'], info['d_num'] * 3)
            self.e = unpack_list_from(Ushort[f], buffer, info['e_offset'], info['e_num'] * 3)

            # self.c = unpack_list_from(Uint[f], buffer, info['c_offset'], info['c_num'])
            # self.d = unpack_list_from(Uint[f], buffer, info['d_offset'], info['d_num'] * 3)
            # self.e = unpack_list_from(Uint[f], buffer, info['e_offset'], info['e_num'] * 3)
        elif info['type'] > 6:
            warnings.warn(f"Unknown HkShape type {info['type']}, this is probably fine for now")
        self.max_offset = lotrc.types.MAX_OFFSET
        self.min_offset = lotrc.types.MIN_OFFSET
        return self
            
    def pack_into(self, buffer, info, f="<"):
        if info['type'] == 5:
            pack_into(self.a, buffer, info['a_offset'], f)
            pack_into(self.b, buffer, info['b_offset'], f)
        elif info['type'] == 6:
            buffer[info['c_offset']:info['c_offset']+len(self.c)] = self.c
            # pack_into(self.c, buffer, info['c_offset'], f)
            pack_into(self.d, buffer, info['d_offset'], f)
            pack_into(self.e, buffer, info['e_offset'], f)

class HkConstraint:
    @classmethod
    def unpack_from(Self, buffer, info, f="<"):
        lotrc.types.MIN_OFFSET = np.inf
        lotrc.types.MAX_OFFSET = 0
        self = Self()
        if info['type'] != 0:
            warnings.warn(f"Unknown HkConstraint type {info['type']}")

        self.shorts = unpack_list_from(Ushort[f], buffer, info['shorts_offset'], info['shorts_num'])
        assert self.shorts[0]['val'] == 0xFFFF
        
        self.strings = []
        max_offset = 0
        self.string_offsets = unpack_list_from(Uint[f], buffer, info['strings_offset'], info['strings_num'])
        for offset_ in self.string_offsets['val']:
            (offset, val) = unpack_list_from(Uint[f], buffer, offset_, 2)['val']
            start = offset
            while buffer[offset] != 0:
                offset += 1
            string = buffer[start:offset]
            self.strings.append((string, start, val))
            max_offset = max(offset+1, max_offset)
        self.vals = unpack_list_from(Int[f], buffer, info['vals_offset'], info['vals_num'] * 12)
        self.keys = unpack_list_from(Uint[f], buffer, info['keys_offset'], info['keys_num'])
        self.keys2 = unpack_list_from(Uint[f], buffer, info['keys2_offset'], info['keys2_num'] * 2)
        self.max_offset = max(lotrc.types.MAX_OFFSET, max_offset)
        self.min_offset = lotrc.types.MIN_OFFSET
        return self
        
    def pack_into(self, buffer, info, f="<"):
        pack_into(self.shorts, buffer, info['shorts_offset'], f)

        pack_into(self.string_offsets, buffer, info['strings_offset'], f)
        for offset_, (string, offset, val) in zip(self.string_offsets['val'], self.strings):
            pack_into(new(Uint[f], [offset, val]), buffer, offset_, f)
            buffer[offset:offset+len(string)] = string
        pack_into(self.vals, buffer, info['vals_offset'], f)
        pack_into(self.keys, buffer, info['keys_offset'], f)
        pack_into(self.keys2, buffer, info['keys2_offset'], f)

class Animation:
    Obj5Heder = structtuple("Animation_Obj5Header", 
        "objA_num", "I",
        "objA_offset", "I",
        "objB_num", "I",
        "objB_offset", "I",
    )
    def __init__(self):
        self.obj2 = {}
        self.obj3 = {}
        self.keys = {}
        self.obj5_header = {}
        self.obj5A = {}
        self.obj5B = {}
        self.objC = {}

    def unpack_from_block(self, buffer, offset, index, info, f="<"):
        self.obj2[index] = unpack_list_from(Int[f], buffer, offset + info['obj2_offset'], info['obj2_num']*4)
        self.obj3[index] = unpack_list_from(Int[f], buffer, offset + info['obj3_offset'], info['obj3_num']*11)
        self.keys[index] = unpack_list_from(Int[f], buffer, offset + info['keys_offset'], info['keys_num'])
        if info['obj5_offset'] != 0:
            self.obj5_header[index] = unpack_from(self.Obj5Heder[f], buffer, offset + info['obj5_offset'])
            self.obj5A[index] = unpack_list_from(Int[f], buffer, offset + self.obj5_header[index]['objA_offset'], self.obj5_header[index]['objA_num']*7)
            self.obj5B[index] = unpack_list_from(Int[f], buffer, offset + self.obj5_header[index]['objB_offset'], self.obj5_header[index]['objB_num']*7)
        if info['type'] == 3:
            self.objC[index] = hkaSplineSkeletalAnimation.unpack_from(buffer, offset, info, f)
        elif info['type'] < 3:
            warnings.warn(f"Unhandled amination type {info['type']}")
        else:
            warnings.warn(f"Unkown amination type {info['type']}")
        
    def pack_into_block(self, buffer, offset, index, info, f="<"):
        pack_into(self.obj2[index], buffer, offset + info['obj2_offset'], f)
        pack_into(self.obj3[index], buffer, offset + info['obj3_offset'], f)
        pack_into(self.keys[index], buffer, offset + info['keys_offset'], f)
        if info['obj5_offset'] != 0:
            pack_into(self.obj5_header[index], buffer, offset + info['obj5_offset'], f)
            pack_into(self.obj5A[index], buffer, offset + self.obj5_header[index]['objA_offset'], f)
            pack_into(self.obj5B[index], buffer, offset + self.obj5_header[index]['objB_offset'], f)
        if info['type'] == 3:
            self.objC[index].pack_into(buffer, offset, info, f)

    @staticmethod
    def unpack_block(anims, infos, buffer, offset, index, f="<"):
        for anim, info in zip(anims, infos):
            block_flag = 1 << index
            if block_flag & info['block_flag'] != 0:
                anim.unpack_from_block(buffer, offset, index, info, f)
                offset += info['size']
                
    @staticmethod
    def pack_block(anims, infos, buffer, offset, index, f="<"):
        for anim, info in zip(anims, infos):
            block_flag = 1 << index
            if block_flag & info['block_flag'] != 0:
                anim.pack_into_block(buffer, offset, index, info, f)
                offset += info['size']

class hkaSplineSkeletalAnimationObj1:
    Types = [Ubyte, Ushort, Ubyte, Ushort]
    ItemSizes = [1, 2, 1, 2]
    Counts = [0, 1, 1, 2, 1, 2, 2, 3]
    
    @classmethod
    def unpack_from(Self, buffer, offset, flags, flag_, f="<"):
        offset_ = offset
        self = Self()
        if flags == 0:
            offset = (offset + 3) & 0xfffffffc
            self.nbytes = offset - offset_
            return self
        if flags & 0xF0 == 0:
            self.s1, self.s2 = new(Short[f], 0), new(Byte[f], 0)
        else:
            self.s1 = unpack_from(Short[f], buffer, offset)
            offset += self.s1.nbytes
            self.s2 = unpack_from(Byte[f], buffer, offset)
            offset += self.s2.nbytes
            self.data = unpack_list_from(Byte[f], buffer, offset, self.s1['val'] + self.s2['val'] + 2)
            offset += self.data.nbytes
        offset = (offset + 3) & 0xfffffffc

        num = self.Counts[flags & 7] + self.Counts[((flags >> 4) & ~flags) & 7] * 2

        self.vals_a = unpack_list_from(Float[f], buffer, offset, num)
        offset += self.vals_a.nbytes
        
        if flags & 0xf0 == 0:
            offset = (offset + 3) & 0xfffffffc
            self.nbytes = offset - offset_
            return self
            
        offset = (offset + 1) & 0xfffffffe
        num = self.Counts[(flags >> 4) & 7] * (self.s1['val'] + 1)
        self.vals = unpack_list_from(self.Types[flag_][f], buffer, offset, num)
        offset += self.vals.nbytes
        
        offset = (offset + 3) & 0xfffffffc
        self.nbytes = offset - offset_
        return self

    def pack_into(self, buffer, offset, flags, flag_, f="<"):
        if flags == 0:
            return
        if flags & 0xF0 != 0:
            offset += pack_into(self.s1, buffer, offset, f)
            offset += pack_into(self.s2, buffer, offset, f)
            offset += pack_into(self.data, buffer, offset, f)
        offset = (offset + 3) & 0xfffffffc
        
        offset += pack_into(self.vals_a, buffer, offset, f)
        
        if flags & 0xf0 == 0:
            return
            
        offset = (offset + 1) & 0xfffffffe
        pack_into(self.vals, buffer, offset, f)
        return

class hkaSplineSkeletalAnimationObj2:
    Alignments = [4, 1, 2, 1, 2, 4]
    Type1 = structtuple("Type1", 'a', 'I')
    Type2 = structtuple("Type2", 'a', 'B', 'b', 'B', 'c', 'B', 'd', 'b', 'e', 'b') # should be bbbH, but for xbox conv it is bbbbb
    Type3 = structtuple("Type3", 'a', 'H', 'b', 'H', 'c', 'H')
    Type4 = structtuple("Type4", 'a', 'B', 'b', 'B', 'c', 'B')
    Type5 = structtuple("Type5", 'a', 'B', 'b', 'B')
    Type6 = structtuple("Type6", 'a', 'I', 'b', 'I', 'c', 'I', 'd', 'I')    
    Types = [Type1, Type2, Type3, Type4, Type5, Type6]
    
    @classmethod
    def unpack_from(Self, buffer, offset, flags, flag_, f="<"):
        offset_ = offset
        self = Self()
        if flags != 0:
            self.vals = []
            self.align = self.Alignments[flag_]
            
            if flags & 0xf0 != 0:
                self.s1 = unpack_from(Short[f], buffer, offset)
                offset += self.s1.nbytes
                self.s2 = unpack_from(Byte[f], buffer, offset)
                offset += self.s2.nbytes
                self.data = unpack_list_from(Byte[f], buffer, offset, self.s1['val'] + self.s2['val'] + 2)
                offset += self.data.nbytes
            else:
                self.s1 = new(Short[f], 0)

            offset = (offset + self.align - 1) & ~(np.int32(self.align) - 1)
            self.vals = unpack_list_from(self.Types[flag_][f], buffer, offset, self.s1['val'] + 1)
            offset += self.vals.nbytes

        offset = (offset + 3) & 0xfffffffc
        self.nbytes = offset - offset_
        return self
        
    def pack_into(self, buffer, offset, flags, flag_, f="<"):
        if flags != 0:
            if flags & 0xf0 != 0:
                offset += pack_into(self.s1, buffer, offset, f)
                offset += pack_into(self.s2, buffer, offset, f)
                offset += pack_into(self.data, buffer, offset, f)

            offset = (offset + self.align - 1) & ~(np.int32(self.align) - 1)
            pack_into(self.vals, buffer, offset, f)
        return

class hkaSplineSkeletalAnimation:
    Flags = structtuple("Flags", "f", "B", "a", "B", "b", "B", "c", "B")
    @classmethod
    def unpack_from(Self, buffer, offset, header, f="<"):
        self = Self()
        self.block_starts = unpack_list_from(Int[f], buffer, offset + header['block_starts_offset'], header['block_starts_num'])
        self.block_ends = unpack_list_from(Int[f], buffer, offset + header['block_ends_offset'], header['block_ends_num'])
        self.objC3 = unpack_list_from(Int[f], buffer, offset + header['objC3_offset'], header['objC3_num'])
        self.objC4 = unpack_list_from(Int[f], buffer, offset + header['objC4_offset'], header['objC4_num'])
        self.flags, self.vals_a, self.vals_b, self.vals_c = [], [], [], []
        for start in self.block_starts['val']:
            self.flags.append(unpack_list_from(self.Flags[f], buffer, offset + header['block_offset'] + start, header['keys_num']))
            off = offset + header['block_offset'] + start + header['data_offset']
            self.vals_a.append([])
            self.vals_b.append([])
            self.vals_c.append([])
            for flag in self.flags[-1]:
                self.vals_a[-1].append(hkaSplineSkeletalAnimationObj1.unpack_from(buffer, off, flag['a'], flag['f'] & 3, f))
                off += self.vals_a[-1][-1].nbytes
                self.vals_b[-1].append(hkaSplineSkeletalAnimationObj2.unpack_from(buffer, off, flag['b'], (flag['f'] >> 2) & 0xf, f))
                off += self.vals_b[-1][-1].nbytes
                self.vals_c[-1].append(hkaSplineSkeletalAnimationObj1.unpack_from(buffer, off, flag['c'], (flag['f'] >> 6) & 3, f))
                off += self.vals_c[-1][-1].nbytes
        return self
        
    def pack_into(self, buffer, offset, header, f="<"):
        pack_into(self.block_starts, buffer, offset + header['block_starts_offset'], f)
        pack_into(self.block_ends, buffer, offset + header['block_ends_offset'], f)
        pack_into(self.objC3, buffer, offset + header['objC3_offset'], f)
        pack_into(self.objC4, buffer, offset + header['objC4_offset'], f)
        for i, start in enumerate(self.block_starts['val']):
            pack_into(self.flags[i], buffer, offset + header['block_offset'] + start, f)
            off = offset + header['block_offset'] + start + header['data_offset']
            for j, flag in enumerate(self.flags[i]):
                self.vals_a[i][j].pack_into(buffer, off, flag['a'], flag['f'] & 3, f)
                off += self.vals_a[i][j].nbytes
                self.vals_b[i][j].pack_into(buffer, off, flag['b'], (flag['f'] >> 2) & 0xf, f)
                off += self.vals_b[i][j].nbytes
                self.vals_c[i][j].pack_into(buffer, off, flag['c'], (flag['f'] >> 6) & 3, f)
                off += self.vals_c[i][j].nbytes
        return

def get_vertex_format(fmt1, fmt2):
    fmt = []
    if fmt2 == 0:
        b1 = fmt1 & 0x40000 != 0
        s = 0
        if fmt1 & 1 != 0:
            fmt.append(Vector3 if not b1 else Vector4)
            s += 12 + 4*b1
        if fmt1 & 0x400 != 0:
            fmt.append(Color)
            s += 4
        if fmt1 & 0x800 != 0:
            fmt.append(Color)
            s += 4
        if fmt1 & 2:
            if b1:
                for _ in range(((s + 15) & 0xFFFF0) - s):
                    fmt.append('B')
                    s += 1
                fmt.append(Vector4)
                s += 16
            else:
                fmt.append(Color)
                s += 4        
        if fmt1 & 0x100 != 0:
            fmt.append(Color)
            s += 4
        if fmt1 & 0x200 != 0:
            fmt.append(Color)
            s += 4
        for _ in range((fmt1 >> 2) & 0xF):
            fmt.append(Vector2)
            s += 8
        if fmt1 & 0x40:
            if b1:
                for _ in range(((s + 15) & 0xFFFF0) - s):
                    fmt.append('B')
                    s += 1
                fmt.append(Vector4)
                s += 16
            else:
                fmt.append(Color)
                s += 4
        if fmt1 & 0x80:
            fmt.append(Vector3)
            s += 12
        if b1:
            for _ in range(((s + 15) & 0xFFFF0) - s):
                fmt.append('B')
    else:
        if fmt1 & 1 != 0:
            fmt.append(Vector3)
        if fmt1 & 0x400 != 0:
            fmt.append(Color)
        if fmt1 & 0x800 != 0:
            fmt.append(Color)
        if fmt1 & 2:
            fmt.append(Color)
        if fmt1 & 0x100 != 0:
            fmt.append(Color)
        if fmt1 & 0x200 != 0:
            fmt.append(Color)
        if (n := (fmt1 >> 2) & 0xf) <= 2:
            for _ in range(n):
                fmt.append(Vector2)
        if fmt1 & 0x40:
            fmt.append(Color)
        if fmt1 & 0x80:
            fmt.append(Vector3)
    return structtuple(f"VertexFormat_{fmt1:03X}_{fmt2:03X}", *[j for i in zip([f"f{i}" for i in range(len(fmt))], fmt) for j in i])

class VertexBuffer:
    @classmethod
    def unpack_from(Self, buffer, info, formats, f):
        fmt1, fmt2 = info[['fmt1', 'fmt2']]
        if (fmt := formats.get((fmt1, fmt2), None)) is None:
            fmt = get_vertex_format(fmt1, fmt2)
            formats[(fmt1, fmt2)] = fmt
        assert info['size'] % fmt[f].itemsize == 0
        n = info['size'] // fmt[f].itemsize
        self = Self()
        self.data = unpack_list_from(fmt[f], buffer, info['offset'], n)
        return self
    def pack_into(self, buffer, info, f):
        pack_into(self.data, buffer, info['offset'], f)

class IndexBuffer:
    @classmethod
    def unpack_from(Self, buffer, info, f):
        self = Self()
        if info['format'] == 0x10:
            fmt = Ushort
        else:
            fmt = Uint
        assert info['size'] % fmt[f].itemsize == 0
        n = info['size'] // fmt[f].itemsize
        self.data = unpack_list_from(fmt[f], buffer, info['offset'], n)
        return self
    def pack_into(self, buffer, info, f):
        pack_into(self.data, buffer, info['offset'], f)

class Obj14:
    # The pc version in the same as the xbox version except every list has two 0xFFFFFFFFs at the end (so list is 2 elems longer)
    @classmethod
    def unpack_from(Self, buffer, info, f="<"):
        self = Self()
        self.data = unpack_list_from(Uint[f], buffer, info['offset'], info['num'])
        return self
    def pack_into(self, buffer, info, f="<"):
        pack_into(self.data, buffer, info['offset'], f)
