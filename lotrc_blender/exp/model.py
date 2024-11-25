import bpy
import pathlib
import numpy as np
from mathutils import Matrix, Vector, Quaternion
from .. import lotrc
from ..loader import GEOM_TREES, LOADED_LEVELS
from .conv import *

class DumpModels(bpy.types.Operator):
    """Load Models from a Lord of the Rings Conquest Level"""
    bl_idname = "lotrc.load_models"
    bl_label = "Load LOTRC Models"

    def execute(self, context):
        model = context.scene.lotrc_props.selected_model
        import_models(LOADED_LEVELS[context.scene.name], model, context)
        return {'FINISHED'}

class ClearModels(bpy.types.Operator):
    """Delete Previously Loaded Models from a Lord of the Rings Conquest Level"""
    bl_idname = "lotrc.clear_models"
    bl_label = "Clear Loaded LOTRC Models"

    def execute(self, context):
        level = LOADED_LEVELS[context.scene.name]
        level.col.children.unlink(level.models_col)
        level.models_col = None
        bpy.ops.outliner.orphans_purge()
        return {'FINISHED'}

CLASSES = [LoadModels, ClearModels]

UNKNOWN = np.uint32(1)
STATIC = np.uint32(2)
SKINNED = np.uint32(4)
PHYSICS = np.uint32(8)
BREAKABLE = np.uint32(16)

def add_skeleton(model, name, model_col, context):
    bones = model.bones
    if bones == []:
        return None, None
    armature_data = bpy.data.armatures.new(f"SKELETON.{name}")
    arma_obj = bpy.data.objects.new(armature_data.name, armature_data)
    arma_obj.show_in_front = True
    context.scene.collection.objects.link(arma_obj)
    context.view_layer.objects.active = arma_obj
    bpy.ops.object.mode_set(mode='EDIT', toggle=False)
    for name, mat, parent in zip(bones, model.bone_transforms, model.bone_parents):
        bone = armature_data.edit_bones.new(name)
        mat = mat_to_blender(mat)
        if parent != -1:
            bone.parent = armature_data.edit_bones[bones[parent]]
            mat = bone.parent.matrix @ mat
        bone.length = 0.2
        bone.matrix = mat
    bpy.ops.object.mode_set(mode='OBJECT', toggle=False)
    context.scene.collection.objects.unlink(arma_obj)
    model_col.objects.link(arma_obj)
    return arma_obj, bones

def add_hk_skeleton(hk_constraint, name, model_col, context):
    if hk_constraint is None:
        return None, None
    bones = [i[0] for i in hk_constraint.bone_names]
    armature_data = bpy.data.armatures.new(f"HK_SKELETON.{name}")
    arma_obj = bpy.data.objects.new(armature_data.name, armature_data)
    arma_obj.show_in_front = True
    context.scene.collection.objects.link(arma_obj)
    context.view_layer.objects.active = arma_obj
    bpy.ops.object.mode_set(mode='EDIT', toggle=False)
    for name, transform, parent in zip(bones, hk_constraint.bone_transforms, hk_constraint.bone_parents):
        bone = armature_data.edit_bones.new(name)
        mat = Matrix.LocRotScale(
            pos_to_blender_single(transform.translation),
            quat_to_blender(transform.rotation),
            size_to_blender(transform.scale),
        )
        if parent != -1:
            bone.parent = armature_data.edit_bones[bones[parent]]
            mat = bone.parent.matrix @ mat
        bone.length = 0.2
        bone.matrix = mat
    bpy.ops.object.mode_set(mode='OBJECT', toggle=False)
    context.scene.collection.objects.unlink(arma_obj)
    model_col.objects.link(arma_obj)
    return arma_obj, bones
   
def add_collision_box(obj, shape, name):
    tree, tree_in, tree_out = GEOM_TREES['Box.COLLISION']
    mod = obj.modifiers.new(name, 'NODES')
    mod.node_group = tree
    mod[tree_in.outputs['half_extents'].identifier] = size_to_blender(shape.half_extents)

