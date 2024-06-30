import json
import shutil
import os
import string
import struct
import zipfile
from operator import attrgetter
from pathlib import Path

# try to add required objects to a gamemode by setting the corresponding gamemodemask

src_path = "DumpedLevels/MinasTirith_Top.zip"
gamemodeguid = 144015924

class ZipFile(zipfile.ZipFile):
    """
        Modified zipfile to allow for removing files. 
        Uses slightly modified code from https://github.com/python/cpython/blob/659eb048cc9cac73c46349eb29845bc5cd630f09/Lib/zipfile.py
    """
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
    def remove(self, *members):
        """Remove a file from the archive. The archive must be open with mode 'a'"""

        if self.mode != 'a':
            raise RuntimeError("remove() requires mode 'a'")
        if not self.fp:
            raise ValueError(
                "Attempt to write to ZIP archive that was already closed")
        if self._writing:
            raise ValueError(
                "Can't write to ZIP archive while an open writing handle exists."
            )

        zinfos = []
        for member in members:
            # Make sure we have an info object
            if isinstance(member, zipfile.ZipInfo):
                # 'member' is already an info object
                zinfo = member
            else:
                # get the info object
                zinfo = self.getinfo(member)
            zinfos.append(zinfo)

        return self._remove_member(*zinfos)

    def _remove_member(self, *members):
        # get a sorted filelist by header offset, in case the dir order
        # doesn't match the actual entry order
        fp = self.fp
        entry_offset = 0
        filelist = sorted(self.filelist, key=attrgetter('header_offset'))
        min_header_offset = min(i.header_offset for i in members)
        members = set(members)
        for i in range(len(filelist)):
            info = filelist[i]
            # find the target member
            if info.header_offset < min_header_offset:
                continue

            # get the total size of the entry
            entry_size = None
            if i == len(filelist) - 1:
                entry_size = self.start_dir - info.header_offset
            else:
                entry_size = filelist[i + 1].header_offset - info.header_offset

            # found the member, set the entry offset
            if info in members:
                entry_offset += entry_size
                continue

            # Move entry
            # read the actual entry data
            fp.seek(info.header_offset)
            entry_data = fp.read(entry_size)

            # update the header
            info.header_offset -= entry_offset

            # write the entry to the new position
            fp.seek(info.header_offset)
            fp.write(entry_data)
            fp.flush()

        # update state
        self.start_dir -= entry_offset
        for member in members:
            self.filelist.remove(member)
            del self.NameToInfo[member.filename]
        self._didModify = True

        # seek to the start of the central dir
        fp.seek(self.start_dir)

# some utilities for getting things from dumped level file
def find_obj(vals, guid):
    for obj in vals['objs']:
        if obj['fields']['guid'] == guid:
            return obj

def get_layer(vals, guid):
    objs = []
    for obj in vals['objs']:
        if obj['layer'] == guid:
            objs.append(obj)
    return objs

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

def scan(vals, guid, gamemodemask=None, scripts=None, meshes=None, effects=None):
    if scripts is None:
        scripts = set()
    if meshes is None:
        meshes = set()
    if effects is None:
        effects = set()
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
        if 'Effect' in t['name']:
            if t['type'] == 'crc' and (val:=obj['fields'][t['name']]) != '':
                effects.add(val)
            elif t['type'] == 'crclist':
                effects.update(obj['fields'][t['name']])
                
to_remove = set()
to_add = {}

