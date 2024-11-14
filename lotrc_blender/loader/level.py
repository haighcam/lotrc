import bpy
import pathlib
import numpy as np
from mathutils import Matrix, Vector, Quaternion
from .conv import *
from .loader import LOADED_LEVELS

class LoadLevelBlock(bpy.types.Operator):
    """Load Level Block from a Lord of the Rings Conquest Level"""
    bl_idname = "lotrc.load_level_block"
    bl_label = "Load LOTRC Level Block"

    def execute(self, context):
        parse_level_block(LOADED_LEVELS[context.scene.name])
        return {'FINISHED'}

CLASSES = [LoadLevelBlock]

def parse_level_block(level):
    col = bpy.data.collections.new("level")
    level.col.children.link(col)
    col.hide_viewport = True
    gameobjs = level.level.sub_blocks1.blocks[-1][0]
    level.types.update({key: {i.key: i.kind for i in fields} for key, fields in gameobjs.types.items()})

    objs = {}
    for obj in gameobjs.objs:
        ty = level.types[obj.key]
        fields = {key: val[0] for key, val in zip(ty, obj.fields)}
        fields['Layer'] = obj.layer
        fields['__type__'] = obj.key
        objs[fields['GUID']] = fields
    
    blender_objs = {}
    parents = {}
    for fields in objs.values():
        guid = fields['GUID']
        name = fields['Name']
        ty = fields['__type__']
        obj = bpy.data.objects.new(f"{name}.{ty}.{guid}", None)
        col.objects.link(obj)
        obj.empty_display_type = 'SPHERE'
        if ty in ['Road', 'Collision', 'CPSpline']:
            if ty == 'CPSpline': 
                model_name = f"Road_{guid}"
            else:
                model_name = f'{ty}_{guid}'
            model = ''
            for name_ in level.models.keys():
                if name_.endswith(model_name):
                    model = name_
                    break
        elif ty == 'speed_tree':
            model = fields.get('Tree', '') + '#spt'
        else:
            model = fields.get('Mesh', '')
        if model != '' and model in level.models:
            obj.instance_collection = level.models[model]['base']
            obj.instance_type = 'COLLECTION'
            obj.empty_display_size = 0
            col_obj = bpy.data.objects.new(f"{name}.{ty}.{guid}.COLLISION", None)
            col_obj.empty_display_type = 'SPHERE'
            col_obj.empty_display_size = 0
            col_obj.instance_collection = level.models[model]['collision']
            col_obj.instance_type = 'COLLECTION'
            col_obj.parent = obj
            col.objects.link(col_obj)
        else:
            obj.empty_display_size = 0.25
        for k,v in fields.items():
            if k in ['WorldTransform']: continue
            obj[k] = v
        if (m := fields.get('WorldTransform')) is not None:
            obj.matrix_world = mat_to_blender(m)
        if ty == "Road":
            for i, (model, m) in enumerate(zip(fields['RoadMeshes'], fields['RoadMatrices'])):
                if model not in level.models: continue
                road_obj = bpy.data.objects.new(f"{name}.{ty}{i}.{model}", None)
                col.objects.link(road_obj)
                road_obj.instance_collection = level.models[model]['base']
                road_obj.instance_type = 'COLLECTION'
                road_obj.empty_display_size = 0
                road_obj.empty_display_type = 'SPHERE'
                road_obj.parent = obj
                road_obj.matrix_world = mat_to_blender(m)
        elif ty == "templateLevel":
            for model in level.models:
                if model.startswith("Terrain_"):
                    ter_obj = bpy.data.objects.new(f"{name}.{model}", None)
                    col.objects.link(ter_obj)
                    ter_obj.instance_collection = level.models[model]['base']
                    ter_obj.instance_type = 'COLLECTION'
                    ter_obj.empty_display_size = 0
                    ter_obj.empty_display_type = 'SPHERE'
                    ter_obj.parent = obj
        elif ty == "spawn_point":
            for child in fields["Nodes"]:
                parents[child] = guid
        elif ty == "ResourceEmitter":
            for child in fields["SpawnRegions"]:
                parents[child] = guid
        if (children := fields.get("InitialChildObjects")) is not None:
            for child in children:
                parents[child] = guid
        blender_objs[guid] = obj
    
    for guid, obj in blender_objs.items():
        if (parent := obj['ParentGUID']) != 0:
            obj.parent = blender_objs[parent]
            obj.matrix_local = mat_to_blender(obj['Transform'])
        elif (parent := parents.get(guid)) is not None:
            obj.parent = blender_objs[parent]
            obj.matrix_local = mat_to_blender(obj['Transform'])
    
    col.hide_viewport = False
