try:
    from zlib_ng import zlib_ng as zlib
except:
    import zlib
import numpy as np
import struct
import subprocess
import tempfile
import os
import pathlib
import string
import lotrc
from lupa.lua51 import LuaRuntime

lua = LuaRuntime(encoding=None)
lua_dump = lua.eval("function(obj) return dofile(\"" + pathlib.Path(os.path.dirname(lotrc.__file__)).as_posix() + "/lua-bytecode.lua\")(string.dump(obj), \"L4404\") end")
lua_conv = lua.eval("function(obj, f) return dofile(\"" + pathlib.Path(os.path.dirname(lotrc.__file__)).as_posix() + "/lua-bytecode.lua\")(obj, f) end")

def print_data(data):
    self = data.dtype.metadata['self']
    print(f"{self['name']}(\n\t"+'\n\t'.join(list(f'{name} = {data[name]}' for name in data.dtype.names))+'\n)')

def get_global_keys():
    with open(pathlib.Path(os.path.dirname(lotrc.__file__)).as_posix() + "/conquest_strings.txt") as f:
        vals = f.read().split('\n')
    return {hash_string(i): i for i in vals}

#### Lua stuff

def decomp_lua(data):
    with tempfile.NamedTemporaryFile(delete=False) as f:
        f.write(data)
        f.close()
        out = subprocess.run(["java", "-jar", pathlib.Path(os.path.dirname(lotrc.__file__)).as_posix() + "/unluac.jar", f.name], stdout=subprocess.PIPE)
        os.remove(f.name)
    return out.stdout.decode()

def compile_lua(code, name):
    return lua_dump(lua.compile(code, name=name))
    # return subprocess.run([os.path.dirname(lotrc.__file__) + "/luac5.1", "-o", "/dev/stdout", "-"], input=code.encode(), stdout=subprocess.PIPE).stdout

def convert_lua(code, format):
    return lua_conv(code, format)

#### Zlib Stuff
def check_zlib(data, offset):
    (a,b) = struct.unpack_from("<BB", data, offset)
    if (a * 0x100 + b) % 0x1f == 0:
        if (a & 0xf) == 8:
            if (a >> 4) + 8 <= 0xf:
                return True
    return False

def decomp_zlib(data):
    d = zlib.decompressobj()
    ret = d.decompress(data)
    del d
    return ret

def comp_zlib(data):
    return zlib.compress(data)