def add_collision_sphere(obj, shape, name):
    tree, tree_in, tree_out = GEOM_TREES['Sphere.COLLISION']
    mod = obj.modifiers.new(name, 'NODES')
    mod.node_group = tree
    mod[tree_in.outputs['radius'].identifier] = shape.radius
    return obj

def add_collision_cylinder(obj, shape, name):
    tree, tree_in, tree_out = GEOM_TREES['Cylinder.COLLISION']
    mod = obj.modifiers.new(name, 'NODES')
    mod.node_group = tree
    mod[tree_in.outputs['point1'].identifier] = pos_to_blender_single(shape.point1)
    mod[tree_in.outputs['point2'].identifier] = pos_to_blender_single(shape.point2)
    mod[tree_in.outputs['radius'].identifier] = shape.radius
    return obj

def add_collision_capsule(obj, shape, name):
    tree, tree_in, tree_out = GEOM_TREES['Capsule.COLLISION']
    mod = obj.modifiers.new(name, 'NODES')
    mod.node_group = tree
    mod[tree_in.outputs['point1'].identifier] = pos_to_blender_single(shape.point1)
    mod[tree_in.outputs['point2'].identifier] = pos_to_blender_single(shape.point2)
    mod[tree_in.outputs['radius'].identifier] = shape.radius
    return obj

def add_collision(shape, col, base_name, i, skeleton, bones):
    par_obj = bpy.data.objects.new(f"{base_name}.COLLISION{i}", None)
    col.objects.link(par_obj)
#    bone = skeleton.pose.bones[bones[shape.info.offset]]
    if shape.info.kind != 0 and bones[shape.info.offset] != '':
        par_obj.parent = skeleton
        par_obj.parent_type = "BONE"
        par_obj.parent_bone = bones[shape.info.offset]
        # account for the position being at the tail of the bone
        par_obj.matrix_local = Matrix.Translation([0, -0.2, 0])
    par_obj.empty_display_size = 0.1
    for j, hkshp in enumerate(shape.hk_shapes):
        name = hkshp.__class__.__name__.split('_')[-1]
        name = f"{base_name}.COLLISION{i}.{name}{j}"
        mesh = bpy.data.meshes.new(name)
        obj = bpy.data.objects.new(name, mesh)
        obj.parent = par_obj
        obj.display_type = 'WIRE'      
        col.objects.link(obj)
        if isinstance(hkshp, lotrc.pak_alt.HkShape.Box):
            add_collision_box(obj, hkshp.info, name)
        elif isinstance(hkshp, lotrc.pak_alt.HkShape.Sphere):
            add_collision_sphere(obj, hkshp.info, name)
        elif isinstance(hkshp, lotrc.pak_alt.HkShape.Capsule):
            add_collision_capsule(obj, hkshp.info, name)
        elif isinstance(hkshp, lotrc.pak_alt.HkShape.Cylinder):
            add_collision_cylinder(obj, hkshp.info, name)
        elif isinstance(hkshp, lotrc.pak_alt.HkShape.ConvexVertices):
            mesh.from_pydata(pos_to_blender_alt(hkshp.shape.verts), [], [])
        elif isinstance(hkshp, lotrc.pak_alt.HkShape.BVTreeMesh):
            inds = hkshp.shape.inds
            mesh.from_pydata(pos_to_blender_alt(hkshp.shape.verts), [], [inds[i:i+3] for i in range(0,len(inds),3)])
        else:
            return
        
        obj.matrix_local = Matrix.LocRotScale(
            pos_to_blender_single(hkshp.info.translation),
            quat_to_blender(hkshp.info.rotation),
            (1,1,1)
        )

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
    attrs = {i.usage: i.val for i in vertex_data[offset]}
    mesh.from_pydata(pos_to_blender(attrs.pop(lotrc.pak.VertexUsage.Position())), [], [inds[i:i+3] for i in range(0,len(inds),3)])
    normals = attrs.pop(lotrc.pak.VertexUsage.Normal(), None)
    if normals is not None:
        if isinstance(normals, lotrc.pak.VertexTypes.Unorm4x8):
            normals = np.frombuffer(np.array(normals[0], 'I').tobytes(), 'B').reshape(-1, 4).astype('f') / 127.0 - 1.0
            #normals[:, [0,2]] *= normals[:, 3, None]
        attribute = mesh.attributes.new(f'raw_norms', 'FLOAT_COLOR', 'POINT')
        attribute.data.foreach_set('color', normals.flatten().copy())
        mesh.normals_split_custom_set_from_vertices(pos_to_blender(normals.T))

    
    for i in range(4):
        uv = attrs.pop(lotrc.pak.VertexUsage.TextureCoord(i), None)
        if uv is not None:
            uv_layer = mesh.uv_layers.new(name='UVMap' if i == 0 else f'UV{i}')
            uv_layer.uv.foreach_set('vector', np.array([uv[0],uv[1]], 'f').T[inds].flatten())
    
    psize = attrs.pop(lotrc.pak.VertexUsage.PSize(), None)
    if psize is not None:
        attribute = mesh.attributes.new(f'PSize', 'FLOAT_VECTOR', 'POINT')
        attribute.data.foreach_set('vector', [i for j in zip(psize[0], psize[1], psize[2]) for i in j])
        tree, tree_in, tree_out = GEOM_TREES['Billboard']
        mod = obj.modifiers.new('Billboard', 'NODES')
        mod.node_group = tree
    
    weights = attrs.pop(lotrc.pak.VertexUsage.BlendWeight(), None)
    indices = attrs.pop(lotrc.pak.VertexUsage.BlendIndices(), None)
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
            continue
        attr_name = attr.__class__.__name__.split('_')[-1]
        attribute = mesh.attributes.new(f'{attr_name}{i}', ty, 'POINT')
        attribute.data.foreach_set(dat_name, data)
    return obj

