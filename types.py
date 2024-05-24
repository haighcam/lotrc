import numpy as np
import warnings

from lotrc.utils import *
DECOMP_LUA = True
MIN_OFFSET = np.inf
MAX_OFFSET = 0

class CompressedBlock:
    def __init__(self, data, data_comp=None):
        self.data = data
        self.size = len(data)
        self.data_comp = data_comp

    @classmethod
    def unpack_from(Self, buffer, size, size_comp, offset):
        if size_comp != 0:
            data_comp = buffer[offset:offset+size_comp]
            data = decomp_zlib(data_comp)
        else:
            data = buffer[offset:offset+size]
            data_comp = None
        return Self(data, data_comp)
    
    def pack_into(self, buffer, offset, compress=True):
        if self.size == 0:
            self.size_comp = 0
            return self.size
        elif compress:
            data = comp_zlib(self.data)
            self.comp_size = len(data)
            buffer[offset:offset+self.comp_size] = data
            return self.comp_size
        else:
            self.comp_size = 0
            buffer[offset:offset+self.size] = self.data
            return self.size

    def pack(self, compress=True):
        if self.size == 0:
            self.size_comp = 0
            return b''
        elif compress:
            data = comp_zlib(self.data)
            self.size_comp = len(data)
            return data
        else:
            self.size_comp = 0
            return self.data

def formatpair(name, f0, f1, simple_conv=True, common_names=None):
    self = {}
    self.update({
        'name': name,
        'simple_conv': simple_conv,
        'common_names': common_names,
        '<': np.dtype(f0, metadata={'f': '<', 'self': self}),
        '>': np.dtype(f1, metadata={'f': '>', 'self': self})
    })
    return self

def conv(data):
    if data.dtype.metadata is None:
        return data.astype(data.dtype.newbyteorder())
    else:
        self = data.dtype.metadata['self']
        t = self['>' if data.dtype.metadata['f'] == '<' else '<']
        if self['simple_conv']:
            return data.astype(t)
        else:
            d = np.zeros(data.shape, t)
            for name in self['common_names']:
                d[name] = conv(data[name])
            return d

def unpack_from(T, buffer, offset):
    global MIN_OFFSET, MAX_OFFSET
    MIN_OFFSET = min(MIN_OFFSET, offset)
    MAX_OFFSET = max(MAX_OFFSET, offset + T.itemsize)
    return np.frombuffer(buffer, T, 1, offset)[0].copy()

def unpack_list_from(T, buffer, offset, num):
    global MIN_OFFSET, MAX_OFFSET
    MIN_OFFSET = min(MIN_OFFSET, offset)
    MAX_OFFSET = max(MAX_OFFSET, offset + T.itemsize * num)
    return np.frombuffer(buffer, T, num, offset).copy()

def new(T, val):
    return np.array([val], T)[0]

def new_list(T, vals):
    return np.array(vals, T)

def pack_into(data, buffer, offset, f='<'):
    if data.dtype.metadata is not None and data.dtype.metadata['f'] != f:
        data = conv(data)
    s = data.nbytes
    buffer[offset:offset+s] = data.tobytes()
    return s

def pack(data, f='<'):
    if data.dtype.metadata is not None and data.dtype.metadata['f'] != f:
        data = conv(data)
    return data.tobytes()
    
def structtuple(name, *args, alt_fmt=None):
    assert len(args)%2 == 0, "Needs an even number of arguments"
    names = []
    formats = []
    for i in range(0, len(args), 2):
        names.append(args[i])
        formats.append(args[i+1])
    if alt_fmt is not None:
        assert len(alt_fmt)%2 == 0, "Needs an even number of arguments"
        alt_names = []
        alt_formats = []
        for i in range(0, len(alt_fmt), 2):
            alt_names.append(alt_fmt[i])
            alt_formats.append(alt_fmt[i+1])
    else:
        alt_names = names
        alt_formats = formats
    f0 = np.dtype([(name, i['<'] if isinstance(i, dict) else i if i[0] == '<' or i[0] == '>' else '<'+i) for name, i in zip(names, formats)])
    f1 = np.dtype([(name, i['>'] if isinstance(i, dict) else i if i[0] == '<' or i[0] == '>' else '>'+i) for name, i in zip(alt_names, alt_formats)])
    simple_conv = names == alt_names
    common_names = None if simple_conv else [i for i in names if i in alt_names]
    return formatpair(name, f0, f1, simple_conv, common_names)

