import bpy
from . import ui
from .loader import loader, level, model, textures

# things to add / fix
#   - bow / banner mesh (second vbuff thing)
#   -

CLASSES = ui.CLASSES + loader.CLASSES + level.CLASSES + model.CLASSES + textures.CLASSES

class LotrcProps(bpy.types.PropertyGroup):
    filepath: bpy.props.StringProperty(subtype="FILE_PATH")
    selected_model: bpy.props.StringProperty(default="All Models")
    models_collision: bpy.props.BoolProperty(default=True)
    models_only_lod1: bpy.props.BoolProperty(default=False)
    models_skeleton: bpy.props.BoolProperty(default=True)
    models_hk_skeleton: bpy.props.BoolProperty(default=True)
    selected_texture: bpy.props.StringProperty(default="All Textures")

CLASSES.append(LotrcProps)

def register():
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.Scene.lotrc_props = bpy.props.PointerProperty(type=LotrcProps)

def unregister():
    for cls in CLASSES:
        bpy.utils.unregister_class(cls)
    #del bpy.types.Scene.lotrc_props
