#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define LodInfo_UNKNOWN 1

#define LodInfo_STATIC 2

#define LodInfo_SKINNED 4

#define LodInfo_PHYSICS 8

#define LodInfo_BREAKABLE 16

#define LodInfo_LOD0 32

#define LodInfo_LOD1 64

#define LodInfo_LOD2 128

#define LodInfo_LOD3 256

enum Version
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Version_Pc = 0,
  Version_Xbox,
  Version_Ps3,
  Version_Err,
};
#if __STDC_VERSION__ >= 202311L
typedef enum Version Version;
#else
typedef uint8_t Version;
#endif // __STDC_VERSION__ >= 202311L

/**
 * Owned
 */
typedef struct OwnedAlignedBuf OwnedAlignedBuf;

/**
 * Owned
 */
typedef struct OwnedDumpInfosPc OwnedDumpInfosPc;

/**
 * Owned
 */
typedef struct OwnedDumpInfosPs3 OwnedDumpInfosPs3;

/**
 * Owned
 */
typedef struct OwnedDumpInfosXbox OwnedDumpInfosXbox;

/**
 * Owned
 */
typedef struct OwnedInfoCounts OwnedInfoCounts;

/**
 * Owned
 */
typedef struct OwnedLevelCompressedData OwnedLevelCompressedData;

/**
 * Owned
 */
typedef struct OwnedLevelData OwnedLevelData;

/**
 * Owned
 */
typedef struct OwnedLevelRefPc OwnedLevelRefPc;

/**
 * Owned
 */
typedef struct OwnedLevelRefPs3 OwnedLevelRefPs3;

/**
 * Owned
 */
typedef struct OwnedLevelRefXbox OwnedLevelRefXbox;

/**
 * Owned
 */
typedef struct OwnedVecCompressedData OwnedVecCompressedData;

typedef struct slice_AnimationBlockInfoPc slice_AnimationBlockInfoPc;

typedef struct slice_AnimationBlockInfoPs3 slice_AnimationBlockInfoPs3;

typedef struct slice_AnimationBlockInfoXbox slice_AnimationBlockInfoXbox;

typedef struct slice_AnimationInfoPc slice_AnimationInfoPc;

typedef struct slice_AnimationInfoPs3 slice_AnimationInfoPs3;

typedef struct slice_AnimationInfoXbox slice_AnimationInfoXbox;

typedef struct slice_BufferInfoPc slice_BufferInfoPc;

typedef struct slice_BufferInfoPs3 slice_BufferInfoPs3;

typedef struct slice_BufferInfoXbox slice_BufferInfoXbox;

typedef struct slice_EffectInfoPc slice_EffectInfoPc;

typedef struct slice_EffectInfoPs3 slice_EffectInfoPs3;

typedef struct slice_EffectInfoXbox slice_EffectInfoXbox;

typedef struct slice_FoliageInfoPc slice_FoliageInfoPc;

typedef struct slice_FoliageInfoPs3 slice_FoliageInfoPs3;

typedef struct slice_FoliageInfoXbox slice_FoliageInfoXbox;

typedef struct slice_GFXBlockInfoPc slice_GFXBlockInfoPc;

typedef struct slice_GFXBlockInfoPs3 slice_GFXBlockInfoPs3;

typedef struct slice_GFXBlockInfoXbox slice_GFXBlockInfoXbox;

typedef struct slice_HkConstraintDataPc slice_HkConstraintDataPc;

typedef struct slice_HkConstraintDataPs3 slice_HkConstraintDataPs3;

typedef struct slice_HkConstraintDataXbox slice_HkConstraintDataXbox;

typedef struct slice_HkConstraintInfoPc slice_HkConstraintInfoPc;

typedef struct slice_HkConstraintInfoPs3 slice_HkConstraintInfoPs3;

typedef struct slice_HkConstraintInfoXbox slice_HkConstraintInfoXbox;

typedef struct slice_HkShapeInfoPc slice_HkShapeInfoPc;

typedef struct slice_HkShapeInfoPs3 slice_HkShapeInfoPs3;

typedef struct slice_HkShapeInfoXbox slice_HkShapeInfoXbox;

typedef struct slice_IBuffInfoPc slice_IBuffInfoPc;

typedef struct slice_IBuffInfoPs3 slice_IBuffInfoPs3;

typedef struct slice_IBuffInfoXbox slice_IBuffInfoXbox;

typedef struct slice_Mat1Pc slice_Mat1Pc;

typedef struct slice_Mat1Ps3 slice_Mat1Ps3;

typedef struct slice_Mat1Xbox slice_Mat1Xbox;

typedef struct slice_Mat2Pc slice_Mat2Pc;

typedef struct slice_Mat2Ps3 slice_Mat2Ps3;

typedef struct slice_Mat2Xbox slice_Mat2Xbox;

typedef struct slice_Mat3Pc slice_Mat3Pc;

typedef struct slice_Mat3Ps3 slice_Mat3Ps3;

typedef struct slice_Mat3Xbox slice_Mat3Xbox;

typedef struct slice_Mat4Pc slice_Mat4Pc;

typedef struct slice_Mat4Ps3 slice_Mat4Ps3;

typedef struct slice_Mat4Xbox slice_Mat4Xbox;

typedef struct slice_MatExtraPc slice_MatExtraPc;

typedef struct slice_MatExtraPs3 slice_MatExtraPs3;

typedef struct slice_MatExtraXbox slice_MatExtraXbox;

typedef struct slice_ModelInfoPc slice_ModelInfoPc;

typedef struct slice_ModelInfoPs3 slice_ModelInfoPs3;

typedef struct slice_ModelInfoXbox slice_ModelInfoXbox;

typedef struct slice_Obj0Pc slice_Obj0Pc;

typedef struct slice_Obj0Ps3 slice_Obj0Ps3;

typedef struct slice_Obj0Xbox slice_Obj0Xbox;

typedef struct slice_ObjAPc slice_ObjAPc;

typedef struct slice_ObjAPs3 slice_ObjAPs3;

typedef struct slice_ObjAXbox slice_ObjAXbox;

typedef struct slice_PFieldInfoPc slice_PFieldInfoPc;

typedef struct slice_PFieldInfoPs3 slice_PFieldInfoPs3;

typedef struct slice_PFieldInfoXbox slice_PFieldInfoXbox;

typedef struct slice_RadiosityValsInfoPc slice_RadiosityValsInfoPc;

typedef struct slice_RadiosityValsInfoPs3 slice_RadiosityValsInfoPs3;

typedef struct slice_RadiosityValsInfoXbox slice_RadiosityValsInfoXbox;

typedef struct slice_ShapeInfoPc slice_ShapeInfoPc;

typedef struct slice_ShapeInfoPs3 slice_ShapeInfoPs3;

typedef struct slice_ShapeInfoXbox slice_ShapeInfoXbox;

typedef struct slice_TextureInfoPc slice_TextureInfoPc;

typedef struct slice_TextureInfoPs3 slice_TextureInfoPs3;

typedef struct slice_TextureInfoXbox slice_TextureInfoXbox;

typedef struct slice_VBuffInfoPc slice_VBuffInfoPc;

typedef struct slice_VBuffInfoPs3 slice_VBuffInfoPs3;

typedef struct slice_VBuffInfoXbox slice_VBuffInfoXbox;

typedef struct slice_VertexUsage slice_VertexUsage;

typedef struct slice_u32 slice_u32;

typedef struct slice_u32Pc slice_u32Pc;

typedef struct slice_u32Ps3 slice_u32Ps3;

typedef struct slice_u32Xbox slice_u32Xbox;

typedef struct slice_u8 slice_u8;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_AlignmentHelper {
  uint64_t align;
  uint8_t pad[8];
} slice_AlignmentHelper;

typedef struct AlignedBuf {
  struct slice_AlignmentHelper data;
  uintptr_t size;
} AlignedBuf;

typedef struct LevelData {
  struct AlignedBuf pak;
  struct AlignedBuf bin;
} LevelData;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_u8 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_u8;

typedef struct CompressedDataRef {
  struct ref_slice_u8 data;
  struct AlignedBuf data_decomp;
} CompressedDataRef;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_CompressedDataRef {
  uint64_t align;
  uint8_t pad[8];
} slice_CompressedDataRef;

