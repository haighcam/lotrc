import json
from pathlib import Path
import shutil
import os
import string
import struct
from lupa.lua51 import LuaRuntime

# this script needs lua-bytecode.lua to be present at the path set below
# it can be optained from https://github.com/lua-bytecode/lua-bytecode.github.io
LUA_BYTECODE_PATH = "./lua-bytecode.lua" # change me to wherever lua-bytecode.lua is

### Porting the Balrog from the Shire to Minas Tirith Top. 

# soundbanks required for the character
soundbnks = [
    "ChatterHeroBalrog.bnk",
    "SFXBalrog.bnk",
]

src_path = Path("DumpedLevels/Shire")
dst_path = Path("DumpedLevels/MinasTirith_Top")

# these are the guids for the corresponding objects in the dst level
spawn_emmiter_guid = 7052754
gamemode_guid = 7052744 # gamemode object, needed for adding relevant soundbanks
gamemodemask = 3 # grabbed from a different class in the spawn list, ensures the ported objects will be visible in the targeted gamemode

# from the src level
class_guid = 109012487


### some utility functions

# some utilities for getting things from dumped level file
def find_obj(vals, guid):
    for obj in vals['objs']:
        if obj['fields']['guid'] == guid:
            return obj

def find_type(vals, name):
    for ty in vals['types']:
        if ty['name'] == name:
            return ty
    
# grabs an object and all sub objects from a dumped level file
# parts can be uncommented to print some stuff about 
#    meshes, effects and scripts that are needed for the objects to work propoerly (or you can try to find everything in a dumped json file
def copy_tree(vals, guid, processed=None, gamemodemask=None, scripts=None, meshes=None, effects=None):
    if processed is None:
        processed = set()
    if scripts is None:
        scripts = set()
    if meshes is None:
        meshes = set()
    if effects is None:
        effects = set()
    elif guid in processed:
        return []
    processed.add(guid)
    obj = find_obj(vals, guid)
    ty = find_type(vals, obj['type'])
    objs = [obj]
    if (val:=obj['fields'].get('AnimationScript')) is not None and val != '':
        scripts.add(val)
    if (val:=obj['fields'].get('InputEventScript')) is not None and val != '':
        scripts.add(val)
    if (val:=obj['fields'].get('EffectLookupTable')) is not None and val != '':
        scripts.add(val)
    if (val:=obj['fields'].get('CameraScript')) is not None and val != '':
        scripts.add(val)
    if (val:=obj['fields'].get('BehaviorScriptList')) is not None:
        scripts.update(val)
    if (val:=obj['fields'].get('mesh')) is not None and val != '':
        meshes.add(val)
    if (val:=obj['fields'].get('PhysMesh')) is not None and val != '':
        meshes.add(val)
    if (val:=obj['fields'].get('meshes')) is not None:
        meshes.update(val)
    if gamemodemask is not None and 'GameModeMask' in obj['fields']:
        obj['fields']['GameModeMask'] |= gamemodemask
    for t in ty['fields']:
        if t['type'] == 'guid':
            val = obj['fields'][t['name']]
            if val != 0:
                objs.extend(copy_tree(vals, val, processed, gamemodemask, scripts, meshes, effects))
        elif t['type'] == 'objectlist':
            for val in obj['fields'][t['name']]:
                objs.extend(copy_tree(vals, val, processed, gamemodemask, scripts, meshes, effects))
        elif 'Effect' in t['name']:
            if t['type'] == 'crc' and (val:=obj['fields'][t['name']]) != '':
                effects.add(val)
            elif t['type'] == 'crclist':
                effects.update(obj['fields'][t['name']])
    return objs
    
def get_lua_strings(data):
    valid_chars = set(string.printable.encode())
    strings = []
    off = 0
    while off < len(data):
        if data[off-1] == 0 and data[off] in valid_chars:
            valid = True
            l = struct.unpack_from("I", data, off-4)[0] - 1
            # print(l, data[off:off+l])
            if l > len(data) or l <= 1:
                valid = False
            else:
                for i in range(l):
                    if data[off+i] not in valid_chars:
                        valid = False
                        break
            if valid:
                strings.append(data[off:off+l].decode())
                off += l
        off += 1
    return strings

