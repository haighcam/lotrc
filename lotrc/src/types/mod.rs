
use anyhow::{anyhow, Context, Result};
use lotrc_proc::{make_endian, derive_ordered_data};
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use std::collections::HashMap;
use std::io::Read;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};
use enum_dispatch::enum_dispatch;

pub use lotrc_wrappers::*;
pub mod sub_blocks;

pub type AlignmentHelper = u32;

const _: () = assert!(std::mem::align_of::<AlignmentHelper>() == 4);

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Default, PartialEq)]
pub struct AlignedBuf {
    data: slice<AlignmentHelper>,
    size: usize
}

impl std::ops::Deref for AlignedBuf {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.data.as_bytes()[..self.size]
    }
}
impl std::ops::DerefMut for AlignedBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data.as_mut_bytes()[..self.size]
    }
}

impl AlignedBuf {
    pub fn with_capacity(size: usize) -> Self {
        let inner_size = (size + std::mem::size_of::<AlignmentHelper>() - 1) / std::mem::size_of::<AlignmentHelper>();
        let data = Box::<[AlignmentHelper]>::new_zeroed_slice(inner_size);
        let data = unsafe { data.assume_init() };
        Self { data: data.into(), size }
    }
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.size
    }
}

#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct DumpSlice<'a> {
    pub vals: mut_slice<'a, u8>,
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
impl<'a> From<&'a mut [u8]> for DumpSlice<'a> {
    fn from(vals: &'a mut [u8]) -> Self {
        Self { vals, offset: 0 }
    }
}

impl<'a> DumpSlice<'a> {
    pub fn view<'b>(&'b mut self, off: usize) -> DumpSlice<'b> {
        DumpSlice {
            vals: &mut self.vals[off..],
            offset: self.offset + off,
        }
    }
    // return a new DumpSlice of [offset, offset + off) leaving this one with [offset + off, ...)
    pub fn split(&mut self, off: usize) -> Result<Self> {
        let Self { vals, mut offset } = std::mem::take(self);
        let (vals, rem) = vals.split_at_mut_checked(off).ok_or(anyhow::anyhow!("dump slice too small"))?;
        let res = Self { vals, offset };
        offset += off;
        *self = Self { vals: rem, offset };
        Ok(res)
    }
    pub fn align(&mut self, size: usize) -> Result<()> {
        let new_off = align_offset(self.offset, size);
        self.split(new_off - self.offset).context("align")?;
        Ok(())
    }
    pub fn adjusted_align(&mut self, size: usize, adj: usize) -> Result<()> {
        let new_off = align_offset(self.offset - adj, size) + adj;
        self.split(new_off - self.offset).context("align")?;
        Ok(())
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
        dst.split(std::mem::size_of::<T>())?;
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
        dst.split(std::mem::size_of_val(self))?;
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
        let val = FromBytes::mut_from_bytes(src.split(std::mem::size_of::<Self>())?.vals)
            .map_err(|e| anyhow!(e.to_string()))?;
        Ok(val)
    }
    fn mut_slice_from_data<'a>(src: &mut DumpSlice<'a>, count: usize) -> Result<&'a mut [Self]> {
        let val = FromBytes::mut_from_bytes_with_elems(
            src.split(std::mem::size_of::<Self>() * count)?.vals,
            count,
        )
        .map_err(|e| anyhow!(e.to_string()))?;
        Ok(val)
    }
}

impl<T> RefFromData for T where T: Sized + KnownLayout + Immutable + FromBytes + IntoBytes + 'static {}

pub type Color = u32;
#[make_endian]
pub type Color_XE_ = u32_XE_;

#[derive(Default, Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Crc {
    pub val: u32,
}
#[make_endian]
pub type Crc_XE_ = u32_XE_;

impl Crc {
    pub const fn new(val: u32) -> Self {
        Self { val }
    }
    pub const fn get(&self) -> u32 {
        self.val
    }
}

impl From<u32> for Crc {
    fn from(value: u32) -> Self {
        Self { val: value }
    }
}
impl From<Crc> for u32 {
    fn from(value: Crc) -> Self {
        value.val
    }
}

impl indexmap::Equivalent<u32> for Crc {
    fn equivalent(&self, key: &u32) -> bool {
        self.val.eq(key)
    }
}
impl indexmap::Equivalent<Crc> for u32 {
    fn equivalent(&self, key: &Crc) -> bool {
        self.eq(&key.val)
    }
}

