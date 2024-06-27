import pprint
from ..utils import *
from ..types import *

class Mesh:
    Info = structtuple("MeshInfo",
        'key', 'I',
        'level_flag', 'i',
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
        'block_start', 'I',
        'block_end', 'I',
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
        'shape_offset', 'I', # optional pointer to shape_info
        'shape_num', 'I',
        'hk_constraint_data_offset', 'I', # optional pointer to hkConstraintData
        'hk_constraint_data_num', 'I',
        'hk_constraint_offset', 'I', # optional pointer to hkConstraint
        'keys2_offset', 'I',
        'keys2_order_offset', 'I',
        'valAs_offset', 'I', # 8 ints
    )
    BlockHeader = structtuple("BlockHeader",
        "a", "I",
        "b", "I",
        "unk_2", "I",
        "unk_3", "I",
        "unk_4", "I",
    )
    BlockVal = structtuple("BlockVal",
        "unk_0", "I",
        "unk_1", "I",
        "unk_2", "I",
        "unk_3", "I",
        "unk_4", "H",
        "unk_5", "H",
    )

    def __init__(self, data, offset, f='<'):
        self.info = unpack_from(self.Info[f], data, offset)

        self.keys = None
        if self.info['keys_offset'] != 0:
            self.keys = unpack_list_from(Uint[f], data, self.info['keys_offset'], self.info['keys_num'])
        self.valAs = unpack_list_from(Uint[f], data, self.info['valAs_offset'], self.info['keys_num'] * 8)
        self.valJs = unpack_list_from(Uint[f], data, self.info['valJs_offset'], self.info['valJs_num'])
        self.valGs = unpack_list_from(Uint[f], data, self.info['valGs_offset'], self.info['valGs_num'] * 16)
        self.valIs = None
        if self.info['valIs_offset'] != 0:
            self.valIs = unpack_list_from(Uint[f], data, self.info['valIs_offset'], self.info['valGs_num'])
        self.indices = unpack_list_from(Uint[f], data, self.info['indices_offset'], self.info['keys_num'])
        self.matrices = unpack_list_from(Matrix4x4[f], data, self.info['matrices_offset'], self.info['keys_num'])
        self.mat_order = unpack_list_from(Uint[f], data, self.info['mat_offset'], self.info['mat_num'])
        self.valCs = unpack_list_from(Uint[f], data, self.info['valCs_offset'], self.info['valCs_num'])
        self.valDs = unpack_list_from(Uint[f], data, self.info['valDs_offset'], self.info['valCs_num'] * 8)
        self.vbuff_order = unpack_list_from(Uint[f], data, self.info['vbuff_offset'], self.info['vbuff_num'])
        self.ibuff_order = unpack_list_from(Uint[f], data, self.info['ibuff_offset'], self.info['ibuff_num'])
        self.valKs_header = None
        self.valKs = new(Float[f], [])
        if self.info['valKs_offset'] != 0:
            self.valKs_header = unpack_list_from(Ushort[f], data, self.info['valKs_offset'], 2)
            self.valKs = unpack_list_from(Float[f], data, self.info['valKs_offset'] + 4, 35)
        self.keys2 = None
        if self.info['keys2_offset'] != 0:
            assert self.info['keys2_order_offset'] != 0
            i = 0
            while unpack_from(Uint[f], data, self.info["keys2_offset"] + 8*i)['val'] != 0:
                i += 1
            i += 1
            self.keys2 = unpack_list_from(Uint[f], data, self.info['keys2_offset'], i * 2)
            self.keys2_order = unpack_list_from(Uint[f], data, self.info['keys2_order_offset'], self.keys2[-1]['val'])
        self.block_header = None
        if self.info['block_offset'] != 0:
            offset = self.info['block_offset']
            self.block_header = unpack_from(Uint[f], data, offset)
            n = self.info['block_end'] - self.info['block_start']
            self.block_sizes = unpack_list_from(Uint[f], data, offset + 4, n+1)
            self.blocks = []
            for i in range(n):
                size = self.block_sizes[i+1]['val'] - self.block_sizes[i]['val']
                offset = self.block_sizes[i]['val'] + self.info['block_offset']
                header = unpack_from(self.BlockHeader[f], data, offset)
                s = header.nbytes
                vals_a = unpack_list_from(Uint[f], data, offset + s, (header['a'] + header['b']) * 12)
                s += vals_a.nbytes
                vals_b = unpack_list_from(self.BlockVal[f], data, offset + s, (size - s)//self.BlockVal[f].itemsize)
                s += vals_b.nbytes
                extra = unpack_list_from(Uint[f], data, offset + s, (size - s)//4)
                self.blocks.append((header, vals_a, vals_b, extra))
        # not sure why this pops up once, maybe it is padding between items?
        self.val = None
        if self.info['valCs_offset'] == self.info['vbuff_offset'] and self.info['valCs_offset'] == self.info['ibuff_offset'] and self.info['valCs_offset'] == self.info['valDs_offset']:
            val = unpack_list_from(Uint[f], data, self.info['valCs_offset'], 4)
            if (val['val'] == 0xFFFFFFFF).all():
                self.val = val

        assert self.indices[0]['val'] == 0xFFFFFFFF

        self.shapes = []
        off = self.info['shape_offset']
        for i in range(self.info['shape_num']):
            self.shapes.append(Shape(data, off, f))
            off += lotrc.level.pak.ShapeInfo[f].itemsize

        self.hk_constraint = None
        off = self.info['hk_constraint_offset']
        if off != 0:
            self.hk_constraint = HkConstraint(data, off, f)
        self.hk_constraint_datas = unpack_list_from(lotrc.level.pak.HkConstraintData[f], data, self.info['hk_constraint_data_offset'], self.info['hk_constraint_data_num'])

        self.mats = []
        self.mat_extras = []
        if len(self.mat_order) != 0:
            mat_map = {j:i for i,j in enumerate(np.unique(self.mat_order['val']))}
            self.mat_order['val'] = np.vectorize(mat_map.get)(self.mat_order['val'])
            for off in mat_map.keys():
                ty = unpack_from(Uint[f], data, off + 208)['val']
                mat = unpack_from(Mats[ty][f], data, off)
                mat_extra = None
                if mat['mat_extra_offset'] != 0:
                    mat_extra = unpack_from(lotrc.level.pak.MatExtra[f], data, mat['mat_extra_offset'])
                self.mats.append(mat)
                self.mat_extras.append(mat_extra)
                
        self.vbuffs = []
        vbuff_map = {}
        if len(self.vbuff_order) != 0:
            vbuff_map = {j:i for i,j in enumerate(np.unique(self.vbuff_order['val']))}
            self.vbuff_order['val'] = np.vectorize(vbuff_map.get)(self.vbuff_order['val'])
            for off in vbuff_map.keys():
                self.vbuffs.append(unpack_from(lotrc.level.pak.VBuffInfo[f], data, off))
                
        self.ibuffs = []
        ibuff_map = {}
        if len(self.vbuff_order) != 0:
            ibuff_map = {j:i for i,j in enumerate(np.unique(self.ibuff_order['val']))}
            self.ibuff_order['val'] = np.vectorize(ibuff_map.get)(self.ibuff_order['val'])
            for off in ibuff_map.keys():
                self.ibuffs.append(unpack_from(lotrc.level.pak.IBuffInfo[f], data, off))
        
        self.buffer_infos = unpack_list_from(lotrc.level.pak.BufferInfo[f], data, self.info['buffer_info_offset'], self.info['mat_num'])
        if len(self.buffer_infos) != 0:
            ibuff_map[0] = 0xFFFFFFFF
            vbuff_map[0] = 0xFFFFFFFF
            for buff in self.buffer_infos:
                buff['vbuff_info_offset'] = vbuff_map[buff['vbuff_info_offset']]
                buff['vbuff_info_offset_2'] = vbuff_map[buff['vbuff_info_offset_2']]
                buff['vbuff_info_offset_3'] = vbuff_map[buff['vbuff_info_offset_3']]
                buff['ibuff_info_offset'] = ibuff_map[buff['ibuff_info_offset']]

        # self.extra_off = None
        # if self.info['keys_offset'] == 0:
        #     offset = self.info['ibuff_offset'] + self.ibuff_order.nbytes
        #     self.extra_off = self.shapes[0].hk_shapes[0].info['d_offset'] - offset
    
    def infos_count(self):
        hk_shapes = 0
        for shape in self.shapes:
            hk_shapes += len(shape.hk_shapes)
        mat1_num = 0
        mat2_num = 0
        mat3_num = 0
        mat4_num = 0
        for mat in self.mats:
            if mat['type'] == 0:
                mat1_num += 1
            elif mat['type'] == 1:
                mat4_num += 1
            elif mat['type'] == 2:
                mat2_num += 1
            elif mat['type'] == 3:
                mat3_num += 1
        return np.array((
            len(self.shapes),
            hk_shapes,
            1 if self.hk_constraint is not None else 0,
            len(self.hk_constraint_datas),
            mat1_num,
            mat2_num,
            mat3_num,
            mat4_num,
            sum(i is not None for i in self.mat_extras),
            len(self.buffer_infos),
            len(self.vbuffs),
            len(self.ibuffs),
        ))
    
    def dump(self, offset, infos, f='<'):
        info = self.info.copy()
        data = bytes()

        mat_order = self.mat_order.copy()
        if len(self.mat_order) != 0:
            mat_map = {}
            for i, (mat, mat_extra) in enumerate(zip(self.mats, self.mat_extras)):
                mat = mat.copy()
                if mat_extra is not None:
                    mat['mat_extra_offset'] = infos['header']['mat_extra_offset'] + len(infos['mat_extra']) * lotrc.level.pak.MatExtra[f].itemsize
                    infos['mat_extra'].append(mat_extra)
                if mat['type'] == 0:
                    mat_map[i] = infos['header']['mat1_offset'] + len(infos['mat1']) * lotrc.level.pak.Mat1[f].itemsize
                    infos['mat1'].append(mat)
                elif mat['type'] == 1:
                    mat_map[i] = infos['header']['mat4_offset'] + len(infos['mat4']) * lotrc.level.pak.Mat4[f].itemsize
                    infos['mat4'].append(mat)
                elif mat['type'] == 2:
                    mat_map[i] = infos['header']['mat2_offset'] + len(infos['mat2']) * lotrc.level.pak.Mat2[f].itemsize
                    infos['mat2'].append(mat)
                elif mat['type'] == 3:
                    mat_map[i] = infos['header']['mat3_offset'] + len(infos['mat3']) * lotrc.level.pak.Mat3[f].itemsize
                    infos['mat3'].append(mat)
            mat_order['val'] = np.vectorize(mat_map.get)(mat_order['val'])
            
        vbuff_order = self.vbuff_order.copy()
        vbuff_map = {}
        if len(self.vbuff_order) != 0:
            vbuff_map = {
                i: infos['header']['vbuff_info_offset'] + (len(infos['vbuff']) + i) * lotrc.level.pak.VBuffInfo[f].itemsize
                for i in range(len(self.vbuffs))
            }
            vbuff_order['val'] = np.vectorize(vbuff_map.get)(vbuff_order['val'])
            infos['vbuff'].extend(self.vbuffs)
            
        ibuff_order = self.ibuff_order.copy()
        ibuff_map = {}
        if len(self.ibuff_order) != 0:
            ibuff_map = {
                i: infos['header']['ibuff_info_offset'] + (len(infos['ibuff']) + i) * lotrc.level.pak.IBuffInfo[f].itemsize
                for i in range(len(self.ibuffs))
            }
            ibuff_order['val'] = np.vectorize(ibuff_map.get)(ibuff_order['val'])
            infos['ibuff'].extend(self.ibuffs)

        vbuff_map[0xFFFFFFFF] = 0
        ibuff_map[0xFFFFFFFF] = 0
        buffer_infos = self.buffer_infos.copy()
        if len(buffer_infos) != 0:
            buffer_infos['vbuff_info_offset'] = np.vectorize(vbuff_map.get)(buffer_infos['vbuff_info_offset'])
            buffer_infos['vbuff_info_offset_2'] = np.vectorize(vbuff_map.get)(buffer_infos['vbuff_info_offset_2'])
            buffer_infos['vbuff_info_offset_3'] = np.vectorize(vbuff_map.get)(buffer_infos['vbuff_info_offset_3'])
            buffer_infos['ibuff_info_offset'] = np.vectorize(ibuff_map.get)(buffer_infos['ibuff_info_offset'])
        info['buffer_info_offset'] = infos['header']['buffer_info_offset'] + len(infos['buffer']) * lotrc.level.pak.BufferInfo[f].itemsize
        infos['buffer'].extend(buffer_infos)

        info['hk_constraint_data_num'] = len(self.hk_constraint_datas)
        if len(self.hk_constraint_datas) != 0:
            info['hk_constraint_data_offset'] = infos['header']['hk_constraint_data_offset'] + len(infos['hk_constraint_data']) * lotrc.level.pak.HkConstraintData[f].itemsize
        else:
            info['hk_constraint_data_offset'] = 0
        infos['hk_constraint_data'].extend(self.hk_constraint_datas)
        
        info['keys_offset'] = offset
        info['keys_num'] = len(self.keys)
        vals = pack(self.keys, f)
        offset += len(vals)
        data += vals

        off = (offset + 15) & 0xFFFFFFF0
        data += bytes(off - offset)
        offset = off

        info['valAs_offset'] = offset
        vals = pack(self.valAs)
        offset += len(vals)
        data += vals
            
        info['valJs_offset'] = offset
        info['valJs_num'] = len(self.valJs)
        vals = pack(self.valJs)
        offset += len(vals)
        data += vals
        if self.hk_constraint is not None:
            info['hk_constraint_offset'] = infos['header']['hk_constraint_info_offset'] + len(infos['hk_constraint']) * lotrc.level.pak.HkConstraintInfo[f].itemsize
            vals = self.hk_constraint.dump(offset, info['keys_offset'], info['keys_num'], infos, f)
            offset += len(vals)
            data += vals
        else:           
            info['hk_constraint_offset'] = 0
        
        off = (offset + 15) & 0xFFFFFFF0
        data += bytes(off - offset)
        offset = off
        
        info['valGs_offset'] = offset
        info['valGs_num'] = len(self.valGs) // 16
        vals = pack(self.valGs)
        offset += len(vals)
        data += vals

        if self.valIs is not None:
            info['valIs_offset'] = offset
            vals = pack(self.valIs, f)
            offset += len(vals)
            data += vals
        else:
            info['valIs_offset'] = 0
            
        info['indices_offset'] = offset
        vals = pack(self.indices)
        offset += len(vals)
        data += vals

        off = (offset + 15) & 0xFFFFFFF0
        data += bytes(off - offset)
        offset = off
        
        info['matrices_offset'] = offset
        vals = pack(self.matrices)
        offset += len(vals)
        data += vals

        info['mat_offset'] = offset
        info['mat_num'] = len(mat_order)
        infos['block2_offsets'].extend(offset + i * 4 for i in range(len(mat_order)))
        vals = pack(mat_order)
        offset += len(vals)
        data += vals

        info['shape_num'] = len(self.shapes)
        if len(self.shapes) != 0:
            info['shape_offset'] = infos['header']['shape_info_offset'] + len(infos['shape']) * lotrc.level.pak.ShapeInfo[f].itemsize
        else:
            info['shape_offset'] = 0
        for shape in self.shapes:
            vals = shape.dump(offset, None, infos, f)
            offset += len(vals)
            data += vals

        info['valCs_offset'] = offset
        info['valCs_num'] = len(self.valCs)
        vals = pack(self.valCs)
        offset += len(vals)
        data += vals

        off = (offset + 15) & 0xFFFFFFF0
        data += bytes(off - offset)
        offset = off

        info['valDs_offset'] = offset
        vals = pack(self.valDs)
        offset += len(vals)
        data += vals

        info['vbuff_offset'] = offset
        info['vbuff_num'] = len(vbuff_order)
        infos['block2_offsets'].extend(offset + i * 4 for i in range(len(vbuff_order)))
        vals = pack(vbuff_order)
        offset += len(vals)
        data += vals

        info['ibuff_offset'] = offset
        info['ibuff_num'] = len(ibuff_order)
        infos['block2_offsets'].extend(offset + i * 4 for i in range(len(ibuff_order)))
        vals = pack(ibuff_order)
        offset += len(vals)
        data += vals

        if self.valKs_header is not None:
            off = (offset + 15) & 0xFFFFFFF0
            data += bytes(off - offset)
            offset = off

            info['valKs_offset'] = offset
            vals = pack(self.valKs_header, f) + pack(self.valKs, f)
            offset += len(vals)
            data += vals

        if self.keys2 is not None:
            info['keys2_offset'] = offset
            vals = pack(self.keys2, f)
            offset += len(vals)
            data += vals
            info['keys2_order_offset'] = offset
            vals = pack(self.keys2_order, f)
            offset += len(vals)
            data += vals

        if self.block_header is not None:
            off = (offset + 15) & 0xFFFFFFF0
            data += bytes(off - offset)
            offset = off

            info['block_offset'] = offset
            vals = pack(self.block_header, f) + pack(self.block_sizes, f)
            offset += len(vals)
            data += vals
            for i, (header, vals_a, vals_b, extra) in enumerate(self.blocks):
                off = self.block_sizes[i]['val'] + info['block_offset']
                data += bytes(off - offset)
                offset = off
                vals = pack(header, f) + pack(vals_a, f) + pack(vals_b, f) + pack(extra, f)
                offset += len(vals)
                data += vals
    
            # if self.val is not None:
            #     vals = pack(self.val, f)
            #     offset += len(vals)
            #     data += vals
        infos['mesh'].append(info)

        return data
    
    def __repr__(self):
        return pprint.pformat(self.__dict__)
        
class TerrainMesh(Mesh):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.extra_off = None
        if len(self.shapes) != 0:
            offset = self.info['ibuff_offset'] + self.ibuff_order.nbytes
            self.extra_off = self.shapes[0].hk_shapes[0].info['d_offset'] - offset

    def dump(self, offset, indices_offset, infos, f='<'):
        info = self.info.copy()
        data = bytes()
        
        mat_order = self.mat_order.copy()
        if len(self.mat_order) != 0:
            mat_map = {}
            for i, (mat, mat_extra) in enumerate(zip(self.mats, self.mat_extras)):
                mat = mat.copy()
                if mat_extra is not None:
                    mat['mat_extra_offset'] = infos['header']['mat_extra_offset'] + len(infos['mat_extra']) * lotrc.level.pak.MatExtra[f].itemsize
                    infos['mat_extra'].append(mat_extra)
                if mat['type'] == 0:
                    mat_map[i] = infos['header']['mat1_offset'] + len(infos['mat1']) * lotrc.level.pak.Mat1[f].itemsize
                    infos['mat1'].append(mat)
                elif mat['type'] == 1:
                    mat_map[i] = infos['header']['mat4_offset'] + len(infos['mat4']) * lotrc.level.pak.Mat4[f].itemsize
                    infos['mat4'].append(mat)
                elif mat['type'] == 2:
                    mat_map[i] = infos['header']['mat2_offset'] + len(infos['mat2']) * lotrc.level.pak.Mat2[f].itemsize
                    infos['mat2'].append(mat)
                elif mat['type'] == 3:
                    mat_map[i] = infos['header']['mat3_offset'] + len(infos['mat3']) * lotrc.level.pak.Mat3[f].itemsize
                    infos['mat3'].append(mat)
            mat_order['val'] = np.vectorize(mat_map.get)(mat_order['val'])
            
        vbuff_order = self.vbuff_order.copy()
        vbuff_map = {}
        if len(self.vbuff_order) != 0:
            vbuff_map = {
                i: infos['header']['vbuff_info_offset'] + (len(infos['vbuff']) + i) * lotrc.level.pak.VBuffInfo[f].itemsize
                for i in range(len(self.vbuffs))
            }
            vbuff_order['val'] = np.vectorize(vbuff_map.get)(vbuff_order['val'])
            infos['vbuff'].extend(self.vbuffs)
            
        ibuff_order = self.ibuff_order.copy()
        ibuff_map = {}
        if len(self.ibuff_order) != 0:
            ibuff_map = {
                i: infos['header']['ibuff_info_offset'] + (len(infos['ibuff']) + i) * lotrc.level.pak.IBuffInfo[f].itemsize
                for i in range(len(self.ibuffs))
            }
            ibuff_order['val'] = np.vectorize(ibuff_map.get)(ibuff_order['val'])
            infos['ibuff'].extend(self.ibuffs)

        vbuff_map[0xFFFFFFFF] = 0
        ibuff_map[0xFFFFFFFF] = 0
        buffer_infos = self.buffer_infos.copy()
        if len(buffer_infos) != 0:
            buffer_infos['vbuff_info_offset'] = np.vectorize(vbuff_map.get)(buffer_infos['vbuff_info_offset'])
            buffer_infos['vbuff_info_offset_2'] = np.vectorize(vbuff_map.get)(buffer_infos['vbuff_info_offset_2'])
            buffer_infos['vbuff_info_offset_3'] = np.vectorize(vbuff_map.get)(buffer_infos['vbuff_info_offset_3'])
            buffer_infos['ibuff_info_offset'] = np.vectorize(ibuff_map.get)(buffer_infos['ibuff_info_offset'])
        info['buffer_info_offset'] = infos['header']['buffer_info_offset'] + len(infos['buffer']) * lotrc.level.pak.BufferInfo[f].itemsize
        infos['buffer'].extend(buffer_infos)

        info['hk_constraint_data_num'] = len(self.hk_constraint_datas)
        if len(self.hk_constraint_datas) != 0:
            info['hk_constraint_data_offset'] = infos['header']['hk_constraint_data_offset'] + len(infos['hk_constraint_data']) * lotrc.level.pak.HkConstraintData[f].itemsize
        else:
            info['hk_constraint_data_offset'] = 0
        infos['hk_constraint_data'].extend(self.hk_constraint_datas)

        info['keys_offset'] = 0
        info['keys_num'] = len(self.valAs) // 8

        info['valAs_offset'] = len(infos['mesh']) * 256 + infos['header']['mesh_info_offset'] + 16

        info['valGs_offset'] = indices_offset
        info['valGs_num'] = 0
        info['valIs_offset'] = 0
        info['indices_offset'] = indices_offset

        if self.hk_constraint is not None:
            info['hk_constraint_offset'] = infos['header']['hk_constraint_info_offset'] + len(infos['hk_constraint']) * lotrc.level.pak.HkConstraintInfo[f].itemsize
            vals = self.hk_constraint.dump(offset, info['keys_offset'], info['keys_num'], infos, f)
            offset += len(vals)
            data += vals
        else:           
            info['hk_constraint_offset'] = 0
        
        shape_offsets = []
        for shape in self.shapes:
            shape_offsets.append(offset)
            vals = shape.extra.dump(f)
            offset += len(vals)
            data += vals

        off = (offset + 15) & 0xFFFFFFF0
        data += bytes(off - offset)
        offset = off
        
        info['valDs_offset'] = offset
        vals = pack(self.valDs)
        offset += len(vals)
        data += vals
        
        info['matrices_offset'] = offset
        vals = pack(self.matrices)
        offset += len(vals)
        data += vals

        info['mat_offset'] = offset
        info['mat_num'] = len(mat_order)
        infos['block2_offsets'].extend(offset + i * 4 for i in range(len(mat_order)))
        vals = pack(mat_order)
        offset += len(vals)
        data += vals

        info['valCs_offset'] = offset
        info['valCs_num'] = len(self.valCs)
        vals = pack(self.valCs)
        offset += len(vals)
        data += vals

        info['vbuff_offset'] = offset
        info['vbuff_num'] = len(vbuff_order)
        infos['block2_offsets'].extend(offset + i * 4 for i in range(len(vbuff_order)))
        vals = pack(vbuff_order)
        offset += len(vals)
        data += vals

        off_dest = offset + 320
        info['ibuff_offset'] = offset
        info['ibuff_num'] = len(ibuff_order)
        infos['block2_offsets'].extend(offset + i * 4 for i in range(len(ibuff_order)))
        vals = pack(ibuff_order)
        offset += len(vals)
        data += vals
        
        if self.valKs_header is not None:
            off = (offset + 15) & 0xFFFFFFF0
            data += bytes(off - offset)
            offset = off

            info['valKs_offset'] = offset
            vals = pack(self.valKs_header, f) + pack(self.valKs, f)
            offset += len(vals)
            data += vals

        if self.keys2 is not None:
            info['keys2_offset'] = offset
            vals = pack(self.keys2, f)
            offset += len(vals)
            data += vals
            info['keys2_order_offset'] = offset
            vals = pack(self.keys2_order, f)
            offset += len(vals)
            data += vals

        if self.block_header is not None:
            off = (offset + 15) & 0xFFFFFFF0
            data += bytes(off - offset)
            offset = off

            info['block_offset'] = offset
            vals = pack(self.block_header, f) + pack(self.block_sizes, f)
            offset += len(vals)
            data += vals
            for i, (header, vals_a, vals_b, extra) in enumerate(self.blocks):
                off = self.block_sizes[i]['val'] + info['block_offset']
                data += bytes(off - offset)
                offset = off
                vals = pack(header, f) + pack(vals_a, f) + pack(vals_b, f) + pack(extra, f)
                offset += len(vals)
                data += vals

        # if self.extra_off is not None:
        #     offset += self.extra_off
        #     data += bytes(self.extra_off)
        data += bytes(off_dest - offset)
        offset = off_dest

        info['shape_num'] = len(self.shapes)
        if len(self.shapes) != 0:
            info['shape_offset'] = infos['header']['shape_info_offset'] + len(infos['shape']) * lotrc.level.pak.ShapeInfo[f].itemsize
        else:
            info['shape_offset'] = 0
        for shape, shape_off in zip(self.shapes, shape_offsets):
            vals = shape.dump(offset, shape_off, infos, f)
            offset += len(vals)
            data += vals

        infos['mesh'].append(info)
            
        return data

Mats = [
    lotrc.level.pak.Mat1,
    lotrc.level.pak.Mat4,
    lotrc.level.pak.Mat2,
    lotrc.level.pak.Mat3,
]

class Shape:
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
        'hk_shape_num', 'I',
        'hk_shape_offset', 'I', # pointer to hk_shape_info
        'unk_29', '4S',
        'unk_30', 'I',
    )
    def __init__(self, data, offset, f='<'):
        self.info = unpack_from(self.ShapeInfo[f], data, offset)
        self.extra = None
        if self.info['type'] == 0:
            self.extra = ShapeExtra(data, self.info['offset'], f)
        self.hk_shapes = []
        off = self.info['hk_shape_offset']
        for i in range(self.info['hk_shape_num']):
            ty = unpack_from(Uint[f], data, off + 32)['val']
            if ty == 5:
                self.hk_shapes.append(HkShape5(data, off, f))
            elif ty == 6:
                self.hk_shapes.append(HkShape6(data, off, f))
            elif ty <= 6:
                self.hk_shapes.append(unpack_from(HkShapes[ty][f], data, off))
            else:
                warnings.warn(f"Unknown HkShape type {ty} this is probably fine for now")
                self.hk_shapes.append(unpack_from(HkShapes[0][f], data, off))
            off += lotrc.level.pak.HkShapeInfo[f].itemsize

    def dump(self, offset, extra_offset, infos, f='<'):
        info = self.info.copy()
        if extra_offset is not None:
            info['offset'] = extra_offset
            infos['block2_offsets'].append(infos['header']['shape_info_offset'] + len(infos['shape']) * self.ShapeInfo[f].itemsize)

        info['hk_shape_offset'] = infos['header']['hk_shape_info_offset'] + len(infos['hk_shape']) * lotrc.level.pak.HkShapeInfo[f].itemsize
        data = bytes()
        for hk_shape in self.hk_shapes:
            if isinstance(hk_shape, HkShape5):
                # if hk_shape.info.tolist() in processed_shapes:
                #     continue
                # else:
                #     processed_shapes.add(hk_shape.info.tolist())
                off = (offset + 15) & 0xFFFFFFF0
                data += bytes(off - offset)
                offset = off
                
                hk_shape_data = hk_shape.dump(offset, infos, f)
                offset += len(hk_shape_data)
                data += hk_shape_data
            elif isinstance(hk_shape, HkShape6):
                # if hk_shape.info.tolist() in processed_shapes:
                #     continue
                # else:
                #     processed_shapes.add(hk_shape.info.tolist())
                hk_shape_data = hk_shape.dump(offset, infos, f)
                offset += len(hk_shape_data)
                data += hk_shape_data
                
                off = (offset + 3) & 0xFFFFFFFC
                data += bytes(off - offset)
                offset = off

            else:
                infos['hk_shape'].append(hk_shape)

        infos['shape'].append(info)
                
        return data
        
    def __repr__(self):
        return pprint.pformat(self.__dict__)

class ShapeExtra:
    def __init__(self, data, offset, f='<'):
        self.info = unpack_from(lotrc.level.pak.Shape.Header[f], data, offset)
        offset += self.info.nbytes
        self.offs = unpack_list_from(Uint[f], data, offset, self.info['num'])
        offset += self.offs.nbytes
        off = self.offs[-1]['val'] + offset
        while data[off] != 0 or data[off+1] != 0:
            off += 1
        self.data = data[offset:off] # 2 seems to be the correct amount, not sure what the data is so I don't know how much extra is needed

    def dump(self, f='<'):
        return pack(self.info, f) + pack(self.offs, f) + self.data

class HkConstraint:
    Info = structtuple("HkConstraint_Info",
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
        'vals2_num', 'I',
        'vals2_offset', 'I',
        'unk_17', 'I',
    )
    def __init__(self, data, offset, f='<'):
        self.info = unpack_from(self.Info[f], data, offset)
        self.shorts = unpack_list_from(Ushort[f], data, self.info['shorts_offset'], self.info['shorts_num'])
        assert self.shorts[0]['val'] == 0xFFFF
        
        self.strings = []
        max_offset = 0
        self.string_offsets = unpack_list_from(Uint[f], data, self.info['strings_offset'], self.info['strings_num'])
        for offset_ in self.string_offsets['val']:
            (offset, val) = unpack_list_from(Uint[f], data, offset_, 2)['val']
            start = offset
            while data[offset] != 0:
                offset += 1
            string = data[start:offset]
            self.strings.append((string, start, val))
            max_offset = max(offset+1, max_offset)
        self.vals = unpack_list_from(Uint[f], data, self.info['vals_offset'], self.info['vals_num'] * 12)
        self.keys = unpack_list_from(Uint[f], data, self.info['keys_offset'], self.info['keys_num'])
        self.keys2 = unpack_list_from(Uint[f], data, self.info['keys2_offset'], self.info['keys2_num'] * 2)
        self.vals2 = unpack_list_from(Uint[f], data, self.info['vals2_offset'], self.info['vals2_num'] * 42)

    def dump(self, offset, keys_offset, keys_num, infos, f='<'):
        # don't dump keys since they are part of the parent mesh
        info = self.info.copy()
        info['keys_offset'] = keys_offset
        info['keys_num'] = keys_num

        info['strings_offset'] = offset
        info['strings_num'] = len(self.strings)
        data = bytes()
        offset = offset + (12 * len(self.strings))
        off = (offset + 15) & 0xFFFFFFF0
        data += bytes(off - offset)
        offset = off

        info['vals_offset'] = offset
        info['vals_num'] = len(self.vals) // 12
        val = pack(self.vals, f)
        offset += len(val)
        data += val
        
        info['keys2_offset'] = offset
        info['keys2_num'] = len(self.keys2) // 2
        val = pack(self.keys2, f)
        offset += len(val)
        data += val
        
        info['shorts_offset'] = offset
        info['shorts_num'] = len(self.shorts)
        val = pack(self.shorts, f)
        offset += len(val)
        data += val
        
        off = (offset + 3) & 0xFFFFFFFC
        data += bytes(off - offset)
        offset = off
        
        offsets = []
        string_offsets = []
        block2_off = info['strings_offset']
        offset_off = info['strings_offset'] + 4 * len(self.strings)
        for (string, _, val) in self.strings:
            string_offsets.append([offset, val])
            data += string
            offset += len(string)
            off = (offset + 4) & 0xFFFFFFFC
            data += bytes(off - offset)
            offset = off
            
            offsets.append(offset_off)
            infos['block2_offsets'].append(block2_off)
            infos['block2_offsets'].append(offset_off)
            offset_off += 8
            block2_off += 4

        if len(self.vals2) != 0:
            info['vals2_offset'] = offset
            info['vals2_num'] = len(self.vals2) // 42
            vals = pack(self.vals2, f)
            offset += len(vals)
            data += vals
        else:
            info['vals2_offset'] = 0
            info['vals2_num'] = 0

        infos['hk_constraint'].append(info)
        data = pack(new(Uint[f], offsets), f) + pack(new(Uint[f], string_offsets).flatten(), f) + data
        return data
    
    def __repr__(self):
        return pprint.pformat(self.__dict__)

HkShapes = [
structtuple("HkShape0",
    'unk_0', Vector4,
    'unk_4', Vector4,
    'type', 'I',
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
), structtuple("HkShape1",
    'unk_0', Vector4,
    'unk_4', Vector4,
    'type', 'I',
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
), structtuple("HkShape2", # HkShapeInfo
    'unk_0', Vector4,
    'unk_4', Vector4,
    'type', 'I',
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
), structtuple("HkShape3", # HkShapeInfo
    'unk_0', Vector4,
    'unk_4', Vector4,
    'type', 'I',
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
), structtuple("HkShape4", # HkShapeInfo
    'unk_0', Vector4,
    'unk_4', Vector4,
    'type', 'I',
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
)
]

class HkShape5:
    Header = structtuple("HkShape5_Header", # HkShapeInfo
        'unk_0', Vector4,
        'unk_4', Vector4,
        'type', 'I',
        'unk_9', 'I',
        'a_num', 'I',
        'a_offset', 'I',
        'b_num', 'I',
        'b_offset', 'I',
        'unk_14', 'I',
        'unk_15', 'I',
        'unk_16', 'I',
        'unk_17', 'I',
        'unk_18', 'I',
        'unk_19', 'I',
    )
    def __init__(self, data, offset, f='<'):
        self.info = unpack_from(self.Header[f], data, offset)
        self.a = unpack_list_from(Uint[f], data, self.info['a_offset'], self.info['a_num'] * 4)
        b_num = self.info['b_num']
        while (self.info['b_offset'] + b_num * 12) % 16 != 0:
            b_num += 1
        self.b = unpack_list_from(Uint[f], data, self.info['b_offset'], b_num * 3) # somethimes seems to be off by 1
        self.b_extra = b_num - self.info['b_num']
        
    def dump(self, offset, infos, f='<'):
        info = self.info.copy()
        data = bytes()
        
        info['a_offset'] = offset
        info['a_num'] = len(self.a) // 4
        val = pack(self.a, f)
        offset += len(val)
        data += val

        info['b_offset'] = offset
        info['b_num'] = len(self.b) // 3 - self.b_extra
        val = pack(self.b, f)
        offset += len(val)
        data += val

        infos['hk_shape'].append(info)
        return data
    
    def __repr__(self):
        return pprint.pformat(self.__dict__)

class HkShape6:
    Header = structtuple("HkShape6_Header", # HkShapeInfo
        'unk_0', Vector4,
        'unk_4', Vector4,
        'type', 'I',
        'unk_9', 'I',
        'unk_10', 'I',
        'unk_11', 'I',
        'unk_12', 'I',
        'unk_13', 'I',
        'c_num', 'I',
        'c_offset', 'I',
        'd_num', 'I',
        'd_offset', 'I',
        'e_num', 'I',
        'e_offset', 'I',
    )
    def __init__(self, data, offset, f='<'):
        self.info = unpack_from(self.Header[f], data, offset)
        self.d = unpack_list_from(Uint[f], data, self.info['d_offset'], self.info['d_num'] * 3)
        self.e = unpack_list_from(Ushort[f], data, self.info['e_offset'], self.info['e_num'] * 3)
        self.c = data[self.info['c_offset']:self.info['c_offset']+self.info['c_num']]

    def dump(self, offset, infos, f='<'):
        info = self.info.copy()
        data = bytes()
        
        info['d_offset'] = offset
        info['d_num'] = len(self.d) // 3
        val = pack(self.d, f)
        offset += len(val)
        data += val

        info['e_offset'] = offset
        info['e_num'] = len(self.e) // 3
        val = pack(self.e, f)
        offset += len(val)
        data += val

        off = (offset + 3) & 0xFFFFFFFC
        data += bytes(off - offset)
        offset = off
        
        info['c_offset'] = offset
        info['c_num'] = len(self.c)
        offset += len(self.c)
        data += self.c

        infos['hk_shape'].append(info)
        return data
        
    def __repr__(self):
        return pprint.pformat(self.__dict__)

class Animation:
    Obj5Heder = structtuple("Animation_Obj5Header",
        "objA_num", "I",
        "objA_offset", "I",
        "objB_num", "I",
        "objB_offset", "I",
    )
    def __init__(self, info, block, offset, f='<'):
        # self.info = unpack_from(lotrc.level.pak.AnimationInfo[f], buffer, off)
        self.info = info
        # for i, key in enumerate(blocks.keys()):
        #     if (self.info['level_flag'] & (1 << i)) == 0:
        #         continue
        #     block = blocks[key]
        #     offset = offsets[key]
        #     print(len(block), offset)
        self.obj1 = unpack_list_from(Uint[f], block, offset + self.info['obj1_offset'], self.info['obj1_num']*2)
        self.obj2 = unpack_list_from(Uint[f], block, offset + self.info['obj2_offset'], self.info['obj2_num']*4)
        self.obj3 = unpack_list_from(Uint[f], block, offset + self.info['obj3_offset'], self.info['obj3_num']*11)
        self.keys = unpack_list_from(Uint[f], block, offset + self.info['keys_offset'], self.info['keys_num'] + info['obj1_num'])
        if self.info['obj5_offset'] != 0:
            self.obj5_header = unpack_from(self.Obj5Heder[f], block, offset + self.info['obj5_offset'])
            self.obj5A = unpack_list_from(Uint[f], block, offset + self.obj5_header['objA_offset'], self.obj5_header['objA_num']*7)
            self.obj5B = unpack_list_from(Uint[f], block, offset + self.obj5_header['objB_offset'], self.obj5_header['objB_num']*7)
        if self.info['type'] == 3:
            self.objC = lotrc.level.pak.hkaSplineSkeletalAnimation.unpack_from(block, offset, self.info, f)
        elif self.info['type'] < 3:
            warnings.warn(f"Unhandled amination type {self.info['type']}")
        else:
            warnings.warn(f"Unkown amination type {self.info['type']}")
        #     break
        # for i, key in enumerate(blocks.keys()):
        #     if (self.info['level_flag'] & (1 << i)) != 0:
        #         offsets[key] += self.info['size']

    def dump(self, offset, infos, f='<'):
        info = self.info.copy()
        buffer = bytearray(info['size'])
        
        info['offset'] = offset
        pack_into(self.obj1, buffer, self.info['obj1_offset'], f)
        pack_into(self.obj2, buffer, self.info['obj2_offset'], f)
        pack_into(self.obj3, buffer, self.info['obj3_offset'], f)
        pack_into(self.keys, buffer, self.info['keys_offset'], f)
        if self.info['obj5_offset'] != 0:
            pack_into(self.obj5_header, buffer, self.info['obj5_offset'], f)
            pack_into(self.obj5A, buffer, self.obj5_header['objA_offset'], f)
            pack_into(self.obj5B, buffer, self.obj5_header['objB_offset'], f)
        if self.info['type'] == 3:
            self.objC.pack_into(buffer, 0, self.info, f)
        infos['animation'].append(info)
        return bytes(buffer)