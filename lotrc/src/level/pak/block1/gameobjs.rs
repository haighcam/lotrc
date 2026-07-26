use crate::types::{
    get_default_ref, align_offset, get_str_debug, hash_string, Crc, DumpData, DumpSlice, Matrix4x4, RefFromData,
    Vector2, Vector3, Vector4, Weight, OrderedData, ref_slice, slice, string 
};
#[make_endian]
use crate::types::{
    Crc_XE_, Matrix4x4_XE_, Vector2_XE_, Vector3_XE_, Vector4_XE_, Weight_XE_, f32_XE_, i32_XE_, u32_XE_, u16_XE_, U32_XE_, U16_XE_ 
};
use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use enum_dispatch::enum_dispatch;
use lotrc_proc::{make_endian, derive_ordered_data};

// probably need to switch the other list types to be unaligned, since they could come after a
// string and get their alignment messed up

pub mod keys {
    use super::*;
    pub const CRC_KEY: u32 = hash_string("Crc".as_bytes(), None);
    pub const GUID_KEY: u32 = hash_string("GUID".as_bytes(), None);
    pub const COLOR_KEY: u32 = hash_string("Color".as_bytes(), None);
    pub const VECTOR2_KEY: u32 = hash_string("Vector2".as_bytes(), None);
    pub const VECTOR3_KEY: u32 = hash_string("Vector3".as_bytes(), None);
    pub const VECTOR4_KEY: u32 = hash_string("Vector4".as_bytes(), None);
    pub const MATRIX4X4_KEY: u32 = hash_string("Matrix4x4".as_bytes(), None);
    pub const FLOAT_KEY: u32 = hash_string("Float".as_bytes(), None);
    pub const INT_KEY: u32 = hash_string("Int".as_bytes(), None);
    pub const BOOL_KEY: u32 = hash_string("Bool".as_bytes(), None);
    pub const STRING_KEY: u32 = hash_string("String".as_bytes(), None);
    pub const STRINGLIST_KEY: u32 = hash_string("StringList".as_bytes(), None);
    pub const OBJECTLIST_KEY: u32 = hash_string("ObjectList".as_bytes(), None);
    pub const NODELIST_KEY: u32 = hash_string("NodeList".as_bytes(), None);
    pub const INTLIST_KEY: u32 = hash_string("IntList".as_bytes(), None);
    pub const CRCLIST_KEY: u32 = hash_string("CrcList".as_bytes(), None);
    pub const WEIGHTLIST_KEY: u32 = hash_string("WeightList".as_bytes(), None);
    pub const MATRIXLIST_KEY: u32 = hash_string("MatrixList".as_bytes(), None);
}

#[make_endian]
#[derive(Debug, Default, Clone, zerocopy::Immutable, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::KnownLayout, zerocopy::Unaligned)]
#[repr(C)]
struct List_XE_ {
    num: U16_XE_,
    offset: U16_XE_
}

