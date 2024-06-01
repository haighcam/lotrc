use std::{any::TypeId, collections::HashMap, fs::{self, File}, iter::{repeat, zip}, mem::size_of};
use log::warn;
use serde_json::{Value, json, to_vec_pretty, Map};
use zerocopy::{AsBytes, ByteOrder, FromBytes, BE, F32, LE, U16, U32, U64};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::sync::Mutex;
use std::io::prelude::*;

use super::lua_stuff::LuaCompiler;

use lotrc_rs_proc::OrderedData;
pub trait OrderedData where Self: Sized + Clone + Default {
    type LE: Into<Self> + From<Self> + FromBytes + AsBytes + Clone;
    type BE: Into<Self> + From<Self> + FromBytes + AsBytes + Clone;
    #[inline]
    fn from_bytes<O: ByteOrder + 'static>(data: &[u8]) -> Self {
        if TypeId::of::<O>() == TypeId::of::<LE>() {
            Self::LE::read_from_prefix(data).unwrap().into()
        } else {
            Self::BE::read_from_prefix(data).unwrap().into()
        }
    }
    #[inline]
    fn to_bytes<O: ByteOrder + 'static>(&self, data: &mut [u8]) {
        if TypeId::of::<O>() == TypeId::of::<LE>() {
            Self::LE::write_to_prefix(&self.clone().into(), data).unwrap();
        } else {
            Self::BE::write_to_prefix(&self.clone().into(), data).unwrap();
        }
    }
    #[inline]
    fn dump_bytes<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        if TypeId::of::<O>() == TypeId::of::<LE>() {
            Self::LE::as_bytes(&self.clone().into()).to_vec()
        } else {
            Self::BE::as_bytes(&self.clone().into()).to_vec()
        }
    }
    #[inline]
    fn size<O: ByteOrder + 'static>() -> usize {
        if TypeId::of::<O>() == TypeId::of::<LE>() {
            size_of::<Self::LE>()
        } else {
            size_of::<Self::BE>()
        }
    }
    // fn from_data(data: &[u8], offset: usize) -> Self;
}
pub trait OrderedDataVec {
    fn from_bytes<O: ByteOrder + 'static>(data: &[u8], num: usize) -> Self;
    fn to_bytes<O: ByteOrder + 'static>(&self, data: &mut [u8]);
    fn dump_bytes<O: ByteOrder + 'static>(&self) -> Vec<u8>;
    fn size<O: ByteOrder + 'static>(&self) -> usize;
}

impl <T> OrderedDataVec for Vec<T> where T: OrderedData {
    #[inline]
    fn from_bytes<O: ByteOrder + 'static>(data: &[u8], num: usize) -> Self {
        if TypeId::of::<O>() == TypeId::of::<LE>() {
            T::LE::slice_from_prefix(data, num).unwrap().0.iter().cloned().map(|x| x.into()).collect()
        } else {
            T::BE::slice_from_prefix(data, num).unwrap().0.iter().cloned().map(|x| x.into()).collect()
        }
    }
    #[inline]
    fn to_bytes<O: ByteOrder + 'static>(&self, data: &mut [u8]) {
        if TypeId::of::<O>() == TypeId::of::<LE>() {
            self.iter().cloned().map(|x| T::LE::from(x)).collect::<Vec<_>>().as_slice().write_to_prefix(data).unwrap()
        } else {
            self.iter().cloned().map(|x| T::BE::from(x)).collect::<Vec<_>>().as_slice().write_to_prefix(data).unwrap()
        }
    }
    #[inline]
    fn dump_bytes<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        if TypeId::of::<O>() == TypeId::of::<LE>() {
            self.iter().cloned().flat_map(|x| T::LE::from(x).as_bytes().iter().cloned().collect::<Vec<_>>()).collect()
        } else {
            self.iter().cloned().flat_map(|x| T::BE::from(x).as_bytes().iter().cloned().collect::<Vec<_>>()).collect()
        }
    }
    #[inline]
    fn size<O: ByteOrder + 'static>(&self) -> usize {
        self.len() * T::size::<O>()
    }
}

impl OrderedData for f32 { type LE = F32<LE>; type BE = F32<BE>; }
impl OrderedData for u64 { type LE = U64<LE>; type BE = U64<BE>; }
impl OrderedData for u32 { type LE = U32<LE>; type BE = U32<BE>; }
impl OrderedData for u16 { type LE = U16<LE>; type BE = U16<BE>; }
impl OrderedData for u8 { type LE = u8; type BE = u8;}

const HASHING_ARRAY: [u32; 256] = [
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
];

const INDEX_ARRAY: [usize; 256] = [
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
];

pub const fn hash_string(string: &[u8], mask: Option<u32>) -> u32 {
    let mut h = !match mask {
        Some(val) => val,
        None => 0,
    };
    let mut i: usize = 0;
    loop {
        if i >= string.len() { break; }
        h = (h << 8) ^ HASHING_ARRAY[INDEX_ARRAY[string[i] as usize] ^ (h >> 24) as usize];
        i += 1;
    }
    !h
}

lazy_static::lazy_static! {
    pub static ref STRING_LOOKUP: Mutex<HashMap<u32, String>> = {
        const CONQUEST_STRINGS: &str = include_str!("../res/conquest_strings.txt");
        Mutex::new(CONQUEST_STRINGS.split('\n').map(|x| (hash_string(x.as_bytes(), None), String::from(x))).collect())
    };

    pub static ref DECOMP_LUA: Mutex<bool> = Mutex::new(true);
}

