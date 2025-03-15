import bpy
import pathlib
import numpy as np
from mathutils import Matrix, Vector, Quaternion
from .. import lotrc
from ..loader import GEOM_TREES, LOADED_LEVELS
from .conv import *

class DumpModels(bpy.types.Operator):
    """Dump Models from a Lord of the Rings Conquest Level"""
    bl_idname = "lotrc.load_models"
    bl_label = "Load LOTRC Models"

    def execute(self, context):
        model = context.scene.lotrc_props.selected_model
        import_models(LOADED_LEVELS[context.scene.name], model, context)
        return {'FINISHED'}

CLASSES = [LoadModels, ClearModels]

UNKNOWN = np.uint32(1)
STATIC = np.uint32(2)
SKINNED = np.uint32(4)
PHYSICS = np.uint32(8)
BREAKABLE = np.uint32(16)

def parse_skeleton(model, arma_obj):
    bones = arma_obj['bones']

    bone_order = {bone.name: i for i,bone in enumerate(bones)}
    bone_parents = []
    bone_mats = []
    for name in bones:
        bone = arma_obj.data.bones[name]
        mat = bone.matrix_local
        bone_parent = -1
        if (parent := bone.parent) is not None:
            mat = parent.matrix_local.inverted() @ mat
            bone_parent = bone_order[parent.name]
        bone_parents.append(bone_parent)
        bone_mats.append(mat_from_blender(mat))
    model.bones = bones
    model.bone_parents = bone_parents
    model.bone_mats = bone_mats

def parse_hk_skeleton(arma_obj):
    hk_constraint = lotrc.pak_alt.HkConstraint()
    hk_constraint.info = lotrc.pak.HkConstraintInfo.from_json(arma_obj['info'])
    hk_constraint.vals2 = arma_obj['vals2']
    hk_constraint.bone_names = arma_obj['bone_names'] 
    hk_constraint.bone_order = arma_obj['bone_order']

    bones = [i for i, _ in arma_obj['bone_names']] 

    bone_order = {bone.name: i for i,bone in enumerate(bones)}
    bone_parents = []
    bone_transforms = []
    for name in bones:
        bone = arma_obj.data.bones[name]
        mat = bone.matrix_local
        bone_parent = -1
        if (parent := bone.parent) is not None:
            mat = parent.matrix_local.inverted() @ mat
            bone_parent = bone_order[parent.name]
        bone_parents.append(bone_parent)
        transform = lotrc.pak_alt.TRS()
        transform.translation = pos_from_blender_single(mat.to_translation())
        transform.rotation = quat_from_blender(mat.to_quaternion())
        transform.scale = size_from_blender(mat.to_scale())
        bone_transforms.append(transform)

    hk_constraint.bone_parents = bone_parents
    hk_constraint.bone_transforms = bone_transforms
    return hk_constraint

        )

def parse_collision(obj):
    """TBA"""
    ty = obj['type']
    if ty == 'Box':
        hkshp = lotrc.pak_alt.HkShape.Box()
    elif ty == 'Sphere':
        hkshp = lotrc.pak_alt.HkShape.Sphere()
    elif ty == 'Capsule':
        hkshp = lotrc.pak_alt.HkShape.Capsule()
    elif ty == 'Cylinder':
        hkshp = lotrc.pak_alt.HkShape.Cylinder()
    elif ty == 'ConvexVertices':
        hkshp = lotrc.pak_alt.HkShape.ConvexVertices()
    elif ty == 'BVTreeMesh':
        hkshp = lotrc.pak_alt.HkShape.BVTreeMesh()
    else:
        return 

def parse_mat():
    pass

def parse_mesh(obj, vertex_data, index_data):
    mesh = obj.data
    # check if mesh in existing data
    # if yes then just grab existing indices
    # if no then add them
    info = 
    return info

    pass


