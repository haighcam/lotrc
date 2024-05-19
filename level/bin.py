import warnings

from lotrc.utils import *
from lotrc.types import *

Header = structtuple("LevelBIN_Header",
    'constx06', 'I',
    'version', 'I',
    'strings_offset', 'I',
    'strings_size', 'I',
    'strings_num', 'I',
    'asset_handle_num', 'I',
    'asset_handle_offset', 'I',
    'unk_7', 'I',
    'unk_8', 'I',
    'unk_9', 'I',
    'unk_10', 'I',
    'unk_11', 'I',
    'unk_12', 'I',
    'unk_13', 'I',
    'unk_14', 'I',
    'unk_15', 'I',
    'unk_16', 'I',
    'unk_17', 'I',
    'unk_18', 'I',
    'unk_19', 'I',
    'unk_20', 'I',
    'unk_21', 'I',
    'unk_22', 'I',
    'unk_23', 'I',
    'unk_24', 'I',
    'unk_25', 'I',
    'unk_26', 'I',
    'unk_27', 'I',
    'unk_28', 'I',
    'unk_29', 'I',
    'unk_30', 'I',
    'unk_31', 'I',
    'unk_32', 'I',
    'unk_33', 'I',
    'unk_34', 'I',
    'unk_35', 'I',
    'unk_36', 'I',
    'unk_37', 'I',
    'unk_38', 'I',
    'unk_39', 'I',
    'unk_40', 'I',
    'unk_41', 'I',
    'unk_42', 'I',
)

AssetHandle = structtuple("AssetHandle",
    'key', 'I',
    'offset', 'I',
    'size', 'I',
    'size_comp', 'I',
    'type', 'I',
)

class Radiosity:
    Val = structtuple("RadiosityVal",
        "a", Vector4,
        "b", Vector4
    )
    # I have no idea, vertex data, maybe just 8 floats??
    @classmethod
    def unpack_from(Self, data, f="<"):
        # m = len(data)
        # assert m % 32 == 0
        # self = Self()
        # self.data = unpack_list_from(self.Val[f], data, 0, m // 32)
        # return self
        self = Self()
        self.data = data
        return self
    def dump(self, f="<"):
        # return pack(self.data, f)
        return self.data

