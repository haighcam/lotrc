import lotrc
import lotrc.level_alt
from lotrc.utils import *

lotrc.types.DECOMP_LUA = False

# replace CH_NML_SIL_Scout mesh in helm's deep with CH_hum_meshtongue_01 from isengaurd
src = 'CH_hum_meshtongue_01'
clss = 'CH_NML_SIL_Scout'

# load the levels, this parser is currently limited and will break some levels, use the non alt version for other stuff
dst_name = "Helm'sDeep"
levelDst = lotrc.level_alt.LevelData(f"Levels/{dst_name}")
levelSrc = lotrc.level_alt.LevelData("Levels/Isengard")

# make the change in the level file
vals = levelDst.sub_blocks1.blocks[-1].to_dict(levelDst.keys)
for i, obj in enumerate(vals['objs']):
    if obj['fields'].get('name') == clss or obj['fields'].get('Name') == clss: 
        print('found')
        break
if 'meshes' in vals['objs'][i]['fields']:
    old_mesh = lotrc.types.hash_(vals['objs'][i]['fields']['meshes'].pop())
    vals['objs'][i]['fields']['meshes'].append(src)
else:
    old_mesh = lotrc.types.hash_(vals['objs'][i]['fields']['mesh'])
    vals['objs'][i]['fields']['mesh'] = src
levelDst.sub_blocks1.blocks[-1] = lotrc.types.GameObjs.from_dict(vals, levelDst.f)

# grab the mesh and textures
mesh = levelSrc.meshes[hash_string(src)]

textures = set()
for mat in mesh.mats:
    textures.update(i for i in mat['textures'] if i != 0)
    
# set the level_flags so that the mesh actually shows up
flags = levelDst.meshes[old_mesh].info['level_flag']
new_textures = {}
for k in textures:
    if k in levelDst.textures:
        levelDst.textures[k][0]['level_flag'] |= flags
    else:
        levelSrc.textures[k][0]['level_flag'] = flags
        new_textures[k] = levelSrc.textures[k]
mesh.info['level_flag'] = flags

# add the stuff to helm's deep
levelDst.textures.update(new_textures)
levelDst.meshes[hash_string(src)] = mesh

# add the debug strings as well, not sure if it is needed
s = set(levelDst.pak_strings)
levelDst.pak_strings.extend(i for i in levelSrc.pak_strings if i not in s)
s = set(levelDst.bin_strings)
levelDst.bin_strings.extend(i for i in levelSrc.bin_strings if i not in s)
levelDst.keys.update(levelSrc.keys)
# dump and write the level
(infos, pak_data, bin_data) = levelDst.dump()
with open(f"Levels/{dst_name}.BIN", "wb") as f:
    f.write(bin_data)
with open(f"Levels/{dst_name}.PAK", "wb") as f:
    f.write(pak_data)