#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint32_t LodInfo_UNKNOWN = 1;

constexpr static const uint32_t LodInfo_STATIC = 2;

constexpr static const uint32_t LodInfo_SKINNED = 4;

constexpr static const uint32_t LodInfo_PHYSICS = 8;

constexpr static const uint32_t LodInfo_BREAKABLE = 16;

constexpr static const uint32_t LodInfo_LOD0 = 32;

constexpr static const uint32_t LodInfo_LOD1 = 64;

constexpr static const uint32_t LodInfo_LOD2 = 128;

constexpr static const uint32_t LodInfo_LOD3 = 256;

enum class Version : uint8_t {
  Pc,
  Xbox,
  Ps3,
};

enum class BaseType : uint8_t {
  Crc = 0,
  GUID,
  Color,
  Vector2,
  Vector3,
  Vector4,
  Matrix4x4,
  Float,
  Int,
  Bool,
  String,
  StringList,
  ObjectList,
  NodeList,
  IntList,
  CrcList,
  WeightList,
  MatrixList,
  Err = 255,
};

struct AtlasUVPc;

struct AtlasUVPs3;

struct AtlasUVXbox;

struct BaseTypePc;

struct BaseTypePs3;

struct BaseTypeXbox;

struct CrowdItemPc;

struct CrowdItemPs3;

struct CrowdItemXbox;

struct CrowdPc;

struct CrowdPs3;

struct CrowdXbox;

struct GameObjsPc;

struct GameObjsPs3;

struct GameObjsXbox;

struct LevelPc;

struct LevelPs3;

struct LevelXbox;

struct ObjPc;

struct ObjPs3;

struct ObjXbox;

struct TypePc;

struct TypePs3;

struct TypeXbox;

using u32Pc = u32_le;

struct PakHeaderPc {
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
};

struct GameObjsHeaderPc {
  u32Pc const_;
  u32Pc types_num;
  u32Pc types_offset;
  u32Pc obj_num;
  u32Pc obj_offset;
  u32Pc z5;
  u32Pc z6;
  u32Pc z7;
};

using CrcPc = u32Pc;

using u16Pc = u16_le;

struct ObjHeaderPc {
  u32Pc layer;
  CrcPc key;
  u16Pc size;
  u16Pc z3;
  u32Pc z4;
};

struct TypeHeaderPc {
  CrcPc key;
  u32Pc size;
  u32Pc fields;
};

struct TypeFieldPc {
  CrcPc key;
  CrcPc kind;
  u32Pc offset;
};

using ColorPc = u32Pc;

using f32Pc = f32_le;

struct Vector2Pc {
  f32Pc x;
  f32Pc y;
};

struct Vector3Pc {
  f32Pc x;
  f32Pc y;
  f32Pc z;
};

struct Vector4Pc {
  f32Pc x;
  f32Pc y;
  f32Pc z;
  f32Pc w;
};

struct Matrix4x4Pc {
  Vector4Pc x;
  Vector4Pc y;
  Vector4Pc z;
  Vector4Pc w;
};

using i32Pc = i32_le;

using u8Pc = uint8_t;

struct WeightPc {
  u32Pc x;
  u8Pc a;
  u8Pc b;
  u8Pc c;
  u8Pc d;
};

using u32Xbox = u32_be;

struct GameObjsHeaderXbox {
  u32Xbox const_;
  u32Xbox types_num;
  u32Xbox types_offset;
  u32Xbox obj_num;
  u32Xbox obj_offset;
  u32Xbox z5;
  u32Xbox z6;
  u32Xbox z7;
};

using CrcXbox = u32Xbox;

using u16Xbox = u16_be;

struct ObjHeaderXbox {
  u32Xbox layer;
  CrcXbox key;
  u16Xbox size;
  u16Xbox z3;
  u32Xbox z4;
};

struct TypeHeaderXbox {
  CrcXbox key;
  u32Xbox size;
  u32Xbox fields;
};

struct TypeFieldXbox {
  CrcXbox key;
  CrcXbox kind;
  u32Xbox offset;
};

using ColorXbox = u32Xbox;

using f32Xbox = f32_be;

struct Vector2Xbox {
  f32Xbox x;
  f32Xbox y;
};

struct Vector3Xbox {
  f32Xbox x;
  f32Xbox y;
  f32Xbox z;
};

struct Vector4Xbox {
  f32Xbox x;
  f32Xbox y;
  f32Xbox z;
  f32Xbox w;
};

