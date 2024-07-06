import json
import zipfile
from operator import attrgetter

# try to add required objects to a gamemode by setting the corresponding gamemodemask

src_path = "DumpedLevels/MinasTirith_Top.zip"
gamemodeguid = 144015924

# for finding missing objects in the gamemode, set to dst of get_obj_index.py
obj_index = None

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

def scan(vals, guid, infos=None):
    if infos is None:
        infos = set()
    obj = find_obj(vals, guid)
    ty = find_type(vals, obj['type'])
    objs = [obj]
    for t in ty['fields']:
        if (t['type'] == 'crc' or t['type'] == 'string') and (val:=obj['fields'][t['name']]) != '':
            infos.add(val.casefold())
        elif t['type'] == 'crclist' or t['type'] == 'stringlist':
            infos.update([i.casefold() for i in obj['fields'][t['name']]])
                
to_remove = set()
to_add = {}

with ZipFile(src_path, "a", compression=zipfile.ZIP_DEFLATED) as src:
    files = {i.filename.casefold(): i.filename for i in src.filelist}
    
    with src.open('animation_block_infos.json', "r") as f:
        anim_infos = json.load(f)

    gamemodemask = -1
    for i, val in enumerate(anim_infos):
        if val.get('guid', val.get('unk_1')) == gamemodeguid:
            gamemodemask = 1 << i
            print(f'found gamemode at index {i}, {gamemodemask}')
            break

    with src.open('sub_blocks1/level.json', "r") as f:
        vals = json.load(f)

    # update the gamemodemask of all objects used in the gamemode
    print("updating GameModeMask in level data")
    gmd = find_obj(vals, gamemodeguid)
    # gmd['fields']['GameModeMask'] = -1
    processed = set()
    objs = copy_tree(vals, gamemodeguid, processed=processed)
    guids = set(i['fields']['guid'] for i in objs)
    for layer in gmd['fields']['layers']:
        for i in get_layer(vals, layer):
            if i['fields']['guid'] not in guids:
                for j in copy_tree(vals, i['fields']['guid'], processed=processed):
                    if j['fields']['guid'] not in guids:
                        guids.add(j['fields']['guid'])
                        objs.append(j)
    for obj in objs:
        obj['fields']['GameModeMask'] |= gamemodemask

    to_remove.add('sub_blocks1/level.json')
    to_add['sub_blocks1/level.json'] = json.dumps(vals, indent=1)

    # get all used scripts, meshes, effects and animations 
    # if the gamemode is brand new then this is the same objects as above
    print("finding objects used in gamemode")
    infos = set()
    for i in vals['objs']:
        if 'GameModeMask' in i['fields'] and (i['fields']['GameModeMask'] & gamemodemask) == 0: continue
        scan(vals, i['fields']['guid'], infos=infos)

    scripts = set([i for i in infos if f'sub_blocks1/{i}.lua' in files])

    for i in scripts:
        if not i.startswith("anm_"): continue
        with src.open(files[f'animation_tables/{i}.json'], "r") as f:
            anim_table = json.load(f)
        for anim in anim_table.values():
            if isinstance(anim, list):
                infos.update([i.casefold() for i in anim])
            else:
                infos.add(anim.casefold())

    print("mesh / effects / anims")
    textures = set([i for i in infos if f'textures/{i}.json' in files])
    for k in infos:
        f_name = f"meshes/{k}.json"
        if f_name in files:
            obj = src.read(files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask | int(obj[a:b])).encode() + obj[b:]
            to_remove.add(f_name)
            to_add[f_name] = obj
            textures.update([val.decode().casefold() for i in obj.split(b'tex_')[1:] if i[:4] != b'data' and (val:=i.split(b'"')[2]) != b''])
        f_name = f"effects/{k}.json"
        if f_name in files:
            obj = src.read(files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask | int(obj[a:b])).encode() + obj[b:]
            to_remove.add(f_name)
            to_add[f_name] = obj
        f_name = f"animations/{k}.json"
        if f_name in files:
            obj = src.read(files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask | int(obj[a:b])).encode() + obj[b:]
            to_remove.add(f_name)
            to_add[f_name] = obj
    
    print("textures")
    for k in textures:
        f_name = f"textures/{k}.json"
        if f_name in files:
            obj = src.read(files[f_name])
            a = obj.find(b'"gamemodemask": ') + len(b'"gamemodemask": ')
            b = obj.find(b',', a)
            obj = obj[:a] + str(gamemodemask | int(obj[a:b])).encode() + obj[b:]
            to_remove.add(f_name)
            to_add[f_name] = obj
        
    print("applying changes")
    src.remove(*[files[i] for i in to_remove])
    for f_name, data in to_add.items():
        src.writestr(files[f_name], data)

if obj_index is not None:
    with open(obj_index, "rb") as f:
        obj_index = json.load(f)

    missing_meshes = [i for i in infos if i in obj_index['meshes'] and f'meshes/{i}.json' not in files]
    missing_animations = [i for i in infos if i in obj_index['animations'] and f'animations/{i}.json' not in files]
    missing_effects = [i for i in infos if i in obj_index['effects'] and f'effects/{i}.json' not in files]
    missing_scripts = [i for i in infos if i in obj_index['scripts'] and f'sub_blocks1/{i}.lua' not in files]

    print("missing meshes:", missing_meshes)
    print("missing animations:", missing_animations)
    print("missing effects:", missing_effects)
    print("missing scripts:", missing_scripts)