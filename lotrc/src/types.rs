use std::any::TypeId;
use std::{collections::HashMap, iter::zip, mem::size_of};
use flate2::Decompress;
use log::warn;
use serde_json::{Value, json, to_vec_pretty, from_slice, Map};
use zerocopy::{Immutable, KnownLayout, IntoBytes, Unaligned, ByteOrder, FromBytes, BE, F32, LE, U16, U32, U64, I32, I16};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::io::prelude::*;
use anyhow::{anyhow, Result};
use indicatif::ProgressBar;
use indexmap::IndexMap;
use pyo3::prelude::*;

use super::lua_stuff::LuaCompiler;
use super::read_write::{Reader, Writer, PathStuff};

use lotrc_proc::{OrderedData, basicpymethods, PyMethods};

pub trait PyMethods<'a> where Self: std::fmt::Debug + Sized + Deserialize<'a> {
    fn from_json(s: &'a str) -> Result<Self>;
    fn to_json(&self) -> Result<String>;
}

pub struct PC;
pub struct XBOX;
pub struct PS3;
pub trait Version {
    fn from_bytes<T: OrderedDataImpl>(data: &[u8]) -> Result<T>;
    fn to_bytes<T: OrderedDataImpl>(val: &T, data: &mut [u8]) -> Result<()>;
    fn dump_bytes<T: OrderedDataImpl>(val: &T) -> Vec<u8>;
    fn size<T: OrderedDataImpl>() -> usize;
    fn from_bytes_vec<T: OrderedDataImpl>(data: &[u8], num: usize) -> Result<Vec<T>>;
    fn to_bytes_vec<T: OrderedDataImpl>(val: &Vec<T>, data: &mut [u8]) -> Result<()>;
    fn dump_bytes_vec<T: OrderedDataImpl>(val: &Vec<T>) -> Vec<u8>;
    #[inline]
    fn size_vec<T: OrderedDataImpl>(val: &Vec<T>) -> usize {
        val.len() * Self::size::<T>()
    }
}

impl Version for PC {
    #[inline]
    fn from_bytes<T: OrderedData>(data: &[u8]) -> Result<T> {
        Ok(T::PC::read_from_prefix(data).map_err(|e| anyhow!("{}", e))?.0.into())
    }
    #[inline]
    fn to_bytes<T: OrderedData>(val: &T, data: &mut [u8]) -> Result<()> {
        T::PC::write_to_prefix(&val.clone().into(), data).map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }
    #[inline]
    fn dump_bytes<T: OrderedData>(val: &T) -> Vec<u8> {
        T::PC::as_bytes(&val.clone().into()).to_vec()
    }
    #[inline]
    fn size<T: OrderedData>() -> usize {
        size_of::<T::PC>()
    }
    #[inline]
    fn from_bytes_vec<T: OrderedData>(data: &[u8], num: usize) -> Result<Vec<T>> {
       Ok(<[T::PC]>::ref_from_prefix_with_elems(data, num).map_err(|e| anyhow!("{}", e))?.0.iter().map(|x| x.clone().into()).collect())
    }
    #[inline]
    fn to_bytes_vec<T: OrderedData>(val: &Vec<T>, data: &mut [u8]) -> Result<()> {
        val.iter().cloned().map(|x| T::PC::from(x)).collect::<Vec<_>>().as_slice().write_to_prefix(data).map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }
    #[inline]
    fn dump_bytes_vec<T: OrderedData>(val: &Vec<T>) -> Vec<u8> {
        val.iter().cloned().flat_map(|x| T::PC::from(x).as_bytes().iter().cloned().collect::<Vec<_>>()).collect()
    }
}

impl Version for XBOX {
    #[inline]
    fn from_bytes<T: OrderedData>(data: &[u8]) -> Result<T> {
        Ok(T::XBOX::read_from_prefix(data).map_err(|e| anyhow!("{}", e))?.0.into())
    }
    #[inline]
    fn to_bytes<T: OrderedData>(val: &T, data: &mut [u8]) -> Result<()> {
        T::XBOX::write_to_prefix(&val.clone().into(), data).map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }
    #[inline]
    fn dump_bytes<T: OrderedData>(val: &T) -> Vec<u8> {
        T::XBOX::as_bytes(&val.clone().into()).to_vec()
    }
    #[inline]
    fn size<T: OrderedData>() -> usize {
        size_of::<T::XBOX>()
    }
    #[inline]
    fn from_bytes_vec<T: OrderedData>(data: &[u8], num: usize) -> Result<Vec<T>> {
       Ok(<[T::XBOX]>::ref_from_prefix_with_elems(data, num).map_err(|e| anyhow!("{}", e))?.0.iter().map(|x| x.clone().into()).collect())
    }
    #[inline]
    fn to_bytes_vec<T: OrderedData>(val: &Vec<T>, data: &mut [u8]) -> Result<()> {
        val.iter().cloned().map(|x| T::XBOX::from(x)).collect::<Vec<_>>().as_slice().write_to_prefix(data).map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }
    #[inline]
    fn dump_bytes_vec<T: OrderedData>(val: &Vec<T>) -> Vec<u8> {
        val.iter().cloned().flat_map(|x| T::XBOX::from(x).as_bytes().iter().cloned().collect::<Vec<_>>()).collect()
    }
}

impl Version for PS3 {
    #[inline]
    fn from_bytes<T: OrderedData>(data: &[u8]) -> Result<T> {
        Ok(T::PS3::read_from_prefix(data).map_err(|e| anyhow!("{}", e))?.0.into())
    }
    #[inline]
    fn to_bytes<T: OrderedData>(val: &T, data: &mut [u8]) -> Result<()> {
        T::PS3::write_to_prefix(&val.clone().into(), data).map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }
    #[inline]
    fn dump_bytes<T: OrderedData>(val: &T) -> Vec<u8> {
        T::PS3::as_bytes(&val.clone().into()).to_vec()
    }
    #[inline]
    fn size<T: OrderedData>() -> usize {
        size_of::<T::PS3>()
    }
    #[inline]
    fn from_bytes_vec<T: OrderedData>(data: &[u8], num: usize) -> Result<Vec<T>> {
       Ok(<[T::PS3]>::ref_from_prefix_with_elems(data, num).map_err(|e| anyhow!("{}", e))?.0.iter().map(|x| x.clone().into()).collect())
    }
    #[inline]
    fn to_bytes_vec<T: OrderedData>(val: &Vec<T>, data: &mut [u8]) -> Result<()> {
        val.iter().cloned().map(|x| T::PS3::from(x)).collect::<Vec<_>>().as_slice().write_to_prefix(data).map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }
    #[inline]
    fn dump_bytes_vec<T: OrderedData>(val: &Vec<T>) -> Vec<u8> {
        val.iter().cloned().flat_map(|x| T::PS3::from(x).as_bytes().iter().cloned().collect::<Vec<_>>()).collect()
    }
}

pub trait OrderedData where Self: OrderedDataImpl {
    #[inline]
    fn from_bytes<V: Version>(data: &[u8]) -> Result<Self> { V::from_bytes(data) }
    #[inline]
    fn to_bytes<V: Version>(&self, data: &mut [u8]) -> Result<()> { V::to_bytes(self, data) }
    #[inline]
    fn dump_bytes<V: Version>(&self) -> Vec<u8> { V::dump_bytes(self) }
    #[inline]
    fn size<V: Version>() -> usize { V::size::<Self>() }
}

impl <T: OrderedDataImpl> OrderedData for T {}

pub trait OrderedDataImpl where Self: Sized + Clone + Default {
    type PC: Into<Self> + From<Self> + Immutable + KnownLayout + FromBytes + IntoBytes + Unaligned + Clone + std::fmt::Debug + Default;
    type XBOX: Into<Self> + From<Self> + Immutable + KnownLayout + FromBytes + IntoBytes + Unaligned + Clone + std::fmt::Debug + Default;
    type PS3: Into<Self> + From<Self> + Immutable + KnownLayout + FromBytes + IntoBytes + Unaligned + Clone + std::fmt::Debug + Default;
}
pub trait OrderedDataVec {
    fn from_bytes<V: Version>(data: &[u8], num: usize) -> Result<Self> where Self: Sized;
    fn to_bytes<V: Version>(&self, data: &mut [u8]) -> Result<()>;
    fn dump_bytes<V: Version>(&self) -> Vec<u8>;
    fn size<V: Version>(&self) -> usize;
}

impl <T: OrderedDataImpl> OrderedDataVec for Vec<T> {
    #[inline]
    fn from_bytes<V: Version>(data: &[u8], num: usize) -> Result<Self> { V::from_bytes_vec(data, num) }
    #[inline]
    fn to_bytes<V: Version>(&self, data: &mut [u8]) -> Result<()> { V::to_bytes_vec(self, data) }
    #[inline]
    fn dump_bytes<V: Version>(&self) -> Vec<u8> { V::dump_bytes_vec(self) }
    #[inline]
    fn size<V: Version>(&self) -> usize { V::size_vec(self) }
}


impl OrderedDataImpl for f32 { type PC = F32<LE>; type XBOX = F32<BE>; type PS3 = F32<BE>; }
impl OrderedDataImpl for u64 { type PC = U64<LE>; type XBOX = U64<BE>; type PS3 = U64<BE>; }
impl OrderedDataImpl for u32 { type PC = U32<LE>; type XBOX = U32<BE>; type PS3 = U32<BE>; }
impl OrderedDataImpl for i32 { type PC = I32<LE>; type XBOX = I32<BE>; type PS3 = I32<BE>; }
impl OrderedDataImpl for u16 { type PC = U16<LE>; type XBOX = U16<BE>; type PS3 = U16<BE>; }
impl OrderedDataImpl for i16 { type PC = I16<LE>; type XBOX = I16<BE>; type PS3 = I16<BE>; }
impl OrderedDataImpl for u8 { type PC = u8; type XBOX = u8; type PS3 = u8; }
impl OrderedDataImpl for i8 { type PC = i8; type XBOX = i8; type PS3 = i8; }

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

#[pyfunction]
#[pyo3(signature = (string, mask=None))]
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