def get_animation_table(name, script_data):
    def lua_import(script):
        if script in loaded_scripts: return 
        loaded_scripts.add(script)
        lua.execute(lua_conv(script_data[script], b"L4808"))

    lua = LuaRuntime()
    lua_ = LuaRuntime(encoding=None)

    lua_conv = lua_.eval("function(obj, f) return dofile(\"" + LUA_BYTECODE_PATH + "\")(obj, f) end")

    
    loaded_scripts = set()
        
    lua.globals()['import'] = lua_import
    lua.globals()['inherit'] = lua_import
    lua.globals()['imports'] = lua.table()
    lua.globals()['MgScript'] = lua.table_from({
        'Assert': lambda x, y: None,
        'GetRandomNumber': lambda: 1
    }) 
    lua.globals()['DeepCopy'] = lambda x: x
    # lua.globals()['AppendTableIndex'] = lambda x,y: None
    lua.globals()['AppendTableIndex'] = lua.eval("""
function (t1, t2)
    for key, val in pairs(t2) do
        t1[key] = val
    end
end
    """)
    # lua.globals()['AppendTable'] = lambda x,y: None
    lua.globals()['AppendTable'] = lua.eval("""
function (t1, t2)
    table.insert(t1, t2)
end
    """)
    lua.globals()['MgAnim'] = lua.table_from({'GetRootSpeed': lambda x: None}) 
    
    lua.execute(lua_conv(script_data[name], b"L4808"))
    
    if lua.globals()['AnimTableUsed'] is None:
        anim_table = dict(lua.globals()['AnimTable'])
    else:
        anim_table = {}
        for table_name in list(lua.globals()['AnimTableUsed'].values()):
            anim_table.update(dict(lua.globals()[table_name]))
    for k,v in anim_table.items():
        if isinstance(v, lua.table().__class__):
            anim_table[k] = list(v.values())
    return anim_table

# dump the level block to move relevant infomation
with open(dst_path.joinpath('sub_blocks1', 'level.json'), "rb") as f:
    vals_dest = json.load(f)
with open(src_path.joinpath('sub_blocks1', 'level.json'), "rb") as f:
    vals = json.load(f)

# get the needed objects and items associated with them
scripts = set()
meshes = set()
effects = set()
class_items = copy_tree(vals, class_guid, gamemodemask=gamemodemask, scripts=scripts, meshes=meshes, effects=effects)

# effects = effects.intersection(levelSrc.keys[i] for i in levelSrc.effects.keys())

script_strings = {}
script_data = {}
for path in os.listdir(src_path.joinpath("sub_blocks1")):
    if path.split('.')[-1] == 'lua':
        name = path.split('.')[0]
        with open(src_path.joinpath("sub_blocks1", path), "rb") as f:
            data = f.read()
        strings = set(get_lua_strings(data))
        script_strings[name] = strings
        script_data[name] = data

animations = set()
anim_tables = [get_animation_table(i, script_data) for i in scripts if i.startswith('ANM_')]
for anim_table in anim_tables:
    for anim in anim_table.values():
        if isinstance(anim, list):
            animations.update(anim)
        else:
            animations.add(anim)

old_scripts = set(i.split('.lua')[0] for i in os.listdir(dst_path.joinpath("sub_blocks1")) if i.endswith('.lua'))
scripts.difference_update(old_scripts)
new_scripts = set()
common_scripts = set()
while len(scripts) != 0:
    k = scripts.pop()
    new_scripts.add(k)
    strings = script_strings[k]
    children = strings.intersection(script_strings.keys()).difference(new_scripts).difference(scripts)
    common_scripts.update(children.intersection(old_scripts))
    for k in children.difference(old_scripts):
        scripts.add(k)

textures = set()
for k in meshes:
    if os.path.exists(dst_path.joinpath("meshes", k+'.json')):
        with open(dst_path.joinpath("meshes", k+'.json'), "rb") as f:
            mesh = json.load(f)
        mesh['info']['gamemodemask'] |= gamemodemask
        with open(dst_path.joinpath("meshes", k+'.json'), "w") as f:
            json.dump(mesh, f, indent=1)
    else:
        with open(src_path.joinpath("meshes", k+'.json'), "rb") as f:
            mesh = json.load(f)
        mesh['info']['gamemodemask'] = gamemodemask
        with open(dst_path.joinpath("meshes", k+'.json'), "w") as f:
            json.dump(mesh, f, indent=1)
    for mat in [j['base'] if 'base' in j else j for i in mesh['mats'] for j in i.values()]:
        textures.update([mat[f'tex_{i}'] for i in range(2,18) if mat[f'tex_{i}'] != ''])

