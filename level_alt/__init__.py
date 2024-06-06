from collections import OrderedDict
from  ..level import pak as pak_
from  ..level import bin as bin_
from ..utils import *
from ..types import *
from .pak import *

class LevelData:
    def __init__(self, file):
        with open(f"{file}.pak", "rb") as f:
            data = f.read()
        if data[8] == 19:
            self.f = '<'
        elif data[11] == 19:
            self.f = '>'
            
        with open(f"{file}.bin", "rb") as f:
            bin_data = f.read()
        self.bin_data = bin_data
        self.pak_data = data

        self.keys = get_global_keys()
        self.game_objs_types = {}

        self.bin_header = unpack_from(bin_.Header[self.f], bin_data, 0)
        self.bin_strings = read_strings(bin_data, self.bin_header['strings_offset'], self.bin_header['strings_num'], self.f)
        self.keys.update({hash_string(i):i.decode() for i in self.bin_strings})

        self.asset_handles = unpack_list_from(bin_.AssetHandle[self.f], bin_data, self.bin_header['asset_handle_offset'], self.bin_header['asset_handle_num'])

        asset_data = {
            (info['key'], info['type']): CompressedBlock.unpack_from(bin_data, info['size'], info['size_comp'], info['offset']).data for info in self.asset_handles
        }

        self.radiosity = None
        for info in self.asset_handles:
            if self.keys[info['key']].endswith('_radiosity'):
                self.radiosity = ((key:=(info['key'], info['type'])), asset_data.pop(key))
                break
        if self.radiosity is None:
            raise ValueError("Radiosity not found")

        self.pak_header = unpack_from(pak_.Header[self.f], data, 0)

        self.pak_strings = read_strings(data, self.pak_header['strings_offset'], self.pak_header['strings_num'], self.f)
        self.keys.update({hash_string(i):i.decode() for i in self.pak_strings})
        
        self.block1 = CompressedBlock.unpack_from(data, self.pak_header['block1_size'], self.pak_header['block1_size_comp'], self.pak_header['block1_offset']).data
        self.block2 = CompressedBlock.unpack_from(data, self.pak_header['block2_size'], self.pak_header['block2_size_comp'], self.pak_header['block2_offset']).data

        self.sub_blocks2 = SubBlocks.unpack_from(self.block2, self.pak_header['sub_blocks2_offset'], self.keys, self.game_objs_types, self.f)
        self.block2_offsets = unpack_list_from(Uint[self.f], self.block2, self.pak_header['block2_offsets_offset'], self.pak_header['block2_offsets_num'])
        
        self.objas = unpack_list_from(pak_.ObjA[self.f], self.block1, self.pak_header['obja_offset'], self.pak_header['obja_num'])
        self.obj0s = unpack_list_from(pak_.Obj0[self.f], self.block1, self.pak_header['obj0_offset'], self.pak_header['obj0_num'])
        self.animation_block_infos = unpack_list_from(pak_.AnimationBlockInfo[self.f], self.block1, self.pak_header['animation_block_info_offset'], self.pak_header['animation_block_info_num'])
        self.pfield_infos = unpack_list_from(pak_.PFieldInfo[self.f], self.block1, self.pak_header['pfield_info_offset'], self.pak_header['pfield_info_num'])
        
        self.meshes = OrderedDict()
        for i in range(self.pak_header['mesh_info_num']):
            if unpack_from(Uint[self.f], self.block1, self.pak_header['mesh_info_offset'] + i * Mesh.Info[self.f].itemsize + 136)['val'] != 0:
                mesh = Mesh(self.block1, self.pak_header['mesh_info_offset'] + i * Mesh.Info[self.f].itemsize, self.f)
            else:
                mesh = TerrainMesh(self.block1, self.pak_header['mesh_info_offset'] + i * Mesh.Info[self.f].itemsize, self.f)
            if len(mesh.vbuffs) == 0 and len(mesh.ibuffs) == 0:
                mesh.vertex_data = None
            else:
                assert(mesh.info['vbuff_num'] != 0 or mesh.info['ibuff_num'] != 0)
                mesh.vertex_data = asset_data.pop((mesh.info['asset_key'], mesh.info['asset_type']))
            self.meshes[mesh.info['key']] = mesh
            
        self.effects = {
            info['key']: GameObjs.unpack_from(self.block1, info['offset'], info['size'], self.game_objs_types, self.f, info['level_flag'])
            for info in unpack_list_from(pak_.EffectInfo[self.f], self.block1, self.pak_header['effect_info_offset'], self.pak_header['effect_info_num'])
        }
        self.gfx_blocks = {
            info['key']: self.block1[info['offset']:info['offset']+info['size']]
            for info in unpack_list_from(pak_.GFXBlockInfo[self.f], self.block1, self.pak_header['gfx_block_info_offset'], self.pak_header['gfx_block_info_num'])
        }
        self.light_blocks = {
            info['guid']: unpack_list_from(Uint[self.f], self.block1, info['offset'], info['num']) 
            for info in unpack_list_from(pak_.IllumationInfo[self.f], self.block1, self.pak_header['illumation_info_offset'], self.pak_header['illumation_info_num'])
        }
        self.textures = {
            info['key']: (info, asset_data.pop(((info['asset_key']), info['asset_type']), None), asset_data.pop((hash_string('*', info['asset_key']), info['asset_type']), None))
            for info in unpack_list_from(pak_.TextureInfo[self.f], self.block1, self.pak_header['texture_info_offset'], self.pak_header['texture_info_num'])
        }
        self.foliages = OrderedDict()
        for info in unpack_list_from(pak_.FoliageInfo[self.f], self.block1, self.pak_header['foliage_info_offset'], self.pak_header['foliage_info_num']):
            if info['key'] not in self.foliages: self.foliages[info['key']] = []
            self.foliages[info['key']].append((info, unpack_list_from(Uint[self.f], self.block1, info['offset'], (info['s1b'] - info['s1a']) * (info['s2b'] - info['s2a']) * 2)))

        blocks = [CompressedBlock.unpack_from(data, info['size'], info['size_comp'], info['offset']).data for info in self.animation_block_infos]
        offsets = OrderedDict((k, 0) for k in blocks)
        self.animations = {}
        anim_infos = unpack_list_from(lotrc.level.pak.AnimationInfo[self.f], self.block1, self.pak_header['animation_info_offset'], self.pak_header['animation_info_num'])
        for i, block in enumerate(blocks):
            offset = 0
            level_flag = 1 << i
            for info in anim_infos:
                if level_flag & info['level_flag'] != 0:
                    if info['key'] not in self.animations:
                        self.animations[info['key']] = pak.Animation(info, block, offset, self.f)
                    offset += info['size']

        # for info in range(self.pak_header['animation_info_num']):
        #     anim = pak.Animation(
        #         self.block1,
        #         self.pak_header['animation_info_offset'] + i*lotrc.level.pak.AnimationInfo['<'].itemsize,
        #         offsets,
        #         blocks,
        #         self.f
        #     )
        #     self.animations[anim.info['key']] = anim

        self.sub_blocks1 = SubBlocks.unpack_from(self.block1, self.pak_header['sub_blocks1_offset'], self.keys, self.game_objs_types, self.f)
        self.string_keys = StringKeys.unpack_from(self.block1, self.pak_header['string_keys_offset'], self.f)
        self.pak_blockA = unpack_list_from(pak_.BlockAVal[self.f], data, self.pak_header['blockA_offset'], self.pak_header['blockA_num'])
        print(len(asset_data))

    def dump(self, f='<', compress=True):

        ######### bin stuff
        asset_data = OrderedDict()
        bin_header = self.bin_header.copy()
        bin_header['version'] = 1 if f == '<' else 2

        mesh_datas = [self.radiosity]
        for mesh in self.meshes.values():
            if mesh.vertex_data is not None:
                mesh_datas.append(((mesh.info['asset_key'], mesh.info['asset_type']), mesh.vertex_data))
                # asset_data[(mesh.info['asset_key'], mesh.info['asset_type'])] = mesh.vertex_data
        bin_header['vdata_num'] = len(mesh_datas)
        bin_header['vdata_num_'] = len(mesh_datas)
        asset_data.update(sorted(mesh_datas, key=lambda x: x[0][0]))

        texture_infos = []
        texture_datas = []
        for info, data0, data1 in self.textures.values():
            if data0 is not None:
                texture_datas.append(((info['asset_key'], info['asset_type']), data0))
            if data1 is not None:
                texture_datas.append(((hash_string('*', info['asset_key']), info['asset_type']), data1))
            texture_infos.append(info.copy())
        bin_header['texdata_num'] = len(texture_datas)
        asset_data.update(sorted(texture_datas, key=lambda x: x[0][0]))
        def sort_textures(x):
            if x['key'] == 3804089404:
                return 0
            elif x['key'] == 4026460901:
                return 1
            return x['key']
        texture_infos = np.stack(sorted(texture_infos, key=sort_textures))

        offset = bin_header.nbytes
        bin_data = bytearray()
        off =  (offset + 2047) & 0xfffff800
        bin_data += bytes(off - offset)
        offset = off

        asset_handles = []
        for (key, ty), data in asset_data.items():
            data = CompressedBlock(data)
            data_comp = data.pack(compress)
            asset_handles.append((key, offset, data.size, data.size_comp, ty))
            if len(data_comp) != 0:
                bin_data += data_comp
                offset += len(data_comp)
                off =  (offset + 2047) & 0xfffff800
                bin_data += bytes(off - offset)
                offset = off
        asset_handles = new(bin_.AssetHandle[f], asset_handles)
        off =  (offset + 2047) & 0xfffff800
        bin_data += bytes(off - offset)
        offset = off

        bin_header['asset_handle_offset']  = offset
        bin_header['asset_handle_num'] = len(asset_handles)
        data = pack(asset_handles, f)
        offset += len(data)
        bin_data += data

        bin_header['strings_offset'] = offset
        data = pack_strings(self.bin_strings, f)
        bin_header['strings_num'] = len(self.bin_strings)
        bin_header['strings_size'] = len(data)
        bin_data += data
        offset += len(data)

        off = (offset + 2047) & 0xfffff800
        bin_data += bytes(off - offset)
        offset = off

        bin_data = pack(bin_header, f) + bytes(bin_data)

        #### pak stuff
        pak_header = self.pak_header.copy()
        pak_header['version'] = 1 if f == '<' else 2

        pak_offset = pak_header.nbytes
        pak_data = bytes()
        
        #### block1 stuff
        
        (
            pak_header['shape_info_num'],
            pak_header['hk_shape_info_num'],
            pak_header['hk_constraint_info_num'],
            pak_header['hk_constraint_data_num'],
            pak_header['mat1_num'],
            pak_header['mat2_num'],
            pak_header['mat3_num'],
            pak_header['mat4_num'],
            pak_header['mat_extra_num'],
            pak_header['buffer_info_num'],
            pak_header['vbuff_info_num'],
            pak_header['ibuff_info_num'],
        ) = sum(i.infos_count() for i in self.meshes.values()).tolist()
        pak_header['mesh_info_num'] = len(self.meshes)
        pak_header['texture_info_num'] = texture_infos.size
        pak_header['effect_info_num'] = len(self.effects)
        pak_header['gfx_block_info_num'] = len(self.gfx_blocks)
        pak_header['illumation_info_num'] = len(self.light_blocks)
        pak_header['animation_block_info_offset'] = self.animation_block_infos.size
        pak_header['foliage_info_num'] = sum(len(i) for i in self.foliages.values())
        pak_header['animation_info_num'] = len(self.animations)
        
        block1 = bytes()
        pak_header['obja_offset'] = len(block1)
        block1 += bytes(pak_header['obja_num'] * pak_.ObjA[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['obj0_offset'] = len(block1)
        block1 += bytes(pak_header['obj0_num'] * pak_.Obj0[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['mesh_info_offset'] = len(block1)
        block1 += bytes(pak_header['mesh_info_num'] * pak_.MeshInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['buffer_info_offset'] = len(block1)
        block1 += bytes(pak_header['buffer_info_num'] * pak_.BufferInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['mat1_offset'] = len(block1)
        block1 += bytes(pak_header['mat1_num'] * pak_.Mat1[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['mat2_offset'] = len(block1)
        block1 += bytes(pak_header['mat2_num'] * pak_.Mat2[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['mat3_offset'] = len(block1)
        block1 += bytes(pak_header['mat3_num'] * pak_.Mat3[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['mat4_offset'] = len(block1)
        block1 += bytes(pak_header['mat4_num'] * pak_.Mat4[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['mat_extra_offset'] = len(block1)
        block1 += bytes(pak_header['mat_extra_num'] * pak_.MatExtra[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['shape_info_offset'] = len(block1)
        block1 += bytes(pak_header['shape_info_num'] * pak_.ShapeInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['hk_shape_info_offset'] = len(block1)
        block1 += bytes(pak_header['hk_shape_info_num'] * pak_.HkShapeInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['hk_constraint_data_offset'] = len(block1)
        block1 += bytes(pak_header['hk_constraint_data_num'] * pak_.HkConstraintData[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['vbuff_info_offset'] = len(block1)
        block1 += bytes(pak_header['vbuff_info_num'] * pak_.VBuffInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['ibuff_info_offset'] = len(block1)
        block1 += bytes(pak_header['ibuff_info_num'] * pak_.IBuffInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['texture_info_offset'] = len(block1)
        block1 += bytes(pak_header['texture_info_num'] * pak_.TextureInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['animation_info_offset'] = len(block1)
        block1 += bytes(pak_header['animation_info_num'] * pak_.AnimationInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['hk_constraint_info_offset'] = len(block1)
        block1 += bytes(pak_header['hk_constraint_info_num'] * pak_.HkConstraintInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['effect_info_offset'] = len(block1)
        block1 += bytes(pak_header['effect_info_num'] * pak_.EffectInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['pfield_info_offset'] = len(block1)
        block1 += bytes(pak_header['pfield_info_num'] * pak_.PFieldInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['gfx_block_info_offset'] = len(block1)
        block1 += bytes(pak_header['gfx_block_info_num'] * pak_.GFXBlockInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['animation_block_info_offset'] = len(block1)
        block1 += bytes(pak_header['animation_block_info_num'] * pak_.AnimationBlockInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['foliage_info_offset'] = len(block1)
        block1 += bytes(pak_header['foliage_info_num'] * pak_.FoliageInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['illumation_info_offset'] = len(block1)
        block1 += bytes(pak_header['illumation_info_num'] * pak_.IllumationInfo[f].itemsize)
        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        
        infos = {
            'header': pak_header,
            'mesh': [],
            'buffer': [],
            'mat1': [],
            'mat2': [],
            'mat3': [],
            'mat4': [],
            'mat_extra': [],
            'shape': [],
            'hk_shape': [],
            'hk_constraint_data': [],
            'vbuff': [],
            'ibuff': [],
            'hk_constraint': [],
            'block2_offsets': [],
            'animation': [],
        }
        
        print("infos done", len(block1))

        offset = 0
        animation_vals = []
        for k in sorted(self.animations.keys()):
            vals = self.animations[k].dump(offset, infos)
            offset += len(vals)
            animation_vals.append((vals, self.animations[k].info['level_flag']))
        offset = 0
        animation_blocks = []
        for i in range(len(self.animation_block_infos)):
            block = bytes()
            level_flag = 1 << i
            for (vals, level_flag_) in animation_vals:
                if level_flag_ & level_flag != 0:
                    block += vals
            animation_blocks.append(block)

        animation_block_infos = self.animation_block_infos.copy()
        for info, data in zip(animation_block_infos, animation_blocks):
            # data = animation_blocks[info['key']]
            off =  (pak_offset + 4095) & 0xfffff000
            pak_data += bytes(off - pak_offset)
            pak_offset = off
            data = CompressedBlock(bytes(data))
            data_comp = data.pack(compress)
            info['offset'] = pak_offset
            info['size'] = data.size
            info['size_comp'] = data.size_comp
            pak_offset += len(data_comp)
            pak_data += data_comp
        print("animations done")
        
        effects = []
        for key, effect in sorted(self.effects.items(), key=lambda x: x[0]):
            vals = effect.dump(f)
            effects.append((key, effect.level_flag, len(block1), len(vals)))
            block1 += vals
        effects = new(pak_.EffectInfo[f], effects)
        infos['effects'] = effects
        print("effects done", len(block1))

        key_occluder = hash_string('occluder')
        normal = []
        collisions_roads = []
        terrain = []
        for k in self.meshes.keys():
            if k == key_occluder:
                continue
            elif k in self.keys and self.keys[k].startswith("Terrain"):
                terrain.append(k)
            elif k in self.keys and ("_Road_" in self.keys[k] or "_Collision_" in self.keys[k]):
                collisions_roads.append(k)
            else:
                normal.append(k)
        # for key in sorted(normal) + collisions_roads:
        for key in sorted(normal) + sorted(collisions_roads, key=lambda x: int(self.keys[x].split('_')[-1])):
            vals = self.meshes[key].dump(len(block1), infos, f)
            block1 += vals
            block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))

        terrain_start_offset = len(block1)
        block1 += b'\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff'
        for key in sorted(terrain, key=lambda x: int(self.keys[x].split('_')[-1])):
            vals = self.meshes[key].dump(len(block1), terrain_start_offset, infos, f)
            block1 += vals

        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        foliages = []
        for info, val in [j for i in self.foliages.values() for j in i]:
            info = info.copy()
            info['offset'] = len(block1)
            block1 += pack(val, f)
            block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
            foliages.append(info)
        infos['foliage'] = foliages

        for key in [key_occluder]:
            vals = self.meshes[key].dump(len(block1), infos, f)
            block1 += vals
            block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        
        assert(len(infos['mesh']) == pak_header['mesh_info_num'])
        assert(len(infos['shape']) == pak_header['shape_info_num'])
        assert(len(infos['hk_shape']) == pak_header['hk_shape_info_num'])
        assert(len(infos['hk_constraint']) == pak_header['hk_constraint_info_num'])
        assert(len(infos['hk_constraint_data']) == pak_header['hk_constraint_data_num'])
        assert(len(infos['mat1']) == pak_header['mat1_num'])
        assert(len(infos['mat2']) == pak_header['mat2_num'])
        assert(len(infos['mat3']) == pak_header['mat3_num'])
        assert(len(infos['mat4']) == pak_header['mat4_num'])
        assert(len(infos['mat_extra']) == pak_header['mat_extra_num'])
        assert(len(infos['buffer']) == pak_header['buffer_info_num'])
        assert(len(infos['vbuff']) == pak_header['vbuff_info_num'])
        assert(len(infos['ibuff']) == pak_header['ibuff_info_num'])
        assert(len(infos['foliage']) == pak_header['foliage_info_num'])
        assert(len(infos['animation']) == pak_header['animation_info_num'])
        print("meshes done", len(block1), len(normal), len(collisions_roads), len(terrain))

        gfx_blocks = []
        for key, val in self.gfx_blocks.items():
            gfx_blocks.append((key, len(block1), len(val)))
            block1 += val
            block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        gfx_blocks = new(pak_.GFXBlockInfo[f], gfx_blocks)
        infos['gfx_blocks'] = gfx_blocks

        light_blocks = []
        for guid, val in self.light_blocks.items():
            light_blocks.append((guid, len(val), len(block1)))
            block1 += pack(val, f)
            block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        light_blocks = new(pak_.IllumationInfo[f], light_blocks)
        infos['light_blocks'] = light_blocks

        block1 += bytes(((len(block1) + 15) & 0xFFFFFFF0) - len(block1))
        pak_header['sub_blocks1_offset'] = len(block1)
        block1 += self.sub_blocks1.pack(f)
        pak_header['string_keys_offset'] = len(block1)
        block1 += self.string_keys.pack(f)

        block1 = bytearray(block1)
        pack_into(self.objas, block1, pak_header['obja_offset'], f)
        pack_into(self.obj0s, block1, pak_header['obj0_offset'], f)
        pack_into(np.stack(infos['mesh']), block1, pak_header['mesh_info_offset'], f)
        pack_into(np.stack(infos['buffer']), block1, pak_header['buffer_info_offset'], f)
        if pak_header['mat1_num'] != 0:
            pack_into(np.stack(infos['mat1']), block1, pak_header['mat1_offset'], f)
        if pak_header['mat2_num'] != 0:
            pack_into(np.stack(infos['mat2']), block1, pak_header['mat2_offset'], f)
        if pak_header['mat3_num'] != 0:
            pack_into(np.stack(infos['mat3']), block1, pak_header['mat3_offset'], f)
        if pak_header['mat4_num'] != 0:
            pack_into(np.stack(infos['mat4']), block1, pak_header['mat4_offset'], f)
        pack_into(np.stack(infos['mat_extra']), block1, pak_header['mat_extra_offset'], f)
        pack_into(np.stack(infos['shape']), block1, pak_header['shape_info_offset'], f)
        for i, hk_shape in enumerate(infos['hk_shape']):
            pack_into(hk_shape, block1, pak_header['hk_shape_info_offset'] + i * pak_.HkShapeInfo[f].itemsize, f)
        pack_into(np.stack(infos['hk_constraint_data']), block1, pak_header['hk_constraint_data_offset'], f)
        pack_into(np.stack(infos['vbuff']), block1, pak_header['vbuff_info_offset'], f)
        pack_into(np.stack(infos['ibuff']), block1, pak_header['ibuff_info_offset'], f)
        pack_into(texture_infos, block1, pak_header['texture_info_offset'], f)
        pack_into(np.stack(infos['animation']), block1, pak_header['animation_info_offset'], f)
        pack_into(np.stack(infos['hk_constraint']), block1, pak_header['hk_constraint_info_offset'], f)
        pack_into(effects, block1, pak_header['effect_info_offset'], f)
        pack_into(self.pfield_infos, block1, pak_header['pfield_info_offset'], f)
        pack_into(gfx_blocks, block1, pak_header['gfx_block_info_offset'], f)
        pack_into(animation_block_infos, block1, pak_header['animation_block_info_offset'], f)
        if len(foliages) != 0:
            pack_into(np.stack(foliages), block1, pak_header['foliage_info_offset'], f)
        pack_into(light_blocks, block1, pak_header['illumation_info_offset'], f)
        
        block1 = bytes(block1)

        #### block2
        block2_offsets = []
        for i, mesh in enumerate(infos['mesh']):
            block2_offsets.extend([
                pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 8,
                pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 12,
                pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 48,
                pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 140,
                pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 144,
                pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 152,
                pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 164,
                pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 172,
                pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 180,
                pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 252,
            ])
            if mesh['keys_offset'] != 0:
                block2_offsets.append(pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 136)
            if mesh['valIs_offset'] != 0:
                block2_offsets.append(pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 160)
            if mesh['valJs_offset'] != 0:
                block2_offsets.append(pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 196)
            if mesh['block_offset'] != 0:
                block2_offsets.append(pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 200)
            if mesh['valKs_offset'] != 0:
                block2_offsets.append(pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 204)
            if mesh['shape_offset'] != 0:
                block2_offsets.append(pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 224)
            if mesh['hk_constraint_data_offset'] != 0:
                block2_offsets.append(pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 232)
            if mesh['hk_constraint_offset'] != 0:
                block2_offsets.append(pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 240)
            if mesh['keys2_offset'] != 0:
                block2_offsets.append(pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 244)
            if mesh['keys2_order_offset'] != 0:
                block2_offsets.append(pak_header['mesh_info_offset'] + i * Mesh.Info['<'].itemsize + 248)
                
        for i, buffer in enumerate(infos['buffer']):
            block2_offsets.extend([
                pak_header['buffer_info_offset'] + i * pak_.BufferInfo['<'].itemsize,
                pak_header['buffer_info_offset'] + i * pak_.BufferInfo['<'].itemsize + 260,
            ])
            if buffer['vbuff_info_offset_2'] != 0:
                block2_offsets.append(pak_header['buffer_info_offset'] + i * pak_.BufferInfo['<'].itemsize + 4)
            if buffer['vbuff_info_offset_3'] != 0:
                block2_offsets.append(pak_header['buffer_info_offset'] + i * pak_.BufferInfo['<'].itemsize + 8)
                
        for i, mat in enumerate(infos['mat1']):
            if mat['mat_extra_offset'] != 0:
                block2_offsets.append(pak_header['mat1_offset'] + i * pak_.Mat1['<'].itemsize + 344)
        for i, mat in enumerate(infos['mat2']):
            if mat['mat_extra_offset'] != 0:
                block2_offsets.append(pak_header['mat2_offset'] + i * pak_.Mat2['<'].itemsize + 344)
        for i, mat in enumerate(infos['mat3']):
            if mat['mat_extra_offset'] != 0:
                block2_offsets.append(pak_header['mat3_offset'] + i * pak_.Mat3['<'].itemsize + 344)
        for i, mat in enumerate(infos['mat4']):
            if mat['mat_extra_offset'] != 0:
                block2_offsets.append(pak_header['mat4_offset'] + i * pak_.Mat4['<'].itemsize + 344)
        for i, shape in enumerate(infos['shape']):
            if shape['hk_shape_offset'] != 0:
                block2_offsets.append(pak_header['shape_info_offset'] + i * pak_.ShapeInfo['<'].itemsize + 112)
        for i, hk_shape in enumerate(infos['hk_shape']):
            if hk_shape['type'] == 5:
                block2_offsets.extend([
                    pak_header['hk_shape_info_offset'] + i * pak_.HkShapeInfo['<'].itemsize + 44,
                    pak_header['hk_shape_info_offset'] + i * pak_.HkShapeInfo['<'].itemsize + 52,
                ])
            elif hk_shape['type'] == 6:
                block2_offsets.extend([
                    pak_header['hk_shape_info_offset'] + i * pak_.HkShapeInfo['<'].itemsize + 60,
                    pak_header['hk_shape_info_offset'] + i * pak_.HkShapeInfo['<'].itemsize + 68,
                    pak_header['hk_shape_info_offset'] + i * pak_.HkShapeInfo['<'].itemsize + 76,
                ])
        for i, hk_constraint in enumerate(infos['hk_constraint']):
            block2_offsets.extend([
                pak_header['hk_constraint_info_offset'] + i * pak_.HkConstraintInfo['<'].itemsize + 4,
                pak_header['hk_constraint_info_offset'] + i * pak_.HkConstraintInfo['<'].itemsize + 12,
                pak_header['hk_constraint_info_offset'] + i * pak_.HkConstraintInfo['<'].itemsize + 20,
                pak_header['hk_constraint_info_offset'] + i * pak_.HkConstraintInfo['<'].itemsize + 40,
                pak_header['hk_constraint_info_offset'] + i * pak_.HkConstraintInfo['<'].itemsize + 48,
            ])
            if hk_constraint['vals2_offset'] != 0:
                block2_offsets.append(pak_header['hk_constraint_info_offset'] + i * pak_.HkConstraintInfo['<'].itemsize + 60)

        for i in range(pak_header['effect_info_num']):
            block2_offsets.append(pak_header['effect_info_offset'] + i * pak_.EffectInfo['<'].itemsize + 8)    
        for i in range(pak_header['gfx_block_info_num']):
            block2_offsets.append(pak_header['gfx_block_info_offset'] + i * pak_.GFXBlockInfo['<'].itemsize + 4)
        for i in range(pak_header['illumation_info_num']):
            block2_offsets.append(pak_header['illumation_info_offset'] + i * pak_.IllumationInfo['<'].itemsize + 8)
        for i in range(pak_header['foliage_info_num']):
            block2_offsets.append(pak_header['foliage_info_offset'] + i * pak_.FoliageInfo['<'].itemsize + 28)
        block2_offsets.extend(infos['block2_offsets'])
        infos['block2_offsets'] = block2_offsets
        block2_offsets = new(Uint[f], block2_offsets)
        pak_header['sub_blocks2_offset'] = 0
        block2 = self.sub_blocks2.pack(f)
        pak_header['block2_offsets_offset'] = len(block2)
        pak_header['block2_offsets_num'] = len(block2_offsets)
        block2 += pack(block2_offsets, f)

        ### pack everything else
        off =  (pak_offset + 4095) & 0xfffff000
        pak_data += bytes(off - pak_offset)
        pak_offset = off
        data = CompressedBlock(block1)
        data_comp = data.pack(compress)
        pak_header['block1_offset'] = pak_offset
        pak_header['block1_size'] = data.size
        pak_header['block1_size_comp'] = data.size_comp
        pak_offset += len(data_comp)
        pak_data += data_comp

        off =  (pak_offset + 4095) & 0xfffff000
        pak_data += bytes(off - pak_offset)
        pak_offset = off
        data = CompressedBlock(block2)
        data_comp = data.pack(compress)
        pak_header['block2_offset'] = pak_offset
        pak_header['block2_size'] = data.size
        pak_header['block2_size_comp'] = data.size_comp
        pak_offset += len(data_comp)
        pak_data += data_comp

        off =  (pak_offset + 4095) & 0xfffff000
        pak_data += bytes(off - pak_offset)
        pak_offset = off
        data = pack_strings(self.pak_strings, f)
        pak_header['strings_offset'] = pak_offset
        pak_header['strings_num'] = len(self.pak_strings)
        pak_header['strings_size'] = len(data)
        pak_offset += pak_header['strings_size']
        pak_data += data

        pak_header['blockA_offset'] = pak_offset
        pak_header['blockA_num'] = self.pak_blockA.size
        pak_data += pack(self.pak_blockA, f)

        pak_data = pack(pak_header, f) + bytes(pak_data)
        infos['block1'] = block1
        infos['block2'] = block2

        return infos, pak_data, bin_data