Float = structtuple("Float", "val", "f")
Int = structtuple("Int", "val", "i")
Uint = structtuple("Uint", "val", "I")
Ushort = structtuple("Ushort", "val", "H")
Ubyte = structtuple("Ubyte", "val", "B")
Short = structtuple("Short", "val", "h")
Byte = structtuple("Byte", "val", "b")

CRC = structtuple("CRC", "val", "I") # string key
GUID = structtuple("GUID", "val", "I")
# Color = structtuple("Color", "r", "B", "g", "B", "b", "B", "a", "B")
Color = structtuple("Color", "val", "I")
Vector2 = structtuple("Vector2", "x", "f", "y", "f")
Vector3 = structtuple("Vector3", "x", "f", "y", "f", "z", "f")
Vector4 = structtuple("Vector4", "x", "f", "y", "f", "z", "f", "w", "f")
Matrix4x4 = structtuple("Matrix4x4", "xx", "f", "xy", "f", "xz", "f", "xw", "f", "yx", "f", "yy", "f", "yz", "f", "yw", "f", "zx", "f", "zy", "f", "zz", "f", "zw", "f", "wx", "f", "wy", "f", "wz", "f", "ww", "f",)
Bool = structtuple("Bool", "val", "B", "p0", "B", "p1", "B", "p2", "B")
Node = structtuple("Node", "a", "I", "b", "I", "c", "I", "d", "I")
Weight = structtuple("Weight", "a", "I", "b", "B", "c", "B", "d", "B", "e", "B")
StringElem = structtuple("StringElem", "val", "S1")

String = structtuple("String", "num", "H", "offset", "H")
StringList = structtuple("StringList", "num", "H", "offset", "H")
ObjectList = structtuple("ObjectList", "num", "H", "offset", "H")
NodeList = structtuple("NodeList", "num", "H", "offset", "H")
IntList = structtuple("IntList", "num", "H", "offset", "H")
CRCList = structtuple("CRCList", "num", "H", "offset", "H")
WeightList = structtuple("WeightList", "num", "H", "offset", "H")
MatrixList = structtuple("MatrixList", "num", "H", "offset", "H")

# Float = structtuple("Float", "val", "I")
# Int = structtuple("Int", "val", "i")
# Uint = structtuple("Uint", "val", "I")
# Ushort = structtuple("Ushort", "val", "H")
# Ubyte = structtuple("Ubyte", "val", "B")
# Short = structtuple("Short", "val", "h")
# Byte = structtuple("Byte", "val", "b")

# CRC = structtuple("CRC", "val", "I")
# GUID = structtuple("GUID", "val", "I")
# Color = structtuple("Color", "r", "B", "g", "B", "b", "B", "a", "B")
# Vector2 = structtuple("Vector2", "x", "I", "y", "I")
# Vector3 = structtuple("Vector3", "x", "I", "y", "I", "z", "I")
# Vector4 = structtuple("Vector4", "x", "I", "y", "I", "z", "I", "w", "I")
# Matrix4x4 = structtuple("Matrix4x4", "xx", "I", "xy", "I", "xz", "I", "xw", "I", "yx", "I", "yy", "I", "yz", "I", "yw", "I", "zx", "I", "zy", "I", "zz", "I", "zw", "I", "wx", "I", "wy", "I", "wz", "I", "ww", "I",)
# Bool = structtuple("Bool", "val", "I")
# Node = structtuple("Node", "a", "I", "b", "I", "c", "I", "d", "I")
# Weight = structtuple("Weight", "a", "I", "b", "I")
# StringElem = structtuple("StringElem", "val", "S1")

# String = structtuple("String", "num", "H", "offset", "H")
# StringList = structtuple("StringList", "num", "H", "offset", "H")
# ObjectList = structtuple("ObjectList", "num", "H", "offset", "H")
# NodeList = structtuple("NodeList", "num", "H", "offset", "H")
# IntList = structtuple("IntList", "num", "H", "offset", "H")
# CRCList = structtuple("CRCList", "num", "H", "offset", "H")
# WeightList = structtuple("WeightList", "num", "H", "offset", "H")
# MatrixList = structtuple("MatrixList", "num", "H", "offset", "H")

ListTypes = {hash_string(i['name']): j for i,j in [
    (String, StringElem),
    (StringList, String),
    (ObjectList, Int),
    (NodeList, Node),
    (IntList, Int),
    (CRCList, CRC),
    (WeightList, Weight),
    (MatrixList, Matrix4x4),
]}

