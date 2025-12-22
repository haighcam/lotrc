use anyhow::{anyhow, Context, Result};
#[cfg(not(feature = "python"))]
use lotrc_proc::getter;
use lotrc_proc::{make_platforms, OrderedData};
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::collections::HashMap;
use std::io::Read;
use std::ptr::NonNull;
use std::sync::Arc;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

mod wrappers;
pub use wrappers::*;

#[cfg(feature = "python")]
pub fn init(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "types")?;
    m.add_function(wrap_pyfunction!(hash_string, &m)?)?;
    m.add_class::<Matrix4x4>()?;
    m.add_class::<StringKeys>()?;
    m.add_class::<StringKeysHeader>()?;
    m.add_class::<StringKeysVal>()?;
    m.add_class::<Strings>()?;
    m.add_class::<Vector2>()?;
    m.add_class::<Vector3>()?;
    m.add_class::<Vector4>()?;
    m.add_class::<Weight>()?;
    init_pc(&m)?;
    init_xbox(&m)?;
    init_ps3(&m)?;
    Ok(m)
}
#[cfg(feature = "python")]
#[make_platforms]
pub fn init_ver(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StringsVER>()?;
    m.add_class::<StringKeysVER>()
}

pub trait OrderedDataStrict
where
    Self: Sized + Clone + Default,
    for<'a> Self: From<&'a Self::PC> + From<&'a Self::XBOX> + From<&'a Self::PS3>,
    Self::PC: Immutable
        + KnownLayout
        + FromBytes
        + IntoBytes
        + Unaligned
        + Clone
        + std::fmt::Debug
        + Default,
    Self::XBOX: Immutable
        + KnownLayout
        + FromBytes
        + IntoBytes
        + Unaligned
        + Clone
        + std::fmt::Debug
        + Default,
    Self::PS3: Immutable
        + KnownLayout
        + FromBytes
        + IntoBytes
        + Unaligned
        + Clone
        + std::fmt::Debug
        + Default,
{
    type PC;
    type XBOX;
    type PS3;
    const SIZE_PC: usize = std::mem::size_of::<Self::PC>();
    const SIZE_XBOX: usize = std::mem::size_of::<Self::XBOX>();
    const SIZE_PS3: usize = std::mem::size_of::<Self::PS3>();
}

#[derive(Default)]
pub struct ParseSlice {
    pub src: Arc<[u8]>,
    pub offset: usize,
}

impl From<Arc<[u8]>> for ParseSlice {
    fn from(src: Arc<[u8]>) -> Self {
        Self { src, offset: 0 }
    }
}

#[derive(Default)]
pub struct DumpSlice<'a> {
    pub vals: &'a mut [u8],
    pub offset: usize,
}

impl<'a, T: AsMut<[u8]>> From<&'a mut T> for DumpSlice<'a> {
    fn from(val: &'a mut T) -> Self {
        Self {
            vals: val.as_mut(),
            offset: 0,
        }
    }
}

impl<'a> DumpSlice<'a> {
    pub fn view<'b>(&'b mut self, off: usize) -> DumpSlice<'b> {
        DumpSlice {
            vals: &mut self.vals[off..],
            offset: self.offset + off,
        }
    }
    pub fn split(&mut self, off: usize) -> Self {
        let Self { vals, mut offset } = std::mem::take(self);
        let (vals, res) = vals.split_at_mut(off);
        let res = Self { vals: res, offset };
        offset += off;
        *self = Self { vals, offset };
        res
    }
    pub fn align(&mut self, size: usize) {
        let new_off = align_offset(self.offset, size);
        self.split(new_off - self.offset);
    }
}

pub trait DumpData {
    fn write_from(&mut self, val: &Self) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
    fn dump(&self) -> Result<Vec<u8>> {
        let mut data = vec![0u8; self.size()];
        let mut slice = (&mut data).into();
        self.dump_into(&mut slice)?;
        Ok(data)
    }
    fn size(&self) -> usize;
}

impl<T: Sized + KnownLayout + Immutable + IntoBytes + FromBytes> DumpData for T {
    fn write_from(&mut self, val: &Self) -> Result<()> {
        val.write_to(self.as_mut_bytes())
            .map_err(|e| anyhow!(e.to_string()))
    }
    fn dump_into<'a>(&self, dst: &mut DumpSlice) -> Result<()> {
        self.write_to_prefix(dst.vals)
            .map_err(|e| anyhow!(e.to_string()))?;
        dst.split(std::mem::size_of::<T>());
        Ok(())
    }
    fn size(&self) -> usize {
        std::mem::size_of::<T>()
    }
}

