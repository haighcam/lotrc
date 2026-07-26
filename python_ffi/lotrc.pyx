cimport lotrc_rs
cimport numpy as np
from cython.operator import dereference

cdef class OwnedAlignedBuf:
    cdef lotrc_rs.OwnedAlignedBuf* ptr
    def get(self):
        val = AlignedBuf()
        val.ptr = lotrc_rs.OwnedAlignedBuf_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedAlignedBuf_free(self.ptr)

cdef class OwnedDumpInfosPc:
    cdef lotrc_rs.OwnedDumpInfosPc* ptr
    def get(self):
        val = DumpInfosPc()
        val.ptr = lotrc_rs.OwnedDumpInfosPc_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedDumpInfosPc_free(self.ptr)

cdef class OwnedDumpInfosPs3:
    cdef lotrc_rs.OwnedDumpInfosPs3* ptr
    def get(self):
        val = DumpInfosPs3()
        val.ptr = lotrc_rs.OwnedDumpInfosPs3_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedDumpInfosPs3_free(self.ptr)

cdef class OwnedDumpInfosXbox:
    cdef lotrc_rs.OwnedDumpInfosXbox* ptr
    def get(self):
        val = DumpInfosXbox()
        val.ptr = lotrc_rs.OwnedDumpInfosXbox_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedDumpInfosXbox_free(self.ptr)

cdef class OwnedInfoCounts:
    cdef lotrc_rs.OwnedInfoCounts* ptr
    def get(self):
        val = InfoCounts()
        val.ptr = lotrc_rs.OwnedInfoCounts_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedInfoCounts_free(self.ptr)

cdef class OwnedLevelCompressedData:
    cdef lotrc_rs.OwnedLevelCompressedData* ptr
    def get(self):
        val = LevelCompressedData()
        val.ptr = lotrc_rs.OwnedLevelCompressedData_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedLevelCompressedData_free(self.ptr)
    @staticmethod
    def new():
        val = OwnedLevelCompressedData()
        val.ptr = lotrc_rs.OwnedLevelCompressedData_new()
        return val

cdef class OwnedLevelData:
    cdef lotrc_rs.OwnedLevelData* ptr
    def get(self):
        val = LevelData()
        val.ptr = lotrc_rs.OwnedLevelData_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedLevelData_free(self.ptr)
    @staticmethod
    def read_data(path):
        val = OwnedLevelData()
        val.ptr = lotrc_rs.OwnedLevelData_read_data(path)
        return val

cdef class OwnedLevelRefPc:
    cdef lotrc_rs.OwnedLevelRefPc* ptr
    def get(self):
        val = LevelRefPc()
        val.ptr = lotrc_rs.OwnedLevelRefPc_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedLevelRefPc_free(self.ptr)
    @staticmethod
    def from_data(LevelData src, LevelCompressedData data):
        val = OwnedLevelRefPc()
        val.ptr = lotrc_rs.OwnedLevelRefPc_from_data(src.ptr, data.ptr)
        return val

cdef class OwnedLevelRefPs3:
    cdef lotrc_rs.OwnedLevelRefPs3* ptr
    def get(self):
        val = LevelRefPs3()
        val.ptr = lotrc_rs.OwnedLevelRefPs3_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedLevelRefPs3_free(self.ptr)
    @staticmethod
    def from_data(LevelData src, LevelCompressedData data):
        val = OwnedLevelRefPs3()
        val.ptr = lotrc_rs.OwnedLevelRefPs3_from_data(src.ptr, data.ptr)
        return val

cdef class OwnedLevelRefXbox:
    cdef lotrc_rs.OwnedLevelRefXbox* ptr
    def get(self):
        val = LevelRefXbox()
        val.ptr = lotrc_rs.OwnedLevelRefXbox_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedLevelRefXbox_free(self.ptr)
    @staticmethod
    def from_data(LevelData src, LevelCompressedData data):
        val = OwnedLevelRefXbox()
        val.ptr = lotrc_rs.OwnedLevelRefXbox_from_data(src.ptr, data.ptr)
        return val

cdef class OwnedVecCompressedData:
    cdef lotrc_rs.OwnedVecCompressedData* ptr
    def get(self):
        val = Vec______CompressedDataRef()
        val.ptr = lotrc_rs.OwnedVecCompressedData_get(self.ptr)
        return val
    def __dealloc__(self):
        lotrc_rs.OwnedVecCompressedData_free(self.ptr)

cdef class Vec______CompressedDataRef:
    cdef lotrc_rs.Vec______CompressedDataRef* ptr

cdef class slice_AnimationBlockInfoPc:
    cdef lotrc_rs.slice_AnimationBlockInfoPc* ptr

cdef class slice_AnimationBlockInfoPs3:
    cdef lotrc_rs.slice_AnimationBlockInfoPs3* ptr

cdef class slice_AnimationBlockInfoXbox:
    cdef lotrc_rs.slice_AnimationBlockInfoXbox* ptr

cdef class slice_AnimationInfoPc:
    cdef lotrc_rs.slice_AnimationInfoPc* ptr

cdef class slice_AnimationInfoPs3:
    cdef lotrc_rs.slice_AnimationInfoPs3* ptr

cdef class slice_AnimationInfoXbox:
    cdef lotrc_rs.slice_AnimationInfoXbox* ptr

cdef class slice_BufferInfoPc:
    cdef lotrc_rs.slice_BufferInfoPc* ptr

cdef class slice_BufferInfoPs3:
    cdef lotrc_rs.slice_BufferInfoPs3* ptr

cdef class slice_BufferInfoXbox:
    cdef lotrc_rs.slice_BufferInfoXbox* ptr

cdef class slice_EffectInfoPc:
    cdef lotrc_rs.slice_EffectInfoPc* ptr

cdef class slice_EffectInfoPs3:
    cdef lotrc_rs.slice_EffectInfoPs3* ptr

cdef class slice_EffectInfoXbox:
    cdef lotrc_rs.slice_EffectInfoXbox* ptr

cdef class slice_FoliageInfoPc:
    cdef lotrc_rs.slice_FoliageInfoPc* ptr

cdef class slice_FoliageInfoPs3:
    cdef lotrc_rs.slice_FoliageInfoPs3* ptr

cdef class slice_FoliageInfoXbox:
    cdef lotrc_rs.slice_FoliageInfoXbox* ptr

cdef class slice_GFXBlockInfoPc:
    cdef lotrc_rs.slice_GFXBlockInfoPc* ptr

cdef class slice_GFXBlockInfoPs3:
    cdef lotrc_rs.slice_GFXBlockInfoPs3* ptr

cdef class slice_GFXBlockInfoXbox:
    cdef lotrc_rs.slice_GFXBlockInfoXbox* ptr

cdef class slice_HkConstraintDataPc:
    cdef lotrc_rs.slice_HkConstraintDataPc* ptr

cdef class slice_HkConstraintDataPs3:
    cdef lotrc_rs.slice_HkConstraintDataPs3* ptr

cdef class slice_HkConstraintDataXbox:
    cdef lotrc_rs.slice_HkConstraintDataXbox* ptr

cdef class slice_HkConstraintInfoPc:
    cdef lotrc_rs.slice_HkConstraintInfoPc* ptr

cdef class slice_HkConstraintInfoPs3:
    cdef lotrc_rs.slice_HkConstraintInfoPs3* ptr

cdef class slice_HkConstraintInfoXbox:
    cdef lotrc_rs.slice_HkConstraintInfoXbox* ptr

cdef class slice_HkShapeInfoPc:
    cdef lotrc_rs.slice_HkShapeInfoPc* ptr

cdef class slice_HkShapeInfoPs3:
    cdef lotrc_rs.slice_HkShapeInfoPs3* ptr

cdef class slice_HkShapeInfoXbox:
    cdef lotrc_rs.slice_HkShapeInfoXbox* ptr

cdef class slice_IBuffInfoPc:
    cdef lotrc_rs.slice_IBuffInfoPc* ptr

cdef class slice_IBuffInfoPs3:
    cdef lotrc_rs.slice_IBuffInfoPs3* ptr

cdef class slice_IBuffInfoXbox:
    cdef lotrc_rs.slice_IBuffInfoXbox* ptr

cdef class slice_Mat1Pc:
    cdef lotrc_rs.slice_Mat1Pc* ptr

cdef class slice_Mat1Ps3:
    cdef lotrc_rs.slice_Mat1Ps3* ptr

cdef class slice_Mat1Xbox:
    cdef lotrc_rs.slice_Mat1Xbox* ptr

cdef class slice_Mat2Pc:
    cdef lotrc_rs.slice_Mat2Pc* ptr

cdef class slice_Mat2Ps3:
    cdef lotrc_rs.slice_Mat2Ps3* ptr

cdef class slice_Mat2Xbox:
    cdef lotrc_rs.slice_Mat2Xbox* ptr

cdef class slice_Mat3Pc:
    cdef lotrc_rs.slice_Mat3Pc* ptr

cdef class slice_Mat3Ps3:
    cdef lotrc_rs.slice_Mat3Ps3* ptr

cdef class slice_Mat3Xbox:
    cdef lotrc_rs.slice_Mat3Xbox* ptr

cdef class slice_Mat4Pc:
    cdef lotrc_rs.slice_Mat4Pc* ptr

cdef class slice_Mat4Ps3:
    cdef lotrc_rs.slice_Mat4Ps3* ptr

cdef class slice_Mat4Xbox:
    cdef lotrc_rs.slice_Mat4Xbox* ptr

cdef class slice_MatExtraPc:
    cdef lotrc_rs.slice_MatExtraPc* ptr

cdef class slice_MatExtraPs3:
    cdef lotrc_rs.slice_MatExtraPs3* ptr

cdef class slice_MatExtraXbox:
    cdef lotrc_rs.slice_MatExtraXbox* ptr

cdef class slice_ModelInfoPc:
    cdef lotrc_rs.slice_ModelInfoPc* ptr

cdef class slice_ModelInfoPs3:
    cdef lotrc_rs.slice_ModelInfoPs3* ptr

cdef class slice_ModelInfoXbox:
    cdef lotrc_rs.slice_ModelInfoXbox* ptr

cdef class slice_Obj0Pc:
    cdef lotrc_rs.slice_Obj0Pc* ptr

cdef class slice_Obj0Ps3:
    cdef lotrc_rs.slice_Obj0Ps3* ptr

cdef class slice_Obj0Xbox:
    cdef lotrc_rs.slice_Obj0Xbox* ptr

cdef class slice_ObjAPc:
    cdef lotrc_rs.slice_ObjAPc* ptr

cdef class slice_ObjAPs3:
    cdef lotrc_rs.slice_ObjAPs3* ptr

cdef class slice_ObjAXbox:
    cdef lotrc_rs.slice_ObjAXbox* ptr

cdef class slice_PFieldInfoPc:
    cdef lotrc_rs.slice_PFieldInfoPc* ptr

cdef class slice_PFieldInfoPs3:
    cdef lotrc_rs.slice_PFieldInfoPs3* ptr

cdef class slice_PFieldInfoXbox:
    cdef lotrc_rs.slice_PFieldInfoXbox* ptr

cdef class slice_RadiosityValsInfoPc:
    cdef lotrc_rs.slice_RadiosityValsInfoPc* ptr

cdef class slice_RadiosityValsInfoPs3:
    cdef lotrc_rs.slice_RadiosityValsInfoPs3* ptr

cdef class slice_RadiosityValsInfoXbox:
    cdef lotrc_rs.slice_RadiosityValsInfoXbox* ptr

cdef class slice_ShapeInfoPc:
    cdef lotrc_rs.slice_ShapeInfoPc* ptr

cdef class slice_ShapeInfoPs3:
    cdef lotrc_rs.slice_ShapeInfoPs3* ptr

cdef class slice_ShapeInfoXbox:
    cdef lotrc_rs.slice_ShapeInfoXbox* ptr

cdef class slice_TextureInfoPc:
    cdef lotrc_rs.slice_TextureInfoPc* ptr

cdef class slice_TextureInfoPs3:
    cdef lotrc_rs.slice_TextureInfoPs3* ptr

cdef class slice_TextureInfoXbox:
    cdef lotrc_rs.slice_TextureInfoXbox* ptr

cdef class slice_VBuffInfoPc:
    cdef lotrc_rs.slice_VBuffInfoPc* ptr

cdef class slice_VBuffInfoPs3:
    cdef lotrc_rs.slice_VBuffInfoPs3* ptr

cdef class slice_VBuffInfoXbox:
    cdef lotrc_rs.slice_VBuffInfoXbox* ptr

cdef class slice_VertexUsage:
    cdef lotrc_rs.slice_VertexUsage* ptr

cdef class slice_u32:
    cdef lotrc_rs.slice_u32* ptr

cdef class slice_u32Pc:
    cdef lotrc_rs.slice_u32Pc* ptr

cdef class slice_u32Ps3:
    cdef lotrc_rs.slice_u32Ps3* ptr

cdef class slice_u32Xbox:
    cdef lotrc_rs.slice_u32Xbox* ptr

cdef class slice_u8:
    cdef lotrc_rs.slice_u8* ptr

cdef class slice_AlignmentHelper:
    cdef lotrc_rs.slice_AlignmentHelper* ptr

cdef class AlignedBuf:
    cdef lotrc_rs.AlignedBuf* ptr
    @staticmethod
    def with_capacity(lotrc_rs.uintptr_t size):
        val = OwnedAlignedBuf()
        val.ptr = lotrc_rs.AlignedBuf_with_capacity(size)
        return val
    @property
    def data(self):
        val = slice_AlignmentHelper()
        val.ptr = &self.ptr.data
        return val
    @property
    def size(self):
        return self.ptr.size

cdef class LevelData:
    cdef lotrc_rs.LevelData* ptr
    def version(self):
        return lotrc_rs.LevelData_version(self.ptr)
    @property
    def pak(self):
        val = AlignedBuf()
        val.ptr = &self.ptr.pak
        return val
    @property
    def bin(self):
        val = AlignedBuf()
        val.ptr = &self.ptr.bin
        return val

cdef class ref_slice_u8:
    cdef lotrc_rs.ref_slice_u8* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_u8_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_u8_len(self.ptr)

cdef class CompressedDataRef:
    cdef lotrc_rs.CompressedDataRef* ptr
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val
    @property
    def data_decomp(self):
        val = AlignedBuf()
        val.ptr = &self.ptr.data_decomp
        return val

cdef class slice_CompressedDataRef:
    cdef lotrc_rs.slice_CompressedDataRef* ptr

cdef class PakCompressedData:
    cdef lotrc_rs.PakCompressedData* ptr
    @property
    def block1(self):
        val = CompressedDataRef()
        val.ptr = &self.ptr.block1
        return val
    @property
    def block2(self):
        val = CompressedDataRef()
        val.ptr = &self.ptr.block2
        return val
    @property
    def animations(self):
        val = slice_CompressedDataRef()
        val.ptr = &self.ptr.animations
        return val

cdef class IndexMap_u32__CompressedDataRef:
    cdef lotrc_rs.IndexMap_u32__CompressedDataRef* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = CompressedDataRef()
        val.ptr = lotrc_rs.IndexMap_u32__CompressedDataRef_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__CompressedDataRef_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__CompressedDataRef_keys(self.ptr, keys.ptr)

cdef class BinCompressedData:
    cdef lotrc_rs.BinCompressedData* ptr
    @property
    def model_data(self):
        val = IndexMap_u32__CompressedDataRef()
        val.ptr = &self.ptr.model_data
        return val
    @property
    def texture_data(self):
        val = IndexMap_u32__CompressedDataRef()
        val.ptr = &self.ptr.texture_data
        return val

cdef class LevelCompressedData:
    cdef lotrc_rs.LevelCompressedData* ptr
    @property
    def pak(self):
        val = PakCompressedData()
        val.ptr = &self.ptr.pak
        return val
    @property
    def bin(self):
        val = BinCompressedData()
        val.ptr = &self.ptr.bin
        return val

cdef class PakHeaderPc:
    cdef lotrc_rs.PakHeaderPc* ptr
    @property
    def block_a_num(self):
        return self.ptr.block_a_num
    @property
    def block_a_offset(self):
        return self.ptr.block_a_offset
    @property
    def constx13(self):
        return self.ptr.constx13
    @property
    def version(self):
        return self.ptr.version
    @property
    def strings_offset(self):
        return self.ptr.strings_offset
    @property
    def strings_size(self):
        return self.ptr.strings_size
    @property
    def strings_num(self):
        return self.ptr.strings_num
    @property
    def block1_offset(self):
        return self.ptr.block1_offset
    @property
    def block1_size(self):
        return self.ptr.block1_size
    @property
    def block1_size_comp(self):
        return self.ptr.block1_size_comp
    @property
    def sub_blocks1_offset(self):
        return self.ptr.sub_blocks1_offset
    @property
    def block2_offset(self):
        return self.ptr.block2_offset
    @property
    def block2_size(self):
        return self.ptr.block2_size
    @property
    def block2_size_comp(self):
        return self.ptr.block2_size_comp
    @property
    def sub_blocks2_offset(self):
        return self.ptr.sub_blocks2_offset
    @property
    def string_keys_offset(self):
        return self.ptr.string_keys_offset
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def obja_size(self):
        return self.ptr.obja_size
    @property
    def obj0_size(self):
        return self.ptr.obj0_size
    @property
    def model_info_size(self):
        return self.ptr.model_info_size
    @property
    def buffer_info_size(self):
        return self.ptr.buffer_info_size
    @property
    def mat1_size(self):
        return self.ptr.mat1_size
    @property
    def mat2_size(self):
        return self.ptr.mat2_size
    @property
    def mat3_size(self):
        return self.ptr.mat3_size
    @property
    def mat4_size(self):
        return self.ptr.mat4_size
    @property
    def mat_extra_size(self):
        return self.ptr.mat_extra_size
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def shape_info_size(self):
        return self.ptr.shape_info_size
    @property
    def hk_shape_info_size(self):
        return self.ptr.hk_shape_info_size
    @property
    def hk_constraint_data_size(self):
        return self.ptr.hk_constraint_data_size
    @property
    def vbuff_info_size(self):
        return self.ptr.vbuff_info_size
    @property
    def ibuff_info_size(self):
        return self.ptr.ibuff_info_size
    @property
    def texture_info_size(self):
        return self.ptr.texture_info_size
    @property
    def animation_info_size(self):
        return self.ptr.animation_info_size
    @property
    def hk_constraint_info_size(self):
        return self.ptr.hk_constraint_info_size
    @property
    def effect_info_size(self):
        return self.ptr.effect_info_size
    @property
    def pfield_info_size(self):
        return self.ptr.pfield_info_size
    @property
    def gfx_block_info_size(self):
        return self.ptr.gfx_block_info_size
    @property
    def animation_block_info_size(self):
        return self.ptr.animation_block_info_size
    @property
    def foliage_info_size(self):
        return self.ptr.foliage_info_size
    @property
    def radiosity_vals_info_size(self):
        return self.ptr.radiosity_vals_info_size
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def obja_num(self):
        return self.ptr.obja_num
    @property
    def obj0_num(self):
        return self.ptr.obj0_num
    @property
    def model_info_num(self):
        return self.ptr.model_info_num
    @property
    def buffer_info_num(self):
        return self.ptr.buffer_info_num
    @property
    def mat1_num(self):
        return self.ptr.mat1_num
    @property
    def mat2_num(self):
        return self.ptr.mat2_num
    @property
    def mat3_num(self):
        return self.ptr.mat3_num
    @property
    def mat4_num(self):
        return self.ptr.mat4_num
    @property
    def mat_extra_num(self):
        return self.ptr.mat_extra_num
    @property
    def unk_51(self):
        return self.ptr.unk_51
    @property
    def shape_info_num(self):
        return self.ptr.shape_info_num
    @property
    def hk_shape_info_num(self):
        return self.ptr.hk_shape_info_num
    @property
    def hk_constraint_data_num(self):
        return self.ptr.hk_constraint_data_num
    @property
    def vbuff_info_num(self):
        return self.ptr.vbuff_info_num
    @property
    def ibuff_info_num(self):
        return self.ptr.ibuff_info_num
    @property
    def texture_info_num(self):
        return self.ptr.texture_info_num
    @property
    def animation_info_num(self):
        return self.ptr.animation_info_num
    @property
    def hk_constraint_info_num(self):
        return self.ptr.hk_constraint_info_num
    @property
    def effect_info_num(self):
        return self.ptr.effect_info_num
    @property
    def pfield_info_num(self):
        return self.ptr.pfield_info_num
    @property
    def gfx_block_info_num(self):
        return self.ptr.gfx_block_info_num
    @property
    def animation_block_info_num(self):
        return self.ptr.animation_block_info_num
    @property
    def foliage_info_num(self):
        return self.ptr.foliage_info_num
    @property
    def radiosity_vals_info_num(self):
        return self.ptr.radiosity_vals_info_num
    @property
    def unk_66(self):
        return self.ptr.unk_66
    @property
    def obja_offset(self):
        return self.ptr.obja_offset
    @property
    def obj0_offset(self):
        return self.ptr.obj0_offset
    @property
    def model_info_offset(self):
        return self.ptr.model_info_offset
    @property
    def buffer_info_offset(self):
        return self.ptr.buffer_info_offset
    @property
    def mat1_offset(self):
        return self.ptr.mat1_offset
    @property
    def mat2_offset(self):
        return self.ptr.mat2_offset
    @property
    def mat3_offset(self):
        return self.ptr.mat3_offset
    @property
    def mat4_offset(self):
        return self.ptr.mat4_offset
    @property
    def mat_extra_offset(self):
        return self.ptr.mat_extra_offset
    @property
    def unk_76(self):
        return self.ptr.unk_76
    @property
    def shape_info_offset(self):
        return self.ptr.shape_info_offset
    @property
    def hk_shape_info_offset(self):
        return self.ptr.hk_shape_info_offset
    @property
    def hk_constraint_data_offset(self):
        return self.ptr.hk_constraint_data_offset
    @property
    def vbuff_info_offset(self):
        return self.ptr.vbuff_info_offset
    @property
    def ibuff_info_offset(self):
        return self.ptr.ibuff_info_offset
    @property
    def texture_info_offset(self):
        return self.ptr.texture_info_offset
    @property
    def animation_info_offset(self):
        return self.ptr.animation_info_offset
    @property
    def hk_constraint_info_offset(self):
        return self.ptr.hk_constraint_info_offset
    @property
    def effect_info_offset(self):
        return self.ptr.effect_info_offset
    @property
    def pfield_info_offset(self):
        return self.ptr.pfield_info_offset
    @property
    def gfx_block_info_offset(self):
        return self.ptr.gfx_block_info_offset
    @property
    def animation_block_info_offset(self):
        return self.ptr.animation_block_info_offset
    @property
    def foliage_info_offset(self):
        return self.ptr.foliage_info_offset
    @property
    def radiosity_vals_info_offset(self):
        return self.ptr.radiosity_vals_info_offset
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def unk_114(self):
        return self.ptr.unk_114
    @property
    def unk_115(self):
        return self.ptr.unk_115
    @property
    def block2_offsets_num(self):
        return self.ptr.block2_offsets_num
    @property
    def block2_offsets_offset(self):
        return self.ptr.block2_offsets_offset

cdef class slice_string:
    cdef lotrc_rs.slice_string* ptr

cdef class ref_slice_ObjAPc:
    cdef lotrc_rs.ref_slice_ObjAPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ObjAPc()
        val.ptr = lotrc_rs.ref_slice_ObjAPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_ObjAPc_len(self.ptr)

cdef class ref_slice_Obj0Pc:
    cdef lotrc_rs.ref_slice_Obj0Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj0Pc()
        val.ptr = lotrc_rs.ref_slice_Obj0Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Obj0Pc_len(self.ptr)

cdef class ref_slice_ModelInfoPc:
    cdef lotrc_rs.ref_slice_ModelInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ModelInfoPc()
        val.ptr = lotrc_rs.ref_slice_ModelInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_ModelInfoPc_len(self.ptr)
    def get_arr(self):
        cdef lotrc_rs.ModelInfoPc[:] arr = <lotrc_rs.ModelInfoPc[:lotrc_rs.ref_slice_ModelInfoPc_len(self.ptr)]> lotrc_rs.ref_slice_ModelInfoPc_get(self.ptr, 0)
        return arr

cdef class ref_slice_BufferInfoPc:
    cdef lotrc_rs.ref_slice_BufferInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BufferInfoPc()
        val.ptr = lotrc_rs.ref_slice_BufferInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BufferInfoPc_len(self.ptr)

cdef class ref_slice_Mat1Pc:
    cdef lotrc_rs.ref_slice_Mat1Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat1Pc()
        val.ptr = lotrc_rs.ref_slice_Mat1Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat1Pc_len(self.ptr)
    def get_arr(self):
        cdef lotrc_rs.Mat1Pc[:] arr = <lotrc_rs.Mat1Pc[:lotrc_rs.ref_slice_Mat1Pc_len(self.ptr)]> lotrc_rs.ref_slice_Mat1Pc_get(self.ptr, 0)
        return arr

cdef class ref_slice_Mat2Pc:
    cdef lotrc_rs.ref_slice_Mat2Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat2Pc()
        val.ptr = lotrc_rs.ref_slice_Mat2Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat2Pc_len(self.ptr)

cdef class ref_slice_Mat3Pc:
    cdef lotrc_rs.ref_slice_Mat3Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat3Pc()
        val.ptr = lotrc_rs.ref_slice_Mat3Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat3Pc_len(self.ptr)

cdef class ref_slice_Mat4Pc:
    cdef lotrc_rs.ref_slice_Mat4Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat4Pc()
        val.ptr = lotrc_rs.ref_slice_Mat4Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat4Pc_len(self.ptr)

cdef class ref_slice_MatExtraPc:
    cdef lotrc_rs.ref_slice_MatExtraPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = MatExtraPc()
        val.ptr = lotrc_rs.ref_slice_MatExtraPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_MatExtraPc_len(self.ptr)

cdef class ref_slice_ShapeInfoPc:
    cdef lotrc_rs.ref_slice_ShapeInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ShapeInfoPc()
        val.ptr = lotrc_rs.ref_slice_ShapeInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_ShapeInfoPc_len(self.ptr)
    def get_arr(self):
        cdef lotrc_rs.ShapeInfoPc[:] arr = <lotrc_rs.ShapeInfoPc[:lotrc_rs.ref_slice_ShapeInfoPc_len(self.ptr)]> lotrc_rs.ref_slice_ShapeInfoPc_get(self.ptr, 0)
        return arr

cdef class ref_slice_HkShapeInfoPc:
    cdef lotrc_rs.ref_slice_HkShapeInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkShapeInfoPc()
        val.ptr = lotrc_rs.ref_slice_HkShapeInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_HkShapeInfoPc_len(self.ptr)

cdef class ref_slice_HkConstraintDataPc:
    cdef lotrc_rs.ref_slice_HkConstraintDataPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintDataPc()
        val.ptr = lotrc_rs.ref_slice_HkConstraintDataPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_HkConstraintDataPc_len(self.ptr)

cdef class ref_slice_VBuffInfoPc:
    cdef lotrc_rs.ref_slice_VBuffInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = VBuffInfoPc()
        val.ptr = lotrc_rs.ref_slice_VBuffInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_VBuffInfoPc_len(self.ptr)

cdef class ref_slice_IBuffInfoPc:
    cdef lotrc_rs.ref_slice_IBuffInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = IBuffInfoPc()
        val.ptr = lotrc_rs.ref_slice_IBuffInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_IBuffInfoPc_len(self.ptr)

cdef class ref_slice_TextureInfoPc:
    cdef lotrc_rs.ref_slice_TextureInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TextureInfoPc()
        val.ptr = lotrc_rs.ref_slice_TextureInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_TextureInfoPc_len(self.ptr)

cdef class ref_slice_AnimationInfoPc:
    cdef lotrc_rs.ref_slice_AnimationInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationInfoPc()
        val.ptr = lotrc_rs.ref_slice_AnimationInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AnimationInfoPc_len(self.ptr)
    def get_arr(self):
        cdef lotrc_rs.AnimationInfoPc[:] arr = <lotrc_rs.AnimationInfoPc[:lotrc_rs.ref_slice_AnimationInfoPc_len(self.ptr)]> lotrc_rs.ref_slice_AnimationInfoPc_get(self.ptr, 0)
        return arr

cdef class ref_slice_HkConstraintInfoPc:
    cdef lotrc_rs.ref_slice_HkConstraintInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintInfoPc()
        val.ptr = lotrc_rs.ref_slice_HkConstraintInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_HkConstraintInfoPc_len(self.ptr)

cdef class ref_slice_EffectInfoPc:
    cdef lotrc_rs.ref_slice_EffectInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = EffectInfoPc()
        val.ptr = lotrc_rs.ref_slice_EffectInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_EffectInfoPc_len(self.ptr)

cdef class ref_slice_PFieldInfoPc:
    cdef lotrc_rs.ref_slice_PFieldInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = PFieldInfoPc()
        val.ptr = lotrc_rs.ref_slice_PFieldInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_PFieldInfoPc_len(self.ptr)

cdef class ref_slice_GFXBlockInfoPc:
    cdef lotrc_rs.ref_slice_GFXBlockInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = GFXBlockInfoPc()
        val.ptr = lotrc_rs.ref_slice_GFXBlockInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_GFXBlockInfoPc_len(self.ptr)

cdef class ref_slice_AnimationBlockInfoPc:
    cdef lotrc_rs.ref_slice_AnimationBlockInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationBlockInfoPc()
        val.ptr = lotrc_rs.ref_slice_AnimationBlockInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AnimationBlockInfoPc_len(self.ptr)

cdef class ref_slice_FoliageInfoPc:
    cdef lotrc_rs.ref_slice_FoliageInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = FoliageInfoPc()
        val.ptr = lotrc_rs.ref_slice_FoliageInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_FoliageInfoPc_len(self.ptr)

cdef class ref_slice_RadiosityValsInfoPc:
    cdef lotrc_rs.ref_slice_RadiosityValsInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RadiosityValsInfoPc()
        val.ptr = lotrc_rs.ref_slice_RadiosityValsInfoPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RadiosityValsInfoPc_len(self.ptr)

cdef class InfosRefPc:
    cdef lotrc_rs.InfosRefPc* ptr
    @property
    def objas(self):
        val = ref_slice_ObjAPc()
        val.ptr = &self.ptr.objas
        return val
    @property
    def obj0s(self):
        val = ref_slice_Obj0Pc()
        val.ptr = &self.ptr.obj0s
        return val
    @property
    def models(self):
        val = ref_slice_ModelInfoPc()
        val.ptr = &self.ptr.models
        return val
    @property
    def buffers(self):
        val = ref_slice_BufferInfoPc()
        val.ptr = &self.ptr.buffers
        return val
    @property
    def mat1s(self):
        val = ref_slice_Mat1Pc()
        val.ptr = &self.ptr.mat1s
        return val
    @property
    def mat2s(self):
        val = ref_slice_Mat2Pc()
        val.ptr = &self.ptr.mat2s
        return val
    @property
    def mat3s(self):
        val = ref_slice_Mat3Pc()
        val.ptr = &self.ptr.mat3s
        return val
    @property
    def mat4s(self):
        val = ref_slice_Mat4Pc()
        val.ptr = &self.ptr.mat4s
        return val
    @property
    def mat_extras(self):
        val = ref_slice_MatExtraPc()
        val.ptr = &self.ptr.mat_extras
        return val
    @property
    def shapes(self):
        val = ref_slice_ShapeInfoPc()
        val.ptr = &self.ptr.shapes
        return val
    @property
    def hk_shapes(self):
        val = ref_slice_HkShapeInfoPc()
        val.ptr = &self.ptr.hk_shapes
        return val
    @property
    def hk_constraint_datas(self):
        val = ref_slice_HkConstraintDataPc()
        val.ptr = &self.ptr.hk_constraint_datas
        return val
    @property
    def vbuffs(self):
        val = ref_slice_VBuffInfoPc()
        val.ptr = &self.ptr.vbuffs
        return val
    @property
    def ibuffs(self):
        val = ref_slice_IBuffInfoPc()
        val.ptr = &self.ptr.ibuffs
        return val
    @property
    def textures(self):
        val = ref_slice_TextureInfoPc()
        val.ptr = &self.ptr.textures
        return val
    @property
    def animations(self):
        val = ref_slice_AnimationInfoPc()
        val.ptr = &self.ptr.animations
        return val
    @property
    def hk_constraints(self):
        val = ref_slice_HkConstraintInfoPc()
        val.ptr = &self.ptr.hk_constraints
        return val
    @property
    def effects(self):
        val = ref_slice_EffectInfoPc()
        val.ptr = &self.ptr.effects
        return val
    @property
    def pfields(self):
        val = ref_slice_PFieldInfoPc()
        val.ptr = &self.ptr.pfields
        return val
    @property
    def gfxs(self):
        val = ref_slice_GFXBlockInfoPc()
        val.ptr = &self.ptr.gfxs
        return val
    @property
    def animation_blocks(self):
        val = ref_slice_AnimationBlockInfoPc()
        val.ptr = &self.ptr.animation_blocks
        return val
    @property
    def foliages(self):
        val = ref_slice_FoliageInfoPc()
        val.ptr = &self.ptr.foliages
        return val
    @property
    def radiosity_vals(self):
        val = ref_slice_RadiosityValsInfoPc()
        val.ptr = &self.ptr.radiosity_vals
        return val

cdef class IndexMap_u32__TextureRefPc:
    cdef lotrc_rs.IndexMap_u32__TextureRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = TextureRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__TextureRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__TextureRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__TextureRefPc_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__ModelRefPc:
    cdef lotrc_rs.IndexMap_u32__ModelRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = ModelRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__ModelRefPc_get(self.ptr, &key)
        if val.ptr == NULL:
            return None
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__ModelRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__ModelRefPc_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__EffectRefPc:
    cdef lotrc_rs.IndexMap_u32__EffectRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = EffectRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__EffectRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__EffectRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__EffectRefPc_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__slice_FoliageRefPc:
    cdef lotrc_rs.IndexMap_u32__slice_FoliageRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = slice_FoliageRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__slice_FoliageRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__slice_FoliageRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__slice_FoliageRefPc_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__ref_slice_u8:
    cdef lotrc_rs.IndexMap_u32__ref_slice_u8* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = ref_slice_u8()
        val.ptr = lotrc_rs.IndexMap_u32__ref_slice_u8_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__ref_slice_u8_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__ref_slice_u8_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__RadiosityValsRefPc:
    cdef lotrc_rs.IndexMap_u32__RadiosityValsRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = RadiosityValsRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__RadiosityValsRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__RadiosityValsRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__RadiosityValsRefPc_keys(self.ptr, keys.ptr)

cdef class RadiosityRefPc:
    cdef lotrc_rs.RadiosityRefPc* ptr
    @property
    def data(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data
        return val
    @property
    def vals(self):
        val = IndexMap_u32__RadiosityValsRefPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def usage(self):
        return self.ptr.usage

cdef class ObjsRefPc:
    cdef lotrc_rs.ObjsRefPc* ptr
    @property
    def textures(self):
        val = IndexMap_u32__TextureRefPc()
        val.ptr = &self.ptr.textures
        return val
    @property
    def models(self):
        val = IndexMap_u32__ModelRefPc()
        val.ptr = &self.ptr.models
        return val
    @property
    def effects(self):
        val = IndexMap_u32__EffectRefPc()
        val.ptr = &self.ptr.effects
        return val
    @property
    def foliages(self):
        val = IndexMap_u32__slice_FoliageRefPc()
        val.ptr = &self.ptr.foliages
        return val
    @property
    def gfxs(self):
        val = IndexMap_u32__ref_slice_u8()
        val.ptr = &self.ptr.gfxs
        return val
    @property
    def radiosity(self):
        val = RadiosityRefPc()
        val.ptr = &self.ptr.radiosity
        return val

cdef class SubBlocksHeaderPc:
    cdef lotrc_rs.SubBlocksHeaderPc* ptr
    @property
    def z0(self):
        return self.ptr.z0
    @property
    def block_num(self):
        return self.ptr.block_num
    @property
    def z2(self):
        return self.ptr.z2
    @property
    def z3(self):
        return self.ptr.z3

cdef class ref_slice_SubBlocksBlockHeaderPc:
    cdef lotrc_rs.ref_slice_SubBlocksBlockHeaderPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SubBlocksBlockHeaderPc()
        val.ptr = lotrc_rs.ref_slice_SubBlocksBlockHeaderPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SubBlocksBlockHeaderPc_len(self.ptr)

cdef class SubBlocksInfoRefPc:
    cdef lotrc_rs.SubBlocksInfoRefPc* ptr
    @property
    def header(self):
        val = SubBlocksHeaderPc()
        val.ptr = self.ptr.header
        return val
    @property
    def block_headers(self):
        val = ref_slice_SubBlocksBlockHeaderPc()
        val.ptr = &self.ptr.block_headers
        return val

cdef class IndexMap_u32__DataRefPc:
    cdef lotrc_rs.IndexMap_u32__DataRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = DataRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__DataRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__DataRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__DataRefPc_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__LuaRefPc:
    cdef lotrc_rs.IndexMap_u32__LuaRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = DataRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__LuaRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__LuaRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__LuaRefPc_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__SSARefPc:
    cdef lotrc_rs.IndexMap_u32__SSARefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = SSARefPc()
        val.ptr = lotrc_rs.IndexMap_u32__SSARefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__SSARefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__SSARefPc_keys(self.ptr, keys.ptr)

cdef class Option_AtlasUVRefPc:
    cdef lotrc_rs.Option_AtlasUVRefPc* ptr
    def get(self):
        val = AtlasUVRefPc()
        val.ptr = lotrc_rs.Option_AtlasUVRefPc_get(self.ptr)
        return val

cdef class GameObjsHeaderPc:
    cdef lotrc_rs.GameObjsHeaderPc* ptr
    @property
    def const_(self):
        return self.ptr.const_
    @property
    def types_num(self):
        return self.ptr.types_num
    @property
    def types_offset(self):
        return self.ptr.types_offset
    @property
    def obj_num(self):
        return self.ptr.obj_num
    @property
    def obj_offset(self):
        return self.ptr.obj_offset
    @property
    def z5(self):
        return self.ptr.z5
    @property
    def z6(self):
        return self.ptr.z6
    @property
    def z7(self):
        return self.ptr.z7

cdef class IndexMap_u32__TypeRefPc:
    cdef lotrc_rs.IndexMap_u32__TypeRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = TypeRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__TypeRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__TypeRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__TypeRefPc_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__ObjRefPc:
    cdef lotrc_rs.IndexMap_u32__ObjRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = ObjRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__ObjRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__ObjRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__ObjRefPc_keys(self.ptr, keys.ptr)

cdef class GameObjsRefPc:
    cdef lotrc_rs.GameObjsRefPc* ptr
    @property
    def header(self):
        val = GameObjsHeaderPc()
        val.ptr = self.ptr.header
        return val
    @property
    def types(self):
        val = IndexMap_u32__TypeRefPc()
        val.ptr = &self.ptr.types
        return val
    @property
    def objs(self):
        val = IndexMap_u32__ObjRefPc()
        val.ptr = &self.ptr.objs
        return val

cdef class SubBlocks1RefPc:
    cdef lotrc_rs.SubBlocks1RefPc* ptr
    @property
    def info(self):
        val = SubBlocksInfoRefPc()
        val.ptr = &self.ptr.info
        return val
    @property
    def files(self):
        val = IndexMap_u32__DataRefPc()
        val.ptr = &self.ptr.files
        return val
    @property
    def lua(self):
        val = IndexMap_u32__LuaRefPc()
        val.ptr = &self.ptr.lua
        return val
    @property
    def subtitles(self):
        val = IndexMap_u32__SSARefPc()
        val.ptr = &self.ptr.subtitles
        return val
    @property
    def atlas1(self):
        val = Option_AtlasUVRefPc()
        val.ptr = &self.ptr.atlas1
        return val
    @property
    def atlas2(self):
        val = Option_AtlasUVRefPc()
        val.ptr = &self.ptr.atlas2
        return val
    @property
    def level(self):
        val = GameObjsRefPc()
        val.ptr = &self.ptr.level
        return val

cdef class StringKeysHeaderPc:
    cdef lotrc_rs.StringKeysHeaderPc* ptr
    @property
    def num_a(self):
        return self.ptr.num_a
    @property
    def num_b(self):
        return self.ptr.num_b
    @property
    def z2(self):
        return self.ptr.z2
    @property
    def z3(self):
        return self.ptr.z3
    @property
    def z4(self):
        return self.ptr.z4
    @property
    def z5(self):
        return self.ptr.z5

cdef class ref_slice_StringKeysValPc:
    cdef lotrc_rs.ref_slice_StringKeysValPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = StringKeysValPc()
        val.ptr = lotrc_rs.ref_slice_StringKeysValPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_StringKeysValPc_len(self.ptr)

cdef class ref_slice_u32Pc:
    cdef lotrc_rs.ref_slice_u32Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_u32Pc_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_u32Pc_len(self.ptr)

cdef class StringKeysRefPc:
    cdef lotrc_rs.StringKeysRefPc* ptr
    @property
    def header(self):
        val = StringKeysHeaderPc()
        val.ptr = self.ptr.header
        return val
    @property
    def vals(self):
        val = ref_slice_StringKeysValPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def pad(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.pad
        return val

cdef class Block1RefPc:
    cdef lotrc_rs.Block1RefPc* ptr
    @property
    def infos(self):
        val = InfosRefPc()
        val.ptr = &self.ptr.infos
        return val
    @property
    def objs(self):
        val = ObjsRefPc()
        val.ptr = &self.ptr.objs
        return val
    @property
    def sub_blocks(self):
        val = SubBlocks1RefPc()
        val.ptr = &self.ptr.sub_blocks
        return val
    @property
    def string_keys(self):
        val = StringKeysRefPc()
        val.ptr = &self.ptr.string_keys
        return val

cdef class Option_SprayRefPc:
    cdef lotrc_rs.Option_SprayRefPc* ptr
    def get(self):
        val = SprayRefPc()
        val.ptr = lotrc_rs.Option_SprayRefPc_get(self.ptr)
        return val

cdef class Option_CrowdRefPc:
    cdef lotrc_rs.Option_CrowdRefPc* ptr
    def get(self):
        val = CrowdRefPc()
        val.ptr = lotrc_rs.Option_CrowdRefPc_get(self.ptr)
        return val

cdef class Option_PFieldsRefPc:
    cdef lotrc_rs.Option_PFieldsRefPc* ptr
    def get(self):
        val = DataRefPc()
        val.ptr = lotrc_rs.Option_PFieldsRefPc_get(self.ptr)
        return val

cdef class IndexMap_u32__LangStringsRefPc:
    cdef lotrc_rs.IndexMap_u32__LangStringsRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = LangStringsRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__LangStringsRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__LangStringsRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__LangStringsRefPc_keys(self.ptr, keys.ptr)

cdef class SubBlocks2RefPc:
    cdef lotrc_rs.SubBlocks2RefPc* ptr
    @property
    def info(self):
        val = SubBlocksInfoRefPc()
        val.ptr = &self.ptr.info
        return val
    @property
    def spray(self):
        val = Option_SprayRefPc()
        val.ptr = &self.ptr.spray
        return val
    @property
    def crowd(self):
        val = Option_CrowdRefPc()
        val.ptr = &self.ptr.crowd
        return val
    @property
    def pfields(self):
        val = Option_PFieldsRefPc()
        val.ptr = &self.ptr.pfields
        return val
    @property
    def langs(self):
        val = IndexMap_u32__LangStringsRefPc()
        val.ptr = &self.ptr.langs
        return val
    @property
    def files(self):
        val = IndexMap_u32__DataRefPc()
        val.ptr = &self.ptr.files
        return val

cdef class Block2RefPc:
    cdef lotrc_rs.Block2RefPc* ptr
    @property
    def sub_blocks(self):
        val = SubBlocks2RefPc()
        val.ptr = &self.ptr.sub_blocks
        return val
    @property
    def offsets(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.offsets
        return val

cdef class IndexMap_u32__AnimationRefPc:
    cdef lotrc_rs.IndexMap_u32__AnimationRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = AnimationRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__AnimationRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__AnimationRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__AnimationRefPc_keys(self.ptr, keys.ptr)

cdef class AnimationsRefPc:
    cdef lotrc_rs.AnimationsRefPc* ptr
    def dump(self, DumpInfosPc infos):
        val = OwnedVecCompressedData()
        val.ptr = lotrc_rs.AnimationsRefPc_dump(self.ptr, infos.ptr)
        return val
    @property
    def animations(self):
        val = IndexMap_u32__AnimationRefPc()
        val.ptr = &self.ptr.animations
        return val
    @property
    def block_infos(self):
        val = ref_slice_AnimationBlockInfoPc()
        val.ptr = &self.ptr.block_infos
        return val

cdef class ref_slice_BlockAValPc:
    cdef lotrc_rs.ref_slice_BlockAValPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BlockAValPc()
        val.ptr = lotrc_rs.ref_slice_BlockAValPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BlockAValPc_len(self.ptr)

cdef class ref_slice_CompressedDataRef:
    cdef lotrc_rs.ref_slice_CompressedDataRef* ptr
    @staticmethod
    def get(slice_CompressedDataRef slice, lotrc_rs.uintptr_t idx):
        val = CompressedDataRef()
        val.ptr = lotrc_rs.ref_slice_CompressedDataRef_get(slice.ptr, idx)
        return val
    @staticmethod
    def len(slice_CompressedDataRef slice):
        return lotrc_rs.ref_slice_CompressedDataRef_len(slice.ptr)

cdef class PakRefPc:
    cdef lotrc_rs.PakRefPc* ptr
    @property
    def header(self):
        val = PakHeaderPc()
        val.ptr = self.ptr.header
        return val
    @property
    def strings(self):
        val = slice_string()
        val.ptr = &self.ptr.strings
        return val
    @property
    def block1(self):
        val = Block1RefPc()
        val.ptr = &self.ptr.block1
        return val
    @property
    def block2(self):
        val = Block2RefPc()
        val.ptr = &self.ptr.block2
        return val
    @property
    def animations(self):
        val = AnimationsRefPc()
        val.ptr = &self.ptr.animations
        return val
    @property
    def vals_a(self):
        val = ref_slice_BlockAValPc()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def animation_data(self):
        val = ref_slice_CompressedDataRef()
        val.ptr = &self.ptr.animation_data
        return val

cdef class BinHeaderPc:
    cdef lotrc_rs.BinHeaderPc* ptr
    @property
    def constx06(self):
        return self.ptr.constx06
    @property
    def version(self):
        return self.ptr.version
    @property
    def strings_offset(self):
        return self.ptr.strings_offset
    @property
    def strings_size(self):
        return self.ptr.strings_size
    @property
    def strings_num(self):
        return self.ptr.strings_num
    @property
    def asset_handle_num(self):
        return self.ptr.asset_handle_num
    @property
    def asset_handle_offset(self):
        return self.ptr.asset_handle_offset
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def vdata_num(self):
        return self.ptr.vdata_num
    @property
    def vdata_num_alt(self):
        return self.ptr.vdata_num_alt
    @property
    def texdata_num(self):
        return self.ptr.texdata_num
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def unk_33(self):
        return self.ptr.unk_33
    @property
    def unk_34(self):
        return self.ptr.unk_34
    @property
    def unk_35(self):
        return self.ptr.unk_35
    @property
    def unk_36(self):
        return self.ptr.unk_36
    @property
    def unk_37(self):
        return self.ptr.unk_37
    @property
    def unk_38(self):
        return self.ptr.unk_38
    @property
    def unk_39(self):
        return self.ptr.unk_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42

cdef class ref_slice_AssetHandlePc:
    cdef lotrc_rs.ref_slice_AssetHandlePc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AssetHandlePc()
        val.ptr = lotrc_rs.ref_slice_AssetHandlePc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AssetHandlePc_len(self.ptr)

cdef class IndexMap_u32_______CompressedDataRef:
    cdef lotrc_rs.IndexMap_u32_______CompressedDataRef* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = CompressedDataRef()
        val.ptr = dereference(lotrc_rs.IndexMap_u32_______CompressedDataRef_get(self.ptr, &key))
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32_______CompressedDataRef_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32_______CompressedDataRef_keys(self.ptr, keys.ptr)

cdef class BinRefPc:
    cdef lotrc_rs.BinRefPc* ptr
    @property
    def header(self):
        val = BinHeaderPc()
        val.ptr = self.ptr.header
        return val
    @property
    def strings(self):
        val = slice_string()
        val.ptr = &self.ptr.strings
        return val
    @property
    def asset_handles(self):
        val = ref_slice_AssetHandlePc()
        val.ptr = &self.ptr.asset_handles
        return val
    @property
    def model_data(self):
        val = IndexMap_u32_______CompressedDataRef()
        val.ptr = &self.ptr.model_data
        return val
    @property
    def texture_data(self):
        val = IndexMap_u32_______CompressedDataRef()
        val.ptr = &self.ptr.texture_data
        return val

cdef class LevelRefPc:
    cdef lotrc_rs.LevelRefPc* ptr
    def dump(self, lotrc_rs.uint32_t compression):
        val = OwnedLevelData()
        val.ptr = lotrc_rs.LevelRefPc_dump(self.ptr, compression)
        return val
    @property
    def pak(self):
        val = PakRefPc()
        val.ptr = &self.ptr.pak
        return val
    @property
    def bin(self):
        val = BinRefPc()
        val.ptr = &self.ptr.bin
        return val

cdef class PakHeaderXbox:
    cdef lotrc_rs.PakHeaderXbox* ptr
    @property
    def block_a_num(self):
        return self.ptr.block_a_num
    @property
    def block_a_offset(self):
        return self.ptr.block_a_offset
    @property
    def constx13(self):
        return self.ptr.constx13
    @property
    def version(self):
        return self.ptr.version
    @property
    def strings_offset(self):
        return self.ptr.strings_offset
    @property
    def strings_size(self):
        return self.ptr.strings_size
    @property
    def strings_num(self):
        return self.ptr.strings_num
    @property
    def block1_offset(self):
        return self.ptr.block1_offset
    @property
    def block1_size(self):
        return self.ptr.block1_size
    @property
    def block1_size_comp(self):
        return self.ptr.block1_size_comp
    @property
    def sub_blocks1_offset(self):
        return self.ptr.sub_blocks1_offset
    @property
    def block2_offset(self):
        return self.ptr.block2_offset
    @property
    def block2_size(self):
        return self.ptr.block2_size
    @property
    def block2_size_comp(self):
        return self.ptr.block2_size_comp
    @property
    def sub_blocks2_offset(self):
        return self.ptr.sub_blocks2_offset
    @property
    def string_keys_offset(self):
        return self.ptr.string_keys_offset
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def obja_size(self):
        return self.ptr.obja_size
    @property
    def obj0_size(self):
        return self.ptr.obj0_size
    @property
    def model_info_size(self):
        return self.ptr.model_info_size
    @property
    def buffer_info_size(self):
        return self.ptr.buffer_info_size
    @property
    def mat1_size(self):
        return self.ptr.mat1_size
    @property
    def mat2_size(self):
        return self.ptr.mat2_size
    @property
    def mat3_size(self):
        return self.ptr.mat3_size
    @property
    def mat4_size(self):
        return self.ptr.mat4_size
    @property
    def mat_extra_size(self):
        return self.ptr.mat_extra_size
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def shape_info_size(self):
        return self.ptr.shape_info_size
    @property
    def hk_shape_info_size(self):
        return self.ptr.hk_shape_info_size
    @property
    def hk_constraint_data_size(self):
        return self.ptr.hk_constraint_data_size
    @property
    def vbuff_info_size(self):
        return self.ptr.vbuff_info_size
    @property
    def ibuff_info_size(self):
        return self.ptr.ibuff_info_size
    @property
    def texture_info_size(self):
        return self.ptr.texture_info_size
    @property
    def animation_info_size(self):
        return self.ptr.animation_info_size
    @property
    def hk_constraint_info_size(self):
        return self.ptr.hk_constraint_info_size
    @property
    def effect_info_size(self):
        return self.ptr.effect_info_size
    @property
    def pfield_info_size(self):
        return self.ptr.pfield_info_size
    @property
    def gfx_block_info_size(self):
        return self.ptr.gfx_block_info_size
    @property
    def animation_block_info_size(self):
        return self.ptr.animation_block_info_size
    @property
    def foliage_info_size(self):
        return self.ptr.foliage_info_size
    @property
    def radiosity_vals_info_size(self):
        return self.ptr.radiosity_vals_info_size
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def obja_num(self):
        return self.ptr.obja_num
    @property
    def obj0_num(self):
        return self.ptr.obj0_num
    @property
    def model_info_num(self):
        return self.ptr.model_info_num
    @property
    def buffer_info_num(self):
        return self.ptr.buffer_info_num
    @property
    def mat1_num(self):
        return self.ptr.mat1_num
    @property
    def mat2_num(self):
        return self.ptr.mat2_num
    @property
    def mat3_num(self):
        return self.ptr.mat3_num
    @property
    def mat4_num(self):
        return self.ptr.mat4_num
    @property
    def mat_extra_num(self):
        return self.ptr.mat_extra_num
    @property
    def unk_51(self):
        return self.ptr.unk_51
    @property
    def shape_info_num(self):
        return self.ptr.shape_info_num
    @property
    def hk_shape_info_num(self):
        return self.ptr.hk_shape_info_num
    @property
    def hk_constraint_data_num(self):
        return self.ptr.hk_constraint_data_num
    @property
    def vbuff_info_num(self):
        return self.ptr.vbuff_info_num
    @property
    def ibuff_info_num(self):
        return self.ptr.ibuff_info_num
    @property
    def texture_info_num(self):
        return self.ptr.texture_info_num
    @property
    def animation_info_num(self):
        return self.ptr.animation_info_num
    @property
    def hk_constraint_info_num(self):
        return self.ptr.hk_constraint_info_num
    @property
    def effect_info_num(self):
        return self.ptr.effect_info_num
    @property
    def pfield_info_num(self):
        return self.ptr.pfield_info_num
    @property
    def gfx_block_info_num(self):
        return self.ptr.gfx_block_info_num
    @property
    def animation_block_info_num(self):
        return self.ptr.animation_block_info_num
    @property
    def foliage_info_num(self):
        return self.ptr.foliage_info_num
    @property
    def radiosity_vals_info_num(self):
        return self.ptr.radiosity_vals_info_num
    @property
    def unk_66(self):
        return self.ptr.unk_66
    @property
    def obja_offset(self):
        return self.ptr.obja_offset
    @property
    def obj0_offset(self):
        return self.ptr.obj0_offset
    @property
    def model_info_offset(self):
        return self.ptr.model_info_offset
    @property
    def buffer_info_offset(self):
        return self.ptr.buffer_info_offset
    @property
    def mat1_offset(self):
        return self.ptr.mat1_offset
    @property
    def mat2_offset(self):
        return self.ptr.mat2_offset
    @property
    def mat3_offset(self):
        return self.ptr.mat3_offset
    @property
    def mat4_offset(self):
        return self.ptr.mat4_offset
    @property
    def mat_extra_offset(self):
        return self.ptr.mat_extra_offset
    @property
    def unk_76(self):
        return self.ptr.unk_76
    @property
    def shape_info_offset(self):
        return self.ptr.shape_info_offset
    @property
    def hk_shape_info_offset(self):
        return self.ptr.hk_shape_info_offset
    @property
    def hk_constraint_data_offset(self):
        return self.ptr.hk_constraint_data_offset
    @property
    def vbuff_info_offset(self):
        return self.ptr.vbuff_info_offset
    @property
    def ibuff_info_offset(self):
        return self.ptr.ibuff_info_offset
    @property
    def texture_info_offset(self):
        return self.ptr.texture_info_offset
    @property
    def animation_info_offset(self):
        return self.ptr.animation_info_offset
    @property
    def hk_constraint_info_offset(self):
        return self.ptr.hk_constraint_info_offset
    @property
    def effect_info_offset(self):
        return self.ptr.effect_info_offset
    @property
    def pfield_info_offset(self):
        return self.ptr.pfield_info_offset
    @property
    def gfx_block_info_offset(self):
        return self.ptr.gfx_block_info_offset
    @property
    def animation_block_info_offset(self):
        return self.ptr.animation_block_info_offset
    @property
    def foliage_info_offset(self):
        return self.ptr.foliage_info_offset
    @property
    def radiosity_vals_info_offset(self):
        return self.ptr.radiosity_vals_info_offset
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def unk_114(self):
        return self.ptr.unk_114
    @property
    def unk_115(self):
        return self.ptr.unk_115
    @property
    def block2_offsets_num(self):
        return self.ptr.block2_offsets_num
    @property
    def block2_offsets_offset(self):
        return self.ptr.block2_offsets_offset

cdef class ref_slice_ObjAXbox:
    cdef lotrc_rs.ref_slice_ObjAXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ObjAXbox()
        val.ptr = lotrc_rs.ref_slice_ObjAXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_ObjAXbox_len(self.ptr)

cdef class ref_slice_Obj0Xbox:
    cdef lotrc_rs.ref_slice_Obj0Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj0Xbox()
        val.ptr = lotrc_rs.ref_slice_Obj0Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Obj0Xbox_len(self.ptr)

cdef class ref_slice_ModelInfoXbox:
    cdef lotrc_rs.ref_slice_ModelInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ModelInfoXbox()
        val.ptr = lotrc_rs.ref_slice_ModelInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_ModelInfoXbox_len(self.ptr)

cdef class ref_slice_BufferInfoXbox:
    cdef lotrc_rs.ref_slice_BufferInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BufferInfoXbox()
        val.ptr = lotrc_rs.ref_slice_BufferInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BufferInfoXbox_len(self.ptr)

cdef class ref_slice_Mat1Xbox:
    cdef lotrc_rs.ref_slice_Mat1Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat1Xbox()
        val.ptr = lotrc_rs.ref_slice_Mat1Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat1Xbox_len(self.ptr)

cdef class ref_slice_Mat2Xbox:
    cdef lotrc_rs.ref_slice_Mat2Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat2Xbox()
        val.ptr = lotrc_rs.ref_slice_Mat2Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat2Xbox_len(self.ptr)

cdef class ref_slice_Mat3Xbox:
    cdef lotrc_rs.ref_slice_Mat3Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat3Xbox()
        val.ptr = lotrc_rs.ref_slice_Mat3Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat3Xbox_len(self.ptr)

cdef class ref_slice_Mat4Xbox:
    cdef lotrc_rs.ref_slice_Mat4Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat4Xbox()
        val.ptr = lotrc_rs.ref_slice_Mat4Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat4Xbox_len(self.ptr)

cdef class ref_slice_MatExtraXbox:
    cdef lotrc_rs.ref_slice_MatExtraXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = MatExtraXbox()
        val.ptr = lotrc_rs.ref_slice_MatExtraXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_MatExtraXbox_len(self.ptr)

cdef class ref_slice_ShapeInfoXbox:
    cdef lotrc_rs.ref_slice_ShapeInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ShapeInfoXbox()
        val.ptr = lotrc_rs.ref_slice_ShapeInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_ShapeInfoXbox_len(self.ptr)

cdef class ref_slice_HkShapeInfoXbox:
    cdef lotrc_rs.ref_slice_HkShapeInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkShapeInfoXbox()
        val.ptr = lotrc_rs.ref_slice_HkShapeInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_HkShapeInfoXbox_len(self.ptr)

cdef class ref_slice_HkConstraintDataXbox:
    cdef lotrc_rs.ref_slice_HkConstraintDataXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintDataXbox()
        val.ptr = lotrc_rs.ref_slice_HkConstraintDataXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_HkConstraintDataXbox_len(self.ptr)

cdef class ref_slice_VBuffInfoXbox:
    cdef lotrc_rs.ref_slice_VBuffInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = VBuffInfoXbox()
        val.ptr = lotrc_rs.ref_slice_VBuffInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_VBuffInfoXbox_len(self.ptr)

cdef class ref_slice_IBuffInfoXbox:
    cdef lotrc_rs.ref_slice_IBuffInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = IBuffInfoXbox()
        val.ptr = lotrc_rs.ref_slice_IBuffInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_IBuffInfoXbox_len(self.ptr)

cdef class ref_slice_TextureInfoXbox:
    cdef lotrc_rs.ref_slice_TextureInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TextureInfoXbox()
        val.ptr = lotrc_rs.ref_slice_TextureInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_TextureInfoXbox_len(self.ptr)

cdef class ref_slice_AnimationInfoXbox:
    cdef lotrc_rs.ref_slice_AnimationInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationInfoXbox()
        val.ptr = lotrc_rs.ref_slice_AnimationInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AnimationInfoXbox_len(self.ptr)

cdef class ref_slice_HkConstraintInfoXbox:
    cdef lotrc_rs.ref_slice_HkConstraintInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintInfoXbox()
        val.ptr = lotrc_rs.ref_slice_HkConstraintInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_HkConstraintInfoXbox_len(self.ptr)

cdef class ref_slice_EffectInfoXbox:
    cdef lotrc_rs.ref_slice_EffectInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = EffectInfoXbox()
        val.ptr = lotrc_rs.ref_slice_EffectInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_EffectInfoXbox_len(self.ptr)

cdef class ref_slice_PFieldInfoXbox:
    cdef lotrc_rs.ref_slice_PFieldInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = PFieldInfoXbox()
        val.ptr = lotrc_rs.ref_slice_PFieldInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_PFieldInfoXbox_len(self.ptr)

cdef class ref_slice_GFXBlockInfoXbox:
    cdef lotrc_rs.ref_slice_GFXBlockInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = GFXBlockInfoXbox()
        val.ptr = lotrc_rs.ref_slice_GFXBlockInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_GFXBlockInfoXbox_len(self.ptr)

cdef class ref_slice_AnimationBlockInfoXbox:
    cdef lotrc_rs.ref_slice_AnimationBlockInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationBlockInfoXbox()
        val.ptr = lotrc_rs.ref_slice_AnimationBlockInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AnimationBlockInfoXbox_len(self.ptr)

cdef class ref_slice_FoliageInfoXbox:
    cdef lotrc_rs.ref_slice_FoliageInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = FoliageInfoXbox()
        val.ptr = lotrc_rs.ref_slice_FoliageInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_FoliageInfoXbox_len(self.ptr)

cdef class ref_slice_RadiosityValsInfoXbox:
    cdef lotrc_rs.ref_slice_RadiosityValsInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RadiosityValsInfoXbox()
        val.ptr = lotrc_rs.ref_slice_RadiosityValsInfoXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RadiosityValsInfoXbox_len(self.ptr)

cdef class InfosRefXbox:
    cdef lotrc_rs.InfosRefXbox* ptr
    @property
    def objas(self):
        val = ref_slice_ObjAXbox()
        val.ptr = &self.ptr.objas
        return val
    @property
    def obj0s(self):
        val = ref_slice_Obj0Xbox()
        val.ptr = &self.ptr.obj0s
        return val
    @property
    def models(self):
        val = ref_slice_ModelInfoXbox()
        val.ptr = &self.ptr.models
        return val
    @property
    def buffers(self):
        val = ref_slice_BufferInfoXbox()
        val.ptr = &self.ptr.buffers
        return val
    @property
    def mat1s(self):
        val = ref_slice_Mat1Xbox()
        val.ptr = &self.ptr.mat1s
        return val
    @property
    def mat2s(self):
        val = ref_slice_Mat2Xbox()
        val.ptr = &self.ptr.mat2s
        return val
    @property
    def mat3s(self):
        val = ref_slice_Mat3Xbox()
        val.ptr = &self.ptr.mat3s
        return val
    @property
    def mat4s(self):
        val = ref_slice_Mat4Xbox()
        val.ptr = &self.ptr.mat4s
        return val
    @property
    def mat_extras(self):
        val = ref_slice_MatExtraXbox()
        val.ptr = &self.ptr.mat_extras
        return val
    @property
    def shapes(self):
        val = ref_slice_ShapeInfoXbox()
        val.ptr = &self.ptr.shapes
        return val
    @property
    def hk_shapes(self):
        val = ref_slice_HkShapeInfoXbox()
        val.ptr = &self.ptr.hk_shapes
        return val
    @property
    def hk_constraint_datas(self):
        val = ref_slice_HkConstraintDataXbox()
        val.ptr = &self.ptr.hk_constraint_datas
        return val
    @property
    def vbuffs(self):
        val = ref_slice_VBuffInfoXbox()
        val.ptr = &self.ptr.vbuffs
        return val
    @property
    def ibuffs(self):
        val = ref_slice_IBuffInfoXbox()
        val.ptr = &self.ptr.ibuffs
        return val
    @property
    def textures(self):
        val = ref_slice_TextureInfoXbox()
        val.ptr = &self.ptr.textures
        return val
    @property
    def animations(self):
        val = ref_slice_AnimationInfoXbox()
        val.ptr = &self.ptr.animations
        return val
    @property
    def hk_constraints(self):
        val = ref_slice_HkConstraintInfoXbox()
        val.ptr = &self.ptr.hk_constraints
        return val
    @property
    def effects(self):
        val = ref_slice_EffectInfoXbox()
        val.ptr = &self.ptr.effects
        return val
    @property
    def pfields(self):
        val = ref_slice_PFieldInfoXbox()
        val.ptr = &self.ptr.pfields
        return val
    @property
    def gfxs(self):
        val = ref_slice_GFXBlockInfoXbox()
        val.ptr = &self.ptr.gfxs
        return val
    @property
    def animation_blocks(self):
        val = ref_slice_AnimationBlockInfoXbox()
        val.ptr = &self.ptr.animation_blocks
        return val
    @property
    def foliages(self):
        val = ref_slice_FoliageInfoXbox()
        val.ptr = &self.ptr.foliages
        return val
    @property
    def radiosity_vals(self):
        val = ref_slice_RadiosityValsInfoXbox()
        val.ptr = &self.ptr.radiosity_vals
        return val

cdef class IndexMap_u32__TextureRefXbox:
    cdef lotrc_rs.IndexMap_u32__TextureRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = TextureRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__TextureRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__TextureRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__TextureRefXbox_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__ModelRefXbox:
    cdef lotrc_rs.IndexMap_u32__ModelRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = ModelRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__ModelRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__ModelRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__ModelRefXbox_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__EffectRefXbox:
    cdef lotrc_rs.IndexMap_u32__EffectRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = EffectRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__EffectRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__EffectRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__EffectRefXbox_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__slice_FoliageRefXbox:
    cdef lotrc_rs.IndexMap_u32__slice_FoliageRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = slice_FoliageRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__slice_FoliageRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__slice_FoliageRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__slice_FoliageRefXbox_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__RadiosityValsRefXbox:
    cdef lotrc_rs.IndexMap_u32__RadiosityValsRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = RadiosityValsRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__RadiosityValsRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__RadiosityValsRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__RadiosityValsRefXbox_keys(self.ptr, keys.ptr)

cdef class RadiosityRefXbox:
    cdef lotrc_rs.RadiosityRefXbox* ptr
    @property
    def data(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data
        return val
    @property
    def vals(self):
        val = IndexMap_u32__RadiosityValsRefXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def usage(self):
        return self.ptr.usage

cdef class ObjsRefXbox:
    cdef lotrc_rs.ObjsRefXbox* ptr
    @property
    def textures(self):
        val = IndexMap_u32__TextureRefXbox()
        val.ptr = &self.ptr.textures
        return val
    @property
    def models(self):
        val = IndexMap_u32__ModelRefXbox()
        val.ptr = &self.ptr.models
        return val
    @property
    def effects(self):
        val = IndexMap_u32__EffectRefXbox()
        val.ptr = &self.ptr.effects
        return val
    @property
    def foliages(self):
        val = IndexMap_u32__slice_FoliageRefXbox()
        val.ptr = &self.ptr.foliages
        return val
    @property
    def gfxs(self):
        val = IndexMap_u32__ref_slice_u8()
        val.ptr = &self.ptr.gfxs
        return val
    @property
    def radiosity(self):
        val = RadiosityRefXbox()
        val.ptr = &self.ptr.radiosity
        return val

cdef class SubBlocksHeaderXbox:
    cdef lotrc_rs.SubBlocksHeaderXbox* ptr
    @property
    def z0(self):
        return self.ptr.z0
    @property
    def block_num(self):
        return self.ptr.block_num
    @property
    def z2(self):
        return self.ptr.z2
    @property
    def z3(self):
        return self.ptr.z3

cdef class ref_slice_SubBlocksBlockHeaderXbox:
    cdef lotrc_rs.ref_slice_SubBlocksBlockHeaderXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SubBlocksBlockHeaderXbox()
        val.ptr = lotrc_rs.ref_slice_SubBlocksBlockHeaderXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SubBlocksBlockHeaderXbox_len(self.ptr)

cdef class SubBlocksInfoRefXbox:
    cdef lotrc_rs.SubBlocksInfoRefXbox* ptr
    @property
    def header(self):
        val = SubBlocksHeaderXbox()
        val.ptr = self.ptr.header
        return val
    @property
    def block_headers(self):
        val = ref_slice_SubBlocksBlockHeaderXbox()
        val.ptr = &self.ptr.block_headers
        return val

cdef class IndexMap_u32__DataRefXbox:
    cdef lotrc_rs.IndexMap_u32__DataRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = DataRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__DataRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__DataRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__DataRefXbox_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__LuaRefXbox:
    cdef lotrc_rs.IndexMap_u32__LuaRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = DataRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__LuaRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__LuaRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__LuaRefXbox_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__SSARefXbox:
    cdef lotrc_rs.IndexMap_u32__SSARefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = SSARefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__SSARefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__SSARefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__SSARefXbox_keys(self.ptr, keys.ptr)

cdef class Option_AtlasUVRefXbox:
    cdef lotrc_rs.Option_AtlasUVRefXbox* ptr
    def get(self):
        val = AtlasUVRefXbox()
        val.ptr = lotrc_rs.Option_AtlasUVRefXbox_get(self.ptr)
        return val

cdef class GameObjsHeaderXbox:
    cdef lotrc_rs.GameObjsHeaderXbox* ptr
    @property
    def const_(self):
        return self.ptr.const_
    @property
    def types_num(self):
        return self.ptr.types_num
    @property
    def types_offset(self):
        return self.ptr.types_offset
    @property
    def obj_num(self):
        return self.ptr.obj_num
    @property
    def obj_offset(self):
        return self.ptr.obj_offset
    @property
    def z5(self):
        return self.ptr.z5
    @property
    def z6(self):
        return self.ptr.z6
    @property
    def z7(self):
        return self.ptr.z7

cdef class IndexMap_u32__TypeRefXbox:
    cdef lotrc_rs.IndexMap_u32__TypeRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = TypeRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__TypeRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__TypeRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__TypeRefXbox_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__ObjRefXbox:
    cdef lotrc_rs.IndexMap_u32__ObjRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = ObjRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__ObjRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__ObjRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__ObjRefXbox_keys(self.ptr, keys.ptr)

cdef class GameObjsRefXbox:
    cdef lotrc_rs.GameObjsRefXbox* ptr
    @property
    def header(self):
        val = GameObjsHeaderXbox()
        val.ptr = self.ptr.header
        return val
    @property
    def types(self):
        val = IndexMap_u32__TypeRefXbox()
        val.ptr = &self.ptr.types
        return val
    @property
    def objs(self):
        val = IndexMap_u32__ObjRefXbox()
        val.ptr = &self.ptr.objs
        return val

cdef class SubBlocks1RefXbox:
    cdef lotrc_rs.SubBlocks1RefXbox* ptr
    @property
    def info(self):
        val = SubBlocksInfoRefXbox()
        val.ptr = &self.ptr.info
        return val
    @property
    def files(self):
        val = IndexMap_u32__DataRefXbox()
        val.ptr = &self.ptr.files
        return val
    @property
    def lua(self):
        val = IndexMap_u32__LuaRefXbox()
        val.ptr = &self.ptr.lua
        return val
    @property
    def subtitles(self):
        val = IndexMap_u32__SSARefXbox()
        val.ptr = &self.ptr.subtitles
        return val
    @property
    def atlas1(self):
        val = Option_AtlasUVRefXbox()
        val.ptr = &self.ptr.atlas1
        return val
    @property
    def atlas2(self):
        val = Option_AtlasUVRefXbox()
        val.ptr = &self.ptr.atlas2
        return val
    @property
    def level(self):
        val = GameObjsRefXbox()
        val.ptr = &self.ptr.level
        return val

cdef class StringKeysHeaderXbox:
    cdef lotrc_rs.StringKeysHeaderXbox* ptr
    @property
    def num_a(self):
        return self.ptr.num_a
    @property
    def num_b(self):
        return self.ptr.num_b
    @property
    def z2(self):
        return self.ptr.z2
    @property
    def z3(self):
        return self.ptr.z3
    @property
    def z4(self):
        return self.ptr.z4
    @property
    def z5(self):
        return self.ptr.z5

cdef class ref_slice_StringKeysValXbox:
    cdef lotrc_rs.ref_slice_StringKeysValXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = StringKeysValXbox()
        val.ptr = lotrc_rs.ref_slice_StringKeysValXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_StringKeysValXbox_len(self.ptr)

cdef class ref_slice_u32Xbox:
    cdef lotrc_rs.ref_slice_u32Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_u32Xbox_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_u32Xbox_len(self.ptr)

cdef class StringKeysRefXbox:
    cdef lotrc_rs.StringKeysRefXbox* ptr
    @property
    def header(self):
        val = StringKeysHeaderXbox()
        val.ptr = self.ptr.header
        return val
    @property
    def vals(self):
        val = ref_slice_StringKeysValXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def pad(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.pad
        return val

cdef class Block1RefXbox:
    cdef lotrc_rs.Block1RefXbox* ptr
    @property
    def infos(self):
        val = InfosRefXbox()
        val.ptr = &self.ptr.infos
        return val
    @property
    def objs(self):
        val = ObjsRefXbox()
        val.ptr = &self.ptr.objs
        return val
    @property
    def sub_blocks(self):
        val = SubBlocks1RefXbox()
        val.ptr = &self.ptr.sub_blocks
        return val
    @property
    def string_keys(self):
        val = StringKeysRefXbox()
        val.ptr = &self.ptr.string_keys
        return val

cdef class Option_SprayRefXbox:
    cdef lotrc_rs.Option_SprayRefXbox* ptr
    def get(self):
        val = SprayRefXbox()
        val.ptr = lotrc_rs.Option_SprayRefXbox_get(self.ptr)
        return val

cdef class Option_CrowdRefXbox:
    cdef lotrc_rs.Option_CrowdRefXbox* ptr
    def get(self):
        val = CrowdRefXbox()
        val.ptr = lotrc_rs.Option_CrowdRefXbox_get(self.ptr)
        return val

cdef class Option_PFieldsRefXbox:
    cdef lotrc_rs.Option_PFieldsRefXbox* ptr
    def get(self):
        val = DataRefXbox()
        val.ptr = lotrc_rs.Option_PFieldsRefXbox_get(self.ptr)
        return val

cdef class IndexMap_u32__LangStringsRefXbox:
    cdef lotrc_rs.IndexMap_u32__LangStringsRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = LangStringsRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__LangStringsRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__LangStringsRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__LangStringsRefXbox_keys(self.ptr, keys.ptr)

cdef class SubBlocks2RefXbox:
    cdef lotrc_rs.SubBlocks2RefXbox* ptr
    @property
    def info(self):
        val = SubBlocksInfoRefXbox()
        val.ptr = &self.ptr.info
        return val
    @property
    def spray(self):
        val = Option_SprayRefXbox()
        val.ptr = &self.ptr.spray
        return val
    @property
    def crowd(self):
        val = Option_CrowdRefXbox()
        val.ptr = &self.ptr.crowd
        return val
    @property
    def pfields(self):
        val = Option_PFieldsRefXbox()
        val.ptr = &self.ptr.pfields
        return val
    @property
    def langs(self):
        val = IndexMap_u32__LangStringsRefXbox()
        val.ptr = &self.ptr.langs
        return val
    @property
    def files(self):
        val = IndexMap_u32__DataRefXbox()
        val.ptr = &self.ptr.files
        return val

cdef class Block2RefXbox:
    cdef lotrc_rs.Block2RefXbox* ptr
    @property
    def sub_blocks(self):
        val = SubBlocks2RefXbox()
        val.ptr = &self.ptr.sub_blocks
        return val
    @property
    def offsets(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.offsets
        return val

cdef class IndexMap_u32__AnimationRefXbox:
    cdef lotrc_rs.IndexMap_u32__AnimationRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = AnimationRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__AnimationRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__AnimationRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__AnimationRefXbox_keys(self.ptr, keys.ptr)

cdef class AnimationsRefXbox:
    cdef lotrc_rs.AnimationsRefXbox* ptr
    def dump(self, DumpInfosXbox infos):
        val = OwnedVecCompressedData()
        val.ptr = lotrc_rs.AnimationsRefXbox_dump(self.ptr, infos.ptr)
        return val
    @property
    def animations(self):
        val = IndexMap_u32__AnimationRefXbox()
        val.ptr = &self.ptr.animations
        return val
    @property
    def block_infos(self):
        val = ref_slice_AnimationBlockInfoXbox()
        val.ptr = &self.ptr.block_infos
        return val

cdef class ref_slice_BlockAValXbox:
    cdef lotrc_rs.ref_slice_BlockAValXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BlockAValXbox()
        val.ptr = lotrc_rs.ref_slice_BlockAValXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BlockAValXbox_len(self.ptr)

cdef class PakRefXbox:
    cdef lotrc_rs.PakRefXbox* ptr
    @property
    def header(self):
        val = PakHeaderXbox()
        val.ptr = self.ptr.header
        return val
    @property
    def strings(self):
        val = slice_string()
        val.ptr = &self.ptr.strings
        return val
    @property
    def block1(self):
        val = Block1RefXbox()
        val.ptr = &self.ptr.block1
        return val
    @property
    def block2(self):
        val = Block2RefXbox()
        val.ptr = &self.ptr.block2
        return val
    @property
    def animations(self):
        val = AnimationsRefXbox()
        val.ptr = &self.ptr.animations
        return val
    @property
    def vals_a(self):
        val = ref_slice_BlockAValXbox()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def animation_data(self):
        val = ref_slice_CompressedDataRef()
        val.ptr = &self.ptr.animation_data
        return val

cdef class BinHeaderXbox:
    cdef lotrc_rs.BinHeaderXbox* ptr
    @property
    def constx06(self):
        return self.ptr.constx06
    @property
    def version(self):
        return self.ptr.version
    @property
    def strings_offset(self):
        return self.ptr.strings_offset
    @property
    def strings_size(self):
        return self.ptr.strings_size
    @property
    def strings_num(self):
        return self.ptr.strings_num
    @property
    def asset_handle_num(self):
        return self.ptr.asset_handle_num
    @property
    def asset_handle_offset(self):
        return self.ptr.asset_handle_offset
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def vdata_num(self):
        return self.ptr.vdata_num
    @property
    def vdata_num_alt(self):
        return self.ptr.vdata_num_alt
    @property
    def texdata_num(self):
        return self.ptr.texdata_num
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def unk_33(self):
        return self.ptr.unk_33
    @property
    def unk_34(self):
        return self.ptr.unk_34
    @property
    def unk_35(self):
        return self.ptr.unk_35
    @property
    def unk_36(self):
        return self.ptr.unk_36
    @property
    def unk_37(self):
        return self.ptr.unk_37
    @property
    def unk_38(self):
        return self.ptr.unk_38
    @property
    def unk_39(self):
        return self.ptr.unk_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42

cdef class ref_slice_AssetHandleXbox:
    cdef lotrc_rs.ref_slice_AssetHandleXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AssetHandleXbox()
        val.ptr = lotrc_rs.ref_slice_AssetHandleXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AssetHandleXbox_len(self.ptr)

cdef class BinRefXbox:
    cdef lotrc_rs.BinRefXbox* ptr
    @property
    def header(self):
        val = BinHeaderXbox()
        val.ptr = self.ptr.header
        return val
    @property
    def strings(self):
        val = slice_string()
        val.ptr = &self.ptr.strings
        return val
    @property
    def asset_handles(self):
        val = ref_slice_AssetHandleXbox()
        val.ptr = &self.ptr.asset_handles
        return val
    @property
    def model_data(self):
        val = IndexMap_u32_______CompressedDataRef()
        val.ptr = &self.ptr.model_data
        return val
    @property
    def texture_data(self):
        val = IndexMap_u32_______CompressedDataRef()
        val.ptr = &self.ptr.texture_data
        return val

cdef class LevelRefXbox:
    cdef lotrc_rs.LevelRefXbox* ptr
    def dump(self, lotrc_rs.uint32_t compression):
        val = OwnedLevelData()
        val.ptr = lotrc_rs.LevelRefXbox_dump(self.ptr, compression)
        return val
    @property
    def pak(self):
        val = PakRefXbox()
        val.ptr = &self.ptr.pak
        return val
    @property
    def bin(self):
        val = BinRefXbox()
        val.ptr = &self.ptr.bin
        return val

cdef class PakHeaderPs3:
    cdef lotrc_rs.PakHeaderPs3* ptr
    @property
    def block_a_num(self):
        return self.ptr.block_a_num
    @property
    def block_a_offset(self):
        return self.ptr.block_a_offset
    @property
    def constx13(self):
        return self.ptr.constx13
    @property
    def version(self):
        return self.ptr.version
    @property
    def strings_offset(self):
        return self.ptr.strings_offset
    @property
    def strings_size(self):
        return self.ptr.strings_size
    @property
    def strings_num(self):
        return self.ptr.strings_num
    @property
    def block1_offset(self):
        return self.ptr.block1_offset
    @property
    def block1_size(self):
        return self.ptr.block1_size
    @property
    def block1_size_comp(self):
        return self.ptr.block1_size_comp
    @property
    def sub_blocks1_offset(self):
        return self.ptr.sub_blocks1_offset
    @property
    def block2_offset(self):
        return self.ptr.block2_offset
    @property
    def block2_size(self):
        return self.ptr.block2_size
    @property
    def block2_size_comp(self):
        return self.ptr.block2_size_comp
    @property
    def sub_blocks2_offset(self):
        return self.ptr.sub_blocks2_offset
    @property
    def string_keys_offset(self):
        return self.ptr.string_keys_offset
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def obja_size(self):
        return self.ptr.obja_size
    @property
    def obj0_size(self):
        return self.ptr.obj0_size
    @property
    def model_info_size(self):
        return self.ptr.model_info_size
    @property
    def buffer_info_size(self):
        return self.ptr.buffer_info_size
    @property
    def mat1_size(self):
        return self.ptr.mat1_size
    @property
    def mat2_size(self):
        return self.ptr.mat2_size
    @property
    def mat3_size(self):
        return self.ptr.mat3_size
    @property
    def mat4_size(self):
        return self.ptr.mat4_size
    @property
    def mat_extra_size(self):
        return self.ptr.mat_extra_size
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def shape_info_size(self):
        return self.ptr.shape_info_size
    @property
    def hk_shape_info_size(self):
        return self.ptr.hk_shape_info_size
    @property
    def hk_constraint_data_size(self):
        return self.ptr.hk_constraint_data_size
    @property
    def vbuff_info_size(self):
        return self.ptr.vbuff_info_size
    @property
    def ibuff_info_size(self):
        return self.ptr.ibuff_info_size
    @property
    def texture_info_size(self):
        return self.ptr.texture_info_size
    @property
    def animation_info_size(self):
        return self.ptr.animation_info_size
    @property
    def hk_constraint_info_size(self):
        return self.ptr.hk_constraint_info_size
    @property
    def effect_info_size(self):
        return self.ptr.effect_info_size
    @property
    def pfield_info_size(self):
        return self.ptr.pfield_info_size
    @property
    def gfx_block_info_size(self):
        return self.ptr.gfx_block_info_size
    @property
    def animation_block_info_size(self):
        return self.ptr.animation_block_info_size
    @property
    def foliage_info_size(self):
        return self.ptr.foliage_info_size
    @property
    def radiosity_vals_info_size(self):
        return self.ptr.radiosity_vals_info_size
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def obja_num(self):
        return self.ptr.obja_num
    @property
    def obj0_num(self):
        return self.ptr.obj0_num
    @property
    def model_info_num(self):
        return self.ptr.model_info_num
    @property
    def buffer_info_num(self):
        return self.ptr.buffer_info_num
    @property
    def mat1_num(self):
        return self.ptr.mat1_num
    @property
    def mat2_num(self):
        return self.ptr.mat2_num
    @property
    def mat3_num(self):
        return self.ptr.mat3_num
    @property
    def mat4_num(self):
        return self.ptr.mat4_num
    @property
    def mat_extra_num(self):
        return self.ptr.mat_extra_num
    @property
    def unk_51(self):
        return self.ptr.unk_51
    @property
    def shape_info_num(self):
        return self.ptr.shape_info_num
    @property
    def hk_shape_info_num(self):
        return self.ptr.hk_shape_info_num
    @property
    def hk_constraint_data_num(self):
        return self.ptr.hk_constraint_data_num
    @property
    def vbuff_info_num(self):
        return self.ptr.vbuff_info_num
    @property
    def ibuff_info_num(self):
        return self.ptr.ibuff_info_num
    @property
    def texture_info_num(self):
        return self.ptr.texture_info_num
    @property
    def animation_info_num(self):
        return self.ptr.animation_info_num
    @property
    def hk_constraint_info_num(self):
        return self.ptr.hk_constraint_info_num
    @property
    def effect_info_num(self):
        return self.ptr.effect_info_num
    @property
    def pfield_info_num(self):
        return self.ptr.pfield_info_num
    @property
    def gfx_block_info_num(self):
        return self.ptr.gfx_block_info_num
    @property
    def animation_block_info_num(self):
        return self.ptr.animation_block_info_num
    @property
    def foliage_info_num(self):
        return self.ptr.foliage_info_num
    @property
    def radiosity_vals_info_num(self):
        return self.ptr.radiosity_vals_info_num
    @property
    def unk_66(self):
        return self.ptr.unk_66
    @property
    def obja_offset(self):
        return self.ptr.obja_offset
    @property
    def obj0_offset(self):
        return self.ptr.obj0_offset
    @property
    def model_info_offset(self):
        return self.ptr.model_info_offset
    @property
    def buffer_info_offset(self):
        return self.ptr.buffer_info_offset
    @property
    def mat1_offset(self):
        return self.ptr.mat1_offset
    @property
    def mat2_offset(self):
        return self.ptr.mat2_offset
    @property
    def mat3_offset(self):
        return self.ptr.mat3_offset
    @property
    def mat4_offset(self):
        return self.ptr.mat4_offset
    @property
    def mat_extra_offset(self):
        return self.ptr.mat_extra_offset
    @property
    def unk_76(self):
        return self.ptr.unk_76
    @property
    def shape_info_offset(self):
        return self.ptr.shape_info_offset
    @property
    def hk_shape_info_offset(self):
        return self.ptr.hk_shape_info_offset
    @property
    def hk_constraint_data_offset(self):
        return self.ptr.hk_constraint_data_offset
    @property
    def vbuff_info_offset(self):
        return self.ptr.vbuff_info_offset
    @property
    def ibuff_info_offset(self):
        return self.ptr.ibuff_info_offset
    @property
    def texture_info_offset(self):
        return self.ptr.texture_info_offset
    @property
    def animation_info_offset(self):
        return self.ptr.animation_info_offset
    @property
    def hk_constraint_info_offset(self):
        return self.ptr.hk_constraint_info_offset
    @property
    def effect_info_offset(self):
        return self.ptr.effect_info_offset
    @property
    def pfield_info_offset(self):
        return self.ptr.pfield_info_offset
    @property
    def gfx_block_info_offset(self):
        return self.ptr.gfx_block_info_offset
    @property
    def animation_block_info_offset(self):
        return self.ptr.animation_block_info_offset
    @property
    def foliage_info_offset(self):
        return self.ptr.foliage_info_offset
    @property
    def radiosity_vals_info_offset(self):
        return self.ptr.radiosity_vals_info_offset
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def unk_114(self):
        return self.ptr.unk_114
    @property
    def unk_115(self):
        return self.ptr.unk_115
    @property
    def block2_offsets_num(self):
        return self.ptr.block2_offsets_num
    @property
    def block2_offsets_offset(self):
        return self.ptr.block2_offsets_offset

cdef class ref_slice_ObjAPs3:
    cdef lotrc_rs.ref_slice_ObjAPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ObjAPs3()
        val.ptr = lotrc_rs.ref_slice_ObjAPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_ObjAPs3_len(self.ptr)

cdef class ref_slice_Obj0Ps3:
    cdef lotrc_rs.ref_slice_Obj0Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj0Ps3()
        val.ptr = lotrc_rs.ref_slice_Obj0Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Obj0Ps3_len(self.ptr)

cdef class ref_slice_ModelInfoPs3:
    cdef lotrc_rs.ref_slice_ModelInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ModelInfoPs3()
        val.ptr = lotrc_rs.ref_slice_ModelInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_ModelInfoPs3_len(self.ptr)

cdef class ref_slice_BufferInfoPs3:
    cdef lotrc_rs.ref_slice_BufferInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BufferInfoPs3()
        val.ptr = lotrc_rs.ref_slice_BufferInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BufferInfoPs3_len(self.ptr)

cdef class ref_slice_Mat1Ps3:
    cdef lotrc_rs.ref_slice_Mat1Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat1Ps3()
        val.ptr = lotrc_rs.ref_slice_Mat1Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat1Ps3_len(self.ptr)

cdef class ref_slice_Mat2Ps3:
    cdef lotrc_rs.ref_slice_Mat2Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat2Ps3()
        val.ptr = lotrc_rs.ref_slice_Mat2Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat2Ps3_len(self.ptr)

cdef class ref_slice_Mat3Ps3:
    cdef lotrc_rs.ref_slice_Mat3Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat3Ps3()
        val.ptr = lotrc_rs.ref_slice_Mat3Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat3Ps3_len(self.ptr)

cdef class ref_slice_Mat4Ps3:
    cdef lotrc_rs.ref_slice_Mat4Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat4Ps3()
        val.ptr = lotrc_rs.ref_slice_Mat4Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Mat4Ps3_len(self.ptr)

cdef class ref_slice_MatExtraPs3:
    cdef lotrc_rs.ref_slice_MatExtraPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = MatExtraPs3()
        val.ptr = lotrc_rs.ref_slice_MatExtraPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_MatExtraPs3_len(self.ptr)

cdef class ref_slice_ShapeInfoPs3:
    cdef lotrc_rs.ref_slice_ShapeInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ShapeInfoPs3()
        val.ptr = lotrc_rs.ref_slice_ShapeInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_ShapeInfoPs3_len(self.ptr)

cdef class ref_slice_HkShapeInfoPs3:
    cdef lotrc_rs.ref_slice_HkShapeInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkShapeInfoPs3()
        val.ptr = lotrc_rs.ref_slice_HkShapeInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_HkShapeInfoPs3_len(self.ptr)

cdef class ref_slice_HkConstraintDataPs3:
    cdef lotrc_rs.ref_slice_HkConstraintDataPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintDataPs3()
        val.ptr = lotrc_rs.ref_slice_HkConstraintDataPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_HkConstraintDataPs3_len(self.ptr)

cdef class ref_slice_VBuffInfoPs3:
    cdef lotrc_rs.ref_slice_VBuffInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = VBuffInfoPs3()
        val.ptr = lotrc_rs.ref_slice_VBuffInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_VBuffInfoPs3_len(self.ptr)

cdef class ref_slice_IBuffInfoPs3:
    cdef lotrc_rs.ref_slice_IBuffInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = IBuffInfoPs3()
        val.ptr = lotrc_rs.ref_slice_IBuffInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_IBuffInfoPs3_len(self.ptr)

cdef class ref_slice_TextureInfoPs3:
    cdef lotrc_rs.ref_slice_TextureInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TextureInfoPs3()
        val.ptr = lotrc_rs.ref_slice_TextureInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_TextureInfoPs3_len(self.ptr)

cdef class ref_slice_AnimationInfoPs3:
    cdef lotrc_rs.ref_slice_AnimationInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationInfoPs3()
        val.ptr = lotrc_rs.ref_slice_AnimationInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AnimationInfoPs3_len(self.ptr)

cdef class ref_slice_HkConstraintInfoPs3:
    cdef lotrc_rs.ref_slice_HkConstraintInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintInfoPs3()
        val.ptr = lotrc_rs.ref_slice_HkConstraintInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_HkConstraintInfoPs3_len(self.ptr)

cdef class ref_slice_EffectInfoPs3:
    cdef lotrc_rs.ref_slice_EffectInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = EffectInfoPs3()
        val.ptr = lotrc_rs.ref_slice_EffectInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_EffectInfoPs3_len(self.ptr)

cdef class ref_slice_PFieldInfoPs3:
    cdef lotrc_rs.ref_slice_PFieldInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = PFieldInfoPs3()
        val.ptr = lotrc_rs.ref_slice_PFieldInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_PFieldInfoPs3_len(self.ptr)

cdef class ref_slice_GFXBlockInfoPs3:
    cdef lotrc_rs.ref_slice_GFXBlockInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = GFXBlockInfoPs3()
        val.ptr = lotrc_rs.ref_slice_GFXBlockInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_GFXBlockInfoPs3_len(self.ptr)

cdef class ref_slice_AnimationBlockInfoPs3:
    cdef lotrc_rs.ref_slice_AnimationBlockInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationBlockInfoPs3()
        val.ptr = lotrc_rs.ref_slice_AnimationBlockInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AnimationBlockInfoPs3_len(self.ptr)

cdef class ref_slice_FoliageInfoPs3:
    cdef lotrc_rs.ref_slice_FoliageInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = FoliageInfoPs3()
        val.ptr = lotrc_rs.ref_slice_FoliageInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_FoliageInfoPs3_len(self.ptr)

cdef class ref_slice_RadiosityValsInfoPs3:
    cdef lotrc_rs.ref_slice_RadiosityValsInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RadiosityValsInfoPs3()
        val.ptr = lotrc_rs.ref_slice_RadiosityValsInfoPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RadiosityValsInfoPs3_len(self.ptr)

cdef class InfosRefPs3:
    cdef lotrc_rs.InfosRefPs3* ptr
    @property
    def objas(self):
        val = ref_slice_ObjAPs3()
        val.ptr = &self.ptr.objas
        return val
    @property
    def obj0s(self):
        val = ref_slice_Obj0Ps3()
        val.ptr = &self.ptr.obj0s
        return val
    @property
    def models(self):
        val = ref_slice_ModelInfoPs3()
        val.ptr = &self.ptr.models
        return val
    @property
    def buffers(self):
        val = ref_slice_BufferInfoPs3()
        val.ptr = &self.ptr.buffers
        return val
    @property
    def mat1s(self):
        val = ref_slice_Mat1Ps3()
        val.ptr = &self.ptr.mat1s
        return val
    @property
    def mat2s(self):
        val = ref_slice_Mat2Ps3()
        val.ptr = &self.ptr.mat2s
        return val
    @property
    def mat3s(self):
        val = ref_slice_Mat3Ps3()
        val.ptr = &self.ptr.mat3s
        return val
    @property
    def mat4s(self):
        val = ref_slice_Mat4Ps3()
        val.ptr = &self.ptr.mat4s
        return val
    @property
    def mat_extras(self):
        val = ref_slice_MatExtraPs3()
        val.ptr = &self.ptr.mat_extras
        return val
    @property
    def shapes(self):
        val = ref_slice_ShapeInfoPs3()
        val.ptr = &self.ptr.shapes
        return val
    @property
    def hk_shapes(self):
        val = ref_slice_HkShapeInfoPs3()
        val.ptr = &self.ptr.hk_shapes
        return val
    @property
    def hk_constraint_datas(self):
        val = ref_slice_HkConstraintDataPs3()
        val.ptr = &self.ptr.hk_constraint_datas
        return val
    @property
    def vbuffs(self):
        val = ref_slice_VBuffInfoPs3()
        val.ptr = &self.ptr.vbuffs
        return val
    @property
    def ibuffs(self):
        val = ref_slice_IBuffInfoPs3()
        val.ptr = &self.ptr.ibuffs
        return val
    @property
    def textures(self):
        val = ref_slice_TextureInfoPs3()
        val.ptr = &self.ptr.textures
        return val
    @property
    def animations(self):
        val = ref_slice_AnimationInfoPs3()
        val.ptr = &self.ptr.animations
        return val
    @property
    def hk_constraints(self):
        val = ref_slice_HkConstraintInfoPs3()
        val.ptr = &self.ptr.hk_constraints
        return val
    @property
    def effects(self):
        val = ref_slice_EffectInfoPs3()
        val.ptr = &self.ptr.effects
        return val
    @property
    def pfields(self):
        val = ref_slice_PFieldInfoPs3()
        val.ptr = &self.ptr.pfields
        return val
    @property
    def gfxs(self):
        val = ref_slice_GFXBlockInfoPs3()
        val.ptr = &self.ptr.gfxs
        return val
    @property
    def animation_blocks(self):
        val = ref_slice_AnimationBlockInfoPs3()
        val.ptr = &self.ptr.animation_blocks
        return val
    @property
    def foliages(self):
        val = ref_slice_FoliageInfoPs3()
        val.ptr = &self.ptr.foliages
        return val
    @property
    def radiosity_vals(self):
        val = ref_slice_RadiosityValsInfoPs3()
        val.ptr = &self.ptr.radiosity_vals
        return val

cdef class IndexMap_u32__TextureRefPs3:
    cdef lotrc_rs.IndexMap_u32__TextureRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = TextureRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__TextureRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__TextureRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__TextureRefPs3_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__ModelRefPs3:
    cdef lotrc_rs.IndexMap_u32__ModelRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = ModelRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__ModelRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__ModelRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__ModelRefPs3_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__EffectRefPs3:
    cdef lotrc_rs.IndexMap_u32__EffectRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = EffectRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__EffectRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__EffectRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__EffectRefPs3_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__slice_FoliageRefPs3:
    cdef lotrc_rs.IndexMap_u32__slice_FoliageRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = slice_FoliageRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__slice_FoliageRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__slice_FoliageRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__slice_FoliageRefPs3_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__RadiosityValsRefPs3:
    cdef lotrc_rs.IndexMap_u32__RadiosityValsRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = RadiosityValsRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__RadiosityValsRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__RadiosityValsRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__RadiosityValsRefPs3_keys(self.ptr, keys.ptr)

cdef class RadiosityRefPs3:
    cdef lotrc_rs.RadiosityRefPs3* ptr
    @property
    def data(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data
        return val
    @property
    def vals(self):
        val = IndexMap_u32__RadiosityValsRefPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def usage(self):
        return self.ptr.usage

cdef class ObjsRefPs3:
    cdef lotrc_rs.ObjsRefPs3* ptr
    @property
    def textures(self):
        val = IndexMap_u32__TextureRefPs3()
        val.ptr = &self.ptr.textures
        return val
    @property
    def models(self):
        val = IndexMap_u32__ModelRefPs3()
        val.ptr = &self.ptr.models
        return val
    @property
    def effects(self):
        val = IndexMap_u32__EffectRefPs3()
        val.ptr = &self.ptr.effects
        return val
    @property
    def foliages(self):
        val = IndexMap_u32__slice_FoliageRefPs3()
        val.ptr = &self.ptr.foliages
        return val
    @property
    def gfxs(self):
        val = IndexMap_u32__ref_slice_u8()
        val.ptr = &self.ptr.gfxs
        return val
    @property
    def radiosity(self):
        val = RadiosityRefPs3()
        val.ptr = &self.ptr.radiosity
        return val

cdef class SubBlocksHeaderPs3:
    cdef lotrc_rs.SubBlocksHeaderPs3* ptr
    @property
    def z0(self):
        return self.ptr.z0
    @property
    def block_num(self):
        return self.ptr.block_num
    @property
    def z2(self):
        return self.ptr.z2
    @property
    def z3(self):
        return self.ptr.z3

cdef class ref_slice_SubBlocksBlockHeaderPs3:
    cdef lotrc_rs.ref_slice_SubBlocksBlockHeaderPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SubBlocksBlockHeaderPs3()
        val.ptr = lotrc_rs.ref_slice_SubBlocksBlockHeaderPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SubBlocksBlockHeaderPs3_len(self.ptr)

cdef class SubBlocksInfoRefPs3:
    cdef lotrc_rs.SubBlocksInfoRefPs3* ptr
    @property
    def header(self):
        val = SubBlocksHeaderPs3()
        val.ptr = self.ptr.header
        return val
    @property
    def block_headers(self):
        val = ref_slice_SubBlocksBlockHeaderPs3()
        val.ptr = &self.ptr.block_headers
        return val

cdef class IndexMap_u32__DataRefPs3:
    cdef lotrc_rs.IndexMap_u32__DataRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = DataRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__DataRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__DataRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__DataRefPs3_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__LuaRefPs3:
    cdef lotrc_rs.IndexMap_u32__LuaRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = DataRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__LuaRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__LuaRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__LuaRefPs3_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__SSARefPs3:
    cdef lotrc_rs.IndexMap_u32__SSARefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = SSARefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__SSARefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__SSARefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__SSARefPs3_keys(self.ptr, keys.ptr)

cdef class Option_AtlasUVRefPs3:
    cdef lotrc_rs.Option_AtlasUVRefPs3* ptr
    def get(self):
        val = AtlasUVRefPs3()
        val.ptr = lotrc_rs.Option_AtlasUVRefPs3_get(self.ptr)
        return val

cdef class GameObjsHeaderPs3:
    cdef lotrc_rs.GameObjsHeaderPs3* ptr
    @property
    def const_(self):
        return self.ptr.const_
    @property
    def types_num(self):
        return self.ptr.types_num
    @property
    def types_offset(self):
        return self.ptr.types_offset
    @property
    def obj_num(self):
        return self.ptr.obj_num
    @property
    def obj_offset(self):
        return self.ptr.obj_offset
    @property
    def z5(self):
        return self.ptr.z5
    @property
    def z6(self):
        return self.ptr.z6
    @property
    def z7(self):
        return self.ptr.z7

cdef class IndexMap_u32__TypeRefPs3:
    cdef lotrc_rs.IndexMap_u32__TypeRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = TypeRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__TypeRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__TypeRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__TypeRefPs3_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32__ObjRefPs3:
    cdef lotrc_rs.IndexMap_u32__ObjRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = ObjRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__ObjRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__ObjRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__ObjRefPs3_keys(self.ptr, keys.ptr)

cdef class GameObjsRefPs3:
    cdef lotrc_rs.GameObjsRefPs3* ptr
    @property
    def header(self):
        val = GameObjsHeaderPs3()
        val.ptr = self.ptr.header
        return val
    @property
    def types(self):
        val = IndexMap_u32__TypeRefPs3()
        val.ptr = &self.ptr.types
        return val
    @property
    def objs(self):
        val = IndexMap_u32__ObjRefPs3()
        val.ptr = &self.ptr.objs
        return val

cdef class SubBlocks1RefPs3:
    cdef lotrc_rs.SubBlocks1RefPs3* ptr
    @property
    def info(self):
        val = SubBlocksInfoRefPs3()
        val.ptr = &self.ptr.info
        return val
    @property
    def files(self):
        val = IndexMap_u32__DataRefPs3()
        val.ptr = &self.ptr.files
        return val
    @property
    def lua(self):
        val = IndexMap_u32__LuaRefPs3()
        val.ptr = &self.ptr.lua
        return val
    @property
    def subtitles(self):
        val = IndexMap_u32__SSARefPs3()
        val.ptr = &self.ptr.subtitles
        return val
    @property
    def atlas1(self):
        val = Option_AtlasUVRefPs3()
        val.ptr = &self.ptr.atlas1
        return val
    @property
    def atlas2(self):
        val = Option_AtlasUVRefPs3()
        val.ptr = &self.ptr.atlas2
        return val
    @property
    def level(self):
        val = GameObjsRefPs3()
        val.ptr = &self.ptr.level
        return val

cdef class StringKeysHeaderPs3:
    cdef lotrc_rs.StringKeysHeaderPs3* ptr
    @property
    def num_a(self):
        return self.ptr.num_a
    @property
    def num_b(self):
        return self.ptr.num_b
    @property
    def z2(self):
        return self.ptr.z2
    @property
    def z3(self):
        return self.ptr.z3
    @property
    def z4(self):
        return self.ptr.z4
    @property
    def z5(self):
        return self.ptr.z5

cdef class ref_slice_StringKeysValPs3:
    cdef lotrc_rs.ref_slice_StringKeysValPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = StringKeysValPs3()
        val.ptr = lotrc_rs.ref_slice_StringKeysValPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_StringKeysValPs3_len(self.ptr)

cdef class ref_slice_u32Ps3:
    cdef lotrc_rs.ref_slice_u32Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_u32Ps3_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_u32Ps3_len(self.ptr)

cdef class StringKeysRefPs3:
    cdef lotrc_rs.StringKeysRefPs3* ptr
    @property
    def header(self):
        val = StringKeysHeaderPs3()
        val.ptr = self.ptr.header
        return val
    @property
    def vals(self):
        val = ref_slice_StringKeysValPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def pad(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.pad
        return val

cdef class Block1RefPs3:
    cdef lotrc_rs.Block1RefPs3* ptr
    @property
    def infos(self):
        val = InfosRefPs3()
        val.ptr = &self.ptr.infos
        return val
    @property
    def objs(self):
        val = ObjsRefPs3()
        val.ptr = &self.ptr.objs
        return val
    @property
    def sub_blocks(self):
        val = SubBlocks1RefPs3()
        val.ptr = &self.ptr.sub_blocks
        return val
    @property
    def string_keys(self):
        val = StringKeysRefPs3()
        val.ptr = &self.ptr.string_keys
        return val

cdef class Option_SprayRefPs3:
    cdef lotrc_rs.Option_SprayRefPs3* ptr
    def get(self):
        val = SprayRefPs3()
        val.ptr = lotrc_rs.Option_SprayRefPs3_get(self.ptr)
        return val

cdef class Option_CrowdRefPs3:
    cdef lotrc_rs.Option_CrowdRefPs3* ptr
    def get(self):
        val = CrowdRefPs3()
        val.ptr = lotrc_rs.Option_CrowdRefPs3_get(self.ptr)
        return val

cdef class Option_PFieldsRefPs3:
    cdef lotrc_rs.Option_PFieldsRefPs3* ptr
    def get(self):
        val = DataRefPs3()
        val.ptr = lotrc_rs.Option_PFieldsRefPs3_get(self.ptr)
        return val

cdef class IndexMap_u32__LangStringsRefPs3:
    cdef lotrc_rs.IndexMap_u32__LangStringsRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = LangStringsRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__LangStringsRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__LangStringsRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__LangStringsRefPs3_keys(self.ptr, keys.ptr)

cdef class SubBlocks2RefPs3:
    cdef lotrc_rs.SubBlocks2RefPs3* ptr
    @property
    def info(self):
        val = SubBlocksInfoRefPs3()
        val.ptr = &self.ptr.info
        return val
    @property
    def spray(self):
        val = Option_SprayRefPs3()
        val.ptr = &self.ptr.spray
        return val
    @property
    def crowd(self):
        val = Option_CrowdRefPs3()
        val.ptr = &self.ptr.crowd
        return val
    @property
    def pfields(self):
        val = Option_PFieldsRefPs3()
        val.ptr = &self.ptr.pfields
        return val
    @property
    def langs(self):
        val = IndexMap_u32__LangStringsRefPs3()
        val.ptr = &self.ptr.langs
        return val
    @property
    def files(self):
        val = IndexMap_u32__DataRefPs3()
        val.ptr = &self.ptr.files
        return val

cdef class Block2RefPs3:
    cdef lotrc_rs.Block2RefPs3* ptr
    @property
    def sub_blocks(self):
        val = SubBlocks2RefPs3()
        val.ptr = &self.ptr.sub_blocks
        return val
    @property
    def offsets(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.offsets
        return val

cdef class IndexMap_u32__AnimationRefPs3:
    cdef lotrc_rs.IndexMap_u32__AnimationRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = AnimationRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__AnimationRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__AnimationRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__AnimationRefPs3_keys(self.ptr, keys.ptr)

cdef class AnimationsRefPs3:
    cdef lotrc_rs.AnimationsRefPs3* ptr
    def dump(self, DumpInfosPs3 infos):
        val = OwnedVecCompressedData()
        val.ptr = lotrc_rs.AnimationsRefPs3_dump(self.ptr, infos.ptr)
        return val
    @property
    def animations(self):
        val = IndexMap_u32__AnimationRefPs3()
        val.ptr = &self.ptr.animations
        return val
    @property
    def block_infos(self):
        val = ref_slice_AnimationBlockInfoPs3()
        val.ptr = &self.ptr.block_infos
        return val

cdef class ref_slice_BlockAValPs3:
    cdef lotrc_rs.ref_slice_BlockAValPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BlockAValPs3()
        val.ptr = lotrc_rs.ref_slice_BlockAValPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BlockAValPs3_len(self.ptr)

cdef class PakRefPs3:
    cdef lotrc_rs.PakRefPs3* ptr
    @property
    def header(self):
        val = PakHeaderPs3()
        val.ptr = self.ptr.header
        return val
    @property
    def strings(self):
        val = slice_string()
        val.ptr = &self.ptr.strings
        return val
    @property
    def block1(self):
        val = Block1RefPs3()
        val.ptr = &self.ptr.block1
        return val
    @property
    def block2(self):
        val = Block2RefPs3()
        val.ptr = &self.ptr.block2
        return val
    @property
    def animations(self):
        val = AnimationsRefPs3()
        val.ptr = &self.ptr.animations
        return val
    @property
    def vals_a(self):
        val = ref_slice_BlockAValPs3()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def animation_data(self):
        val = ref_slice_CompressedDataRef()
        val.ptr = &self.ptr.animation_data
        return val

cdef class BinHeaderPs3:
    cdef lotrc_rs.BinHeaderPs3* ptr
    @property
    def constx06(self):
        return self.ptr.constx06
    @property
    def version(self):
        return self.ptr.version
    @property
    def strings_offset(self):
        return self.ptr.strings_offset
    @property
    def strings_size(self):
        return self.ptr.strings_size
    @property
    def strings_num(self):
        return self.ptr.strings_num
    @property
    def asset_handle_num(self):
        return self.ptr.asset_handle_num
    @property
    def asset_handle_offset(self):
        return self.ptr.asset_handle_offset
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def vdata_num(self):
        return self.ptr.vdata_num
    @property
    def vdata_num_alt(self):
        return self.ptr.vdata_num_alt
    @property
    def texdata_num(self):
        return self.ptr.texdata_num
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def unk_33(self):
        return self.ptr.unk_33
    @property
    def unk_34(self):
        return self.ptr.unk_34
    @property
    def unk_35(self):
        return self.ptr.unk_35
    @property
    def unk_36(self):
        return self.ptr.unk_36
    @property
    def unk_37(self):
        return self.ptr.unk_37
    @property
    def unk_38(self):
        return self.ptr.unk_38
    @property
    def unk_39(self):
        return self.ptr.unk_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42

cdef class ref_slice_AssetHandlePs3:
    cdef lotrc_rs.ref_slice_AssetHandlePs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AssetHandlePs3()
        val.ptr = lotrc_rs.ref_slice_AssetHandlePs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AssetHandlePs3_len(self.ptr)

cdef class BinRefPs3:
    cdef lotrc_rs.BinRefPs3* ptr
    @property
    def header(self):
        val = BinHeaderPs3()
        val.ptr = self.ptr.header
        return val
    @property
    def strings(self):
        val = slice_string()
        val.ptr = &self.ptr.strings
        return val
    @property
    def asset_handles(self):
        val = ref_slice_AssetHandlePs3()
        val.ptr = &self.ptr.asset_handles
        return val
    @property
    def model_data(self):
        val = IndexMap_u32_______CompressedDataRef()
        val.ptr = &self.ptr.model_data
        return val
    @property
    def texture_data(self):
        val = IndexMap_u32_______CompressedDataRef()
        val.ptr = &self.ptr.texture_data
        return val

cdef class LevelRefPs3:
    cdef lotrc_rs.LevelRefPs3* ptr
    def dump(self, lotrc_rs.uint32_t compression):
        val = OwnedLevelData()
        val.ptr = lotrc_rs.LevelRefPs3_dump(self.ptr, compression)
        return val
    @property
    def pak(self):
        val = PakRefPs3()
        val.ptr = &self.ptr.pak
        return val
    @property
    def bin(self):
        val = BinRefPs3()
        val.ptr = &self.ptr.bin
        return val

cdef class InfoCounts:
    cdef lotrc_rs.InfoCounts* ptr
    @staticmethod
    def new():
        val = OwnedInfoCounts()
        val.ptr = lotrc_rs.InfoCounts_new()
        return val
    def size_pc(self):
        return lotrc_rs.InfoCounts_size_pc(self.ptr)
    def size_xbox(self):
        return lotrc_rs.InfoCounts_size_xbox(self.ptr)
    def size_ps3(self):
        return lotrc_rs.InfoCounts_size_ps3(self.ptr)
    @property
    def objas(self):
        return self.ptr.objas
    @property
    def obj0s(self):
        return self.ptr.obj0s
    @property
    def models(self):
        return self.ptr.models
    @property
    def buffers(self):
        return self.ptr.buffers
    @property
    def mat1s(self):
        return self.ptr.mat1s
    @property
    def mat2s(self):
        return self.ptr.mat2s
    @property
    def mat3s(self):
        return self.ptr.mat3s
    @property
    def mat4s(self):
        return self.ptr.mat4s
    @property
    def mat_extras(self):
        return self.ptr.mat_extras
    @property
    def shapes(self):
        return self.ptr.shapes
    @property
    def hk_shapes(self):
        return self.ptr.hk_shapes
    @property
    def hk_constraint_datas(self):
        return self.ptr.hk_constraint_datas
    @property
    def vbuffs(self):
        return self.ptr.vbuffs
    @property
    def ibuffs(self):
        return self.ptr.ibuffs
    @property
    def textures(self):
        return self.ptr.textures
    @property
    def animations(self):
        return self.ptr.animations
    @property
    def hk_constraints(self):
        return self.ptr.hk_constraints
    @property
    def effects(self):
        return self.ptr.effects
    @property
    def pfields(self):
        return self.ptr.pfields
    @property
    def gfxs(self):
        return self.ptr.gfxs
    @property
    def animation_blocks(self):
        return self.ptr.animation_blocks
    @property
    def foliages(self):
        return self.ptr.foliages
    @property
    def radiosity_vals(self):
        return self.ptr.radiosity_vals
    @property
    def offsets(self):
        return self.ptr.offsets

cdef class mut_slice_ObjAPc:
    cdef lotrc_rs.mut_slice_ObjAPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ObjAPc()
        val.ptr = lotrc_rs.mut_slice_ObjAPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_ObjAPc slice):
        return lotrc_rs.mut_slice_ObjAPc_len(slice.ptr)

cdef class DumpInfo_ObjAPc:
    cdef lotrc_rs.DumpInfo_ObjAPc* ptr
    @property
    def vals(self):
        val = mut_slice_ObjAPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Obj0Pc:
    cdef lotrc_rs.mut_slice_Obj0Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj0Pc()
        val.ptr = lotrc_rs.mut_slice_Obj0Pc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Obj0Pc slice):
        return lotrc_rs.mut_slice_Obj0Pc_len(slice.ptr)

cdef class DumpInfo_Obj0Pc:
    cdef lotrc_rs.DumpInfo_Obj0Pc* ptr
    @property
    def vals(self):
        val = mut_slice_Obj0Pc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_ModelInfoPc:
    cdef lotrc_rs.mut_slice_ModelInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ModelInfoPc()
        val.ptr = lotrc_rs.mut_slice_ModelInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_ModelInfoPc slice):
        return lotrc_rs.mut_slice_ModelInfoPc_len(slice.ptr)

cdef class DumpInfo_ModelInfoPc:
    cdef lotrc_rs.DumpInfo_ModelInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_ModelInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_BufferInfoPc:
    cdef lotrc_rs.mut_slice_BufferInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BufferInfoPc()
        val.ptr = lotrc_rs.mut_slice_BufferInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_BufferInfoPc slice):
        return lotrc_rs.mut_slice_BufferInfoPc_len(slice.ptr)

cdef class DumpInfo_BufferInfoPc:
    cdef lotrc_rs.DumpInfo_BufferInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_BufferInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat1Pc:
    cdef lotrc_rs.mut_slice_Mat1Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat1Pc()
        val.ptr = lotrc_rs.mut_slice_Mat1Pc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat1Pc slice):
        return lotrc_rs.mut_slice_Mat1Pc_len(slice.ptr)

cdef class DumpInfo_Mat1Pc:
    cdef lotrc_rs.DumpInfo_Mat1Pc* ptr
    @property
    def vals(self):
        val = mut_slice_Mat1Pc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat2Pc:
    cdef lotrc_rs.mut_slice_Mat2Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat2Pc()
        val.ptr = lotrc_rs.mut_slice_Mat2Pc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat2Pc slice):
        return lotrc_rs.mut_slice_Mat2Pc_len(slice.ptr)

cdef class DumpInfo_Mat2Pc:
    cdef lotrc_rs.DumpInfo_Mat2Pc* ptr
    @property
    def vals(self):
        val = mut_slice_Mat2Pc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat3Pc:
    cdef lotrc_rs.mut_slice_Mat3Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat3Pc()
        val.ptr = lotrc_rs.mut_slice_Mat3Pc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat3Pc slice):
        return lotrc_rs.mut_slice_Mat3Pc_len(slice.ptr)

cdef class DumpInfo_Mat3Pc:
    cdef lotrc_rs.DumpInfo_Mat3Pc* ptr
    @property
    def vals(self):
        val = mut_slice_Mat3Pc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat4Pc:
    cdef lotrc_rs.mut_slice_Mat4Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat4Pc()
        val.ptr = lotrc_rs.mut_slice_Mat4Pc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat4Pc slice):
        return lotrc_rs.mut_slice_Mat4Pc_len(slice.ptr)

cdef class DumpInfo_Mat4Pc:
    cdef lotrc_rs.DumpInfo_Mat4Pc* ptr
    @property
    def vals(self):
        val = mut_slice_Mat4Pc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_MatExtraPc:
    cdef lotrc_rs.mut_slice_MatExtraPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = MatExtraPc()
        val.ptr = lotrc_rs.mut_slice_MatExtraPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_MatExtraPc slice):
        return lotrc_rs.mut_slice_MatExtraPc_len(slice.ptr)

cdef class DumpInfo_MatExtraPc:
    cdef lotrc_rs.DumpInfo_MatExtraPc* ptr
    @property
    def vals(self):
        val = mut_slice_MatExtraPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_ShapeInfoPc:
    cdef lotrc_rs.mut_slice_ShapeInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ShapeInfoPc()
        val.ptr = lotrc_rs.mut_slice_ShapeInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_ShapeInfoPc slice):
        return lotrc_rs.mut_slice_ShapeInfoPc_len(slice.ptr)

cdef class DumpInfo_ShapeInfoPc:
    cdef lotrc_rs.DumpInfo_ShapeInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_ShapeInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_HkShapeInfoPc:
    cdef lotrc_rs.mut_slice_HkShapeInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkShapeInfoPc()
        val.ptr = lotrc_rs.mut_slice_HkShapeInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_HkShapeInfoPc slice):
        return lotrc_rs.mut_slice_HkShapeInfoPc_len(slice.ptr)

cdef class DumpInfo_HkShapeInfoPc:
    cdef lotrc_rs.DumpInfo_HkShapeInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_HkShapeInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_HkConstraintDataPc:
    cdef lotrc_rs.mut_slice_HkConstraintDataPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintDataPc()
        val.ptr = lotrc_rs.mut_slice_HkConstraintDataPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_HkConstraintDataPc slice):
        return lotrc_rs.mut_slice_HkConstraintDataPc_len(slice.ptr)

cdef class DumpInfo_HkConstraintDataPc:
    cdef lotrc_rs.DumpInfo_HkConstraintDataPc* ptr
    @property
    def vals(self):
        val = mut_slice_HkConstraintDataPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_VBuffInfoPc:
    cdef lotrc_rs.mut_slice_VBuffInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = VBuffInfoPc()
        val.ptr = lotrc_rs.mut_slice_VBuffInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_VBuffInfoPc slice):
        return lotrc_rs.mut_slice_VBuffInfoPc_len(slice.ptr)

cdef class DumpInfo_VBuffInfoPc:
    cdef lotrc_rs.DumpInfo_VBuffInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_VBuffInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_IBuffInfoPc:
    cdef lotrc_rs.mut_slice_IBuffInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = IBuffInfoPc()
        val.ptr = lotrc_rs.mut_slice_IBuffInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_IBuffInfoPc slice):
        return lotrc_rs.mut_slice_IBuffInfoPc_len(slice.ptr)

cdef class DumpInfo_IBuffInfoPc:
    cdef lotrc_rs.DumpInfo_IBuffInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_IBuffInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_TextureInfoPc:
    cdef lotrc_rs.mut_slice_TextureInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TextureInfoPc()
        val.ptr = lotrc_rs.mut_slice_TextureInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_TextureInfoPc slice):
        return lotrc_rs.mut_slice_TextureInfoPc_len(slice.ptr)

cdef class DumpInfo_TextureInfoPc:
    cdef lotrc_rs.DumpInfo_TextureInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_TextureInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_AnimationInfoPc:
    cdef lotrc_rs.mut_slice_AnimationInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationInfoPc()
        val.ptr = lotrc_rs.mut_slice_AnimationInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_AnimationInfoPc slice):
        return lotrc_rs.mut_slice_AnimationInfoPc_len(slice.ptr)

cdef class DumpInfo_AnimationInfoPc:
    cdef lotrc_rs.DumpInfo_AnimationInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_AnimationInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_HkConstraintInfoPc:
    cdef lotrc_rs.mut_slice_HkConstraintInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintInfoPc()
        val.ptr = lotrc_rs.mut_slice_HkConstraintInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_HkConstraintInfoPc slice):
        return lotrc_rs.mut_slice_HkConstraintInfoPc_len(slice.ptr)

cdef class DumpInfo_HkConstraintInfoPc:
    cdef lotrc_rs.DumpInfo_HkConstraintInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_HkConstraintInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_EffectInfoPc:
    cdef lotrc_rs.mut_slice_EffectInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = EffectInfoPc()
        val.ptr = lotrc_rs.mut_slice_EffectInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_EffectInfoPc slice):
        return lotrc_rs.mut_slice_EffectInfoPc_len(slice.ptr)

cdef class DumpInfo_EffectInfoPc:
    cdef lotrc_rs.DumpInfo_EffectInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_EffectInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_PFieldInfoPc:
    cdef lotrc_rs.mut_slice_PFieldInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = PFieldInfoPc()
        val.ptr = lotrc_rs.mut_slice_PFieldInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_PFieldInfoPc slice):
        return lotrc_rs.mut_slice_PFieldInfoPc_len(slice.ptr)

cdef class DumpInfo_PFieldInfoPc:
    cdef lotrc_rs.DumpInfo_PFieldInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_PFieldInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_GFXBlockInfoPc:
    cdef lotrc_rs.mut_slice_GFXBlockInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = GFXBlockInfoPc()
        val.ptr = lotrc_rs.mut_slice_GFXBlockInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_GFXBlockInfoPc slice):
        return lotrc_rs.mut_slice_GFXBlockInfoPc_len(slice.ptr)

cdef class DumpInfo_GFXBlockInfoPc:
    cdef lotrc_rs.DumpInfo_GFXBlockInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_GFXBlockInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_AnimationBlockInfoPc:
    cdef lotrc_rs.mut_slice_AnimationBlockInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationBlockInfoPc()
        val.ptr = lotrc_rs.mut_slice_AnimationBlockInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_AnimationBlockInfoPc slice):
        return lotrc_rs.mut_slice_AnimationBlockInfoPc_len(slice.ptr)

cdef class DumpInfo_AnimationBlockInfoPc:
    cdef lotrc_rs.DumpInfo_AnimationBlockInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_AnimationBlockInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_FoliageInfoPc:
    cdef lotrc_rs.mut_slice_FoliageInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = FoliageInfoPc()
        val.ptr = lotrc_rs.mut_slice_FoliageInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_FoliageInfoPc slice):
        return lotrc_rs.mut_slice_FoliageInfoPc_len(slice.ptr)

cdef class DumpInfo_FoliageInfoPc:
    cdef lotrc_rs.DumpInfo_FoliageInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_FoliageInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_RadiosityValsInfoPc:
    cdef lotrc_rs.mut_slice_RadiosityValsInfoPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RadiosityValsInfoPc()
        val.ptr = lotrc_rs.mut_slice_RadiosityValsInfoPc_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_RadiosityValsInfoPc slice):
        return lotrc_rs.mut_slice_RadiosityValsInfoPc_len(slice.ptr)

cdef class DumpInfo_RadiosityValsInfoPc:
    cdef lotrc_rs.DumpInfo_RadiosityValsInfoPc* ptr
    @property
    def vals(self):
        val = mut_slice_RadiosityValsInfoPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_u32Pc:
    cdef lotrc_rs.mut_slice_u32Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.mut_slice_u32Pc_get(self.ptr, idx))
    @staticmethod
    def len(slice_u32Pc slice):
        return lotrc_rs.mut_slice_u32Pc_len(slice.ptr)

cdef class DumpInfo_u32Pc:
    cdef lotrc_rs.DumpInfo_u32Pc* ptr
    @property
    def vals(self):
        val = mut_slice_u32Pc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class Vec_DumpInfoDataPc:
    cdef lotrc_rs.Vec_DumpInfoDataPc* ptr

cdef class DumpInfosPc:
    cdef lotrc_rs.DumpInfosPc* ptr
    @staticmethod
    def from_data(DumpSlice dst, InfoCounts counts, mut_slice_u32Pc offsets):
        val = OwnedDumpInfosPc()
        val.ptr = lotrc_rs.DumpInfosPc_from_data(dst.ptr, counts.ptr, offsets.ptr)
        return val
    @property
    def objas(self):
        val = DumpInfo_ObjAPc()
        val.ptr = &self.ptr.objas
        return val
    @property
    def obj0s(self):
        val = DumpInfo_Obj0Pc()
        val.ptr = &self.ptr.obj0s
        return val
    @property
    def models(self):
        val = DumpInfo_ModelInfoPc()
        val.ptr = &self.ptr.models
        return val
    @property
    def buffers(self):
        val = DumpInfo_BufferInfoPc()
        val.ptr = &self.ptr.buffers
        return val
    @property
    def mat1s(self):
        val = DumpInfo_Mat1Pc()
        val.ptr = &self.ptr.mat1s
        return val
    @property
    def mat2s(self):
        val = DumpInfo_Mat2Pc()
        val.ptr = &self.ptr.mat2s
        return val
    @property
    def mat3s(self):
        val = DumpInfo_Mat3Pc()
        val.ptr = &self.ptr.mat3s
        return val
    @property
    def mat4s(self):
        val = DumpInfo_Mat4Pc()
        val.ptr = &self.ptr.mat4s
        return val
    @property
    def mat_extras(self):
        val = DumpInfo_MatExtraPc()
        val.ptr = &self.ptr.mat_extras
        return val
    @property
    def shapes(self):
        val = DumpInfo_ShapeInfoPc()
        val.ptr = &self.ptr.shapes
        return val
    @property
    def hk_shapes(self):
        val = DumpInfo_HkShapeInfoPc()
        val.ptr = &self.ptr.hk_shapes
        return val
    @property
    def hk_constraint_datas(self):
        val = DumpInfo_HkConstraintDataPc()
        val.ptr = &self.ptr.hk_constraint_datas
        return val
    @property
    def vbuffs(self):
        val = DumpInfo_VBuffInfoPc()
        val.ptr = &self.ptr.vbuffs
        return val
    @property
    def ibuffs(self):
        val = DumpInfo_IBuffInfoPc()
        val.ptr = &self.ptr.ibuffs
        return val
    @property
    def textures(self):
        val = DumpInfo_TextureInfoPc()
        val.ptr = &self.ptr.textures
        return val
    @property
    def animations(self):
        val = DumpInfo_AnimationInfoPc()
        val.ptr = &self.ptr.animations
        return val
    @property
    def hk_constraints(self):
        val = DumpInfo_HkConstraintInfoPc()
        val.ptr = &self.ptr.hk_constraints
        return val
    @property
    def effects(self):
        val = DumpInfo_EffectInfoPc()
        val.ptr = &self.ptr.effects
        return val
    @property
    def pfields(self):
        val = DumpInfo_PFieldInfoPc()
        val.ptr = &self.ptr.pfields
        return val
    @property
    def gfxs(self):
        val = DumpInfo_GFXBlockInfoPc()
        val.ptr = &self.ptr.gfxs
        return val
    @property
    def animation_blocks(self):
        val = DumpInfo_AnimationBlockInfoPc()
        val.ptr = &self.ptr.animation_blocks
        return val
    @property
    def foliages(self):
        val = DumpInfo_FoliageInfoPc()
        val.ptr = &self.ptr.foliages
        return val
    @property
    def radiosity_vals(self):
        val = DumpInfo_RadiosityValsInfoPc()
        val.ptr = &self.ptr.radiosity_vals
        return val
    @property
    def offsets(self):
        val = DumpInfo_u32Pc()
        val.ptr = &self.ptr.offsets
        return val
    @property
    def model_data(self):
        val = Vec_DumpInfoDataPc()
        val.ptr = &self.ptr.model_data
        return val
    @property
    def texture_data(self):
        val = Vec_DumpInfoDataPc()
        val.ptr = &self.ptr.texture_data
        return val

cdef class mut_slice_ObjAXbox:
    cdef lotrc_rs.mut_slice_ObjAXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ObjAXbox()
        val.ptr = lotrc_rs.mut_slice_ObjAXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_ObjAXbox slice):
        return lotrc_rs.mut_slice_ObjAXbox_len(slice.ptr)

cdef class DumpInfo_ObjAXbox:
    cdef lotrc_rs.DumpInfo_ObjAXbox* ptr
    @property
    def vals(self):
        val = mut_slice_ObjAXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Obj0Xbox:
    cdef lotrc_rs.mut_slice_Obj0Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj0Xbox()
        val.ptr = lotrc_rs.mut_slice_Obj0Xbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Obj0Xbox slice):
        return lotrc_rs.mut_slice_Obj0Xbox_len(slice.ptr)

cdef class DumpInfo_Obj0Xbox:
    cdef lotrc_rs.DumpInfo_Obj0Xbox* ptr
    @property
    def vals(self):
        val = mut_slice_Obj0Xbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_ModelInfoXbox:
    cdef lotrc_rs.mut_slice_ModelInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ModelInfoXbox()
        val.ptr = lotrc_rs.mut_slice_ModelInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_ModelInfoXbox slice):
        return lotrc_rs.mut_slice_ModelInfoXbox_len(slice.ptr)

cdef class DumpInfo_ModelInfoXbox:
    cdef lotrc_rs.DumpInfo_ModelInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_ModelInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_BufferInfoXbox:
    cdef lotrc_rs.mut_slice_BufferInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BufferInfoXbox()
        val.ptr = lotrc_rs.mut_slice_BufferInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_BufferInfoXbox slice):
        return lotrc_rs.mut_slice_BufferInfoXbox_len(slice.ptr)

cdef class DumpInfo_BufferInfoXbox:
    cdef lotrc_rs.DumpInfo_BufferInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_BufferInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat1Xbox:
    cdef lotrc_rs.mut_slice_Mat1Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat1Xbox()
        val.ptr = lotrc_rs.mut_slice_Mat1Xbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat1Xbox slice):
        return lotrc_rs.mut_slice_Mat1Xbox_len(slice.ptr)

cdef class DumpInfo_Mat1Xbox:
    cdef lotrc_rs.DumpInfo_Mat1Xbox* ptr
    @property
    def vals(self):
        val = mut_slice_Mat1Xbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat2Xbox:
    cdef lotrc_rs.mut_slice_Mat2Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat2Xbox()
        val.ptr = lotrc_rs.mut_slice_Mat2Xbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat2Xbox slice):
        return lotrc_rs.mut_slice_Mat2Xbox_len(slice.ptr)

cdef class DumpInfo_Mat2Xbox:
    cdef lotrc_rs.DumpInfo_Mat2Xbox* ptr
    @property
    def vals(self):
        val = mut_slice_Mat2Xbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat3Xbox:
    cdef lotrc_rs.mut_slice_Mat3Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat3Xbox()
        val.ptr = lotrc_rs.mut_slice_Mat3Xbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat3Xbox slice):
        return lotrc_rs.mut_slice_Mat3Xbox_len(slice.ptr)

cdef class DumpInfo_Mat3Xbox:
    cdef lotrc_rs.DumpInfo_Mat3Xbox* ptr
    @property
    def vals(self):
        val = mut_slice_Mat3Xbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat4Xbox:
    cdef lotrc_rs.mut_slice_Mat4Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat4Xbox()
        val.ptr = lotrc_rs.mut_slice_Mat4Xbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat4Xbox slice):
        return lotrc_rs.mut_slice_Mat4Xbox_len(slice.ptr)

cdef class DumpInfo_Mat4Xbox:
    cdef lotrc_rs.DumpInfo_Mat4Xbox* ptr
    @property
    def vals(self):
        val = mut_slice_Mat4Xbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_MatExtraXbox:
    cdef lotrc_rs.mut_slice_MatExtraXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = MatExtraXbox()
        val.ptr = lotrc_rs.mut_slice_MatExtraXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_MatExtraXbox slice):
        return lotrc_rs.mut_slice_MatExtraXbox_len(slice.ptr)

cdef class DumpInfo_MatExtraXbox:
    cdef lotrc_rs.DumpInfo_MatExtraXbox* ptr
    @property
    def vals(self):
        val = mut_slice_MatExtraXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_ShapeInfoXbox:
    cdef lotrc_rs.mut_slice_ShapeInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ShapeInfoXbox()
        val.ptr = lotrc_rs.mut_slice_ShapeInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_ShapeInfoXbox slice):
        return lotrc_rs.mut_slice_ShapeInfoXbox_len(slice.ptr)

cdef class DumpInfo_ShapeInfoXbox:
    cdef lotrc_rs.DumpInfo_ShapeInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_ShapeInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_HkShapeInfoXbox:
    cdef lotrc_rs.mut_slice_HkShapeInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkShapeInfoXbox()
        val.ptr = lotrc_rs.mut_slice_HkShapeInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_HkShapeInfoXbox slice):
        return lotrc_rs.mut_slice_HkShapeInfoXbox_len(slice.ptr)

cdef class DumpInfo_HkShapeInfoXbox:
    cdef lotrc_rs.DumpInfo_HkShapeInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_HkShapeInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_HkConstraintDataXbox:
    cdef lotrc_rs.mut_slice_HkConstraintDataXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintDataXbox()
        val.ptr = lotrc_rs.mut_slice_HkConstraintDataXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_HkConstraintDataXbox slice):
        return lotrc_rs.mut_slice_HkConstraintDataXbox_len(slice.ptr)

cdef class DumpInfo_HkConstraintDataXbox:
    cdef lotrc_rs.DumpInfo_HkConstraintDataXbox* ptr
    @property
    def vals(self):
        val = mut_slice_HkConstraintDataXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_VBuffInfoXbox:
    cdef lotrc_rs.mut_slice_VBuffInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = VBuffInfoXbox()
        val.ptr = lotrc_rs.mut_slice_VBuffInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_VBuffInfoXbox slice):
        return lotrc_rs.mut_slice_VBuffInfoXbox_len(slice.ptr)

cdef class DumpInfo_VBuffInfoXbox:
    cdef lotrc_rs.DumpInfo_VBuffInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_VBuffInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_IBuffInfoXbox:
    cdef lotrc_rs.mut_slice_IBuffInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = IBuffInfoXbox()
        val.ptr = lotrc_rs.mut_slice_IBuffInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_IBuffInfoXbox slice):
        return lotrc_rs.mut_slice_IBuffInfoXbox_len(slice.ptr)

cdef class DumpInfo_IBuffInfoXbox:
    cdef lotrc_rs.DumpInfo_IBuffInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_IBuffInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_TextureInfoXbox:
    cdef lotrc_rs.mut_slice_TextureInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TextureInfoXbox()
        val.ptr = lotrc_rs.mut_slice_TextureInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_TextureInfoXbox slice):
        return lotrc_rs.mut_slice_TextureInfoXbox_len(slice.ptr)

cdef class DumpInfo_TextureInfoXbox:
    cdef lotrc_rs.DumpInfo_TextureInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_TextureInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_AnimationInfoXbox:
    cdef lotrc_rs.mut_slice_AnimationInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationInfoXbox()
        val.ptr = lotrc_rs.mut_slice_AnimationInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_AnimationInfoXbox slice):
        return lotrc_rs.mut_slice_AnimationInfoXbox_len(slice.ptr)

cdef class DumpInfo_AnimationInfoXbox:
    cdef lotrc_rs.DumpInfo_AnimationInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_AnimationInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_HkConstraintInfoXbox:
    cdef lotrc_rs.mut_slice_HkConstraintInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintInfoXbox()
        val.ptr = lotrc_rs.mut_slice_HkConstraintInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_HkConstraintInfoXbox slice):
        return lotrc_rs.mut_slice_HkConstraintInfoXbox_len(slice.ptr)

cdef class DumpInfo_HkConstraintInfoXbox:
    cdef lotrc_rs.DumpInfo_HkConstraintInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_HkConstraintInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_EffectInfoXbox:
    cdef lotrc_rs.mut_slice_EffectInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = EffectInfoXbox()
        val.ptr = lotrc_rs.mut_slice_EffectInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_EffectInfoXbox slice):
        return lotrc_rs.mut_slice_EffectInfoXbox_len(slice.ptr)

cdef class DumpInfo_EffectInfoXbox:
    cdef lotrc_rs.DumpInfo_EffectInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_EffectInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_PFieldInfoXbox:
    cdef lotrc_rs.mut_slice_PFieldInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = PFieldInfoXbox()
        val.ptr = lotrc_rs.mut_slice_PFieldInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_PFieldInfoXbox slice):
        return lotrc_rs.mut_slice_PFieldInfoXbox_len(slice.ptr)

cdef class DumpInfo_PFieldInfoXbox:
    cdef lotrc_rs.DumpInfo_PFieldInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_PFieldInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_GFXBlockInfoXbox:
    cdef lotrc_rs.mut_slice_GFXBlockInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = GFXBlockInfoXbox()
        val.ptr = lotrc_rs.mut_slice_GFXBlockInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_GFXBlockInfoXbox slice):
        return lotrc_rs.mut_slice_GFXBlockInfoXbox_len(slice.ptr)

cdef class DumpInfo_GFXBlockInfoXbox:
    cdef lotrc_rs.DumpInfo_GFXBlockInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_GFXBlockInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_AnimationBlockInfoXbox:
    cdef lotrc_rs.mut_slice_AnimationBlockInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationBlockInfoXbox()
        val.ptr = lotrc_rs.mut_slice_AnimationBlockInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_AnimationBlockInfoXbox slice):
        return lotrc_rs.mut_slice_AnimationBlockInfoXbox_len(slice.ptr)

cdef class DumpInfo_AnimationBlockInfoXbox:
    cdef lotrc_rs.DumpInfo_AnimationBlockInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_AnimationBlockInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_FoliageInfoXbox:
    cdef lotrc_rs.mut_slice_FoliageInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = FoliageInfoXbox()
        val.ptr = lotrc_rs.mut_slice_FoliageInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_FoliageInfoXbox slice):
        return lotrc_rs.mut_slice_FoliageInfoXbox_len(slice.ptr)

cdef class DumpInfo_FoliageInfoXbox:
    cdef lotrc_rs.DumpInfo_FoliageInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_FoliageInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_RadiosityValsInfoXbox:
    cdef lotrc_rs.mut_slice_RadiosityValsInfoXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RadiosityValsInfoXbox()
        val.ptr = lotrc_rs.mut_slice_RadiosityValsInfoXbox_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_RadiosityValsInfoXbox slice):
        return lotrc_rs.mut_slice_RadiosityValsInfoXbox_len(slice.ptr)

cdef class DumpInfo_RadiosityValsInfoXbox:
    cdef lotrc_rs.DumpInfo_RadiosityValsInfoXbox* ptr
    @property
    def vals(self):
        val = mut_slice_RadiosityValsInfoXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_u32Xbox:
    cdef lotrc_rs.mut_slice_u32Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.mut_slice_u32Xbox_get(self.ptr, idx))
    @staticmethod
    def len(slice_u32Xbox slice):
        return lotrc_rs.mut_slice_u32Xbox_len(slice.ptr)

cdef class DumpInfo_u32Xbox:
    cdef lotrc_rs.DumpInfo_u32Xbox* ptr
    @property
    def vals(self):
        val = mut_slice_u32Xbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class Vec_DumpInfoDataXbox:
    cdef lotrc_rs.Vec_DumpInfoDataXbox* ptr

cdef class DumpInfosXbox:
    cdef lotrc_rs.DumpInfosXbox* ptr
    @staticmethod
    def from_data(DumpSlice dst, InfoCounts counts, mut_slice_u32Xbox offsets):
        val = OwnedDumpInfosXbox()
        val.ptr = lotrc_rs.DumpInfosXbox_from_data(dst.ptr, counts.ptr, offsets.ptr)
        return val
    @property
    def objas(self):
        val = DumpInfo_ObjAXbox()
        val.ptr = &self.ptr.objas
        return val
    @property
    def obj0s(self):
        val = DumpInfo_Obj0Xbox()
        val.ptr = &self.ptr.obj0s
        return val
    @property
    def models(self):
        val = DumpInfo_ModelInfoXbox()
        val.ptr = &self.ptr.models
        return val
    @property
    def buffers(self):
        val = DumpInfo_BufferInfoXbox()
        val.ptr = &self.ptr.buffers
        return val
    @property
    def mat1s(self):
        val = DumpInfo_Mat1Xbox()
        val.ptr = &self.ptr.mat1s
        return val
    @property
    def mat2s(self):
        val = DumpInfo_Mat2Xbox()
        val.ptr = &self.ptr.mat2s
        return val
    @property
    def mat3s(self):
        val = DumpInfo_Mat3Xbox()
        val.ptr = &self.ptr.mat3s
        return val
    @property
    def mat4s(self):
        val = DumpInfo_Mat4Xbox()
        val.ptr = &self.ptr.mat4s
        return val
    @property
    def mat_extras(self):
        val = DumpInfo_MatExtraXbox()
        val.ptr = &self.ptr.mat_extras
        return val
    @property
    def shapes(self):
        val = DumpInfo_ShapeInfoXbox()
        val.ptr = &self.ptr.shapes
        return val
    @property
    def hk_shapes(self):
        val = DumpInfo_HkShapeInfoXbox()
        val.ptr = &self.ptr.hk_shapes
        return val
    @property
    def hk_constraint_datas(self):
        val = DumpInfo_HkConstraintDataXbox()
        val.ptr = &self.ptr.hk_constraint_datas
        return val
    @property
    def vbuffs(self):
        val = DumpInfo_VBuffInfoXbox()
        val.ptr = &self.ptr.vbuffs
        return val
    @property
    def ibuffs(self):
        val = DumpInfo_IBuffInfoXbox()
        val.ptr = &self.ptr.ibuffs
        return val
    @property
    def textures(self):
        val = DumpInfo_TextureInfoXbox()
        val.ptr = &self.ptr.textures
        return val
    @property
    def animations(self):
        val = DumpInfo_AnimationInfoXbox()
        val.ptr = &self.ptr.animations
        return val
    @property
    def hk_constraints(self):
        val = DumpInfo_HkConstraintInfoXbox()
        val.ptr = &self.ptr.hk_constraints
        return val
    @property
    def effects(self):
        val = DumpInfo_EffectInfoXbox()
        val.ptr = &self.ptr.effects
        return val
    @property
    def pfields(self):
        val = DumpInfo_PFieldInfoXbox()
        val.ptr = &self.ptr.pfields
        return val
    @property
    def gfxs(self):
        val = DumpInfo_GFXBlockInfoXbox()
        val.ptr = &self.ptr.gfxs
        return val
    @property
    def animation_blocks(self):
        val = DumpInfo_AnimationBlockInfoXbox()
        val.ptr = &self.ptr.animation_blocks
        return val
    @property
    def foliages(self):
        val = DumpInfo_FoliageInfoXbox()
        val.ptr = &self.ptr.foliages
        return val
    @property
    def radiosity_vals(self):
        val = DumpInfo_RadiosityValsInfoXbox()
        val.ptr = &self.ptr.radiosity_vals
        return val
    @property
    def offsets(self):
        val = DumpInfo_u32Xbox()
        val.ptr = &self.ptr.offsets
        return val
    @property
    def model_data(self):
        val = Vec_DumpInfoDataXbox()
        val.ptr = &self.ptr.model_data
        return val
    @property
    def texture_data(self):
        val = Vec_DumpInfoDataXbox()
        val.ptr = &self.ptr.texture_data
        return val

cdef class mut_slice_ObjAPs3:
    cdef lotrc_rs.mut_slice_ObjAPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ObjAPs3()
        val.ptr = lotrc_rs.mut_slice_ObjAPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_ObjAPs3 slice):
        return lotrc_rs.mut_slice_ObjAPs3_len(slice.ptr)

cdef class DumpInfo_ObjAPs3:
    cdef lotrc_rs.DumpInfo_ObjAPs3* ptr
    @property
    def vals(self):
        val = mut_slice_ObjAPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Obj0Ps3:
    cdef lotrc_rs.mut_slice_Obj0Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj0Ps3()
        val.ptr = lotrc_rs.mut_slice_Obj0Ps3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Obj0Ps3 slice):
        return lotrc_rs.mut_slice_Obj0Ps3_len(slice.ptr)

cdef class DumpInfo_Obj0Ps3:
    cdef lotrc_rs.DumpInfo_Obj0Ps3* ptr
    @property
    def vals(self):
        val = mut_slice_Obj0Ps3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_ModelInfoPs3:
    cdef lotrc_rs.mut_slice_ModelInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ModelInfoPs3()
        val.ptr = lotrc_rs.mut_slice_ModelInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_ModelInfoPs3 slice):
        return lotrc_rs.mut_slice_ModelInfoPs3_len(slice.ptr)

cdef class DumpInfo_ModelInfoPs3:
    cdef lotrc_rs.DumpInfo_ModelInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_ModelInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_BufferInfoPs3:
    cdef lotrc_rs.mut_slice_BufferInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BufferInfoPs3()
        val.ptr = lotrc_rs.mut_slice_BufferInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_BufferInfoPs3 slice):
        return lotrc_rs.mut_slice_BufferInfoPs3_len(slice.ptr)

cdef class DumpInfo_BufferInfoPs3:
    cdef lotrc_rs.DumpInfo_BufferInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_BufferInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat1Ps3:
    cdef lotrc_rs.mut_slice_Mat1Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat1Ps3()
        val.ptr = lotrc_rs.mut_slice_Mat1Ps3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat1Ps3 slice):
        return lotrc_rs.mut_slice_Mat1Ps3_len(slice.ptr)

cdef class DumpInfo_Mat1Ps3:
    cdef lotrc_rs.DumpInfo_Mat1Ps3* ptr
    @property
    def vals(self):
        val = mut_slice_Mat1Ps3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat2Ps3:
    cdef lotrc_rs.mut_slice_Mat2Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat2Ps3()
        val.ptr = lotrc_rs.mut_slice_Mat2Ps3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat2Ps3 slice):
        return lotrc_rs.mut_slice_Mat2Ps3_len(slice.ptr)

cdef class DumpInfo_Mat2Ps3:
    cdef lotrc_rs.DumpInfo_Mat2Ps3* ptr
    @property
    def vals(self):
        val = mut_slice_Mat2Ps3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat3Ps3:
    cdef lotrc_rs.mut_slice_Mat3Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat3Ps3()
        val.ptr = lotrc_rs.mut_slice_Mat3Ps3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat3Ps3 slice):
        return lotrc_rs.mut_slice_Mat3Ps3_len(slice.ptr)

cdef class DumpInfo_Mat3Ps3:
    cdef lotrc_rs.DumpInfo_Mat3Ps3* ptr
    @property
    def vals(self):
        val = mut_slice_Mat3Ps3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_Mat4Ps3:
    cdef lotrc_rs.mut_slice_Mat4Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Mat4Ps3()
        val.ptr = lotrc_rs.mut_slice_Mat4Ps3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_Mat4Ps3 slice):
        return lotrc_rs.mut_slice_Mat4Ps3_len(slice.ptr)

cdef class DumpInfo_Mat4Ps3:
    cdef lotrc_rs.DumpInfo_Mat4Ps3* ptr
    @property
    def vals(self):
        val = mut_slice_Mat4Ps3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_MatExtraPs3:
    cdef lotrc_rs.mut_slice_MatExtraPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = MatExtraPs3()
        val.ptr = lotrc_rs.mut_slice_MatExtraPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_MatExtraPs3 slice):
        return lotrc_rs.mut_slice_MatExtraPs3_len(slice.ptr)

cdef class DumpInfo_MatExtraPs3:
    cdef lotrc_rs.DumpInfo_MatExtraPs3* ptr
    @property
    def vals(self):
        val = mut_slice_MatExtraPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_ShapeInfoPs3:
    cdef lotrc_rs.mut_slice_ShapeInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = ShapeInfoPs3()
        val.ptr = lotrc_rs.mut_slice_ShapeInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_ShapeInfoPs3 slice):
        return lotrc_rs.mut_slice_ShapeInfoPs3_len(slice.ptr)

cdef class DumpInfo_ShapeInfoPs3:
    cdef lotrc_rs.DumpInfo_ShapeInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_ShapeInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_HkShapeInfoPs3:
    cdef lotrc_rs.mut_slice_HkShapeInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkShapeInfoPs3()
        val.ptr = lotrc_rs.mut_slice_HkShapeInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_HkShapeInfoPs3 slice):
        return lotrc_rs.mut_slice_HkShapeInfoPs3_len(slice.ptr)

cdef class DumpInfo_HkShapeInfoPs3:
    cdef lotrc_rs.DumpInfo_HkShapeInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_HkShapeInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_HkConstraintDataPs3:
    cdef lotrc_rs.mut_slice_HkConstraintDataPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintDataPs3()
        val.ptr = lotrc_rs.mut_slice_HkConstraintDataPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_HkConstraintDataPs3 slice):
        return lotrc_rs.mut_slice_HkConstraintDataPs3_len(slice.ptr)

cdef class DumpInfo_HkConstraintDataPs3:
    cdef lotrc_rs.DumpInfo_HkConstraintDataPs3* ptr
    @property
    def vals(self):
        val = mut_slice_HkConstraintDataPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_VBuffInfoPs3:
    cdef lotrc_rs.mut_slice_VBuffInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = VBuffInfoPs3()
        val.ptr = lotrc_rs.mut_slice_VBuffInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_VBuffInfoPs3 slice):
        return lotrc_rs.mut_slice_VBuffInfoPs3_len(slice.ptr)

cdef class DumpInfo_VBuffInfoPs3:
    cdef lotrc_rs.DumpInfo_VBuffInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_VBuffInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_IBuffInfoPs3:
    cdef lotrc_rs.mut_slice_IBuffInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = IBuffInfoPs3()
        val.ptr = lotrc_rs.mut_slice_IBuffInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_IBuffInfoPs3 slice):
        return lotrc_rs.mut_slice_IBuffInfoPs3_len(slice.ptr)

cdef class DumpInfo_IBuffInfoPs3:
    cdef lotrc_rs.DumpInfo_IBuffInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_IBuffInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_TextureInfoPs3:
    cdef lotrc_rs.mut_slice_TextureInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TextureInfoPs3()
        val.ptr = lotrc_rs.mut_slice_TextureInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_TextureInfoPs3 slice):
        return lotrc_rs.mut_slice_TextureInfoPs3_len(slice.ptr)

cdef class DumpInfo_TextureInfoPs3:
    cdef lotrc_rs.DumpInfo_TextureInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_TextureInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_AnimationInfoPs3:
    cdef lotrc_rs.mut_slice_AnimationInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationInfoPs3()
        val.ptr = lotrc_rs.mut_slice_AnimationInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_AnimationInfoPs3 slice):
        return lotrc_rs.mut_slice_AnimationInfoPs3_len(slice.ptr)

cdef class DumpInfo_AnimationInfoPs3:
    cdef lotrc_rs.DumpInfo_AnimationInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_AnimationInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_HkConstraintInfoPs3:
    cdef lotrc_rs.mut_slice_HkConstraintInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = HkConstraintInfoPs3()
        val.ptr = lotrc_rs.mut_slice_HkConstraintInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_HkConstraintInfoPs3 slice):
        return lotrc_rs.mut_slice_HkConstraintInfoPs3_len(slice.ptr)

cdef class DumpInfo_HkConstraintInfoPs3:
    cdef lotrc_rs.DumpInfo_HkConstraintInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_HkConstraintInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_EffectInfoPs3:
    cdef lotrc_rs.mut_slice_EffectInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = EffectInfoPs3()
        val.ptr = lotrc_rs.mut_slice_EffectInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_EffectInfoPs3 slice):
        return lotrc_rs.mut_slice_EffectInfoPs3_len(slice.ptr)

cdef class DumpInfo_EffectInfoPs3:
    cdef lotrc_rs.DumpInfo_EffectInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_EffectInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_PFieldInfoPs3:
    cdef lotrc_rs.mut_slice_PFieldInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = PFieldInfoPs3()
        val.ptr = lotrc_rs.mut_slice_PFieldInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_PFieldInfoPs3 slice):
        return lotrc_rs.mut_slice_PFieldInfoPs3_len(slice.ptr)

cdef class DumpInfo_PFieldInfoPs3:
    cdef lotrc_rs.DumpInfo_PFieldInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_PFieldInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_GFXBlockInfoPs3:
    cdef lotrc_rs.mut_slice_GFXBlockInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = GFXBlockInfoPs3()
        val.ptr = lotrc_rs.mut_slice_GFXBlockInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_GFXBlockInfoPs3 slice):
        return lotrc_rs.mut_slice_GFXBlockInfoPs3_len(slice.ptr)

cdef class DumpInfo_GFXBlockInfoPs3:
    cdef lotrc_rs.DumpInfo_GFXBlockInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_GFXBlockInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_AnimationBlockInfoPs3:
    cdef lotrc_rs.mut_slice_AnimationBlockInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AnimationBlockInfoPs3()
        val.ptr = lotrc_rs.mut_slice_AnimationBlockInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_AnimationBlockInfoPs3 slice):
        return lotrc_rs.mut_slice_AnimationBlockInfoPs3_len(slice.ptr)

cdef class DumpInfo_AnimationBlockInfoPs3:
    cdef lotrc_rs.DumpInfo_AnimationBlockInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_AnimationBlockInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_FoliageInfoPs3:
    cdef lotrc_rs.mut_slice_FoliageInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = FoliageInfoPs3()
        val.ptr = lotrc_rs.mut_slice_FoliageInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_FoliageInfoPs3 slice):
        return lotrc_rs.mut_slice_FoliageInfoPs3_len(slice.ptr)

cdef class DumpInfo_FoliageInfoPs3:
    cdef lotrc_rs.DumpInfo_FoliageInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_FoliageInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_RadiosityValsInfoPs3:
    cdef lotrc_rs.mut_slice_RadiosityValsInfoPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RadiosityValsInfoPs3()
        val.ptr = lotrc_rs.mut_slice_RadiosityValsInfoPs3_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_RadiosityValsInfoPs3 slice):
        return lotrc_rs.mut_slice_RadiosityValsInfoPs3_len(slice.ptr)

cdef class DumpInfo_RadiosityValsInfoPs3:
    cdef lotrc_rs.DumpInfo_RadiosityValsInfoPs3* ptr
    @property
    def vals(self):
        val = mut_slice_RadiosityValsInfoPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class mut_slice_u32Ps3:
    cdef lotrc_rs.mut_slice_u32Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.mut_slice_u32Ps3_get(self.ptr, idx))
    @staticmethod
    def len(slice_u32Ps3 slice):
        return lotrc_rs.mut_slice_u32Ps3_len(slice.ptr)

cdef class DumpInfo_u32Ps3:
    cdef lotrc_rs.DumpInfo_u32Ps3* ptr
    @property
    def vals(self):
        val = mut_slice_u32Ps3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def ind(self):
        return self.ptr.ind
    @property
    def offset(self):
        return self.ptr.offset

cdef class Vec_DumpInfoDataPs3:
    cdef lotrc_rs.Vec_DumpInfoDataPs3* ptr

cdef class DumpInfosPs3:
    cdef lotrc_rs.DumpInfosPs3* ptr
    @staticmethod
    def from_data(DumpSlice dst, InfoCounts counts, mut_slice_u32Ps3 offsets):
        val = OwnedDumpInfosPs3()
        val.ptr = lotrc_rs.DumpInfosPs3_from_data(dst.ptr, counts.ptr, offsets.ptr)
        return val
    @property
    def objas(self):
        val = DumpInfo_ObjAPs3()
        val.ptr = &self.ptr.objas
        return val
    @property
    def obj0s(self):
        val = DumpInfo_Obj0Ps3()
        val.ptr = &self.ptr.obj0s
        return val
    @property
    def models(self):
        val = DumpInfo_ModelInfoPs3()
        val.ptr = &self.ptr.models
        return val
    @property
    def buffers(self):
        val = DumpInfo_BufferInfoPs3()
        val.ptr = &self.ptr.buffers
        return val
    @property
    def mat1s(self):
        val = DumpInfo_Mat1Ps3()
        val.ptr = &self.ptr.mat1s
        return val
    @property
    def mat2s(self):
        val = DumpInfo_Mat2Ps3()
        val.ptr = &self.ptr.mat2s
        return val
    @property
    def mat3s(self):
        val = DumpInfo_Mat3Ps3()
        val.ptr = &self.ptr.mat3s
        return val
    @property
    def mat4s(self):
        val = DumpInfo_Mat4Ps3()
        val.ptr = &self.ptr.mat4s
        return val
    @property
    def mat_extras(self):
        val = DumpInfo_MatExtraPs3()
        val.ptr = &self.ptr.mat_extras
        return val
    @property
    def shapes(self):
        val = DumpInfo_ShapeInfoPs3()
        val.ptr = &self.ptr.shapes
        return val
    @property
    def hk_shapes(self):
        val = DumpInfo_HkShapeInfoPs3()
        val.ptr = &self.ptr.hk_shapes
        return val
    @property
    def hk_constraint_datas(self):
        val = DumpInfo_HkConstraintDataPs3()
        val.ptr = &self.ptr.hk_constraint_datas
        return val
    @property
    def vbuffs(self):
        val = DumpInfo_VBuffInfoPs3()
        val.ptr = &self.ptr.vbuffs
        return val
    @property
    def ibuffs(self):
        val = DumpInfo_IBuffInfoPs3()
        val.ptr = &self.ptr.ibuffs
        return val
    @property
    def textures(self):
        val = DumpInfo_TextureInfoPs3()
        val.ptr = &self.ptr.textures
        return val
    @property
    def animations(self):
        val = DumpInfo_AnimationInfoPs3()
        val.ptr = &self.ptr.animations
        return val
    @property
    def hk_constraints(self):
        val = DumpInfo_HkConstraintInfoPs3()
        val.ptr = &self.ptr.hk_constraints
        return val
    @property
    def effects(self):
        val = DumpInfo_EffectInfoPs3()
        val.ptr = &self.ptr.effects
        return val
    @property
    def pfields(self):
        val = DumpInfo_PFieldInfoPs3()
        val.ptr = &self.ptr.pfields
        return val
    @property
    def gfxs(self):
        val = DumpInfo_GFXBlockInfoPs3()
        val.ptr = &self.ptr.gfxs
        return val
    @property
    def animation_blocks(self):
        val = DumpInfo_AnimationBlockInfoPs3()
        val.ptr = &self.ptr.animation_blocks
        return val
    @property
    def foliages(self):
        val = DumpInfo_FoliageInfoPs3()
        val.ptr = &self.ptr.foliages
        return val
    @property
    def radiosity_vals(self):
        val = DumpInfo_RadiosityValsInfoPs3()
        val.ptr = &self.ptr.radiosity_vals
        return val
    @property
    def offsets(self):
        val = DumpInfo_u32Ps3()
        val.ptr = &self.ptr.offsets
        return val
    @property
    def model_data(self):
        val = Vec_DumpInfoDataPs3()
        val.ptr = &self.ptr.model_data
        return val
    @property
    def texture_data(self):
        val = Vec_DumpInfoDataPs3()
        val.ptr = &self.ptr.texture_data
        return val

cdef class mut_slice_u8:
    cdef lotrc_rs.mut_slice_u8* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.mut_slice_u8_get(self.ptr, idx))
    @staticmethod
    def len(slice_u8 slice):
        return lotrc_rs.mut_slice_u8_len(slice.ptr)

cdef class DumpSlice:
    cdef lotrc_rs.DumpSlice* ptr
    @property
    def vals(self):
        val = mut_slice_u8()
        val.ptr = &self.ptr.vals
        return val
    @property
    def offset(self):
        return self.ptr.offset

cdef class string:
    cdef lotrc_rs.string* ptr
    def get(self):
        return dereference(lotrc_rs.string_get(self.ptr))
    def len(self):
        return lotrc_rs.string_len(self.ptr)

cdef class mut_slice_u32:
    cdef lotrc_rs.mut_slice_u32* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.mut_slice_u32_get(self.ptr, idx))
    @staticmethod
    def len(slice_u32 slice):
        return lotrc_rs.mut_slice_u32_len(slice.ptr)

cdef class VertexDataIndex:
    cdef lotrc_rs.VertexDataIndex* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset

cdef class IndexMap_VertexUsage__VertexDataIndex:
    cdef lotrc_rs.IndexMap_VertexUsage__VertexDataIndex* ptr
    def get(self, VertexUsage key):
        val = VertexDataIndex()
        val.ptr = lotrc_rs.IndexMap_VertexUsage__VertexDataIndex_get(self.ptr, key.ptr)
        return val
    def len(self):
        return lotrc_rs.IndexMap_VertexUsage__VertexDataIndex_len(self.ptr)
    def keys(self, mut_slice_VertexUsage keys):
        lotrc_rs.IndexMap_VertexUsage__VertexDataIndex_keys(self.ptr, keys.ptr)

cdef class VertexUsage:
    cdef lotrc_rs.VertexUsage* ptr

cdef class mut_slice_VertexUsage:
    cdef lotrc_rs.mut_slice_VertexUsage* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = VertexUsage()
        val.ptr = lotrc_rs.mut_slice_VertexUsage_get(self.ptr, idx)
        return val
    @staticmethod
    def len(slice_VertexUsage slice):
        return lotrc_rs.mut_slice_VertexUsage_len(slice.ptr)

cdef class ref_slice_u32:
    cdef lotrc_rs.ref_slice_u32* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_u32_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_u32_len(self.ptr)

cdef class ref_slice_VertexUsage:
    cdef lotrc_rs.ref_slice_VertexUsage* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = VertexUsage()
        val.ptr = lotrc_rs.ref_slice_VertexUsage_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_VertexUsage_len(self.ptr)

cdef class slice______CompressedDataRef:
    cdef lotrc_rs.slice______CompressedDataRef* ptr

cdef class IBuffInfoPc:
    cdef lotrc_rs.IBuffInfoPc* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def size(self):
        return self.ptr.size
    @property
    def format(self):
        return self.ptr.format
    @property
    def vbuff_alt_fmt(self):
        return self.ptr.vbuff_alt_fmt
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def unk_5(self):
        return self.ptr.unk_5

cdef class ref_slice_u16Pc:
    cdef lotrc_rs.ref_slice_u16Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_u16Pc_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_u16Pc_len(self.ptr)

cdef class IndexBufferValsRefPc:
    cdef lotrc_rs.IndexBufferValsRefPc* ptr

cdef class IndexBufferRefPc:
    cdef lotrc_rs.IndexBufferRefPc* ptr
    @property
    def info(self):
        val = IBuffInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def vals(self):
        val = IndexBufferValsRefPc()
        val.ptr = &self.ptr.vals
        return val

cdef class IndexMap_u32__IndexBufferRefPc:
    cdef lotrc_rs.IndexMap_u32__IndexBufferRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = IndexBufferRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__IndexBufferRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__IndexBufferRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__IndexBufferRefPc_keys(self.ptr, keys.ptr)

cdef class VBuffInfoPc:
    cdef lotrc_rs.VBuffInfoPc* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def size(self):
        return self.ptr.size
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def fmt1(self):
        return self.ptr.fmt1
    @property
    def fmt2(self):
        return self.ptr.fmt2
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7

cdef class VertexBufferRefPc:
    cdef lotrc_rs.VertexBufferRefPc* ptr
    @property
    def info(self):
        val = VBuffInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def offsets(self):
        val = IndexMap_VertexUsage__VertexDataIndex()
        val.ptr = &self.ptr.offsets
        return val
    @property
    def size(self):
        return self.ptr.size
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val

cdef class IndexMap_u32__VertexBufferRefPc:
    cdef lotrc_rs.IndexMap_u32__VertexBufferRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = VertexBufferRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__VertexBufferRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__VertexBufferRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__VertexBufferRefPc_keys(self.ptr, keys.ptr)

cdef class MatBasePc:
    cdef lotrc_rs.MatBasePc* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def tex0(self):
        return self.ptr.tex0
    @property
    def tex1(self):
        return self.ptr.tex1
    @property
    def tex2(self):
        return self.ptr.tex2
    @property
    def tex3(self):
        return self.ptr.tex3
    @property
    def tex4(self):
        return self.ptr.tex4
    @property
    def tex5(self):
        return self.ptr.tex5
    @property
    def key_guid(self):
        return self.ptr.key_guid
    @property
    def mask0(self):
        return self.ptr.mask0
    @property
    def mask1(self):
        return self.ptr.mask1
    @property
    def mask2(self):
        return self.ptr.mask2
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def unk_33(self):
        return self.ptr.unk_33
    @property
    def z_34(self):
        return self.ptr.z_34
    @property
    def z_35(self):
        return self.ptr.z_35
    @property
    def z_36(self):
        return self.ptr.z_36
    @property
    def z_37(self):
        return self.ptr.z_37
    @property
    def z_38(self):
        return self.ptr.z_38
    @property
    def z_39(self):
        return self.ptr.z_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42
    @property
    def unk_43(self):
        return self.ptr.unk_43
    @property
    def unk_44(self):
        return self.ptr.unk_44
    @property
    def unk_45(self):
        return self.ptr.unk_45
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def unk_47(self):
        return self.ptr.unk_47
    @property
    def unk_48(self):
        return self.ptr.unk_48
    @property
    def unk_49(self):
        return self.ptr.unk_49
    @property
    def flags(self):
        return self.ptr.flags
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_53(self):
        return self.ptr.unk_53
    @property
    def unk_54a(self):
        return self.ptr.unk_54a
    @property
    def unk_54b(self):
        return self.ptr.unk_54b
    @property
    def side_flags(self):
        return self.ptr.side_flags
    @property
    def unk_55(self):
        return self.ptr.unk_55
    @property
    def unk_56(self):
        return self.ptr.unk_56
    @property
    def unk_57(self):
        return self.ptr.unk_57
    @property
    def unk_58(self):
        return self.ptr.unk_58
    @property
    def unk_59(self):
        return self.ptr.unk_59
    @property
    def unk_60(self):
        return self.ptr.unk_60
    @property
    def unk_61(self):
        return self.ptr.unk_61
    @property
    def unk_62(self):
        return self.ptr.unk_62
    @property
    def unk_63(self):
        return self.ptr.unk_63
    @property
    def unk_64(self):
        return self.ptr.unk_64
    @property
    def unk_65(self):
        return self.ptr.unk_65
    @property
    def unk_66(self):
        return self.ptr.unk_66
    @property
    def unk_67(self):
        return self.ptr.unk_67
    @property
    def unk_68(self):
        return self.ptr.unk_68
    @property
    def unk_69(self):
        return self.ptr.unk_69
    @property
    def unk_70(self):
        return self.ptr.unk_70
    @property
    def unk_71(self):
        return self.ptr.unk_71
    @property
    def unk_72(self):
        return self.ptr.unk_72
    @property
    def unk_73(self):
        return self.ptr.unk_73
    @property
    def unk_74(self):
        return self.ptr.unk_74
    @property
    def unk_75(self):
        return self.ptr.unk_75
    @property
    def unk_76(self):
        return self.ptr.unk_76
    @property
    def unk_77(self):
        return self.ptr.unk_77
    @property
    def unk_78(self):
        return self.ptr.unk_78
    @property
    def unk_79(self):
        return self.ptr.unk_79
    @property
    def unk_80(self):
        return self.ptr.unk_80
    @property
    def unk_81(self):
        return self.ptr.unk_81
    @property
    def unk_82(self):
        return self.ptr.unk_82
    @property
    def unk_83(self):
        return self.ptr.unk_83
    @property
    def unk_84(self):
        return self.ptr.unk_84
    @property
    def unk_85(self):
        return self.ptr.unk_85
    @property
    def mat_extra_offset(self):
        return self.ptr.mat_extra_offset
    @property
    def key(self):
        return self.ptr.key
    @property
    def unk_88(self):
        return self.ptr.unk_88
    @property
    def z_89(self):
        return self.ptr.z_89

cdef class Mat1Pc:
    cdef lotrc_rs.Mat1Pc* ptr
    @property
    def base(self):
        val = MatBasePc()
        val.ptr = &self.ptr.base
        return val

cdef class MatExtraPc:
    cdef lotrc_rs.MatExtraPc* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def unk_33(self):
        return self.ptr.unk_33
    @property
    def unk_34(self):
        return self.ptr.unk_34
    @property
    def unk_35(self):
        return self.ptr.unk_35
    @property
    def unk_36(self):
        return self.ptr.unk_36
    @property
    def unk_37(self):
        return self.ptr.unk_37
    @property
    def unk_38(self):
        return self.ptr.unk_38
    @property
    def unk_39(self):
        return self.ptr.unk_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42
    @property
    def unk_43(self):
        return self.ptr.unk_43
    @property
    def unk_44(self):
        return self.ptr.unk_44
    @property
    def unk_45(self):
        return self.ptr.unk_45
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def unk_47(self):
        return self.ptr.unk_47
    @property
    def unk_48(self):
        return self.ptr.unk_48
    @property
    def unk_49(self):
        return self.ptr.unk_49

cdef class Mat1RefPc:
    cdef lotrc_rs.Mat1RefPc* ptr
    @property
    def info(self):
        val = Mat1Pc()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraPc()
        val.ptr = self.ptr.extra
        return val

cdef class Mat2Pc:
    cdef lotrc_rs.Mat2Pc* ptr
    @property
    def base(self):
        val = MatBasePc()
        val.ptr = &self.ptr.base
        return val
    @property
    def unk_90(self):
        return self.ptr.unk_90
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def unk_114(self):
        return self.ptr.unk_114
    @property
    def unk_115(self):
        return self.ptr.unk_115
    @property
    def unk_116(self):
        return self.ptr.unk_116
    @property
    def unk_117(self):
        return self.ptr.unk_117
    @property
    def unk_118(self):
        return self.ptr.unk_118
    @property
    def unk_119(self):
        return self.ptr.unk_119
    @property
    def unk_120a(self):
        return self.ptr.unk_120a
    @property
    def unk_120b(self):
        return self.ptr.unk_120b
    @property
    def unk_120c(self):
        return self.ptr.unk_120c
    @property
    def unk_120d(self):
        return self.ptr.unk_120d
    @property
    def unk_121(self):
        return self.ptr.unk_121

cdef class Mat2RefPc:
    cdef lotrc_rs.Mat2RefPc* ptr
    @property
    def info(self):
        val = Mat2Pc()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraPc()
        val.ptr = self.ptr.extra
        return val

cdef class Mat3Pc:
    cdef lotrc_rs.Mat3Pc* ptr
    @property
    def base(self):
        val = MatBasePc()
        val.ptr = &self.ptr.base
        return val
    @property
    def unk_90(self):
        return self.ptr.unk_90
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def variation_id_color(self):
        return self.ptr.variation_id_color
    @property
    def variation_id_texture(self):
        return self.ptr.variation_id_texture
    @property
    def variation_id_specular(self):
        return self.ptr.variation_id_specular
    @property
    def unk_114d(self):
        return self.ptr.unk_114d
    @property
    def unk_115(self):
        return self.ptr.unk_115

cdef class Mat3RefPc:
    cdef lotrc_rs.Mat3RefPc* ptr
    @property
    def info(self):
        val = Mat3Pc()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraPc()
        val.ptr = self.ptr.extra
        return val

cdef class Mat4Pc:
    cdef lotrc_rs.Mat4Pc* ptr
    @property
    def base(self):
        val = MatBasePc()
        val.ptr = &self.ptr.base
        return val
    @property
    def unk_90(self):
        return self.ptr.unk_90
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def unk_114(self):
        return self.ptr.unk_114
    @property
    def unk_115(self):
        return self.ptr.unk_115
    @property
    def unk_116(self):
        return self.ptr.unk_116
    @property
    def unk_117(self):
        return self.ptr.unk_117
    @property
    def unk_118(self):
        return self.ptr.unk_118
    @property
    def unk_119(self):
        return self.ptr.unk_119
    @property
    def unk_120(self):
        return self.ptr.unk_120
    @property
    def unk_121(self):
        return self.ptr.unk_121
    @property
    def unk_122(self):
        return self.ptr.unk_122
    @property
    def unk_123(self):
        return self.ptr.unk_123
    @property
    def unk_124(self):
        return self.ptr.unk_124
    @property
    def unk_125(self):
        return self.ptr.unk_125
    @property
    def unk_126(self):
        return self.ptr.unk_126
    @property
    def unk_127(self):
        return self.ptr.unk_127
    @property
    def unk_128(self):
        return self.ptr.unk_128
    @property
    def unk_129(self):
        return self.ptr.unk_129
    @property
    def unk_130(self):
        return self.ptr.unk_130
    @property
    def unk_131(self):
        return self.ptr.unk_131
    @property
    def unk_132(self):
        return self.ptr.unk_132
    @property
    def unk_133(self):
        return self.ptr.unk_133
    @property
    def unk_134(self):
        return self.ptr.unk_134
    @property
    def unk_135(self):
        return self.ptr.unk_135
    @property
    def unk_136(self):
        return self.ptr.unk_136
    @property
    def unk_137(self):
        return self.ptr.unk_137
    @property
    def unk_138(self):
        return self.ptr.unk_138
    @property
    def unk_139(self):
        return self.ptr.unk_139
    @property
    def unk_140(self):
        return self.ptr.unk_140
    @property
    def unk_141(self):
        return self.ptr.unk_141
    @property
    def unk_142(self):
        return self.ptr.unk_142
    @property
    def unk_143(self):
        return self.ptr.unk_143
    @property
    def unk_144(self):
        return self.ptr.unk_144
    @property
    def unk_145(self):
        return self.ptr.unk_145

cdef class Mat4RefPc:
    cdef lotrc_rs.Mat4RefPc* ptr
    @property
    def info(self):
        val = Mat4Pc()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraPc()
        val.ptr = self.ptr.extra
        return val

cdef class MatRefPc:
    cdef lotrc_rs.MatRefPc* ptr

cdef class IndexMap_u32__MatRefPc:
    cdef lotrc_rs.IndexMap_u32__MatRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = MatRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__MatRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__MatRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__MatRefPc_keys(self.ptr, keys.ptr)

cdef class Vector3Pc:
    cdef lotrc_rs.Vector3Pc* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def y(self):
        return self.ptr.y
    @property
    def z(self):
        return self.ptr.z

cdef class BoundingBoxPc:
    cdef lotrc_rs.BoundingBoxPc* ptr
    @property
    def center(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.center
        return val
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def half_width(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.half_width
        return val
    @property
    def unk_7(self):
        return self.ptr.unk_7

cdef class LodInfoPc:
    cdef lotrc_rs.LodInfoPc* ptr
    @property
    def start(self):
        return self.ptr.start
    @property
    def static_end(self):
        return self.ptr.static_end
    @property
    def skinned_end(self):
        return self.ptr.skinned_end
    @property
    def physics_end(self):
        return self.ptr.physics_end
    @property
    def breakable_end(self):
        return self.ptr.breakable_end

cdef class ModelInfoPc:
    cdef lotrc_rs.ModelInfoPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def mat_offset(self):
        return self.ptr.mat_offset
    @property
    def buffer_info_offset(self):
        return self.ptr.buffer_info_offset
    #@property
    #def bounding_box(self):
    #    val = BoundingBoxPc()
    #    val.ptr = &self.ptr.bounding_box
    #    return val
    @property
    def mesh_order_offset(self):
        return self.ptr.mesh_order_offset
    @property
    def lod0(self):
        val = LodInfoPc()
        val.ptr = &self.ptr.lod0
        return val
    @property
    def lod1(self):
        val = LodInfoPc()
        val.ptr = &self.ptr.lod1
        return val
    @property
    def lod2(self):
        val = LodInfoPc()
        val.ptr = &self.ptr.lod2
        return val
    @property
    def lod3(self):
        val = LodInfoPc()
        val.ptr = &self.ptr.lod3
        return val
    @property
    def mat_num(self):
        return self.ptr.mat_num
    @property
    def bones_offset(self):
        return self.ptr.bones_offset
    @property
    def bone_parents_offset(self):
        return self.ptr.bone_parents_offset
    @property
    def bone_transforms_offset(self):
        return self.ptr.bone_transforms_offset
    @property
    def bones_num(self):
        return self.ptr.bones_num
    @property
    def skin_binds_offset(self):
        return self.ptr.skin_binds_offset
    @property
    def skin_binds_num(self):
        return self.ptr.skin_binds_num
    @property
    def skin_order_offset(self):
        return self.ptr.skin_order_offset
    @property
    def vbuff_offset(self):
        return self.ptr.vbuff_offset
    @property
    def vbuff_num(self):
        return self.ptr.vbuff_num
    @property
    def ibuff_offset(self):
        return self.ptr.ibuff_offset
    @property
    def ibuff_num(self):
        return self.ptr.ibuff_num
    @property
    def mesh_bounding_boxes_offset(self):
        return self.ptr.mesh_bounding_boxes_offset
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def variation_counts(self):
        return self.ptr.variation_counts
    @property
    def vals_j_num(self):
        return self.ptr.vals_j_num
    @property
    def vals_j_offset(self):
        return self.ptr.vals_j_offset
    @property
    def block_offset(self):
        return self.ptr.block_offset
    @property
    def vals_k_offset(self):
        return self.ptr.vals_k_offset
    @property
    def asset_key(self):
        return self.ptr.asset_key
    @property
    def asset_type(self):
        return self.ptr.asset_type
    @property
    def unk_54(self):
        return self.ptr.unk_54
    @property
    def unk_55(self):
        return self.ptr.unk_55
    @property
    def shape_offset(self):
        return self.ptr.shape_offset
    @property
    def shape_num(self):
        return self.ptr.shape_num
    @property
    def hk_constraint_data_offset(self):
        return self.ptr.hk_constraint_data_offset
    @property
    def hk_constraint_data_num(self):
        return self.ptr.hk_constraint_data_num
    @property
    def hk_constraint_offset(self):
        return self.ptr.hk_constraint_offset
    @property
    def slots_offset(self):
        return self.ptr.slots_offset
    @property
    def slot_map_offset(self):
        return self.ptr.slot_map_offset
    @property
    def bone_bounding_boxes_offset(self):
        return self.ptr.bone_bounding_boxes_offset

cdef class ref_slice_CrcPc:
    cdef lotrc_rs.ref_slice_CrcPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_CrcPc_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_CrcPc_len(self.ptr)

cdef class ref_slice_i32Pc:
    cdef lotrc_rs.ref_slice_i32Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_i32Pc_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_i32Pc_len(self.ptr)

cdef class ref_slice_Matrix4x4Pc:
    cdef lotrc_rs.ref_slice_Matrix4x4Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Matrix4x4Pc()
        val.ptr = lotrc_rs.ref_slice_Matrix4x4Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Matrix4x4Pc_len(self.ptr)

cdef class ref_slice_BoundingBoxPc:
    cdef lotrc_rs.ref_slice_BoundingBoxPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BoundingBoxPc()
        val.ptr = lotrc_rs.ref_slice_BoundingBoxPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BoundingBoxPc_len(self.ptr)
    def get_arr(self):
        cdef lotrc_rs.BoundingBoxPc[:] arr = <lotrc_rs.BoundingBoxPc[:lotrc_rs.ref_slice_BoundingBoxPc_len(self.ptr)]> lotrc_rs.ref_slice_BoundingBoxPc_get(self.ptr, 0)
        return arr

cdef class BonesRefPc:
    cdef lotrc_rs.BonesRefPc* ptr
    @property
    def names(self):
        val = ref_slice_CrcPc()
        val.ptr = &self.ptr.names
        return val
    @property
    def parents(self):
        val = ref_slice_i32Pc()
        val.ptr = &self.ptr.parents
        return val
    @property
    def transforms(self):
        val = ref_slice_Matrix4x4Pc()
        val.ptr = &self.ptr.transforms
        return val
    @property
    def bounding_boxes(self):
        val = ref_slice_BoundingBoxPc()
        val.ptr = &self.ptr.bounding_boxes
        return val

cdef class ref_slice_Key2Pc:
    cdef lotrc_rs.ref_slice_Key2Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Key2Pc()
        val.ptr = lotrc_rs.ref_slice_Key2Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Key2Pc_len(self.ptr)

cdef class slice_BlockRefPc:
    cdef lotrc_rs.slice_BlockRefPc* ptr

cdef class IndexMap_u32_______VBuffInfoPc:
    cdef lotrc_rs.IndexMap_u32_______VBuffInfoPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = VBuffInfoPc()
        val.ptr = dereference(lotrc_rs.IndexMap_u32_______VBuffInfoPc_get(self.ptr, &key))
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32_______VBuffInfoPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32_______VBuffInfoPc_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32_______IBuffInfoPc:
    cdef lotrc_rs.IndexMap_u32_______IBuffInfoPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = IBuffInfoPc()
        val.ptr = dereference(lotrc_rs.IndexMap_u32_______IBuffInfoPc_get(self.ptr, &key))
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32_______IBuffInfoPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32_______IBuffInfoPc_keys(self.ptr, keys.ptr)

cdef class Option_HkConstraintRefPc:
    cdef lotrc_rs.Option_HkConstraintRefPc* ptr
    def get(self):
        val = HkConstraintRefPc()
        val.ptr = lotrc_rs.Option_HkConstraintRefPc_get(self.ptr)
        return val

cdef class slice_ShapeRefPc:
    cdef lotrc_rs.slice_ShapeRefPc* ptr

cdef class ModelDataRefPc:
    cdef lotrc_rs.ModelDataRefPc* ptr
    @property
    def infos(self):
        val = ref_slice_BufferInfoPc()
        val.ptr = &self.ptr.infos
        return val
    @property
    def vbuff_order(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.vbuff_order
        return val
    @property
    def ibuff_order(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.ibuff_order
        return val
    @property
    def vertex(self):
        val = IndexMap_u32__VertexBufferRefPc()
        val.ptr = &self.ptr.vertex
        return val
    @property
    def index(self):
        val = IndexMap_u32__IndexBufferRefPc()
        val.ptr = &self.ptr.index
        return val
    @property
    def data(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data
        return val

cdef class ModelRefPc:
    cdef lotrc_rs.ModelRefPc* ptr
    @property
    def info(self):
        val = ModelInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def bones(self):
        val = BonesRefPc()
        val.ptr = &self.ptr.bones
        return val
    @property
    def mat_order(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.mat_order
        return val
    @property
    def mesh_order(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.mesh_order
        return val
    @property
    def mesh_bounding_boxes(self):
        val = ref_slice_BoundingBoxPc()
        val.ptr = &self.ptr.mesh_bounding_boxes
        return val
    @property
    def skin_binds(self):
        val = ref_slice_Matrix4x4Pc()
        val.ptr = &self.ptr.skin_binds
        return val
    @property
    def vals_j(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.vals_j
        return val
    @property
    def val_k_header(self):
        val = ref_slice_u16Pc()
        val.ptr = &self.ptr.val_k_header
        return val
    @property
    def vals_k(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.vals_k
        return val
    @property
    def skin_order(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.skin_order
        return val
    @property
    def slots(self):
        val = ref_slice_Key2Pc()
        val.ptr = &self.ptr.slots
        return val
    @property
    def slot_map(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.slot_map
        return val
    @property
    def block_header(self):
        return dereference(self.ptr.block_header)
    @property
    def block_offsets(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.block_offsets
        return val
    @property
    def blocks(self):
        val = slice_BlockRefPc()
        val.ptr = &self.ptr.blocks
        return val
    @property
    def buffer_infos(self):
        val = ref_slice_BufferInfoPc()
        val.ptr = &self.ptr.buffer_infos
        return val
    @property
    def vbuff_order(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.vbuff_order
        return val
    @property
    def ibuff_order(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.ibuff_order
        return val
    @property
    def vbuffs(self):
        val = IndexMap_u32_______VBuffInfoPc()
        val.ptr = &self.ptr.vbuffs
        return val
    @property
    def ibuffs(self):
        val = IndexMap_u32_______IBuffInfoPc()
        val.ptr = &self.ptr.ibuffs
        return val
    @property
    def mats(self):
        val = IndexMap_u32__MatRefPc()
        val.ptr = &self.ptr.mats
        return val
    @property
    def hk_constraint(self):
        val = Option_HkConstraintRefPc()
        val.ptr = &self.ptr.hk_constraint
        return val
    @property
    def hk_constraint_datas(self):
        val = ref_slice_HkConstraintDataPc()
        val.ptr = &self.ptr.hk_constraint_datas
        return val
    @property
    def shapes(self):
        val = slice_ShapeRefPc()
        val.ptr = &self.ptr.shapes
        return val
    @property
    def data(self):
        val = ModelDataRefPc()
        val.ptr = &self.ptr.data
        return val

cdef class AnimationInfoPc:
    cdef lotrc_rs.AnimationInfoPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def vals_num(self):
        return self.ptr.vals_num
    @property
    def vals2_num(self):
        return self.ptr.vals2_num
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def vala(self):
        return self.ptr.vala
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def data_offset(self):
        return self.ptr.data_offset
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def t_scale(self):
        return self.ptr.t_scale
    @property
    def block_starts_offset(self):
        return self.ptr.block_starts_offset
    @property
    def block_starts_num(self):
        return self.ptr.block_starts_num
    @property
    def block_starts2_offset(self):
        return self.ptr.block_starts2_offset
    @property
    def block_starts2_num(self):
        return self.ptr.block_starts2_num
    @property
    def obj_c3_offset(self):
        return self.ptr.obj_c3_offset
    @property
    def obj_c3_num(self):
        return self.ptr.obj_c3_num
    @property
    def obj_c4_offset(self):
        return self.ptr.obj_c4_offset
    @property
    def obj_c4_num(self):
        return self.ptr.obj_c4_num
    @property
    def block_offset(self):
        return self.ptr.block_offset
    @property
    def block_size(self):
        return self.ptr.block_size
    @property
    def obj3_num(self):
        return self.ptr.obj3_num
    @property
    def obj3_offset(self):
        return self.ptr.obj3_offset
    @property
    def bones_num1(self):
        return self.ptr.bones_num1
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def obj1_num(self):
        return self.ptr.obj1_num
    @property
    def bones_offset(self):
        return self.ptr.bones_offset
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def obj1_offset(self):
        return self.ptr.obj1_offset
    @property
    def obj2_offset(self):
        return self.ptr.obj2_offset
    @property
    def obj2_num(self):
        return self.ptr.obj2_num
    @property
    def obj5_offset(self):
        return self.ptr.obj5_offset

cdef class ref_slice_Obj3Pc:
    cdef lotrc_rs.ref_slice_Obj3Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj3Pc()
        val.ptr = lotrc_rs.ref_slice_Obj3Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Obj3Pc_len(self.ptr)

cdef class Obj5HeaderPc:
    cdef lotrc_rs.Obj5HeaderPc* ptr
    @property
    def obj_a_num(self):
        return self.ptr.obj_a_num
    @property
    def obj_a_offset(self):
        return self.ptr.obj_a_offset
    @property
    def obj_b_num(self):
        return self.ptr.obj_b_num
    @property
    def obj_b_offset(self):
        return self.ptr.obj_b_offset

cdef class ref_slice_Obj5ValPc:
    cdef lotrc_rs.ref_slice_Obj5ValPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj5ValPc()
        val.ptr = lotrc_rs.ref_slice_Obj5ValPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Obj5ValPc_len(self.ptr)

cdef class Option_BlocksRefPc:
    cdef lotrc_rs.Option_BlocksRefPc* ptr
    def get(self):
        val = BlocksRefPc()
        val.ptr = lotrc_rs.Option_BlocksRefPc_get(self.ptr)
        return val

cdef class AnimationRefPc:
    cdef lotrc_rs.AnimationRefPc* ptr
    @property
    def info(self):
        val = AnimationInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def obj1(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.obj1
        return val
    @property
    def obj2(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.obj2
        return val
    @property
    def obj3(self):
        val = ref_slice_Obj3Pc()
        val.ptr = &self.ptr.obj3
        return val
    @property
    def bones(self):
        val = ref_slice_CrcPc()
        val.ptr = &self.ptr.bones
        return val
    @property
    def obj5_header(self):
        val = Obj5HeaderPc()
        val.ptr = self.ptr.obj5_header
        return val
    @property
    def obj5_a(self):
        val = ref_slice_Obj5ValPc()
        val.ptr = &self.ptr.obj5_a
        return val
    @property
    def obj5_b(self):
        val = ref_slice_Obj5ValPc()
        val.ptr = &self.ptr.obj5_b
        return val
    @property
    def blocks(self):
        val = Option_BlocksRefPc()
        val.ptr = &self.ptr.blocks
        return val
    @property
    def size(self):
        return self.ptr.size

cdef class DataRefPc:
    cdef lotrc_rs.DataRefPc* ptr
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val

cdef class EffectInfoPc:
    cdef lotrc_rs.EffectInfoPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size

cdef class EffectRefPc:
    cdef lotrc_rs.EffectRefPc* ptr
    @property
    def info(self):
        val = EffectInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def vals(self):
        val = GameObjsRefPc()
        val.ptr = &self.ptr.vals
        return val

cdef class IndexMap_u32__ref_slice_u16Pc:
    cdef lotrc_rs.IndexMap_u32__ref_slice_u16Pc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = ref_slice_u16Pc()
        val.ptr = lotrc_rs.IndexMap_u32__ref_slice_u16Pc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__ref_slice_u16Pc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__ref_slice_u16Pc_keys(self.ptr, keys.ptr)

cdef class LangStringsRefPc:
    cdef lotrc_rs.LangStringsRefPc* ptr
    @property
    def strings(self):
        val = IndexMap_u32__ref_slice_u16Pc()
        val.ptr = &self.ptr.strings
        return val

cdef class ObjHeaderPc:
    cdef lotrc_rs.ObjHeaderPc* ptr
    @property
    def layer(self):
        return self.ptr.layer
    @property
    def key(self):
        return self.ptr.key
    @property
    def size(self):
        return self.ptr.size
    @property
    def z3(self):
        return self.ptr.z3
    @property
    def z4(self):
        return self.ptr.z4

cdef class IndexMap_u32__BaseTypeRefPc:
    cdef lotrc_rs.IndexMap_u32__BaseTypeRefPc* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = BaseTypeRefPc()
        val.ptr = lotrc_rs.IndexMap_u32__BaseTypeRefPc_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__BaseTypeRefPc_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__BaseTypeRefPc_keys(self.ptr, keys.ptr)

cdef class ObjRefPc:
    cdef lotrc_rs.ObjRefPc* ptr
    @property
    def header(self):
        val = ObjHeaderPc()
        val.ptr = self.ptr.header
        return val
    @property
    def fields(self):
        val = IndexMap_u32__BaseTypeRefPc()
        val.ptr = &self.ptr.fields
        return val

cdef class RadiosityValsInfoPc:
    cdef lotrc_rs.RadiosityValsInfoPc* ptr
    @property
    def guid(self):
        return self.ptr.guid
    @property
    def num(self):
        return self.ptr.num
    @property
    def offset(self):
        return self.ptr.offset

cdef class RadiosityValsRefPc:
    cdef lotrc_rs.RadiosityValsRefPc* ptr
    @property
    def info(self):
        val = RadiosityValsInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def offs(self):
        val = ref_slice_i32Pc()
        val.ptr = &self.ptr.offs
        return val

cdef class ref_slice_SSAValPc:
    cdef lotrc_rs.ref_slice_SSAValPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SSAValPc()
        val.ptr = lotrc_rs.ref_slice_SSAValPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SSAValPc_len(self.ptr)

cdef class slice_ref_slice_u16Pc:
    cdef lotrc_rs.slice_ref_slice_u16Pc* ptr

cdef class SSARefPc:
    cdef lotrc_rs.SSARefPc* ptr
    @property
    def vals(self):
        val = ref_slice_SSAValPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def strings(self):
        val = slice_ref_slice_u16Pc()
        val.ptr = &self.ptr.strings
        return val

cdef class TextureInfoPc:
    cdef lotrc_rs.TextureInfoPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def asset_key(self):
        return self.ptr.asset_key
    @property
    def asset_type(self):
        return self.ptr.asset_type
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def format(self):
        return self.ptr.format
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def width(self):
        return self.ptr.width
    @property
    def height(self):
        return self.ptr.height
    @property
    def depth(self):
        return self.ptr.depth
    @property
    def levels(self):
        return self.ptr.levels
    @property
    def unk_16_1(self):
        return self.ptr.unk_16_1
    @property
    def unk_16_2(self):
        return self.ptr.unk_16_2
    @property
    def unk_16_3(self):
        return self.ptr.unk_16_3
    @property
    def unk_16_4(self):
        return self.ptr.unk_16_4
    @property
    def unk_16_5(self):
        return self.ptr.unk_16_5
    @property
    def unk_16_6(self):
        return self.ptr.unk_16_6
    @property
    def unk_16_7(self):
        return self.ptr.unk_16_7
    @property
    def unk_16_8(self):
        return self.ptr.unk_16_8
    @property
    def unk_16_9(self):
        return self.ptr.unk_16_9
    @property
    def unk_16_10(self):
        return self.ptr.unk_16_10
    @property
    def unk_16_11(self):
        return self.ptr.unk_16_11
    @property
    def unk_16_12(self):
        return self.ptr.unk_16_12
    @property
    def unk_16_13(self):
        return self.ptr.unk_16_13
    @property
    def unk_16_14(self):
        return self.ptr.unk_16_14
    @property
    def unk_16_15(self):
        return self.ptr.unk_16_15
    @property
    def unk_16_16(self):
        return self.ptr.unk_16_16

cdef class TextureRefPc:
    cdef lotrc_rs.TextureRefPc* ptr
    @property
    def info(self):
        val = TextureInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def data0(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data0
        return val
    @property
    def data1(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data1
        return val

cdef class TypeHeaderPc:
    cdef lotrc_rs.TypeHeaderPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def size(self):
        return self.ptr.size
    @property
    def fields(self):
        return self.ptr.fields

cdef class ref_slice_TypeFieldPc:
    cdef lotrc_rs.ref_slice_TypeFieldPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TypeFieldPc()
        val.ptr = lotrc_rs.ref_slice_TypeFieldPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_TypeFieldPc_len(self.ptr)

cdef class TypeRefPc:
    cdef lotrc_rs.TypeRefPc* ptr
    @property
    def header(self):
        val = TypeHeaderPc()
        val.ptr = self.ptr.header
        return val
    @property
    def fields(self):
        val = ref_slice_TypeFieldPc()
        val.ptr = &self.ptr.fields
        return val

cdef class slice_FoliageRefPc:
    cdef lotrc_rs.slice_FoliageRefPc* ptr

cdef class Vector2Pc:
    cdef lotrc_rs.Vector2Pc* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def y(self):
        return self.ptr.y

cdef class Vector4Pc:
    cdef lotrc_rs.Vector4Pc* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def y(self):
        return self.ptr.y
    @property
    def z(self):
        return self.ptr.z
    @property
    def w(self):
        return self.ptr.w

cdef class Matrix4x4Pc:
    cdef lotrc_rs.Matrix4x4Pc* ptr
    @property
    def x(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.x
        return val
    @property
    def y(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.y
        return val
    @property
    def z(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.z
        return val
    @property
    def w(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.w
        return val

cdef class ref_slice_U32Pc:
    cdef lotrc_rs.ref_slice_U32Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_U32Pc_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_U32Pc_len(self.ptr)

cdef class ref_slice_Vector4Pc:
    cdef lotrc_rs.ref_slice_Vector4Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Vector4Pc()
        val.ptr = lotrc_rs.ref_slice_Vector4Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Vector4Pc_len(self.ptr)

cdef class ref_slice_WeightPc:
    cdef lotrc_rs.ref_slice_WeightPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = WeightPc()
        val.ptr = lotrc_rs.ref_slice_WeightPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_WeightPc_len(self.ptr)

cdef class BaseTypeRefPc:
    cdef lotrc_rs.BaseTypeRefPc* ptr

cdef class BlockHeader1Pc:
    cdef lotrc_rs.BlockHeader1Pc* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3

cdef class BlockHeader2Pc:
    cdef lotrc_rs.BlockHeader2Pc* ptr
    @property
    def n(self):
        return self.ptr.n
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4

cdef class ref_slice_BlockValAPc:
    cdef lotrc_rs.ref_slice_BlockValAPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BlockValAPc()
        val.ptr = lotrc_rs.ref_slice_BlockValAPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BlockValAPc_len(self.ptr)

cdef class ref_slice_BlockValBPc:
    cdef lotrc_rs.ref_slice_BlockValBPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BlockValBPc()
        val.ptr = lotrc_rs.ref_slice_BlockValBPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BlockValBPc_len(self.ptr)

cdef class BlockRefPc:
    cdef lotrc_rs.BlockRefPc* ptr
    @property
    def info1(self):
        val = BlockHeader1Pc()
        val.ptr = self.ptr.info1
        return val
    @property
    def info2(self):
        val = BlockHeader2Pc()
        val.ptr = self.ptr.info2
        return val
    @property
    def vals_a(self):
        val = ref_slice_BlockValAPc()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def vals_b(self):
        val = ref_slice_BlockValAPc()
        val.ptr = &self.ptr.vals_b
        return val
    @property
    def vals_c(self):
        val = ref_slice_BlockValBPc()
        val.ptr = &self.ptr.vals_c
        return val
    @property
    def pad(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.pad
        return val

cdef class ShapeInfoPc:
    cdef lotrc_rs.ShapeInfoPc* ptr
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def translation(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def hk_shape_num(self):
        return self.ptr.hk_shape_num
    @property
    def hk_shape_offset(self):
        return self.ptr.hk_shape_offset
    @property
    def unk_29a(self):
        return self.ptr.unk_29a
    @property
    def unk_29b(self):
        return self.ptr.unk_29b
    @property
    def unk_29c(self):
        return self.ptr.unk_29c
    @property
    def unk_29d(self):
        return self.ptr.unk_29d
    @property
    def unk_30(self):
        return self.ptr.unk_30

cdef class Option_ShapeExtraRefPc:
    cdef lotrc_rs.Option_ShapeExtraRefPc* ptr
    def get(self):
        val = ShapeExtraRefPc()
        val.ptr = lotrc_rs.Option_ShapeExtraRefPc_get(self.ptr)
        return val

cdef class slice_HkShapeRefPc:
    cdef lotrc_rs.slice_HkShapeRefPc* ptr

cdef class ShapeRefPc:
    cdef lotrc_rs.ShapeRefPc* ptr
    @property
    def info(self):
        val = ShapeInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = Option_ShapeExtraRefPc()
        val.ptr = &self.ptr.extra
        return val
    @property
    def hk_shapes(self):
        val = slice_HkShapeRefPc()
        val.ptr = &self.ptr.hk_shapes
        return val

cdef class BoxShapePc:
    cdef lotrc_rs.BoxShapePc* ptr
    @property
    def translation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def half_extents(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.half_extents
        return val
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class SphereShapePc:
    cdef lotrc_rs.SphereShapePc* ptr
    @property
    def translation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def radius(self):
        return self.ptr.radius
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class CapsuleShapePc:
    cdef lotrc_rs.CapsuleShapePc* ptr
    @property
    def translation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def point1(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.point1
        return val
    @property
    def point2(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.point2
        return val
    @property
    def radius(self):
        return self.ptr.radius
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class CylinderShapePc:
    cdef lotrc_rs.CylinderShapePc* ptr
    @property
    def translation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def point1(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.point1
        return val
    @property
    def point2(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.point2
        return val
    @property
    def radius(self):
        return self.ptr.radius
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class ConvexVerticesInfoPc:
    cdef lotrc_rs.ConvexVerticesInfoPc* ptr
    @property
    def translation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def norm_num(self):
        return self.ptr.norm_num
    @property
    def norms_offset(self):
        return self.ptr.norms_offset
    @property
    def vert_num(self):
        return self.ptr.vert_num
    @property
    def verts_offset(self):
        return self.ptr.verts_offset
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class ref_slice_Vector3Pc:
    cdef lotrc_rs.ref_slice_Vector3Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Vector3Pc()
        val.ptr = lotrc_rs.ref_slice_Vector3Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Vector3Pc_len(self.ptr)

cdef class ConvexVerticesRefPc:
    cdef lotrc_rs.ConvexVerticesRefPc* ptr
    @property
    def info(self):
        val = ConvexVerticesInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def norms(self):
        val = ref_slice_Vector4Pc()
        val.ptr = &self.ptr.norms
        return val
    @property
    def verts(self):
        val = ref_slice_Vector3Pc()
        val.ptr = &self.ptr.verts
        return val

cdef class BVTreeMeshInfoPc:
    cdef lotrc_rs.BVTreeMeshInfoPc* ptr
    @property
    def translation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.offset
        return val
    @property
    def tree_scale(self):
        return self.ptr.tree_scale
    @property
    def tree_size(self):
        return self.ptr.tree_size
    @property
    def tree_offset(self):
        return self.ptr.tree_offset
    @property
    def vert_num(self):
        return self.ptr.vert_num
    @property
    def verts_offset(self):
        return self.ptr.verts_offset
    @property
    def tri_num(self):
        return self.ptr.tri_num
    @property
    def inds_offset(self):
        return self.ptr.inds_offset

cdef class BVTreeMeshRefPc:
    cdef lotrc_rs.BVTreeMeshRefPc* ptr
    @property
    def info(self):
        val = BVTreeMeshInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def tree(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.tree
        return val
    @property
    def verts(self):
        val = ref_slice_Vector3Pc()
        val.ptr = &self.ptr.verts
        return val
    @property
    def inds(self):
        val = ref_slice_u16Pc()
        val.ptr = &self.ptr.inds
        return val

cdef class HkShapeInfoPc:
    cdef lotrc_rs.HkShapeInfoPc* ptr
    @property
    def unk_0(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.unk_0
        return val
    @property
    def unk_4(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.unk_4
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class HkShapeRefPc:
    cdef lotrc_rs.HkShapeRefPc* ptr

cdef class FoliageInfoPc:
    cdef lotrc_rs.FoliageInfoPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def lb_w(self):
        return self.ptr.lb_w
    @property
    def lb_h(self):
        return self.ptr.lb_h
    @property
    def ub_w(self):
        return self.ptr.ub_w
    @property
    def ub_h(self):
        return self.ptr.ub_h
    @property
    def scale(self):
        return self.ptr.scale
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def key_mesh(self):
        return self.ptr.key_mesh
    @property
    def key_mesh_lod1(self):
        return self.ptr.key_mesh_lod1
    @property
    def key_mesh_lod2(self):
        return self.ptr.key_mesh_lod2
    @property
    def color(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.color
        return val
    @property
    def lod1a(self):
        return self.ptr.lod1a
    @property
    def lod1b(self):
        return self.ptr.lod1b
    @property
    def lod2a(self):
        return self.ptr.lod2a
    @property
    def lod2b(self):
        return self.ptr.lod2b
    @property
    def lod_max(self):
        return self.ptr.lod_max

cdef class ref_slice_FoliageValPc:
    cdef lotrc_rs.ref_slice_FoliageValPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = FoliageValPc()
        val.ptr = lotrc_rs.ref_slice_FoliageValPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_FoliageValPc_len(self.ptr)

cdef class FoliageRefPc:
    cdef lotrc_rs.FoliageRefPc* ptr
    @property
    def info(self):
        val = FoliageInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def vals(self):
        val = ref_slice_FoliageValPc()
        val.ptr = &self.ptr.vals
        return val

cdef class slice_BlockValARefPc:
    cdef lotrc_rs.slice_BlockValARefPc* ptr

cdef class slice_Obj1RefPc:
    cdef lotrc_rs.slice_Obj1RefPc* ptr

cdef class BlockValRefPc:
    cdef lotrc_rs.BlockValRefPc* ptr
    @property
    def vals_a(self):
        val = slice_BlockValARefPc()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def vals_b(self):
        val = slice_Obj1RefPc()
        val.ptr = &self.ptr.vals_b
        return val

cdef class slice_BlockValRefPc:
    cdef lotrc_rs.slice_BlockValRefPc* ptr

cdef class CrowdItemHeaderPc:
    cdef lotrc_rs.CrowdItemHeaderPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def key_main(self):
        return self.ptr.key_main
    @property
    def key_right(self):
        return self.ptr.key_right
    @property
    def key_left(self):
        return self.ptr.key_left
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def animation_num(self):
        return self.ptr.animation_num
    @property
    def instance_num(self):
        return self.ptr.instance_num

cdef class ref_slice_CrowdValPc:
    cdef lotrc_rs.ref_slice_CrowdValPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = CrowdValPc()
        val.ptr = lotrc_rs.ref_slice_CrowdValPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_CrowdValPc_len(self.ptr)

cdef class CrowdItemRefPc:
    cdef lotrc_rs.CrowdItemRefPc* ptr
    @property
    def header(self):
        val = CrowdItemHeaderPc()
        val.ptr = self.ptr.header
        return val
    @property
    def animations(self):
        val = ref_slice_CrcPc()
        val.ptr = &self.ptr.animations
        return val
    @property
    def instances(self):
        val = ref_slice_CrowdValPc()
        val.ptr = &self.ptr.instances
        return val

cdef class slice_CrowdItemRefPc:
    cdef lotrc_rs.slice_CrowdItemRefPc* ptr

cdef class HkConstraintBoneRefPc:
    cdef lotrc_rs.HkConstraintBoneRefPc* ptr
    @property
    def name(self):
        val = string()
        val.ptr = &self.ptr.name
        return val
    @property
    def start(self):
        return self.ptr.start
    @property
    def val(self):
        return self.ptr.val

cdef class slice_HkConstraintBoneRefPc:
    cdef lotrc_rs.slice_HkConstraintBoneRefPc* ptr

cdef class ref_slice_f32Pc:
    cdef lotrc_rs.ref_slice_f32Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_f32Pc_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_f32Pc_len(self.ptr)

cdef class AnimVals1RefPc:
    cdef lotrc_rs.AnimVals1RefPc* ptr

cdef class Obj1RefPc:
    cdef lotrc_rs.Obj1RefPc* ptr
    @property
    def flags(self):
        return self.ptr.flags
    @property
    def s2(self):
        return self.ptr.s2
    @property
    def s1(self):
        return self.ptr.s1
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val
    @property
    def vals_a(self):
        val = ref_slice_f32Pc()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def vals(self):
        val = AnimVals1RefPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def size(self):
        return self.ptr.size

cdef class ref_slice_RotationPolar32Pc:
    cdef lotrc_rs.ref_slice_RotationPolar32Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationPolar32Pc()
        val.ptr = lotrc_rs.ref_slice_RotationPolar32Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationPolar32Pc_len(self.ptr)

cdef class ref_slice_RotationThreeComp40Pc:
    cdef lotrc_rs.ref_slice_RotationThreeComp40Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationThreeComp40Pc()
        val.ptr = lotrc_rs.ref_slice_RotationThreeComp40Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationThreeComp40Pc_len(self.ptr)

cdef class ref_slice_RotationThreeComp48Pc:
    cdef lotrc_rs.ref_slice_RotationThreeComp48Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationThreeComp48Pc()
        val.ptr = lotrc_rs.ref_slice_RotationThreeComp48Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationThreeComp48Pc_len(self.ptr)

cdef class ref_slice_RotationThreeComp24Pc:
    cdef lotrc_rs.ref_slice_RotationThreeComp24Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationThreeComp24Pc()
        val.ptr = lotrc_rs.ref_slice_RotationThreeComp24Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationThreeComp24Pc_len(self.ptr)

cdef class ref_slice_RotationStraight16Pc:
    cdef lotrc_rs.ref_slice_RotationStraight16Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationStraight16Pc()
        val.ptr = lotrc_rs.ref_slice_RotationStraight16Pc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationStraight16Pc_len(self.ptr)

cdef class ref_slice_RotationUncompressedPc:
    cdef lotrc_rs.ref_slice_RotationUncompressedPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationUncompressedPc()
        val.ptr = lotrc_rs.ref_slice_RotationUncompressedPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationUncompressedPc_len(self.ptr)

cdef class RotationQuantizationRefPc:
    cdef lotrc_rs.RotationQuantizationRefPc* ptr

cdef class Obj2RefPc:
    cdef lotrc_rs.Obj2RefPc* ptr
    @property
    def flags(self):
        return self.ptr.flags
    @property
    def s2(self):
        return self.ptr.s2
    @property
    def s1(self):
        return self.ptr.s1
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val
    @property
    def vals(self):
        val = RotationQuantizationRefPc()
        val.ptr = &self.ptr.vals
        return val
    @property
    def size(self):
        return self.ptr.size

cdef class BlockValARefPc:
    cdef lotrc_rs.BlockValARefPc* ptr
    @property
    def a(self):
        val = Obj1RefPc()
        val.ptr = &self.ptr.a
        return val
    @property
    def b(self):
        val = Obj2RefPc()
        val.ptr = &self.ptr.b
        return val
    @property
    def c(self):
        val = Obj1RefPc()
        val.ptr = &self.ptr.c
        return val

cdef class BufferInfoPc:
    cdef lotrc_rs.BufferInfoPc* ptr
    @property
    def vbuff_info_offset(self):
        return self.ptr.vbuff_info_offset
    @property
    def vbuff_info_offset_2(self):
        return self.ptr.vbuff_info_offset_2
    @property
    def vbuff_info_offset_3(self):
        return self.ptr.vbuff_info_offset_3
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def v_size(self):
        return self.ptr.v_size
    @property
    def v_size_2(self):
        return self.ptr.v_size_2
    @property
    def v_size_3(self):
        return self.ptr.v_size_3
    @property
    def unk_35(self):
        return self.ptr.unk_35
    @property
    def unk_36(self):
        return self.ptr.unk_36
    @property
    def unk_37(self):
        return self.ptr.unk_37
    @property
    def unk_38(self):
        return self.ptr.unk_38
    @property
    def unk_39(self):
        return self.ptr.unk_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42
    @property
    def unk_43(self):
        return self.ptr.unk_43
    @property
    def unk_44(self):
        return self.ptr.unk_44
    @property
    def unk_45(self):
        return self.ptr.unk_45
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def unk_47(self):
        return self.ptr.unk_47
    @property
    def vbuff_size(self):
        return self.ptr.vbuff_size
    @property
    def vbuff_size_2(self):
        return self.ptr.vbuff_size_2
    @property
    def vbuff_size_3(self):
        return self.ptr.vbuff_size_3
    @property
    def unk_51(self):
        return self.ptr.unk_51
    @property
    def unk_52(self):
        return self.ptr.unk_52
    @property
    def unk_53(self):
        return self.ptr.unk_53
    @property
    def unk_54(self):
        return self.ptr.unk_54
    @property
    def unk_55(self):
        return self.ptr.unk_55
    @property
    def unk_56(self):
        return self.ptr.unk_56
    @property
    def unk_57(self):
        return self.ptr.unk_57
    @property
    def unk_58(self):
        return self.ptr.unk_58
    @property
    def unk_59(self):
        return self.ptr.unk_59
    @property
    def unk_60(self):
        return self.ptr.unk_60
    @property
    def unk_61(self):
        return self.ptr.unk_61
    @property
    def unk_62(self):
        return self.ptr.unk_62
    @property
    def unk_63(self):
        return self.ptr.unk_63
    @property
    def unk_64(self):
        return self.ptr.unk_64
    @property
    def ibuff_info_offset(self):
        return self.ptr.ibuff_info_offset
    @property
    def i_num(self):
        return self.ptr.i_num
    @property
    def unk_67(self):
        return self.ptr.unk_67
    @property
    def skin_offset(self):
        return self.ptr.skin_offset
    @property
    def skin_size(self):
        return self.ptr.skin_size
    @property
    def unk_70(self):
        return self.ptr.unk_70
    @property
    def tri_num(self):
        return self.ptr.tri_num
    @property
    def unk_72(self):
        return self.ptr.unk_72
    @property
    def unk_73(self):
        return self.ptr.unk_73
    @property
    def unk_74(self):
        return self.ptr.unk_74
    @property
    def unk_75(self):
        return self.ptr.unk_75
    @property
    def unk_76(self):
        return self.ptr.unk_76
    @property
    def unk_77(self):
        return self.ptr.unk_77
    @property
    def unk_78(self):
        return self.ptr.unk_78
    @property
    def unk_79(self):
        return self.ptr.unk_79
    @property
    def unk_80(self):
        return self.ptr.unk_80
    @property
    def unk_81(self):
        return self.ptr.unk_81
    @property
    def unk_82(self):
        return self.ptr.unk_82
    @property
    def unk_83(self):
        return self.ptr.unk_83
    @property
    def unk_84(self):
        return self.ptr.unk_84
    @property
    def unk_85(self):
        return self.ptr.unk_85
    @property
    def unk_86(self):
        return self.ptr.unk_86
    @property
    def unk_87(self):
        return self.ptr.unk_87
    @property
    def variation_id(self):
        return self.ptr.variation_id
    @property
    def variation(self):
        return self.ptr.variation
    @property
    def unk_88c(self):
        return self.ptr.unk_88c
    @property
    def unk_88d(self):
        return self.ptr.unk_88d

cdef class HkConstraintDataPc:
    cdef lotrc_rs.HkConstraintDataPc* ptr
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28

cdef class Key2Pc:
    cdef lotrc_rs.Key2Pc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def val(self):
        return self.ptr.val

cdef class BlockValAPc:
    cdef lotrc_rs.BlockValAPc* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11

cdef class BlockValBPc:
    cdef lotrc_rs.BlockValBPc* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5

cdef class AnimationBlockInfoPc:
    cdef lotrc_rs.AnimationBlockInfoPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def guid(self):
        return self.ptr.guid
    @property
    def key_name(self):
        return self.ptr.key_name
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size
    @property
    def size_comp(self):
        return self.ptr.size_comp
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8

cdef class AssetHandlePc:
    cdef lotrc_rs.AssetHandlePc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size
    @property
    def size_comp(self):
        return self.ptr.size_comp
    @property
    def kind(self):
        return self.ptr.kind

cdef class BlockAValPc:
    cdef lotrc_rs.BlockAValPc* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def key(self):
        return self.ptr.key
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6

cdef class GFXBlockInfoPc:
    cdef lotrc_rs.GFXBlockInfoPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size

cdef class HkConstraintInfoPc:
    cdef lotrc_rs.HkConstraintInfoPc* ptr
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def bone_parents_offset(self):
        return self.ptr.bone_parents_offset
    @property
    def bone_parents_num(self):
        return self.ptr.bone_parents_num
    @property
    def bone_names_offset(self):
        return self.ptr.bone_names_offset
    @property
    def bone_names_num(self):
        return self.ptr.bone_names_num
    @property
    def bone_transforms_offset(self):
        return self.ptr.bone_transforms_offset
    @property
    def bone_transforms_num(self):
        return self.ptr.bone_transforms_num
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def bones_offset(self):
        return self.ptr.bones_offset
    @property
    def bones_num(self):
        return self.ptr.bones_num
    @property
    def bone_order_num(self):
        return self.ptr.bone_order_num
    @property
    def bone_order_offset(self):
        return self.ptr.bone_order_offset
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def vals2_num(self):
        return self.ptr.vals2_num
    @property
    def vals2_offset(self):
        return self.ptr.vals2_offset
    @property
    def unk_17(self):
        return self.ptr.unk_17

cdef class Obj0Pc:
    cdef lotrc_rs.Obj0Pc* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def key(self):
        return self.ptr.key

cdef class ObjAPc:
    cdef lotrc_rs.ObjAPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def size(self):
        return self.ptr.size
    @property
    def size_comp(self):
        return self.ptr.size_comp
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def kind(self):
        return self.ptr.kind

cdef class PFieldInfoPc:
    cdef lotrc_rs.PFieldInfoPc* ptr
    @property
    def link_guid(self):
        return self.ptr.link_guid
    @property
    def gamemode_guid(self):
        return self.ptr.gamemode_guid
    @property
    def width(self):
        return self.ptr.width
    @property
    def height(self):
        return self.ptr.height
    @property
    def offset(self):
        return self.ptr.offset

cdef class StringKeysValPc:
    cdef lotrc_rs.StringKeysValPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset

cdef class SubBlocksBlockHeaderPc:
    cdef lotrc_rs.SubBlocksBlockHeaderPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size

cdef class Obj3Pc:
    cdef lotrc_rs.Obj3Pc* ptr
    @property
    def t(self):
        return self.ptr.t
    @property
    def event(self):
        return self.ptr.event
    @property
    def dat_2(self):
        return self.ptr.dat_2
    @property
    def dat_3(self):
        return self.ptr.dat_3
    @property
    def dat_4(self):
        return self.ptr.dat_4
    @property
    def dat_5(self):
        return self.ptr.dat_5
    @property
    def dat_6(self):
        return self.ptr.dat_6
    @property
    def dat_7(self):
        return self.ptr.dat_7
    @property
    def dat_8(self):
        return self.ptr.dat_8
    @property
    def dat_9(self):
        return self.ptr.dat_9
    @property
    def dat_10(self):
        return self.ptr.dat_10

cdef class Obj5ValPc:
    cdef lotrc_rs.Obj5ValPc* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6

cdef class SSAValPc:
    cdef lotrc_rs.SSAValPc* ptr
    @property
    def t_start(self):
        return self.ptr.t_start
    @property
    def t_end(self):
        return self.ptr.t_end
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def off(self):
        return self.ptr.off

cdef class TypeFieldPc:
    cdef lotrc_rs.TypeFieldPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def offset(self):
        return self.ptr.offset

cdef class FoliageValPc:
    cdef lotrc_rs.FoliageValPc* ptr
    @property
    def height(self):
        return self.ptr.height
    @property
    def var_mask(self):
        return self.ptr.var_mask
    @property
    def slope_x(self):
        return self.ptr.slope_x
    @property
    def slope_z(self):
        return self.ptr.slope_z

cdef class WeightPc:
    cdef lotrc_rs.WeightPc* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c
    @property
    def d(self):
        return self.ptr.d

cdef class AtlasUVValPc:
    cdef lotrc_rs.AtlasUVValPc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def vals(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.vals
        return val

cdef class ref_slice_AtlasUVValPc:
    cdef lotrc_rs.ref_slice_AtlasUVValPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AtlasUVValPc()
        val.ptr = lotrc_rs.ref_slice_AtlasUVValPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AtlasUVValPc_len(self.ptr)

cdef class SprayInstancePc:
    cdef lotrc_rs.SprayInstancePc* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def tex1(self):
        return self.ptr.tex1
    @property
    def tex2(self):
        return self.ptr.tex2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def width(self):
        return self.ptr.width
    @property
    def height(self):
        return self.ptr.height
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def size_w(self):
        return self.ptr.size_w
    @property
    def size_h(self):
        return self.ptr.size_h
    @property
    def scale_w(self):
        return self.ptr.scale_w
    @property
    def scale_h(self):
        return self.ptr.scale_h
    @property
    def delay(self):
        return self.ptr.delay
    @property
    def stride_x(self):
        return self.ptr.stride_x
    @property
    def stride_y(self):
        return self.ptr.stride_y
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16

cdef class ref_slice_SprayInstancePc:
    cdef lotrc_rs.ref_slice_SprayInstancePc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SprayInstancePc()
        val.ptr = lotrc_rs.ref_slice_SprayInstancePc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SprayInstancePc_len(self.ptr)

cdef class SprayValPc:
    cdef lotrc_rs.SprayValPc* ptr
    @property
    def position(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.position
        return val
    @property
    def scale(self):
        return self.ptr.scale
    @property
    def instance(self):
        return self.ptr.instance
    @property
    def rotation(self):
        return self.ptr.rotation

cdef class ref_slice_SprayValPc:
    cdef lotrc_rs.ref_slice_SprayValPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SprayValPc()
        val.ptr = lotrc_rs.ref_slice_SprayValPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SprayValPc_len(self.ptr)

cdef class TRSPc:
    cdef lotrc_rs.TRSPc* ptr
    @property
    def translation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def scale(self):
        val = Vector4Pc()
        val.ptr = &self.ptr.scale
        return val

cdef class ref_slice_TRSPc:
    cdef lotrc_rs.ref_slice_TRSPc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TRSPc()
        val.ptr = lotrc_rs.ref_slice_TRSPc_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_TRSPc_len(self.ptr)

cdef class ref_slice_i16Pc:
    cdef lotrc_rs.ref_slice_i16Pc* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_i16Pc_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_i16Pc_len(self.ptr)

cdef class CrowdValPc:
    cdef lotrc_rs.CrowdValPc* ptr
    @property
    def position(self):
        val = Vector3Pc()
        val.ptr = &self.ptr.position
        return val
    @property
    def rotation(self):
        return self.ptr.rotation
    @property
    def lod(self):
        return self.ptr.lod

cdef class RotationPolar32Pc:
    cdef lotrc_rs.RotationPolar32Pc* ptr
    @property
    def a(self):
        return self.ptr.a

cdef class RotationStraight16Pc:
    cdef lotrc_rs.RotationStraight16Pc* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b

cdef class RotationThreeComp24Pc:
    cdef lotrc_rs.RotationThreeComp24Pc* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c

cdef class RotationThreeComp40Pc:
    cdef lotrc_rs.RotationThreeComp40Pc* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c
    @property
    def d(self):
        return self.ptr.d
    @property
    def e(self):
        return self.ptr.e

cdef class RotationThreeComp48Pc:
    cdef lotrc_rs.RotationThreeComp48Pc* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c

cdef class RotationUncompressedPc:
    cdef lotrc_rs.RotationUncompressedPc* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c
    @property
    def d(self):
        return self.ptr.d

cdef class HkConstraintRefPc:
    cdef lotrc_rs.HkConstraintRefPc* ptr
    @property
    def info(self):
        val = HkConstraintInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def bone_parents(self):
        val = ref_slice_i16Pc()
        val.ptr = &self.ptr.bone_parents
        return val
    @property
    def bone_names(self):
        val = slice_HkConstraintBoneRefPc()
        val.ptr = &self.ptr.bone_names
        return val
    @property
    def name_offsets(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.name_offsets
        return val
    @property
    def bone_transforms(self):
        val = ref_slice_TRSPc()
        val.ptr = &self.ptr.bone_transforms
        return val
    @property
    def bones(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.bones
        return val
    @property
    def bones_order(self):
        val = ref_slice_Key2Pc()
        val.ptr = &self.ptr.bones_order
        return val
    @property
    def vals2(self):
        val = ref_slice_f32Pc()
        val.ptr = &self.ptr.vals2
        return val

cdef class ShapeExtraInfoPc:
    cdef lotrc_rs.ShapeExtraInfoPc* ptr
    @property
    def size(self):
        return self.ptr.size
    @property
    def scale(self):
        return self.ptr.scale
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b

cdef class ShapeExtraRefPc:
    cdef lotrc_rs.ShapeExtraRefPc* ptr
    @property
    def info(self):
        val = ShapeExtraInfoPc()
        val.ptr = self.ptr.info
        return val
    @property
    def offs(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.offs
        return val
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val

cdef class AtlasUVRefPc:
    cdef lotrc_rs.AtlasUVRefPc* ptr
    @property
    def vals(self):
        val = ref_slice_AtlasUVValPc()
        val.ptr = &self.ptr.vals
        return val

cdef class BlocksRefPc:
    cdef lotrc_rs.BlocksRefPc* ptr
    @property
    def block_starts(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.block_starts
        return val
    @property
    def block_starts2(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.block_starts2
        return val
    @property
    def obj_c3(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.obj_c3
        return val
    @property
    def obj_c4(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.obj_c4
        return val
    @property
    def blocks(self):
        val = slice_BlockValRefPc()
        val.ptr = &self.ptr.blocks
        return val

cdef class CrowdHeaderPc:
    cdef lotrc_rs.CrowdHeaderPc* ptr
    @property
    def const0x65(self):
        return self.ptr.const0x65
    @property
    def n(self):
        return self.ptr.n

cdef class CrowdRefPc:
    cdef lotrc_rs.CrowdRefPc* ptr
    @property
    def header(self):
        val = CrowdHeaderPc()
        val.ptr = self.ptr.header
        return val
    @property
    def offs(self):
        val = ref_slice_u32Pc()
        val.ptr = &self.ptr.offs
        return val
    @property
    def vals(self):
        val = slice_CrowdItemRefPc()
        val.ptr = &self.ptr.vals
        return val

cdef class SprayRefPc:
    cdef lotrc_rs.SprayRefPc* ptr
    @property
    def instances(self):
        val = ref_slice_SprayInstancePc()
        val.ptr = &self.ptr.instances
        return val
    @property
    def vals(self):
        val = ref_slice_SprayValPc()
        val.ptr = &self.ptr.vals
        return val

cdef class IBuffInfoXbox:
    cdef lotrc_rs.IBuffInfoXbox* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def size(self):
        return self.ptr.size
    @property
    def format(self):
        return self.ptr.format
    @property
    def vbuff_alt_fmt(self):
        return self.ptr.vbuff_alt_fmt
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12

cdef class ref_slice_u16Xbox:
    cdef lotrc_rs.ref_slice_u16Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_u16Xbox_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_u16Xbox_len(self.ptr)

cdef class IndexBufferValsRefXbox:
    cdef lotrc_rs.IndexBufferValsRefXbox* ptr

cdef class IndexBufferRefXbox:
    cdef lotrc_rs.IndexBufferRefXbox* ptr
    @property
    def info(self):
        val = IBuffInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def vals(self):
        val = IndexBufferValsRefXbox()
        val.ptr = &self.ptr.vals
        return val

cdef class IndexMap_u32__IndexBufferRefXbox:
    cdef lotrc_rs.IndexMap_u32__IndexBufferRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = IndexBufferRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__IndexBufferRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__IndexBufferRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__IndexBufferRefXbox_keys(self.ptr, keys.ptr)

cdef class VBuffInfoXbox:
    cdef lotrc_rs.VBuffInfoXbox* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def size(self):
        return self.ptr.size
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def fmt2(self):
        return self.ptr.fmt2
    @property
    def fmt1(self):
        return self.ptr.fmt1
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13

cdef class VertexBufferRefXbox:
    cdef lotrc_rs.VertexBufferRefXbox* ptr
    @property
    def info(self):
        val = VBuffInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def offsets(self):
        val = IndexMap_VertexUsage__VertexDataIndex()
        val.ptr = &self.ptr.offsets
        return val
    @property
    def size(self):
        return self.ptr.size
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val

cdef class IndexMap_u32__VertexBufferRefXbox:
    cdef lotrc_rs.IndexMap_u32__VertexBufferRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = VertexBufferRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__VertexBufferRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__VertexBufferRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__VertexBufferRefXbox_keys(self.ptr, keys.ptr)

cdef class MatBaseXbox:
    cdef lotrc_rs.MatBaseXbox* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def tex0(self):
        return self.ptr.tex0
    @property
    def tex1(self):
        return self.ptr.tex1
    @property
    def tex2(self):
        return self.ptr.tex2
    @property
    def tex3(self):
        return self.ptr.tex3
    @property
    def tex4(self):
        return self.ptr.tex4
    @property
    def tex5(self):
        return self.ptr.tex5
    @property
    def key_guid(self):
        return self.ptr.key_guid
    @property
    def mask0(self):
        return self.ptr.mask0
    @property
    def mask1(self):
        return self.ptr.mask1
    @property
    def mask2(self):
        return self.ptr.mask2
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def unk_33(self):
        return self.ptr.unk_33
    @property
    def z_34(self):
        return self.ptr.z_34
    @property
    def z_35(self):
        return self.ptr.z_35
    @property
    def z_36(self):
        return self.ptr.z_36
    @property
    def z_37(self):
        return self.ptr.z_37
    @property
    def z_38(self):
        return self.ptr.z_38
    @property
    def z_39(self):
        return self.ptr.z_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42
    @property
    def unk_43(self):
        return self.ptr.unk_43
    @property
    def unk_44(self):
        return self.ptr.unk_44
    @property
    def unk_45(self):
        return self.ptr.unk_45
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def unk_47(self):
        return self.ptr.unk_47
    @property
    def unk_48(self):
        return self.ptr.unk_48
    @property
    def unk_49(self):
        return self.ptr.unk_49
    @property
    def flags(self):
        return self.ptr.flags
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_53(self):
        return self.ptr.unk_53
    @property
    def unk_54a(self):
        return self.ptr.unk_54a
    @property
    def unk_54b(self):
        return self.ptr.unk_54b
    @property
    def side_flags(self):
        return self.ptr.side_flags
    @property
    def unk_55(self):
        return self.ptr.unk_55
    @property
    def unk_56(self):
        return self.ptr.unk_56
    @property
    def unk_57(self):
        return self.ptr.unk_57
    @property
    def unk_58(self):
        return self.ptr.unk_58
    @property
    def unk_59(self):
        return self.ptr.unk_59
    @property
    def unk_60(self):
        return self.ptr.unk_60
    @property
    def unk_61(self):
        return self.ptr.unk_61
    @property
    def unk_62(self):
        return self.ptr.unk_62
    @property
    def unk_63(self):
        return self.ptr.unk_63
    @property
    def unk_64(self):
        return self.ptr.unk_64
    @property
    def unk_65(self):
        return self.ptr.unk_65
    @property
    def unk_66(self):
        return self.ptr.unk_66
    @property
    def unk_67(self):
        return self.ptr.unk_67
    @property
    def unk_68(self):
        return self.ptr.unk_68
    @property
    def unk_69(self):
        return self.ptr.unk_69
    @property
    def unk_70(self):
        return self.ptr.unk_70
    @property
    def unk_71(self):
        return self.ptr.unk_71
    @property
    def unk_72(self):
        return self.ptr.unk_72
    @property
    def unk_73(self):
        return self.ptr.unk_73
    @property
    def unk_74(self):
        return self.ptr.unk_74
    @property
    def unk_75(self):
        return self.ptr.unk_75
    @property
    def unk_76(self):
        return self.ptr.unk_76
    @property
    def unk_77(self):
        return self.ptr.unk_77
    @property
    def unk_78(self):
        return self.ptr.unk_78
    @property
    def unk_79(self):
        return self.ptr.unk_79
    @property
    def unk_80(self):
        return self.ptr.unk_80
    @property
    def unk_81(self):
        return self.ptr.unk_81
    @property
    def unk_82(self):
        return self.ptr.unk_82
    @property
    def unk_83(self):
        return self.ptr.unk_83
    @property
    def unk_84(self):
        return self.ptr.unk_84
    @property
    def unk_85(self):
        return self.ptr.unk_85
    @property
    def mat_extra_offset(self):
        return self.ptr.mat_extra_offset
    @property
    def key(self):
        return self.ptr.key
    @property
    def unk_88(self):
        return self.ptr.unk_88
    @property
    def z_89(self):
        return self.ptr.z_89

cdef class Mat1Xbox:
    cdef lotrc_rs.Mat1Xbox* ptr
    @property
    def base(self):
        val = MatBaseXbox()
        val.ptr = &self.ptr.base
        return val

cdef class MatExtraXbox:
    cdef lotrc_rs.MatExtraXbox* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def unk_33(self):
        return self.ptr.unk_33
    @property
    def unk_34(self):
        return self.ptr.unk_34
    @property
    def unk_35(self):
        return self.ptr.unk_35
    @property
    def unk_36(self):
        return self.ptr.unk_36
    @property
    def unk_37(self):
        return self.ptr.unk_37
    @property
    def unk_38(self):
        return self.ptr.unk_38
    @property
    def unk_39(self):
        return self.ptr.unk_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42
    @property
    def unk_43(self):
        return self.ptr.unk_43
    @property
    def unk_44(self):
        return self.ptr.unk_44
    @property
    def unk_45(self):
        return self.ptr.unk_45
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def unk_47(self):
        return self.ptr.unk_47
    @property
    def unk_48(self):
        return self.ptr.unk_48
    @property
    def unk_49(self):
        return self.ptr.unk_49

cdef class Mat1RefXbox:
    cdef lotrc_rs.Mat1RefXbox* ptr
    @property
    def info(self):
        val = Mat1Xbox()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraXbox()
        val.ptr = self.ptr.extra
        return val

cdef class Mat2Xbox:
    cdef lotrc_rs.Mat2Xbox* ptr
    @property
    def base(self):
        val = MatBaseXbox()
        val.ptr = &self.ptr.base
        return val
    @property
    def unk_90(self):
        return self.ptr.unk_90
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def unk_114(self):
        return self.ptr.unk_114
    @property
    def unk_115(self):
        return self.ptr.unk_115
    @property
    def unk_116(self):
        return self.ptr.unk_116
    @property
    def unk_117(self):
        return self.ptr.unk_117
    @property
    def unk_118(self):
        return self.ptr.unk_118
    @property
    def unk_119(self):
        return self.ptr.unk_119
    @property
    def unk_120a(self):
        return self.ptr.unk_120a
    @property
    def unk_120b(self):
        return self.ptr.unk_120b
    @property
    def unk_120c(self):
        return self.ptr.unk_120c
    @property
    def unk_120d(self):
        return self.ptr.unk_120d
    @property
    def unk_121(self):
        return self.ptr.unk_121

cdef class Mat2RefXbox:
    cdef lotrc_rs.Mat2RefXbox* ptr
    @property
    def info(self):
        val = Mat2Xbox()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraXbox()
        val.ptr = self.ptr.extra
        return val

cdef class Mat3Xbox:
    cdef lotrc_rs.Mat3Xbox* ptr
    @property
    def base(self):
        val = MatBaseXbox()
        val.ptr = &self.ptr.base
        return val
    @property
    def unk_90(self):
        return self.ptr.unk_90
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def variation_id_color(self):
        return self.ptr.variation_id_color
    @property
    def variation_id_texture(self):
        return self.ptr.variation_id_texture
    @property
    def variation_id_specular(self):
        return self.ptr.variation_id_specular
    @property
    def unk_114d(self):
        return self.ptr.unk_114d
    @property
    def unk_115(self):
        return self.ptr.unk_115

cdef class Mat3RefXbox:
    cdef lotrc_rs.Mat3RefXbox* ptr
    @property
    def info(self):
        val = Mat3Xbox()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraXbox()
        val.ptr = self.ptr.extra
        return val

cdef class Mat4Xbox:
    cdef lotrc_rs.Mat4Xbox* ptr
    @property
    def base(self):
        val = MatBaseXbox()
        val.ptr = &self.ptr.base
        return val
    @property
    def unk_90(self):
        return self.ptr.unk_90
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def unk_114(self):
        return self.ptr.unk_114
    @property
    def unk_115(self):
        return self.ptr.unk_115
    @property
    def unk_116(self):
        return self.ptr.unk_116
    @property
    def unk_117(self):
        return self.ptr.unk_117
    @property
    def unk_118(self):
        return self.ptr.unk_118
    @property
    def unk_119(self):
        return self.ptr.unk_119
    @property
    def unk_120(self):
        return self.ptr.unk_120
    @property
    def unk_121(self):
        return self.ptr.unk_121
    @property
    def unk_122(self):
        return self.ptr.unk_122
    @property
    def unk_123(self):
        return self.ptr.unk_123
    @property
    def unk_124(self):
        return self.ptr.unk_124
    @property
    def unk_125(self):
        return self.ptr.unk_125
    @property
    def unk_126(self):
        return self.ptr.unk_126
    @property
    def unk_127(self):
        return self.ptr.unk_127
    @property
    def unk_128(self):
        return self.ptr.unk_128
    @property
    def unk_129(self):
        return self.ptr.unk_129
    @property
    def unk_130(self):
        return self.ptr.unk_130
    @property
    def unk_131(self):
        return self.ptr.unk_131
    @property
    def unk_132(self):
        return self.ptr.unk_132
    @property
    def unk_133(self):
        return self.ptr.unk_133
    @property
    def unk_134(self):
        return self.ptr.unk_134
    @property
    def unk_135(self):
        return self.ptr.unk_135
    @property
    def unk_136(self):
        return self.ptr.unk_136
    @property
    def unk_137(self):
        return self.ptr.unk_137
    @property
    def unk_138(self):
        return self.ptr.unk_138
    @property
    def unk_139(self):
        return self.ptr.unk_139
    @property
    def unk_140(self):
        return self.ptr.unk_140
    @property
    def unk_141(self):
        return self.ptr.unk_141
    @property
    def unk_142(self):
        return self.ptr.unk_142
    @property
    def unk_143(self):
        return self.ptr.unk_143
    @property
    def unk_144(self):
        return self.ptr.unk_144
    @property
    def unk_145(self):
        return self.ptr.unk_145

cdef class Mat4RefXbox:
    cdef lotrc_rs.Mat4RefXbox* ptr
    @property
    def info(self):
        val = Mat4Xbox()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraXbox()
        val.ptr = self.ptr.extra
        return val

cdef class MatRefXbox:
    cdef lotrc_rs.MatRefXbox* ptr

cdef class IndexMap_u32__MatRefXbox:
    cdef lotrc_rs.IndexMap_u32__MatRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = MatRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__MatRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__MatRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__MatRefXbox_keys(self.ptr, keys.ptr)

cdef class Vector3Xbox:
    cdef lotrc_rs.Vector3Xbox* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def y(self):
        return self.ptr.y
    @property
    def z(self):
        return self.ptr.z

cdef class BoundingBoxXbox:
    cdef lotrc_rs.BoundingBoxXbox* ptr
    @property
    def center(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.center
        return val
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def half_width(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.half_width
        return val
    @property
    def unk_7(self):
        return self.ptr.unk_7

cdef class LodInfoXbox:
    cdef lotrc_rs.LodInfoXbox* ptr
    @property
    def start(self):
        return self.ptr.start
    @property
    def static_end(self):
        return self.ptr.static_end
    @property
    def skinned_end(self):
        return self.ptr.skinned_end
    @property
    def physics_end(self):
        return self.ptr.physics_end
    @property
    def breakable_end(self):
        return self.ptr.breakable_end

cdef class ModelInfoXbox:
    cdef lotrc_rs.ModelInfoXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def mat_offset(self):
        return self.ptr.mat_offset
    @property
    def buffer_info_offset(self):
        return self.ptr.buffer_info_offset
    @property
    def bounding_box(self):
        val = BoundingBoxXbox()
        val.ptr = &self.ptr.bounding_box
        return val
    @property
    def mesh_order_offset(self):
        return self.ptr.mesh_order_offset
    @property
    def lod0(self):
        val = LodInfoXbox()
        val.ptr = &self.ptr.lod0
        return val
    @property
    def lod1(self):
        val = LodInfoXbox()
        val.ptr = &self.ptr.lod1
        return val
    @property
    def lod2(self):
        val = LodInfoXbox()
        val.ptr = &self.ptr.lod2
        return val
    @property
    def lod3(self):
        val = LodInfoXbox()
        val.ptr = &self.ptr.lod3
        return val
    @property
    def mat_num(self):
        return self.ptr.mat_num
    @property
    def bones_offset(self):
        return self.ptr.bones_offset
    @property
    def bone_parents_offset(self):
        return self.ptr.bone_parents_offset
    @property
    def bone_transforms_offset(self):
        return self.ptr.bone_transforms_offset
    @property
    def bones_num(self):
        return self.ptr.bones_num
    @property
    def skin_binds_offset(self):
        return self.ptr.skin_binds_offset
    @property
    def skin_binds_num(self):
        return self.ptr.skin_binds_num
    @property
    def skin_order_offset(self):
        return self.ptr.skin_order_offset
    @property
    def vbuff_offset(self):
        return self.ptr.vbuff_offset
    @property
    def vbuff_num(self):
        return self.ptr.vbuff_num
    @property
    def ibuff_offset(self):
        return self.ptr.ibuff_offset
    @property
    def ibuff_num(self):
        return self.ptr.ibuff_num
    @property
    def mesh_bounding_boxes_offset(self):
        return self.ptr.mesh_bounding_boxes_offset
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def variation_counts(self):
        return self.ptr.variation_counts
    @property
    def vals_j_num(self):
        return self.ptr.vals_j_num
    @property
    def vals_j_offset(self):
        return self.ptr.vals_j_offset
    @property
    def block_offset(self):
        return self.ptr.block_offset
    @property
    def vals_k_offset(self):
        return self.ptr.vals_k_offset
    @property
    def asset_key(self):
        return self.ptr.asset_key
    @property
    def asset_type(self):
        return self.ptr.asset_type
    @property
    def unk_54(self):
        return self.ptr.unk_54
    @property
    def unk_55(self):
        return self.ptr.unk_55
    @property
    def shape_offset(self):
        return self.ptr.shape_offset
    @property
    def shape_num(self):
        return self.ptr.shape_num
    @property
    def hk_constraint_data_offset(self):
        return self.ptr.hk_constraint_data_offset
    @property
    def hk_constraint_data_num(self):
        return self.ptr.hk_constraint_data_num
    @property
    def hk_constraint_offset(self):
        return self.ptr.hk_constraint_offset
    @property
    def slots_offset(self):
        return self.ptr.slots_offset
    @property
    def slot_map_offset(self):
        return self.ptr.slot_map_offset
    @property
    def bone_bounding_boxes_offset(self):
        return self.ptr.bone_bounding_boxes_offset

cdef class ref_slice_CrcXbox:
    cdef lotrc_rs.ref_slice_CrcXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_CrcXbox_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_CrcXbox_len(self.ptr)

cdef class ref_slice_i32Xbox:
    cdef lotrc_rs.ref_slice_i32Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_i32Xbox_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_i32Xbox_len(self.ptr)

cdef class ref_slice_Matrix4x4Xbox:
    cdef lotrc_rs.ref_slice_Matrix4x4Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Matrix4x4Xbox()
        val.ptr = lotrc_rs.ref_slice_Matrix4x4Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Matrix4x4Xbox_len(self.ptr)

cdef class ref_slice_BoundingBoxXbox:
    cdef lotrc_rs.ref_slice_BoundingBoxXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BoundingBoxXbox()
        val.ptr = lotrc_rs.ref_slice_BoundingBoxXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BoundingBoxXbox_len(self.ptr)

cdef class BonesRefXbox:
    cdef lotrc_rs.BonesRefXbox* ptr
    @property
    def names(self):
        val = ref_slice_CrcXbox()
        val.ptr = &self.ptr.names
        return val
    @property
    def parents(self):
        val = ref_slice_i32Xbox()
        val.ptr = &self.ptr.parents
        return val
    @property
    def transforms(self):
        val = ref_slice_Matrix4x4Xbox()
        val.ptr = &self.ptr.transforms
        return val
    @property
    def bounding_boxes(self):
        val = ref_slice_BoundingBoxXbox()
        val.ptr = &self.ptr.bounding_boxes
        return val

cdef class ref_slice_Key2Xbox:
    cdef lotrc_rs.ref_slice_Key2Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Key2Xbox()
        val.ptr = lotrc_rs.ref_slice_Key2Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Key2Xbox_len(self.ptr)

cdef class slice_BlockRefXbox:
    cdef lotrc_rs.slice_BlockRefXbox* ptr

cdef class IndexMap_u32_______VBuffInfoXbox:
    cdef lotrc_rs.IndexMap_u32_______VBuffInfoXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = VBuffInfoXbox()
        val.ptr = dereference(lotrc_rs.IndexMap_u32_______VBuffInfoXbox_get(self.ptr, &key))
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32_______VBuffInfoXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32_______VBuffInfoXbox_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32_______IBuffInfoXbox:
    cdef lotrc_rs.IndexMap_u32_______IBuffInfoXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = IBuffInfoXbox()
        val.ptr = dereference(lotrc_rs.IndexMap_u32_______IBuffInfoXbox_get(self.ptr, &key))
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32_______IBuffInfoXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32_______IBuffInfoXbox_keys(self.ptr, keys.ptr)

cdef class Option_HkConstraintRefXbox:
    cdef lotrc_rs.Option_HkConstraintRefXbox* ptr
    def get(self):
        val = HkConstraintRefXbox()
        val.ptr = lotrc_rs.Option_HkConstraintRefXbox_get(self.ptr)
        return val

cdef class slice_ShapeRefXbox:
    cdef lotrc_rs.slice_ShapeRefXbox* ptr

cdef class ModelDataRefXbox:
    cdef lotrc_rs.ModelDataRefXbox* ptr
    @property
    def infos(self):
        val = ref_slice_BufferInfoXbox()
        val.ptr = &self.ptr.infos
        return val
    @property
    def vbuff_order(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.vbuff_order
        return val
    @property
    def ibuff_order(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.ibuff_order
        return val
    @property
    def vertex(self):
        val = IndexMap_u32__VertexBufferRefXbox()
        val.ptr = &self.ptr.vertex
        return val
    @property
    def index(self):
        val = IndexMap_u32__IndexBufferRefXbox()
        val.ptr = &self.ptr.index
        return val
    @property
    def data(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data
        return val

cdef class ModelRefXbox:
    cdef lotrc_rs.ModelRefXbox* ptr
    @property
    def info(self):
        val = ModelInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def bones(self):
        val = BonesRefXbox()
        val.ptr = &self.ptr.bones
        return val
    @property
    def mat_order(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.mat_order
        return val
    @property
    def mesh_order(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.mesh_order
        return val
    @property
    def mesh_bounding_boxes(self):
        val = ref_slice_BoundingBoxXbox()
        val.ptr = &self.ptr.mesh_bounding_boxes
        return val
    @property
    def skin_binds(self):
        val = ref_slice_Matrix4x4Xbox()
        val.ptr = &self.ptr.skin_binds
        return val
    @property
    def vals_j(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.vals_j
        return val
    @property
    def val_k_header(self):
        val = ref_slice_u16Xbox()
        val.ptr = &self.ptr.val_k_header
        return val
    @property
    def vals_k(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.vals_k
        return val
    @property
    def skin_order(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.skin_order
        return val
    @property
    def slots(self):
        val = ref_slice_Key2Xbox()
        val.ptr = &self.ptr.slots
        return val
    @property
    def slot_map(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.slot_map
        return val
    @property
    def block_header(self):
        return dereference(self.ptr.block_header)
    @property
    def block_offsets(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.block_offsets
        return val
    @property
    def blocks(self):
        val = slice_BlockRefXbox()
        val.ptr = &self.ptr.blocks
        return val
    @property
    def buffer_infos(self):
        val = ref_slice_BufferInfoXbox()
        val.ptr = &self.ptr.buffer_infos
        return val
    @property
    def vbuff_order(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.vbuff_order
        return val
    @property
    def ibuff_order(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.ibuff_order
        return val
    @property
    def vbuffs(self):
        val = IndexMap_u32_______VBuffInfoXbox()
        val.ptr = &self.ptr.vbuffs
        return val
    @property
    def ibuffs(self):
        val = IndexMap_u32_______IBuffInfoXbox()
        val.ptr = &self.ptr.ibuffs
        return val
    @property
    def mats(self):
        val = IndexMap_u32__MatRefXbox()
        val.ptr = &self.ptr.mats
        return val
    @property
    def hk_constraint(self):
        val = Option_HkConstraintRefXbox()
        val.ptr = &self.ptr.hk_constraint
        return val
    @property
    def hk_constraint_datas(self):
        val = ref_slice_HkConstraintDataXbox()
        val.ptr = &self.ptr.hk_constraint_datas
        return val
    @property
    def shapes(self):
        val = slice_ShapeRefXbox()
        val.ptr = &self.ptr.shapes
        return val
    @property
    def data(self):
        val = ModelDataRefXbox()
        val.ptr = &self.ptr.data
        return val

cdef class AnimationInfoXbox:
    cdef lotrc_rs.AnimationInfoXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def vals_num(self):
        return self.ptr.vals_num
    @property
    def vals2_num(self):
        return self.ptr.vals2_num
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def vala(self):
        return self.ptr.vala
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def data_offset(self):
        return self.ptr.data_offset
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def t_scale(self):
        return self.ptr.t_scale
    @property
    def block_starts_offset(self):
        return self.ptr.block_starts_offset
    @property
    def block_starts_num(self):
        return self.ptr.block_starts_num
    @property
    def block_starts2_offset(self):
        return self.ptr.block_starts2_offset
    @property
    def block_starts2_num(self):
        return self.ptr.block_starts2_num
    @property
    def obj_c3_offset(self):
        return self.ptr.obj_c3_offset
    @property
    def obj_c3_num(self):
        return self.ptr.obj_c3_num
    @property
    def obj_c4_offset(self):
        return self.ptr.obj_c4_offset
    @property
    def obj_c4_num(self):
        return self.ptr.obj_c4_num
    @property
    def block_offset(self):
        return self.ptr.block_offset
    @property
    def block_size(self):
        return self.ptr.block_size
    @property
    def obj3_num(self):
        return self.ptr.obj3_num
    @property
    def obj3_offset(self):
        return self.ptr.obj3_offset
    @property
    def bones_num1(self):
        return self.ptr.bones_num1
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def obj1_num(self):
        return self.ptr.obj1_num
    @property
    def bones_offset(self):
        return self.ptr.bones_offset
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def obj1_offset(self):
        return self.ptr.obj1_offset
    @property
    def obj2_offset(self):
        return self.ptr.obj2_offset
    @property
    def obj2_num(self):
        return self.ptr.obj2_num
    @property
    def obj5_offset(self):
        return self.ptr.obj5_offset

cdef class ref_slice_Obj3Xbox:
    cdef lotrc_rs.ref_slice_Obj3Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj3Xbox()
        val.ptr = lotrc_rs.ref_slice_Obj3Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Obj3Xbox_len(self.ptr)

cdef class Obj5HeaderXbox:
    cdef lotrc_rs.Obj5HeaderXbox* ptr
    @property
    def obj_a_num(self):
        return self.ptr.obj_a_num
    @property
    def obj_a_offset(self):
        return self.ptr.obj_a_offset
    @property
    def obj_b_num(self):
        return self.ptr.obj_b_num
    @property
    def obj_b_offset(self):
        return self.ptr.obj_b_offset

cdef class ref_slice_Obj5ValXbox:
    cdef lotrc_rs.ref_slice_Obj5ValXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj5ValXbox()
        val.ptr = lotrc_rs.ref_slice_Obj5ValXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Obj5ValXbox_len(self.ptr)

cdef class Option_BlocksRefXbox:
    cdef lotrc_rs.Option_BlocksRefXbox* ptr
    def get(self):
        val = BlocksRefXbox()
        val.ptr = lotrc_rs.Option_BlocksRefXbox_get(self.ptr)
        return val

cdef class AnimationRefXbox:
    cdef lotrc_rs.AnimationRefXbox* ptr
    @property
    def info(self):
        val = AnimationInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def obj1(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.obj1
        return val
    @property
    def obj2(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.obj2
        return val
    @property
    def obj3(self):
        val = ref_slice_Obj3Xbox()
        val.ptr = &self.ptr.obj3
        return val
    @property
    def bones(self):
        val = ref_slice_CrcXbox()
        val.ptr = &self.ptr.bones
        return val
    @property
    def obj5_header(self):
        val = Obj5HeaderXbox()
        val.ptr = self.ptr.obj5_header
        return val
    @property
    def obj5_a(self):
        val = ref_slice_Obj5ValXbox()
        val.ptr = &self.ptr.obj5_a
        return val
    @property
    def obj5_b(self):
        val = ref_slice_Obj5ValXbox()
        val.ptr = &self.ptr.obj5_b
        return val
    @property
    def blocks(self):
        val = Option_BlocksRefXbox()
        val.ptr = &self.ptr.blocks
        return val
    @property
    def size(self):
        return self.ptr.size

cdef class DataRefXbox:
    cdef lotrc_rs.DataRefXbox* ptr
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val

cdef class EffectInfoXbox:
    cdef lotrc_rs.EffectInfoXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size

cdef class EffectRefXbox:
    cdef lotrc_rs.EffectRefXbox* ptr
    @property
    def info(self):
        val = EffectInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def vals(self):
        val = GameObjsRefXbox()
        val.ptr = &self.ptr.vals
        return val

cdef class IndexMap_u32__ref_slice_u16Xbox:
    cdef lotrc_rs.IndexMap_u32__ref_slice_u16Xbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = ref_slice_u16Xbox()
        val.ptr = lotrc_rs.IndexMap_u32__ref_slice_u16Xbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__ref_slice_u16Xbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__ref_slice_u16Xbox_keys(self.ptr, keys.ptr)

cdef class LangStringsRefXbox:
    cdef lotrc_rs.LangStringsRefXbox* ptr
    @property
    def strings(self):
        val = IndexMap_u32__ref_slice_u16Xbox()
        val.ptr = &self.ptr.strings
        return val

cdef class ObjHeaderXbox:
    cdef lotrc_rs.ObjHeaderXbox* ptr
    @property
    def layer(self):
        return self.ptr.layer
    @property
    def key(self):
        return self.ptr.key
    @property
    def size(self):
        return self.ptr.size
    @property
    def z3(self):
        return self.ptr.z3
    @property
    def z4(self):
        return self.ptr.z4

cdef class IndexMap_u32__BaseTypeRefXbox:
    cdef lotrc_rs.IndexMap_u32__BaseTypeRefXbox* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = BaseTypeRefXbox()
        val.ptr = lotrc_rs.IndexMap_u32__BaseTypeRefXbox_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__BaseTypeRefXbox_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__BaseTypeRefXbox_keys(self.ptr, keys.ptr)

cdef class ObjRefXbox:
    cdef lotrc_rs.ObjRefXbox* ptr
    @property
    def header(self):
        val = ObjHeaderXbox()
        val.ptr = self.ptr.header
        return val
    @property
    def fields(self):
        val = IndexMap_u32__BaseTypeRefXbox()
        val.ptr = &self.ptr.fields
        return val

cdef class RadiosityValsInfoXbox:
    cdef lotrc_rs.RadiosityValsInfoXbox* ptr
    @property
    def guid(self):
        return self.ptr.guid
    @property
    def num(self):
        return self.ptr.num
    @property
    def offset(self):
        return self.ptr.offset

cdef class RadiosityValsRefXbox:
    cdef lotrc_rs.RadiosityValsRefXbox* ptr
    @property
    def info(self):
        val = RadiosityValsInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def offs(self):
        val = ref_slice_i32Xbox()
        val.ptr = &self.ptr.offs
        return val

cdef class ref_slice_SSAValXbox:
    cdef lotrc_rs.ref_slice_SSAValXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SSAValXbox()
        val.ptr = lotrc_rs.ref_slice_SSAValXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SSAValXbox_len(self.ptr)

cdef class slice_ref_slice_u16Xbox:
    cdef lotrc_rs.slice_ref_slice_u16Xbox* ptr

cdef class SSARefXbox:
    cdef lotrc_rs.SSARefXbox* ptr
    @property
    def vals(self):
        val = ref_slice_SSAValXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def strings(self):
        val = slice_ref_slice_u16Xbox()
        val.ptr = &self.ptr.strings
        return val

cdef class TextureInfoXbox:
    cdef lotrc_rs.TextureInfoXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def asset_key(self):
        return self.ptr.asset_key
    @property
    def asset_type(self):
        return self.ptr.asset_type
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def format(self):
        return self.ptr.format
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def width(self):
        return self.ptr.width
    @property
    def height(self):
        return self.ptr.height
    @property
    def depth(self):
        return self.ptr.depth
    @property
    def levels(self):
        return self.ptr.levels
    @property
    def unk_16_1(self):
        return self.ptr.unk_16_1
    @property
    def unk_16_2(self):
        return self.ptr.unk_16_2
    @property
    def unk_16_3(self):
        return self.ptr.unk_16_3
    @property
    def unk_16_4(self):
        return self.ptr.unk_16_4
    @property
    def unk_16_5(self):
        return self.ptr.unk_16_5
    @property
    def unk_16_6(self):
        return self.ptr.unk_16_6
    @property
    def unk_16_7(self):
        return self.ptr.unk_16_7
    @property
    def unk_16_8(self):
        return self.ptr.unk_16_8
    @property
    def unk_16_9(self):
        return self.ptr.unk_16_9
    @property
    def unk_16_10(self):
        return self.ptr.unk_16_10
    @property
    def unk_16_11(self):
        return self.ptr.unk_16_11
    @property
    def unk_16_12(self):
        return self.ptr.unk_16_12
    @property
    def unk_16_13(self):
        return self.ptr.unk_16_13
    @property
    def unk_16_14(self):
        return self.ptr.unk_16_14
    @property
    def unk_16_15(self):
        return self.ptr.unk_16_15
    @property
    def unk_16_16(self):
        return self.ptr.unk_16_16

cdef class TextureRefXbox:
    cdef lotrc_rs.TextureRefXbox* ptr
    @property
    def info(self):
        val = TextureInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def data0(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data0
        return val
    @property
    def data1(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data1
        return val

cdef class TypeHeaderXbox:
    cdef lotrc_rs.TypeHeaderXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def size(self):
        return self.ptr.size
    @property
    def fields(self):
        return self.ptr.fields

cdef class ref_slice_TypeFieldXbox:
    cdef lotrc_rs.ref_slice_TypeFieldXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TypeFieldXbox()
        val.ptr = lotrc_rs.ref_slice_TypeFieldXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_TypeFieldXbox_len(self.ptr)

cdef class TypeRefXbox:
    cdef lotrc_rs.TypeRefXbox* ptr
    @property
    def header(self):
        val = TypeHeaderXbox()
        val.ptr = self.ptr.header
        return val
    @property
    def fields(self):
        val = ref_slice_TypeFieldXbox()
        val.ptr = &self.ptr.fields
        return val

cdef class slice_FoliageRefXbox:
    cdef lotrc_rs.slice_FoliageRefXbox* ptr

cdef class Vector2Xbox:
    cdef lotrc_rs.Vector2Xbox* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def y(self):
        return self.ptr.y

cdef class Vector4Xbox:
    cdef lotrc_rs.Vector4Xbox* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def y(self):
        return self.ptr.y
    @property
    def z(self):
        return self.ptr.z
    @property
    def w(self):
        return self.ptr.w

cdef class Matrix4x4Xbox:
    cdef lotrc_rs.Matrix4x4Xbox* ptr
    @property
    def x(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.x
        return val
    @property
    def y(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.y
        return val
    @property
    def z(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.z
        return val
    @property
    def w(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.w
        return val

cdef class ref_slice_U32Xbox:
    cdef lotrc_rs.ref_slice_U32Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_U32Xbox_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_U32Xbox_len(self.ptr)

cdef class ref_slice_Vector4Xbox:
    cdef lotrc_rs.ref_slice_Vector4Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Vector4Xbox()
        val.ptr = lotrc_rs.ref_slice_Vector4Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Vector4Xbox_len(self.ptr)

cdef class ref_slice_WeightXbox:
    cdef lotrc_rs.ref_slice_WeightXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = WeightXbox()
        val.ptr = lotrc_rs.ref_slice_WeightXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_WeightXbox_len(self.ptr)

cdef class BaseTypeRefXbox:
    cdef lotrc_rs.BaseTypeRefXbox* ptr

cdef class BlockHeader1Xbox:
    cdef lotrc_rs.BlockHeader1Xbox* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3

cdef class BlockHeader2Xbox:
    cdef lotrc_rs.BlockHeader2Xbox* ptr
    @property
    def n(self):
        return self.ptr.n
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4

cdef class ref_slice_BlockValAXbox:
    cdef lotrc_rs.ref_slice_BlockValAXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BlockValAXbox()
        val.ptr = lotrc_rs.ref_slice_BlockValAXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BlockValAXbox_len(self.ptr)

cdef class ref_slice_BlockValBXbox:
    cdef lotrc_rs.ref_slice_BlockValBXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BlockValBXbox()
        val.ptr = lotrc_rs.ref_slice_BlockValBXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BlockValBXbox_len(self.ptr)

cdef class BlockRefXbox:
    cdef lotrc_rs.BlockRefXbox* ptr
    @property
    def info1(self):
        val = BlockHeader1Xbox()
        val.ptr = self.ptr.info1
        return val
    @property
    def info2(self):
        val = BlockHeader2Xbox()
        val.ptr = self.ptr.info2
        return val
    @property
    def vals_a(self):
        val = ref_slice_BlockValAXbox()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def vals_b(self):
        val = ref_slice_BlockValAXbox()
        val.ptr = &self.ptr.vals_b
        return val
    @property
    def vals_c(self):
        val = ref_slice_BlockValBXbox()
        val.ptr = &self.ptr.vals_c
        return val
    @property
    def pad(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.pad
        return val

cdef class ShapeInfoXbox:
    cdef lotrc_rs.ShapeInfoXbox* ptr
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def translation(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def hk_shape_num(self):
        return self.ptr.hk_shape_num
    @property
    def hk_shape_offset(self):
        return self.ptr.hk_shape_offset
    @property
    def unk_29a(self):
        return self.ptr.unk_29a
    @property
    def unk_29b(self):
        return self.ptr.unk_29b
    @property
    def unk_29c(self):
        return self.ptr.unk_29c
    @property
    def unk_29d(self):
        return self.ptr.unk_29d
    @property
    def unk_30(self):
        return self.ptr.unk_30

cdef class Option_ShapeExtraRefXbox:
    cdef lotrc_rs.Option_ShapeExtraRefXbox* ptr
    def get(self):
        val = ShapeExtraRefXbox()
        val.ptr = lotrc_rs.Option_ShapeExtraRefXbox_get(self.ptr)
        return val

cdef class slice_HkShapeRefXbox:
    cdef lotrc_rs.slice_HkShapeRefXbox* ptr

cdef class ShapeRefXbox:
    cdef lotrc_rs.ShapeRefXbox* ptr
    @property
    def info(self):
        val = ShapeInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = Option_ShapeExtraRefXbox()
        val.ptr = &self.ptr.extra
        return val
    @property
    def hk_shapes(self):
        val = slice_HkShapeRefXbox()
        val.ptr = &self.ptr.hk_shapes
        return val

cdef class BoxShapeXbox:
    cdef lotrc_rs.BoxShapeXbox* ptr
    @property
    def translation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def half_extents(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.half_extents
        return val
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class SphereShapeXbox:
    cdef lotrc_rs.SphereShapeXbox* ptr
    @property
    def translation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def radius(self):
        return self.ptr.radius
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class CapsuleShapeXbox:
    cdef lotrc_rs.CapsuleShapeXbox* ptr
    @property
    def translation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def point1(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.point1
        return val
    @property
    def point2(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.point2
        return val
    @property
    def radius(self):
        return self.ptr.radius
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class CylinderShapeXbox:
    cdef lotrc_rs.CylinderShapeXbox* ptr
    @property
    def translation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def point1(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.point1
        return val
    @property
    def point2(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.point2
        return val
    @property
    def radius(self):
        return self.ptr.radius
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class ConvexVerticesInfoXbox:
    cdef lotrc_rs.ConvexVerticesInfoXbox* ptr
    @property
    def translation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def norm_num(self):
        return self.ptr.norm_num
    @property
    def norms_offset(self):
        return self.ptr.norms_offset
    @property
    def vert_num(self):
        return self.ptr.vert_num
    @property
    def verts_offset(self):
        return self.ptr.verts_offset
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class ref_slice_Vector3Xbox:
    cdef lotrc_rs.ref_slice_Vector3Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Vector3Xbox()
        val.ptr = lotrc_rs.ref_slice_Vector3Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Vector3Xbox_len(self.ptr)

cdef class ConvexVerticesRefXbox:
    cdef lotrc_rs.ConvexVerticesRefXbox* ptr
    @property
    def info(self):
        val = ConvexVerticesInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def norms(self):
        val = ref_slice_Vector4Xbox()
        val.ptr = &self.ptr.norms
        return val
    @property
    def verts(self):
        val = ref_slice_Vector3Xbox()
        val.ptr = &self.ptr.verts
        return val

cdef class BVTreeMeshInfoXbox:
    cdef lotrc_rs.BVTreeMeshInfoXbox* ptr
    @property
    def translation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.offset
        return val
    @property
    def tree_scale(self):
        return self.ptr.tree_scale
    @property
    def tree_size(self):
        return self.ptr.tree_size
    @property
    def tree_offset(self):
        return self.ptr.tree_offset
    @property
    def vert_num(self):
        return self.ptr.vert_num
    @property
    def verts_offset(self):
        return self.ptr.verts_offset
    @property
    def tri_num(self):
        return self.ptr.tri_num
    @property
    def inds_offset(self):
        return self.ptr.inds_offset

cdef class BVTreeMeshRefXbox:
    cdef lotrc_rs.BVTreeMeshRefXbox* ptr
    @property
    def info(self):
        val = BVTreeMeshInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def tree(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.tree
        return val
    @property
    def verts(self):
        val = ref_slice_Vector3Xbox()
        val.ptr = &self.ptr.verts
        return val
    @property
    def inds(self):
        val = ref_slice_u16Xbox()
        val.ptr = &self.ptr.inds
        return val

cdef class HkShapeInfoXbox:
    cdef lotrc_rs.HkShapeInfoXbox* ptr
    @property
    def unk_0(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.unk_0
        return val
    @property
    def unk_4(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.unk_4
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class HkShapeRefXbox:
    cdef lotrc_rs.HkShapeRefXbox* ptr

cdef class FoliageInfoXbox:
    cdef lotrc_rs.FoliageInfoXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def lb_w(self):
        return self.ptr.lb_w
    @property
    def lb_h(self):
        return self.ptr.lb_h
    @property
    def ub_w(self):
        return self.ptr.ub_w
    @property
    def ub_h(self):
        return self.ptr.ub_h
    @property
    def scale(self):
        return self.ptr.scale
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def key_mesh(self):
        return self.ptr.key_mesh
    @property
    def key_mesh_lod1(self):
        return self.ptr.key_mesh_lod1
    @property
    def key_mesh_lod2(self):
        return self.ptr.key_mesh_lod2
    @property
    def color(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.color
        return val
    @property
    def lod1a(self):
        return self.ptr.lod1a
    @property
    def lod1b(self):
        return self.ptr.lod1b
    @property
    def lod2a(self):
        return self.ptr.lod2a
    @property
    def lod2b(self):
        return self.ptr.lod2b
    @property
    def lod_max(self):
        return self.ptr.lod_max

cdef class ref_slice_FoliageValXbox:
    cdef lotrc_rs.ref_slice_FoliageValXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = FoliageValXbox()
        val.ptr = lotrc_rs.ref_slice_FoliageValXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_FoliageValXbox_len(self.ptr)

cdef class FoliageRefXbox:
    cdef lotrc_rs.FoliageRefXbox* ptr
    @property
    def info(self):
        val = FoliageInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def vals(self):
        val = ref_slice_FoliageValXbox()
        val.ptr = &self.ptr.vals
        return val

cdef class slice_BlockValARefXbox:
    cdef lotrc_rs.slice_BlockValARefXbox* ptr

cdef class slice_Obj1RefXbox:
    cdef lotrc_rs.slice_Obj1RefXbox* ptr

cdef class BlockValRefXbox:
    cdef lotrc_rs.BlockValRefXbox* ptr
    @property
    def vals_a(self):
        val = slice_BlockValARefXbox()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def vals_b(self):
        val = slice_Obj1RefXbox()
        val.ptr = &self.ptr.vals_b
        return val

cdef class slice_BlockValRefXbox:
    cdef lotrc_rs.slice_BlockValRefXbox* ptr

cdef class CrowdItemHeaderXbox:
    cdef lotrc_rs.CrowdItemHeaderXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def key_main(self):
        return self.ptr.key_main
    @property
    def key_right(self):
        return self.ptr.key_right
    @property
    def key_left(self):
        return self.ptr.key_left
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def animation_num(self):
        return self.ptr.animation_num
    @property
    def instance_num(self):
        return self.ptr.instance_num

cdef class ref_slice_CrowdValXbox:
    cdef lotrc_rs.ref_slice_CrowdValXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = CrowdValXbox()
        val.ptr = lotrc_rs.ref_slice_CrowdValXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_CrowdValXbox_len(self.ptr)

cdef class CrowdItemRefXbox:
    cdef lotrc_rs.CrowdItemRefXbox* ptr
    @property
    def header(self):
        val = CrowdItemHeaderXbox()
        val.ptr = self.ptr.header
        return val
    @property
    def animations(self):
        val = ref_slice_CrcXbox()
        val.ptr = &self.ptr.animations
        return val
    @property
    def instances(self):
        val = ref_slice_CrowdValXbox()
        val.ptr = &self.ptr.instances
        return val

cdef class slice_CrowdItemRefXbox:
    cdef lotrc_rs.slice_CrowdItemRefXbox* ptr

cdef class HkConstraintBoneRefXbox:
    cdef lotrc_rs.HkConstraintBoneRefXbox* ptr
    @property
    def name(self):
        val = string()
        val.ptr = &self.ptr.name
        return val
    @property
    def start(self):
        return self.ptr.start
    @property
    def val(self):
        return self.ptr.val

cdef class slice_HkConstraintBoneRefXbox:
    cdef lotrc_rs.slice_HkConstraintBoneRefXbox* ptr

cdef class ref_slice_f32Xbox:
    cdef lotrc_rs.ref_slice_f32Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_f32Xbox_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_f32Xbox_len(self.ptr)

cdef class AnimVals1RefXbox:
    cdef lotrc_rs.AnimVals1RefXbox* ptr

cdef class Obj1RefXbox:
    cdef lotrc_rs.Obj1RefXbox* ptr
    @property
    def flags(self):
        return self.ptr.flags
    @property
    def s2(self):
        return self.ptr.s2
    @property
    def s1(self):
        return self.ptr.s1
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val
    @property
    def vals_a(self):
        val = ref_slice_f32Xbox()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def vals(self):
        val = AnimVals1RefXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def size(self):
        return self.ptr.size

cdef class ref_slice_RotationPolar32Xbox:
    cdef lotrc_rs.ref_slice_RotationPolar32Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationPolar32Xbox()
        val.ptr = lotrc_rs.ref_slice_RotationPolar32Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationPolar32Xbox_len(self.ptr)

cdef class ref_slice_RotationThreeComp40Xbox:
    cdef lotrc_rs.ref_slice_RotationThreeComp40Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationThreeComp40Xbox()
        val.ptr = lotrc_rs.ref_slice_RotationThreeComp40Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationThreeComp40Xbox_len(self.ptr)

cdef class ref_slice_RotationThreeComp48Xbox:
    cdef lotrc_rs.ref_slice_RotationThreeComp48Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationThreeComp48Xbox()
        val.ptr = lotrc_rs.ref_slice_RotationThreeComp48Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationThreeComp48Xbox_len(self.ptr)

cdef class ref_slice_RotationThreeComp24Xbox:
    cdef lotrc_rs.ref_slice_RotationThreeComp24Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationThreeComp24Xbox()
        val.ptr = lotrc_rs.ref_slice_RotationThreeComp24Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationThreeComp24Xbox_len(self.ptr)

cdef class ref_slice_RotationStraight16Xbox:
    cdef lotrc_rs.ref_slice_RotationStraight16Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationStraight16Xbox()
        val.ptr = lotrc_rs.ref_slice_RotationStraight16Xbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationStraight16Xbox_len(self.ptr)

cdef class ref_slice_RotationUncompressedXbox:
    cdef lotrc_rs.ref_slice_RotationUncompressedXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationUncompressedXbox()
        val.ptr = lotrc_rs.ref_slice_RotationUncompressedXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationUncompressedXbox_len(self.ptr)

cdef class RotationQuantizationRefXbox:
    cdef lotrc_rs.RotationQuantizationRefXbox* ptr

cdef class Obj2RefXbox:
    cdef lotrc_rs.Obj2RefXbox* ptr
    @property
    def flags(self):
        return self.ptr.flags
    @property
    def s2(self):
        return self.ptr.s2
    @property
    def s1(self):
        return self.ptr.s1
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val
    @property
    def vals(self):
        val = RotationQuantizationRefXbox()
        val.ptr = &self.ptr.vals
        return val
    @property
    def size(self):
        return self.ptr.size

cdef class BlockValARefXbox:
    cdef lotrc_rs.BlockValARefXbox* ptr
    @property
    def a(self):
        val = Obj1RefXbox()
        val.ptr = &self.ptr.a
        return val
    @property
    def b(self):
        val = Obj2RefXbox()
        val.ptr = &self.ptr.b
        return val
    @property
    def c(self):
        val = Obj1RefXbox()
        val.ptr = &self.ptr.c
        return val

cdef class BufferInfoXbox:
    cdef lotrc_rs.BufferInfoXbox* ptr
    @property
    def vbuff_info_offset(self):
        return self.ptr.vbuff_info_offset
    @property
    def vbuff_info_offset_2(self):
        return self.ptr.vbuff_info_offset_2
    @property
    def vbuff_info_offset_3(self):
        return self.ptr.vbuff_info_offset_3
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def v_size(self):
        return self.ptr.v_size
    @property
    def v_size_2(self):
        return self.ptr.v_size_2
    @property
    def v_size_3(self):
        return self.ptr.v_size_3
    @property
    def unk_35(self):
        return self.ptr.unk_35
    @property
    def unk_36(self):
        return self.ptr.unk_36
    @property
    def unk_37(self):
        return self.ptr.unk_37
    @property
    def unk_38(self):
        return self.ptr.unk_38
    @property
    def unk_39(self):
        return self.ptr.unk_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42
    @property
    def unk_43(self):
        return self.ptr.unk_43
    @property
    def unk_44(self):
        return self.ptr.unk_44
    @property
    def unk_45(self):
        return self.ptr.unk_45
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def unk_47(self):
        return self.ptr.unk_47
    @property
    def vbuff_size(self):
        return self.ptr.vbuff_size
    @property
    def vbuff_size_2(self):
        return self.ptr.vbuff_size_2
    @property
    def vbuff_size_3(self):
        return self.ptr.vbuff_size_3
    @property
    def unk_51(self):
        return self.ptr.unk_51
    @property
    def unk_52(self):
        return self.ptr.unk_52
    @property
    def unk_53(self):
        return self.ptr.unk_53
    @property
    def unk_54(self):
        return self.ptr.unk_54
    @property
    def unk_55(self):
        return self.ptr.unk_55
    @property
    def unk_56(self):
        return self.ptr.unk_56
    @property
    def unk_57(self):
        return self.ptr.unk_57
    @property
    def unk_58(self):
        return self.ptr.unk_58
    @property
    def unk_59(self):
        return self.ptr.unk_59
    @property
    def unk_60(self):
        return self.ptr.unk_60
    @property
    def unk_61(self):
        return self.ptr.unk_61
    @property
    def unk_62(self):
        return self.ptr.unk_62
    @property
    def unk_63(self):
        return self.ptr.unk_63
    @property
    def unk_64(self):
        return self.ptr.unk_64
    @property
    def ibuff_info_offset(self):
        return self.ptr.ibuff_info_offset
    @property
    def i_num(self):
        return self.ptr.i_num
    @property
    def unk_67(self):
        return self.ptr.unk_67
    @property
    def skin_offset(self):
        return self.ptr.skin_offset
    @property
    def skin_size(self):
        return self.ptr.skin_size
    @property
    def unk_70(self):
        return self.ptr.unk_70
    @property
    def tri_num(self):
        return self.ptr.tri_num
    @property
    def unk_72(self):
        return self.ptr.unk_72
    @property
    def unk_73(self):
        return self.ptr.unk_73
    @property
    def unk_74(self):
        return self.ptr.unk_74
    @property
    def unk_75(self):
        return self.ptr.unk_75
    @property
    def unk_76(self):
        return self.ptr.unk_76
    @property
    def unk_77(self):
        return self.ptr.unk_77
    @property
    def unk_78(self):
        return self.ptr.unk_78
    @property
    def unk_79(self):
        return self.ptr.unk_79
    @property
    def unk_80(self):
        return self.ptr.unk_80
    @property
    def unk_81(self):
        return self.ptr.unk_81
    @property
    def unk_82(self):
        return self.ptr.unk_82
    @property
    def unk_83(self):
        return self.ptr.unk_83
    @property
    def unk_84(self):
        return self.ptr.unk_84
    @property
    def unk_85(self):
        return self.ptr.unk_85
    @property
    def unk_86(self):
        return self.ptr.unk_86
    @property
    def unk_87(self):
        return self.ptr.unk_87
    @property
    def variation_id(self):
        return self.ptr.variation_id
    @property
    def variation(self):
        return self.ptr.variation
    @property
    def unk_88c(self):
        return self.ptr.unk_88c
    @property
    def unk_88d(self):
        return self.ptr.unk_88d

cdef class HkConstraintDataXbox:
    cdef lotrc_rs.HkConstraintDataXbox* ptr
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28

cdef class Key2Xbox:
    cdef lotrc_rs.Key2Xbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def val(self):
        return self.ptr.val

cdef class BlockValAXbox:
    cdef lotrc_rs.BlockValAXbox* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11

cdef class BlockValBXbox:
    cdef lotrc_rs.BlockValBXbox* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5

cdef class AnimationBlockInfoXbox:
    cdef lotrc_rs.AnimationBlockInfoXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def guid(self):
        return self.ptr.guid
    @property
    def key_name(self):
        return self.ptr.key_name
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size
    @property
    def size_comp(self):
        return self.ptr.size_comp
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8

cdef class AssetHandleXbox:
    cdef lotrc_rs.AssetHandleXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size
    @property
    def size_comp(self):
        return self.ptr.size_comp
    @property
    def kind(self):
        return self.ptr.kind

cdef class BlockAValXbox:
    cdef lotrc_rs.BlockAValXbox* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def key(self):
        return self.ptr.key
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6

cdef class GFXBlockInfoXbox:
    cdef lotrc_rs.GFXBlockInfoXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size

cdef class HkConstraintInfoXbox:
    cdef lotrc_rs.HkConstraintInfoXbox* ptr
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def bone_parents_offset(self):
        return self.ptr.bone_parents_offset
    @property
    def bone_parents_num(self):
        return self.ptr.bone_parents_num
    @property
    def bone_names_offset(self):
        return self.ptr.bone_names_offset
    @property
    def bone_names_num(self):
        return self.ptr.bone_names_num
    @property
    def bone_transforms_offset(self):
        return self.ptr.bone_transforms_offset
    @property
    def bone_transforms_num(self):
        return self.ptr.bone_transforms_num
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def bones_offset(self):
        return self.ptr.bones_offset
    @property
    def bones_num(self):
        return self.ptr.bones_num
    @property
    def bone_order_num(self):
        return self.ptr.bone_order_num
    @property
    def bone_order_offset(self):
        return self.ptr.bone_order_offset
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def vals2_num(self):
        return self.ptr.vals2_num
    @property
    def vals2_offset(self):
        return self.ptr.vals2_offset
    @property
    def unk_17(self):
        return self.ptr.unk_17

cdef class Obj0Xbox:
    cdef lotrc_rs.Obj0Xbox* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def key(self):
        return self.ptr.key

cdef class ObjAXbox:
    cdef lotrc_rs.ObjAXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def size(self):
        return self.ptr.size
    @property
    def size_comp(self):
        return self.ptr.size_comp
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def kind(self):
        return self.ptr.kind

cdef class PFieldInfoXbox:
    cdef lotrc_rs.PFieldInfoXbox* ptr
    @property
    def link_guid(self):
        return self.ptr.link_guid
    @property
    def gamemode_guid(self):
        return self.ptr.gamemode_guid
    @property
    def width(self):
        return self.ptr.width
    @property
    def height(self):
        return self.ptr.height
    @property
    def offset(self):
        return self.ptr.offset

cdef class StringKeysValXbox:
    cdef lotrc_rs.StringKeysValXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset

cdef class SubBlocksBlockHeaderXbox:
    cdef lotrc_rs.SubBlocksBlockHeaderXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size

cdef class Obj3Xbox:
    cdef lotrc_rs.Obj3Xbox* ptr
    @property
    def t(self):
        return self.ptr.t
    @property
    def event(self):
        return self.ptr.event
    @property
    def dat_2(self):
        return self.ptr.dat_2
    @property
    def dat_3(self):
        return self.ptr.dat_3
    @property
    def dat_4(self):
        return self.ptr.dat_4
    @property
    def dat_5(self):
        return self.ptr.dat_5
    @property
    def dat_6(self):
        return self.ptr.dat_6
    @property
    def dat_7(self):
        return self.ptr.dat_7
    @property
    def dat_8(self):
        return self.ptr.dat_8
    @property
    def dat_9(self):
        return self.ptr.dat_9
    @property
    def dat_10(self):
        return self.ptr.dat_10

cdef class Obj5ValXbox:
    cdef lotrc_rs.Obj5ValXbox* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6

cdef class SSAValXbox:
    cdef lotrc_rs.SSAValXbox* ptr
    @property
    def t_start(self):
        return self.ptr.t_start
    @property
    def t_end(self):
        return self.ptr.t_end
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def off(self):
        return self.ptr.off

cdef class TypeFieldXbox:
    cdef lotrc_rs.TypeFieldXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def offset(self):
        return self.ptr.offset

cdef class FoliageValXbox:
    cdef lotrc_rs.FoliageValXbox* ptr
    @property
    def height(self):
        return self.ptr.height
    @property
    def var_mask(self):
        return self.ptr.var_mask
    @property
    def slope_x(self):
        return self.ptr.slope_x
    @property
    def slope_z(self):
        return self.ptr.slope_z

cdef class WeightXbox:
    cdef lotrc_rs.WeightXbox* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c
    @property
    def d(self):
        return self.ptr.d

cdef class AtlasUVValXbox:
    cdef lotrc_rs.AtlasUVValXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def vals(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.vals
        return val

cdef class ref_slice_AtlasUVValXbox:
    cdef lotrc_rs.ref_slice_AtlasUVValXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AtlasUVValXbox()
        val.ptr = lotrc_rs.ref_slice_AtlasUVValXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AtlasUVValXbox_len(self.ptr)

cdef class SprayInstanceXbox:
    cdef lotrc_rs.SprayInstanceXbox* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def tex1(self):
        return self.ptr.tex1
    @property
    def tex2(self):
        return self.ptr.tex2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def width(self):
        return self.ptr.width
    @property
    def height(self):
        return self.ptr.height
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def size_w(self):
        return self.ptr.size_w
    @property
    def size_h(self):
        return self.ptr.size_h
    @property
    def scale_w(self):
        return self.ptr.scale_w
    @property
    def scale_h(self):
        return self.ptr.scale_h
    @property
    def delay(self):
        return self.ptr.delay
    @property
    def stride_x(self):
        return self.ptr.stride_x
    @property
    def stride_y(self):
        return self.ptr.stride_y
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16

cdef class ref_slice_SprayInstanceXbox:
    cdef lotrc_rs.ref_slice_SprayInstanceXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SprayInstanceXbox()
        val.ptr = lotrc_rs.ref_slice_SprayInstanceXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SprayInstanceXbox_len(self.ptr)

cdef class SprayValXbox:
    cdef lotrc_rs.SprayValXbox* ptr
    @property
    def position(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.position
        return val
    @property
    def scale(self):
        return self.ptr.scale
    @property
    def instance(self):
        return self.ptr.instance
    @property
    def rotation(self):
        return self.ptr.rotation

cdef class ref_slice_SprayValXbox:
    cdef lotrc_rs.ref_slice_SprayValXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SprayValXbox()
        val.ptr = lotrc_rs.ref_slice_SprayValXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SprayValXbox_len(self.ptr)

cdef class TRSXbox:
    cdef lotrc_rs.TRSXbox* ptr
    @property
    def translation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def scale(self):
        val = Vector4Xbox()
        val.ptr = &self.ptr.scale
        return val

cdef class ref_slice_TRSXbox:
    cdef lotrc_rs.ref_slice_TRSXbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TRSXbox()
        val.ptr = lotrc_rs.ref_slice_TRSXbox_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_TRSXbox_len(self.ptr)

cdef class ref_slice_i16Xbox:
    cdef lotrc_rs.ref_slice_i16Xbox* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_i16Xbox_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_i16Xbox_len(self.ptr)

cdef class CrowdValXbox:
    cdef lotrc_rs.CrowdValXbox* ptr
    @property
    def position(self):
        val = Vector3Xbox()
        val.ptr = &self.ptr.position
        return val
    @property
    def rotation(self):
        return self.ptr.rotation
    @property
    def lod(self):
        return self.ptr.lod

cdef class RotationPolar32Xbox:
    cdef lotrc_rs.RotationPolar32Xbox* ptr
    @property
    def a(self):
        return self.ptr.a

cdef class RotationStraight16Xbox:
    cdef lotrc_rs.RotationStraight16Xbox* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b

cdef class RotationThreeComp24Xbox:
    cdef lotrc_rs.RotationThreeComp24Xbox* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c

cdef class RotationThreeComp40Xbox:
    cdef lotrc_rs.RotationThreeComp40Xbox* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c
    @property
    def d(self):
        return self.ptr.d
    @property
    def e(self):
        return self.ptr.e

cdef class RotationThreeComp48Xbox:
    cdef lotrc_rs.RotationThreeComp48Xbox* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c

cdef class RotationUncompressedXbox:
    cdef lotrc_rs.RotationUncompressedXbox* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c
    @property
    def d(self):
        return self.ptr.d

cdef class HkConstraintRefXbox:
    cdef lotrc_rs.HkConstraintRefXbox* ptr
    @property
    def info(self):
        val = HkConstraintInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def bone_parents(self):
        val = ref_slice_i16Xbox()
        val.ptr = &self.ptr.bone_parents
        return val
    @property
    def bone_names(self):
        val = slice_HkConstraintBoneRefXbox()
        val.ptr = &self.ptr.bone_names
        return val
    @property
    def name_offsets(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.name_offsets
        return val
    @property
    def bone_transforms(self):
        val = ref_slice_TRSXbox()
        val.ptr = &self.ptr.bone_transforms
        return val
    @property
    def bones(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.bones
        return val
    @property
    def bones_order(self):
        val = ref_slice_Key2Xbox()
        val.ptr = &self.ptr.bones_order
        return val
    @property
    def vals2(self):
        val = ref_slice_f32Xbox()
        val.ptr = &self.ptr.vals2
        return val

cdef class ShapeExtraInfoXbox:
    cdef lotrc_rs.ShapeExtraInfoXbox* ptr
    @property
    def size(self):
        return self.ptr.size
    @property
    def scale(self):
        return self.ptr.scale
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b

cdef class ShapeExtraRefXbox:
    cdef lotrc_rs.ShapeExtraRefXbox* ptr
    @property
    def info(self):
        val = ShapeExtraInfoXbox()
        val.ptr = self.ptr.info
        return val
    @property
    def offs(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.offs
        return val
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val

cdef class AtlasUVRefXbox:
    cdef lotrc_rs.AtlasUVRefXbox* ptr
    @property
    def vals(self):
        val = ref_slice_AtlasUVValXbox()
        val.ptr = &self.ptr.vals
        return val

cdef class BlocksRefXbox:
    cdef lotrc_rs.BlocksRefXbox* ptr
    @property
    def block_starts(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.block_starts
        return val
    @property
    def block_starts2(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.block_starts2
        return val
    @property
    def obj_c3(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.obj_c3
        return val
    @property
    def obj_c4(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.obj_c4
        return val
    @property
    def blocks(self):
        val = slice_BlockValRefXbox()
        val.ptr = &self.ptr.blocks
        return val

cdef class CrowdHeaderXbox:
    cdef lotrc_rs.CrowdHeaderXbox* ptr
    @property
    def const0x65(self):
        return self.ptr.const0x65
    @property
    def n(self):
        return self.ptr.n

cdef class CrowdRefXbox:
    cdef lotrc_rs.CrowdRefXbox* ptr
    @property
    def header(self):
        val = CrowdHeaderXbox()
        val.ptr = self.ptr.header
        return val
    @property
    def offs(self):
        val = ref_slice_u32Xbox()
        val.ptr = &self.ptr.offs
        return val
    @property
    def vals(self):
        val = slice_CrowdItemRefXbox()
        val.ptr = &self.ptr.vals
        return val

cdef class SprayRefXbox:
    cdef lotrc_rs.SprayRefXbox* ptr
    @property
    def instances(self):
        val = ref_slice_SprayInstanceXbox()
        val.ptr = &self.ptr.instances
        return val
    @property
    def vals(self):
        val = ref_slice_SprayValXbox()
        val.ptr = &self.ptr.vals
        return val

cdef class IBuffInfoPs3:
    cdef lotrc_rs.IBuffInfoPs3* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def vbuff_alt_fmt(self):
        return self.ptr.vbuff_alt_fmt
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def size(self):
        return self.ptr.size
    @property
    def format(self):
        return self.ptr.format
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def offset(self):
        return self.ptr.offset

cdef class ref_slice_u16Ps3:
    cdef lotrc_rs.ref_slice_u16Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_u16Ps3_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_u16Ps3_len(self.ptr)

cdef class IndexBufferValsRefPs3:
    cdef lotrc_rs.IndexBufferValsRefPs3* ptr

cdef class IndexBufferRefPs3:
    cdef lotrc_rs.IndexBufferRefPs3* ptr
    @property
    def info(self):
        val = IBuffInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def vals(self):
        val = IndexBufferValsRefPs3()
        val.ptr = &self.ptr.vals
        return val

cdef class IndexMap_u32__IndexBufferRefPs3:
    cdef lotrc_rs.IndexMap_u32__IndexBufferRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = IndexBufferRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__IndexBufferRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__IndexBufferRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__IndexBufferRefPs3_keys(self.ptr, keys.ptr)

cdef class VBuffInfoPs3:
    cdef lotrc_rs.VBuffInfoPs3* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def size(self):
        return self.ptr.size
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def fmt2(self):
        return self.ptr.fmt2
    @property
    def fmt1(self):
        return self.ptr.fmt1

cdef class VertexBufferRefPs3:
    cdef lotrc_rs.VertexBufferRefPs3* ptr
    @property
    def info(self):
        val = VBuffInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def offsets(self):
        val = IndexMap_VertexUsage__VertexDataIndex()
        val.ptr = &self.ptr.offsets
        return val
    @property
    def size(self):
        return self.ptr.size
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val

cdef class IndexMap_u32__VertexBufferRefPs3:
    cdef lotrc_rs.IndexMap_u32__VertexBufferRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = VertexBufferRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__VertexBufferRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__VertexBufferRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__VertexBufferRefPs3_keys(self.ptr, keys.ptr)

cdef class MatBasePs3:
    cdef lotrc_rs.MatBasePs3* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def tex0(self):
        return self.ptr.tex0
    @property
    def tex1(self):
        return self.ptr.tex1
    @property
    def tex2(self):
        return self.ptr.tex2
    @property
    def tex3(self):
        return self.ptr.tex3
    @property
    def tex4(self):
        return self.ptr.tex4
    @property
    def tex5(self):
        return self.ptr.tex5
    @property
    def key_guid(self):
        return self.ptr.key_guid
    @property
    def mask0(self):
        return self.ptr.mask0
    @property
    def mask1(self):
        return self.ptr.mask1
    @property
    def mask2(self):
        return self.ptr.mask2
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def unk_33(self):
        return self.ptr.unk_33
    @property
    def z_35(self):
        return self.ptr.z_35
    @property
    def z_36(self):
        return self.ptr.z_36
    @property
    def z_37(self):
        return self.ptr.z_37
    @property
    def z_38(self):
        return self.ptr.z_38
    @property
    def z_39(self):
        return self.ptr.z_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42
    @property
    def unk_43(self):
        return self.ptr.unk_43
    @property
    def unk_44(self):
        return self.ptr.unk_44
    @property
    def unk_45(self):
        return self.ptr.unk_45
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def unk_47(self):
        return self.ptr.unk_47
    @property
    def unk_48(self):
        return self.ptr.unk_48
    @property
    def unk_49(self):
        return self.ptr.unk_49
    @property
    def flags(self):
        return self.ptr.flags
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_53(self):
        return self.ptr.unk_53
    @property
    def unk_54a(self):
        return self.ptr.unk_54a
    @property
    def unk_54b(self):
        return self.ptr.unk_54b
    @property
    def side_flags(self):
        return self.ptr.side_flags
    @property
    def unk_55(self):
        return self.ptr.unk_55
    @property
    def unk_56(self):
        return self.ptr.unk_56
    @property
    def unk_57(self):
        return self.ptr.unk_57
    @property
    def unk_58(self):
        return self.ptr.unk_58
    @property
    def unk_59(self):
        return self.ptr.unk_59
    @property
    def unk_60(self):
        return self.ptr.unk_60
    @property
    def unk_61(self):
        return self.ptr.unk_61
    @property
    def unk_62(self):
        return self.ptr.unk_62
    @property
    def unk_63(self):
        return self.ptr.unk_63
    @property
    def unk_64(self):
        return self.ptr.unk_64
    @property
    def unk_65(self):
        return self.ptr.unk_65
    @property
    def unk_66(self):
        return self.ptr.unk_66
    @property
    def unk_67(self):
        return self.ptr.unk_67
    @property
    def unk_68(self):
        return self.ptr.unk_68
    @property
    def unk_69(self):
        return self.ptr.unk_69
    @property
    def unk_70(self):
        return self.ptr.unk_70
    @property
    def unk_71(self):
        return self.ptr.unk_71
    @property
    def unk_72(self):
        return self.ptr.unk_72
    @property
    def unk_73(self):
        return self.ptr.unk_73
    @property
    def unk_74(self):
        return self.ptr.unk_74
    @property
    def unk_75(self):
        return self.ptr.unk_75
    @property
    def unk_76(self):
        return self.ptr.unk_76
    @property
    def unk_77(self):
        return self.ptr.unk_77
    @property
    def unk_78(self):
        return self.ptr.unk_78
    @property
    def unk_79(self):
        return self.ptr.unk_79
    @property
    def unk_80(self):
        return self.ptr.unk_80
    @property
    def unk_81(self):
        return self.ptr.unk_81
    @property
    def unk_82(self):
        return self.ptr.unk_82
    @property
    def unk_83(self):
        return self.ptr.unk_83
    @property
    def unk_84(self):
        return self.ptr.unk_84
    @property
    def unk_85(self):
        return self.ptr.unk_85
    @property
    def mat_extra_offset(self):
        return self.ptr.mat_extra_offset
    @property
    def key(self):
        return self.ptr.key
    @property
    def unk_88(self):
        return self.ptr.unk_88
    @property
    def z_89(self):
        return self.ptr.z_89

cdef class Mat1Ps3:
    cdef lotrc_rs.Mat1Ps3* ptr
    @property
    def base(self):
        val = MatBasePs3()
        val.ptr = &self.ptr.base
        return val

cdef class MatExtraPs3:
    cdef lotrc_rs.MatExtraPs3* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def unk_30(self):
        return self.ptr.unk_30
    @property
    def unk_31(self):
        return self.ptr.unk_31
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def unk_33(self):
        return self.ptr.unk_33
    @property
    def unk_34(self):
        return self.ptr.unk_34
    @property
    def unk_35(self):
        return self.ptr.unk_35
    @property
    def unk_36(self):
        return self.ptr.unk_36
    @property
    def unk_37(self):
        return self.ptr.unk_37
    @property
    def unk_38(self):
        return self.ptr.unk_38
    @property
    def unk_39(self):
        return self.ptr.unk_39
    @property
    def unk_40(self):
        return self.ptr.unk_40
    @property
    def unk_41(self):
        return self.ptr.unk_41
    @property
    def unk_42(self):
        return self.ptr.unk_42
    @property
    def unk_43(self):
        return self.ptr.unk_43
    @property
    def unk_44(self):
        return self.ptr.unk_44
    @property
    def unk_45(self):
        return self.ptr.unk_45
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def unk_47(self):
        return self.ptr.unk_47
    @property
    def unk_48(self):
        return self.ptr.unk_48
    @property
    def unk_49(self):
        return self.ptr.unk_49

cdef class Mat1RefPs3:
    cdef lotrc_rs.Mat1RefPs3* ptr
    @property
    def info(self):
        val = Mat1Ps3()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraPs3()
        val.ptr = self.ptr.extra
        return val

cdef class Mat2Ps3:
    cdef lotrc_rs.Mat2Ps3* ptr
    @property
    def base(self):
        val = MatBasePs3()
        val.ptr = &self.ptr.base
        return val
    @property
    def unk_90(self):
        return self.ptr.unk_90
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def unk_114(self):
        return self.ptr.unk_114
    @property
    def unk_115(self):
        return self.ptr.unk_115
    @property
    def unk_116(self):
        return self.ptr.unk_116
    @property
    def unk_117(self):
        return self.ptr.unk_117
    @property
    def unk_118(self):
        return self.ptr.unk_118
    @property
    def unk_119(self):
        return self.ptr.unk_119
    @property
    def unk_120a(self):
        return self.ptr.unk_120a
    @property
    def unk_120b(self):
        return self.ptr.unk_120b
    @property
    def unk_120c(self):
        return self.ptr.unk_120c
    @property
    def unk_120d(self):
        return self.ptr.unk_120d
    @property
    def unk_121(self):
        return self.ptr.unk_121

cdef class Mat2RefPs3:
    cdef lotrc_rs.Mat2RefPs3* ptr
    @property
    def info(self):
        val = Mat2Ps3()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraPs3()
        val.ptr = self.ptr.extra
        return val

cdef class Mat3Ps3:
    cdef lotrc_rs.Mat3Ps3* ptr
    @property
    def base(self):
        val = MatBasePs3()
        val.ptr = &self.ptr.base
        return val
    @property
    def unk_90(self):
        return self.ptr.unk_90
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def variation_id_color(self):
        return self.ptr.variation_id_color
    @property
    def variation_id_texture(self):
        return self.ptr.variation_id_texture
    @property
    def variation_id_specular(self):
        return self.ptr.variation_id_specular
    @property
    def unk_114d(self):
        return self.ptr.unk_114d
    @property
    def unk_115(self):
        return self.ptr.unk_115
    @property
    def unk_116(self):
        return self.ptr.unk_116
    @property
    def unk_117(self):
        return self.ptr.unk_117

cdef class Mat3RefPs3:
    cdef lotrc_rs.Mat3RefPs3* ptr
    @property
    def info(self):
        val = Mat3Ps3()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraPs3()
        val.ptr = self.ptr.extra
        return val

cdef class Mat4Ps3:
    cdef lotrc_rs.Mat4Ps3* ptr
    @property
    def base(self):
        val = MatBasePs3()
        val.ptr = &self.ptr.base
        return val
    @property
    def unk_90(self):
        return self.ptr.unk_90
    @property
    def unk_91(self):
        return self.ptr.unk_91
    @property
    def unk_92(self):
        return self.ptr.unk_92
    @property
    def unk_93(self):
        return self.ptr.unk_93
    @property
    def unk_94(self):
        return self.ptr.unk_94
    @property
    def unk_95(self):
        return self.ptr.unk_95
    @property
    def unk_96(self):
        return self.ptr.unk_96
    @property
    def unk_97(self):
        return self.ptr.unk_97
    @property
    def unk_98(self):
        return self.ptr.unk_98
    @property
    def unk_99(self):
        return self.ptr.unk_99
    @property
    def unk_100(self):
        return self.ptr.unk_100
    @property
    def unk_101(self):
        return self.ptr.unk_101
    @property
    def unk_102(self):
        return self.ptr.unk_102
    @property
    def unk_103(self):
        return self.ptr.unk_103
    @property
    def unk_104(self):
        return self.ptr.unk_104
    @property
    def unk_105(self):
        return self.ptr.unk_105
    @property
    def unk_106(self):
        return self.ptr.unk_106
    @property
    def unk_107(self):
        return self.ptr.unk_107
    @property
    def unk_108(self):
        return self.ptr.unk_108
    @property
    def unk_109(self):
        return self.ptr.unk_109
    @property
    def unk_110(self):
        return self.ptr.unk_110
    @property
    def unk_111(self):
        return self.ptr.unk_111
    @property
    def unk_112(self):
        return self.ptr.unk_112
    @property
    def unk_113(self):
        return self.ptr.unk_113
    @property
    def unk_114(self):
        return self.ptr.unk_114
    @property
    def unk_115(self):
        return self.ptr.unk_115
    @property
    def unk_116(self):
        return self.ptr.unk_116
    @property
    def unk_117(self):
        return self.ptr.unk_117
    @property
    def unk_118(self):
        return self.ptr.unk_118
    @property
    def unk_119(self):
        return self.ptr.unk_119
    @property
    def unk_120(self):
        return self.ptr.unk_120
    @property
    def unk_121(self):
        return self.ptr.unk_121
    @property
    def unk_122(self):
        return self.ptr.unk_122
    @property
    def unk_123(self):
        return self.ptr.unk_123
    @property
    def unk_124(self):
        return self.ptr.unk_124
    @property
    def unk_125(self):
        return self.ptr.unk_125
    @property
    def unk_126(self):
        return self.ptr.unk_126
    @property
    def unk_127(self):
        return self.ptr.unk_127
    @property
    def unk_128(self):
        return self.ptr.unk_128
    @property
    def unk_129(self):
        return self.ptr.unk_129
    @property
    def unk_130(self):
        return self.ptr.unk_130
    @property
    def unk_131(self):
        return self.ptr.unk_131
    @property
    def unk_132(self):
        return self.ptr.unk_132
    @property
    def unk_133(self):
        return self.ptr.unk_133
    @property
    def unk_134(self):
        return self.ptr.unk_134
    @property
    def unk_135(self):
        return self.ptr.unk_135
    @property
    def unk_136(self):
        return self.ptr.unk_136
    @property
    def unk_137(self):
        return self.ptr.unk_137
    @property
    def unk_138(self):
        return self.ptr.unk_138
    @property
    def unk_139(self):
        return self.ptr.unk_139
    @property
    def unk_140(self):
        return self.ptr.unk_140
    @property
    def unk_141(self):
        return self.ptr.unk_141
    @property
    def unk_142(self):
        return self.ptr.unk_142
    @property
    def unk_143(self):
        return self.ptr.unk_143
    @property
    def unk_144(self):
        return self.ptr.unk_144
    @property
    def unk_145(self):
        return self.ptr.unk_145

cdef class Mat4RefPs3:
    cdef lotrc_rs.Mat4RefPs3* ptr
    @property
    def info(self):
        val = Mat4Ps3()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = MatExtraPs3()
        val.ptr = self.ptr.extra
        return val

cdef class MatRefPs3:
    cdef lotrc_rs.MatRefPs3* ptr

cdef class IndexMap_u32__MatRefPs3:
    cdef lotrc_rs.IndexMap_u32__MatRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = MatRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__MatRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__MatRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__MatRefPs3_keys(self.ptr, keys.ptr)

cdef class Vector3Ps3:
    cdef lotrc_rs.Vector3Ps3* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def y(self):
        return self.ptr.y
    @property
    def z(self):
        return self.ptr.z

cdef class BoundingBoxPs3:
    cdef lotrc_rs.BoundingBoxPs3* ptr
    @property
    def center(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.center
        return val
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def half_width(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.half_width
        return val
    @property
    def unk_7(self):
        return self.ptr.unk_7

cdef class LodInfoPs3:
    cdef lotrc_rs.LodInfoPs3* ptr
    @property
    def start(self):
        return self.ptr.start
    @property
    def static_end(self):
        return self.ptr.static_end
    @property
    def skinned_end(self):
        return self.ptr.skinned_end
    @property
    def physics_end(self):
        return self.ptr.physics_end
    @property
    def breakable_end(self):
        return self.ptr.breakable_end

cdef class ModelInfoPs3:
    cdef lotrc_rs.ModelInfoPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def mat_offset(self):
        return self.ptr.mat_offset
    @property
    def buffer_info_offset(self):
        return self.ptr.buffer_info_offset
    @property
    def bounding_box(self):
        val = BoundingBoxPs3()
        val.ptr = &self.ptr.bounding_box
        return val
    @property
    def mesh_order_offset(self):
        return self.ptr.mesh_order_offset
    @property
    def lod0(self):
        val = LodInfoPs3()
        val.ptr = &self.ptr.lod0
        return val
    @property
    def lod1(self):
        val = LodInfoPs3()
        val.ptr = &self.ptr.lod1
        return val
    @property
    def lod2(self):
        val = LodInfoPs3()
        val.ptr = &self.ptr.lod2
        return val
    @property
    def lod3(self):
        val = LodInfoPs3()
        val.ptr = &self.ptr.lod3
        return val
    @property
    def mat_num(self):
        return self.ptr.mat_num
    @property
    def bones_offset(self):
        return self.ptr.bones_offset
    @property
    def bone_parents_offset(self):
        return self.ptr.bone_parents_offset
    @property
    def bone_transforms_offset(self):
        return self.ptr.bone_transforms_offset
    @property
    def bones_num(self):
        return self.ptr.bones_num
    @property
    def skin_binds_offset(self):
        return self.ptr.skin_binds_offset
    @property
    def skin_binds_num(self):
        return self.ptr.skin_binds_num
    @property
    def skin_order_offset(self):
        return self.ptr.skin_order_offset
    @property
    def vbuff_offset(self):
        return self.ptr.vbuff_offset
    @property
    def vbuff_num(self):
        return self.ptr.vbuff_num
    @property
    def ibuff_offset(self):
        return self.ptr.ibuff_offset
    @property
    def ibuff_num(self):
        return self.ptr.ibuff_num
    @property
    def mesh_bounding_boxes_offset(self):
        return self.ptr.mesh_bounding_boxes_offset
    @property
    def unk_46(self):
        return self.ptr.unk_46
    @property
    def variation_counts(self):
        return self.ptr.variation_counts
    @property
    def vals_j_num(self):
        return self.ptr.vals_j_num
    @property
    def vals_j_offset(self):
        return self.ptr.vals_j_offset
    @property
    def block_offset(self):
        return self.ptr.block_offset
    @property
    def vals_k_offset(self):
        return self.ptr.vals_k_offset
    @property
    def asset_key(self):
        return self.ptr.asset_key
    @property
    def asset_type(self):
        return self.ptr.asset_type
    @property
    def unk_54(self):
        return self.ptr.unk_54
    @property
    def unk_55(self):
        return self.ptr.unk_55
    @property
    def shape_offset(self):
        return self.ptr.shape_offset
    @property
    def shape_num(self):
        return self.ptr.shape_num
    @property
    def hk_constraint_data_offset(self):
        return self.ptr.hk_constraint_data_offset
    @property
    def hk_constraint_data_num(self):
        return self.ptr.hk_constraint_data_num
    @property
    def hk_constraint_offset(self):
        return self.ptr.hk_constraint_offset
    @property
    def slots_offset(self):
        return self.ptr.slots_offset
    @property
    def slot_map_offset(self):
        return self.ptr.slot_map_offset
    @property
    def bone_bounding_boxes_offset(self):
        return self.ptr.bone_bounding_boxes_offset

cdef class ref_slice_CrcPs3:
    cdef lotrc_rs.ref_slice_CrcPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_CrcPs3_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_CrcPs3_len(self.ptr)

cdef class ref_slice_i32Ps3:
    cdef lotrc_rs.ref_slice_i32Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_i32Ps3_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_i32Ps3_len(self.ptr)

cdef class ref_slice_Matrix4x4Ps3:
    cdef lotrc_rs.ref_slice_Matrix4x4Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Matrix4x4Ps3()
        val.ptr = lotrc_rs.ref_slice_Matrix4x4Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Matrix4x4Ps3_len(self.ptr)

cdef class ref_slice_BoundingBoxPs3:
    cdef lotrc_rs.ref_slice_BoundingBoxPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BoundingBoxPs3()
        val.ptr = lotrc_rs.ref_slice_BoundingBoxPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BoundingBoxPs3_len(self.ptr)

cdef class BonesRefPs3:
    cdef lotrc_rs.BonesRefPs3* ptr
    @property
    def names(self):
        val = ref_slice_CrcPs3()
        val.ptr = &self.ptr.names
        return val
    @property
    def parents(self):
        val = ref_slice_i32Ps3()
        val.ptr = &self.ptr.parents
        return val
    @property
    def transforms(self):
        val = ref_slice_Matrix4x4Ps3()
        val.ptr = &self.ptr.transforms
        return val
    @property
    def bounding_boxes(self):
        val = ref_slice_BoundingBoxPs3()
        val.ptr = &self.ptr.bounding_boxes
        return val

cdef class ref_slice_Key2Ps3:
    cdef lotrc_rs.ref_slice_Key2Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Key2Ps3()
        val.ptr = lotrc_rs.ref_slice_Key2Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Key2Ps3_len(self.ptr)

cdef class slice_BlockRefPs3:
    cdef lotrc_rs.slice_BlockRefPs3* ptr

cdef class IndexMap_u32_______VBuffInfoPs3:
    cdef lotrc_rs.IndexMap_u32_______VBuffInfoPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = VBuffInfoPs3()
        val.ptr = dereference(lotrc_rs.IndexMap_u32_______VBuffInfoPs3_get(self.ptr, &key))
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32_______VBuffInfoPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32_______VBuffInfoPs3_keys(self.ptr, keys.ptr)

cdef class IndexMap_u32_______IBuffInfoPs3:
    cdef lotrc_rs.IndexMap_u32_______IBuffInfoPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = IBuffInfoPs3()
        val.ptr = dereference(lotrc_rs.IndexMap_u32_______IBuffInfoPs3_get(self.ptr, &key))
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32_______IBuffInfoPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32_______IBuffInfoPs3_keys(self.ptr, keys.ptr)

cdef class Option_HkConstraintRefPs3:
    cdef lotrc_rs.Option_HkConstraintRefPs3* ptr
    def get(self):
        val = HkConstraintRefPs3()
        val.ptr = lotrc_rs.Option_HkConstraintRefPs3_get(self.ptr)
        return val

cdef class slice_ShapeRefPs3:
    cdef lotrc_rs.slice_ShapeRefPs3* ptr

cdef class ModelDataRefPs3:
    cdef lotrc_rs.ModelDataRefPs3* ptr
    @property
    def infos(self):
        val = ref_slice_BufferInfoPs3()
        val.ptr = &self.ptr.infos
        return val
    @property
    def vbuff_order(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.vbuff_order
        return val
    @property
    def ibuff_order(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.ibuff_order
        return val
    @property
    def vertex(self):
        val = IndexMap_u32__VertexBufferRefPs3()
        val.ptr = &self.ptr.vertex
        return val
    @property
    def index(self):
        val = IndexMap_u32__IndexBufferRefPs3()
        val.ptr = &self.ptr.index
        return val
    @property
    def data(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data
        return val

cdef class ModelRefPs3:
    cdef lotrc_rs.ModelRefPs3* ptr
    @property
    def info(self):
        val = ModelInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def bones(self):
        val = BonesRefPs3()
        val.ptr = &self.ptr.bones
        return val
    @property
    def mat_order(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.mat_order
        return val
    @property
    def mesh_order(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.mesh_order
        return val
    @property
    def mesh_bounding_boxes(self):
        val = ref_slice_BoundingBoxPs3()
        val.ptr = &self.ptr.mesh_bounding_boxes
        return val
    @property
    def skin_binds(self):
        val = ref_slice_Matrix4x4Ps3()
        val.ptr = &self.ptr.skin_binds
        return val
    @property
    def vals_j(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.vals_j
        return val
    @property
    def val_k_header(self):
        val = ref_slice_u16Ps3()
        val.ptr = &self.ptr.val_k_header
        return val
    @property
    def vals_k(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.vals_k
        return val
    @property
    def skin_order(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.skin_order
        return val
    @property
    def slots(self):
        val = ref_slice_Key2Ps3()
        val.ptr = &self.ptr.slots
        return val
    @property
    def slot_map(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.slot_map
        return val
    @property
    def block_header(self):
        return dereference(self.ptr.block_header)
    @property
    def block_offsets(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.block_offsets
        return val
    @property
    def blocks(self):
        val = slice_BlockRefPs3()
        val.ptr = &self.ptr.blocks
        return val
    @property
    def buffer_infos(self):
        val = ref_slice_BufferInfoPs3()
        val.ptr = &self.ptr.buffer_infos
        return val
    @property
    def vbuff_order(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.vbuff_order
        return val
    @property
    def ibuff_order(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.ibuff_order
        return val
    @property
    def vbuffs(self):
        val = IndexMap_u32_______VBuffInfoPs3()
        val.ptr = &self.ptr.vbuffs
        return val
    @property
    def ibuffs(self):
        val = IndexMap_u32_______IBuffInfoPs3()
        val.ptr = &self.ptr.ibuffs
        return val
    @property
    def mats(self):
        val = IndexMap_u32__MatRefPs3()
        val.ptr = &self.ptr.mats
        return val
    @property
    def hk_constraint(self):
        val = Option_HkConstraintRefPs3()
        val.ptr = &self.ptr.hk_constraint
        return val
    @property
    def hk_constraint_datas(self):
        val = ref_slice_HkConstraintDataPs3()
        val.ptr = &self.ptr.hk_constraint_datas
        return val
    @property
    def shapes(self):
        val = slice_ShapeRefPs3()
        val.ptr = &self.ptr.shapes
        return val
    @property
    def data(self):
        val = ModelDataRefPs3()
        val.ptr = &self.ptr.data
        return val

cdef class AnimationInfoPs3:
    cdef lotrc_rs.AnimationInfoPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def vals_num(self):
        return self.ptr.vals_num
    @property
    def vals2_num(self):
        return self.ptr.vals2_num
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def vala(self):
        return self.ptr.vala
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def data_offset(self):
        return self.ptr.data_offset
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def t_scale(self):
        return self.ptr.t_scale
    @property
    def block_starts_offset(self):
        return self.ptr.block_starts_offset
    @property
    def block_starts_num(self):
        return self.ptr.block_starts_num
    @property
    def block_starts2_offset(self):
        return self.ptr.block_starts2_offset
    @property
    def block_starts2_num(self):
        return self.ptr.block_starts2_num
    @property
    def obj_c3_offset(self):
        return self.ptr.obj_c3_offset
    @property
    def obj_c3_num(self):
        return self.ptr.obj_c3_num
    @property
    def obj_c4_offset(self):
        return self.ptr.obj_c4_offset
    @property
    def obj_c4_num(self):
        return self.ptr.obj_c4_num
    @property
    def block_offset(self):
        return self.ptr.block_offset
    @property
    def block_size(self):
        return self.ptr.block_size
    @property
    def obj3_num(self):
        return self.ptr.obj3_num
    @property
    def obj3_offset(self):
        return self.ptr.obj3_offset
    @property
    def bones_num1(self):
        return self.ptr.bones_num1
    @property
    def unk_29(self):
        return self.ptr.unk_29
    @property
    def obj1_num(self):
        return self.ptr.obj1_num
    @property
    def bones_offset(self):
        return self.ptr.bones_offset
    @property
    def unk_32(self):
        return self.ptr.unk_32
    @property
    def obj1_offset(self):
        return self.ptr.obj1_offset
    @property
    def obj2_offset(self):
        return self.ptr.obj2_offset
    @property
    def obj2_num(self):
        return self.ptr.obj2_num
    @property
    def obj5_offset(self):
        return self.ptr.obj5_offset

cdef class ref_slice_Obj3Ps3:
    cdef lotrc_rs.ref_slice_Obj3Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj3Ps3()
        val.ptr = lotrc_rs.ref_slice_Obj3Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Obj3Ps3_len(self.ptr)

cdef class Obj5HeaderPs3:
    cdef lotrc_rs.Obj5HeaderPs3* ptr
    @property
    def obj_a_num(self):
        return self.ptr.obj_a_num
    @property
    def obj_a_offset(self):
        return self.ptr.obj_a_offset
    @property
    def obj_b_num(self):
        return self.ptr.obj_b_num
    @property
    def obj_b_offset(self):
        return self.ptr.obj_b_offset

cdef class ref_slice_Obj5ValPs3:
    cdef lotrc_rs.ref_slice_Obj5ValPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Obj5ValPs3()
        val.ptr = lotrc_rs.ref_slice_Obj5ValPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Obj5ValPs3_len(self.ptr)

cdef class Option_BlocksRefPs3:
    cdef lotrc_rs.Option_BlocksRefPs3* ptr
    def get(self):
        val = BlocksRefPs3()
        val.ptr = lotrc_rs.Option_BlocksRefPs3_get(self.ptr)
        return val

cdef class AnimationRefPs3:
    cdef lotrc_rs.AnimationRefPs3* ptr
    @property
    def info(self):
        val = AnimationInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def obj1(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.obj1
        return val
    @property
    def obj2(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.obj2
        return val
    @property
    def obj3(self):
        val = ref_slice_Obj3Ps3()
        val.ptr = &self.ptr.obj3
        return val
    @property
    def bones(self):
        val = ref_slice_CrcPs3()
        val.ptr = &self.ptr.bones
        return val
    @property
    def obj5_header(self):
        val = Obj5HeaderPs3()
        val.ptr = self.ptr.obj5_header
        return val
    @property
    def obj5_a(self):
        val = ref_slice_Obj5ValPs3()
        val.ptr = &self.ptr.obj5_a
        return val
    @property
    def obj5_b(self):
        val = ref_slice_Obj5ValPs3()
        val.ptr = &self.ptr.obj5_b
        return val
    @property
    def blocks(self):
        val = Option_BlocksRefPs3()
        val.ptr = &self.ptr.blocks
        return val
    @property
    def size(self):
        return self.ptr.size

cdef class DataRefPs3:
    cdef lotrc_rs.DataRefPs3* ptr
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val

cdef class EffectInfoPs3:
    cdef lotrc_rs.EffectInfoPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size

cdef class EffectRefPs3:
    cdef lotrc_rs.EffectRefPs3* ptr
    @property
    def info(self):
        val = EffectInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def vals(self):
        val = GameObjsRefPs3()
        val.ptr = &self.ptr.vals
        return val

cdef class IndexMap_u32__ref_slice_u16Ps3:
    cdef lotrc_rs.IndexMap_u32__ref_slice_u16Ps3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = ref_slice_u16Ps3()
        val.ptr = lotrc_rs.IndexMap_u32__ref_slice_u16Ps3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__ref_slice_u16Ps3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__ref_slice_u16Ps3_keys(self.ptr, keys.ptr)

cdef class LangStringsRefPs3:
    cdef lotrc_rs.LangStringsRefPs3* ptr
    @property
    def strings(self):
        val = IndexMap_u32__ref_slice_u16Ps3()
        val.ptr = &self.ptr.strings
        return val

cdef class ObjHeaderPs3:
    cdef lotrc_rs.ObjHeaderPs3* ptr
    @property
    def layer(self):
        return self.ptr.layer
    @property
    def key(self):
        return self.ptr.key
    @property
    def size(self):
        return self.ptr.size
    @property
    def z3(self):
        return self.ptr.z3
    @property
    def z4(self):
        return self.ptr.z4

cdef class IndexMap_u32__BaseTypeRefPs3:
    cdef lotrc_rs.IndexMap_u32__BaseTypeRefPs3* ptr
    def get(self, lotrc_rs.uint32_t key):
        val = BaseTypeRefPs3()
        val.ptr = lotrc_rs.IndexMap_u32__BaseTypeRefPs3_get(self.ptr, &key)
        return val
    def len(self):
        return lotrc_rs.IndexMap_u32__BaseTypeRefPs3_len(self.ptr)
    def keys(self, mut_slice_u32 keys):
        lotrc_rs.IndexMap_u32__BaseTypeRefPs3_keys(self.ptr, keys.ptr)

cdef class ObjRefPs3:
    cdef lotrc_rs.ObjRefPs3* ptr
    @property
    def header(self):
        val = ObjHeaderPs3()
        val.ptr = self.ptr.header
        return val
    @property
    def fields(self):
        val = IndexMap_u32__BaseTypeRefPs3()
        val.ptr = &self.ptr.fields
        return val

cdef class RadiosityValsInfoPs3:
    cdef lotrc_rs.RadiosityValsInfoPs3* ptr
    @property
    def guid(self):
        return self.ptr.guid
    @property
    def num(self):
        return self.ptr.num
    @property
    def offset(self):
        return self.ptr.offset

cdef class RadiosityValsRefPs3:
    cdef lotrc_rs.RadiosityValsRefPs3* ptr
    @property
    def info(self):
        val = RadiosityValsInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def offs(self):
        val = ref_slice_i32Ps3()
        val.ptr = &self.ptr.offs
        return val

cdef class ref_slice_SSAValPs3:
    cdef lotrc_rs.ref_slice_SSAValPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SSAValPs3()
        val.ptr = lotrc_rs.ref_slice_SSAValPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SSAValPs3_len(self.ptr)

cdef class slice_ref_slice_u16Ps3:
    cdef lotrc_rs.slice_ref_slice_u16Ps3* ptr

cdef class SSARefPs3:
    cdef lotrc_rs.SSARefPs3* ptr
    @property
    def vals(self):
        val = ref_slice_SSAValPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def strings(self):
        val = slice_ref_slice_u16Ps3()
        val.ptr = &self.ptr.strings
        return val

cdef class TextureInfoPs3:
    cdef lotrc_rs.TextureInfoPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def asset_key(self):
        return self.ptr.asset_key
    @property
    def asset_type(self):
        return self.ptr.asset_type
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def format(self):
        return self.ptr.format
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def width(self):
        return self.ptr.width
    @property
    def height(self):
        return self.ptr.height
    @property
    def depth(self):
        return self.ptr.depth
    @property
    def levels(self):
        return self.ptr.levels
    @property
    def unk_16_1(self):
        return self.ptr.unk_16_1
    @property
    def unk_16_2(self):
        return self.ptr.unk_16_2
    @property
    def unk_16_3(self):
        return self.ptr.unk_16_3
    @property
    def unk_16_4(self):
        return self.ptr.unk_16_4
    @property
    def unk_16_5(self):
        return self.ptr.unk_16_5
    @property
    def unk_16_6(self):
        return self.ptr.unk_16_6
    @property
    def unk_16_7(self):
        return self.ptr.unk_16_7
    @property
    def unk_16_8(self):
        return self.ptr.unk_16_8
    @property
    def unk_16_9(self):
        return self.ptr.unk_16_9
    @property
    def unk_16_10(self):
        return self.ptr.unk_16_10
    @property
    def unk_16_11(self):
        return self.ptr.unk_16_11
    @property
    def unk_16_12(self):
        return self.ptr.unk_16_12
    @property
    def unk_16_13(self):
        return self.ptr.unk_16_13
    @property
    def unk_16_14(self):
        return self.ptr.unk_16_14
    @property
    def unk_16_15(self):
        return self.ptr.unk_16_15
    @property
    def unk_16_16(self):
        return self.ptr.unk_16_16

cdef class TextureRefPs3:
    cdef lotrc_rs.TextureRefPs3* ptr
    @property
    def info(self):
        val = TextureInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def data0(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data0
        return val
    @property
    def data1(self):
        val = CompressedDataRef()
        val.ptr = self.ptr.data1
        return val

cdef class TypeHeaderPs3:
    cdef lotrc_rs.TypeHeaderPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def size(self):
        return self.ptr.size
    @property
    def fields(self):
        return self.ptr.fields

cdef class ref_slice_TypeFieldPs3:
    cdef lotrc_rs.ref_slice_TypeFieldPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TypeFieldPs3()
        val.ptr = lotrc_rs.ref_slice_TypeFieldPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_TypeFieldPs3_len(self.ptr)

cdef class TypeRefPs3:
    cdef lotrc_rs.TypeRefPs3* ptr
    @property
    def header(self):
        val = TypeHeaderPs3()
        val.ptr = self.ptr.header
        return val
    @property
    def fields(self):
        val = ref_slice_TypeFieldPs3()
        val.ptr = &self.ptr.fields
        return val

cdef class slice_FoliageRefPs3:
    cdef lotrc_rs.slice_FoliageRefPs3* ptr

cdef class Vector2Ps3:
    cdef lotrc_rs.Vector2Ps3* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def y(self):
        return self.ptr.y

cdef class Vector4Ps3:
    cdef lotrc_rs.Vector4Ps3* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def y(self):
        return self.ptr.y
    @property
    def z(self):
        return self.ptr.z
    @property
    def w(self):
        return self.ptr.w

cdef class Matrix4x4Ps3:
    cdef lotrc_rs.Matrix4x4Ps3* ptr
    @property
    def x(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.x
        return val
    @property
    def y(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.y
        return val
    @property
    def z(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.z
        return val
    @property
    def w(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.w
        return val

cdef class ref_slice_U32Ps3:
    cdef lotrc_rs.ref_slice_U32Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_U32Ps3_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_U32Ps3_len(self.ptr)

cdef class ref_slice_Vector4Ps3:
    cdef lotrc_rs.ref_slice_Vector4Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Vector4Ps3()
        val.ptr = lotrc_rs.ref_slice_Vector4Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Vector4Ps3_len(self.ptr)

cdef class ref_slice_WeightPs3:
    cdef lotrc_rs.ref_slice_WeightPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = WeightPs3()
        val.ptr = lotrc_rs.ref_slice_WeightPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_WeightPs3_len(self.ptr)

cdef class BaseTypeRefPs3:
    cdef lotrc_rs.BaseTypeRefPs3* ptr

cdef class BlockHeader1Ps3:
    cdef lotrc_rs.BlockHeader1Ps3* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3

cdef class BlockHeader2Ps3:
    cdef lotrc_rs.BlockHeader2Ps3* ptr
    @property
    def n(self):
        return self.ptr.n
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4

cdef class ref_slice_BlockValAPs3:
    cdef lotrc_rs.ref_slice_BlockValAPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BlockValAPs3()
        val.ptr = lotrc_rs.ref_slice_BlockValAPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BlockValAPs3_len(self.ptr)

cdef class ref_slice_BlockValBPs3:
    cdef lotrc_rs.ref_slice_BlockValBPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = BlockValBPs3()
        val.ptr = lotrc_rs.ref_slice_BlockValBPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_BlockValBPs3_len(self.ptr)

cdef class BlockRefPs3:
    cdef lotrc_rs.BlockRefPs3* ptr
    @property
    def info1(self):
        val = BlockHeader1Ps3()
        val.ptr = self.ptr.info1
        return val
    @property
    def info2(self):
        val = BlockHeader2Ps3()
        val.ptr = self.ptr.info2
        return val
    @property
    def vals_a(self):
        val = ref_slice_BlockValAPs3()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def vals_b(self):
        val = ref_slice_BlockValAPs3()
        val.ptr = &self.ptr.vals_b
        return val
    @property
    def vals_c(self):
        val = ref_slice_BlockValBPs3()
        val.ptr = &self.ptr.vals_c
        return val
    @property
    def pad(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.pad
        return val

cdef class ShapeInfoPs3:
    cdef lotrc_rs.ShapeInfoPs3* ptr
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def translation(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def hk_shape_num(self):
        return self.ptr.hk_shape_num
    @property
    def hk_shape_offset(self):
        return self.ptr.hk_shape_offset
    @property
    def unk_29a(self):
        return self.ptr.unk_29a
    @property
    def unk_29b(self):
        return self.ptr.unk_29b
    @property
    def unk_29c(self):
        return self.ptr.unk_29c
    @property
    def unk_29d(self):
        return self.ptr.unk_29d
    @property
    def unk_30(self):
        return self.ptr.unk_30

cdef class Option_ShapeExtraRefPs3:
    cdef lotrc_rs.Option_ShapeExtraRefPs3* ptr
    def get(self):
        val = ShapeExtraRefPs3()
        val.ptr = lotrc_rs.Option_ShapeExtraRefPs3_get(self.ptr)
        return val

cdef class slice_HkShapeRefPs3:
    cdef lotrc_rs.slice_HkShapeRefPs3* ptr

cdef class ShapeRefPs3:
    cdef lotrc_rs.ShapeRefPs3* ptr
    @property
    def info(self):
        val = ShapeInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def extra(self):
        val = Option_ShapeExtraRefPs3()
        val.ptr = &self.ptr.extra
        return val
    @property
    def hk_shapes(self):
        val = slice_HkShapeRefPs3()
        val.ptr = &self.ptr.hk_shapes
        return val

cdef class BoxShapePs3:
    cdef lotrc_rs.BoxShapePs3* ptr
    @property
    def translation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def half_extents(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.half_extents
        return val
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class SphereShapePs3:
    cdef lotrc_rs.SphereShapePs3* ptr
    @property
    def translation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def radius(self):
        return self.ptr.radius
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class CapsuleShapePs3:
    cdef lotrc_rs.CapsuleShapePs3* ptr
    @property
    def translation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def point1(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.point1
        return val
    @property
    def point2(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.point2
        return val
    @property
    def radius(self):
        return self.ptr.radius
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class CylinderShapePs3:
    cdef lotrc_rs.CylinderShapePs3* ptr
    @property
    def translation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def point1(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.point1
        return val
    @property
    def point2(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.point2
        return val
    @property
    def radius(self):
        return self.ptr.radius
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class ConvexVerticesInfoPs3:
    cdef lotrc_rs.ConvexVerticesInfoPs3* ptr
    @property
    def translation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def norm_num(self):
        return self.ptr.norm_num
    @property
    def norms_offset(self):
        return self.ptr.norms_offset
    @property
    def vert_num(self):
        return self.ptr.vert_num
    @property
    def verts_offset(self):
        return self.ptr.verts_offset
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class ref_slice_Vector3Ps3:
    cdef lotrc_rs.ref_slice_Vector3Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = Vector3Ps3()
        val.ptr = lotrc_rs.ref_slice_Vector3Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_Vector3Ps3_len(self.ptr)

cdef class ConvexVerticesRefPs3:
    cdef lotrc_rs.ConvexVerticesRefPs3* ptr
    @property
    def info(self):
        val = ConvexVerticesInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def norms(self):
        val = ref_slice_Vector4Ps3()
        val.ptr = &self.ptr.norms
        return val
    @property
    def verts(self):
        val = ref_slice_Vector3Ps3()
        val.ptr = &self.ptr.verts
        return val

cdef class BVTreeMeshInfoPs3:
    cdef lotrc_rs.BVTreeMeshInfoPs3* ptr
    @property
    def translation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.offset
        return val
    @property
    def tree_scale(self):
        return self.ptr.tree_scale
    @property
    def tree_size(self):
        return self.ptr.tree_size
    @property
    def tree_offset(self):
        return self.ptr.tree_offset
    @property
    def vert_num(self):
        return self.ptr.vert_num
    @property
    def verts_offset(self):
        return self.ptr.verts_offset
    @property
    def tri_num(self):
        return self.ptr.tri_num
    @property
    def inds_offset(self):
        return self.ptr.inds_offset

cdef class BVTreeMeshRefPs3:
    cdef lotrc_rs.BVTreeMeshRefPs3* ptr
    @property
    def info(self):
        val = BVTreeMeshInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def tree(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.tree
        return val
    @property
    def verts(self):
        val = ref_slice_Vector3Ps3()
        val.ptr = &self.ptr.verts
        return val
    @property
    def inds(self):
        val = ref_slice_u16Ps3()
        val.ptr = &self.ptr.inds
        return val

cdef class HkShapeInfoPs3:
    cdef lotrc_rs.HkShapeInfoPs3* ptr
    @property
    def unk_0(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.unk_0
        return val
    @property
    def unk_4(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.unk_4
        return val
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19

cdef class HkShapeRefPs3:
    cdef lotrc_rs.HkShapeRefPs3* ptr

cdef class FoliageInfoPs3:
    cdef lotrc_rs.FoliageInfoPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def lb_w(self):
        return self.ptr.lb_w
    @property
    def lb_h(self):
        return self.ptr.lb_h
    @property
    def ub_w(self):
        return self.ptr.ub_w
    @property
    def ub_h(self):
        return self.ptr.ub_h
    @property
    def scale(self):
        return self.ptr.scale
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def key_mesh(self):
        return self.ptr.key_mesh
    @property
    def key_mesh_lod1(self):
        return self.ptr.key_mesh_lod1
    @property
    def key_mesh_lod2(self):
        return self.ptr.key_mesh_lod2
    @property
    def color(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.color
        return val
    @property
    def lod1a(self):
        return self.ptr.lod1a
    @property
    def lod1b(self):
        return self.ptr.lod1b
    @property
    def lod2a(self):
        return self.ptr.lod2a
    @property
    def lod2b(self):
        return self.ptr.lod2b
    @property
    def lod_max(self):
        return self.ptr.lod_max

cdef class ref_slice_FoliageValPs3:
    cdef lotrc_rs.ref_slice_FoliageValPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = FoliageValPs3()
        val.ptr = lotrc_rs.ref_slice_FoliageValPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_FoliageValPs3_len(self.ptr)

cdef class FoliageRefPs3:
    cdef lotrc_rs.FoliageRefPs3* ptr
    @property
    def info(self):
        val = FoliageInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def vals(self):
        val = ref_slice_FoliageValPs3()
        val.ptr = &self.ptr.vals
        return val

cdef class slice_BlockValARefPs3:
    cdef lotrc_rs.slice_BlockValARefPs3* ptr

cdef class slice_Obj1RefPs3:
    cdef lotrc_rs.slice_Obj1RefPs3* ptr

cdef class BlockValRefPs3:
    cdef lotrc_rs.BlockValRefPs3* ptr
    @property
    def vals_a(self):
        val = slice_BlockValARefPs3()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def vals_b(self):
        val = slice_Obj1RefPs3()
        val.ptr = &self.ptr.vals_b
        return val

cdef class slice_BlockValRefPs3:
    cdef lotrc_rs.slice_BlockValRefPs3* ptr

cdef class CrowdItemHeaderPs3:
    cdef lotrc_rs.CrowdItemHeaderPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def key_main(self):
        return self.ptr.key_main
    @property
    def key_right(self):
        return self.ptr.key_right
    @property
    def key_left(self):
        return self.ptr.key_left
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def animation_num(self):
        return self.ptr.animation_num
    @property
    def instance_num(self):
        return self.ptr.instance_num

cdef class ref_slice_CrowdValPs3:
    cdef lotrc_rs.ref_slice_CrowdValPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = CrowdValPs3()
        val.ptr = lotrc_rs.ref_slice_CrowdValPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_CrowdValPs3_len(self.ptr)
    def get_arr(self):
        cdef lotrc_rs.CrowdValPs3[:] arr = <lotrc_rs.CrowdValPs3[:lotrc_rs.ref_slice_CrowdValPs3_len(self.ptr)]> lotrc_rs.ref_slice_CrowdValPs3_get(self.ptr, 0)
        return arr

cdef class CrowdItemRefPs3:
    cdef lotrc_rs.CrowdItemRefPs3* ptr
    @property
    def header(self):
        val = CrowdItemHeaderPs3()
        val.ptr = self.ptr.header
        return val
    @property
    def animations(self):
        val = ref_slice_CrcPs3()
        val.ptr = &self.ptr.animations
        return val
    @property
    def instances(self):
        val = ref_slice_CrowdValPs3()
        val.ptr = &self.ptr.instances
        return val

cdef class slice_CrowdItemRefPs3:
    cdef lotrc_rs.slice_CrowdItemRefPs3* ptr

cdef class HkConstraintBoneRefPs3:
    cdef lotrc_rs.HkConstraintBoneRefPs3* ptr
    @property
    def name(self):
        val = string()
        val.ptr = &self.ptr.name
        return val
    @property
    def start(self):
        return self.ptr.start
    @property
    def val(self):
        return self.ptr.val

cdef class slice_HkConstraintBoneRefPs3:
    cdef lotrc_rs.slice_HkConstraintBoneRefPs3* ptr

cdef class ref_slice_f32Ps3:
    cdef lotrc_rs.ref_slice_f32Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_f32Ps3_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_f32Ps3_len(self.ptr)

cdef class AnimVals1RefPs3:
    cdef lotrc_rs.AnimVals1RefPs3* ptr

cdef class Obj1RefPs3:
    cdef lotrc_rs.Obj1RefPs3* ptr
    @property
    def flags(self):
        return self.ptr.flags
    @property
    def s2(self):
        return self.ptr.s2
    @property
    def s1(self):
        return self.ptr.s1
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val
    @property
    def vals_a(self):
        val = ref_slice_f32Ps3()
        val.ptr = &self.ptr.vals_a
        return val
    @property
    def vals(self):
        val = AnimVals1RefPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def size(self):
        return self.ptr.size

cdef class ref_slice_RotationPolar32Ps3:
    cdef lotrc_rs.ref_slice_RotationPolar32Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationPolar32Ps3()
        val.ptr = lotrc_rs.ref_slice_RotationPolar32Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationPolar32Ps3_len(self.ptr)

cdef class ref_slice_RotationThreeComp40Ps3:
    cdef lotrc_rs.ref_slice_RotationThreeComp40Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationThreeComp40Ps3()
        val.ptr = lotrc_rs.ref_slice_RotationThreeComp40Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationThreeComp40Ps3_len(self.ptr)

cdef class ref_slice_RotationThreeComp48Ps3:
    cdef lotrc_rs.ref_slice_RotationThreeComp48Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationThreeComp48Ps3()
        val.ptr = lotrc_rs.ref_slice_RotationThreeComp48Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationThreeComp48Ps3_len(self.ptr)

cdef class ref_slice_RotationThreeComp24Ps3:
    cdef lotrc_rs.ref_slice_RotationThreeComp24Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationThreeComp24Ps3()
        val.ptr = lotrc_rs.ref_slice_RotationThreeComp24Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationThreeComp24Ps3_len(self.ptr)

cdef class ref_slice_RotationStraight16Ps3:
    cdef lotrc_rs.ref_slice_RotationStraight16Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationStraight16Ps3()
        val.ptr = lotrc_rs.ref_slice_RotationStraight16Ps3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationStraight16Ps3_len(self.ptr)

cdef class ref_slice_RotationUncompressedPs3:
    cdef lotrc_rs.ref_slice_RotationUncompressedPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = RotationUncompressedPs3()
        val.ptr = lotrc_rs.ref_slice_RotationUncompressedPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_RotationUncompressedPs3_len(self.ptr)

cdef class RotationQuantizationRefPs3:
    cdef lotrc_rs.RotationQuantizationRefPs3* ptr

cdef class Obj2RefPs3:
    cdef lotrc_rs.Obj2RefPs3* ptr
    @property
    def flags(self):
        return self.ptr.flags
    @property
    def s2(self):
        return self.ptr.s2
    @property
    def s1(self):
        return self.ptr.s1
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val
    @property
    def vals(self):
        val = RotationQuantizationRefPs3()
        val.ptr = &self.ptr.vals
        return val
    @property
    def size(self):
        return self.ptr.size

cdef class BlockValARefPs3:
    cdef lotrc_rs.BlockValARefPs3* ptr
    @property
    def a(self):
        val = Obj1RefPs3()
        val.ptr = &self.ptr.a
        return val
    @property
    def b(self):
        val = Obj2RefPs3()
        val.ptr = &self.ptr.b
        return val
    @property
    def c(self):
        val = Obj1RefPs3()
        val.ptr = &self.ptr.c
        return val

cdef class BufferInfoPs3:
    cdef lotrc_rs.BufferInfoPs3* ptr
    @property
    def vbuff_info_offset(self):
        return self.ptr.vbuff_info_offset
    @property
    def vbuff_info_offset_2(self):
        return self.ptr.vbuff_info_offset_2
    @property
    def vbuff_info_offset_3(self):
        return self.ptr.vbuff_info_offset_3
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def ibuff_info_offset(self):
        return self.ptr.ibuff_info_offset
    @property
    def v_size(self):
        return self.ptr.v_size
    @property
    def vbuff_size(self):
        return self.ptr.vbuff_size

cdef class HkConstraintDataPs3:
    cdef lotrc_rs.HkConstraintDataPs3* ptr
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11
    @property
    def unk_12(self):
        return self.ptr.unk_12
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16
    @property
    def unk_17(self):
        return self.ptr.unk_17
    @property
    def unk_18(self):
        return self.ptr.unk_18
    @property
    def unk_19(self):
        return self.ptr.unk_19
    @property
    def unk_20(self):
        return self.ptr.unk_20
    @property
    def unk_21(self):
        return self.ptr.unk_21
    @property
    def unk_22(self):
        return self.ptr.unk_22
    @property
    def unk_23(self):
        return self.ptr.unk_23
    @property
    def unk_24(self):
        return self.ptr.unk_24
    @property
    def unk_25(self):
        return self.ptr.unk_25
    @property
    def unk_26(self):
        return self.ptr.unk_26
    @property
    def unk_27(self):
        return self.ptr.unk_27
    @property
    def unk_28(self):
        return self.ptr.unk_28

cdef class Key2Ps3:
    cdef lotrc_rs.Key2Ps3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def val(self):
        return self.ptr.val

cdef class BlockValAPs3:
    cdef lotrc_rs.BlockValAPs3* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def unk_10(self):
        return self.ptr.unk_10
    @property
    def unk_11(self):
        return self.ptr.unk_11

cdef class BlockValBPs3:
    cdef lotrc_rs.BlockValBPs3* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5

cdef class AnimationBlockInfoPs3:
    cdef lotrc_rs.AnimationBlockInfoPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def guid(self):
        return self.ptr.guid
    @property
    def key_name(self):
        return self.ptr.key_name
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size
    @property
    def size_comp(self):
        return self.ptr.size_comp
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8

cdef class AssetHandlePs3:
    cdef lotrc_rs.AssetHandlePs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size
    @property
    def size_comp(self):
        return self.ptr.size_comp
    @property
    def kind(self):
        return self.ptr.kind

cdef class BlockAValPs3:
    cdef lotrc_rs.BlockAValPs3* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def gamemodemask(self):
        return self.ptr.gamemodemask
    @property
    def key(self):
        return self.ptr.key
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6

cdef class GFXBlockInfoPs3:
    cdef lotrc_rs.GFXBlockInfoPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size

cdef class HkConstraintInfoPs3:
    cdef lotrc_rs.HkConstraintInfoPs3* ptr
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def bone_parents_offset(self):
        return self.ptr.bone_parents_offset
    @property
    def bone_parents_num(self):
        return self.ptr.bone_parents_num
    @property
    def bone_names_offset(self):
        return self.ptr.bone_names_offset
    @property
    def bone_names_num(self):
        return self.ptr.bone_names_num
    @property
    def bone_transforms_offset(self):
        return self.ptr.bone_transforms_offset
    @property
    def bone_transforms_num(self):
        return self.ptr.bone_transforms_num
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def unk_8(self):
        return self.ptr.unk_8
    @property
    def unk_9(self):
        return self.ptr.unk_9
    @property
    def bones_offset(self):
        return self.ptr.bones_offset
    @property
    def bones_num(self):
        return self.ptr.bones_num
    @property
    def bone_order_num(self):
        return self.ptr.bone_order_num
    @property
    def bone_order_offset(self):
        return self.ptr.bone_order_offset
    @property
    def unk_13(self):
        return self.ptr.unk_13
    @property
    def unk_14(self):
        return self.ptr.unk_14
    @property
    def vals2_num(self):
        return self.ptr.vals2_num
    @property
    def vals2_offset(self):
        return self.ptr.vals2_offset
    @property
    def unk_17(self):
        return self.ptr.unk_17

cdef class Obj0Ps3:
    cdef lotrc_rs.Obj0Ps3* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def key(self):
        return self.ptr.key

cdef class ObjAPs3:
    cdef lotrc_rs.ObjAPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def size(self):
        return self.ptr.size
    @property
    def size_comp(self):
        return self.ptr.size_comp
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def kind(self):
        return self.ptr.kind

cdef class PFieldInfoPs3:
    cdef lotrc_rs.PFieldInfoPs3* ptr
    @property
    def link_guid(self):
        return self.ptr.link_guid
    @property
    def gamemode_guid(self):
        return self.ptr.gamemode_guid
    @property
    def width(self):
        return self.ptr.width
    @property
    def height(self):
        return self.ptr.height
    @property
    def offset(self):
        return self.ptr.offset

cdef class StringKeysValPs3:
    cdef lotrc_rs.StringKeysValPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset

cdef class SubBlocksBlockHeaderPs3:
    cdef lotrc_rs.SubBlocksBlockHeaderPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def offset(self):
        return self.ptr.offset
    @property
    def size(self):
        return self.ptr.size

cdef class Obj3Ps3:
    cdef lotrc_rs.Obj3Ps3* ptr
    @property
    def t(self):
        return self.ptr.t
    @property
    def event(self):
        return self.ptr.event
    @property
    def dat_2(self):
        return self.ptr.dat_2
    @property
    def dat_3(self):
        return self.ptr.dat_3
    @property
    def dat_4(self):
        return self.ptr.dat_4
    @property
    def dat_5(self):
        return self.ptr.dat_5
    @property
    def dat_6(self):
        return self.ptr.dat_6
    @property
    def dat_7(self):
        return self.ptr.dat_7
    @property
    def dat_8(self):
        return self.ptr.dat_8
    @property
    def dat_9(self):
        return self.ptr.dat_9
    @property
    def dat_10(self):
        return self.ptr.dat_10

cdef class Obj5ValPs3:
    cdef lotrc_rs.Obj5ValPs3* ptr
    @property
    def unk_0(self):
        return self.ptr.unk_0
    @property
    def unk_1(self):
        return self.ptr.unk_1
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def unk_4(self):
        return self.ptr.unk_4
    @property
    def unk_5(self):
        return self.ptr.unk_5
    @property
    def unk_6(self):
        return self.ptr.unk_6

cdef class SSAValPs3:
    cdef lotrc_rs.SSAValPs3* ptr
    @property
    def t_start(self):
        return self.ptr.t_start
    @property
    def t_end(self):
        return self.ptr.t_end
    @property
    def unk_2(self):
        return self.ptr.unk_2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def off(self):
        return self.ptr.off

cdef class TypeFieldPs3:
    cdef lotrc_rs.TypeFieldPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def kind(self):
        return self.ptr.kind
    @property
    def offset(self):
        return self.ptr.offset

cdef class FoliageValPs3:
    cdef lotrc_rs.FoliageValPs3* ptr
    @property
    def height(self):
        return self.ptr.height
    @property
    def var_mask(self):
        return self.ptr.var_mask
    @property
    def slope_x(self):
        return self.ptr.slope_x
    @property
    def slope_z(self):
        return self.ptr.slope_z

cdef class WeightPs3:
    cdef lotrc_rs.WeightPs3* ptr
    @property
    def x(self):
        return self.ptr.x
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c
    @property
    def d(self):
        return self.ptr.d

cdef class AtlasUVValPs3:
    cdef lotrc_rs.AtlasUVValPs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def vals(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.vals
        return val

cdef class ref_slice_AtlasUVValPs3:
    cdef lotrc_rs.ref_slice_AtlasUVValPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = AtlasUVValPs3()
        val.ptr = lotrc_rs.ref_slice_AtlasUVValPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_AtlasUVValPs3_len(self.ptr)

cdef class SprayInstancePs3:
    cdef lotrc_rs.SprayInstancePs3* ptr
    @property
    def key(self):
        return self.ptr.key
    @property
    def tex1(self):
        return self.ptr.tex1
    @property
    def tex2(self):
        return self.ptr.tex2
    @property
    def unk_3(self):
        return self.ptr.unk_3
    @property
    def width(self):
        return self.ptr.width
    @property
    def height(self):
        return self.ptr.height
    @property
    def unk_6(self):
        return self.ptr.unk_6
    @property
    def unk_7(self):
        return self.ptr.unk_7
    @property
    def size_w(self):
        return self.ptr.size_w
    @property
    def size_h(self):
        return self.ptr.size_h
    @property
    def scale_w(self):
        return self.ptr.scale_w
    @property
    def scale_h(self):
        return self.ptr.scale_h
    @property
    def delay(self):
        return self.ptr.delay
    @property
    def stride_x(self):
        return self.ptr.stride_x
    @property
    def stride_y(self):
        return self.ptr.stride_y
    @property
    def unk_15(self):
        return self.ptr.unk_15
    @property
    def unk_16(self):
        return self.ptr.unk_16

cdef class ref_slice_SprayInstancePs3:
    cdef lotrc_rs.ref_slice_SprayInstancePs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SprayInstancePs3()
        val.ptr = lotrc_rs.ref_slice_SprayInstancePs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SprayInstancePs3_len(self.ptr)

cdef class SprayValPs3:
    cdef lotrc_rs.SprayValPs3* ptr
    @property
    def position(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.position
        return val
    @property
    def scale(self):
        return self.ptr.scale
    @property
    def instance(self):
        return self.ptr.instance
    @property
    def rotation(self):
        return self.ptr.rotation

cdef class ref_slice_SprayValPs3:
    cdef lotrc_rs.ref_slice_SprayValPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = SprayValPs3()
        val.ptr = lotrc_rs.ref_slice_SprayValPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_SprayValPs3_len(self.ptr)

cdef class TRSPs3:
    cdef lotrc_rs.TRSPs3* ptr
    @property
    def translation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.translation
        return val
    @property
    def rotation(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.rotation
        return val
    @property
    def scale(self):
        val = Vector4Ps3()
        val.ptr = &self.ptr.scale
        return val

cdef class ref_slice_TRSPs3:
    cdef lotrc_rs.ref_slice_TRSPs3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        val = TRSPs3()
        val.ptr = lotrc_rs.ref_slice_TRSPs3_get(self.ptr, idx)
        return val
    def len(self):
        return lotrc_rs.ref_slice_TRSPs3_len(self.ptr)

cdef class ref_slice_i16Ps3:
    cdef lotrc_rs.ref_slice_i16Ps3* ptr
    def get(self, lotrc_rs.uintptr_t idx):
        return dereference(lotrc_rs.ref_slice_i16Ps3_get(self.ptr, idx))
    def len(self):
        return lotrc_rs.ref_slice_i16Ps3_len(self.ptr)

cdef class CrowdValPs3:
    cdef lotrc_rs.CrowdValPs3* ptr
    @property
    def position(self):
        val = Vector3Ps3()
        val.ptr = &self.ptr.position
        return val
    @property
    def rotation(self):
        return self.ptr.rotation
    @property
    def lod(self):
        return self.ptr.lod

cdef class RotationPolar32Ps3:
    cdef lotrc_rs.RotationPolar32Ps3* ptr
    @property
    def a(self):
        return self.ptr.a

cdef class RotationStraight16Ps3:
    cdef lotrc_rs.RotationStraight16Ps3* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b

cdef class RotationThreeComp24Ps3:
    cdef lotrc_rs.RotationThreeComp24Ps3* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c

cdef class RotationThreeComp40Ps3:
    cdef lotrc_rs.RotationThreeComp40Ps3* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c
    @property
    def d(self):
        return self.ptr.d
    @property
    def e(self):
        return self.ptr.e

cdef class RotationThreeComp48Ps3:
    cdef lotrc_rs.RotationThreeComp48Ps3* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c

cdef class RotationUncompressedPs3:
    cdef lotrc_rs.RotationUncompressedPs3* ptr
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b
    @property
    def c(self):
        return self.ptr.c
    @property
    def d(self):
        return self.ptr.d

cdef class HkConstraintRefPs3:
    cdef lotrc_rs.HkConstraintRefPs3* ptr
    @property
    def info(self):
        val = HkConstraintInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def bone_parents(self):
        val = ref_slice_i16Ps3()
        val.ptr = &self.ptr.bone_parents
        return val
    @property
    def bone_names(self):
        val = slice_HkConstraintBoneRefPs3()
        val.ptr = &self.ptr.bone_names
        return val
    @property
    def name_offsets(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.name_offsets
        return val
    @property
    def bone_transforms(self):
        val = ref_slice_TRSPs3()
        val.ptr = &self.ptr.bone_transforms
        return val
    @property
    def bones(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.bones
        return val
    @property
    def bones_order(self):
        val = ref_slice_Key2Ps3()
        val.ptr = &self.ptr.bones_order
        return val
    @property
    def vals2(self):
        val = ref_slice_f32Ps3()
        val.ptr = &self.ptr.vals2
        return val

cdef class ShapeExtraInfoPs3:
    cdef lotrc_rs.ShapeExtraInfoPs3* ptr
    @property
    def size(self):
        return self.ptr.size
    @property
    def scale(self):
        return self.ptr.scale
    @property
    def a(self):
        return self.ptr.a
    @property
    def b(self):
        return self.ptr.b

cdef class ShapeExtraRefPs3:
    cdef lotrc_rs.ShapeExtraRefPs3* ptr
    @property
    def info(self):
        val = ShapeExtraInfoPs3()
        val.ptr = self.ptr.info
        return val
    @property
    def offs(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.offs
        return val
    @property
    def data(self):
        val = ref_slice_u8()
        val.ptr = &self.ptr.data
        return val

cdef class AtlasUVRefPs3:
    cdef lotrc_rs.AtlasUVRefPs3* ptr
    @property
    def vals(self):
        val = ref_slice_AtlasUVValPs3()
        val.ptr = &self.ptr.vals
        return val

cdef class BlocksRefPs3:
    cdef lotrc_rs.BlocksRefPs3* ptr
    @property
    def block_starts(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.block_starts
        return val
    @property
    def block_starts2(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.block_starts2
        return val
    @property
    def obj_c3(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.obj_c3
        return val
    @property
    def obj_c4(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.obj_c4
        return val
    @property
    def blocks(self):
        val = slice_BlockValRefPs3()
        val.ptr = &self.ptr.blocks
        return val

cdef class CrowdHeaderPs3:
    cdef lotrc_rs.CrowdHeaderPs3* ptr
    @property
    def const0x65(self):
        return self.ptr.const0x65
    @property
    def n(self):
        return self.ptr.n

cdef class CrowdRefPs3:
    cdef lotrc_rs.CrowdRefPs3* ptr
    @property
    def header(self):
        val = CrowdHeaderPs3()
        val.ptr = self.ptr.header
        return val
    @property
    def offs(self):
        val = ref_slice_u32Ps3()
        val.ptr = &self.ptr.offs
        return val
    @property
    def vals(self):
        val = slice_CrowdItemRefPs3()
        val.ptr = &self.ptr.vals
        return val

cdef class SprayRefPs3:
    cdef lotrc_rs.SprayRefPs3* ptr
    @property
    def instances(self):
        val = ref_slice_SprayInstancePs3()
        val.ptr = &self.ptr.instances
        return val
    @property
    def vals(self):
        val = ref_slice_SprayValPs3()
        val.ptr = &self.ptr.vals
        return val

def utils_hash_string(s):
    return lotrc_rs.utils_hash_string(s)