struct Matrix4x4Xbox {
  Vector4Xbox x;
  Vector4Xbox y;
  Vector4Xbox z;
  Vector4Xbox w;
};

using i32Xbox = i32_be;

using u8Xbox = uint8_t;

struct WeightXbox {
  u32Xbox x;
  u8Xbox a;
  u8Xbox b;
  u8Xbox c;
  u8Xbox d;
};

using u32Ps3 = u32_be;

struct GameObjsHeaderPs3 {
  u32Ps3 const_;
  u32Ps3 types_num;
  u32Ps3 types_offset;
  u32Ps3 obj_num;
  u32Ps3 obj_offset;
  u32Ps3 z5;
  u32Ps3 z6;
  u32Ps3 z7;
};

using CrcPs3 = u32Ps3;

using u16Ps3 = u16_be;

struct ObjHeaderPs3 {
  u32Ps3 layer;
  CrcPs3 key;
  u16Ps3 size;
  u16Ps3 z3;
  u32Ps3 z4;
};

struct TypeHeaderPs3 {
  CrcPs3 key;
  u32Ps3 size;
  u32Ps3 fields;
};

struct TypeFieldPs3 {
  CrcPs3 key;
  CrcPs3 kind;
  u32Ps3 offset;
};

using ColorPs3 = u32Ps3;

using f32Ps3 = f32_be;

struct Vector2Ps3 {
  f32Ps3 x;
  f32Ps3 y;
};

struct Vector3Ps3 {
  f32Ps3 x;
  f32Ps3 y;
  f32Ps3 z;
};

struct Vector4Ps3 {
  f32Ps3 x;
  f32Ps3 y;
  f32Ps3 z;
  f32Ps3 w;
};

struct Matrix4x4Ps3 {
  Vector4Ps3 x;
  Vector4Ps3 y;
  Vector4Ps3 z;
  Vector4Ps3 w;
};

using i32Ps3 = i32_be;

using u8Ps3 = uint8_t;

struct WeightPs3 {
  u32Ps3 x;
  u8Ps3 a;
  u8Ps3 b;
  u8Ps3 c;
  u8Ps3 d;
};

struct CrowdHeaderPc {
  u32Pc const0x65;
  u32Pc n;
};

struct AtlasUVValPc {
  CrcPc key;
  Vector4Pc vals;
};

struct CrowdHeaderXbox {
  u32Xbox const0x65;
  u32Xbox n;
};

struct AtlasUVValXbox {
  CrcXbox key;
  Vector4Xbox vals;
};

struct CrowdHeaderPs3 {
  u32Ps3 const0x65;
  u32Ps3 n;
};

struct AtlasUVValPs3 {
  CrcPs3 key;
  Vector4Ps3 vals;
};