    pub static ref DECOMP_LUA: Mutex<bool> = Mutex::new(false);

    pub static ref RECOMP_LUA: Mutex<bool> = Mutex::new(false);

    pub static ref UNLUAC: Mutex<String> = Mutex::new("unluac.jar".to_string());

    pub static ref COMPRESSION: Mutex<flate2::Compression> = Mutex::new(flate2::Compression::default());

    pub static ref ANIM_TABLES: Mutex<bool> = Mutex::new(true);

    pub static ref ZIP: Mutex<bool> = Mutex::new(true);

    pub static ref GLTF: Mutex<bool> = Mutex::new(false);
}

pub fn update_strings(vals: &[String]) {
    let mut strings = STRING_LOOKUP.lock().unwrap();
    let new_strings: Vec<_> = vals.iter().filter_map(|x| {
        let key = hash_string(x.as_bytes(), None);
        if strings.contains_key(&key) {
            None
        } else {
            Some((key, x.clone()))
        }
    }).collect();
    strings.extend(new_strings);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "U32String", into = "String")]
pub enum Crc {
    Str(String),
    Key(u32)
}

impl IntoPy<PyObject> for Crc {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_string().into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Crc {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self::from_string(String::extract_bound(ob)?.as_str()))
    }
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

    pub fn from_key(val: u32) -> Self {
        match STRING_LOOKUP.lock().ok().and_then(|m| m.get(&val).map(|x| x.clone())) {
            Some(str) => Self::Str(str),
            None => Self::Key(val)
        }
    }

    pub fn from_string(val: &str) -> Self {
        if val.starts_with("0x") {
            Self::from_key(u32::from_str_radix(&val[2..], 16).unwrap())
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

impl PartialOrd for Crc {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        self.key().partial_cmp(&other.key())
    }

}

impl std::hash::Hash for Crc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}

impl Eq for Crc {}
impl Ord for Crc {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key().cmp(&other.key())
    }
}

impl Default for Crc {
    fn default() -> Self {
        Self::Key(0)
    }
}