"""
type:
    0,7,8 -> Texture
    1,9 -> CubeTexture
    2,10 -> VolumeTexture
    4 -> RenderTarget
    5 -> DepthStencilSurface
    3 -> Nothing
    11 -> Surface

format
    0 -> 0x17 R5G6B5
    1 -> 0x1a A4R4G4B4
    2 -> 0x19 A1R5G5B5
    3, 0x10, 0x12, 0x13, 0x26 -> 0x15 A8R8G8B8
    4 -> 0x16 X8R8G8B8
    defualt -> 0 UNKNOWN
    6 -> 0x1c A8
    7,8 -> 0x31545844 DXT1
    9 -> 0x33545844 DXT3
    10, 0xb, 0xc, 0x11 -> 0x35545844 DXT5
    0x15 -> 0x23 A2R10G10B10
    0x17 -> 0x24 A16B16G16R16
    0x18 -> 0x71 A16B16G16R16F 
    0x1b -> 0x4b D24S8
    0x1c -> 0x53 D24FS8
    0x1d -> 0x50 D16
    0x1e -> 0x5a574152 or 0x5a544e49 or 0x34324644, RAWZ or INTZ or DF24
    0x1f, 0x20 -> 0x32 L8
    0x21 -> 0x6f R16F 
    0x22 -> 0x70 G16R16F
    0x23 -> 0x72 R32F
    0x24 -> 0x73 G32R32F
    0x25 -> 0x74 A32B32G32R32F 
    0x27 -> 0x20 A8B8G8R8
    0x28 -> 0x4c4c554e or 0x15, Null or A8R8G8B8
    13 -> bc4 alpha texture (xbox only, converts to A8 texture on PC)
"""
def bin_mip(arr, w, h):
    return np.frombuffer(arr, np.ubyte).reshape(h//2, 2, w//2, 2)[:,0,:,0].tobytes()

class Texture:
    def __init__(self, data0, data1, info, f='<'):
        self.data0 = data0
        self.data1 = data1
        self.f = f
        data = self.data0 + self.data1
        self.sizes = np.ceil([(info['width'], info['height'])]/(2 ** np.arange(info['levels'])[:, None])).astype(int)
        self.format = info['format']
        self.type = info['asset_type']
        try:
            if self.format in [10, 0xb, 0xc, 0x11]:
                s = 4
                d = 16
            elif self.format in [7,8,13]:
                s = 4
                d = 8
            elif self.format == 3:
                if info['levels'] > 1:
                    raise ValueError("Not Supported")
                s = 1
                d = 4
            elif self.format == 6:
                s = 1
                d = 1
            else:
                warnings.warn(f"Unknown Texture Format {self.format}, treating as raw data. See this file for the actual format if you want to parse it")
                return
            block_sizes = np.maximum(self.sizes//s, 1)

            if f == '<':
                data_sizes = block_sizes[:, 0] * block_sizes[:, 1] * d
                offsets = [0] + np.cumsum(data_sizes[:-1]).tolist()
                self.levels = [data[offset:offset+size] for offset, size in zip(offsets, data_sizes)]
            elif f == '>':
                if info['levels'] == 1:
                    self.levels = [conv_img(data, self.sizes[0][1], self.sizes[0][0], self.format)]
                    if self.format == 13:
                        self.levels[-1] = decomp_bc4(self.levels[-1], self.sizes[0][1], self.sizes[0][0])
                else:
                    data_sizes = np.maximum(block_sizes[:, 0], 32) * np.maximum(block_sizes[:, 1], 32) * d
                    wide_img = info['width'] > info['height']
                    self.levels = []
                    offset = 0
                    # packed_inds = []
                    for i in range(info['levels']):
                        m,M = self.sizes[i].min(), self.sizes[i].max()
                        if m > 16:
                            self.levels.append(conv_img(data[offset:offset+data_sizes[i]], self.sizes[i][1], self.sizes[i][0], self.format))
                            offset += data_sizes[i]
                        elif m >= 4:
                            if m == 16:
                                if wide_img:
                                    packed_data = conv_img(data[offset:], self.sizes[i][1]*2, self.sizes[i][0], self.format, False)
                                else:
                                    packed_data = conv_img(data[offset:], self.sizes[i][1], self.sizes[i][0]*2, self.format, False)
                                    
                            off = m >> 2
                            if wide_img:
                                self.levels.append(packed_data[off:off+block_sizes[i][1], :block_sizes[i][0]].tobytes())
                            else:
                                self.levels.append(packed_data[:block_sizes[i][1], off:off+block_sizes[i][0]].tobytes())
                        
                        else:
                            off = M
                            if wide_img:
                                self.levels.append(packed_data[:block_sizes[i][1], off:off+block_sizes[i][0]].tobytes())
                            else:
                                self.levels.append(packed_data[off:off+block_sizes[i][1], :block_sizes[i][0]].tobytes())
                    self.packed_size = packed_data.shape
                    self.packed_data = packed_data.tobytes()
                if self.format == 13:
                    info['format'] = 6
                    self.format = 6 
                    for i in range(info['levels']):
                        self.levels[i] = (decomp_bc4(self.levels[i], max(self.sizes[i][1], 4), max(self.sizes[i][0], 4)) * 255).astype(np.ubyte).tobytes()
                    self.levels[-2] = bin_mip(self.levels[-3], self.sizes[-3][0], self.sizes[-3][1])
                    self.levels[-1] = bin_mip(self.levels[-2], self.sizes[-2][0], self.sizes[-2][1])
                    s = 1
            self.sizes = np.maximum(self.sizes, s)
        except Exception as e:
            warnings.warn(f"Could not parse texture {info['key']}")
            print(e)

    def dump(self, f='<'):
        if self.format not in [3, 7, 8, 10, 0xb, 0xc, 0x11, 6]:
            return self.data0, self.data1
        if f == '<':
            if len(self.levels) > 1:
                return self.levels[0], b''.join(self.levels[1:])
            else:
                return b'', self.levels[0]
        elif self.f == f:
            return self.data0, self.data1
        else:
            raise ValueError("Not yet implemented")

    def get_img(self, level=0):
        if self.format in [10, 0xb, 0xc, 0x11]:
            return decomp_dxt5(self.levels[level], self.sizes[level][1], self.sizes[level][0])
        elif self.format in [7,8]:
            return decomp_dxt1(self.levels[level], self.sizes[level][1], self.sizes[level][0])
        elif self.format == 3:
            return np.frombuffer(self.levels[level], 'B').reshape(self.sizes[level][1], self.sizes[level][0], 4)
        elif self.format == 6:
            return np.frombuffer(self.levels[level], 'B').reshape(self.sizes[level][1], self.sizes[level][0])

class CubeTexture:
    # only 1 level, to the * handle
    def __init__(self, data0, data1, info, f='<'):
        self.data = data1
        self.f = f
        self.format = info['format']
        self.type = info['asset_type']
        self.size = np.array((info['width'], info['height']))

        if self.format in [10, 0xb, 0xc, 0x11]:
            s = 4
            d = 16
        elif self.format in [7,8]:
            s = 4
            d = 8
        elif self.format == 3:
            s = 1
            d = 4
        else:
            raise ValueError(f"Unsupported Texture Format {self.format}")
        if info['levels'] > 1:
            raise ValueError("Not Supported")
        block_size = np.maximum(self.size//s, 1)
        if f == '<':
            data_size = block_size[0] * block_size[1] * d
            self.faces = [self.data[data_size*i:data_size*i+data_size] for i in range(6)]
        elif f == '>':
            data_size = max(block_size[0], 32) * max(block_size[1], 32) * d
            self.faces = [conv_img(self.data[data_size*i:data_size*i+data_size], self.size[1], self.size[0], self.format) for i in range(6)]

    def dump(self, f='<'):
        if f == '<':
            return b'', b''.join(self.faces)
        elif self.f == f:
            return b'', self.data
        else:
            raise ValueError("Not yet implemented")

    def get_img(self, face=0):
        if self.format in [10, 0xb, 0xc, 0x11]:
            return decomp_dxt5(self.faces[face], self.size[1], self.size[0])
        elif self.format in [7,8]:
            return decomp_dxt1(self.faces[face], self.size[1], self.size[0])
        elif self.format == 3:
            return np.frombuffer(self.faces[face], 'B').reshape(self.size[1], self.size[0], 4)