#### Texture Stuff
def decomp_dxt5(buffer, height, width):
    size = width * height // 16

    vals = np.array([*buffer], dtype=np.ubyte).reshape(-1, 16)
    
    alpha_vals = np.zeros((size, 8))
    alpha_vals[:, :2] = vals[:, :2] / 255
    ainds = alpha_vals[:, 0] > alpha_vals[:, 1]
    alpha_vals[ainds, 2:] = (alpha_vals[ainds][:, [[0,1], [0,1], [0,1], [0,1], [0,1], [0,1]]] * np.array([[6,1], [5,2], [4,3], [3,4], [2,5], [1,6]])[None]).sum(2) / 7
    alpha_vals[~ainds, 2:6] = (alpha_vals[~ainds][:, [[0,1], [0,1], [0,1], [0,1]]] * np.array([[4,1], [3, 2], [2,3], [1,4]])[None]).sum(2) / 5
    alpha_vals[~ainds, 6] = 0
    alpha_vals[~ainds, 7] = 1.0
    
    alpha_tables = np.transpose(np.squeeze(np.packbits(
        np.unpackbits(
            vals[:, 2:8], bitorder='little'
        ).reshape(-1, 16, 3), 2, 'little'
    )).reshape(-1, 4, 4), [1,2,0])
    
    color_vals = np.zeros((size, 4, 3))
    raw_colors = np.unpackbits(
        vals[:, 8:12], bitorder='little'
    ).reshape(-1, 2, 16)
    
    color_vals[:, :2, 2] = np.squeeze(np.packbits(raw_colors[:, :, :5], 2, 'little')) / 0x1f
    color_vals[:, :2, 1] = np.squeeze(np.packbits(raw_colors[:, :, 5:11], 2, 'little')) / 0x3f
    color_vals[:, :2, 0] = np.squeeze(np.packbits(raw_colors[:, :, 11:], 2, 'little')) / 0x1f
    
    cinds = (vals[:, 8].astype(np.uint32) | vals[:, 9].astype(np.uint32) << 8) > (vals[:, 10].astype(np.uint32) | vals[:, 11].astype(np.uint32) << 8)
    color_vals[cinds, 2:] = (color_vals[cinds][:, [[0,1], [0,1]]] * np.array([[2,1], [1,2]])[None,:,:,None]).sum(2) / 3
    color_vals[~cinds, 2] = (color_vals[~cinds, 0, :] + color_vals[~cinds, 1, :])/2
    color_vals[~cinds, 3] = 0
    
    color_tables = np.transpose(np.squeeze(np.packbits(
        np.unpackbits(
            vals[:, 12:], bitorder='little'
        ).reshape(-1, 16, 2), 2, 'little'
    )).reshape(-1, 4, 4), [1,2,0])
    
    img_vals = np.concatenate([color_vals[np.arange(size), color_tables], alpha_vals[np.arange(size), alpha_tables][:,:,:,None]], -1)
    
    img = np.concatenate([
        np.concatenate([img_vals[:,:,i + j * width//4, :] for i in range(width//4)], 1) for j in range(height//4)
    ], 0)
    return img

def decomp_dxt1(buffer, height, width):
    size = width * height // 16
    
    vals = np.array([*buffer], dtype=np.ubyte).reshape(-1, 8)
        
    color_vals = np.zeros((size, 4, 3))
    raw_colors = np.unpackbits(
        vals[:, :4], bitorder='little'
    ).reshape(-1, 2, 16)
    
    color_vals[:, :2, 2] = np.squeeze(np.packbits(raw_colors[:, :, :5], 2, 'little')) / 0x1f
    color_vals[:, :2, 1] = np.squeeze(np.packbits(raw_colors[:, :, 5:11], 2, 'little')) / 0x3f
    color_vals[:, :2, 0] = np.squeeze(np.packbits(raw_colors[:, :, 11:], 2, 'little')) / 0x1f
    
    cinds = (vals[:, 0].astype(np.uint32) | vals[:, 1].astype(np.uint32) << 8) > (vals[:, 2].astype(np.uint32) | vals[:, 3].astype(np.uint32) << 8)
    color_vals[cinds, 2:] = (color_vals[cinds][:, [[0,1], [0,1]]] * np.array([[2,1], [1,2]])[None,:,:,None]).sum(2) / 3
    color_vals[~cinds, 2] = (color_vals[~cinds, 0, :] + color_vals[~cinds, 1, :])/2
    color_vals[~cinds, 3] = 0
    
    color_tables = np.transpose(np.squeeze(np.packbits(
        np.unpackbits(
            vals[:, 4:], bitorder='little'
        ).reshape(-1, 16, 2), 2, 'little'
    )).reshape(-1, 4, 4), [1,2,0])
    
    img_vals = color_vals[np.arange(size), color_tables]
    
    img = np.concatenate([
        np.concatenate([img_vals[:,:,i + j * width//4, :] for i in range(width//4)], 1) for j in range(height//4)
    ], 0)
    return img

def decomp_bc4(buffer, height, width):
    size = width * height // 16

    vals = np.array([*buffer], dtype=np.ubyte).reshape(-1, 8)
    
    color_vals = np.zeros((size, 8))
    color_vals[:, :2] = vals[:, :2] / 255
    cind = color_vals[:, 0] > color_vals[:, 1]
    color_vals[cind, 2:] = (color_vals[cind][:, [[0,1], [0,1], [0,1], [0,1], [0,1], [0,1]]] * np.array([[6,1], [5,2], [4,3], [3,4], [2,5], [1,6]])[None]).sum(2) / 7
    color_vals[~cind, 2:6] = (color_vals[~cind][:, [[0,1], [0,1], [0,1], [0,1]]] * np.array([[4,1], [3, 2], [2,3], [1,4]])[None]).sum(2) / 5
    color_vals[~cind, 6] = 0
    color_vals[~cind, 7] = 1.0
    
    color_tables = np.transpose(np.squeeze(np.packbits(
        np.unpackbits(
            vals[:, 2:8], bitorder='little'
        ).reshape(-1, 16, 3), 2, 'little'
    )).reshape(-1, 4, 4), [1,2,0])
    
    img_vals = color_vals[np.arange(size), color_tables]
    
    img = np.concatenate([
        np.concatenate([img_vals[:,:,i + j * width//4] for i in range(width//4)], 1) for j in range(height//4)
    ], 0)
    return img

def decomp_cubemap(buffer, height, width, format=1):
    imgs = []
    size = len(buffer)//6
    for i in range(6):
        if format==1:
            imgs.append(decomp_dxt1(buffer[i*size:i*size+size], height, width))
        if format==5:
            imgs.append(decomp_dxt5(buffer[i*size:i*size+size], height, width))
    return imgs

def XGAddress2DTiledX(Offset, Width, TexelPitch):
    # https://github.com/NCDyson/RareView/blob/master/RareView/Texture.cs
    AlignedWidth = (Width + 31) & ~31;

    LogBpp = (TexelPitch >> 2) + ((TexelPitch >> 1) >> (TexelPitch >> 2));
    OffsetB = Offset << LogBpp;
    OffsetT = ((OffsetB & ~4095) >> 3) + ((OffsetB & 1792) >> 2) + (OffsetB & 63);
    OffsetM = OffsetT >> (7 + LogBpp);

    MacroX = ((OffsetM % (AlignedWidth >> 5)) << 2);
    Tile = ((((OffsetT >> (5 + LogBpp)) & 2) + (OffsetB >> 6)) & 3);
    Macro = (MacroX + Tile) << 3;
    Micro = ((((OffsetT >> 1) & ~15) + (OffsetT & 15)) & ((TexelPitch << 3) - 1)) >> LogBpp;

    return Macro + Micro;

def XGAddress2DTiledY(Offset, Width, TexelPitch):
    # https://github.com/NCDyson/RareView/blob/master/RareView/Texture.cs
    AlignedWidth = (Width + 31) & ~31;

    LogBpp = (TexelPitch >> 2) + ((TexelPitch >> 1) >> (TexelPitch >> 2));
    OffsetB = Offset << LogBpp;
    OffsetT = ((OffsetB & ~4095) >> 3) + ((OffsetB & 1792) >> 2) + (OffsetB & 63);
    OffsetM = OffsetT >> (7 + LogBpp);

    MacroY = ((OffsetM // (AlignedWidth >> 5)) << 2);
    Tile = ((OffsetT >> (6 + LogBpp)) & 1) + (((OffsetB & 2048) >> 10));
    Macro = (MacroY + Tile) << 3;
    Micro = ((((OffsetT & (((TexelPitch << 6) - 1) & ~31)) + ((OffsetT & 15) << 1)) >> (3 + LogBpp)) & ~1);

    return Macro + Micro + ((OffsetT & 16) >> 4);
    
ConvIndexStorage = {}

def conv_img(buffer, height, width, f, get_buffer=True):
    # https://github.com/NCDyson/RareView/blob/master/RareView/Texture.cs
    data = np.frombuffer(buffer, 'b')
    if f in [10, 0xb, 0xc, 0x11]:
        data = data.reshape(-1, 2)[:, ::-1].flatten()
        s = 4
        d = 16
    elif f in [7,8,13]:
        data = data.reshape(-1, 2)[:, ::-1].flatten()
        s = 4
        d = 8
    elif f == 3:
        data = data.reshape(-1, 4)[:, ::-1].flatten()
        s = 1
        d = 4
    h = height // s
    w = width // s
    h_ = h
    w_ = w
    if f in [7, 8, 13, 10, 0xb, 0xc, 0x11]:
        h = max(h, 32)
        w = max(w, 32)
    
    if (inds := ConvIndexStorage.get((h,w,d), None)) is None:
        inds = np.zeros((h,w), dtype='i')
        for j in range(h):
            for i in range(w):
                offset = j * w + i
                x = XGAddress2DTiledX(offset, w, d)
                y = XGAddress2DTiledY(offset, w, d)
                inds[y,x] = offset
        ConvIndexStorage[(h,w,d)] = inds
    if get_buffer:
        return data.reshape(-1,d)[inds][:h_, :w_].tobytes()
    else:
        return data.reshape(-1,d)[inds][:h_, :w_]

#### Utils

def read_strings(data, offset, num, format="<"):
    strings = []
    for i in range(num):
        k = struct.unpack_from(format+"I", data, offset)[0]
        offset += 4
        strings.append(data[offset:offset+k])
        offset +=  k
    return strings

def write_strings(data, offset, strings, format="<"):
    for s in strings:
        k = len(s)
        struct.pack_into(format+"I", data, offset, k)
        offset += 4
        data[offset:offset+k] = s
        offset +=  k

def pack_strings(strings, format="<"):
    data = bytes()
    for s in strings:
        k = len(s)
        data += struct.pack(format+"I", k) + s
    return data

hashing_array = [
    0x00000000, 0x04c11db7, 0x09823b6e, 0x0d4326d9, 0x130476dc, 0x17c56b6b, 0x1a864db2, 0x1e475005, 
    0x2608edb8, 0x22c9f00f, 0x2f8ad6d6, 0x2b4bcb61, 0x350c9b64, 0x31cd86d3, 0x3c8ea00a, 0x384fbdbd, 
    0x4c11db70, 0x48d0c6c7, 0x4593e01e, 0x4152fda9, 0x5f15adac, 0x5bd4b01b, 0x569796c2, 0x52568b75, 
    0x6a1936c8, 0x6ed82b7f, 0x639b0da6, 0x675a1011, 0x791d4014, 0x7ddc5da3, 0x709f7b7a, 0x745e66cd, 
    0x9823b6e0, 0x9ce2ab57, 0x91a18d8e, 0x95609039, 0x8b27c03c, 0x8fe6dd8b, 0x82a5fb52, 0x8664e6e5, 
    0xbe2b5b58, 0xbaea46ef, 0xb7a96036, 0xb3687d81, 0xad2f2d84, 0xa9ee3033, 0xa4ad16ea, 0xa06c0b5d, 
    0xd4326d90, 0xd0f37027, 0xddb056fe, 0xd9714b49, 0xc7361b4c, 0xc3f706fb, 0xceb42022, 0xca753d95, 
    0xf23a8028, 0xf6fb9d9f, 0xfbb8bb46, 0xff79a6f1, 0xe13ef6f4, 0xe5ffeb43, 0xe8bccd9a, 0xec7dd02d, 
    0x34867077, 0x30476dc0, 0x3d044b19, 0x39c556ae, 0x278206ab, 0x23431b1c, 0x2e003dc5, 0x2ac12072, 
    0x128e9dcf, 0x164f8078, 0x1b0ca6a1, 0x1fcdbb16, 0x018aeb13, 0x054bf6a4, 0x0808d07d, 0x0cc9cdca, 
    0x7897ab07, 0x7c56b6b0, 0x71159069, 0x75d48dde, 0x6b93dddb, 0x6f52c06c, 0x6211e6b5, 0x66d0fb02, 
    0x5e9f46bf, 0x5a5e5b08, 0x571d7dd1, 0x53dc6066, 0x4d9b3063, 0x495a2dd4, 0x44190b0d, 0x40d816ba, 
    0xaca5c697, 0xa864db20, 0xa527fdf9, 0xa1e6e04e, 0xbfa1b04b, 0xbb60adfc, 0xb6238b25, 0xb2e29692, 
    0x8aad2b2f, 0x8e6c3698, 0x832f1041, 0x87ee0df6, 0x99a95df3, 0x9d684044, 0x902b669d, 0x94ea7b2a, 
    0xe0b41de7, 0xe4750050, 0xe9362689, 0xedf73b3e, 0xf3b06b3b, 0xf771768c, 0xfa325055, 0xfef34de2, 
    0xc6bcf05f, 0xc27dede8, 0xcf3ecb31, 0xcbffd686, 0xd5b88683, 0xd1799b34, 0xdc3abded, 0xd8fba05a, 
    0x690ce0ee, 0x6dcdfd59, 0x608edb80, 0x644fc637, 0x7a089632, 0x7ec98b85, 0x738aad5c, 0x774bb0eb, 
    0x4f040d56, 0x4bc510e1, 0x46863638, 0x42472b8f, 0x5c007b8a, 0x58c1663d, 0x558240e4, 0x51435d53, 
    0x251d3b9e, 0x21dc2629, 0x2c9f00f0, 0x285e1d47, 0x36194d42, 0x32d850f5, 0x3f9b762c, 0x3b5a6b9b, 
    0x0315d626, 0x07d4cb91, 0x0a97ed48, 0x0e56f0ff, 0x1011a0fa, 0x14d0bd4d, 0x19939b94, 0x1d528623, 
    0xf12f560e, 0xf5ee4bb9, 0xf8ad6d60, 0xfc6c70d7, 0xe22b20d2, 0xe6ea3d65, 0xeba91bbc, 0xef68060b, 
    0xd727bbb6, 0xd3e6a601, 0xdea580d8, 0xda649d6f, 0xc423cd6a, 0xc0e2d0dd, 0xcda1f604, 0xc960ebb3, 
    0xbd3e8d7e, 0xb9ff90c9, 0xb4bcb610, 0xb07daba7, 0xae3afba2, 0xaafbe615, 0xa7b8c0cc, 0xa379dd7b, 
    0x9b3660c6, 0x9ff77d71, 0x92b45ba8, 0x9675461f, 0x8832161a, 0x8cf30bad, 0x81b02d74, 0x857130c3, 
    0x5d8a9099, 0x594b8d2e, 0x5408abf7, 0x50c9b640, 0x4e8ee645, 0x4a4ffbf2, 0x470cdd2b, 0x43cdc09c, 
    0x7b827d21, 0x7f436096, 0x7200464f, 0x76c15bf8, 0x68860bfd, 0x6c47164a, 0x61043093, 0x65c52d24, 
    0x119b4be9, 0x155a565e, 0x18197087, 0x1cd86d30, 0x029f3d35, 0x065e2082, 0x0b1d065b, 0x0fdc1bec, 
    0x3793a651, 0x3352bbe6, 0x3e119d3f, 0x3ad08088, 0x2497d08d, 0x2056cd3a, 0x2d15ebe3, 0x29d4f654, 
    0xc5a92679, 0xc1683bce, 0xcc2b1d17, 0xc8ea00a0, 0xd6ad50a5, 0xd26c4d12, 0xdf2f6bcb, 0xdbee767c, 
    0xe3a1cbc1, 0xe760d676, 0xea23f0af, 0xeee2ed18, 0xf0a5bd1d, 0xf464a0aa, 0xf9278673, 0xfde69bc4, 
    0x89b8fd09, 0x8d79e0be, 0x803ac667, 0x84fbdbd0, 0x9abc8bd5, 0x9e7d9662, 0x933eb0bb, 0x97ffad0c, 
    0xafb010b1, 0xab710d06, 0xa6322bdf, 0xa2f33668, 0xbcb4666d, 0xb8757bda, 0xb5365d03, 0xb1f740b4 
]

index_array = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 
    0x40, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 
    0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 
    0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 
    0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 
    0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 
    0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 
    0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 
    0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 
    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff 
]

def hash_string(string, mask=0):
    h = ~np.uint32(mask)
    if isinstance(string, str):
        string = string.encode('utf-8')
    for val in string:
        h = ((h << np.uint32(8))) ^ np.uint32(hashing_array[np.uint32(index_array[val])^(h>>np.uint32(24))])
    h = ~h
    return h


### Level Editing Stuff

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
                strings.append(data[off:off+l].decode())
                off += l
        off += 1
    return strings

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