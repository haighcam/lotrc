import lotrc
import lotrc.level_alt
from lotrc.utils import *
from lotrc.types import *
lotrc.types.DECOMP_LUA = False

### Porting the Balrog from the Shire to Minas Tirith Top. 

# soundbanks required for the character
soundbnks = [
    "ChatterHeroBalrog.bnk",
    "SFXBalrog.bnk",
]

# parse the level files
dst_name = "MinasTirith_Top"
levelDst = lotrc.level_alt.LevelData(f"Levels/{dst_name}")
levelSrc = lotrc.level_alt.LevelData("Levels/Shire")

# dump the level block to move relevant infomation
vals = levelSrc.sub_blocks1.blocks[-1].to_dict(levelSrc.keys)
vals_dest = levelDst.sub_blocks1.blocks[-1].to_dict(levelDst.keys)

# these are the guids for the corresponding objects in the dst level
spawn_emmiter_guid = 7052754
gamemode_guid = 7052744 # gamemode object, needed for adding relevant soundbanks
gamemodemask = 3 # grabbed from a different class in the spawn list, ensures the ported objects will be visible in the targeted gamemode

# from the src level
class_guid = 109012487

# get the needed objects and items associated with them
scripts = set()
meshes = set()
effects = set()
class_items = copy_tree(vals, class_guid, gamemodemask=gamemodemask, scripts=scripts, meshes=meshes, effects=effects)
effects = effects.intersection(levelSrc.keys[i] for i in levelSrc.effects.keys())

script_strings = {}
script_data = {}
for block in levelSrc.sub_blocks1.blocks:
    if isinstance(block, Lua):
        name = block.name.split('.lua')[0]
        strings = set(get_lua_strings(block.data))
        script_strings[name] = strings
        script_data[hash_string(name)] = block.data

animations = set()
anim_tables = [get_animation_table(i, script_data) for i in scripts if i.startswith('ANM_')]
for anim_table in anim_tables:
    for anim in anim_table.values():
        if isinstance(anim, list):
            animations.update(anim)
        else:
            animations.add(anim)

old_scripts = set(levelDst.keys[i].split('.lua')[0] for i in levelDst.sub_blocks1.block_headers['key'])
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

# check that the scripts that are imported by the new ones are the same as expected
for v in common_scripts:
    k = hash_string(v + '.lua')
    i = np.nonzero(levelDst.sub_blocks1.block_headers['key'] == k)[0].item()
    j = np.nonzero(levelSrc.sub_blocks1.block_headers['key'] == k)[0].item()
    if levelDst.sub_blocks1.blocks[i].data != levelSrc.sub_blocks1.blocks[j].data:
        print(f"WARNING: script {v} is used by a new script and is different between levels")

# grab the meshes and textures needed for the balrog
meshes = {(k := lotrc.types.hash_(i)): levelSrc.meshes[k] for i in meshes}
textures = {}
for mat in [j for i in meshes.values() for j in i.mats]:
    textures.update({i: levelSrc.textures[i] for i in mat['textures'] if i != 0})

# grab the effects needed for the balrog
effects = {(k := lotrc.types.hash_(i)): levelSrc.effects[k] for i in effects}

# grab the animations needed for the balrog (from AT_GNT_Balrog)

animations = {(k := lotrc.types.hash_(i)): levelSrc.animations[k] for i in animations}


# update the gamemode mask for all objects
new_animations = {}
for k, anim in animations.items():
    if k in levelDst.animations:
        levelDst.animations[k].info['level_flag'] |= gamemodemask
    else:
        anim.info['level_flag'] = gamemodemask
        new_animations[k] = anim
new_meshes = {}
for k, mesh in meshes.items():
    if k in levelDst.meshes:
        levelDst.meshes[k].info['level_flag'] |= gamemodemask
    else:
        mesh.info['level_flag'] = gamemodemask
        new_meshes[k] = mesh

new_textures = {}
for k, texture in textures.items():
    if k in levelDst.textures:
        levelDst.textures[k][0]['level_flag'] |= gamemodemask
    else:
        texture[0]['level_flag'] = gamemodemask
        new_textures[k] = texture

new_effects = {}
for k, effect in effects.items():
    if k in levelDst.effects:
        levelDst.effects[k].level_flag |= gamemodemask
    else:
        effect.level_flag = gamemodemask
        new_effects[k] = effect

# add all of the meshes, textures, effects and animations if they are not already there
# (if they are already there you may need to update the gamemodemask, which this does not do)
levelDst.textures.update({k:v for k,v in textures.items() if k not in levelDst.textures})
levelDst.meshes.update({k:v for k,v in meshes.items() if k not in levelDst.meshes})
levelDst.effects.update({k:v for k,v in effects.items() if k not in levelDst.effects})
levelDst.animations.update({k:v for k,v in animations.items() if k not in levelDst.animations})

# add the sripts befoer the uv and level files since those always seem to be at the back. There is probably a better order for the files, but this works
inds = sorted([np.nonzero(levelSrc.sub_blocks1.block_headers['key'] == lotrc.types.hash_(i + '.lua'))[0].item() for i in new_scripts])
for i in inds:
    levelDst.sub_blocks1.blocks.insert(-3, levelSrc.sub_blocks1.blocks[i])
levelDst.sub_blocks1.block_headers = np.insert(levelDst.sub_blocks1.block_headers, -3, levelSrc.sub_blocks1.block_headers[inds])

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

# load the modified level block
levelDst.sub_blocks1.blocks[-1] = lotrc.types.GameObjs.from_dict(vals_dest)

# copy over the debug strings
s = set(levelDst.pak_strings)
levelDst.pak_strings.extend(i for i in levelSrc.pak_strings if i not in s)
s = set(levelDst.bin_strings)
levelDst.bin_strings.extend(i for i in levelSrc.bin_strings if i not in s)
levelDst.keys.update(levelSrc.keys)

# dump the modified level
(_, pak_data, bin_data) = levelDst.dump(compress=True)
with open(f"Levels/{dst_name}.BIN", "wb") as f:
    f.write(bin_data)
with open(f"Levels/{dst_name}.PAK", "wb") as f:
    f.write(pak_data)

