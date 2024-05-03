# An overview of level file formats

- unless otherwise stated the xbox version seems to just be a big endian version of the pc version  
- fileds prefixed by key are a crc representing a debug string, not all correspond to values in the debug_strings for the file, but most do
- headers pointing to zlib blocks contain both the compressed size (size_comp) and uncompressed size (size). If size_comp is 0 then the data is assumed to be uncompressed
- For the simpler objects the format should be correct. For more complex objects (in particular mesh objects) more reverse engineering is needed since the associated data seems to vary (likely the object has different types that have not been found yet), although the current format should grab all the needed information for the level to work, just in a hacky way
- For textures in .bin files the "name*" block is the main texture data and "name" is the mipmap data if applicable, they are stored in .DDS format

> # *.dat
> layout:
> - header
> - unused_data
> - gamemodes
> - levels
> - string_keys
> - language_strings
> - debug_strings
>
> header:
> - const, u32, always 0x04
> - valA, u32
> - strings_offset, u32
> - strings_size, u32
> - strings_num, u32
> - string_keys_size, u32
> - string_keys_offset, u32
> - local_strings_size, u32
> - local_strings_offset, u32
> - gamemodes_num, u32
> - gamemodes_offset, u32
> - levels_num, u32
> - levels_offset, u32
> - valB, u32  
>
> gamemodes: 
    > - key, u32
    > - key_name, u32
    > - key_description, u32
>
> levels:
    > - name, 32 chars
    > - key_name, u32
    > - key_description, u32
    > - valA, u32
    > - gamemodes, u32

---

> # *.bin
> layout:
> - header
> - asset_handles, pointed to by header
> - assets (vbuffs, ibuffs, textures, radiosity), zlib compressed, pointed to by asset_handle
> - debug_strings, pointed to by header
>
> header:
> - constx06, u32
> - version, u32
> - strings_offset, u32
> - strings_size, u32
> - strings_num, u32
> - asset_handle_num, u32
> - asset_handle_offset, u32
> - unk_7, u32
> - unk_8, u32
> - unk_9, u32
> - unk_10, u32
> - unk_11, u32
> - unk_12, u32
> - unk_13, u32
> - unk_14, u32
> - unk_15, u32
> - unk_16, u32
> - unk_17, u32
> - unk_18, u32
> - unk_19, u32
> - unk_20, u32
> - unk_21, u32
> - unk_22, u32
> - unk_23, u32
> - unk_24, u32
> - unk_25, u32
> - unk_26, u32
> - unk_27, u32
> - unk_28, u32
> - unk_29, u32
> - unk_30, u32
> - unk_31, u32
> - unk_32, u32
> - unk_33, u32
> - unk_34, u32
> - unk_35, u32
> - unk_36, u32
> - unk_37, u32
> - unk_38, u32
> - unk_39, u32
> - unk_40, u32
> - unk_41, u32
> - unk_42, u32
>
> asset_handle:
> - key, u32
> - offset, u32
> - size, u32
> - size_comp, u32
> - type, u32
>
> vbuff:
> vbuff vals with a format determined by the corresponding vbuff_info, see code for details
>
> ibuff:
> u16s or u32s based on ibuff_info
>
> textures:
> mip-mapped or cube texture, see code for details
>
> radiosity:
> not yet reversed
---

