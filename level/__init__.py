from lotrc.utils import *
from lotrc.types import *
from collections import OrderedDict

import lotrc.level.pak as pak
import lotrc.level.bin as bin

class LevelData:
    def __init__(self, name):
        with open(f"{name}.PAK", 'rb') as f:
            self.pak_data = f.read()
        with open(f"{name}.BIN", 'rb') as f:
            self.bin_data = f.read()

        if self.bin_data[:4] == b'\x06\x00\x00\x00':
            self.f = "<"
        elif self.bin_data[:4] == b'\x00\x00\x00\x06':
            self.f = ">"
        else:
            raise ValueError("Wrong file type?!!")

        self.bin_header = unpack_from(bin.Header[self.f], self.bin_data, 0)
        self.bin_strings = read_strings(self.bin_data, self.bin_header['strings_offset'], self.bin_header['strings_num'], self.f)

        self.asset_handles = unpack_list_from(bin.AssetHandle[self.f], self.bin_data, self.bin_header['asset_handle_offset'], self.bin_header['asset_handle_num'])

        self.asset_data = {
            (info['key'], info['type']): CompressedBlock.unpack_from(self.bin_data, info['size'], info['size_comp'], info['offset']) for info in self.asset_handles
        }

        self.vertex_formats = {}
        self.game_objs_types = {}
        self.keys = {hash_string(i):i for i in self.bin_strings}

        self.pak_header = unpack_from(pak.Header[self.f], self.pak_data, 0)
        self.pak_strings = read_strings(self.pak_data, self.pak_header['strings_offset'], self.pak_header['strings_num'], self.f)
        self.keys.update({hash_string(i):i for i in self.pak_strings})
        self.keys.update({i: j['name'].encode() for i,j in BaseTypes.items()})

        self.block1 = CompressedBlock.unpack_from(self.pak_data, self.pak_header['block1_size'], self.pak_header['block1_size_comp'], self.pak_header['block1_offset'])
        self.block2 = CompressedBlock.unpack_from(self.pak_data, self.pak_header['block2_size'], self.pak_header['block2_size_comp'], self.pak_header['block2_offset'])

        self.objas = unpack_list_from(pak.ObjA[self.f], self.block1.data, self.pak_header['obja_offset'], self.pak_header['obja_num'])
        self.obj0s = unpack_list_from(pak.Obj0[self.f], self.block1.data, self.pak_header['obj0_offset'], self.pak_header['obj0_num'])
        self.mesh_infos = unpack_list_from(pak.MeshInfo[self.f], self.block1.data, self.pak_header['mesh_info_offset'], self.pak_header['mesh_info_num'])
        self.buffer_infos = unpack_list_from(pak.BufferInfo[self.f], self.block1.data, self.pak_header['buffer_info_offset'], self.pak_header['buffer_info_num'])
        self.mat1s = unpack_list_from(pak.Mat1[self.f], self.block1.data, self.pak_header['mat1_offset'], self.pak_header['mat1_num'])
        self.mat2s = unpack_list_from(pak.Mat2[self.f], self.block1.data, self.pak_header['mat2_offset'], self.pak_header['mat2_num'])
        self.mat3s = unpack_list_from(pak.Mat3[self.f], self.block1.data, self.pak_header['mat3_offset'], self.pak_header['mat3_num'])
        self.mat4s = unpack_list_from(pak.Mat4[self.f], self.block1.data, self.pak_header['mat4_offset'], self.pak_header['mat4_num'])
        self.mat_extras = unpack_list_from(pak.MatExtra[self.f], self.block1.data, self.pak_header['mat_extra_offset'], self.pak_header['mat_extra_num'])
        self.shape_infos = unpack_list_from(pak.ShapeInfo[self.f], self.block1.data, self.pak_header['shape_info_offset'], self.pak_header['shape_info_num'])
        self.hk_shape_infos = unpack_list_from(pak.HkShapeInfo[self.f], self.block1.data, self.pak_header['hk_shape_info_offset'], self.pak_header['hk_shape_info_num'])
        self.hk_constraint_datas = unpack_list_from(pak.HkConstraintData[self.f], self.block1.data, self.pak_header['hk_constraint_data_offset'], self.pak_header['hk_constraint_data_num'])
        self.vbuff_infos = unpack_list_from(pak.VBuffInfo[self.f], self.block1.data, self.pak_header['vbuff_info_offset'], self.pak_header['vbuff_info_num'])
        self.ibuff_infos = unpack_list_from(pak.IBuffInfo[self.f], self.block1.data, self.pak_header['ibuff_info_offset'], self.pak_header['ibuff_info_num'])
        self.texture_infos = unpack_list_from(pak.TextureInfo[self.f], self.block1.data, self.pak_header['texture_info_offset'], self.pak_header['texture_info_num'])
        self.animation_infos = unpack_list_from(pak.AnimationInfo[self.f], self.block1.data, self.pak_header['animation_info_offset'], self.pak_header['animation_info_num'])
        self.hk_constraint_infos = unpack_list_from(pak.HkConstraintInfo[self.f], self.block1.data, self.pak_header['hk_constraint_info_offset'], self.pak_header['hk_constraint_info_num'])
        self.game_objs_block_infos = unpack_list_from(pak.GameObjBlockInfo[self.f], self.block1.data, self.pak_header['game_objs_block_info_offset'], self.pak_header['game_objs_block_info_num'])
        self.obj11s = unpack_list_from(pak.Obj11[self.f], self.block1.data, self.pak_header['obj11_offset'], self.pak_header['obj11_num'])
        self.pfield_infos = unpack_list_from(pak.PFieldInfo[self.f], self.block1.data, self.pak_header['pfield_info_offset'], self.pak_header['pfield_info_num'])
        self.gfx_block_infos = unpack_list_from(pak.GFXBlockInfo[self.f], self.block1.data, self.pak_header['gfx_block_info_offset'], self.pak_header['gfx_block_info_num'])
        self.obj14_infos = unpack_list_from(pak.Obj14Info[self.f], self.block1.data, self.pak_header['obj14_info_offset'], self.pak_header['obj14_info_num'])
        self.animation_block_infos = unpack_list_from(pak.AnimationBlockInfo[self.f], self.block1.data, self.pak_header['animation_block_info_offset'], self.pak_header['animation_block_info_num'])

        self.meshes = [pak.Mesh.unpack_from(self.block1.data, info, self.f) for info in self.mesh_infos]
        self.shapes = [pak.Shape.unpack_from(self.block1.data, info, self.f) for info in self.shape_infos]
        self.hk_shapes = [pak.HkShape.unpack_from(self.block1.data, info, self.f) for info in self.hk_shape_infos]
        self.hk_constraints = [pak.HkConstraint.unpack_from(self.block1.data, info, self.f) for info in self.hk_constraint_infos]
        self.game_objs_blocks = [GameObjs.unpack_from(self.block1.data, info['offset'], info['size'], self.game_objs_types, self.f) for info in self.game_objs_block_infos]

        self.obj14s = [pak.Obj14.unpack_from(self.block1.data, info, self.f) for info in self.obj14_infos]
        self.gfx_blocks = [Data.unpack_from(self.block1.data, info['offset'], info['size'], self.f) for info in self.gfx_block_infos]

        self.animation_blocks = [CompressedBlock.unpack_from(self.pak_data, info['size'], info['size_comp'], info['offset']) for info in self.animation_block_infos]
        self.animations = [pak.Animation() for _ in range(len(self.animation_infos))]
        for i, block in enumerate(self.animation_blocks):
            pak.Animation.unpack_block(self.animations, self.animation_infos, block.data, 0, i, self.f)

        self.sub_blocks1 = SubBlocks.unpack_from(self.block1.data, self.pak_header['sub_blocks1_offset'], self.keys, self.game_objs_types, self.f)
        self.string_keys = StringKeys.unpack_from(self.block1.data, self.pak_header['string_keys_offset'], self.f)
        self.sub_blocks2 = SubBlocks.unpack_from(self.block2.data, self.pak_header['sub_blocks2_offset'], self.keys, self.game_objs_types, self.f)
        self.block2_offsets = unpack_list_from(Uint[self.f], self.block2.data, self.pak_header['block2_offsets_offset'], self.pak_header['block2_offsets_num'])

        self.radiosity = {(info['key'], info['type']):bin.Radiosity.unpack_from(self.asset_data[(info['key'], info['type'])].data, self.f) for info in self.asset_handles if self.keys[info['key']].endswith(b'_radiosity')}
        
        self.textures = {}
        for info in self.texture_infos:
            data0 = self.asset_data[((key := info['asset_key']), info['asset_type'])].data
            data1 = self.asset_data[(hash_string('*', key), info['asset_type'])].data
            if info['type'] in [0,7,8]:
                self.textures[key] = bin.Texture(data0, data1, info, self.f)
            elif info['type'] in [1,9]:
                self.textures[key] = bin.CubeTexture(data0, data1, info, self.f)
            else:
                raise ValueError(f"Unsuported Texture Type {info['type']}")

        self.mat1_map = {
            self.pak_header['mat1_offset']+pak.Mat1[self.f].itemsize*i:i for i in range(self.pak_header['mat1_num'])
        }
        self.mat2_map = {
            self.pak_header['mat2_offset']+pak.Mat2[self.f].itemsize*i:i for i in range(self.pak_header['mat2_num'])
        }
        self.mat3_map = {
            self.pak_header['mat3_offset']+pak.Mat3[self.f].itemsize*i:i for i in range(self.pak_header['mat3_num'])
        }
        self.mat4_map = {
            self.pak_header['mat4_offset']+pak.Mat4[self.f].itemsize*i:i for i in range(self.pak_header['mat4_num'])
        }
        self.mat_extra_map = {
            self.pak_header['mat_extra_offset']+pak.MatExtra[self.f].itemsize*i:i for i in range(self.pak_header['mat_extra_num'])
        }
        self.shape_info_map = {
            self.pak_header['shape_info_offset']+pak.ShapeInfo[self.f].itemsize*i:i for i in range(self.pak_header['shape_info_num'])
        }
        self.hk_constraint_data_map = {
            self.pak_header['hk_constraint_data_offset']+pak.HkConstraintData[self.f].itemsize*i:i for i in range(self.pak_header['hk_constraint_data_num'])
        }
        self.hk_constraint_info_map = {
            self.pak_header['hk_constraint_info_offset']+pak.HkConstraintInfo[self.f].itemsize*i:i for i in range(self.pak_header['hk_constraint_info_num'])
        }
        self.hk_shape_info_map = {
            self.pak_header['hk_shape_info_offset']+pak.HkShapeInfo[self.f].itemsize*i:i for i in range(self.pak_header['hk_shape_info_num'])
        }
        self.buffer_info_map = {
            self.pak_header['buffer_info_offset']+pak.BufferInfo[self.f].itemsize*i:i for i in range(self.pak_header['buffer_info_num'])
        }
        self.ibuff_info_map = {
            self.pak_header['ibuff_info_offset']+pak.IBuffInfo[self.f].itemsize*i:i for i in range(self.pak_header['ibuff_info_num'])
        }
        self.vbuff_info_map = {
            self.pak_header['vbuff_info_offset']+pak.VBuffInfo[self.f].itemsize*i:i for i in range(self.pak_header['vbuff_info_num'])
        }
        self.asset_handle_lookup = {
            (i['key'], i['type']):j for j,i in enumerate(self.asset_handles)
        }
        self.valid_offsets = set(i for i in self.block2_offsets['val'])

        self.vbuffs = []
        self.ibuffs = []
        self.processed_buffers = set()
        for mesh, info in zip(self.meshes, self.mesh_infos):
            if info['vbuff_num'] == 0 and info['ibuff_num'] == 0: 
                self.vbuffs.append(None)
                self.ibuffs.append(None)
            else:
                buffer = self.asset_data[(info['asset_key'], info['asset_type'])].data
                self.vbuffs.append([pak.VertexBuffer.unpack_from(buffer, self.vbuff_infos[self.vbuff_info_map[info]], self.vertex_formats, self.f) for info in mesh.vbuffs['val']])
                self.ibuffs.append([pak.IndexBuffer.unpack_from(buffer, self.ibuff_infos[self.ibuff_info_map[info]], self.f) for info in mesh.ibuffs['val']])
                self.processed_buffers.add((info['asset_key'], info['asset_type']))
        if self.f == ">":
            for vbuffs in self.vbuffs:
                if vbuffs is None: continue
                for vbuff in vbuffs:
                    if 'weight' in vbuff.data.dtype.names:
                        if 'x' in vbuff.data.dtype.fields['weight'][0].names:
                            vbuff.data['weight']['x'] = vbuff.data['weight']['x']/2 + 0.5
                            vbuff.data['weight']['y'] = vbuff.data['weight']['y']/2 + 0.5
                            vbuff.data['weight']['z'] = vbuff.data['weight']['z']/2 + 0.5
                        else:
                            # sketchy conv of xbox 10 bit weights to pc 8 bit weights
                            val = vbuff.data['weight']['val']
                            v = np.array([(val>>20)&0x3FF, (val>>10)&0x3FF, val&0x3FF])
                            inds = v & 0x200 != 0
                            v[~inds] += 511
                            v[inds] -= 512
                            v = (v-2) >> 2
                            vbuff.data['weight']['val'] = v[0] | (v[1] << 8) | (v[2] << 16) | (127 << 24)

        self.pak_blockA = unpack_list_from(pak.BlockAVal[self.f], self.pak_data, self.pak_header['blockA_offset'], self.pak_header['blockA_num'])
        self.remaps = []

    def dump(self, f="<", compress=True):
        if f == ">" and self.f == "<":
            raise ValueError("\nUnsupported conversion from '<' to '>'")

        dump_bin_header = self.bin_header.copy()

        self.dump_asset_data = OrderedDict()
        for mesh, info, vbuffs, ibuffs in zip(self.meshes, self.mesh_infos, self.vbuffs, self.ibuffs):
            key = (info['asset_key'], info['asset_type'])
            if (info['vbuff_num'] == 0 and info['ibuff_num'] == 0): continue # or key in self.textures: continue
            buffer = bytearray(self.asset_handles[self.asset_handle_lookup[key]]['size'])
            for vbuff, info in zip(vbuffs, mesh.vbuffs['val']):
                vbuff.pack_into(buffer, self.vbuff_infos[self.vbuff_info_map[info]], f)
            for ibuff, info in zip(ibuffs, mesh.ibuffs['val']):
                ibuff.pack_into(buffer, self.ibuff_infos[self.ibuff_info_map[info]], f)
            self.dump_asset_data[key] = bytes(buffer)
            
        for key, texture in self.textures.items():
            data0, data1 = texture.dump(f)
            self.dump_asset_data[(key, texture.type)] = data0
            self.dump_asset_data[(hash_string('*', key), texture.type)] = data1
        
        for key, radiosity in self.radiosity.items():
            self.dump_asset_data[key] = radiosity.dump(f)

        bin_offset = dump_bin_header.nbytes
        bin_dump = bytearray()
        dump_asset_handles = self.asset_handles.copy()
        dump_asset_handles['size'] = 0
        dump_asset_handles['size_comp'] = 0
        dump_asset_handles['offset'] = 0
        off =  (bin_offset + 2047) & 0xfffff800
        bin_dump += bytes(off - bin_offset)
        bin_offset = off

        for key, data in self.dump_asset_data.items():
            i = self.asset_handle_lookup[key]
            data = CompressedBlock(data)
            data_comp = data.pack(compress)
            dump_asset_handles[i]['offset'] = bin_offset
            dump_asset_handles[i]['size'] = data.size
            dump_asset_handles[i]['size_comp'] = data.size_comp
            bin_dump += data_comp
            if len(data_comp) != 0:
                bin_offset += len(data_comp)
                off =  (bin_offset + 2047) & 0xfffff800
                bin_dump += bytes(off - bin_offset)
                bin_offset = off
        off =  (bin_offset + 2047) & 0xfffff800
        bin_dump += bytes(off - bin_offset)
        bin_offset = off

        dump_bin_header['asset_handle_offset']  = bin_offset
        dump_bin_header['asset_handle_num'] = dump_asset_handles.size
        data = pack(dump_asset_handles, f)
        bin_offset += len(data)
        bin_dump += data

        dump_bin_header['strings_offset'] = bin_offset
        data = pack_strings(self.bin_strings, f)
        dump_bin_header['strings_num'] = len(self.bin_strings)
        dump_bin_header['strings_size'] = len(data)
        bin_dump += data
        bin_offset += len(data)

        off = (bin_offset + 2047) & 0xfffff800
        bin_dump += bytes(off - bin_offset)
        bin_offset = off

        bin_dump = pack(dump_bin_header, f) + bytes(bin_dump)
        dump_pak_header = self.pak_header.copy()

        pak_offset = dump_pak_header.nbytes
        pak_dump = bytearray()

        self.dump_animation_block_infos = self.animation_block_infos.copy()
        self.dump_animation_blocks = []
        for i, info in enumerate(self.dump_animation_block_infos):
            block = bytearray(info['size'])
            pak.Animation.pack_block(self.animations, self.animation_infos, block, 0, i, f)
            off =  (pak_offset + 4095) & 0xfffff000
            pak_dump += bytes(off - pak_offset)
            pak_offset = off
            data = CompressedBlock(bytes(block))
            data_comp = data.pack(compress)
            info['offset'] = pak_offset
            info['size'] = data.size
            info['size_comp'] = data.size_comp
            pak_offset += len(data_comp)
            pak_dump += data_comp
            self.dump_animation_blocks.append(data)
    
        dump_block1 = bytearray(dump_pak_header['sub_blocks1_offset'])

        for obj14, info in zip(self.obj14s, self.obj14_infos):
            obj14.pack_into(dump_block1, info, f)
        for gfx_block, info in zip(self.gfx_blocks, self.gfx_block_infos):
            gfx_block.pack_into(dump_block1, info['offset'], f)

        for mesh, info in zip(self.meshes, self.mesh_infos):
            mesh.pack_into(dump_block1, info, f)
        for shape, info in zip(self.shapes, self.shape_infos):
            shape.pack_into(dump_block1, info, f)
        for hk_shape, info in zip(self.hk_shapes, self.hk_shape_infos):
            hk_shape.pack_into(dump_block1, info, f)
        for hk_constraint, info in zip(self.hk_constraints, self.hk_constraint_infos):
            hk_constraint.pack_into(dump_block1, info, f)
        for game_objs_block, info in zip(self.game_objs_blocks, self.game_objs_block_infos):
            game_objs_block.pack_into(dump_block1, info['offset'], f)

        pack_into(self.objas, dump_block1, dump_pak_header['obja_offset'], f)
        pack_into(self.obj0s, dump_block1, dump_pak_header['obj0_offset'], f)
        pack_into(self.mesh_infos, dump_block1, dump_pak_header['mesh_info_offset'], f)
        pack_into(self.buffer_infos, dump_block1, dump_pak_header['buffer_info_offset'], f)
        pack_into(self.mat1s, dump_block1, dump_pak_header['mat1_offset'], f)
        pack_into(self.mat2s, dump_block1, dump_pak_header['mat2_offset'], f)
        pack_into(self.mat3s, dump_block1, dump_pak_header['mat3_offset'], f)
        pack_into(self.mat4s, dump_block1, dump_pak_header['mat4_offset'], f)
        pack_into(self.mat_extras, dump_block1, dump_pak_header['mat_extra_offset'], f)
        pack_into(self.shape_infos, dump_block1, dump_pak_header['shape_info_offset'], f)
        pack_into(self.hk_shape_infos, dump_block1, dump_pak_header['hk_shape_info_offset'], f)
        pack_into(self.hk_constraint_datas, dump_block1, dump_pak_header['hk_constraint_data_offset'], f)
        pack_into(self.vbuff_infos, dump_block1, dump_pak_header['vbuff_info_offset'], f)
        pack_into(self.ibuff_infos, dump_block1, dump_pak_header['ibuff_info_offset'], f)
        pack_into(self.texture_infos, dump_block1, dump_pak_header['texture_info_offset'], f)
        pack_into(self.animation_infos, dump_block1, dump_pak_header['animation_info_offset'], f)
        pack_into(self.hk_constraint_infos, dump_block1, dump_pak_header['hk_constraint_info_offset'], f)
        pack_into(self.game_objs_block_infos, dump_block1, dump_pak_header['game_objs_block_info_offset'], f)
        pack_into(self.obj11s, dump_block1, dump_pak_header['obj11_offset'], f)
        pack_into(self.pfield_infos, dump_block1, dump_pak_header['pfield_info_offset'], f)
        pack_into(self.gfx_block_infos, dump_block1, dump_pak_header['gfx_block_info_offset'], f)
        pack_into(self.obj14_infos, dump_block1, dump_pak_header['obj14_info_offset'], f)
        pack_into(self.dump_animation_block_infos, dump_block1, dump_pak_header['animation_block_info_offset'], f)

        obj_map = {
            i:dump_pak_header['mat1_offset']+pak.Mat1[f].itemsize*j for i,j in self.mat1_map.items()
        }
        obj_map.update({
            i:dump_pak_header['mat2_offset']+pak.Mat2[f].itemsize*j for i,j in self.mat2_map.items()
        })
        obj_map.update({
            i:dump_pak_header['mat3_offset']+pak.Mat3[f].itemsize*j for i,j in self.mat3_map.items()
        })
        obj_map.update({
            i:dump_pak_header['mat4_offset']+pak.Mat4[f].itemsize*j for i,j in self.mat4_map.items()
        })
        obj_map.update({
            i:dump_pak_header['mat_extra_offset']+pak.MatExtra[f].itemsize*j for i,j in self.mat_extra_map.items()
        })
        obj_map.update({
            i:dump_pak_header['shape_info_offset']+pak.ShapeInfo[f].itemsize*j for i,j in self.shape_info_map.items()
        })
        obj_map.update({
            i:dump_pak_header['hk_constraint_data_offset']+pak.HkConstraintData[f].itemsize*j for i,j in self.hk_constraint_data_map.items()
        })
        obj_map.update({
            i:dump_pak_header['hk_constraint_info_offset']+pak.HkConstraintInfo[f].itemsize*j for i,j in self.hk_constraint_info_map.items()
        })
        obj_map.update({
            i:dump_pak_header['hk_shape_info_offset']+pak.HkShapeInfo[f].itemsize*j for i,j in self.hk_shape_info_map.items()
        })
        obj_map.update({
            i:dump_pak_header['buffer_info_offset']+pak.BufferInfo[f].itemsize*j for i,j in self.buffer_info_map.items()
        })
        obj_map.update({
            i:dump_pak_header['ibuff_info_offset']+pak.IBuffInfo[f].itemsize*j for i,j in self.ibuff_info_map.items()
        })
        obj_map.update({
            i:dump_pak_header['vbuff_info_offset']+pak.VBuffInfo[f].itemsize*j for i,j in self.vbuff_info_map.items()
        })

        for offset in self.block2_offsets:
            if (new_val := obj_map.get(offset['val'], None)) is not None:
                offset['val'] = new_val
            for (off, start, end) in self.remaps:
                if start <= offset['val'] and end > offset['val']:
                    offset['val'] = offset['val'] + off - start
            offset_ = unpack_from(Uint[f], dump_block1, offset['val'])['val']
            if (new_val := obj_map.get(offset_, None)) is not None:
                pack_into(new(Uint[f], new_val), dump_block1, offset['val'], f)
            for (off, start, end) in self.remaps:
                if start <= offset_ and end > offset_:
                    pack_into(new(Uint[f], offset_ + off - start), dump_block1, offset['val'], f)

        dump_block1 = dump_block1
        block1_offset = len(dump_block1)
        off = (block1_offset + 15) & 0xfffffff0
        dump_block1 += bytes(off - block1_offset)
        block1_offset = off
        dump_pak_header['sub_blocks1_offset'] = block1_offset
        # data = self.sub_blocks1.pack(f)
        data = self.sub_blocks1.pack(f)
        block1_offset += len(data)
        dump_block1 += data
        off = (block1_offset + 31) & 0xffffffe0
        dump_block1 += bytes(off - block1_offset)
        block1_offset = off
        dump_pak_header['string_keys_offset'] = block1_offset
        self.dump_block1 = bytes(dump_block1 + self.string_keys.pack(f))

        off =  (pak_offset + 4095) & 0xfffff000
        pak_dump += bytes(off - pak_offset)
        pak_offset = off
        data = CompressedBlock(self.dump_block1)
        data_comp = data.pack(compress)
        # data = self.block1.pack()
        dump_pak_header['block1_offset'] = pak_offset
        dump_pak_header['block1_size'] = data.size
        dump_pak_header['block1_size_comp'] = data.size_comp
        pak_offset += len(data_comp)
        pak_dump += data_comp

        dump_pak_header['sub_blocks2_offset'] = 0
        dump_block2 = self.sub_blocks2.pack(f)
        dump_pak_header['block2_offsets_offset'] = len(dump_block2)
        dump_pak_header['block2_offsets_num'] = self.block2_offsets.size
        self.dump_block2 = dump_block2 + pack(self.block2_offsets, f)

        off =  (pak_offset + 4095) & 0xfffff000
        pak_dump += bytes(off - pak_offset)
        pak_offset = off
        data = CompressedBlock(self.dump_block2)
        data_comp = data.pack(compress)
        dump_pak_header['block2_offset'] = pak_offset
        dump_pak_header['block2_size'] = data.size
        dump_pak_header['block2_size_comp'] = data.size_comp
        pak_offset += len(data_comp)
        pak_dump += data_comp

        off =  (pak_offset + 4095) & 0xfffff000
        pak_dump += bytes(off - pak_offset)
        pak_offset = off
        data = pack_strings(self.pak_strings, f)
        dump_pak_header['strings_offset'] = pak_offset
        dump_pak_header['strings_num'] = len(self.pak_strings)
        dump_pak_header['strings_size'] = len(data)
        pak_offset += dump_pak_header['strings_size']
        pak_dump += data

        dump_pak_header['blockA_offset'] = pak_offset
        dump_pak_header['blockA_num'] = self.pak_blockA.size
        pak_dump += pack(self.pak_blockA, f)

        pak_dump = pack(dump_pak_header, f) + bytes(pak_dump)

        return pak_dump, bin_dump