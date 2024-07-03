import json
import zipfile
from operator import attrgetter

### Replacing Helm'sDeep good scout mesh with wormtongue from Isengard
# first dump the needed levels
# run this script pointing to the needed levels with the correct src mesh and target class name
# then compile the dumped dst level back into a .PAK and .BIN file

new_mesh = 'CH_hum_Wormtongue_01'
targ_clss = 'CH_NML_SIL_Scout'

src_path = "DumpedLevels/Isengard.zip"
dst_path = "DumpedLevels/Helm'sDeep.zip"

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

to_remove = set()
to_add = {}

with ZipFile(src_path, "r") as src, ZipFile(dst_path, "a", compression=zipfile.ZIP_DEFLATED) as dst:
    src_files = {i.filename.casefold(): i.filename for i in src.filelist}
    dst_files = {i.filename.casefold(): i.filename for i in dst.filelist}

    # dump the level block to move relevant infomation
    with dst.open(dst_files['sub_blocks1/level.json'], "r") as f:
        vals = json.load(f)

    for i, obj in enumerate(vals['objs']):
        if obj['fields'].get('name') == targ_clss or obj['fields'].get('Name') == targ_clss: 
            obj['fields'].get('name')
            print('found')
            break
    
    if 'meshes' in vals['objs'][i]['fields']:
        old_mesh = vals['objs'][i]['fields']['meshes'].pop()
        vals['objs'][i]['fields']['meshes'].append(new_mesh)
    else:
        old_mesh = vals['objs'][i]['fields']['mesh']
        vals['objs'][i]['fields']['mesh'] = new_mesh
        
    to_remove.add('sub_blocks1/level.json')
    to_add[dst_files['sub_blocks1/level.json']] = json.dumps(vals, indent=1)
    with dst.open(f"meshes/{old_mesh}.json", "r") as f:
        old_mesh = json.load(f)
    gamemodemask = old_mesh['info']['gamemodemask']

    textures = set()
    for k in [new_mesh.casefold()]:
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