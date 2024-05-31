import lotrc
import lotrc.level_alt
from lotrc.utils import *

lotrc.types.DECOMP_LUA = False

# replace CH_elf_ancn_bow_all_01 in helm's deep with CH_hum_Wormtongue_01 from isengaurd

# load the levels, this parser is currently limited and will break some levels, use the non alt version for other stuff
levelHelm = lotrc.level_alt.LevelData("Levels/Helm'sDeep")
levelIsen = lotrc.level_alt.LevelData("Levels/Isengard")

# make the change in the level file
vals = levelHelm.sub_blocks1.blocks[-1].to_dict(levelHelm.keys)
for i, obj in enumerate(vals['objs']):
    if obj['fields'].get('name') == 'CH_NML_SIL_Scout': 
        print('found')
        break
vals['objs'][i]['fields']['mesh'] = 'CH_hum_Wormtongue_01'
levelHelm.sub_blocks1.blocks[-1] = lotrc.types.GameObjs.from_dict(vals, levelHelm.f)

# grab the mesh and textures
worm = levelIsen.meshes[hash_string('CH_hum_Wormtongue_01')]

textures = []
for mat in worm.mats:
    textures.extend(i for i in mat['textures'] if i != 0)
    
# set the level_flags so that the mesh actually shows up
# setting it properly wasn't working but 0xFF should make it visible for all instant action modes
for k in textures:
    levelIsen.textures[k][0]['level_flag'] = 0xFF
worm.info['level_flag'] = 0xFF

# add the stuff to helm's deep
levelHelm.textures.update({k: levelIsen.textures[k] for k in np.unique(textures)})
levelHelm.meshes[hash_string('CH_hum_Wormtongue_01')] = worm

# add the debug strings as well, not sure if it is needed
s = set(levelHelm.pak_strings)
levelHelm.pak_strings.extend(i for i in levelIsen.pak_strings if i not in s)
s = set(levelHelm.bin_strings)
levelHelm.bin_strings.extend(i for i in levelIsen.bin_strings if i not in s)

# dump and write the level
(infos, pak_data, bin_data) = levelHelm.dump()

with open("Levels/Helm'sDeep.BIN", "wb") as f:
    f.write(bin_data)
with open("Levels/Helm'sDeep.PAK", "wb") as f:
    f.write(pak_data)