impl<O: ByteOrder> From<U32<O>> for Crc {
    fn from(value: U32<O>) -> Self {
        Self::from_key(value.into())
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

impl From<&str> for Crc {
    fn from(value: &str) -> Self {
        Crc::from_string(value)
    }
}

impl From<Crc> for String {
    fn from(value: Crc) -> Self {
        value.to_string()
    }
}

impl From<U32String> for Crc {
    fn from(value: U32String) -> Self {
        match value {
            U32String::U32(x) => Crc::from_key(x),
            U32String::String(x) => Crc::from_string(&x),
        }
    }
}

impl OrderedDataImpl for Crc { type PC = U32<LE>; type XBOX = U32<BE>; type PS3 = U32<BE>; }

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum U32String {
    U32(u32),
    String(String),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Strings {
    pub strings: Vec<String>,
}

impl IntoPy<PyObject> for Strings {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.strings.into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Strings {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self { strings: Vec::extract_bound(ob)? })
    }
}

impl Strings {
    pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize, num: usize) -> Result<Self> {
        let mut strings = Vec::with_capacity(num);
        let mut offset = offset;
        for _ in 0..num {
            let k = u32::from_bytes::<O>(&data[offset..])? as usize;
            offset += 4;
            strings.push(String::from_utf8_lossy(&data[offset..offset+k]).to_string());
            offset += k;
        }
        Ok(Self { strings, ..Default::default() })
    }

    pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], offset: usize) -> Result<()> {
        let mut offset = offset;
        for string in &self.strings {
            (string.len() as u32).to_bytes::<O>(&mut data[offset..])?;
            offset += 4;
            data[offset..(offset+string.len())].copy_from_slice(string.as_bytes());
            offset += string.len();
        }
        Ok(())
    }

    pub fn dump<O: Version + 'static>(&self) -> Vec<u8> {
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

    pub fn to_file(&self, writer: Writer) -> Result<()> {
        writer.with_extension("json").write(&to_vec_pretty(&json!(self.strings))?)?;
        Ok(())
    }

    pub fn from_file(reader: Reader) -> Result<Self> {
        let vals = from_slice::<Value>(&reader.with_extension("json").read()?)?;
        let strings = vals.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect::<Vec<_>>();
        Ok(Self { strings })
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CompressedBlock {
    pub data: Vec<u8>,
}

impl IntoPy<PyObject> for CompressedBlock {
    fn into_py(self, py: Python<'_>) -> PyObject {
        std::borrow::Cow::from(&self.data[..]).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for CompressedBlock {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self { data: Vec::extract_bound(ob)? })
    }
}

impl CompressedBlock {
    pub fn from_data<O: Version + 'static>(data: &[u8], size: usize, size_comp: usize, offset: usize) -> Self {
        let data = match size_comp {
            0 => data[offset..offset + size].to_vec(),
            _ => if TypeId::of::<O>() == TypeId::of::<PS3>() {
                let mut out = Vec::with_capacity(size);
                let data = &data[offset..offset+size_comp];
                let mut s = 2;
                while s < size_comp {
                    let mut decoder = ZlibDecoder::new_with_decompress(&data[s..], Decompress::new(false));
                    decoder.read_to_end(&mut out).unwrap();
                    s += decoder.total_in() as usize + 2;
                }
                out
            } else {
                let mut out = Vec::with_capacity(size);
                ZlibDecoder::new(&data[offset..offset+size_comp]).read_to_end(&mut out).unwrap();
                out
            }
        };
        Self { data }
    }

    pub fn dump(&self) -> Vec<u8> {
        if self.data.is_empty() { return Vec::new(); }
        // let mut z = ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
        let mut z = ZlibEncoder::new(Vec::new(), COMPRESSION.lock().unwrap().clone());
        z.write_all(self.data.as_slice()).unwrap();
        z.finish().unwrap()
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

impl IntoPy<PyObject> for Vector2 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        (self.x, self.y).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Vector2 {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let (x, y) = <(f32, f32)>::extract_bound(ob)?;
        Ok(Self { x, y })
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl IntoPy<PyObject> for Vector3 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        (self.x, self.y, self.z).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Vector3 {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let (x,y,z) = <(f32, f32, f32)>::extract_bound(ob)?;
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl IntoPy<PyObject> for Vector4 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        (self.x, self.y, self.z, self.w).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Vector4 {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let (x,y,z,w) = <(f32, f32, f32, f32)>::extract_bound(ob)?;
        Ok(Self { x, y, z, w })
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Matrix4x4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

impl IntoPy<PyObject> for Matrix4x4 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        [self.x, self.y, self.z, self.w].into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Matrix4x4 {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let [x,y,z,w] = <[Vector4; 4]>::extract_bound(ob)?;
        Ok(Self { x, y, z, w })
    }
}

impl From<&[f32; 16]> for Matrix4x4 {
    fn from(x: &[f32; 16]) -> Self {
        Self {
            x: Vector4 { x: x[0], y: x[1], z: x[2], w: x[3] },
            y: Vector4 { x: x[4], y: x[5], z: x[6], w: x[7] },
            z: Vector4 { x: x[8], y: x[9], z: x[10], w: x[11] },
            w: Vector4 { x: x[12], y: x[13], z: x[14], w: x[15] },
        }
    }
}

impl std::convert::TryFrom<&[f32]> for Matrix4x4 {
    type Error = ();
    fn try_from(x: &[f32]) -> Result<Self, ()> {
        if x.len() >= 16 { Ok(Self {
            x: Vector4 { x: x[0], y: x[1], z: x[2], w: x[3] },
            y: Vector4 { x: x[4], y: x[5], z: x[6], w: x[7] },
            z: Vector4 { x: x[8], y: x[9], z: x[10], w: x[11] },
            w: Vector4 { x: x[12], y: x[13], z: x[14], w: x[15] },
        }) } else { Err(()) }
    }
}

impl From<&Matrix4x4> for [f32; 16] {
    fn from(mat: &Matrix4x4) -> Self { [
        mat.x.x, mat.x.y, mat.x.z, mat.x.w,
        mat.y.x, mat.y.y, mat.y.z, mat.y.w,
        mat.z.x, mat.z.y, mat.z.z, mat.z.w,
        mat.w.x, mat.w.y, mat.w.z, mat.w.w
    ]}
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Bool {
    pub val: u8,
    pub _pad1: u8,
    pub _pad2: u8,
    pub _pad3: u8,
}

impl IntoPy<PyObject> for Bool {
    fn into_py(self, py: Python<'_>) -> PyObject {
        (self.val != 0).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Bool {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let v = bool::extract_bound(ob)?;
        Ok(Self { val: if v { 1 } else { 0 }, ..Default::default() })
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Weight {
    pub x: u32,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}

impl IntoPy<PyObject> for Weight {
    fn into_py(self, py: Python<'_>) -> PyObject {
        (self.x, self.a, self.b, self.c, self.d).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Weight {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let (x, a, b, c, d) = <(u32, u8, u8, u8, u8)>::extract_bound(ob)?;
        Ok(Self { x, a, b, c, d })
    }
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
#[serde(try_from = "&str", into = "String")]
pub struct Color(pub u32);

impl From<Color> for String {
    fn from(value: Color) -> Self {
        format!("0x{:08X}", value.0)
    }
}

impl TryFrom<&str> for Color  {
    type Error = std::num::ParseIntError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        u32::from_str_radix(&value[2..], 16).map(|x| Self(x))
    }
}

impl IntoPy<PyObject> for Color {
    fn into_py(self, py: Python<'_>) -> PyObject {
        format!("0x{:08X}", self.0).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Color {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let val: String = ob.extract()?;
        Ok(Self(u32::from_str_radix(&val[2..], 16)?))
    }
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Clone, Serialize, Deserialize, PyMethods)]
pub enum BaseTypes {
    CRC(Crc),
    GUID(u32),
    Color(Color),
    Vector2(Vector2),
    Vector3(Vector3),
    Vector4(Vector4),
    Matrix4x4(Matrix4x4),
    Float(f32),
    Int(i32),
    Bool(Bool),
    String(String),
    StringList(Vec<String>),
    ObjectList(Vec<u32>),
    NodeList(Vec<Vector4>),
    IntList(Vec<i32>),
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
    pub fn from_data<O: Version + 'static>(data: &[u8], kind: u32) -> Result<Self> {
        Ok(match kind {
            Self::CRC_KEY => Self::CRC(OrderedData::from_bytes::<O>(data)?),
            Self::GUID_KEY => Self::GUID(OrderedData::from_bytes::<O>(data)?),
            Self::COLOR_KEY => Self::Color(OrderedData::from_bytes::<O>(data)?),
            Self::VECTOR2_KEY => Self::Vector2(OrderedData::from_bytes::<O>(data)?),
            Self::VECTOR3_KEY => Self::Vector3(OrderedData::from_bytes::<O>(data)?),
            Self::VECTOR4_KEY => Self::Vector4(OrderedData::from_bytes::<O>(data)?),
            Self::MATRIX4X4_KEY => Self::Matrix4x4(OrderedData::from_bytes::<O>(data)?),
            Self::FLOAT_KEY => Self::Float(OrderedData::from_bytes::<O>(data)?),
            Self::INT_KEY  => Self::Int(OrderedData::from_bytes::<O>(data)?),
            Self::BOOL_KEY => Self::Bool(OrderedData::from_bytes::<O>(data)?),
            Self::BYTE_KEY => Self::Byte(OrderedData::from_bytes::<O>(data)?),
            Self::STRING_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data)?;
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize)?;
                Self::String(String::from_utf8(vals).unwrap())
            },
            Self::STRINGLIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data)?;
                let vals: Vec<List> = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize)?;
                let valss = vals.iter().enumerate().map(|(i, v)| { Ok(String::from_utf8(
                    OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>() * (i + 2) + v.offset as usize..], v.num as usize)?
                ).unwrap())}).collect::<Result<Vec<_>>>()?;
                Self::StringList(valss)
            },
            Self::OBJECTLIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data)?;
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize)?;
                Self::ObjectList(vals)
            },
            Self::NODELIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data)?;
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize)?;
                Self::NodeList(vals)
            },
            Self::INTLISTS_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data)?;
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize)?;
                Self::IntList(vals)
            },
            Self::CRCLIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data)?;
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize)?;
                Self::CRCList(vals)
            },
            Self::WEIGHTLIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data)?;
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize)?;
                Self::WeightList(vals)
            },
            Self::MATRIXLIST_KEY => {
                let val: List = OrderedData::from_bytes::<O>(data)?;
                let vals = OrderedDataVec::from_bytes::<O>(&data[val.offset as usize + List::size::<O>()..], val.num as usize)?;
                Self::MatrixList(vals)
            },
            _ => panic!("Unkown Type {:?}", kind)
        })
    }

    pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], off: &mut usize) -> Result<()> {
        match self {
            Self::CRC(val) => val.to_bytes::<O>(data)?,
            Self::GUID(val) => val.to_bytes::<O>(data)?,
            Self::Color(val) => val.to_bytes::<O>(data)?,
            Self::Vector2(val) => val.to_bytes::<O>(data)?,
            Self::Vector3(val) => val.to_bytes::<O>(data)?,
            Self::Vector4(val) => val.to_bytes::<O>(data)?,
            Self::Matrix4x4(val) => val.to_bytes::<O>(data)?,
            Self::Float(val) => val.to_bytes::<O>(data)?,
            Self::Int(val) => val.to_bytes::<O>(data)?,
            Self::Bool(val) => val.to_bytes::<O>(data)?,
            Self::Byte(val) => val.to_bytes::<O>(data)?,
            Self::String(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data)?;
                data[*off..*off + vals.len()].copy_from_slice(vals.as_bytes());
                if vals.len() != 0 { *off += vals.len() + 1 };
            },
            Self::StringList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data)?;
                let mut off_ = *off;
                *off += vals.len() * List::size::<O>();
                for v in vals {
                    List{ num: v.len() as u16, offset: (*off - off_ - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off_..])?;
                    data[*off..*off + v.len()].copy_from_slice(v.as_bytes());                    
                    if v.len() != 0 {
                        *off += v.len() + 1;
                    }
                    off_ += 4;
                }
            },
            Self::ObjectList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data)?;
                vals.to_bytes::<O>(&mut data[*off..])?;
                *off += vals.size::<O>();
            },
            Self:: NodeList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data)?;
                vals.to_bytes::<O>(&mut data[*off..])?;
                *off += vals.size::<O>();
            },
            Self::IntList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data)?;
                vals.to_bytes::<O>(&mut data[*off..])?;
                *off += vals.size::<O>();
            },
            Self::CRCList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data)?;
                vals.to_bytes::<O>(&mut data[*off..])?;
                *off += vals.size::<O>();
            },
            Self::WeightList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data)?;
                vals.to_bytes::<O>(&mut data[*off..])?;
                *off += vals.size::<O>();
            },
            Self::MatrixList(vals) => {
                List{ num: vals.len() as u16, offset: (*off - List::size::<O>()) as u16}.to_bytes::<O>(data)?;
                vals.to_bytes::<O>(&mut data[*off..])?;
                *off += vals.size::<O>();
            },
        };
        Ok(())
    }

    pub fn dump<O: Version + 'static>(&self, data: &mut Vec<u8>, off: usize) -> Result<()> {
        match self {
            Self::CRC(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::GUID(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::Color(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::Vector2(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::Vector3(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::Vector4(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::Matrix4x4(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::Float(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::Int(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::Bool(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::Byte(val) => val.to_bytes::<O>(&mut data[off..])?,
            Self::String(vals) => {
                List{ num: vals.len() as u16, offset: (data.len() - off - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off..])?;
                data.extend(vals.as_bytes());
                if vals.len() != 0 { data.push(0) };
            },
            Self::StringList(vals) => {
                List{ num: vals.len() as u16, offset: (data.len() - off - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off..])?;
                let mut off_ = data.len();
                data.extend(vec![0u8; vals.len() * List::size::<O>()]);
                for v in vals {
                    List{ num: v.len() as u16, offset: (data.len() - off_ - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off_..])?;
                    data.extend(v.as_bytes());
                    if v.len() != 0 { data.push(0) }
                    off_ += 4;
                }
            },
            Self::ObjectList(vals) => {
                List{ num: vals.len() as u16, offset: (data.len() - off - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off..])?;
                data.extend(vals.dump_bytes::<O>());
            },
            Self:: NodeList(vals) => {
                List{ num: vals.len() as u16, offset: (data.len() - off - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off..])?;
                data.extend(vals.dump_bytes::<O>());
            },
            Self::IntList(vals) => {
                List{ num: vals.len() as u16, offset: (data.len() - off - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off..])?;
                data.extend(vals.dump_bytes::<O>());
            },
            Self::CRCList(vals) => {
                List{ num: vals.len() as u16, offset: (data.len() - off - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off..])?;
                data.extend(vals.dump_bytes::<O>());
            },
            Self::WeightList(vals) => {
                List{ num: vals.len() as u16, offset: (data.len() - off - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off..])?;
                data.extend(vals.dump_bytes::<O>());
            },
            Self::MatrixList(vals) => {
                List{ num: vals.len() as u16, offset: (data.len() - off - List::size::<O>()) as u16}.to_bytes::<O>(&mut data[off..])?;
                data.extend(vals.dump_bytes::<O>());
            },
        }
        Ok(())
    }


    pub fn dump_bytes<O: Version + 'static>(&self) -> Vec<u8> {
        match self {
            Self::CRC(val) => val.dump_bytes::<O>(),
            Self::GUID(val) => val.dump_bytes::<O>(),
            Self::Color(val) => val.dump_bytes::<O>(),
            Self::Vector2(val) => val.dump_bytes::<O>(),
            Self::Vector3(val) => val.dump_bytes::<O>(),
            Self::Vector4(val) => val.dump_bytes::<O>(),
            Self::Matrix4x4(val) => val.dump_bytes::<O>(),
            Self::Float(val) => val.dump_bytes::<O>(),
            Self::Int(val) => val.dump_bytes::<O>(),
            Self::Bool(val) => val.dump_bytes::<O>(),
            Self::Byte(val) => val.dump_bytes::<O>(),
            _ => panic!("Not implemented for this type"),
        }
    }


    pub fn off_size<O: Version + 'static>(&self) -> usize {
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

    pub fn size<O: Version + 'static>(&self) -> usize {
        match self {
            Self::CRC(..) => u32::size::<O>(),
            Self::GUID(..) => u32::size::<O>(),
            Self::Color(..) => u32::size::<O>(),
            Self::Vector2(..) => Vector2::size::<O>(),
            Self::Vector3(..) => Vector3::size::<O>(),
            Self::Vector4(..) => Vector4::size::<O>(),
            Self::Matrix4x4(..) => Matrix4x4::size::<O>(),
            Self::Float(..) => f32::size::<O>(),
            Self::Int(..) => u32::size::<O>(),
            Self::Bool(..) => Bool::size::<O>(),
            Self::Byte(..) => u8::size::<O>(),
            Self::String(..) => List::size::<O>(),
            Self::StringList(..) => List::size::<O>(),
            Self::ObjectList(..) => List::size::<O>(),
            Self:: NodeList(..) => List::size::<O>(),
            Self::IntList(..) => List::size::<O>(),
            Self::CRCList(..) => List::size::<O>(),
            Self::WeightList(..) => List::size::<O>(),
            Self::MatrixList(..) => List::size::<O>(),
        }
    }

    pub fn to_json(&self) -> Value {
        match self {
            Self::CRC(val) => json!(val.to_string()),
            Self::GUID(val) => json!(val),
            Self::Color(val) => json!(String::from(val.clone())),
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
            Self::WeightList(vals) => json!(vals.iter().map(|x| json!([x.x, x.a, x.b, x.c, x.d])).collect::<Vec<_>>()),
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
            Self::COLOR_KEY => Self::Color(Color::try_from(val.as_str().unwrap()).unwrap()),
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
            Self::INT_KEY  => Self::Int(val.as_i64().unwrap() as i32),
            Self::BOOL_KEY => Self::Bool(Bool { val: val.as_bool().unwrap() as u8, _pad1: 0, _pad2: 0, _pad3: 0 }),
            Self::BYTE_KEY => Self::Byte(val.as_u64().unwrap() as u8),
            Self::STRING_KEY => Self::String(val.as_str().unwrap().into()),
            Self::STRINGLIST_KEY => Self::StringList(val.as_array().unwrap().into_iter().map(|x| x.as_str().unwrap().into()).collect()),
            Self::OBJECTLIST_KEY => Self::ObjectList(val.as_array().unwrap().into_iter().map(|x| x.as_u64().unwrap() as u32).collect()),
            Self::NODELIST_KEY => Self::NodeList(val.as_array().unwrap().into_iter().map(|x| {
                let vals = x.as_array().unwrap().into_iter().map(|x| x.as_f64().unwrap() as f32).collect::<Vec<_>>();
                Vector4 { x: vals[0], y: vals[1], z: vals[2], w: vals[3] }
            }).collect()),
            Self::INTLISTS_KEY => Self::IntList(val.as_array().unwrap().into_iter().map(|x| x.as_i64().unwrap() as i32).collect()),
            Self::CRCLIST_KEY => Self::CRCList(val.as_array().unwrap().into_iter().map(|x| Crc::from_string(x.as_str().unwrap())).collect()),
            Self::WEIGHTLIST_KEY => Self::WeightList(val.as_array().unwrap().into_iter().map(|x| {
                let vals = x.as_array().unwrap().into_iter().map(|x| x.as_u64().unwrap() as u32).collect::<Vec<_>>();
                Weight { x: vals[0], a: vals[1] as u8, b: vals[2] as u8, c: vals[3] as u8, d: vals[4] as u8}
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

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Clone, Serialize, Deserialize, PyMethods)]
pub enum SubBlock {
    LangStrings(LangStrings),
    Data(Data),
    Spray(Spray),
    Crowd(Crowd),
    GameObjs(GameObjs),
    AtlasUV(AtlasUV),
    Lua(Lua),
    SSA(SSA),
}

impl SubBlock {
    pub fn from_data<O: Version + 'static>(data: &[u8], info: &SubBlocksBlockHeader, lua: &LuaCompiler) -> Result<Self> {
        Ok(match info.key.key() {
            LangStrings::KEY_POLISH | LangStrings::KEY_GERMAN | LangStrings::KEY_FRENCH | LangStrings::KEY_SPANISH | LangStrings::KEY_RUSSIAN | LangStrings::KEY_SWEDISH | LangStrings::KEY_ENGLISH | LangStrings::KEY_ITALIAN | LangStrings::KEY_NORWEGIAN => 
                SubBlock::LangStrings(LangStrings::from_data::<O>(data, info.offset as usize, info.size as usize)?),
            Data::KEY_PFIELDS => SubBlock::Data(Data::from_data(data, info.offset as usize, info.size as usize)),
            Spray::KEY => SubBlock::Spray(Spray::from_data::<O>(data, info.offset as usize, info.size as usize)?),
            Crowd::KEY => SubBlock::Crowd(Crowd::from_data::<O>(data, info.offset as usize, info.size as usize)?),
            GameObjs::KEY => SubBlock::GameObjs(GameObjs::from_data::<O>(data, info.offset as usize, info.size as usize, -1)?),
            AtlasUV::KEY1 | AtlasUV::KEY2 => SubBlock::AtlasUV(AtlasUV::from_data::<O>(data, info.offset as usize, info.size as usize)?),
            _ => match info.key.str() {
                Some(x) if x.ends_with(".lua") => SubBlock::Lua(Lua::from_data(data, info.offset as usize, info.size as usize, lua, x.to_string())?),
                Some(x) if x.ends_with(".ssa") => SubBlock::SSA(SSA::from_data::<O>(data, info.offset as usize, info.size as usize)?),
                Some(x) if x.ends_with(".csv") || x.ends_with(".txt") || x.ends_with(".dat") => 
                    SubBlock::Data(Data::from_data(data, info.offset as usize, info.size as usize)),
                _ => {
                    warn!("Unknown block type {:?}", info.key);
                    SubBlock::Data(Data::from_data(data, info.offset as usize, info.size as usize))
                } 
            }
        })
    }

    pub fn dump<O: Version + 'static>(&self, lua: &LuaCompiler) -> Result<Vec<u8>> {
        Ok(match self {
            SubBlock::LangStrings(val) => val.dump::<O>(),
            SubBlock::Data(val) => val.dump(),
            SubBlock::Spray(val) => val.dump::<O>()?,
            SubBlock::Crowd(val) => val.dump::<O>()?,
            SubBlock::GameObjs(val) => val.dump::<O>()?,
            SubBlock::AtlasUV(val) => val.dump::<O>(),
            SubBlock::Lua(val) => val.dump(lua),
            SubBlock::SSA(val) => val.dump::<O>()?,
        })
    }

    pub fn size<O: Version + 'static>(&self) -> usize {
        match self {
            SubBlock::LangStrings(val) => val.size(),
            SubBlock::Data(val) => val.data.len(),
            SubBlock::Spray(val) => val.size::<O>(),
            SubBlock::Crowd(val) => val.size::<O>(),
            SubBlock::GameObjs(val) => val.size::<O>(),
            SubBlock::AtlasUV(val) => val.vals.size::<O>(),
            SubBlock::Lua(val) => val.data.len(),
            SubBlock::SSA(val) => val.size(),
        }
    }

    pub fn to_file(&self, writer: Writer, keys: &StringKeys) -> Result<()> {
        match self {
            SubBlock::LangStrings(val) => val.to_file(writer, keys),
            SubBlock::Data(val) => val.to_file(writer),
            SubBlock::Spray(val) => val.to_file(writer),
            SubBlock::Crowd(val) => val.to_file(writer),
            SubBlock::GameObjs(val) => val.to_file(writer),
            SubBlock::AtlasUV(val) => val.to_file(writer),
            SubBlock::Lua(val) => val.to_file(writer),
            SubBlock::SSA(val) => val.to_file(writer),
        }
    }

    pub fn from_file(reader: Reader, info: &SubBlocksBlockHeader, lua: &LuaCompiler) -> Result<Self> {
        Ok(match info.key.key() {
            LangStrings::KEY_POLISH | LangStrings::KEY_GERMAN | LangStrings::KEY_FRENCH | LangStrings::KEY_SPANISH | LangStrings::KEY_RUSSIAN | LangStrings::KEY_SWEDISH | LangStrings::KEY_ENGLISH | LangStrings::KEY_ITALIAN | LangStrings::KEY_NORWEGIAN => 
                SubBlock::LangStrings(LangStrings::from_file(reader)?),
            Data::KEY_PFIELDS => SubBlock::Data(Data::from_file(reader)?),
            Spray::KEY => SubBlock::Spray(Spray::from_file(reader)?),
            Crowd::KEY => SubBlock::Crowd(Crowd::from_file(reader)?),
            GameObjs::KEY => SubBlock::GameObjs(GameObjs::from_file(reader.with_file_name("level.json"))?),
            AtlasUV::KEY1 | AtlasUV::KEY2 => SubBlock::AtlasUV(AtlasUV::from_file(reader)?),
            _ => match info.key.str() {
                Some(x) if x.ends_with(".lua") => SubBlock::Lua(Lua::from_file(reader, lua)?),
                Some(x) if x.ends_with(".ssa") => SubBlock::SSA(SSA::from_file(reader)?),
                Some(x) if x.ends_with(".csv") || x.ends_with(".txt") || x.ends_with(".dat") => 
                    SubBlock::Data(Data::from_file(reader)?),
                _ =>  {
                    warn!("Unknown block type {:?}", info.key);
                    SubBlock::Data(Data::from_file(reader)?)
                }    
            }
        })
    }
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct SubBlocksHeader {
    pub z0: u32,
    pub block_num: u32,
    pub z2: u32,
    pub z3: u32,
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct SubBlocksBlockHeader {
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Default, Debug, Clone, Serialize, Deserialize, PyMethods)]
pub struct SubBlocks {
    #[serde(skip)]
    pub header: SubBlocksHeader,
    pub block_headers: Vec<SubBlocksBlockHeader>,
    #[serde(skip)]
    pub blocks: Vec<SubBlock>,
}

impl SubBlocks {
    pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize, lua: &LuaCompiler, prog: Option<&ProgressBar>) -> Result<Self> {
        let header: SubBlocksHeader = OrderedData::from_bytes::<O>(&data[offset..])?;
        prog.map(|x| x.set_length(header.block_num as u64));
        let block_headers: Vec<SubBlocksBlockHeader> = OrderedDataVec::from_bytes::<O>(&data[offset+SubBlocksHeader::size::<O>()..], header.block_num as usize)?;
        let blocks = block_headers.iter().map(|info| {
            prog.map(|x| { x.inc(1); x.set_message(info.key.to_string()) });
            SubBlock::from_data::<O>(&data[offset..], info, lua)
        }).collect::<Result<Vec<_>>>()?;
        prog.map(|x| x.finish());
        
        Ok(Self { header, block_headers, blocks })
    }

    pub fn size<O: Version + 'static>(&self) -> usize {
        let mut s = SubBlocksHeader::size::<O>() + self.block_headers.size::<O>();
        for block in &self.blocks {
            s = (s + 16) & 0xFFFFFFF0;
            s += block.size::<O>();
        }
        s = (s + 16) & 0xFFFFFFF0;
        return s
    }

    pub fn dump<O: Version + 'static>(&self, lua: &LuaCompiler, prog: Option<&ProgressBar>) -> Result<Vec<u8>> {
        prog.map(|x| x.set_length(self.header.block_num as u64));
        let mut block_headers = self.block_headers.clone();
        let mut offset = SubBlocksHeader::size::<O>() + block_headers.size::<O>();
        let mut data = vec![];
        let off = (offset + 15) & 0xfffffff0;
        data.extend(vec![0u8; off - offset]);
        offset = off;
        for (block, info) in zip(&self.blocks, &mut block_headers) {
            prog.map(|x| { x.inc(1); x.set_message(info.key.to_string()) });
            let block_data: Vec<u8> = block.dump::<O>(lua)?;
            info.offset = offset as u32;
            info.size = block_data.len() as u32;
            offset += block_data.len();
            data.extend(block_data);
            let off = (offset + 16) & 0xfffffff0;
            data.extend(vec![0u8; off - offset]);
            offset = off;
        }
        prog.map(|x| x.finish());
        Ok(self.header.dump_bytes::<O>().into_iter().chain(block_headers.dump_bytes::<O>().into_iter()).chain(data.into_iter()).collect())
    }

    pub fn to_file(&self, writer: Writer, keys: &StringKeys, prog: Option<&ProgressBar>) -> Result<()> {
        prog.map(|x| x.set_length(self.header.block_num as u64));
        writer.join("index.json").write(&to_vec_pretty(self)?)?;
        for (block, info) in zip(&self.blocks, &self.block_headers) {
            prog.map(|x| { x.inc(1); x.set_message(info.key.to_string()) });
            block.to_file(writer.join(info.key.str().unwrap()), keys)?
        }
        prog.map(|x| x.finish());
        Ok(())
    }

    pub fn from_file(reader: Reader, lua: &LuaCompiler, prog: Option<&ProgressBar>) -> Result<Self> {
        let mut val = from_slice::<Self>(&reader.join("index.json").read()?)?;
        prog.map(|x| x.set_length(val.block_headers.len() as u64));
        val.blocks = Result::from_iter(val.block_headers.iter().map(|info| {
            prog.map(|x| { x.inc(1); x.set_message(info.key.to_string()) });
            SubBlock::from_file(reader.join(info.key.str().unwrap()), info, lua)
        }))?;
        val.header.block_num = val.blocks.len() as u32;
        prog.map(|x| x.finish());
        Ok(val)
    }
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct StringKeysHeader {
    pub num_a: u16,
    pub num_b: u16,
    pub z2: u32,
    pub z3: u32,
    pub z4: u32,
    pub z5: u32,
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct StringKeysVal {
    pub key: Crc,
    pub offset: u32,
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Default, Debug, Clone, Serialize, Deserialize, PyMethods)]
pub struct StringKeys {
    pub header: StringKeysHeader,
    pub vals: Vec<StringKeysVal>,
    pub pad: Vec<u32>,
}

impl StringKeys {
    pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize) -> Result<Self> {
        let mut offset: usize = offset;
        let header: StringKeysHeader = OrderedData::from_bytes::<O>(&data[offset..])?;
        assert!(header.num_a == header.num_b, "Seems to be true");
        offset += StringKeysHeader::size::<O>();
        let vals: Vec<StringKeysVal> = OrderedDataVec::from_bytes::<O>(&data[offset..], header.num_a as usize)?;
        offset += vals.size::<O>();
        let pad = OrderedDataVec::from_bytes::<O>(&data[offset..], header.num_a as usize)?;
        Ok(Self { header, vals, pad })
    }

    pub fn into_data<O: Version + 'static>(&self, data: &mut [u8], offset: usize) -> Result<()> {
        let mut offset = offset;
        self.header.to_bytes::<O>(&mut data[offset..])?;
        offset += StringKeysHeader::size::<O>();
        self.vals.to_bytes::<O>(&mut data[offset..])?;
        offset += self.vals.size::<O>();
        self.pad.to_bytes::<O>(&mut data[offset..])?;
        Ok(())
    }

    pub fn size<O: Version + 'static>(&self) -> usize {
        StringKeysHeader::size::<O>() + self.vals.size::<O>() + self.pad.size::<O>()
    }

    pub fn dump<O: Version + 'static>(&self) -> Vec<u8> {
        self.header.dump_bytes::<O>().into_iter().chain(self.vals.dump_bytes::<O>().into_iter()).chain(self.pad.dump_bytes::<O>().into_iter()).collect()
    }

    pub fn to_file(&self, writer: Writer) -> Result<()> {
        writer.with_extension("json").write(&to_vec_pretty(&json!(self.vals.iter().map(|x| x.key.to_string()).collect::<Vec<_>>()))?)?;
        Ok(())
    }

    pub fn from_file(reader: Reader) -> Result<Self> {
        let vals = from_slice::<Value>(&reader.with_extension("json").read()?)?;
        let keys = vals.as_array().unwrap().iter().map(|val| Crc::from_string(val.as_str().unwrap())).collect::<Vec<_>>();
        let header = StringKeysHeader {
            num_a: keys.len() as u16,
            num_b: keys.len() as u16,
            z2: 0,
            z3: 0,
            z4: 0,
            z5: 0,
        };
        let pad = vec![0u32; keys.len()];
        let mut off = StringKeysHeader::size::<PC>() + keys.len() * StringKeysVal::size::<PC>();
        let vals = keys.into_iter().map(|key| {
            let val = StringKeysVal { key, offset: off as u32};
            off += 4;
            val
        }).collect::<Vec<_>>();
        Ok(Self { header, vals, pad })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LangStrings {
    pub strings: Vec<String>,
}

impl IntoPy<PyObject> for LangStrings {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.strings.into_py(py)
    }
}

impl <'py> FromPyObject<'py> for LangStrings {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self { strings: Vec::extract_bound(ob)? })
    }
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

    pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize, size: usize) -> Result<Self> {
        let mut val = Self::default();
        let mut offset_ = offset;
        while offset_ < size + offset {
            let start = offset_;
            while data[offset_] != 0 || data[offset_+1] != 0 {
                offset_ += 2;
            }
            let string: Vec<u16> = OrderedDataVec::from_bytes::<O>(&data[start..offset_], (offset_-start)/2)?;
            val.strings.push(String::from_utf16(string.as_slice()).unwrap());
            offset_ += 2;
        }
        Ok(val)
    }

    pub fn dump<O: Version + 'static>(&self) -> Vec<u8> {
        let vals = self.strings.iter().flat_map(|x| x.encode_utf16().chain([0u16])).collect::<Vec<_>>();
        vals.dump_bytes::<O>()
    }

    pub fn size(&self) -> usize {
        self.strings.iter().map(|x| x.encode_utf16().map(|_| 2).sum::<usize>() + 2).sum::<usize>()
    }

    pub fn to_file(&self, writer: Writer, keys: &StringKeys) -> Result<()> {
        let vals = zip(&keys.vals, &self.strings).map(|(key, string)| (key.key.to_string(), json!(string))).collect::<Map<_,_>>();
        writer.with_extension("json").write(&to_vec_pretty(&vals)?)?;
        Ok(())
    }

    pub fn from_file(reader: Reader) -> Result<Self> {
        let vals = from_slice::<Value>(&reader.with_extension("json").read()?)?;
        let strings = vals.as_object().unwrap().iter().map(|(_, s)| s.as_str().unwrap().to_string()).collect::<Vec<_>>();
        Ok(Self { strings })
    }
}

