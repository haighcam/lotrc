from .types import *
import xml.etree.ElementTree as ET
import struct
import io

def load_bnk(file):
    from wwiser.parser import wparser
    from wwiser.viewer import wdumper
    parser = wparser.Parser()
    parser.parse_bank(file)
    dumper = wdumper.DumpPrinter(parser.get_banks(), wdumper.TYPE_XSL, None)
    dumper._file = io.StringIO()
    dumper._formatted = True
    dumper._print_xml()
    text = dumper._file.getvalue()
    tree = ET.fromstring(text)[1][0]
    with open(file, "rb") as f:
        data = f.read()
    return data, tree

def get_wem(data, tree, get_fmt=False):
    for field in tree.find('.//object[@name="DataChunk"]'):
        if field.attrib['name'] == 'pData':
            offset = int(field.attrib['offset'], 16)
            
    blocks = {}
    for i, block in enumerate(tree.findall('.//object[@name="MediaHeader"]')):
        off = int(block.find('.//field[@name="uOffset"]').attrib['value'])
        size = int(block.find('.//field[@name="uSize"]').attrib['value'])
        id_ = int(block.find('.//field[@name="id"]').attrib['value'])
        blocks[id_] = data[offset+off:offset+off+size]
    if get_fmt:
        fmts = {}
        for a in tree.findall('.//object[@name="AkBankSourceData"]'):
            fmt = int(a.find('.//field[@name="ulPluginID"]').attrib['value'])
            audio_fmt = a.find('.//object[@name="AkAudioFormat"]')
            rate = int(audio_fmt.find('.//field[@name="uSampleRate"]').attrib['value'])
            bits = int(audio_fmt.find('.//field[@name="uFormatBits"]').attrib['value'])
            media_information = a.find('.//object[@name="AkMediaInformation"]')
            source_id = int(media_information.find('.//field[@name="sourceID"]').attrib['value'])
            fmts[source_id] = (fmt, rate, bits)

        return blocks, fmts
    else:
        return blocks
    
def flip_endian(data, tree, f='<'):
    fields = []
    q = [tree]
    while q != []:
        n = q.pop()
        for child in n:
            if child.tag == 'field':
                fields.append((child, n))
            q.append(child)

    dump_data = bytearray(len(data))
    for (field, parent) in fields:
        kind = field.attrib['type']
        val = field.attrib['value']
    
        if (addr := field.attrib.get('offset', None)) is not None:
            addr = int(addr,16)
            if kind == '4cc':
                dump_data[addr:addr+4] = val.encode()[2:-1]
            elif kind == 'f32':
                dump_data[addr:addr+4] = struct.pack(f+"f", float(val))
            # elif kind == 'gap' or kind == 'u32':
            elif kind == 'u32':
                dump_data[addr:addr+4] = struct.pack(f+"I", int(val))
            elif kind == 'sid' or kind == 'tid':
                dump_data[addr:addr+4] = struct.pack(f+"I", int(val))
            elif kind == 'u16':
                dump_data[addr:addr+2] = struct.pack(f+"H", int(val))
            elif kind == 'u8':
                dump_data[addr:addr+1] = struct.pack(f+"B", int(val))
            elif kind == 's32':
                dump_data[addr:addr+4] = struct.pack(f+"i", int(val))
            elif kind == 's16':
                dump_data[addr:addr+2] = struct.pack(f+"h", int(val))
            elif kind == 's8':
                dump_data[addr:addr+1] = struct.pack(f+"b", int(val))
            elif kind == 'str':
                b_str = val.encode()
                dump_data[addr:addr+len(b_str)] = b_str
            elif kind == 'gap' and field.attrib['name'] == '_reserved':
                dump_data[addr:addr+4] = struct.pack(f+"I", 1)
    
    dump_data[:4] = b"BKHD"
    
    for field in tree.find('.//object[@name="DataChunk"]'):
        if field.attrib['name'] == 'pData':
            size = int(field.attrib['value'])
            offset = int(field.attrib['offset'], 16)
    dump_data[offset:offset+size] = data[offset:offset+size]
    return bytes(dump_data)

