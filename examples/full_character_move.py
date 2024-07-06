import json
import string
import struct
import zipfile
from operator import attrgetter
from pathlib import Path

### Porting the Balrog from the Shire to Minas Tirith Top. 
# first dump the needed levels
# run this script pointing to the needed levels with the correct src mesh and target class name
# then compile the dumped dst level back into a .PAK and .BIN file

# soundbanks required for the character
soundbnks = [
    "ChatterHeroBalrog.bnk",
    "SFXBalrog.bnk",
]

src_path = "DumpedLevels/Shire.zip"
dst_path = "DumpedLevels/MinasTirith_Top.zip"

# these are the guids for the corresponding objects in the dst level
spawn_emmiter_guid = 7052754
gamemode_guid = 7052744 # gamemode object, needed for adding relevant soundbanks
gamemodemask = 3 # grabbed from a different class in the spawn list, ensures the ported objects will be visible in the targeted gamemode

# from the src level
class_guid = 109012487

### some utility functions

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

def find_type(vals, name):
    for ty in vals['types']:
        if ty['name'] == name:
            return ty
    
# grabs an object and all sub objects from a dumped level file
# parts can be uncommented to print some stuff about 
#    meshes, effects and scripts that are needed for the objects to work propoerly (or you can try to find everything in a dumped json file
def copy_tree(vals, guid, gamemodemask=None, processed=None, infos=None):
    if processed is None:
        processed = set()
    if infos is None:
        infos = set()
    elif guid in processed:
        return []
    processed.add(guid)
    obj = find_obj(vals, guid)
    ty = find_type(vals, obj['type'])
    objs = [obj]
    if gamemodemask is not None and 'GameModeMask' in obj['fields']:
        obj['fields']['GameModeMask'] |= gamemodemask
    for t in ty['fields']:
        if t['type'] == 'guid':
            val = obj['fields'][t['name']]
            if val != 0:
                objs.extend(copy_tree(vals, val, processed=processed, gamemodemask=gamemodemask, infos=infos))
        elif t['type'] == 'objectlist':
            for val in obj['fields'][t['name']]:
                objs.extend(copy_tree(vals, val, processed=processed, gamemodemask=gamemodemask, infos=infos))
        elif (t['type'] == 'crc' or t['type'] == 'string') and (val:=obj['fields'][t['name']]) != '':
            infos.add(val.casefold())
        elif t['type'] == 'crclist' or t['type'] == 'stringlist':
            infos.update([i.casefold() for i in obj['fields'][t['name']]])
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
                strings.append(data[off:off+l].decode().casefold())
                off += l
        off += 1
    return strings

to_remove = set()
to_add = {}

