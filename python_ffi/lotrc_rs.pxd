from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from "lotrc.h":

  const uint32_t LodInfo_UNKNOWN # = 1

  const uint32_t LodInfo_STATIC # = 2

  const uint32_t LodInfo_SKINNED # = 4

  const uint32_t LodInfo_PHYSICS # = 8

  const uint32_t LodInfo_BREAKABLE # = 16

  const uint32_t LodInfo_LOD0 # = 32

  const uint32_t LodInfo_LOD1 # = 64

  const uint32_t LodInfo_LOD2 # = 128

  const uint32_t LodInfo_LOD3 # = 256

  cdef enum:
    Version_Pc # = 0,
    Version_Xbox,
    Version_Ps3,
    Version_Err,
  ctypedef uint8_t Version;

  # Owned
  cdef struct OwnedAlignedBuf:
    pass

  # Owned
  cdef struct OwnedDumpInfosPc:
    pass

  # Owned
  cdef struct OwnedDumpInfosPs3:
    pass

  # Owned
  cdef struct OwnedDumpInfosXbox:
    pass

  # Owned
  cdef struct OwnedInfoCounts:
    pass

  # Owned
  cdef struct OwnedLevelCompressedData:
    pass

  # Owned
  cdef struct OwnedLevelData:
    pass

  # Owned
  cdef struct OwnedLevelRefPc:
    pass

  # Owned
  cdef struct OwnedLevelRefPs3:
    pass

  # Owned
  cdef struct OwnedLevelRefXbox:
    pass

  # Owned
  cdef struct OwnedVecCompressedData:
    pass

  cdef struct Vec______CompressedDataRef:
    pass

  cdef struct slice_AnimationBlockInfoPc:
    pass

  cdef struct slice_AnimationBlockInfoPs3:
    pass

  cdef struct slice_AnimationBlockInfoXbox:
    pass

  cdef struct slice_AnimationInfoPc:
    pass

  cdef struct slice_AnimationInfoPs3:
    pass

  cdef struct slice_AnimationInfoXbox:
    pass

  cdef struct slice_BufferInfoPc:
    pass

  cdef struct slice_BufferInfoPs3:
    pass

  cdef struct slice_BufferInfoXbox:
    pass

  cdef struct slice_EffectInfoPc:
    pass

  cdef struct slice_EffectInfoPs3:
    pass

  cdef struct slice_EffectInfoXbox:
    pass

  cdef struct slice_FoliageInfoPc:
    pass

  cdef struct slice_FoliageInfoPs3:
    pass

  cdef struct slice_FoliageInfoXbox:
    pass

  cdef struct slice_GFXBlockInfoPc:
    pass

  cdef struct slice_GFXBlockInfoPs3:
    pass

  cdef struct slice_GFXBlockInfoXbox:
    pass

  cdef struct slice_HkConstraintDataPc:
    pass

  cdef struct slice_HkConstraintDataPs3:
    pass

  cdef struct slice_HkConstraintDataXbox:
    pass

  cdef struct slice_HkConstraintInfoPc:
    pass

  cdef struct slice_HkConstraintInfoPs3:
    pass

  cdef struct slice_HkConstraintInfoXbox:
    pass

  cdef struct slice_HkShapeInfoPc:
    pass

  cdef struct slice_HkShapeInfoPs3:
    pass

  cdef struct slice_HkShapeInfoXbox:
    pass

  cdef struct slice_IBuffInfoPc:
    pass

  cdef struct slice_IBuffInfoPs3:
    pass

  cdef struct slice_IBuffInfoXbox:
    pass

  cdef struct slice_Mat1Pc:
    pass

  cdef struct slice_Mat1Ps3:
    pass

  cdef struct slice_Mat1Xbox:
    pass

  cdef struct slice_Mat2Pc:
    pass

  cdef struct slice_Mat2Ps3:
    pass

  cdef struct slice_Mat2Xbox:
    pass

  cdef struct slice_Mat3Pc:
    pass

  cdef struct slice_Mat3Ps3:
    pass

  cdef struct slice_Mat3Xbox:
    pass

  cdef struct slice_Mat4Pc:
    pass

  cdef struct slice_Mat4Ps3:
    pass

  cdef struct slice_Mat4Xbox:
    pass

  cdef struct slice_MatExtraPc:
    pass

  cdef struct slice_MatExtraPs3:
    pass

  cdef struct slice_MatExtraXbox:
    pass

  cdef struct slice_ModelInfoPc:
    pass

  cdef struct slice_ModelInfoPs3:
    pass

  cdef struct slice_ModelInfoXbox:
    pass

  cdef struct slice_Obj0Pc:
    pass

  cdef struct slice_Obj0Ps3:
    pass

  cdef struct slice_Obj0Xbox:
    pass

  cdef struct slice_ObjAPc:
    pass

  cdef struct slice_ObjAPs3:
    pass

  cdef struct slice_ObjAXbox:
    pass

  cdef struct slice_PFieldInfoPc:
    pass

  cdef struct slice_PFieldInfoPs3:
    pass

  cdef struct slice_PFieldInfoXbox:
    pass

  cdef struct slice_RadiosityValsInfoPc:
    pass

  cdef struct slice_RadiosityValsInfoPs3:
    pass

  cdef struct slice_RadiosityValsInfoXbox:
    pass

  cdef struct slice_ShapeInfoPc:
    pass

  cdef struct slice_ShapeInfoPs3:
    pass

  cdef struct slice_ShapeInfoXbox:
    pass

  cdef struct slice_TextureInfoPc:
    pass

  cdef struct slice_TextureInfoPs3:
    pass

  cdef struct slice_TextureInfoXbox:
    pass

  cdef struct slice_VBuffInfoPc:
    pass

  cdef struct slice_VBuffInfoPs3:
    pass

  cdef struct slice_VBuffInfoXbox:
    pass

  cdef struct slice_VertexUsage:
    pass

  cdef struct slice_u32:
    pass

  cdef struct slice_u32Pc:
    pass

  cdef struct slice_u32Ps3:
    pass

  cdef struct slice_u32Xbox:
    pass

  cdef struct slice_u8:
    pass

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_AlignmentHelper:
    uint64_t align;
    uint8_t pad[8];

  cdef struct AlignedBuf:
    slice_AlignmentHelper data;
    uintptr_t size;

  cdef struct LevelData:
    AlignedBuf pak;
    AlignedBuf bin;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_u8:
    uint64_t align;
    uint8_t pad[8];

  cdef struct CompressedDataRef:
    ref_slice_u8 data;
    AlignedBuf data_decomp;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_CompressedDataRef:
    uint64_t align;
    uint8_t pad[8];

  cdef struct PakCompressedData:
    CompressedDataRef block1;
    CompressedDataRef block2;
    slice_CompressedDataRef animations;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__CompressedDataRef:
    uint64_t align;
    uint8_t pad[64];

  cdef struct BinCompressedData:
    IndexMap_u32__CompressedDataRef model_data;
    IndexMap_u32__CompressedDataRef texture_data;

  cdef struct LevelCompressedData:
    PakCompressedData pak;
    BinCompressedData bin;

  ctypedef uint32_t u32_le;

  ctypedef u32_le u32Pc;

  #gen_ffi:export
  cdef struct PakHeaderPc:
    u32Pc block_a_num;
    u32Pc block_a_offset;
    u32Pc constx13;
    u32Pc version;
    u32Pc strings_offset;
    u32Pc strings_size;
    u32Pc strings_num;
    u32Pc block1_offset;
    u32Pc block1_size;
    u32Pc block1_size_comp;
    u32Pc sub_blocks1_offset;
    u32Pc block2_offset;
    u32Pc block2_size;
    u32Pc block2_size_comp;
    u32Pc sub_blocks2_offset;
    u32Pc string_keys_offset;
    u32Pc unk_16;
    u32Pc obja_size;
    u32Pc obj0_size;
    u32Pc model_info_size;
    u32Pc buffer_info_size;
    u32Pc mat1_size;
    u32Pc mat2_size;
    u32Pc mat3_size;
    u32Pc mat4_size;
    u32Pc mat_extra_size;
    u32Pc unk_26;
    u32Pc shape_info_size;
    u32Pc hk_shape_info_size;
    u32Pc hk_constraint_data_size;
    u32Pc vbuff_info_size;
    u32Pc ibuff_info_size;
    u32Pc texture_info_size;
    u32Pc animation_info_size;
    u32Pc hk_constraint_info_size;
    u32Pc effect_info_size;
    u32Pc pfield_info_size;
    u32Pc gfx_block_info_size;
    u32Pc animation_block_info_size;
    u32Pc foliage_info_size;
    u32Pc radiosity_vals_info_size;
    u32Pc unk_41;
    u32Pc obja_num;
    u32Pc obj0_num;
    u32Pc model_info_num;
    u32Pc buffer_info_num;
    u32Pc mat1_num;
    u32Pc mat2_num;
    u32Pc mat3_num;
    u32Pc mat4_num;
    u32Pc mat_extra_num;
    u32Pc unk_51;
    u32Pc shape_info_num;
    u32Pc hk_shape_info_num;
    u32Pc hk_constraint_data_num;
    u32Pc vbuff_info_num;
    u32Pc ibuff_info_num;
    u32Pc texture_info_num;
    u32Pc animation_info_num;
    u32Pc hk_constraint_info_num;
    u32Pc effect_info_num;
    u32Pc pfield_info_num;
    u32Pc gfx_block_info_num;
    u32Pc animation_block_info_num;
    u32Pc foliage_info_num;
    u32Pc radiosity_vals_info_num;
    u32Pc unk_66;
    u32Pc obja_offset;
    u32Pc obj0_offset;
    u32Pc model_info_offset;
    u32Pc buffer_info_offset;
    u32Pc mat1_offset;
    u32Pc mat2_offset;
    u32Pc mat3_offset;
    u32Pc mat4_offset;
    u32Pc mat_extra_offset;
    u32Pc unk_76;
    u32Pc shape_info_offset;
    u32Pc hk_shape_info_offset;
    u32Pc hk_constraint_data_offset;
    u32Pc vbuff_info_offset;
    u32Pc ibuff_info_offset;
    u32Pc texture_info_offset;
    u32Pc animation_info_offset;
    u32Pc hk_constraint_info_offset;
    u32Pc effect_info_offset;
    u32Pc pfield_info_offset;
    u32Pc gfx_block_info_offset;
    u32Pc animation_block_info_offset;
    u32Pc foliage_info_offset;
    u32Pc radiosity_vals_info_offset;
    u32Pc unk_91;
    u32Pc unk_92;
    u32Pc unk_93;
    u32Pc unk_94;
    u32Pc unk_95;
    u32Pc unk_96;
    u32Pc unk_97;
    u32Pc unk_98;
    u32Pc unk_99;
    u32Pc unk_100;
    u32Pc unk_101;
    u32Pc unk_102;
    u32Pc unk_103;
    u32Pc unk_104;
    u32Pc unk_105;
    u32Pc unk_106;
    u32Pc unk_107;
    u32Pc unk_108;
    u32Pc unk_109;
    u32Pc unk_110;
    u32Pc unk_111;
    u32Pc unk_112;
    u32Pc unk_113;
    u32Pc unk_114;
    u32Pc unk_115;
    u32Pc block2_offsets_num;
    u32Pc block2_offsets_offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_string:
    uint64_t align;
    uint8_t pad[8];

  ctypedef slice_string StringsRefPc;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_ObjAPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Obj0Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_ModelInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BufferInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat1Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat2Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat3Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat4Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_MatExtraPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_ShapeInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_HkShapeInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_HkConstraintDataPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_VBuffInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_IBuffInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_TextureInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AnimationInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_HkConstraintInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_EffectInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_PFieldInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_GFXBlockInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AnimationBlockInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_FoliageInfoPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RadiosityValsInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct InfosRefPc:
    ref_slice_ObjAPc objas;
    ref_slice_Obj0Pc obj0s;
    ref_slice_ModelInfoPc models;
    ref_slice_BufferInfoPc buffers;
    ref_slice_Mat1Pc mat1s;
    ref_slice_Mat2Pc mat2s;
    ref_slice_Mat3Pc mat3s;
    ref_slice_Mat4Pc mat4s;
    ref_slice_MatExtraPc mat_extras;
    ref_slice_ShapeInfoPc shapes;
    ref_slice_HkShapeInfoPc hk_shapes;
    ref_slice_HkConstraintDataPc hk_constraint_datas;
    ref_slice_VBuffInfoPc vbuffs;
    ref_slice_IBuffInfoPc ibuffs;
    ref_slice_TextureInfoPc textures;
    ref_slice_AnimationInfoPc animations;
    ref_slice_HkConstraintInfoPc hk_constraints;
    ref_slice_EffectInfoPc effects;
    ref_slice_PFieldInfoPc pfields;
    ref_slice_GFXBlockInfoPc gfxs;
    ref_slice_AnimationBlockInfoPc animation_blocks;
    ref_slice_FoliageInfoPc foliages;
    ref_slice_RadiosityValsInfoPc radiosity_vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__TextureRefPc:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__ModelRefPc:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__EffectRefPc:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__slice_FoliageRefPc:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__ref_slice_u8:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__RadiosityValsRefPc:
    uint64_t align;
    uint8_t pad[64];

  cdef struct RadiosityRefPc:
    const CompressedDataRef *data;
    IndexMap_u32__RadiosityValsRefPc vals;
    uint32_t usage;

  cdef struct ObjsRefPc:
    IndexMap_u32__TextureRefPc textures;
    IndexMap_u32__ModelRefPc models;
    IndexMap_u32__EffectRefPc effects;
    IndexMap_u32__slice_FoliageRefPc foliages;
    IndexMap_u32__ref_slice_u8 gfxs;
    RadiosityRefPc radiosity;

  #gen_ffi:export
  cdef struct SubBlocksHeaderPc:
    u32Pc z0;
    u32Pc block_num;
    u32Pc z2;
    u32Pc z3;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SubBlocksBlockHeaderPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct SubBlocksInfoRefPc:
    const SubBlocksHeaderPc *header;
    ref_slice_SubBlocksBlockHeaderPc block_headers;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__DataRefPc:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__LuaRefPc:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__SSARefPc:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_AtlasUVRefPc:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct GameObjsHeaderPc:
    u32Pc const_;
    u32Pc types_num;
    u32Pc types_offset;
    u32Pc obj_num;
    u32Pc obj_offset;
    u32Pc z5;
    u32Pc z6;
    u32Pc z7;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__TypeRefPc:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__ObjRefPc:
    uint64_t align;
    uint8_t pad[64];

  cdef struct GameObjsRefPc:
    const GameObjsHeaderPc *header;
    IndexMap_u32__TypeRefPc types;
    IndexMap_u32__ObjRefPc objs;

  cdef struct SubBlocks1RefPc:
    SubBlocksInfoRefPc info;
    IndexMap_u32__DataRefPc files;
    IndexMap_u32__LuaRefPc lua;
    IndexMap_u32__SSARefPc subtitles;
    Option_AtlasUVRefPc atlas1;
    Option_AtlasUVRefPc atlas2;
    GameObjsRefPc level;

  ctypedef uint16_t u16_le;

  ctypedef u16_le u16Pc;

  #gen_ffi:export
  cdef struct StringKeysHeaderPc:
    u16Pc num_a;
    u16Pc num_b;
    u32Pc z2;
    u32Pc z3;
    u32Pc z4;
    u32Pc z5;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_StringKeysValPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_u32Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct StringKeysRefPc:
    const StringKeysHeaderPc *header;
    ref_slice_StringKeysValPc vals;
    ref_slice_u32Pc pad;

  cdef struct Block1RefPc:
    InfosRefPc infos;
    ObjsRefPc objs;
    SubBlocks1RefPc sub_blocks;
    StringKeysRefPc string_keys;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_SprayRefPc:
    uint64_t align;
    uint8_t pad[24];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_CrowdRefPc:
    uint64_t align;
    uint8_t pad[32];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_PFieldsRefPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__LangStringsRefPc:
    uint64_t align;
    uint8_t pad[64];

  cdef struct SubBlocks2RefPc:
    SubBlocksInfoRefPc info;
    Option_SprayRefPc spray;
    Option_CrowdRefPc crowd;
    Option_PFieldsRefPc pfields;
    IndexMap_u32__LangStringsRefPc langs;
    IndexMap_u32__DataRefPc files;

  cdef struct Block2RefPc:
    SubBlocks2RefPc sub_blocks;
    ref_slice_u32Pc offsets;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__AnimationRefPc:
    uint64_t align;
    uint8_t pad[64];

  cdef struct AnimationsRefPc:
    IndexMap_u32__AnimationRefPc animations;
    ref_slice_AnimationBlockInfoPc block_infos;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BlockAValPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_CompressedDataRef:
    uint64_t align;
    uint8_t pad[8];

  cdef struct PakRefPc:
    const PakHeaderPc *header;
    StringsRefPc strings;
    Block1RefPc block1;
    Block2RefPc block2;
    AnimationsRefPc animations;
    ref_slice_BlockAValPc vals_a;
    ref_slice_CompressedDataRef animation_data;

  #gen_ffi:export
  cdef struct BinHeaderPc:
    u32Pc constx06;
    u32Pc version;
    u32Pc strings_offset;
    u32Pc strings_size;
    u32Pc strings_num;
    u32Pc asset_handle_num;
    u32Pc asset_handle_offset;
    u32Pc unk_7;
    u32Pc vdata_num;
    u32Pc vdata_num_alt;
    u32Pc texdata_num;
    u32Pc unk_11;
    u32Pc unk_12;
    u32Pc unk_13;
    u32Pc unk_14;
    u32Pc unk_15;
    u32Pc unk_16;
    u32Pc unk_17;
    u32Pc unk_18;
    u32Pc unk_19;
    u32Pc unk_20;
    u32Pc unk_21;
    u32Pc unk_22;
    u32Pc unk_23;
    u32Pc unk_24;
    u32Pc unk_25;
    u32Pc unk_26;
    u32Pc unk_27;
    u32Pc unk_28;
    u32Pc unk_29;
    u32Pc unk_30;
    u32Pc unk_31;
    u32Pc unk_32;
    u32Pc unk_33;
    u32Pc unk_34;
    u32Pc unk_35;
    u32Pc unk_36;
    u32Pc unk_37;
    u32Pc unk_38;
    u32Pc unk_39;
    u32Pc unk_40;
    u32Pc unk_41;
    u32Pc unk_42;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AssetHandlePc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32_______CompressedDataRef:
    uint64_t align;
    uint8_t pad[64];

  cdef struct BinRefPc:
    const BinHeaderPc *header;
    StringsRefPc strings;
    ref_slice_AssetHandlePc asset_handles;
    IndexMap_u32_______CompressedDataRef model_data;
    IndexMap_u32_______CompressedDataRef texture_data;

  cdef struct LevelRefPc:
    PakRefPc pak;
    BinRefPc bin;

  ctypedef uint32_t u32_be;

  ctypedef u32_be u32Xbox;

  #gen_ffi:export
  cdef struct PakHeaderXbox:
    u32Xbox block_a_num;
    u32Xbox block_a_offset;
    u32Xbox constx13;
    u32Xbox version;
    u32Xbox strings_offset;
    u32Xbox strings_size;
    u32Xbox strings_num;
    u32Xbox block1_offset;
    u32Xbox block1_size;
    u32Xbox block1_size_comp;
    u32Xbox sub_blocks1_offset;
    u32Xbox block2_offset;
    u32Xbox block2_size;
    u32Xbox block2_size_comp;
    u32Xbox sub_blocks2_offset;
    u32Xbox string_keys_offset;
    u32Xbox unk_16;
    u32Xbox obja_size;
    u32Xbox obj0_size;
    u32Xbox model_info_size;
    u32Xbox buffer_info_size;
    u32Xbox mat1_size;
    u32Xbox mat2_size;
    u32Xbox mat3_size;
    u32Xbox mat4_size;
    u32Xbox mat_extra_size;
    u32Xbox unk_26;
    u32Xbox shape_info_size;
    u32Xbox hk_shape_info_size;
    u32Xbox hk_constraint_data_size;
    u32Xbox vbuff_info_size;
    u32Xbox ibuff_info_size;
    u32Xbox texture_info_size;
    u32Xbox animation_info_size;
    u32Xbox hk_constraint_info_size;
    u32Xbox effect_info_size;
    u32Xbox pfield_info_size;
    u32Xbox gfx_block_info_size;
    u32Xbox animation_block_info_size;
    u32Xbox foliage_info_size;
    u32Xbox radiosity_vals_info_size;
    u32Xbox unk_41;
    u32Xbox obja_num;
    u32Xbox obj0_num;
    u32Xbox model_info_num;
    u32Xbox buffer_info_num;
    u32Xbox mat1_num;
    u32Xbox mat2_num;
    u32Xbox mat3_num;
    u32Xbox mat4_num;
    u32Xbox mat_extra_num;
    u32Xbox unk_51;
    u32Xbox shape_info_num;
    u32Xbox hk_shape_info_num;
    u32Xbox hk_constraint_data_num;
    u32Xbox vbuff_info_num;
    u32Xbox ibuff_info_num;
    u32Xbox texture_info_num;
    u32Xbox animation_info_num;
    u32Xbox hk_constraint_info_num;
    u32Xbox effect_info_num;
    u32Xbox pfield_info_num;
    u32Xbox gfx_block_info_num;
    u32Xbox animation_block_info_num;
    u32Xbox foliage_info_num;
    u32Xbox radiosity_vals_info_num;
    u32Xbox unk_66;
    u32Xbox obja_offset;
    u32Xbox obj0_offset;
    u32Xbox model_info_offset;
    u32Xbox buffer_info_offset;
    u32Xbox mat1_offset;
    u32Xbox mat2_offset;
    u32Xbox mat3_offset;
    u32Xbox mat4_offset;
    u32Xbox mat_extra_offset;
    u32Xbox unk_76;
    u32Xbox shape_info_offset;
    u32Xbox hk_shape_info_offset;
    u32Xbox hk_constraint_data_offset;
    u32Xbox vbuff_info_offset;
    u32Xbox ibuff_info_offset;
    u32Xbox texture_info_offset;
    u32Xbox animation_info_offset;
    u32Xbox hk_constraint_info_offset;
    u32Xbox effect_info_offset;
    u32Xbox pfield_info_offset;
    u32Xbox gfx_block_info_offset;
    u32Xbox animation_block_info_offset;
    u32Xbox foliage_info_offset;
    u32Xbox radiosity_vals_info_offset;
    u32Xbox unk_91;
    u32Xbox unk_92;
    u32Xbox unk_93;
    u32Xbox unk_94;
    u32Xbox unk_95;
    u32Xbox unk_96;
    u32Xbox unk_97;
    u32Xbox unk_98;
    u32Xbox unk_99;
    u32Xbox unk_100;
    u32Xbox unk_101;
    u32Xbox unk_102;
    u32Xbox unk_103;
    u32Xbox unk_104;
    u32Xbox unk_105;
    u32Xbox unk_106;
    u32Xbox unk_107;
    u32Xbox unk_108;
    u32Xbox unk_109;
    u32Xbox unk_110;
    u32Xbox unk_111;
    u32Xbox unk_112;
    u32Xbox unk_113;
    u32Xbox unk_114;
    u32Xbox unk_115;
    u32Xbox block2_offsets_num;
    u32Xbox block2_offsets_offset;

  ctypedef slice_string StringsRefXbox;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_ObjAXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Obj0Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_ModelInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BufferInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat1Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat2Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat3Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat4Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_MatExtraXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_ShapeInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_HkShapeInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_HkConstraintDataXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_VBuffInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_IBuffInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_TextureInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AnimationInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_HkConstraintInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_EffectInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_PFieldInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_GFXBlockInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AnimationBlockInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_FoliageInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RadiosityValsInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct InfosRefXbox:
    ref_slice_ObjAXbox objas;
    ref_slice_Obj0Xbox obj0s;
    ref_slice_ModelInfoXbox models;
    ref_slice_BufferInfoXbox buffers;
    ref_slice_Mat1Xbox mat1s;
    ref_slice_Mat2Xbox mat2s;
    ref_slice_Mat3Xbox mat3s;
    ref_slice_Mat4Xbox mat4s;
    ref_slice_MatExtraXbox mat_extras;
    ref_slice_ShapeInfoXbox shapes;
    ref_slice_HkShapeInfoXbox hk_shapes;
    ref_slice_HkConstraintDataXbox hk_constraint_datas;
    ref_slice_VBuffInfoXbox vbuffs;
    ref_slice_IBuffInfoXbox ibuffs;
    ref_slice_TextureInfoXbox textures;
    ref_slice_AnimationInfoXbox animations;
    ref_slice_HkConstraintInfoXbox hk_constraints;
    ref_slice_EffectInfoXbox effects;
    ref_slice_PFieldInfoXbox pfields;
    ref_slice_GFXBlockInfoXbox gfxs;
    ref_slice_AnimationBlockInfoXbox animation_blocks;
    ref_slice_FoliageInfoXbox foliages;
    ref_slice_RadiosityValsInfoXbox radiosity_vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__TextureRefXbox:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__ModelRefXbox:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__EffectRefXbox:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__slice_FoliageRefXbox:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__RadiosityValsRefXbox:
    uint64_t align;
    uint8_t pad[64];

  cdef struct RadiosityRefXbox:
    const CompressedDataRef *data;
    IndexMap_u32__RadiosityValsRefXbox vals;
    uint32_t usage;

  cdef struct ObjsRefXbox:
    IndexMap_u32__TextureRefXbox textures;
    IndexMap_u32__ModelRefXbox models;
    IndexMap_u32__EffectRefXbox effects;
    IndexMap_u32__slice_FoliageRefXbox foliages;
    IndexMap_u32__ref_slice_u8 gfxs;
    RadiosityRefXbox radiosity;

  #gen_ffi:export
  cdef struct SubBlocksHeaderXbox:
    u32Xbox z0;
    u32Xbox block_num;
    u32Xbox z2;
    u32Xbox z3;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SubBlocksBlockHeaderXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct SubBlocksInfoRefXbox:
    const SubBlocksHeaderXbox *header;
    ref_slice_SubBlocksBlockHeaderXbox block_headers;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__DataRefXbox:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__LuaRefXbox:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__SSARefXbox:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_AtlasUVRefXbox:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct GameObjsHeaderXbox:
    u32Xbox const_;
    u32Xbox types_num;
    u32Xbox types_offset;
    u32Xbox obj_num;
    u32Xbox obj_offset;
    u32Xbox z5;
    u32Xbox z6;
    u32Xbox z7;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__TypeRefXbox:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__ObjRefXbox:
    uint64_t align;
    uint8_t pad[64];

  cdef struct GameObjsRefXbox:
    const GameObjsHeaderXbox *header;
    IndexMap_u32__TypeRefXbox types;
    IndexMap_u32__ObjRefXbox objs;

  cdef struct SubBlocks1RefXbox:
    SubBlocksInfoRefXbox info;
    IndexMap_u32__DataRefXbox files;
    IndexMap_u32__LuaRefXbox lua;
    IndexMap_u32__SSARefXbox subtitles;
    Option_AtlasUVRefXbox atlas1;
    Option_AtlasUVRefXbox atlas2;
    GameObjsRefXbox level;

  ctypedef uint16_t u16_be;

  ctypedef u16_be u16Xbox;

  #gen_ffi:export
  cdef struct StringKeysHeaderXbox:
    u16Xbox num_a;
    u16Xbox num_b;
    u32Xbox z2;
    u32Xbox z3;
    u32Xbox z4;
    u32Xbox z5;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_StringKeysValXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_u32Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct StringKeysRefXbox:
    const StringKeysHeaderXbox *header;
    ref_slice_StringKeysValXbox vals;
    ref_slice_u32Xbox pad;

  cdef struct Block1RefXbox:
    InfosRefXbox infos;
    ObjsRefXbox objs;
    SubBlocks1RefXbox sub_blocks;
    StringKeysRefXbox string_keys;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_SprayRefXbox:
    uint64_t align;
    uint8_t pad[24];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_CrowdRefXbox:
    uint64_t align;
    uint8_t pad[32];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_PFieldsRefXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__LangStringsRefXbox:
    uint64_t align;
    uint8_t pad[64];

  cdef struct SubBlocks2RefXbox:
    SubBlocksInfoRefXbox info;
    Option_SprayRefXbox spray;
    Option_CrowdRefXbox crowd;
    Option_PFieldsRefXbox pfields;
    IndexMap_u32__LangStringsRefXbox langs;
    IndexMap_u32__DataRefXbox files;

  cdef struct Block2RefXbox:
    SubBlocks2RefXbox sub_blocks;
    ref_slice_u32Xbox offsets;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__AnimationRefXbox:
    uint64_t align;
    uint8_t pad[64];

  cdef struct AnimationsRefXbox:
    IndexMap_u32__AnimationRefXbox animations;
    ref_slice_AnimationBlockInfoXbox block_infos;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BlockAValXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct PakRefXbox:
    const PakHeaderXbox *header;
    StringsRefXbox strings;
    Block1RefXbox block1;
    Block2RefXbox block2;
    AnimationsRefXbox animations;
    ref_slice_BlockAValXbox vals_a;
    ref_slice_CompressedDataRef animation_data;

  #gen_ffi:export
  cdef struct BinHeaderXbox:
    u32Xbox constx06;
    u32Xbox version;
    u32Xbox strings_offset;
    u32Xbox strings_size;
    u32Xbox strings_num;
    u32Xbox asset_handle_num;
    u32Xbox asset_handle_offset;
    u32Xbox unk_7;
    u32Xbox vdata_num;
    u32Xbox vdata_num_alt;
    u32Xbox texdata_num;
    u32Xbox unk_11;
    u32Xbox unk_12;
    u32Xbox unk_13;
    u32Xbox unk_14;
    u32Xbox unk_15;
    u32Xbox unk_16;
    u32Xbox unk_17;
    u32Xbox unk_18;
    u32Xbox unk_19;
    u32Xbox unk_20;
    u32Xbox unk_21;
    u32Xbox unk_22;
    u32Xbox unk_23;
    u32Xbox unk_24;
    u32Xbox unk_25;
    u32Xbox unk_26;
    u32Xbox unk_27;
    u32Xbox unk_28;
    u32Xbox unk_29;
    u32Xbox unk_30;
    u32Xbox unk_31;
    u32Xbox unk_32;
    u32Xbox unk_33;
    u32Xbox unk_34;
    u32Xbox unk_35;
    u32Xbox unk_36;
    u32Xbox unk_37;
    u32Xbox unk_38;
    u32Xbox unk_39;
    u32Xbox unk_40;
    u32Xbox unk_41;
    u32Xbox unk_42;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AssetHandleXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BinRefXbox:
    const BinHeaderXbox *header;
    StringsRefXbox strings;
    ref_slice_AssetHandleXbox asset_handles;
    IndexMap_u32_______CompressedDataRef model_data;
    IndexMap_u32_______CompressedDataRef texture_data;

  cdef struct LevelRefXbox:
    PakRefXbox pak;
    BinRefXbox bin;

  ctypedef u32_be u32Ps3;

  #gen_ffi:export
  cdef struct PakHeaderPs3:
    u32Ps3 block_a_num;
    u32Ps3 block_a_offset;
    u32Ps3 constx13;
    u32Ps3 version;
    u32Ps3 strings_offset;
    u32Ps3 strings_size;
    u32Ps3 strings_num;
    u32Ps3 block1_offset;
    u32Ps3 block1_size;
    u32Ps3 block1_size_comp;
    u32Ps3 sub_blocks1_offset;
    u32Ps3 block2_offset;
    u32Ps3 block2_size;
    u32Ps3 block2_size_comp;
    u32Ps3 sub_blocks2_offset;
    u32Ps3 string_keys_offset;
    u32Ps3 unk_16;
    u32Ps3 obja_size;
    u32Ps3 obj0_size;
    u32Ps3 model_info_size;
    u32Ps3 buffer_info_size;
    u32Ps3 mat1_size;
    u32Ps3 mat2_size;
    u32Ps3 mat3_size;
    u32Ps3 mat4_size;
    u32Ps3 mat_extra_size;
    u32Ps3 unk_26;
    u32Ps3 shape_info_size;
    u32Ps3 hk_shape_info_size;
    u32Ps3 hk_constraint_data_size;
    u32Ps3 vbuff_info_size;
    u32Ps3 ibuff_info_size;
    u32Ps3 texture_info_size;
    u32Ps3 animation_info_size;
    u32Ps3 hk_constraint_info_size;
    u32Ps3 effect_info_size;
    u32Ps3 pfield_info_size;
    u32Ps3 gfx_block_info_size;
    u32Ps3 animation_block_info_size;
    u32Ps3 foliage_info_size;
    u32Ps3 radiosity_vals_info_size;
    u32Ps3 unk_41;
    u32Ps3 obja_num;
    u32Ps3 obj0_num;
    u32Ps3 model_info_num;
    u32Ps3 buffer_info_num;
    u32Ps3 mat1_num;
    u32Ps3 mat2_num;
    u32Ps3 mat3_num;
    u32Ps3 mat4_num;
    u32Ps3 mat_extra_num;
    u32Ps3 unk_51;
    u32Ps3 shape_info_num;
    u32Ps3 hk_shape_info_num;
    u32Ps3 hk_constraint_data_num;
    u32Ps3 vbuff_info_num;
    u32Ps3 ibuff_info_num;
    u32Ps3 texture_info_num;
    u32Ps3 animation_info_num;
    u32Ps3 hk_constraint_info_num;
    u32Ps3 effect_info_num;
    u32Ps3 pfield_info_num;
    u32Ps3 gfx_block_info_num;
    u32Ps3 animation_block_info_num;
    u32Ps3 foliage_info_num;
    u32Ps3 radiosity_vals_info_num;
    u32Ps3 unk_66;
    u32Ps3 obja_offset;
    u32Ps3 obj0_offset;
    u32Ps3 model_info_offset;
    u32Ps3 buffer_info_offset;
    u32Ps3 mat1_offset;
    u32Ps3 mat2_offset;
    u32Ps3 mat3_offset;
    u32Ps3 mat4_offset;
    u32Ps3 mat_extra_offset;
    u32Ps3 unk_76;
    u32Ps3 shape_info_offset;
    u32Ps3 hk_shape_info_offset;
    u32Ps3 hk_constraint_data_offset;
    u32Ps3 vbuff_info_offset;
    u32Ps3 ibuff_info_offset;
    u32Ps3 texture_info_offset;
    u32Ps3 animation_info_offset;
    u32Ps3 hk_constraint_info_offset;
    u32Ps3 effect_info_offset;
    u32Ps3 pfield_info_offset;
    u32Ps3 gfx_block_info_offset;
    u32Ps3 animation_block_info_offset;
    u32Ps3 foliage_info_offset;
    u32Ps3 radiosity_vals_info_offset;
    u32Ps3 unk_91;
    u32Ps3 unk_92;
    u32Ps3 unk_93;
    u32Ps3 unk_94;
    u32Ps3 unk_95;
    u32Ps3 unk_96;
    u32Ps3 unk_97;
    u32Ps3 unk_98;
    u32Ps3 unk_99;
    u32Ps3 unk_100;
    u32Ps3 unk_101;
    u32Ps3 unk_102;
    u32Ps3 unk_103;
    u32Ps3 unk_104;
    u32Ps3 unk_105;
    u32Ps3 unk_106;
    u32Ps3 unk_107;
    u32Ps3 unk_108;
    u32Ps3 unk_109;
    u32Ps3 unk_110;
    u32Ps3 unk_111;
    u32Ps3 unk_112;
    u32Ps3 unk_113;
    u32Ps3 unk_114;
    u32Ps3 unk_115;
    u32Ps3 block2_offsets_num;
    u32Ps3 block2_offsets_offset;

  ctypedef slice_string StringsRefPs3;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_ObjAPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Obj0Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_ModelInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BufferInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat1Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat2Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat3Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Mat4Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_MatExtraPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_ShapeInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_HkShapeInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_HkConstraintDataPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_VBuffInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_IBuffInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_TextureInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AnimationInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_HkConstraintInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_EffectInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_PFieldInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_GFXBlockInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AnimationBlockInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_FoliageInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RadiosityValsInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct InfosRefPs3:
    ref_slice_ObjAPs3 objas;
    ref_slice_Obj0Ps3 obj0s;
    ref_slice_ModelInfoPs3 models;
    ref_slice_BufferInfoPs3 buffers;
    ref_slice_Mat1Ps3 mat1s;
    ref_slice_Mat2Ps3 mat2s;
    ref_slice_Mat3Ps3 mat3s;
    ref_slice_Mat4Ps3 mat4s;
    ref_slice_MatExtraPs3 mat_extras;
    ref_slice_ShapeInfoPs3 shapes;
    ref_slice_HkShapeInfoPs3 hk_shapes;
    ref_slice_HkConstraintDataPs3 hk_constraint_datas;
    ref_slice_VBuffInfoPs3 vbuffs;
    ref_slice_IBuffInfoPs3 ibuffs;
    ref_slice_TextureInfoPs3 textures;
    ref_slice_AnimationInfoPs3 animations;
    ref_slice_HkConstraintInfoPs3 hk_constraints;
    ref_slice_EffectInfoPs3 effects;
    ref_slice_PFieldInfoPs3 pfields;
    ref_slice_GFXBlockInfoPs3 gfxs;
    ref_slice_AnimationBlockInfoPs3 animation_blocks;
    ref_slice_FoliageInfoPs3 foliages;
    ref_slice_RadiosityValsInfoPs3 radiosity_vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__TextureRefPs3:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__ModelRefPs3:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__EffectRefPs3:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__slice_FoliageRefPs3:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__RadiosityValsRefPs3:
    uint64_t align;
    uint8_t pad[64];

  cdef struct RadiosityRefPs3:
    const CompressedDataRef *data;
    IndexMap_u32__RadiosityValsRefPs3 vals;
    uint32_t usage;

  cdef struct ObjsRefPs3:
    IndexMap_u32__TextureRefPs3 textures;
    IndexMap_u32__ModelRefPs3 models;
    IndexMap_u32__EffectRefPs3 effects;
    IndexMap_u32__slice_FoliageRefPs3 foliages;
    IndexMap_u32__ref_slice_u8 gfxs;
    RadiosityRefPs3 radiosity;

  #gen_ffi:export
  cdef struct SubBlocksHeaderPs3:
    u32Ps3 z0;
    u32Ps3 block_num;
    u32Ps3 z2;
    u32Ps3 z3;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SubBlocksBlockHeaderPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct SubBlocksInfoRefPs3:
    const SubBlocksHeaderPs3 *header;
    ref_slice_SubBlocksBlockHeaderPs3 block_headers;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__DataRefPs3:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__LuaRefPs3:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__SSARefPs3:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_AtlasUVRefPs3:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct GameObjsHeaderPs3:
    u32Ps3 const_;
    u32Ps3 types_num;
    u32Ps3 types_offset;
    u32Ps3 obj_num;
    u32Ps3 obj_offset;
    u32Ps3 z5;
    u32Ps3 z6;
    u32Ps3 z7;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__TypeRefPs3:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__ObjRefPs3:
    uint64_t align;
    uint8_t pad[64];

  cdef struct GameObjsRefPs3:
    const GameObjsHeaderPs3 *header;
    IndexMap_u32__TypeRefPs3 types;
    IndexMap_u32__ObjRefPs3 objs;

  cdef struct SubBlocks1RefPs3:
    SubBlocksInfoRefPs3 info;
    IndexMap_u32__DataRefPs3 files;
    IndexMap_u32__LuaRefPs3 lua;
    IndexMap_u32__SSARefPs3 subtitles;
    Option_AtlasUVRefPs3 atlas1;
    Option_AtlasUVRefPs3 atlas2;
    GameObjsRefPs3 level;

  ctypedef u16_be u16Ps3;

  #gen_ffi:export
  cdef struct StringKeysHeaderPs3:
    u16Ps3 num_a;
    u16Ps3 num_b;
    u32Ps3 z2;
    u32Ps3 z3;
    u32Ps3 z4;
    u32Ps3 z5;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_StringKeysValPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_u32Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct StringKeysRefPs3:
    const StringKeysHeaderPs3 *header;
    ref_slice_StringKeysValPs3 vals;
    ref_slice_u32Ps3 pad;

  cdef struct Block1RefPs3:
    InfosRefPs3 infos;
    ObjsRefPs3 objs;
    SubBlocks1RefPs3 sub_blocks;
    StringKeysRefPs3 string_keys;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_SprayRefPs3:
    uint64_t align;
    uint8_t pad[24];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_CrowdRefPs3:
    uint64_t align;
    uint8_t pad[32];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_PFieldsRefPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__LangStringsRefPs3:
    uint64_t align;
    uint8_t pad[64];

  cdef struct SubBlocks2RefPs3:
    SubBlocksInfoRefPs3 info;
    Option_SprayRefPs3 spray;
    Option_CrowdRefPs3 crowd;
    Option_PFieldsRefPs3 pfields;
    IndexMap_u32__LangStringsRefPs3 langs;
    IndexMap_u32__DataRefPs3 files;

  cdef struct Block2RefPs3:
    SubBlocks2RefPs3 sub_blocks;
    ref_slice_u32Ps3 offsets;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__AnimationRefPs3:
    uint64_t align;
    uint8_t pad[64];

  cdef struct AnimationsRefPs3:
    IndexMap_u32__AnimationRefPs3 animations;
    ref_slice_AnimationBlockInfoPs3 block_infos;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BlockAValPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct PakRefPs3:
    const PakHeaderPs3 *header;
    StringsRefPs3 strings;
    Block1RefPs3 block1;
    Block2RefPs3 block2;
    AnimationsRefPs3 animations;
    ref_slice_BlockAValPs3 vals_a;
    ref_slice_CompressedDataRef animation_data;

  #gen_ffi:export
  cdef struct BinHeaderPs3:
    u32Ps3 constx06;
    u32Ps3 version;
    u32Ps3 strings_offset;
    u32Ps3 strings_size;
    u32Ps3 strings_num;
    u32Ps3 asset_handle_num;
    u32Ps3 asset_handle_offset;
    u32Ps3 unk_7;
    u32Ps3 vdata_num;
    u32Ps3 vdata_num_alt;
    u32Ps3 texdata_num;
    u32Ps3 unk_11;
    u32Ps3 unk_12;
    u32Ps3 unk_13;
    u32Ps3 unk_14;
    u32Ps3 unk_15;
    u32Ps3 unk_16;
    u32Ps3 unk_17;
    u32Ps3 unk_18;
    u32Ps3 unk_19;
    u32Ps3 unk_20;
    u32Ps3 unk_21;
    u32Ps3 unk_22;
    u32Ps3 unk_23;
    u32Ps3 unk_24;
    u32Ps3 unk_25;
    u32Ps3 unk_26;
    u32Ps3 unk_27;
    u32Ps3 unk_28;
    u32Ps3 unk_29;
    u32Ps3 unk_30;
    u32Ps3 unk_31;
    u32Ps3 unk_32;
    u32Ps3 unk_33;
    u32Ps3 unk_34;
    u32Ps3 unk_35;
    u32Ps3 unk_36;
    u32Ps3 unk_37;
    u32Ps3 unk_38;
    u32Ps3 unk_39;
    u32Ps3 unk_40;
    u32Ps3 unk_41;
    u32Ps3 unk_42;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AssetHandlePs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BinRefPs3:
    const BinHeaderPs3 *header;
    StringsRefPs3 strings;
    ref_slice_AssetHandlePs3 asset_handles;
    IndexMap_u32_______CompressedDataRef model_data;
    IndexMap_u32_______CompressedDataRef texture_data;

  cdef struct LevelRefPs3:
    PakRefPs3 pak;
    BinRefPs3 bin;

  cdef struct InfoCounts:
    uintptr_t objas;
    uintptr_t obj0s;
    uintptr_t models;
    uintptr_t buffers;
    uintptr_t mat1s;
    uintptr_t mat2s;
    uintptr_t mat3s;
    uintptr_t mat4s;
    uintptr_t mat_extras;
    uintptr_t shapes;
    uintptr_t hk_shapes;
    uintptr_t hk_constraint_datas;
    uintptr_t vbuffs;
    uintptr_t ibuffs;
    uintptr_t textures;
    uintptr_t animations;
    uintptr_t hk_constraints;
    uintptr_t effects;
    uintptr_t pfields;
    uintptr_t gfxs;
    uintptr_t animation_blocks;
    uintptr_t foliages;
    uintptr_t radiosity_vals;
    uintptr_t offsets;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_ObjAPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_ObjAPc:
    mut_slice_ObjAPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Obj0Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Obj0Pc:
    mut_slice_Obj0Pc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_ModelInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_ModelInfoPc:
    mut_slice_ModelInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_BufferInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_BufferInfoPc:
    mut_slice_BufferInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat1Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat1Pc:
    mut_slice_Mat1Pc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat2Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat2Pc:
    mut_slice_Mat2Pc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat3Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat3Pc:
    mut_slice_Mat3Pc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat4Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat4Pc:
    mut_slice_Mat4Pc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_MatExtraPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_MatExtraPc:
    mut_slice_MatExtraPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_ShapeInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_ShapeInfoPc:
    mut_slice_ShapeInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_HkShapeInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_HkShapeInfoPc:
    mut_slice_HkShapeInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_HkConstraintDataPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_HkConstraintDataPc:
    mut_slice_HkConstraintDataPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_VBuffInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_VBuffInfoPc:
    mut_slice_VBuffInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_IBuffInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_IBuffInfoPc:
    mut_slice_IBuffInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_TextureInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_TextureInfoPc:
    mut_slice_TextureInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_AnimationInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_AnimationInfoPc:
    mut_slice_AnimationInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_HkConstraintInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_HkConstraintInfoPc:
    mut_slice_HkConstraintInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_EffectInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_EffectInfoPc:
    mut_slice_EffectInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_PFieldInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_PFieldInfoPc:
    mut_slice_PFieldInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_GFXBlockInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_GFXBlockInfoPc:
    mut_slice_GFXBlockInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_AnimationBlockInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_AnimationBlockInfoPc:
    mut_slice_AnimationBlockInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_FoliageInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_FoliageInfoPc:
    mut_slice_FoliageInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_RadiosityValsInfoPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_RadiosityValsInfoPc:
    mut_slice_RadiosityValsInfoPc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_u32Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_u32Pc:
    mut_slice_u32Pc vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Vec_DumpInfoDataPc:
    uint64_t align;
    uint8_t pad[16];

  cdef struct DumpInfosPc:
    DumpInfo_ObjAPc objas;
    DumpInfo_Obj0Pc obj0s;
    DumpInfo_ModelInfoPc models;
    DumpInfo_BufferInfoPc buffers;
    DumpInfo_Mat1Pc mat1s;
    DumpInfo_Mat2Pc mat2s;
    DumpInfo_Mat3Pc mat3s;
    DumpInfo_Mat4Pc mat4s;
    DumpInfo_MatExtraPc mat_extras;
    DumpInfo_ShapeInfoPc shapes;
    DumpInfo_HkShapeInfoPc hk_shapes;
    DumpInfo_HkConstraintDataPc hk_constraint_datas;
    DumpInfo_VBuffInfoPc vbuffs;
    DumpInfo_IBuffInfoPc ibuffs;
    DumpInfo_TextureInfoPc textures;
    DumpInfo_AnimationInfoPc animations;
    DumpInfo_HkConstraintInfoPc hk_constraints;
    DumpInfo_EffectInfoPc effects;
    DumpInfo_PFieldInfoPc pfields;
    DumpInfo_GFXBlockInfoPc gfxs;
    DumpInfo_AnimationBlockInfoPc animation_blocks;
    DumpInfo_FoliageInfoPc foliages;
    DumpInfo_RadiosityValsInfoPc radiosity_vals;
    DumpInfo_u32Pc offsets;
    Vec_DumpInfoDataPc model_data;
    Vec_DumpInfoDataPc texture_data;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_ObjAXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_ObjAXbox:
    mut_slice_ObjAXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Obj0Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Obj0Xbox:
    mut_slice_Obj0Xbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_ModelInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_ModelInfoXbox:
    mut_slice_ModelInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_BufferInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_BufferInfoXbox:
    mut_slice_BufferInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat1Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat1Xbox:
    mut_slice_Mat1Xbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat2Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat2Xbox:
    mut_slice_Mat2Xbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat3Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat3Xbox:
    mut_slice_Mat3Xbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat4Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat4Xbox:
    mut_slice_Mat4Xbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_MatExtraXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_MatExtraXbox:
    mut_slice_MatExtraXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_ShapeInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_ShapeInfoXbox:
    mut_slice_ShapeInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_HkShapeInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_HkShapeInfoXbox:
    mut_slice_HkShapeInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_HkConstraintDataXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_HkConstraintDataXbox:
    mut_slice_HkConstraintDataXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_VBuffInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_VBuffInfoXbox:
    mut_slice_VBuffInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_IBuffInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_IBuffInfoXbox:
    mut_slice_IBuffInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_TextureInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_TextureInfoXbox:
    mut_slice_TextureInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_AnimationInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_AnimationInfoXbox:
    mut_slice_AnimationInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_HkConstraintInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_HkConstraintInfoXbox:
    mut_slice_HkConstraintInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_EffectInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_EffectInfoXbox:
    mut_slice_EffectInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_PFieldInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_PFieldInfoXbox:
    mut_slice_PFieldInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_GFXBlockInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_GFXBlockInfoXbox:
    mut_slice_GFXBlockInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_AnimationBlockInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_AnimationBlockInfoXbox:
    mut_slice_AnimationBlockInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_FoliageInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_FoliageInfoXbox:
    mut_slice_FoliageInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_RadiosityValsInfoXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_RadiosityValsInfoXbox:
    mut_slice_RadiosityValsInfoXbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_u32Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_u32Xbox:
    mut_slice_u32Xbox vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Vec_DumpInfoDataXbox:
    uint64_t align;
    uint8_t pad[16];

  cdef struct DumpInfosXbox:
    DumpInfo_ObjAXbox objas;
    DumpInfo_Obj0Xbox obj0s;
    DumpInfo_ModelInfoXbox models;
    DumpInfo_BufferInfoXbox buffers;
    DumpInfo_Mat1Xbox mat1s;
    DumpInfo_Mat2Xbox mat2s;
    DumpInfo_Mat3Xbox mat3s;
    DumpInfo_Mat4Xbox mat4s;
    DumpInfo_MatExtraXbox mat_extras;
    DumpInfo_ShapeInfoXbox shapes;
    DumpInfo_HkShapeInfoXbox hk_shapes;
    DumpInfo_HkConstraintDataXbox hk_constraint_datas;
    DumpInfo_VBuffInfoXbox vbuffs;
    DumpInfo_IBuffInfoXbox ibuffs;
    DumpInfo_TextureInfoXbox textures;
    DumpInfo_AnimationInfoXbox animations;
    DumpInfo_HkConstraintInfoXbox hk_constraints;
    DumpInfo_EffectInfoXbox effects;
    DumpInfo_PFieldInfoXbox pfields;
    DumpInfo_GFXBlockInfoXbox gfxs;
    DumpInfo_AnimationBlockInfoXbox animation_blocks;
    DumpInfo_FoliageInfoXbox foliages;
    DumpInfo_RadiosityValsInfoXbox radiosity_vals;
    DumpInfo_u32Xbox offsets;
    Vec_DumpInfoDataXbox model_data;
    Vec_DumpInfoDataXbox texture_data;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_ObjAPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_ObjAPs3:
    mut_slice_ObjAPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Obj0Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Obj0Ps3:
    mut_slice_Obj0Ps3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_ModelInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_ModelInfoPs3:
    mut_slice_ModelInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_BufferInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_BufferInfoPs3:
    mut_slice_BufferInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat1Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat1Ps3:
    mut_slice_Mat1Ps3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat2Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat2Ps3:
    mut_slice_Mat2Ps3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat3Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat3Ps3:
    mut_slice_Mat3Ps3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_Mat4Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_Mat4Ps3:
    mut_slice_Mat4Ps3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_MatExtraPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_MatExtraPs3:
    mut_slice_MatExtraPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_ShapeInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_ShapeInfoPs3:
    mut_slice_ShapeInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_HkShapeInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_HkShapeInfoPs3:
    mut_slice_HkShapeInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_HkConstraintDataPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_HkConstraintDataPs3:
    mut_slice_HkConstraintDataPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_VBuffInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_VBuffInfoPs3:
    mut_slice_VBuffInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_IBuffInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_IBuffInfoPs3:
    mut_slice_IBuffInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_TextureInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_TextureInfoPs3:
    mut_slice_TextureInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_AnimationInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_AnimationInfoPs3:
    mut_slice_AnimationInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_HkConstraintInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_HkConstraintInfoPs3:
    mut_slice_HkConstraintInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_EffectInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_EffectInfoPs3:
    mut_slice_EffectInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_PFieldInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_PFieldInfoPs3:
    mut_slice_PFieldInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_GFXBlockInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_GFXBlockInfoPs3:
    mut_slice_GFXBlockInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_AnimationBlockInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_AnimationBlockInfoPs3:
    mut_slice_AnimationBlockInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_FoliageInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_FoliageInfoPs3:
    mut_slice_FoliageInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_RadiosityValsInfoPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_RadiosityValsInfoPs3:
    mut_slice_RadiosityValsInfoPs3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_u32Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpInfo_u32Ps3:
    mut_slice_u32Ps3 vals;
    uintptr_t ind;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Vec_DumpInfoDataPs3:
    uint64_t align;
    uint8_t pad[16];

  cdef struct DumpInfosPs3:
    DumpInfo_ObjAPs3 objas;
    DumpInfo_Obj0Ps3 obj0s;
    DumpInfo_ModelInfoPs3 models;
    DumpInfo_BufferInfoPs3 buffers;
    DumpInfo_Mat1Ps3 mat1s;
    DumpInfo_Mat2Ps3 mat2s;
    DumpInfo_Mat3Ps3 mat3s;
    DumpInfo_Mat4Ps3 mat4s;
    DumpInfo_MatExtraPs3 mat_extras;
    DumpInfo_ShapeInfoPs3 shapes;
    DumpInfo_HkShapeInfoPs3 hk_shapes;
    DumpInfo_HkConstraintDataPs3 hk_constraint_datas;
    DumpInfo_VBuffInfoPs3 vbuffs;
    DumpInfo_IBuffInfoPs3 ibuffs;
    DumpInfo_TextureInfoPs3 textures;
    DumpInfo_AnimationInfoPs3 animations;
    DumpInfo_HkConstraintInfoPs3 hk_constraints;
    DumpInfo_EffectInfoPs3 effects;
    DumpInfo_PFieldInfoPs3 pfields;
    DumpInfo_GFXBlockInfoPs3 gfxs;
    DumpInfo_AnimationBlockInfoPs3 animation_blocks;
    DumpInfo_FoliageInfoPs3 foliages;
    DumpInfo_RadiosityValsInfoPs3 radiosity_vals;
    DumpInfo_u32Ps3 offsets;
    Vec_DumpInfoDataPs3 model_data;
    Vec_DumpInfoDataPs3 texture_data;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_u8:
    uint64_t align;
    uint8_t pad[8];

  cdef struct DumpSlice:
    mut_slice_u8 vals;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct string:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_u32:
    uint64_t align;
    uint8_t pad[8];

  cdef struct VertexDataIndex:
    uint32_t key;
    uintptr_t offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_VertexUsage__VertexDataIndex:
    uint64_t align;
    uint8_t pad[64];

  cdef enum:
    VertexUsage_Position,
    VertexUsage_Normal,
    VertexUsage_Tangent,
    VertexUsage_BlendWeight,
    VertexUsage_BlendIndices,
    VertexUsage_Color,
    VertexUsage_TextureCoord,
    VertexUsage_PSize,
    VertexUsage_Pad,
  ctypedef uint8_t VertexUsage_Tag;

  cdef struct VertexUsage:
    VertexUsage_Tag tag;
    uintptr_t color;
    uintptr_t texture_coord;
    uintptr_t pad;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct mut_slice_VertexUsage:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_u32:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_VertexUsage:
    uint64_t align;
    uint8_t pad[8];

  ctypedef uint32_t AlignmentHelper;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice______CompressedDataRef:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct IBuffInfoPc:
    u32Pc unk_0;
    u32Pc size;
    u32Pc format;
    u32Pc vbuff_alt_fmt;
    u32Pc offset;
    u32Pc unk_5;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_u16Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    IndexBufferValsRefPc_U16,
    IndexBufferValsRefPc_U32,
  ctypedef uint8_t IndexBufferValsRefPc_Tag;

  cdef struct IndexBufferValsRefPc:
    IndexBufferValsRefPc_Tag tag;
    ref_slice_u16Pc u16;
    ref_slice_u32Pc u32;

  cdef struct IndexBufferRefPc:
    const IBuffInfoPc *info;
    IndexBufferValsRefPc vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__IndexBufferRefPc:
    uint64_t align;
    uint8_t pad[64];

  #gen_ffi:export
  cdef struct VBuffInfoPc:
    u32Pc unk_0;
    u32Pc size;
    u32Pc unk_3;
    u32Pc offset;
    u32Pc fmt1;
    u32Pc fmt2;
    u32Pc unk_6;
    u32Pc unk_7;

  cdef struct VertexBufferRefPc:
    const VBuffInfoPc *info;
    IndexMap_VertexUsage__VertexDataIndex offsets;
    uintptr_t size;
    ref_slice_u8 data;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__VertexBufferRefPc:
    uint64_t align;
    uint8_t pad[64];

  ctypedef u32Pc CrcPc;

  ctypedef uint64_t u64_le;

  ctypedef u64_le u64Pc;

  ctypedef uint8_t u8Pc;

  ctypedef float f32_le;

  ctypedef f32_le f32Pc;

  #gen_ffi:export
  cdef struct MatBasePc:
    u32Pc unk_0;
    u32Pc unk_1;
    CrcPc tex0;
    CrcPc tex1;
    CrcPc tex2;
    CrcPc tex3;
    CrcPc tex4;
    CrcPc tex5;
    CrcPc key_guid;
    CrcPc mask0;
    CrcPc mask1;
    CrcPc mask2;
    u32Pc unk_12;
    u32Pc unk_13;
    u32Pc unk_14;
    u32Pc unk_15;
    u32Pc unk_16;
    u32Pc unk_17;
    u32Pc unk_18;
    u32Pc unk_19;
    u32Pc unk_20;
    u32Pc unk_21;
    u32Pc unk_22;
    u32Pc unk_23;
    u32Pc unk_24;
    u32Pc unk_25;
    u32Pc unk_26;
    u32Pc unk_27;
    u32Pc unk_28;
    u32Pc unk_29;
    u32Pc unk_30;
    u32Pc unk_31;
    u32Pc unk_32;
    u32Pc unk_33;
    u32Pc z_34;
    u32Pc z_35;
    u32Pc z_36;
    u32Pc z_37;
    u32Pc z_38;
    u32Pc z_39;
    u32Pc unk_40;
    u32Pc unk_41;
    u32Pc unk_42;
    u32Pc unk_43;
    u32Pc unk_44;
    u32Pc unk_45;
    u32Pc unk_46;
    u32Pc unk_47;
    u32Pc unk_48;
    u32Pc unk_49;
    u64Pc flags;
    u32Pc kind;
    u32Pc unk_53;
    u8Pc unk_54a;
    u8Pc unk_54b;
    u16Pc side_flags;
    u32Pc unk_55;
    u32Pc unk_56;
    u32Pc unk_57;
    f32Pc unk_58;
    f32Pc unk_59;
    f32Pc unk_60;
    f32Pc unk_61;
    f32Pc unk_62;
    f32Pc unk_63;
    f32Pc unk_64;
    f32Pc unk_65;
    f32Pc unk_66;
    f32Pc unk_67;
    f32Pc unk_68;
    f32Pc unk_69;
    u32Pc unk_70;
    u32Pc unk_71;
    u32Pc unk_72;
    f32Pc unk_73;
    f32Pc unk_74;
    f32Pc unk_75;
    f32Pc unk_76;
    u32Pc unk_77;
    f32Pc unk_78;
    f32Pc unk_79;
    f32Pc unk_80;
    f32Pc unk_81;
    u32Pc unk_82;
    u32Pc unk_83;
    u32Pc unk_84;
    f32Pc unk_85;
    u32Pc mat_extra_offset;
    CrcPc key;
    u32Pc unk_88;
    u32Pc z_89;

  #gen_ffi:export
  cdef struct Mat1Pc:
    MatBasePc base;

  #gen_ffi:export
  cdef struct MatExtraPc:
    u32Pc unk_0;
    u32Pc unk_1;
    u32Pc unk_2;
    u32Pc unk_3;
    u32Pc unk_4;
    u32Pc unk_5;
    u32Pc unk_6;
    u32Pc unk_7;
    u32Pc unk_8;
    u32Pc unk_9;
    u32Pc unk_10;
    f32Pc unk_11;
    f32Pc unk_12;
    f32Pc unk_13;
    f32Pc unk_14;
    f32Pc unk_15;
    u32Pc unk_16;
    f32Pc unk_17;
    f32Pc unk_18;
    f32Pc unk_19;
    f32Pc unk_20;
    f32Pc unk_21;
    u32Pc unk_22;
    u32Pc unk_23;
    u32Pc unk_24;
    u32Pc unk_25;
    u32Pc unk_26;
    u32Pc unk_27;
    u32Pc unk_28;
    u32Pc unk_29;
    u32Pc unk_30;
    u32Pc unk_31;
    u32Pc unk_32;
    u32Pc unk_33;
    u32Pc unk_34;
    u32Pc unk_35;
    u32Pc unk_36;
    u32Pc unk_37;
    u32Pc unk_38;
    u32Pc unk_39;
    u32Pc unk_40;
    u32Pc unk_41;
    u32Pc unk_42;
    u32Pc unk_43;
    u32Pc unk_44;
    u32Pc unk_45;
    u32Pc unk_46;
    u32Pc unk_47;
    u32Pc unk_48;
    u32Pc unk_49;

  cdef struct Mat1RefPc:
    const Mat1Pc *info;
    const MatExtraPc *extra;

  #gen_ffi:export
  cdef struct Mat2Pc:
    MatBasePc base;
    u32Pc unk_90;
    u32Pc unk_91;
    u32Pc unk_92;
    u32Pc unk_93;
    u32Pc unk_94;
    u32Pc unk_95;
    u32Pc unk_96;
    u32Pc unk_97;
    u32Pc unk_98;
    u32Pc unk_99;
    u32Pc unk_100;
    u32Pc unk_101;
    f32Pc unk_102;
    f32Pc unk_103;
    f32Pc unk_104;
    f32Pc unk_105;
    f32Pc unk_106;
    f32Pc unk_107;
    f32Pc unk_108;
    f32Pc unk_109;
    f32Pc unk_110;
    f32Pc unk_111;
    f32Pc unk_112;
    f32Pc unk_113;
    u32Pc unk_114;
    u32Pc unk_115;
    u32Pc unk_116;
    CrcPc unk_117;
    CrcPc unk_118;
    CrcPc unk_119;
    u8Pc unk_120a;
    u8Pc unk_120b;
    u8Pc unk_120c;
    u8Pc unk_120d;
    u32Pc unk_121;

  cdef struct Mat2RefPc:
    const Mat2Pc *info;
    const MatExtraPc *extra;

  #gen_ffi:export
  cdef struct Mat3Pc:
    MatBasePc base;
    f32Pc unk_90;
    f32Pc unk_91;
    f32Pc unk_92;
    f32Pc unk_93;
    f32Pc unk_94;
    f32Pc unk_95;
    f32Pc unk_96;
    f32Pc unk_97;
    f32Pc unk_98;
    f32Pc unk_99;
    f32Pc unk_100;
    f32Pc unk_101;
    f32Pc unk_102;
    f32Pc unk_103;
    f32Pc unk_104;
    f32Pc unk_105;
    f32Pc unk_106;
    f32Pc unk_107;
    f32Pc unk_108;
    f32Pc unk_109;
    f32Pc unk_110;
    f32Pc unk_111;
    f32Pc unk_112;
    f32Pc unk_113;
    u8Pc variation_id_color;
    u8Pc variation_id_texture;
    u8Pc variation_id_specular;
    u8Pc unk_114d;
    u32Pc unk_115;

  cdef struct Mat3RefPc:
    const Mat3Pc *info;
    const MatExtraPc *extra;

  #gen_ffi:export
  cdef struct Mat4Pc:
    MatBasePc base;
    f32Pc unk_90;
    u32Pc unk_91;
    u32Pc unk_92;
    u32Pc unk_93;
    u32Pc unk_94;
    u32Pc unk_95;
    f32Pc unk_96;
    u32Pc unk_97;
    f32Pc unk_98;
    u32Pc unk_99;
    u32Pc unk_100;
    u32Pc unk_101;
    u32Pc unk_102;
    u32Pc unk_103;
    f32Pc unk_104;
    u32Pc unk_105;
    f32Pc unk_106;
    u32Pc unk_107;
    u32Pc unk_108;
    f32Pc unk_109;
    u32Pc unk_110;
    u32Pc unk_111;
    f32Pc unk_112;
    f32Pc unk_113;
    f32Pc unk_114;
    u32Pc unk_115;
    u32Pc unk_116;
    f32Pc unk_117;
    u32Pc unk_118;
    u32Pc unk_119;
    f32Pc unk_120;
    f32Pc unk_121;
    f32Pc unk_122;
    u32Pc unk_123;
    u32Pc unk_124;
    f32Pc unk_125;
    u32Pc unk_126;
    u32Pc unk_127;
    f32Pc unk_128;
    f32Pc unk_129;
    f32Pc unk_130;
    u32Pc unk_131;
    u32Pc unk_132;
    f32Pc unk_133;
    u32Pc unk_134;
    u32Pc unk_135;
    f32Pc unk_136;
    f32Pc unk_137;
    f32Pc unk_138;
    u32Pc unk_139;
    u32Pc unk_140;
    f32Pc unk_141;
    u32Pc unk_142;
    u32Pc unk_143;
    f32Pc unk_144;
    f32Pc unk_145;

  cdef struct Mat4RefPc:
    const Mat4Pc *info;
    const MatExtraPc *extra;

  cdef enum:
    MatRefPc_Mat1,
    MatRefPc_Mat2,
    MatRefPc_Mat3,
    MatRefPc_Mat4,
  ctypedef uint8_t MatRefPc_Tag;

  cdef struct MatRefPc:
    MatRefPc_Tag tag;
    Mat1RefPc mat1;
    Mat2RefPc mat2;
    Mat3RefPc mat3;
    Mat4RefPc mat4;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__MatRefPc:
    uint64_t align;
    uint8_t pad[64];

  ctypedef int32_t i32_le;

  ctypedef i32_le i32Pc;

  #gen_ffi:export
  cdef struct Vector3Pc:
    f32Pc x;
    f32Pc y;
    f32Pc z;

  #gen_ffi:export
  cdef struct BoundingBoxPc:
    Vector3Pc center;
    f32Pc unk_3;
    Vector3Pc half_width;
    f32Pc unk_7;

  #gen_ffi:export
  cdef struct LodInfoPc:
    u32Pc start;
    u32Pc static_end;
    u32Pc skinned_end;
    u32Pc physics_end;
    u32Pc breakable_end;

  #gen_ffi:export
  cdef struct ModelInfoPc:
    CrcPc key;
    i32Pc gamemodemask;
    u32Pc mat_offset;
    u32Pc buffer_info_offset;
    BoundingBoxPc bounding_box;
    u32Pc mesh_order_offset;
    LodInfoPc lod0;
    LodInfoPc lod1;
    LodInfoPc lod2;
    LodInfoPc lod3;
    u32Pc mat_num;
    u32Pc bones_offset;
    u32Pc bone_parents_offset;
    u32Pc bone_transforms_offset;
    u32Pc bones_num;
    u32Pc skin_binds_offset;
    u32Pc skin_binds_num;
    u32Pc skin_order_offset;
    u32Pc vbuff_offset;
    u32Pc vbuff_num;
    u32Pc ibuff_offset;
    u32Pc ibuff_num;
    u32Pc mesh_bounding_boxes_offset;
    f32Pc unk_46;
    u32Pc variation_counts;
    u32Pc vals_j_num;
    u32Pc vals_j_offset;
    u32Pc block_offset;
    u32Pc vals_k_offset;
    CrcPc asset_key;
    u32Pc asset_type;
    u32Pc unk_54;
    u32Pc unk_55;
    u32Pc shape_offset;
    u32Pc shape_num;
    u32Pc hk_constraint_data_offset;
    u32Pc hk_constraint_data_num;
    u32Pc hk_constraint_offset;
    u32Pc slots_offset;
    u32Pc slot_map_offset;
    u32Pc bone_bounding_boxes_offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_CrcPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_i32Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Matrix4x4Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BoundingBoxPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BonesRefPc:
    ref_slice_CrcPc names;
    ref_slice_i32Pc parents;
    ref_slice_Matrix4x4Pc transforms;
    ref_slice_BoundingBoxPc bounding_boxes;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Key2Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_BlockRefPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32_______VBuffInfoPc:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32_______IBuffInfoPc:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_HkConstraintRefPc:
    uint64_t align;
    uint8_t pad[112];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_ShapeRefPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct ModelDataRefPc:
    ref_slice_BufferInfoPc infos;
    ref_slice_u32Pc vbuff_order;
    ref_slice_u32Pc ibuff_order;
    IndexMap_u32__VertexBufferRefPc vertex;
    IndexMap_u32__IndexBufferRefPc index;
    const CompressedDataRef *data;

  cdef struct ModelRefPc:
    const ModelInfoPc *info;
    BonesRefPc bones;
    ref_slice_u32Pc mat_order;
    ref_slice_u32Pc mesh_order;
    ref_slice_BoundingBoxPc mesh_bounding_boxes;
    ref_slice_Matrix4x4Pc skin_binds;
    ref_slice_u32Pc vals_j;
    ref_slice_u16Pc val_k_header;
    ref_slice_u32Pc vals_k;
    ref_slice_u32Pc skin_order;
    ref_slice_Key2Pc slots;
    ref_slice_u32Pc slot_map;
    const u32Pc *block_header;
    ref_slice_u32Pc block_offsets;
    slice_BlockRefPc blocks;
    ref_slice_BufferInfoPc buffer_infos;
    ref_slice_u32Pc vbuff_order;
    ref_slice_u32Pc ibuff_order;
    IndexMap_u32_______VBuffInfoPc vbuffs;
    IndexMap_u32_______IBuffInfoPc ibuffs;
    IndexMap_u32__MatRefPc mats;
    Option_HkConstraintRefPc hk_constraint;
    ref_slice_HkConstraintDataPc hk_constraint_datas;
    slice_ShapeRefPc shapes;
    ModelDataRefPc data;

  #gen_ffi:export
  cdef struct AnimationInfoPc:
    CrcPc key;
    i32Pc gamemodemask;
    u32Pc offset;
    u32Pc size;
    u32Pc kind;
    f32Pc unk_5;
    u32Pc vals_num;
    u32Pc vals2_num;
    u32Pc unk_8;
    u32Pc vala;
    u32Pc unk_10;
    u32Pc unk_11;
    u32Pc data_offset;
    f32Pc unk_13;
    f32Pc unk_14;
    f32Pc t_scale;
    u32Pc block_starts_offset;
    u32Pc block_starts_num;
    u32Pc block_starts2_offset;
    u32Pc block_starts2_num;
    u32Pc obj_c3_offset;
    u32Pc obj_c3_num;
    u32Pc obj_c4_offset;
    u32Pc obj_c4_num;
    u32Pc block_offset;
    u32Pc block_size;
    u32Pc obj3_num;
    u32Pc obj3_offset;
    u32Pc bones_num1;
    u32Pc unk_29;
    u32Pc obj1_num;
    u32Pc bones_offset;
    u32Pc unk_32;
    u32Pc obj1_offset;
    u32Pc obj2_offset;
    u32Pc obj2_num;
    u32Pc obj5_offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Obj3Pc:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct Obj5HeaderPc:
    u32Pc obj_a_num;
    u32Pc obj_a_offset;
    u32Pc obj_b_num;
    u32Pc obj_b_offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Obj5ValPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_BlocksRefPc:
    uint64_t align;
    uint8_t pad[72];

  cdef struct AnimationRefPc:
    const AnimationInfoPc *info;
    ref_slice_u32Pc obj1;
    ref_slice_u32Pc obj2;
    ref_slice_Obj3Pc obj3;
    ref_slice_CrcPc bones;
    const Obj5HeaderPc *obj5_header;
    ref_slice_Obj5ValPc obj5_a;
    ref_slice_Obj5ValPc obj5_b;
    Option_BlocksRefPc blocks;
    uintptr_t size;

  cdef struct DataRefPc:
    ref_slice_u8 data;

  #gen_ffi:export
  cdef struct EffectInfoPc:
    CrcPc key;
    i32Pc gamemodemask;
    u32Pc offset;
    u32Pc size;

  cdef struct EffectRefPc:
    const EffectInfoPc *info;
    GameObjsRefPc vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__ref_slice_u16Pc:
    uint64_t align;
    uint8_t pad[64];

  cdef struct LangStringsRefPc:
    IndexMap_u32__ref_slice_u16Pc strings;

  ctypedef DataRefPc LuaRefPc;

  #gen_ffi:export
  cdef struct ObjHeaderPc:
    u32Pc layer;
    CrcPc key;
    u16Pc size;
    u16Pc z3;
    u32Pc z4;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__BaseTypeRefPc:
    uint64_t align;
    uint8_t pad[64];

  cdef struct ObjRefPc:
    const ObjHeaderPc *header;
    IndexMap_u32__BaseTypeRefPc fields;

  #gen_ffi:export
  cdef struct RadiosityValsInfoPc:
    u32Pc guid;
    u32Pc num;
    u32Pc offset;

  cdef struct RadiosityValsRefPc:
    const RadiosityValsInfoPc *info;
    ref_slice_i32Pc offs;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SSAValPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_ref_slice_u16Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct SSARefPc:
    ref_slice_SSAValPc vals;
    slice_ref_slice_u16Pc strings;

  #gen_ffi:export
  cdef struct TextureInfoPc:
    CrcPc key;
    i32Pc gamemodemask;
    CrcPc asset_key;
    u32Pc asset_type;
    u32Pc kind;
    u32Pc format;
    u32Pc unk_6;
    u32Pc unk_7;
    u32Pc unk_8;
    u32Pc unk_9;
    u32Pc unk_10;
    u32Pc unk_11;
    u16Pc width;
    u16Pc height;
    u16Pc depth;
    u16Pc levels;
    u8Pc unk_16_1;
    u8Pc unk_16_2;
    u8Pc unk_16_3;
    u8Pc unk_16_4;
    u8Pc unk_16_5;
    u8Pc unk_16_6;
    u8Pc unk_16_7;
    u8Pc unk_16_8;
    u8Pc unk_16_9;
    u8Pc unk_16_10;
    u8Pc unk_16_11;
    u8Pc unk_16_12;
    u8Pc unk_16_13;
    u8Pc unk_16_14;
    u8Pc unk_16_15;
    u8Pc unk_16_16;

  cdef struct TextureRefPc:
    const TextureInfoPc *info;
    const CompressedDataRef *data0;
    const CompressedDataRef *data1;

  #gen_ffi:export
  cdef struct TypeHeaderPc:
    CrcPc key;
    u32Pc size;
    u32Pc fields;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_TypeFieldPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct TypeRefPc:
    const TypeHeaderPc *header;
    ref_slice_TypeFieldPc fields;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_FoliageRefPc:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct Vector2Pc:
    f32Pc x;
    f32Pc y;

  #gen_ffi:export
  cdef struct Vector4Pc:
    f32Pc x;
    f32Pc y;
    f32Pc z;
    f32Pc w;

  #gen_ffi:export
  cdef struct Matrix4x4Pc:
    Vector4Pc x;
    Vector4Pc y;
    Vector4Pc z;
    Vector4Pc w;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_U32Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Vector4Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_WeightPc:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    BaseTypeRefPc_Crc,
    BaseTypeRefPc_GUID,
    BaseTypeRefPc_Color,
    BaseTypeRefPc_Vector2,
    BaseTypeRefPc_Vector3,
    BaseTypeRefPc_Vector4,
    BaseTypeRefPc_Matrix4x4,
    BaseTypeRefPc_Float,
    BaseTypeRefPc_Int,
    BaseTypeRefPc_Bool,
    BaseTypeRefPc_String,
    BaseTypeRefPc_StringList,
    BaseTypeRefPc_ObjectList,
    BaseTypeRefPc_NodeList,
    BaseTypeRefPc_IntList,
    BaseTypeRefPc_CrcList,
    BaseTypeRefPc_WeightList,
    BaseTypeRefPc_MatrixList,
  ctypedef uint8_t BaseTypeRefPc_Tag;

  cdef struct BaseTypeRefPc:
    BaseTypeRefPc_Tag tag;
    const CrcPc *crc;
    const u32Pc *guid;
    const u32Pc *color;
    const Vector2Pc *vector2;
    const Vector3Pc *vector3;
    const Vector4Pc *vector4;
    const Matrix4x4Pc *matrix4x4;
    const f32Pc *float_;
    const i32Pc *int_;
    const u32Pc *bool_;
    string string;
    slice_string string_list;
    ref_slice_U32Pc object_list;
    ref_slice_Vector4Pc node_list;
    ref_slice_i32Pc int_list;
    ref_slice_U32Pc crc_list;
    ref_slice_WeightPc weight_list;
    ref_slice_Matrix4x4Pc matrix_list;

  #gen_ffi:export
  cdef struct BlockHeader1Pc:
    u32Pc a;
    u32Pc b;
    u32Pc unk_2;
    u32Pc unk_3;

  #gen_ffi:export
  cdef struct BlockHeader2Pc:
    u32Pc n;
    f32Pc unk_1;
    f32Pc unk_2;
    u32Pc unk_3;
    u32Pc unk_4;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BlockValAPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BlockValBPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BlockRefPc:
    const BlockHeader1Pc *info1;
    const BlockHeader2Pc *info2;
    ref_slice_BlockValAPc vals_a;
    ref_slice_BlockValAPc vals_b;
    ref_slice_BlockValBPc vals_c;
    ref_slice_u8 pad;

  #gen_ffi:export
  cdef struct ShapeInfoPc:
    u32Pc offset;
    u32Pc kind;
    u32Pc unk_2;
    f32Pc unk_3;
    f32Pc unk_4;
    f32Pc unk_5;
    Vector3Pc translation;
    Vector4Pc rotation;
    f32Pc unk_13;
    f32Pc unk_14;
    f32Pc unk_15;
    f32Pc unk_16;
    f32Pc unk_17;
    f32Pc unk_18;
    f32Pc unk_19;
    f32Pc unk_20;
    f32Pc unk_21;
    f32Pc unk_22;
    f32Pc unk_23;
    f32Pc unk_24;
    f32Pc unk_25;
    f32Pc unk_26;
    u32Pc hk_shape_num;
    u32Pc hk_shape_offset;
    u8Pc unk_29a;
    u8Pc unk_29b;
    u8Pc unk_29c;
    u8Pc unk_29d;
    f32Pc unk_30;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_ShapeExtraRefPc:
    uint64_t align;
    uint8_t pad[32];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_HkShapeRefPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct ShapeRefPc:
    const ShapeInfoPc *info;
    Option_ShapeExtraRefPc extra;
    slice_HkShapeRefPc hk_shapes;

  #gen_ffi:export
  cdef struct BoxShapePc:
    Vector4Pc translation;
    Vector4Pc rotation;
    u32Pc kind;
    CrcPc key;
    Vector3Pc half_extents;
    u32Pc unk_13;
    u32Pc unk_14;
    f32Pc unk_15;
    f32Pc unk_16;
    f32Pc unk_17;
    f32Pc unk_18;
    f32Pc unk_19;

  #gen_ffi:export
  cdef struct SphereShapePc:
    Vector4Pc translation;
    Vector4Pc rotation;
    u32Pc kind;
    CrcPc key;
    f32Pc radius;
    u32Pc unk_11;
    f32Pc unk_12;
    f32Pc unk_13;
    f32Pc unk_14;
    f32Pc unk_15;
    f32Pc unk_16;
    f32Pc unk_17;
    f32Pc unk_18;
    f32Pc unk_19;

  #gen_ffi:export
  cdef struct CapsuleShapePc:
    Vector4Pc translation;
    Vector4Pc rotation;
    u32Pc kind;
    CrcPc key;
    Vector3Pc point1;
    Vector3Pc point2;
    f32Pc radius;
    f32Pc unk_17;
    f32Pc unk_18;
    f32Pc unk_19;

  #gen_ffi:export
  cdef struct CylinderShapePc:
    Vector4Pc translation;
    Vector4Pc rotation;
    u32Pc kind;
    CrcPc key;
    Vector3Pc point1;
    Vector3Pc point2;
    f32Pc radius;
    f32Pc unk_17;
    f32Pc unk_18;
    f32Pc unk_19;

  #gen_ffi:export
  cdef struct ConvexVerticesInfoPc:
    Vector4Pc translation;
    Vector4Pc rotation;
    u32Pc kind;
    CrcPc key;
    u32Pc norm_num;
    u32Pc norms_offset;
    u32Pc vert_num;
    u32Pc verts_offset;
    f32Pc unk_14;
    f32Pc unk_15;
    f32Pc unk_16;
    f32Pc unk_17;
    f32Pc unk_18;
    f32Pc unk_19;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Vector3Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct ConvexVerticesRefPc:
    const ConvexVerticesInfoPc *info;
    ref_slice_Vector4Pc norms;
    ref_slice_Vector3Pc verts;

  #gen_ffi:export
  cdef struct BVTreeMeshInfoPc:
    Vector4Pc translation;
    Vector4Pc rotation;
    u32Pc kind;
    CrcPc key;
    Vector3Pc offset;
    f32Pc tree_scale;
    u32Pc tree_size;
    u32Pc tree_offset;
    u32Pc vert_num;
    u32Pc verts_offset;
    u32Pc tri_num;
    u32Pc inds_offset;

  cdef struct BVTreeMeshRefPc:
    const BVTreeMeshInfoPc *info;
    ref_slice_u8 tree;
    ref_slice_Vector3Pc verts;
    ref_slice_u16Pc inds;

  #gen_ffi:export
  cdef struct HkShapeInfoPc:
    Vector4Pc unk_0;
    Vector4Pc unk_4;
    u32Pc kind;
    u32Pc unk_9;
    u32Pc unk_10;
    u32Pc unk_11;
    u32Pc unk_12;
    u32Pc unk_13;
    u32Pc unk_14;
    u32Pc unk_15;
    u32Pc unk_16;
    u32Pc unk_17;
    u32Pc unk_18;
    u32Pc unk_19;

  cdef enum:
    HkShapeRefPc_Box,
    HkShapeRefPc_Sphere,
    HkShapeRefPc_Capsule,
    HkShapeRefPc_Cylinder,
    HkShapeRefPc_ConvexVertices,
    HkShapeRefPc_BVTreeMesh,
    HkShapeRefPc_Unknown,
  ctypedef uint8_t HkShapeRefPc_Tag;

  cdef struct HkShapeRefPc:
    HkShapeRefPc_Tag tag;
    const BoxShapePc *box;
    const SphereShapePc *sphere;
    const CapsuleShapePc *capsule;
    const CylinderShapePc *cylinder;
    ConvexVerticesRefPc convex_vertices;
    BVTreeMeshRefPc bv_tree_mesh;
    const HkShapeInfoPc *unknown;

  #gen_ffi:export
  cdef struct FoliageInfoPc:
    CrcPc key;
    u32Pc kind;
    i32Pc lb_w;
    i32Pc lb_h;
    i32Pc ub_w;
    i32Pc ub_h;
    f32Pc scale;
    u32Pc offset;
    CrcPc key_mesh;
    CrcPc key_mesh_lod1;
    CrcPc key_mesh_lod2;
    Vector4Pc color;
    f32Pc lod1a;
    f32Pc lod1b;
    f32Pc lod2a;
    f32Pc lod2b;
    f32Pc lod_max;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_FoliageValPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct FoliageRefPc:
    const FoliageInfoPc *info;
    ref_slice_FoliageValPc vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_BlockValARefPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_Obj1RefPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BlockValRefPc:
    slice_BlockValARefPc vals_a;
    slice_Obj1RefPc vals_b;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_BlockValRefPc:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct CrowdItemHeaderPc:
    CrcPc key;
    CrcPc key_main;
    CrcPc key_right;
    CrcPc key_left;
    f32Pc unk_4;
    u32Pc animation_num;
    u32Pc instance_num;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_CrowdValPc:
    uint64_t align;
    uint8_t pad[8];

  cdef struct CrowdItemRefPc:
    const CrowdItemHeaderPc *header;
    ref_slice_CrcPc animations;
    ref_slice_CrowdValPc instances;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_CrowdItemRefPc:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct HkConstraintBoneRefPc:
    string name;
    u32Pc start;
    u32Pc val;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_HkConstraintBoneRefPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_f32Pc:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    AnimVals1RefPc_Type1,
    AnimVals1RefPc_Type2,
    AnimVals1RefPc_Type3,
    AnimVals1RefPc_Type4,
  ctypedef uint8_t AnimVals1RefPc_Tag;

  cdef struct AnimVals1RefPc:
    AnimVals1RefPc_Tag tag;
    ref_slice_u8 type1;
    ref_slice_u16Pc type2;
    ref_slice_u16Pc type3;
    ref_slice_u16Pc type4;

  cdef struct Obj1RefPc:
    uint8_t flags;
    uint8_t s2;
    u16Pc s1;
    ref_slice_u8 data;
    ref_slice_f32Pc vals_a;
    AnimVals1RefPc vals;
    uintptr_t size;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationPolar32Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationThreeComp40Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationThreeComp48Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationThreeComp24Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationStraight16Pc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationUncompressedPc:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    RotationQuantizationRefPc_Polar32,
    RotationQuantizationRefPc_ThreeComp40,
    RotationQuantizationRefPc_ThreeComp48,
    RotationQuantizationRefPc_ThreeComp24,
    RotationQuantizationRefPc_Straight16,
    RotationQuantizationRefPc_Uncompressed,
  ctypedef uint8_t RotationQuantizationRefPc_Tag;

  cdef struct RotationQuantizationRefPc:
    RotationQuantizationRefPc_Tag tag;
    ref_slice_RotationPolar32Pc polar32;
    ref_slice_RotationThreeComp40Pc three_comp40;
    ref_slice_RotationThreeComp48Pc three_comp48;
    ref_slice_RotationThreeComp24Pc three_comp24;
    ref_slice_RotationStraight16Pc straight16;
    ref_slice_RotationUncompressedPc uncompressed;

  cdef struct Obj2RefPc:
    uint8_t flags;
    uint8_t s2;
    u16Pc s1;
    ref_slice_u8 data;
    RotationQuantizationRefPc vals;
    uintptr_t size;

  cdef struct BlockValARefPc:
    Obj1RefPc a;
    Obj2RefPc b;
    Obj1RefPc c;

  #gen_ffi:export
  cdef struct BufferInfoPc:
    u32Pc vbuff_info_offset;
    u32Pc vbuff_info_offset_2;
    u32Pc vbuff_info_offset_3;
    u32Pc unk_3;
    u32Pc unk_4;
    u32Pc unk_5;
    u32Pc unk_6;
    u32Pc unk_7;
    u32Pc unk_8;
    u32Pc unk_9;
    u32Pc unk_10;
    u32Pc unk_11;
    u32Pc unk_12;
    u32Pc unk_13;
    u32Pc unk_14;
    u32Pc unk_15;
    u32Pc unk_16;
    u32Pc unk_17;
    u32Pc unk_18;
    u32Pc unk_19;
    u32Pc unk_20;
    u32Pc unk_21;
    u32Pc unk_22;
    u32Pc unk_23;
    u32Pc unk_24;
    u32Pc unk_25;
    u32Pc unk_26;
    u32Pc unk_27;
    u32Pc unk_28;
    u32Pc unk_29;
    u32Pc unk_30;
    u32Pc unk_31;
    u32Pc v_size;
    u32Pc v_size_2;
    u32Pc v_size_3;
    u32Pc unk_35;
    u32Pc unk_36;
    u32Pc unk_37;
    u32Pc unk_38;
    u32Pc unk_39;
    u32Pc unk_40;
    u32Pc unk_41;
    u32Pc unk_42;
    u32Pc unk_43;
    u32Pc unk_44;
    u32Pc unk_45;
    u32Pc unk_46;
    u32Pc unk_47;
    u32Pc vbuff_size;
    u32Pc vbuff_size_2;
    u32Pc vbuff_size_3;
    u32Pc unk_51;
    u32Pc unk_52;
    u32Pc unk_53;
    u32Pc unk_54;
    u32Pc unk_55;
    u32Pc unk_56;
    u32Pc unk_57;
    u32Pc unk_58;
    u32Pc unk_59;
    u32Pc unk_60;
    u32Pc unk_61;
    u32Pc unk_62;
    u32Pc unk_63;
    u32Pc unk_64;
    u32Pc ibuff_info_offset;
    u32Pc i_num;
    u32Pc unk_67;
    u32Pc skin_offset;
    u32Pc skin_size;
    u32Pc unk_70;
    u32Pc tri_num;
    u32Pc unk_72;
    u32Pc unk_73;
    u32Pc unk_74;
    u32Pc unk_75;
    u32Pc unk_76;
    u32Pc unk_77;
    u32Pc unk_78;
    u32Pc unk_79;
    u32Pc unk_80;
    u32Pc unk_81;
    u32Pc unk_82;
    u32Pc unk_83;
    u32Pc unk_84;
    u32Pc unk_85;
    u32Pc unk_86;
    u32Pc unk_87;
    u8Pc variation_id;
    u8Pc variation;
    u8Pc unk_88c;
    u8Pc unk_88d;

  #gen_ffi:export
  cdef struct HkConstraintDataPc:
    u32Pc kind;
    u32Pc unk_1;
    u32Pc unk_2;
    u32Pc unk_3;
    u32Pc unk_4;
    u32Pc unk_5;
    u32Pc unk_6;
    u32Pc unk_7;
    u32Pc unk_8;
    u32Pc unk_9;
    u32Pc unk_10;
    u32Pc unk_11;
    u32Pc unk_12;
    u32Pc unk_13;
    u32Pc unk_14;
    u32Pc unk_15;
    u32Pc unk_16;
    u32Pc unk_17;
    u32Pc unk_18;
    u32Pc unk_19;
    u32Pc unk_20;
    u32Pc unk_21;
    u32Pc unk_22;
    u32Pc unk_23;
    u32Pc unk_24;
    u32Pc unk_25;
    u32Pc unk_26;
    u32Pc unk_27;
    u32Pc unk_28;

  #gen_ffi:export
  cdef struct Key2Pc:
    CrcPc key;
    u32Pc val;

  #gen_ffi:export
  cdef struct BlockValAPc:
    f32Pc unk_0;
    f32Pc unk_1;
    f32Pc unk_2;
    f32Pc unk_3;
    f32Pc unk_4;
    f32Pc unk_5;
    f32Pc unk_6;
    f32Pc unk_7;
    f32Pc unk_8;
    u32Pc unk_9;
    u32Pc unk_10;
    u32Pc unk_11;

  #gen_ffi:export
  cdef struct BlockValBPc:
    u16Pc unk_0;
    u16Pc unk_1;
    f32Pc unk_2;
    f32Pc unk_3;
    f32Pc unk_4;
    f32Pc unk_5;

  #gen_ffi:export
  cdef struct AnimationBlockInfoPc:
    CrcPc key;
    u32Pc guid;
    CrcPc key_name;
    u32Pc offset;
    u32Pc size;
    u32Pc size_comp;
    u32Pc unk_6;
    u32Pc unk_7;
    u32Pc unk_8;

  #gen_ffi:export
  cdef struct AssetHandlePc:
    CrcPc key;
    u32Pc offset;
    u32Pc size;
    u32Pc size_comp;
    u32Pc kind;

  ctypedef uint32_t U32LE;

  ctypedef U32LE U32Pc;

  ctypedef int32_t I32LE;

  ctypedef I32LE I32Pc;

  cdef struct BlockAValPc:
    U32Pc unk_0;
    I32Pc gamemodemask;
    U32Pc key;
    U32Pc unk_3;
    U32Pc unk_4;
    U32Pc unk_5;
    U32Pc unk_6;

  #gen_ffi:export
  cdef struct GFXBlockInfoPc:
    CrcPc key;
    u32Pc offset;
    u32Pc size;

  #gen_ffi:export
  cdef struct HkConstraintInfoPc:
    u32Pc kind;
    u32Pc bone_parents_offset;
    u32Pc bone_parents_num;
    u32Pc bone_names_offset;
    u32Pc bone_names_num;
    u32Pc bone_transforms_offset;
    u32Pc bone_transforms_num;
    u32Pc unk_7;
    u32Pc unk_8;
    u32Pc unk_9;
    u32Pc bones_offset;
    u16Pc bones_num;
    u16Pc bone_order_num;
    u32Pc bone_order_offset;
    u32Pc unk_13;
    f32Pc unk_14;
    u32Pc vals2_num;
    u32Pc vals2_offset;
    u32Pc unk_17;

  #gen_ffi:export
  cdef struct Obj0Pc:
    u32Pc unk_0;
    CrcPc key;

  #gen_ffi:export
  cdef struct ObjAPc:
    CrcPc key;
    u32Pc unk_1;
    u32Pc size;
    u32Pc size_comp;
    u32Pc unk_4;
    u32Pc kind;

  #gen_ffi:export
  cdef struct PFieldInfoPc:
    u32Pc link_guid;
    u32Pc gamemode_guid;
    u32Pc width;
    u32Pc height;
    u32Pc offset;

  #gen_ffi:export
  cdef struct StringKeysValPc:
    CrcPc key;
    u32Pc offset;

  #gen_ffi:export
  cdef struct SubBlocksBlockHeaderPc:
    CrcPc key;
    u32Pc offset;
    u32Pc size;

  #gen_ffi:export
  cdef struct Obj3Pc:
    f32Pc t;
    CrcPc event;
    u32Pc dat_2;
    u32Pc dat_3;
    u32Pc dat_4;
    u32Pc dat_5;
    u32Pc dat_6;
    u32Pc dat_7;
    u32Pc dat_8;
    u32Pc dat_9;
    u32Pc dat_10;

  #gen_ffi:export
  cdef struct Obj5ValPc:
    f32Pc unk_0;
    f32Pc unk_1;
    f32Pc unk_2;
    f32Pc unk_3;
    f32Pc unk_4;
    f32Pc unk_5;
    f32Pc unk_6;

  #gen_ffi:export
  cdef struct SSAValPc:
    f32Pc t_start;
    f32Pc t_end;
    u32Pc unk_2;
    u32Pc unk_3;
    u32Pc off;

  #gen_ffi:export
  cdef struct TypeFieldPc:
    CrcPc key;
    CrcPc kind;
    u32Pc offset;

  ctypedef int16_t i16_le;

  ctypedef i16_le i16Pc;

  #gen_ffi:export
  cdef struct FoliageValPc:
    u16Pc height;
    u16Pc var_mask;
    i16Pc slope_x;
    i16Pc slope_z;

  #gen_ffi:export
  cdef struct WeightPc:
    u32Pc x;
    u8Pc a;
    u8Pc b;
    u8Pc c;
    u8Pc d;

  #gen_ffi:export
  cdef struct AtlasUVValPc:
    CrcPc key;
    Vector4Pc vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AtlasUVValPc:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct SprayInstancePc:
    CrcPc key;
    CrcPc tex1;
    CrcPc tex2;
    u32Pc unk_3;
    u32Pc width;
    u32Pc height;
    f32Pc unk_6;
    f32Pc unk_7;
    u32Pc size_w;
    u32Pc size_h;
    f32Pc scale_w;
    f32Pc scale_h;
    u32Pc delay;
    f32Pc stride_x;
    f32Pc stride_y;
    u32Pc unk_15;
    u32Pc unk_16;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SprayInstancePc:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct SprayValPc:
    Vector3Pc position;
    f32Pc scale;
    u16Pc instance;
    u16Pc rotation;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SprayValPc:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct TRSPc:
    Vector4Pc translation;
    Vector4Pc rotation;
    Vector4Pc scale;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_TRSPc:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_i16Pc:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct CrowdValPc:
    Vector3Pc position;
    f32Pc rotation;
    f32Pc lod;

  #gen_ffi:export
  cdef struct RotationPolar32Pc:
    u32Pc a;

  #gen_ffi:export
  cdef struct RotationStraight16Pc:
    u8Pc a;
    u8Pc b;

  #gen_ffi:export
  cdef struct RotationThreeComp24Pc:
    u8Pc a;
    u8Pc b;
    u8Pc c;

  #gen_ffi:export
  cdef struct RotationThreeComp40Pc:
    u8Pc a;
    u8Pc b;
    u8Pc c;
    u8Pc d;
    u8Pc e;

  #gen_ffi:export
  cdef struct RotationThreeComp48Pc:
    u16Pc a;
    u16Pc b;
    u16Pc c;

  #gen_ffi:export
  cdef struct RotationUncompressedPc:
    f32Pc a;
    f32Pc b;
    f32Pc c;
    f32Pc d;

  #gen_ffi:export
  cdef struct HkConstraintRefPc:
    const HkConstraintInfoPc *info;
    ref_slice_i16Pc bone_parents;
    slice_HkConstraintBoneRefPc bone_names;
    ref_slice_u32Pc name_offsets;
    ref_slice_TRSPc bone_transforms;
    ref_slice_u32Pc bones;
    ref_slice_Key2Pc bones_order;
    ref_slice_f32Pc vals2;

  #gen_ffi:export
  cdef struct ShapeExtraInfoPc:
    u32Pc size;
    f32Pc scale;
    f32Pc a;
    f32Pc b;

  cdef struct ShapeExtraRefPc:
    const ShapeExtraInfoPc *info;
    ref_slice_u32Pc offs;
    ref_slice_u8 data;

  cdef struct AtlasUVRefPc:
    ref_slice_AtlasUVValPc vals;

  cdef struct BlocksRefPc:
    ref_slice_u32Pc block_starts;
    ref_slice_u32Pc block_starts2;
    ref_slice_u32Pc obj_c3;
    ref_slice_u32Pc obj_c4;
    slice_BlockValRefPc blocks;

  #gen_ffi:export
  cdef struct CrowdHeaderPc:
    u32Pc const0x65;
    u32Pc n;

  cdef struct CrowdRefPc:
    const CrowdHeaderPc *header;
    ref_slice_u32Pc offs;
    slice_CrowdItemRefPc vals;

  ctypedef DataRefPc PFieldsRefPc;

  cdef struct SprayRefPc:
    ref_slice_SprayInstancePc instances;
    ref_slice_SprayValPc vals;

  #gen_ffi:export
  cdef struct IBuffInfoXbox:
    u32Xbox unk_0;
    u32Xbox size;
    u32Xbox format;
    u32Xbox vbuff_alt_fmt;
    u32Xbox offset;
    u32Xbox unk_5;
    u32Xbox unk_6;
    u32Xbox unk_7;
    u32Xbox unk_8;
    u32Xbox unk_9;
    u32Xbox unk_10;
    u32Xbox unk_11;
    u32Xbox unk_12;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_u16Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    IndexBufferValsRefXbox_U16,
    IndexBufferValsRefXbox_U32,
  ctypedef uint8_t IndexBufferValsRefXbox_Tag;

  cdef struct IndexBufferValsRefXbox:
    IndexBufferValsRefXbox_Tag tag;
    ref_slice_u16Xbox u16;
    ref_slice_u32Xbox u32;

  cdef struct IndexBufferRefXbox:
    const IBuffInfoXbox *info;
    IndexBufferValsRefXbox vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__IndexBufferRefXbox:
    uint64_t align;
    uint8_t pad[64];

  #gen_ffi:export
  cdef struct VBuffInfoXbox:
    u32Xbox unk_0;
    u32Xbox size;
    u32Xbox unk_3;
    u32Xbox offset;
    u32Xbox fmt2;
    u32Xbox fmt1;
    u32Xbox unk_6;
    u32Xbox unk_7;
    u32Xbox unk_8;
    u32Xbox unk_9;
    u32Xbox unk_10;
    u32Xbox unk_11;
    u32Xbox unk_12;
    u32Xbox unk_13;

  cdef struct VertexBufferRefXbox:
    const VBuffInfoXbox *info;
    IndexMap_VertexUsage__VertexDataIndex offsets;
    uintptr_t size;
    ref_slice_u8 data;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__VertexBufferRefXbox:
    uint64_t align;
    uint8_t pad[64];

  ctypedef u32Xbox CrcXbox;

  ctypedef uint64_t u64_be;

  ctypedef u64_be u64Xbox;

  ctypedef uint8_t u8Xbox;

  ctypedef float f32_be;

  ctypedef f32_be f32Xbox;

  #gen_ffi:export
  cdef struct MatBaseXbox:
    u32Xbox unk_0;
    u32Xbox unk_1;
    CrcXbox tex0;
    CrcXbox tex1;
    CrcXbox tex2;
    CrcXbox tex3;
    CrcXbox tex4;
    CrcXbox tex5;
    CrcXbox key_guid;
    CrcXbox mask0;
    CrcXbox mask1;
    CrcXbox mask2;
    u32Xbox unk_12;
    u32Xbox unk_13;
    u32Xbox unk_14;
    u32Xbox unk_15;
    u32Xbox unk_16;
    u32Xbox unk_17;
    u32Xbox unk_18;
    u32Xbox unk_19;
    u32Xbox unk_20;
    u32Xbox unk_21;
    u32Xbox unk_22;
    u32Xbox unk_23;
    u32Xbox unk_24;
    u32Xbox unk_25;
    u32Xbox unk_26;
    u32Xbox unk_27;
    u32Xbox unk_28;
    u32Xbox unk_29;
    u32Xbox unk_30;
    u32Xbox unk_31;
    u32Xbox unk_32;
    u32Xbox unk_33;
    u32Xbox z_34;
    u32Xbox z_35;
    u32Xbox z_36;
    u32Xbox z_37;
    u32Xbox z_38;
    u32Xbox z_39;
    u32Xbox unk_40;
    u32Xbox unk_41;
    u32Xbox unk_42;
    u32Xbox unk_43;
    u32Xbox unk_44;
    u32Xbox unk_45;
    u32Xbox unk_46;
    u32Xbox unk_47;
    u32Xbox unk_48;
    u32Xbox unk_49;
    u64Xbox flags;
    u32Xbox kind;
    u32Xbox unk_53;
    u8Xbox unk_54a;
    u8Xbox unk_54b;
    u16Xbox side_flags;
    u32Xbox unk_55;
    u32Xbox unk_56;
    u32Xbox unk_57;
    f32Xbox unk_58;
    f32Xbox unk_59;
    f32Xbox unk_60;
    f32Xbox unk_61;
    f32Xbox unk_62;
    f32Xbox unk_63;
    f32Xbox unk_64;
    f32Xbox unk_65;
    f32Xbox unk_66;
    f32Xbox unk_67;
    f32Xbox unk_68;
    f32Xbox unk_69;
    u32Xbox unk_70;
    u32Xbox unk_71;
    u32Xbox unk_72;
    f32Xbox unk_73;
    f32Xbox unk_74;
    f32Xbox unk_75;
    f32Xbox unk_76;
    u32Xbox unk_77;
    f32Xbox unk_78;
    f32Xbox unk_79;
    f32Xbox unk_80;
    f32Xbox unk_81;
    u32Xbox unk_82;
    u32Xbox unk_83;
    u32Xbox unk_84;
    f32Xbox unk_85;
    u32Xbox mat_extra_offset;
    CrcXbox key;
    u32Xbox unk_88;
    u32Xbox z_89;

  #gen_ffi:export
  cdef struct Mat1Xbox:
    MatBaseXbox base;

  #gen_ffi:export
  cdef struct MatExtraXbox:
    u32Xbox unk_0;
    u32Xbox unk_1;
    u32Xbox unk_2;
    u32Xbox unk_3;
    u32Xbox unk_4;
    u32Xbox unk_5;
    u32Xbox unk_6;
    u32Xbox unk_7;
    u32Xbox unk_8;
    u32Xbox unk_9;
    u32Xbox unk_10;
    f32Xbox unk_11;
    f32Xbox unk_12;
    f32Xbox unk_13;
    f32Xbox unk_14;
    f32Xbox unk_15;
    u32Xbox unk_16;
    f32Xbox unk_17;
    f32Xbox unk_18;
    f32Xbox unk_19;
    f32Xbox unk_20;
    f32Xbox unk_21;
    u32Xbox unk_22;
    u32Xbox unk_23;
    u32Xbox unk_24;
    u32Xbox unk_25;
    u32Xbox unk_26;
    u32Xbox unk_27;
    u32Xbox unk_28;
    u32Xbox unk_29;
    u32Xbox unk_30;
    u32Xbox unk_31;
    u32Xbox unk_32;
    u32Xbox unk_33;
    u32Xbox unk_34;
    u32Xbox unk_35;
    u32Xbox unk_36;
    u32Xbox unk_37;
    u32Xbox unk_38;
    u32Xbox unk_39;
    u32Xbox unk_40;
    u32Xbox unk_41;
    u32Xbox unk_42;
    u32Xbox unk_43;
    u32Xbox unk_44;
    u32Xbox unk_45;
    u32Xbox unk_46;
    u32Xbox unk_47;
    u32Xbox unk_48;
    u32Xbox unk_49;

  cdef struct Mat1RefXbox:
    const Mat1Xbox *info;
    const MatExtraXbox *extra;

  #gen_ffi:export
  cdef struct Mat2Xbox:
    MatBaseXbox base;
    u32Xbox unk_90;
    u32Xbox unk_91;
    u32Xbox unk_92;
    u32Xbox unk_93;
    u32Xbox unk_94;
    u32Xbox unk_95;
    u32Xbox unk_96;
    u32Xbox unk_97;
    u32Xbox unk_98;
    u32Xbox unk_99;
    u32Xbox unk_100;
    u32Xbox unk_101;
    f32Xbox unk_102;
    f32Xbox unk_103;
    f32Xbox unk_104;
    f32Xbox unk_105;
    f32Xbox unk_106;
    f32Xbox unk_107;
    f32Xbox unk_108;
    f32Xbox unk_109;
    f32Xbox unk_110;
    f32Xbox unk_111;
    f32Xbox unk_112;
    f32Xbox unk_113;
    u32Xbox unk_114;
    u32Xbox unk_115;
    u32Xbox unk_116;
    CrcXbox unk_117;
    CrcXbox unk_118;
    CrcXbox unk_119;
    u8Xbox unk_120a;
    u8Xbox unk_120b;
    u8Xbox unk_120c;
    u8Xbox unk_120d;
    u32Xbox unk_121;

  cdef struct Mat2RefXbox:
    const Mat2Xbox *info;
    const MatExtraXbox *extra;

  #gen_ffi:export
  cdef struct Mat3Xbox:
    MatBaseXbox base;
    f32Xbox unk_90;
    f32Xbox unk_91;
    f32Xbox unk_92;
    f32Xbox unk_93;
    f32Xbox unk_94;
    f32Xbox unk_95;
    f32Xbox unk_96;
    f32Xbox unk_97;
    f32Xbox unk_98;
    f32Xbox unk_99;
    f32Xbox unk_100;
    f32Xbox unk_101;
    f32Xbox unk_102;
    f32Xbox unk_103;
    f32Xbox unk_104;
    f32Xbox unk_105;
    f32Xbox unk_106;
    f32Xbox unk_107;
    f32Xbox unk_108;
    f32Xbox unk_109;
    f32Xbox unk_110;
    f32Xbox unk_111;
    f32Xbox unk_112;
    f32Xbox unk_113;
    u8Xbox variation_id_color;
    u8Xbox variation_id_texture;
    u8Xbox variation_id_specular;
    u8Xbox unk_114d;
    u32Xbox unk_115;

  cdef struct Mat3RefXbox:
    const Mat3Xbox *info;
    const MatExtraXbox *extra;

  #gen_ffi:export
  cdef struct Mat4Xbox:
    MatBaseXbox base;
    f32Xbox unk_90;
    u32Xbox unk_91;
    u32Xbox unk_92;
    u32Xbox unk_93;
    u32Xbox unk_94;
    u32Xbox unk_95;
    f32Xbox unk_96;
    u32Xbox unk_97;
    f32Xbox unk_98;
    u32Xbox unk_99;
    u32Xbox unk_100;
    u32Xbox unk_101;
    u32Xbox unk_102;
    u32Xbox unk_103;
    f32Xbox unk_104;
    u32Xbox unk_105;
    f32Xbox unk_106;
    u32Xbox unk_107;
    u32Xbox unk_108;
    f32Xbox unk_109;
    u32Xbox unk_110;
    u32Xbox unk_111;
    f32Xbox unk_112;
    f32Xbox unk_113;
    f32Xbox unk_114;
    u32Xbox unk_115;
    u32Xbox unk_116;
    f32Xbox unk_117;
    u32Xbox unk_118;
    u32Xbox unk_119;
    f32Xbox unk_120;
    f32Xbox unk_121;
    f32Xbox unk_122;
    u32Xbox unk_123;
    u32Xbox unk_124;
    f32Xbox unk_125;
    u32Xbox unk_126;
    u32Xbox unk_127;
    f32Xbox unk_128;
    f32Xbox unk_129;
    f32Xbox unk_130;
    u32Xbox unk_131;
    u32Xbox unk_132;
    f32Xbox unk_133;
    u32Xbox unk_134;
    u32Xbox unk_135;
    f32Xbox unk_136;
    f32Xbox unk_137;
    f32Xbox unk_138;
    u32Xbox unk_139;
    u32Xbox unk_140;
    f32Xbox unk_141;
    u32Xbox unk_142;
    u32Xbox unk_143;
    f32Xbox unk_144;
    f32Xbox unk_145;

  cdef struct Mat4RefXbox:
    const Mat4Xbox *info;
    const MatExtraXbox *extra;

  cdef enum:
    MatRefXbox_Mat1,
    MatRefXbox_Mat2,
    MatRefXbox_Mat3,
    MatRefXbox_Mat4,
  ctypedef uint8_t MatRefXbox_Tag;

  cdef struct MatRefXbox:
    MatRefXbox_Tag tag;
    Mat1RefXbox mat1;
    Mat2RefXbox mat2;
    Mat3RefXbox mat3;
    Mat4RefXbox mat4;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__MatRefXbox:
    uint64_t align;
    uint8_t pad[64];

  ctypedef int32_t i32_be;

  ctypedef i32_be i32Xbox;

  #gen_ffi:export
  cdef struct Vector3Xbox:
    f32Xbox x;
    f32Xbox y;
    f32Xbox z;

  #gen_ffi:export
  cdef struct BoundingBoxXbox:
    Vector3Xbox center;
    f32Xbox unk_3;
    Vector3Xbox half_width;
    f32Xbox unk_7;

  #gen_ffi:export
  cdef struct LodInfoXbox:
    u32Xbox start;
    u32Xbox static_end;
    u32Xbox skinned_end;
    u32Xbox physics_end;
    u32Xbox breakable_end;

  #gen_ffi:export
  cdef struct ModelInfoXbox:
    CrcXbox key;
    i32Xbox gamemodemask;
    u32Xbox mat_offset;
    u32Xbox buffer_info_offset;
    BoundingBoxXbox bounding_box;
    u32Xbox mesh_order_offset;
    LodInfoXbox lod0;
    LodInfoXbox lod1;
    LodInfoXbox lod2;
    LodInfoXbox lod3;
    u32Xbox mat_num;
    u32Xbox bones_offset;
    u32Xbox bone_parents_offset;
    u32Xbox bone_transforms_offset;
    u32Xbox bones_num;
    u32Xbox skin_binds_offset;
    u32Xbox skin_binds_num;
    u32Xbox skin_order_offset;
    u32Xbox vbuff_offset;
    u32Xbox vbuff_num;
    u32Xbox ibuff_offset;
    u32Xbox ibuff_num;
    u32Xbox mesh_bounding_boxes_offset;
    f32Xbox unk_46;
    u32Xbox variation_counts;
    u32Xbox vals_j_num;
    u32Xbox vals_j_offset;
    u32Xbox block_offset;
    u32Xbox vals_k_offset;
    CrcXbox asset_key;
    u32Xbox asset_type;
    u32Xbox unk_54;
    u32Xbox unk_55;
    u32Xbox shape_offset;
    u32Xbox shape_num;
    u32Xbox hk_constraint_data_offset;
    u32Xbox hk_constraint_data_num;
    u32Xbox hk_constraint_offset;
    u32Xbox slots_offset;
    u32Xbox slot_map_offset;
    u32Xbox bone_bounding_boxes_offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_CrcXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_i32Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Matrix4x4Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BoundingBoxXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BonesRefXbox:
    ref_slice_CrcXbox names;
    ref_slice_i32Xbox parents;
    ref_slice_Matrix4x4Xbox transforms;
    ref_slice_BoundingBoxXbox bounding_boxes;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Key2Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_BlockRefXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32_______VBuffInfoXbox:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32_______IBuffInfoXbox:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_HkConstraintRefXbox:
    uint64_t align;
    uint8_t pad[112];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_ShapeRefXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct ModelDataRefXbox:
    ref_slice_BufferInfoXbox infos;
    ref_slice_u32Xbox vbuff_order;
    ref_slice_u32Xbox ibuff_order;
    IndexMap_u32__VertexBufferRefXbox vertex;
    IndexMap_u32__IndexBufferRefXbox index;
    const CompressedDataRef *data;

  cdef struct ModelRefXbox:
    const ModelInfoXbox *info;
    BonesRefXbox bones;
    ref_slice_u32Xbox mat_order;
    ref_slice_u32Xbox mesh_order;
    ref_slice_BoundingBoxXbox mesh_bounding_boxes;
    ref_slice_Matrix4x4Xbox skin_binds;
    ref_slice_u32Xbox vals_j;
    ref_slice_u16Xbox val_k_header;
    ref_slice_u32Xbox vals_k;
    ref_slice_u32Xbox skin_order;
    ref_slice_Key2Xbox slots;
    ref_slice_u32Xbox slot_map;
    const u32Xbox *block_header;
    ref_slice_u32Xbox block_offsets;
    slice_BlockRefXbox blocks;
    ref_slice_BufferInfoXbox buffer_infos;
    ref_slice_u32Xbox vbuff_order;
    ref_slice_u32Xbox ibuff_order;
    IndexMap_u32_______VBuffInfoXbox vbuffs;
    IndexMap_u32_______IBuffInfoXbox ibuffs;
    IndexMap_u32__MatRefXbox mats;
    Option_HkConstraintRefXbox hk_constraint;
    ref_slice_HkConstraintDataXbox hk_constraint_datas;
    slice_ShapeRefXbox shapes;
    ModelDataRefXbox data;

  #gen_ffi:export
  cdef struct AnimationInfoXbox:
    CrcXbox key;
    i32Xbox gamemodemask;
    u32Xbox offset;
    u32Xbox size;
    u32Xbox kind;
    f32Xbox unk_5;
    u32Xbox vals_num;
    u32Xbox vals2_num;
    u32Xbox unk_8;
    u32Xbox vala;
    u32Xbox unk_10;
    u32Xbox unk_11;
    u32Xbox data_offset;
    f32Xbox unk_13;
    f32Xbox unk_14;
    f32Xbox t_scale;
    u32Xbox block_starts_offset;
    u32Xbox block_starts_num;
    u32Xbox block_starts2_offset;
    u32Xbox block_starts2_num;
    u32Xbox obj_c3_offset;
    u32Xbox obj_c3_num;
    u32Xbox obj_c4_offset;
    u32Xbox obj_c4_num;
    u32Xbox block_offset;
    u32Xbox block_size;
    u32Xbox obj3_num;
    u32Xbox obj3_offset;
    u32Xbox bones_num1;
    u32Xbox unk_29;
    u32Xbox obj1_num;
    u32Xbox bones_offset;
    u32Xbox unk_32;
    u32Xbox obj1_offset;
    u32Xbox obj2_offset;
    u32Xbox obj2_num;
    u32Xbox obj5_offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Obj3Xbox:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct Obj5HeaderXbox:
    u32Xbox obj_a_num;
    u32Xbox obj_a_offset;
    u32Xbox obj_b_num;
    u32Xbox obj_b_offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Obj5ValXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_BlocksRefXbox:
    uint64_t align;
    uint8_t pad[72];

  cdef struct AnimationRefXbox:
    const AnimationInfoXbox *info;
    ref_slice_u32Xbox obj1;
    ref_slice_u32Xbox obj2;
    ref_slice_Obj3Xbox obj3;
    ref_slice_CrcXbox bones;
    const Obj5HeaderXbox *obj5_header;
    ref_slice_Obj5ValXbox obj5_a;
    ref_slice_Obj5ValXbox obj5_b;
    Option_BlocksRefXbox blocks;
    uintptr_t size;

  cdef struct DataRefXbox:
    ref_slice_u8 data;

  #gen_ffi:export
  cdef struct EffectInfoXbox:
    CrcXbox key;
    i32Xbox gamemodemask;
    u32Xbox offset;
    u32Xbox size;

  cdef struct EffectRefXbox:
    const EffectInfoXbox *info;
    GameObjsRefXbox vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__ref_slice_u16Xbox:
    uint64_t align;
    uint8_t pad[64];

  cdef struct LangStringsRefXbox:
    IndexMap_u32__ref_slice_u16Xbox strings;

  ctypedef DataRefXbox LuaRefXbox;

  #gen_ffi:export
  cdef struct ObjHeaderXbox:
    u32Xbox layer;
    CrcXbox key;
    u16Xbox size;
    u16Xbox z3;
    u32Xbox z4;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__BaseTypeRefXbox:
    uint64_t align;
    uint8_t pad[64];

  cdef struct ObjRefXbox:
    const ObjHeaderXbox *header;
    IndexMap_u32__BaseTypeRefXbox fields;

  #gen_ffi:export
  cdef struct RadiosityValsInfoXbox:
    u32Xbox guid;
    u32Xbox num;
    u32Xbox offset;

  cdef struct RadiosityValsRefXbox:
    const RadiosityValsInfoXbox *info;
    ref_slice_i32Xbox offs;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SSAValXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_ref_slice_u16Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct SSARefXbox:
    ref_slice_SSAValXbox vals;
    slice_ref_slice_u16Xbox strings;

  #gen_ffi:export
  cdef struct TextureInfoXbox:
    CrcXbox key;
    i32Xbox gamemodemask;
    CrcXbox asset_key;
    u32Xbox asset_type;
    u32Xbox kind;
    u32Xbox format;
    u32Xbox unk_6;
    u32Xbox unk_7;
    u32Xbox unk_8;
    u32Xbox unk_9;
    u32Xbox unk_10;
    u32Xbox unk_11;
    u16Xbox width;
    u16Xbox height;
    u16Xbox depth;
    u16Xbox levels;
    u8Xbox unk_16_1;
    u8Xbox unk_16_2;
    u8Xbox unk_16_3;
    u8Xbox unk_16_4;
    u8Xbox unk_16_5;
    u8Xbox unk_16_6;
    u8Xbox unk_16_7;
    u8Xbox unk_16_8;
    u8Xbox unk_16_9;
    u8Xbox unk_16_10;
    u8Xbox unk_16_11;
    u8Xbox unk_16_12;
    u8Xbox unk_16_13;
    u8Xbox unk_16_14;
    u8Xbox unk_16_15;
    u8Xbox unk_16_16;

  cdef struct TextureRefXbox:
    const TextureInfoXbox *info;
    const CompressedDataRef *data0;
    const CompressedDataRef *data1;

  #gen_ffi:export
  cdef struct TypeHeaderXbox:
    CrcXbox key;
    u32Xbox size;
    u32Xbox fields;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_TypeFieldXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct TypeRefXbox:
    const TypeHeaderXbox *header;
    ref_slice_TypeFieldXbox fields;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_FoliageRefXbox:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct Vector2Xbox:
    f32Xbox x;
    f32Xbox y;

  #gen_ffi:export
  cdef struct Vector4Xbox:
    f32Xbox x;
    f32Xbox y;
    f32Xbox z;
    f32Xbox w;

  #gen_ffi:export
  cdef struct Matrix4x4Xbox:
    Vector4Xbox x;
    Vector4Xbox y;
    Vector4Xbox z;
    Vector4Xbox w;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_U32Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Vector4Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_WeightXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    BaseTypeRefXbox_Crc,
    BaseTypeRefXbox_GUID,
    BaseTypeRefXbox_Color,
    BaseTypeRefXbox_Vector2,
    BaseTypeRefXbox_Vector3,
    BaseTypeRefXbox_Vector4,
    BaseTypeRefXbox_Matrix4x4,
    BaseTypeRefXbox_Float,
    BaseTypeRefXbox_Int,
    BaseTypeRefXbox_Bool,
    BaseTypeRefXbox_String,
    BaseTypeRefXbox_StringList,
    BaseTypeRefXbox_ObjectList,
    BaseTypeRefXbox_NodeList,
    BaseTypeRefXbox_IntList,
    BaseTypeRefXbox_CrcList,
    BaseTypeRefXbox_WeightList,
    BaseTypeRefXbox_MatrixList,
  ctypedef uint8_t BaseTypeRefXbox_Tag;

  cdef struct BaseTypeRefXbox:
    BaseTypeRefXbox_Tag tag;
    const CrcXbox *crc;
    const u32Xbox *guid;
    const u32Xbox *color;
    const Vector2Xbox *vector2;
    const Vector3Xbox *vector3;
    const Vector4Xbox *vector4;
    const Matrix4x4Xbox *matrix4x4;
    const f32Xbox *float_;
    const i32Xbox *int_;
    const u32Xbox *bool_;
    string string;
    slice_string string_list;
    ref_slice_U32Xbox object_list;
    ref_slice_Vector4Xbox node_list;
    ref_slice_i32Xbox int_list;
    ref_slice_U32Xbox crc_list;
    ref_slice_WeightXbox weight_list;
    ref_slice_Matrix4x4Xbox matrix_list;

  #gen_ffi:export
  cdef struct BlockHeader1Xbox:
    u32Xbox a;
    u32Xbox b;
    u32Xbox unk_2;
    u32Xbox unk_3;

  #gen_ffi:export
  cdef struct BlockHeader2Xbox:
    u32Xbox n;
    f32Xbox unk_1;
    f32Xbox unk_2;
    u32Xbox unk_3;
    u32Xbox unk_4;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BlockValAXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BlockValBXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BlockRefXbox:
    const BlockHeader1Xbox *info1;
    const BlockHeader2Xbox *info2;
    ref_slice_BlockValAXbox vals_a;
    ref_slice_BlockValAXbox vals_b;
    ref_slice_BlockValBXbox vals_c;
    ref_slice_u8 pad;

  #gen_ffi:export
  cdef struct ShapeInfoXbox:
    u32Xbox offset;
    u32Xbox kind;
    u32Xbox unk_2;
    f32Xbox unk_3;
    f32Xbox unk_4;
    f32Xbox unk_5;
    Vector3Xbox translation;
    Vector4Xbox rotation;
    f32Xbox unk_13;
    f32Xbox unk_14;
    f32Xbox unk_15;
    f32Xbox unk_16;
    f32Xbox unk_17;
    f32Xbox unk_18;
    f32Xbox unk_19;
    f32Xbox unk_20;
    f32Xbox unk_21;
    f32Xbox unk_22;
    f32Xbox unk_23;
    f32Xbox unk_24;
    f32Xbox unk_25;
    f32Xbox unk_26;
    u32Xbox hk_shape_num;
    u32Xbox hk_shape_offset;
    u8Xbox unk_29a;
    u8Xbox unk_29b;
    u8Xbox unk_29c;
    u8Xbox unk_29d;
    f32Xbox unk_30;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_ShapeExtraRefXbox:
    uint64_t align;
    uint8_t pad[32];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_HkShapeRefXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct ShapeRefXbox:
    const ShapeInfoXbox *info;
    Option_ShapeExtraRefXbox extra;
    slice_HkShapeRefXbox hk_shapes;

  #gen_ffi:export
  cdef struct BoxShapeXbox:
    Vector4Xbox translation;
    Vector4Xbox rotation;
    u32Xbox kind;
    CrcXbox key;
    Vector3Xbox half_extents;
    u32Xbox unk_13;
    u32Xbox unk_14;
    f32Xbox unk_15;
    f32Xbox unk_16;
    f32Xbox unk_17;
    f32Xbox unk_18;
    f32Xbox unk_19;

  #gen_ffi:export
  cdef struct SphereShapeXbox:
    Vector4Xbox translation;
    Vector4Xbox rotation;
    u32Xbox kind;
    CrcXbox key;
    f32Xbox radius;
    u32Xbox unk_11;
    f32Xbox unk_12;
    f32Xbox unk_13;
    f32Xbox unk_14;
    f32Xbox unk_15;
    f32Xbox unk_16;
    f32Xbox unk_17;
    f32Xbox unk_18;
    f32Xbox unk_19;

  #gen_ffi:export
  cdef struct CapsuleShapeXbox:
    Vector4Xbox translation;
    Vector4Xbox rotation;
    u32Xbox kind;
    CrcXbox key;
    Vector3Xbox point1;
    Vector3Xbox point2;
    f32Xbox radius;
    f32Xbox unk_17;
    f32Xbox unk_18;
    f32Xbox unk_19;

  #gen_ffi:export
  cdef struct CylinderShapeXbox:
    Vector4Xbox translation;
    Vector4Xbox rotation;
    u32Xbox kind;
    CrcXbox key;
    Vector3Xbox point1;
    Vector3Xbox point2;
    f32Xbox radius;
    f32Xbox unk_17;
    f32Xbox unk_18;
    f32Xbox unk_19;

  #gen_ffi:export
  cdef struct ConvexVerticesInfoXbox:
    Vector4Xbox translation;
    Vector4Xbox rotation;
    u32Xbox kind;
    CrcXbox key;
    u32Xbox norm_num;
    u32Xbox norms_offset;
    u32Xbox vert_num;
    u32Xbox verts_offset;
    f32Xbox unk_14;
    f32Xbox unk_15;
    f32Xbox unk_16;
    f32Xbox unk_17;
    f32Xbox unk_18;
    f32Xbox unk_19;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Vector3Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct ConvexVerticesRefXbox:
    const ConvexVerticesInfoXbox *info;
    ref_slice_Vector4Xbox norms;
    ref_slice_Vector3Xbox verts;

  #gen_ffi:export
  cdef struct BVTreeMeshInfoXbox:
    Vector4Xbox translation;
    Vector4Xbox rotation;
    u32Xbox kind;
    CrcXbox key;
    Vector3Xbox offset;
    f32Xbox tree_scale;
    u32Xbox tree_size;
    u32Xbox tree_offset;
    u32Xbox vert_num;
    u32Xbox verts_offset;
    u32Xbox tri_num;
    u32Xbox inds_offset;

  cdef struct BVTreeMeshRefXbox:
    const BVTreeMeshInfoXbox *info;
    ref_slice_u8 tree;
    ref_slice_Vector3Xbox verts;
    ref_slice_u16Xbox inds;

  #gen_ffi:export
  cdef struct HkShapeInfoXbox:
    Vector4Xbox unk_0;
    Vector4Xbox unk_4;
    u32Xbox kind;
    u32Xbox unk_9;
    u32Xbox unk_10;
    u32Xbox unk_11;
    u32Xbox unk_12;
    u32Xbox unk_13;
    u32Xbox unk_14;
    u32Xbox unk_15;
    u32Xbox unk_16;
    u32Xbox unk_17;
    u32Xbox unk_18;
    u32Xbox unk_19;

  cdef enum:
    HkShapeRefXbox_Box,
    HkShapeRefXbox_Sphere,
    HkShapeRefXbox_Capsule,
    HkShapeRefXbox_Cylinder,
    HkShapeRefXbox_ConvexVertices,
    HkShapeRefXbox_BVTreeMesh,
    HkShapeRefXbox_Unknown,
  ctypedef uint8_t HkShapeRefXbox_Tag;

  cdef struct HkShapeRefXbox:
    HkShapeRefXbox_Tag tag;
    const BoxShapeXbox *box;
    const SphereShapeXbox *sphere;
    const CapsuleShapeXbox *capsule;
    const CylinderShapeXbox *cylinder;
    ConvexVerticesRefXbox convex_vertices;
    BVTreeMeshRefXbox bv_tree_mesh;
    const HkShapeInfoXbox *unknown;

  #gen_ffi:export
  cdef struct FoliageInfoXbox:
    CrcXbox key;
    u32Xbox kind;
    i32Xbox lb_w;
    i32Xbox lb_h;
    i32Xbox ub_w;
    i32Xbox ub_h;
    f32Xbox scale;
    u32Xbox offset;
    CrcXbox key_mesh;
    CrcXbox key_mesh_lod1;
    CrcXbox key_mesh_lod2;
    Vector4Xbox color;
    f32Xbox lod1a;
    f32Xbox lod1b;
    f32Xbox lod2a;
    f32Xbox lod2b;
    f32Xbox lod_max;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_FoliageValXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct FoliageRefXbox:
    const FoliageInfoXbox *info;
    ref_slice_FoliageValXbox vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_BlockValARefXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_Obj1RefXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BlockValRefXbox:
    slice_BlockValARefXbox vals_a;
    slice_Obj1RefXbox vals_b;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_BlockValRefXbox:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct CrowdItemHeaderXbox:
    CrcXbox key;
    CrcXbox key_main;
    CrcXbox key_right;
    CrcXbox key_left;
    f32Xbox unk_4;
    u32Xbox animation_num;
    u32Xbox instance_num;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_CrowdValXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef struct CrowdItemRefXbox:
    const CrowdItemHeaderXbox *header;
    ref_slice_CrcXbox animations;
    ref_slice_CrowdValXbox instances;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_CrowdItemRefXbox:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct HkConstraintBoneRefXbox:
    string name;
    u32Xbox start;
    u32Xbox val;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_HkConstraintBoneRefXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_f32Xbox:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    AnimVals1RefXbox_Type1,
    AnimVals1RefXbox_Type2,
    AnimVals1RefXbox_Type3,
    AnimVals1RefXbox_Type4,
  ctypedef uint8_t AnimVals1RefXbox_Tag;

  cdef struct AnimVals1RefXbox:
    AnimVals1RefXbox_Tag tag;
    ref_slice_u8 type1;
    ref_slice_u16Xbox type2;
    ref_slice_u16Xbox type3;
    ref_slice_u16Xbox type4;

  cdef struct Obj1RefXbox:
    uint8_t flags;
    uint8_t s2;
    u16Xbox s1;
    ref_slice_u8 data;
    ref_slice_f32Xbox vals_a;
    AnimVals1RefXbox vals;
    uintptr_t size;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationPolar32Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationThreeComp40Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationThreeComp48Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationThreeComp24Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationStraight16Xbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationUncompressedXbox:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    RotationQuantizationRefXbox_Polar32,
    RotationQuantizationRefXbox_ThreeComp40,
    RotationQuantizationRefXbox_ThreeComp48,
    RotationQuantizationRefXbox_ThreeComp24,
    RotationQuantizationRefXbox_Straight16,
    RotationQuantizationRefXbox_Uncompressed,
  ctypedef uint8_t RotationQuantizationRefXbox_Tag;

  cdef struct RotationQuantizationRefXbox:
    RotationQuantizationRefXbox_Tag tag;
    ref_slice_RotationPolar32Xbox polar32;
    ref_slice_RotationThreeComp40Xbox three_comp40;
    ref_slice_RotationThreeComp48Xbox three_comp48;
    ref_slice_RotationThreeComp24Xbox three_comp24;
    ref_slice_RotationStraight16Xbox straight16;
    ref_slice_RotationUncompressedXbox uncompressed;

  cdef struct Obj2RefXbox:
    uint8_t flags;
    uint8_t s2;
    u16Xbox s1;
    ref_slice_u8 data;
    RotationQuantizationRefXbox vals;
    uintptr_t size;

  cdef struct BlockValARefXbox:
    Obj1RefXbox a;
    Obj2RefXbox b;
    Obj1RefXbox c;

  #gen_ffi:export
  cdef struct BufferInfoXbox:
    u32Xbox vbuff_info_offset;
    u32Xbox vbuff_info_offset_2;
    u32Xbox vbuff_info_offset_3;
    u32Xbox unk_3;
    u32Xbox unk_4;
    u32Xbox unk_5;
    u32Xbox unk_6;
    u32Xbox unk_7;
    u32Xbox unk_8;
    u32Xbox unk_9;
    u32Xbox unk_10;
    u32Xbox unk_11;
    u32Xbox unk_12;
    u32Xbox unk_13;
    u32Xbox unk_14;
    u32Xbox unk_15;
    u32Xbox unk_16;
    u32Xbox unk_17;
    u32Xbox unk_18;
    u32Xbox unk_19;
    u32Xbox unk_20;
    u32Xbox unk_21;
    u32Xbox unk_22;
    u32Xbox unk_23;
    u32Xbox unk_24;
    u32Xbox unk_25;
    u32Xbox unk_26;
    u32Xbox unk_27;
    u32Xbox unk_28;
    u32Xbox unk_29;
    u32Xbox unk_30;
    u32Xbox unk_31;
    u32Xbox v_size;
    u32Xbox v_size_2;
    u32Xbox v_size_3;
    u32Xbox unk_35;
    u32Xbox unk_36;
    u32Xbox unk_37;
    u32Xbox unk_38;
    u32Xbox unk_39;
    u32Xbox unk_40;
    u32Xbox unk_41;
    u32Xbox unk_42;
    u32Xbox unk_43;
    u32Xbox unk_44;
    u32Xbox unk_45;
    u32Xbox unk_46;
    u32Xbox unk_47;
    u32Xbox vbuff_size;
    u32Xbox vbuff_size_2;
    u32Xbox vbuff_size_3;
    u32Xbox unk_51;
    u32Xbox unk_52;
    u32Xbox unk_53;
    u32Xbox unk_54;
    u32Xbox unk_55;
    u32Xbox unk_56;
    u32Xbox unk_57;
    u32Xbox unk_58;
    u32Xbox unk_59;
    u32Xbox unk_60;
    u32Xbox unk_61;
    u32Xbox unk_62;
    u32Xbox unk_63;
    u32Xbox unk_64;
    u32Xbox ibuff_info_offset;
    u32Xbox i_num;
    u32Xbox unk_67;
    u32Xbox skin_offset;
    u32Xbox skin_size;
    u32Xbox unk_70;
    u32Xbox tri_num;
    u32Xbox unk_72;
    u32Xbox unk_73;
    u32Xbox unk_74;
    u32Xbox unk_75;
    u32Xbox unk_76;
    u32Xbox unk_77;
    u32Xbox unk_78;
    u32Xbox unk_79;
    u32Xbox unk_80;
    u32Xbox unk_81;
    u32Xbox unk_82;
    u32Xbox unk_83;
    u32Xbox unk_84;
    u32Xbox unk_85;
    u32Xbox unk_86;
    u32Xbox unk_87;
    u8Xbox variation_id;
    u8Xbox variation;
    u8Xbox unk_88c;
    u8Xbox unk_88d;

  #gen_ffi:export
  cdef struct HkConstraintDataXbox:
    u32Xbox kind;
    u32Xbox unk_1;
    u32Xbox unk_2;
    u32Xbox unk_3;
    u32Xbox unk_4;
    u32Xbox unk_5;
    u32Xbox unk_6;
    u32Xbox unk_7;
    u32Xbox unk_8;
    u32Xbox unk_9;
    u32Xbox unk_10;
    u32Xbox unk_11;
    u32Xbox unk_12;
    u32Xbox unk_13;
    u32Xbox unk_14;
    u32Xbox unk_15;
    u32Xbox unk_16;
    u32Xbox unk_17;
    u32Xbox unk_18;
    u32Xbox unk_19;
    u32Xbox unk_20;
    u32Xbox unk_21;
    u32Xbox unk_22;
    u32Xbox unk_23;
    u32Xbox unk_24;
    u32Xbox unk_25;
    u32Xbox unk_26;
    u32Xbox unk_27;
    u32Xbox unk_28;

  #gen_ffi:export
  cdef struct Key2Xbox:
    CrcXbox key;
    u32Xbox val;

  #gen_ffi:export
  cdef struct BlockValAXbox:
    f32Xbox unk_0;
    f32Xbox unk_1;
    f32Xbox unk_2;
    f32Xbox unk_3;
    f32Xbox unk_4;
    f32Xbox unk_5;
    f32Xbox unk_6;
    f32Xbox unk_7;
    f32Xbox unk_8;
    u32Xbox unk_9;
    u32Xbox unk_10;
    u32Xbox unk_11;

  #gen_ffi:export
  cdef struct BlockValBXbox:
    u16Xbox unk_0;
    u16Xbox unk_1;
    f32Xbox unk_2;
    f32Xbox unk_3;
    f32Xbox unk_4;
    f32Xbox unk_5;

  #gen_ffi:export
  cdef struct AnimationBlockInfoXbox:
    CrcXbox key;
    u32Xbox guid;
    CrcXbox key_name;
    u32Xbox offset;
    u32Xbox size;
    u32Xbox size_comp;
    u32Xbox unk_6;
    u32Xbox unk_7;
    u32Xbox unk_8;

  #gen_ffi:export
  cdef struct AssetHandleXbox:
    CrcXbox key;
    u32Xbox offset;
    u32Xbox size;
    u32Xbox size_comp;
    u32Xbox kind;

  ctypedef U32LE U32Xbox;

  ctypedef int32_t I32BE;

  ctypedef I32BE I32Xbox;

  cdef struct BlockAValXbox:
    U32Xbox unk_0;
    I32Xbox gamemodemask;
    U32Xbox key;
    U32Xbox unk_3;
    U32Xbox unk_4;
    U32Xbox unk_5;
    U32Xbox unk_6;

  #gen_ffi:export
  cdef struct GFXBlockInfoXbox:
    CrcXbox key;
    u32Xbox offset;
    u32Xbox size;

  #gen_ffi:export
  cdef struct HkConstraintInfoXbox:
    u32Xbox kind;
    u32Xbox bone_parents_offset;
    u32Xbox bone_parents_num;
    u32Xbox bone_names_offset;
    u32Xbox bone_names_num;
    u32Xbox bone_transforms_offset;
    u32Xbox bone_transforms_num;
    u32Xbox unk_7;
    u32Xbox unk_8;
    u32Xbox unk_9;
    u32Xbox bones_offset;
    u16Xbox bones_num;
    u16Xbox bone_order_num;
    u32Xbox bone_order_offset;
    u32Xbox unk_13;
    f32Xbox unk_14;
    u32Xbox vals2_num;
    u32Xbox vals2_offset;
    u32Xbox unk_17;

  #gen_ffi:export
  cdef struct Obj0Xbox:
    u32Xbox unk_0;
    CrcXbox key;

  #gen_ffi:export
  cdef struct ObjAXbox:
    CrcXbox key;
    u32Xbox unk_1;
    u32Xbox size;
    u32Xbox size_comp;
    u32Xbox unk_4;
    u32Xbox kind;

  #gen_ffi:export
  cdef struct PFieldInfoXbox:
    u32Xbox link_guid;
    u32Xbox gamemode_guid;
    u32Xbox width;
    u32Xbox height;
    u32Xbox offset;

  #gen_ffi:export
  cdef struct StringKeysValXbox:
    CrcXbox key;
    u32Xbox offset;

  #gen_ffi:export
  cdef struct SubBlocksBlockHeaderXbox:
    CrcXbox key;
    u32Xbox offset;
    u32Xbox size;

  #gen_ffi:export
  cdef struct Obj3Xbox:
    f32Xbox t;
    CrcXbox event;
    u32Xbox dat_2;
    u32Xbox dat_3;
    u32Xbox dat_4;
    u32Xbox dat_5;
    u32Xbox dat_6;
    u32Xbox dat_7;
    u32Xbox dat_8;
    u32Xbox dat_9;
    u32Xbox dat_10;

  #gen_ffi:export
  cdef struct Obj5ValXbox:
    f32Xbox unk_0;
    f32Xbox unk_1;
    f32Xbox unk_2;
    f32Xbox unk_3;
    f32Xbox unk_4;
    f32Xbox unk_5;
    f32Xbox unk_6;

  #gen_ffi:export
  cdef struct SSAValXbox:
    f32Xbox t_start;
    f32Xbox t_end;
    u32Xbox unk_2;
    u32Xbox unk_3;
    u32Xbox off;

  #gen_ffi:export
  cdef struct TypeFieldXbox:
    CrcXbox key;
    CrcXbox kind;
    u32Xbox offset;

  ctypedef int16_t i16_be;

  ctypedef i16_be i16Xbox;

  #gen_ffi:export
  cdef struct FoliageValXbox:
    u16Xbox height;
    u16Xbox var_mask;
    i16Xbox slope_x;
    i16Xbox slope_z;

  #gen_ffi:export
  cdef struct WeightXbox:
    u32Xbox x;
    u8Xbox a;
    u8Xbox b;
    u8Xbox c;
    u8Xbox d;

  #gen_ffi:export
  cdef struct AtlasUVValXbox:
    CrcXbox key;
    Vector4Xbox vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AtlasUVValXbox:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct SprayInstanceXbox:
    CrcXbox key;
    CrcXbox tex1;
    CrcXbox tex2;
    u32Xbox unk_3;
    u32Xbox width;
    u32Xbox height;
    f32Xbox unk_6;
    f32Xbox unk_7;
    u32Xbox size_w;
    u32Xbox size_h;
    f32Xbox scale_w;
    f32Xbox scale_h;
    u32Xbox delay;
    f32Xbox stride_x;
    f32Xbox stride_y;
    u32Xbox unk_15;
    u32Xbox unk_16;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SprayInstanceXbox:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct SprayValXbox:
    Vector3Xbox position;
    f32Xbox scale;
    u16Xbox instance;
    u16Xbox rotation;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SprayValXbox:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct TRSXbox:
    Vector4Xbox translation;
    Vector4Xbox rotation;
    Vector4Xbox scale;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_TRSXbox:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_i16Xbox:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct CrowdValXbox:
    Vector3Xbox position;
    f32Xbox rotation;
    f32Xbox lod;

  #gen_ffi:export
  cdef struct RotationPolar32Xbox:
    u32Xbox a;

  #gen_ffi:export
  cdef struct RotationStraight16Xbox:
    u8Xbox a;
    u8Xbox b;

  #gen_ffi:export
  cdef struct RotationThreeComp24Xbox:
    u8Xbox a;
    u8Xbox b;
    u8Xbox c;

  #gen_ffi:export
  cdef struct RotationThreeComp40Xbox:
    u8Xbox a;
    u8Xbox b;
    u8Xbox c;
    u8Xbox d;
    u8Xbox e;

  #gen_ffi:export
  cdef struct RotationThreeComp48Xbox:
    u16Xbox a;
    u16Xbox b;
    u16Xbox c;

  #gen_ffi:export
  cdef struct RotationUncompressedXbox:
    f32Xbox a;
    f32Xbox b;
    f32Xbox c;
    f32Xbox d;

  #gen_ffi:export
  cdef struct HkConstraintRefXbox:
    const HkConstraintInfoXbox *info;
    ref_slice_i16Xbox bone_parents;
    slice_HkConstraintBoneRefXbox bone_names;
    ref_slice_u32Xbox name_offsets;
    ref_slice_TRSXbox bone_transforms;
    ref_slice_u32Xbox bones;
    ref_slice_Key2Xbox bones_order;
    ref_slice_f32Xbox vals2;

  #gen_ffi:export
  cdef struct ShapeExtraInfoXbox:
    u32Xbox size;
    f32Xbox scale;
    f32Xbox a;
    f32Xbox b;

  cdef struct ShapeExtraRefXbox:
    const ShapeExtraInfoXbox *info;
    ref_slice_u32Xbox offs;
    ref_slice_u8 data;

  cdef struct AtlasUVRefXbox:
    ref_slice_AtlasUVValXbox vals;

  cdef struct BlocksRefXbox:
    ref_slice_u32Xbox block_starts;
    ref_slice_u32Xbox block_starts2;
    ref_slice_u32Xbox obj_c3;
    ref_slice_u32Xbox obj_c4;
    slice_BlockValRefXbox blocks;

  #gen_ffi:export
  cdef struct CrowdHeaderXbox:
    u32Xbox const0x65;
    u32Xbox n;

  cdef struct CrowdRefXbox:
    const CrowdHeaderXbox *header;
    ref_slice_u32Xbox offs;
    slice_CrowdItemRefXbox vals;

  ctypedef DataRefXbox PFieldsRefXbox;

  cdef struct SprayRefXbox:
    ref_slice_SprayInstanceXbox instances;
    ref_slice_SprayValXbox vals;

  #gen_ffi:export
  cdef struct IBuffInfoPs3:
    u32Ps3 unk_0;
    u32Ps3 unk_5;
    u32Ps3 unk_6;
    u32Ps3 vbuff_alt_fmt;
    u32Ps3 unk_8;
    u32Ps3 size;
    u32Ps3 format;
    u32Ps3 unk_7;
    u32Ps3 offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_u16Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    IndexBufferValsRefPs3_U16,
    IndexBufferValsRefPs3_U32,
  ctypedef uint8_t IndexBufferValsRefPs3_Tag;

  cdef struct IndexBufferValsRefPs3:
    IndexBufferValsRefPs3_Tag tag;
    ref_slice_u16Ps3 u16;
    ref_slice_u32Ps3 u32;

  cdef struct IndexBufferRefPs3:
    const IBuffInfoPs3 *info;
    IndexBufferValsRefPs3 vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__IndexBufferRefPs3:
    uint64_t align;
    uint8_t pad[64];

  #gen_ffi:export
  cdef struct VBuffInfoPs3:
    u32Ps3 unk_0;
    u32Ps3 unk_7;
    u32Ps3 unk_3;
    u32Ps3 unk_8;
    u32Ps3 unk_9;
    u32Ps3 size;
    u32Ps3 unk_6;
    u32Ps3 offset;
    u32Ps3 fmt2;
    u32Ps3 fmt1;

  cdef struct VertexBufferRefPs3:
    const VBuffInfoPs3 *info;
    IndexMap_VertexUsage__VertexDataIndex offsets;
    uintptr_t size;
    ref_slice_u8 data;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__VertexBufferRefPs3:
    uint64_t align;
    uint8_t pad[64];

  ctypedef u32Ps3 CrcPs3;

  ctypedef u64_be u64Ps3;

  ctypedef uint8_t u8Ps3;

  ctypedef f32_be f32Ps3;

  #gen_ffi:export
  cdef struct MatBasePs3:
    u32Ps3 unk_0;
    CrcPs3 tex0;
    CrcPs3 tex1;
    CrcPs3 tex2;
    CrcPs3 tex3;
    CrcPs3 tex4;
    CrcPs3 tex5;
    CrcPs3 key_guid;
    CrcPs3 mask0;
    CrcPs3 mask1;
    CrcPs3 mask2;
    u32Ps3 unk_12;
    u32Ps3 unk_13;
    u32Ps3 unk_14;
    u32Ps3 unk_15;
    u32Ps3 unk_16;
    u32Ps3 unk_17;
    u32Ps3 unk_18;
    u32Ps3 unk_19;
    u32Ps3 unk_20;
    u32Ps3 unk_21;
    u32Ps3 unk_22;
    u32Ps3 unk_23;
    u32Ps3 unk_24;
    u32Ps3 unk_25;
    u32Ps3 unk_26;
    u32Ps3 unk_27;
    u32Ps3 unk_28;
    u32Ps3 unk_29;
    u32Ps3 unk_30;
    u32Ps3 unk_31;
    u32Ps3 unk_32;
    u32Ps3 unk_33;
    u32Ps3 z_35;
    u32Ps3 z_36;
    u32Ps3 z_37;
    u32Ps3 z_38;
    u32Ps3 z_39;
    u32Ps3 unk_40;
    u32Ps3 unk_41;
    u32Ps3 unk_42;
    u32Ps3 unk_43;
    u32Ps3 unk_44;
    u32Ps3 unk_45;
    u32Ps3 unk_46;
    u32Ps3 unk_47;
    u32Ps3 unk_48;
    u32Ps3 unk_49;
    u64Ps3 flags;
    u32Ps3 kind;
    u32Ps3 unk_53;
    u8Ps3 unk_54a;
    u8Ps3 unk_54b;
    u16Ps3 side_flags;
    u32Ps3 unk_55;
    u32Ps3 unk_56;
    u32Ps3 unk_57;
    f32Ps3 unk_58;
    f32Ps3 unk_59;
    f32Ps3 unk_60;
    f32Ps3 unk_61;
    f32Ps3 unk_62;
    f32Ps3 unk_63;
    f32Ps3 unk_64;
    f32Ps3 unk_65;
    f32Ps3 unk_66;
    f32Ps3 unk_67;
    f32Ps3 unk_68;
    f32Ps3 unk_69;
    u32Ps3 unk_70;
    u32Ps3 unk_71;
    u32Ps3 unk_72;
    f32Ps3 unk_73;
    f32Ps3 unk_74;
    f32Ps3 unk_75;
    f32Ps3 unk_76;
    u32Ps3 unk_77;
    f32Ps3 unk_78;
    f32Ps3 unk_79;
    f32Ps3 unk_80;
    f32Ps3 unk_81;
    u32Ps3 unk_82;
    u32Ps3 unk_83;
    u32Ps3 unk_84;
    f32Ps3 unk_85;
    u32Ps3 mat_extra_offset;
    CrcPs3 key;
    u32Ps3 unk_88;
    u32Ps3 z_89;

  #gen_ffi:export
  cdef struct Mat1Ps3:
    MatBasePs3 base;

  #gen_ffi:export
  cdef struct MatExtraPs3:
    u32Ps3 unk_0;
    u32Ps3 unk_1;
    u32Ps3 unk_2;
    u32Ps3 unk_3;
    u32Ps3 unk_4;
    u32Ps3 unk_5;
    u32Ps3 unk_6;
    u32Ps3 unk_7;
    u32Ps3 unk_8;
    u32Ps3 unk_9;
    u32Ps3 unk_10;
    f32Ps3 unk_11;
    f32Ps3 unk_12;
    f32Ps3 unk_13;
    f32Ps3 unk_14;
    f32Ps3 unk_15;
    u32Ps3 unk_16;
    f32Ps3 unk_17;
    f32Ps3 unk_18;
    f32Ps3 unk_19;
    f32Ps3 unk_20;
    f32Ps3 unk_21;
    u32Ps3 unk_22;
    u32Ps3 unk_23;
    u32Ps3 unk_24;
    u32Ps3 unk_25;
    u32Ps3 unk_26;
    u32Ps3 unk_27;
    u32Ps3 unk_28;
    u32Ps3 unk_29;
    u32Ps3 unk_30;
    u32Ps3 unk_31;
    u32Ps3 unk_32;
    u32Ps3 unk_33;
    u32Ps3 unk_34;
    u32Ps3 unk_35;
    u32Ps3 unk_36;
    u32Ps3 unk_37;
    u32Ps3 unk_38;
    u32Ps3 unk_39;
    u32Ps3 unk_40;
    u32Ps3 unk_41;
    u32Ps3 unk_42;
    u32Ps3 unk_43;
    u32Ps3 unk_44;
    u32Ps3 unk_45;
    u32Ps3 unk_46;
    u32Ps3 unk_47;
    u32Ps3 unk_48;
    u32Ps3 unk_49;

  cdef struct Mat1RefPs3:
    const Mat1Ps3 *info;
    const MatExtraPs3 *extra;

  #gen_ffi:export
  cdef struct Mat2Ps3:
    MatBasePs3 base;
    u32Ps3 unk_90;
    u32Ps3 unk_91;
    u32Ps3 unk_92;
    u32Ps3 unk_93;
    u32Ps3 unk_94;
    u32Ps3 unk_95;
    u32Ps3 unk_96;
    u32Ps3 unk_97;
    u32Ps3 unk_98;
    u32Ps3 unk_99;
    u32Ps3 unk_100;
    u32Ps3 unk_101;
    f32Ps3 unk_102;
    f32Ps3 unk_103;
    f32Ps3 unk_104;
    f32Ps3 unk_105;
    f32Ps3 unk_106;
    f32Ps3 unk_107;
    f32Ps3 unk_108;
    f32Ps3 unk_109;
    f32Ps3 unk_110;
    f32Ps3 unk_111;
    f32Ps3 unk_112;
    f32Ps3 unk_113;
    u32Ps3 unk_114;
    u32Ps3 unk_115;
    u32Ps3 unk_116;
    CrcPs3 unk_117;
    CrcPs3 unk_118;
    CrcPs3 unk_119;
    u8Ps3 unk_120a;
    u8Ps3 unk_120b;
    u8Ps3 unk_120c;
    u8Ps3 unk_120d;
    u32Ps3 unk_121;

  cdef struct Mat2RefPs3:
    const Mat2Ps3 *info;
    const MatExtraPs3 *extra;

  #gen_ffi:export
  cdef struct Mat3Ps3:
    MatBasePs3 base;
    f32Ps3 unk_90;
    f32Ps3 unk_91;
    f32Ps3 unk_92;
    f32Ps3 unk_93;
    f32Ps3 unk_94;
    f32Ps3 unk_95;
    f32Ps3 unk_96;
    f32Ps3 unk_97;
    f32Ps3 unk_98;
    f32Ps3 unk_99;
    f32Ps3 unk_100;
    f32Ps3 unk_101;
    f32Ps3 unk_102;
    f32Ps3 unk_103;
    f32Ps3 unk_104;
    f32Ps3 unk_105;
    f32Ps3 unk_106;
    f32Ps3 unk_107;
    f32Ps3 unk_108;
    f32Ps3 unk_109;
    f32Ps3 unk_110;
    f32Ps3 unk_111;
    f32Ps3 unk_112;
    f32Ps3 unk_113;
    u8Ps3 variation_id_color;
    u8Ps3 variation_id_texture;
    u8Ps3 variation_id_specular;
    u8Ps3 unk_114d;
    u32Ps3 unk_115;
    u32Ps3 unk_116;
    u32Ps3 unk_117;

  cdef struct Mat3RefPs3:
    const Mat3Ps3 *info;
    const MatExtraPs3 *extra;

  #gen_ffi:export
  cdef struct Mat4Ps3:
    MatBasePs3 base;
    f32Ps3 unk_90;
    u32Ps3 unk_91;
    u32Ps3 unk_92;
    u32Ps3 unk_93;
    u32Ps3 unk_94;
    u32Ps3 unk_95;
    f32Ps3 unk_96;
    u32Ps3 unk_97;
    f32Ps3 unk_98;
    u32Ps3 unk_99;
    u32Ps3 unk_100;
    u32Ps3 unk_101;
    u32Ps3 unk_102;
    u32Ps3 unk_103;
    f32Ps3 unk_104;
    u32Ps3 unk_105;
    f32Ps3 unk_106;
    u32Ps3 unk_107;
    u32Ps3 unk_108;
    f32Ps3 unk_109;
    u32Ps3 unk_110;
    u32Ps3 unk_111;
    f32Ps3 unk_112;
    f32Ps3 unk_113;
    f32Ps3 unk_114;
    u32Ps3 unk_115;
    u32Ps3 unk_116;
    f32Ps3 unk_117;
    u32Ps3 unk_118;
    u32Ps3 unk_119;
    f32Ps3 unk_120;
    f32Ps3 unk_121;
    f32Ps3 unk_122;
    u32Ps3 unk_123;
    u32Ps3 unk_124;
    f32Ps3 unk_125;
    u32Ps3 unk_126;
    u32Ps3 unk_127;
    f32Ps3 unk_128;
    f32Ps3 unk_129;
    f32Ps3 unk_130;
    u32Ps3 unk_131;
    u32Ps3 unk_132;
    f32Ps3 unk_133;
    u32Ps3 unk_134;
    u32Ps3 unk_135;
    f32Ps3 unk_136;
    f32Ps3 unk_137;
    f32Ps3 unk_138;
    u32Ps3 unk_139;
    u32Ps3 unk_140;
    f32Ps3 unk_141;
    u32Ps3 unk_142;
    u32Ps3 unk_143;
    f32Ps3 unk_144;
    f32Ps3 unk_145;

  cdef struct Mat4RefPs3:
    const Mat4Ps3 *info;
    const MatExtraPs3 *extra;

  cdef enum:
    MatRefPs3_Mat1,
    MatRefPs3_Mat2,
    MatRefPs3_Mat3,
    MatRefPs3_Mat4,
  ctypedef uint8_t MatRefPs3_Tag;

  cdef struct MatRefPs3:
    MatRefPs3_Tag tag;
    Mat1RefPs3 mat1;
    Mat2RefPs3 mat2;
    Mat3RefPs3 mat3;
    Mat4RefPs3 mat4;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__MatRefPs3:
    uint64_t align;
    uint8_t pad[64];

  ctypedef i32_be i32Ps3;

  #gen_ffi:export
  cdef struct Vector3Ps3:
    f32Ps3 x;
    f32Ps3 y;
    f32Ps3 z;

  #gen_ffi:export
  cdef struct BoundingBoxPs3:
    Vector3Ps3 center;
    f32Ps3 unk_3;
    Vector3Ps3 half_width;
    f32Ps3 unk_7;

  #gen_ffi:export
  cdef struct LodInfoPs3:
    u32Ps3 start;
    u32Ps3 static_end;
    u32Ps3 skinned_end;
    u32Ps3 physics_end;
    u32Ps3 breakable_end;

  #gen_ffi:export
  cdef struct ModelInfoPs3:
    CrcPs3 key;
    i32Ps3 gamemodemask;
    u32Ps3 mat_offset;
    u32Ps3 buffer_info_offset;
    BoundingBoxPs3 bounding_box;
    u32Ps3 mesh_order_offset;
    LodInfoPs3 lod0;
    LodInfoPs3 lod1;
    LodInfoPs3 lod2;
    LodInfoPs3 lod3;
    u32Ps3 mat_num;
    u32Ps3 bones_offset;
    u32Ps3 bone_parents_offset;
    u32Ps3 bone_transforms_offset;
    u32Ps3 bones_num;
    u32Ps3 skin_binds_offset;
    u32Ps3 skin_binds_num;
    u32Ps3 skin_order_offset;
    u32Ps3 vbuff_offset;
    u32Ps3 vbuff_num;
    u32Ps3 ibuff_offset;
    u32Ps3 ibuff_num;
    u32Ps3 mesh_bounding_boxes_offset;
    f32Ps3 unk_46;
    u32Ps3 variation_counts;
    u32Ps3 vals_j_num;
    u32Ps3 vals_j_offset;
    u32Ps3 block_offset;
    u32Ps3 vals_k_offset;
    CrcPs3 asset_key;
    u32Ps3 asset_type;
    u32Ps3 unk_54;
    u32Ps3 unk_55;
    u32Ps3 shape_offset;
    u32Ps3 shape_num;
    u32Ps3 hk_constraint_data_offset;
    u32Ps3 hk_constraint_data_num;
    u32Ps3 hk_constraint_offset;
    u32Ps3 slots_offset;
    u32Ps3 slot_map_offset;
    u32Ps3 bone_bounding_boxes_offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_CrcPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_i32Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Matrix4x4Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BoundingBoxPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BonesRefPs3:
    ref_slice_CrcPs3 names;
    ref_slice_i32Ps3 parents;
    ref_slice_Matrix4x4Ps3 transforms;
    ref_slice_BoundingBoxPs3 bounding_boxes;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Key2Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_BlockRefPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32_______VBuffInfoPs3:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32_______IBuffInfoPs3:
    uint64_t align;
    uint8_t pad[64];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_HkConstraintRefPs3:
    uint64_t align;
    uint8_t pad[112];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_ShapeRefPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct ModelDataRefPs3:
    ref_slice_BufferInfoPs3 infos;
    ref_slice_u32Ps3 vbuff_order;
    ref_slice_u32Ps3 ibuff_order;
    IndexMap_u32__VertexBufferRefPs3 vertex;
    IndexMap_u32__IndexBufferRefPs3 index;
    const CompressedDataRef *data;

  cdef struct ModelRefPs3:
    const ModelInfoPs3 *info;
    BonesRefPs3 bones;
    ref_slice_u32Ps3 mat_order;
    ref_slice_u32Ps3 mesh_order;
    ref_slice_BoundingBoxPs3 mesh_bounding_boxes;
    ref_slice_Matrix4x4Ps3 skin_binds;
    ref_slice_u32Ps3 vals_j;
    ref_slice_u16Ps3 val_k_header;
    ref_slice_u32Ps3 vals_k;
    ref_slice_u32Ps3 skin_order;
    ref_slice_Key2Ps3 slots;
    ref_slice_u32Ps3 slot_map;
    const u32Ps3 *block_header;
    ref_slice_u32Ps3 block_offsets;
    slice_BlockRefPs3 blocks;
    ref_slice_BufferInfoPs3 buffer_infos;
    ref_slice_u32Ps3 vbuff_order;
    ref_slice_u32Ps3 ibuff_order;
    IndexMap_u32_______VBuffInfoPs3 vbuffs;
    IndexMap_u32_______IBuffInfoPs3 ibuffs;
    IndexMap_u32__MatRefPs3 mats;
    Option_HkConstraintRefPs3 hk_constraint;
    ref_slice_HkConstraintDataPs3 hk_constraint_datas;
    slice_ShapeRefPs3 shapes;
    ModelDataRefPs3 data;

  #gen_ffi:export
  cdef struct AnimationInfoPs3:
    CrcPs3 key;
    i32Ps3 gamemodemask;
    u32Ps3 offset;
    u32Ps3 size;
    u32Ps3 kind;
    f32Ps3 unk_5;
    u32Ps3 vals_num;
    u32Ps3 vals2_num;
    u32Ps3 unk_8;
    u32Ps3 vala;
    u32Ps3 unk_10;
    u32Ps3 unk_11;
    u32Ps3 data_offset;
    f32Ps3 unk_13;
    f32Ps3 unk_14;
    f32Ps3 t_scale;
    u32Ps3 block_starts_offset;
    u32Ps3 block_starts_num;
    u32Ps3 block_starts2_offset;
    u32Ps3 block_starts2_num;
    u32Ps3 obj_c3_offset;
    u32Ps3 obj_c3_num;
    u32Ps3 obj_c4_offset;
    u32Ps3 obj_c4_num;
    u32Ps3 block_offset;
    u32Ps3 block_size;
    u32Ps3 obj3_num;
    u32Ps3 obj3_offset;
    u32Ps3 bones_num1;
    u32Ps3 unk_29;
    u32Ps3 obj1_num;
    u32Ps3 bones_offset;
    u32Ps3 unk_32;
    u32Ps3 obj1_offset;
    u32Ps3 obj2_offset;
    u32Ps3 obj2_num;
    u32Ps3 obj5_offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Obj3Ps3:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct Obj5HeaderPs3:
    u32Ps3 obj_a_num;
    u32Ps3 obj_a_offset;
    u32Ps3 obj_b_num;
    u32Ps3 obj_b_offset;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Obj5ValPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_BlocksRefPs3:
    uint64_t align;
    uint8_t pad[72];

  cdef struct AnimationRefPs3:
    const AnimationInfoPs3 *info;
    ref_slice_u32Ps3 obj1;
    ref_slice_u32Ps3 obj2;
    ref_slice_Obj3Ps3 obj3;
    ref_slice_CrcPs3 bones;
    const Obj5HeaderPs3 *obj5_header;
    ref_slice_Obj5ValPs3 obj5_a;
    ref_slice_Obj5ValPs3 obj5_b;
    Option_BlocksRefPs3 blocks;
    uintptr_t size;

  cdef struct DataRefPs3:
    ref_slice_u8 data;

  #gen_ffi:export
  cdef struct EffectInfoPs3:
    CrcPs3 key;
    i32Ps3 gamemodemask;
    u32Ps3 offset;
    u32Ps3 size;

  cdef struct EffectRefPs3:
    const EffectInfoPs3 *info;
    GameObjsRefPs3 vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__ref_slice_u16Ps3:
    uint64_t align;
    uint8_t pad[64];

  cdef struct LangStringsRefPs3:
    IndexMap_u32__ref_slice_u16Ps3 strings;

  ctypedef DataRefPs3 LuaRefPs3;

  #gen_ffi:export
  cdef struct ObjHeaderPs3:
    u32Ps3 layer;
    CrcPs3 key;
    u16Ps3 size;
    u16Ps3 z3;
    u32Ps3 z4;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct IndexMap_u32__BaseTypeRefPs3:
    uint64_t align;
    uint8_t pad[64];

  cdef struct ObjRefPs3:
    const ObjHeaderPs3 *header;
    IndexMap_u32__BaseTypeRefPs3 fields;

  #gen_ffi:export
  cdef struct RadiosityValsInfoPs3:
    u32Ps3 guid;
    u32Ps3 num;
    u32Ps3 offset;

  cdef struct RadiosityValsRefPs3:
    const RadiosityValsInfoPs3 *info;
    ref_slice_i32Ps3 offs;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SSAValPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_ref_slice_u16Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct SSARefPs3:
    ref_slice_SSAValPs3 vals;
    slice_ref_slice_u16Ps3 strings;

  #gen_ffi:export
  cdef struct TextureInfoPs3:
    CrcPs3 key;
    i32Ps3 gamemodemask;
    CrcPs3 asset_key;
    u32Ps3 asset_type;
    u32Ps3 kind;
    u32Ps3 format;
    u32Ps3 unk_6;
    u32Ps3 unk_7;
    u32Ps3 unk_8;
    u32Ps3 unk_9;
    u32Ps3 unk_10;
    u32Ps3 unk_11;
    u16Ps3 width;
    u16Ps3 height;
    u16Ps3 depth;
    u16Ps3 levels;
    u8Ps3 unk_16_1;
    u8Ps3 unk_16_2;
    u8Ps3 unk_16_3;
    u8Ps3 unk_16_4;
    u8Ps3 unk_16_5;
    u8Ps3 unk_16_6;
    u8Ps3 unk_16_7;
    u8Ps3 unk_16_8;
    u8Ps3 unk_16_9;
    u8Ps3 unk_16_10;
    u8Ps3 unk_16_11;
    u8Ps3 unk_16_12;
    u8Ps3 unk_16_13;
    u8Ps3 unk_16_14;
    u8Ps3 unk_16_15;
    u8Ps3 unk_16_16;

  cdef struct TextureRefPs3:
    const TextureInfoPs3 *info;
    const CompressedDataRef *data0;
    const CompressedDataRef *data1;

  #gen_ffi:export
  cdef struct TypeHeaderPs3:
    CrcPs3 key;
    u32Ps3 size;
    u32Ps3 fields;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_TypeFieldPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct TypeRefPs3:
    const TypeHeaderPs3 *header;
    ref_slice_TypeFieldPs3 fields;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_FoliageRefPs3:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct Vector2Ps3:
    f32Ps3 x;
    f32Ps3 y;

  #gen_ffi:export
  cdef struct Vector4Ps3:
    f32Ps3 x;
    f32Ps3 y;
    f32Ps3 z;
    f32Ps3 w;

  #gen_ffi:export
  cdef struct Matrix4x4Ps3:
    Vector4Ps3 x;
    Vector4Ps3 y;
    Vector4Ps3 z;
    Vector4Ps3 w;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_U32Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Vector4Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_WeightPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    BaseTypeRefPs3_Crc,
    BaseTypeRefPs3_GUID,
    BaseTypeRefPs3_Color,
    BaseTypeRefPs3_Vector2,
    BaseTypeRefPs3_Vector3,
    BaseTypeRefPs3_Vector4,
    BaseTypeRefPs3_Matrix4x4,
    BaseTypeRefPs3_Float,
    BaseTypeRefPs3_Int,
    BaseTypeRefPs3_Bool,
    BaseTypeRefPs3_String,
    BaseTypeRefPs3_StringList,
    BaseTypeRefPs3_ObjectList,
    BaseTypeRefPs3_NodeList,
    BaseTypeRefPs3_IntList,
    BaseTypeRefPs3_CrcList,
    BaseTypeRefPs3_WeightList,
    BaseTypeRefPs3_MatrixList,
  ctypedef uint8_t BaseTypeRefPs3_Tag;

  cdef struct BaseTypeRefPs3:
    BaseTypeRefPs3_Tag tag;
    const CrcPs3 *crc;
    const u32Ps3 *guid;
    const u32Ps3 *color;
    const Vector2Ps3 *vector2;
    const Vector3Ps3 *vector3;
    const Vector4Ps3 *vector4;
    const Matrix4x4Ps3 *matrix4x4;
    const f32Ps3 *float_;
    const i32Ps3 *int_;
    const u32Ps3 *bool_;
    string string;
    slice_string string_list;
    ref_slice_U32Ps3 object_list;
    ref_slice_Vector4Ps3 node_list;
    ref_slice_i32Ps3 int_list;
    ref_slice_U32Ps3 crc_list;
    ref_slice_WeightPs3 weight_list;
    ref_slice_Matrix4x4Ps3 matrix_list;

  #gen_ffi:export
  cdef struct BlockHeader1Ps3:
    u32Ps3 a;
    u32Ps3 b;
    u32Ps3 unk_2;
    u32Ps3 unk_3;

  #gen_ffi:export
  cdef struct BlockHeader2Ps3:
    u32Ps3 n;
    f32Ps3 unk_1;
    f32Ps3 unk_2;
    u32Ps3 unk_3;
    u32Ps3 unk_4;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BlockValAPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_BlockValBPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BlockRefPs3:
    const BlockHeader1Ps3 *info1;
    const BlockHeader2Ps3 *info2;
    ref_slice_BlockValAPs3 vals_a;
    ref_slice_BlockValAPs3 vals_b;
    ref_slice_BlockValBPs3 vals_c;
    ref_slice_u8 pad;

  #gen_ffi:export
  cdef struct ShapeInfoPs3:
    u32Ps3 offset;
    u32Ps3 kind;
    u32Ps3 unk_2;
    f32Ps3 unk_3;
    f32Ps3 unk_4;
    f32Ps3 unk_5;
    Vector3Ps3 translation;
    Vector4Ps3 rotation;
    f32Ps3 unk_13;
    f32Ps3 unk_14;
    f32Ps3 unk_15;
    f32Ps3 unk_16;
    f32Ps3 unk_17;
    f32Ps3 unk_18;
    f32Ps3 unk_19;
    f32Ps3 unk_20;
    f32Ps3 unk_21;
    f32Ps3 unk_22;
    f32Ps3 unk_23;
    f32Ps3 unk_24;
    f32Ps3 unk_25;
    f32Ps3 unk_26;
    u32Ps3 hk_shape_num;
    u32Ps3 hk_shape_offset;
    u8Ps3 unk_29a;
    u8Ps3 unk_29b;
    u8Ps3 unk_29c;
    u8Ps3 unk_29d;
    f32Ps3 unk_30;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct Option_ShapeExtraRefPs3:
    uint64_t align;
    uint8_t pad[32];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_HkShapeRefPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct ShapeRefPs3:
    const ShapeInfoPs3 *info;
    Option_ShapeExtraRefPs3 extra;
    slice_HkShapeRefPs3 hk_shapes;

  #gen_ffi:export
  cdef struct BoxShapePs3:
    Vector4Ps3 translation;
    Vector4Ps3 rotation;
    u32Ps3 kind;
    CrcPs3 key;
    Vector3Ps3 half_extents;
    u32Ps3 unk_13;
    u32Ps3 unk_14;
    f32Ps3 unk_15;
    f32Ps3 unk_16;
    f32Ps3 unk_17;
    f32Ps3 unk_18;
    f32Ps3 unk_19;

  #gen_ffi:export
  cdef struct SphereShapePs3:
    Vector4Ps3 translation;
    Vector4Ps3 rotation;
    u32Ps3 kind;
    CrcPs3 key;
    f32Ps3 radius;
    u32Ps3 unk_11;
    f32Ps3 unk_12;
    f32Ps3 unk_13;
    f32Ps3 unk_14;
    f32Ps3 unk_15;
    f32Ps3 unk_16;
    f32Ps3 unk_17;
    f32Ps3 unk_18;
    f32Ps3 unk_19;

  #gen_ffi:export
  cdef struct CapsuleShapePs3:
    Vector4Ps3 translation;
    Vector4Ps3 rotation;
    u32Ps3 kind;
    CrcPs3 key;
    Vector3Ps3 point1;
    Vector3Ps3 point2;
    f32Ps3 radius;
    f32Ps3 unk_17;
    f32Ps3 unk_18;
    f32Ps3 unk_19;

  #gen_ffi:export
  cdef struct CylinderShapePs3:
    Vector4Ps3 translation;
    Vector4Ps3 rotation;
    u32Ps3 kind;
    CrcPs3 key;
    Vector3Ps3 point1;
    Vector3Ps3 point2;
    f32Ps3 radius;
    f32Ps3 unk_17;
    f32Ps3 unk_18;
    f32Ps3 unk_19;

  #gen_ffi:export
  cdef struct ConvexVerticesInfoPs3:
    Vector4Ps3 translation;
    Vector4Ps3 rotation;
    u32Ps3 kind;
    CrcPs3 key;
    u32Ps3 norm_num;
    u32Ps3 norms_offset;
    u32Ps3 vert_num;
    u32Ps3 verts_offset;
    f32Ps3 unk_14;
    f32Ps3 unk_15;
    f32Ps3 unk_16;
    f32Ps3 unk_17;
    f32Ps3 unk_18;
    f32Ps3 unk_19;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_Vector3Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct ConvexVerticesRefPs3:
    const ConvexVerticesInfoPs3 *info;
    ref_slice_Vector4Ps3 norms;
    ref_slice_Vector3Ps3 verts;

  #gen_ffi:export
  cdef struct BVTreeMeshInfoPs3:
    Vector4Ps3 translation;
    Vector4Ps3 rotation;
    u32Ps3 kind;
    CrcPs3 key;
    Vector3Ps3 offset;
    f32Ps3 tree_scale;
    u32Ps3 tree_size;
    u32Ps3 tree_offset;
    u32Ps3 vert_num;
    u32Ps3 verts_offset;
    u32Ps3 tri_num;
    u32Ps3 inds_offset;

  cdef struct BVTreeMeshRefPs3:
    const BVTreeMeshInfoPs3 *info;
    ref_slice_u8 tree;
    ref_slice_Vector3Ps3 verts;
    ref_slice_u16Ps3 inds;

  #gen_ffi:export
  cdef struct HkShapeInfoPs3:
    Vector4Ps3 unk_0;
    Vector4Ps3 unk_4;
    u32Ps3 kind;
    u32Ps3 unk_9;
    u32Ps3 unk_10;
    u32Ps3 unk_11;
    u32Ps3 unk_12;
    u32Ps3 unk_13;
    u32Ps3 unk_14;
    u32Ps3 unk_15;
    u32Ps3 unk_16;
    u32Ps3 unk_17;
    u32Ps3 unk_18;
    u32Ps3 unk_19;

  cdef enum:
    HkShapeRefPs3_Box,
    HkShapeRefPs3_Sphere,
    HkShapeRefPs3_Capsule,
    HkShapeRefPs3_Cylinder,
    HkShapeRefPs3_ConvexVertices,
    HkShapeRefPs3_BVTreeMesh,
    HkShapeRefPs3_Unknown,
  ctypedef uint8_t HkShapeRefPs3_Tag;

  cdef struct HkShapeRefPs3:
    HkShapeRefPs3_Tag tag;
    const BoxShapePs3 *box;
    const SphereShapePs3 *sphere;
    const CapsuleShapePs3 *capsule;
    const CylinderShapePs3 *cylinder;
    ConvexVerticesRefPs3 convex_vertices;
    BVTreeMeshRefPs3 bv_tree_mesh;
    const HkShapeInfoPs3 *unknown;

  #gen_ffi:export
  cdef struct FoliageInfoPs3:
    CrcPs3 key;
    u32Ps3 kind;
    i32Ps3 lb_w;
    i32Ps3 lb_h;
    i32Ps3 ub_w;
    i32Ps3 ub_h;
    f32Ps3 scale;
    u32Ps3 offset;
    CrcPs3 key_mesh;
    CrcPs3 key_mesh_lod1;
    CrcPs3 key_mesh_lod2;
    Vector4Ps3 color;
    f32Ps3 lod1a;
    f32Ps3 lod1b;
    f32Ps3 lod2a;
    f32Ps3 lod2b;
    f32Ps3 lod_max;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_FoliageValPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct FoliageRefPs3:
    const FoliageInfoPs3 *info;
    ref_slice_FoliageValPs3 vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_BlockValARefPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_Obj1RefPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct BlockValRefPs3:
    slice_BlockValARefPs3 vals_a;
    slice_Obj1RefPs3 vals_b;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_BlockValRefPs3:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct CrowdItemHeaderPs3:
    CrcPs3 key;
    CrcPs3 key_main;
    CrcPs3 key_right;
    CrcPs3 key_left;
    f32Ps3 unk_4;
    u32Ps3 animation_num;
    u32Ps3 instance_num;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_CrowdValPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef struct CrowdItemRefPs3:
    const CrowdItemHeaderPs3 *header;
    ref_slice_CrcPs3 animations;
    ref_slice_CrowdValPs3 instances;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_CrowdItemRefPs3:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct HkConstraintBoneRefPs3:
    string name;
    u32Ps3 start;
    u32Ps3 val;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct slice_HkConstraintBoneRefPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_f32Ps3:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    AnimVals1RefPs3_Type1,
    AnimVals1RefPs3_Type2,
    AnimVals1RefPs3_Type3,
    AnimVals1RefPs3_Type4,
  ctypedef uint8_t AnimVals1RefPs3_Tag;

  cdef struct AnimVals1RefPs3:
    AnimVals1RefPs3_Tag tag;
    ref_slice_u8 type1;
    ref_slice_u16Ps3 type2;
    ref_slice_u16Ps3 type3;
    ref_slice_u16Ps3 type4;

  cdef struct Obj1RefPs3:
    uint8_t flags;
    uint8_t s2;
    u16Ps3 s1;
    ref_slice_u8 data;
    ref_slice_f32Ps3 vals_a;
    AnimVals1RefPs3 vals;
    uintptr_t size;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationPolar32Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationThreeComp40Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationThreeComp48Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationThreeComp24Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationStraight16Ps3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_RotationUncompressedPs3:
    uint64_t align;
    uint8_t pad[8];

  cdef enum:
    RotationQuantizationRefPs3_Polar32,
    RotationQuantizationRefPs3_ThreeComp40,
    RotationQuantizationRefPs3_ThreeComp48,
    RotationQuantizationRefPs3_ThreeComp24,
    RotationQuantizationRefPs3_Straight16,
    RotationQuantizationRefPs3_Uncompressed,
  ctypedef uint8_t RotationQuantizationRefPs3_Tag;

  cdef struct RotationQuantizationRefPs3:
    RotationQuantizationRefPs3_Tag tag;
    ref_slice_RotationPolar32Ps3 polar32;
    ref_slice_RotationThreeComp40Ps3 three_comp40;
    ref_slice_RotationThreeComp48Ps3 three_comp48;
    ref_slice_RotationThreeComp24Ps3 three_comp24;
    ref_slice_RotationStraight16Ps3 straight16;
    ref_slice_RotationUncompressedPs3 uncompressed;

  cdef struct Obj2RefPs3:
    uint8_t flags;
    uint8_t s2;
    u16Ps3 s1;
    ref_slice_u8 data;
    RotationQuantizationRefPs3 vals;
    uintptr_t size;

  cdef struct BlockValARefPs3:
    Obj1RefPs3 a;
    Obj2RefPs3 b;
    Obj1RefPs3 c;

  #gen_ffi:export
  cdef struct BufferInfoPs3:
    u32Ps3 vbuff_info_offset;
    u32Ps3 vbuff_info_offset_2;
    u32Ps3 vbuff_info_offset_3;
    u32Ps3 unk_3;
    u32Ps3 unk_4;
    u32Ps3 unk_5;
    u32Ps3 unk_6;
    u32Ps3 unk_7;
    u32Ps3 unk_8;
    u32Ps3 unk_9;
    u32Ps3 unk_10;
    u32Ps3 unk_11;
    u32Ps3 unk_12;
    u32Ps3 unk_13;
    u32Ps3 unk_14;
    u32Ps3 unk_15;
    u32Ps3 unk_16;
    u32Ps3 ibuff_info_offset;
    u32Ps3 v_size;
    u32Ps3 vbuff_size;

  #gen_ffi:export
  cdef struct HkConstraintDataPs3:
    u32Ps3 kind;
    u32Ps3 unk_1;
    u32Ps3 unk_2;
    u32Ps3 unk_3;
    u32Ps3 unk_4;
    u32Ps3 unk_5;
    u32Ps3 unk_6;
    u32Ps3 unk_7;
    u32Ps3 unk_8;
    u32Ps3 unk_9;
    u32Ps3 unk_10;
    u32Ps3 unk_11;
    u32Ps3 unk_12;
    u32Ps3 unk_13;
    u32Ps3 unk_14;
    u32Ps3 unk_15;
    u32Ps3 unk_16;
    u32Ps3 unk_17;
    u32Ps3 unk_18;
    u32Ps3 unk_19;
    u32Ps3 unk_20;
    u32Ps3 unk_21;
    u32Ps3 unk_22;
    u32Ps3 unk_23;
    u32Ps3 unk_24;
    u32Ps3 unk_25;
    u32Ps3 unk_26;
    u32Ps3 unk_27;
    u32Ps3 unk_28;

  #gen_ffi:export
  cdef struct Key2Ps3:
    CrcPs3 key;
    u32Ps3 val;

  #gen_ffi:export
  cdef struct BlockValAPs3:
    f32Ps3 unk_0;
    f32Ps3 unk_1;
    f32Ps3 unk_2;
    f32Ps3 unk_3;
    f32Ps3 unk_4;
    f32Ps3 unk_5;
    f32Ps3 unk_6;
    f32Ps3 unk_7;
    f32Ps3 unk_8;
    u32Ps3 unk_9;
    u32Ps3 unk_10;
    u32Ps3 unk_11;

  #gen_ffi:export
  cdef struct BlockValBPs3:
    u16Ps3 unk_0;
    u16Ps3 unk_1;
    f32Ps3 unk_2;
    f32Ps3 unk_3;
    f32Ps3 unk_4;
    f32Ps3 unk_5;

  #gen_ffi:export
  cdef struct AnimationBlockInfoPs3:
    CrcPs3 key;
    u32Ps3 guid;
    CrcPs3 key_name;
    u32Ps3 offset;
    u32Ps3 size;
    u32Ps3 size_comp;
    u32Ps3 unk_6;
    u32Ps3 unk_7;
    u32Ps3 unk_8;

  #gen_ffi:export
  cdef struct AssetHandlePs3:
    CrcPs3 key;
    u32Ps3 offset;
    u32Ps3 size;
    u32Ps3 size_comp;
    u32Ps3 kind;

  ctypedef uint32_t U32BE;

  ctypedef U32BE U32Ps3;

  ctypedef I32BE I32Ps3;

  cdef struct BlockAValPs3:
    U32Ps3 unk_0;
    I32Ps3 gamemodemask;
    U32Ps3 key;
    U32Ps3 unk_3;
    U32Ps3 unk_4;
    U32Ps3 unk_5;
    U32Ps3 unk_6;

  #gen_ffi:export
  cdef struct GFXBlockInfoPs3:
    CrcPs3 key;
    u32Ps3 offset;
    u32Ps3 size;

  #gen_ffi:export
  cdef struct HkConstraintInfoPs3:
    u32Ps3 kind;
    u32Ps3 bone_parents_offset;
    u32Ps3 bone_parents_num;
    u32Ps3 bone_names_offset;
    u32Ps3 bone_names_num;
    u32Ps3 bone_transforms_offset;
    u32Ps3 bone_transforms_num;
    u32Ps3 unk_7;
    u32Ps3 unk_8;
    u32Ps3 unk_9;
    u32Ps3 bones_offset;
    u16Ps3 bones_num;
    u16Ps3 bone_order_num;
    u32Ps3 bone_order_offset;
    u32Ps3 unk_13;
    f32Ps3 unk_14;
    u32Ps3 vals2_num;
    u32Ps3 vals2_offset;
    u32Ps3 unk_17;

  #gen_ffi:export
  cdef struct Obj0Ps3:
    u32Ps3 unk_0;
    CrcPs3 key;

  #gen_ffi:export
  cdef struct ObjAPs3:
    CrcPs3 key;
    u32Ps3 unk_1;
    u32Ps3 size;
    u32Ps3 size_comp;
    u32Ps3 unk_4;
    u32Ps3 kind;

  #gen_ffi:export
  cdef struct PFieldInfoPs3:
    u32Ps3 link_guid;
    u32Ps3 gamemode_guid;
    u32Ps3 width;
    u32Ps3 height;
    u32Ps3 offset;

  #gen_ffi:export
  cdef struct StringKeysValPs3:
    CrcPs3 key;
    u32Ps3 offset;

  #gen_ffi:export
  cdef struct SubBlocksBlockHeaderPs3:
    CrcPs3 key;
    u32Ps3 offset;
    u32Ps3 size;

  #gen_ffi:export
  cdef struct Obj3Ps3:
    f32Ps3 t;
    CrcPs3 event;
    u32Ps3 dat_2;
    u32Ps3 dat_3;
    u32Ps3 dat_4;
    u32Ps3 dat_5;
    u32Ps3 dat_6;
    u32Ps3 dat_7;
    u32Ps3 dat_8;
    u32Ps3 dat_9;
    u32Ps3 dat_10;

  #gen_ffi:export
  cdef struct Obj5ValPs3:
    f32Ps3 unk_0;
    f32Ps3 unk_1;
    f32Ps3 unk_2;
    f32Ps3 unk_3;
    f32Ps3 unk_4;
    f32Ps3 unk_5;
    f32Ps3 unk_6;

  #gen_ffi:export
  cdef struct SSAValPs3:
    f32Ps3 t_start;
    f32Ps3 t_end;
    u32Ps3 unk_2;
    u32Ps3 unk_3;
    u32Ps3 off;

  #gen_ffi:export
  cdef struct TypeFieldPs3:
    CrcPs3 key;
    CrcPs3 kind;
    u32Ps3 offset;

  ctypedef i16_be i16Ps3;

  #gen_ffi:export
  cdef struct FoliageValPs3:
    u16Ps3 height;
    u16Ps3 var_mask;
    i16Ps3 slope_x;
    i16Ps3 slope_z;

  #gen_ffi:export
  cdef struct WeightPs3:
    u32Ps3 x;
    u8Ps3 a;
    u8Ps3 b;
    u8Ps3 c;
    u8Ps3 d;

  #gen_ffi:export
  cdef struct AtlasUVValPs3:
    CrcPs3 key;
    Vector4Ps3 vals;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_AtlasUVValPs3:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct SprayInstancePs3:
    CrcPs3 key;
    CrcPs3 tex1;
    CrcPs3 tex2;
    u32Ps3 unk_3;
    u32Ps3 width;
    u32Ps3 height;
    f32Ps3 unk_6;
    f32Ps3 unk_7;
    u32Ps3 size_w;
    u32Ps3 size_h;
    f32Ps3 scale_w;
    f32Ps3 scale_h;
    u32Ps3 delay;
    f32Ps3 stride_x;
    f32Ps3 stride_y;
    u32Ps3 unk_15;
    u32Ps3 unk_16;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SprayInstancePs3:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct SprayValPs3:
    Vector3Ps3 position;
    f32Ps3 scale;
    u16Ps3 instance;
    u16Ps3 rotation;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_SprayValPs3:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct TRSPs3:
    Vector4Ps3 translation;
    Vector4Ps3 rotation;
    Vector4Ps3 scale;

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_TRSPs3:
    uint64_t align;
    uint8_t pad[8];

  # This is a proxy struct for correct size and alignment only
  # don't construct this directly and use the provided methods to access
  cdef struct ref_slice_i16Ps3:
    uint64_t align;
    uint8_t pad[8];

  #gen_ffi:export
  cdef struct CrowdValPs3:
    Vector3Ps3 position;
    f32Ps3 rotation;
    f32Ps3 lod;

  #gen_ffi:export
  cdef struct RotationPolar32Ps3:
    u32Ps3 a;

  #gen_ffi:export
  cdef struct RotationStraight16Ps3:
    u8Ps3 a;
    u8Ps3 b;

  #gen_ffi:export
  cdef struct RotationThreeComp24Ps3:
    u8Ps3 a;
    u8Ps3 b;
    u8Ps3 c;

  #gen_ffi:export
  cdef struct RotationThreeComp40Ps3:
    u8Ps3 a;
    u8Ps3 b;
    u8Ps3 c;
    u8Ps3 d;
    u8Ps3 e;

  #gen_ffi:export
  cdef struct RotationThreeComp48Ps3:
    u16Ps3 a;
    u16Ps3 b;
    u16Ps3 c;

  #gen_ffi:export
  cdef struct RotationUncompressedPs3:
    f32Ps3 a;
    f32Ps3 b;
    f32Ps3 c;
    f32Ps3 d;

  #gen_ffi:export
  cdef struct HkConstraintRefPs3:
    const HkConstraintInfoPs3 *info;
    ref_slice_i16Ps3 bone_parents;
    slice_HkConstraintBoneRefPs3 bone_names;
    ref_slice_u32Ps3 name_offsets;
    ref_slice_TRSPs3 bone_transforms;
    ref_slice_u32Ps3 bones;
    ref_slice_Key2Ps3 bones_order;
    ref_slice_f32Ps3 vals2;

  #gen_ffi:export
  cdef struct ShapeExtraInfoPs3:
    u32Ps3 size;
    f32Ps3 scale;
    f32Ps3 a;
    f32Ps3 b;

  cdef struct ShapeExtraRefPs3:
    const ShapeExtraInfoPs3 *info;
    ref_slice_u32Ps3 offs;
    ref_slice_u8 data;

  cdef struct AtlasUVRefPs3:
    ref_slice_AtlasUVValPs3 vals;

  cdef struct BlocksRefPs3:
    ref_slice_u32Ps3 block_starts;
    ref_slice_u32Ps3 block_starts2;
    ref_slice_u32Ps3 obj_c3;
    ref_slice_u32Ps3 obj_c4;
    slice_BlockValRefPs3 blocks;

  #gen_ffi:export
  cdef struct CrowdHeaderPs3:
    u32Ps3 const0x65;
    u32Ps3 n;

  cdef struct CrowdRefPs3:
    const CrowdHeaderPs3 *header;
    ref_slice_u32Ps3 offs;
    slice_CrowdItemRefPs3 vals;

  ctypedef DataRefPs3 PFieldsRefPs3;

  cdef struct SprayRefPs3:
    ref_slice_SprayInstancePs3 instances;
    ref_slice_SprayValPs3 vals;

  LevelData *OwnedLevelData_get(OwnedLevelData *val);

  void OwnedLevelData_free(OwnedLevelData *val);

  OwnedLevelData *OwnedLevelData_read_data(const char *path);

  Version LevelData_version(const LevelData *src);

  LevelCompressedData *OwnedLevelCompressedData_get(OwnedLevelCompressedData *val);

  void OwnedLevelCompressedData_free(OwnedLevelCompressedData *val);

  OwnedLevelCompressedData *OwnedLevelCompressedData_new();

  LevelRefPc *OwnedLevelRefPc_get(OwnedLevelRefPc *val);

  void OwnedLevelRefPc_free(OwnedLevelRefPc *val);

  LevelRefXbox *OwnedLevelRefXbox_get(OwnedLevelRefXbox *val);

  void OwnedLevelRefXbox_free(OwnedLevelRefXbox *val);

  LevelRefPs3 *OwnedLevelRefPs3_get(OwnedLevelRefPs3 *val);

  void OwnedLevelRefPs3_free(OwnedLevelRefPs3 *val);

  OwnedLevelRefPc *OwnedLevelRefPc_from_data(const LevelData *src, LevelCompressedData *data);

  OwnedLevelRefXbox *OwnedLevelRefXbox_from_data(const LevelData *src, LevelCompressedData *data);

  OwnedLevelRefPs3 *OwnedLevelRefPs3_from_data(const LevelData *src, LevelCompressedData *data);

  OwnedLevelData *LevelRefPc_dump(const LevelRefPc *src, uint32_t compression);

  OwnedLevelData *LevelRefXbox_dump(const LevelRefXbox *src, uint32_t compression);

  OwnedLevelData *LevelRefPs3_dump(const LevelRefPs3 *src, uint32_t compression);

  InfoCounts *OwnedInfoCounts_get(OwnedInfoCounts *val);

  void OwnedInfoCounts_free(OwnedInfoCounts *val);

  OwnedInfoCounts *InfoCounts_new();

  uintptr_t InfoCounts_size_pc(const InfoCounts *counts);

  uintptr_t InfoCounts_size_xbox(const InfoCounts *counts);

  uintptr_t InfoCounts_size_ps3(const InfoCounts *counts);

  DumpInfosPc *OwnedDumpInfosPc_get(OwnedDumpInfosPc *val);

  void OwnedDumpInfosPc_free(OwnedDumpInfosPc *val);

  DumpInfosXbox *OwnedDumpInfosXbox_get(OwnedDumpInfosXbox *val);

  void OwnedDumpInfosXbox_free(OwnedDumpInfosXbox *val);

  DumpInfosPs3 *OwnedDumpInfosPs3_get(OwnedDumpInfosPs3 *val);

  void OwnedDumpInfosPs3_free(OwnedDumpInfosPs3 *val);

  OwnedDumpInfosPc *DumpInfosPc_from_data(DumpSlice *dst,
                                          const InfoCounts *counts,
                                          mut_slice_u32Pc *offsets);

  OwnedDumpInfosXbox *DumpInfosXbox_from_data(DumpSlice *dst,
                                              const InfoCounts *counts,
                                              mut_slice_u32Xbox *offsets);

  OwnedDumpInfosPs3 *DumpInfosPs3_from_data(DumpSlice *dst,
                                            const InfoCounts *counts,
                                            mut_slice_u32Ps3 *offsets);

  Vec______CompressedDataRef *OwnedVecCompressedData_get(OwnedVecCompressedData *val);

  void OwnedVecCompressedData_free(OwnedVecCompressedData *val);

  OwnedVecCompressedData *AnimationsRefPc_dump(const AnimationsRefPc *anims, DumpInfosPc *infos);

  OwnedVecCompressedData *AnimationsRefXbox_dump(const AnimationsRefXbox *anims,
                                                 DumpInfosXbox *infos);

  OwnedVecCompressedData *AnimationsRefPs3_dump(const AnimationsRefPs3 *anims, DumpInfosPs3 *infos);

  AlignedBuf *OwnedAlignedBuf_get(OwnedAlignedBuf *val);

  void OwnedAlignedBuf_free(OwnedAlignedBuf *val);

  const uint8_t *string_get(const string *string);

  uintptr_t string_len(const string *string);

  const CompressedDataRef *IndexMap_u32__CompressedDataRef_get(const IndexMap_u32__CompressedDataRef *map,
                                                               const uint32_t *key);

  uintptr_t IndexMap_u32__CompressedDataRef_len(const IndexMap_u32__CompressedDataRef *map);

  void IndexMap_u32__CompressedDataRef_keys(const IndexMap_u32__CompressedDataRef *map,
                                            mut_slice_u32 *keys);

  const ref_slice_u8 *IndexMap_u32__ref_slice_u8_get(const IndexMap_u32__ref_slice_u8 *map,
                                                     const uint32_t *key);

  uintptr_t IndexMap_u32__ref_slice_u8_len(const IndexMap_u32__ref_slice_u8 *map);

  void IndexMap_u32__ref_slice_u8_keys(const IndexMap_u32__ref_slice_u8 *map, mut_slice_u32 *keys);

  const CompressedDataRef *const *IndexMap_u32_______CompressedDataRef_get(const IndexMap_u32_______CompressedDataRef *map,
                                                                           const uint32_t *key);

  uintptr_t IndexMap_u32_______CompressedDataRef_len(const IndexMap_u32_______CompressedDataRef *map);

  void IndexMap_u32_______CompressedDataRef_keys(const IndexMap_u32_______CompressedDataRef *map,
                                                 mut_slice_u32 *keys);

  const VertexDataIndex *IndexMap_VertexUsage__VertexDataIndex_get(const IndexMap_VertexUsage__VertexDataIndex *map,
                                                                   const VertexUsage *key);

  uintptr_t IndexMap_VertexUsage__VertexDataIndex_len(const IndexMap_VertexUsage__VertexDataIndex *map);

  void IndexMap_VertexUsage__VertexDataIndex_keys(const IndexMap_VertexUsage__VertexDataIndex *map,
                                                  mut_slice_VertexUsage *keys);

  const uint8_t *ref_slice_u8_get(const ref_slice_u8 *slice, uintptr_t idx);

  uintptr_t ref_slice_u8_len(const ref_slice_u8 *slice);

  const uint32_t *ref_slice_u32_get(const ref_slice_u32 *slice, uintptr_t idx);

  uintptr_t ref_slice_u32_len(const ref_slice_u32 *slice);

  const VertexUsage *ref_slice_VertexUsage_get(const ref_slice_VertexUsage *slice, uintptr_t idx);

  uintptr_t ref_slice_VertexUsage_len(const ref_slice_VertexUsage *slice);

  uint8_t *mut_slice_u8_get(mut_slice_u8 *slice, uintptr_t idx);

  uintptr_t mut_slice_u8_len(const slice_u8 *slice);

  VertexUsage *mut_slice_VertexUsage_get(mut_slice_VertexUsage *slice, uintptr_t idx);

  uintptr_t mut_slice_VertexUsage_len(const slice_VertexUsage *slice);

  uint32_t *mut_slice_u32_get(mut_slice_u32 *slice, uintptr_t idx);

  uintptr_t mut_slice_u32_len(const slice_u32 *slice);

  const string *slice_string_get(const slice_string *slice, uintptr_t idx);

  uintptr_t slice_string_len(const slice_string *slice);

  const CompressedDataRef *slice_CompressedDataRef_get(const slice_CompressedDataRef *slice,
                                                       uintptr_t idx);

  uintptr_t slice_CompressedDataRef_len(const slice_CompressedDataRef *slice);

  const AlignmentHelper *slice_AlignmentHelper_get(const slice_AlignmentHelper *slice,
                                                   uintptr_t idx);

  uintptr_t slice_AlignmentHelper_len(const slice_AlignmentHelper *slice);

  const CompressedDataRef *const *slice______CompressedDataRef_get(const slice______CompressedDataRef *slice,
                                                                   uintptr_t idx);

  uintptr_t slice______CompressedDataRef_len(const slice______CompressedDataRef *slice);

  const CompressedDataRef *ref_slice_CompressedDataRef_get(const slice_CompressedDataRef *slice,
                                                           uintptr_t idx);

  uintptr_t ref_slice_CompressedDataRef_len(const slice_CompressedDataRef *slice);

  OwnedAlignedBuf *AlignedBuf_with_capacity(uintptr_t size);

  const IndexBufferRefPc *IndexMap_u32__IndexBufferRefPc_get(const IndexMap_u32__IndexBufferRefPc *map,
                                                             const uint32_t *key);

  uintptr_t IndexMap_u32__IndexBufferRefPc_len(const IndexMap_u32__IndexBufferRefPc *map);

  void IndexMap_u32__IndexBufferRefPc_keys(const IndexMap_u32__IndexBufferRefPc *map,
                                           mut_slice_u32 *keys);

  const VertexBufferRefPc *IndexMap_u32__VertexBufferRefPc_get(const IndexMap_u32__VertexBufferRefPc *map,
                                                               const uint32_t *key);

  uintptr_t IndexMap_u32__VertexBufferRefPc_len(const IndexMap_u32__VertexBufferRefPc *map);

  void IndexMap_u32__VertexBufferRefPc_keys(const IndexMap_u32__VertexBufferRefPc *map,
                                            mut_slice_u32 *keys);

  const MatRefPc *IndexMap_u32__MatRefPc_get(const IndexMap_u32__MatRefPc *map,
                                             const uint32_t *key);

  uintptr_t IndexMap_u32__MatRefPc_len(const IndexMap_u32__MatRefPc *map);

  void IndexMap_u32__MatRefPc_keys(const IndexMap_u32__MatRefPc *map, mut_slice_u32 *keys);

  const ModelRefPc *IndexMap_u32__ModelRefPc_get(const IndexMap_u32__ModelRefPc *map,
                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__ModelRefPc_len(const IndexMap_u32__ModelRefPc *map);

  void IndexMap_u32__ModelRefPc_keys(const IndexMap_u32__ModelRefPc *map, mut_slice_u32 *keys);

  const IBuffInfoPc *const *IndexMap_u32_______IBuffInfoPc_get(const IndexMap_u32_______IBuffInfoPc *map,
                                                               const uint32_t *key);

  uintptr_t IndexMap_u32_______IBuffInfoPc_len(const IndexMap_u32_______IBuffInfoPc *map);

  void IndexMap_u32_______IBuffInfoPc_keys(const IndexMap_u32_______IBuffInfoPc *map,
                                           mut_slice_u32 *keys);

  const VBuffInfoPc *const *IndexMap_u32_______VBuffInfoPc_get(const IndexMap_u32_______VBuffInfoPc *map,
                                                               const uint32_t *key);

  uintptr_t IndexMap_u32_______VBuffInfoPc_len(const IndexMap_u32_______VBuffInfoPc *map);

  void IndexMap_u32_______VBuffInfoPc_keys(const IndexMap_u32_______VBuffInfoPc *map,
                                           mut_slice_u32 *keys);

  const AnimationRefPc *IndexMap_u32__AnimationRefPc_get(const IndexMap_u32__AnimationRefPc *map,
                                                         const uint32_t *key);

  uintptr_t IndexMap_u32__AnimationRefPc_len(const IndexMap_u32__AnimationRefPc *map);

  void IndexMap_u32__AnimationRefPc_keys(const IndexMap_u32__AnimationRefPc *map,
                                         mut_slice_u32 *keys);

  const DataRefPc *IndexMap_u32__DataRefPc_get(const IndexMap_u32__DataRefPc *map,
                                               const uint32_t *key);

  uintptr_t IndexMap_u32__DataRefPc_len(const IndexMap_u32__DataRefPc *map);

  void IndexMap_u32__DataRefPc_keys(const IndexMap_u32__DataRefPc *map, mut_slice_u32 *keys);

  const EffectRefPc *IndexMap_u32__EffectRefPc_get(const IndexMap_u32__EffectRefPc *map,
                                                   const uint32_t *key);

  uintptr_t IndexMap_u32__EffectRefPc_len(const IndexMap_u32__EffectRefPc *map);

  void IndexMap_u32__EffectRefPc_keys(const IndexMap_u32__EffectRefPc *map, mut_slice_u32 *keys);

  const LangStringsRefPc *IndexMap_u32__LangStringsRefPc_get(const IndexMap_u32__LangStringsRefPc *map,
                                                             const uint32_t *key);

  uintptr_t IndexMap_u32__LangStringsRefPc_len(const IndexMap_u32__LangStringsRefPc *map);

  void IndexMap_u32__LangStringsRefPc_keys(const IndexMap_u32__LangStringsRefPc *map,
                                           mut_slice_u32 *keys);

  const LuaRefPc *IndexMap_u32__LuaRefPc_get(const IndexMap_u32__LuaRefPc *map,
                                             const uint32_t *key);

  uintptr_t IndexMap_u32__LuaRefPc_len(const IndexMap_u32__LuaRefPc *map);

  void IndexMap_u32__LuaRefPc_keys(const IndexMap_u32__LuaRefPc *map, mut_slice_u32 *keys);

  const ObjRefPc *IndexMap_u32__ObjRefPc_get(const IndexMap_u32__ObjRefPc *map,
                                             const uint32_t *key);

  uintptr_t IndexMap_u32__ObjRefPc_len(const IndexMap_u32__ObjRefPc *map);

  void IndexMap_u32__ObjRefPc_keys(const IndexMap_u32__ObjRefPc *map, mut_slice_u32 *keys);

  const RadiosityValsRefPc *IndexMap_u32__RadiosityValsRefPc_get(const IndexMap_u32__RadiosityValsRefPc *map,
                                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__RadiosityValsRefPc_len(const IndexMap_u32__RadiosityValsRefPc *map);

  void IndexMap_u32__RadiosityValsRefPc_keys(const IndexMap_u32__RadiosityValsRefPc *map,
                                             mut_slice_u32 *keys);

  const SSARefPc *IndexMap_u32__SSARefPc_get(const IndexMap_u32__SSARefPc *map,
                                             const uint32_t *key);

  uintptr_t IndexMap_u32__SSARefPc_len(const IndexMap_u32__SSARefPc *map);

  void IndexMap_u32__SSARefPc_keys(const IndexMap_u32__SSARefPc *map, mut_slice_u32 *keys);

  const TextureRefPc *IndexMap_u32__TextureRefPc_get(const IndexMap_u32__TextureRefPc *map,
                                                     const uint32_t *key);

  uintptr_t IndexMap_u32__TextureRefPc_len(const IndexMap_u32__TextureRefPc *map);

  void IndexMap_u32__TextureRefPc_keys(const IndexMap_u32__TextureRefPc *map, mut_slice_u32 *keys);

  const TypeRefPc *IndexMap_u32__TypeRefPc_get(const IndexMap_u32__TypeRefPc *map,
                                               const uint32_t *key);

  uintptr_t IndexMap_u32__TypeRefPc_len(const IndexMap_u32__TypeRefPc *map);

  void IndexMap_u32__TypeRefPc_keys(const IndexMap_u32__TypeRefPc *map, mut_slice_u32 *keys);

  const slice_FoliageRefPc *IndexMap_u32__slice_FoliageRefPc_get(const IndexMap_u32__slice_FoliageRefPc *map,
                                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__slice_FoliageRefPc_len(const IndexMap_u32__slice_FoliageRefPc *map);

  void IndexMap_u32__slice_FoliageRefPc_keys(const IndexMap_u32__slice_FoliageRefPc *map,
                                             mut_slice_u32 *keys);

  const BaseTypeRefPc *IndexMap_u32__BaseTypeRefPc_get(const IndexMap_u32__BaseTypeRefPc *map,
                                                       const uint32_t *key);

  uintptr_t IndexMap_u32__BaseTypeRefPc_len(const IndexMap_u32__BaseTypeRefPc *map);

  void IndexMap_u32__BaseTypeRefPc_keys(const IndexMap_u32__BaseTypeRefPc *map,
                                        mut_slice_u32 *keys);

  const ref_slice_u16Pc *IndexMap_u32__ref_slice_u16Pc_get(const IndexMap_u32__ref_slice_u16Pc *map,
                                                           const uint32_t *key);

  uintptr_t IndexMap_u32__ref_slice_u16Pc_len(const IndexMap_u32__ref_slice_u16Pc *map);

  void IndexMap_u32__ref_slice_u16Pc_keys(const IndexMap_u32__ref_slice_u16Pc *map,
                                          mut_slice_u32 *keys);

  const BlockRefPc *slice_BlockRefPc_get(const slice_BlockRefPc *slice, uintptr_t idx);

  uintptr_t slice_BlockRefPc_len(const slice_BlockRefPc *slice);

  const ShapeRefPc *slice_ShapeRefPc_get(const slice_ShapeRefPc *slice, uintptr_t idx);

  uintptr_t slice_ShapeRefPc_len(const slice_ShapeRefPc *slice);

  const HkShapeRefPc *slice_HkShapeRefPc_get(const slice_HkShapeRefPc *slice, uintptr_t idx);

  uintptr_t slice_HkShapeRefPc_len(const slice_HkShapeRefPc *slice);

  const FoliageRefPc *slice_FoliageRefPc_get(const slice_FoliageRefPc *slice, uintptr_t idx);

  uintptr_t slice_FoliageRefPc_len(const slice_FoliageRefPc *slice);

  const ref_slice_u16Pc *slice_ref_slice_u16Pc_get(const slice_ref_slice_u16Pc *slice,
                                                   uintptr_t idx);

  uintptr_t slice_ref_slice_u16Pc_len(const slice_ref_slice_u16Pc *slice);

  const BlockValRefPc *slice_BlockValRefPc_get(const slice_BlockValRefPc *slice, uintptr_t idx);

  uintptr_t slice_BlockValRefPc_len(const slice_BlockValRefPc *slice);

  const CrowdItemRefPc *slice_CrowdItemRefPc_get(const slice_CrowdItemRefPc *slice, uintptr_t idx);

  uintptr_t slice_CrowdItemRefPc_len(const slice_CrowdItemRefPc *slice);

  const HkConstraintBoneRefPc *slice_HkConstraintBoneRefPc_get(const slice_HkConstraintBoneRefPc *slice,
                                                               uintptr_t idx);

  uintptr_t slice_HkConstraintBoneRefPc_len(const slice_HkConstraintBoneRefPc *slice);

  const BlockValARefPc *slice_BlockValARefPc_get(const slice_BlockValARefPc *slice, uintptr_t idx);

  uintptr_t slice_BlockValARefPc_len(const slice_BlockValARefPc *slice);

  const Obj1RefPc *slice_Obj1RefPc_get(const slice_Obj1RefPc *slice, uintptr_t idx);

  uintptr_t slice_Obj1RefPc_len(const slice_Obj1RefPc *slice);

  const BoundingBoxPc *ref_slice_BoundingBoxPc_get(const ref_slice_BoundingBoxPc *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_BoundingBoxPc_len(const ref_slice_BoundingBoxPc *slice);

  const BufferInfoPc *ref_slice_BufferInfoPc_get(const ref_slice_BufferInfoPc *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_BufferInfoPc_len(const ref_slice_BufferInfoPc *slice);

  const CrcPc *ref_slice_CrcPc_get(const ref_slice_CrcPc *slice, uintptr_t idx);

  uintptr_t ref_slice_CrcPc_len(const ref_slice_CrcPc *slice);

  const HkConstraintDataPc *ref_slice_HkConstraintDataPc_get(const ref_slice_HkConstraintDataPc *slice,
                                                             uintptr_t idx);

  uintptr_t ref_slice_HkConstraintDataPc_len(const ref_slice_HkConstraintDataPc *slice);

  const Key2Pc *ref_slice_Key2Pc_get(const ref_slice_Key2Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_Key2Pc_len(const ref_slice_Key2Pc *slice);

  const Matrix4x4Pc *ref_slice_Matrix4x4Pc_get(const ref_slice_Matrix4x4Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_Matrix4x4Pc_len(const ref_slice_Matrix4x4Pc *slice);

  const Vector3Pc *ref_slice_Vector3Pc_get(const ref_slice_Vector3Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_Vector3Pc_len(const ref_slice_Vector3Pc *slice);

  const Vector4Pc *ref_slice_Vector4Pc_get(const ref_slice_Vector4Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_Vector4Pc_len(const ref_slice_Vector4Pc *slice);

  const i32Pc *ref_slice_i32Pc_get(const ref_slice_i32Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_i32Pc_len(const ref_slice_i32Pc *slice);

  const u16Pc *ref_slice_u16Pc_get(const ref_slice_u16Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_u16Pc_len(const ref_slice_u16Pc *slice);

  const u32Pc *ref_slice_u32Pc_get(const ref_slice_u32Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_u32Pc_len(const ref_slice_u32Pc *slice);

  const BlockValAPc *ref_slice_BlockValAPc_get(const ref_slice_BlockValAPc *slice, uintptr_t idx);

  uintptr_t ref_slice_BlockValAPc_len(const ref_slice_BlockValAPc *slice);

  const BlockValBPc *ref_slice_BlockValBPc_get(const ref_slice_BlockValBPc *slice, uintptr_t idx);

  uintptr_t ref_slice_BlockValBPc_len(const ref_slice_BlockValBPc *slice);

  const AnimationBlockInfoPc *ref_slice_AnimationBlockInfoPc_get(const ref_slice_AnimationBlockInfoPc *slice,
                                                                 uintptr_t idx);

  uintptr_t ref_slice_AnimationBlockInfoPc_len(const ref_slice_AnimationBlockInfoPc *slice);

  const AnimationInfoPc *ref_slice_AnimationInfoPc_get(const ref_slice_AnimationInfoPc *slice,
                                                       uintptr_t idx);

  uintptr_t ref_slice_AnimationInfoPc_len(const ref_slice_AnimationInfoPc *slice);

  const AssetHandlePc *ref_slice_AssetHandlePc_get(const ref_slice_AssetHandlePc *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_AssetHandlePc_len(const ref_slice_AssetHandlePc *slice);

  const BlockAValPc *ref_slice_BlockAValPc_get(const ref_slice_BlockAValPc *slice, uintptr_t idx);

  uintptr_t ref_slice_BlockAValPc_len(const ref_slice_BlockAValPc *slice);

  const EffectInfoPc *ref_slice_EffectInfoPc_get(const ref_slice_EffectInfoPc *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_EffectInfoPc_len(const ref_slice_EffectInfoPc *slice);

  const FoliageInfoPc *ref_slice_FoliageInfoPc_get(const ref_slice_FoliageInfoPc *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_FoliageInfoPc_len(const ref_slice_FoliageInfoPc *slice);

  const GFXBlockInfoPc *ref_slice_GFXBlockInfoPc_get(const ref_slice_GFXBlockInfoPc *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_GFXBlockInfoPc_len(const ref_slice_GFXBlockInfoPc *slice);

  const HkConstraintInfoPc *ref_slice_HkConstraintInfoPc_get(const ref_slice_HkConstraintInfoPc *slice,
                                                             uintptr_t idx);

  uintptr_t ref_slice_HkConstraintInfoPc_len(const ref_slice_HkConstraintInfoPc *slice);

  const HkShapeInfoPc *ref_slice_HkShapeInfoPc_get(const ref_slice_HkShapeInfoPc *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_HkShapeInfoPc_len(const ref_slice_HkShapeInfoPc *slice);

  const IBuffInfoPc *ref_slice_IBuffInfoPc_get(const ref_slice_IBuffInfoPc *slice, uintptr_t idx);

  uintptr_t ref_slice_IBuffInfoPc_len(const ref_slice_IBuffInfoPc *slice);

  const Mat1Pc *ref_slice_Mat1Pc_get(const ref_slice_Mat1Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat1Pc_len(const ref_slice_Mat1Pc *slice);

  const Mat2Pc *ref_slice_Mat2Pc_get(const ref_slice_Mat2Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat2Pc_len(const ref_slice_Mat2Pc *slice);

  const Mat3Pc *ref_slice_Mat3Pc_get(const ref_slice_Mat3Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat3Pc_len(const ref_slice_Mat3Pc *slice);

  const Mat4Pc *ref_slice_Mat4Pc_get(const ref_slice_Mat4Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat4Pc_len(const ref_slice_Mat4Pc *slice);

  const MatExtraPc *ref_slice_MatExtraPc_get(const ref_slice_MatExtraPc *slice, uintptr_t idx);

  uintptr_t ref_slice_MatExtraPc_len(const ref_slice_MatExtraPc *slice);

  const ModelInfoPc *ref_slice_ModelInfoPc_get(const ref_slice_ModelInfoPc *slice, uintptr_t idx);

  uintptr_t ref_slice_ModelInfoPc_len(const ref_slice_ModelInfoPc *slice);

  const Obj0Pc *ref_slice_Obj0Pc_get(const ref_slice_Obj0Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_Obj0Pc_len(const ref_slice_Obj0Pc *slice);

  const ObjAPc *ref_slice_ObjAPc_get(const ref_slice_ObjAPc *slice, uintptr_t idx);

  uintptr_t ref_slice_ObjAPc_len(const ref_slice_ObjAPc *slice);

  const PFieldInfoPc *ref_slice_PFieldInfoPc_get(const ref_slice_PFieldInfoPc *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_PFieldInfoPc_len(const ref_slice_PFieldInfoPc *slice);

  const RadiosityValsInfoPc *ref_slice_RadiosityValsInfoPc_get(const ref_slice_RadiosityValsInfoPc *slice,
                                                               uintptr_t idx);

  uintptr_t ref_slice_RadiosityValsInfoPc_len(const ref_slice_RadiosityValsInfoPc *slice);

  const ShapeInfoPc *ref_slice_ShapeInfoPc_get(const ref_slice_ShapeInfoPc *slice, uintptr_t idx);

  uintptr_t ref_slice_ShapeInfoPc_len(const ref_slice_ShapeInfoPc *slice);

  const StringKeysValPc *ref_slice_StringKeysValPc_get(const ref_slice_StringKeysValPc *slice,
                                                       uintptr_t idx);

  uintptr_t ref_slice_StringKeysValPc_len(const ref_slice_StringKeysValPc *slice);

  const SubBlocksBlockHeaderPc *ref_slice_SubBlocksBlockHeaderPc_get(const ref_slice_SubBlocksBlockHeaderPc *slice,
                                                                     uintptr_t idx);

  uintptr_t ref_slice_SubBlocksBlockHeaderPc_len(const ref_slice_SubBlocksBlockHeaderPc *slice);

  const TextureInfoPc *ref_slice_TextureInfoPc_get(const ref_slice_TextureInfoPc *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_TextureInfoPc_len(const ref_slice_TextureInfoPc *slice);

  const VBuffInfoPc *ref_slice_VBuffInfoPc_get(const ref_slice_VBuffInfoPc *slice, uintptr_t idx);

  uintptr_t ref_slice_VBuffInfoPc_len(const ref_slice_VBuffInfoPc *slice);

  const Obj3Pc *ref_slice_Obj3Pc_get(const ref_slice_Obj3Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_Obj3Pc_len(const ref_slice_Obj3Pc *slice);

  const Obj5ValPc *ref_slice_Obj5ValPc_get(const ref_slice_Obj5ValPc *slice, uintptr_t idx);

  uintptr_t ref_slice_Obj5ValPc_len(const ref_slice_Obj5ValPc *slice);

  const SSAValPc *ref_slice_SSAValPc_get(const ref_slice_SSAValPc *slice, uintptr_t idx);

  uintptr_t ref_slice_SSAValPc_len(const ref_slice_SSAValPc *slice);

  const TypeFieldPc *ref_slice_TypeFieldPc_get(const ref_slice_TypeFieldPc *slice, uintptr_t idx);

  uintptr_t ref_slice_TypeFieldPc_len(const ref_slice_TypeFieldPc *slice);

  const FoliageValPc *ref_slice_FoliageValPc_get(const ref_slice_FoliageValPc *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_FoliageValPc_len(const ref_slice_FoliageValPc *slice);

  const U32Pc *ref_slice_U32Pc_get(const ref_slice_U32Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_U32Pc_len(const ref_slice_U32Pc *slice);

  const WeightPc *ref_slice_WeightPc_get(const ref_slice_WeightPc *slice, uintptr_t idx);

  uintptr_t ref_slice_WeightPc_len(const ref_slice_WeightPc *slice);

  const AtlasUVValPc *ref_slice_AtlasUVValPc_get(const ref_slice_AtlasUVValPc *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_AtlasUVValPc_len(const ref_slice_AtlasUVValPc *slice);

  const SprayInstancePc *ref_slice_SprayInstancePc_get(const ref_slice_SprayInstancePc *slice,
                                                       uintptr_t idx);

  uintptr_t ref_slice_SprayInstancePc_len(const ref_slice_SprayInstancePc *slice);

  const SprayValPc *ref_slice_SprayValPc_get(const ref_slice_SprayValPc *slice, uintptr_t idx);

  uintptr_t ref_slice_SprayValPc_len(const ref_slice_SprayValPc *slice);

  const TRSPc *ref_slice_TRSPc_get(const ref_slice_TRSPc *slice, uintptr_t idx);

  uintptr_t ref_slice_TRSPc_len(const ref_slice_TRSPc *slice);

  const f32Pc *ref_slice_f32Pc_get(const ref_slice_f32Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_f32Pc_len(const ref_slice_f32Pc *slice);

  const i16Pc *ref_slice_i16Pc_get(const ref_slice_i16Pc *slice, uintptr_t idx);

  uintptr_t ref_slice_i16Pc_len(const ref_slice_i16Pc *slice);

  const CrowdValPc *ref_slice_CrowdValPc_get(const ref_slice_CrowdValPc *slice, uintptr_t idx);

  uintptr_t ref_slice_CrowdValPc_len(const ref_slice_CrowdValPc *slice);

  const RotationPolar32Pc *ref_slice_RotationPolar32Pc_get(const ref_slice_RotationPolar32Pc *slice,
                                                           uintptr_t idx);

  uintptr_t ref_slice_RotationPolar32Pc_len(const ref_slice_RotationPolar32Pc *slice);

  const RotationStraight16Pc *ref_slice_RotationStraight16Pc_get(const ref_slice_RotationStraight16Pc *slice,
                                                                 uintptr_t idx);

  uintptr_t ref_slice_RotationStraight16Pc_len(const ref_slice_RotationStraight16Pc *slice);

  const RotationThreeComp24Pc *ref_slice_RotationThreeComp24Pc_get(const ref_slice_RotationThreeComp24Pc *slice,
                                                                   uintptr_t idx);

  uintptr_t ref_slice_RotationThreeComp24Pc_len(const ref_slice_RotationThreeComp24Pc *slice);

  const RotationThreeComp40Pc *ref_slice_RotationThreeComp40Pc_get(const ref_slice_RotationThreeComp40Pc *slice,
                                                                   uintptr_t idx);

  uintptr_t ref_slice_RotationThreeComp40Pc_len(const ref_slice_RotationThreeComp40Pc *slice);

  const RotationThreeComp48Pc *ref_slice_RotationThreeComp48Pc_get(const ref_slice_RotationThreeComp48Pc *slice,
                                                                   uintptr_t idx);

  uintptr_t ref_slice_RotationThreeComp48Pc_len(const ref_slice_RotationThreeComp48Pc *slice);

  const RotationUncompressedPc *ref_slice_RotationUncompressedPc_get(const ref_slice_RotationUncompressedPc *slice,
                                                                     uintptr_t idx);

  uintptr_t ref_slice_RotationUncompressedPc_len(const ref_slice_RotationUncompressedPc *slice);

  u32Pc *mut_slice_u32Pc_get(mut_slice_u32Pc *slice, uintptr_t idx);

  uintptr_t mut_slice_u32Pc_len(const slice_u32Pc *slice);

  AnimationBlockInfoPc *mut_slice_AnimationBlockInfoPc_get(mut_slice_AnimationBlockInfoPc *slice,
                                                           uintptr_t idx);

  uintptr_t mut_slice_AnimationBlockInfoPc_len(const slice_AnimationBlockInfoPc *slice);

  AnimationInfoPc *mut_slice_AnimationInfoPc_get(mut_slice_AnimationInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_AnimationInfoPc_len(const slice_AnimationInfoPc *slice);

  BufferInfoPc *mut_slice_BufferInfoPc_get(mut_slice_BufferInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_BufferInfoPc_len(const slice_BufferInfoPc *slice);

  EffectInfoPc *mut_slice_EffectInfoPc_get(mut_slice_EffectInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_EffectInfoPc_len(const slice_EffectInfoPc *slice);

  FoliageInfoPc *mut_slice_FoliageInfoPc_get(mut_slice_FoliageInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_FoliageInfoPc_len(const slice_FoliageInfoPc *slice);

  GFXBlockInfoPc *mut_slice_GFXBlockInfoPc_get(mut_slice_GFXBlockInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_GFXBlockInfoPc_len(const slice_GFXBlockInfoPc *slice);

  HkConstraintDataPc *mut_slice_HkConstraintDataPc_get(mut_slice_HkConstraintDataPc *slice,
                                                       uintptr_t idx);

  uintptr_t mut_slice_HkConstraintDataPc_len(const slice_HkConstraintDataPc *slice);

  HkConstraintInfoPc *mut_slice_HkConstraintInfoPc_get(mut_slice_HkConstraintInfoPc *slice,
                                                       uintptr_t idx);

  uintptr_t mut_slice_HkConstraintInfoPc_len(const slice_HkConstraintInfoPc *slice);

  HkShapeInfoPc *mut_slice_HkShapeInfoPc_get(mut_slice_HkShapeInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_HkShapeInfoPc_len(const slice_HkShapeInfoPc *slice);

  IBuffInfoPc *mut_slice_IBuffInfoPc_get(mut_slice_IBuffInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_IBuffInfoPc_len(const slice_IBuffInfoPc *slice);

  Mat1Pc *mut_slice_Mat1Pc_get(mut_slice_Mat1Pc *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat1Pc_len(const slice_Mat1Pc *slice);

  Mat2Pc *mut_slice_Mat2Pc_get(mut_slice_Mat2Pc *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat2Pc_len(const slice_Mat2Pc *slice);

  Mat3Pc *mut_slice_Mat3Pc_get(mut_slice_Mat3Pc *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat3Pc_len(const slice_Mat3Pc *slice);

  Mat4Pc *mut_slice_Mat4Pc_get(mut_slice_Mat4Pc *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat4Pc_len(const slice_Mat4Pc *slice);

  MatExtraPc *mut_slice_MatExtraPc_get(mut_slice_MatExtraPc *slice, uintptr_t idx);

  uintptr_t mut_slice_MatExtraPc_len(const slice_MatExtraPc *slice);

  ModelInfoPc *mut_slice_ModelInfoPc_get(mut_slice_ModelInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_ModelInfoPc_len(const slice_ModelInfoPc *slice);

  Obj0Pc *mut_slice_Obj0Pc_get(mut_slice_Obj0Pc *slice, uintptr_t idx);

  uintptr_t mut_slice_Obj0Pc_len(const slice_Obj0Pc *slice);

  ObjAPc *mut_slice_ObjAPc_get(mut_slice_ObjAPc *slice, uintptr_t idx);

  uintptr_t mut_slice_ObjAPc_len(const slice_ObjAPc *slice);

  PFieldInfoPc *mut_slice_PFieldInfoPc_get(mut_slice_PFieldInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_PFieldInfoPc_len(const slice_PFieldInfoPc *slice);

  RadiosityValsInfoPc *mut_slice_RadiosityValsInfoPc_get(mut_slice_RadiosityValsInfoPc *slice,
                                                         uintptr_t idx);

  uintptr_t mut_slice_RadiosityValsInfoPc_len(const slice_RadiosityValsInfoPc *slice);

  ShapeInfoPc *mut_slice_ShapeInfoPc_get(mut_slice_ShapeInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_ShapeInfoPc_len(const slice_ShapeInfoPc *slice);

  TextureInfoPc *mut_slice_TextureInfoPc_get(mut_slice_TextureInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_TextureInfoPc_len(const slice_TextureInfoPc *slice);

  VBuffInfoPc *mut_slice_VBuffInfoPc_get(mut_slice_VBuffInfoPc *slice, uintptr_t idx);

  uintptr_t mut_slice_VBuffInfoPc_len(const slice_VBuffInfoPc *slice);

  const HkConstraintRefPc *Option_HkConstraintRefPc_get(const Option_HkConstraintRefPc *slice);

  const ShapeExtraRefPc *Option_ShapeExtraRefPc_get(const Option_ShapeExtraRefPc *slice);

  const AtlasUVRefPc *Option_AtlasUVRefPc_get(const Option_AtlasUVRefPc *slice);

  const BlocksRefPc *Option_BlocksRefPc_get(const Option_BlocksRefPc *slice);

  const CrowdRefPc *Option_CrowdRefPc_get(const Option_CrowdRefPc *slice);

  const PFieldsRefPc *Option_PFieldsRefPc_get(const Option_PFieldsRefPc *slice);

  const SprayRefPc *Option_SprayRefPc_get(const Option_SprayRefPc *slice);

  const IndexBufferRefXbox *IndexMap_u32__IndexBufferRefXbox_get(const IndexMap_u32__IndexBufferRefXbox *map,
                                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__IndexBufferRefXbox_len(const IndexMap_u32__IndexBufferRefXbox *map);

  void IndexMap_u32__IndexBufferRefXbox_keys(const IndexMap_u32__IndexBufferRefXbox *map,
                                             mut_slice_u32 *keys);

  const VertexBufferRefXbox *IndexMap_u32__VertexBufferRefXbox_get(const IndexMap_u32__VertexBufferRefXbox *map,
                                                                   const uint32_t *key);

  uintptr_t IndexMap_u32__VertexBufferRefXbox_len(const IndexMap_u32__VertexBufferRefXbox *map);

  void IndexMap_u32__VertexBufferRefXbox_keys(const IndexMap_u32__VertexBufferRefXbox *map,
                                              mut_slice_u32 *keys);

  const MatRefXbox *IndexMap_u32__MatRefXbox_get(const IndexMap_u32__MatRefXbox *map,
                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__MatRefXbox_len(const IndexMap_u32__MatRefXbox *map);

  void IndexMap_u32__MatRefXbox_keys(const IndexMap_u32__MatRefXbox *map, mut_slice_u32 *keys);

  const ModelRefXbox *IndexMap_u32__ModelRefXbox_get(const IndexMap_u32__ModelRefXbox *map,
                                                     const uint32_t *key);

  uintptr_t IndexMap_u32__ModelRefXbox_len(const IndexMap_u32__ModelRefXbox *map);

  void IndexMap_u32__ModelRefXbox_keys(const IndexMap_u32__ModelRefXbox *map, mut_slice_u32 *keys);

  const IBuffInfoXbox *const *IndexMap_u32_______IBuffInfoXbox_get(const IndexMap_u32_______IBuffInfoXbox *map,
                                                                   const uint32_t *key);

  uintptr_t IndexMap_u32_______IBuffInfoXbox_len(const IndexMap_u32_______IBuffInfoXbox *map);

  void IndexMap_u32_______IBuffInfoXbox_keys(const IndexMap_u32_______IBuffInfoXbox *map,
                                             mut_slice_u32 *keys);

  const VBuffInfoXbox *const *IndexMap_u32_______VBuffInfoXbox_get(const IndexMap_u32_______VBuffInfoXbox *map,
                                                                   const uint32_t *key);

  uintptr_t IndexMap_u32_______VBuffInfoXbox_len(const IndexMap_u32_______VBuffInfoXbox *map);

  void IndexMap_u32_______VBuffInfoXbox_keys(const IndexMap_u32_______VBuffInfoXbox *map,
                                             mut_slice_u32 *keys);

  const AnimationRefXbox *IndexMap_u32__AnimationRefXbox_get(const IndexMap_u32__AnimationRefXbox *map,
                                                             const uint32_t *key);

  uintptr_t IndexMap_u32__AnimationRefXbox_len(const IndexMap_u32__AnimationRefXbox *map);

  void IndexMap_u32__AnimationRefXbox_keys(const IndexMap_u32__AnimationRefXbox *map,
                                           mut_slice_u32 *keys);

  const DataRefXbox *IndexMap_u32__DataRefXbox_get(const IndexMap_u32__DataRefXbox *map,
                                                   const uint32_t *key);

  uintptr_t IndexMap_u32__DataRefXbox_len(const IndexMap_u32__DataRefXbox *map);

  void IndexMap_u32__DataRefXbox_keys(const IndexMap_u32__DataRefXbox *map, mut_slice_u32 *keys);

  const EffectRefXbox *IndexMap_u32__EffectRefXbox_get(const IndexMap_u32__EffectRefXbox *map,
                                                       const uint32_t *key);

  uintptr_t IndexMap_u32__EffectRefXbox_len(const IndexMap_u32__EffectRefXbox *map);

  void IndexMap_u32__EffectRefXbox_keys(const IndexMap_u32__EffectRefXbox *map,
                                        mut_slice_u32 *keys);

  const LangStringsRefXbox *IndexMap_u32__LangStringsRefXbox_get(const IndexMap_u32__LangStringsRefXbox *map,
                                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__LangStringsRefXbox_len(const IndexMap_u32__LangStringsRefXbox *map);

  void IndexMap_u32__LangStringsRefXbox_keys(const IndexMap_u32__LangStringsRefXbox *map,
                                             mut_slice_u32 *keys);

  const LuaRefXbox *IndexMap_u32__LuaRefXbox_get(const IndexMap_u32__LuaRefXbox *map,
                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__LuaRefXbox_len(const IndexMap_u32__LuaRefXbox *map);

  void IndexMap_u32__LuaRefXbox_keys(const IndexMap_u32__LuaRefXbox *map, mut_slice_u32 *keys);

  const ObjRefXbox *IndexMap_u32__ObjRefXbox_get(const IndexMap_u32__ObjRefXbox *map,
                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__ObjRefXbox_len(const IndexMap_u32__ObjRefXbox *map);

  void IndexMap_u32__ObjRefXbox_keys(const IndexMap_u32__ObjRefXbox *map, mut_slice_u32 *keys);

  const RadiosityValsRefXbox *IndexMap_u32__RadiosityValsRefXbox_get(const IndexMap_u32__RadiosityValsRefXbox *map,
                                                                     const uint32_t *key);

  uintptr_t IndexMap_u32__RadiosityValsRefXbox_len(const IndexMap_u32__RadiosityValsRefXbox *map);

  void IndexMap_u32__RadiosityValsRefXbox_keys(const IndexMap_u32__RadiosityValsRefXbox *map,
                                               mut_slice_u32 *keys);

  const SSARefXbox *IndexMap_u32__SSARefXbox_get(const IndexMap_u32__SSARefXbox *map,
                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__SSARefXbox_len(const IndexMap_u32__SSARefXbox *map);

  void IndexMap_u32__SSARefXbox_keys(const IndexMap_u32__SSARefXbox *map, mut_slice_u32 *keys);

  const TextureRefXbox *IndexMap_u32__TextureRefXbox_get(const IndexMap_u32__TextureRefXbox *map,
                                                         const uint32_t *key);

  uintptr_t IndexMap_u32__TextureRefXbox_len(const IndexMap_u32__TextureRefXbox *map);

  void IndexMap_u32__TextureRefXbox_keys(const IndexMap_u32__TextureRefXbox *map,
                                         mut_slice_u32 *keys);

  const TypeRefXbox *IndexMap_u32__TypeRefXbox_get(const IndexMap_u32__TypeRefXbox *map,
                                                   const uint32_t *key);

  uintptr_t IndexMap_u32__TypeRefXbox_len(const IndexMap_u32__TypeRefXbox *map);

  void IndexMap_u32__TypeRefXbox_keys(const IndexMap_u32__TypeRefXbox *map, mut_slice_u32 *keys);

  const slice_FoliageRefXbox *IndexMap_u32__slice_FoliageRefXbox_get(const IndexMap_u32__slice_FoliageRefXbox *map,
                                                                     const uint32_t *key);

  uintptr_t IndexMap_u32__slice_FoliageRefXbox_len(const IndexMap_u32__slice_FoliageRefXbox *map);

  void IndexMap_u32__slice_FoliageRefXbox_keys(const IndexMap_u32__slice_FoliageRefXbox *map,
                                               mut_slice_u32 *keys);

  const BaseTypeRefXbox *IndexMap_u32__BaseTypeRefXbox_get(const IndexMap_u32__BaseTypeRefXbox *map,
                                                           const uint32_t *key);

  uintptr_t IndexMap_u32__BaseTypeRefXbox_len(const IndexMap_u32__BaseTypeRefXbox *map);

  void IndexMap_u32__BaseTypeRefXbox_keys(const IndexMap_u32__BaseTypeRefXbox *map,
                                          mut_slice_u32 *keys);

  const ref_slice_u16Xbox *IndexMap_u32__ref_slice_u16Xbox_get(const IndexMap_u32__ref_slice_u16Xbox *map,
                                                               const uint32_t *key);

  uintptr_t IndexMap_u32__ref_slice_u16Xbox_len(const IndexMap_u32__ref_slice_u16Xbox *map);

  void IndexMap_u32__ref_slice_u16Xbox_keys(const IndexMap_u32__ref_slice_u16Xbox *map,
                                            mut_slice_u32 *keys);

  const BlockRefXbox *slice_BlockRefXbox_get(const slice_BlockRefXbox *slice, uintptr_t idx);

  uintptr_t slice_BlockRefXbox_len(const slice_BlockRefXbox *slice);

  const ShapeRefXbox *slice_ShapeRefXbox_get(const slice_ShapeRefXbox *slice, uintptr_t idx);

  uintptr_t slice_ShapeRefXbox_len(const slice_ShapeRefXbox *slice);

  const HkShapeRefXbox *slice_HkShapeRefXbox_get(const slice_HkShapeRefXbox *slice, uintptr_t idx);

  uintptr_t slice_HkShapeRefXbox_len(const slice_HkShapeRefXbox *slice);

  const FoliageRefXbox *slice_FoliageRefXbox_get(const slice_FoliageRefXbox *slice, uintptr_t idx);

  uintptr_t slice_FoliageRefXbox_len(const slice_FoliageRefXbox *slice);

  const ref_slice_u16Xbox *slice_ref_slice_u16Xbox_get(const slice_ref_slice_u16Xbox *slice,
                                                       uintptr_t idx);

  uintptr_t slice_ref_slice_u16Xbox_len(const slice_ref_slice_u16Xbox *slice);

  const BlockValRefXbox *slice_BlockValRefXbox_get(const slice_BlockValRefXbox *slice,
                                                   uintptr_t idx);

  uintptr_t slice_BlockValRefXbox_len(const slice_BlockValRefXbox *slice);

  const CrowdItemRefXbox *slice_CrowdItemRefXbox_get(const slice_CrowdItemRefXbox *slice,
                                                     uintptr_t idx);

  uintptr_t slice_CrowdItemRefXbox_len(const slice_CrowdItemRefXbox *slice);

  const HkConstraintBoneRefXbox *slice_HkConstraintBoneRefXbox_get(const slice_HkConstraintBoneRefXbox *slice,
                                                                   uintptr_t idx);

  uintptr_t slice_HkConstraintBoneRefXbox_len(const slice_HkConstraintBoneRefXbox *slice);

  const BlockValARefXbox *slice_BlockValARefXbox_get(const slice_BlockValARefXbox *slice,
                                                     uintptr_t idx);

  uintptr_t slice_BlockValARefXbox_len(const slice_BlockValARefXbox *slice);

  const Obj1RefXbox *slice_Obj1RefXbox_get(const slice_Obj1RefXbox *slice, uintptr_t idx);

  uintptr_t slice_Obj1RefXbox_len(const slice_Obj1RefXbox *slice);

  const BoundingBoxXbox *ref_slice_BoundingBoxXbox_get(const ref_slice_BoundingBoxXbox *slice,
                                                       uintptr_t idx);

  uintptr_t ref_slice_BoundingBoxXbox_len(const ref_slice_BoundingBoxXbox *slice);

  const BufferInfoXbox *ref_slice_BufferInfoXbox_get(const ref_slice_BufferInfoXbox *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_BufferInfoXbox_len(const ref_slice_BufferInfoXbox *slice);

  const CrcXbox *ref_slice_CrcXbox_get(const ref_slice_CrcXbox *slice, uintptr_t idx);

  uintptr_t ref_slice_CrcXbox_len(const ref_slice_CrcXbox *slice);

  const HkConstraintDataXbox *ref_slice_HkConstraintDataXbox_get(const ref_slice_HkConstraintDataXbox *slice,
                                                                 uintptr_t idx);

  uintptr_t ref_slice_HkConstraintDataXbox_len(const ref_slice_HkConstraintDataXbox *slice);

  const Key2Xbox *ref_slice_Key2Xbox_get(const ref_slice_Key2Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_Key2Xbox_len(const ref_slice_Key2Xbox *slice);

  const Matrix4x4Xbox *ref_slice_Matrix4x4Xbox_get(const ref_slice_Matrix4x4Xbox *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_Matrix4x4Xbox_len(const ref_slice_Matrix4x4Xbox *slice);

  const Vector3Xbox *ref_slice_Vector3Xbox_get(const ref_slice_Vector3Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_Vector3Xbox_len(const ref_slice_Vector3Xbox *slice);

  const Vector4Xbox *ref_slice_Vector4Xbox_get(const ref_slice_Vector4Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_Vector4Xbox_len(const ref_slice_Vector4Xbox *slice);

  const i32Xbox *ref_slice_i32Xbox_get(const ref_slice_i32Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_i32Xbox_len(const ref_slice_i32Xbox *slice);

  const u16Xbox *ref_slice_u16Xbox_get(const ref_slice_u16Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_u16Xbox_len(const ref_slice_u16Xbox *slice);

  const u32Xbox *ref_slice_u32Xbox_get(const ref_slice_u32Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_u32Xbox_len(const ref_slice_u32Xbox *slice);

  const BlockValAXbox *ref_slice_BlockValAXbox_get(const ref_slice_BlockValAXbox *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_BlockValAXbox_len(const ref_slice_BlockValAXbox *slice);

  const BlockValBXbox *ref_slice_BlockValBXbox_get(const ref_slice_BlockValBXbox *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_BlockValBXbox_len(const ref_slice_BlockValBXbox *slice);

  const AnimationBlockInfoXbox *ref_slice_AnimationBlockInfoXbox_get(const ref_slice_AnimationBlockInfoXbox *slice,
                                                                     uintptr_t idx);

  uintptr_t ref_slice_AnimationBlockInfoXbox_len(const ref_slice_AnimationBlockInfoXbox *slice);

  const AnimationInfoXbox *ref_slice_AnimationInfoXbox_get(const ref_slice_AnimationInfoXbox *slice,
                                                           uintptr_t idx);

  uintptr_t ref_slice_AnimationInfoXbox_len(const ref_slice_AnimationInfoXbox *slice);

  const AssetHandleXbox *ref_slice_AssetHandleXbox_get(const ref_slice_AssetHandleXbox *slice,
                                                       uintptr_t idx);

  uintptr_t ref_slice_AssetHandleXbox_len(const ref_slice_AssetHandleXbox *slice);

  const BlockAValXbox *ref_slice_BlockAValXbox_get(const ref_slice_BlockAValXbox *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_BlockAValXbox_len(const ref_slice_BlockAValXbox *slice);

  const EffectInfoXbox *ref_slice_EffectInfoXbox_get(const ref_slice_EffectInfoXbox *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_EffectInfoXbox_len(const ref_slice_EffectInfoXbox *slice);

  const FoliageInfoXbox *ref_slice_FoliageInfoXbox_get(const ref_slice_FoliageInfoXbox *slice,
                                                       uintptr_t idx);

  uintptr_t ref_slice_FoliageInfoXbox_len(const ref_slice_FoliageInfoXbox *slice);

  const GFXBlockInfoXbox *ref_slice_GFXBlockInfoXbox_get(const ref_slice_GFXBlockInfoXbox *slice,
                                                         uintptr_t idx);

  uintptr_t ref_slice_GFXBlockInfoXbox_len(const ref_slice_GFXBlockInfoXbox *slice);

  const HkConstraintInfoXbox *ref_slice_HkConstraintInfoXbox_get(const ref_slice_HkConstraintInfoXbox *slice,
                                                                 uintptr_t idx);

  uintptr_t ref_slice_HkConstraintInfoXbox_len(const ref_slice_HkConstraintInfoXbox *slice);

  const HkShapeInfoXbox *ref_slice_HkShapeInfoXbox_get(const ref_slice_HkShapeInfoXbox *slice,
                                                       uintptr_t idx);

  uintptr_t ref_slice_HkShapeInfoXbox_len(const ref_slice_HkShapeInfoXbox *slice);

  const IBuffInfoXbox *ref_slice_IBuffInfoXbox_get(const ref_slice_IBuffInfoXbox *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_IBuffInfoXbox_len(const ref_slice_IBuffInfoXbox *slice);

  const Mat1Xbox *ref_slice_Mat1Xbox_get(const ref_slice_Mat1Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat1Xbox_len(const ref_slice_Mat1Xbox *slice);

  const Mat2Xbox *ref_slice_Mat2Xbox_get(const ref_slice_Mat2Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat2Xbox_len(const ref_slice_Mat2Xbox *slice);

  const Mat3Xbox *ref_slice_Mat3Xbox_get(const ref_slice_Mat3Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat3Xbox_len(const ref_slice_Mat3Xbox *slice);

  const Mat4Xbox *ref_slice_Mat4Xbox_get(const ref_slice_Mat4Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat4Xbox_len(const ref_slice_Mat4Xbox *slice);

  const MatExtraXbox *ref_slice_MatExtraXbox_get(const ref_slice_MatExtraXbox *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_MatExtraXbox_len(const ref_slice_MatExtraXbox *slice);

  const ModelInfoXbox *ref_slice_ModelInfoXbox_get(const ref_slice_ModelInfoXbox *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_ModelInfoXbox_len(const ref_slice_ModelInfoXbox *slice);

  const Obj0Xbox *ref_slice_Obj0Xbox_get(const ref_slice_Obj0Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_Obj0Xbox_len(const ref_slice_Obj0Xbox *slice);

  const ObjAXbox *ref_slice_ObjAXbox_get(const ref_slice_ObjAXbox *slice, uintptr_t idx);

  uintptr_t ref_slice_ObjAXbox_len(const ref_slice_ObjAXbox *slice);

  const PFieldInfoXbox *ref_slice_PFieldInfoXbox_get(const ref_slice_PFieldInfoXbox *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_PFieldInfoXbox_len(const ref_slice_PFieldInfoXbox *slice);

  const RadiosityValsInfoXbox *ref_slice_RadiosityValsInfoXbox_get(const ref_slice_RadiosityValsInfoXbox *slice,
                                                                   uintptr_t idx);

  uintptr_t ref_slice_RadiosityValsInfoXbox_len(const ref_slice_RadiosityValsInfoXbox *slice);

  const ShapeInfoXbox *ref_slice_ShapeInfoXbox_get(const ref_slice_ShapeInfoXbox *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_ShapeInfoXbox_len(const ref_slice_ShapeInfoXbox *slice);

  const StringKeysValXbox *ref_slice_StringKeysValXbox_get(const ref_slice_StringKeysValXbox *slice,
                                                           uintptr_t idx);

  uintptr_t ref_slice_StringKeysValXbox_len(const ref_slice_StringKeysValXbox *slice);

  const SubBlocksBlockHeaderXbox *ref_slice_SubBlocksBlockHeaderXbox_get(const ref_slice_SubBlocksBlockHeaderXbox *slice,
                                                                         uintptr_t idx);

  uintptr_t ref_slice_SubBlocksBlockHeaderXbox_len(const ref_slice_SubBlocksBlockHeaderXbox *slice);

  const TextureInfoXbox *ref_slice_TextureInfoXbox_get(const ref_slice_TextureInfoXbox *slice,
                                                       uintptr_t idx);

  uintptr_t ref_slice_TextureInfoXbox_len(const ref_slice_TextureInfoXbox *slice);

  const VBuffInfoXbox *ref_slice_VBuffInfoXbox_get(const ref_slice_VBuffInfoXbox *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_VBuffInfoXbox_len(const ref_slice_VBuffInfoXbox *slice);

  const Obj3Xbox *ref_slice_Obj3Xbox_get(const ref_slice_Obj3Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_Obj3Xbox_len(const ref_slice_Obj3Xbox *slice);

  const Obj5ValXbox *ref_slice_Obj5ValXbox_get(const ref_slice_Obj5ValXbox *slice, uintptr_t idx);

  uintptr_t ref_slice_Obj5ValXbox_len(const ref_slice_Obj5ValXbox *slice);

  const SSAValXbox *ref_slice_SSAValXbox_get(const ref_slice_SSAValXbox *slice, uintptr_t idx);

  uintptr_t ref_slice_SSAValXbox_len(const ref_slice_SSAValXbox *slice);

  const TypeFieldXbox *ref_slice_TypeFieldXbox_get(const ref_slice_TypeFieldXbox *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_TypeFieldXbox_len(const ref_slice_TypeFieldXbox *slice);

  const FoliageValXbox *ref_slice_FoliageValXbox_get(const ref_slice_FoliageValXbox *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_FoliageValXbox_len(const ref_slice_FoliageValXbox *slice);

  const U32Xbox *ref_slice_U32Xbox_get(const ref_slice_U32Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_U32Xbox_len(const ref_slice_U32Xbox *slice);

  const WeightXbox *ref_slice_WeightXbox_get(const ref_slice_WeightXbox *slice, uintptr_t idx);

  uintptr_t ref_slice_WeightXbox_len(const ref_slice_WeightXbox *slice);

  const AtlasUVValXbox *ref_slice_AtlasUVValXbox_get(const ref_slice_AtlasUVValXbox *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_AtlasUVValXbox_len(const ref_slice_AtlasUVValXbox *slice);

  const SprayInstanceXbox *ref_slice_SprayInstanceXbox_get(const ref_slice_SprayInstanceXbox *slice,
                                                           uintptr_t idx);

  uintptr_t ref_slice_SprayInstanceXbox_len(const ref_slice_SprayInstanceXbox *slice);

  const SprayValXbox *ref_slice_SprayValXbox_get(const ref_slice_SprayValXbox *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_SprayValXbox_len(const ref_slice_SprayValXbox *slice);

  const TRSXbox *ref_slice_TRSXbox_get(const ref_slice_TRSXbox *slice, uintptr_t idx);

  uintptr_t ref_slice_TRSXbox_len(const ref_slice_TRSXbox *slice);

  const f32Xbox *ref_slice_f32Xbox_get(const ref_slice_f32Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_f32Xbox_len(const ref_slice_f32Xbox *slice);

  const i16Xbox *ref_slice_i16Xbox_get(const ref_slice_i16Xbox *slice, uintptr_t idx);

  uintptr_t ref_slice_i16Xbox_len(const ref_slice_i16Xbox *slice);

  const CrowdValXbox *ref_slice_CrowdValXbox_get(const ref_slice_CrowdValXbox *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_CrowdValXbox_len(const ref_slice_CrowdValXbox *slice);

  const RotationPolar32Xbox *ref_slice_RotationPolar32Xbox_get(const ref_slice_RotationPolar32Xbox *slice,
                                                               uintptr_t idx);

  uintptr_t ref_slice_RotationPolar32Xbox_len(const ref_slice_RotationPolar32Xbox *slice);

  const RotationStraight16Xbox *ref_slice_RotationStraight16Xbox_get(const ref_slice_RotationStraight16Xbox *slice,
                                                                     uintptr_t idx);

  uintptr_t ref_slice_RotationStraight16Xbox_len(const ref_slice_RotationStraight16Xbox *slice);

  const RotationThreeComp24Xbox *ref_slice_RotationThreeComp24Xbox_get(const ref_slice_RotationThreeComp24Xbox *slice,
                                                                       uintptr_t idx);

  uintptr_t ref_slice_RotationThreeComp24Xbox_len(const ref_slice_RotationThreeComp24Xbox *slice);

  const RotationThreeComp40Xbox *ref_slice_RotationThreeComp40Xbox_get(const ref_slice_RotationThreeComp40Xbox *slice,
                                                                       uintptr_t idx);

  uintptr_t ref_slice_RotationThreeComp40Xbox_len(const ref_slice_RotationThreeComp40Xbox *slice);

  const RotationThreeComp48Xbox *ref_slice_RotationThreeComp48Xbox_get(const ref_slice_RotationThreeComp48Xbox *slice,
                                                                       uintptr_t idx);

  uintptr_t ref_slice_RotationThreeComp48Xbox_len(const ref_slice_RotationThreeComp48Xbox *slice);

  const RotationUncompressedXbox *ref_slice_RotationUncompressedXbox_get(const ref_slice_RotationUncompressedXbox *slice,
                                                                         uintptr_t idx);

  uintptr_t ref_slice_RotationUncompressedXbox_len(const ref_slice_RotationUncompressedXbox *slice);

  u32Xbox *mut_slice_u32Xbox_get(mut_slice_u32Xbox *slice, uintptr_t idx);

  uintptr_t mut_slice_u32Xbox_len(const slice_u32Xbox *slice);

  AnimationBlockInfoXbox *mut_slice_AnimationBlockInfoXbox_get(mut_slice_AnimationBlockInfoXbox *slice,
                                                               uintptr_t idx);

  uintptr_t mut_slice_AnimationBlockInfoXbox_len(const slice_AnimationBlockInfoXbox *slice);

  AnimationInfoXbox *mut_slice_AnimationInfoXbox_get(mut_slice_AnimationInfoXbox *slice,
                                                     uintptr_t idx);

  uintptr_t mut_slice_AnimationInfoXbox_len(const slice_AnimationInfoXbox *slice);

  BufferInfoXbox *mut_slice_BufferInfoXbox_get(mut_slice_BufferInfoXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_BufferInfoXbox_len(const slice_BufferInfoXbox *slice);

  EffectInfoXbox *mut_slice_EffectInfoXbox_get(mut_slice_EffectInfoXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_EffectInfoXbox_len(const slice_EffectInfoXbox *slice);

  FoliageInfoXbox *mut_slice_FoliageInfoXbox_get(mut_slice_FoliageInfoXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_FoliageInfoXbox_len(const slice_FoliageInfoXbox *slice);

  GFXBlockInfoXbox *mut_slice_GFXBlockInfoXbox_get(mut_slice_GFXBlockInfoXbox *slice,
                                                   uintptr_t idx);

  uintptr_t mut_slice_GFXBlockInfoXbox_len(const slice_GFXBlockInfoXbox *slice);

  HkConstraintDataXbox *mut_slice_HkConstraintDataXbox_get(mut_slice_HkConstraintDataXbox *slice,
                                                           uintptr_t idx);

  uintptr_t mut_slice_HkConstraintDataXbox_len(const slice_HkConstraintDataXbox *slice);

  HkConstraintInfoXbox *mut_slice_HkConstraintInfoXbox_get(mut_slice_HkConstraintInfoXbox *slice,
                                                           uintptr_t idx);

  uintptr_t mut_slice_HkConstraintInfoXbox_len(const slice_HkConstraintInfoXbox *slice);

  HkShapeInfoXbox *mut_slice_HkShapeInfoXbox_get(mut_slice_HkShapeInfoXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_HkShapeInfoXbox_len(const slice_HkShapeInfoXbox *slice);

  IBuffInfoXbox *mut_slice_IBuffInfoXbox_get(mut_slice_IBuffInfoXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_IBuffInfoXbox_len(const slice_IBuffInfoXbox *slice);

  Mat1Xbox *mut_slice_Mat1Xbox_get(mut_slice_Mat1Xbox *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat1Xbox_len(const slice_Mat1Xbox *slice);

  Mat2Xbox *mut_slice_Mat2Xbox_get(mut_slice_Mat2Xbox *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat2Xbox_len(const slice_Mat2Xbox *slice);

  Mat3Xbox *mut_slice_Mat3Xbox_get(mut_slice_Mat3Xbox *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat3Xbox_len(const slice_Mat3Xbox *slice);

  Mat4Xbox *mut_slice_Mat4Xbox_get(mut_slice_Mat4Xbox *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat4Xbox_len(const slice_Mat4Xbox *slice);

  MatExtraXbox *mut_slice_MatExtraXbox_get(mut_slice_MatExtraXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_MatExtraXbox_len(const slice_MatExtraXbox *slice);

  ModelInfoXbox *mut_slice_ModelInfoXbox_get(mut_slice_ModelInfoXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_ModelInfoXbox_len(const slice_ModelInfoXbox *slice);

  Obj0Xbox *mut_slice_Obj0Xbox_get(mut_slice_Obj0Xbox *slice, uintptr_t idx);

  uintptr_t mut_slice_Obj0Xbox_len(const slice_Obj0Xbox *slice);

  ObjAXbox *mut_slice_ObjAXbox_get(mut_slice_ObjAXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_ObjAXbox_len(const slice_ObjAXbox *slice);

  PFieldInfoXbox *mut_slice_PFieldInfoXbox_get(mut_slice_PFieldInfoXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_PFieldInfoXbox_len(const slice_PFieldInfoXbox *slice);

  RadiosityValsInfoXbox *mut_slice_RadiosityValsInfoXbox_get(mut_slice_RadiosityValsInfoXbox *slice,
                                                             uintptr_t idx);

  uintptr_t mut_slice_RadiosityValsInfoXbox_len(const slice_RadiosityValsInfoXbox *slice);

  ShapeInfoXbox *mut_slice_ShapeInfoXbox_get(mut_slice_ShapeInfoXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_ShapeInfoXbox_len(const slice_ShapeInfoXbox *slice);

  TextureInfoXbox *mut_slice_TextureInfoXbox_get(mut_slice_TextureInfoXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_TextureInfoXbox_len(const slice_TextureInfoXbox *slice);

  VBuffInfoXbox *mut_slice_VBuffInfoXbox_get(mut_slice_VBuffInfoXbox *slice, uintptr_t idx);

  uintptr_t mut_slice_VBuffInfoXbox_len(const slice_VBuffInfoXbox *slice);

  const HkConstraintRefXbox *Option_HkConstraintRefXbox_get(const Option_HkConstraintRefXbox *slice);

  const ShapeExtraRefXbox *Option_ShapeExtraRefXbox_get(const Option_ShapeExtraRefXbox *slice);

  const AtlasUVRefXbox *Option_AtlasUVRefXbox_get(const Option_AtlasUVRefXbox *slice);

  const BlocksRefXbox *Option_BlocksRefXbox_get(const Option_BlocksRefXbox *slice);

  const CrowdRefXbox *Option_CrowdRefXbox_get(const Option_CrowdRefXbox *slice);

  const PFieldsRefXbox *Option_PFieldsRefXbox_get(const Option_PFieldsRefXbox *slice);

  const SprayRefXbox *Option_SprayRefXbox_get(const Option_SprayRefXbox *slice);

  const IndexBufferRefPs3 *IndexMap_u32__IndexBufferRefPs3_get(const IndexMap_u32__IndexBufferRefPs3 *map,
                                                               const uint32_t *key);

  uintptr_t IndexMap_u32__IndexBufferRefPs3_len(const IndexMap_u32__IndexBufferRefPs3 *map);

  void IndexMap_u32__IndexBufferRefPs3_keys(const IndexMap_u32__IndexBufferRefPs3 *map,
                                            mut_slice_u32 *keys);

  const VertexBufferRefPs3 *IndexMap_u32__VertexBufferRefPs3_get(const IndexMap_u32__VertexBufferRefPs3 *map,
                                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__VertexBufferRefPs3_len(const IndexMap_u32__VertexBufferRefPs3 *map);

  void IndexMap_u32__VertexBufferRefPs3_keys(const IndexMap_u32__VertexBufferRefPs3 *map,
                                             mut_slice_u32 *keys);

  const MatRefPs3 *IndexMap_u32__MatRefPs3_get(const IndexMap_u32__MatRefPs3 *map,
                                               const uint32_t *key);

  uintptr_t IndexMap_u32__MatRefPs3_len(const IndexMap_u32__MatRefPs3 *map);

  void IndexMap_u32__MatRefPs3_keys(const IndexMap_u32__MatRefPs3 *map, mut_slice_u32 *keys);

  const ModelRefPs3 *IndexMap_u32__ModelRefPs3_get(const IndexMap_u32__ModelRefPs3 *map,
                                                   const uint32_t *key);

  uintptr_t IndexMap_u32__ModelRefPs3_len(const IndexMap_u32__ModelRefPs3 *map);

  void IndexMap_u32__ModelRefPs3_keys(const IndexMap_u32__ModelRefPs3 *map, mut_slice_u32 *keys);

  const IBuffInfoPs3 *const *IndexMap_u32_______IBuffInfoPs3_get(const IndexMap_u32_______IBuffInfoPs3 *map,
                                                                 const uint32_t *key);

  uintptr_t IndexMap_u32_______IBuffInfoPs3_len(const IndexMap_u32_______IBuffInfoPs3 *map);

  void IndexMap_u32_______IBuffInfoPs3_keys(const IndexMap_u32_______IBuffInfoPs3 *map,
                                            mut_slice_u32 *keys);

  const VBuffInfoPs3 *const *IndexMap_u32_______VBuffInfoPs3_get(const IndexMap_u32_______VBuffInfoPs3 *map,
                                                                 const uint32_t *key);

  uintptr_t IndexMap_u32_______VBuffInfoPs3_len(const IndexMap_u32_______VBuffInfoPs3 *map);

  void IndexMap_u32_______VBuffInfoPs3_keys(const IndexMap_u32_______VBuffInfoPs3 *map,
                                            mut_slice_u32 *keys);

  const AnimationRefPs3 *IndexMap_u32__AnimationRefPs3_get(const IndexMap_u32__AnimationRefPs3 *map,
                                                           const uint32_t *key);

  uintptr_t IndexMap_u32__AnimationRefPs3_len(const IndexMap_u32__AnimationRefPs3 *map);

  void IndexMap_u32__AnimationRefPs3_keys(const IndexMap_u32__AnimationRefPs3 *map,
                                          mut_slice_u32 *keys);

  const DataRefPs3 *IndexMap_u32__DataRefPs3_get(const IndexMap_u32__DataRefPs3 *map,
                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__DataRefPs3_len(const IndexMap_u32__DataRefPs3 *map);

  void IndexMap_u32__DataRefPs3_keys(const IndexMap_u32__DataRefPs3 *map, mut_slice_u32 *keys);

  const EffectRefPs3 *IndexMap_u32__EffectRefPs3_get(const IndexMap_u32__EffectRefPs3 *map,
                                                     const uint32_t *key);

  uintptr_t IndexMap_u32__EffectRefPs3_len(const IndexMap_u32__EffectRefPs3 *map);

  void IndexMap_u32__EffectRefPs3_keys(const IndexMap_u32__EffectRefPs3 *map, mut_slice_u32 *keys);

  const LangStringsRefPs3 *IndexMap_u32__LangStringsRefPs3_get(const IndexMap_u32__LangStringsRefPs3 *map,
                                                               const uint32_t *key);

  uintptr_t IndexMap_u32__LangStringsRefPs3_len(const IndexMap_u32__LangStringsRefPs3 *map);

  void IndexMap_u32__LangStringsRefPs3_keys(const IndexMap_u32__LangStringsRefPs3 *map,
                                            mut_slice_u32 *keys);

  const LuaRefPs3 *IndexMap_u32__LuaRefPs3_get(const IndexMap_u32__LuaRefPs3 *map,
                                               const uint32_t *key);

  uintptr_t IndexMap_u32__LuaRefPs3_len(const IndexMap_u32__LuaRefPs3 *map);

  void IndexMap_u32__LuaRefPs3_keys(const IndexMap_u32__LuaRefPs3 *map, mut_slice_u32 *keys);

  const ObjRefPs3 *IndexMap_u32__ObjRefPs3_get(const IndexMap_u32__ObjRefPs3 *map,
                                               const uint32_t *key);

  uintptr_t IndexMap_u32__ObjRefPs3_len(const IndexMap_u32__ObjRefPs3 *map);

  void IndexMap_u32__ObjRefPs3_keys(const IndexMap_u32__ObjRefPs3 *map, mut_slice_u32 *keys);

  const RadiosityValsRefPs3 *IndexMap_u32__RadiosityValsRefPs3_get(const IndexMap_u32__RadiosityValsRefPs3 *map,
                                                                   const uint32_t *key);

  uintptr_t IndexMap_u32__RadiosityValsRefPs3_len(const IndexMap_u32__RadiosityValsRefPs3 *map);

  void IndexMap_u32__RadiosityValsRefPs3_keys(const IndexMap_u32__RadiosityValsRefPs3 *map,
                                              mut_slice_u32 *keys);

  const SSARefPs3 *IndexMap_u32__SSARefPs3_get(const IndexMap_u32__SSARefPs3 *map,
                                               const uint32_t *key);

  uintptr_t IndexMap_u32__SSARefPs3_len(const IndexMap_u32__SSARefPs3 *map);

  void IndexMap_u32__SSARefPs3_keys(const IndexMap_u32__SSARefPs3 *map, mut_slice_u32 *keys);

  const TextureRefPs3 *IndexMap_u32__TextureRefPs3_get(const IndexMap_u32__TextureRefPs3 *map,
                                                       const uint32_t *key);

  uintptr_t IndexMap_u32__TextureRefPs3_len(const IndexMap_u32__TextureRefPs3 *map);

  void IndexMap_u32__TextureRefPs3_keys(const IndexMap_u32__TextureRefPs3 *map,
                                        mut_slice_u32 *keys);

  const TypeRefPs3 *IndexMap_u32__TypeRefPs3_get(const IndexMap_u32__TypeRefPs3 *map,
                                                 const uint32_t *key);

  uintptr_t IndexMap_u32__TypeRefPs3_len(const IndexMap_u32__TypeRefPs3 *map);

  void IndexMap_u32__TypeRefPs3_keys(const IndexMap_u32__TypeRefPs3 *map, mut_slice_u32 *keys);

  const slice_FoliageRefPs3 *IndexMap_u32__slice_FoliageRefPs3_get(const IndexMap_u32__slice_FoliageRefPs3 *map,
                                                                   const uint32_t *key);

  uintptr_t IndexMap_u32__slice_FoliageRefPs3_len(const IndexMap_u32__slice_FoliageRefPs3 *map);

  void IndexMap_u32__slice_FoliageRefPs3_keys(const IndexMap_u32__slice_FoliageRefPs3 *map,
                                              mut_slice_u32 *keys);

  const BaseTypeRefPs3 *IndexMap_u32__BaseTypeRefPs3_get(const IndexMap_u32__BaseTypeRefPs3 *map,
                                                         const uint32_t *key);

  uintptr_t IndexMap_u32__BaseTypeRefPs3_len(const IndexMap_u32__BaseTypeRefPs3 *map);

  void IndexMap_u32__BaseTypeRefPs3_keys(const IndexMap_u32__BaseTypeRefPs3 *map,
                                         mut_slice_u32 *keys);

  const ref_slice_u16Ps3 *IndexMap_u32__ref_slice_u16Ps3_get(const IndexMap_u32__ref_slice_u16Ps3 *map,
                                                             const uint32_t *key);

  uintptr_t IndexMap_u32__ref_slice_u16Ps3_len(const IndexMap_u32__ref_slice_u16Ps3 *map);

  void IndexMap_u32__ref_slice_u16Ps3_keys(const IndexMap_u32__ref_slice_u16Ps3 *map,
                                           mut_slice_u32 *keys);

  const BlockRefPs3 *slice_BlockRefPs3_get(const slice_BlockRefPs3 *slice, uintptr_t idx);

  uintptr_t slice_BlockRefPs3_len(const slice_BlockRefPs3 *slice);

  const ShapeRefPs3 *slice_ShapeRefPs3_get(const slice_ShapeRefPs3 *slice, uintptr_t idx);

  uintptr_t slice_ShapeRefPs3_len(const slice_ShapeRefPs3 *slice);

  const HkShapeRefPs3 *slice_HkShapeRefPs3_get(const slice_HkShapeRefPs3 *slice, uintptr_t idx);

  uintptr_t slice_HkShapeRefPs3_len(const slice_HkShapeRefPs3 *slice);

  const FoliageRefPs3 *slice_FoliageRefPs3_get(const slice_FoliageRefPs3 *slice, uintptr_t idx);

  uintptr_t slice_FoliageRefPs3_len(const slice_FoliageRefPs3 *slice);

  const ref_slice_u16Ps3 *slice_ref_slice_u16Ps3_get(const slice_ref_slice_u16Ps3 *slice,
                                                     uintptr_t idx);

  uintptr_t slice_ref_slice_u16Ps3_len(const slice_ref_slice_u16Ps3 *slice);

  const BlockValRefPs3 *slice_BlockValRefPs3_get(const slice_BlockValRefPs3 *slice, uintptr_t idx);

  uintptr_t slice_BlockValRefPs3_len(const slice_BlockValRefPs3 *slice);

  const CrowdItemRefPs3 *slice_CrowdItemRefPs3_get(const slice_CrowdItemRefPs3 *slice,
                                                   uintptr_t idx);

  uintptr_t slice_CrowdItemRefPs3_len(const slice_CrowdItemRefPs3 *slice);

  const HkConstraintBoneRefPs3 *slice_HkConstraintBoneRefPs3_get(const slice_HkConstraintBoneRefPs3 *slice,
                                                                 uintptr_t idx);

  uintptr_t slice_HkConstraintBoneRefPs3_len(const slice_HkConstraintBoneRefPs3 *slice);

  const BlockValARefPs3 *slice_BlockValARefPs3_get(const slice_BlockValARefPs3 *slice,
                                                   uintptr_t idx);

  uintptr_t slice_BlockValARefPs3_len(const slice_BlockValARefPs3 *slice);

  const Obj1RefPs3 *slice_Obj1RefPs3_get(const slice_Obj1RefPs3 *slice, uintptr_t idx);

  uintptr_t slice_Obj1RefPs3_len(const slice_Obj1RefPs3 *slice);

  const BoundingBoxPs3 *ref_slice_BoundingBoxPs3_get(const ref_slice_BoundingBoxPs3 *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_BoundingBoxPs3_len(const ref_slice_BoundingBoxPs3 *slice);

  const BufferInfoPs3 *ref_slice_BufferInfoPs3_get(const ref_slice_BufferInfoPs3 *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_BufferInfoPs3_len(const ref_slice_BufferInfoPs3 *slice);

  const CrcPs3 *ref_slice_CrcPs3_get(const ref_slice_CrcPs3 *slice, uintptr_t idx);

  uintptr_t ref_slice_CrcPs3_len(const ref_slice_CrcPs3 *slice);

  const HkConstraintDataPs3 *ref_slice_HkConstraintDataPs3_get(const ref_slice_HkConstraintDataPs3 *slice,
                                                               uintptr_t idx);

  uintptr_t ref_slice_HkConstraintDataPs3_len(const ref_slice_HkConstraintDataPs3 *slice);

  const Key2Ps3 *ref_slice_Key2Ps3_get(const ref_slice_Key2Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_Key2Ps3_len(const ref_slice_Key2Ps3 *slice);

  const Matrix4x4Ps3 *ref_slice_Matrix4x4Ps3_get(const ref_slice_Matrix4x4Ps3 *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_Matrix4x4Ps3_len(const ref_slice_Matrix4x4Ps3 *slice);

  const Vector3Ps3 *ref_slice_Vector3Ps3_get(const ref_slice_Vector3Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_Vector3Ps3_len(const ref_slice_Vector3Ps3 *slice);

  const Vector4Ps3 *ref_slice_Vector4Ps3_get(const ref_slice_Vector4Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_Vector4Ps3_len(const ref_slice_Vector4Ps3 *slice);

  const i32Ps3 *ref_slice_i32Ps3_get(const ref_slice_i32Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_i32Ps3_len(const ref_slice_i32Ps3 *slice);

  const u16Ps3 *ref_slice_u16Ps3_get(const ref_slice_u16Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_u16Ps3_len(const ref_slice_u16Ps3 *slice);

  const u32Ps3 *ref_slice_u32Ps3_get(const ref_slice_u32Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_u32Ps3_len(const ref_slice_u32Ps3 *slice);

  const BlockValAPs3 *ref_slice_BlockValAPs3_get(const ref_slice_BlockValAPs3 *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_BlockValAPs3_len(const ref_slice_BlockValAPs3 *slice);

  const BlockValBPs3 *ref_slice_BlockValBPs3_get(const ref_slice_BlockValBPs3 *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_BlockValBPs3_len(const ref_slice_BlockValBPs3 *slice);

  const AnimationBlockInfoPs3 *ref_slice_AnimationBlockInfoPs3_get(const ref_slice_AnimationBlockInfoPs3 *slice,
                                                                   uintptr_t idx);

  uintptr_t ref_slice_AnimationBlockInfoPs3_len(const ref_slice_AnimationBlockInfoPs3 *slice);

  const AnimationInfoPs3 *ref_slice_AnimationInfoPs3_get(const ref_slice_AnimationInfoPs3 *slice,
                                                         uintptr_t idx);

  uintptr_t ref_slice_AnimationInfoPs3_len(const ref_slice_AnimationInfoPs3 *slice);

  const AssetHandlePs3 *ref_slice_AssetHandlePs3_get(const ref_slice_AssetHandlePs3 *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_AssetHandlePs3_len(const ref_slice_AssetHandlePs3 *slice);

  const BlockAValPs3 *ref_slice_BlockAValPs3_get(const ref_slice_BlockAValPs3 *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_BlockAValPs3_len(const ref_slice_BlockAValPs3 *slice);

  const EffectInfoPs3 *ref_slice_EffectInfoPs3_get(const ref_slice_EffectInfoPs3 *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_EffectInfoPs3_len(const ref_slice_EffectInfoPs3 *slice);

  const FoliageInfoPs3 *ref_slice_FoliageInfoPs3_get(const ref_slice_FoliageInfoPs3 *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_FoliageInfoPs3_len(const ref_slice_FoliageInfoPs3 *slice);

  const GFXBlockInfoPs3 *ref_slice_GFXBlockInfoPs3_get(const ref_slice_GFXBlockInfoPs3 *slice,
                                                       uintptr_t idx);

  uintptr_t ref_slice_GFXBlockInfoPs3_len(const ref_slice_GFXBlockInfoPs3 *slice);

  const HkConstraintInfoPs3 *ref_slice_HkConstraintInfoPs3_get(const ref_slice_HkConstraintInfoPs3 *slice,
                                                               uintptr_t idx);

  uintptr_t ref_slice_HkConstraintInfoPs3_len(const ref_slice_HkConstraintInfoPs3 *slice);

  const HkShapeInfoPs3 *ref_slice_HkShapeInfoPs3_get(const ref_slice_HkShapeInfoPs3 *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_HkShapeInfoPs3_len(const ref_slice_HkShapeInfoPs3 *slice);

  const IBuffInfoPs3 *ref_slice_IBuffInfoPs3_get(const ref_slice_IBuffInfoPs3 *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_IBuffInfoPs3_len(const ref_slice_IBuffInfoPs3 *slice);

  const Mat1Ps3 *ref_slice_Mat1Ps3_get(const ref_slice_Mat1Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat1Ps3_len(const ref_slice_Mat1Ps3 *slice);

  const Mat2Ps3 *ref_slice_Mat2Ps3_get(const ref_slice_Mat2Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat2Ps3_len(const ref_slice_Mat2Ps3 *slice);

  const Mat3Ps3 *ref_slice_Mat3Ps3_get(const ref_slice_Mat3Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat3Ps3_len(const ref_slice_Mat3Ps3 *slice);

  const Mat4Ps3 *ref_slice_Mat4Ps3_get(const ref_slice_Mat4Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_Mat4Ps3_len(const ref_slice_Mat4Ps3 *slice);

  const MatExtraPs3 *ref_slice_MatExtraPs3_get(const ref_slice_MatExtraPs3 *slice, uintptr_t idx);

  uintptr_t ref_slice_MatExtraPs3_len(const ref_slice_MatExtraPs3 *slice);

  const ModelInfoPs3 *ref_slice_ModelInfoPs3_get(const ref_slice_ModelInfoPs3 *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_ModelInfoPs3_len(const ref_slice_ModelInfoPs3 *slice);

  const Obj0Ps3 *ref_slice_Obj0Ps3_get(const ref_slice_Obj0Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_Obj0Ps3_len(const ref_slice_Obj0Ps3 *slice);

  const ObjAPs3 *ref_slice_ObjAPs3_get(const ref_slice_ObjAPs3 *slice, uintptr_t idx);

  uintptr_t ref_slice_ObjAPs3_len(const ref_slice_ObjAPs3 *slice);

  const PFieldInfoPs3 *ref_slice_PFieldInfoPs3_get(const ref_slice_PFieldInfoPs3 *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_PFieldInfoPs3_len(const ref_slice_PFieldInfoPs3 *slice);

  const RadiosityValsInfoPs3 *ref_slice_RadiosityValsInfoPs3_get(const ref_slice_RadiosityValsInfoPs3 *slice,
                                                                 uintptr_t idx);

  uintptr_t ref_slice_RadiosityValsInfoPs3_len(const ref_slice_RadiosityValsInfoPs3 *slice);

  const ShapeInfoPs3 *ref_slice_ShapeInfoPs3_get(const ref_slice_ShapeInfoPs3 *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_ShapeInfoPs3_len(const ref_slice_ShapeInfoPs3 *slice);

  const StringKeysValPs3 *ref_slice_StringKeysValPs3_get(const ref_slice_StringKeysValPs3 *slice,
                                                         uintptr_t idx);

  uintptr_t ref_slice_StringKeysValPs3_len(const ref_slice_StringKeysValPs3 *slice);

  const SubBlocksBlockHeaderPs3 *ref_slice_SubBlocksBlockHeaderPs3_get(const ref_slice_SubBlocksBlockHeaderPs3 *slice,
                                                                       uintptr_t idx);

  uintptr_t ref_slice_SubBlocksBlockHeaderPs3_len(const ref_slice_SubBlocksBlockHeaderPs3 *slice);

  const TextureInfoPs3 *ref_slice_TextureInfoPs3_get(const ref_slice_TextureInfoPs3 *slice,
                                                     uintptr_t idx);

  uintptr_t ref_slice_TextureInfoPs3_len(const ref_slice_TextureInfoPs3 *slice);

  const VBuffInfoPs3 *ref_slice_VBuffInfoPs3_get(const ref_slice_VBuffInfoPs3 *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_VBuffInfoPs3_len(const ref_slice_VBuffInfoPs3 *slice);

  const Obj3Ps3 *ref_slice_Obj3Ps3_get(const ref_slice_Obj3Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_Obj3Ps3_len(const ref_slice_Obj3Ps3 *slice);

  const Obj5ValPs3 *ref_slice_Obj5ValPs3_get(const ref_slice_Obj5ValPs3 *slice, uintptr_t idx);

  uintptr_t ref_slice_Obj5ValPs3_len(const ref_slice_Obj5ValPs3 *slice);

  const SSAValPs3 *ref_slice_SSAValPs3_get(const ref_slice_SSAValPs3 *slice, uintptr_t idx);

  uintptr_t ref_slice_SSAValPs3_len(const ref_slice_SSAValPs3 *slice);

  const TypeFieldPs3 *ref_slice_TypeFieldPs3_get(const ref_slice_TypeFieldPs3 *slice,
                                                 uintptr_t idx);

  uintptr_t ref_slice_TypeFieldPs3_len(const ref_slice_TypeFieldPs3 *slice);

  const FoliageValPs3 *ref_slice_FoliageValPs3_get(const ref_slice_FoliageValPs3 *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_FoliageValPs3_len(const ref_slice_FoliageValPs3 *slice);

  const U32Ps3 *ref_slice_U32Ps3_get(const ref_slice_U32Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_U32Ps3_len(const ref_slice_U32Ps3 *slice);

  const WeightPs3 *ref_slice_WeightPs3_get(const ref_slice_WeightPs3 *slice, uintptr_t idx);

  uintptr_t ref_slice_WeightPs3_len(const ref_slice_WeightPs3 *slice);

  const AtlasUVValPs3 *ref_slice_AtlasUVValPs3_get(const ref_slice_AtlasUVValPs3 *slice,
                                                   uintptr_t idx);

  uintptr_t ref_slice_AtlasUVValPs3_len(const ref_slice_AtlasUVValPs3 *slice);

  const SprayInstancePs3 *ref_slice_SprayInstancePs3_get(const ref_slice_SprayInstancePs3 *slice,
                                                         uintptr_t idx);

  uintptr_t ref_slice_SprayInstancePs3_len(const ref_slice_SprayInstancePs3 *slice);

  const SprayValPs3 *ref_slice_SprayValPs3_get(const ref_slice_SprayValPs3 *slice, uintptr_t idx);

  uintptr_t ref_slice_SprayValPs3_len(const ref_slice_SprayValPs3 *slice);

  const TRSPs3 *ref_slice_TRSPs3_get(const ref_slice_TRSPs3 *slice, uintptr_t idx);

  uintptr_t ref_slice_TRSPs3_len(const ref_slice_TRSPs3 *slice);

  const f32Ps3 *ref_slice_f32Ps3_get(const ref_slice_f32Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_f32Ps3_len(const ref_slice_f32Ps3 *slice);

  const i16Ps3 *ref_slice_i16Ps3_get(const ref_slice_i16Ps3 *slice, uintptr_t idx);

  uintptr_t ref_slice_i16Ps3_len(const ref_slice_i16Ps3 *slice);

  const CrowdValPs3 *ref_slice_CrowdValPs3_get(const ref_slice_CrowdValPs3 *slice, uintptr_t idx);

  uintptr_t ref_slice_CrowdValPs3_len(const ref_slice_CrowdValPs3 *slice);

  const RotationPolar32Ps3 *ref_slice_RotationPolar32Ps3_get(const ref_slice_RotationPolar32Ps3 *slice,
                                                             uintptr_t idx);

  uintptr_t ref_slice_RotationPolar32Ps3_len(const ref_slice_RotationPolar32Ps3 *slice);

  const RotationStraight16Ps3 *ref_slice_RotationStraight16Ps3_get(const ref_slice_RotationStraight16Ps3 *slice,
                                                                   uintptr_t idx);

  uintptr_t ref_slice_RotationStraight16Ps3_len(const ref_slice_RotationStraight16Ps3 *slice);

  const RotationThreeComp24Ps3 *ref_slice_RotationThreeComp24Ps3_get(const ref_slice_RotationThreeComp24Ps3 *slice,
                                                                     uintptr_t idx);

  uintptr_t ref_slice_RotationThreeComp24Ps3_len(const ref_slice_RotationThreeComp24Ps3 *slice);

  const RotationThreeComp40Ps3 *ref_slice_RotationThreeComp40Ps3_get(const ref_slice_RotationThreeComp40Ps3 *slice,
                                                                     uintptr_t idx);

  uintptr_t ref_slice_RotationThreeComp40Ps3_len(const ref_slice_RotationThreeComp40Ps3 *slice);

  const RotationThreeComp48Ps3 *ref_slice_RotationThreeComp48Ps3_get(const ref_slice_RotationThreeComp48Ps3 *slice,
                                                                     uintptr_t idx);

  uintptr_t ref_slice_RotationThreeComp48Ps3_len(const ref_slice_RotationThreeComp48Ps3 *slice);

  const RotationUncompressedPs3 *ref_slice_RotationUncompressedPs3_get(const ref_slice_RotationUncompressedPs3 *slice,
                                                                       uintptr_t idx);

  uintptr_t ref_slice_RotationUncompressedPs3_len(const ref_slice_RotationUncompressedPs3 *slice);

  u32Ps3 *mut_slice_u32Ps3_get(mut_slice_u32Ps3 *slice, uintptr_t idx);

  uintptr_t mut_slice_u32Ps3_len(const slice_u32Ps3 *slice);

  AnimationBlockInfoPs3 *mut_slice_AnimationBlockInfoPs3_get(mut_slice_AnimationBlockInfoPs3 *slice,
                                                             uintptr_t idx);

  uintptr_t mut_slice_AnimationBlockInfoPs3_len(const slice_AnimationBlockInfoPs3 *slice);

  AnimationInfoPs3 *mut_slice_AnimationInfoPs3_get(mut_slice_AnimationInfoPs3 *slice,
                                                   uintptr_t idx);

  uintptr_t mut_slice_AnimationInfoPs3_len(const slice_AnimationInfoPs3 *slice);

  BufferInfoPs3 *mut_slice_BufferInfoPs3_get(mut_slice_BufferInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_BufferInfoPs3_len(const slice_BufferInfoPs3 *slice);

  EffectInfoPs3 *mut_slice_EffectInfoPs3_get(mut_slice_EffectInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_EffectInfoPs3_len(const slice_EffectInfoPs3 *slice);

  FoliageInfoPs3 *mut_slice_FoliageInfoPs3_get(mut_slice_FoliageInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_FoliageInfoPs3_len(const slice_FoliageInfoPs3 *slice);

  GFXBlockInfoPs3 *mut_slice_GFXBlockInfoPs3_get(mut_slice_GFXBlockInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_GFXBlockInfoPs3_len(const slice_GFXBlockInfoPs3 *slice);

  HkConstraintDataPs3 *mut_slice_HkConstraintDataPs3_get(mut_slice_HkConstraintDataPs3 *slice,
                                                         uintptr_t idx);

  uintptr_t mut_slice_HkConstraintDataPs3_len(const slice_HkConstraintDataPs3 *slice);

  HkConstraintInfoPs3 *mut_slice_HkConstraintInfoPs3_get(mut_slice_HkConstraintInfoPs3 *slice,
                                                         uintptr_t idx);

  uintptr_t mut_slice_HkConstraintInfoPs3_len(const slice_HkConstraintInfoPs3 *slice);

  HkShapeInfoPs3 *mut_slice_HkShapeInfoPs3_get(mut_slice_HkShapeInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_HkShapeInfoPs3_len(const slice_HkShapeInfoPs3 *slice);

  IBuffInfoPs3 *mut_slice_IBuffInfoPs3_get(mut_slice_IBuffInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_IBuffInfoPs3_len(const slice_IBuffInfoPs3 *slice);

  Mat1Ps3 *mut_slice_Mat1Ps3_get(mut_slice_Mat1Ps3 *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat1Ps3_len(const slice_Mat1Ps3 *slice);

  Mat2Ps3 *mut_slice_Mat2Ps3_get(mut_slice_Mat2Ps3 *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat2Ps3_len(const slice_Mat2Ps3 *slice);

  Mat3Ps3 *mut_slice_Mat3Ps3_get(mut_slice_Mat3Ps3 *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat3Ps3_len(const slice_Mat3Ps3 *slice);

  Mat4Ps3 *mut_slice_Mat4Ps3_get(mut_slice_Mat4Ps3 *slice, uintptr_t idx);

  uintptr_t mut_slice_Mat4Ps3_len(const slice_Mat4Ps3 *slice);

  MatExtraPs3 *mut_slice_MatExtraPs3_get(mut_slice_MatExtraPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_MatExtraPs3_len(const slice_MatExtraPs3 *slice);

  ModelInfoPs3 *mut_slice_ModelInfoPs3_get(mut_slice_ModelInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_ModelInfoPs3_len(const slice_ModelInfoPs3 *slice);

  Obj0Ps3 *mut_slice_Obj0Ps3_get(mut_slice_Obj0Ps3 *slice, uintptr_t idx);

  uintptr_t mut_slice_Obj0Ps3_len(const slice_Obj0Ps3 *slice);

  ObjAPs3 *mut_slice_ObjAPs3_get(mut_slice_ObjAPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_ObjAPs3_len(const slice_ObjAPs3 *slice);

  PFieldInfoPs3 *mut_slice_PFieldInfoPs3_get(mut_slice_PFieldInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_PFieldInfoPs3_len(const slice_PFieldInfoPs3 *slice);

  RadiosityValsInfoPs3 *mut_slice_RadiosityValsInfoPs3_get(mut_slice_RadiosityValsInfoPs3 *slice,
                                                           uintptr_t idx);

  uintptr_t mut_slice_RadiosityValsInfoPs3_len(const slice_RadiosityValsInfoPs3 *slice);

  ShapeInfoPs3 *mut_slice_ShapeInfoPs3_get(mut_slice_ShapeInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_ShapeInfoPs3_len(const slice_ShapeInfoPs3 *slice);

  TextureInfoPs3 *mut_slice_TextureInfoPs3_get(mut_slice_TextureInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_TextureInfoPs3_len(const slice_TextureInfoPs3 *slice);

  VBuffInfoPs3 *mut_slice_VBuffInfoPs3_get(mut_slice_VBuffInfoPs3 *slice, uintptr_t idx);

  uintptr_t mut_slice_VBuffInfoPs3_len(const slice_VBuffInfoPs3 *slice);

  const HkConstraintRefPs3 *Option_HkConstraintRefPs3_get(const Option_HkConstraintRefPs3 *slice);

  const ShapeExtraRefPs3 *Option_ShapeExtraRefPs3_get(const Option_ShapeExtraRefPs3 *slice);

  const AtlasUVRefPs3 *Option_AtlasUVRefPs3_get(const Option_AtlasUVRefPs3 *slice);

  const BlocksRefPs3 *Option_BlocksRefPs3_get(const Option_BlocksRefPs3 *slice);

  const CrowdRefPs3 *Option_CrowdRefPs3_get(const Option_CrowdRefPs3 *slice);

  const PFieldsRefPs3 *Option_PFieldsRefPs3_get(const Option_PFieldsRefPs3 *slice);

  const SprayRefPs3 *Option_SprayRefPs3_get(const Option_SprayRefPs3 *slice);

  uint32_t utils_hash_string(const char *s);