#[make_endian]
mod crc_impl_xe_ {
    use super::*;
    impl OrderedData<Crc> for U32_XE_ {
        #[inline(always)]
        fn conv(&self) -> Crc {
            self.get().into()
        }
    }
    impl OrderedData<U32_XE_> for Crc {
        #[inline(always)]
        fn conv(&self) -> U32_XE_ {
            self.get().into()
        }
    }
    impl OrderedData<Crc> for u32_XE_ {
        #[inline(always)]
        fn conv(&self) -> Crc {
            self.to_native().into()
        }
    }
    impl OrderedData<u32_XE_> for Crc {
        #[inline(always)]
        fn conv(&self) -> u32_XE_ {
            self.get().into()
        }
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Vector2_XE_ {
    pub x: f32_XE_,
    pub y: f32_XE_,
}
impl TryFrom<&[f32]> for Vector2 {
    type Error = anyhow::Error;
    fn try_from(val: &[f32]) -> Result<Self> {
        if val.len() != 2 {
            Err(anyhow!("need 2 values"))
        } else {
            Ok(Self { x: val[0], y: val[1] })
        }
    }
}
impl From<Vector2> for Vec<f32> {
    fn from(val: Vector2) -> Self {
        vec![val.x, val.y]
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Vector3_XE_ {
    pub x: f32_XE_,
    pub y: f32_XE_,
    pub z: f32_XE_,
}
impl TryFrom<&[f32]> for Vector3 {
    type Error = anyhow::Error;
    fn try_from(val: &[f32]) -> Result<Self> {
        if val.len() != 3 {
            Err(anyhow!("need 3 values"))
        } else {
            Ok(Self { x: val[0], y: val[1], z: val[2] })
        }
    }
}
impl From<Vector3> for Vec<f32> {
    fn from(val: Vector3) -> Self {
        vec![val.x, val.y, val.z]
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Vector4_XE_ {
    pub x: f32_XE_,
    pub y: f32_XE_,
    pub z: f32_XE_,
    pub w: f32_XE_,
}
impl TryFrom<&[f32]> for Vector4 {
    type Error = anyhow::Error;
    fn try_from(val: &[f32]) -> Result<Self> {
        if val.len() != 4 {
            Err(anyhow!("need 4 values"))
        } else {
            Ok(Self { x: val[0], y: val[1], z: val[2], w: val[3] })
        }
    }
}
impl From<Vector4> for Vec<f32> {
    fn from(val: Vector4) -> Self {
        vec![val.x, val.y, val.z, val.w]
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Matrix4x4_XE_ {
    pub x: Vector4_XE_,
    pub y: Vector4_XE_,
    pub z: Vector4_XE_,
    pub w: Vector4_XE_,
}
impl TryFrom<&[f32]> for Matrix4x4 {
    type Error = anyhow::Error;
    fn try_from(val: &[f32]) -> Result<Self> {
        if val.len() != 16 {
            Err(anyhow!("need 16 values"))
        } else {
            Ok(Self {
                x: Vector4 { x: val[0], y: val[1], z: val[2], w: val[3] },
                y: Vector4 { x: val[4], y: val[5], z: val[6], w: val[7] },
                z: Vector4 { x: val[8], y: val[9], z: val[10], w: val[11] },
                w: Vector4 { x: val[12], y: val[13], z: val[14], w: val[15] },
            })
        }
    }
}
impl From<Matrix4x4> for Vec<f32> {
    fn from(val: Matrix4x4) -> Self {
        vec![
            val.x.x, val.x.y, val.x.z, val.x.w,
            val.y.x, val.y.y, val.y.z, val.y.w,
            val.z.x, val.z.y, val.z.z, val.z.w,
            val.w.x, val.w.y, val.w.z, val.w.w
        ]
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Weight_XE_ {
    pub x: u32_XE_,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}
impl TryFrom<&[u32]> for Weight {
    type Error = anyhow::Error;
    fn try_from(val: &[u32]) -> Result<Self> {
        if val.len() != 5 {
            Err(anyhow!("need 16 values"))
        } else if val[1] > 255 || val[2] > 255 || val[3] > 255 || val[4] > 255 {
            Err(anyhow!("values 1-5 must be < 255"))
        } else {
            Ok(Weight { x: val[0], a: val[1] as u8, b: val[2] as u8, c: val[3] as u8, d: val[4] as u8 })
        }
    }
}
impl From<Weight> for Vec<u32> {
    fn from(val: Weight) -> Self {
        vec![val.x, val.a as u32, val.b as u32, val.c as u32, val.d as u32]
    }
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

pub fn decompress_block_into(src: &[u8], dst: &mut [u8], size_comp: usize) -> Result<()> {
    match size_comp {
        0 => dst.copy_from_slice(&src[..dst.len()]),
        _ => {
            let mut off = 0;
            let mut reader = flate2::read::ZlibDecoder::new(src);
            while off < dst.len() {
                off += reader.read(&mut dst[off..])?;
            }
        }
    }
    Ok(())
}

pub fn compress_block(src: &[u8], is_pak: bool, c: flate2::Compression) -> Result<Vec<u8>> {
    use std::io::Write; 
    Ok(if c.level() == 0 && is_pak {
        src.to_vec()
    } else {
        // let mut z = ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), c);
        z.write_all(src)?;
        let data = z.finish()?;
        if data.len() > 0xffffff {
            return Err(anyhow!("Could not compress data to be smaller than 1048575 bytes, try again with a higher compression ratio"))
        }
        data
    })
}

pub fn compress_segmented<'a>(src: impl Iterator<Item=&'a [u8]>, is_pak: bool, c: flate2::Compression) -> Result<Vec<u8>> {
    use std::io::Write; 
    Ok(if c.level() == 0 && is_pak {
        src.flatten().copied().collect()
    } else {
        // let mut z = ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), c);
        for val in src {
            z.write_all(val)?;
        }
        let data = z.finish()?;
        if data.len() > 0xffffff {
            return Err(anyhow!("Could not compress data to be smaller than 1048575 bytes, try again with a higher compression ratio"))
        }
        data
    })
}

#[macro_export]
macro_rules! wrap_args {
    () => { () };
    ( $a:expr ) => { $a };
    ( $($a:expr),* ) => { ($($a),*) };
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

#[make_endian]
#[derive(Default)]
#[repr(transparent)]
pub struct StringsRef_XE_<'a> {
    pub strings: slice<string<'a>>
}

#[make_endian]
impl<'a> StringsRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], num: usize) -> Result<Self> {
        let mut offset = 0;
        let mut strings = Vec::with_capacity(num);
        for i in 0..num {
            let k = U32_XE_::from_data(&src[offset..]).context("size")?;
            offset += 4;
            strings.push(
                std::str::from_utf8(&src[offset..offset + k.get() as usize])
                    .with_context(|| format!("string {} of size {}", i, k.get()))?
                    .into(),
            );
            offset += k.get() as usize;
        }
        Ok(Self {
            strings: strings.into_boxed_slice().into(),
        })
    }
    pub fn strings(&self) -> impl IntoIterator<Item=&str> {
        self.strings.iter().map(|x| x.as_ref())
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Strings {
    pub strings: Vec<String>,
}

#[make_endian]
impl From<&StringsRef_XE_<'_>> for Strings {
    fn from(val: &StringsRef_XE_) -> Self {
        Self {
            strings: val.strings().into_iter().map(|x| x.to_string()).collect(),
        }
    }
}

#[make_endian]
pub trait DumpStrings_XE_ {
    fn strings(&self) -> impl Iterator<Item = &str>;
    fn num_strings(&self) -> usize;
    fn dump_into<'a>(&self, dst: &mut DumpSlice) -> Result<()> {
        for val in self.strings() {
            let k = U32_XE_::mut_from_data(dst)?;
            *k = (val.len() as u32).into();
            val.as_bytes().dump_into(dst)?;
        }
        Ok(())
    }
    fn size(&self) -> usize {
        self.strings().map(|x| x.len() + 4).sum::<usize>()
    }
}

#[make_endian]
impl DumpStrings_XE_ for StringsRef_XE_<'_> {
    fn num_strings(&self) -> usize {
        self.strings.len()
    }
    fn strings(&self) -> impl Iterator<Item = &str> {
        self.strings.iter().map(|x| x.as_ref())
    }
}

#[make_endian]
impl DumpStrings_XE_ for Strings {
    fn num_strings(&self) -> usize {
        self.strings.len()
    }
    fn strings(&self) -> impl Iterator<Item = &str> {
        self.strings.iter().map(|x| x.as_ref())
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StringKeysHeader_XE_ {
    pub num_a: u16_XE_,
    pub num_b: u16_XE_,
    pub z2: u32_XE_,
    pub z3: u32_XE_,
    pub z4: u32_XE_,
    pub z5: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StringKeysVal_XE_ {
    pub key: Crc_XE_,
    pub offset: u32_XE_,
}

#[make_endian]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct StringKeysRef_XE_<'a> {
    pub header: &'a StringKeysHeader_XE_,
    pub vals: ref_slice<'a, StringKeysVal_XE_>,
    pub pad: ref_slice<'a, u32_XE_>,
}

#[make_endian]
impl Default for StringKeysRef_XE_<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            vals: ref_slice::default(),
            pad: ref_slice::default()
        }
    }
}

const INIT_SIZE: usize = 1024;
static INIT_BYTES: [u8; INIT_SIZE] = [0u8; INIT_SIZE];
pub fn get_default_ref<'a, T: Sized + RefFromData>() -> &'a T {
    &unsafe { (&INIT_BYTES[..]).align_to::<T>() }.1[0]
}

#[make_endian]
impl<'a> StringKeysRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = StringKeysHeader_XE_::from_data(&src[offset..]).context("header")?;
        assert!(header.num_a == header.num_b, "Seems to be true");
        offset += header.size();
        let vals = StringKeysVal_XE_::slice_from_data(&src[offset..], header.num_a.conv())
            .context("vals")?;
        offset += vals.size();
        let pad = u32_XE_::slice_from_data(&src[offset..], vals.len()).context("pad")?;
        Ok(Self { header, vals: vals.into(), pad: pad.into() })
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct StringKeys {
    pub vals: Vec<Crc>,
}

#[make_endian]
impl From<&StringKeysRef_XE_<'_>> for StringKeys {
    fn from(val: &StringKeysRef_XE_) -> Self {
        Self {
            vals: val.vals.iter().map(|x| x.key.conv()).collect(),
        }
    }
}

#[make_endian]
pub trait DumpStringKeys_XE_ {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc_XE_>);
    fn num(&self) -> usize;
    fn size(&self) -> usize {
        StringKeysHeader_XE_::size_of()
            + self.num() * (u32_XE_::size_of() + StringKeysVal_XE_::size_of())
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let num = self.num();
        let header = StringKeysHeader_XE_::mut_from_data(dst).context("header")?;
        let vals = StringKeysVal_XE_::mut_slice_from_data(dst, num).context("vals")?;
        u32_XE_::mut_slice_from_data(dst, num).context("pad")?;

        header.num_a = num.conv();
        header.num_b = num.conv();
        let mut off = std::mem::size_of::<StringKeysHeader_XE_>() + num * std::mem::size_of::<StringKeysVal_XE_>();
        self.write_keys(vals.iter_mut().map(|x| &mut x.key));
        for val in vals {
            val.offset = off.conv();
            off += size_of::<u32_XE_>();
        }
        Ok(())
    }
}

#[make_endian]
impl DumpStringKeys_XE_ for StringKeysRef_XE_<'_> {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc_XE_>) {
        for (key, val) in keys.zip(&self.vals[..]) {
            *key = val.key;
        }
    }
    fn num(&self) -> usize {
        self.vals.len()
    }
}

#[make_endian]
impl DumpStringKeys_XE_ for [u32] {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc_XE_>) {
        for (key, val) in keys.zip(self.iter()) {
            *key = val.conv();
        }
    }
    fn num(&self) -> usize {
        self.len()
    }
    
}

#[make_endian]
impl DumpStringKeys_XE_ for StringKeys {
    fn write_keys<'a>(&self, keys: impl Iterator<Item = &'a mut Crc_XE_>) {
        for (key, val) in keys.zip(&self.vals) {
            *key = val.conv();
        }
    }
    fn num(&self) -> usize {
        self.vals.len()
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct CompressedBlock {
    pub data: AlignedBuf,
    pub compressed: Vec<u8> 
}

impl CompressedBlock {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            data: AlignedBuf::with_capacity(size),
            compressed: vec![]
        }
    }
    pub fn compress(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct CompressedDataRef<'a> {
    pub data: ref_slice<'a, u8>,
    pub data_decomp: AlignedBuf
}

impl PartialEq for CompressedDataRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.data_decomp == other.data_decomp
    }
}