impl<T: Sized + KnownLayout + Immutable + IntoBytes + FromBytes> DumpData for [T] {
    fn write_from(&mut self, val: &Self) -> Result<()> {
        val.write_to(self.as_mut_bytes())
            .map_err(|e| anyhow!(e.to_string()))
    }
    fn dump_into<'a>(&self, dst: &mut DumpSlice) -> Result<()> {
        self.write_to_prefix(dst.vals)
            .map_err(|e| anyhow!(e.to_string()))?;
        dst.split(std::mem::size_of_val(self));
        Ok(())
    }
    fn size(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

pub fn align_offset(offset: usize, size: usize) -> usize {
    (offset + (size - 1)) & (0xFFFFFFFF - (size - 1))
}

pub trait RefFromData
where
    Self: Sized + KnownLayout + Immutable + FromBytes + IntoBytes + 'static,
{
    fn from_data(data: &[u8]) -> Result<&Self> {
        Ok(FromBytes::ref_from_prefix(data)
            .map_err(|e| anyhow!(e.to_string()))?
            .0)
    }
    fn slice_from_data(data: &[u8], count: usize) -> Result<&[Self]> {
        Ok(FromBytes::ref_from_prefix_with_elems(data, count)
            .map_err(|e| anyhow!(e.to_string()))?
            .0)
    }
    fn size_of() -> usize {
        std::mem::size_of::<Self>()
    }
    fn mut_from_data<'a>(src: &mut DumpSlice<'a>) -> Result<&'a mut Self> {
        let val = FromBytes::mut_from_bytes(src.split(std::mem::size_of::<Self>()).vals)
            .map_err(|e| anyhow!(e.to_string()))?;
        Ok(val)
    }
    fn mut_slice_from_data<'a>(src: &mut DumpSlice<'a>, count: usize) -> Result<&'a mut [Self]> {
        let val = FromBytes::mut_from_bytes_with_elems(
            src.split(std::mem::size_of::<Self>() * count).vals,
            count,
        )
        .map_err(|e| anyhow!(e.to_string()))?;
        Ok(val)
    }
}

impl<T> RefFromData for T where T: Sized + KnownLayout + Immutable + FromBytes + IntoBytes + 'static {}

pub type Color = u32;
#[make_platforms]
pub type ColorVER = U32VER;

#[derive(Default, Debug, Clone, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Crc {
    pub val: u32,
}

impl Crc {
    pub const fn get(&self) -> u32 {
        self.val
    }
}

impl From<u32> for Crc {
    fn from(value: u32) -> Self {
        Self { val: value }
    }
}
impl From<&u32> for Crc {
    fn from(value: &u32) -> Self {
        Self { val: *value }
    }
}

#[cfg(feature = "python")]
impl<'py> FromPyObject<'_, 'py> for Crc {
    type Error = PyErr;
    #[inline(always)]
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            val: u32::extract(obj)?,
        })
    }
}

#[cfg(feature = "python")]
impl<'py> IntoPyObject<'py> for Crc {
    type Target = <u32 as IntoPyObject<'py>>::Target;
    type Output = <u32 as IntoPyObject<'py>>::Output;
    type Error = <u32 as IntoPyObject<'py>>::Error;
    #[inline(always)]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.val.into_pyobject(py)
    }
}

impl From<Crc> for u32 {
    fn from(value: Crc) -> Self {
        value.val
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "types", get_all, set_all))]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "types", get_all, set_all))]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "types", get_all, set_all))]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "types", get_all, set_all))]
pub struct Matrix4x4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "types", get_all, set_all))]
pub struct Weight {
    pub x: u32,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}

lazy_static::lazy_static! {
    pub static ref CRC_VALS: Mutex<HashMap<u32, String>> = {
        //const CONQUEST_STRINGS: &str = include_str!("../res/conquest_strings.txt");
        //Mutex::new(CONQUEST_STRINGS.split('\n').map(|x| (hash_string(x.as_bytes(), None), String::from(x))).collect())
        Mutex::new(HashMap::new())
    };
}

pub fn update_crc<T>(vals: impl IntoIterator<Item = T>)
where
    String: From<T>,
    T: Sized + AsRef<[u8]>,
{
    let mut strings = CRC_VALS.lock();
    for string in vals {
        let k = hash_string(string.as_ref(), None);
        if strings.contains_key(&k) {
            continue;
        }
        strings.insert(k, String::from(string));
    }
}

pub fn get_str<'a>(val: &u32) -> Option<MappedMutexGuard<'a, str>> {
    MutexGuard::try_map(CRC_VALS.lock(), |x| x.get_mut(val).map(|x| x.as_mut_str())).ok()
}