def add_mesh(info, vertex_data, index_data, usage, name, col, obj_arma, skin_bones):
    mesh = bpy.data.meshes.new(name)
    mesh['variation_id'] = info.variation_id
    mesh['variation'] = info.variation
    obj = bpy.data.objects.new(mesh.name, mesh)
    col.objects.link(obj)

    skinned = usage & SKINNED != 0 and obj_arma is not None
    if skinned:
        skin = obj.modifiers.new("Armature", "ARMATURE")
        skin.object = obj_arma
        
    inds = index_data[info.ibuff_info_offset].vals
    offset = info.vbuff_info_offset_2
    if offset == 0xFFFFFFFF:
        offset = info.vbuff_info_offset
    attrs = {i: j for i,j in vertex_data[offset].items()}
    mesh['info'] = attrs.pop('info')
    mesh.from_pydata(pos_to_blender(attrs.pop('Position')), [], [inds[i:i+3] for i in range(0,len(inds),3)])
    normals = attrs.pop('Normal', None)
    if normals is not None:
        if isinstance(normals, lotrc.pak.VertexTypes.Unorm4x8):
            normals = np.frombuffer(np.array(normals[0], 'I').tobytes(), 'B').reshape(-1, 4).astype('f') / 127.5 - 1.0
            #normals[:, [0,2]] *= normals[:, 3, None]
            attribute = mesh.attributes.new(f'raw_norms', 'FLOAT_COLOR', 'POINT')
            attribute.data.foreach_set('color', normals.flatten().copy())
        elif isinstance(normals, lotrc.pak.VertexTypes.Vector4):
            normals = np.array([normals[0], normals[1], normals[2]]).T
        mesh.normals_split_custom_set_from_vertices(pos_to_blender(normals.T))
    
    for i in range(4):
        uv = attrs.pop(f'TextureCoord({i})', None)
        if uv is not None:
            uv_layer = mesh.uv_layers.new(name='UVMap' if i == 0 else f'UV{i}')
            uv_layer.uv.foreach_set('vector', np.array([uv[0],uv[1]], 'f').T[inds].flatten())
    
    psize = attrs.pop('PSize', None)
    if psize is not None:
        attribute = mesh.attributes.new(f'PSize', 'FLOAT_VECTOR', 'POINT')
        attribute.data.foreach_set('vector', [i for j in zip(psize[0], psize[1], psize[2]) for i in j])
        tree, tree_in, tree_out = GEOM_TREES['Billboard']
        mod = obj.modifiers.new('Billboard', 'NODES')
        mod.node_group = tree
    
    weights = attrs.pop('BlendWeight', None)
    indices = attrs.pop('BlendIndices', None)
    if skinned and weights is not None and indices is not None:
        vertex_groups = [obj.vertex_groups.new(name=i) for i in skin_bones[info.skin_offset:info.skin_offset+info.skin_size]]
        n = len(weights[0])
        weights = np.array(weights[0], 'I').tobytes()
        indices = np.array(indices[0], 'I').tobytes()
        for j in range(len(weights)//4):
            for i,w in zip([2,1,0,3], weights[j*4:j*4+4]):
                if w != 0:
                    vertex_groups[indices[j*4+i]].add((j,), w/255.0, 'REPLACE')      

    for i, (attr, data) in enumerate(attrs.items()):
        if isinstance(data, lotrc.pak.VertexTypes.Vector3):
            ty = 'FLOAT_VECTOR'
            data = [i for j in zip(data[0], data[1], data[2]) for i in j]
            dat_name = 'vector'
        elif isinstance(data, lotrc.pak.VertexTypes.Vector4):
            ty = 'FLOAT_COLOR'
            data = [i for j in zip(data[0], data[1], data[2], data[3]) for i in j]
            dat_name = 'color'
        elif isinstance(data, lotrc.pak.VertexTypes.Vector2):
            ty = 'FLOAT2'
            data = [i for j in zip(data[0], data[1]) for i in j]
            dat_name = 'vector'
        elif isinstance(data, lotrc.pak.VertexTypes.Unorm4x8):
            ty = 'BYTE_COLOR'
            data = np.frombuffer(np.array(data[0], 'I').tobytes(), 'B').astype('f')/255.0
            dat_name = 'color'
        elif isinstance(data, lotrc.pak.VertexTypes.Pad):
            ty = 'INT'
            data = data[0]
            dat_name = 'value'
        else:
            # handle buffer_info case
            continue
        attribute = mesh.attributes.new(attr, ty, 'POINT')
        attribute.data.foreach_set(dat_name, data)
    return obj


def parse_model():
    pass

def parse_models():
    pass