impl<'a> CompressedDataRef<'a> {
    pub fn from_data(src: &'a [u8], size_comp: usize, size: usize) -> Self {
        let mut data_decomp = AlignedBuf::with_capacity(size);
        let data = if size_comp == 0 {
            data_decomp.copy_from_slice(&src[..size]);
            &[] as _
        } else {
            &src[..size_comp]
        };
        Self { data: data.into(), data_decomp }
    }
    pub fn decompress(&mut self) -> Result<()> {
        if self.data.len() != 0 {
            decompress_block_into(&self.data[..], &mut self.data_decomp[..], self.data.len())?;
        }
        Ok(())
    }
    pub fn get(&self) -> &[u8] {
        &*self.data_decomp
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct CompressedDataAlt<'a> {
    pub data: ref_slice<'a, u8>,
    pub compressed_data: Vec<u8>
}

#[cfg_attr(feature = "ffi", repr(C))]
pub struct OwnedCompressedData {
    pub data: AlignedBuf,
    pub compressed_data: Vec<u8>
}

impl OwnedCompressedData {
    pub fn with_capacity(size: usize) -> Self {
        OwnedCompressedData { 
            data: AlignedBuf::with_capacity(size),
            compressed_data: Default::default() 
        }
    }
    pub fn dump_slice(&mut self) -> DumpSlice<'_> {
        (&mut self.data[..]).into()
    }
}