extern "C" {

uint32_t hash_string_(const char *s);

PakHeaderPc lotrc_get_pak_header();

/// the returned version indicates if level contains a ptr to LevelPc, LevelXbox or LevelPs3
/// the obtained level ptr needs to be freed with the corresponding free function
void *lotrc_level_parse(const char *path, Version *version);

void lotrc_level_free_pc(LevelPc *level);

void lotrc_level_free_xbox(LevelXbox *level);

void lotrc_level_free_ps3(LevelPs3 *level);

const GameObjsPc *lotrc_level_get_level_block_pc(const LevelPc *level);

const GameObjsXbox *lotrc_level_get_level_block_xbox(const LevelXbox *level);

const GameObjsPs3 *lotrc_level_get_level_block_ps3(const LevelPs3 *level);

const GameObjsHeaderPc *lotrc_gameobjs_get_header_pc(const GameObjsPc *gameobjs);

size_t lotrc_gameobjs_get_types_num_pc(const GameObjsPc *gameobjs);

/// types is a caller allocated array for returning type keys
void lotrc_gameobjs_get_types_pc(const GameObjsPc *gameobjs, uint32_t *types);

const TypePc *lotrc_gameobjs_get_type_pc(const GameObjsPc *gameobjs, uint32_t key);

size_t lotrc_gameobjs_get_objs_num_pc(const GameObjsPc *gameobjs);

/// objs is a caller allocated array for returning obj keys
void lotrc_gameobjs_get_objs_pc(const GameObjsPc *gameobjs, uint32_t *objs);

const ObjPc *lotrc_gameobjs_get_obj_pc(const GameObjsPc *gameobjs, uint32_t key);

const ObjHeaderPc *lotrc_obj_get_header_pc(const ObjPc *obj);

size_t lotrc_obj_get_fields_num_pc(const ObjPc *obj);

/// fields is a caller allocated array for returning field keys
void lotrc_obj_get_fields_pc(const ObjPc *obj, uint32_t *fields);

const BaseTypePc *lotrc_obj_get_field_pc(const ObjPc *obj, uint32_t key);

const TypeHeaderPc *lotrc_type_get_header_pc(const TypePc *ty);

size_t lotrc_type_get_fields_num_pc(const TypePc *ty);

/// fields is a caller allocated array for returning fields
void lotrc_type_get_fields_pc(const TypePc *ty, TypeFieldPc *fields);

BaseType lotrc_basetype_get_type_pc(const BaseTypePc *basetype);

size_t lotrc_basetype_get_len_pc(const BaseTypePc *basetype);

/// sizes is a caller allocated array for returning string sizes
void lotrc_basetype_get_stringlist_sizes_pc(const BaseTypePc *basetype, size_t *sizes);

CrcPc lotrc_basetype_get_crc_pc(const BaseTypePc *basetype);

u32Pc lotrc_basetype_get_guid_pc(const BaseTypePc *basetype);

ColorPc lotrc_basetype_get_color_pc(const BaseTypePc *basetype);

const Vector2Pc *lotrc_basetype_get_vector2_pc(const BaseTypePc *basetype);

const Vector3Pc *lotrc_basetype_get_vector3_pc(const BaseTypePc *basetype);

const Vector4Pc *lotrc_basetype_get_vector4_pc(const BaseTypePc *basetype);

const Matrix4x4Pc *lotrc_basetype_get_matrix4x4_pc(const BaseTypePc *basetype);

f32Pc lotrc_basetype_get_float_pc(const BaseTypePc *basetype);

i32Pc lotrc_basetype_get_int_pc(const BaseTypePc *basetype);

bool lotrc_basetype_get_bool_pc(const BaseTypePc *basetype);

const char *lotrc_basetype_get_string_pc(const BaseTypePc *basetype);

const char *const *lotrc_basetype_get_stringlist_pc(const BaseTypePc *basetype);

const u32Pc *lotrc_basetype_get_objectlist_pc(const BaseTypePc *basetype);

const Vector4Pc *lotrc_basetype_get_nodelist_pc(const BaseTypePc *basetype);

const i32Pc *lotrc_basetype_get_intlist_pc(const BaseTypePc *basetype);

const CrcPc *lotrc_basetype_get_crclist_pc(const BaseTypePc *basetype);

const WeightPc *lotrc_basetype_get_weightlist_pc(const BaseTypePc *basetype);

const Matrix4x4Pc *lotrc_basetype_get_matrixlist_pc(const BaseTypePc *basetype);

const GameObjsHeaderXbox *lotrc_gameobjs_get_header_xbox(const GameObjsXbox *gameobjs);

size_t lotrc_gameobjs_get_types_num_xbox(const GameObjsXbox *gameobjs);

/// types is a caller allocated array for returning type keys
void lotrc_gameobjs_get_types_xbox(const GameObjsXbox *gameobjs, uint32_t *types);

const TypeXbox *lotrc_gameobjs_get_type_xbox(const GameObjsXbox *gameobjs, uint32_t key);

size_t lotrc_gameobjs_get_objs_num_xbox(const GameObjsXbox *gameobjs);

/// objs is a caller allocated array for returning obj keys
void lotrc_gameobjs_get_objs_xbox(const GameObjsXbox *gameobjs, uint32_t *objs);

const ObjXbox *lotrc_gameobjs_get_obj_xbox(const GameObjsXbox *gameobjs, uint32_t key);

const ObjHeaderXbox *lotrc_obj_get_header_xbox(const ObjXbox *obj);

size_t lotrc_obj_get_fields_num_xbox(const ObjXbox *obj);

/// fields is a caller allocated array for returning field keys
void lotrc_obj_get_fields_xbox(const ObjXbox *obj, uint32_t *fields);

const BaseTypeXbox *lotrc_obj_get_field_xbox(const ObjXbox *obj, uint32_t key);

const TypeHeaderXbox *lotrc_type_get_header_xbox(const TypeXbox *ty);

size_t lotrc_type_get_fields_num_xbox(const TypeXbox *ty);

/// fields is a caller allocated array for returning fields
void lotrc_type_get_fields_xbox(const TypeXbox *ty, TypeFieldXbox *fields);

BaseType lotrc_basetype_get_type_xbox(const BaseTypeXbox *basetype);

size_t lotrc_basetype_get_len_xbox(const BaseTypeXbox *basetype);

/// sizes is a caller allocated array for returning string sizes
void lotrc_basetype_get_stringlist_sizes_xbox(const BaseTypeXbox *basetype, size_t *sizes);

CrcXbox lotrc_basetype_get_crc_xbox(const BaseTypeXbox *basetype);

u32Xbox lotrc_basetype_get_guid_xbox(const BaseTypeXbox *basetype);

ColorXbox lotrc_basetype_get_color_xbox(const BaseTypeXbox *basetype);

const Vector2Xbox *lotrc_basetype_get_vector2_xbox(const BaseTypeXbox *basetype);

const Vector3Xbox *lotrc_basetype_get_vector3_xbox(const BaseTypeXbox *basetype);

const Vector4Xbox *lotrc_basetype_get_vector4_xbox(const BaseTypeXbox *basetype);

const Matrix4x4Xbox *lotrc_basetype_get_matrix4x4_xbox(const BaseTypeXbox *basetype);

f32Xbox lotrc_basetype_get_float_xbox(const BaseTypeXbox *basetype);

i32Xbox lotrc_basetype_get_int_xbox(const BaseTypeXbox *basetype);

bool lotrc_basetype_get_bool_xbox(const BaseTypeXbox *basetype);

const char *lotrc_basetype_get_string_xbox(const BaseTypeXbox *basetype);

const char *const *lotrc_basetype_get_stringlist_xbox(const BaseTypeXbox *basetype);

const u32Xbox *lotrc_basetype_get_objectlist_xbox(const BaseTypeXbox *basetype);

const Vector4Xbox *lotrc_basetype_get_nodelist_xbox(const BaseTypeXbox *basetype);

const i32Xbox *lotrc_basetype_get_intlist_xbox(const BaseTypeXbox *basetype);

const CrcXbox *lotrc_basetype_get_crclist_xbox(const BaseTypeXbox *basetype);

const WeightXbox *lotrc_basetype_get_weightlist_xbox(const BaseTypeXbox *basetype);

const Matrix4x4Xbox *lotrc_basetype_get_matrixlist_xbox(const BaseTypeXbox *basetype);

const GameObjsHeaderPs3 *lotrc_gameobjs_get_header_ps3(const GameObjsPs3 *gameobjs);

size_t lotrc_gameobjs_get_types_num_ps3(const GameObjsPs3 *gameobjs);

/// types is a caller allocated array for returning type keys
void lotrc_gameobjs_get_types_ps3(const GameObjsPs3 *gameobjs, uint32_t *types);

const TypePs3 *lotrc_gameobjs_get_type_ps3(const GameObjsPs3 *gameobjs, uint32_t key);

size_t lotrc_gameobjs_get_objs_num_ps3(const GameObjsPs3 *gameobjs);

/// objs is a caller allocated array for returning obj keys
void lotrc_gameobjs_get_objs_ps3(const GameObjsPs3 *gameobjs, uint32_t *objs);

const ObjPs3 *lotrc_gameobjs_get_obj_ps3(const GameObjsPs3 *gameobjs, uint32_t key);

const ObjHeaderPs3 *lotrc_obj_get_header_ps3(const ObjPs3 *obj);

size_t lotrc_obj_get_fields_num_ps3(const ObjPs3 *obj);

/// fields is a caller allocated array for returning field keys
void lotrc_obj_get_fields_ps3(const ObjPs3 *obj, uint32_t *fields);

const BaseTypePs3 *lotrc_obj_get_field_ps3(const ObjPs3 *obj, uint32_t key);

const TypeHeaderPs3 *lotrc_type_get_header_ps3(const TypePs3 *ty);

size_t lotrc_type_get_fields_num_ps3(const TypePs3 *ty);

/// fields is a caller allocated array for returning fields
void lotrc_type_get_fields_ps3(const TypePs3 *ty, TypeFieldPs3 *fields);

BaseType lotrc_basetype_get_type_ps3(const BaseTypePs3 *basetype);

size_t lotrc_basetype_get_len_ps3(const BaseTypePs3 *basetype);

/// sizes is a caller allocated array for returning string sizes
void lotrc_basetype_get_stringlist_sizes_ps3(const BaseTypePs3 *basetype, size_t *sizes);

CrcPs3 lotrc_basetype_get_crc_ps3(const BaseTypePs3 *basetype);

u32Ps3 lotrc_basetype_get_guid_ps3(const BaseTypePs3 *basetype);

ColorPs3 lotrc_basetype_get_color_ps3(const BaseTypePs3 *basetype);

const Vector2Ps3 *lotrc_basetype_get_vector2_ps3(const BaseTypePs3 *basetype);

const Vector3Ps3 *lotrc_basetype_get_vector3_ps3(const BaseTypePs3 *basetype);

const Vector4Ps3 *lotrc_basetype_get_vector4_ps3(const BaseTypePs3 *basetype);

const Matrix4x4Ps3 *lotrc_basetype_get_matrix4x4_ps3(const BaseTypePs3 *basetype);

f32Ps3 lotrc_basetype_get_float_ps3(const BaseTypePs3 *basetype);

i32Ps3 lotrc_basetype_get_int_ps3(const BaseTypePs3 *basetype);

bool lotrc_basetype_get_bool_ps3(const BaseTypePs3 *basetype);

const char *lotrc_basetype_get_string_ps3(const BaseTypePs3 *basetype);

const char *const *lotrc_basetype_get_stringlist_ps3(const BaseTypePs3 *basetype);

const u32Ps3 *lotrc_basetype_get_objectlist_ps3(const BaseTypePs3 *basetype);

const Vector4Ps3 *lotrc_basetype_get_nodelist_ps3(const BaseTypePs3 *basetype);

const i32Ps3 *lotrc_basetype_get_intlist_ps3(const BaseTypePs3 *basetype);

const CrcPs3 *lotrc_basetype_get_crclist_ps3(const BaseTypePs3 *basetype);

const WeightPs3 *lotrc_basetype_get_weightlist_ps3(const BaseTypePs3 *basetype);

const Matrix4x4Ps3 *lotrc_basetype_get_matrixlist_ps3(const BaseTypePs3 *basetype);

const CrowdHeaderPc *lotrc_crowd_get_header_pc(const CrowdPc *crowd);

const u32Pc *lotrc_crowd_get_offs_pc(const CrowdPc *crowd);

size_t lotrc_crowd_get_offs_num_pc(const CrowdPc *crowd);

const CrowdItemPc *lotrc_crowd_get_vals_pc(const CrowdPc *crowd);

size_t lotrc_crowd_get_vals_num_pc(const CrowdPc *crowd);

const AtlasUVValPc *lotrc_atlasuv_get_vals_pc(const AtlasUVPc *atlasuv);

size_t lotrc_atlasuv_get_vals_num_pc(const AtlasUVPc *atlasuv);

const CrowdHeaderXbox *lotrc_crowd_get_header_xbox(const CrowdXbox *crowd);

const u32Xbox *lotrc_crowd_get_offs_xbox(const CrowdXbox *crowd);

size_t lotrc_crowd_get_offs_num_xbox(const CrowdXbox *crowd);

const CrowdItemXbox *lotrc_crowd_get_vals_xbox(const CrowdXbox *crowd);

size_t lotrc_crowd_get_vals_num_xbox(const CrowdXbox *crowd);

const AtlasUVValXbox *lotrc_atlasuv_get_vals_xbox(const AtlasUVXbox *atlasuv);

size_t lotrc_atlasuv_get_vals_num_xbox(const AtlasUVXbox *atlasuv);

const CrowdHeaderPs3 *lotrc_crowd_get_header_ps3(const CrowdPs3 *crowd);

const u32Ps3 *lotrc_crowd_get_offs_ps3(const CrowdPs3 *crowd);

size_t lotrc_crowd_get_offs_num_ps3(const CrowdPs3 *crowd);

const CrowdItemPs3 *lotrc_crowd_get_vals_ps3(const CrowdPs3 *crowd);

size_t lotrc_crowd_get_vals_num_ps3(const CrowdPs3 *crowd);

const AtlasUVValPs3 *lotrc_atlasuv_get_vals_ps3(const AtlasUVPs3 *atlasuv);

size_t lotrc_atlasuv_get_vals_num_ps3(const AtlasUVPs3 *atlasuv);

}  // extern "C"