#[basicpymethods]
#[pyclass(module="types", set_all, get_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct SSAVal {
    pub t_start: f32,
    pub t_end: f32,
    pub unk_2: u32,
    pub unk_3: u32,
    pub off: u32,
}

#[basicpymethods]
#[pyclass(module="types", set_all, get_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
pub struct SSA {
    pub vals: Vec<SSAVal>,
    pub strings: Vec<String>,
}

impl SSA {
    pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize, size: usize) -> Result<Self> {
        let n = u32::from_bytes::<O>(&data[offset..])? as usize;
        let vals: Vec<SSAVal> = OrderedDataVec::from_bytes::<O>(&data[offset + 4..], n as usize)?;
        let offs = vals.iter().map(|x| x.off as usize).chain([size]).collect::<Vec<_>>();
        let strings = (0..n).map(|i| {
            let s: Vec<u16> = OrderedDataVec::from_bytes::<O>(&data[offset + offs[i]..], (offs[i+1] - offs[i])/2)?;
            Ok(String::from_utf16(s.as_slice()).unwrap())
        }).collect::<Result<Vec<_>>>()?;
        Ok(Self { vals, strings })
    }

    pub fn dump<O: Version + 'static>(&self) -> Result<Vec<u8>> {
        let mut data = vec![0u8; 4 + (SSAVal::size::<O>() * self.vals.len())];
        (self.vals.len() as u32).to_bytes::<O>(&mut data)?;
        let mut vals: Vec<SSAVal> = self.vals.clone();
        for (string, val) in zip(&self.strings, &mut vals) {
            val.off = data.len() as u32;
            data.extend(string.encode_utf16().collect::<Vec<_>>().dump_bytes::<O>());
        }
        vals.to_bytes::<O>(&mut data[4..])?;
        Ok(data)
    }

    pub fn size(&self) -> usize {
        self.strings.iter().map(|x| x.encode_utf16().map(|_| 2).sum::<usize>()).sum::<usize>() + 4 + (SSAVal::size::<PC>() * self.vals.len())
    }

    pub fn to_file(&self, writer: Writer) -> Result<()> {
        writer.with_extension("json").write(&to_vec_pretty(self)?)?;
        Ok(())
    }

    pub fn from_file(reader: Reader) -> Result<Self> {
        Ok(from_slice(&reader.with_extension("json").read()?)?)
    }
}