#[enum_dispatch(DumpCompressedData)]
#[cfg_attr(feature = "ffi", repr(C))]
pub enum CompressedData<'a> {
    Ref(&'a CompressedDataRef<'a>),
    Alt(CompressedDataAlt<'a>),
    Owned(OwnedCompressedData),
    Texture0(crate::level::texture::DumpTexture0<'a>),
    Texture1(crate::level::texture::DumpTexture1<'a>),
    None(())
}

impl<'a> CompressedData<'a> {
    pub fn is_some(&self) -> bool {
        match self {
            Self::None(_) => false,
            _ => true
        }
    }
}

impl<'a> From<Option<&'a CompressedDataRef<'a>>> for CompressedData<'a> {
    fn from(val: Option<&'a CompressedDataRef<'a>>) -> Self {
        match val {
            Some(val) if val.data_decomp.len() != 0 => val.into(),
            _ => Self::None(())
        }
    }
}

impl<'a> From<&'a [u8]> for CompressedData<'a> {
    fn from(val: &'a [u8]) -> Self {
        Self::Alt(CompressedDataAlt { data: val, compressed_data: Default::default() })
    }
}

#[enum_dispatch]
pub trait DumpCompressedData {
    fn compress(&mut self, is_pak: bool, c: flate2::Compression) -> Result<()>;
    fn size_comp(&self) -> usize;
    fn size(&self) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
}

impl DumpCompressedData for &'_ CompressedDataRef<'_> {
    fn compress(&mut self, _is_pak: bool, _c: flate2::Compression) -> Result<()> {
        Ok(())
    }
    fn size_comp(&self) -> usize {
        self.data.len()
    }
    fn size(&self) -> usize {
        self.data_decomp.len()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        (&self.data[..]).dump_into(dst)
    }
}

impl DumpCompressedData for CompressedDataAlt<'_> {
    fn compress(&mut self, is_pak: bool, c: flate2::Compression) -> Result<()> {
        self.compressed_data = compress_block(self.data, is_pak, c)?;
        Ok(())
    }
    fn size_comp(&self) -> usize {
        self.compressed_data.len()
    }
    fn size(&self) -> usize {
        self.data.len()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        (&self.compressed_data).dump_into(dst)
    }
}

