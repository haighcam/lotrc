from lotrc.utils import *
from lotrc.types import *

class LevelInfo:
    Header = structtuple("LevelInfo_Header", 
        'constx04', 'I',
        'valA', 'I',
        'strings_offset', 'I',
        'strings_size', 'I',
        'strings_num', 'I',
        'string_keys_size', 'I',
        'string_keys_offset', 'I',
        'local_strings_size', 'I',
        'local_strings_offset', 'I',
        'gamemodes_num', 'I',
        'gamemodes_offset', 'I',
        'levels_num', 'I',
        'levels_offset', 'I',
        'valB', 'I',
    )
    LevelVal = structtuple("LevelInfo_LevelVal", 
        'name', '32S',
        'key_name', 'I',
        'key_description', 'I',
        'valA', 'I',
        'gamemodes', 'I',
    )
    GamemodeVal = structtuple("LevelInfo_GamemodeVal", 
        'key', 'I',
        'key_name', 'I',
        'key_description', 'I',
    )        
    def __init__(self, file):
        with open(file, "rb") as f:
            self.data = f.read()
            
        if self.data[:4] == b'\x04\x00\x00\x00':
            self.f = "<"
        elif self.data[:4] == b'\x00\x00\x00\x04':
            self.f = ">"
        else:
            raise ValueError("Wrong file type?!!")

        self.header = unpack_from(self.Header[self.f], self.data, 0)

        self.strings = read_strings(self.data, self.header['strings_offset'], self.header['strings_num'], self.f)
        self.keys = {hash_string(i):i for i in self.strings}
        
        self.string_keys = StringKeys.unpack_from(self.data, self.header['string_keys_offset'], self.f)
        # assert self.string_keys.string_keys[-1]['offset']+4 == self.header['string_keys_size'], "I think this is true"

        self.local_strings = SubBlocks.unpack_from(self.data, self.header['local_strings_offset'], self.keys, None, self.f)
                        
        self.gamemodes = unpack_list_from(self.GamemodeVal[self.f], self.data, self.header['gamemodes_offset'], self.header['gamemodes_num'])
        self.levels = unpack_list_from(self.LevelVal[self.f], self.data, self.header['levels_offset'], self.header['levels_num'])

        self.key = self.data[0x38:0x13c]

    def dump(self, f):
        file = bytearray(len(self.data))
        pack_into(self.header, file, 0, f)

        write_strings(file, self.header['strings_offset'], self.strings, f)

        self.string_keys.pack_into(file, self.header['string_keys_offset'], f)

        self.local_strings.pack_into(file, self.header['local_strings_offset'], f)

        pack_into(self.gamemodes, file, self.header['gamemodes_offset'], f)
        pack_into(self.levels, file, self.header['levels_offset'], f)

        file[0x38:0x13c] = self.key
        
        return file