#[pyclass(module="types", set_all)]
#[derive(Default, Debug, Clone, Serialize, Deserialize, PyMethods)]
pub struct Lua {
    #[pyo3(get)]
    pub name: String,
    pub data: Vec<u8>,
    #[pyo3(get)]
    pub code: String,
}

#[basicpymethods]
#[pymethods]
impl Lua {
    #[getter]
    fn get_data(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::from(&self.data[..])
    }
}

impl Lua {
    pub fn from_data(data: &[u8], offset: usize, size: usize, lua: &LuaCompiler, name: String) -> Result<Self> {
        let data = data[offset..offset+size].to_vec();
        let code = if *DECOMP_LUA.lock().unwrap() {
            lua.decomp(data.as_slice(), UNLUAC.lock().unwrap().clone())?
        } else {
            String::new()
        };
        Ok(Self { code, data, name })
    }

    pub fn conv(&self, fmt: &str, lua: &LuaCompiler) -> Result<Vec<u8>> {
        let val = &self.data;
        Ok(if (val.len() > 3) && (val[0] == 0x1bu8) && (val[1] == 76) && (val[2] == 117) && (val[3] == 97) {
            lua.convert(&val, fmt)?
        } else {
            val.clone()
        })
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
        } else if *RECOMP_LUA.lock().unwrap() {
            lua.convert(&self.data, "L4404").unwrap()
        } else {
            self.data.clone()
        }
    }

    pub fn to_file(&self, writer: Writer) -> Result<()> {
        if *DECOMP_LUA.lock().unwrap() {
            writer.write(self.code.as_bytes())?;
        } else {
            writer.write(&self.data)?;
        }
        Ok(())
    }

    pub fn from_file(reader: Reader, lua: &LuaCompiler) -> Result<Self> {
        let name: String = reader.path().file_name().unwrap().to_str().unwrap().into();
        let mut val = reader.read()?;
        let (data, code) = if (val.len() > 3) && (val[0] == 0x1bu8) && (val[1] == 76) && (val[2] == 117) && (val[3] == 97) {
            let code = if *DECOMP_LUA.lock().unwrap() {
                lua.decomp(&val, UNLUAC.lock().unwrap().clone())?
            } else {
                String::new()
            };
            if *RECOMP_LUA.lock().unwrap() {
                val = lua.convert(&val, "L4404")?;
            }
            (val, code)
        } else {
            let code = String::from_utf8(val.clone())?;
            let data = if *RECOMP_LUA.lock().unwrap() {
                lua.compile(&code, &name)?
            } else {
                val
            };
            (data, code)
        };
        
        Ok(Self { code, name, data })
    }
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
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

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct GameObjsTypeHeader {
    pub key: Crc,
    pub size: u32,
    pub fields: u32,
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct GameObjsTypeField {
    pub key: Crc,
    pub kind: Crc,
    pub offset: u32,
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct GameObjsObjHeader {
    pub layer: u32,
    pub key: Crc,
    pub size: u16,
    pub z3: u16,
    pub z4: u32,
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
pub struct GameObj {
    pub layer: u32,
    pub key: Crc,
    pub fields: Vec<BaseTypes>
}

#[basicpymethods]
#[pyclass(module="types", get_all, set_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
pub struct GameObjs {
    pub gamemodemask: i32,
    pub types: IndexMap<Crc, Vec<GameObjsTypeField>>,
    pub objs: Vec<GameObj>,
}

impl GameObjs {
    pub const KEY: u32 = hash_string("Level".as_bytes(), None);
    pub fn from_data<O: Version + 'static>(data: &[u8], offset_: usize, _size: usize, gamemodemask: i32) -> Result<Self> {
        let header: GameObjsHeader = OrderedData::from_bytes::<O>(&data[offset_..])?;
        let mut types = IndexMap::with_capacity(header.types_num as usize);
        let mut objs = Vec::with_capacity(header.obj_num as usize);
        let mut offset = offset_ + header.types_offset as usize;
        for _ in 0..header.types_num {
            let info: GameObjsTypeHeader = OrderedData::from_bytes::<O>(&data[offset..])?;
            offset += GameObjsTypeHeader::size::<O>();
            let fields: Vec<GameObjsTypeField> = OrderedDataVec::from_bytes::<O>(&data[offset..], info.size as usize)?;
            offset += fields.size::<O>();
            types.insert(info.key, fields);
        }
        offset = offset_ + header.obj_offset as usize;
        for _ in 0..header.obj_num {
            let info: GameObjsObjHeader = OrderedData::from_bytes::<O>(&data[offset..])?;
            offset += GameObjsObjHeader::size::<O>();
            let ts = types.get(&info.key).unwrap();
            let mut fields = Vec::with_capacity(ts.len());
            for t in ts.iter() {
                let val = BaseTypes::from_data::<O>(&data[offset + t.offset as usize..], t.kind.key())?;
                fields.push(val)
            }
            offset += info.size as usize;
            objs.push(GameObj { layer: info.layer, key: info.key, fields });
        }
        Ok(Self { gamemodemask, types, objs })
    }

    pub fn size<O: Version + 'static>(&self) -> usize {
        let size = ((self.types.len() * GameObjsTypeHeader::size::<O>()) + (self.types.values().map(|x| x.len() as usize).sum::<usize>() * GameObjsTypeField::size::<O>()) + GameObjsHeader::size::<O>() + 15) & 0xFFFFFFF0;
        size + self.obj_headers().iter().map(|x| x.size as usize).sum::<usize>() + self.objs.len() * GameObjsObjHeader::size::<O>() 
    }

    pub fn obj_headers(&self) -> Vec<GameObjsObjHeader> {
        let mut headers = Vec::with_capacity(self.objs.len());
        for obj in &self.objs {
            let ts = self.types.get(&obj.key).unwrap();
            let mut off = zip(&obj.fields, ts).map(|(t, f)| f.offset as usize + t.size::<PC>()).fold(0, usize::max);
            for (val, t) in zip(&obj.fields, ts) {
                off += val.off_size::<PC>();
                if t.key.key() == BaseTypes::INTLISTS_KEY {
                    off = (off + 15) & 0xFFFFFFF0;
                }
            }
            let size = (off + 15) as u16 & 0xFFF0;
            headers.push(GameObjsObjHeader {
                layer: obj.layer,
                key: obj.key.clone(),
                size,
                z3: 0,
                z4: 0
            });
        }
        headers
    }

    pub fn dump<O: Version + 'static>(&self) -> Result<Vec<u8>> {
        let mut type_data = vec![];
        for (key, fields) in &self.types {
            type_data.extend(GameObjsTypeHeader { key: key.clone(), size: fields.len() as u32, fields: 0 }.dump_bytes::<O>());
            type_data.extend(fields.dump_bytes::<O>());
        }
        type_data.extend(vec![0u8; ((type_data.len() + 15) & 0xFFFFFFF0) - type_data.len()]);
        let mut data = GameObjsHeader {
            const_: 1296123652,
            types_num: self.types.len() as u32,
            types_offset: GameObjsHeader::size::<O>() as u32,
            obj_num: self.objs.len() as u32,
            obj_offset: (type_data.len() + GameObjsHeader::size::<O>()) as u32,
            z5: 0,
            z6: 0,
            z7: 0
        }.dump_bytes::<O>();
        data.extend(type_data);
        
        for obj in &self.objs {
            let ts = self.types.get(&obj.key).unwrap();
            let obj_size = zip(&obj.fields, ts).map(|(t, f)| f.offset as usize + t.size::<PC>()).fold(0, usize::max);
            let mut obj_vals = vec![0u8; (obj_size + 15) & 0xFFFFFFF0];
            for (val, t) in zip(&obj.fields, ts) {
                val.dump::<O>(&mut obj_vals, t.offset as usize)?;
                if t.kind.key() == BaseTypes::INTLISTS_KEY {
                    // println!("{}", vals.len());
                    obj_vals.extend(vec![0u8; ((obj_vals.len() + 15) & 0xFFFFFFF0) - obj_vals.len()]);
                }
            }
            obj_vals.extend(vec![0u8; ((obj_vals.len() + 15) & 0xFFFFFFF0) - obj_vals.len()]);
            data.extend(GameObjsObjHeader {
                layer: obj.layer,
                key: obj.key.clone(),
                size: obj_vals.len() as u16,
                z3: 0,
                z4: 0
            }.dump_bytes::<O>());
            data.extend(obj_vals);
        }
        Ok(data)
    }

    pub fn into_data<O: Version + 'static>(&self, data: &mut[u8], offset: usize) -> Result<()> {
        let vals = self.dump::<O>()?;
        data[offset..(offset + vals.len())].copy_from_slice(&vals[..]);
        Ok(())

    }

    pub fn to_file(&self, writer: Writer) -> Result<()> {
        let val = json!({
            "gamemodemask": self.gamemodemask,
            "objs": self.objs.iter().map(|GameObj { layer, key, fields }| {
                let ts = &self.types.get(key).unwrap();
                let mut order: Vec<_> = (0..ts.len()).collect();
                order.sort_by_key(|x| ts[*x].offset);
                json!({
                    "type": key.to_string(),
                    "layer": layer,
                    "fields": order.into_iter().map(|i| (ts[i].key.to_string(), fields[i].to_json())).collect::<Map<_,_>>()
                })
            }).collect::<Vec<_>>(),
            "types": self.types.iter().map(|(key, fs)| {
                json!({
                    "name": key.to_string(),
                    "fields": fs.iter().map(|f| {
                        json!({
                            "name": f.key.to_string(),
                            "type": f.kind.to_string(),
                            "offset": f.offset
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>(),

        });
        writer.with_extension("json").write(&to_vec_pretty(&val)?)?;
        Ok(())
    }

    pub fn from_file(reader: Reader) -> Result<Self> {
        let val = from_slice::<Value>(&reader.with_extension("json").read()?)?;
        let ts = val["types"].as_array().unwrap();
        let mut types = IndexMap::with_capacity(ts.len());
        for t in ts {
            types.insert(
                Crc::from_string(t["name"].as_str().unwrap()),
                t["fields"].as_array().unwrap().iter().map(|v| GameObjsTypeField {
                    key: Crc::from_string(v["name"].as_str().unwrap()),
                    kind: Crc::from_string(v["type"].as_str().unwrap()),
                    offset: v["offset"].as_u64().unwrap() as u32,
                }).collect::<Vec<_>>(),
            );
        }

        let os = val["objs"].as_array().unwrap();
        let mut objs = Vec::with_capacity(os.len());

        for o in os {
            let key = Crc::from_string(o["type"].as_str().unwrap());
            let ts = types.get(&key).unwrap();
            let o_ = o["fields"].as_object().unwrap();
            let fields = ts.iter().map(|t| BaseTypes::from_json(&o_[&t.key.to_string()], t.kind.key())).collect::<Vec<_>>();
            objs.push(GameObj { 
                layer: o["layer"].as_u64().unwrap() as u32,
                key,
                fields
            });
        }
        let gamemodemask = val["gamemodemask"].as_i64().unwrap() as i32;
        Ok(Self { gamemodemask, types, objs })
    }
}

#[basicpymethods]
#[pyclass(module="types", set_all, get_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct SprayInstance {
    pub key: Crc,
    pub tex1: Crc,
    pub tex2: Crc,
    pub unk_3: u32,
    pub width: u32,
    pub height: u32,
    pub unk_6: f32,
    pub unk_7: f32,
    pub size_w: u32,
    pub size_h: u32,
    pub scale_w: f32,
    pub scale_h: f32,
    pub delay: u32,
    pub stride_x: f32,
    pub stride_y: f32,
    pub unk_15: u32,
    pub unk_16: u32,
}

#[basicpymethods]
#[pyclass(module="types", set_all, get_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct SprayVal {
    pub position: Vector3,
    pub scale: f32,
    pub instance: u16,
    pub rotation: u16,
}

#[basicpymethods]
#[pyclass(module="types", set_all, get_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
pub struct Spray {
    instances: Vec<SprayInstance>,
    vals: Vec<SprayVal>,
}

impl Spray {
    pub const KEY: u32 = hash_string("Spray".as_bytes(), None);
    pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize, _size: usize) -> Result<Self> {
        let mut offset = offset;
        let n = u32::from_bytes::<O>(&data[offset..])? as usize;
        offset += u32::size::<O>();
        let instances: Vec<SprayInstance> = OrderedDataVec::from_bytes::<O>(&data[offset..], n)?;
        offset += instances.size::<O>();
        let n = u32::from_bytes::<O>(&data[offset..])? as usize;
        offset += u32::size::<O>();
        let vals: Vec<SprayVal> = OrderedDataVec::from_bytes::<O>(&data[offset..], n)?;
        Ok(Self { instances, vals })
    }

    pub fn size<O: Version + 'static>(&self) -> usize {
        self.instances.size::<O>() + self.vals.size::<O>() + (u32::size::<O>() * 2)
    }

    pub fn dump<O: Version + 'static>(&self) -> Result<Vec<u8>> {
        Ok((self.instances.len() as u32).dump_bytes::<O>().into_iter()
            .chain(self.instances.dump_bytes::<O>())
            .chain((self.vals.len() as u32).dump_bytes::<O>())
            .chain(self.vals.dump_bytes::<O>())
            .collect())
    }

    pub fn to_file(&self, writer: Writer) -> Result<()> {
        writer.with_extension("json").write(&to_vec_pretty(self)?)?;
        Ok(())
    }

    pub fn from_file(reader: Reader) -> Result<Self> {
        Ok(from_slice(&reader.with_extension("json").read()?)?)
    }
}

#[basicpymethods]
#[pyclass(module="types", set_all, get_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct CrowdHeader {
    pub key: Crc,
    pub key_main: Crc,
    pub key_right: Crc,
    pub key_left: Crc,
    pub unk_4: f32,
    pub animation_num: u32,
    pub instance_num: u32,
}

#[basicpymethods]
#[pyclass(module="types", set_all, get_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct CrowdVal {
    pub position: Vector3,
    pub rotation: f32,
    pub lod: f32,
}