with ZipFile(src_path, "a", compression=zipfile.ZIP_DEFLATED) as src:
    files = {i.filename.casefold(): i.filename for i in src.filelist}
    
    with src.open('animation_block_infos.json', "r") as f:
        anim_infos = json.load(f)

    gamemodemask = -1
    for i, val in enumerate(anim_infos):
        if val.get('guid', val['unk_1']) == gamemodeguid:
            gamemodemask = 1 << i
            print(f'found gamemode at index {i}, {gamemodemask}')
            break
            

    with src.open('sub_blocks1/level.json', "r") as f:
        vals = json.load(f)

    # update the gamemodemask of all objects used in the gamemode
    print("updating GameModeMask in level data")
    gmd = find_obj(vals, gamemodeguid)
    # gmd['fields']['GameModeMask'] = -1
    objs = copy_tree(vals, gamemodeguid)
    guids = set(i['fields']['guid'] for i in objs)
    for layer in gmd['fields']['layers']:
        for i in get_layer(vals, layer):
            if i['fields']['guid'] not in guids:
                for j in copy_tree(vals, i['fields']['guid']):
                    if j['fields']['guid'] not in guids:
                        guids.add(j['fields']['guid'])
                        objs.append(j)
    for obj in objs:
        obj['fields']['GameModeMask'] |= gamemodemask

    to_remove.add('sub_blocks1/level.json')
    to_add['sub_blocks1/level.json'] = json.dumps(vals, indent=1)

    # uncomment this to just add everything to the gamemeode
    for f_name in files.values():
        if f_name.startswith('animations') or f_name.startswith('effects') or f_name.startswith('meshes') or f_name.startswith('textures'):
            if not f_name.endswith('json'):
                continue
            obj = src.read(f_name)
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask | int(obj[a:b])).encode() + obj[b:]
            to_remove.add(f_name)
            to_add[f_name] = obj

    # get all used scripts, meshes, effects and animations 
    # if the gamemode is brand new then this is the same objects as above, however this will also
    # does not seem to get all animations / meshes so the using the above 'add everything approach'
    # print("finding objects used in gamemode")
    # scripts = set()
    # meshes = set()
    # effects = set()
    # for i in vals['objs']:
    #     if 'GameModeMask' in i['fields'] and (i['fields']['GameModeMask'] & gamemodemask) == 0: continue
    #     scan(vals, i['fields']['guid'], scripts=scripts, meshes=meshes, effects=effects)

    # animations = set()
    # for i in scripts:
    #     if not i.startswith("ANM_"): continue
    #     with src.open(f'animation_tables/{i}.json', "r") as f:
    #         anim_table = json.load(f)
    #     for anim in anim_table.values():
    #         if isinstance(anim, list):
    #             animations.update(anim)
    #         else:
    #             animations.add(anim)

    # print("updating meshes")
    # textures = set()
    # for k in meshes:
    #     f_name = f"meshes/{k}.json"
    #     if (f_name := files.get(f_name.casefold())) is None: continue
    #     with src.open(f_name, "r") as f:
    #         mesh = json.load(f)
    #     mesh['info']['gamemodemask'] |= gamemodemask
    #     to_remove.add(f_name)
    #     to_add[f_name] = json.dumps(mesh, indent=1)
    
    # print("updating textures")
    # for k in textures:
    #     f_name = f"textures/{k}.json"
    #     if (f_name := files.get(f_name.casefold())) is None: continue
    #     tex = src.read(f_name)
    #     a = tex.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
    #     b = tex.find(b',', a)
    #     tex = tex[:a] + str(gamemodemask | int(tex[a:b])).encode() + tex[b:]
    #     to_remove.add(f_name)
    #     to_add[f_name] = tex
    
    # print("updating animations")
    # for k in animations:
    #     f_name = f"animations/{k}.json"
    #     if (f_name := files.get(f_name.casefold())) is None: continue
    #     anim = src.read(f_name)
    #     a = anim.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
    #     b = anim.find(b',', a)
    #     anim = anim[:a] + str(gamemodemask | int(anim[a:b])).encode() + anim[b:]
    #     to_remove.add(f_name)
    #     to_add[f_name] = anim

    # print("updating effects")
    # for k in effects:
    #     f_name = f"effects/{k}.json"
    #     if (f_name := files.get(f_name.casefold())) is None: continue
    #     effect = src.read(f_name)
    #     a = effect.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
    #     b = effect.find(b',', a)
    #     effect = effect[:a] + str(gamemodemask | int(effect[a:b])).encode() + effect[b:]
    #     to_remove.add(f_name)
    #     to_add[f_name] = effect

    print("applying changes")
    src.remove(*to_remove)
    for f_name, data in to_add.items():
        src.writestr(f_name, data)