# Some base types are stored in place, others are 2 shorts (num, offset) to data after them
BaseTypes = {hash_string(i['name']):i for i in [
    CRC,
    GUID,
    Color,
    Vector3,
    Vector4,
    Matrix4x4,
    Float,
    Int,
    Bool,
    String,
    StringList,
    ObjectList,
    NodeList,
    IntList,
    CRCList,
    WeightList,
    MatrixList,
]}

class StringKeys:
    Header = structtuple("StringKeys_Header", 
        'numA', 'H',
        'numB', 'H',
        'z2', 'I',
        'z3', 'I',
        'z4', 'I',
        'z5', 'I',
    )
    Val = structtuple("StringKeys_Val", 
        'key', 'I',
        'offset', 'I',
    )
    @classmethod
    def unpack_from(Self, buffer, offset, f="<"):
        self = Self()
        self.header = unpack_from(self.Header[f], buffer, offset)
        assert self.header['numA'] == self.header['numB'], "Probably needs to be true"
        offset += self.header.nbytes
        self.string_keys = unpack_list_from(self.Val[f], buffer, offset, self.header['numA'])
        offset += self.string_keys.nbytes
        self.extra = unpack_list_from(Uint[f], buffer, offset, self.header['numA'])
        # assert self.string_keys[-1].offset+4 == size, "I think this is true"
        return self
    def pack_into(self, buffer, offset, f="<"):
        offset += pack_into(self.header, buffer, offset, f)
        pack_into(self.string_keys, buffer, offset, f)
        offset += self.string_keys.nbytes
        pack_into(self.extra, buffer, offset, f)
    def pack(self, f="<"):
        return pack(self.header, f) + pack(self.string_keys, f) + pack(self.extra, f)

class LangStrings:
    Keys = {hash_string(i):i for i in [
        'Polish',
        'German',
        'French',
        'Norwegian',
        'Spanish',
        'Russian',
        'Swedish',
        'English',
        'Italian'
    ]}
    @classmethod
    def unpack_from(Self, buffer, offset_, size, f="<"):
        if f == "<":
            str_format = "utf-16-le"
        elif f == ">":
            str_format = "utf-16-be"
        else:
            raise ValueError("invalid format")
        self = Self()
        self.strings = []
        self.size = size
        offset = offset_
        while offset < size + offset_:
            start = offset
            while buffer[offset:offset+2] != b'\x00\x00':
                offset += 2
            self.strings.append(buffer[start:offset].decode(str_format))
            offset += 2
        return self

    def pack_into(self, buffer, offset, f="<"):
        if f == "<":
            str_format = "utf-16-le"
        elif f == ">":
            str_format = "utf-16-be"
        else:
            raise ValueError("invalid format")

        for s in self.strings:
            s_ = s.encode(str_format)
            buffer[offset:offset+len(s_)] = s_
            offset += len(s_) + 2

    def dump(self, f="<"):
        buffer = bytearray(self.size)
        self.pack_into(buffer, 0, f)
        return bytes(buffer)