impl DumpCompressedData for OwnedCompressedData {
    fn compress(&mut self, is_pak: bool, c: flate2::Compression) -> Result<()> {
        self.compressed_data = compress_block(&self.data, is_pak, c)?;
        Ok(())
    }
    fn size_comp(&self) -> usize {
        self.compressed_data.len()
    }
    fn size(&self) -> usize {
        self.data.len()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        (&self.compressed_data).dump_into(dst)
    }
}


impl DumpCompressedData for () {
    fn compress(&mut self, _is_pak: bool, _c: flate2::Compression) -> Result<()> {
        Ok(())
    }
    fn size_comp(&self) -> usize {
        0
    }
    fn size(&self) -> usize {
        0
    }
    fn dump_into(&self, _dst: &mut DumpSlice) -> Result<()> {
        Ok(())
    }

}

#[make_endian]
pub struct DumpCompressedDataImpl_XE_<'a, D: DumpCompressedData> {
    pub data: &'a mut D,
    pub offset: &'a mut u32_XE_,
    pub size: &'a mut u32_XE_,
    pub size_comp: &'a mut u32_XE_
}

#[cfg(feature="ffi")]
pub unsafe fn c_str_ptr(s: Option<&std::ffi::c_char>) -> &str {
    s.and_then(|s| 
        unsafe { std::ffi::CStr::from_ptr(s as _) }.to_str().ok()
    ).unwrap_or_default()
}