def swap_audio(data, tree, new_audio, fmts):
    file_infos = []
    for a in tree.findall('.//object[@name="AkBankSourceData"]'):
        format_offset = int(a.find('.//field[@name="ulPluginID"]').attrib['offset'], 16)
        media_information = a.find('.//object[@name="AkMediaInformation"]')
        source_id = int(media_information.find('.//field[@name="sourceID"]').attrib['value'])
        file_id = int(media_information.find('.//field[@name="uFileID"]').attrib['value'])
        if source_id == file_id or file_id == 0:
            print(source_id)
            continue
        size_offset = int(media_information.find('.//field[@name="uInMemoryMediaSize"]').attrib['offset'], 16)
        offset_offset = int(media_information.find('.//field[@name="uFileOffset"]').attrib['offset'], 16)
        audio_fmt = a.find('.//object[@name="AkAudioFormat"]')
        rate_offset = int(audio_fmt.find('.//field[@name="uSampleRate"]').attrib['offset'], 16)
        bits_offset = int(audio_fmt.find('.//field[@name="uFormatBits"]').attrib['offset'], 16)
        file_infos.append((source_id, format_offset, size_offset, offset_offset, rate_offset, bits_offset))
    
    media_headers = {}
    data_alt = bytes()
    for media in tree.findall('.//object[@name="MediaHeader"]'):
        offset_offset = int(media.find('.//field[@name="uOffset"]').attrib['offset'], 16)
        size_offset = int(media.find('.//field[@name="uSize"]').attrib['offset'], 16)
        source_id = int(media.find('.//field[@name="id"]').attrib['value'])
        data_alt += bytes(((len(data_alt) + 15) & 0xFFFFFFF0) - len(data_alt))
        media_headers[source_id] = (offset_offset, size_offset, len(data_alt))
        data_alt += new_audio[source_id]
    
    data_chunk = tree.find('.//object[@name="DataChunk"]')
    data_offset = int(data_chunk.find('.//field[@name="pData"]').attrib['offset'], 16)
    data_size_offset = int(data_chunk.find('.//field[@name="dwChunkSize"]').attrib['offset'], 16)
    data_size = int(data_chunk.find('.//field[@name="pData"]').attrib['value'])
    
    data = bytearray(data)
    
    data[data_size_offset:data_size_offset+4] = struct.pack("I", len(data_alt))
    for source_id, (offset_offset, size_offset, new_off) in media_headers.items():
        struct.pack_into("I", data, offset_offset, new_off)
        struct.pack_into("I", data, size_offset, len(new_audio[source_id]))
    
    for source_id, format_offset, size_offset, offset_offset, rate_offset, bits_offset in file_infos:
        _, _, new_off = media_headers[source_id]
        struct.pack_into("I", data, offset_offset, new_off)
        struct.pack_into("I", data, size_offset, len(new_audio[source_id]))
        (fmt, rate, bits) = fmts[source_id]
        struct.pack_into("I", data, format_offset, fmt)
        struct.pack_into("I", data, rate_offset, rate)
        struct.pack_into("I", data, bits_offset, bits)
        # data[format_offset:format_offset+4] = struct.pack("I", 0x00020001) # ADPCM
        # data[format_offset:format_offset+4] = struct.pack("I", 0x00040001) # VORBIS
    data[data_offset:data_offset+data_size] = data_alt
    return data

class AudioTable:
    Header = structtuple("SoundTableself.header",
        "const0x2", "I",
        "n1", "I",                               
        "n2", "I",                               
        "n3", "I",                               
        "n4", "I",                               
        "n5", "I",                               
        "n6", "I",                               
        "n7", "I",                               
    )

    Obj1 = structtuple("SoundTableObj1", 
        "key", "I",                            
        "val", "I",                         
    )

    Obj2 = structtuple("SoundTableObj2", 
        "unk_0", "I",                            
        "unk_1", "I",                         
        "n", "I",                         
    )

    @classmethod
    def from_file(Self, path):
        with open(path, "rb") as f:
            data = f.read()
        return Self(data)

    def __init__(self, data):
        if data[0] == 2:
            self.f = '<'
        else:
            self.f = '>'

        self.header = unpack_from(self.Header[self.f], data, 0)
        offset = self.header.nbytes
        self.obj1s = unpack_list_from(self.Obj1[self.f], data, offset, self.header['n1'])
        offset += self.obj1s.nbytes
        self.obj2s = []
        for _ in range(self.header['n2']):
            obj2 = unpack_from(self.Obj2[self.f], data, offset)
            offset += obj2.nbytes
            obj2a = unpack_list_from(self.Obj1[self.f], data, offset, obj2['n'])
            offset += obj2a.nbytes
            self.obj2s.append((obj2, obj2a))

        self.obj3s = []
        for _ in range(self.header['n3']):
            obj2 = unpack_from(self.Obj2[self.f], data, offset)
            offset += obj2.nbytes
            obj2a = unpack_list_from(self.Obj1[self.f], data, offset, obj2['n'])
            offset += obj2a.nbytes
            self.obj3s.append((obj2, obj2a))

        self.obj4s = unpack_list_from(self.Obj1[self.f], data, offset, self.header['n4'])
        offset += self.obj4s.nbytes

        self.obj5s = unpack_list_from(self.Obj1[self.f], data, offset, self.header['n5'])
        offset += self.obj5s.nbytes

        self.obj6s = unpack_list_from(self.Obj1[self.f], data, offset, self.header['n6'])
        offset += self.obj6s.nbytes

        self.obj7s = unpack_list_from(self.Obj1[self.f], data, offset, self.header['n7'])
        offset += self.obj7s.nbytes

        n = (len(data) - offset) // 4
        self.extra = unpack_list_from(Uint[self.f], data, offset, n)

    def dump(self, f='<'):
        header = self.header.copy()
        header['n1'] = len(self.obj1s)
        header['n2'] = len(self.obj2s)
        header['n3'] = len(self.obj3s)
        header['n4'] = len(self.obj4s)
        header['n5'] = len(self.obj5s)
        header['n6'] = len(self.obj6s)
        header['n7'] = len(self.obj7s)

        data = pack(header, f)
        data += pack(self.obj1s, f)
        for obj2, obj2a in self.obj2s:
            data += pack(obj2, f)
            data += pack(obj2a, f)
        for obj2, obj2a in self.obj3s:
            data += pack(obj2, f)
            data += pack(obj2a, f)
        data += pack(self.obj4s, f)
        data += pack(self.obj5s, f)
        data += pack(self.obj6s, f)
        data += pack(self.obj7s, f)
        data += pack(self.extra, f)

        return data