#[basicpymethods]
#[pyclass(module="types", set_all, get_all)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
pub struct CrowdItem {
    pub header: CrowdHeader,
    pub animations: Vec<Crc>,
    pub instances: Vec<CrowdVal>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Crowd {
    pub vals: Vec<CrowdItem>
}

impl IntoPy<PyObject> for Crowd {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.vals.into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Crowd {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self { vals: Vec::extract_bound(ob)? })
    }
}

impl Crowd {
    pub const KEY: u32 = hash_string("3dCrowd".as_bytes(), None);
    pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize, _size: usize) -> Result<Self> {
        let val: u32 = OrderedData::from_bytes::<O>(&data[offset..])?;
        if val != 0x65 { return Err(anyhow!("Invalid Block Data for Crowd Block")); }
        let n: u32 = OrderedData::from_bytes::<O>(&data[offset + u32::size::<O>()..])?;
        let offs: Vec<u32> = OrderedDataVec::from_bytes::<O>(&data[offset + u32::size::<O>() * 2..], n as usize)?;
        let vals = offs.into_iter().map(|off| {
            let mut offset = offset + off as usize;
            let header: CrowdHeader = OrderedData::from_bytes::<O>(&data[offset..])?;
            offset += CrowdHeader::size::<O>();
            let animations: Vec<Crc> = OrderedDataVec::from_bytes::<O>(&data[offset..], header.animation_num as usize)?;
            offset += animations.size::<O>();
            let instances = OrderedDataVec::from_bytes::<O>(&data[offset..], header.instance_num as usize)?;
            Ok(CrowdItem { header, animations, instances })
        }).collect::<Result<Vec<_>>>()?;
        Ok(Self { vals })
    }

    pub fn size<O: Version + 'static>(&self) -> usize {
        let n = (2 + self.vals.len()) * u32::size::<O>();
        let m: usize = self.vals.iter().map(|CrowdItem { animations, instances, .. }| {
            CrowdHeader::size::<O>() + animations.size::<O>() + instances.size::<O>()
        }).sum();
        n + m
    }

    pub fn dump<O: Version + 'static>(&self) -> Result<Vec<u8>> {
        let n = self.vals.len() as u32;

        let mut vals = vec![];
        let mut offs = vec![];
        let off = (n + 2) * (u32::size::<O>() as u32);
        for CrowdItem { header, animations, instances } in &self.vals {
            offs.push(off + (vals.len() as u32));
            vals.extend(header.dump_bytes::<O>());
            vals.extend(animations.dump_bytes::<O>());
            vals.extend(instances.dump_bytes::<O>());
        }

        let mut data = Vec::with_capacity(off as usize + vals.len());
        data.extend(0x65u32.dump_bytes::<O>());
        data.extend(n.dump_bytes::<O>());
        data.extend(offs.dump_bytes::<O>());
        data.extend(vals);
        Ok(data)
    }

    pub fn to_file(&self, writer: Writer) -> Result<()> {
        writer.with_extension("json").write(&to_vec_pretty(self)?)?;
        Ok(())
    }

    pub fn from_file(reader: Reader) -> Result<Self> {
        Ok(from_slice(&reader.with_extension("json").read()?)?)
    }
}