def create_mat(level, info, model_name, i=''):
    base = info[0].base
    mat = bpy.data.materials.new(f"MAT{i}.{model_name}")
    mat.use_nodes = True
    tree = mat.node_tree
    bsdf = tree.nodes.get("Principled BSDF")
    if isinstance(info[0], (lotrc.pak.Mat1, lotrc.pak.Mat3)):
        if (diffuse := level.textures.get(base.tex0)) is not None:
            node = tree.nodes.new(type="ShaderNodeTexImage")
            node.image = diffuse
            tree.links.new(node.outputs[0], bsdf.inputs[0])
            #tree.links.new(node.outputs['Alpha'], bsdf.inputs['Alpha'])
        if (rough := level.textures.get(base.tex1)) is not None:
            node = tree.nodes.new(type="ShaderNodeTexImage")
            node.image = rough
            tree.links.new(node.outputs[0], bsdf.inputs[2])
        if (specular := level.textures.get(base.tex3)) is not None:
            node = tree.nodes.new(type="ShaderNodeTexImage")
            node.image = specular
            tree.links.new(node.outputs[0], bsdf.inputs[13])
        if (normal := level.textures.get(base.tex2)) is not None:
            node = tree.nodes.new(type="ShaderNodeTexImage")
            node.image = normal
            tree.links.new(node.outputs[0], bsdf.inputs[5])
        if (normal2 := level.textures.get(base.tex4)) is not None:
            node = tree.nodes.new(type="ShaderNodeTexImage")
            node.image = normal2
            tree.links.new(node.outputs[0], bsdf.inputs[5])
    elif isinstance(info[0], lotrc.pak.Mat4):
        diffuse = None
        normal = None
        pos = tree.nodes.new(type="ShaderNodeNewGeometry")
        uv = tree.nodes.new(type="ShaderNodeMapping")
        uv.inputs['Scale'].default_value = (0.004, -0.004, 0.004)
        tree.links.new(pos.outputs['Position'], uv.inputs['Vector'])
        mixd1 = tree.nodes.new(type="ShaderNodeMix")
        mixd2 = tree.nodes.new(type="ShaderNodeMix")
        mixd3 = tree.nodes.new(type="ShaderNodeMix")
        mixn1 = tree.nodes.new(type="ShaderNodeMix")
        mixn2 = tree.nodes.new(type="ShaderNodeMix")
        mixn3 = tree.nodes.new(type="ShaderNodeMix")
        mixd1.blend_type = 'OVERLAY'
        mixd2.blend_type = 'OVERLAY'
        mixd3.blend_type = 'OVERLAY'
        mixn1.blend_type = 'OVERLAY'
        mixn2.blend_type = 'OVERLAY'
        mixn3.blend_type = 'OVERLAY'
        mixd1.data_type = 'RGBA'
        mixd2.data_type = 'RGBA'
        mixd3.data_type = 'RGBA'
        mixn1.data_type = 'RGBA'
        mixn2.data_type = 'RGBA'
        mixn3.data_type = 'RGBA'
        tree.links.new(mixd1.outputs['Result'], mixd2.inputs['A'])
        tree.links.new(mixd2.outputs['Result'], mixd3.inputs['A'])
        tree.links.new(mixd3.outputs['Result'], bsdf.inputs[0])
        tree.links.new(mixn1.outputs['Result'], mixn2.inputs['A'])
        tree.links.new(mixn2.outputs['Result'], mixn3.inputs['A'])
        tree.links.new(mixn3.outputs['Result'], bsdf.inputs[5])
        for mask, diffuse, normal, mixd, mixn in zip(
            [base.mask0, base.mask1, base.mask2],
            [base.tex0, base.tex2, base.tex4],
            [base.tex1, base.tex3, base.tex5],
            [mixd1, mixd2, mixd3],
            [mixn1, mixn2, mixn3]
        ):    
            if (mask := level.textures.get(mask)) is not None: 
                node = tree.nodes.new(type="ShaderNodeTexImage")
                node.image = mask
                tree.links.new(uv.outputs['Vector'], node.inputs['Vector'])
                tree.links.new(node.outputs['Alpha'], mixd.inputs['Factor'])
                tree.links.new(node.outputs['Alpha'], mixn.inputs['Factor'])
                if (diffuse := level.textures.get(diffuse)) is not None:
                    node = tree.nodes.new(type="ShaderNodeTexImage")
                    node.image = diffuse
                    tree.links.new(pos.outputs['Position'], node.inputs['Vector'])
                    tree.links.new(node.outputs['Color'], mixd.inputs['B'])
                if (normal := level.textures.get(normal)) is not None:
                    node = tree.nodes.new(type="ShaderNodeTexImage")
                    node.image = normal
                    tree.links.new(pos.outputs['Position'], node.inputs['Vector'])
                    tree.links.new(node.outputs['Color'], mixn.inputs['B'])
    return mat