typedef struct PakCompressedData {
  struct CompressedDataRef block1;
  struct CompressedDataRef block2;
  struct slice_CompressedDataRef animations;
} PakCompressedData;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__CompressedDataRef {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__CompressedDataRef;

typedef struct BinCompressedData {
  struct IndexMap_u32__CompressedDataRef model_data;
  struct IndexMap_u32__CompressedDataRef texture_data;
} BinCompressedData;

typedef struct LevelCompressedData {
  struct PakCompressedData pak;
  struct BinCompressedData bin;
} LevelCompressedData;

typedef uint32_t u32_le;

typedef u32_le u32Pc;

/**
 *gen_ffi:export
 */
typedef struct PakHeaderPc {
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
} PakHeaderPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_string {
  uint64_t align;
  uint8_t pad[8];
} slice_string;

typedef struct slice_string StringsRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_ObjAPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_ObjAPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Obj0Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Obj0Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_ModelInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_ModelInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BufferInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BufferInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat1Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat1Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat2Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat2Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat3Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat3Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat4Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat4Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_MatExtraPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_MatExtraPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_ShapeInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_ShapeInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_HkShapeInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_HkShapeInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_HkConstraintDataPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_HkConstraintDataPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_VBuffInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_VBuffInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_IBuffInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_IBuffInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_TextureInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_TextureInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AnimationInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AnimationInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_HkConstraintInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_HkConstraintInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_EffectInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_EffectInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_PFieldInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_PFieldInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_GFXBlockInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_GFXBlockInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AnimationBlockInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AnimationBlockInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_FoliageInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_FoliageInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RadiosityValsInfoPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RadiosityValsInfoPc;

typedef struct InfosRefPc {
  struct ref_slice_ObjAPc objas;
  struct ref_slice_Obj0Pc obj0s;
  struct ref_slice_ModelInfoPc models;
  struct ref_slice_BufferInfoPc buffers;
  struct ref_slice_Mat1Pc mat1s;
  struct ref_slice_Mat2Pc mat2s;
  struct ref_slice_Mat3Pc mat3s;
  struct ref_slice_Mat4Pc mat4s;
  struct ref_slice_MatExtraPc mat_extras;
  struct ref_slice_ShapeInfoPc shapes;
  struct ref_slice_HkShapeInfoPc hk_shapes;
  struct ref_slice_HkConstraintDataPc hk_constraint_datas;
  struct ref_slice_VBuffInfoPc vbuffs;
  struct ref_slice_IBuffInfoPc ibuffs;
  struct ref_slice_TextureInfoPc textures;
  struct ref_slice_AnimationInfoPc animations;
  struct ref_slice_HkConstraintInfoPc hk_constraints;
  struct ref_slice_EffectInfoPc effects;
  struct ref_slice_PFieldInfoPc pfields;
  struct ref_slice_GFXBlockInfoPc gfxs;
  struct ref_slice_AnimationBlockInfoPc animation_blocks;
  struct ref_slice_FoliageInfoPc foliages;
  struct ref_slice_RadiosityValsInfoPc radiosity_vals;
} InfosRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__TextureRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__TextureRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__ModelRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__ModelRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__EffectRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__EffectRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__slice_FoliageRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__slice_FoliageRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__ref_slice_u8 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__ref_slice_u8;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__RadiosityValsRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__RadiosityValsRefPc;

typedef struct RadiosityRefPc {
  const struct CompressedDataRef *data;
  struct IndexMap_u32__RadiosityValsRefPc vals;
  uint32_t usage;
} RadiosityRefPc;

typedef struct ObjsRefPc {
  struct IndexMap_u32__TextureRefPc textures;
  struct IndexMap_u32__ModelRefPc models;
  struct IndexMap_u32__EffectRefPc effects;
  struct IndexMap_u32__slice_FoliageRefPc foliages;
  struct IndexMap_u32__ref_slice_u8 gfxs;
  struct RadiosityRefPc radiosity;
} ObjsRefPc;

/**
 *gen_ffi:export
 */
typedef struct SubBlocksHeaderPc {
  u32Pc z0;
  u32Pc block_num;
  u32Pc z2;
  u32Pc z3;
} SubBlocksHeaderPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SubBlocksBlockHeaderPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SubBlocksBlockHeaderPc;

typedef struct SubBlocksInfoRefPc {
  const struct SubBlocksHeaderPc *header;
  struct ref_slice_SubBlocksBlockHeaderPc block_headers;
} SubBlocksInfoRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__DataRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__DataRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__LuaRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__LuaRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__SSARefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__SSARefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_AtlasUVRefPc {
  uint64_t align;
  uint8_t pad[8];
} Option_AtlasUVRefPc;

/**
 *gen_ffi:export
 */
typedef struct GameObjsHeaderPc {
  u32Pc const_;
  u32Pc types_num;
  u32Pc types_offset;
  u32Pc obj_num;
  u32Pc obj_offset;
  u32Pc z5;
  u32Pc z6;
  u32Pc z7;
} GameObjsHeaderPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__TypeRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__TypeRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__ObjRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__ObjRefPc;

typedef struct GameObjsRefPc {
  const struct GameObjsHeaderPc *header;
  struct IndexMap_u32__TypeRefPc types;
  struct IndexMap_u32__ObjRefPc objs;
} GameObjsRefPc;

typedef struct SubBlocks1RefPc {
  struct SubBlocksInfoRefPc info;
  struct IndexMap_u32__DataRefPc files;
  struct IndexMap_u32__LuaRefPc lua;
  struct IndexMap_u32__SSARefPc subtitles;
  struct Option_AtlasUVRefPc atlas1;
  struct Option_AtlasUVRefPc atlas2;
  struct GameObjsRefPc level;
} SubBlocks1RefPc;

typedef uint16_t u16_le;

typedef u16_le u16Pc;

/**
 *gen_ffi:export
 */
typedef struct StringKeysHeaderPc {
  u16Pc num_a;
  u16Pc num_b;
  u32Pc z2;
  u32Pc z3;
  u32Pc z4;
  u32Pc z5;
} StringKeysHeaderPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_StringKeysValPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_StringKeysValPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_u32Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_u32Pc;

typedef struct StringKeysRefPc {
  const struct StringKeysHeaderPc *header;
  struct ref_slice_StringKeysValPc vals;
  struct ref_slice_u32Pc pad;
} StringKeysRefPc;

typedef struct Block1RefPc {
  struct InfosRefPc infos;
  struct ObjsRefPc objs;
  struct SubBlocks1RefPc sub_blocks;
  struct StringKeysRefPc string_keys;
} Block1RefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_SprayRefPc {
  uint64_t align;
  uint8_t pad[24];
} Option_SprayRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_CrowdRefPc {
  uint64_t align;
  uint8_t pad[32];
} Option_CrowdRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_PFieldsRefPc {
  uint64_t align;
  uint8_t pad[8];
} Option_PFieldsRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__LangStringsRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__LangStringsRefPc;

typedef struct SubBlocks2RefPc {
  struct SubBlocksInfoRefPc info;
  struct Option_SprayRefPc spray;
  struct Option_CrowdRefPc crowd;
  struct Option_PFieldsRefPc pfields;
  struct IndexMap_u32__LangStringsRefPc langs;
  struct IndexMap_u32__DataRefPc files;
} SubBlocks2RefPc;

typedef struct Block2RefPc {
  struct SubBlocks2RefPc sub_blocks;
  struct ref_slice_u32Pc offsets;
} Block2RefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__AnimationRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__AnimationRefPc;

typedef struct AnimationsRefPc {
  struct IndexMap_u32__AnimationRefPc animations;
  struct ref_slice_AnimationBlockInfoPc block_infos;
} AnimationsRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BlockAValPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BlockAValPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_CompressedDataRef {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_CompressedDataRef;

typedef struct PakRefPc {
  const struct PakHeaderPc *header;
  StringsRefPc strings;
  struct Block1RefPc block1;
  struct Block2RefPc block2;
  struct AnimationsRefPc animations;
  struct ref_slice_BlockAValPc vals_a;
  struct ref_slice_CompressedDataRef animation_data;
} PakRefPc;

/**
 *gen_ffi:export
 */
typedef struct BinHeaderPc {
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
} BinHeaderPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AssetHandlePc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AssetHandlePc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32_______CompressedDataRef {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32_______CompressedDataRef;

typedef struct BinRefPc {
  const struct BinHeaderPc *header;
  StringsRefPc strings;
  struct ref_slice_AssetHandlePc asset_handles;
  struct IndexMap_u32_______CompressedDataRef model_data;
  struct IndexMap_u32_______CompressedDataRef texture_data;
} BinRefPc;

typedef struct LevelRefPc {
  struct PakRefPc pak;
  struct BinRefPc bin;
} LevelRefPc;

typedef uint32_t u32_be;

typedef u32_be u32Xbox;

/**
 *gen_ffi:export
 */
typedef struct PakHeaderXbox {
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
} PakHeaderXbox;

typedef struct slice_string StringsRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_ObjAXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_ObjAXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Obj0Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Obj0Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_ModelInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_ModelInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BufferInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BufferInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat1Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat1Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat2Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat2Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat3Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat3Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat4Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat4Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_MatExtraXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_MatExtraXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_ShapeInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_ShapeInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_HkShapeInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_HkShapeInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_HkConstraintDataXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_HkConstraintDataXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_VBuffInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_VBuffInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_IBuffInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_IBuffInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_TextureInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_TextureInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AnimationInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AnimationInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_HkConstraintInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_HkConstraintInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_EffectInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_EffectInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_PFieldInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_PFieldInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_GFXBlockInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_GFXBlockInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AnimationBlockInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AnimationBlockInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_FoliageInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_FoliageInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RadiosityValsInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RadiosityValsInfoXbox;

typedef struct InfosRefXbox {
  struct ref_slice_ObjAXbox objas;
  struct ref_slice_Obj0Xbox obj0s;
  struct ref_slice_ModelInfoXbox models;
  struct ref_slice_BufferInfoXbox buffers;
  struct ref_slice_Mat1Xbox mat1s;
  struct ref_slice_Mat2Xbox mat2s;
  struct ref_slice_Mat3Xbox mat3s;
  struct ref_slice_Mat4Xbox mat4s;
  struct ref_slice_MatExtraXbox mat_extras;
  struct ref_slice_ShapeInfoXbox shapes;
  struct ref_slice_HkShapeInfoXbox hk_shapes;
  struct ref_slice_HkConstraintDataXbox hk_constraint_datas;
  struct ref_slice_VBuffInfoXbox vbuffs;
  struct ref_slice_IBuffInfoXbox ibuffs;
  struct ref_slice_TextureInfoXbox textures;
  struct ref_slice_AnimationInfoXbox animations;
  struct ref_slice_HkConstraintInfoXbox hk_constraints;
  struct ref_slice_EffectInfoXbox effects;
  struct ref_slice_PFieldInfoXbox pfields;
  struct ref_slice_GFXBlockInfoXbox gfxs;
  struct ref_slice_AnimationBlockInfoXbox animation_blocks;
  struct ref_slice_FoliageInfoXbox foliages;
  struct ref_slice_RadiosityValsInfoXbox radiosity_vals;
} InfosRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__TextureRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__TextureRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__ModelRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__ModelRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__EffectRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__EffectRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__slice_FoliageRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__slice_FoliageRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__RadiosityValsRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__RadiosityValsRefXbox;

typedef struct RadiosityRefXbox {
  const struct CompressedDataRef *data;
  struct IndexMap_u32__RadiosityValsRefXbox vals;
  uint32_t usage;
} RadiosityRefXbox;

typedef struct ObjsRefXbox {
  struct IndexMap_u32__TextureRefXbox textures;
  struct IndexMap_u32__ModelRefXbox models;
  struct IndexMap_u32__EffectRefXbox effects;
  struct IndexMap_u32__slice_FoliageRefXbox foliages;
  struct IndexMap_u32__ref_slice_u8 gfxs;
  struct RadiosityRefXbox radiosity;
} ObjsRefXbox;

/**
 *gen_ffi:export
 */
typedef struct SubBlocksHeaderXbox {
  u32Xbox z0;
  u32Xbox block_num;
  u32Xbox z2;
  u32Xbox z3;
} SubBlocksHeaderXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SubBlocksBlockHeaderXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SubBlocksBlockHeaderXbox;

typedef struct SubBlocksInfoRefXbox {
  const struct SubBlocksHeaderXbox *header;
  struct ref_slice_SubBlocksBlockHeaderXbox block_headers;
} SubBlocksInfoRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__DataRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__DataRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__LuaRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__LuaRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__SSARefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__SSARefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_AtlasUVRefXbox {
  uint64_t align;
  uint8_t pad[8];
} Option_AtlasUVRefXbox;

/**
 *gen_ffi:export
 */
typedef struct GameObjsHeaderXbox {
  u32Xbox const_;
  u32Xbox types_num;
  u32Xbox types_offset;
  u32Xbox obj_num;
  u32Xbox obj_offset;
  u32Xbox z5;
  u32Xbox z6;
  u32Xbox z7;
} GameObjsHeaderXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__TypeRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__TypeRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__ObjRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__ObjRefXbox;

typedef struct GameObjsRefXbox {
  const struct GameObjsHeaderXbox *header;
  struct IndexMap_u32__TypeRefXbox types;
  struct IndexMap_u32__ObjRefXbox objs;
} GameObjsRefXbox;

typedef struct SubBlocks1RefXbox {
  struct SubBlocksInfoRefXbox info;
  struct IndexMap_u32__DataRefXbox files;
  struct IndexMap_u32__LuaRefXbox lua;
  struct IndexMap_u32__SSARefXbox subtitles;
  struct Option_AtlasUVRefXbox atlas1;
  struct Option_AtlasUVRefXbox atlas2;
  struct GameObjsRefXbox level;
} SubBlocks1RefXbox;

typedef uint16_t u16_be;

typedef u16_be u16Xbox;

/**
 *gen_ffi:export
 */
typedef struct StringKeysHeaderXbox {
  u16Xbox num_a;
  u16Xbox num_b;
  u32Xbox z2;
  u32Xbox z3;
  u32Xbox z4;
  u32Xbox z5;
} StringKeysHeaderXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_StringKeysValXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_StringKeysValXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_u32Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_u32Xbox;

typedef struct StringKeysRefXbox {
  const struct StringKeysHeaderXbox *header;
  struct ref_slice_StringKeysValXbox vals;
  struct ref_slice_u32Xbox pad;
} StringKeysRefXbox;

typedef struct Block1RefXbox {
  struct InfosRefXbox infos;
  struct ObjsRefXbox objs;
  struct SubBlocks1RefXbox sub_blocks;
  struct StringKeysRefXbox string_keys;
} Block1RefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_SprayRefXbox {
  uint64_t align;
  uint8_t pad[24];
} Option_SprayRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_CrowdRefXbox {
  uint64_t align;
  uint8_t pad[32];
} Option_CrowdRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_PFieldsRefXbox {
  uint64_t align;
  uint8_t pad[8];
} Option_PFieldsRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__LangStringsRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__LangStringsRefXbox;

typedef struct SubBlocks2RefXbox {
  struct SubBlocksInfoRefXbox info;
  struct Option_SprayRefXbox spray;
  struct Option_CrowdRefXbox crowd;
  struct Option_PFieldsRefXbox pfields;
  struct IndexMap_u32__LangStringsRefXbox langs;
  struct IndexMap_u32__DataRefXbox files;
} SubBlocks2RefXbox;

typedef struct Block2RefXbox {
  struct SubBlocks2RefXbox sub_blocks;
  struct ref_slice_u32Xbox offsets;
} Block2RefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__AnimationRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__AnimationRefXbox;

typedef struct AnimationsRefXbox {
  struct IndexMap_u32__AnimationRefXbox animations;
  struct ref_slice_AnimationBlockInfoXbox block_infos;
} AnimationsRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BlockAValXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BlockAValXbox;

typedef struct PakRefXbox {
  const struct PakHeaderXbox *header;
  StringsRefXbox strings;
  struct Block1RefXbox block1;
  struct Block2RefXbox block2;
  struct AnimationsRefXbox animations;
  struct ref_slice_BlockAValXbox vals_a;
  struct ref_slice_CompressedDataRef animation_data;
} PakRefXbox;

/**
 *gen_ffi:export
 */
typedef struct BinHeaderXbox {
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
} BinHeaderXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AssetHandleXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AssetHandleXbox;

typedef struct BinRefXbox {
  const struct BinHeaderXbox *header;
  StringsRefXbox strings;
  struct ref_slice_AssetHandleXbox asset_handles;
  struct IndexMap_u32_______CompressedDataRef model_data;
  struct IndexMap_u32_______CompressedDataRef texture_data;
} BinRefXbox;

typedef struct LevelRefXbox {
  struct PakRefXbox pak;
  struct BinRefXbox bin;
} LevelRefXbox;

typedef u32_be u32Ps3;

/**
 *gen_ffi:export
 */
typedef struct PakHeaderPs3 {
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
} PakHeaderPs3;

typedef struct slice_string StringsRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_ObjAPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_ObjAPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Obj0Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Obj0Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_ModelInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_ModelInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BufferInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BufferInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat1Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat1Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat2Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat2Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat3Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat3Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Mat4Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Mat4Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_MatExtraPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_MatExtraPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_ShapeInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_ShapeInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_HkShapeInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_HkShapeInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_HkConstraintDataPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_HkConstraintDataPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_VBuffInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_VBuffInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_IBuffInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_IBuffInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_TextureInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_TextureInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AnimationInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AnimationInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_HkConstraintInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_HkConstraintInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_EffectInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_EffectInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_PFieldInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_PFieldInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_GFXBlockInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_GFXBlockInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AnimationBlockInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AnimationBlockInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_FoliageInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_FoliageInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RadiosityValsInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RadiosityValsInfoPs3;

typedef struct InfosRefPs3 {
  struct ref_slice_ObjAPs3 objas;
  struct ref_slice_Obj0Ps3 obj0s;
  struct ref_slice_ModelInfoPs3 models;
  struct ref_slice_BufferInfoPs3 buffers;
  struct ref_slice_Mat1Ps3 mat1s;
  struct ref_slice_Mat2Ps3 mat2s;
  struct ref_slice_Mat3Ps3 mat3s;
  struct ref_slice_Mat4Ps3 mat4s;
  struct ref_slice_MatExtraPs3 mat_extras;
  struct ref_slice_ShapeInfoPs3 shapes;
  struct ref_slice_HkShapeInfoPs3 hk_shapes;
  struct ref_slice_HkConstraintDataPs3 hk_constraint_datas;
  struct ref_slice_VBuffInfoPs3 vbuffs;
  struct ref_slice_IBuffInfoPs3 ibuffs;
  struct ref_slice_TextureInfoPs3 textures;
  struct ref_slice_AnimationInfoPs3 animations;
  struct ref_slice_HkConstraintInfoPs3 hk_constraints;
  struct ref_slice_EffectInfoPs3 effects;
  struct ref_slice_PFieldInfoPs3 pfields;
  struct ref_slice_GFXBlockInfoPs3 gfxs;
  struct ref_slice_AnimationBlockInfoPs3 animation_blocks;
  struct ref_slice_FoliageInfoPs3 foliages;
  struct ref_slice_RadiosityValsInfoPs3 radiosity_vals;
} InfosRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__TextureRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__TextureRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__ModelRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__ModelRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__EffectRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__EffectRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__slice_FoliageRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__slice_FoliageRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__RadiosityValsRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__RadiosityValsRefPs3;

typedef struct RadiosityRefPs3 {
  const struct CompressedDataRef *data;
  struct IndexMap_u32__RadiosityValsRefPs3 vals;
  uint32_t usage;
} RadiosityRefPs3;

typedef struct ObjsRefPs3 {
  struct IndexMap_u32__TextureRefPs3 textures;
  struct IndexMap_u32__ModelRefPs3 models;
  struct IndexMap_u32__EffectRefPs3 effects;
  struct IndexMap_u32__slice_FoliageRefPs3 foliages;
  struct IndexMap_u32__ref_slice_u8 gfxs;
  struct RadiosityRefPs3 radiosity;
} ObjsRefPs3;

/**
 *gen_ffi:export
 */
typedef struct SubBlocksHeaderPs3 {
  u32Ps3 z0;
  u32Ps3 block_num;
  u32Ps3 z2;
  u32Ps3 z3;
} SubBlocksHeaderPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SubBlocksBlockHeaderPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SubBlocksBlockHeaderPs3;

typedef struct SubBlocksInfoRefPs3 {
  const struct SubBlocksHeaderPs3 *header;
  struct ref_slice_SubBlocksBlockHeaderPs3 block_headers;
} SubBlocksInfoRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__DataRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__DataRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__LuaRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__LuaRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__SSARefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__SSARefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_AtlasUVRefPs3 {
  uint64_t align;
  uint8_t pad[8];
} Option_AtlasUVRefPs3;

/**
 *gen_ffi:export
 */
typedef struct GameObjsHeaderPs3 {
  u32Ps3 const_;
  u32Ps3 types_num;
  u32Ps3 types_offset;
  u32Ps3 obj_num;
  u32Ps3 obj_offset;
  u32Ps3 z5;
  u32Ps3 z6;
  u32Ps3 z7;
} GameObjsHeaderPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__TypeRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__TypeRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__ObjRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__ObjRefPs3;

typedef struct GameObjsRefPs3 {
  const struct GameObjsHeaderPs3 *header;
  struct IndexMap_u32__TypeRefPs3 types;
  struct IndexMap_u32__ObjRefPs3 objs;
} GameObjsRefPs3;

typedef struct SubBlocks1RefPs3 {
  struct SubBlocksInfoRefPs3 info;
  struct IndexMap_u32__DataRefPs3 files;
  struct IndexMap_u32__LuaRefPs3 lua;
  struct IndexMap_u32__SSARefPs3 subtitles;
  struct Option_AtlasUVRefPs3 atlas1;
  struct Option_AtlasUVRefPs3 atlas2;
  struct GameObjsRefPs3 level;
} SubBlocks1RefPs3;

typedef u16_be u16Ps3;

/**
 *gen_ffi:export
 */
typedef struct StringKeysHeaderPs3 {
  u16Ps3 num_a;
  u16Ps3 num_b;
  u32Ps3 z2;
  u32Ps3 z3;
  u32Ps3 z4;
  u32Ps3 z5;
} StringKeysHeaderPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_StringKeysValPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_StringKeysValPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_u32Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_u32Ps3;

typedef struct StringKeysRefPs3 {
  const struct StringKeysHeaderPs3 *header;
  struct ref_slice_StringKeysValPs3 vals;
  struct ref_slice_u32Ps3 pad;
} StringKeysRefPs3;

typedef struct Block1RefPs3 {
  struct InfosRefPs3 infos;
  struct ObjsRefPs3 objs;
  struct SubBlocks1RefPs3 sub_blocks;
  struct StringKeysRefPs3 string_keys;
} Block1RefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_SprayRefPs3 {
  uint64_t align;
  uint8_t pad[24];
} Option_SprayRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_CrowdRefPs3 {
  uint64_t align;
  uint8_t pad[32];
} Option_CrowdRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_PFieldsRefPs3 {
  uint64_t align;
  uint8_t pad[8];
} Option_PFieldsRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__LangStringsRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__LangStringsRefPs3;

typedef struct SubBlocks2RefPs3 {
  struct SubBlocksInfoRefPs3 info;
  struct Option_SprayRefPs3 spray;
  struct Option_CrowdRefPs3 crowd;
  struct Option_PFieldsRefPs3 pfields;
  struct IndexMap_u32__LangStringsRefPs3 langs;
  struct IndexMap_u32__DataRefPs3 files;
} SubBlocks2RefPs3;

typedef struct Block2RefPs3 {
  struct SubBlocks2RefPs3 sub_blocks;
  struct ref_slice_u32Ps3 offsets;
} Block2RefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__AnimationRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__AnimationRefPs3;

typedef struct AnimationsRefPs3 {
  struct IndexMap_u32__AnimationRefPs3 animations;
  struct ref_slice_AnimationBlockInfoPs3 block_infos;
} AnimationsRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BlockAValPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BlockAValPs3;

typedef struct PakRefPs3 {
  const struct PakHeaderPs3 *header;
  StringsRefPs3 strings;
  struct Block1RefPs3 block1;
  struct Block2RefPs3 block2;
  struct AnimationsRefPs3 animations;
  struct ref_slice_BlockAValPs3 vals_a;
  struct ref_slice_CompressedDataRef animation_data;
} PakRefPs3;

/**
 *gen_ffi:export
 */
typedef struct BinHeaderPs3 {
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
} BinHeaderPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AssetHandlePs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AssetHandlePs3;

typedef struct BinRefPs3 {
  const struct BinHeaderPs3 *header;
  StringsRefPs3 strings;
  struct ref_slice_AssetHandlePs3 asset_handles;
  struct IndexMap_u32_______CompressedDataRef model_data;
  struct IndexMap_u32_______CompressedDataRef texture_data;
} BinRefPs3;

typedef struct LevelRefPs3 {
  struct PakRefPs3 pak;
  struct BinRefPs3 bin;
} LevelRefPs3;

typedef struct InfoCounts {
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
} InfoCounts;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_u8 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_u8;

typedef struct DumpSlice {
  struct mut_slice_u8 vals;
  uintptr_t offset;
} DumpSlice;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_u32Pc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_u32Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_u32Xbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_u32Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_u32Ps3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_u32Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_ObjAPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_ObjAPc;

typedef struct DumpInfo_ObjAPc {
  struct mut_slice_ObjAPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_ObjAPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Obj0Pc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Obj0Pc;

typedef struct DumpInfo_Obj0Pc {
  struct mut_slice_Obj0Pc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Obj0Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_ModelInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_ModelInfoPc;

typedef struct DumpInfo_ModelInfoPc {
  struct mut_slice_ModelInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_ModelInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_BufferInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_BufferInfoPc;

typedef struct DumpInfo_BufferInfoPc {
  struct mut_slice_BufferInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_BufferInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat1Pc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat1Pc;

typedef struct DumpInfo_Mat1Pc {
  struct mut_slice_Mat1Pc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat1Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat2Pc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat2Pc;

typedef struct DumpInfo_Mat2Pc {
  struct mut_slice_Mat2Pc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat2Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat3Pc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat3Pc;

typedef struct DumpInfo_Mat3Pc {
  struct mut_slice_Mat3Pc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat3Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat4Pc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat4Pc;

typedef struct DumpInfo_Mat4Pc {
  struct mut_slice_Mat4Pc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat4Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_MatExtraPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_MatExtraPc;

typedef struct DumpInfo_MatExtraPc {
  struct mut_slice_MatExtraPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_MatExtraPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_ShapeInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_ShapeInfoPc;

typedef struct DumpInfo_ShapeInfoPc {
  struct mut_slice_ShapeInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_ShapeInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_HkShapeInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_HkShapeInfoPc;

typedef struct DumpInfo_HkShapeInfoPc {
  struct mut_slice_HkShapeInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_HkShapeInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_HkConstraintDataPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_HkConstraintDataPc;

typedef struct DumpInfo_HkConstraintDataPc {
  struct mut_slice_HkConstraintDataPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_HkConstraintDataPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_VBuffInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_VBuffInfoPc;

typedef struct DumpInfo_VBuffInfoPc {
  struct mut_slice_VBuffInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_VBuffInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_IBuffInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_IBuffInfoPc;

typedef struct DumpInfo_IBuffInfoPc {
  struct mut_slice_IBuffInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_IBuffInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_TextureInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_TextureInfoPc;

typedef struct DumpInfo_TextureInfoPc {
  struct mut_slice_TextureInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_TextureInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_AnimationInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_AnimationInfoPc;

typedef struct DumpInfo_AnimationInfoPc {
  struct mut_slice_AnimationInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_AnimationInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_HkConstraintInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_HkConstraintInfoPc;

typedef struct DumpInfo_HkConstraintInfoPc {
  struct mut_slice_HkConstraintInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_HkConstraintInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_EffectInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_EffectInfoPc;

typedef struct DumpInfo_EffectInfoPc {
  struct mut_slice_EffectInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_EffectInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_PFieldInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_PFieldInfoPc;

typedef struct DumpInfo_PFieldInfoPc {
  struct mut_slice_PFieldInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_PFieldInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_GFXBlockInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_GFXBlockInfoPc;

typedef struct DumpInfo_GFXBlockInfoPc {
  struct mut_slice_GFXBlockInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_GFXBlockInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_AnimationBlockInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_AnimationBlockInfoPc;

typedef struct DumpInfo_AnimationBlockInfoPc {
  struct mut_slice_AnimationBlockInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_AnimationBlockInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_FoliageInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_FoliageInfoPc;

typedef struct DumpInfo_FoliageInfoPc {
  struct mut_slice_FoliageInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_FoliageInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_RadiosityValsInfoPc {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_RadiosityValsInfoPc;

typedef struct DumpInfo_RadiosityValsInfoPc {
  struct mut_slice_RadiosityValsInfoPc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_RadiosityValsInfoPc;

typedef struct DumpInfo_u32Pc {
  struct mut_slice_u32Pc vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_u32Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Vec_DumpInfoDataPc {
  uint64_t align;
  uint8_t pad[16];
} Vec_DumpInfoDataPc;

typedef struct DumpInfosPc {
  struct DumpInfo_ObjAPc objas;
  struct DumpInfo_Obj0Pc obj0s;
  struct DumpInfo_ModelInfoPc models;
  struct DumpInfo_BufferInfoPc buffers;
  struct DumpInfo_Mat1Pc mat1s;
  struct DumpInfo_Mat2Pc mat2s;
  struct DumpInfo_Mat3Pc mat3s;
  struct DumpInfo_Mat4Pc mat4s;
  struct DumpInfo_MatExtraPc mat_extras;
  struct DumpInfo_ShapeInfoPc shapes;
  struct DumpInfo_HkShapeInfoPc hk_shapes;
  struct DumpInfo_HkConstraintDataPc hk_constraint_datas;
  struct DumpInfo_VBuffInfoPc vbuffs;
  struct DumpInfo_IBuffInfoPc ibuffs;
  struct DumpInfo_TextureInfoPc textures;
  struct DumpInfo_AnimationInfoPc animations;
  struct DumpInfo_HkConstraintInfoPc hk_constraints;
  struct DumpInfo_EffectInfoPc effects;
  struct DumpInfo_PFieldInfoPc pfields;
  struct DumpInfo_GFXBlockInfoPc gfxs;
  struct DumpInfo_AnimationBlockInfoPc animation_blocks;
  struct DumpInfo_FoliageInfoPc foliages;
  struct DumpInfo_RadiosityValsInfoPc radiosity_vals;
  struct DumpInfo_u32Pc offsets;
  struct Vec_DumpInfoDataPc model_data;
  struct Vec_DumpInfoDataPc texture_data;
} DumpInfosPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_ObjAXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_ObjAXbox;

typedef struct DumpInfo_ObjAXbox {
  struct mut_slice_ObjAXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_ObjAXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Obj0Xbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Obj0Xbox;

typedef struct DumpInfo_Obj0Xbox {
  struct mut_slice_Obj0Xbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Obj0Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_ModelInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_ModelInfoXbox;

typedef struct DumpInfo_ModelInfoXbox {
  struct mut_slice_ModelInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_ModelInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_BufferInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_BufferInfoXbox;

typedef struct DumpInfo_BufferInfoXbox {
  struct mut_slice_BufferInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_BufferInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat1Xbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat1Xbox;

typedef struct DumpInfo_Mat1Xbox {
  struct mut_slice_Mat1Xbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat1Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat2Xbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat2Xbox;

typedef struct DumpInfo_Mat2Xbox {
  struct mut_slice_Mat2Xbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat2Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat3Xbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat3Xbox;

typedef struct DumpInfo_Mat3Xbox {
  struct mut_slice_Mat3Xbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat3Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat4Xbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat4Xbox;

typedef struct DumpInfo_Mat4Xbox {
  struct mut_slice_Mat4Xbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat4Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_MatExtraXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_MatExtraXbox;

typedef struct DumpInfo_MatExtraXbox {
  struct mut_slice_MatExtraXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_MatExtraXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_ShapeInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_ShapeInfoXbox;

typedef struct DumpInfo_ShapeInfoXbox {
  struct mut_slice_ShapeInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_ShapeInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_HkShapeInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_HkShapeInfoXbox;

typedef struct DumpInfo_HkShapeInfoXbox {
  struct mut_slice_HkShapeInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_HkShapeInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_HkConstraintDataXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_HkConstraintDataXbox;

typedef struct DumpInfo_HkConstraintDataXbox {
  struct mut_slice_HkConstraintDataXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_HkConstraintDataXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_VBuffInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_VBuffInfoXbox;

typedef struct DumpInfo_VBuffInfoXbox {
  struct mut_slice_VBuffInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_VBuffInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_IBuffInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_IBuffInfoXbox;

typedef struct DumpInfo_IBuffInfoXbox {
  struct mut_slice_IBuffInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_IBuffInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_TextureInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_TextureInfoXbox;

typedef struct DumpInfo_TextureInfoXbox {
  struct mut_slice_TextureInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_TextureInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_AnimationInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_AnimationInfoXbox;

typedef struct DumpInfo_AnimationInfoXbox {
  struct mut_slice_AnimationInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_AnimationInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_HkConstraintInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_HkConstraintInfoXbox;

typedef struct DumpInfo_HkConstraintInfoXbox {
  struct mut_slice_HkConstraintInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_HkConstraintInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_EffectInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_EffectInfoXbox;

typedef struct DumpInfo_EffectInfoXbox {
  struct mut_slice_EffectInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_EffectInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_PFieldInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_PFieldInfoXbox;

typedef struct DumpInfo_PFieldInfoXbox {
  struct mut_slice_PFieldInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_PFieldInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_GFXBlockInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_GFXBlockInfoXbox;

typedef struct DumpInfo_GFXBlockInfoXbox {
  struct mut_slice_GFXBlockInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_GFXBlockInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_AnimationBlockInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_AnimationBlockInfoXbox;

typedef struct DumpInfo_AnimationBlockInfoXbox {
  struct mut_slice_AnimationBlockInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_AnimationBlockInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_FoliageInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_FoliageInfoXbox;

typedef struct DumpInfo_FoliageInfoXbox {
  struct mut_slice_FoliageInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_FoliageInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_RadiosityValsInfoXbox {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_RadiosityValsInfoXbox;

typedef struct DumpInfo_RadiosityValsInfoXbox {
  struct mut_slice_RadiosityValsInfoXbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_RadiosityValsInfoXbox;

typedef struct DumpInfo_u32Xbox {
  struct mut_slice_u32Xbox vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_u32Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Vec_DumpInfoDataXbox {
  uint64_t align;
  uint8_t pad[16];
} Vec_DumpInfoDataXbox;

typedef struct DumpInfosXbox {
  struct DumpInfo_ObjAXbox objas;
  struct DumpInfo_Obj0Xbox obj0s;
  struct DumpInfo_ModelInfoXbox models;
  struct DumpInfo_BufferInfoXbox buffers;
  struct DumpInfo_Mat1Xbox mat1s;
  struct DumpInfo_Mat2Xbox mat2s;
  struct DumpInfo_Mat3Xbox mat3s;
  struct DumpInfo_Mat4Xbox mat4s;
  struct DumpInfo_MatExtraXbox mat_extras;
  struct DumpInfo_ShapeInfoXbox shapes;
  struct DumpInfo_HkShapeInfoXbox hk_shapes;
  struct DumpInfo_HkConstraintDataXbox hk_constraint_datas;
  struct DumpInfo_VBuffInfoXbox vbuffs;
  struct DumpInfo_IBuffInfoXbox ibuffs;
  struct DumpInfo_TextureInfoXbox textures;
  struct DumpInfo_AnimationInfoXbox animations;
  struct DumpInfo_HkConstraintInfoXbox hk_constraints;
  struct DumpInfo_EffectInfoXbox effects;
  struct DumpInfo_PFieldInfoXbox pfields;
  struct DumpInfo_GFXBlockInfoXbox gfxs;
  struct DumpInfo_AnimationBlockInfoXbox animation_blocks;
  struct DumpInfo_FoliageInfoXbox foliages;
  struct DumpInfo_RadiosityValsInfoXbox radiosity_vals;
  struct DumpInfo_u32Xbox offsets;
  struct Vec_DumpInfoDataXbox model_data;
  struct Vec_DumpInfoDataXbox texture_data;
} DumpInfosXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_ObjAPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_ObjAPs3;

typedef struct DumpInfo_ObjAPs3 {
  struct mut_slice_ObjAPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_ObjAPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Obj0Ps3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Obj0Ps3;

typedef struct DumpInfo_Obj0Ps3 {
  struct mut_slice_Obj0Ps3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Obj0Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_ModelInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_ModelInfoPs3;

typedef struct DumpInfo_ModelInfoPs3 {
  struct mut_slice_ModelInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_ModelInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_BufferInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_BufferInfoPs3;

typedef struct DumpInfo_BufferInfoPs3 {
  struct mut_slice_BufferInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_BufferInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat1Ps3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat1Ps3;

typedef struct DumpInfo_Mat1Ps3 {
  struct mut_slice_Mat1Ps3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat1Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat2Ps3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat2Ps3;

typedef struct DumpInfo_Mat2Ps3 {
  struct mut_slice_Mat2Ps3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat2Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat3Ps3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat3Ps3;

typedef struct DumpInfo_Mat3Ps3 {
  struct mut_slice_Mat3Ps3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat3Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_Mat4Ps3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_Mat4Ps3;

typedef struct DumpInfo_Mat4Ps3 {
  struct mut_slice_Mat4Ps3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_Mat4Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_MatExtraPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_MatExtraPs3;

typedef struct DumpInfo_MatExtraPs3 {
  struct mut_slice_MatExtraPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_MatExtraPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_ShapeInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_ShapeInfoPs3;

typedef struct DumpInfo_ShapeInfoPs3 {
  struct mut_slice_ShapeInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_ShapeInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_HkShapeInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_HkShapeInfoPs3;

typedef struct DumpInfo_HkShapeInfoPs3 {
  struct mut_slice_HkShapeInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_HkShapeInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_HkConstraintDataPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_HkConstraintDataPs3;

typedef struct DumpInfo_HkConstraintDataPs3 {
  struct mut_slice_HkConstraintDataPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_HkConstraintDataPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_VBuffInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_VBuffInfoPs3;

typedef struct DumpInfo_VBuffInfoPs3 {
  struct mut_slice_VBuffInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_VBuffInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_IBuffInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_IBuffInfoPs3;

typedef struct DumpInfo_IBuffInfoPs3 {
  struct mut_slice_IBuffInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_IBuffInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_TextureInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_TextureInfoPs3;

typedef struct DumpInfo_TextureInfoPs3 {
  struct mut_slice_TextureInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_TextureInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_AnimationInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_AnimationInfoPs3;

typedef struct DumpInfo_AnimationInfoPs3 {
  struct mut_slice_AnimationInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_AnimationInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_HkConstraintInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_HkConstraintInfoPs3;

typedef struct DumpInfo_HkConstraintInfoPs3 {
  struct mut_slice_HkConstraintInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_HkConstraintInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_EffectInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_EffectInfoPs3;

typedef struct DumpInfo_EffectInfoPs3 {
  struct mut_slice_EffectInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_EffectInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_PFieldInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_PFieldInfoPs3;

typedef struct DumpInfo_PFieldInfoPs3 {
  struct mut_slice_PFieldInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_PFieldInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_GFXBlockInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_GFXBlockInfoPs3;

typedef struct DumpInfo_GFXBlockInfoPs3 {
  struct mut_slice_GFXBlockInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_GFXBlockInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_AnimationBlockInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_AnimationBlockInfoPs3;

typedef struct DumpInfo_AnimationBlockInfoPs3 {
  struct mut_slice_AnimationBlockInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_AnimationBlockInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_FoliageInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_FoliageInfoPs3;

typedef struct DumpInfo_FoliageInfoPs3 {
  struct mut_slice_FoliageInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_FoliageInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_RadiosityValsInfoPs3 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_RadiosityValsInfoPs3;

typedef struct DumpInfo_RadiosityValsInfoPs3 {
  struct mut_slice_RadiosityValsInfoPs3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_RadiosityValsInfoPs3;

typedef struct DumpInfo_u32Ps3 {
  struct mut_slice_u32Ps3 vals;
  uintptr_t ind;
  uintptr_t offset;
} DumpInfo_u32Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Vec_DumpInfoDataPs3 {
  uint64_t align;
  uint8_t pad[16];
} Vec_DumpInfoDataPs3;

typedef struct DumpInfosPs3 {
  struct DumpInfo_ObjAPs3 objas;
  struct DumpInfo_Obj0Ps3 obj0s;
  struct DumpInfo_ModelInfoPs3 models;
  struct DumpInfo_BufferInfoPs3 buffers;
  struct DumpInfo_Mat1Ps3 mat1s;
  struct DumpInfo_Mat2Ps3 mat2s;
  struct DumpInfo_Mat3Ps3 mat3s;
  struct DumpInfo_Mat4Ps3 mat4s;
  struct DumpInfo_MatExtraPs3 mat_extras;
  struct DumpInfo_ShapeInfoPs3 shapes;
  struct DumpInfo_HkShapeInfoPs3 hk_shapes;
  struct DumpInfo_HkConstraintDataPs3 hk_constraint_datas;
  struct DumpInfo_VBuffInfoPs3 vbuffs;
  struct DumpInfo_IBuffInfoPs3 ibuffs;
  struct DumpInfo_TextureInfoPs3 textures;
  struct DumpInfo_AnimationInfoPs3 animations;
  struct DumpInfo_HkConstraintInfoPs3 hk_constraints;
  struct DumpInfo_EffectInfoPs3 effects;
  struct DumpInfo_PFieldInfoPs3 pfields;
  struct DumpInfo_GFXBlockInfoPs3 gfxs;
  struct DumpInfo_AnimationBlockInfoPs3 animation_blocks;
  struct DumpInfo_FoliageInfoPs3 foliages;
  struct DumpInfo_RadiosityValsInfoPs3 radiosity_vals;
  struct DumpInfo_u32Ps3 offsets;
  struct Vec_DumpInfoDataPs3 model_data;
  struct Vec_DumpInfoDataPs3 texture_data;
} DumpInfosPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct string {
  uint64_t align;
  uint8_t pad[8];
} string;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_u32 {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_u32;

typedef struct VertexDataIndex {
  uint32_t key;
  uintptr_t offset;
} VertexDataIndex;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_VertexUsage__VertexDataIndex {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_VertexUsage__VertexDataIndex;

enum VertexUsage_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  VertexUsage_Position,
  VertexUsage_Normal,
  VertexUsage_Tangent,
  VertexUsage_BlendWeight,
  VertexUsage_BlendIndices,
  VertexUsage_Color,
  VertexUsage_TextureCoord,
  VertexUsage_PSize,
  VertexUsage_Pad,
};
#if __STDC_VERSION__ >= 202311L
typedef enum VertexUsage_Tag VertexUsage_Tag;
#else
typedef uint8_t VertexUsage_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct VertexUsage {
  VertexUsage_Tag tag;
  union {
    struct {
      uintptr_t color;
    };
    struct {
      uintptr_t texture_coord;
    };
    struct {
      uintptr_t pad;
    };
  };
} VertexUsage;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct mut_slice_VertexUsage {
  uint64_t align;
  uint8_t pad[8];
} mut_slice_VertexUsage;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_u32 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_u32;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_VertexUsage {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_VertexUsage;

typedef struct AlignmentHelper {
  uint32_t a;
} AlignmentHelper;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice______CompressedDataRef {
  uint64_t align;
  uint8_t pad[8];
} slice______CompressedDataRef;

/**
 *gen_ffi:export
 */
typedef struct IBuffInfoPc {
  u32Pc unk_0;
  u32Pc size;
  u32Pc format;
  u32Pc vbuff_alt_fmt;
  u32Pc offset;
  u32Pc unk_5;
} IBuffInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_u16Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_u16Pc;

enum IndexBufferValsRefPc_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  IndexBufferValsRefPc_U16,
  IndexBufferValsRefPc_U32,
};
#if __STDC_VERSION__ >= 202311L
typedef enum IndexBufferValsRefPc_Tag IndexBufferValsRefPc_Tag;
#else
typedef uint8_t IndexBufferValsRefPc_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct IndexBufferValsRefPc {
  IndexBufferValsRefPc_Tag tag;
  union {
    struct {
      struct ref_slice_u16Pc u16;
    };
    struct {
      struct ref_slice_u32Pc u32;
    };
  };
} IndexBufferValsRefPc;

typedef struct IndexBufferRefPc {
  const struct IBuffInfoPc *info;
  struct IndexBufferValsRefPc vals;
} IndexBufferRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__IndexBufferRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__IndexBufferRefPc;

/**
 *gen_ffi:export
 */
typedef struct VBuffInfoPc {
  u32Pc unk_0;
  u32Pc size;
  u32Pc unk_3;
  u32Pc offset;
  u32Pc fmt1;
  u32Pc fmt2;
  u32Pc unk_6;
  u32Pc unk_7;
} VBuffInfoPc;

typedef struct VertexBufferRefPc {
  const struct VBuffInfoPc *info;
  struct IndexMap_VertexUsage__VertexDataIndex offsets;
  uintptr_t size;
  struct ref_slice_u8 data;
} VertexBufferRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__VertexBufferRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__VertexBufferRefPc;

typedef u32Pc CrcPc;

typedef uint64_t u64_le;

typedef u64_le u64Pc;

typedef uint8_t u8Pc;

typedef float f32_le;

typedef f32_le f32Pc;

/**
 *gen_ffi:export
 */
typedef struct MatBasePc {
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
} MatBasePc;

/**
 *gen_ffi:export
 */
typedef struct Mat1Pc {
  struct MatBasePc base;
} Mat1Pc;

/**
 *gen_ffi:export
 */
typedef struct MatExtraPc {
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
} MatExtraPc;

typedef struct Mat1RefPc {
  const struct Mat1Pc *info;
  const struct MatExtraPc *extra;
} Mat1RefPc;

/**
 *gen_ffi:export
 */
typedef struct Mat2Pc {
  struct MatBasePc base;
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
} Mat2Pc;

typedef struct Mat2RefPc {
  const struct Mat2Pc *info;
  const struct MatExtraPc *extra;
} Mat2RefPc;

/**
 *gen_ffi:export
 */
typedef struct Mat3Pc {
  struct MatBasePc base;
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
} Mat3Pc;

typedef struct Mat3RefPc {
  const struct Mat3Pc *info;
  const struct MatExtraPc *extra;
} Mat3RefPc;

/**
 *gen_ffi:export
 */
typedef struct Mat4Pc {
  struct MatBasePc base;
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
} Mat4Pc;

typedef struct Mat4RefPc {
  const struct Mat4Pc *info;
  const struct MatExtraPc *extra;
} Mat4RefPc;

enum MatRefPc_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  MatRefPc_Mat1,
  MatRefPc_Mat2,
  MatRefPc_Mat3,
  MatRefPc_Mat4,
};
#if __STDC_VERSION__ >= 202311L
typedef enum MatRefPc_Tag MatRefPc_Tag;
#else
typedef uint8_t MatRefPc_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct MatRefPc {
  MatRefPc_Tag tag;
  union {
    struct {
      struct Mat1RefPc mat1;
    };
    struct {
      struct Mat2RefPc mat2;
    };
    struct {
      struct Mat3RefPc mat3;
    };
    struct {
      struct Mat4RefPc mat4;
    };
  };
} MatRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__MatRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__MatRefPc;

typedef int32_t i32_le;

typedef i32_le i32Pc;

/**
 *gen_ffi:export
 */
typedef struct Vector3Pc {
  f32Pc x;
  f32Pc y;
  f32Pc z;
} Vector3Pc;

/**
 *gen_ffi:export
 */
typedef struct BoundingBoxPc {
  struct Vector3Pc center;
  f32Pc unk_3;
  struct Vector3Pc half_width;
  f32Pc unk_7;
} BoundingBoxPc;

/**
 *gen_ffi:export
 */
typedef struct LodInfoPc {
  u32Pc start;
  u32Pc static_end;
  u32Pc skinned_end;
  u32Pc physics_end;
  u32Pc breakable_end;
} LodInfoPc;

/**
 *gen_ffi:export
 */
typedef struct ModelInfoPc {
  CrcPc key;
  i32Pc gamemodemask;
  u32Pc mat_offset;
  u32Pc buffer_info_offset;
  struct BoundingBoxPc bounding_box;
  u32Pc mesh_order_offset;
  struct LodInfoPc lod0;
  struct LodInfoPc lod1;
  struct LodInfoPc lod2;
  struct LodInfoPc lod3;
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
} ModelInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_CrcPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_CrcPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_i32Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_i32Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Matrix4x4Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Matrix4x4Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BoundingBoxPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BoundingBoxPc;

typedef struct BonesRefPc {
  struct ref_slice_CrcPc names;
  struct ref_slice_i32Pc parents;
  struct ref_slice_Matrix4x4Pc transforms;
  struct ref_slice_BoundingBoxPc bounding_boxes;
} BonesRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Key2Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Key2Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_BlockRefPc {
  uint64_t align;
  uint8_t pad[8];
} slice_BlockRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32_______VBuffInfoPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32_______VBuffInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32_______IBuffInfoPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32_______IBuffInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_HkConstraintRefPc {
  uint64_t align;
  uint8_t pad[112];
} Option_HkConstraintRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_ShapeRefPc {
  uint64_t align;
  uint8_t pad[8];
} slice_ShapeRefPc;

typedef struct ModelDataRefPc {
  struct ref_slice_BufferInfoPc infos;
  struct ref_slice_u32Pc vbuff_order;
  struct ref_slice_u32Pc ibuff_order;
  struct IndexMap_u32__VertexBufferRefPc vertex;
  struct IndexMap_u32__IndexBufferRefPc index;
  const struct CompressedDataRef *data;
} ModelDataRefPc;

typedef struct ModelRefPc {
  const struct ModelInfoPc *info;
  struct BonesRefPc bones;
  struct ref_slice_u32Pc mat_order;
  struct ref_slice_u32Pc mesh_order;
  struct ref_slice_BoundingBoxPc mesh_bounding_boxes;
  struct ref_slice_Matrix4x4Pc skin_binds;
  struct ref_slice_u32Pc vals_j;
  struct ref_slice_u16Pc val_k_header;
  struct ref_slice_u32Pc vals_k;
  struct ref_slice_u32Pc skin_order;
  struct ref_slice_Key2Pc slots;
  struct ref_slice_u32Pc slot_map;
  const u32Pc *block_header;
  struct ref_slice_u32Pc block_offsets;
  struct slice_BlockRefPc blocks;
  struct ref_slice_BufferInfoPc buffer_infos;
  struct ref_slice_u32Pc vbuff_order;
  struct ref_slice_u32Pc ibuff_order;
  struct IndexMap_u32_______VBuffInfoPc vbuffs;
  struct IndexMap_u32_______IBuffInfoPc ibuffs;
  struct IndexMap_u32__MatRefPc mats;
  struct Option_HkConstraintRefPc hk_constraint;
  struct ref_slice_HkConstraintDataPc hk_constraint_datas;
  struct slice_ShapeRefPc shapes;
  struct ModelDataRefPc data;
} ModelRefPc;

/**
 *gen_ffi:export
 */
typedef struct AnimationInfoPc {
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
} AnimationInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Obj3Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Obj3Pc;

/**
 *gen_ffi:export
 */
typedef struct Obj5HeaderPc {
  u32Pc obj_a_num;
  u32Pc obj_a_offset;
  u32Pc obj_b_num;
  u32Pc obj_b_offset;
} Obj5HeaderPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Obj5ValPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Obj5ValPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_BlocksRefPc {
  uint64_t align;
  uint8_t pad[72];
} Option_BlocksRefPc;

typedef struct AnimationRefPc {
  const struct AnimationInfoPc *info;
  struct ref_slice_u32Pc obj1;
  struct ref_slice_u32Pc obj2;
  struct ref_slice_Obj3Pc obj3;
  struct ref_slice_CrcPc bones;
  const struct Obj5HeaderPc *obj5_header;
  struct ref_slice_Obj5ValPc obj5_a;
  struct ref_slice_Obj5ValPc obj5_b;
  struct Option_BlocksRefPc blocks;
  uintptr_t size;
} AnimationRefPc;

typedef struct DataRefPc {
  struct ref_slice_u8 data;
} DataRefPc;

/**
 *gen_ffi:export
 */
typedef struct EffectInfoPc {
  CrcPc key;
  i32Pc gamemodemask;
  u32Pc offset;
  u32Pc size;
} EffectInfoPc;

typedef struct EffectRefPc {
  const struct EffectInfoPc *info;
  struct GameObjsRefPc vals;
} EffectRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__ref_slice_u16Pc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__ref_slice_u16Pc;

typedef struct LangStringsRefPc {
  struct IndexMap_u32__ref_slice_u16Pc strings;
} LangStringsRefPc;

typedef struct DataRefPc LuaRefPc;

/**
 *gen_ffi:export
 */
typedef struct ObjHeaderPc {
  u32Pc layer;
  CrcPc key;
  u16Pc size;
  u16Pc z3;
  u32Pc z4;
} ObjHeaderPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__BaseTypeRefPc {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__BaseTypeRefPc;

typedef struct ObjRefPc {
  const struct ObjHeaderPc *header;
  struct IndexMap_u32__BaseTypeRefPc fields;
} ObjRefPc;

/**
 *gen_ffi:export
 */
typedef struct RadiosityValsInfoPc {
  u32Pc guid;
  u32Pc num;
  u32Pc offset;
} RadiosityValsInfoPc;

typedef struct RadiosityValsRefPc {
  const struct RadiosityValsInfoPc *info;
  struct ref_slice_i32Pc offs;
} RadiosityValsRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SSAValPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SSAValPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_ref_slice_u16Pc {
  uint64_t align;
  uint8_t pad[8];
} slice_ref_slice_u16Pc;

typedef struct SSARefPc {
  struct ref_slice_SSAValPc vals;
  struct slice_ref_slice_u16Pc strings;
} SSARefPc;

/**
 *gen_ffi:export
 */
typedef struct TextureInfoPc {
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
} TextureInfoPc;

typedef struct TextureRefPc {
  const struct TextureInfoPc *info;
  const struct CompressedDataRef *data0;
  const struct CompressedDataRef *data1;
} TextureRefPc;

/**
 *gen_ffi:export
 */
typedef struct TypeHeaderPc {
  CrcPc key;
  u32Pc size;
  u32Pc fields;
} TypeHeaderPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_TypeFieldPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_TypeFieldPc;

typedef struct TypeRefPc {
  const struct TypeHeaderPc *header;
  struct ref_slice_TypeFieldPc fields;
} TypeRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_FoliageRefPc {
  uint64_t align;
  uint8_t pad[8];
} slice_FoliageRefPc;

/**
 *gen_ffi:export
 */
typedef struct Vector2Pc {
  f32Pc x;
  f32Pc y;
} Vector2Pc;

/**
 *gen_ffi:export
 */
typedef struct Vector4Pc {
  f32Pc x;
  f32Pc y;
  f32Pc z;
  f32Pc w;
} Vector4Pc;

/**
 *gen_ffi:export
 */
typedef struct Matrix4x4Pc {
  struct Vector4Pc x;
  struct Vector4Pc y;
  struct Vector4Pc z;
  struct Vector4Pc w;
} Matrix4x4Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_U32Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_U32Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Vector4Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Vector4Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_WeightPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_WeightPc;

enum BaseTypeRefPc_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
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
};
#if __STDC_VERSION__ >= 202311L
typedef enum BaseTypeRefPc_Tag BaseTypeRefPc_Tag;
#else
typedef uint8_t BaseTypeRefPc_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct BaseTypeRefPc {
  BaseTypeRefPc_Tag tag;
  union {
    struct {
      const CrcPc *crc;
    };
    struct {
      const u32Pc *guid;
    };
    struct {
      const u32Pc *color;
    };
    struct {
      const struct Vector2Pc *vector2;
    };
    struct {
      const struct Vector3Pc *vector3;
    };
    struct {
      const struct Vector4Pc *vector4;
    };
    struct {
      const struct Matrix4x4Pc *matrix4x4;
    };
    struct {
      const f32Pc *float_;
    };
    struct {
      const i32Pc *int_;
    };
    struct {
      const u32Pc *bool_;
    };
    struct {
      struct string string;
    };
    struct {
      struct slice_string string_list;
    };
    struct {
      struct ref_slice_U32Pc object_list;
    };
    struct {
      struct ref_slice_Vector4Pc node_list;
    };
    struct {
      struct ref_slice_i32Pc int_list;
    };
    struct {
      struct ref_slice_U32Pc crc_list;
    };
    struct {
      struct ref_slice_WeightPc weight_list;
    };
    struct {
      struct ref_slice_Matrix4x4Pc matrix_list;
    };
  };
} BaseTypeRefPc;

/**
 *gen_ffi:export
 */
typedef struct BlockHeader1Pc {
  u32Pc a;
  u32Pc b;
  u32Pc unk_2;
  u32Pc unk_3;
} BlockHeader1Pc;

/**
 *gen_ffi:export
 */
typedef struct BlockHeader2Pc {
  u32Pc n;
  f32Pc unk_1;
  f32Pc unk_2;
  u32Pc unk_3;
  u32Pc unk_4;
} BlockHeader2Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BlockValAPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BlockValAPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BlockValBPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BlockValBPc;

typedef struct BlockRefPc {
  const struct BlockHeader1Pc *info1;
  const struct BlockHeader2Pc *info2;
  struct ref_slice_BlockValAPc vals_a;
  struct ref_slice_BlockValAPc vals_b;
  struct ref_slice_BlockValBPc vals_c;
  struct ref_slice_u8 pad;
} BlockRefPc;

/**
 *gen_ffi:export
 */
typedef struct ShapeInfoPc {
  u32Pc offset;
  u32Pc kind;
  u32Pc unk_2;
  f32Pc unk_3;
  f32Pc unk_4;
  f32Pc unk_5;
  struct Vector3Pc translation;
  struct Vector4Pc rotation;
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
} ShapeInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_ShapeExtraRefPc {
  uint64_t align;
  uint8_t pad[32];
} Option_ShapeExtraRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_HkShapeRefPc {
  uint64_t align;
  uint8_t pad[8];
} slice_HkShapeRefPc;

typedef struct ShapeRefPc {
  const struct ShapeInfoPc *info;
  struct Option_ShapeExtraRefPc extra;
  struct slice_HkShapeRefPc hk_shapes;
} ShapeRefPc;

/**
 *gen_ffi:export
 */
typedef struct BoxShapePc {
  struct Vector4Pc translation;
  struct Vector4Pc rotation;
  u32Pc kind;
  CrcPc key;
  struct Vector3Pc half_extents;
  u32Pc unk_13;
  u32Pc unk_14;
  f32Pc unk_15;
  f32Pc unk_16;
  f32Pc unk_17;
  f32Pc unk_18;
  f32Pc unk_19;
} BoxShapePc;

/**
 *gen_ffi:export
 */
typedef struct SphereShapePc {
  struct Vector4Pc translation;
  struct Vector4Pc rotation;
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
} SphereShapePc;

/**
 *gen_ffi:export
 */
typedef struct CapsuleShapePc {
  struct Vector4Pc translation;
  struct Vector4Pc rotation;
  u32Pc kind;
  CrcPc key;
  struct Vector3Pc point1;
  struct Vector3Pc point2;
  f32Pc radius;
  f32Pc unk_17;
  f32Pc unk_18;
  f32Pc unk_19;
} CapsuleShapePc;

/**
 *gen_ffi:export
 */
typedef struct CylinderShapePc {
  struct Vector4Pc translation;
  struct Vector4Pc rotation;
  u32Pc kind;
  CrcPc key;
  struct Vector3Pc point1;
  struct Vector3Pc point2;
  f32Pc radius;
  f32Pc unk_17;
  f32Pc unk_18;
  f32Pc unk_19;
} CylinderShapePc;

/**
 *gen_ffi:export
 */
typedef struct ConvexVerticesInfoPc {
  struct Vector4Pc translation;
  struct Vector4Pc rotation;
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
} ConvexVerticesInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Vector3Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Vector3Pc;

typedef struct ConvexVerticesRefPc {
  const struct ConvexVerticesInfoPc *info;
  struct ref_slice_Vector4Pc norms;
  struct ref_slice_Vector3Pc verts;
} ConvexVerticesRefPc;

/**
 *gen_ffi:export
 */
typedef struct BVTreeMeshInfoPc {
  struct Vector4Pc translation;
  struct Vector4Pc rotation;
  u32Pc kind;
  CrcPc key;
  struct Vector3Pc offset;
  f32Pc tree_scale;
  u32Pc tree_size;
  u32Pc tree_offset;
  u32Pc vert_num;
  u32Pc verts_offset;
  u32Pc tri_num;
  u32Pc inds_offset;
} BVTreeMeshInfoPc;

typedef struct BVTreeMeshRefPc {
  const struct BVTreeMeshInfoPc *info;
  struct ref_slice_u8 tree;
  struct ref_slice_Vector3Pc verts;
  struct ref_slice_u16Pc inds;
} BVTreeMeshRefPc;

/**
 *gen_ffi:export
 */
typedef struct HkShapeInfoPc {
  struct Vector4Pc unk_0;
  struct Vector4Pc unk_4;
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
} HkShapeInfoPc;

enum HkShapeRefPc_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  HkShapeRefPc_Box,
  HkShapeRefPc_Sphere,
  HkShapeRefPc_Capsule,
  HkShapeRefPc_Cylinder,
  HkShapeRefPc_ConvexVertices,
  HkShapeRefPc_BVTreeMesh,
  HkShapeRefPc_Unknown,
};
#if __STDC_VERSION__ >= 202311L
typedef enum HkShapeRefPc_Tag HkShapeRefPc_Tag;
#else
typedef uint8_t HkShapeRefPc_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct HkShapeRefPc {
  HkShapeRefPc_Tag tag;
  union {
    struct {
      const struct BoxShapePc *box;
    };
    struct {
      const struct SphereShapePc *sphere;
    };
    struct {
      const struct CapsuleShapePc *capsule;
    };
    struct {
      const struct CylinderShapePc *cylinder;
    };
    struct {
      struct ConvexVerticesRefPc convex_vertices;
    };
    struct {
      struct BVTreeMeshRefPc bv_tree_mesh;
    };
    struct {
      const struct HkShapeInfoPc *unknown;
    };
  };
} HkShapeRefPc;

/**
 *gen_ffi:export
 */
typedef struct FoliageInfoPc {
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
  struct Vector4Pc color;
  f32Pc lod1a;
  f32Pc lod1b;
  f32Pc lod2a;
  f32Pc lod2b;
  f32Pc lod_max;
} FoliageInfoPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_FoliageValPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_FoliageValPc;

typedef struct FoliageRefPc {
  const struct FoliageInfoPc *info;
  struct ref_slice_FoliageValPc vals;
} FoliageRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_BlockValARefPc {
  uint64_t align;
  uint8_t pad[8];
} slice_BlockValARefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_Obj1RefPc {
  uint64_t align;
  uint8_t pad[8];
} slice_Obj1RefPc;

typedef struct BlockValRefPc {
  struct slice_BlockValARefPc vals_a;
  struct slice_Obj1RefPc vals_b;
} BlockValRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_BlockValRefPc {
  uint64_t align;
  uint8_t pad[8];
} slice_BlockValRefPc;

/**
 *gen_ffi:export
 */
typedef struct CrowdItemHeaderPc {
  CrcPc key;
  CrcPc key_main;
  CrcPc key_right;
  CrcPc key_left;
  f32Pc unk_4;
  u32Pc animation_num;
  u32Pc instance_num;
} CrowdItemHeaderPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_CrowdValPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_CrowdValPc;

typedef struct CrowdItemRefPc {
  const struct CrowdItemHeaderPc *header;
  struct ref_slice_CrcPc animations;
  struct ref_slice_CrowdValPc instances;
} CrowdItemRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_CrowdItemRefPc {
  uint64_t align;
  uint8_t pad[8];
} slice_CrowdItemRefPc;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintBoneRefPc {
  struct string name;
  u32Pc start;
  u32Pc val;
} HkConstraintBoneRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_HkConstraintBoneRefPc {
  uint64_t align;
  uint8_t pad[8];
} slice_HkConstraintBoneRefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_f32Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_f32Pc;

enum AnimVals1RefPc_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  AnimVals1RefPc_Type1,
  AnimVals1RefPc_Type2,
  AnimVals1RefPc_Type3,
  AnimVals1RefPc_Type4,
};
#if __STDC_VERSION__ >= 202311L
typedef enum AnimVals1RefPc_Tag AnimVals1RefPc_Tag;
#else
typedef uint8_t AnimVals1RefPc_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct AnimVals1RefPc {
  AnimVals1RefPc_Tag tag;
  union {
    struct {
      struct ref_slice_u8 type1;
    };
    struct {
      struct ref_slice_u16Pc type2;
    };
    struct {
      struct ref_slice_u16Pc type3;
    };
    struct {
      struct ref_slice_u16Pc type4;
    };
  };
} AnimVals1RefPc;

typedef struct Obj1RefPc {
  uint8_t flags;
  uint8_t s2;
  u16Pc s1;
  struct ref_slice_u8 data;
  struct ref_slice_f32Pc vals_a;
  struct AnimVals1RefPc vals;
  uintptr_t size;
} Obj1RefPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationPolar32Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationPolar32Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationThreeComp40Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationThreeComp40Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationThreeComp48Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationThreeComp48Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationThreeComp24Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationThreeComp24Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationStraight16Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationStraight16Pc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationUncompressedPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationUncompressedPc;

enum RotationQuantizationRefPc_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  RotationQuantizationRefPc_Polar32,
  RotationQuantizationRefPc_ThreeComp40,
  RotationQuantizationRefPc_ThreeComp48,
  RotationQuantizationRefPc_ThreeComp24,
  RotationQuantizationRefPc_Straight16,
  RotationQuantizationRefPc_Uncompressed,
};
#if __STDC_VERSION__ >= 202311L
typedef enum RotationQuantizationRefPc_Tag RotationQuantizationRefPc_Tag;
#else
typedef uint8_t RotationQuantizationRefPc_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct RotationQuantizationRefPc {
  RotationQuantizationRefPc_Tag tag;
  union {
    struct {
      struct ref_slice_RotationPolar32Pc polar32;
    };
    struct {
      struct ref_slice_RotationThreeComp40Pc three_comp40;
    };
    struct {
      struct ref_slice_RotationThreeComp48Pc three_comp48;
    };
    struct {
      struct ref_slice_RotationThreeComp24Pc three_comp24;
    };
    struct {
      struct ref_slice_RotationStraight16Pc straight16;
    };
    struct {
      struct ref_slice_RotationUncompressedPc uncompressed;
    };
  };
} RotationQuantizationRefPc;

typedef struct Obj2RefPc {
  uint8_t flags;
  uint8_t s2;
  u16Pc s1;
  struct ref_slice_u8 data;
  struct RotationQuantizationRefPc vals;
  uintptr_t size;
} Obj2RefPc;

typedef struct BlockValARefPc {
  struct Obj1RefPc a;
  struct Obj2RefPc b;
  struct Obj1RefPc c;
} BlockValARefPc;

/**
 *gen_ffi:export
 */
typedef struct BufferInfoPc {
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
} BufferInfoPc;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintDataPc {
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
} HkConstraintDataPc;

/**
 *gen_ffi:export
 */
typedef struct Key2Pc {
  CrcPc key;
  u32Pc val;
} Key2Pc;

/**
 *gen_ffi:export
 */
typedef struct BlockValAPc {
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
} BlockValAPc;

/**
 *gen_ffi:export
 */
typedef struct BlockValBPc {
  u16Pc unk_0;
  u16Pc unk_1;
  f32Pc unk_2;
  f32Pc unk_3;
  f32Pc unk_4;
  f32Pc unk_5;
} BlockValBPc;

/**
 *gen_ffi:export
 */
typedef struct AnimationBlockInfoPc {
  CrcPc key;
  u32Pc guid;
  CrcPc key_name;
  u32Pc offset;
  u32Pc size;
  u32Pc size_comp;
  u32Pc unk_6;
  u32Pc unk_7;
  u32Pc unk_8;
} AnimationBlockInfoPc;

/**
 *gen_ffi:export
 */
typedef struct AssetHandlePc {
  CrcPc key;
  u32Pc offset;
  u32Pc size;
  u32Pc size_comp;
  u32Pc kind;
} AssetHandlePc;

typedef uint32_t U32LE;

typedef U32LE U32Pc;

typedef int32_t I32LE;

typedef I32LE I32Pc;

typedef struct BlockAValPc {
  U32Pc unk_0;
  I32Pc gamemodemask;
  U32Pc key;
  U32Pc unk_3;
  U32Pc unk_4;
  U32Pc unk_5;
  U32Pc unk_6;
} BlockAValPc;

/**
 *gen_ffi:export
 */
typedef struct GFXBlockInfoPc {
  CrcPc key;
  u32Pc offset;
  u32Pc size;
} GFXBlockInfoPc;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintInfoPc {
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
} HkConstraintInfoPc;

/**
 *gen_ffi:export
 */
typedef struct Obj0Pc {
  u32Pc unk_0;
  CrcPc key;
} Obj0Pc;

/**
 *gen_ffi:export
 */
typedef struct ObjAPc {
  CrcPc key;
  u32Pc unk_1;
  u32Pc size;
  u32Pc size_comp;
  u32Pc unk_4;
  u32Pc kind;
} ObjAPc;

/**
 *gen_ffi:export
 */
typedef struct PFieldInfoPc {
  u32Pc link_guid;
  u32Pc gamemode_guid;
  u32Pc width;
  u32Pc height;
  u32Pc offset;
} PFieldInfoPc;

/**
 *gen_ffi:export
 */
typedef struct StringKeysValPc {
  CrcPc key;
  u32Pc offset;
} StringKeysValPc;

/**
 *gen_ffi:export
 */
typedef struct SubBlocksBlockHeaderPc {
  CrcPc key;
  u32Pc offset;
  u32Pc size;
} SubBlocksBlockHeaderPc;

/**
 *gen_ffi:export
 */
typedef struct Obj3Pc {
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
} Obj3Pc;

/**
 *gen_ffi:export
 */
typedef struct Obj5ValPc {
  f32Pc unk_0;
  f32Pc unk_1;
  f32Pc unk_2;
  f32Pc unk_3;
  f32Pc unk_4;
  f32Pc unk_5;
  f32Pc unk_6;
} Obj5ValPc;

/**
 *gen_ffi:export
 */
typedef struct SSAValPc {
  f32Pc t_start;
  f32Pc t_end;
  u32Pc unk_2;
  u32Pc unk_3;
  u32Pc off;
} SSAValPc;

/**
 *gen_ffi:export
 */
typedef struct TypeFieldPc {
  CrcPc key;
  CrcPc kind;
  u32Pc offset;
} TypeFieldPc;

typedef int16_t i16_le;

typedef i16_le i16Pc;

/**
 *gen_ffi:export
 */
typedef struct FoliageValPc {
  u16Pc height;
  u16Pc var_mask;
  i16Pc slope_x;
  i16Pc slope_z;
} FoliageValPc;

/**
 *gen_ffi:export
 */
typedef struct WeightPc {
  u32Pc x;
  u8Pc a;
  u8Pc b;
  u8Pc c;
  u8Pc d;
} WeightPc;

/**
 *gen_ffi:export
 */
typedef struct AtlasUVValPc {
  CrcPc key;
  struct Vector4Pc vals;
} AtlasUVValPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AtlasUVValPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AtlasUVValPc;

/**
 *gen_ffi:export
 */
typedef struct SprayInstancePc {
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
} SprayInstancePc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SprayInstancePc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SprayInstancePc;

/**
 *gen_ffi:export
 */
typedef struct SprayValPc {
  struct Vector3Pc position;
  f32Pc scale;
  u16Pc instance;
  u16Pc rotation;
} SprayValPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SprayValPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SprayValPc;

/**
 *gen_ffi:export
 */
typedef struct TRSPc {
  struct Vector4Pc translation;
  struct Vector4Pc rotation;
  struct Vector4Pc scale;
} TRSPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_TRSPc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_TRSPc;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_i16Pc {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_i16Pc;

/**
 *gen_ffi:export
 */
typedef struct CrowdValPc {
  struct Vector3Pc position;
  f32Pc rotation;
  f32Pc lod;
} CrowdValPc;

/**
 *gen_ffi:export
 */
typedef struct RotationPolar32Pc {
  u32Pc a;
} RotationPolar32Pc;

/**
 *gen_ffi:export
 */
typedef struct RotationStraight16Pc {
  u8Pc a;
  u8Pc b;
} RotationStraight16Pc;

/**
 *gen_ffi:export
 */
typedef struct RotationThreeComp24Pc {
  u8Pc a;
  u8Pc b;
  u8Pc c;
} RotationThreeComp24Pc;

/**
 *gen_ffi:export
 */
typedef struct RotationThreeComp40Pc {
  u8Pc a;
  u8Pc b;
  u8Pc c;
  u8Pc d;
  u8Pc e;
} RotationThreeComp40Pc;

/**
 *gen_ffi:export
 */
typedef struct RotationThreeComp48Pc {
  u16Pc a;
  u16Pc b;
  u16Pc c;
} RotationThreeComp48Pc;

/**
 *gen_ffi:export
 */
typedef struct RotationUncompressedPc {
  f32Pc a;
  f32Pc b;
  f32Pc c;
  f32Pc d;
} RotationUncompressedPc;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintRefPc {
  const struct HkConstraintInfoPc *info;
  struct ref_slice_i16Pc bone_parents;
  struct slice_HkConstraintBoneRefPc bone_names;
  struct ref_slice_u32Pc name_offsets;
  struct ref_slice_TRSPc bone_transforms;
  struct ref_slice_u32Pc bones;
  struct ref_slice_Key2Pc bones_order;
  struct ref_slice_f32Pc vals2;
} HkConstraintRefPc;

/**
 *gen_ffi:export
 */
typedef struct ShapeExtraInfoPc {
  u32Pc size;
  f32Pc scale;
  f32Pc a;
  f32Pc b;
} ShapeExtraInfoPc;

typedef struct ShapeExtraRefPc {
  const struct ShapeExtraInfoPc *info;
  struct ref_slice_u32Pc offs;
  struct ref_slice_u8 data;
} ShapeExtraRefPc;

typedef struct AtlasUVRefPc {
  struct ref_slice_AtlasUVValPc vals;
} AtlasUVRefPc;

typedef struct BlocksRefPc {
  struct ref_slice_u32Pc block_starts;
  struct ref_slice_u32Pc block_starts2;
  struct ref_slice_u32Pc obj_c3;
  struct ref_slice_u32Pc obj_c4;
  struct slice_BlockValRefPc blocks;
} BlocksRefPc;

/**
 *gen_ffi:export
 */
typedef struct CrowdHeaderPc {
  u32Pc const0x65;
  u32Pc n;
} CrowdHeaderPc;

typedef struct CrowdRefPc {
  const struct CrowdHeaderPc *header;
  struct ref_slice_u32Pc offs;
  struct slice_CrowdItemRefPc vals;
} CrowdRefPc;

typedef struct DataRefPc PFieldsRefPc;

typedef struct SprayRefPc {
  struct ref_slice_SprayInstancePc instances;
  struct ref_slice_SprayValPc vals;
} SprayRefPc;

/**
 *gen_ffi:export
 */
typedef struct IBuffInfoXbox {
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
} IBuffInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_u16Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_u16Xbox;

enum IndexBufferValsRefXbox_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  IndexBufferValsRefXbox_U16,
  IndexBufferValsRefXbox_U32,
};
#if __STDC_VERSION__ >= 202311L
typedef enum IndexBufferValsRefXbox_Tag IndexBufferValsRefXbox_Tag;
#else
typedef uint8_t IndexBufferValsRefXbox_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct IndexBufferValsRefXbox {
  IndexBufferValsRefXbox_Tag tag;
  union {
    struct {
      struct ref_slice_u16Xbox u16;
    };
    struct {
      struct ref_slice_u32Xbox u32;
    };
  };
} IndexBufferValsRefXbox;

typedef struct IndexBufferRefXbox {
  const struct IBuffInfoXbox *info;
  struct IndexBufferValsRefXbox vals;
} IndexBufferRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__IndexBufferRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__IndexBufferRefXbox;

/**
 *gen_ffi:export
 */
typedef struct VBuffInfoXbox {
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
} VBuffInfoXbox;

typedef struct VertexBufferRefXbox {
  const struct VBuffInfoXbox *info;
  struct IndexMap_VertexUsage__VertexDataIndex offsets;
  uintptr_t size;
  struct ref_slice_u8 data;
} VertexBufferRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__VertexBufferRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__VertexBufferRefXbox;

typedef u32Xbox CrcXbox;

typedef uint64_t u64_be;

typedef u64_be u64Xbox;

typedef uint8_t u8Xbox;

typedef float f32_be;

typedef f32_be f32Xbox;

/**
 *gen_ffi:export
 */
typedef struct MatBaseXbox {
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
} MatBaseXbox;

/**
 *gen_ffi:export
 */
typedef struct Mat1Xbox {
  struct MatBaseXbox base;
} Mat1Xbox;

/**
 *gen_ffi:export
 */
typedef struct MatExtraXbox {
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
} MatExtraXbox;

typedef struct Mat1RefXbox {
  const struct Mat1Xbox *info;
  const struct MatExtraXbox *extra;
} Mat1RefXbox;

/**
 *gen_ffi:export
 */
typedef struct Mat2Xbox {
  struct MatBaseXbox base;
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
} Mat2Xbox;

typedef struct Mat2RefXbox {
  const struct Mat2Xbox *info;
  const struct MatExtraXbox *extra;
} Mat2RefXbox;

/**
 *gen_ffi:export
 */
typedef struct Mat3Xbox {
  struct MatBaseXbox base;
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
} Mat3Xbox;

typedef struct Mat3RefXbox {
  const struct Mat3Xbox *info;
  const struct MatExtraXbox *extra;
} Mat3RefXbox;

/**
 *gen_ffi:export
 */
typedef struct Mat4Xbox {
  struct MatBaseXbox base;
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
} Mat4Xbox;

typedef struct Mat4RefXbox {
  const struct Mat4Xbox *info;
  const struct MatExtraXbox *extra;
} Mat4RefXbox;

enum MatRefXbox_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  MatRefXbox_Mat1,
  MatRefXbox_Mat2,
  MatRefXbox_Mat3,
  MatRefXbox_Mat4,
};
#if __STDC_VERSION__ >= 202311L
typedef enum MatRefXbox_Tag MatRefXbox_Tag;
#else
typedef uint8_t MatRefXbox_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct MatRefXbox {
  MatRefXbox_Tag tag;
  union {
    struct {
      struct Mat1RefXbox mat1;
    };
    struct {
      struct Mat2RefXbox mat2;
    };
    struct {
      struct Mat3RefXbox mat3;
    };
    struct {
      struct Mat4RefXbox mat4;
    };
  };
} MatRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__MatRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__MatRefXbox;

typedef int32_t i32_be;

typedef i32_be i32Xbox;

/**
 *gen_ffi:export
 */
typedef struct Vector3Xbox {
  f32Xbox x;
  f32Xbox y;
  f32Xbox z;
} Vector3Xbox;

/**
 *gen_ffi:export
 */
typedef struct BoundingBoxXbox {
  struct Vector3Xbox center;
  f32Xbox unk_3;
  struct Vector3Xbox half_width;
  f32Xbox unk_7;
} BoundingBoxXbox;

/**
 *gen_ffi:export
 */
typedef struct LodInfoXbox {
  u32Xbox start;
  u32Xbox static_end;
  u32Xbox skinned_end;
  u32Xbox physics_end;
  u32Xbox breakable_end;
} LodInfoXbox;

/**
 *gen_ffi:export
 */
typedef struct ModelInfoXbox {
  CrcXbox key;
  i32Xbox gamemodemask;
  u32Xbox mat_offset;
  u32Xbox buffer_info_offset;
  struct BoundingBoxXbox bounding_box;
  u32Xbox mesh_order_offset;
  struct LodInfoXbox lod0;
  struct LodInfoXbox lod1;
  struct LodInfoXbox lod2;
  struct LodInfoXbox lod3;
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
} ModelInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_CrcXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_CrcXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_i32Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_i32Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Matrix4x4Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Matrix4x4Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BoundingBoxXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BoundingBoxXbox;

typedef struct BonesRefXbox {
  struct ref_slice_CrcXbox names;
  struct ref_slice_i32Xbox parents;
  struct ref_slice_Matrix4x4Xbox transforms;
  struct ref_slice_BoundingBoxXbox bounding_boxes;
} BonesRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Key2Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Key2Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_BlockRefXbox {
  uint64_t align;
  uint8_t pad[8];
} slice_BlockRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32_______VBuffInfoXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32_______VBuffInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32_______IBuffInfoXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32_______IBuffInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_HkConstraintRefXbox {
  uint64_t align;
  uint8_t pad[112];
} Option_HkConstraintRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_ShapeRefXbox {
  uint64_t align;
  uint8_t pad[8];
} slice_ShapeRefXbox;

typedef struct ModelDataRefXbox {
  struct ref_slice_BufferInfoXbox infos;
  struct ref_slice_u32Xbox vbuff_order;
  struct ref_slice_u32Xbox ibuff_order;
  struct IndexMap_u32__VertexBufferRefXbox vertex;
  struct IndexMap_u32__IndexBufferRefXbox index;
  const struct CompressedDataRef *data;
} ModelDataRefXbox;

typedef struct ModelRefXbox {
  const struct ModelInfoXbox *info;
  struct BonesRefXbox bones;
  struct ref_slice_u32Xbox mat_order;
  struct ref_slice_u32Xbox mesh_order;
  struct ref_slice_BoundingBoxXbox mesh_bounding_boxes;
  struct ref_slice_Matrix4x4Xbox skin_binds;
  struct ref_slice_u32Xbox vals_j;
  struct ref_slice_u16Xbox val_k_header;
  struct ref_slice_u32Xbox vals_k;
  struct ref_slice_u32Xbox skin_order;
  struct ref_slice_Key2Xbox slots;
  struct ref_slice_u32Xbox slot_map;
  const u32Xbox *block_header;
  struct ref_slice_u32Xbox block_offsets;
  struct slice_BlockRefXbox blocks;
  struct ref_slice_BufferInfoXbox buffer_infos;
  struct ref_slice_u32Xbox vbuff_order;
  struct ref_slice_u32Xbox ibuff_order;
  struct IndexMap_u32_______VBuffInfoXbox vbuffs;
  struct IndexMap_u32_______IBuffInfoXbox ibuffs;
  struct IndexMap_u32__MatRefXbox mats;
  struct Option_HkConstraintRefXbox hk_constraint;
  struct ref_slice_HkConstraintDataXbox hk_constraint_datas;
  struct slice_ShapeRefXbox shapes;
  struct ModelDataRefXbox data;
} ModelRefXbox;

/**
 *gen_ffi:export
 */
typedef struct AnimationInfoXbox {
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
} AnimationInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Obj3Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Obj3Xbox;

/**
 *gen_ffi:export
 */
typedef struct Obj5HeaderXbox {
  u32Xbox obj_a_num;
  u32Xbox obj_a_offset;
  u32Xbox obj_b_num;
  u32Xbox obj_b_offset;
} Obj5HeaderXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Obj5ValXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Obj5ValXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_BlocksRefXbox {
  uint64_t align;
  uint8_t pad[72];
} Option_BlocksRefXbox;

typedef struct AnimationRefXbox {
  const struct AnimationInfoXbox *info;
  struct ref_slice_u32Xbox obj1;
  struct ref_slice_u32Xbox obj2;
  struct ref_slice_Obj3Xbox obj3;
  struct ref_slice_CrcXbox bones;
  const struct Obj5HeaderXbox *obj5_header;
  struct ref_slice_Obj5ValXbox obj5_a;
  struct ref_slice_Obj5ValXbox obj5_b;
  struct Option_BlocksRefXbox blocks;
  uintptr_t size;
} AnimationRefXbox;

typedef struct DataRefXbox {
  struct ref_slice_u8 data;
} DataRefXbox;

/**
 *gen_ffi:export
 */
typedef struct EffectInfoXbox {
  CrcXbox key;
  i32Xbox gamemodemask;
  u32Xbox offset;
  u32Xbox size;
} EffectInfoXbox;

typedef struct EffectRefXbox {
  const struct EffectInfoXbox *info;
  struct GameObjsRefXbox vals;
} EffectRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__ref_slice_u16Xbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__ref_slice_u16Xbox;

typedef struct LangStringsRefXbox {
  struct IndexMap_u32__ref_slice_u16Xbox strings;
} LangStringsRefXbox;

typedef struct DataRefXbox LuaRefXbox;

/**
 *gen_ffi:export
 */
typedef struct ObjHeaderXbox {
  u32Xbox layer;
  CrcXbox key;
  u16Xbox size;
  u16Xbox z3;
  u32Xbox z4;
} ObjHeaderXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__BaseTypeRefXbox {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__BaseTypeRefXbox;

typedef struct ObjRefXbox {
  const struct ObjHeaderXbox *header;
  struct IndexMap_u32__BaseTypeRefXbox fields;
} ObjRefXbox;

/**
 *gen_ffi:export
 */
typedef struct RadiosityValsInfoXbox {
  u32Xbox guid;
  u32Xbox num;
  u32Xbox offset;
} RadiosityValsInfoXbox;

typedef struct RadiosityValsRefXbox {
  const struct RadiosityValsInfoXbox *info;
  struct ref_slice_i32Xbox offs;
} RadiosityValsRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SSAValXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SSAValXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_ref_slice_u16Xbox {
  uint64_t align;
  uint8_t pad[8];
} slice_ref_slice_u16Xbox;

typedef struct SSARefXbox {
  struct ref_slice_SSAValXbox vals;
  struct slice_ref_slice_u16Xbox strings;
} SSARefXbox;

/**
 *gen_ffi:export
 */
typedef struct TextureInfoXbox {
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
} TextureInfoXbox;

typedef struct TextureRefXbox {
  const struct TextureInfoXbox *info;
  const struct CompressedDataRef *data0;
  const struct CompressedDataRef *data1;
} TextureRefXbox;

/**
 *gen_ffi:export
 */
typedef struct TypeHeaderXbox {
  CrcXbox key;
  u32Xbox size;
  u32Xbox fields;
} TypeHeaderXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_TypeFieldXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_TypeFieldXbox;

typedef struct TypeRefXbox {
  const struct TypeHeaderXbox *header;
  struct ref_slice_TypeFieldXbox fields;
} TypeRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_FoliageRefXbox {
  uint64_t align;
  uint8_t pad[8];
} slice_FoliageRefXbox;

/**
 *gen_ffi:export
 */
typedef struct Vector2Xbox {
  f32Xbox x;
  f32Xbox y;
} Vector2Xbox;

/**
 *gen_ffi:export
 */
typedef struct Vector4Xbox {
  f32Xbox x;
  f32Xbox y;
  f32Xbox z;
  f32Xbox w;
} Vector4Xbox;

/**
 *gen_ffi:export
 */
typedef struct Matrix4x4Xbox {
  struct Vector4Xbox x;
  struct Vector4Xbox y;
  struct Vector4Xbox z;
  struct Vector4Xbox w;
} Matrix4x4Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_U32Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_U32Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Vector4Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Vector4Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_WeightXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_WeightXbox;

enum BaseTypeRefXbox_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
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
};
#if __STDC_VERSION__ >= 202311L
typedef enum BaseTypeRefXbox_Tag BaseTypeRefXbox_Tag;
#else
typedef uint8_t BaseTypeRefXbox_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct BaseTypeRefXbox {
  BaseTypeRefXbox_Tag tag;
  union {
    struct {
      const CrcXbox *crc;
    };
    struct {
      const u32Xbox *guid;
    };
    struct {
      const u32Xbox *color;
    };
    struct {
      const struct Vector2Xbox *vector2;
    };
    struct {
      const struct Vector3Xbox *vector3;
    };
    struct {
      const struct Vector4Xbox *vector4;
    };
    struct {
      const struct Matrix4x4Xbox *matrix4x4;
    };
    struct {
      const f32Xbox *float_;
    };
    struct {
      const i32Xbox *int_;
    };
    struct {
      const u32Xbox *bool_;
    };
    struct {
      struct string string;
    };
    struct {
      struct slice_string string_list;
    };
    struct {
      struct ref_slice_U32Xbox object_list;
    };
    struct {
      struct ref_slice_Vector4Xbox node_list;
    };
    struct {
      struct ref_slice_i32Xbox int_list;
    };
    struct {
      struct ref_slice_U32Xbox crc_list;
    };
    struct {
      struct ref_slice_WeightXbox weight_list;
    };
    struct {
      struct ref_slice_Matrix4x4Xbox matrix_list;
    };
  };
} BaseTypeRefXbox;

/**
 *gen_ffi:export
 */
typedef struct BlockHeader1Xbox {
  u32Xbox a;
  u32Xbox b;
  u32Xbox unk_2;
  u32Xbox unk_3;
} BlockHeader1Xbox;

/**
 *gen_ffi:export
 */
typedef struct BlockHeader2Xbox {
  u32Xbox n;
  f32Xbox unk_1;
  f32Xbox unk_2;
  u32Xbox unk_3;
  u32Xbox unk_4;
} BlockHeader2Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BlockValAXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BlockValAXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BlockValBXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BlockValBXbox;

typedef struct BlockRefXbox {
  const struct BlockHeader1Xbox *info1;
  const struct BlockHeader2Xbox *info2;
  struct ref_slice_BlockValAXbox vals_a;
  struct ref_slice_BlockValAXbox vals_b;
  struct ref_slice_BlockValBXbox vals_c;
  struct ref_slice_u8 pad;
} BlockRefXbox;

/**
 *gen_ffi:export
 */
typedef struct ShapeInfoXbox {
  u32Xbox offset;
  u32Xbox kind;
  u32Xbox unk_2;
  f32Xbox unk_3;
  f32Xbox unk_4;
  f32Xbox unk_5;
  struct Vector3Xbox translation;
  struct Vector4Xbox rotation;
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
} ShapeInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_ShapeExtraRefXbox {
  uint64_t align;
  uint8_t pad[32];
} Option_ShapeExtraRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_HkShapeRefXbox {
  uint64_t align;
  uint8_t pad[8];
} slice_HkShapeRefXbox;

typedef struct ShapeRefXbox {
  const struct ShapeInfoXbox *info;
  struct Option_ShapeExtraRefXbox extra;
  struct slice_HkShapeRefXbox hk_shapes;
} ShapeRefXbox;

/**
 *gen_ffi:export
 */
typedef struct BoxShapeXbox {
  struct Vector4Xbox translation;
  struct Vector4Xbox rotation;
  u32Xbox kind;
  CrcXbox key;
  struct Vector3Xbox half_extents;
  u32Xbox unk_13;
  u32Xbox unk_14;
  f32Xbox unk_15;
  f32Xbox unk_16;
  f32Xbox unk_17;
  f32Xbox unk_18;
  f32Xbox unk_19;
} BoxShapeXbox;

/**
 *gen_ffi:export
 */
typedef struct SphereShapeXbox {
  struct Vector4Xbox translation;
  struct Vector4Xbox rotation;
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
} SphereShapeXbox;

/**
 *gen_ffi:export
 */
typedef struct CapsuleShapeXbox {
  struct Vector4Xbox translation;
  struct Vector4Xbox rotation;
  u32Xbox kind;
  CrcXbox key;
  struct Vector3Xbox point1;
  struct Vector3Xbox point2;
  f32Xbox radius;
  f32Xbox unk_17;
  f32Xbox unk_18;
  f32Xbox unk_19;
} CapsuleShapeXbox;

/**
 *gen_ffi:export
 */
typedef struct CylinderShapeXbox {
  struct Vector4Xbox translation;
  struct Vector4Xbox rotation;
  u32Xbox kind;
  CrcXbox key;
  struct Vector3Xbox point1;
  struct Vector3Xbox point2;
  f32Xbox radius;
  f32Xbox unk_17;
  f32Xbox unk_18;
  f32Xbox unk_19;
} CylinderShapeXbox;

/**
 *gen_ffi:export
 */
typedef struct ConvexVerticesInfoXbox {
  struct Vector4Xbox translation;
  struct Vector4Xbox rotation;
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
} ConvexVerticesInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Vector3Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Vector3Xbox;

typedef struct ConvexVerticesRefXbox {
  const struct ConvexVerticesInfoXbox *info;
  struct ref_slice_Vector4Xbox norms;
  struct ref_slice_Vector3Xbox verts;
} ConvexVerticesRefXbox;

/**
 *gen_ffi:export
 */
typedef struct BVTreeMeshInfoXbox {
  struct Vector4Xbox translation;
  struct Vector4Xbox rotation;
  u32Xbox kind;
  CrcXbox key;
  struct Vector3Xbox offset;
  f32Xbox tree_scale;
  u32Xbox tree_size;
  u32Xbox tree_offset;
  u32Xbox vert_num;
  u32Xbox verts_offset;
  u32Xbox tri_num;
  u32Xbox inds_offset;
} BVTreeMeshInfoXbox;

typedef struct BVTreeMeshRefXbox {
  const struct BVTreeMeshInfoXbox *info;
  struct ref_slice_u8 tree;
  struct ref_slice_Vector3Xbox verts;
  struct ref_slice_u16Xbox inds;
} BVTreeMeshRefXbox;

/**
 *gen_ffi:export
 */
typedef struct HkShapeInfoXbox {
  struct Vector4Xbox unk_0;
  struct Vector4Xbox unk_4;
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
} HkShapeInfoXbox;

enum HkShapeRefXbox_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  HkShapeRefXbox_Box,
  HkShapeRefXbox_Sphere,
  HkShapeRefXbox_Capsule,
  HkShapeRefXbox_Cylinder,
  HkShapeRefXbox_ConvexVertices,
  HkShapeRefXbox_BVTreeMesh,
  HkShapeRefXbox_Unknown,
};
#if __STDC_VERSION__ >= 202311L
typedef enum HkShapeRefXbox_Tag HkShapeRefXbox_Tag;
#else
typedef uint8_t HkShapeRefXbox_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct HkShapeRefXbox {
  HkShapeRefXbox_Tag tag;
  union {
    struct {
      const struct BoxShapeXbox *box;
    };
    struct {
      const struct SphereShapeXbox *sphere;
    };
    struct {
      const struct CapsuleShapeXbox *capsule;
    };
    struct {
      const struct CylinderShapeXbox *cylinder;
    };
    struct {
      struct ConvexVerticesRefXbox convex_vertices;
    };
    struct {
      struct BVTreeMeshRefXbox bv_tree_mesh;
    };
    struct {
      const struct HkShapeInfoXbox *unknown;
    };
  };
} HkShapeRefXbox;

/**
 *gen_ffi:export
 */
typedef struct FoliageInfoXbox {
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
  struct Vector4Xbox color;
  f32Xbox lod1a;
  f32Xbox lod1b;
  f32Xbox lod2a;
  f32Xbox lod2b;
  f32Xbox lod_max;
} FoliageInfoXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_FoliageValXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_FoliageValXbox;

typedef struct FoliageRefXbox {
  const struct FoliageInfoXbox *info;
  struct ref_slice_FoliageValXbox vals;
} FoliageRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_BlockValARefXbox {
  uint64_t align;
  uint8_t pad[8];
} slice_BlockValARefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_Obj1RefXbox {
  uint64_t align;
  uint8_t pad[8];
} slice_Obj1RefXbox;

typedef struct BlockValRefXbox {
  struct slice_BlockValARefXbox vals_a;
  struct slice_Obj1RefXbox vals_b;
} BlockValRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_BlockValRefXbox {
  uint64_t align;
  uint8_t pad[8];
} slice_BlockValRefXbox;

/**
 *gen_ffi:export
 */
typedef struct CrowdItemHeaderXbox {
  CrcXbox key;
  CrcXbox key_main;
  CrcXbox key_right;
  CrcXbox key_left;
  f32Xbox unk_4;
  u32Xbox animation_num;
  u32Xbox instance_num;
} CrowdItemHeaderXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_CrowdValXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_CrowdValXbox;

typedef struct CrowdItemRefXbox {
  const struct CrowdItemHeaderXbox *header;
  struct ref_slice_CrcXbox animations;
  struct ref_slice_CrowdValXbox instances;
} CrowdItemRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_CrowdItemRefXbox {
  uint64_t align;
  uint8_t pad[8];
} slice_CrowdItemRefXbox;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintBoneRefXbox {
  struct string name;
  u32Xbox start;
  u32Xbox val;
} HkConstraintBoneRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_HkConstraintBoneRefXbox {
  uint64_t align;
  uint8_t pad[8];
} slice_HkConstraintBoneRefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_f32Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_f32Xbox;

enum AnimVals1RefXbox_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  AnimVals1RefXbox_Type1,
  AnimVals1RefXbox_Type2,
  AnimVals1RefXbox_Type3,
  AnimVals1RefXbox_Type4,
};
#if __STDC_VERSION__ >= 202311L
typedef enum AnimVals1RefXbox_Tag AnimVals1RefXbox_Tag;
#else
typedef uint8_t AnimVals1RefXbox_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct AnimVals1RefXbox {
  AnimVals1RefXbox_Tag tag;
  union {
    struct {
      struct ref_slice_u8 type1;
    };
    struct {
      struct ref_slice_u16Xbox type2;
    };
    struct {
      struct ref_slice_u16Xbox type3;
    };
    struct {
      struct ref_slice_u16Xbox type4;
    };
  };
} AnimVals1RefXbox;

typedef struct Obj1RefXbox {
  uint8_t flags;
  uint8_t s2;
  u16Xbox s1;
  struct ref_slice_u8 data;
  struct ref_slice_f32Xbox vals_a;
  struct AnimVals1RefXbox vals;
  uintptr_t size;
} Obj1RefXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationPolar32Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationPolar32Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationThreeComp40Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationThreeComp40Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationThreeComp48Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationThreeComp48Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationThreeComp24Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationThreeComp24Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationStraight16Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationStraight16Xbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationUncompressedXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationUncompressedXbox;

enum RotationQuantizationRefXbox_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  RotationQuantizationRefXbox_Polar32,
  RotationQuantizationRefXbox_ThreeComp40,
  RotationQuantizationRefXbox_ThreeComp48,
  RotationQuantizationRefXbox_ThreeComp24,
  RotationQuantizationRefXbox_Straight16,
  RotationQuantizationRefXbox_Uncompressed,
};
#if __STDC_VERSION__ >= 202311L
typedef enum RotationQuantizationRefXbox_Tag RotationQuantizationRefXbox_Tag;
#else
typedef uint8_t RotationQuantizationRefXbox_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct RotationQuantizationRefXbox {
  RotationQuantizationRefXbox_Tag tag;
  union {
    struct {
      struct ref_slice_RotationPolar32Xbox polar32;
    };
    struct {
      struct ref_slice_RotationThreeComp40Xbox three_comp40;
    };
    struct {
      struct ref_slice_RotationThreeComp48Xbox three_comp48;
    };
    struct {
      struct ref_slice_RotationThreeComp24Xbox three_comp24;
    };
    struct {
      struct ref_slice_RotationStraight16Xbox straight16;
    };
    struct {
      struct ref_slice_RotationUncompressedXbox uncompressed;
    };
  };
} RotationQuantizationRefXbox;

typedef struct Obj2RefXbox {
  uint8_t flags;
  uint8_t s2;
  u16Xbox s1;
  struct ref_slice_u8 data;
  struct RotationQuantizationRefXbox vals;
  uintptr_t size;
} Obj2RefXbox;

typedef struct BlockValARefXbox {
  struct Obj1RefXbox a;
  struct Obj2RefXbox b;
  struct Obj1RefXbox c;
} BlockValARefXbox;

/**
 *gen_ffi:export
 */
typedef struct BufferInfoXbox {
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
} BufferInfoXbox;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintDataXbox {
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
} HkConstraintDataXbox;

/**
 *gen_ffi:export
 */
typedef struct Key2Xbox {
  CrcXbox key;
  u32Xbox val;
} Key2Xbox;

/**
 *gen_ffi:export
 */
typedef struct BlockValAXbox {
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
} BlockValAXbox;

/**
 *gen_ffi:export
 */
typedef struct BlockValBXbox {
  u16Xbox unk_0;
  u16Xbox unk_1;
  f32Xbox unk_2;
  f32Xbox unk_3;
  f32Xbox unk_4;
  f32Xbox unk_5;
} BlockValBXbox;

/**
 *gen_ffi:export
 */
typedef struct AnimationBlockInfoXbox {
  CrcXbox key;
  u32Xbox guid;
  CrcXbox key_name;
  u32Xbox offset;
  u32Xbox size;
  u32Xbox size_comp;
  u32Xbox unk_6;
  u32Xbox unk_7;
  u32Xbox unk_8;
} AnimationBlockInfoXbox;

/**
 *gen_ffi:export
 */
typedef struct AssetHandleXbox {
  CrcXbox key;
  u32Xbox offset;
  u32Xbox size;
  u32Xbox size_comp;
  u32Xbox kind;
} AssetHandleXbox;

typedef U32LE U32Xbox;

typedef int32_t I32BE;

typedef I32BE I32Xbox;

typedef struct BlockAValXbox {
  U32Xbox unk_0;
  I32Xbox gamemodemask;
  U32Xbox key;
  U32Xbox unk_3;
  U32Xbox unk_4;
  U32Xbox unk_5;
  U32Xbox unk_6;
} BlockAValXbox;

/**
 *gen_ffi:export
 */
typedef struct GFXBlockInfoXbox {
  CrcXbox key;
  u32Xbox offset;
  u32Xbox size;
} GFXBlockInfoXbox;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintInfoXbox {
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
} HkConstraintInfoXbox;

/**
 *gen_ffi:export
 */
typedef struct Obj0Xbox {
  u32Xbox unk_0;
  CrcXbox key;
} Obj0Xbox;

/**
 *gen_ffi:export
 */
typedef struct ObjAXbox {
  CrcXbox key;
  u32Xbox unk_1;
  u32Xbox size;
  u32Xbox size_comp;
  u32Xbox unk_4;
  u32Xbox kind;
} ObjAXbox;

/**
 *gen_ffi:export
 */
typedef struct PFieldInfoXbox {
  u32Xbox link_guid;
  u32Xbox gamemode_guid;
  u32Xbox width;
  u32Xbox height;
  u32Xbox offset;
} PFieldInfoXbox;

/**
 *gen_ffi:export
 */
typedef struct StringKeysValXbox {
  CrcXbox key;
  u32Xbox offset;
} StringKeysValXbox;

/**
 *gen_ffi:export
 */
typedef struct SubBlocksBlockHeaderXbox {
  CrcXbox key;
  u32Xbox offset;
  u32Xbox size;
} SubBlocksBlockHeaderXbox;

/**
 *gen_ffi:export
 */
typedef struct Obj3Xbox {
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
} Obj3Xbox;

/**
 *gen_ffi:export
 */
typedef struct Obj5ValXbox {
  f32Xbox unk_0;
  f32Xbox unk_1;
  f32Xbox unk_2;
  f32Xbox unk_3;
  f32Xbox unk_4;
  f32Xbox unk_5;
  f32Xbox unk_6;
} Obj5ValXbox;

/**
 *gen_ffi:export
 */
typedef struct SSAValXbox {
  f32Xbox t_start;
  f32Xbox t_end;
  u32Xbox unk_2;
  u32Xbox unk_3;
  u32Xbox off;
} SSAValXbox;

/**
 *gen_ffi:export
 */
typedef struct TypeFieldXbox {
  CrcXbox key;
  CrcXbox kind;
  u32Xbox offset;
} TypeFieldXbox;

typedef int16_t i16_be;

typedef i16_be i16Xbox;

/**
 *gen_ffi:export
 */
typedef struct FoliageValXbox {
  u16Xbox height;
  u16Xbox var_mask;
  i16Xbox slope_x;
  i16Xbox slope_z;
} FoliageValXbox;

/**
 *gen_ffi:export
 */
typedef struct WeightXbox {
  u32Xbox x;
  u8Xbox a;
  u8Xbox b;
  u8Xbox c;
  u8Xbox d;
} WeightXbox;

/**
 *gen_ffi:export
 */
typedef struct AtlasUVValXbox {
  CrcXbox key;
  struct Vector4Xbox vals;
} AtlasUVValXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AtlasUVValXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AtlasUVValXbox;

/**
 *gen_ffi:export
 */
typedef struct SprayInstanceXbox {
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
} SprayInstanceXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SprayInstanceXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SprayInstanceXbox;

/**
 *gen_ffi:export
 */
typedef struct SprayValXbox {
  struct Vector3Xbox position;
  f32Xbox scale;
  u16Xbox instance;
  u16Xbox rotation;
} SprayValXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SprayValXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SprayValXbox;

/**
 *gen_ffi:export
 */
typedef struct TRSXbox {
  struct Vector4Xbox translation;
  struct Vector4Xbox rotation;
  struct Vector4Xbox scale;
} TRSXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_TRSXbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_TRSXbox;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_i16Xbox {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_i16Xbox;

/**
 *gen_ffi:export
 */
typedef struct CrowdValXbox {
  struct Vector3Xbox position;
  f32Xbox rotation;
  f32Xbox lod;
} CrowdValXbox;

/**
 *gen_ffi:export
 */
typedef struct RotationPolar32Xbox {
  u32Xbox a;
} RotationPolar32Xbox;

/**
 *gen_ffi:export
 */
typedef struct RotationStraight16Xbox {
  u8Xbox a;
  u8Xbox b;
} RotationStraight16Xbox;

/**
 *gen_ffi:export
 */
typedef struct RotationThreeComp24Xbox {
  u8Xbox a;
  u8Xbox b;
  u8Xbox c;
} RotationThreeComp24Xbox;

/**
 *gen_ffi:export
 */
typedef struct RotationThreeComp40Xbox {
  u8Xbox a;
  u8Xbox b;
  u8Xbox c;
  u8Xbox d;
  u8Xbox e;
} RotationThreeComp40Xbox;

/**
 *gen_ffi:export
 */
typedef struct RotationThreeComp48Xbox {
  u16Xbox a;
  u16Xbox b;
  u16Xbox c;
} RotationThreeComp48Xbox;

/**
 *gen_ffi:export
 */
typedef struct RotationUncompressedXbox {
  f32Xbox a;
  f32Xbox b;
  f32Xbox c;
  f32Xbox d;
} RotationUncompressedXbox;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintRefXbox {
  const struct HkConstraintInfoXbox *info;
  struct ref_slice_i16Xbox bone_parents;
  struct slice_HkConstraintBoneRefXbox bone_names;
  struct ref_slice_u32Xbox name_offsets;
  struct ref_slice_TRSXbox bone_transforms;
  struct ref_slice_u32Xbox bones;
  struct ref_slice_Key2Xbox bones_order;
  struct ref_slice_f32Xbox vals2;
} HkConstraintRefXbox;

/**
 *gen_ffi:export
 */
typedef struct ShapeExtraInfoXbox {
  u32Xbox size;
  f32Xbox scale;
  f32Xbox a;
  f32Xbox b;
} ShapeExtraInfoXbox;

typedef struct ShapeExtraRefXbox {
  const struct ShapeExtraInfoXbox *info;
  struct ref_slice_u32Xbox offs;
  struct ref_slice_u8 data;
} ShapeExtraRefXbox;

typedef struct AtlasUVRefXbox {
  struct ref_slice_AtlasUVValXbox vals;
} AtlasUVRefXbox;

typedef struct BlocksRefXbox {
  struct ref_slice_u32Xbox block_starts;
  struct ref_slice_u32Xbox block_starts2;
  struct ref_slice_u32Xbox obj_c3;
  struct ref_slice_u32Xbox obj_c4;
  struct slice_BlockValRefXbox blocks;
} BlocksRefXbox;

/**
 *gen_ffi:export
 */
typedef struct CrowdHeaderXbox {
  u32Xbox const0x65;
  u32Xbox n;
} CrowdHeaderXbox;

typedef struct CrowdRefXbox {
  const struct CrowdHeaderXbox *header;
  struct ref_slice_u32Xbox offs;
  struct slice_CrowdItemRefXbox vals;
} CrowdRefXbox;

typedef struct DataRefXbox PFieldsRefXbox;

typedef struct SprayRefXbox {
  struct ref_slice_SprayInstanceXbox instances;
  struct ref_slice_SprayValXbox vals;
} SprayRefXbox;

/**
 *gen_ffi:export
 */
typedef struct IBuffInfoPs3 {
  u32Ps3 unk_0;
  u32Ps3 unk_5;
  u32Ps3 unk_6;
  u32Ps3 vbuff_alt_fmt;
  u32Ps3 unk_8;
  u32Ps3 size;
  u32Ps3 format;
  u32Ps3 unk_7;
  u32Ps3 offset;
} IBuffInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_u16Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_u16Ps3;

enum IndexBufferValsRefPs3_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  IndexBufferValsRefPs3_U16,
  IndexBufferValsRefPs3_U32,
};
#if __STDC_VERSION__ >= 202311L
typedef enum IndexBufferValsRefPs3_Tag IndexBufferValsRefPs3_Tag;
#else
typedef uint8_t IndexBufferValsRefPs3_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct IndexBufferValsRefPs3 {
  IndexBufferValsRefPs3_Tag tag;
  union {
    struct {
      struct ref_slice_u16Ps3 u16;
    };
    struct {
      struct ref_slice_u32Ps3 u32;
    };
  };
} IndexBufferValsRefPs3;

typedef struct IndexBufferRefPs3 {
  const struct IBuffInfoPs3 *info;
  struct IndexBufferValsRefPs3 vals;
} IndexBufferRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__IndexBufferRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__IndexBufferRefPs3;

/**
 *gen_ffi:export
 */
typedef struct VBuffInfoPs3 {
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
} VBuffInfoPs3;

typedef struct VertexBufferRefPs3 {
  const struct VBuffInfoPs3 *info;
  struct IndexMap_VertexUsage__VertexDataIndex offsets;
  uintptr_t size;
  struct ref_slice_u8 data;
} VertexBufferRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__VertexBufferRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__VertexBufferRefPs3;

typedef u32Ps3 CrcPs3;

typedef u64_be u64Ps3;

typedef uint8_t u8Ps3;

typedef f32_be f32Ps3;

/**
 *gen_ffi:export
 */
typedef struct MatBasePs3 {
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
} MatBasePs3;

/**
 *gen_ffi:export
 */
typedef struct Mat1Ps3 {
  struct MatBasePs3 base;
} Mat1Ps3;

/**
 *gen_ffi:export
 */
typedef struct MatExtraPs3 {
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
} MatExtraPs3;

typedef struct Mat1RefPs3 {
  const struct Mat1Ps3 *info;
  const struct MatExtraPs3 *extra;
} Mat1RefPs3;

/**
 *gen_ffi:export
 */
typedef struct Mat2Ps3 {
  struct MatBasePs3 base;
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
} Mat2Ps3;

typedef struct Mat2RefPs3 {
  const struct Mat2Ps3 *info;
  const struct MatExtraPs3 *extra;
} Mat2RefPs3;

/**
 *gen_ffi:export
 */
typedef struct Mat3Ps3 {
  struct MatBasePs3 base;
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
} Mat3Ps3;

typedef struct Mat3RefPs3 {
  const struct Mat3Ps3 *info;
  const struct MatExtraPs3 *extra;
} Mat3RefPs3;

/**
 *gen_ffi:export
 */
typedef struct Mat4Ps3 {
  struct MatBasePs3 base;
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
} Mat4Ps3;

typedef struct Mat4RefPs3 {
  const struct Mat4Ps3 *info;
  const struct MatExtraPs3 *extra;
} Mat4RefPs3;

enum MatRefPs3_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  MatRefPs3_Mat1,
  MatRefPs3_Mat2,
  MatRefPs3_Mat3,
  MatRefPs3_Mat4,
};
#if __STDC_VERSION__ >= 202311L
typedef enum MatRefPs3_Tag MatRefPs3_Tag;
#else
typedef uint8_t MatRefPs3_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct MatRefPs3 {
  MatRefPs3_Tag tag;
  union {
    struct {
      struct Mat1RefPs3 mat1;
    };
    struct {
      struct Mat2RefPs3 mat2;
    };
    struct {
      struct Mat3RefPs3 mat3;
    };
    struct {
      struct Mat4RefPs3 mat4;
    };
  };
} MatRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__MatRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__MatRefPs3;

typedef i32_be i32Ps3;

/**
 *gen_ffi:export
 */
typedef struct Vector3Ps3 {
  f32Ps3 x;
  f32Ps3 y;
  f32Ps3 z;
} Vector3Ps3;

/**
 *gen_ffi:export
 */
typedef struct BoundingBoxPs3 {
  struct Vector3Ps3 center;
  f32Ps3 unk_3;
  struct Vector3Ps3 half_width;
  f32Ps3 unk_7;
} BoundingBoxPs3;

/**
 *gen_ffi:export
 */
typedef struct LodInfoPs3 {
  u32Ps3 start;
  u32Ps3 static_end;
  u32Ps3 skinned_end;
  u32Ps3 physics_end;
  u32Ps3 breakable_end;
} LodInfoPs3;

/**
 *gen_ffi:export
 */
typedef struct ModelInfoPs3 {
  CrcPs3 key;
  i32Ps3 gamemodemask;
  u32Ps3 mat_offset;
  u32Ps3 buffer_info_offset;
  struct BoundingBoxPs3 bounding_box;
  u32Ps3 mesh_order_offset;
  struct LodInfoPs3 lod0;
  struct LodInfoPs3 lod1;
  struct LodInfoPs3 lod2;
  struct LodInfoPs3 lod3;
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
} ModelInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_CrcPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_CrcPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_i32Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_i32Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Matrix4x4Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Matrix4x4Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BoundingBoxPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BoundingBoxPs3;

typedef struct BonesRefPs3 {
  struct ref_slice_CrcPs3 names;
  struct ref_slice_i32Ps3 parents;
  struct ref_slice_Matrix4x4Ps3 transforms;
  struct ref_slice_BoundingBoxPs3 bounding_boxes;
} BonesRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Key2Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Key2Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_BlockRefPs3 {
  uint64_t align;
  uint8_t pad[8];
} slice_BlockRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32_______VBuffInfoPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32_______VBuffInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32_______IBuffInfoPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32_______IBuffInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_HkConstraintRefPs3 {
  uint64_t align;
  uint8_t pad[112];
} Option_HkConstraintRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_ShapeRefPs3 {
  uint64_t align;
  uint8_t pad[8];
} slice_ShapeRefPs3;

typedef struct ModelDataRefPs3 {
  struct ref_slice_BufferInfoPs3 infos;
  struct ref_slice_u32Ps3 vbuff_order;
  struct ref_slice_u32Ps3 ibuff_order;
  struct IndexMap_u32__VertexBufferRefPs3 vertex;
  struct IndexMap_u32__IndexBufferRefPs3 index;
  const struct CompressedDataRef *data;
} ModelDataRefPs3;

typedef struct ModelRefPs3 {
  const struct ModelInfoPs3 *info;
  struct BonesRefPs3 bones;
  struct ref_slice_u32Ps3 mat_order;
  struct ref_slice_u32Ps3 mesh_order;
  struct ref_slice_BoundingBoxPs3 mesh_bounding_boxes;
  struct ref_slice_Matrix4x4Ps3 skin_binds;
  struct ref_slice_u32Ps3 vals_j;
  struct ref_slice_u16Ps3 val_k_header;
  struct ref_slice_u32Ps3 vals_k;
  struct ref_slice_u32Ps3 skin_order;
  struct ref_slice_Key2Ps3 slots;
  struct ref_slice_u32Ps3 slot_map;
  const u32Ps3 *block_header;
  struct ref_slice_u32Ps3 block_offsets;
  struct slice_BlockRefPs3 blocks;
  struct ref_slice_BufferInfoPs3 buffer_infos;
  struct ref_slice_u32Ps3 vbuff_order;
  struct ref_slice_u32Ps3 ibuff_order;
  struct IndexMap_u32_______VBuffInfoPs3 vbuffs;
  struct IndexMap_u32_______IBuffInfoPs3 ibuffs;
  struct IndexMap_u32__MatRefPs3 mats;
  struct Option_HkConstraintRefPs3 hk_constraint;
  struct ref_slice_HkConstraintDataPs3 hk_constraint_datas;
  struct slice_ShapeRefPs3 shapes;
  struct ModelDataRefPs3 data;
} ModelRefPs3;

/**
 *gen_ffi:export
 */
typedef struct AnimationInfoPs3 {
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
} AnimationInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Obj3Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Obj3Ps3;

/**
 *gen_ffi:export
 */
typedef struct Obj5HeaderPs3 {
  u32Ps3 obj_a_num;
  u32Ps3 obj_a_offset;
  u32Ps3 obj_b_num;
  u32Ps3 obj_b_offset;
} Obj5HeaderPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Obj5ValPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Obj5ValPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_BlocksRefPs3 {
  uint64_t align;
  uint8_t pad[72];
} Option_BlocksRefPs3;

typedef struct AnimationRefPs3 {
  const struct AnimationInfoPs3 *info;
  struct ref_slice_u32Ps3 obj1;
  struct ref_slice_u32Ps3 obj2;
  struct ref_slice_Obj3Ps3 obj3;
  struct ref_slice_CrcPs3 bones;
  const struct Obj5HeaderPs3 *obj5_header;
  struct ref_slice_Obj5ValPs3 obj5_a;
  struct ref_slice_Obj5ValPs3 obj5_b;
  struct Option_BlocksRefPs3 blocks;
  uintptr_t size;
} AnimationRefPs3;

typedef struct DataRefPs3 {
  struct ref_slice_u8 data;
} DataRefPs3;

/**
 *gen_ffi:export
 */
typedef struct EffectInfoPs3 {
  CrcPs3 key;
  i32Ps3 gamemodemask;
  u32Ps3 offset;
  u32Ps3 size;
} EffectInfoPs3;

typedef struct EffectRefPs3 {
  const struct EffectInfoPs3 *info;
  struct GameObjsRefPs3 vals;
} EffectRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__ref_slice_u16Ps3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__ref_slice_u16Ps3;

typedef struct LangStringsRefPs3 {
  struct IndexMap_u32__ref_slice_u16Ps3 strings;
} LangStringsRefPs3;

typedef struct DataRefPs3 LuaRefPs3;

/**
 *gen_ffi:export
 */
typedef struct ObjHeaderPs3 {
  u32Ps3 layer;
  CrcPs3 key;
  u16Ps3 size;
  u16Ps3 z3;
  u32Ps3 z4;
} ObjHeaderPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct IndexMap_u32__BaseTypeRefPs3 {
  uint64_t align;
  uint8_t pad[64];
} IndexMap_u32__BaseTypeRefPs3;

typedef struct ObjRefPs3 {
  const struct ObjHeaderPs3 *header;
  struct IndexMap_u32__BaseTypeRefPs3 fields;
} ObjRefPs3;

/**
 *gen_ffi:export
 */
typedef struct RadiosityValsInfoPs3 {
  u32Ps3 guid;
  u32Ps3 num;
  u32Ps3 offset;
} RadiosityValsInfoPs3;

typedef struct RadiosityValsRefPs3 {
  const struct RadiosityValsInfoPs3 *info;
  struct ref_slice_i32Ps3 offs;
} RadiosityValsRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SSAValPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SSAValPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_ref_slice_u16Ps3 {
  uint64_t align;
  uint8_t pad[8];
} slice_ref_slice_u16Ps3;

typedef struct SSARefPs3 {
  struct ref_slice_SSAValPs3 vals;
  struct slice_ref_slice_u16Ps3 strings;
} SSARefPs3;

/**
 *gen_ffi:export
 */
typedef struct TextureInfoPs3 {
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
} TextureInfoPs3;

typedef struct TextureRefPs3 {
  const struct TextureInfoPs3 *info;
  const struct CompressedDataRef *data0;
  const struct CompressedDataRef *data1;
} TextureRefPs3;

/**
 *gen_ffi:export
 */
typedef struct TypeHeaderPs3 {
  CrcPs3 key;
  u32Ps3 size;
  u32Ps3 fields;
} TypeHeaderPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_TypeFieldPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_TypeFieldPs3;

typedef struct TypeRefPs3 {
  const struct TypeHeaderPs3 *header;
  struct ref_slice_TypeFieldPs3 fields;
} TypeRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_FoliageRefPs3 {
  uint64_t align;
  uint8_t pad[8];
} slice_FoliageRefPs3;

/**
 *gen_ffi:export
 */
typedef struct Vector2Ps3 {
  f32Ps3 x;
  f32Ps3 y;
} Vector2Ps3;

/**
 *gen_ffi:export
 */
typedef struct Vector4Ps3 {
  f32Ps3 x;
  f32Ps3 y;
  f32Ps3 z;
  f32Ps3 w;
} Vector4Ps3;

/**
 *gen_ffi:export
 */
typedef struct Matrix4x4Ps3 {
  struct Vector4Ps3 x;
  struct Vector4Ps3 y;
  struct Vector4Ps3 z;
  struct Vector4Ps3 w;
} Matrix4x4Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_U32Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_U32Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Vector4Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Vector4Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_WeightPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_WeightPs3;

enum BaseTypeRefPs3_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
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
};
#if __STDC_VERSION__ >= 202311L
typedef enum BaseTypeRefPs3_Tag BaseTypeRefPs3_Tag;
#else
typedef uint8_t BaseTypeRefPs3_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct BaseTypeRefPs3 {
  BaseTypeRefPs3_Tag tag;
  union {
    struct {
      const CrcPs3 *crc;
    };
    struct {
      const u32Ps3 *guid;
    };
    struct {
      const u32Ps3 *color;
    };
    struct {
      const struct Vector2Ps3 *vector2;
    };
    struct {
      const struct Vector3Ps3 *vector3;
    };
    struct {
      const struct Vector4Ps3 *vector4;
    };
    struct {
      const struct Matrix4x4Ps3 *matrix4x4;
    };
    struct {
      const f32Ps3 *float_;
    };
    struct {
      const i32Ps3 *int_;
    };
    struct {
      const u32Ps3 *bool_;
    };
    struct {
      struct string string;
    };
    struct {
      struct slice_string string_list;
    };
    struct {
      struct ref_slice_U32Ps3 object_list;
    };
    struct {
      struct ref_slice_Vector4Ps3 node_list;
    };
    struct {
      struct ref_slice_i32Ps3 int_list;
    };
    struct {
      struct ref_slice_U32Ps3 crc_list;
    };
    struct {
      struct ref_slice_WeightPs3 weight_list;
    };
    struct {
      struct ref_slice_Matrix4x4Ps3 matrix_list;
    };
  };
} BaseTypeRefPs3;

/**
 *gen_ffi:export
 */
typedef struct BlockHeader1Ps3 {
  u32Ps3 a;
  u32Ps3 b;
  u32Ps3 unk_2;
  u32Ps3 unk_3;
} BlockHeader1Ps3;

/**
 *gen_ffi:export
 */
typedef struct BlockHeader2Ps3 {
  u32Ps3 n;
  f32Ps3 unk_1;
  f32Ps3 unk_2;
  u32Ps3 unk_3;
  u32Ps3 unk_4;
} BlockHeader2Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BlockValAPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BlockValAPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_BlockValBPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_BlockValBPs3;

typedef struct BlockRefPs3 {
  const struct BlockHeader1Ps3 *info1;
  const struct BlockHeader2Ps3 *info2;
  struct ref_slice_BlockValAPs3 vals_a;
  struct ref_slice_BlockValAPs3 vals_b;
  struct ref_slice_BlockValBPs3 vals_c;
  struct ref_slice_u8 pad;
} BlockRefPs3;

/**
 *gen_ffi:export
 */
typedef struct ShapeInfoPs3 {
  u32Ps3 offset;
  u32Ps3 kind;
  u32Ps3 unk_2;
  f32Ps3 unk_3;
  f32Ps3 unk_4;
  f32Ps3 unk_5;
  struct Vector3Ps3 translation;
  struct Vector4Ps3 rotation;
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
} ShapeInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct Option_ShapeExtraRefPs3 {
  uint64_t align;
  uint8_t pad[32];
} Option_ShapeExtraRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_HkShapeRefPs3 {
  uint64_t align;
  uint8_t pad[8];
} slice_HkShapeRefPs3;

typedef struct ShapeRefPs3 {
  const struct ShapeInfoPs3 *info;
  struct Option_ShapeExtraRefPs3 extra;
  struct slice_HkShapeRefPs3 hk_shapes;
} ShapeRefPs3;

/**
 *gen_ffi:export
 */
typedef struct BoxShapePs3 {
  struct Vector4Ps3 translation;
  struct Vector4Ps3 rotation;
  u32Ps3 kind;
  CrcPs3 key;
  struct Vector3Ps3 half_extents;
  u32Ps3 unk_13;
  u32Ps3 unk_14;
  f32Ps3 unk_15;
  f32Ps3 unk_16;
  f32Ps3 unk_17;
  f32Ps3 unk_18;
  f32Ps3 unk_19;
} BoxShapePs3;

/**
 *gen_ffi:export
 */
typedef struct SphereShapePs3 {
  struct Vector4Ps3 translation;
  struct Vector4Ps3 rotation;
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
} SphereShapePs3;

/**
 *gen_ffi:export
 */
typedef struct CapsuleShapePs3 {
  struct Vector4Ps3 translation;
  struct Vector4Ps3 rotation;
  u32Ps3 kind;
  CrcPs3 key;
  struct Vector3Ps3 point1;
  struct Vector3Ps3 point2;
  f32Ps3 radius;
  f32Ps3 unk_17;
  f32Ps3 unk_18;
  f32Ps3 unk_19;
} CapsuleShapePs3;

/**
 *gen_ffi:export
 */
typedef struct CylinderShapePs3 {
  struct Vector4Ps3 translation;
  struct Vector4Ps3 rotation;
  u32Ps3 kind;
  CrcPs3 key;
  struct Vector3Ps3 point1;
  struct Vector3Ps3 point2;
  f32Ps3 radius;
  f32Ps3 unk_17;
  f32Ps3 unk_18;
  f32Ps3 unk_19;
} CylinderShapePs3;

/**
 *gen_ffi:export
 */
typedef struct ConvexVerticesInfoPs3 {
  struct Vector4Ps3 translation;
  struct Vector4Ps3 rotation;
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
} ConvexVerticesInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_Vector3Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_Vector3Ps3;

typedef struct ConvexVerticesRefPs3 {
  const struct ConvexVerticesInfoPs3 *info;
  struct ref_slice_Vector4Ps3 norms;
  struct ref_slice_Vector3Ps3 verts;
} ConvexVerticesRefPs3;

/**
 *gen_ffi:export
 */
typedef struct BVTreeMeshInfoPs3 {
  struct Vector4Ps3 translation;
  struct Vector4Ps3 rotation;
  u32Ps3 kind;
  CrcPs3 key;
  struct Vector3Ps3 offset;
  f32Ps3 tree_scale;
  u32Ps3 tree_size;
  u32Ps3 tree_offset;
  u32Ps3 vert_num;
  u32Ps3 verts_offset;
  u32Ps3 tri_num;
  u32Ps3 inds_offset;
} BVTreeMeshInfoPs3;

typedef struct BVTreeMeshRefPs3 {
  const struct BVTreeMeshInfoPs3 *info;
  struct ref_slice_u8 tree;
  struct ref_slice_Vector3Ps3 verts;
  struct ref_slice_u16Ps3 inds;
} BVTreeMeshRefPs3;

/**
 *gen_ffi:export
 */
typedef struct HkShapeInfoPs3 {
  struct Vector4Ps3 unk_0;
  struct Vector4Ps3 unk_4;
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
} HkShapeInfoPs3;

enum HkShapeRefPs3_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  HkShapeRefPs3_Box,
  HkShapeRefPs3_Sphere,
  HkShapeRefPs3_Capsule,
  HkShapeRefPs3_Cylinder,
  HkShapeRefPs3_ConvexVertices,
  HkShapeRefPs3_BVTreeMesh,
  HkShapeRefPs3_Unknown,
};
#if __STDC_VERSION__ >= 202311L
typedef enum HkShapeRefPs3_Tag HkShapeRefPs3_Tag;
#else
typedef uint8_t HkShapeRefPs3_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct HkShapeRefPs3 {
  HkShapeRefPs3_Tag tag;
  union {
    struct {
      const struct BoxShapePs3 *box;
    };
    struct {
      const struct SphereShapePs3 *sphere;
    };
    struct {
      const struct CapsuleShapePs3 *capsule;
    };
    struct {
      const struct CylinderShapePs3 *cylinder;
    };
    struct {
      struct ConvexVerticesRefPs3 convex_vertices;
    };
    struct {
      struct BVTreeMeshRefPs3 bv_tree_mesh;
    };
    struct {
      const struct HkShapeInfoPs3 *unknown;
    };
  };
} HkShapeRefPs3;

/**
 *gen_ffi:export
 */
typedef struct FoliageInfoPs3 {
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
  struct Vector4Ps3 color;
  f32Ps3 lod1a;
  f32Ps3 lod1b;
  f32Ps3 lod2a;
  f32Ps3 lod2b;
  f32Ps3 lod_max;
} FoliageInfoPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_FoliageValPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_FoliageValPs3;

typedef struct FoliageRefPs3 {
  const struct FoliageInfoPs3 *info;
  struct ref_slice_FoliageValPs3 vals;
} FoliageRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_BlockValARefPs3 {
  uint64_t align;
  uint8_t pad[8];
} slice_BlockValARefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_Obj1RefPs3 {
  uint64_t align;
  uint8_t pad[8];
} slice_Obj1RefPs3;

typedef struct BlockValRefPs3 {
  struct slice_BlockValARefPs3 vals_a;
  struct slice_Obj1RefPs3 vals_b;
} BlockValRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_BlockValRefPs3 {
  uint64_t align;
  uint8_t pad[8];
} slice_BlockValRefPs3;

/**
 *gen_ffi:export
 */
typedef struct CrowdItemHeaderPs3 {
  CrcPs3 key;
  CrcPs3 key_main;
  CrcPs3 key_right;
  CrcPs3 key_left;
  f32Ps3 unk_4;
  u32Ps3 animation_num;
  u32Ps3 instance_num;
} CrowdItemHeaderPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_CrowdValPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_CrowdValPs3;

typedef struct CrowdItemRefPs3 {
  const struct CrowdItemHeaderPs3 *header;
  struct ref_slice_CrcPs3 animations;
  struct ref_slice_CrowdValPs3 instances;
} CrowdItemRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_CrowdItemRefPs3 {
  uint64_t align;
  uint8_t pad[8];
} slice_CrowdItemRefPs3;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintBoneRefPs3 {
  struct string name;
  u32Ps3 start;
  u32Ps3 val;
} HkConstraintBoneRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct slice_HkConstraintBoneRefPs3 {
  uint64_t align;
  uint8_t pad[8];
} slice_HkConstraintBoneRefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_f32Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_f32Ps3;

enum AnimVals1RefPs3_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  AnimVals1RefPs3_Type1,
  AnimVals1RefPs3_Type2,
  AnimVals1RefPs3_Type3,
  AnimVals1RefPs3_Type4,
};
#if __STDC_VERSION__ >= 202311L
typedef enum AnimVals1RefPs3_Tag AnimVals1RefPs3_Tag;
#else
typedef uint8_t AnimVals1RefPs3_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct AnimVals1RefPs3 {
  AnimVals1RefPs3_Tag tag;
  union {
    struct {
      struct ref_slice_u8 type1;
    };
    struct {
      struct ref_slice_u16Ps3 type2;
    };
    struct {
      struct ref_slice_u16Ps3 type3;
    };
    struct {
      struct ref_slice_u16Ps3 type4;
    };
  };
} AnimVals1RefPs3;

typedef struct Obj1RefPs3 {
  uint8_t flags;
  uint8_t s2;
  u16Ps3 s1;
  struct ref_slice_u8 data;
  struct ref_slice_f32Ps3 vals_a;
  struct AnimVals1RefPs3 vals;
  uintptr_t size;
} Obj1RefPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationPolar32Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationPolar32Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationThreeComp40Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationThreeComp40Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationThreeComp48Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationThreeComp48Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationThreeComp24Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationThreeComp24Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationStraight16Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationStraight16Ps3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_RotationUncompressedPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_RotationUncompressedPs3;

enum RotationQuantizationRefPs3_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  RotationQuantizationRefPs3_Polar32,
  RotationQuantizationRefPs3_ThreeComp40,
  RotationQuantizationRefPs3_ThreeComp48,
  RotationQuantizationRefPs3_ThreeComp24,
  RotationQuantizationRefPs3_Straight16,
  RotationQuantizationRefPs3_Uncompressed,
};
#if __STDC_VERSION__ >= 202311L
typedef enum RotationQuantizationRefPs3_Tag RotationQuantizationRefPs3_Tag;
#else
typedef uint8_t RotationQuantizationRefPs3_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct RotationQuantizationRefPs3 {
  RotationQuantizationRefPs3_Tag tag;
  union {
    struct {
      struct ref_slice_RotationPolar32Ps3 polar32;
    };
    struct {
      struct ref_slice_RotationThreeComp40Ps3 three_comp40;
    };
    struct {
      struct ref_slice_RotationThreeComp48Ps3 three_comp48;
    };
    struct {
      struct ref_slice_RotationThreeComp24Ps3 three_comp24;
    };
    struct {
      struct ref_slice_RotationStraight16Ps3 straight16;
    };
    struct {
      struct ref_slice_RotationUncompressedPs3 uncompressed;
    };
  };
} RotationQuantizationRefPs3;

typedef struct Obj2RefPs3 {
  uint8_t flags;
  uint8_t s2;
  u16Ps3 s1;
  struct ref_slice_u8 data;
  struct RotationQuantizationRefPs3 vals;
  uintptr_t size;
} Obj2RefPs3;

typedef struct BlockValARefPs3 {
  struct Obj1RefPs3 a;
  struct Obj2RefPs3 b;
  struct Obj1RefPs3 c;
} BlockValARefPs3;

/**
 *gen_ffi:export
 */
typedef struct BufferInfoPs3 {
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
} BufferInfoPs3;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintDataPs3 {
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
} HkConstraintDataPs3;

/**
 *gen_ffi:export
 */
typedef struct Key2Ps3 {
  CrcPs3 key;
  u32Ps3 val;
} Key2Ps3;

/**
 *gen_ffi:export
 */
typedef struct BlockValAPs3 {
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
} BlockValAPs3;

/**
 *gen_ffi:export
 */
typedef struct BlockValBPs3 {
  u16Ps3 unk_0;
  u16Ps3 unk_1;
  f32Ps3 unk_2;
  f32Ps3 unk_3;
  f32Ps3 unk_4;
  f32Ps3 unk_5;
} BlockValBPs3;

/**
 *gen_ffi:export
 */
typedef struct AnimationBlockInfoPs3 {
  CrcPs3 key;
  u32Ps3 guid;
  CrcPs3 key_name;
  u32Ps3 offset;
  u32Ps3 size;
  u32Ps3 size_comp;
  u32Ps3 unk_6;
  u32Ps3 unk_7;
  u32Ps3 unk_8;
} AnimationBlockInfoPs3;

/**
 *gen_ffi:export
 */
typedef struct AssetHandlePs3 {
  CrcPs3 key;
  u32Ps3 offset;
  u32Ps3 size;
  u32Ps3 size_comp;
  u32Ps3 kind;
} AssetHandlePs3;

typedef uint32_t U32BE;

typedef U32BE U32Ps3;

typedef I32BE I32Ps3;

typedef struct BlockAValPs3 {
  U32Ps3 unk_0;
  I32Ps3 gamemodemask;
  U32Ps3 key;
  U32Ps3 unk_3;
  U32Ps3 unk_4;
  U32Ps3 unk_5;
  U32Ps3 unk_6;
} BlockAValPs3;

/**
 *gen_ffi:export
 */
typedef struct GFXBlockInfoPs3 {
  CrcPs3 key;
  u32Ps3 offset;
  u32Ps3 size;
} GFXBlockInfoPs3;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintInfoPs3 {
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
} HkConstraintInfoPs3;

/**
 *gen_ffi:export
 */
typedef struct Obj0Ps3 {
  u32Ps3 unk_0;
  CrcPs3 key;
} Obj0Ps3;

/**
 *gen_ffi:export
 */
typedef struct ObjAPs3 {
  CrcPs3 key;
  u32Ps3 unk_1;
  u32Ps3 size;
  u32Ps3 size_comp;
  u32Ps3 unk_4;
  u32Ps3 kind;
} ObjAPs3;

/**
 *gen_ffi:export
 */
typedef struct PFieldInfoPs3 {
  u32Ps3 link_guid;
  u32Ps3 gamemode_guid;
  u32Ps3 width;
  u32Ps3 height;
  u32Ps3 offset;
} PFieldInfoPs3;

/**
 *gen_ffi:export
 */
typedef struct StringKeysValPs3 {
  CrcPs3 key;
  u32Ps3 offset;
} StringKeysValPs3;

/**
 *gen_ffi:export
 */
typedef struct SubBlocksBlockHeaderPs3 {
  CrcPs3 key;
  u32Ps3 offset;
  u32Ps3 size;
} SubBlocksBlockHeaderPs3;

/**
 *gen_ffi:export
 */
typedef struct Obj3Ps3 {
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
} Obj3Ps3;

/**
 *gen_ffi:export
 */
typedef struct Obj5ValPs3 {
  f32Ps3 unk_0;
  f32Ps3 unk_1;
  f32Ps3 unk_2;
  f32Ps3 unk_3;
  f32Ps3 unk_4;
  f32Ps3 unk_5;
  f32Ps3 unk_6;
} Obj5ValPs3;

/**
 *gen_ffi:export
 */
typedef struct SSAValPs3 {
  f32Ps3 t_start;
  f32Ps3 t_end;
  u32Ps3 unk_2;
  u32Ps3 unk_3;
  u32Ps3 off;
} SSAValPs3;

/**
 *gen_ffi:export
 */
typedef struct TypeFieldPs3 {
  CrcPs3 key;
  CrcPs3 kind;
  u32Ps3 offset;
} TypeFieldPs3;

typedef i16_be i16Ps3;

/**
 *gen_ffi:export
 */
typedef struct FoliageValPs3 {
  u16Ps3 height;
  u16Ps3 var_mask;
  i16Ps3 slope_x;
  i16Ps3 slope_z;
} FoliageValPs3;

/**
 *gen_ffi:export
 */
typedef struct WeightPs3 {
  u32Ps3 x;
  u8Ps3 a;
  u8Ps3 b;
  u8Ps3 c;
  u8Ps3 d;
} WeightPs3;

/**
 *gen_ffi:export
 */
typedef struct AtlasUVValPs3 {
  CrcPs3 key;
  struct Vector4Ps3 vals;
} AtlasUVValPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_AtlasUVValPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_AtlasUVValPs3;

/**
 *gen_ffi:export
 */
typedef struct SprayInstancePs3 {
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
} SprayInstancePs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SprayInstancePs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SprayInstancePs3;

/**
 *gen_ffi:export
 */
typedef struct SprayValPs3 {
  struct Vector3Ps3 position;
  f32Ps3 scale;
  u16Ps3 instance;
  u16Ps3 rotation;
} SprayValPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_SprayValPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_SprayValPs3;

/**
 *gen_ffi:export
 */
typedef struct TRSPs3 {
  struct Vector4Ps3 translation;
  struct Vector4Ps3 rotation;
  struct Vector4Ps3 scale;
} TRSPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_TRSPs3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_TRSPs3;

/**
 * This is a proxy struct for correct size and alignment only
 * don't construct this directly and use the provided methods to access
 */
typedef struct ref_slice_i16Ps3 {
  uint64_t align;
  uint8_t pad[8];
} ref_slice_i16Ps3;

/**
 *gen_ffi:export
 */
typedef struct CrowdValPs3 {
  struct Vector3Ps3 position;
  f32Ps3 rotation;
  f32Ps3 lod;
} CrowdValPs3;

/**
 *gen_ffi:export
 */
typedef struct RotationPolar32Ps3 {
  u32Ps3 a;
} RotationPolar32Ps3;

/**
 *gen_ffi:export
 */
typedef struct RotationStraight16Ps3 {
  u8Ps3 a;
  u8Ps3 b;
} RotationStraight16Ps3;

/**
 *gen_ffi:export
 */
typedef struct RotationThreeComp24Ps3 {
  u8Ps3 a;
  u8Ps3 b;
  u8Ps3 c;
} RotationThreeComp24Ps3;

/**
 *gen_ffi:export
 */
typedef struct RotationThreeComp40Ps3 {
  u8Ps3 a;
  u8Ps3 b;
  u8Ps3 c;
  u8Ps3 d;
  u8Ps3 e;
} RotationThreeComp40Ps3;

/**
 *gen_ffi:export
 */
typedef struct RotationThreeComp48Ps3 {
  u16Ps3 a;
  u16Ps3 b;
  u16Ps3 c;
} RotationThreeComp48Ps3;

/**
 *gen_ffi:export
 */
typedef struct RotationUncompressedPs3 {
  f32Ps3 a;
  f32Ps3 b;
  f32Ps3 c;
  f32Ps3 d;
} RotationUncompressedPs3;

/**
 *gen_ffi:export
 */
typedef struct HkConstraintRefPs3 {
  const struct HkConstraintInfoPs3 *info;
  struct ref_slice_i16Ps3 bone_parents;
  struct slice_HkConstraintBoneRefPs3 bone_names;
  struct ref_slice_u32Ps3 name_offsets;
  struct ref_slice_TRSPs3 bone_transforms;
  struct ref_slice_u32Ps3 bones;
  struct ref_slice_Key2Ps3 bones_order;
  struct ref_slice_f32Ps3 vals2;
} HkConstraintRefPs3;

/**
 *gen_ffi:export
 */
typedef struct ShapeExtraInfoPs3 {
  u32Ps3 size;
  f32Ps3 scale;
  f32Ps3 a;
  f32Ps3 b;
} ShapeExtraInfoPs3;

typedef struct ShapeExtraRefPs3 {
  const struct ShapeExtraInfoPs3 *info;
  struct ref_slice_u32Ps3 offs;
  struct ref_slice_u8 data;
} ShapeExtraRefPs3;

typedef struct AtlasUVRefPs3 {
  struct ref_slice_AtlasUVValPs3 vals;
} AtlasUVRefPs3;

typedef struct BlocksRefPs3 {
  struct ref_slice_u32Ps3 block_starts;
  struct ref_slice_u32Ps3 block_starts2;
  struct ref_slice_u32Ps3 obj_c3;
  struct ref_slice_u32Ps3 obj_c4;
  struct slice_BlockValRefPs3 blocks;
} BlocksRefPs3;

/**
 *gen_ffi:export
 */
typedef struct CrowdHeaderPs3 {
  u32Ps3 const0x65;
  u32Ps3 n;
} CrowdHeaderPs3;

typedef struct CrowdRefPs3 {
  const struct CrowdHeaderPs3 *header;
  struct ref_slice_u32Ps3 offs;
  struct slice_CrowdItemRefPs3 vals;
} CrowdRefPs3;

typedef struct DataRefPs3 PFieldsRefPs3;

typedef struct SprayRefPs3 {
  struct ref_slice_SprayInstancePs3 instances;
  struct ref_slice_SprayValPs3 vals;
} SprayRefPs3;

struct LevelData *OwnedLevelData_get(struct OwnedLevelData *val);

void OwnedLevelData_free(struct OwnedLevelData *val);

struct OwnedLevelData *LevelData_read_data(const char *path);

Version LevelData_version(const struct LevelData *src);

struct OwnedLevelCompressedData *LevelCompressedData_new(void);

struct OwnedLevelRefPc *LevelRefPc_from_data(const struct LevelData *src,
                                             struct LevelCompressedData *data);

struct LevelData *LevelRefPc_dump(const struct LevelRefPc *src, uint32_t compression);

struct OwnedLevelRefXbox *LevelRefXbox_from_data(const struct LevelData *src,
                                                 struct LevelCompressedData *data);

struct LevelData *LevelRefXbox_dump(const struct LevelRefXbox *src, uint32_t compression);

struct OwnedLevelRefPs3 *LevelRefPs3_from_data(const struct LevelData *src,
                                               struct LevelCompressedData *data);

struct LevelData *LevelRefPs3_dump(const struct LevelRefPs3 *src, uint32_t compression);

struct InfoCounts *OwnedInfoCounts_get(struct OwnedInfoCounts *val);

void OwnedInfoCounts_free(struct OwnedInfoCounts *val);

struct OwnedInfoCounts *InfoCounts_new(void);

uintptr_t InfoCounts_size_pc(const struct InfoCounts *counts);

uintptr_t InfoCounts_size_xbox(const struct InfoCounts *counts);

uintptr_t InfoCounts_size_ps3(const struct InfoCounts *counts);

struct OwnedDumpInfosPc *DumpInfosPc_from_data(struct DumpSlice *dst,
                                               const struct InfoCounts *counts,
                                               struct mut_slice_u32Pc *offsets);

struct OwnedDumpInfosXbox *DumpInfosXbox_from_data(struct DumpSlice *dst,
                                                   const struct InfoCounts *counts,
                                                   struct mut_slice_u32Xbox *offsets);

struct OwnedDumpInfosPs3 *DumpInfosPs3_from_data(struct DumpSlice *dst,
                                                 const struct InfoCounts *counts,
                                                 struct mut_slice_u32Ps3 *offsets);

struct OwnedVecCompressedData *AnimationsRefPc_dump(const struct AnimationsRefPc *anims,
                                                    struct DumpInfosPc *infos);

struct OwnedVecCompressedData *AnimationsRefXbox_dump(const struct AnimationsRefXbox *anims,
                                                      struct DumpInfosXbox *infos);

struct OwnedVecCompressedData *AnimationsRefPs3_dump(const struct AnimationsRefPs3 *anims,
                                                     struct DumpInfosPs3 *infos);

struct AlignedBuf *OwnedAlignedBuf_get(struct OwnedAlignedBuf *val);

void OwnedAlignedBuf_free(struct OwnedAlignedBuf *val);

const uint8_t *string_get(const struct string *string);

uintptr_t string_len(const struct string *string);

const struct CompressedDataRef *IndexMap_u32__CompressedDataRef_get(const struct IndexMap_u32__CompressedDataRef *map,
                                                                    const uint32_t *key);

uintptr_t IndexMap_u32__CompressedDataRef_len(const struct IndexMap_u32__CompressedDataRef *map);

void IndexMap_u32__CompressedDataRef_keys(const struct IndexMap_u32__CompressedDataRef *map,
                                          struct mut_slice_u32 *keys);

const struct ref_slice_u8 *IndexMap_u32__ref_slice_u8_get(const struct IndexMap_u32__ref_slice_u8 *map,
                                                          const uint32_t *key);

uintptr_t IndexMap_u32__ref_slice_u8_len(const struct IndexMap_u32__ref_slice_u8 *map);

void IndexMap_u32__ref_slice_u8_keys(const struct IndexMap_u32__ref_slice_u8 *map,
                                     struct mut_slice_u32 *keys);

const struct CompressedDataRef *const *IndexMap_u32_______CompressedDataRef_get(const struct IndexMap_u32_______CompressedDataRef *map,
                                                                                const uint32_t *key);

uintptr_t IndexMap_u32_______CompressedDataRef_len(const struct IndexMap_u32_______CompressedDataRef *map);

void IndexMap_u32_______CompressedDataRef_keys(const struct IndexMap_u32_______CompressedDataRef *map,
                                               struct mut_slice_u32 *keys);

const struct VertexDataIndex *IndexMap_VertexUsage__VertexDataIndex_get(const struct IndexMap_VertexUsage__VertexDataIndex *map,
                                                                        const struct VertexUsage *key);

uintptr_t IndexMap_VertexUsage__VertexDataIndex_len(const struct IndexMap_VertexUsage__VertexDataIndex *map);

void IndexMap_VertexUsage__VertexDataIndex_keys(const struct IndexMap_VertexUsage__VertexDataIndex *map,
                                                struct mut_slice_VertexUsage *keys);

const uint8_t *ref_slice_u8_get(const struct ref_slice_u8 *slice, uintptr_t idx);

uintptr_t ref_slice_u8_len(const struct ref_slice_u8 *slice);

const uint32_t *ref_slice_u32_get(const struct ref_slice_u32 *slice, uintptr_t idx);

uintptr_t ref_slice_u32_len(const struct ref_slice_u32 *slice);

const struct VertexUsage *ref_slice_VertexUsage_get(const struct ref_slice_VertexUsage *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_VertexUsage_len(const struct ref_slice_VertexUsage *slice);

uint8_t *mut_slice_u8_get(struct mut_slice_u8 *slice, uintptr_t idx);

uintptr_t mut_slice_u8_len(const struct slice_u8 *slice);

struct VertexUsage *mut_slice_VertexUsage_get(struct mut_slice_VertexUsage *slice, uintptr_t idx);

uintptr_t mut_slice_VertexUsage_len(const struct slice_VertexUsage *slice);

uint32_t *mut_slice_u32_get(struct mut_slice_u32 *slice, uintptr_t idx);

uintptr_t mut_slice_u32_len(const struct slice_u32 *slice);

const struct string *slice_string_get(const struct slice_string *slice, uintptr_t idx);

uintptr_t slice_string_len(const struct slice_string *slice);

const struct CompressedDataRef *slice_CompressedDataRef_get(const struct slice_CompressedDataRef *slice,
                                                            uintptr_t idx);

uintptr_t slice_CompressedDataRef_len(const struct slice_CompressedDataRef *slice);

const struct AlignmentHelper *slice_AlignmentHelper_get(const struct slice_AlignmentHelper *slice,
                                                        uintptr_t idx);

uintptr_t slice_AlignmentHelper_len(const struct slice_AlignmentHelper *slice);

const struct CompressedDataRef *const *slice______CompressedDataRef_get(const struct slice______CompressedDataRef *slice,
                                                                        uintptr_t idx);

uintptr_t slice______CompressedDataRef_len(const struct slice______CompressedDataRef *slice);

const struct CompressedDataRef *ref_slice_CompressedDataRef_get(const struct slice_CompressedDataRef *slice,
                                                                uintptr_t idx);

uintptr_t ref_slice_CompressedDataRef_len(const struct slice_CompressedDataRef *slice);

struct OwnedAlignedBuf *AlignedBuf_with_capacity(uintptr_t size);

const struct IndexBufferRefPc *IndexMap_u32__IndexBufferRefPc_get(const struct IndexMap_u32__IndexBufferRefPc *map,
                                                                  const uint32_t *key);

uintptr_t IndexMap_u32__IndexBufferRefPc_len(const struct IndexMap_u32__IndexBufferRefPc *map);

void IndexMap_u32__IndexBufferRefPc_keys(const struct IndexMap_u32__IndexBufferRefPc *map,
                                         struct mut_slice_u32 *keys);

const struct VertexBufferRefPc *IndexMap_u32__VertexBufferRefPc_get(const struct IndexMap_u32__VertexBufferRefPc *map,
                                                                    const uint32_t *key);

uintptr_t IndexMap_u32__VertexBufferRefPc_len(const struct IndexMap_u32__VertexBufferRefPc *map);

void IndexMap_u32__VertexBufferRefPc_keys(const struct IndexMap_u32__VertexBufferRefPc *map,
                                          struct mut_slice_u32 *keys);

const struct MatRefPc *IndexMap_u32__MatRefPc_get(const struct IndexMap_u32__MatRefPc *map,
                                                  const uint32_t *key);

uintptr_t IndexMap_u32__MatRefPc_len(const struct IndexMap_u32__MatRefPc *map);

void IndexMap_u32__MatRefPc_keys(const struct IndexMap_u32__MatRefPc *map,
                                 struct mut_slice_u32 *keys);

const struct ModelRefPc *IndexMap_u32__ModelRefPc_get(const struct IndexMap_u32__ModelRefPc *map,
                                                      const uint32_t *key);

uintptr_t IndexMap_u32__ModelRefPc_len(const struct IndexMap_u32__ModelRefPc *map);

void IndexMap_u32__ModelRefPc_keys(const struct IndexMap_u32__ModelRefPc *map,
                                   struct mut_slice_u32 *keys);

const struct IBuffInfoPc *const *IndexMap_u32_______IBuffInfoPc_get(const struct IndexMap_u32_______IBuffInfoPc *map,
                                                                    const uint32_t *key);

uintptr_t IndexMap_u32_______IBuffInfoPc_len(const struct IndexMap_u32_______IBuffInfoPc *map);

void IndexMap_u32_______IBuffInfoPc_keys(const struct IndexMap_u32_______IBuffInfoPc *map,
                                         struct mut_slice_u32 *keys);

const struct VBuffInfoPc *const *IndexMap_u32_______VBuffInfoPc_get(const struct IndexMap_u32_______VBuffInfoPc *map,
                                                                    const uint32_t *key);

uintptr_t IndexMap_u32_______VBuffInfoPc_len(const struct IndexMap_u32_______VBuffInfoPc *map);

void IndexMap_u32_______VBuffInfoPc_keys(const struct IndexMap_u32_______VBuffInfoPc *map,
                                         struct mut_slice_u32 *keys);

const struct AnimationRefPc *IndexMap_u32__AnimationRefPc_get(const struct IndexMap_u32__AnimationRefPc *map,
                                                              const uint32_t *key);

uintptr_t IndexMap_u32__AnimationRefPc_len(const struct IndexMap_u32__AnimationRefPc *map);

void IndexMap_u32__AnimationRefPc_keys(const struct IndexMap_u32__AnimationRefPc *map,
                                       struct mut_slice_u32 *keys);

const struct DataRefPc *IndexMap_u32__DataRefPc_get(const struct IndexMap_u32__DataRefPc *map,
                                                    const uint32_t *key);

uintptr_t IndexMap_u32__DataRefPc_len(const struct IndexMap_u32__DataRefPc *map);

void IndexMap_u32__DataRefPc_keys(const struct IndexMap_u32__DataRefPc *map,
                                  struct mut_slice_u32 *keys);

const struct EffectRefPc *IndexMap_u32__EffectRefPc_get(const struct IndexMap_u32__EffectRefPc *map,
                                                        const uint32_t *key);

uintptr_t IndexMap_u32__EffectRefPc_len(const struct IndexMap_u32__EffectRefPc *map);

void IndexMap_u32__EffectRefPc_keys(const struct IndexMap_u32__EffectRefPc *map,
                                    struct mut_slice_u32 *keys);

const struct LangStringsRefPc *IndexMap_u32__LangStringsRefPc_get(const struct IndexMap_u32__LangStringsRefPc *map,
                                                                  const uint32_t *key);

uintptr_t IndexMap_u32__LangStringsRefPc_len(const struct IndexMap_u32__LangStringsRefPc *map);

void IndexMap_u32__LangStringsRefPc_keys(const struct IndexMap_u32__LangStringsRefPc *map,
                                         struct mut_slice_u32 *keys);

const LuaRefPc *IndexMap_u32__LuaRefPc_get(const struct IndexMap_u32__LuaRefPc *map,
                                           const uint32_t *key);

uintptr_t IndexMap_u32__LuaRefPc_len(const struct IndexMap_u32__LuaRefPc *map);

void IndexMap_u32__LuaRefPc_keys(const struct IndexMap_u32__LuaRefPc *map,
                                 struct mut_slice_u32 *keys);

const struct ObjRefPc *IndexMap_u32__ObjRefPc_get(const struct IndexMap_u32__ObjRefPc *map,
                                                  const uint32_t *key);

uintptr_t IndexMap_u32__ObjRefPc_len(const struct IndexMap_u32__ObjRefPc *map);

void IndexMap_u32__ObjRefPc_keys(const struct IndexMap_u32__ObjRefPc *map,
                                 struct mut_slice_u32 *keys);

const struct RadiosityValsRefPc *IndexMap_u32__RadiosityValsRefPc_get(const struct IndexMap_u32__RadiosityValsRefPc *map,
                                                                      const uint32_t *key);

uintptr_t IndexMap_u32__RadiosityValsRefPc_len(const struct IndexMap_u32__RadiosityValsRefPc *map);

void IndexMap_u32__RadiosityValsRefPc_keys(const struct IndexMap_u32__RadiosityValsRefPc *map,
                                           struct mut_slice_u32 *keys);

const struct SSARefPc *IndexMap_u32__SSARefPc_get(const struct IndexMap_u32__SSARefPc *map,
                                                  const uint32_t *key);

uintptr_t IndexMap_u32__SSARefPc_len(const struct IndexMap_u32__SSARefPc *map);

void IndexMap_u32__SSARefPc_keys(const struct IndexMap_u32__SSARefPc *map,
                                 struct mut_slice_u32 *keys);

const struct TextureRefPc *IndexMap_u32__TextureRefPc_get(const struct IndexMap_u32__TextureRefPc *map,
                                                          const uint32_t *key);

uintptr_t IndexMap_u32__TextureRefPc_len(const struct IndexMap_u32__TextureRefPc *map);

void IndexMap_u32__TextureRefPc_keys(const struct IndexMap_u32__TextureRefPc *map,
                                     struct mut_slice_u32 *keys);

const struct TypeRefPc *IndexMap_u32__TypeRefPc_get(const struct IndexMap_u32__TypeRefPc *map,
                                                    const uint32_t *key);

uintptr_t IndexMap_u32__TypeRefPc_len(const struct IndexMap_u32__TypeRefPc *map);

void IndexMap_u32__TypeRefPc_keys(const struct IndexMap_u32__TypeRefPc *map,
                                  struct mut_slice_u32 *keys);

const struct slice_FoliageRefPc *IndexMap_u32__slice_FoliageRefPc_get(const struct IndexMap_u32__slice_FoliageRefPc *map,
                                                                      const uint32_t *key);

uintptr_t IndexMap_u32__slice_FoliageRefPc_len(const struct IndexMap_u32__slice_FoliageRefPc *map);

void IndexMap_u32__slice_FoliageRefPc_keys(const struct IndexMap_u32__slice_FoliageRefPc *map,
                                           struct mut_slice_u32 *keys);

const struct BaseTypeRefPc *IndexMap_u32__BaseTypeRefPc_get(const struct IndexMap_u32__BaseTypeRefPc *map,
                                                            const uint32_t *key);

uintptr_t IndexMap_u32__BaseTypeRefPc_len(const struct IndexMap_u32__BaseTypeRefPc *map);

void IndexMap_u32__BaseTypeRefPc_keys(const struct IndexMap_u32__BaseTypeRefPc *map,
                                      struct mut_slice_u32 *keys);

const struct ref_slice_u16Pc *IndexMap_u32__ref_slice_u16Pc_get(const struct IndexMap_u32__ref_slice_u16Pc *map,
                                                                const uint32_t *key);

uintptr_t IndexMap_u32__ref_slice_u16Pc_len(const struct IndexMap_u32__ref_slice_u16Pc *map);

void IndexMap_u32__ref_slice_u16Pc_keys(const struct IndexMap_u32__ref_slice_u16Pc *map,
                                        struct mut_slice_u32 *keys);

const struct BlockRefPc *slice_BlockRefPc_get(const struct slice_BlockRefPc *slice, uintptr_t idx);

uintptr_t slice_BlockRefPc_len(const struct slice_BlockRefPc *slice);

const struct ShapeRefPc *slice_ShapeRefPc_get(const struct slice_ShapeRefPc *slice, uintptr_t idx);

uintptr_t slice_ShapeRefPc_len(const struct slice_ShapeRefPc *slice);

const struct HkShapeRefPc *owned_slice_HkShapeRefPc_get(const struct slice_HkShapeRefPc *slice,
                                                        uintptr_t idx);

uintptr_t owned_slice_HkShapeRefPc_len(const struct slice_HkShapeRefPc *slice);

const struct FoliageRefPc *slice_FoliageRefPc_get(const struct slice_FoliageRefPc *slice,
                                                  uintptr_t idx);

uintptr_t slice_FoliageRefPc_len(const struct slice_FoliageRefPc *slice);

const struct ref_slice_u16Pc *slice_ref_slice_u16Pc_get(const struct slice_ref_slice_u16Pc *slice,
                                                        uintptr_t idx);

uintptr_t slice_ref_slice_u16Pc_len(const struct slice_ref_slice_u16Pc *slice);

const struct BlockValRefPc *slice_BlockValRefPc_get(const struct slice_BlockValRefPc *slice,
                                                    uintptr_t idx);

uintptr_t slice_BlockValRefPc_len(const struct slice_BlockValRefPc *slice);

const struct CrowdItemRefPc *slice_CrowdItemRefPc_get(const struct slice_CrowdItemRefPc *slice,
                                                      uintptr_t idx);

uintptr_t slice_CrowdItemRefPc_len(const struct slice_CrowdItemRefPc *slice);

const struct HkConstraintBoneRefPc *slice_HkConstraintBoneRefPc_get(const struct slice_HkConstraintBoneRefPc *slice,
                                                                    uintptr_t idx);

uintptr_t slice_HkConstraintBoneRefPc_len(const struct slice_HkConstraintBoneRefPc *slice);

const struct BlockValARefPc *slice_BlockValARefPc_get(const struct slice_BlockValARefPc *slice,
                                                      uintptr_t idx);

uintptr_t slice_BlockValARefPc_len(const struct slice_BlockValARefPc *slice);

const struct Obj1RefPc *slice_Obj1RefPc_get(const struct slice_Obj1RefPc *slice, uintptr_t idx);

uintptr_t slice_Obj1RefPc_len(const struct slice_Obj1RefPc *slice);

const struct HkShapeRefPc *slice_HkShapeRefPc_get(const struct slice_HkShapeRefPc *slice,
                                                  uintptr_t idx);

uintptr_t slice_HkShapeRefPc_len(const struct slice_HkShapeRefPc *slice);

const struct BoundingBoxPc *ref_slice_BoundingBoxPc_get(const struct ref_slice_BoundingBoxPc *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_BoundingBoxPc_len(const struct ref_slice_BoundingBoxPc *slice);

const struct BufferInfoPc *ref_slice_BufferInfoPc_get(const struct ref_slice_BufferInfoPc *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_BufferInfoPc_len(const struct ref_slice_BufferInfoPc *slice);

const CrcPc *ref_slice_CrcPc_get(const struct ref_slice_CrcPc *slice, uintptr_t idx);

uintptr_t ref_slice_CrcPc_len(const struct ref_slice_CrcPc *slice);

const struct HkConstraintDataPc *ref_slice_HkConstraintDataPc_get(const struct ref_slice_HkConstraintDataPc *slice,
                                                                  uintptr_t idx);

uintptr_t ref_slice_HkConstraintDataPc_len(const struct ref_slice_HkConstraintDataPc *slice);

const struct Key2Pc *ref_slice_Key2Pc_get(const struct ref_slice_Key2Pc *slice, uintptr_t idx);

uintptr_t ref_slice_Key2Pc_len(const struct ref_slice_Key2Pc *slice);

const struct Matrix4x4Pc *ref_slice_Matrix4x4Pc_get(const struct ref_slice_Matrix4x4Pc *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_Matrix4x4Pc_len(const struct ref_slice_Matrix4x4Pc *slice);

const struct Vector3Pc *ref_slice_Vector3Pc_get(const struct ref_slice_Vector3Pc *slice,
                                                uintptr_t idx);

uintptr_t ref_slice_Vector3Pc_len(const struct ref_slice_Vector3Pc *slice);

const struct Vector4Pc *ref_slice_Vector4Pc_get(const struct ref_slice_Vector4Pc *slice,
                                                uintptr_t idx);

uintptr_t ref_slice_Vector4Pc_len(const struct ref_slice_Vector4Pc *slice);

const i32Pc *ref_slice_i32Pc_get(const struct ref_slice_i32Pc *slice, uintptr_t idx);

uintptr_t ref_slice_i32Pc_len(const struct ref_slice_i32Pc *slice);

const u16Pc *ref_slice_u16Pc_get(const struct ref_slice_u16Pc *slice, uintptr_t idx);

uintptr_t ref_slice_u16Pc_len(const struct ref_slice_u16Pc *slice);

const u32Pc *ref_slice_u32Pc_get(const struct ref_slice_u32Pc *slice, uintptr_t idx);

uintptr_t ref_slice_u32Pc_len(const struct ref_slice_u32Pc *slice);

const struct BlockValAPc *ref_slice_BlockValAPc_get(const struct ref_slice_BlockValAPc *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_BlockValAPc_len(const struct ref_slice_BlockValAPc *slice);

const struct BlockValBPc *ref_slice_BlockValBPc_get(const struct ref_slice_BlockValBPc *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_BlockValBPc_len(const struct ref_slice_BlockValBPc *slice);

const struct AnimationBlockInfoPc *ref_slice_AnimationBlockInfoPc_get(const struct ref_slice_AnimationBlockInfoPc *slice,
                                                                      uintptr_t idx);

uintptr_t ref_slice_AnimationBlockInfoPc_len(const struct ref_slice_AnimationBlockInfoPc *slice);

const struct AnimationInfoPc *ref_slice_AnimationInfoPc_get(const struct ref_slice_AnimationInfoPc *slice,
                                                            uintptr_t idx);

uintptr_t ref_slice_AnimationInfoPc_len(const struct ref_slice_AnimationInfoPc *slice);

const struct AssetHandlePc *ref_slice_AssetHandlePc_get(const struct ref_slice_AssetHandlePc *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_AssetHandlePc_len(const struct ref_slice_AssetHandlePc *slice);

const struct BlockAValPc *ref_slice_BlockAValPc_get(const struct ref_slice_BlockAValPc *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_BlockAValPc_len(const struct ref_slice_BlockAValPc *slice);

const struct EffectInfoPc *ref_slice_EffectInfoPc_get(const struct ref_slice_EffectInfoPc *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_EffectInfoPc_len(const struct ref_slice_EffectInfoPc *slice);

const struct FoliageInfoPc *ref_slice_FoliageInfoPc_get(const struct ref_slice_FoliageInfoPc *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_FoliageInfoPc_len(const struct ref_slice_FoliageInfoPc *slice);

const struct GFXBlockInfoPc *ref_slice_GFXBlockInfoPc_get(const struct ref_slice_GFXBlockInfoPc *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_GFXBlockInfoPc_len(const struct ref_slice_GFXBlockInfoPc *slice);

const struct HkConstraintInfoPc *ref_slice_HkConstraintInfoPc_get(const struct ref_slice_HkConstraintInfoPc *slice,
                                                                  uintptr_t idx);

uintptr_t ref_slice_HkConstraintInfoPc_len(const struct ref_slice_HkConstraintInfoPc *slice);

const struct HkShapeInfoPc *ref_slice_HkShapeInfoPc_get(const struct ref_slice_HkShapeInfoPc *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_HkShapeInfoPc_len(const struct ref_slice_HkShapeInfoPc *slice);

const struct IBuffInfoPc *ref_slice_IBuffInfoPc_get(const struct ref_slice_IBuffInfoPc *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_IBuffInfoPc_len(const struct ref_slice_IBuffInfoPc *slice);

const struct Mat1Pc *ref_slice_Mat1Pc_get(const struct ref_slice_Mat1Pc *slice, uintptr_t idx);

uintptr_t ref_slice_Mat1Pc_len(const struct ref_slice_Mat1Pc *slice);

const struct Mat2Pc *ref_slice_Mat2Pc_get(const struct ref_slice_Mat2Pc *slice, uintptr_t idx);

uintptr_t ref_slice_Mat2Pc_len(const struct ref_slice_Mat2Pc *slice);

const struct Mat3Pc *ref_slice_Mat3Pc_get(const struct ref_slice_Mat3Pc *slice, uintptr_t idx);

uintptr_t ref_slice_Mat3Pc_len(const struct ref_slice_Mat3Pc *slice);

const struct Mat4Pc *ref_slice_Mat4Pc_get(const struct ref_slice_Mat4Pc *slice, uintptr_t idx);

uintptr_t ref_slice_Mat4Pc_len(const struct ref_slice_Mat4Pc *slice);

const struct MatExtraPc *ref_slice_MatExtraPc_get(const struct ref_slice_MatExtraPc *slice,
                                                  uintptr_t idx);

uintptr_t ref_slice_MatExtraPc_len(const struct ref_slice_MatExtraPc *slice);

const struct ModelInfoPc *ref_slice_ModelInfoPc_get(const struct ref_slice_ModelInfoPc *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_ModelInfoPc_len(const struct ref_slice_ModelInfoPc *slice);

const struct Obj0Pc *ref_slice_Obj0Pc_get(const struct ref_slice_Obj0Pc *slice, uintptr_t idx);

uintptr_t ref_slice_Obj0Pc_len(const struct ref_slice_Obj0Pc *slice);

const struct ObjAPc *ref_slice_ObjAPc_get(const struct ref_slice_ObjAPc *slice, uintptr_t idx);

uintptr_t ref_slice_ObjAPc_len(const struct ref_slice_ObjAPc *slice);

const struct PFieldInfoPc *ref_slice_PFieldInfoPc_get(const struct ref_slice_PFieldInfoPc *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_PFieldInfoPc_len(const struct ref_slice_PFieldInfoPc *slice);

const struct RadiosityValsInfoPc *ref_slice_RadiosityValsInfoPc_get(const struct ref_slice_RadiosityValsInfoPc *slice,
                                                                    uintptr_t idx);

uintptr_t ref_slice_RadiosityValsInfoPc_len(const struct ref_slice_RadiosityValsInfoPc *slice);

const struct ShapeInfoPc *ref_slice_ShapeInfoPc_get(const struct ref_slice_ShapeInfoPc *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_ShapeInfoPc_len(const struct ref_slice_ShapeInfoPc *slice);

const struct StringKeysValPc *ref_slice_StringKeysValPc_get(const struct ref_slice_StringKeysValPc *slice,
                                                            uintptr_t idx);

uintptr_t ref_slice_StringKeysValPc_len(const struct ref_slice_StringKeysValPc *slice);

const struct SubBlocksBlockHeaderPc *ref_slice_SubBlocksBlockHeaderPc_get(const struct ref_slice_SubBlocksBlockHeaderPc *slice,
                                                                          uintptr_t idx);

uintptr_t ref_slice_SubBlocksBlockHeaderPc_len(const struct ref_slice_SubBlocksBlockHeaderPc *slice);

const struct TextureInfoPc *ref_slice_TextureInfoPc_get(const struct ref_slice_TextureInfoPc *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_TextureInfoPc_len(const struct ref_slice_TextureInfoPc *slice);

const struct VBuffInfoPc *ref_slice_VBuffInfoPc_get(const struct ref_slice_VBuffInfoPc *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_VBuffInfoPc_len(const struct ref_slice_VBuffInfoPc *slice);

const struct Obj3Pc *ref_slice_Obj3Pc_get(const struct ref_slice_Obj3Pc *slice, uintptr_t idx);

uintptr_t ref_slice_Obj3Pc_len(const struct ref_slice_Obj3Pc *slice);

const struct Obj5ValPc *ref_slice_Obj5ValPc_get(const struct ref_slice_Obj5ValPc *slice,
                                                uintptr_t idx);

uintptr_t ref_slice_Obj5ValPc_len(const struct ref_slice_Obj5ValPc *slice);

const struct SSAValPc *ref_slice_SSAValPc_get(const struct ref_slice_SSAValPc *slice,
                                              uintptr_t idx);

uintptr_t ref_slice_SSAValPc_len(const struct ref_slice_SSAValPc *slice);

const struct TypeFieldPc *ref_slice_TypeFieldPc_get(const struct ref_slice_TypeFieldPc *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_TypeFieldPc_len(const struct ref_slice_TypeFieldPc *slice);

const struct FoliageValPc *ref_slice_FoliageValPc_get(const struct ref_slice_FoliageValPc *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_FoliageValPc_len(const struct ref_slice_FoliageValPc *slice);

const U32Pc *ref_slice_U32Pc_get(const struct ref_slice_U32Pc *slice, uintptr_t idx);

uintptr_t ref_slice_U32Pc_len(const struct ref_slice_U32Pc *slice);

const struct WeightPc *ref_slice_WeightPc_get(const struct ref_slice_WeightPc *slice,
                                              uintptr_t idx);

uintptr_t ref_slice_WeightPc_len(const struct ref_slice_WeightPc *slice);

const struct AtlasUVValPc *ref_slice_AtlasUVValPc_get(const struct ref_slice_AtlasUVValPc *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_AtlasUVValPc_len(const struct ref_slice_AtlasUVValPc *slice);

const struct SprayInstancePc *ref_slice_SprayInstancePc_get(const struct ref_slice_SprayInstancePc *slice,
                                                            uintptr_t idx);

uintptr_t ref_slice_SprayInstancePc_len(const struct ref_slice_SprayInstancePc *slice);

const struct SprayValPc *ref_slice_SprayValPc_get(const struct ref_slice_SprayValPc *slice,
                                                  uintptr_t idx);

uintptr_t ref_slice_SprayValPc_len(const struct ref_slice_SprayValPc *slice);

const struct TRSPc *ref_slice_TRSPc_get(const struct ref_slice_TRSPc *slice, uintptr_t idx);

uintptr_t ref_slice_TRSPc_len(const struct ref_slice_TRSPc *slice);

const f32Pc *ref_slice_f32Pc_get(const struct ref_slice_f32Pc *slice, uintptr_t idx);

uintptr_t ref_slice_f32Pc_len(const struct ref_slice_f32Pc *slice);

const i16Pc *ref_slice_i16Pc_get(const struct ref_slice_i16Pc *slice, uintptr_t idx);

uintptr_t ref_slice_i16Pc_len(const struct ref_slice_i16Pc *slice);

const struct CrowdValPc *ref_slice_CrowdValPc_get(const struct ref_slice_CrowdValPc *slice,
                                                  uintptr_t idx);

uintptr_t ref_slice_CrowdValPc_len(const struct ref_slice_CrowdValPc *slice);

const struct RotationPolar32Pc *ref_slice_RotationPolar32Pc_get(const struct ref_slice_RotationPolar32Pc *slice,
                                                                uintptr_t idx);

uintptr_t ref_slice_RotationPolar32Pc_len(const struct ref_slice_RotationPolar32Pc *slice);

const struct RotationStraight16Pc *ref_slice_RotationStraight16Pc_get(const struct ref_slice_RotationStraight16Pc *slice,
                                                                      uintptr_t idx);

uintptr_t ref_slice_RotationStraight16Pc_len(const struct ref_slice_RotationStraight16Pc *slice);

const struct RotationThreeComp24Pc *ref_slice_RotationThreeComp24Pc_get(const struct ref_slice_RotationThreeComp24Pc *slice,
                                                                        uintptr_t idx);

uintptr_t ref_slice_RotationThreeComp24Pc_len(const struct ref_slice_RotationThreeComp24Pc *slice);

const struct RotationThreeComp40Pc *ref_slice_RotationThreeComp40Pc_get(const struct ref_slice_RotationThreeComp40Pc *slice,
                                                                        uintptr_t idx);

uintptr_t ref_slice_RotationThreeComp40Pc_len(const struct ref_slice_RotationThreeComp40Pc *slice);

const struct RotationThreeComp48Pc *ref_slice_RotationThreeComp48Pc_get(const struct ref_slice_RotationThreeComp48Pc *slice,
                                                                        uintptr_t idx);

uintptr_t ref_slice_RotationThreeComp48Pc_len(const struct ref_slice_RotationThreeComp48Pc *slice);

const struct RotationUncompressedPc *ref_slice_RotationUncompressedPc_get(const struct ref_slice_RotationUncompressedPc *slice,
                                                                          uintptr_t idx);

uintptr_t ref_slice_RotationUncompressedPc_len(const struct ref_slice_RotationUncompressedPc *slice);

u32Pc *mut_slice_u32Pc_get(struct mut_slice_u32Pc *slice, uintptr_t idx);

uintptr_t mut_slice_u32Pc_len(const struct slice_u32Pc *slice);

struct AnimationBlockInfoPc *mut_slice_AnimationBlockInfoPc_get(struct mut_slice_AnimationBlockInfoPc *slice,
                                                                uintptr_t idx);

uintptr_t mut_slice_AnimationBlockInfoPc_len(const struct slice_AnimationBlockInfoPc *slice);

struct AnimationInfoPc *mut_slice_AnimationInfoPc_get(struct mut_slice_AnimationInfoPc *slice,
                                                      uintptr_t idx);

uintptr_t mut_slice_AnimationInfoPc_len(const struct slice_AnimationInfoPc *slice);

struct BufferInfoPc *mut_slice_BufferInfoPc_get(struct mut_slice_BufferInfoPc *slice,
                                                uintptr_t idx);

uintptr_t mut_slice_BufferInfoPc_len(const struct slice_BufferInfoPc *slice);

struct EffectInfoPc *mut_slice_EffectInfoPc_get(struct mut_slice_EffectInfoPc *slice,
                                                uintptr_t idx);

uintptr_t mut_slice_EffectInfoPc_len(const struct slice_EffectInfoPc *slice);

struct FoliageInfoPc *mut_slice_FoliageInfoPc_get(struct mut_slice_FoliageInfoPc *slice,
                                                  uintptr_t idx);

uintptr_t mut_slice_FoliageInfoPc_len(const struct slice_FoliageInfoPc *slice);

struct GFXBlockInfoPc *mut_slice_GFXBlockInfoPc_get(struct mut_slice_GFXBlockInfoPc *slice,
                                                    uintptr_t idx);

uintptr_t mut_slice_GFXBlockInfoPc_len(const struct slice_GFXBlockInfoPc *slice);

struct HkConstraintDataPc *mut_slice_HkConstraintDataPc_get(struct mut_slice_HkConstraintDataPc *slice,
                                                            uintptr_t idx);

uintptr_t mut_slice_HkConstraintDataPc_len(const struct slice_HkConstraintDataPc *slice);

struct HkConstraintInfoPc *mut_slice_HkConstraintInfoPc_get(struct mut_slice_HkConstraintInfoPc *slice,
                                                            uintptr_t idx);

uintptr_t mut_slice_HkConstraintInfoPc_len(const struct slice_HkConstraintInfoPc *slice);

struct HkShapeInfoPc *mut_slice_HkShapeInfoPc_get(struct mut_slice_HkShapeInfoPc *slice,
                                                  uintptr_t idx);

uintptr_t mut_slice_HkShapeInfoPc_len(const struct slice_HkShapeInfoPc *slice);

struct IBuffInfoPc *mut_slice_IBuffInfoPc_get(struct mut_slice_IBuffInfoPc *slice, uintptr_t idx);

uintptr_t mut_slice_IBuffInfoPc_len(const struct slice_IBuffInfoPc *slice);

struct Mat1Pc *mut_slice_Mat1Pc_get(struct mut_slice_Mat1Pc *slice, uintptr_t idx);

uintptr_t mut_slice_Mat1Pc_len(const struct slice_Mat1Pc *slice);

struct Mat2Pc *mut_slice_Mat2Pc_get(struct mut_slice_Mat2Pc *slice, uintptr_t idx);

uintptr_t mut_slice_Mat2Pc_len(const struct slice_Mat2Pc *slice);

struct Mat3Pc *mut_slice_Mat3Pc_get(struct mut_slice_Mat3Pc *slice, uintptr_t idx);

uintptr_t mut_slice_Mat3Pc_len(const struct slice_Mat3Pc *slice);

struct Mat4Pc *mut_slice_Mat4Pc_get(struct mut_slice_Mat4Pc *slice, uintptr_t idx);

uintptr_t mut_slice_Mat4Pc_len(const struct slice_Mat4Pc *slice);

struct MatExtraPc *mut_slice_MatExtraPc_get(struct mut_slice_MatExtraPc *slice, uintptr_t idx);

uintptr_t mut_slice_MatExtraPc_len(const struct slice_MatExtraPc *slice);

struct ModelInfoPc *mut_slice_ModelInfoPc_get(struct mut_slice_ModelInfoPc *slice, uintptr_t idx);

uintptr_t mut_slice_ModelInfoPc_len(const struct slice_ModelInfoPc *slice);

struct Obj0Pc *mut_slice_Obj0Pc_get(struct mut_slice_Obj0Pc *slice, uintptr_t idx);

uintptr_t mut_slice_Obj0Pc_len(const struct slice_Obj0Pc *slice);

struct ObjAPc *mut_slice_ObjAPc_get(struct mut_slice_ObjAPc *slice, uintptr_t idx);

uintptr_t mut_slice_ObjAPc_len(const struct slice_ObjAPc *slice);

struct PFieldInfoPc *mut_slice_PFieldInfoPc_get(struct mut_slice_PFieldInfoPc *slice,
                                                uintptr_t idx);

uintptr_t mut_slice_PFieldInfoPc_len(const struct slice_PFieldInfoPc *slice);

struct RadiosityValsInfoPc *mut_slice_RadiosityValsInfoPc_get(struct mut_slice_RadiosityValsInfoPc *slice,
                                                              uintptr_t idx);

uintptr_t mut_slice_RadiosityValsInfoPc_len(const struct slice_RadiosityValsInfoPc *slice);

struct ShapeInfoPc *mut_slice_ShapeInfoPc_get(struct mut_slice_ShapeInfoPc *slice, uintptr_t idx);

uintptr_t mut_slice_ShapeInfoPc_len(const struct slice_ShapeInfoPc *slice);

struct TextureInfoPc *mut_slice_TextureInfoPc_get(struct mut_slice_TextureInfoPc *slice,
                                                  uintptr_t idx);

uintptr_t mut_slice_TextureInfoPc_len(const struct slice_TextureInfoPc *slice);

struct VBuffInfoPc *mut_slice_VBuffInfoPc_get(struct mut_slice_VBuffInfoPc *slice, uintptr_t idx);

uintptr_t mut_slice_VBuffInfoPc_len(const struct slice_VBuffInfoPc *slice);

const struct HkConstraintRefPc *Option_HkConstraintRefPc_get(const struct Option_HkConstraintRefPc *slice);

const struct ShapeExtraRefPc *Option_ShapeExtraRefPc_get(const struct Option_ShapeExtraRefPc *slice);

const struct AtlasUVRefPc *Option_AtlasUVRefPc_get(const struct Option_AtlasUVRefPc *slice);

const struct BlocksRefPc *Option_BlocksRefPc_get(const struct Option_BlocksRefPc *slice);

const struct CrowdRefPc *Option_CrowdRefPc_get(const struct Option_CrowdRefPc *slice);

const PFieldsRefPc *Option_PFieldsRefPc_get(const struct Option_PFieldsRefPc *slice);

const struct SprayRefPc *Option_SprayRefPc_get(const struct Option_SprayRefPc *slice);

const struct IndexBufferRefXbox *IndexMap_u32__IndexBufferRefXbox_get(const struct IndexMap_u32__IndexBufferRefXbox *map,
                                                                      const uint32_t *key);

uintptr_t IndexMap_u32__IndexBufferRefXbox_len(const struct IndexMap_u32__IndexBufferRefXbox *map);

void IndexMap_u32__IndexBufferRefXbox_keys(const struct IndexMap_u32__IndexBufferRefXbox *map,
                                           struct mut_slice_u32 *keys);

const struct VertexBufferRefXbox *IndexMap_u32__VertexBufferRefXbox_get(const struct IndexMap_u32__VertexBufferRefXbox *map,
                                                                        const uint32_t *key);

uintptr_t IndexMap_u32__VertexBufferRefXbox_len(const struct IndexMap_u32__VertexBufferRefXbox *map);

void IndexMap_u32__VertexBufferRefXbox_keys(const struct IndexMap_u32__VertexBufferRefXbox *map,
                                            struct mut_slice_u32 *keys);

const struct MatRefXbox *IndexMap_u32__MatRefXbox_get(const struct IndexMap_u32__MatRefXbox *map,
                                                      const uint32_t *key);

uintptr_t IndexMap_u32__MatRefXbox_len(const struct IndexMap_u32__MatRefXbox *map);

void IndexMap_u32__MatRefXbox_keys(const struct IndexMap_u32__MatRefXbox *map,
                                   struct mut_slice_u32 *keys);

const struct ModelRefXbox *IndexMap_u32__ModelRefXbox_get(const struct IndexMap_u32__ModelRefXbox *map,
                                                          const uint32_t *key);

uintptr_t IndexMap_u32__ModelRefXbox_len(const struct IndexMap_u32__ModelRefXbox *map);

void IndexMap_u32__ModelRefXbox_keys(const struct IndexMap_u32__ModelRefXbox *map,
                                     struct mut_slice_u32 *keys);

const struct IBuffInfoXbox *const *IndexMap_u32_______IBuffInfoXbox_get(const struct IndexMap_u32_______IBuffInfoXbox *map,
                                                                        const uint32_t *key);

uintptr_t IndexMap_u32_______IBuffInfoXbox_len(const struct IndexMap_u32_______IBuffInfoXbox *map);

void IndexMap_u32_______IBuffInfoXbox_keys(const struct IndexMap_u32_______IBuffInfoXbox *map,
                                           struct mut_slice_u32 *keys);

const struct VBuffInfoXbox *const *IndexMap_u32_______VBuffInfoXbox_get(const struct IndexMap_u32_______VBuffInfoXbox *map,
                                                                        const uint32_t *key);

uintptr_t IndexMap_u32_______VBuffInfoXbox_len(const struct IndexMap_u32_______VBuffInfoXbox *map);

void IndexMap_u32_______VBuffInfoXbox_keys(const struct IndexMap_u32_______VBuffInfoXbox *map,
                                           struct mut_slice_u32 *keys);

const struct AnimationRefXbox *IndexMap_u32__AnimationRefXbox_get(const struct IndexMap_u32__AnimationRefXbox *map,
                                                                  const uint32_t *key);

uintptr_t IndexMap_u32__AnimationRefXbox_len(const struct IndexMap_u32__AnimationRefXbox *map);

void IndexMap_u32__AnimationRefXbox_keys(const struct IndexMap_u32__AnimationRefXbox *map,
                                         struct mut_slice_u32 *keys);

const struct DataRefXbox *IndexMap_u32__DataRefXbox_get(const struct IndexMap_u32__DataRefXbox *map,
                                                        const uint32_t *key);

uintptr_t IndexMap_u32__DataRefXbox_len(const struct IndexMap_u32__DataRefXbox *map);

void IndexMap_u32__DataRefXbox_keys(const struct IndexMap_u32__DataRefXbox *map,
                                    struct mut_slice_u32 *keys);

const struct EffectRefXbox *IndexMap_u32__EffectRefXbox_get(const struct IndexMap_u32__EffectRefXbox *map,
                                                            const uint32_t *key);

uintptr_t IndexMap_u32__EffectRefXbox_len(const struct IndexMap_u32__EffectRefXbox *map);

void IndexMap_u32__EffectRefXbox_keys(const struct IndexMap_u32__EffectRefXbox *map,
                                      struct mut_slice_u32 *keys);

const struct LangStringsRefXbox *IndexMap_u32__LangStringsRefXbox_get(const struct IndexMap_u32__LangStringsRefXbox *map,
                                                                      const uint32_t *key);

uintptr_t IndexMap_u32__LangStringsRefXbox_len(const struct IndexMap_u32__LangStringsRefXbox *map);

void IndexMap_u32__LangStringsRefXbox_keys(const struct IndexMap_u32__LangStringsRefXbox *map,
                                           struct mut_slice_u32 *keys);

const LuaRefXbox *IndexMap_u32__LuaRefXbox_get(const struct IndexMap_u32__LuaRefXbox *map,
                                               const uint32_t *key);

uintptr_t IndexMap_u32__LuaRefXbox_len(const struct IndexMap_u32__LuaRefXbox *map);

void IndexMap_u32__LuaRefXbox_keys(const struct IndexMap_u32__LuaRefXbox *map,
                                   struct mut_slice_u32 *keys);

const struct ObjRefXbox *IndexMap_u32__ObjRefXbox_get(const struct IndexMap_u32__ObjRefXbox *map,
                                                      const uint32_t *key);

uintptr_t IndexMap_u32__ObjRefXbox_len(const struct IndexMap_u32__ObjRefXbox *map);

void IndexMap_u32__ObjRefXbox_keys(const struct IndexMap_u32__ObjRefXbox *map,
                                   struct mut_slice_u32 *keys);

const struct RadiosityValsRefXbox *IndexMap_u32__RadiosityValsRefXbox_get(const struct IndexMap_u32__RadiosityValsRefXbox *map,
                                                                          const uint32_t *key);

uintptr_t IndexMap_u32__RadiosityValsRefXbox_len(const struct IndexMap_u32__RadiosityValsRefXbox *map);

void IndexMap_u32__RadiosityValsRefXbox_keys(const struct IndexMap_u32__RadiosityValsRefXbox *map,
                                             struct mut_slice_u32 *keys);

const struct SSARefXbox *IndexMap_u32__SSARefXbox_get(const struct IndexMap_u32__SSARefXbox *map,
                                                      const uint32_t *key);

uintptr_t IndexMap_u32__SSARefXbox_len(const struct IndexMap_u32__SSARefXbox *map);

void IndexMap_u32__SSARefXbox_keys(const struct IndexMap_u32__SSARefXbox *map,
                                   struct mut_slice_u32 *keys);

const struct TextureRefXbox *IndexMap_u32__TextureRefXbox_get(const struct IndexMap_u32__TextureRefXbox *map,
                                                              const uint32_t *key);

uintptr_t IndexMap_u32__TextureRefXbox_len(const struct IndexMap_u32__TextureRefXbox *map);

void IndexMap_u32__TextureRefXbox_keys(const struct IndexMap_u32__TextureRefXbox *map,
                                       struct mut_slice_u32 *keys);

const struct TypeRefXbox *IndexMap_u32__TypeRefXbox_get(const struct IndexMap_u32__TypeRefXbox *map,
                                                        const uint32_t *key);

uintptr_t IndexMap_u32__TypeRefXbox_len(const struct IndexMap_u32__TypeRefXbox *map);

void IndexMap_u32__TypeRefXbox_keys(const struct IndexMap_u32__TypeRefXbox *map,
                                    struct mut_slice_u32 *keys);

const struct slice_FoliageRefXbox *IndexMap_u32__slice_FoliageRefXbox_get(const struct IndexMap_u32__slice_FoliageRefXbox *map,
                                                                          const uint32_t *key);

uintptr_t IndexMap_u32__slice_FoliageRefXbox_len(const struct IndexMap_u32__slice_FoliageRefXbox *map);

void IndexMap_u32__slice_FoliageRefXbox_keys(const struct IndexMap_u32__slice_FoliageRefXbox *map,
                                             struct mut_slice_u32 *keys);

const struct BaseTypeRefXbox *IndexMap_u32__BaseTypeRefXbox_get(const struct IndexMap_u32__BaseTypeRefXbox *map,
                                                                const uint32_t *key);

uintptr_t IndexMap_u32__BaseTypeRefXbox_len(const struct IndexMap_u32__BaseTypeRefXbox *map);

void IndexMap_u32__BaseTypeRefXbox_keys(const struct IndexMap_u32__BaseTypeRefXbox *map,
                                        struct mut_slice_u32 *keys);

const struct ref_slice_u16Xbox *IndexMap_u32__ref_slice_u16Xbox_get(const struct IndexMap_u32__ref_slice_u16Xbox *map,
                                                                    const uint32_t *key);

uintptr_t IndexMap_u32__ref_slice_u16Xbox_len(const struct IndexMap_u32__ref_slice_u16Xbox *map);

void IndexMap_u32__ref_slice_u16Xbox_keys(const struct IndexMap_u32__ref_slice_u16Xbox *map,
                                          struct mut_slice_u32 *keys);

const struct BlockRefXbox *slice_BlockRefXbox_get(const struct slice_BlockRefXbox *slice,
                                                  uintptr_t idx);

uintptr_t slice_BlockRefXbox_len(const struct slice_BlockRefXbox *slice);

const struct ShapeRefXbox *slice_ShapeRefXbox_get(const struct slice_ShapeRefXbox *slice,
                                                  uintptr_t idx);

uintptr_t slice_ShapeRefXbox_len(const struct slice_ShapeRefXbox *slice);

const struct HkShapeRefXbox *owned_slice_HkShapeRefXbox_get(const struct slice_HkShapeRefXbox *slice,
                                                            uintptr_t idx);

uintptr_t owned_slice_HkShapeRefXbox_len(const struct slice_HkShapeRefXbox *slice);

const struct FoliageRefXbox *slice_FoliageRefXbox_get(const struct slice_FoliageRefXbox *slice,
                                                      uintptr_t idx);

uintptr_t slice_FoliageRefXbox_len(const struct slice_FoliageRefXbox *slice);

const struct ref_slice_u16Xbox *slice_ref_slice_u16Xbox_get(const struct slice_ref_slice_u16Xbox *slice,
                                                            uintptr_t idx);

uintptr_t slice_ref_slice_u16Xbox_len(const struct slice_ref_slice_u16Xbox *slice);

const struct BlockValRefXbox *slice_BlockValRefXbox_get(const struct slice_BlockValRefXbox *slice,
                                                        uintptr_t idx);

uintptr_t slice_BlockValRefXbox_len(const struct slice_BlockValRefXbox *slice);

const struct CrowdItemRefXbox *slice_CrowdItemRefXbox_get(const struct slice_CrowdItemRefXbox *slice,
                                                          uintptr_t idx);

uintptr_t slice_CrowdItemRefXbox_len(const struct slice_CrowdItemRefXbox *slice);

const struct HkConstraintBoneRefXbox *slice_HkConstraintBoneRefXbox_get(const struct slice_HkConstraintBoneRefXbox *slice,
                                                                        uintptr_t idx);

uintptr_t slice_HkConstraintBoneRefXbox_len(const struct slice_HkConstraintBoneRefXbox *slice);

const struct BlockValARefXbox *slice_BlockValARefXbox_get(const struct slice_BlockValARefXbox *slice,
                                                          uintptr_t idx);

uintptr_t slice_BlockValARefXbox_len(const struct slice_BlockValARefXbox *slice);

const struct Obj1RefXbox *slice_Obj1RefXbox_get(const struct slice_Obj1RefXbox *slice,
                                                uintptr_t idx);

uintptr_t slice_Obj1RefXbox_len(const struct slice_Obj1RefXbox *slice);

const struct HkShapeRefXbox *slice_HkShapeRefXbox_get(const struct slice_HkShapeRefXbox *slice,
                                                      uintptr_t idx);

uintptr_t slice_HkShapeRefXbox_len(const struct slice_HkShapeRefXbox *slice);

const struct BoundingBoxXbox *ref_slice_BoundingBoxXbox_get(const struct ref_slice_BoundingBoxXbox *slice,
                                                            uintptr_t idx);

uintptr_t ref_slice_BoundingBoxXbox_len(const struct ref_slice_BoundingBoxXbox *slice);

const struct BufferInfoXbox *ref_slice_BufferInfoXbox_get(const struct ref_slice_BufferInfoXbox *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_BufferInfoXbox_len(const struct ref_slice_BufferInfoXbox *slice);

const CrcXbox *ref_slice_CrcXbox_get(const struct ref_slice_CrcXbox *slice, uintptr_t idx);

uintptr_t ref_slice_CrcXbox_len(const struct ref_slice_CrcXbox *slice);

const struct HkConstraintDataXbox *ref_slice_HkConstraintDataXbox_get(const struct ref_slice_HkConstraintDataXbox *slice,
                                                                      uintptr_t idx);

uintptr_t ref_slice_HkConstraintDataXbox_len(const struct ref_slice_HkConstraintDataXbox *slice);

const struct Key2Xbox *ref_slice_Key2Xbox_get(const struct ref_slice_Key2Xbox *slice,
                                              uintptr_t idx);

uintptr_t ref_slice_Key2Xbox_len(const struct ref_slice_Key2Xbox *slice);

const struct Matrix4x4Xbox *ref_slice_Matrix4x4Xbox_get(const struct ref_slice_Matrix4x4Xbox *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_Matrix4x4Xbox_len(const struct ref_slice_Matrix4x4Xbox *slice);

const struct Vector3Xbox *ref_slice_Vector3Xbox_get(const struct ref_slice_Vector3Xbox *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_Vector3Xbox_len(const struct ref_slice_Vector3Xbox *slice);

const struct Vector4Xbox *ref_slice_Vector4Xbox_get(const struct ref_slice_Vector4Xbox *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_Vector4Xbox_len(const struct ref_slice_Vector4Xbox *slice);

const i32Xbox *ref_slice_i32Xbox_get(const struct ref_slice_i32Xbox *slice, uintptr_t idx);

uintptr_t ref_slice_i32Xbox_len(const struct ref_slice_i32Xbox *slice);

const u16Xbox *ref_slice_u16Xbox_get(const struct ref_slice_u16Xbox *slice, uintptr_t idx);

uintptr_t ref_slice_u16Xbox_len(const struct ref_slice_u16Xbox *slice);

const u32Xbox *ref_slice_u32Xbox_get(const struct ref_slice_u32Xbox *slice, uintptr_t idx);

uintptr_t ref_slice_u32Xbox_len(const struct ref_slice_u32Xbox *slice);

const struct BlockValAXbox *ref_slice_BlockValAXbox_get(const struct ref_slice_BlockValAXbox *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_BlockValAXbox_len(const struct ref_slice_BlockValAXbox *slice);

const struct BlockValBXbox *ref_slice_BlockValBXbox_get(const struct ref_slice_BlockValBXbox *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_BlockValBXbox_len(const struct ref_slice_BlockValBXbox *slice);

const struct AnimationBlockInfoXbox *ref_slice_AnimationBlockInfoXbox_get(const struct ref_slice_AnimationBlockInfoXbox *slice,
                                                                          uintptr_t idx);

uintptr_t ref_slice_AnimationBlockInfoXbox_len(const struct ref_slice_AnimationBlockInfoXbox *slice);

const struct AnimationInfoXbox *ref_slice_AnimationInfoXbox_get(const struct ref_slice_AnimationInfoXbox *slice,
                                                                uintptr_t idx);

uintptr_t ref_slice_AnimationInfoXbox_len(const struct ref_slice_AnimationInfoXbox *slice);

const struct AssetHandleXbox *ref_slice_AssetHandleXbox_get(const struct ref_slice_AssetHandleXbox *slice,
                                                            uintptr_t idx);

uintptr_t ref_slice_AssetHandleXbox_len(const struct ref_slice_AssetHandleXbox *slice);

const struct BlockAValXbox *ref_slice_BlockAValXbox_get(const struct ref_slice_BlockAValXbox *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_BlockAValXbox_len(const struct ref_slice_BlockAValXbox *slice);

const struct EffectInfoXbox *ref_slice_EffectInfoXbox_get(const struct ref_slice_EffectInfoXbox *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_EffectInfoXbox_len(const struct ref_slice_EffectInfoXbox *slice);

const struct FoliageInfoXbox *ref_slice_FoliageInfoXbox_get(const struct ref_slice_FoliageInfoXbox *slice,
                                                            uintptr_t idx);

uintptr_t ref_slice_FoliageInfoXbox_len(const struct ref_slice_FoliageInfoXbox *slice);

const struct GFXBlockInfoXbox *ref_slice_GFXBlockInfoXbox_get(const struct ref_slice_GFXBlockInfoXbox *slice,
                                                              uintptr_t idx);

uintptr_t ref_slice_GFXBlockInfoXbox_len(const struct ref_slice_GFXBlockInfoXbox *slice);

const struct HkConstraintInfoXbox *ref_slice_HkConstraintInfoXbox_get(const struct ref_slice_HkConstraintInfoXbox *slice,
                                                                      uintptr_t idx);

uintptr_t ref_slice_HkConstraintInfoXbox_len(const struct ref_slice_HkConstraintInfoXbox *slice);

const struct HkShapeInfoXbox *ref_slice_HkShapeInfoXbox_get(const struct ref_slice_HkShapeInfoXbox *slice,
                                                            uintptr_t idx);

uintptr_t ref_slice_HkShapeInfoXbox_len(const struct ref_slice_HkShapeInfoXbox *slice);

const struct IBuffInfoXbox *ref_slice_IBuffInfoXbox_get(const struct ref_slice_IBuffInfoXbox *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_IBuffInfoXbox_len(const struct ref_slice_IBuffInfoXbox *slice);

const struct Mat1Xbox *ref_slice_Mat1Xbox_get(const struct ref_slice_Mat1Xbox *slice,
                                              uintptr_t idx);

uintptr_t ref_slice_Mat1Xbox_len(const struct ref_slice_Mat1Xbox *slice);

const struct Mat2Xbox *ref_slice_Mat2Xbox_get(const struct ref_slice_Mat2Xbox *slice,
                                              uintptr_t idx);

uintptr_t ref_slice_Mat2Xbox_len(const struct ref_slice_Mat2Xbox *slice);

const struct Mat3Xbox *ref_slice_Mat3Xbox_get(const struct ref_slice_Mat3Xbox *slice,
                                              uintptr_t idx);

uintptr_t ref_slice_Mat3Xbox_len(const struct ref_slice_Mat3Xbox *slice);

const struct Mat4Xbox *ref_slice_Mat4Xbox_get(const struct ref_slice_Mat4Xbox *slice,
                                              uintptr_t idx);

uintptr_t ref_slice_Mat4Xbox_len(const struct ref_slice_Mat4Xbox *slice);

const struct MatExtraXbox *ref_slice_MatExtraXbox_get(const struct ref_slice_MatExtraXbox *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_MatExtraXbox_len(const struct ref_slice_MatExtraXbox *slice);

const struct ModelInfoXbox *ref_slice_ModelInfoXbox_get(const struct ref_slice_ModelInfoXbox *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_ModelInfoXbox_len(const struct ref_slice_ModelInfoXbox *slice);

const struct Obj0Xbox *ref_slice_Obj0Xbox_get(const struct ref_slice_Obj0Xbox *slice,
                                              uintptr_t idx);

uintptr_t ref_slice_Obj0Xbox_len(const struct ref_slice_Obj0Xbox *slice);

const struct ObjAXbox *ref_slice_ObjAXbox_get(const struct ref_slice_ObjAXbox *slice,
                                              uintptr_t idx);

uintptr_t ref_slice_ObjAXbox_len(const struct ref_slice_ObjAXbox *slice);

const struct PFieldInfoXbox *ref_slice_PFieldInfoXbox_get(const struct ref_slice_PFieldInfoXbox *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_PFieldInfoXbox_len(const struct ref_slice_PFieldInfoXbox *slice);

const struct RadiosityValsInfoXbox *ref_slice_RadiosityValsInfoXbox_get(const struct ref_slice_RadiosityValsInfoXbox *slice,
                                                                        uintptr_t idx);

uintptr_t ref_slice_RadiosityValsInfoXbox_len(const struct ref_slice_RadiosityValsInfoXbox *slice);

const struct ShapeInfoXbox *ref_slice_ShapeInfoXbox_get(const struct ref_slice_ShapeInfoXbox *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_ShapeInfoXbox_len(const struct ref_slice_ShapeInfoXbox *slice);

const struct StringKeysValXbox *ref_slice_StringKeysValXbox_get(const struct ref_slice_StringKeysValXbox *slice,
                                                                uintptr_t idx);

uintptr_t ref_slice_StringKeysValXbox_len(const struct ref_slice_StringKeysValXbox *slice);

const struct SubBlocksBlockHeaderXbox *ref_slice_SubBlocksBlockHeaderXbox_get(const struct ref_slice_SubBlocksBlockHeaderXbox *slice,
                                                                              uintptr_t idx);

uintptr_t ref_slice_SubBlocksBlockHeaderXbox_len(const struct ref_slice_SubBlocksBlockHeaderXbox *slice);

const struct TextureInfoXbox *ref_slice_TextureInfoXbox_get(const struct ref_slice_TextureInfoXbox *slice,
                                                            uintptr_t idx);

uintptr_t ref_slice_TextureInfoXbox_len(const struct ref_slice_TextureInfoXbox *slice);

const struct VBuffInfoXbox *ref_slice_VBuffInfoXbox_get(const struct ref_slice_VBuffInfoXbox *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_VBuffInfoXbox_len(const struct ref_slice_VBuffInfoXbox *slice);

const struct Obj3Xbox *ref_slice_Obj3Xbox_get(const struct ref_slice_Obj3Xbox *slice,
                                              uintptr_t idx);

uintptr_t ref_slice_Obj3Xbox_len(const struct ref_slice_Obj3Xbox *slice);

const struct Obj5ValXbox *ref_slice_Obj5ValXbox_get(const struct ref_slice_Obj5ValXbox *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_Obj5ValXbox_len(const struct ref_slice_Obj5ValXbox *slice);

const struct SSAValXbox *ref_slice_SSAValXbox_get(const struct ref_slice_SSAValXbox *slice,
                                                  uintptr_t idx);

uintptr_t ref_slice_SSAValXbox_len(const struct ref_slice_SSAValXbox *slice);

const struct TypeFieldXbox *ref_slice_TypeFieldXbox_get(const struct ref_slice_TypeFieldXbox *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_TypeFieldXbox_len(const struct ref_slice_TypeFieldXbox *slice);

const struct FoliageValXbox *ref_slice_FoliageValXbox_get(const struct ref_slice_FoliageValXbox *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_FoliageValXbox_len(const struct ref_slice_FoliageValXbox *slice);

const U32Xbox *ref_slice_U32Xbox_get(const struct ref_slice_U32Xbox *slice, uintptr_t idx);

uintptr_t ref_slice_U32Xbox_len(const struct ref_slice_U32Xbox *slice);

const struct WeightXbox *ref_slice_WeightXbox_get(const struct ref_slice_WeightXbox *slice,
                                                  uintptr_t idx);

uintptr_t ref_slice_WeightXbox_len(const struct ref_slice_WeightXbox *slice);

const struct AtlasUVValXbox *ref_slice_AtlasUVValXbox_get(const struct ref_slice_AtlasUVValXbox *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_AtlasUVValXbox_len(const struct ref_slice_AtlasUVValXbox *slice);

const struct SprayInstanceXbox *ref_slice_SprayInstanceXbox_get(const struct ref_slice_SprayInstanceXbox *slice,
                                                                uintptr_t idx);

uintptr_t ref_slice_SprayInstanceXbox_len(const struct ref_slice_SprayInstanceXbox *slice);

const struct SprayValXbox *ref_slice_SprayValXbox_get(const struct ref_slice_SprayValXbox *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_SprayValXbox_len(const struct ref_slice_SprayValXbox *slice);

const struct TRSXbox *ref_slice_TRSXbox_get(const struct ref_slice_TRSXbox *slice, uintptr_t idx);

uintptr_t ref_slice_TRSXbox_len(const struct ref_slice_TRSXbox *slice);

const f32Xbox *ref_slice_f32Xbox_get(const struct ref_slice_f32Xbox *slice, uintptr_t idx);

uintptr_t ref_slice_f32Xbox_len(const struct ref_slice_f32Xbox *slice);

const i16Xbox *ref_slice_i16Xbox_get(const struct ref_slice_i16Xbox *slice, uintptr_t idx);

uintptr_t ref_slice_i16Xbox_len(const struct ref_slice_i16Xbox *slice);

const struct CrowdValXbox *ref_slice_CrowdValXbox_get(const struct ref_slice_CrowdValXbox *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_CrowdValXbox_len(const struct ref_slice_CrowdValXbox *slice);

const struct RotationPolar32Xbox *ref_slice_RotationPolar32Xbox_get(const struct ref_slice_RotationPolar32Xbox *slice,
                                                                    uintptr_t idx);

uintptr_t ref_slice_RotationPolar32Xbox_len(const struct ref_slice_RotationPolar32Xbox *slice);

const struct RotationStraight16Xbox *ref_slice_RotationStraight16Xbox_get(const struct ref_slice_RotationStraight16Xbox *slice,
                                                                          uintptr_t idx);

uintptr_t ref_slice_RotationStraight16Xbox_len(const struct ref_slice_RotationStraight16Xbox *slice);

const struct RotationThreeComp24Xbox *ref_slice_RotationThreeComp24Xbox_get(const struct ref_slice_RotationThreeComp24Xbox *slice,
                                                                            uintptr_t idx);

uintptr_t ref_slice_RotationThreeComp24Xbox_len(const struct ref_slice_RotationThreeComp24Xbox *slice);

const struct RotationThreeComp40Xbox *ref_slice_RotationThreeComp40Xbox_get(const struct ref_slice_RotationThreeComp40Xbox *slice,
                                                                            uintptr_t idx);

uintptr_t ref_slice_RotationThreeComp40Xbox_len(const struct ref_slice_RotationThreeComp40Xbox *slice);

const struct RotationThreeComp48Xbox *ref_slice_RotationThreeComp48Xbox_get(const struct ref_slice_RotationThreeComp48Xbox *slice,
                                                                            uintptr_t idx);

uintptr_t ref_slice_RotationThreeComp48Xbox_len(const struct ref_slice_RotationThreeComp48Xbox *slice);

const struct RotationUncompressedXbox *ref_slice_RotationUncompressedXbox_get(const struct ref_slice_RotationUncompressedXbox *slice,
                                                                              uintptr_t idx);

uintptr_t ref_slice_RotationUncompressedXbox_len(const struct ref_slice_RotationUncompressedXbox *slice);

u32Xbox *mut_slice_u32Xbox_get(struct mut_slice_u32Xbox *slice, uintptr_t idx);

uintptr_t mut_slice_u32Xbox_len(const struct slice_u32Xbox *slice);

struct AnimationBlockInfoXbox *mut_slice_AnimationBlockInfoXbox_get(struct mut_slice_AnimationBlockInfoXbox *slice,
                                                                    uintptr_t idx);

uintptr_t mut_slice_AnimationBlockInfoXbox_len(const struct slice_AnimationBlockInfoXbox *slice);

struct AnimationInfoXbox *mut_slice_AnimationInfoXbox_get(struct mut_slice_AnimationInfoXbox *slice,
                                                          uintptr_t idx);

uintptr_t mut_slice_AnimationInfoXbox_len(const struct slice_AnimationInfoXbox *slice);

struct BufferInfoXbox *mut_slice_BufferInfoXbox_get(struct mut_slice_BufferInfoXbox *slice,
                                                    uintptr_t idx);

uintptr_t mut_slice_BufferInfoXbox_len(const struct slice_BufferInfoXbox *slice);

struct EffectInfoXbox *mut_slice_EffectInfoXbox_get(struct mut_slice_EffectInfoXbox *slice,
                                                    uintptr_t idx);

uintptr_t mut_slice_EffectInfoXbox_len(const struct slice_EffectInfoXbox *slice);

struct FoliageInfoXbox *mut_slice_FoliageInfoXbox_get(struct mut_slice_FoliageInfoXbox *slice,
                                                      uintptr_t idx);

uintptr_t mut_slice_FoliageInfoXbox_len(const struct slice_FoliageInfoXbox *slice);

struct GFXBlockInfoXbox *mut_slice_GFXBlockInfoXbox_get(struct mut_slice_GFXBlockInfoXbox *slice,
                                                        uintptr_t idx);

uintptr_t mut_slice_GFXBlockInfoXbox_len(const struct slice_GFXBlockInfoXbox *slice);

struct HkConstraintDataXbox *mut_slice_HkConstraintDataXbox_get(struct mut_slice_HkConstraintDataXbox *slice,
                                                                uintptr_t idx);

uintptr_t mut_slice_HkConstraintDataXbox_len(const struct slice_HkConstraintDataXbox *slice);

struct HkConstraintInfoXbox *mut_slice_HkConstraintInfoXbox_get(struct mut_slice_HkConstraintInfoXbox *slice,
                                                                uintptr_t idx);

uintptr_t mut_slice_HkConstraintInfoXbox_len(const struct slice_HkConstraintInfoXbox *slice);

struct HkShapeInfoXbox *mut_slice_HkShapeInfoXbox_get(struct mut_slice_HkShapeInfoXbox *slice,
                                                      uintptr_t idx);

uintptr_t mut_slice_HkShapeInfoXbox_len(const struct slice_HkShapeInfoXbox *slice);

struct IBuffInfoXbox *mut_slice_IBuffInfoXbox_get(struct mut_slice_IBuffInfoXbox *slice,
                                                  uintptr_t idx);

uintptr_t mut_slice_IBuffInfoXbox_len(const struct slice_IBuffInfoXbox *slice);

struct Mat1Xbox *mut_slice_Mat1Xbox_get(struct mut_slice_Mat1Xbox *slice, uintptr_t idx);

uintptr_t mut_slice_Mat1Xbox_len(const struct slice_Mat1Xbox *slice);

struct Mat2Xbox *mut_slice_Mat2Xbox_get(struct mut_slice_Mat2Xbox *slice, uintptr_t idx);

uintptr_t mut_slice_Mat2Xbox_len(const struct slice_Mat2Xbox *slice);

struct Mat3Xbox *mut_slice_Mat3Xbox_get(struct mut_slice_Mat3Xbox *slice, uintptr_t idx);

uintptr_t mut_slice_Mat3Xbox_len(const struct slice_Mat3Xbox *slice);

struct Mat4Xbox *mut_slice_Mat4Xbox_get(struct mut_slice_Mat4Xbox *slice, uintptr_t idx);

uintptr_t mut_slice_Mat4Xbox_len(const struct slice_Mat4Xbox *slice);

struct MatExtraXbox *mut_slice_MatExtraXbox_get(struct mut_slice_MatExtraXbox *slice,
                                                uintptr_t idx);

uintptr_t mut_slice_MatExtraXbox_len(const struct slice_MatExtraXbox *slice);

struct ModelInfoXbox *mut_slice_ModelInfoXbox_get(struct mut_slice_ModelInfoXbox *slice,
                                                  uintptr_t idx);

uintptr_t mut_slice_ModelInfoXbox_len(const struct slice_ModelInfoXbox *slice);

struct Obj0Xbox *mut_slice_Obj0Xbox_get(struct mut_slice_Obj0Xbox *slice, uintptr_t idx);

uintptr_t mut_slice_Obj0Xbox_len(const struct slice_Obj0Xbox *slice);

struct ObjAXbox *mut_slice_ObjAXbox_get(struct mut_slice_ObjAXbox *slice, uintptr_t idx);

uintptr_t mut_slice_ObjAXbox_len(const struct slice_ObjAXbox *slice);

struct PFieldInfoXbox *mut_slice_PFieldInfoXbox_get(struct mut_slice_PFieldInfoXbox *slice,
                                                    uintptr_t idx);

uintptr_t mut_slice_PFieldInfoXbox_len(const struct slice_PFieldInfoXbox *slice);

struct RadiosityValsInfoXbox *mut_slice_RadiosityValsInfoXbox_get(struct mut_slice_RadiosityValsInfoXbox *slice,
                                                                  uintptr_t idx);

uintptr_t mut_slice_RadiosityValsInfoXbox_len(const struct slice_RadiosityValsInfoXbox *slice);

struct ShapeInfoXbox *mut_slice_ShapeInfoXbox_get(struct mut_slice_ShapeInfoXbox *slice,
                                                  uintptr_t idx);

uintptr_t mut_slice_ShapeInfoXbox_len(const struct slice_ShapeInfoXbox *slice);

struct TextureInfoXbox *mut_slice_TextureInfoXbox_get(struct mut_slice_TextureInfoXbox *slice,
                                                      uintptr_t idx);

uintptr_t mut_slice_TextureInfoXbox_len(const struct slice_TextureInfoXbox *slice);

struct VBuffInfoXbox *mut_slice_VBuffInfoXbox_get(struct mut_slice_VBuffInfoXbox *slice,
                                                  uintptr_t idx);

uintptr_t mut_slice_VBuffInfoXbox_len(const struct slice_VBuffInfoXbox *slice);

const struct HkConstraintRefXbox *Option_HkConstraintRefXbox_get(const struct Option_HkConstraintRefXbox *slice);

const struct ShapeExtraRefXbox *Option_ShapeExtraRefXbox_get(const struct Option_ShapeExtraRefXbox *slice);

const struct AtlasUVRefXbox *Option_AtlasUVRefXbox_get(const struct Option_AtlasUVRefXbox *slice);

const struct BlocksRefXbox *Option_BlocksRefXbox_get(const struct Option_BlocksRefXbox *slice);

const struct CrowdRefXbox *Option_CrowdRefXbox_get(const struct Option_CrowdRefXbox *slice);

const PFieldsRefXbox *Option_PFieldsRefXbox_get(const struct Option_PFieldsRefXbox *slice);

const struct SprayRefXbox *Option_SprayRefXbox_get(const struct Option_SprayRefXbox *slice);

const struct IndexBufferRefPs3 *IndexMap_u32__IndexBufferRefPs3_get(const struct IndexMap_u32__IndexBufferRefPs3 *map,
                                                                    const uint32_t *key);

uintptr_t IndexMap_u32__IndexBufferRefPs3_len(const struct IndexMap_u32__IndexBufferRefPs3 *map);

void IndexMap_u32__IndexBufferRefPs3_keys(const struct IndexMap_u32__IndexBufferRefPs3 *map,
                                          struct mut_slice_u32 *keys);

const struct VertexBufferRefPs3 *IndexMap_u32__VertexBufferRefPs3_get(const struct IndexMap_u32__VertexBufferRefPs3 *map,
                                                                      const uint32_t *key);

uintptr_t IndexMap_u32__VertexBufferRefPs3_len(const struct IndexMap_u32__VertexBufferRefPs3 *map);

void IndexMap_u32__VertexBufferRefPs3_keys(const struct IndexMap_u32__VertexBufferRefPs3 *map,
                                           struct mut_slice_u32 *keys);

const struct MatRefPs3 *IndexMap_u32__MatRefPs3_get(const struct IndexMap_u32__MatRefPs3 *map,
                                                    const uint32_t *key);

uintptr_t IndexMap_u32__MatRefPs3_len(const struct IndexMap_u32__MatRefPs3 *map);

void IndexMap_u32__MatRefPs3_keys(const struct IndexMap_u32__MatRefPs3 *map,
                                  struct mut_slice_u32 *keys);

const struct ModelRefPs3 *IndexMap_u32__ModelRefPs3_get(const struct IndexMap_u32__ModelRefPs3 *map,
                                                        const uint32_t *key);

uintptr_t IndexMap_u32__ModelRefPs3_len(const struct IndexMap_u32__ModelRefPs3 *map);

void IndexMap_u32__ModelRefPs3_keys(const struct IndexMap_u32__ModelRefPs3 *map,
                                    struct mut_slice_u32 *keys);

const struct IBuffInfoPs3 *const *IndexMap_u32_______IBuffInfoPs3_get(const struct IndexMap_u32_______IBuffInfoPs3 *map,
                                                                      const uint32_t *key);

uintptr_t IndexMap_u32_______IBuffInfoPs3_len(const struct IndexMap_u32_______IBuffInfoPs3 *map);

void IndexMap_u32_______IBuffInfoPs3_keys(const struct IndexMap_u32_______IBuffInfoPs3 *map,
                                          struct mut_slice_u32 *keys);

const struct VBuffInfoPs3 *const *IndexMap_u32_______VBuffInfoPs3_get(const struct IndexMap_u32_______VBuffInfoPs3 *map,
                                                                      const uint32_t *key);

uintptr_t IndexMap_u32_______VBuffInfoPs3_len(const struct IndexMap_u32_______VBuffInfoPs3 *map);

void IndexMap_u32_______VBuffInfoPs3_keys(const struct IndexMap_u32_______VBuffInfoPs3 *map,
                                          struct mut_slice_u32 *keys);

const struct AnimationRefPs3 *IndexMap_u32__AnimationRefPs3_get(const struct IndexMap_u32__AnimationRefPs3 *map,
                                                                const uint32_t *key);

uintptr_t IndexMap_u32__AnimationRefPs3_len(const struct IndexMap_u32__AnimationRefPs3 *map);

void IndexMap_u32__AnimationRefPs3_keys(const struct IndexMap_u32__AnimationRefPs3 *map,
                                        struct mut_slice_u32 *keys);

const struct DataRefPs3 *IndexMap_u32__DataRefPs3_get(const struct IndexMap_u32__DataRefPs3 *map,
                                                      const uint32_t *key);

uintptr_t IndexMap_u32__DataRefPs3_len(const struct IndexMap_u32__DataRefPs3 *map);

void IndexMap_u32__DataRefPs3_keys(const struct IndexMap_u32__DataRefPs3 *map,
                                   struct mut_slice_u32 *keys);

const struct EffectRefPs3 *IndexMap_u32__EffectRefPs3_get(const struct IndexMap_u32__EffectRefPs3 *map,
                                                          const uint32_t *key);

uintptr_t IndexMap_u32__EffectRefPs3_len(const struct IndexMap_u32__EffectRefPs3 *map);

void IndexMap_u32__EffectRefPs3_keys(const struct IndexMap_u32__EffectRefPs3 *map,
                                     struct mut_slice_u32 *keys);

const struct LangStringsRefPs3 *IndexMap_u32__LangStringsRefPs3_get(const struct IndexMap_u32__LangStringsRefPs3 *map,
                                                                    const uint32_t *key);

uintptr_t IndexMap_u32__LangStringsRefPs3_len(const struct IndexMap_u32__LangStringsRefPs3 *map);

void IndexMap_u32__LangStringsRefPs3_keys(const struct IndexMap_u32__LangStringsRefPs3 *map,
                                          struct mut_slice_u32 *keys);

const LuaRefPs3 *IndexMap_u32__LuaRefPs3_get(const struct IndexMap_u32__LuaRefPs3 *map,
                                             const uint32_t *key);

uintptr_t IndexMap_u32__LuaRefPs3_len(const struct IndexMap_u32__LuaRefPs3 *map);

void IndexMap_u32__LuaRefPs3_keys(const struct IndexMap_u32__LuaRefPs3 *map,
                                  struct mut_slice_u32 *keys);

const struct ObjRefPs3 *IndexMap_u32__ObjRefPs3_get(const struct IndexMap_u32__ObjRefPs3 *map,
                                                    const uint32_t *key);

uintptr_t IndexMap_u32__ObjRefPs3_len(const struct IndexMap_u32__ObjRefPs3 *map);

void IndexMap_u32__ObjRefPs3_keys(const struct IndexMap_u32__ObjRefPs3 *map,
                                  struct mut_slice_u32 *keys);

const struct RadiosityValsRefPs3 *IndexMap_u32__RadiosityValsRefPs3_get(const struct IndexMap_u32__RadiosityValsRefPs3 *map,
                                                                        const uint32_t *key);

uintptr_t IndexMap_u32__RadiosityValsRefPs3_len(const struct IndexMap_u32__RadiosityValsRefPs3 *map);

void IndexMap_u32__RadiosityValsRefPs3_keys(const struct IndexMap_u32__RadiosityValsRefPs3 *map,
                                            struct mut_slice_u32 *keys);

const struct SSARefPs3 *IndexMap_u32__SSARefPs3_get(const struct IndexMap_u32__SSARefPs3 *map,
                                                    const uint32_t *key);

uintptr_t IndexMap_u32__SSARefPs3_len(const struct IndexMap_u32__SSARefPs3 *map);

void IndexMap_u32__SSARefPs3_keys(const struct IndexMap_u32__SSARefPs3 *map,
                                  struct mut_slice_u32 *keys);

const struct TextureRefPs3 *IndexMap_u32__TextureRefPs3_get(const struct IndexMap_u32__TextureRefPs3 *map,
                                                            const uint32_t *key);

uintptr_t IndexMap_u32__TextureRefPs3_len(const struct IndexMap_u32__TextureRefPs3 *map);

void IndexMap_u32__TextureRefPs3_keys(const struct IndexMap_u32__TextureRefPs3 *map,
                                      struct mut_slice_u32 *keys);

const struct TypeRefPs3 *IndexMap_u32__TypeRefPs3_get(const struct IndexMap_u32__TypeRefPs3 *map,
                                                      const uint32_t *key);

uintptr_t IndexMap_u32__TypeRefPs3_len(const struct IndexMap_u32__TypeRefPs3 *map);

void IndexMap_u32__TypeRefPs3_keys(const struct IndexMap_u32__TypeRefPs3 *map,
                                   struct mut_slice_u32 *keys);

const struct slice_FoliageRefPs3 *IndexMap_u32__slice_FoliageRefPs3_get(const struct IndexMap_u32__slice_FoliageRefPs3 *map,
                                                                        const uint32_t *key);

uintptr_t IndexMap_u32__slice_FoliageRefPs3_len(const struct IndexMap_u32__slice_FoliageRefPs3 *map);

void IndexMap_u32__slice_FoliageRefPs3_keys(const struct IndexMap_u32__slice_FoliageRefPs3 *map,
                                            struct mut_slice_u32 *keys);

const struct BaseTypeRefPs3 *IndexMap_u32__BaseTypeRefPs3_get(const struct IndexMap_u32__BaseTypeRefPs3 *map,
                                                              const uint32_t *key);

uintptr_t IndexMap_u32__BaseTypeRefPs3_len(const struct IndexMap_u32__BaseTypeRefPs3 *map);

void IndexMap_u32__BaseTypeRefPs3_keys(const struct IndexMap_u32__BaseTypeRefPs3 *map,
                                       struct mut_slice_u32 *keys);

const struct ref_slice_u16Ps3 *IndexMap_u32__ref_slice_u16Ps3_get(const struct IndexMap_u32__ref_slice_u16Ps3 *map,
                                                                  const uint32_t *key);

uintptr_t IndexMap_u32__ref_slice_u16Ps3_len(const struct IndexMap_u32__ref_slice_u16Ps3 *map);

void IndexMap_u32__ref_slice_u16Ps3_keys(const struct IndexMap_u32__ref_slice_u16Ps3 *map,
                                         struct mut_slice_u32 *keys);

const struct BlockRefPs3 *slice_BlockRefPs3_get(const struct slice_BlockRefPs3 *slice,
                                                uintptr_t idx);

uintptr_t slice_BlockRefPs3_len(const struct slice_BlockRefPs3 *slice);

const struct ShapeRefPs3 *slice_ShapeRefPs3_get(const struct slice_ShapeRefPs3 *slice,
                                                uintptr_t idx);

uintptr_t slice_ShapeRefPs3_len(const struct slice_ShapeRefPs3 *slice);

const struct HkShapeRefPs3 *owned_slice_HkShapeRefPs3_get(const struct slice_HkShapeRefPs3 *slice,
                                                          uintptr_t idx);

uintptr_t owned_slice_HkShapeRefPs3_len(const struct slice_HkShapeRefPs3 *slice);

const struct FoliageRefPs3 *slice_FoliageRefPs3_get(const struct slice_FoliageRefPs3 *slice,
                                                    uintptr_t idx);

uintptr_t slice_FoliageRefPs3_len(const struct slice_FoliageRefPs3 *slice);

const struct ref_slice_u16Ps3 *slice_ref_slice_u16Ps3_get(const struct slice_ref_slice_u16Ps3 *slice,
                                                          uintptr_t idx);

uintptr_t slice_ref_slice_u16Ps3_len(const struct slice_ref_slice_u16Ps3 *slice);

const struct BlockValRefPs3 *slice_BlockValRefPs3_get(const struct slice_BlockValRefPs3 *slice,
                                                      uintptr_t idx);

uintptr_t slice_BlockValRefPs3_len(const struct slice_BlockValRefPs3 *slice);

const struct CrowdItemRefPs3 *slice_CrowdItemRefPs3_get(const struct slice_CrowdItemRefPs3 *slice,
                                                        uintptr_t idx);

uintptr_t slice_CrowdItemRefPs3_len(const struct slice_CrowdItemRefPs3 *slice);

const struct HkConstraintBoneRefPs3 *slice_HkConstraintBoneRefPs3_get(const struct slice_HkConstraintBoneRefPs3 *slice,
                                                                      uintptr_t idx);

uintptr_t slice_HkConstraintBoneRefPs3_len(const struct slice_HkConstraintBoneRefPs3 *slice);

const struct BlockValARefPs3 *slice_BlockValARefPs3_get(const struct slice_BlockValARefPs3 *slice,
                                                        uintptr_t idx);

uintptr_t slice_BlockValARefPs3_len(const struct slice_BlockValARefPs3 *slice);

const struct Obj1RefPs3 *slice_Obj1RefPs3_get(const struct slice_Obj1RefPs3 *slice, uintptr_t idx);

uintptr_t slice_Obj1RefPs3_len(const struct slice_Obj1RefPs3 *slice);

const struct HkShapeRefPs3 *slice_HkShapeRefPs3_get(const struct slice_HkShapeRefPs3 *slice,
                                                    uintptr_t idx);

uintptr_t slice_HkShapeRefPs3_len(const struct slice_HkShapeRefPs3 *slice);

const struct BoundingBoxPs3 *ref_slice_BoundingBoxPs3_get(const struct ref_slice_BoundingBoxPs3 *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_BoundingBoxPs3_len(const struct ref_slice_BoundingBoxPs3 *slice);

const struct BufferInfoPs3 *ref_slice_BufferInfoPs3_get(const struct ref_slice_BufferInfoPs3 *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_BufferInfoPs3_len(const struct ref_slice_BufferInfoPs3 *slice);

const CrcPs3 *ref_slice_CrcPs3_get(const struct ref_slice_CrcPs3 *slice, uintptr_t idx);

uintptr_t ref_slice_CrcPs3_len(const struct ref_slice_CrcPs3 *slice);

const struct HkConstraintDataPs3 *ref_slice_HkConstraintDataPs3_get(const struct ref_slice_HkConstraintDataPs3 *slice,
                                                                    uintptr_t idx);

uintptr_t ref_slice_HkConstraintDataPs3_len(const struct ref_slice_HkConstraintDataPs3 *slice);

const struct Key2Ps3 *ref_slice_Key2Ps3_get(const struct ref_slice_Key2Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_Key2Ps3_len(const struct ref_slice_Key2Ps3 *slice);

const struct Matrix4x4Ps3 *ref_slice_Matrix4x4Ps3_get(const struct ref_slice_Matrix4x4Ps3 *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_Matrix4x4Ps3_len(const struct ref_slice_Matrix4x4Ps3 *slice);

const struct Vector3Ps3 *ref_slice_Vector3Ps3_get(const struct ref_slice_Vector3Ps3 *slice,
                                                  uintptr_t idx);

uintptr_t ref_slice_Vector3Ps3_len(const struct ref_slice_Vector3Ps3 *slice);

const struct Vector4Ps3 *ref_slice_Vector4Ps3_get(const struct ref_slice_Vector4Ps3 *slice,
                                                  uintptr_t idx);

uintptr_t ref_slice_Vector4Ps3_len(const struct ref_slice_Vector4Ps3 *slice);

const i32Ps3 *ref_slice_i32Ps3_get(const struct ref_slice_i32Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_i32Ps3_len(const struct ref_slice_i32Ps3 *slice);

const u16Ps3 *ref_slice_u16Ps3_get(const struct ref_slice_u16Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_u16Ps3_len(const struct ref_slice_u16Ps3 *slice);

const u32Ps3 *ref_slice_u32Ps3_get(const struct ref_slice_u32Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_u32Ps3_len(const struct ref_slice_u32Ps3 *slice);

const struct BlockValAPs3 *ref_slice_BlockValAPs3_get(const struct ref_slice_BlockValAPs3 *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_BlockValAPs3_len(const struct ref_slice_BlockValAPs3 *slice);

const struct BlockValBPs3 *ref_slice_BlockValBPs3_get(const struct ref_slice_BlockValBPs3 *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_BlockValBPs3_len(const struct ref_slice_BlockValBPs3 *slice);

const struct AnimationBlockInfoPs3 *ref_slice_AnimationBlockInfoPs3_get(const struct ref_slice_AnimationBlockInfoPs3 *slice,
                                                                        uintptr_t idx);

uintptr_t ref_slice_AnimationBlockInfoPs3_len(const struct ref_slice_AnimationBlockInfoPs3 *slice);

const struct AnimationInfoPs3 *ref_slice_AnimationInfoPs3_get(const struct ref_slice_AnimationInfoPs3 *slice,
                                                              uintptr_t idx);

uintptr_t ref_slice_AnimationInfoPs3_len(const struct ref_slice_AnimationInfoPs3 *slice);

const struct AssetHandlePs3 *ref_slice_AssetHandlePs3_get(const struct ref_slice_AssetHandlePs3 *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_AssetHandlePs3_len(const struct ref_slice_AssetHandlePs3 *slice);

const struct BlockAValPs3 *ref_slice_BlockAValPs3_get(const struct ref_slice_BlockAValPs3 *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_BlockAValPs3_len(const struct ref_slice_BlockAValPs3 *slice);

const struct EffectInfoPs3 *ref_slice_EffectInfoPs3_get(const struct ref_slice_EffectInfoPs3 *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_EffectInfoPs3_len(const struct ref_slice_EffectInfoPs3 *slice);

const struct FoliageInfoPs3 *ref_slice_FoliageInfoPs3_get(const struct ref_slice_FoliageInfoPs3 *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_FoliageInfoPs3_len(const struct ref_slice_FoliageInfoPs3 *slice);

const struct GFXBlockInfoPs3 *ref_slice_GFXBlockInfoPs3_get(const struct ref_slice_GFXBlockInfoPs3 *slice,
                                                            uintptr_t idx);

uintptr_t ref_slice_GFXBlockInfoPs3_len(const struct ref_slice_GFXBlockInfoPs3 *slice);

const struct HkConstraintInfoPs3 *ref_slice_HkConstraintInfoPs3_get(const struct ref_slice_HkConstraintInfoPs3 *slice,
                                                                    uintptr_t idx);

uintptr_t ref_slice_HkConstraintInfoPs3_len(const struct ref_slice_HkConstraintInfoPs3 *slice);

const struct HkShapeInfoPs3 *ref_slice_HkShapeInfoPs3_get(const struct ref_slice_HkShapeInfoPs3 *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_HkShapeInfoPs3_len(const struct ref_slice_HkShapeInfoPs3 *slice);

const struct IBuffInfoPs3 *ref_slice_IBuffInfoPs3_get(const struct ref_slice_IBuffInfoPs3 *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_IBuffInfoPs3_len(const struct ref_slice_IBuffInfoPs3 *slice);

const struct Mat1Ps3 *ref_slice_Mat1Ps3_get(const struct ref_slice_Mat1Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_Mat1Ps3_len(const struct ref_slice_Mat1Ps3 *slice);

const struct Mat2Ps3 *ref_slice_Mat2Ps3_get(const struct ref_slice_Mat2Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_Mat2Ps3_len(const struct ref_slice_Mat2Ps3 *slice);

const struct Mat3Ps3 *ref_slice_Mat3Ps3_get(const struct ref_slice_Mat3Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_Mat3Ps3_len(const struct ref_slice_Mat3Ps3 *slice);

const struct Mat4Ps3 *ref_slice_Mat4Ps3_get(const struct ref_slice_Mat4Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_Mat4Ps3_len(const struct ref_slice_Mat4Ps3 *slice);

const struct MatExtraPs3 *ref_slice_MatExtraPs3_get(const struct ref_slice_MatExtraPs3 *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_MatExtraPs3_len(const struct ref_slice_MatExtraPs3 *slice);

const struct ModelInfoPs3 *ref_slice_ModelInfoPs3_get(const struct ref_slice_ModelInfoPs3 *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_ModelInfoPs3_len(const struct ref_slice_ModelInfoPs3 *slice);

const struct Obj0Ps3 *ref_slice_Obj0Ps3_get(const struct ref_slice_Obj0Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_Obj0Ps3_len(const struct ref_slice_Obj0Ps3 *slice);

const struct ObjAPs3 *ref_slice_ObjAPs3_get(const struct ref_slice_ObjAPs3 *slice, uintptr_t idx);

uintptr_t ref_slice_ObjAPs3_len(const struct ref_slice_ObjAPs3 *slice);

const struct PFieldInfoPs3 *ref_slice_PFieldInfoPs3_get(const struct ref_slice_PFieldInfoPs3 *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_PFieldInfoPs3_len(const struct ref_slice_PFieldInfoPs3 *slice);

const struct RadiosityValsInfoPs3 *ref_slice_RadiosityValsInfoPs3_get(const struct ref_slice_RadiosityValsInfoPs3 *slice,
                                                                      uintptr_t idx);

uintptr_t ref_slice_RadiosityValsInfoPs3_len(const struct ref_slice_RadiosityValsInfoPs3 *slice);

const struct ShapeInfoPs3 *ref_slice_ShapeInfoPs3_get(const struct ref_slice_ShapeInfoPs3 *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_ShapeInfoPs3_len(const struct ref_slice_ShapeInfoPs3 *slice);

const struct StringKeysValPs3 *ref_slice_StringKeysValPs3_get(const struct ref_slice_StringKeysValPs3 *slice,
                                                              uintptr_t idx);

uintptr_t ref_slice_StringKeysValPs3_len(const struct ref_slice_StringKeysValPs3 *slice);

const struct SubBlocksBlockHeaderPs3 *ref_slice_SubBlocksBlockHeaderPs3_get(const struct ref_slice_SubBlocksBlockHeaderPs3 *slice,
                                                                            uintptr_t idx);

uintptr_t ref_slice_SubBlocksBlockHeaderPs3_len(const struct ref_slice_SubBlocksBlockHeaderPs3 *slice);

const struct TextureInfoPs3 *ref_slice_TextureInfoPs3_get(const struct ref_slice_TextureInfoPs3 *slice,
                                                          uintptr_t idx);

uintptr_t ref_slice_TextureInfoPs3_len(const struct ref_slice_TextureInfoPs3 *slice);

const struct VBuffInfoPs3 *ref_slice_VBuffInfoPs3_get(const struct ref_slice_VBuffInfoPs3 *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_VBuffInfoPs3_len(const struct ref_slice_VBuffInfoPs3 *slice);

const struct Obj3Ps3 *ref_slice_Obj3Ps3_get(const struct ref_slice_Obj3Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_Obj3Ps3_len(const struct ref_slice_Obj3Ps3 *slice);

const struct Obj5ValPs3 *ref_slice_Obj5ValPs3_get(const struct ref_slice_Obj5ValPs3 *slice,
                                                  uintptr_t idx);

uintptr_t ref_slice_Obj5ValPs3_len(const struct ref_slice_Obj5ValPs3 *slice);

const struct SSAValPs3 *ref_slice_SSAValPs3_get(const struct ref_slice_SSAValPs3 *slice,
                                                uintptr_t idx);

uintptr_t ref_slice_SSAValPs3_len(const struct ref_slice_SSAValPs3 *slice);

const struct TypeFieldPs3 *ref_slice_TypeFieldPs3_get(const struct ref_slice_TypeFieldPs3 *slice,
                                                      uintptr_t idx);

uintptr_t ref_slice_TypeFieldPs3_len(const struct ref_slice_TypeFieldPs3 *slice);

const struct FoliageValPs3 *ref_slice_FoliageValPs3_get(const struct ref_slice_FoliageValPs3 *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_FoliageValPs3_len(const struct ref_slice_FoliageValPs3 *slice);

const U32Ps3 *ref_slice_U32Ps3_get(const struct ref_slice_U32Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_U32Ps3_len(const struct ref_slice_U32Ps3 *slice);

const struct WeightPs3 *ref_slice_WeightPs3_get(const struct ref_slice_WeightPs3 *slice,
                                                uintptr_t idx);

uintptr_t ref_slice_WeightPs3_len(const struct ref_slice_WeightPs3 *slice);

const struct AtlasUVValPs3 *ref_slice_AtlasUVValPs3_get(const struct ref_slice_AtlasUVValPs3 *slice,
                                                        uintptr_t idx);

uintptr_t ref_slice_AtlasUVValPs3_len(const struct ref_slice_AtlasUVValPs3 *slice);

const struct SprayInstancePs3 *ref_slice_SprayInstancePs3_get(const struct ref_slice_SprayInstancePs3 *slice,
                                                              uintptr_t idx);

uintptr_t ref_slice_SprayInstancePs3_len(const struct ref_slice_SprayInstancePs3 *slice);

const struct SprayValPs3 *ref_slice_SprayValPs3_get(const struct ref_slice_SprayValPs3 *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_SprayValPs3_len(const struct ref_slice_SprayValPs3 *slice);

const struct TRSPs3 *ref_slice_TRSPs3_get(const struct ref_slice_TRSPs3 *slice, uintptr_t idx);

uintptr_t ref_slice_TRSPs3_len(const struct ref_slice_TRSPs3 *slice);

const f32Ps3 *ref_slice_f32Ps3_get(const struct ref_slice_f32Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_f32Ps3_len(const struct ref_slice_f32Ps3 *slice);

const i16Ps3 *ref_slice_i16Ps3_get(const struct ref_slice_i16Ps3 *slice, uintptr_t idx);

uintptr_t ref_slice_i16Ps3_len(const struct ref_slice_i16Ps3 *slice);

const struct CrowdValPs3 *ref_slice_CrowdValPs3_get(const struct ref_slice_CrowdValPs3 *slice,
                                                    uintptr_t idx);

uintptr_t ref_slice_CrowdValPs3_len(const struct ref_slice_CrowdValPs3 *slice);

const struct RotationPolar32Ps3 *ref_slice_RotationPolar32Ps3_get(const struct ref_slice_RotationPolar32Ps3 *slice,
                                                                  uintptr_t idx);

uintptr_t ref_slice_RotationPolar32Ps3_len(const struct ref_slice_RotationPolar32Ps3 *slice);

const struct RotationStraight16Ps3 *ref_slice_RotationStraight16Ps3_get(const struct ref_slice_RotationStraight16Ps3 *slice,
                                                                        uintptr_t idx);

uintptr_t ref_slice_RotationStraight16Ps3_len(const struct ref_slice_RotationStraight16Ps3 *slice);

const struct RotationThreeComp24Ps3 *ref_slice_RotationThreeComp24Ps3_get(const struct ref_slice_RotationThreeComp24Ps3 *slice,
                                                                          uintptr_t idx);

uintptr_t ref_slice_RotationThreeComp24Ps3_len(const struct ref_slice_RotationThreeComp24Ps3 *slice);

const struct RotationThreeComp40Ps3 *ref_slice_RotationThreeComp40Ps3_get(const struct ref_slice_RotationThreeComp40Ps3 *slice,
                                                                          uintptr_t idx);

uintptr_t ref_slice_RotationThreeComp40Ps3_len(const struct ref_slice_RotationThreeComp40Ps3 *slice);

const struct RotationThreeComp48Ps3 *ref_slice_RotationThreeComp48Ps3_get(const struct ref_slice_RotationThreeComp48Ps3 *slice,
                                                                          uintptr_t idx);

uintptr_t ref_slice_RotationThreeComp48Ps3_len(const struct ref_slice_RotationThreeComp48Ps3 *slice);

const struct RotationUncompressedPs3 *ref_slice_RotationUncompressedPs3_get(const struct ref_slice_RotationUncompressedPs3 *slice,
                                                                            uintptr_t idx);

uintptr_t ref_slice_RotationUncompressedPs3_len(const struct ref_slice_RotationUncompressedPs3 *slice);

u32Ps3 *mut_slice_u32Ps3_get(struct mut_slice_u32Ps3 *slice, uintptr_t idx);

uintptr_t mut_slice_u32Ps3_len(const struct slice_u32Ps3 *slice);

struct AnimationBlockInfoPs3 *mut_slice_AnimationBlockInfoPs3_get(struct mut_slice_AnimationBlockInfoPs3 *slice,
                                                                  uintptr_t idx);

uintptr_t mut_slice_AnimationBlockInfoPs3_len(const struct slice_AnimationBlockInfoPs3 *slice);

struct AnimationInfoPs3 *mut_slice_AnimationInfoPs3_get(struct mut_slice_AnimationInfoPs3 *slice,
                                                        uintptr_t idx);

uintptr_t mut_slice_AnimationInfoPs3_len(const struct slice_AnimationInfoPs3 *slice);

struct BufferInfoPs3 *mut_slice_BufferInfoPs3_get(struct mut_slice_BufferInfoPs3 *slice,
                                                  uintptr_t idx);

uintptr_t mut_slice_BufferInfoPs3_len(const struct slice_BufferInfoPs3 *slice);

struct EffectInfoPs3 *mut_slice_EffectInfoPs3_get(struct mut_slice_EffectInfoPs3 *slice,
                                                  uintptr_t idx);

uintptr_t mut_slice_EffectInfoPs3_len(const struct slice_EffectInfoPs3 *slice);

struct FoliageInfoPs3 *mut_slice_FoliageInfoPs3_get(struct mut_slice_FoliageInfoPs3 *slice,
                                                    uintptr_t idx);

uintptr_t mut_slice_FoliageInfoPs3_len(const struct slice_FoliageInfoPs3 *slice);

struct GFXBlockInfoPs3 *mut_slice_GFXBlockInfoPs3_get(struct mut_slice_GFXBlockInfoPs3 *slice,
                                                      uintptr_t idx);

uintptr_t mut_slice_GFXBlockInfoPs3_len(const struct slice_GFXBlockInfoPs3 *slice);

struct HkConstraintDataPs3 *mut_slice_HkConstraintDataPs3_get(struct mut_slice_HkConstraintDataPs3 *slice,
                                                              uintptr_t idx);

uintptr_t mut_slice_HkConstraintDataPs3_len(const struct slice_HkConstraintDataPs3 *slice);

struct HkConstraintInfoPs3 *mut_slice_HkConstraintInfoPs3_get(struct mut_slice_HkConstraintInfoPs3 *slice,
                                                              uintptr_t idx);

uintptr_t mut_slice_HkConstraintInfoPs3_len(const struct slice_HkConstraintInfoPs3 *slice);

struct HkShapeInfoPs3 *mut_slice_HkShapeInfoPs3_get(struct mut_slice_HkShapeInfoPs3 *slice,
                                                    uintptr_t idx);

uintptr_t mut_slice_HkShapeInfoPs3_len(const struct slice_HkShapeInfoPs3 *slice);

struct IBuffInfoPs3 *mut_slice_IBuffInfoPs3_get(struct mut_slice_IBuffInfoPs3 *slice,
                                                uintptr_t idx);

uintptr_t mut_slice_IBuffInfoPs3_len(const struct slice_IBuffInfoPs3 *slice);

struct Mat1Ps3 *mut_slice_Mat1Ps3_get(struct mut_slice_Mat1Ps3 *slice, uintptr_t idx);

uintptr_t mut_slice_Mat1Ps3_len(const struct slice_Mat1Ps3 *slice);

struct Mat2Ps3 *mut_slice_Mat2Ps3_get(struct mut_slice_Mat2Ps3 *slice, uintptr_t idx);

uintptr_t mut_slice_Mat2Ps3_len(const struct slice_Mat2Ps3 *slice);

struct Mat3Ps3 *mut_slice_Mat3Ps3_get(struct mut_slice_Mat3Ps3 *slice, uintptr_t idx);

uintptr_t mut_slice_Mat3Ps3_len(const struct slice_Mat3Ps3 *slice);

struct Mat4Ps3 *mut_slice_Mat4Ps3_get(struct mut_slice_Mat4Ps3 *slice, uintptr_t idx);

uintptr_t mut_slice_Mat4Ps3_len(const struct slice_Mat4Ps3 *slice);

struct MatExtraPs3 *mut_slice_MatExtraPs3_get(struct mut_slice_MatExtraPs3 *slice, uintptr_t idx);

uintptr_t mut_slice_MatExtraPs3_len(const struct slice_MatExtraPs3 *slice);

struct ModelInfoPs3 *mut_slice_ModelInfoPs3_get(struct mut_slice_ModelInfoPs3 *slice,
                                                uintptr_t idx);

uintptr_t mut_slice_ModelInfoPs3_len(const struct slice_ModelInfoPs3 *slice);

struct Obj0Ps3 *mut_slice_Obj0Ps3_get(struct mut_slice_Obj0Ps3 *slice, uintptr_t idx);

uintptr_t mut_slice_Obj0Ps3_len(const struct slice_Obj0Ps3 *slice);

struct ObjAPs3 *mut_slice_ObjAPs3_get(struct mut_slice_ObjAPs3 *slice, uintptr_t idx);

uintptr_t mut_slice_ObjAPs3_len(const struct slice_ObjAPs3 *slice);

struct PFieldInfoPs3 *mut_slice_PFieldInfoPs3_get(struct mut_slice_PFieldInfoPs3 *slice,
                                                  uintptr_t idx);

uintptr_t mut_slice_PFieldInfoPs3_len(const struct slice_PFieldInfoPs3 *slice);

struct RadiosityValsInfoPs3 *mut_slice_RadiosityValsInfoPs3_get(struct mut_slice_RadiosityValsInfoPs3 *slice,
                                                                uintptr_t idx);

uintptr_t mut_slice_RadiosityValsInfoPs3_len(const struct slice_RadiosityValsInfoPs3 *slice);

struct ShapeInfoPs3 *mut_slice_ShapeInfoPs3_get(struct mut_slice_ShapeInfoPs3 *slice,
                                                uintptr_t idx);

uintptr_t mut_slice_ShapeInfoPs3_len(const struct slice_ShapeInfoPs3 *slice);

struct TextureInfoPs3 *mut_slice_TextureInfoPs3_get(struct mut_slice_TextureInfoPs3 *slice,
                                                    uintptr_t idx);

uintptr_t mut_slice_TextureInfoPs3_len(const struct slice_TextureInfoPs3 *slice);

struct VBuffInfoPs3 *mut_slice_VBuffInfoPs3_get(struct mut_slice_VBuffInfoPs3 *slice,
                                                uintptr_t idx);

uintptr_t mut_slice_VBuffInfoPs3_len(const struct slice_VBuffInfoPs3 *slice);

const struct HkConstraintRefPs3 *Option_HkConstraintRefPs3_get(const struct Option_HkConstraintRefPs3 *slice);

const struct ShapeExtraRefPs3 *Option_ShapeExtraRefPs3_get(const struct Option_ShapeExtraRefPs3 *slice);

const struct AtlasUVRefPs3 *Option_AtlasUVRefPs3_get(const struct Option_AtlasUVRefPs3 *slice);

const struct BlocksRefPs3 *Option_BlocksRefPs3_get(const struct Option_BlocksRefPs3 *slice);

const struct CrowdRefPs3 *Option_CrowdRefPs3_get(const struct Option_CrowdRefPs3 *slice);

const PFieldsRefPs3 *Option_PFieldsRefPs3_get(const struct Option_PFieldsRefPs3 *slice);

const struct SprayRefPs3 *Option_SprayRefPs3_get(const struct Option_SprayRefPs3 *slice);

uint32_t utils_hash_string(const char *s);