> # *.pak
> layout:
> - header
> - animation_blocks (1 per gamemode?), zlib compressed, pointed to by animamation_block_info, see code for format (is packed animation objects)
> - block1, zlib compressed, pointed to by header
> - block2, zlib compressed, pointed to by header
> - debug_strings, pointed to by header
> - blockAs, pointed to by header
> 
> header:
> - blockA_num, u32 always little endian
> - blockA_offset, u32 always little endian
> - constx13, u32
> - version, u32
> - strings_offset, u32
> - strings_size, u32
> - strings_num, u32
> - block1_offset, u32
> - block1_size, u32
> - block1_size_comp, u32
> - sub_blocks1_offset, u32
> - block2_offset, u32
> - block2_size, u32
> - block2_size_comp, u32
> - sub_blocks2_offset, u32
> - string_keys_offset, u32
> - unk_16, u32
> - unk_17, u32
> - unk_18, u32
> - unk_19, u32
> - unk_20, u32
> - unk_21, u32
> - unk_22, u32
> - unk_23, u32
> - unk_24, u32
> - unk_25, u32
> - unk_26, u32
> - unk_27, u32
> - unk_28, u32
> - unk_29, u32
> - unk_30, u32
> - unk_31, u32
> - unk_32, u32
> - unk_33, u32
> - unk_34, u32
> - unk_35, u32
> - unk_36, u32
> - unk_37, u32
> - unk_38, u32
> - unk_39, u32
> - unk_40, u32
> - unk_41, u32
> - obja_num, u32
> - obj0_num, u32
> - mesh_info_num, u32, 1
> - buffer_info_num, u32, 2
> - mat1_num, u32
> - mat2_num, u32
> - mat3_num, u32
> - mat4_num, u32
> - objb_num, u32
> - unk_51, u32
> - objc_num, u32
> - hk_shape_info_num, u32, d
> - hk_constraint_data_num, u32, e
> - vbuff_info_num, u32, f
> - ibuff_info_num, u32, g
> - texture_info_num, u32, 7
> - animation_info_num, u32, 8
> - hk_constraint_info_num, u32, 9
> - game_objs_block_info_num, u32, 10
> - pfield_info_num, u32, 12
> - obj13_info_num, u32
> - animation_block_info_num, u32
> - obj11_num, u32
> - obj14_info_num, u32
> - unk_66, u32
> - obja_offset, u32, 24 bytes
> - obj0_offset, u32
> - mesh_info_offset, u32,256 bytes, max loaded is 0x400
> - buffer_info_offset, u32
> - mat1_offset, u32
> - mat2_offset, u32
> - mat3_offset, u32
> - mat4_offset, u32
> - objb_offset, u32
> - unk_76, u32
> - objc_offset, u32
> - hk_shape_info_offset, u32
> - hk_constraint_data_offset, u32
> - vbuff_info_offset, u32
> - ibuff_info_offset, u32
> - texture_info_offset, u32,0x12 bytes, max loaded is 0x800, related to MgSurfaceWin32
> - animation_info_offset, u32
> - hk_constraint_info_offset, u32
> - game_objs_block_info_offset, u32
> - pfield_info_offset, u32
> - obj13_info_offset, u32, 0xc bytes, max loaded is 0x40
> - animation_block_info_offset, u32, 36 bytes
> - obj11_offset, u32
> - obj14_info_offset, u32
> - unk_91, u32
> - unk_92, u32
> - unk_93, u32
> - unk_94, u32
> - unk_95, u32
> - unk_96, u32
> - unk_97, u32
> - unk_98, u32
> - unk_99, u32
> - unk_100, u32
> - unk_101, u32
> - unk_102, u32
> - unk_103, u32
> - unk_104, u32
> - unk_105, u32
> - unk_106, u32
> - unk_107, u32
> - unk_108, u32
> - unk_109, u32
> - unk_110, u32
> - unk_111, u32
> - unk_112, u32
> - unk_113, u32
> - unk_114, u32
> - unk_115, u32
> - block2_offsets_num, u32
> - block2_offsets_offset, u32
>
> blockA:
> - unk_0, u32
> - unk_1, u32
> - unk_2, u32
> - unk_3, u32
> - unk_4, u32
> - unk_5, u32
> - unk_6, u32
>
> block1 layout:
> - obja, pointed to by header
> - obj0, pointed to by header
> - mesh_info, pointed to by header
> - buffer_info, pointed to by header
> - mat1, pointed to by header
> - mat2, pointed to by header
> - mat3, pointed to by header
> - mat4, pointed to by header
> - objb, pointed to by header
> - objc, pointed to by header
> - hk_shape_info, pointed to by header
> - hk_constraint_data, pointed to by header
> - vbuff_info, pointed to by header
> - ibuff_info, pointed to by header
> - texture_info, pointed to by header
> - animation_info, pointed to by header
> - hk_constraint_info, pointed to by header
> - game_objs_block_info, pointed to by header
> - pfield_info, pointed to by header
> - obj13_info, pointed to by header
> - animation_block_info, pointed to by header
> - obj11, pointed to by header
> - obj14_info, pointed to by header
> - meshes, pointed to by mesh_info
> - hk_shapes, pointed to by buffer_info
> - hk_constraints, pointed to by hk_contraint_info
> - game_obj_blocks, pointed to by game_objs_block_info
> - obj13s, pointed to by obj13_info
> - obj14s, pointed to by obj14_info
> - sub_block1, pointed to by header
> - string_keys, pointed to by header
> 
> block2 layout:
> - sub_block2, pointed to by header
> - offsets, points to pointers (u32) in block1, pointed to by header
> (offsets of other things in block1)
>
> obja:
> - key, u32 always little endian
> - unk_1, u32 always little endian
> - unk_2, u32 always little endian
> - unk_3, u32 always little endian
> - unk_4, u32 always little endian
> - unk_5, u32 always little endian
>
> obj0:
> - unk_0, u32 always little endian
> - key, u32 always little endian
>
>  mesh_info:
> - key, u32
> - unk_1, u32
> - mat_offset, u32
> - buffer_info_offset, u32, pointer to buffer_info, uses mat_num of sequential objects
> - unk_4, u32
> - unk_5, u32
> - unk_6, u32
> - unk_7, u32
> - unk_8, u32
> - unk_9, u32
> - unk_10, u32
> - unk_11, u32
> - valCs_offset, u32, ints (c & 0x3fffffff is an index into the buffer_infos referenced by this object)
> - unk_13, u32, (v1, v2, v3, v4, v5) * 4 (up to unk_23), v1 is a starting offset to buffer_infos, v2 is the end offset
> - unk_14, u32
> - unk_15, u32
> - unk_16, u32
> - unk_17, u32
> - unk_18, u32
> - unk_19, u32
> - unk_20, u32
> - unk_21, u32
> - unk_22, u32
> - unk_23, u32
> - unk_24, u32
> - unk_25, u32
> - unk_26, u32
> - unk_27, u32
> - unk_28, u32
> - unk_29, u32
> - unk_30, u32
> - unk_31, u32
> - valCs_num, u32
> - mat_num, u32
> - keys_offset, u32, ints
> - indices_offset, u32
> - matrices_offset, u32, 16 ints (matrix?) for keys_num
> - keys_num, u32
> - valGs_offset, u32
> - valGs_num, u32
> - valIs_offset, u32
> - vbuff_offset, u32
> - vbuff_num, u32
> - ibuff_offset, u32
> - ibuff_num, u32
> - valDs_offset, u32, f_num * 8 ints
> - unk_46, u32
> - unk_47, u32
> - valJs_num, u32
> - valJs_offset, u32
> - block_offset, u32
> - valKs_offset, u32, not sure on the size, seems to be 36 ints
> - asset_key, u32, data in bin that is vertex & index buffer values
> - asset_type, u32
> - unk_54, u32
> - unk_55, u32
> - objc_offset, u32, optional pointer to objc
> - unk_57, u32
> - hkConstraintData_offset, u32, optional pointer to obje
> - unk_59, u32
> - hkConstraint_offset, u32, optional pointer to hkConstraint
> - keys2_offset, u32
> - keys2_order_offset, u32
> - valAs_offset, u32, 8 ints
>
> buffer_info:
> - vbuff_info_offset, u32, pointer to vbuff_info
> - vbuff_info_offset_2, u32, optional pointer to vbuff_info
> - vbuff_info_offset_3, u32, optional pointer to vbuff_info
> - unk_3, u32
> - unk_4, u32
> - unk_5, u32
> - unk_6, u32
> - unk_7, u32
> - unk_8, u32
> - unk_9, u32
> - unk_10, u32
> - unk_11, u32
> - unk_12, u32
> - unk_13, u32
> - unk_14, u32
> - unk_15, u32
> - unk_16, u32
> - unk_17, u32
> - unk_18, u32
> - unk_19, u32
> - unk_20, u32
> - unk_21, u32
> - unk_22, u32
> - unk_23, u32
> - unk_24, u32
> - unk_25, u32
> - unk_26, u32
> - unk_27, u32
> - unk_28, u32
> - unk_29, u32
> - unk_30, u32
> - unk_31, u32
> - v_size, u32
> - v_size_2, u32
> - v_size_3, u32
> - unk_35, u32
> - unk_36, u32
> - unk_37, u32
> - unk_38, u32
> - unk_39, u32
> - unk_40, u32
> - unk_41, u32
> - unk_42, u32
> - unk_43, u32
> - unk_44, u32
> - unk_45, u32
> - unk_46, u32
> - unk_47, u32
> - vbuff_size, u32
> - vbuff_size_2, u32
> - vbuff_size_3, u32
> - unk_51, u32
> - unk_52, u32
> - unk_53, u32
> - unk_54, u32
> - unk_55, u32
> - unk_56, u32
> - unk_57, u32
> - unk_58, u32
> - unk_59, u32
> - unk_60, u32
> - unk_61, u32
> - unk_62, u32
> - unk_63, u32
> - unk_64, u32
> - ibuff_info_offset, u32, poiner to ibuff_ingo
> - i_num, u32, number of indeices in ibuffer
> - unk_67, u32
> - unk_68, u32
> - unk_69, u32
> - unk_70, u32
> - tri_num, u32, number of objects(triangles) in ibufffer
> - unk_72, u32
> - unk_73, u32
> - unk_74, u32
> - unk_75, u32
> - unk_76, u32
> - unk_77, u32
> - unk_78, u32
> - unk_79, u32
> - unk_80, u32
> - unk_81, u32
> - unk_82, u32
> - unk_83, u32
> - unk_84, u32
> - unk_85, u32
> - unk_86, u32
> - unk_87, u32
> - unk_88, u32
>
> mat_base (materials):
> - unk_0, u32
> - unk_1, u32
> - tex_2, u32
> - tex_3, u32
> - tex_4, u32
> - tex_5, u32
> - tex_6, u32
> - tex_7, u32
> - tex_8, u32
> - tex_9, u32
> - tex_10, u32
> - tex_11, u32
> - tex_12, u32
> - tex_13, u32
> - tex_14, u32
> - tex_15, u32
> - tex_16, u32
> - tex_17, u32
> - unk_18, u32
> - unk_19, u32
> - unk_20, u32
> - unk_21, u32
> - unk_22, u32
> - unk_23, u32
> - unk_24, u32
> - unk_25, u32
> - unk_26, u32
> - unk_27, u32
> - unk_28, u32
> - unk_29, u32
> - unk_30, u32
> - unk_31, u32
> - unk_32, u32
> - unk_33, u32
> - z_34, u32
> - z_35, u32
> - z_36, u32
> - z_37, u32
> - z_38, u32
> - z_39, u32
> - unk_40, u32
> - unk_41, u32
> - unk_42, u32
> - unk_43, u32
> - unk_44, u32
> - unk_45, u32
> - unk_46, u32
> - unk_47, u32
> - unk_48, u32
> - unk_49, u32
> - flags, u64, (flags1 u32, flags2 u32) 
> - type, u32
> - unk_53, u32
> - unk_54, u16
> - side_flags, u16
> - unk_55, u32
> - unk_56, u32
> - unk_57, u32
> - unk_58, u32
> - unk_59, u32
> - unk_60, u32
> - unk_61, u32
> - unk_62, u32
> - unk_63, u32
> - unk_64, u32
> - unk_65, u32
> - unk_66, u32
> - unk_67, u32
> - unk_68, u32
> - unk_69, u32
> - unk_70, u32
> - unk_71, u32
> - unk_72, u32
> - unk_73, u32
> - unk_74, u32
> - unk_75, u32
> - unk_76, u32
> - unk_77, u32
> - unk_78, u32
> - unk_79, u32
> - unk_80, u32
> - unk_81, u32
> - unk_82, u32
> - unk_83, u32
> - unk_84, u32
> - unk_85, u32
> - unk_86, u32
> - key, u32
> - unk_88, u32
> - z_89, u32
>
> mat1: mat_base
>
> mat2: mat_base + 
> - unk_90, u32
> - unk_91, u32
> - unk_92, u32
> - unk_93, u32
> - unk_94, u32
> - unk_95, u32
> - unk_96, u32
> - unk_97, u32
> - unk_98, u32
> - unk_99, u32
> - unk_100, u32
> - unk_101, u32
> - unk_102, u32
> - unk_103, u32
> - unk_104, u32
> - unk_105, u32
> - unk_106, u32
> - unk_107, u32
> - unk_108, u32
> - unk_109, u32
> - unk_110, u32
> - unk_111, u32
> - unk_112, u32
> - unk_113, u32
> - unk_114, u32
> - unk_115, u32
> - unk_116, u32
> - unk_117, u32
> - unk_118, u32
> - unk_119, u32
> - unk_120, u16
> - unk_120_, u16
> - unk_121, u32
>
> mat3: mat_base + 
> - unk_90, u32
> - unk_91, u32
> - unk_92, u32
> - unk_93, u32
> - unk_94, u32
> - unk_95, u32
> - unk_96, u32
> - unk_97, u32
> - unk_98, u32
> - unk_99, u32
> - unk_100, u32
> - unk_101, u32
> - unk_102, u32
> - unk_103, u32
> - unk_104, u32
> - unk_105, u32
> - unk_106, u32
> - unk_107, u32
> - unk_108, u32
> - unk_109, u32
> - unk_110, u32
> - unk_111, u32
> - unk_112, u32
> - unk_113, u32
> - unk_114, u16
> - unk_114_, u16
> - unk_115, u32
>
> mat4: mat_base + 
> - unk_90, u32
> - unk_91, u32
> - unk_92, u32
> - unk_93, u32
> - unk_94, u32
> - unk_95, u32
> - unk_96, u32
> - unk_97, u32
> - unk_98, u32
> - unk_99, u32
> - unk_100, u32
> - unk_101, u32
> - unk_102, u32
> - unk_103, u32
> - unk_104, u32
> - unk_105, u32
> - unk_106, u32
> - unk_107, u32
> - unk_108, u32
> - unk_109, u32
> - unk_110, u32
> - unk_111, u32
> - unk_112, u32
> - unk_113, u32
> - unk_114, u32
> - unk_115, u32
> - unk_116, u32
> - unk_117, u32
> - unk_118, u32
> - unk_119, u32
> - unk_120, u32
> - unk_121, u32
> - unk_122, u32
> - unk_123, u32
> - unk_124, u32
> - unk_125, u32
> - unk_126, u32
> - unk_127, u32
> - unk_128, u32
> - unk_129, u32
> - unk_130, u32
> - unk_131, u32
> - unk_132, u32
> - unk_133, u32
> - unk_134, u32
> - unk_135, u32
> - unk_136, u32
> - unk_137, u32
> - unk_138, u32
> - unk_139, u32
> - unk_140, u32
> - unk_141, u32
> - unk_142, u32
> - unk_143, u32
> - unk_144, u32
> - unk_145, u32
>
> objb:
> - unk_0, u32
> - unk_1, u32
> - unk_2, u32
> - unk_3, u32
> - unk_4, u32
> - unk_5, u32
> - unk_6, u32
> - unk_7, u32
> - unk_8, u32
> - unk_9, u32
> - unk_10, u32
> - unk_11, u32
> - unk_12, u32
> - unk_13, u32
> - unk_14, u32
> - unk_15, u32
> - unk_16, u32
> - unk_17, u32
> - unk_18, u32
> - unk_19, u32
> - unk_20, u32
> - unk_21, u32
> - unk_22, u32
> - unk_23, u32
> - unk_24, u32
> - unk_25, u32
> - unk_26, u32
> - unk_27, u32
> - unk_28, u32
> - unk_29, u32
> - unk_30, u32
> - unk_31, u32
> - unk_32, u32
> - unk_33, u32
> - unk_34, u32
> - unk_35, u32
> - unk_36, u32
> - unk_37, u32
> - unk_38, u32
> - unk_39, u32
> - unk_40, u32
> - unk_41, u32
> - unk_42, u32
> - unk_43, u32
> - unk_44, u32
> - unk_45, u32
> - unk_46, u32
> - unk_47, u32
> - unk_48, u32
> - unk_49, u32
>
> objc (something to do with hkShapes):
> - size, u32, sometimes a pointer to something, otherwise the number of strings from the mesh_info pointing to this
> - type, u32, 0, 1, 2, 3, 4, 5
> - unk_2, u32
> - unk_3, u32
> - unk_4, u32
> - unk_5, u32
> - unk_6, u32
> - unk_7, u32
> - unk_8, u32
> - unk_9, u32
> - unk_10, u32
> - unk_11, u32
> - unk_12, u32
> - unk_13, u32
> - unk_14, u32
> - unk_15, u32
> - unk_16, u32
> - unk_17, u32
> - unk_18, u32
> - unk_19, u32
> - unk_20, u32
> - unk_21, u32
> - unk_22, u32
> - unk_23, u32
> - unk_24, u32
> - unk_25, u32
> - unk_26, u32
> - hkshape_num, u32
> - hkshape_offset, u32, pointer to hkshape
> - unk_29, u32
> - unk_30, u32
>
> hk_shape_info:
> - unk_0, 4 f32s
> - unk_4, 4 f32s
> - type, u32
> - unk_9, u32
> - a_num, u32
> - a_offset, u32
> - b_num, u32
> - b_offset, u32
> - c_num, u32
> - c_offset, u32
> - d_num, u32
> - d_offset, u32
> - e_num, u32
> - e_offset, u32
>
> hk_constraint_data:
> - type, u32
> - unk_1, u32
> - unk_2, u32
> - unk_3, u32
> - unk_4, u32
> - unk_5, u32
> - unk_6, u32
> - unk_7, u32
> - unk_8, u32
> - unk_9, u32
> - unk_10, u32
> - unk_11, u32
> - unk_12, u32
> - unk_13, u32
> - unk_14, u32
> - unk_15, u32
> - unk_16, u32
> - unk_17, u32
> - unk_18, u32
> - unk_19, u32
> - unk_20, u32
> - unk_21, u32
> - unk_22, u32
> - unk_23, u32
> - unk_24, u32
> - unk_25, u32
> - unk_26, u32
> - unk_27, u32
> - unk_28, u32
>
> vbuff_info (pc):
> - unk_0, u32
> - size, u32
> - unk_3, u32
> - offset, u32
> - fmt1, u32
> - fmt2, u32
> - unk_6, u32
> - unk_7, u32
> 
> vbuff_info (xbox):
    > - unk_0, u32
    > - size, u32
    > - unk_3, u32
    > - offset, u32
    > - fmt2, u32
    > - fmt1, u32
    > - unk_6, u32
    > - unk_7, u32
    > - unk_8, u32
    > - unk_9, u32
    > - unk_10, u32
    > - unk_11, u32
    > - unk_12, u32
    > - unk_13, u32