#[basicpymethods]
#[pyclass(module="types", set_all, get_all)]
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize, PyMethods)]
pub struct AtlasUVVal {
    pub key: Crc,
    pub vals: Vector4
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PyMethods)]
#[serde(transparent)]
pub struct AtlasUV {
    pub vals: Vec<AtlasUVVal>,
}

impl IntoPy<PyObject> for AtlasUV {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.vals.into_py(py)
    }
}

impl <'py> FromPyObject<'py> for AtlasUV {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self { vals: Vec::extract_bound(ob)? })
    }
}

impl AtlasUV {
    pub const KEY1: u32 = hash_string("atlas_1.uv".as_bytes(), None);
    pub const KEY2: u32 = hash_string("atlas_2.uv".as_bytes(), None);

    pub fn from_data<O: Version + 'static>(data: &[u8], offset: usize, size: usize) -> Result<Self> {
        assert!(size%AtlasUVVal::size::<O>() == 0, "Invalid UV Atlas size");
        let num = size / AtlasUVVal::size::<O>();
        let vals = OrderedDataVec::from_bytes::<O>(&data[offset..], num)?;

        Ok(Self { vals })
    }

    pub fn dump<O: Version + 'static>(&self) -> Vec<u8> {
        self.vals.dump_bytes::<O>()
    }

    pub fn to_file(&self, writer: Writer) -> Result<()> {
        writer.with_extension("json").write(&to_vec_pretty(self)?)?;
        Ok(())
    }

    pub fn from_file(reader: Reader) -> Result<Self> {
        Ok(from_slice(&reader.with_extension("json").read()?)?)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Data {
    pub data: Vec<u8>,
}

impl IntoPy<PyObject> for Data {
    fn into_py(self, py: Python<'_>) -> PyObject {
        std::borrow::Cow::from(&self.data[..]).into_py(py)
    }
}

impl <'py> FromPyObject<'py> for Data {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self { data: Vec::extract_bound(ob)? })
    }
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

    pub fn to_file(&self, writer: Writer) -> Result<()> {
        writer.write(&self.data)?;
        Ok(())
    }

    pub fn from_file(reader: Reader) -> Result<Self> {
        Ok(Self { data: reader.read()? })
    }
}