with ZipFile(src_path, "r") as src, ZipFile(dst_path, "a", compression=zipfile.ZIP_DEFLATED) as dst:
    src_files = {i.filename.casefold(): i.filename for i in src.filelist}
    dst_files = {i.filename.casefold(): i.filename for i in dst.filelist}

    # dump the level block to move relevant infomation
    with dst.open(dst_files['sub_blocks1/level.json'], "r") as f:
        vals_dest = json.load(f)
    with src.open(src_files['sub_blocks1/level.json'], "r") as f:
        vals = json.load(f)

    # get the needed objects and items associated with them
    infos = set()
    class_items = copy_tree(vals, class_guid, infos=infos)
    scripts = set([i for i in infos if f'sub_blocks1/{i}.lua' in src_files])
    
    script_strings = {}
    for path in src_files.values():
        if path.endswith('.lua'):
            name = Path(path).stem.casefold()
            data = src.read(path)
            strings = set(get_lua_strings(data))
            script_strings[name] = strings
    
    for i in scripts:
        if not i.startswith("anm_"): continue
        f_name = src_files[f'animation_tables/{i}.json']
        with src.open(f_name, "r") as f:
            anim_table = json.load(f)
        for anim in anim_table.values():
            if isinstance(anim, list):
                infos.update([i.casefold() for i in anim])
            else:
                infos.add(anim.casefold())
    
    old_scripts = set(Path(i).stem for i in dst_files.keys() if i.endswith('.lua'))
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
    for k in infos:
        f_name = f"meshes/{k}.json"
        if f_name in dst_files:
            obj = dst.read(dst_files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask | int(obj[a:b])).encode() + obj[b:]
            to_remove.add(f_name)
            to_add[dst_files[f_name]] = obj
            textures.update([val.decode().casefold() for i in obj.split(b'tex_')[1:] if i[:4] != b'data' and (val:=i.split(b'"')[2]) != b''])
        elif f_name in src_files:
            obj = src.read(src_files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask).encode() + obj[b:]
            to_add[src_files[f_name]] = obj
            textures.update([val.decode().casefold() for i in obj.split(b'tex_')[1:] if i[:4] != b'data' and (val:=i.split(b'"')[2]) != b''])

        f_name = f"effects/{k}.json"
        if f_name in dst_files:
            obj = dst.read(dst_files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask | int(obj[a:b])).encode() + obj[b:]
            to_remove.add(f_name)
            to_add[dst_files[f_name]] = obj
        elif f_name in src_files:
            obj = src.read(src_files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask).encode() + obj[b:]
            to_add[src_files[f_name]] = obj
        
        f_name = f"animations/{k}.json"
        if f_name in dst_files:
            obj = dst.read(dst_files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask | int(obj[a:b])).encode() + obj[b:]
            to_remove.add(f_name)
            to_add[dst_files[f_name]] = obj
        elif f_name in src_files:
            obj = src.read(src_files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask).encode() + obj[b:]
            to_add[src_files[f_name]] = obj

    for k in textures:
        f_name = f"textures/{k}.json"
        f_name_alt = f"textures/{k}.dds"
        if f_name in dst_files:
            obj = dst.read(dst_files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask | int(obj[a:b])).encode() + obj[b:]
            to_remove.add(f_name)
            to_add[dst_files[f_name]] = obj
        elif f_name in src_files:
            obj = src.read(src_files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask).encode() + obj[b:]
            to_add[src_files[f_name]] = obj
            to_add[src_files[f_name_alt]] = src.read(src_files[f_name_alt])

    with dst.open(dst_files['sub_blocks1/index.json'], "r") as f:
        index = json.load(f)
    for script in new_scripts:
        f_name = src_files[f"sub_blocks1/{script}.lua"]
        index['block_headers'].insert(-3, {'key': f_name[len("sub_blocks1/"):], 'offset': 0, 'size': 0})
        to_add[f_name] = src.read(f_name)
        if script.startswith("anm_"):
            f_name = src_files[f'animation_tables/{i}.json']
            to_add[f_name] = src.read(f_name)
    to_remove.add('sub_blocks1/index.json')
    to_add[dst_files['sub_blocks1/index.json']] = json.dumps(index, indent=1)
    
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
    
    to_remove.add('sub_blocks1/level.json')
    to_add[dst_files['sub_blocks1/level.json']] = json.dumps(vals_dest, indent=1)

    with src.open(src_files['bin_strings.json'], "r") as f:
        bin_strings_src = json.load(f)
    with src.open(src_files['pak_strings.json'], "r") as f:
        pak_strings_src = json.load(f)
    with dst.open(dst_files['bin_strings.json'], "r") as f:
        bin_strings_dst = json.load(f)
    with dst.open(dst_files['pak_strings.json'], "r") as f:
        pak_strings_dst = json.load(f)
        
    pak_strings = set(pak_strings_dst)
    bin_strings = set(bin_strings_dst)
    pak_strings_dst.extend([i for i in pak_strings_src if i not in pak_strings])
    bin_strings_dst.extend([i for i in bin_strings_src if i not in bin_strings])

    to_remove.add('pak_strings.json')
    to_remove.add('bin_strings.json')
    to_add[dst_files['pak_strings.json']] = json.dumps(pak_strings_dst, indent=1)
    to_add[dst_files['bin_strings.json']] = json.dumps(bin_strings_dst, indent=1)

    dst.remove(*[dst_files[i] for i in to_remove])
    for f_name, data in to_add.items():
        dst.writestr(f_name, data)