>
> ibuff_info (pc):
> - unk_0, u32
> - size, u32
> - format, u32, 0x10 -> u16 otherwise u32
> - unk_3, u32
> - offset, u32
> - unk_5, u32
>
> ibuff_info (xbox):
    > - unk_0, u32
    > - size, u32
    > - format, u32, 0x10 -> u16 otherwise u32
    > - unk_3, u32
    > - offset, u32
    > - unk_5, u32
    > - unk_6, u32
    > - unk_7, u32
    > - unk_8, u32
    > - unk_9, u32
    > - unk_10, u32
    > - unk_11, u32
    > - unk_12, u32
>
> texture_info:
> - key, u32
> - unk_1, u32
> - asset_key, u32
> - asset_type, u32
> - type, u32
> - format, u32
> - unk_6, u32
> - unk_7, u32
> - unk_8, u32
> - unk_9, u32
> - unk_10, u32
> - unk_11, u32
> - width, u16
> - height, u16
> - depth, u16
> - levels, u16
> - unk_16, 16 chars
>
> animation_info:
> - key, u32
> - block_flag, u32
> - offset, u32
> - size, u32
> - type, u32
> - unk_5, u32
> - keys_num, u32
> - something_num, u32
> - unk_8, u32
> - vala, u32
> - unk_10, u32
> - unk_11, u32
> - data_offset, u32
> - unk_13, u32
> - unk_14, u32
> - unk_15, u32
> - block_starts_offset, u32
> - block_starts_num, u32
> - block_ends_offset, u32
> - block_ends_num, u32
> - objC3_offset, u32
> - objC3_num, u32
> - objC4_offset, u32
> - objC4_num, u32
> - block_offset, u32
> - block_size, u32
> - obj3_num, u32
> - obj3_offset, u32
> - unk_28, u32
> - unk_29, u32
> - obj1_num, u32
> - keys_offset, u32
> - unk_32, u32
> - obj1_offset, u32
> - buffer_info_offset, u32
> - buffer_info_num, u32
> - obj5_offset, u32
>
> hk_constraint_info:
> - type, u32
> - shorts_offset, u32 
> - shorts_num, u32
> - strings_offset, u32
> - strings_num, u32
> - vals_offset, u32
> - vals_num, u32
> - unk_7, u32
> - unk_8, u32
> - unk_9, u32
> - keys_offset, u32
> - keys_num, u16
> - keys2_num, u16
> - keys2_offset, u32
> - unk_13, u32
> - unk_14, f32
> - unk_15, u32
> - unk_16, u32
> - unk_17, u32
>
> game_objs_block_info:
> - key, u32
> - unk_1, u32
> - offset, u32
> - size, u32
>
>  pfield_info:
 "key1, u32