pub fn update_strings(vals: &[String]) {
    STRING_LOOKUP.lock().unwrap().extend(vals.iter().map(|x| (hash_string(x.as_bytes(), None), x.clone())));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Crc {
    Str(Box<str>),
    Key(u32)
}

impl Crc {
    pub fn key(&self) -> u32 {
        match self {
            Self::Key(val) => *val,
            Self::Str(val) => hash_string(val.as_bytes(), None)
        }
    }

    pub fn str(&self) -> Option<&str> {
        match self {
            Self::Key(_) => None,
            Self::Str(val) => Some(val)
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Key(val) => format!("0x{val:08X}"),
            Self::Str(val) => val.to_string()
        }
    }

    pub fn from_string(val: &str) -> Self {
        if val.starts_with("0x") {
            Self::Key(u32::from_str_radix(&val[2..], 16).unwrap())
        } else {
            Self::Str(val.into())
        }
    }

}

impl PartialEq for Crc {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl std::hash::Hash for Crc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}

impl Eq for Crc {}

impl Default for Crc {
    fn default() -> Self {
        Self::Key(0)
    }
}

impl<O: ByteOrder> From<U32<O>> for Crc {
    fn from(value: U32<O>) -> Self {
        let val: u32 = value.into();
        match STRING_LOOKUP.lock().ok().and_then(|m| m.get(&val).map(|x| x.clone().into_boxed_str())) {
            Some(str) => Self::Str(str),
            None => Self::Key(val)
        }
    }
}

impl<O: ByteOrder> From<Crc> for U32<O> {
    fn from(value: Crc) -> Self {
        match value {
            Crc::Str(val) => hash_string(val.as_bytes(), None),
            Crc::Key(val) => val,
        }.into()
    }
}

impl OrderedData for Crc { type LE = U32<LE>; type BE = U32<BE>; }

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Strings {
    pub strings: Vec<String>,
}

impl Strings {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, num: usize) -> Self {
        let mut strings = Vec::with_capacity(num);
        let mut offset = offset;
        for _ in 0..num {
            let k = u32::from_bytes::<O>(&data[offset..]) as usize;
            offset += 4;
            strings.push(String::from_utf8_lossy(&data[offset..offset+k]).to_string());
            offset += k;
        }
        Self { strings, ..Default::default() }
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize) {
        let mut offset = offset;
        for string in &self.strings {
            (string.len() as u32).to_bytes::<O>(&mut data[offset..]);
            offset += 4;
            data[offset..(offset+string.len())].copy_from_slice(string.as_bytes());
            offset += string.len();
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        self.strings.iter().flat_map(|string| {
            (string.len() as u32).dump_bytes::<O>().into_iter().chain(string.as_bytes().iter().cloned()).collect::<Vec<_>>()
        }).collect()
    }

    pub fn size(&self) -> usize {
        self.strings.iter().map(|x| x.len()).sum::<usize>() + 4 * self.strings.len()
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        fs::write(path.as_ref().with_extension("json"), serde_json::to_string_pretty(&json!(self.strings)).unwrap()).unwrap();
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let vals = serde_json::from_slice::<Value>(&fs::read(path.as_ref().with_extension("json")).unwrap()).unwrap();
        let strings = vals.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect::<Vec<_>>();
        Self { strings }
    }

}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CompressedBlock {
    pub data: Vec<u8>,
}

impl CompressedBlock {
    pub fn from_data(data: &[u8], size: usize, size_comp: usize, offset: usize) -> Self {
        let data = match size_comp {
            0 => data[offset..offset + size].to_vec(),
            _ => {
                let mut out = Vec::with_capacity(size);
                ZlibDecoder::new(&data[offset..offset+size_comp]).read_to_end(&mut out).unwrap();
                out
            }
        };
        Self { data }
    }

    pub fn into_data(&self, data: &mut [u8], offset: usize) {
        if self.data.is_empty() { return }
        let mut z = ZlibEncoder::new(&mut data[offset..], flate2::Compression::default());
        z.write_all(self.data.as_slice()).unwrap();
        z.finish().unwrap();
    }

    pub fn dump(&self) -> Vec<u8> {
        if self.data.is_empty() { return Vec::new(); }
        let mut z = ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
        z.write_all(self.data.as_slice()).unwrap();
        z.finish().unwrap()
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        std::fs::write(path, &self.data).unwrap();
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let data: Vec<u8> = std::fs::read(path).unwrap();
        Self { data }
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct List {
    pub num: u16,
    pub offset: u16,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Matrix4x4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Bool {
    pub val: u8,
    pub _pad1: u8,
    pub _pad2: u8,
    pub _pad3: u8,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Weight {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Node {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BaseTypes {
    CRC(Crc),
    GUID(u32),
    Color(u32),
    Vector2(Vector2),
    Vector3(Vector3),
    Vector4(Vector4),
    Matrix4x4(Matrix4x4),
    Float(f32),
    Int(u32),
    Bool(Bool),
    String(String),
    StringList(Vec<String>),
    ObjectList(Vec<u32>),
    NodeList(Vec<Node>),
    IntList(Vec<u32>),
    CRCList(Vec<Crc>),
    WeightList(Vec<Weight>),
    MatrixList(Vec<Matrix4x4>),
    Byte(u8),
}

impl BaseTypes {
    pub const CRC_KEY: u32 = hash_string("CRC".as_bytes(), None);
    pub const GUID_KEY: u32 = hash_string("GUID".as_bytes(), None);
    pub const COLOR_KEY: u32 = hash_string("Color".as_bytes(), None);
    pub const VECTOR2_KEY: u32 = hash_string("Vector2".as_bytes(), None);
    pub const VECTOR3_KEY: u32 = hash_string("Vector3".as_bytes(), None);
    pub const VECTOR4_KEY: u32 = hash_string("Vector4".as_bytes(), None);
    pub const MATRIX4X4_KEY: u32 = hash_string("Matrix4x4".as_bytes(), None);
    pub const FLOAT_KEY: u32 = hash_string("Float".as_bytes(), None);
    pub const INT_KEY: u32 = hash_string("Int".as_bytes(), None);
    pub const BOOL_KEY: u32 = hash_string("Bool".as_bytes(), None);
    pub const BYTE_KEY: u32 = hash_string("Byte".as_bytes(), None);
    pub const STRING_KEY: u32 = hash_string("String".as_bytes(), None);
    pub const STRINGLIST_KEY: u32 = hash_string("StringList".as_bytes(), None);
    pub const OBJECTLIST_KEY: u32 = hash_string("ObjectList".as_bytes(), None);
    pub const NODELIST_KEY: u32 = hash_string("NodeList".as_bytes(), None);
    pub const INTLISTS_KEY: u32 = hash_string("IntList".as_bytes(), None);
    pub const CRCLIST_KEY: u32 = hash_string("CRCList".as_bytes(), None);
    pub const WEIGHTLIST_KEY: u32 = hash_string("WeightList".as_bytes(), None);
    pub const MATRIXLIST_KEY: u32 = hash_string("MatrixList".as_bytes(), None);
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], kind: u32) -> Self {
        match kind {
            Self::CRC_KEY => Self::CRC(OrderedData::from_bytes::<O>(data)),
            Self::GUID_KEY => Self::GUID(OrderedData::from_bytes::<O>(data)),
            Self::COLOR_KEY => Self::Color(OrderedData::from_bytes::<O>(data)),
            Self::VECTOR2_KEY => Self::Vector2(OrderedData::from_bytes::<O>(data)),
            Self::VECTOR3_KEY => Self::Vector3(OrderedData::from_bytes::<O>(data)),
            Self::VECTOR4_KEY => Self::Vector4(OrderedData::from_bytes::<O>(data)),
            Self::MATRIX4X4_KEY => Self::Matrix4x4(OrderedData::from_bytes::<O>(data)),
            Self::FLOAT_KEY => Self::Float(OrderedData::from_bytes::<O>(data)),
            Self::INT_KEY  => Self::Int(OrderedData::from_bytes::<O>(data)),
            Self::BOOL_KEY => Self::Bool(OrderedData::from_bytes::<O>(data)),
            Self::BYTE_KEY => Self::Byte(OrderedData::from_bytes::<O>(data)),
            Self::STRING_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data);
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize);
                Self::String(String::from_utf8(vals).unwrap())
            },
            Self::STRINGLIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data);
                let vals: Vec<List> = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize);
                let valss = vals.iter().enumerate().map(|(i, v)| { String::from_utf8(
                    OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>() * (i + 2) + v.offset as usize..], v.num as usize)
                ).unwrap()}).collect();
                Self::StringList(valss)
            },
            Self::OBJECTLIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data);
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize);
                Self::ObjectList(vals)
            },
            Self::NODELIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data);
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize);
                Self::NodeList(vals)
            },
            Self::INTLISTS_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data);
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize);
                Self::IntList(vals)
            },
            Self::CRCLIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data);
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize);
                Self::CRCList(vals)
            },
            Self::WEIGHTLIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data);
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize);
                Self::WeightList(vals)
            },
            Self::MATRIXLIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data);
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize);
                Self::MatrixList(vals)
            },
            _ => panic!("Unkown Type {:?}", kind)
        }
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], off: &mut usize) {
        match self {
            Self::CRC(val) => val.to_bytes::<O>(data),
            Self::GUID(val) => val.to_bytes::<O>(data),
            Self::Color(val) => val.to_bytes::<O>(data),
            Self::Vector2(val) => val.to_bytes::<O>(data),
            Self::Vector3(val) => val.to_bytes::<O>(data),
            Self::Vector4(val) => val.to_bytes::<O>(data),
            Self::Matrix4x4(val) => val.to_bytes::<O>(data),
            Self::Float(val) => val.to_bytes::<O>(data),
            Self::Int(val) => val.to_bytes::<O>(data),
            Self::Bool(val) => val.to_bytes::<O>(data),
            Self::Byte(val) => val.to_bytes::<O>(data),
            Self::String(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data);
                data[*off..*off + vals.len()].copy_from_slice(vals.as_bytes());
                if vals.len() != 0 { *off += vals.len() + 1 };
            },
            Self::StringList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data);
                let mut off_ = *off;
                *off += vals.len() * List::size::<O>();
                for v in vals {
                    List{ num: v.len() as u16, offset: (*off - off_ - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off_..]);
                    data[*off..*off + v.len()].copy_from_slice(v.as_bytes());                    
                    if v.len() != 0 {
                        *off += v.len() + 1;
                    }
                    off_ += 4;
                }
            },
            Self::ObjectList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data);
                vals.to_bytes::<O>(&mut data[*off..]);
                *off += vals.size::<O>();
            },
            Self:: NodeList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data);
                vals.to_bytes::<O>(&mut data[*off..]);
                *off += vals.size::<O>();
            },
            Self::IntList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data);
                vals.to_bytes::<O>(&mut data[*off..]);
                *off += vals.size::<O>();
            },
            Self::CRCList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data);
                vals.to_bytes::<O>(&mut data[*off..]);
                *off += vals.size::<O>();
            },
            Self::WeightList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data);
                vals.to_bytes::<O>(&mut data[*off..]);
                *off += vals.size::<O>();
            },
            Self::MatrixList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data);
                vals.to_bytes::<O>(&mut data[*off..]);
                *off += vals.size::<O>();
            },
        }
    }

    pub fn off_size<O: ByteOrder + 'static>(&self) -> usize {
        match self {
            Self::String(vals) => {
                if vals.len() != 0 { vals.len() + 1 } else { 0 }
            },
            Self::StringList(vals) => {
                let mut s = vals.len() * List::size::<O>();
                for v in vals {
                    if v.len() != 0 {
                        s += v.len() + 1;
                    }
                }
                s
            },
            Self::ObjectList(vals) => {
                vals.size::<O>()
            },
            Self:: NodeList(vals) => {
                vals.size::<O>()
            },
            Self::IntList(vals) => {
                vals.size::<O>()
            },
            Self::CRCList(vals) => {
                vals.size::<O>()
            },
            Self::WeightList(vals) => {
                vals.size::<O>()
            },
            Self::MatrixList(vals) => {
                vals.size::<O>()
            },
            _ => 0,
        }
    }

    pub fn size<O: ByteOrder + 'static>(&self) -> usize {
        match self {
            Self::CRC(val) => u32::size::<O>(),
            Self::GUID(val) => u32::size::<O>(),
            Self::Color(val) => u32::size::<O>(),
            Self::Vector2(val) => Vector2::size::<O>(),
            Self::Vector3(val) => Vector3::size::<O>(),
            Self::Vector4(val) => Vector4::size::<O>(),
            Self::Matrix4x4(val) => Matrix4x4::size::<O>(),
            Self::Float(val) => f32::size::<O>(),
            Self::Int(val) => u32::size::<O>(),
            Self::Bool(val) => Bool::size::<O>(),
            Self::Byte(val) => u8::size::<O>(),
            Self::String(val, ..) => List::size::<O>(),
            Self::StringList(val, ..) => List::size::<O>(),
            Self::ObjectList(val, ..) => List::size::<O>(),
            Self:: NodeList(val, ..) => List::size::<O>(),
            Self::IntList(val, ..) => List::size::<O>(),
            Self::CRCList(val, ..) => List::size::<O>(),
            Self::WeightList(val, ..) => List::size::<O>(),
            Self::MatrixList(val, ..) => List::size::<O>(),
        }
    }

    pub fn to_json(&self) -> Value {
        match self {
            Self::CRC(val) => json!(val.to_string()),
            Self::GUID(val) => json!(val),
            Self::Color(val) => json!(val),
            Self::Vector2(val) => json!([val.x, val.y]),
            Self::Vector3(val) => json!([val.x, val.y, val.z]),
            Self::Vector4(val) => json!([val.x, val.y, val.z, val.w]),
            Self::Matrix4x4(val) => json!([
                val.x.x, val.x.y, val.x.z, val.x.w,
                val.y.x, val.y.y, val.y.z, val.y.w,
                val.z.x, val.z.y, val.z.z, val.z.w,
                val.w.x, val.w.y, val.w.z, val.w.w,
            ]),
            Self::Float(val) => json!(val),
            Self::Int(val) => json!(val),
            Self::Bool(val) => json!(val.val != 0),
            Self::Byte(val) => json!(val),
            Self::String(vals) => json!(vals),
            Self::StringList(vals) => json!(vals.iter().map(|x| json!(x)).collect::<Vec<_>>()),
            Self::ObjectList(vals) => json!(vals.iter().map(|x| json!(x)).collect::<Vec<_>>()),
            Self::NodeList(vals) => json!(vals.iter().map(|x| json!([x.x, x.y, x.z, x.w])).collect::<Vec<_>>()),
            Self::IntList(vals) => json!(vals.iter().map(|x| json!(x)).collect::<Vec<_>>()),
            Self::CRCList(vals) => json!(vals.iter().map(|x| json!(x.to_string())).collect::<Vec<_>>()),
            Self::WeightList(vals) => json!(vals.iter().map(|x| json!([x.x, x.y])).collect::<Vec<_>>()),
            Self::MatrixList(vals) => json!(vals.iter().map(|val| json!([
                val.x.x, val.x.y, val.x.z, val.x.w,
                val.y.x, val.y.y, val.y.z, val.y.w,
                val.z.x, val.z.y, val.z.z, val.z.w,
                val.w.x, val.w.y, val.w.z, val.w.w,
            ])).collect::<Vec<_>>()),
        }
    }

    pub fn from_json(val: &Value, kind: u32) -> Self {
        match kind {
            Self::CRC_KEY => Self::CRC(Crc::from_string(val.as_str().unwrap())),
            Self::GUID_KEY => Self::GUID(val.as_u64().unwrap() as u32),
            Self::COLOR_KEY => Self::Color(val.as_u64().unwrap() as u32),
            Self::VECTOR2_KEY => Self::Vector2({
                let vals = val.as_array().unwrap().into_iter().map(|x| x.as_f64().unwrap() as f32).collect::<Vec<_>>();
                Vector2 { x: vals[0], y: vals[1] }
            }),
            Self::VECTOR3_KEY => Self::Vector3({
                let vals = val.as_array().unwrap().into_iter().map(|x| x.as_f64().unwrap() as f32).collect::<Vec<_>>();
                Vector3 { x: vals[0], y: vals[1], z: vals[2] }
            }),
            Self::VECTOR4_KEY => Self::Vector4({
                let vals = val.as_array().unwrap().into_iter().map(|x| x.as_f64().unwrap() as f32).collect::<Vec<_>>();
                Vector4 { x: vals[0], y: vals[1], z: vals[2], w: vals[4] }
            }),
            Self::MATRIX4X4_KEY => Self::Matrix4x4({
                let vals = val.as_array().unwrap().into_iter().map(|x| x.as_f64().unwrap() as f32).collect::<Vec<_>>();
                Matrix4x4 { 
                    x: Vector4 { x: vals[0], y: vals[1], z: vals[2], w: vals[3]}, 
                    y: Vector4 { x: vals[4], y: vals[5], z: vals[6], w: vals[7]}, 
                    z: Vector4 { x: vals[8], y: vals[9], z: vals[10], w: vals[11]}, 
                    w: Vector4 { x: vals[12], y: vals[13], z: vals[14], w: vals[15]}, 
                }
            }),
            Self::FLOAT_KEY => Self::Float(val.as_f64().unwrap() as f32),
            Self::INT_KEY  => Self::Int(val.as_u64().unwrap() as u32),
            Self::BOOL_KEY => Self::Bool(Bool { val: val.as_bool().unwrap() as u8, _pad1: 0, _pad2: 0, _pad3: 0 }),
            Self::BYTE_KEY => Self::Byte(val.as_u64().unwrap() as u8),
            Self::STRING_KEY => Self::String(val.as_str().unwrap().into()),
            Self::STRINGLIST_KEY => Self::StringList(val.as_array().unwrap().into_iter().map(|x| x.as_str().unwrap().into()).collect()),
            Self::OBJECTLIST_KEY => Self::ObjectList(val.as_array().unwrap().into_iter().map(|x| x.as_u64().unwrap() as u32).collect()),
            Self::NODELIST_KEY => Self::NodeList(val.as_array().unwrap().into_iter().map(|x| {
                let vals = x.as_array().unwrap().into_iter().map(|x| x.as_u64().unwrap() as u32).collect::<Vec<_>>();
                Node { x: vals[0], y: vals[1], z: vals[2], w: vals[3] }
            }).collect()),
            Self::INTLISTS_KEY => Self::IntList(val.as_array().unwrap().into_iter().map(|x| x.as_u64().unwrap() as u32).collect()),
            Self::CRCLIST_KEY => Self::CRCList(val.as_array().unwrap().into_iter().map(|x| Crc::from_string(x.as_str().unwrap())).collect()),
            Self::WEIGHTLIST_KEY => Self::WeightList(val.as_array().unwrap().into_iter().map(|x| {
                let vals = x.as_array().unwrap().into_iter().map(|x| x.as_u64().unwrap() as u32).collect::<Vec<_>>();
                Weight { x: vals[0], y: vals[1]}
            }).collect()),
            Self::MATRIXLIST_KEY => Self::MatrixList(val.as_array().unwrap().into_iter().map(|x| {
                let vals = x.as_array().unwrap().into_iter().map(|x| x.as_f64().unwrap() as f32).collect::<Vec<_>>();
                Matrix4x4 {
                    x: Vector4 { x: vals[0], y: vals[1], z: vals[2], w: vals[3]}, 
                    y: Vector4 { x: vals[4], y: vals[5], z: vals[6], w: vals[7]}, 
                    z: Vector4 { x: vals[8], y: vals[9], z: vals[10], w: vals[11]}, 
                    w: Vector4 { x: vals[12], y: vals[13], z: vals[14], w: vals[15]}, 
                }
            }).collect()),
            _ => panic!("Unkown Type {:?}", kind)
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubBlock {
    LangStrings(LangStrings),
    Data(Data),
    Spray(Spray),
    Crowd(Crowd),
    GameObjs(GameObjs),
    AtlasUV(AtlasUV),
    Lua(Lua),
}

impl SubBlock {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], info: &SubBlocksBlockHeader, lua: &LuaCompiler) -> Self {
        match info.key.key() {
            LangStrings::KEY_POLISH | LangStrings::KEY_GERMAN | LangStrings::KEY_FRENCH | LangStrings::KEY_SPANISH | LangStrings::KEY_RUSSIAN | LangStrings::KEY_SWEDISH | LangStrings::KEY_ENGLISH | LangStrings::KEY_ITALIAN | LangStrings::KEY_NORWEGIAN => 
                SubBlock::LangStrings(LangStrings::from_data::<O>(data, info.offset as usize, info.size as usize)),
            Data::KEY_PFIELDS => SubBlock::Data(Data::from_data(data, info.offset as usize, info.size as usize)),
            Spray::KEY => SubBlock::Spray(Spray::from_data::<O>(data, info.offset as usize, info.size as usize)),
            Crowd::KEY => SubBlock::Crowd(Crowd::from_data::<O>(data, info.offset as usize, info.size as usize)),
            GameObjs::KEY => SubBlock::GameObjs(GameObjs::from_data::<O>(data, info.offset as usize, info.size as usize)),
            AtlasUV::KEY1 | AtlasUV::KEY2 => SubBlock::AtlasUV(AtlasUV::from_data::<O>(data, info.offset as usize, info.size as usize)),
            _ => match info.key.str() {
                Some(x) if x.ends_with(".lua") => SubBlock::Lua(Lua::from_data(data, info.offset as usize, info.size as usize, lua, x.to_string())),
                Some(x) if x.ends_with(".csv") || x.ends_with(".txt") || x.ends_with(".dat") => 
                    SubBlock::Data(Data::from_data(data, info.offset as usize, info.size as usize)),
                _ =>  {
                    warn!("Unknown block type {:?}", info.key);
                    SubBlock::Data(Data::from_data(data, info.offset as usize, info.size as usize))
                }    
            }
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self, lua: &LuaCompiler) -> Vec<u8> {
        match self {
            SubBlock::LangStrings(val) => val.dump::<O>(),
            SubBlock::Data(val) => val.dump(),
            SubBlock::Spray(val) => val.dump::<O>(),
            SubBlock::Crowd(val) => val.dump::<O>(),
            SubBlock::GameObjs(val) => val.dump::<O>(),
            SubBlock::AtlasUV(val) => val.dump::<O>(),
            SubBlock::Lua(val) => val.dump(lua),
        }
    }

    pub fn size<O: ByteOrder + 'static>(&self) -> usize {
        match self {
            SubBlock::LangStrings(val) => val.size(),
            SubBlock::Data(val) => val.data.len(),
            SubBlock::Spray(val) => val.size,
            SubBlock::Crowd(val) => val.size,
            SubBlock::GameObjs(val) => val.size,
            SubBlock::AtlasUV(val) => val.vals.size::<O>(),
            SubBlock::Lua(val) => val.data.len(),
        }
    }


    pub fn to_file<P: AsRef<Path>>(&self, path: P, keys: &StringKeys) {
        match self {
            SubBlock::LangStrings(val) => val.to_file(path, keys),
            SubBlock::Data(val) => val.to_file(path),
            SubBlock::Spray(val) => val.to_file(path),
            SubBlock::Crowd(val) => val.to_file(path),
            SubBlock::GameObjs(val) => val.to_file(path),
            SubBlock::AtlasUV(val) => val.to_file(path),
            SubBlock::Lua(val) => val.to_file(path),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P, info: &SubBlocksBlockHeader, lua: &LuaCompiler) -> Self {
        match info.key.key() {
            LangStrings::KEY_POLISH | LangStrings::KEY_GERMAN | LangStrings::KEY_FRENCH | LangStrings::KEY_SPANISH | LangStrings::KEY_RUSSIAN | LangStrings::KEY_SWEDISH | LangStrings::KEY_ENGLISH | LangStrings::KEY_ITALIAN | LangStrings::KEY_NORWEGIAN => 
                SubBlock::LangStrings(LangStrings::from_file(path)),
            Data::KEY_PFIELDS => SubBlock::Data(Data::from_file(path)),
            Spray::KEY => SubBlock::Spray(Spray::from_file(path)),
            Crowd::KEY => SubBlock::Crowd(Crowd::from_file(path)),
            GameObjs::KEY => SubBlock::GameObjs(GameObjs::from_file(path)),
            AtlasUV::KEY1 | AtlasUV::KEY2 => SubBlock::AtlasUV(AtlasUV::from_file(path)),
            _ => match info.key.str() {
                Some(x) if x.ends_with(".lua") => SubBlock::Lua(Lua::from_file(path, lua)),
                Some(x) if x.ends_with(".csv") || x.ends_with(".txt") || x.ends_with(".dat") => 
                    SubBlock::Data(Data::from_file(path)),
                _ =>  {
                    warn!("Unknown block type {:?}", info.key);
                    SubBlock::Data(Data::from_file(path))
                }    
            }
        }
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct SubBlocksHeader {
    pub z0: u32,
    pub block_num: u32,
    pub z2: u32,
    pub z3: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct SubBlocksBlockHeader {
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SubBlocks {
    pub header: SubBlocksHeader,
    pub block_headers: Vec<SubBlocksBlockHeader>,
    #[serde(skip)]
    pub blocks: Vec<SubBlock>,
}

impl SubBlocks {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, lua: &LuaCompiler) -> Self {
        let mut val = Self::default();
        val.header = OrderedData::from_bytes::<O>(&data[offset..]);
        val.block_headers = OrderedDataVec::from_bytes::<O>(&data[offset+SubBlocksHeader::size::<O>()..], val.header.block_num as usize);
        for info in val.block_headers.iter() {
            val.blocks.push(SubBlock::from_data::<O>(&data[offset..], info, lua));
        }
        val
    }

    pub fn size<O: ByteOrder + 'static>(&self) -> usize {
        let mut s = SubBlocksHeader::size::<O>() + self.block_headers.size::<O>();
        for block in &self.blocks {
            s = (s + 16) & 0xFFFFFFF0;
            s += block.size::<O>();
        }
        s = (s + 16) & 0xFFFFFFF0;
        return s
    }

    pub fn dump<O: ByteOrder + 'static>(&self, lua: &LuaCompiler) -> Vec<u8> {
        let mut block_headers = self.block_headers.clone();
        let mut offset = SubBlocksHeader::size::<O>() + block_headers.size::<O>();
        let data = zip(&self.blocks, &mut block_headers).flat_map(|(block, header)| {
            let block_data = block.dump::<O>(lua);
            let off = (offset + 16) & 0xfffffff0;
            let start = vec![0u8; off - offset];
            offset = off;
            header.offset = offset as u32;
            header.size = block_data.len() as u32;
            offset += block_data.len();
            start.into_iter().chain(block_data)
        }).collect::<Vec<_>>();
        let extra = ((offset + 16) & 0xFFFFFFF0) - offset;
        self.header.dump_bytes::<O>().into_iter().chain(block_headers.dump_bytes::<O>().into_iter()).chain(data.into_iter()).chain(repeat(0u8).take(extra)).collect()
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P, keys: &StringKeys) {
        let path = path.as_ref();
        std::fs::create_dir(path).ok();
        fs::write(path.join("index.json"), serde_json::to_string_pretty(self).unwrap()).unwrap();
        for (block, info) in zip(&self.blocks, &self.block_headers) {
            block.to_file(path.join(info.key.str().unwrap()), keys)
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P, lua: &LuaCompiler) -> Self {
        let path = path.as_ref();
        let mut val = serde_json::from_slice::<Self>(&fs::read(path.join("index.json")).unwrap()).unwrap();
        val.blocks = val.block_headers.iter().map(|info| SubBlock::from_file(path.join(info.key.str().unwrap()), info, lua)).collect();
        val
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct StringKeysHeader {
    pub numA: u16,
    pub numB: u16,
    pub z2: u32,
    pub z3: u32,
    pub z4: u32,
    pub z5: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct StringKeysVal {
    pub key: Crc,
    pub offset: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StringKeys {
    pub header: StringKeysHeader,
    pub vals: Vec<StringKeysVal>,
    pub pad: Vec<u32>,
}

impl StringKeys {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize) -> Self {
        let mut offset: usize = offset;
        let header: StringKeysHeader = OrderedData::from_bytes::<O>(&data[offset..]);
        assert!(header.numA == header.numB, "Seems to be true");
        offset += StringKeysHeader::size::<O>();
        let vals: Vec<StringKeysVal> = OrderedDataVec::from_bytes::<O>(&data[offset..], header.numA as usize);
        offset += vals.size::<O>();
        let pad = OrderedDataVec::from_bytes::<O>(&data[offset..], header.numA as usize);
        Self { header, vals, pad }
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize) {
        let mut offset = offset;
        self.header.to_bytes::<O>(&mut data[offset..]);
        offset += StringKeysHeader::size::<O>();
        self.vals.to_bytes::<O>(&mut data[offset..]);
        offset += self.vals.size::<O>();
        self.pad.to_bytes::<O>(&mut data[offset..]);
    }

    pub fn size<O: ByteOrder + 'static>(&self) -> usize {
        StringKeysHeader::size::<O>() + self.vals.size::<O>() + self.pad.size::<O>()
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        self.header.dump_bytes::<O>().into_iter().chain(self.vals.dump_bytes::<O>().into_iter()).chain(self.pad.dump_bytes::<O>().into_iter()).collect()
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        fs::write(path.as_ref().with_extension("json"), serde_json::to_string_pretty(&json!(self.vals.iter().map(|x| x.key.to_string()).collect::<Vec<_>>())).unwrap()).unwrap();

        // fs::write(path.as_ref().with_extension("json"), serde_json::to_string_pretty(self).unwrap()).unwrap();
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let vals = serde_json::from_slice::<Value>(&fs::read(path.as_ref().with_extension("json")).unwrap()).unwrap();
        let keys = vals.as_array().unwrap().iter().map(|val| Crc::from_string(val.as_str().unwrap())).collect::<Vec<_>>();
        let header = StringKeysHeader {
            numA: keys.len() as u16,
            numB: keys.len() as u16,
            z2: 0,
            z3: 0,
            z4: 0,
            z5: 0,
        };
        let pad = vec![0u32; keys.len()];
        let mut off = StringKeysHeader::size::<LE>() + keys.len() * StringKeysVal::size::<LE>();
        let vals = keys.into_iter().map(|key| {
            let val = StringKeysVal { key, offset: off as u32};
            off += 4;
            val
        }).collect::<Vec<_>>();
        Self { header, vals, pad }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LangStrings {
    pub strings: Vec<String>,
}

impl LangStrings {
    pub const KEY_POLISH: u32 = hash_string("Polish".as_bytes(), None);
    pub const KEY_GERMAN: u32 = hash_string("German".as_bytes(), None);
    pub const KEY_FRENCH: u32 = hash_string("French".as_bytes(), None);
    pub const KEY_SPANISH: u32 = hash_string("Spanish".as_bytes(), None);
    pub const KEY_RUSSIAN: u32 = hash_string("Russian".as_bytes(), None);
    pub const KEY_SWEDISH: u32 = hash_string("Swedish".as_bytes(), None);
    pub const KEY_ENGLISH: u32 = hash_string("English".as_bytes(), None);
    pub const KEY_ITALIAN: u32 = hash_string("Italian".as_bytes(), None);
    pub const KEY_NORWEGIAN: u32 = hash_string("Norwegian".as_bytes(), None);

    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, size: usize) -> Self {
        let mut val = Self::default();
        let mut offset_ = offset;
        while offset_ < size + offset {
            let start = offset_;
            while data[offset_] != 0 || data[offset_+1] != 0 {
                offset_ += 2;
            }
            let string: Vec<u16> = OrderedDataVec::from_bytes::<O>(&data[start..offset_], (offset_-start)/2);
            val.strings.push(String::from_utf16(string.as_slice()).unwrap());
            offset_ += 2;
        }
        val
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut[u8], offset: usize) {
        let mut offset = offset;
        for string in self.strings.iter() {
            let s = string.encode_utf16().collect::<Vec<_>>();
            s.to_bytes::<O>(&mut data[offset..]);
            offset += s.len() * 2 + 2;
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        let vals = self.strings.iter().flat_map(|x| x.encode_utf16().chain([0u16])).collect::<Vec<_>>();
        vals.dump_bytes::<O>()
    }

    pub fn size(&self) -> usize {
        self.strings.iter().map(|x| x.encode_utf16().map(|_| 2).sum::<usize>() + 2).sum::<usize>()
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P, keys: &StringKeys) {
        let vals = zip(&keys.vals, &self.strings).map(|(key, string)| (key.key.to_string(), json!(string))).collect::<Map<_,_>>();
        fs::write(path.as_ref().with_extension("json"), serde_json::to_string_pretty(&vals).unwrap()).unwrap();
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let vals = serde_json::from_slice::<Value>(&fs::read(path.as_ref().with_extension("json")).unwrap()).unwrap();
        let strings = vals.as_object().unwrap().iter().map(|(_, s)| s.as_str().unwrap().to_string()).collect::<Vec<_>>();
        Self { strings }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Lua {
    pub name: String,
    pub data: Vec<u8>,
    pub code: String,
}

impl Lua {
    pub fn from_data(data: &[u8], offset: usize, size: usize, lua: &LuaCompiler, name: String) -> Self {
        let data = data[offset..offset+size].to_vec();
        let code = if *DECOMP_LUA.lock().unwrap() {
            lua.decomp(data.as_slice()).unwrap()
        } else {
            String::new()
        };
        Self { code, data, name }
    }

    pub fn dump(&self, lua: &LuaCompiler) -> Vec<u8> {
        if *DECOMP_LUA.lock().unwrap() {
            match lua.compile(&self.code, &self.name) {
                Ok(val) => val,
                Err(e) => {
                    println!("{:?}", self.name);
                    println!("{}", self.code);
                    panic!("{:?}", e)
                }
            }
        } else {
            lua.convert(&self.data, "L4404").unwrap()
        }
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        if *DECOMP_LUA.lock().unwrap(){
            std::fs::write(path, self.code.as_bytes()).unwrap();
        } else {
            std::fs::write(path, &self.data).unwrap();
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P, lua: &LuaCompiler) -> Self {
        let path = path.as_ref();
        let name: String = path.file_name().unwrap().to_str().unwrap().into();
        let val = std::fs::read(path).unwrap();
        let (data, code) = if (val[0] == 0x1bu8) && (val[1] == 76) && (val[2] == 117) && (val[3] == 97) {
            let code = lua.decomp(&val).unwrap();
            (val, code)
        } else {
            let code = String::from_utf8(val).unwrap();
            let data = lua.compile(&code, &name).unwrap();
            (data, code)
        };
        
        Self { code, name, data }
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct GameObjsHeader {
    pub const_: u32,
    pub types_num: u32,
    pub types_offset: u32,
    pub obj_num: u32,
    pub obj_offset: u32,
    pub z5: u32,
    pub z6: u32,
    pub z7: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct GameObjsTypeHeader {
    pub key: Crc,
    pub size: u32,
    pub fields: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct GameObjsTypeField {
    pub key: Crc,
    pub kind: Crc,
    pub offset: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct GameObjsObjHeader {
    pub unk0: u32,
    pub key: Crc,
    pub size: u16,
    pub z3: u16,
    pub z4: u32,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameObjs {
    pub size: usize,
    pub header: GameObjsHeader,
    pub types: Vec<GameObjsTypeHeader>,
    pub type_fields: Vec<Vec<GameObjsTypeField>>,
    pub type_field_lookup: HashMap<u32, usize>,
    pub obj_headers: Vec<GameObjsObjHeader>,
    pub objs: Vec<Vec<BaseTypes>>,
}

impl GameObjs {
    pub const KEY: u32 = hash_string("Level".as_bytes(), None);
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, size: usize) -> Self {
        let mut val = Self::default();
        val.size = size;
        val.header = OrderedData::from_bytes::<O>(&data[offset..]);
        {
            let mut offset = offset + val.header.types_offset as usize;
            for _ in 0..val.header.types_num {
                let obj: GameObjsTypeHeader = OrderedData::from_bytes::<O>(&data[offset..]);
                offset += GameObjsTypeHeader::size::<O>();
                let type_fields: Vec<GameObjsTypeField> = OrderedDataVec::from_bytes::<O>(&data[offset..], obj.size as usize);
                offset += type_fields.size::<O>();
                val.type_field_lookup.insert(obj.key.key(), val.type_fields.len());
                val.types.push(obj);
                val.type_fields.push(type_fields);
            }
        }
        {
            let mut offset = offset + val.header.obj_offset as usize;
            for _ in 0..val.header.obj_num {
                let obj: GameObjsObjHeader = OrderedData::from_bytes::<O>(&data[offset..]);
                offset += GameObjsObjHeader::size::<O>();
                let ts = val.type_fields.get(*val.type_field_lookup.get(&obj.key.key()).unwrap()).unwrap();
                let mut vals = Vec::with_capacity(ts.len());
                for t in ts.iter() {
                    let val = BaseTypes::from_data::<O>(&data[offset + t.offset as usize..], t.kind.key());
                    vals.push(val)
                }
                offset += obj.size as usize;
                val.objs.push(vals);
                val.obj_headers.push(obj);
            }
        }
        val
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut[u8], offset: usize) {
        self.header.to_bytes::<O>(&mut data[offset..]);
        {
            let mut offset = offset + self.header.types_offset as usize;
            for (obj, type_fields) in zip(self.types.iter(), self.type_fields.iter()) {
                obj.to_bytes::<O>(&mut data[offset..]);
                offset += GameObjsTypeHeader::size::<O>();
                type_fields.to_bytes::<O>(&mut data[offset..]);
                offset += type_fields.size::<O>();
            }
        }
        {
            let mut offset = offset + self.header.obj_offset as usize;
            for (obj, fields) in zip(self.obj_headers.iter(), self.objs.iter()) {
                obj.to_bytes::<O>(&mut data[offset..]);
                let ts = self.type_fields.get(*self.type_field_lookup.get(&obj.key.key()).unwrap()).unwrap();
                offset += GameObjsObjHeader::size::<O>();
                let mut off = zip(fields, ts).map(|(t, f)| f.offset as usize + t.size::<O>()).fold(0, usize::max);
                off = (off + 15) & 0xFFFFFFF0;
                for (val, t) in zip(fields, ts) {
                    off = off - t.offset as usize;
                    val.into_data::<O>(&mut data[offset + t.offset as usize..], &mut off);
                    off = off + t.offset as usize;
                    if t.key.key() == BaseTypes::INTLISTS_KEY {
                        off = (off + 15) & 0xFFFFFFF0;
                    }
                }
                offset += obj.size as usize;
            }
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        let mut data = vec![0u8; self.size];
        self.into_data::<O>(data.as_mut_slice(), 0);
        data
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        let val = json!({
            "types": zip(&self.types,& self.type_fields).map(|(t, fs)| {
                json!({
                    "name": t.key.to_string(),
                    "size": t.size,
                    "fields": fs.iter().map(|f| {
                        json!({
                            "name": f.key.to_string(),
                            "type": f.kind.to_string(),
                            "offset": f.offset
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>(),
            "objs": zip(&self.obj_headers,& self.objs).map(|(o, fs)| {
                let ts = &self.type_fields[*self.type_field_lookup.get(&o.key.key()).unwrap()];
                let mut order: Vec<_> = (0..ts.len()).collect();
                order.sort_by_key(|x| ts[*x].offset);
                json!({
                    "type": o.key.to_string(),
                    "unk0": o.unk0,
                    "fields": order.into_iter().map(|i| (ts[i].key.to_string(), fs[i].to_json())).collect::<Map<_,_>>()
                })
            }).collect::<Vec<_>>()
        });
        fs::write(path.as_ref().with_extension("json"), to_vec_pretty(&val).unwrap()).unwrap();
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let val = serde_json::from_slice::<Value>(&fs::read(path.as_ref().with_extension("json")).unwrap()).unwrap();
        let ts = val["types"].as_array().unwrap();
        let mut types = Vec::with_capacity(ts.len());
        let mut type_fields = Vec::with_capacity(ts.len());
        for t in ts {
            types.push(GameObjsTypeHeader {
                key: Crc::from_string(t["name"].as_str().unwrap()),
                size: t["size"].as_u64().unwrap() as u32,
                fields: 0
            });
            type_fields.push(t["fields"].as_array().unwrap().iter().map(|v| GameObjsTypeField {
                key: Crc::from_string(v["name"].as_str().unwrap()),
                kind: Crc::from_string(v["type"].as_str().unwrap()),
                offset: v["offset"].as_u64().unwrap() as u32,
            }).collect::<Vec<_>>());
        }

        let type_field_lookup = types.iter().enumerate().map(|(i, x)| (x.key.key(), i)).collect::<HashMap<_, _>>();
        let os = val["objs"].as_array().unwrap();
        let mut objs = Vec::with_capacity(os.len());
        let mut obj_headers = Vec::with_capacity(os.len());

        for o in os {
            let key = Crc::from_string(o["type"].as_str().unwrap());
            let ts = &type_fields[*type_field_lookup.get(&key.key()).unwrap()];
            let o_ = o["fields"].as_object().unwrap();
            let fields = ts.iter().map(|t| BaseTypes::from_json(&o_[&t.key.to_string()], t.kind.key())).collect::<Vec<_>>();
            let mut off = zip(&fields, ts).map(|(t, f)| f.offset as usize + t.size::<LE>()).fold(0, usize::max);
            off = (off + 15) & 0xFFFFFFF0;
            for (val, t) in zip(&fields, ts) {
                off += val.off_size::<LE>();
                if t.key.key() == BaseTypes::INTLISTS_KEY {
                    off = (off + 15) & 0xFFFFFFF0;
                }
            }
            let size = (off + 15) as u16 & 0xFFF0;
            obj_headers.push(GameObjsObjHeader {
                unk0: o["unk0"].as_u64().unwrap() as u32,
                key,
                size,
                z3: 0,
                z4: 0
            });
            objs.push(fields);
        }
        let header = GameObjsHeader {
            const_: 1296123652,
            types_num: types.len() as u32,
            types_offset: 32,
            obj_num: objs.len() as u32,
            obj_offset: ((types.len() * GameObjsTypeHeader::size::<LE>()) + (types.iter().map(|x| x.size as usize).sum::<usize>() * GameObjsTypeField::size::<LE>()) + 32 + 15) as u32 & 0xFFFFFFF0,
            z5: 0,
            z6: 0,
            z7: 0
        };
        let size = header.obj_offset as usize + obj_headers.iter().map(|x| x.size as usize).sum::<usize>() + objs.len() * GameObjsObjHeader::size::<LE>() ;
        Self {
            size,
            header,
            types,
            type_fields,
            type_field_lookup,
            obj_headers,
            objs,
        }
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
struct SprayObj1 {
    pub key: Crc,
    pub unk_1: u32,
    pub unk_2: u32,
    pub unk_3: u32,
    pub unk_4: u32,
    pub unk_5: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: u32,
    pub unk_12: u32,
    pub unk_13: u32,
    pub unk_14: u32,
    pub unk_15: u32,
    pub unk_16: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
struct SprayObj2 {
    pub unk_0: f32,
    pub unk_1: f32,
    pub unk_2: f32,
    pub unk_3: f32,
    pub unk_4: f32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Spray {
    pub size: usize,
    obj1_num: u32,
    obj2_num: u32,
    obj1s: Vec<SprayObj1>,
    obj2s: Vec<SprayObj2>,
}

impl Spray {
    pub const KEY: u32 = hash_string("Spray".as_bytes(), None);
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, size: usize) -> Self {
        let mut val = Self::default();
        val.size = size;
        let mut offset = offset;
        val.obj1_num = OrderedData::from_bytes::<O>(&data[offset..]);
        offset += u32::size::<O>();
        val.obj1s = OrderedDataVec::from_bytes::<O>(&data[offset..], val.obj1_num as usize);
        offset += val.obj1s.size::<O>();
        val.obj2_num = OrderedData::from_bytes::<O>(&data[offset..]);
        offset += u32::size::<O>();
        val.obj2s = OrderedDataVec::from_bytes::<O>(&data[offset..], val.obj2_num as usize);
        val
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut[u8], offset: usize) {
        let mut offset = offset;
        self.obj1_num.to_bytes::<O>(&mut data[offset..]);
        offset += u32::size::<O>();
        self.obj1s.to_bytes::<O>(&mut data[offset..]);
        offset += self.obj1s.size::<O>();
        self.obj2_num.to_bytes::<O>(&mut data[offset..]);
        offset += u32::size::<O>();
        self.obj2s.to_bytes::<O>(&mut data[offset..]);
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        let mut data = vec![0u8; self.size];
        self.into_data::<O>(data.as_mut_slice(), 0);
        data
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        fs::write(path.as_ref().with_extension("json"), serde_json::to_string_pretty(self).unwrap()).unwrap();
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        serde_json::from_slice(&fs::read(path.as_ref().with_extension("json")).unwrap()).unwrap()
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
struct CrowdHeader {
    pub key_0: u32,
    pub key_1: u32,
    pub key_2: u32,
    pub key_3: u32,
    pub unk_4: u32,
    pub keys_num: u32,
    pub num: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
struct CrowdVal {
    pub unk_0: u32,
    pub unk_1: u32,
    pub unk_2: u32,
    pub unk_3: u32,
    pub unk_4: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Crowd {
    pub size: usize,
    pub const_: u32,
    pub num: u32,
    pub offsets: Vec<u32>,
    pub headers: Vec<CrowdHeader>,
    pub keys: Vec<Vec<Crc>>,
    pub vals: Vec<Vec<CrowdVal>>,
}

impl Crowd {
    pub const KEY: u32 = hash_string("3dCrowd".as_bytes(), None);
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, size: usize) -> Self {
        let mut val = Self::default();
        val.size = size;
        val.const_ = OrderedData::from_bytes::<O>(&data[offset..]);
        assert!(val.const_ == 0x65, "Wrong Block Type");
        val.num = OrderedData::from_bytes::<O>(&data[offset + u32::size::<O>()..]);
        val.offsets = OrderedDataVec::from_bytes::<O>(&data[offset + u32::size::<O>() * 2..], val.num as usize);
        for offset_ in val.offsets.iter() {
            let mut offset = offset + *offset_ as usize;
            let header: CrowdHeader = OrderedData::from_bytes::<O>(&data[offset..]);
            offset += CrowdHeader::size::<O>();
            let keys: Vec<Crc> = OrderedDataVec::from_bytes::<O>(&data[offset..], header.keys_num as usize);
            offset += keys.size::<O>();
            let vals = OrderedDataVec::from_bytes::<O>(&data[offset..], header.keys_num as usize);
            val.headers.push(header);
            val.keys.push(keys);
            val.vals.push(vals);
        }
        val
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize) {
        self.const_.to_bytes::<O>(&mut data[offset..]);
        self.num.to_bytes::<O>(&mut data[offset + u32::size::<O>()..]);
        self.offsets.to_bytes::<O>(&mut data[offset + u32::size::<O>() * 2..]);
        for ((offset_, header), (keys, vals)) in zip(zip(self.offsets.iter(), self.headers.iter()), zip(self.keys.iter(), self.vals.iter())) {
            let mut offset = offset + *offset_ as usize;
            header.to_bytes::<O>(&mut data[offset..]);
            offset += CrowdHeader::size::<O>();
            keys.to_bytes::<O>(&mut data[offset..]);
            offset += keys.size::<O>();
            vals.to_bytes::<O>(&mut data[offset..]);
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        let mut data = vec![0u8; self.size];
        self.into_data::<O>(data.as_mut_slice(), 0);
        data
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        fs::write(path.as_ref().with_extension("json"), serde_json::to_string_pretty(self).unwrap()).unwrap();
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        serde_json::from_slice(&fs::read(path.as_ref().with_extension("json")).unwrap()).unwrap()
    }
}


#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct AtlasUVVal {
    pub key: Crc,
    pub vals: Vector4
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AtlasUV {
    pub vals: Vec<AtlasUVVal>,
}

impl AtlasUV {
    pub const KEY1: u32 = hash_string("atlas_1.uv".as_bytes(), None);
    pub const KEY2: u32 = hash_string("atlas_2.uv".as_bytes(), None);

    pub fn from_data<O: ByteOrder + 'static>(data: &[u8], offset: usize, size: usize) -> Self {
        assert!(size%AtlasUVVal::size::<O>() == 0, "Invalid UV Atlas size");
        let num = size / AtlasUVVal::size::<O>();
        let vals = OrderedDataVec::from_bytes::<O>(&data[offset..], num);

        Self { vals }
    }

    pub fn into_data<O: ByteOrder + 'static>(&self, data: &mut [u8], offset: usize) {
        self.vals.to_bytes::<O>(&mut data[offset..]);
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        self.vals.dump_bytes::<O>()
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        fs::write(path.as_ref().with_extension("json"), serde_json::to_string_pretty(self).unwrap()).unwrap();
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        serde_json::from_slice(&fs::read(path.as_ref().with_extension("json")).unwrap()).unwrap()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Data {
    pub data: Vec<u8>,
}

impl Data {
    pub const KEY_PFIELDS: u32 = hash_string("PFields".as_bytes(), None);
    // PFields has data from obj12, 2D something

    pub fn from_data(data: &[u8], offset: usize, size: usize) -> Self {
        Self {
            data: data[offset..offset+size].to_vec()
        }
    }

    pub fn into_data(&self, data: &mut[u8], offset: usize) {
        data[offset..offset+self.data.len()].copy_from_slice(self.data.as_slice());
    }

    pub fn dump(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        std::fs::write(path, &self.data).unwrap();
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let data = std::fs::read(path).unwrap();
        Self { data }
    }
}