def import_model(level, model, model_name, models_col, context):
    lotrc_props = context.scene.lotrc_props
    model_col = bpy.data.collections.new(model_name)
    models_col.children.link(model_col)
                
    mats = [create_mat(level, info, model_name, i) for i, info in enumerate(model.mats)]

    if lotrc_props.models_skeleton:
        obj_arma, bones = add_skeleton(model, model_name, model_col, context)
        skin_bones = [bones[i] for i in model.skin_order]
    else:
        obj_arma, skin_bones = None, None
    if lotrc_props.models_hk_skeleton:
        obj_arma_hk, bones_hk = add_hk_skeleton(model.hk_constraint, model_name, model_col, context)
 
    i = 0
    mesh_order = np.array([(i & 0x3FFFFFFF, i >> 30) for i in model.mesh_order], dtype='I').reshape(-1, 2)
    uses = np.zeros(len(mesh_order), 'I')
    info = model.info
    if lotrc_props.models_only_lod1:
        lods = [info.lod0]
    else:
        lods = [info.lod0, info.lod1, info.lod2, info.lod3]
    for lod in lods:
        uses[mesh_order[i:lod.start, 0]] |= UNKNOWN
        uses[mesh_order[lod.start:lod.static_end, 0]] |= STATIC
        uses[mesh_order[lod.static_end:lod.skinned_end, 0]] |= SKINNED
        uses[mesh_order[lod.skinned_end:lod.physics_end, 0]] |= PHYSICS
        uses[mesh_order[lod.physics_end:lod.breakable_end, 0]] |= BREAKABLE
        i = lod.breakable_end
    
    meshes = []
    col = bpy.data.collections.new(f"MESHES.{model_name}")
    model_col.children.link(col)
    vertex_data = model.vertex_data
    index_data = model.index_data
    for i, (info, usage, mat) in enumerate(zip(model.buffer_infos, uses, model.mat_order)):
        if usage != 0:
            mesh = add_mesh(
                info, vertex_data, index_data, usage, 
                f"MESH{i}.{model_name}", col, obj_arma, skin_bones
            )
            mesh.data.materials.append(mats[mat])
        else:
            mesh = None
        meshes.append(mesh)
    
    # collections to organize meshes
    info = model.info
    k = 0
    col_base = bpy.data.collections.new(f"BASE.{model_name}")
    model_col.children.link(col_base)
    for i, lod in enumerate(lods):
        col = bpy.data.collections.new(f"LOD{i}.{model_name}")
        model_col.children.link(col)
        for j, f in mesh_order[k:lod.start]:
            obj = copy_mesh(f"LOD{i}.MESH{j}.UNKNOWN{f}.{model_name}", meshes[j])
            col.objects.link(obj)
            #if i == 0 and f != 1 and meshes[j].data['variation_id'] == 0xFF: col_base.objects.link(obj)
            if i == 0 and f != 1: col_base.objects.link(obj)
        for j, f in mesh_order[lod.start:lod.static_end]:
            obj = copy_mesh(f"LOD{i}.MESH{j}.STATIC{f}.{model_name}", meshes[j])
            col.objects.link(obj)
            #if i == 0 and f != 1 and meshes[j].data['variation_id'] == 0xFF: col_base.objects.link(obj)
            if i == 0 and f != 1: col_base.objects.link(obj)
        for j, f in mesh_order[lod.static_end:lod.skinned_end]:
            obj = copy_mesh(f"LOD{i}.MESH{j}.SKINNED{f}.{model_name}", meshes[j])
            col.objects.link(obj)
            #if i == 0 and f != 1 and meshes[j].data['variation_id'] == 0xFF: col_base.objects.link(obj)
            if i == 0 and f != 1: col_base.objects.link(obj)
        for j, f in mesh_order[lod.skinned_end:lod.physics_end]:
            obj = copy_mesh(f"LOD{i}.MESH{j}.PHYSICS{f}.{model_name}", meshes[j])
            col.objects.link(obj)
            #if i == 0 and f != 1 and meshes[j].data['variation_id'] == 0xFF: col_base.objects.link(obj)
            if i == 0 and f != 1: col_base.objects.link(obj)
        for j, f in mesh_order[lod.physics_end:lod.breakable_end]:
            obj = copy_mesh(f"LOD{i}.MESH{j}.BREAKABLE{f}.{model_name}", meshes[j])
            col.objects.link(obj)
            #if i == 0 and f != 1 and meshes[j].data['variation_id'] == 0xFF: col_base.objects.link(obj)
            if i == 0 and f != 1: col_base.objects.link(obj)
        k = lod.breakable_end
    model_col['base'] = col_base

    if lotrc_props.models_collision and lotrc_props.models_skeleton:
        collision_col = bpy.data.collections.new(f'COLLISION.{model_name}')
        model_col.children.link(collision_col)
        for i, shape in enumerate(model.shapes):
            add_collision(shape, collision_col, model_name, i, obj_arma, bones)

        model_col['collision'] = collision_col
    return model_col

def import_models(level, model, context):
    models = level.level.models
    if model == 'All Models':
        pass
    elif model in models:
        models = {model: models[model]}
    else:
        models = {}

    models_col = bpy.data.collections.new("Models")
    level.col.children.link(models_col)

    models_col.hide_viewport = True
    for model_name, model in models.items():
        level.models[model_name.lower()] = import_model(level, model, model_name, models_col, context)
    models_col.hide_viewport = False
    level.models_col = models_col
    
def copy_mesh(name, mesh):
    obj = bpy.data.objects.new(name, mesh.data)
    if 'Billboard' in mesh.modifiers:
        tree, tree_in, tree_out = GEOM_TREES['Billboard']
        mod = obj.modifiers.new('Billboard', 'NODES')
        mod.node_group = tree
    return obj