> - key2, u32
> - width, u32
> - height, u32
> - offset, u32
>
> obj13_info (points to GFX blocks):
> - key, u32
> - offset, u32, offset pointing to something in block1
> - size, u32
> 
> animation_block_info:
> - key, u32
> - unk_1, u32
> - key_name, u32
> - offset, u32
> - size, u32
> - size_comp, u32
> - unk_6, u32
> - unk_7, u32
> - unk_8, u32
>
> obj11:
> - unk_0, u32
> - unk_1, u32
> - unk_2, u32
> - unk_3, u32
> - unk_4, u32
> - unk_5, u32
> - unk_6, u32
> - unk_7, u32
> - unk_8, u32
> - unk_9, u32
> - unk_10, u32
> - unk_11, u32
> - unk_12, u32
> - unk_13, u32
> - unk_14, u32
> - unk_15, u32
> - unk_16, u32
> - unk_17, u32
> - unk_18, u32
> - unk_19, u32
>
> obj14 (points to list of ints in block1)
> - unk_0, u32
> - num, u32
> - offset, u32
>
> mesh: see code  
> hk_shape: see code
> hk_constraint: see code
> animation: see code
> obj14: list of ints
> obj13: list of ints
>
---

# Common types:
> # sub_blocks
> layout:
> - header
> - block_header, immediately after header
> - blocks (pointed to by block_header)
>
> header:
    > - z0, u32
    > - block_num, u32
    > - z2, u32
    > - z3, u32