#[make_endian]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum BaseTypeRef_XE_<'a> {
    Crc(&'a Crc_XE_),
    GUID(&'a u32_XE_),
    Color(&'a u32_XE_),
    Vector2(&'a Vector2_XE_),
    Vector3(&'a Vector3_XE_),
    Vector4(&'a Vector4_XE_),
    Matrix4x4(&'a Matrix4x4_XE_),
    Float(&'a f32_XE_),
    Int(&'a i32_XE_),
    Bool(&'a u32_XE_),
    String(string<'a>),
    StringList(slice<string<'a>>),
    ObjectList(ref_slice<'a, U32_XE_>),
    NodeList(ref_slice<'a, Vector4_XE_>),
    IntList(ref_slice<'a, i32_XE_>),
    CrcList(ref_slice<'a, U32_XE_>),
    WeightList(ref_slice<'a, Weight_XE_>),
    MatrixList(ref_slice<'a, Matrix4x4_XE_>),
}

#[make_endian]
impl<'a> BaseTypeRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], kind: u32) -> Result<Self> {
        Ok(match kind {
            keys::CRC_KEY => {
                Self::Crc(Crc_XE_::from_data(src).context("val")?.into())
            }
            keys::GUID_KEY => {
                Self::GUID(u32_XE_::from_data(src).context("val")?.into())
            }
            keys::COLOR_KEY => {
                Self::Color(u32_XE_::from_data(src).context("val")?.into())
            }
            keys::VECTOR2_KEY => Self::Vector2(
                Vector2_XE_::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::VECTOR3_KEY => Self::Vector3(
                Vector3_XE_::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::VECTOR4_KEY => Self::Vector4(
                Vector4_XE_::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::MATRIX4X4_KEY => Self::Matrix4x4(
                Matrix4x4_XE_::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::FLOAT_KEY => {
                Self::Float(f32_XE_::from_data(src).context("val")?.into())
            }
            keys::INT_KEY => {
                Self::Int(i32_XE_::from_data(src).context("val")?.into())
            }
            keys::BOOL_KEY => {
                Self::Bool(u32_XE_::from_data(src).context("val")?.into())
            }
            keys::STRING_KEY => Self::String({
                let val = List_XE_::from_data(src).context("val")?;
                let off = val.offset.get() as usize + val.size();
                core::str::from_utf8(&src[off..off + val.num.get() as usize])?
                    .into()
            }),
            keys::STRINGLIST_KEY => Self::StringList({
                let val = List_XE_::from_data(src).context("val")?;
                let vals = List_XE_::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("list info")?;
                vals.iter()
                    .enumerate()
                    .map(|(i, v)| {
                        Ok({
                            let off =
                                (val.offset.get() + v.offset.get()) as usize + val.size() * (i + 2);
                            let s_data = &src[off..off + v.num.get() as usize];
                            let s = std::str::from_utf8(s_data)
                                .with_context(|| format!("string {}, {:?}", i, s_data))?;
                            string::from(s)
                        })
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into_boxed_slice()
                    .into()
            }),
            keys::OBJECTLIST_KEY => Self::ObjectList({
                let val = List_XE_::from_data(src).context("val")?;
                U32_XE_::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::NODELIST_KEY => Self::NodeList({
                let val = List_XE_::from_data(src).context("val")?;
                Vector4_XE_::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::INTLIST_KEY => Self::IntList({
                let val = List_XE_::from_data(src).context("val")?;
                i32_XE_::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::CRCLIST_KEY => Self::CrcList({
                let val = List_XE_::from_data(src).context("val")?;
                U32_XE_::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::WEIGHTLIST_KEY => Self::WeightList({
                let val = List_XE_::from_data(src).context("val")?;
                Weight_XE_::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::MATRIXLIST_KEY => Self::MatrixList({
                let val = List_XE_::from_data(src).context("val")?;
                Matrix4x4_XE_::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            _ => return Err(anyhow!("Unkown Type {:?}", kind)),
        })
    }
    pub fn size(kind: u32) -> usize {
        match kind {
            keys::CRC_KEY => u32_XE_::size_of(),
            keys::GUID_KEY => u32_XE_::size_of(),
            keys::COLOR_KEY => u32_XE_::size_of(),
            keys::VECTOR2_KEY => Vector2_XE_::size_of(),
            keys::VECTOR3_KEY => Vector3_XE_::size_of(),
            keys::VECTOR4_KEY => Vector4_XE_::size_of(),
            keys::MATRIX4X4_KEY => Matrix4x4_XE_::size_of(),
            keys::FLOAT_KEY => f32_XE_::size_of(),
            keys::INT_KEY => u32_XE_::size_of(),
            keys::BOOL_KEY => u32_XE_::size_of(),
            keys::STRING_KEY => List_XE_::size_of(),
            keys::STRINGLIST_KEY => List_XE_::size_of(),
            keys::OBJECTLIST_KEY => List_XE_::size_of(),
            keys::NODELIST_KEY => List_XE_::size_of(),
            keys::INTLIST_KEY => List_XE_::size_of(),
            keys::CRCLIST_KEY => List_XE_::size_of(),
            keys::WEIGHTLIST_KEY => List_XE_::size_of(),
            keys::MATRIXLIST_KEY => List_XE_::size_of(),
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BaseType {
    Crc(Crc),
    GUID(u32),
    Color(u32),
    Vector2(Vector2),
    Vector3(Vector3),
    Vector4(Vector4),
    Matrix4x4(Matrix4x4),
    Float(f32),
    Int(i32),
    Bool(u32),
    String(String),
    StringList(Vec<String>),
    ObjectList(Vec<u32>),
    NodeList(Vec<Vector4>),
    IntList(Vec<i32>),
    CrcList(Vec<Crc>),
    WeightList(Vec<Weight>),
    MatrixList(Vec<Matrix4x4>),
}

#[make_endian]
impl From<&BaseTypeRef_XE_<'_>> for BaseType {
    fn from(val: &BaseTypeRef_XE_) -> Self {
        match val {
            BaseTypeRef_XE_::Crc(val) => Self::Crc(val.conv()),
            BaseTypeRef_XE_::GUID(val) => Self::GUID(val.conv()),
            BaseTypeRef_XE_::Color(val) => Self::Color(val.conv()),
            BaseTypeRef_XE_::Vector2(val) => Self::Vector2(val.conv()),
            BaseTypeRef_XE_::Vector3(val) => Self::Vector3(val.conv()),
            BaseTypeRef_XE_::Vector4(val) => Self::Vector4(val.conv()),
            BaseTypeRef_XE_::Matrix4x4(val) => Self::Matrix4x4(val.conv()),
            BaseTypeRef_XE_::Float(val) => Self::Float(val.conv()),
            BaseTypeRef_XE_::Int(val) => Self::Int(val.conv()),
            BaseTypeRef_XE_::Bool(val) => Self::Bool(val.conv()),
            BaseTypeRef_XE_::String(val) => Self::String(val.to_string()),
            BaseTypeRef_XE_::StringList(vals) => {
                Self::StringList(vals.iter().map(|x| x.to_string()).collect())
            }
            BaseTypeRef_XE_::ObjectList(vals) => {
                Self::ObjectList(vals.iter().map(|x| x.conv()).collect())
            }
            BaseTypeRef_XE_::NodeList(vals) => {
                Self::NodeList(vals.iter().map(|x| x.conv()).collect())
            }
            BaseTypeRef_XE_::IntList(vals) => Self::IntList(vals.iter().map(|x| x.conv()).collect()),
            BaseTypeRef_XE_::CrcList(vals) => Self::CrcList(vals.iter().map(|x| x.conv()).collect()),
            BaseTypeRef_XE_::WeightList(vals) => {
                Self::WeightList(vals.iter().map(|x| x.conv()).collect())
            }
            BaseTypeRef_XE_::MatrixList(vals) => {
                Self::MatrixList(vals.iter().map(|x| x.conv()).collect())
            }
        }
    }
}

#[make_endian]
#[enum_dispatch(DumpBaseType_XE_)]
pub enum BaseType_XE_<'a> {
    Ref(BaseTypeRef_XE_<'a>),
    Owned(BaseType)
}

#[make_endian]
pub enum BaseTypeDump_XE_<'a> {
    Crc(&'a mut Crc_XE_),
    GUID(&'a mut u32_XE_),
    Color(&'a mut u32_XE_),
    Vector2(&'a mut Vector2_XE_),
    Vector3(&'a mut Vector3_XE_),
    Vector4(&'a mut Vector4_XE_),
    Matrix4x4(&'a mut Matrix4x4_XE_),
    Float(&'a mut f32_XE_),
    Int(&'a mut i32_XE_),
    Bool(&'a mut u32_XE_),
    String(&'a mut [u8]),
    StringList(Vec<&'a mut [u8]>),
    ObjectList(&'a mut [U32_XE_]),
    NodeList(&'a mut [Vector4_XE_]),
    IntList(&'a mut [i32_XE_]),
    CrcList(&'a mut [U32_XE_]),
    WeightList(&'a mut [Weight_XE_]),
    MatrixList(&'a mut [Matrix4x4_XE_]),
}

#[make_endian]
pub trait DumpBaseTypeImpl_XE_ {
    fn list_len(&self) -> usize;
    fn string_lens(&self) -> Option<impl Iterator<Item = usize>>;
    fn write_into(&self, other: BaseTypeDump_XE_) -> Result<()>;
    fn off_size(&self) -> usize;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpBaseType_XE_ {
    fn dump_into(&self, obj_dst: &mut DumpSlice, val_dst: &mut DumpSlice, kind: u32) -> Result<()>;
    fn off_size(&self) -> usize;
}

#[make_endian]
impl<T: DumpBaseTypeImpl_XE_> DumpBaseType_XE_ for T {
    fn dump_into(&self, obj_dst: &mut DumpSlice, val_dst: &mut DumpSlice, kind: u32) -> Result<()> {
        let val = match kind {
            keys::CRC_KEY => {
                BaseTypeDump_XE_::Crc(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::GUID_KEY => {
                BaseTypeDump_XE_::GUID(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::COLOR_KEY => {
                BaseTypeDump_XE_::Color(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::VECTOR2_KEY => {
                BaseTypeDump_XE_::Vector2(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::VECTOR3_KEY => {
                BaseTypeDump_XE_::Vector3(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::VECTOR4_KEY => {
                BaseTypeDump_XE_::Vector4(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::MATRIX4X4_KEY => {
                BaseTypeDump_XE_::Matrix4x4(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::FLOAT_KEY => {
                BaseTypeDump_XE_::Float(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::INT_KEY => {
                BaseTypeDump_XE_::Int(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::BOOL_KEY => {
                BaseTypeDump_XE_::Bool(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::STRING_KEY => {
                let val = List_XE_::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                let vals = u8::mut_slice_from_data(val_dst, self.list_len()).context("vals")?;
                if vals.len() != 0 {
                    val_dst.split(1).context("string align")?;
                }
                BaseTypeDump_XE_::String(vals)
            }
            keys::STRINGLIST_KEY => {
                let val = List_XE_::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                let mut offset = val_dst.offset;
                let vals =
                    List_XE_::mut_slice_from_data(val_dst, self.list_len()).context("sub lists")?;
                let mut valss = Vec::with_capacity(vals.len());
                for (i, (len, val)) in self.string_lens().unwrap().zip(vals).enumerate() {
                    offset += List_XE_::size_of();
                    val.offset = (val_dst.offset - offset).conv();
                    val.num = len.conv();
                    let s = u8::mut_slice_from_data(val_dst, len)
                        .with_context(|| format!("val {}", i))?;
                    if s.len() != 0 {
                        val_dst.split(1).context("string align")?;
                    }
                    valss.push(s);
                }
                BaseTypeDump_XE_::StringList(valss)
            }
            keys::OBJECTLIST_KEY => {
                let val = List_XE_::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                BaseTypeDump_XE_::ObjectList(
                    RefFromData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::NODELIST_KEY => {
                let val = List_XE_::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                BaseTypeDump_XE_::NodeList(
                    RefFromData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::INTLIST_KEY => {
                let val = List_XE_::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                let vals = i32_XE_::mut_slice_from_data(val_dst, self.list_len()).context("vals")?;
                // should this be here or at the start of somthing else?
                val_dst.align(16)?;
                BaseTypeDump_XE_::IntList(vals)
            }
            keys::CRCLIST_KEY => {
                let val = List_XE_::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                BaseTypeDump_XE_::CrcList(
                    RefFromData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::WEIGHTLIST_KEY => {
                let val = List_XE_::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                BaseTypeDump_XE_::WeightList(
                    RefFromData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::MATRIXLIST_KEY => {
                let val = List_XE_::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                BaseTypeDump_XE_::MatrixList(
                    RefFromData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            _ => return Err(anyhow!("Unkown Type {:?}", kind)),
        };
        self.write_into(val)
    }
    #[inline(always)]
    fn off_size(&self) -> usize {
        DumpBaseTypeImpl_XE_::off_size(self)
    }
}

#[make_endian]
impl DumpBaseTypeImpl_XE_ for BaseTypeRef_XE_<'_> {
    fn string_lens(&self) -> Option<impl Iterator<Item = usize>> {
        match self {
            Self::StringList(vals) => Some(vals.iter().map(|x| x.len())),
            _ => None,
        }
    }
    fn list_len(&self) -> usize {
        match self {
            Self::String(val) => val.len(),
            Self::StringList(vals) => vals.len(),
            Self::ObjectList(vals) => vals.len(),
            Self::NodeList(vals) => vals.len(),
            Self::IntList(vals) => vals.len(),
            Self::CrcList(vals) => vals.len(),
            Self::WeightList(vals) => vals.len(),
            Self::MatrixList(vals) => vals.len(),
            _ => 0,
        }
    }
    fn off_size(&self) -> usize {
        match self {
            Self::String(vals) => {
                if vals.len() != 0 {
                    vals.len() + 1
                } else {
                    0
                }
            }
            Self::StringList(vals) => {
                let mut s = vals.len() * List_XE_::size_of();
                for v in &vals[..] {
                    if v.len() != 0 {
                        s += v.len() + 1;
                    }
                }
                s
            }
            Self::ObjectList(vals) => vals.size(),
            Self::NodeList(vals) => vals.size(),
            Self::IntList(vals) => vals.size(),
            Self::CrcList(vals) => vals.size(),
            Self::WeightList(vals) => vals.size(),
            Self::MatrixList(vals) => vals.size(),
            _ => 0,
        }
    }
    fn write_into(&self, other: BaseTypeDump_XE_) -> Result<()> {
        match (self, other) {
            (Self::Crc(val), BaseTypeDump_XE_::Crc(other)) => other.write_from(val),
            (Self::GUID(val), BaseTypeDump_XE_::GUID(other)) => other.write_from(val),
            (Self::Color(val), BaseTypeDump_XE_::Color(other)) => other.write_from(val),
            (Self::Vector2(val), BaseTypeDump_XE_::Vector2(other)) => other.write_from(val),
            (Self::Vector3(val), BaseTypeDump_XE_::Vector3(other)) => other.write_from(val),
            (Self::Vector4(val), BaseTypeDump_XE_::Vector4(other)) => other.write_from(val),
            (Self::Matrix4x4(val), BaseTypeDump_XE_::Matrix4x4(other)) => other.write_from(val),
            (Self::Float(val), BaseTypeDump_XE_::Float(other)) => other.write_from(val),
            (Self::Int(val), BaseTypeDump_XE_::Int(other)) => other.write_from(val),
            (Self::Bool(val), BaseTypeDump_XE_::Bool(other)) => other.write_from(val),
            (Self::String(val), BaseTypeDump_XE_::String(other)) => other.write_from(val.as_bytes()),
            (Self::StringList(val), BaseTypeDump_XE_::StringList(other)) => {
                Ok(for (val, other) in val.iter().zip(other) {
                    other.write_from(val.as_bytes())?;
                })
            }
            (Self::ObjectList(val), BaseTypeDump_XE_::ObjectList(other)) => other.write_from(val),
            (Self::NodeList(val), BaseTypeDump_XE_::NodeList(other)) => other.write_from(val),
            (Self::IntList(val), BaseTypeDump_XE_::IntList(other)) => other.write_from(val),
            (Self::CrcList(val), BaseTypeDump_XE_::CrcList(other)) => other.write_from(val),
            (Self::WeightList(val), BaseTypeDump_XE_::WeightList(other)) => other.write_from(val),
            (Self::MatrixList(val), BaseTypeDump_XE_::MatrixList(other)) => other.write_from(val),
            _ => Err(anyhow!("missmatched BaseTypeRef and BaseTypeDump")),
        }
    }
}

#[make_endian]
impl DumpBaseTypeImpl_XE_ for BaseType {
    fn string_lens(&self) -> Option<impl Iterator<Item = usize>> {
        match self {
            BaseType::StringList(vals) => Some(vals.iter().map(|x| x.len())),
            _ => None,
        }
    }
    fn list_len(&self) -> usize {
        match self {
            BaseType::String(val) => val.len(),
            BaseType::StringList(vals) => vals.len(),
            BaseType::ObjectList(vals) => vals.len(),
            BaseType::NodeList(vals) => vals.len(),
            BaseType::IntList(vals) => vals.len(),
            BaseType::CrcList(vals) => vals.len(),
            BaseType::WeightList(vals) => vals.len(),
            BaseType::MatrixList(vals) => vals.len(),
            _ => 0,
        }
    }
    fn off_size(&self) -> usize {
        match self {
            BaseType::String(vals) => {
                if vals.len() != 0 {
                    vals.len() + 1
                } else {
                    0
                }
            }
            BaseType::StringList(vals) => {
                let mut s = vals.len() * List_XE_::size_of();
                for v in vals {
                    if v.len() != 0 {
                        s += v.len() + 1;
                    }
                }
                s
            }
            BaseType::ObjectList(vals) => vals.len() * u32_XE_::size_of(),
            BaseType::NodeList(vals) => vals.len() * Vector4_XE_::size_of(),
            BaseType::IntList(vals) => vals.len() * i32_XE_::size_of(),
            BaseType::CrcList(vals) => vals.len() * Crc_XE_::size_of(),
            BaseType::WeightList(vals) => vals.len() * Weight_XE_::size_of(),
            BaseType::MatrixList(vals) => vals.len() * Matrix4x4_XE_::size_of(),
            _ => 0,
        }
    }
    fn write_into(&self, other: BaseTypeDump_XE_) -> Result<()> {
        match (self, other) {
            (BaseType::Crc(val), BaseTypeDump_XE_::Crc(other)) => *other = val.conv(),
            (BaseType::GUID(val), BaseTypeDump_XE_::GUID(other)) => *other = val.conv(),
            (BaseType::Color(val), BaseTypeDump_XE_::Color(other)) => *other = val.conv(),
            (BaseType::Vector2(val), BaseTypeDump_XE_::Vector2(other)) => *other = val.conv(),
            (BaseType::Vector3(val), BaseTypeDump_XE_::Vector3(other)) => *other = val.conv(),
            (BaseType::Vector4(val), BaseTypeDump_XE_::Vector4(other)) => *other = val.conv(),
            (BaseType::Matrix4x4(val), BaseTypeDump_XE_::Matrix4x4(other)) => *other = val.conv(),
            (BaseType::Float(val), BaseTypeDump_XE_::Float(other)) => *other = val.conv(),
            (BaseType::Int(val), BaseTypeDump_XE_::Int(other)) => *other = val.conv(),
            (BaseType::Bool(val), BaseTypeDump_XE_::Bool(other)) => *other = val.conv(),
            (BaseType::String(val), BaseTypeDump_XE_::String(other)) => {
                other.write_from(val.as_bytes())?
            }
            (BaseType::StringList(val), BaseTypeDump_XE_::StringList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    other.write_from(val.as_bytes())?;
                }
            }
            (BaseType::ObjectList(val), BaseTypeDump_XE_::ObjectList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            (BaseType::NodeList(val), BaseTypeDump_XE_::NodeList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            (BaseType::IntList(val), BaseTypeDump_XE_::IntList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            (BaseType::CrcList(val), BaseTypeDump_XE_::CrcList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            (BaseType::WeightList(val), BaseTypeDump_XE_::WeightList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            (BaseType::MatrixList(val), BaseTypeDump_XE_::MatrixList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            _ => return Err(anyhow!("missmatched BaseTypeRef and BaseTypeDump")),
        }
        Ok(())
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GameObjsHeader_XE_ {
    pub const_: u32_XE_,
    pub types_num: u32_XE_,
    pub types_offset: u32_XE_,
    pub obj_num: u32_XE_,
    pub obj_offset: u32_XE_,
    pub z5: u32_XE_,
    pub z6: u32_XE_,
    pub z7: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TypeHeader_XE_ {
    pub key: Crc_XE_,
    pub size: u32_XE_,
    pub fields: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TypeField_XE_ {
    pub key: Crc_XE_,
    pub kind: Crc_XE_,
    pub offset: u32_XE_,
}

pub trait TypeFieldDump {
    fn key(&self) -> u32;
    fn kind(&self) -> u32;
    fn offset(&self) -> u32;
}

#[make_endian]
impl TypeFieldDump for TypeField_XE_ {
    fn key(&self) -> u32 {
        self.key.into()
    }
    fn kind(&self) -> u32 {
        self.kind.into()
    }
    fn offset(&self) -> u32 {
        self.offset.into()
    }
}

impl TypeFieldDump for TypeField {
    fn key(&self) -> u32 {
        self.key.get()
    }
    fn kind(&self) -> u32 {
        self.kind.get()
    }
    fn offset(&self) -> u32 {
        self.offset
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Clone, PartialEq)]
pub struct TypeRef_XE_<'a> {
    pub header: &'a TypeHeader_XE_,
    pub fields: ref_slice<'a, TypeField_XE_>,
}

#[make_endian]
impl Default for TypeRef_XE_<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            fields: ref_slice::default()
        }
    }
}

#[make_endian]
impl<'a> TypeRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let header = TypeHeader_XE_::from_data(src).context("header")?;
        let fields = TypeField_XE_::slice_from_data(&src[header.size()..], header.size.conv())
            .context("fields")?.into();
        Ok(Self { header, fields })
    }
}

pub type Type = Vec<TypeField>;
#[make_endian]
impl From<TypeRef_XE_<'_>> for Type {
    fn from(val: TypeRef_XE_) -> Self {
        val.fields.iter().map(|x| x.conv()).collect()
    }
}

#[make_endian]
pub trait DumpTypeImpl_XE_ {
    fn fields_len(&self) -> usize;
    fn fields(&self) -> impl Iterator<Item = &impl TypeFieldDump>;
    fn write_header(&self, header: &mut TypeHeader_XE_) -> Result<()>;
    fn write_fields(&self, fields: &mut [TypeField_XE_]) -> Result<()>;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpType_XE_ {
    fn fields_len(&self) -> usize;
    fn get_info(&self) -> (usize, Vec<(u32, u32, u32)>);
    fn dump_into(&self, dst: &mut DumpSlice, key: u32) -> Result<()>; 
    fn size(&self) -> usize; 
}

#[make_endian]
impl<T: DumpTypeImpl_XE_> DumpType_XE_ for T {
    #[inline(always)]
    fn fields_len(&self) -> usize {
        DumpTypeImpl_XE_::fields_len(self)
    }
    fn get_info(&self) -> (usize, Vec<(u32, u32, u32)>) {
        let mut off = 0;
        let infos = self.fields().map(|t| {
            off = off.max(t.offset() as usize + BaseTypeRef_XE_::size(t.kind()));
            (t.key(), t.kind(), t.offset())
        }).collect();
        (off, infos)
    }
    fn dump_into(&self, dst: &mut DumpSlice, key: u32) -> Result<()> {
        let header = TypeHeader_XE_::mut_from_data(dst).context("header")?;
        self.write_header(header).context("write header")?;
        header.key = key.conv();
        let fields = TypeField_XE_::mut_slice_from_data(dst, self.fields_len()).context("fields")?;
        self.write_fields(fields).context("write fields")?;
        Ok(())
    }
    fn size(&self) -> usize {
        TypeHeader_XE_::size_of() + self.fields_len() * TypeField_XE_::size_of()
    }
}

#[make_endian]
impl DumpTypeImpl_XE_ for TypeRef_XE_<'_> {
    fn fields_len(&self) -> usize {
        self.fields.len()
    }
    fn fields(&self) -> impl Iterator<Item = &impl TypeFieldDump> {
        self.fields.iter()
    }
    fn write_header(&self, header: &mut TypeHeader_XE_) -> Result<()> {
        header.write_from(self.header)
    }
    fn write_fields(&self, fields: &mut [TypeField_XE_]) -> Result<()> {
        fields.write_from(&self.fields[..])
    }
}

#[make_endian]
impl DumpTypeImpl_XE_ for Type {
    fn fields_len(&self) -> usize {
        self.len()
    }
    fn fields(&self) -> impl Iterator<Item = &impl TypeFieldDump> {
        self.iter()
    }
    fn write_header(&self, header: &mut TypeHeader_XE_) -> Result<()> {
        header.size = self.len().conv();
        header.fields = 0u32.conv();
        Ok(())
    }
    fn write_fields(&self, fields: &mut [TypeField_XE_]) -> Result<()> {
        for (src, dst) in self.iter().zip(fields) {
            *dst = src.conv()
        }
        Ok(())
    }
}

#[make_endian]
#[enum_dispatch(DumpType_XE_)]
pub enum Type_XE_<'a> {
    Ref(TypeRef_XE_<'a>),
    Owned(Type)
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ObjHeader_XE_ {
    pub layer: u32_XE_,
    pub key: Crc_XE_,
    pub size: u16_XE_,
    pub z3: u16_XE_,
    pub z4: u32_XE_,
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ObjRef_XE_<'a> {
    pub header: &'a ObjHeader_XE_,
    pub fields: IndexMap<u32, BaseTypeRef_XE_<'a>>,
}

#[make_endian]
impl Default for ObjRef_XE_<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            fields: IndexMap::default().into()
        }
    }
}

#[make_endian]
impl<'a> ObjRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], types: &IndexMap<u32, TypeRef_XE_<'_>>) -> Result<Self> {
        let mut offset = 0;
        let header = ObjHeader_XE_::from_data(&src[offset..]).context("header")?;
        offset += header.size();
        let ts = &types
            .get(&header.key.to_native())
            .ok_or(anyhow!("Missing Key {:?}", header.key.to_native()))?
            .fields;
        let mut fields = IndexMap::with_capacity(ts.len());
        for t in ts.iter() {
            fields.insert(
                t.key.into(),
                BaseTypeRef_XE_::from_data(&src[offset + t.offset.to_native() as usize..], t.kind.conv())
                    .with_context(|| {
                        format!(
                            "{}field {}",
                            if let Some(BaseTypeRef_XE_::GUID(o)) =
                                fields.get(&hash_string(b"guid", None))
                            {
                                format!("guid {}, ", o.to_native())
                            } else {
                                String::new()
                            },
                            get_str_debug(&t.key.conv())
                        )
                    })?,
            );
        }
        Ok(Self {
            header: header.into(),
            fields: fields.into(),
        })
    }
    pub fn size(&self) -> usize {
        self.header.size() + self.header.size.to_native() as usize
    }
}

#[derive(Debug, Clone)]
pub struct Obj {
    pub layer: u32,
    pub key: Crc,
    pub fields: IndexMap<Crc, BaseType>,
}

#[make_endian]
impl From<&ObjRef_XE_<'_>> for Obj {
    fn from(val: &ObjRef_XE_) -> Self {
        Self {
            layer: val.header.layer.conv(),
            key: val.header.key.conv(),
            fields: val.fields.iter().map(|(k,v)| ((*k).into(), v.into())).collect()
        }
    }
}

#[make_endian]
pub struct ObjImpl_XE_<'a> {
    pub layer: u32,
    pub key: Crc,
    pub fields: IndexMap<Crc, BaseType_XE_<'a>>
}

#[make_endian]
impl<'a> From<ObjRef_XE_<'a>> for ObjImpl_XE_<'a> {
    fn from(mut val: ObjRef_XE_<'a>) -> Self {
        Self {
            layer: val.header.layer.conv(),
            key: val.header.key.conv(),
            fields: val.fields.drain(..).map(|(k,v)| (k.into(), v.into())).collect()
        }
    }
}

#[make_endian]
#[enum_dispatch(DumpObj_XE_)]
pub enum Obj_XE_<'a> {
    Ref(ObjRef_XE_<'a>),
    Owned(ObjImpl_XE_<'a>),
}

#[make_endian]
pub trait DumpObjImpl_XE_ {
    fn key(&self) -> u32;
    fn layer(&self) -> u32;
    fn fields(&self) -> impl Iterator<Item=&impl DumpBaseType_XE_>;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpObj_XE_ {
    fn key(&self) -> u32;
    fn dump_into(&self, dst: &mut DumpSlice, val_offset: usize, fields: &[(u32, u32, u32)]) -> Result<()>;
    fn size(&self, val_offset: usize, fields: &[(u32, u32, u32)]) -> usize;
}

#[make_endian]
impl<T: DumpObjImpl_XE_> DumpObj_XE_ for T {
    #[inline(always)]
    fn key(&self) -> u32 {
        DumpObjImpl_XE_::key(self)
    }
    fn dump_into(&self, dst: &mut DumpSlice, val_offset: usize, fields: &[(u32, u32, u32)]) -> Result<()> {
        let header = ObjHeader_XE_::mut_from_data(dst).context("header")?;
        let start = dst.offset;
        let mut obj_dst = dst.split(val_offset).context("obj data split")?;
        dst.align(16)?;
        for (f, (key, kind, offset)) in self.fields().zip(fields) {
            let mut obj_dst = obj_dst.view(*offset as usize);
            f.dump_into(&mut obj_dst, dst, *kind)
                .with_context(|| format!("field {}", key))?;
        }
        dst.align(16)?;
        header.layer = self.layer().conv();
        header.key = self.key().conv();
        header.size = (dst.offset - start).conv();
        Ok(())
    }

    fn size(&self, val_offset: usize, fields: &[(u32, u32, u32)]) -> usize {
        let mut off = align_offset(val_offset + std::mem::size_of::<ObjHeader_XE_>(), 16);
        for (f, (key, _, _)) in self.fields().zip(fields) {
            off += f.off_size();
            if *key == keys::INTLIST_KEY {
                off = align_offset(off, 16);
            }
        }
        align_offset(off, 16)
    }
}

#[make_endian]
impl DumpObjImpl_XE_ for ObjRef_XE_<'_> {
    fn key(&self) -> u32 {
        self.header.key.into()
    }
    fn layer(&self) -> u32 {
        self.header.layer.into()
    }
    fn fields(&self) -> impl Iterator<Item=&impl DumpBaseType_XE_> {
        self.fields.values()
    }
}

#[make_endian]
impl DumpObjImpl_XE_ for ObjImpl_XE_<'_> {
    fn key(&self) -> u32 {
        self.key.get()
    }
    fn layer(&self) -> u32 {
        self.layer
    }
    fn fields(&self) -> impl Iterator<Item=&impl DumpBaseType_XE_> {
        self.fields.values()
    }
}

#[make_endian]
impl DumpObjImpl_XE_ for Obj {
    fn key(&self) -> u32 {
        self.key.get()
    }
    fn layer(&self) -> u32 {
        self.layer
    }
    fn fields(&self) -> impl Iterator<Item=&impl DumpBaseType_XE_> {
        self.fields.values()
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct GameObjsRef_XE_<'a> {
    pub header: &'a GameObjsHeader_XE_,
    pub types: IndexMap<u32, TypeRef_XE_<'a>>,
    pub objs: IndexMap<u32, ObjRef_XE_<'a>>
}

#[make_endian]
impl Default for GameObjsRef_XE_<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            types: IndexMap::default().into(),
            objs: IndexMap::default().into()
        }
    }
}

#[make_endian]
impl<'a> GameObjsRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let header = GameObjsHeader_XE_::from_data(src).context("header")?;
        if header.const_ != 1296123652 {
            log::error!("Invalid gameobj block");
        }
        let mut offset: usize = header.types_offset.conv();
        let mut types = IndexMap::with_capacity(header.types_num.conv());
        for i in 0usize..header.types_num.conv() {
            let t = TypeRef_XE_::from_data(&src[offset..]).with_context(|| format!("ty {}", i))?;
            offset += t.size();
            types.insert(t.header.key.conv(), t);
        }

        offset = header.obj_offset.conv();
        let mut objs = IndexMap::with_capacity(header.obj_num.conv());
        for i in 0usize..header.obj_num.conv() {
            let o =
                ObjRef_XE_::from_data(&src[offset..], &types).with_context(|| format!("obj {}", i))?;
            offset += o.size();
            if let BaseTypeRef_XE_::GUID(guid) = o
                .fields
                .get(&hash_string(b"guid", None))
                .ok_or(anyhow!("obj missing guid"))?
            {
                objs.insert(guid.conv(), o);
            } else {
                return Err(anyhow!("obj incorrect guid type"));
            }
        }
        Ok(Self { header, objs: objs.into(), types: types.into() })
    }
}

#[derive(Debug, Clone)]
pub struct GameObjs {
    pub objs: IndexMap<u32, Obj>,
    pub types: IndexMap<Crc, Vec<TypeField>>,
}

#[make_endian]
impl From<&GameObjsRef_XE_<'_>> for GameObjs {
    fn from(val: &GameObjsRef_XE_) -> Self {
        Self {
            objs: val.objs.iter().map(|(k, v)| (*k, v.into())).collect(),
            types: val.types.iter().map(|(k, v)| ((*k).into(), v.fields.iter().map(|x| x.conv()).collect())).collect() 
        }
    }
}

#[make_endian]
pub struct GameObjsImpl_XE_<'a> {
    pub objs: IndexMap<u32, Obj_XE_<'a>>,
    pub types: IndexMap<Crc, Type_XE_<'a>>,
}

#[make_endian]
impl<'a> From<GameObjsRef_XE_<'a>> for GameObjsImpl_XE_<'a> {
    fn from(mut val: GameObjsRef_XE_<'a>) -> Self {
        Self {
            objs: val.objs.drain(..).map(|(k, v)| (k, v.into())).collect(),
            types: val.types.drain(..).map(|(k, v)| (k.into(), v.into())).collect() 
        }
    }
}

#[make_endian]
#[enum_dispatch(DumpGameObjs_XE_)]
pub enum GameObjs_XE_<'a> {
    Ref(GameObjsRef_XE_<'a>),
    Owned(GameObjsImpl_XE_<'a>)
}

#[make_endian]
pub trait DumpGameObjsImpl_XE_ {
    fn types(&self) -> impl Iterator<Item = (u32, &impl DumpType_XE_)>;
    fn types_num(&self) -> usize;
    fn objs(&self) -> impl Iterator<Item = &impl DumpObj_XE_>;
    fn objs_num(&self) -> usize;
    fn level_name(&self) -> Result<u32>;
    fn guid_order(&self, guid: &u32) -> usize;
}

pub type TypeInfos = IndexMap<u32, (usize, Vec<(u32, u32, u32)>)>;

#[make_endian]
#[enum_dispatch]
pub trait DumpGameObjs_XE_ {
    fn size(&self) -> (usize, TypeInfos);
    fn dump_into(&self, dst: &mut DumpSlice, infos: &TypeInfos) -> Result<()>;
    fn level_name(&self) -> Result<u32>;
    fn guid_order(&self, guid: &u32) -> usize;
}

#[make_endian]
impl<T: DumpGameObjsImpl_XE_> DumpGameObjs_XE_ for T {
    fn size(&self) -> (usize, TypeInfos) {
        let mut size = GameObjsHeader_XE_::size_of() + self.types_num() * TypeHeader_XE_::size_of();
        let mut fields_num = 0;
        let type_infos: TypeInfos = self.types().map(|(key, x)| {
            fields_num += x.fields_len();
            (key, x.get_info())
        }).collect();
        size += fields_num * std::mem::size_of::<TypeField_XE_>();

    
        size = align_offset(size, 16);
        for obj in self.objs() {
            let (off, info) = type_infos.get(&obj.key()).unwrap();
            size += obj.size(*off, info);
        }
        (size, type_infos)
    }
    fn dump_into(&self, dst: &mut DumpSlice, infos: &TypeInfos) -> Result<()> {
        let start = dst.offset;
        let header = GameObjsHeader_XE_::mut_from_data(dst).context("header")?;
        header.const_ = 1296123652u32.conv();

        header.types_offset = (dst.offset - start).conv();
        header.types_num = self.types_num().conv();
        for (key, ty) in self.types() {
            ty.dump_into(dst, key)
                .with_context(|| format!("type {}", key))?;
        }
        dst.align(16)?;

        header.obj_offset = (dst.offset - start).conv();
        header.obj_num = self.objs_num().conv();
        for (i, obj) in self.objs().enumerate() {
            let key = obj.key();
            let (off, info) = infos.get(&key).unwrap();
            obj.dump_into(dst, *off, info)
                .with_context(|| format!("obj {}", i))?;
        }

        Ok(())
    }
    #[inline(always)]
    fn level_name(&self) -> Result<u32> {
        DumpGameObjsImpl_XE_::level_name(self)
    }
    #[inline(always)]
    fn guid_order(&self, guid: &u32) -> usize {
        DumpGameObjsImpl_XE_::guid_order(self, guid)
    }
} 

#[make_endian]
impl DumpGameObjsImpl_XE_ for GameObjsRef_XE_<'_> {
    fn types(&self) -> impl Iterator<Item = (u32, &impl DumpType_XE_)> {
        self.types.iter().map(|(k,v)| (*k, v))
    }
    fn types_num(&self) -> usize {
        self.types.len()
    }
    fn objs(&self) -> impl Iterator<Item = &impl DumpObj_XE_> {
        self.objs.values()
    }
    fn objs_num(&self) -> usize {
        self.objs.len()
    }
    fn level_name(&self) -> Result<u32> {
        let name = self.objs.values()
            .find(|v| v.header.key == hash_string(b"templateLevel", None))
            .ok_or(anyhow!("templateLevel not found"))?
            .fields.get(&hash_string(b"name", None))
            .ok_or(anyhow!("templateLevel missing name field"))
            .and_then(|field| if let BaseTypeRef_XE_::Crc(val) = field {
                Ok(val.conv())
            } else {
                Err(anyhow!("templateObject name field is not a crc"))
            })?;
        Ok(name)
    }
    fn guid_order(&self, guid: &u32) -> usize {
        self.objs.get_index_of(guid).unwrap_or_default()
    }
}

#[make_endian]
impl DumpGameObjsImpl_XE_ for GameObjs {
    fn types(&self) -> impl Iterator<Item = (u32, &impl DumpType_XE_)> {
        self.types.iter().map(|(k,v)| (k.get(), v))
    }
    fn types_num(&self) -> usize {
        self.types.len()
    }
    fn objs(&self) -> impl Iterator<Item = &impl DumpObj_XE_> {
        self.objs.values()
    }
    fn objs_num(&self) -> usize {
        self.objs.len()
    }
    fn level_name(&self) -> Result<u32> {
        let name = self.objs.values()
            .find(|v| v.key.get() == hash_string(b"templateLevel", None))
            .ok_or(anyhow!("templateLevel not found"))?
            .fields.get(&Crc::new(hash_string(b"name", None)))
            .ok_or(anyhow!("templateLevel missing name field"))
            .and_then(|field| if let BaseType::Crc(val) = field {
                Ok(val.get())
            } else {
                Err(anyhow!("templateObject name field is not a crc"))
            })?;
        Ok(name)
    }
    fn guid_order(&self, guid: &u32) -> usize {
        self.objs.get_index_of(guid).unwrap_or_default()
    }
}

#[make_endian]
impl DumpGameObjsImpl_XE_ for GameObjsImpl_XE_<'_> {
    fn types(&self) -> impl Iterator<Item = (u32, &impl DumpType_XE_)> {
        self.types.iter().map(|(k,v)| (k.get(), v))
    }
    fn types_num(&self) -> usize {
        self.types.len()
    }
    fn objs(&self) -> impl Iterator<Item = &impl DumpObj_XE_> {
        self.objs.values()
    }
    fn objs_num(&self) -> usize {
        self.objs.len()
    }
    fn level_name(&self) -> Result<u32> {
        let template_level = self.objs.values()
            .find(|v| v.key() == hash_string(b"templateLevel", None))
            .ok_or(anyhow!("templateLevel not found"))?;
        let name = match template_level {
            Obj_XE_::Ref(val) => {
                val.fields.get(&hash_string(b"name", None))
                    .ok_or(anyhow!("templateLevel missing name field"))
                    .and_then(|field| if let BaseTypeRef_XE_::Crc(val) = field {
                        Ok(val.conv())
                    } else {
                        Err(anyhow!("templateObject name field is not a crc"))
                    })?
            },
            Obj_XE_::Owned(val) => {
                val.fields.get(&Crc::new(hash_string(b"name", None)))
                    .ok_or(anyhow!("templateLevel missing name field"))
                    .and_then(|field| match field {
                        BaseType_XE_::Ref(BaseTypeRef_XE_::Crc(val)) => Ok(val.conv()),
                        BaseType_XE_::Owned(BaseType::Crc(val)) => Ok(val.get()),
                        _ => Err(anyhow!("templateObject name field is not a crc"))
                    })?
            }
        };
        Ok(name)
    }
    fn guid_order(&self, guid: &u32) -> usize {
        self.objs.get_index_of(guid).unwrap_or_default()
    }
}