pub fn get_str_debug(val: &u32) -> String {
    CRC_VALS
        .lock()
        .get(val)
        .cloned()
        .unwrap_or_else(|| format!("unknown string {}", val))
}

pub fn decompress_block(data: &[u8], size_comp: usize, size: usize) -> Result<Box<[u8]>> {
    //pub pak_datdda: Box<[u8]>,
    Ok(match size_comp {
        0 => (&data[..size]).into(),
        _ => {
            let mut out = Vec::with_capacity(size);
            flate2::read::ZlibDecoder::new(&data[..size_comp]).read_to_end(&mut out)?;
            out.into()
        }
    })
}

#[macro_export]
macro_rules! wrap_args {
    () => { () };
    ( $a:expr ) => { $a };
    ( $($a:expr),* ) => { ($($a),*) };
}
#[macro_export]
macro_rules! pyobj_ref {
    ($name:ident) => {
        impl<'a, 'py> IntoPyObject<'py> for &'a $name {
            type Target = <$name as IntoPyObject<'py>>::Target;
            type Output = <$name as IntoPyObject<'py>>::Output;
            type Error = <$name as IntoPyObject<'py>>::Error;
            #[inline(always)]
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                (*self).clone().into_pyobject(py)
            }
        }
    };
}

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
    0xafb010b1, 0xab710d06, 0xa6322bdf, 0xa2f33668, 0xbcb4666d, 0xb8757bda, 0xb5365d03, 0xb1f740b4,
];

const LOWERCASE_BYTES: [usize; 256] = [
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
    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
];