class SubBlocks:
    Header = structtuple("SubBlocks_Header", 
        'z0', 'I',
        'block_num', 'I',
        'z2', 'I',
        'z3', 'I',
    )
    BlockHeader = structtuple("SubBlocks_BlockHeader", 
        'key', 'I',
        'offset', 'I',
        'size', 'I',
    )
    @classmethod
    def unpack_from(Self, buffer, offset, string_lookup, types, f="<"):
        self = Self()
        self.header = unpack_from(self.Header[f], buffer, offset)
        self.block_headers = unpack_list_from(self.BlockHeader[f], buffer, offset+self.header.nbytes, self.header['block_num'])
        self.blocks = []
        self.size = 0
        for header in self.block_headers:
            name = string_lookup[(key := header['key'])]
            if key in LangStrings.Keys:
                self.blocks.append(LangStrings.unpack_from(buffer, offset+header['offset'], header['size'], f))
            elif key == PFields.Key:
                self.blocks.append(PFields.unpack_from(buffer, offset+header['offset'], header['size'], f))
            elif key == Spray.Key:
                self.blocks.append(Spray.unpack_from(buffer, offset+header['offset'], header['size'], f))
            elif key == Crowd.Key:
                self.blocks.append(Crowd.unpack_from(buffer, offset+header['offset'], header['size'], f))
            elif key == GameObjs.Key:
                self.blocks.append(GameObjs.unpack_from(buffer, offset+header['offset'], header['size'], types, f))
            elif key in AtlasUV.Keys:
                self.blocks.append(AtlasUV.unpack_from(buffer, offset+header['offset'], header['size'], f))
            elif (t := name.split(b'.')[-1]) == b'lua':
                self.blocks.append(Lua.unpack_from(buffer, offset+header['offset'], header['size'], name, f))
            elif t in [b'csv', b'txt', b'dat', b'ssa']:
                self.blocks.append(Data.unpack_from(buffer, offset+header['offset'], header['size'], f))
            else:
                warnings.warn(f"Unhandled block type: {name}, treating as raw bytes")
                self.blocks.append(Data.unpack_from(buffer, offset+header['offset'], header['size'], f))
            self.size = max(self.size, (header['size'] + header['offset'] + 15) & 0xfffffff0)
        return self

    def pack_into(self, buffer, offset, f="<"):
        pack_into(self.header, buffer, offset, f)
        dump_block_headers = self.block_headers.copy()
        # pack_into(self.block_headers, buffer, self.header.nbytes+offset, f)
        off = dump_block_headers[0]['offset']
        for block, header in zip(self.blocks, dump_block_headers):
            # block.pack_into(buffer, offset+header['offset'], f)
            data = block.dump(f)
            header['offset'] = off
            header['size'] = len(data)
            buffer[offset+off:offset+off+header['size']] = data
            off = (off+header['size'] + 15) & 0xfffffff0
            size = off
        pack_into(dump_block_headers, buffer, self.header.nbytes+offset, f)

    def pack(self, f="<"):
        dump_block_headers = self.block_headers.copy()
        offset = self.header.nbytes + dump_block_headers.nbytes
        buffer = bytes()
        for block, header in zip(self.blocks, dump_block_headers):
            off = (offset + 15) & 0xfffffff0
            buffer += bytes(off - offset)
            offset = off
            data = block.dump(f)
            header['offset'] = offset
            header['size'] = len(data)
            offset += header['size']
            buffer += data
        return pack(self.header, f) + pack(dump_block_headers, f) + buffer

def get_level_obj_format(key, fields):
    order = np.argsort(fields['offset'])
    fmt = []
    offset = 0
    p = 0
    for i in order:
        f = fields[i]
        for _ in range(f['offset'] - offset):
            fmt.extend([f"p{p}", 'B'])
            p += 1
        t = BaseTypes[f['type']]
        fmt.extend([f"f{i}", t])
        offset = f['offset'] + t['<'].itemsize
    return structtuple(f"level_obj_{key:08X}", *fmt)

class Lua:
    @classmethod
    def unpack_from(Self, buffer, offset, size, name, f="<"):
        self = Self()
        self.f = f
        self.data = buffer[offset:offset+size]
        if DECOMP_LUA:
            self.code = decomp_lua(self.data)
        self.name = name
        return self
    def pack_into(self, buffer, offset, f="<"):
        if f == "<":
            if DECOMP_LUA:
                data = compile_lua(self.code, self.name)
            else:
                data = convert_lua(self.data, b"L4404")
            buffer[offset:offset+len(data)] = data
            return len(data)
        else:
            raise ValueError("Not Yet Implemeted")

    def dump(self, f="<"):
        if DECOMP_LUA:
            data = compile_lua(self.code, self.name)
        else:
            data = convert_lua(self.data, b"L4404")
        return data
        # if f == self.f:
        #     return self.data
        # elif f == "<":
        #     data = compile_lua(self.code)
        #     return data
        # else:
        #     raise ValueError("Not Yet Implemeted")
        # return self.data + b"000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        return self.data