>
> block_header:
    > - key, u32
    > - offset, u32
    > - size, u32
> 
> block: could be a file (text, cvs, ssa, dat), compiled lua code, language_strings, game_objs, pfields, spray, crowd, atlasuv based on block_header.key

> # language_strings
> utf16 encoded strings seperated by 0u16

> # string_keys
> related to language strings, needs one element per string
> layout:
> - header
> - vals (key u32 and offset u32), number is numA
> - padding (u32 per val)
>
> header:
    > - numA, u16
    > - numB, u16, must equal numA (probably)
    > - z2, u32
    > - z3, u32
    > - z4, u32
    > - z5, u32


> # lua code
> lua bytecode in the L4404 format (B4404 for xbox)

> # game_objs
> holds variable format objects that define the level, contains the information for how the objects are layed out as well as the data for all obejects  
> see code for more details

> # spray
> - num_obj1
> - obj1s
> - num_obj2
> - obj2s
>
> obj1:
    > - key, u32
    > - unk_1, u32
    > - unk_2, u32
    > - unk_3, u32
    > - unk_4, u32
    > - unk_5, u32
    > - unk_6, u32
    > - unk_7, u32
    > - unk_8, u32
    > - unk_9, u32
    > - unk_10, u32
    > - unk_11, u32
    > - unk_12, u32
    > - unk_13, u32
    > - unk_14, u32
    > - unk_15, u32
    > - unk_16, u32
>
> obj2:
    > - unk_0", "f",
    > - unk_1", "f",
    > - unk_2", "f",
    > - unk_3", "f",
    > - unk_4", "f",

> # crowd
> - const, u32, must be 0x65
> - num, u32
> - headers, keys (u32) and vals, packed in that order number of keys and vals determined by header
> 
> header:
    > - key_0, u32
    > - key_1, u32
    > - key_2, u32
    > - key_3, u32
    > - unk_4, u32
    > - keys_num, u32
    > - num, u32
> 
> val:
    > - unk_0, u32
    > - unk_1, u32
    > - unk_2, u32
    > - unk_3, u32
    > - unk_4, u32

> # atlasuv
> contains vals
> vals:
    > - unk_0, u32
    > - unk_1", "f",
    > - unk_2", "f",
    > - unk_3", "f",
    > - unk_4", "f",

> # pfield
> seems to always be little endian, no idea on the format