#[cfg_attr(feature = "python", pyfunction)]
pub const fn hash_string(string: &[u8], mask: Option<u32>) -> u32 {
    let mut h = !match mask {
        Some(val) => val,
        None => 0,
    };
    let mut i: usize = 0;
    loop {
        if i >= string.len() {
            break;
        }
        h = (h << 8) ^ HASHING_ARRAY[LOWERCASE_BYTES[string[i] as usize] ^ (h >> 24) as usize];
        i += 1;
    }
    !h
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "types"))]
#[derive(Debug, Clone)]
pub struct StringsVER {
    _ptr: Arc<[u8]>,
    strings: Box<[NonNull<str>]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(StringsVER);

#[make_platforms]
unsafe impl Sync for StringsVER {}
#[make_platforms]
unsafe impl Send for StringsVER {}

#[make_platforms]
impl StringsVER {
    pub fn from_bytes(src: &Arc<[u8]>, mut offset: usize, num: usize) -> Result<Self> {
        let mut strings = Vec::with_capacity(num);
        for i in 0..num {
            let k = U32VER::from_data(&src[offset..]).context("size")?;
            offset += 4;
            strings.push(NonNull::from_ref(
                std::str::from_utf8(&src[offset..offset + k.get() as usize])
                    .with_context(|| format!("string {} of size {}", i, k.get()))?,
            ));
            offset += k.get() as usize;
        }
        Ok(Self {
            _ptr: src.clone(),
            strings: strings.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl StringsVER {
    #[getter]
    pub fn len(&self) -> usize {
        self.strings.len()
    }
    #[getter]
    pub fn strings(&self) -> Vec<&str> {
        self.strings.iter().map(|x| unsafe { x.as_ref() }).collect()
    }
}

#[cfg_attr(feature = "python", pyclass(module = "types", get_all, set_all))]
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Strings {
    pub strings: Vec<String>,
}

#[make_platforms]
impl From<&StringsVER> for Strings {
    fn from(val: &StringsVER) -> Self {
        Self {
            strings: val.strings().into_iter().map(|x| x.to_string()).collect(),
        }
    }
}

#[make_platforms]
pub trait DumpStringsVER {
    fn strings(&self) -> impl Iterator<Item = &str>;
    fn dump_into<'a>(&self, dst: &mut DumpSlice) -> Result<()> {
        for val in self.strings() {
            let k = U32VER::mut_from_data(dst)?;
            *k = val.len().into();
            val.as_bytes().dump_into(dst)?;
        }
        Ok(())
    }
    fn size(&self) -> usize {
        self.strings().map(|x| x.len() + 4).sum::<usize>()
    }
}

#[make_platforms]
impl DumpStringsVER for StringsVER {
    fn strings(&self) -> impl Iterator<Item = &str> {
        self.strings.iter().map(|x| unsafe { x.as_ref() })
    }
}
#[make_platforms]
impl DumpStringsVER for Strings {
    fn strings(&self) -> impl Iterator<Item = &str> {
        self.strings.iter().map(|x| x.as_ref())
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "types", get_all, set_all))]
#[repr(C)]
pub struct StringKeysHeader {
    pub num_a: u16,
    pub num_b: u16,
    pub z2: u32,
    pub z3: u32,
    pub z4: u32,
    pub z5: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "types", get_all, set_all))]
#[repr(C)]
pub struct StringKeysVal {
    pub key: Crc,
    pub offset: u32,
}

#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "types"))]
#[derive(Debug, Clone)]
pub struct StringKeysVER {
    _ptr: Arc<[u8]>,
    header: NonNull<StringKeysHeaderVER>,
    vals: NonNull<[StringKeysValVER]>,
    pad: NonNull<[U32VER]>,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(StringKeysVER);

#[make_platforms]
unsafe impl Sync for StringKeysVER {}
#[make_platforms]
unsafe impl Send for StringKeysVER {}

#[make_platforms]
impl StringKeysVER {
    pub fn from_bytes(src: &Arc<[u8]>, mut offset: usize) -> Result<Self> {
        let header = StringKeysHeaderVER::from_data(&src[offset..]).context("header")?;
        assert!(header.num_a.get() == header.num_b.get(), "Seems to be true");
        offset += header.size();
        let vals = StringKeysValVER::slice_from_data(&src[offset..], header.num_a.get() as usize)
            .context("vals")?;
        offset += vals.size();
        let pad = U32VER::slice_from_data(&src[offset..], vals.len()).context("pad")?;
        Ok(Self {
            _ptr: src.clone(),
            header: header.into(),
            vals: vals.into(),
            pad: pad.into(),
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl StringKeysVER {
    #[getter]
    pub fn header(&self) -> &StringKeysHeaderVER {
        unsafe { self.header.as_ref() }
    }
    #[getter]
    pub fn vals(&self) -> &[StringKeysValVER] {
        unsafe { self.vals.as_ref() }
    }
    #[getter]
    pub fn pad(&self) -> &[U32VER] {
        unsafe { self.pad.as_ref() }
    }
}

#[cfg_attr(feature = "python", pyclass(module = "types", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct StringKeys {
    pub vals: Vec<Crc>,
}

#[make_platforms]
impl From<&StringKeysVER> for StringKeys {
    fn from(val: &StringKeysVER) -> Self {
        Self {
            vals: val.vals().iter().map(|x| x.key.into()).collect(),
        }
    }
}

#[make_platforms]
pub trait DumpStringKeysVER {
    fn keys(&self) -> impl Iterator<Item = impl Into<CrcVER>>;
    fn num(&self) -> usize;
    fn size(&self) -> usize {
        StringKeysHeaderVER::size_of()
            + self.num() * (U32VER::size_of() + StringKeysValVER::size_of())
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let num = self.num();
        let header = StringKeysHeaderVER::mut_from_data(dst).context("header")?;
        let vals = StringKeysValVER::mut_slice_from_data(dst, num).context("vals")?;
        let mut off = dst.offset;
        U32VER::mut_slice_from_data(dst, num).context("pad")?;

        header.num_a = num.into();
        header.num_b = num.into();
        for (key, val) in self.keys().zip(vals) {
            val.key = key.into();
            val.offset = off.into();
            off += size_of::<U32VER>();
        }
        Ok(())
    }
}

#[make_platforms]
impl DumpStringKeysVER for StringKeysVER {
    fn keys(&self) -> impl Iterator<Item = impl Into<CrcVER>> {
        self.vals().iter().map(|x| x.key)
    }
    fn num(&self) -> usize {
        self.vals.len()
    }
}

#[make_platforms]
impl DumpStringKeysVER for StringKeys {
    fn keys(&self) -> impl Iterator<Item = impl Into<CrcVER>> {
        self.vals.iter()
    }
    fn num(&self) -> usize {
        self.vals.len()
    }
}

#[derive(Debug, Clone)]
pub struct CompressedDataRef {
    _ptr: Arc<[u8]>,
    data: NonNull<[u8]>,
    size: usize
}

impl CompressedDataRef {
    pub fn get(&self) -> Result<Arc<[u8]>> {
        let data_comp = unsafe { self.data.as_ref() };
        Ok(match self.size {
            0 => data_comp.into(),
            x => {
                let mut out = Vec::with_capacity(x);
                flate2::read::ZlibDecoder::new(data_comp).read_to_end(&mut out)?;
                out.into()
            }
        })
    }
    pub fn empty() -> Self {
        Self {
            _ptr: Arc::new([]),
            data: NonNull::from_ref(&[]),
            size: 0
        }
    }
}