class GameObjs:
    Key = hash_string("Level")
    Header = structtuple("GameObjs_Headaer",
        "const", "I",
        "types_num", "I",
        "types_offset", "I",
        "obj_num", "I",
        "obj_offset", "I", 
        "z5", "I",
        "z6", "I",
        "z8", "I",
    )
    TypeHeader = structtuple("GameObjs_TypeHeader", 
        "key", "I",
        "size", "I",
        "fields", "I",
    )
    TypeField = structtuple("GameObjs_ObjField", 
        "key", "I",
        "type", "I",
        "offset", "I",
    )
    ObjHeader = structtuple("GameObjs_ObjHeader",
        "unk_0", "I",
        "key", "I",
        "size", "H",
        "z3", "H",
        "z4", "I",
    )
    @classmethod
    def unpack_from(Self, buffer, offset_, size, types, f="<"):
        self = Self()
        self.size = size
        self.data = buffer[offset_:offset_+size]
        # return self
        self.header = unpack_from(self.Header[f], buffer, offset_)

        self.types = []
        self.type_fields = []
        # types = {}
        offset = self.header['types_offset'] + offset_
        for _ in range(self.header['types_num']):
            obj = unpack_from(self.TypeHeader[f], buffer, offset)
            offset += obj.nbytes
            type_fields = unpack_list_from(self.TypeField[f], buffer, offset, obj['size'])
            offset += type_fields.nbytes
            if obj['key'] not in types:
                t = get_level_obj_format(obj['key'], type_fields)
                t['fields'] = type_fields
                types[obj['key']] = t
            self.types.append(obj)
            self.type_fields.append(type_fields)

        self.objs = []
        self.obj_fields = []
        self.obj_fields_data = []
        offset = self.header['obj_offset'] + offset_
        for _ in range(self.header['obj_num']):
            obj = unpack_from(self.ObjHeader[f], buffer, offset)
            offset += obj.nbytes
            t = types[obj['key']]
            self.obj_fields.append(unpack_from(t[f], buffer, offset))
            fields = t['fields']
            self.obj_fields_data.append([None for _ in range(len(fields))])
            for i, field in enumerate(fields):
                if (T := ListTypes.get(field['type'], None)) is not None:
                    val = self.obj_fields[-1][f'f{i}']
                    data = unpack_list_from(T[f], buffer, offset + field['offset'] + val['offset'] + val.nbytes, val['num'])
                    if T['name'] == 'String':
                        data = [data] + [
                            unpack_list_from(StringElem[f], buffer, offset + field['offset'] + val['offset'] + val.nbytes * (i+1) + v['offset'] + v.nbytes, v['num']) for i,v in enumerate(data)
                        ]
                    self.obj_fields_data[-1][i] = data
            self.objs.append(obj)
            offset += obj['size']
        return self
            
    def pack_into(self, buffer, offset_, f="<"):
        pack_into(self.header, buffer, offset_, f)

        offset = self.header['types_offset'] + offset_
        for obj, fields in zip(self.types, self.type_fields):
            offset += pack_into(obj, buffer, offset, f)
            offset += pack_into(fields, buffer, offset, f)

        offset = self.header['obj_offset'] + offset_
        for obj, fields_vals, fields_data in zip(self.objs, self.obj_fields, self.obj_fields_data):
            offset += pack_into(obj, buffer, offset, f)
            pack_into(fields_vals, buffer, offset, f)
            fields = fields_vals.dtype.metadata['self']['fields']
            # fields = self.fields_lookup[obj['key']]
            for i, field in enumerate(fields):
                if (T := ListTypes.get(field['type'], None)) is not None:
                    val = fields_vals[f'f{i}']
                    data = fields_data[i]
                    if T['name'] == 'String':
                        for i,(v,d) in enumerate(zip(data[0], data[1:])):
                            pack_into(d, buffer, offset + field['offset'] + val['offset'] + val.nbytes * (i+1) + v['offset'] + v.nbytes, f)
                        data = data[0]
                    pack_into(data, buffer, offset + field['offset'] + val['offset'] + val.nbytes, f)
            offset += obj['size']

    def dump(self, f="<"):
        buffer = bytearray(self.size)
        self.pack_into(buffer, 0, f)
        return bytes(buffer)