for k in textures:
    if os.path.exists(dst_path.joinpath("textures", k+'.json')):
        with open(dst_path.joinpath("textures", k+'.json'), "rb") as f:
            tex = json.load(f)
        tex['gamemodemask'] |= gamemodemask
        with open(dst_path.joinpath("textures", k+'.json'), "w") as f:
            json.dump(tex, f, indent=1)
    else:
        with open(src_path.joinpath("textures", k+'.json'), "rb") as f:
            tex = json.load(f)
        tex['gamemodemask'] = gamemodemask
        with open(dst_path.joinpath("textures", k+'.json'), "w") as f:
            json.dump(tex, f, indent=1)
        shutil.copyfile(src_path.joinpath("textures", k+'.dds'), dst_path.joinpath("textures", k+'.dds'))

for k in animations:
    if os.path.exists(dst_path.joinpath("animations", k+'.json')):
        with open(dst_path.joinpath("animations", k+'.json'), "rb") as f:
            anim = json.load(f)
        anim['info']['gamemodemask'] |= gamemodemask
        with open(dst_path.joinpath("animations", k+'.json'), "w") as f:
            json.dump(anim, f, indent=1)
    else:
        with open(src_path.joinpath("animations", k+'.json'), "rb") as f:
            anim = json.load(f)
        anim['info']['gamemodemask'] = gamemodemask
        with open(dst_path.joinpath("animations", k+'.json'), "w") as f:
            json.dump(anim, f, indent=1)

for k in effects:
    if os.path.exists(dst_path.joinpath("effects", k+'.json')):
        with open(dst_path.joinpath("effects", k+'.json'), "rb") as f:
            effect = json.load(f)
        effect['gamemodemask'] |= gamemodemask
        with open(dst_path.joinpath("effects", k+'.json'), "w") as f:
            json.dump(effect, f, indent=1)
    elif os.path.exists(src_path.joinpath("effects", k+'.json')):
        with open(src_path.joinpath("effects", k+'.json'), "rb") as f:
            effect = json.load(f)
        effect['gamemodemask'] = gamemodemask
        with open(dst_path.joinpath("effects", k+'.json'), "w") as f:
            json.dump(effect, f, indent=1)

with open(dst_path.joinpath("sub_blocks1", "index.json"), "rb") as f:
    index = json.load(f)
for script in new_scripts:
    index['block_headers'].insert(-3, {'key': script+'.lua', 'offset': 0, 'size': 0})
    shutil.copy(src_path.joinpath("sub_blocks1", script+'.lua'), dst_path.joinpath("sub_blocks1", script+'.lua'))
with open(dst_path.joinpath("sub_blocks1", "index.json"), "w") as f:
    json.dump(index, f, indent=1)

# add the soundbanks for the balrog to the team death match gamemode. You'll need to check the source level.json to which banks (if any) are needed
o = find_obj(vals_dest, gamemode_guid)
o['fields']['ModeSpecificBanks'].extend(i for i in soundbnks if i not in o['fields']['ModeSpecificBanks'])

# add the balrog to the evil team
o = find_obj(vals_dest, spawn_emmiter_guid)
o['fields']['classes'].append(class_guid)

# add the objects for the target class, only if they are missing
old_class_objs = []
new_class_objs = []
for i in class_items:
    val = find_obj(vals_dest, i['fields']['guid'])
    if val is not None:
        old_class_objs.append(val)
    else:
        new_class_objs.append(i)

vals_dest['objs'].extend(new_class_objs)
valid_types = set(i['name'] for i in vals_dest['types'])
needed_types = [find_type(vals, t) for t in set(o['type'] for o in class_items).difference(valid_types)]
vals_dest['types'].extend(needed_types)

# update the gamemode mask for needed existing objects
for i in old_class_objs:
    if 'GameModeMask' in i['fields']:
        i['fields']['GameModeMask'] |= gamemodemask

with open(dst_path.joinpath('sub_blocks1', 'level.json'), "w") as f:
    json.dump(vals_dest, f, indent=1)

with open(src_path.joinpath('bin_strings.json'), "rb") as f:
    bin_strings_src = json.load(f)
with open(src_path.joinpath('pak_strings.json'), "rb") as f:
    pak_strings_src = json.load(f)
with open(dst_path.joinpath('bin_strings.json'), "rb") as f:
    bin_strings_dst = json.load(f)
with open(dst_path.joinpath('pak_strings.json'), "rb") as f:
    pak_strings_dst = json.load(f)
pak_strings = set(pak_strings_dst)
bin_strings = set(bin_strings_dst)
pak_strings_dst.extend([i for i in pak_strings_src if i not in pak_strings])
bin_strings_dst.extend([i for i in bin_strings_src if i not in bin_strings])

with open(dst_path.joinpath('bin_strings.json'), "w") as f:
    json.dump(bin_strings_dst, f, indent=1)
with open(dst_path.joinpath('pak_strings.json'), "w") as f:
    json.dump(pak_strings_dst, f, indent=1)