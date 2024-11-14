from bpy_extras.io_utils import ImportHelper
import bpy
from .loader.loader import LOADED_LEVELS, model_enum, texture_enum

class PanelBase(bpy.types.Panel):
    bl_idname = "lotrc.panel_base"
    bl_label = "Base Panel"
    bl_space_type = "VIEW_3D"
    bl_region_type = "UI"
    bl_category = "LOTRC"

class ModelSelectOperator(bpy.types.Operator):
    bl_idname = "lotrc.model_select"
    bl_label = "Model Select Operator"
    bl_property = "model"

    model: bpy.props.EnumProperty(items=model_enum)

    def execute(self, context):
        context.scene.lotrc_props.selected_model = self.model
        return {'FINISHED'}

    def invoke(self, context, event):
        context.window_manager.invoke_search_popup(self)
        return {'FINISHED'}

class TextureSelectOperator(bpy.types.Operator):
    bl_idname = "lotrc.texture_select"
    bl_label = "Texture Select Operator"
    bl_property = "texture"

    texture: bpy.props.EnumProperty(items=texture_enum)

    def execute(self, context):
        context.scene.lotrc_props.selected_texture = self.texture
        return {'FINISHED'}

    def invoke(self, context, event):
        context.window_manager.invoke_search_popup(self)
        return {'FINISHED'}

class LevelPanel(PanelBase):
    bl_idname = "lotrc.level_panel"
    bl_label = "Load Level"

    def draw(self, context):
        props = context.scene.lotrc_props
        #self.layout.operator('lotrc.select_level', text='Select Level')
        self.layout.prop(props, 'filepath')
        row = self.layout.row()
        row.enabled = props.filepath != ''
        row.operator('lotrc.load_level', text='Load Level')
        if context.scene.name in LOADED_LEVELS:
            (header, box) = self.layout.panel('load_textures')
            header.label(text='Textures')
            if box is not None:
                box.operator('lotrc.texture_select', text=f'Select Texture')
                box.operator('lotrc.load_textures', text=f'Load: {props.selected_texture}')
            (header, box) = self.layout.panel('load_models')
            header.label(text='Models')
            if box is not None:
                box.operator('lotrc.model_select', text=f'Select Model')
                box.prop(props, 'models_only_lod1', text='Load Only LOD1')
                box.prop(props, 'models_skeleton', text='Load Skeleton')
                row = box.row()
                row.enabled = props.models_skeleton
                row.prop(props, 'models_collision', text='Load Collision')
                box.prop(props, 'models_hk_skeleton', text='Load HkSkeleton')
                box.operator('lotrc.load_models', text=f'Load: {props.selected_model}')
            (header, box) = self.layout.panel('load_level_block')
            header.label(text='Level Block')
            if box is not None:
                box.operator('lotrc.load_level_block', text='Load')

class TestPanel(PanelBase):
    bl_idname = "lotrc.test_panel"
    bl_label = "Test Panel"
    
    @classmethod
    def poll(cls, context):
        return (context.selected_objects != [])

    def draw(self, context):
        for obj in context.selected_objects:
            self.layout.label(text=obj.name)

class ImportLevel(bpy.types.Operator, ImportHelper):
    """Select a Lord of the Rings Conquest Level"""
    bl_idname = "lotrc.select_level"
    bl_label = "Select LOTRC Level"

    filepath: bpy.props.StringProperty(subtype="FILE_PATH")
    directory: bpy.props.StringProperty(subtype="DIR_PATH")
    
    def execute(self, context):
        context.scene.lotrc_props.filepath = self.filepath
        return {'FINISHED'}

CLASSES = [
    LevelPanel, 
    #TestPanel, 
    ImportLevel,
    ModelSelectOperator,
    TextureSelectOperator,
]