class Spray:
    Key = hash_string("Spray")
    Obj1 = structtuple("Obj1", # a bunch of these are floats
        "key", "I",
        "unk_1", "I",
        "unk_2", "I",
        "unk_3", "I",
        "unk_4", "I",
        "unk_5", "I",
        "unk_6", "I",
        "unk_7", "I",
        "unk_8", "I",
        "unk_9", "I",
        "unk_10", "I",
        "unk_11", "I",
        "unk_12", "I",
        "unk_13", "I",
        "unk_14", "I",
        "unk_15", "I",
        "unk_16", "I",
    )
    Obj2 = structtuple("SprayObj2",
        "unk_0", "f",
        "unk_1", "f",
        "unk_2", "f",
        "unk_3", "f",
        "unk_4", "f",
    )
    @classmethod
    def unpack_from(Self, buffer, offset, size, f="<"):
        self = Self()
        self.size = size
        self.obj1_num = unpack_from(Uint[f], buffer, offset)
        offset += self.obj1_num.nbytes
        self.obj1s = unpack_list_from(self.Obj1[f], buffer, offset, self.obj1_num['val'])
        offset += self.obj1s.nbytes
        self.obj2_num = unpack_from(Uint[f], buffer, offset)
        offset += self.obj2_num.nbytes
        self.obj2s = unpack_list_from(self.Obj2[f], buffer, offset, self.obj2_num['val'])
        return self

    def pack_into(self, buffer, offset, f="<"):
        offset += pack_into(self.obj1_num, buffer, offset, f)
        offset += pack_into(self.obj1s, buffer, offset, f)
        offset += pack_into(self.obj2_num, buffer, offset, f)
        pack_into(self.obj2s, buffer, offset, f)

    def dump(self, f="<"):
        buffer = bytearray(self.size)
        self.pack_into(buffer, 0, f)
        return bytes(buffer)

class Crowd:
    Key = hash_string("3dCrowd")
    Header = structtuple("Crowd_Header", 
        "key_0", "I",
        "key_1", "I",
        "key_2", "I",
        "key_3", "I",
        "unk_4", "I",
        "keys_num", "I",
        "num", "I",
    )
    
    Vals = structtuple("Crowd_Val",
        "unk_0", "I",
        "unk_1", "I",
        "unk_2", "I",
        "unk_3", "I",
        "unk_4", "I",
    )
    @classmethod
    def unpack_from(Self, buffer, offset_, size, f="<"):
        self = Self()
        self.size = size
        self.const, self.num = unpack_list_from(Uint[f], buffer, offset_, 2)
        assert self.const['val'] == 0x65, "Wrong Block Type"
        self.offsets = unpack_list_from(Uint[f], buffer, offset_+8, self.num['val']) 
        self.headers = []
        self.vals = []
        self.keys = []
        for offset in self.offsets['val']:
            offset += offset_
            header = unpack_from(self.Header[f], buffer, offset)
            offset += header.nbytes
            keys = unpack_list_from(Uint[f], buffer, offset, header['keys_num']) 
            offset += keys.nbytes
            vals = unpack_list_from(self.Vals[f], buffer, offset, header['num'])
            self.headers.append(header)
            self.vals.append(vals)
            self.keys.append(keys)
        return self

    def pack_into(self, buffer, offset_, f="<"):
        pack_into(self.const, buffer, offset_, f)
        pack_into(self.num, buffer, offset_ + 4, f)
        pack_into(self.offsets, buffer, offset_ + 8, f)
        for offset, header, vals, keys in zip(self.offsets['val'], self.headers, self.vals, self.keys):
            offset += offset_
            offset += pack_into(header, buffer, offset, f)
            offset += pack_into(keys, buffer, offset, f)
            pack_into(vals, buffer, offset, f)

    def dump(self, f="<"):
        buffer = bytearray(self.size)
        self.pack_into(buffer, 0, f)
        return bytes(buffer)


class AtlasUV:
    Keys = [hash_string('atlas_1.uv'), hash_string('atlas_2.uv')]
    Val = structtuple("AtlasUV_Val",
        "key", "I",
        "vals", Vector4,
    )
    @classmethod
    def unpack_from(Self, buffer, offset, size, f="<"):
        self = Self()
        assert size%self.Val[f].itemsize == 0, "Invalid UV Atlas size?"
        num = size//self.Val[f].itemsize
        self.vals = unpack_list_from(self.Val[f], buffer, offset, num)
        return self

    def pack_into(self, buffer, offset, f="<"):
        pack_into(self.vals, buffer, offset, f)

    def dump(self, f="<"):
        return pack(self.vals, f)

class Data:
    def __init__(self):
        pass
    @classmethod
    def unpack_from(Self, buffer, offset, size, f="<"):
        self = Self()
        self.size = size
        self.data = buffer[offset:offset+size]
        return self

    def pack_into(self, buffer, offset, f="<"):
        buffer[offset:offset+self.size] = self.data
    def dump(self, f="<"):
        return self.data

class PFields(Data):
    Key = hash_string("PFields")
    # has data from obj12, 2D something
    # no change from big to little endian

