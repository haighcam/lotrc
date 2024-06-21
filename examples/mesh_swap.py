import json
from pathlib import Path
import shutil
import os

# first dump the needed levels
# run this script pointing to the needed levels with the correct src mesh and target class name
# then compile the dumped dst level back into a .PAK and .BIN file

src = 'CH_hum_Wormtongue_01'
clss = 'CH_NML_SIL_Scout'

src_path = Path("DumpedLevels/Isengard")
dst_path = Path("DumpedLevels/Helm'sDeep")

with open(dst_path.joinpath('sub_blocks1', 'level.json'), "rb") as f:
    vals = json.load(f)

for i, obj in enumerate(vals['objs']):
    if obj['fields'].get('name') == clss or obj['fields'].get('Name') == clss: 
        obj['fields'].get('name')
        print('found')
        break

if 'meshes' in vals['objs'][i]['fields']:
    old_mesh = vals['objs'][i]['fields']['meshes'].pop()
    vals['objs'][i]['fields']['meshes'].append(src)
else:
    old_mesh = vals['objs'][i]['fields']['mesh']
    vals['objs'][i]['fields']['mesh'] = src

with open(dst_path.joinpath('sub_blocks1', 'level.json'), "w") as f:
    json.dump(vals, f, indent=1)

with open(dst_path.joinpath("meshes", old_mesh+'.json'), "rb") as f:
    old_mesh = json.load(f)

with open(src_path.joinpath("meshes", src+'.json'), "rb") as f:
    mesh = json.load(f)

textures = set()
for mat in [j for i in mesh['mats'] for j in i.values()]:
    textures.update([mat[f'tex_{i}'] for i in range(2,18) if mat[f'tex_{i}'] != ''])

gamemodemask = old_mesh['info']['gamemodemask']
mesh['info']['gamemodemask'] = gamemodemask
with open(dst_path.joinpath("meshes", src+'.json'), "w") as f:
    json.dump(mesh, f, indent=1)
    
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
    json.dump(pak_strings_dst, f, indent=1)s