#[cfg(not(feature = "ffi"))]
use crate::types::GetNative;
use crate::types::{
    get_default_ref, align_offset, get_str_debug, hash_string, Crc, DumpData, DumpSlice, Matrix4x4, RefFromData,
    Vector2, Vector3, Vector4, Weight, OrderedData, OrderedDataStrict, BufType, str_ref, slice, Map, MapImpl
};
#[make_platforms]
use crate::types::{
    CrcVER, Matrix4x4VER, Vector2VER, Vector3VER, Vector4VER, WeightVER, f32VER, i32VER, u32VER, u16VER, U32VER, U16VER
};
use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use lotrc_proc::{make_platforms, OrderedData};
use std::ptr::NonNull;

// probably need to switch the other list types to be unaligned, since they could come after a
// string and get their alignment messed up


#[make_platforms]
#[derive(Debug, Default, Clone, zerocopy::Immutable, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::KnownLayout, zerocopy::Unaligned)]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
struct ListVER {
    num: U16VER,
    offset: U16VER
}

#[make_platforms]
#[derive(Debug, Clone)]
pub enum BaseTypeVER {
    Crc(NonNull<CrcVER>),
    GUID(NonNull<u32VER>),
    Color(NonNull<u32VER>),
    Vector2(NonNull<Vector2VER>),
    Vector3(NonNull<Vector3VER>),
    Vector4(NonNull<Vector4VER>),
    Matrix4x4(NonNull<Matrix4x4VER>),
    Float(NonNull<f32VER>),
    Int(NonNull<i32VER>),
    Bool(NonNull<u32VER>),
    String(NonNull<str>),
    StringList(Box<[NonNull<str>]>),
    ObjectList(NonNull<[U32VER]>),
    NodeList(NonNull<[Vector4VER]>),
    IntList(NonNull<[i32VER]>),
    CrcList(NonNull<[U32VER]>),
    WeightList(NonNull<[WeightVER]>),
    MatrixList(NonNull<[Matrix4x4VER]>),
}

#[make_platforms]
#[derive(Debug, Clone)]
pub enum BaseTypeRefVER<'a> {
    Crc(&'a CrcVER),
    GUID(&'a u32VER),
    Color(&'a u32VER),
    Vector2(&'a Vector2VER),
    Vector3(&'a Vector3VER),
    Vector4(&'a Vector4VER),
    Matrix4x4(&'a Matrix4x4VER),
    Float(&'a f32VER),
    Int(&'a i32VER),
    Bool(&'a u32VER),
    String(&'a str),
    StringList(Box<[str_ref<'a>]>),
    ObjectList(&'a [U32VER]),
    NodeList(&'a [Vector4VER]),
    IntList(&'a [i32VER]),
    CrcList(&'a [U32VER]),
    WeightList(&'a [WeightVER]),
    MatrixList(&'a [Matrix4x4VER]),
}

#[cfg(feature = "ffi")]
#[make_platforms]
unsafe impl safer_ffi::layout::ReprC for BaseTypeRefVER<'_> {
    type CLayout = <safer_ffi::layout::Opaque<Self> as safer_ffi::layout::ReprC>::CLayout;
    fn is_valid(_it: &'_ Self::CLayout) -> bool {
        unreachable! {"opaque type"}
    }
}

#[make_platforms]
impl Default for BaseTypeRefVER<'_> {
    fn default() -> Self {
        Self::MatrixList(&[] as _)
    }
}

#[make_platforms]
impl<'a> BaseTypeRefVER<'a> {
    pub fn from_data(src: &'a [u8], kind: u32) -> Result<Self> {
        Ok(match kind {
            keys::CRC_KEY => {
                Self::Crc(CrcVER::from_data(src).context("val")?.into())
            }
            keys::GUID_KEY => {
                Self::GUID(u32VER::from_data(src).context("val")?.into())
            }
            keys::COLOR_KEY => {
                Self::Color(u32VER::from_data(src).context("val")?.into())
            }
            keys::VECTOR2_KEY => Self::Vector2(
                Vector2VER::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::VECTOR3_KEY => Self::Vector3(
                Vector3VER::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::VECTOR4_KEY => Self::Vector4(
                Vector4VER::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::MATRIX4X4_KEY => Self::Matrix4x4(
                Matrix4x4VER::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::FLOAT_KEY => {
                Self::Float(f32VER::from_data(src).context("val")?.into())
            }
            keys::INT_KEY => {
                Self::Int(i32VER::from_data(src).context("val")?.into())
            }
            keys::BOOL_KEY => {
                Self::Bool(u32VER::from_data(src).context("val")?.into())
            }
            keys::STRING_KEY => Self::String({
                let val = ListVER::from_data(src).context("val")?;
                let off = val.offset.get() as usize + val.size();
                core::str::from_utf8(&src[off..off + val.num.get() as usize])?
                    .into()
            }),
            keys::STRINGLIST_KEY => Self::StringList({
                let val = ListVER::from_data(src).context("val")?;
                let vals = ListVER::slice_from_data(
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
                            str_ref::from(s)
                        })
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into()
            }),
            keys::OBJECTLIST_KEY => Self::ObjectList({
                let val = ListVER::from_data(src).context("val")?;
                U32VER::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::NODELIST_KEY => Self::NodeList({
                let val = ListVER::from_data(src).context("val")?;
                Vector4VER::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::INTLIST_KEY => Self::IntList({
                let val = ListVER::from_data(src).context("val")?;
                i32VER::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::CRCLIST_KEY => Self::CrcList({
                let val = ListVER::from_data(src).context("val")?;
                U32VER::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::WEIGHTLIST_KEY => Self::WeightList({
                let val = ListVER::from_data(src).context("val")?;
                WeightVER::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::MATRIXLIST_KEY => Self::MatrixList({
                let val = ListVER::from_data(src).context("val")?;
                Matrix4x4VER::slice_from_data(
                    &src[val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            _ => return Err(anyhow!("Unkown Type {:?}", kind)),
        })
    }

}

#[make_platforms]
impl BaseTypeVER {
    pub fn from_bytes(data: &BufType, offset: usize, kind: u32) -> Result<Self> {
        Ok(match kind {
            keys::CRC_KEY => {
                BaseTypeVER::Crc(CrcVER::from_data(&data[offset..]).context("val")?.into())
            }
            keys::GUID_KEY => {
                BaseTypeVER::GUID(u32VER::from_data(&data[offset..]).context("val")?.into())
            }
            keys::COLOR_KEY => {
                BaseTypeVER::Color(u32VER::from_data(&data[offset..]).context("val")?.into())
            }
            keys::VECTOR2_KEY => BaseTypeVER::Vector2(
                Vector2VER::from_data(&data[offset..])
                    .context("val")?
                    .into(),
            ),
            keys::VECTOR3_KEY => BaseTypeVER::Vector3(
                Vector3VER::from_data(&data[offset..])
                    .context("val")?
                    .into(),
            ),
            keys::VECTOR4_KEY => BaseTypeVER::Vector4(
                Vector4VER::from_data(&data[offset..])
                    .context("val")?
                    .into(),
            ),
            keys::MATRIX4X4_KEY => BaseTypeVER::Matrix4x4(
                Matrix4x4VER::from_data(&data[offset..])
                    .context("val")?
                    .into(),
            ),
            keys::FLOAT_KEY => {
                BaseTypeVER::Float(f32VER::from_data(&data[offset..]).context("val")?.into())
            }
            keys::INT_KEY => {
                BaseTypeVER::Int(i32VER::from_data(&data[offset..]).context("val")?.into())
            }
            keys::BOOL_KEY => {
                BaseTypeVER::Bool(u32VER::from_data(&data[offset..]).context("val")?.into())
            }
            keys::STRING_KEY => BaseTypeVER::String({
                let val = ListVER::from_data(&data[offset..]).context("val")?;
                let off = val.offset.get() as usize + val.size();
                core::str::from_utf8(&data[offset + off..offset + off + val.num.get() as usize])?
                    .into()
            }),
            keys::STRINGLIST_KEY => BaseTypeVER::StringList({
                let val = ListVER::from_data(&data[offset..]).context("val")?;
                let vals = ListVER::slice_from_data(
                    &data[offset + val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("list info")?;
                vals.iter()
                    .enumerate()
                    .map(|(i, v)| {
                        Ok({
                            let off =
                                (val.offset.get() + v.offset.get()) as usize + val.size() * (i + 2);
                            let s_data = &data[offset + off..offset + off + v.num.get() as usize];
                            let s = std::str::from_utf8(s_data)
                                .with_context(|| format!("string {}, {:?}", i, s_data))?;
                            NonNull::from_ref(s)
                        })
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into()
            }),
            keys::OBJECTLIST_KEY => BaseTypeVER::ObjectList({
                let val = ListVER::from_data(&data[offset..]).context("val")?;
                U32VER::slice_from_data(
                    &data[offset + val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::NODELIST_KEY => BaseTypeVER::NodeList({
                let val = ListVER::from_data(&data[offset..]).context("val")?;
                Vector4VER::slice_from_data(
                    &data[offset + val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::INTLIST_KEY => BaseTypeVER::IntList({
                let val = ListVER::from_data(&data[offset..]).context("val")?;
                i32VER::slice_from_data(
                    &data[offset + val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::CRCLIST_KEY => BaseTypeVER::CrcList({
                let val = ListVER::from_data(&data[offset..]).context("val")?;
                U32VER::slice_from_data(
                    &data[offset + val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::WEIGHTLIST_KEY => BaseTypeVER::WeightList({
                let val = ListVER::from_data(&data[offset..]).context("val")?;
                WeightVER::slice_from_data(
                    &data[offset + val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::MATRIXLIST_KEY => BaseTypeVER::MatrixList({
                let val = ListVER::from_data(&data[offset..]).context("val")?;
                Matrix4x4VER::slice_from_data(
                    &data[offset + val.offset.get() as usize + val.size()..],
                    val.num.get() as usize,
                )
                .context("vals")?
                .into()
            }),
            _ => return Err(anyhow!("Unkown Type {:?}", kind)),
        })
    }
    pub unsafe fn as_ref(&self) -> BaseTypeRefVER<'_> {
        match self {
            BaseTypeVER::Crc(val) => BaseTypeRefVER::Crc(val.as_ref()),
            BaseTypeVER::GUID(val) => BaseTypeRefVER::GUID(val.as_ref()),
            BaseTypeVER::Color(val) => BaseTypeRefVER::Color(val.as_ref()),
            BaseTypeVER::Vector2(val) => BaseTypeRefVER::Vector2(val.as_ref()),
            BaseTypeVER::Vector3(val) => BaseTypeRefVER::Vector3(val.as_ref()),
            BaseTypeVER::Vector4(val) => BaseTypeRefVER::Vector4(val.as_ref()),
            BaseTypeVER::Matrix4x4(val) => BaseTypeRefVER::Matrix4x4(val.as_ref()),
            BaseTypeVER::Float(val) => BaseTypeRefVER::Float(val.as_ref()),
            BaseTypeVER::Int(val) => BaseTypeRefVER::Int(val.as_ref()),
            BaseTypeVER::Bool(val) => BaseTypeRefVER::Bool(val.as_ref()),
            BaseTypeVER::String(val) => BaseTypeRefVER::String(val.as_ref()),
            BaseTypeVER::StringList(vals) => {
                BaseTypeRefVER::StringList(vals.iter().map(|x| x.as_ref().into()).collect())
            }
            BaseTypeVER::ObjectList(vals) => BaseTypeRefVER::ObjectList(vals.as_ref()),
            BaseTypeVER::NodeList(vals) => BaseTypeRefVER::NodeList(vals.as_ref()),
            BaseTypeVER::IntList(vals) => BaseTypeRefVER::IntList(vals.as_ref()),
            BaseTypeVER::CrcList(vals) => BaseTypeRefVER::CrcList(vals.as_ref()),
            BaseTypeVER::WeightList(vals) => BaseTypeRefVER::WeightList(vals.as_ref()),
            BaseTypeVER::MatrixList(vals) => BaseTypeRefVER::MatrixList(vals.as_ref()),
        }
    }
    pub fn size(kind: u32) -> usize {
        match kind {
            keys::CRC_KEY => u32VER::size_of(),
            keys::GUID_KEY => u32VER::size_of(),
            keys::COLOR_KEY => u32VER::size_of(),
            keys::VECTOR2_KEY => Vector2VER::size_of(),
            keys::VECTOR3_KEY => Vector3VER::size_of(),
            keys::VECTOR4_KEY => Vector4VER::size_of(),
            keys::MATRIX4X4_KEY => Matrix4x4VER::size_of(),
            keys::FLOAT_KEY => f32VER::size_of(),
            keys::INT_KEY => u32VER::size_of(),
            keys::BOOL_KEY => u32VER::size_of(),
            keys::STRING_KEY => ListVER::size_of(),
            keys::STRINGLIST_KEY => ListVER::size_of(),
            keys::OBJECTLIST_KEY => ListVER::size_of(),
            keys::NODELIST_KEY => ListVER::size_of(),
            keys::INTLIST_KEY => ListVER::size_of(),
            keys::CRCLIST_KEY => ListVER::size_of(),
            keys::WEIGHTLIST_KEY => ListVER::size_of(),
            keys::MATRIXLIST_KEY => ListVER::size_of(),
            _ => 0,
        }
    }
}

#[make_platforms]
impl BaseTypeRefVER<'_> {
    pub fn crc(&self) -> Option<&CrcVER> {
        match self {
            Self::Crc(val) => Some(val),
            _ => None,
        }
    }
    pub fn guid(&self) -> Option<&u32VER> {
        match self {
            Self::GUID(val) => Some(val),
            _ => None,
        }
    }
    pub fn color(&self) -> Option<&u32VER> {
        match self {
            Self::Color(val) => Some(val),
            _ => None,
        }
    }
    pub fn vector2(&self) -> Option<&Vector2VER> {
        match self {
            Self::Vector2(val) => Some(val),
            _ => None,
        }
    }
    pub fn vector3(&self) -> Option<&Vector3VER> {
        match self {
            Self::Vector3(val) => Some(val),
            _ => None,
        }
    }
    pub fn vector4(&self) -> Option<&Vector4VER> {
        match self {
            Self::Vector4(val) => Some(val),
            _ => None,
        }
    }
    pub fn matrix4x4(&self) -> Option<&Matrix4x4VER> {
        match self {
            Self::Matrix4x4(val) => Some(val),
            _ => None,
        }
    }
    pub fn float(&self) -> Option<&f32VER> {
        match self {
            Self::Float(val) => Some(val),
            _ => None,
        }
    }
    pub fn int(&self) -> Option<&i32VER> {
        match self {
            Self::Int(val) => Some(val),
            _ => None,
        }
    }
    pub fn bool(&self) -> Option<&u32VER> {
        match self {
            Self::Bool(val) => Some(val),
            _ => None,
        }
    }
    pub fn string(&self) -> Option<&str> {
        match self {
            Self::String(val) => Some(val),
            _ => None,
        }
    }
    pub fn string_list(&self) -> Option<&[str_ref<'_>]> {
        match &self {
            Self::StringList(val) => Some(&val[..]),
            _ => None,
        }
    }
    pub fn object_list(&self) -> Option<&[U32VER]> {
        match self {
            Self::ObjectList(val) => Some(val),
            _ => None,
        }
    }
    pub fn node_list(&self) -> Option<&[Vector4VER]> {
        match self {
            Self::NodeList(val) => Some(val),
            _ => None,
        }
    }
    pub fn int_list(&self) -> Option<&[i32VER]> {
        match self {
            Self::IntList(val) => Some(val),
            _ => None,
        }
    }
    pub fn crc_list(&self) -> Option<&[U32VER]> {
        match self {
            Self::CrcList(val) => Some(val),
            _ => None,
        }
    }
    pub fn weight_list(&self) -> Option<&[WeightVER]> {
        match self {
            Self::WeightList(val) => Some(val),
            _ => None,
        }
    }
    pub fn matrix_list(&self) -> Option<&[Matrix4x4VER]> {
        match self {
            Self::MatrixList(val) => Some(val),
            _ => None,
        }
    }
}

#[make_platforms]
unsafe impl Sync for BaseTypeVER {}
#[make_platforms]
unsafe impl Send for BaseTypeVER {}

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

#[make_platforms]
impl From<BaseTypeRefVER<'_>> for BaseType {
    fn from(val: BaseTypeRefVER) -> Self {
        match val {
            BaseTypeRefVER::Crc(val) => Self::Crc(val.conv()),
            BaseTypeRefVER::GUID(val) => Self::GUID(val.get()),
            BaseTypeRefVER::Color(val) => Self::Color(val.get()),
            BaseTypeRefVER::Vector2(val) => Self::Vector2(val.conv()),
            BaseTypeRefVER::Vector3(val) => Self::Vector3(val.conv()),
            BaseTypeRefVER::Vector4(val) => Self::Vector4(val.conv()),
            BaseTypeRefVER::Matrix4x4(val) => Self::Matrix4x4(val.conv()),
            BaseTypeRefVER::Float(val) => Self::Float(val.get()),
            BaseTypeRefVER::Int(val) => Self::Int(val.get()),
            BaseTypeRefVER::Bool(val) => Self::Bool(val.get()),
            BaseTypeRefVER::String(val) => Self::String(val.to_string()),
            BaseTypeRefVER::StringList(vals) => {
                Self::StringList(vals.iter().map(|x| x.to_string()).collect())
            }
            BaseTypeRefVER::ObjectList(vals) => {
                Self::ObjectList(vals.iter().map(|x| x.get()).collect())
            }
            BaseTypeRefVER::NodeList(vals) => {
                Self::NodeList(vals.iter().map(|x| x.conv()).collect())
            }
            BaseTypeRefVER::IntList(vals) => Self::IntList(vals.iter().map(|x| x.get()).collect()),
            BaseTypeRefVER::CrcList(vals) => Self::CrcList(vals.iter().map(|x| x.conv()).collect()),
            BaseTypeRefVER::WeightList(vals) => {
                Self::WeightList(vals.iter().map(|x| x.conv()).collect())
            }
            BaseTypeRefVER::MatrixList(vals) => {
                Self::MatrixList(vals.iter().map(|x| x.conv()).collect())
            }
        }
    }
}

#[make_platforms]
pub enum BaseTypeDumpVER<'a> {
    Crc(&'a mut CrcVER),
    GUID(&'a mut u32VER),
    Color(&'a mut u32VER),
    Vector2(&'a mut Vector2VER),
    Vector3(&'a mut Vector3VER),
    Vector4(&'a mut Vector4VER),
    Matrix4x4(&'a mut Matrix4x4VER),
    Float(&'a mut f32VER),
    Int(&'a mut i32VER),
    Bool(&'a mut u32VER),
    String(&'a mut [u8]),
    StringList(Vec<&'a mut [u8]>),
    ObjectList(&'a mut [U32VER]),
    NodeList(&'a mut [Vector4VER]),
    IntList(&'a mut [i32VER]),
    CrcList(&'a mut [U32VER]),
    WeightList(&'a mut [WeightVER]),
    MatrixList(&'a mut [Matrix4x4VER]),
}

#[make_platforms]
pub trait DumpBaseTypeVER {
    fn off_size(&self) -> usize;
    fn list_len(&self) -> usize;
    fn string_lens(&self) -> Option<impl Iterator<Item = usize>>;
    fn write_into(&self, other: BaseTypeDumpVER) -> Result<()>;
    fn dump_into(&self, obj_dst: &mut DumpSlice, val_dst: &mut DumpSlice, kind: u32) -> Result<()> {
        let val = match kind {
            keys::CRC_KEY => {
                BaseTypeDumpVER::Crc(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::GUID_KEY => {
                BaseTypeDumpVER::GUID(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::COLOR_KEY => {
                BaseTypeDumpVER::Color(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::VECTOR2_KEY => {
                BaseTypeDumpVER::Vector2(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::VECTOR3_KEY => {
                BaseTypeDumpVER::Vector3(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::VECTOR4_KEY => {
                BaseTypeDumpVER::Vector4(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::MATRIX4X4_KEY => {
                BaseTypeDumpVER::Matrix4x4(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::FLOAT_KEY => {
                BaseTypeDumpVER::Float(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::INT_KEY => {
                BaseTypeDumpVER::Int(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::BOOL_KEY => {
                BaseTypeDumpVER::Bool(RefFromData::mut_from_data(obj_dst).context("val")?)
            }
            keys::STRING_KEY => {
                let val = ListVER::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                let vals = u8::mut_slice_from_data(val_dst, self.list_len()).context("vals")?;
                if vals.len() != 0 {
                    val_dst.split(1);
                }
                BaseTypeDumpVER::String(vals)
            }
            keys::STRINGLIST_KEY => {
                let val = ListVER::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                let mut offset = val_dst.offset;
                let vals =
                    ListVER::mut_slice_from_data(val_dst, self.list_len()).context("sub lists")?;
                let mut valss = Vec::with_capacity(vals.len());
                for (i, (len, val)) in self.string_lens().unwrap().zip(vals).enumerate() {
                    offset += ListVER::size_of();
                    val.offset = (val_dst.offset - offset).conv();
                    val.num = len.conv();
                    let s = u8::mut_slice_from_data(val_dst, len)
                        .with_context(|| format!("val {}", i))?;
                    if s.len() != 0 {
                        val_dst.split(1);
                    }
                    valss.push(s);
                }
                BaseTypeDumpVER::StringList(valss)
            }
            keys::OBJECTLIST_KEY => {
                let val = ListVER::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                BaseTypeDumpVER::ObjectList(
                    RefFromData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::NODELIST_KEY => {
                let val = ListVER::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                BaseTypeDumpVER::NodeList(
                    RefFromData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::INTLIST_KEY => {
                let val = ListVER::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                let vals = i32VER::mut_slice_from_data(val_dst, self.list_len()).context("vals")?;
                // should this be here or at the start of somthing else?
                val_dst.align(16);
                BaseTypeDumpVER::IntList(vals)
            }
            keys::CRCLIST_KEY => {
                let val = ListVER::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                BaseTypeDumpVER::CrcList(
                    RefFromData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::WEIGHTLIST_KEY => {
                let val = ListVER::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                BaseTypeDumpVER::WeightList(
                    RefFromData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::MATRIXLIST_KEY => {
                let val = ListVER::mut_from_data(obj_dst).context("list")?;
                val.offset = (val_dst.offset - obj_dst.offset).conv();
                val.num = self.list_len().conv();
                BaseTypeDumpVER::MatrixList(
                    RefFromData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            _ => return Err(anyhow!("Unkown Type {:?}", kind)),
        };
        self.write_into(val)
    }
}

#[make_platforms]
impl DumpBaseTypeVER for BaseTypeRefVER<'_> {
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
                let mut s = vals.len() * ListVER::size_of();
                for v in vals {
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
    fn write_into(&self, other: BaseTypeDumpVER) -> Result<()> {
        match (self, other) {
            (Self::Crc(val), BaseTypeDumpVER::Crc(other)) => other.write_from(val),
            (Self::GUID(val), BaseTypeDumpVER::GUID(other)) => other.write_from(val),
            (Self::Color(val), BaseTypeDumpVER::Color(other)) => other.write_from(val),
            (Self::Vector2(val), BaseTypeDumpVER::Vector2(other)) => other.write_from(val),
            (Self::Vector3(val), BaseTypeDumpVER::Vector3(other)) => other.write_from(val),
            (Self::Vector4(val), BaseTypeDumpVER::Vector4(other)) => other.write_from(val),
            (Self::Matrix4x4(val), BaseTypeDumpVER::Matrix4x4(other)) => other.write_from(val),
            (Self::Float(val), BaseTypeDumpVER::Float(other)) => other.write_from(val),
            (Self::Int(val), BaseTypeDumpVER::Int(other)) => other.write_from(val),
            (Self::Bool(val), BaseTypeDumpVER::Bool(other)) => other.write_from(val),
            (Self::String(val), BaseTypeDumpVER::String(other)) => other.write_from(val.as_bytes()),
            (Self::StringList(val), BaseTypeDumpVER::StringList(other)) => {
                Ok(for (val, other) in val.iter().zip(other) {
                    other.write_from(val.as_bytes())?;
                })
            }
            (Self::ObjectList(val), BaseTypeDumpVER::ObjectList(other)) => other.write_from(val),
            (Self::NodeList(val), BaseTypeDumpVER::NodeList(other)) => other.write_from(val),
            (Self::IntList(val), BaseTypeDumpVER::IntList(other)) => other.write_from(val),
            (Self::CrcList(val), BaseTypeDumpVER::CrcList(other)) => other.write_from(val),
            (Self::WeightList(val), BaseTypeDumpVER::WeightList(other)) => other.write_from(val),
            (Self::MatrixList(val), BaseTypeDumpVER::MatrixList(other)) => other.write_from(val),
            _ => Err(anyhow!("missmatched BaseTypeRef and BaseTypeDump")),
        }
    }
}

#[make_platforms]
impl DumpBaseTypeVER for &BaseType {
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
                let mut s = vals.len() * ListVER::size_of();
                for v in vals {
                    if v.len() != 0 {
                        s += v.len() + 1;
                    }
                }
                s
            }
            BaseType::ObjectList(vals) => vals.len() * u32VER::size_of(),
            BaseType::NodeList(vals) => vals.len() * Vector4VER::size_of(),
            BaseType::IntList(vals) => vals.len() * i32VER::size_of(),
            BaseType::CrcList(vals) => vals.len() * CrcVER::size_of(),
            BaseType::WeightList(vals) => vals.len() * WeightVER::size_of(),
            BaseType::MatrixList(vals) => vals.len() * Matrix4x4VER::size_of(),
            _ => 0,
        }
    }
    fn write_into(&self, other: BaseTypeDumpVER) -> Result<()> {
        match (self, other) {
            (BaseType::Crc(val), BaseTypeDumpVER::Crc(other)) => *other = val.conv(),
            (BaseType::GUID(val), BaseTypeDumpVER::GUID(other)) => *other = val.conv(),
            (BaseType::Color(val), BaseTypeDumpVER::Color(other)) => *other = val.conv(),
            (BaseType::Vector2(val), BaseTypeDumpVER::Vector2(other)) => *other = val.conv(),
            (BaseType::Vector3(val), BaseTypeDumpVER::Vector3(other)) => *other = val.conv(),
            (BaseType::Vector4(val), BaseTypeDumpVER::Vector4(other)) => *other = val.conv(),
            (BaseType::Matrix4x4(val), BaseTypeDumpVER::Matrix4x4(other)) => *other = val.conv(),
            (BaseType::Float(val), BaseTypeDumpVER::Float(other)) => *other = val.conv(),
            (BaseType::Int(val), BaseTypeDumpVER::Int(other)) => *other = val.conv(),
            (BaseType::Bool(val), BaseTypeDumpVER::Bool(other)) => *other = val.conv(),
            (BaseType::String(val), BaseTypeDumpVER::String(other)) => {
                other.write_from(val.as_bytes())?
            }
            (BaseType::StringList(val), BaseTypeDumpVER::StringList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    other.write_from(val.as_bytes())?;
                }
            }
            (BaseType::ObjectList(val), BaseTypeDumpVER::ObjectList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            (BaseType::NodeList(val), BaseTypeDumpVER::NodeList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            (BaseType::IntList(val), BaseTypeDumpVER::IntList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            (BaseType::CrcList(val), BaseTypeDumpVER::CrcList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            (BaseType::WeightList(val), BaseTypeDumpVER::WeightList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            (BaseType::MatrixList(val), BaseTypeDumpVER::MatrixList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    *other = val.conv()
                }
            }
            _ => return Err(anyhow!("missmatched BaseTypeRef and BaseTypeDump")),
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
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

#[derive(Debug, Default, Clone, OrderedData)]
pub struct TypeHeader {
    pub key: Crc,
    pub size: u32,
    pub fields: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct TypeField {
    pub key: Crc,
    pub kind: Crc,
    pub offset: u32,
}

pub trait TypeFieldDump {
    fn key(&self) -> u32;
    fn kind(&self) -> u32;
    fn offset(&self) -> u32;
}

#[make_platforms]
impl TypeFieldDump for TypeFieldVER {
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

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TypeRefVER<'a> {
    pub header: &'a TypeHeaderVER,
    pub fields: slice<'a, TypeFieldVER>,
}

#[make_platforms]
impl Default for TypeRefVER<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            fields: slice::default()
        }
    }
}

#[make_platforms]
impl<'a> TypeRefVER<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let header = TypeHeaderVER::from_data(src).context("header")?;
        let fields = TypeFieldVER::slice_from_data(&src[header.size()..], header.size.get() as usize)
            .context("fields")?.into();
        Ok(Self { header, fields })
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct TypeVER {
    _ptr: BufType,
    header: NonNull<TypeHeaderVER>,
    fields: NonNull<[TypeFieldVER]>,
    size: usize,
}

#[make_platforms]
unsafe impl Sync for TypeVER {}
#[make_platforms]
unsafe impl Send for TypeVER {}

#[make_platforms]
impl TypeVER {
    pub fn from_bytes(src: &BufType, mut offset: usize) -> Result<Self> {
        let start = offset;
        let header = TypeHeaderVER::from_data(&src[offset..]).context("header")?;
        offset += header.size();
        let fields = TypeFieldVER::slice_from_data(&src[offset..], header.size.get() as usize)
            .context("fields")?;
        offset += fields.size();
        Ok(Self {
            _ptr: src.clone(),
            header: header.into(),
            fields: fields.into(),
            size: offset - start,
        })
    }
}

#[make_platforms]
impl TypeVER {
    pub fn header(&self) -> &TypeHeaderVER {
        unsafe { self.header.as_ref() }
    }
    pub fn fields(&self) -> &[TypeFieldVER] {
        unsafe { self.fields.as_ref() }
    }
    pub fn size(&self) -> usize {
        self.size
    }
}

#[make_platforms]
pub trait DumpTypeVER {
    fn key(&self) -> u32;
    fn fields_len(&self) -> usize;
    fn fields(&self) -> impl Iterator<Item = &impl TypeFieldDump>;
    fn write_header(&self, header: &mut TypeHeaderVER) -> Result<()>;
    fn write_fields(&self, fields: &mut [TypeFieldVER]) -> Result<()>;

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let header = TypeHeaderVER::mut_from_data(dst).context("header")?;
        self.write_header(header).context("write header")?;
        let fields = TypeFieldVER::mut_slice_from_data(dst, self.fields_len()).context("fields")?;
        self.write_fields(fields).context("write fileds")?;
        Ok(())
    }
    fn size(&self) -> usize {
        TypeHeaderVER::size_of() + self.fields_len() * TypeFieldVER::size_of()
    }
}

#[make_platforms]
impl DumpTypeVER for TypeRefVER<'_> {
    fn key(&self) -> u32 {
        self.header.key.into()
    }
    fn fields_len(&self) -> usize {
        self.fields.len()
    }
    fn fields(&self) -> impl Iterator<Item = &impl TypeFieldDump> {
        self.fields.iter()
    }
    fn write_header(&self, header: &mut TypeHeaderVER) -> Result<()> {
        header.write_from(self.header)
    }
    fn write_fields(&self, fields: &mut [TypeFieldVER]) -> Result<()> {
        fields.write_from(&self.fields[..])
    }
}

#[make_platforms]
impl DumpTypeVER for &TypeVER {
    fn key(&self) -> u32 {
        self.header().key.into()
    }
    fn fields_len(&self) -> usize {
        self.fields.len()
    }
    fn fields(&self) -> impl Iterator<Item = &impl TypeFieldDump> {
        TypeVER::fields(self).iter()
    }
    fn write_header(&self, header: &mut TypeHeaderVER) -> Result<()> {
        header.write_from(self.header())
    }
    fn write_fields(&self, fields: &mut [TypeFieldVER]) -> Result<()> {
        fields.write_from(TypeVER::fields(self))
    }
}

#[make_platforms]
impl DumpTypeVER for (&Crc, &Vec<TypeField>) {
    fn key(&self) -> u32 {
        self.0.get()
    }
    fn fields_len(&self) -> usize {
        self.1.len()
    }
    fn fields(&self) -> impl Iterator<Item = &impl TypeFieldDump> {
        self.1.iter()
    }
    fn write_header(&self, header: &mut TypeHeaderVER) -> Result<()> {
        header.key = self.0.conv();
        header.size = self.1.len().conv();
        header.fields = 0u32.conv();
        Ok(())
    }
    fn write_fields(&self, fields: &mut [TypeFieldVER]) -> Result<()> {
        for (src, dst) in self.1.iter().zip(fields) {
            *dst = src.conv()
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, OrderedData)]
pub struct ObjHeader {
    pub layer: u32,
    pub key: Crc,
    pub size: u16,
    pub z3: u16,
    pub z4: u32,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct ObjRefVER<'a> {
    pub header: &'a ObjHeaderVER,
    pub fields: Map<u32, BaseTypeRefVER<'a>>,
}

#[make_platforms]
impl Default for ObjRefVER<'_> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            fields: MapImpl::default().into()
        }
    }
}

#[make_platforms]
impl<'a> ObjRefVER<'a> {
    pub fn from_data(src: &'a [u8], types: &MapImpl<u32, TypeRefVER<'_>>) -> Result<Self> {
        let mut offset = 0;
        let header = ObjHeaderVER::from_data(&src[offset..]).context("header")?;
        offset += header.size();
        let ts = types
            .get(&header.key.get())
            .ok_or(anyhow!("Missing Key {:?}", header.key.get()))?
            .fields;
        let mut fields = MapImpl::with_capacity(ts.len());
        for t in ts.iter() {
            fields.insert(
                t.key.into(),
                BaseTypeRefVER::from_data(&src[offset + t.offset.get() as usize..], t.kind.get())
                    .with_context(|| {
                        format!(
                            "{}field {}",
                            if let Some(BaseTypeRefVER::GUID(o)) =
                                fields.get(&hash_string(b"guid", None))
                            {
                                format!("guid {}, ", o.get())
                            } else {
                                String::new()
                            },
                            get_str_debug(&t.key.get())
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
        self.header.size() + self.header.size.get() as usize
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct ObjVER {
    _ptr: BufType,
    header: NonNull<ObjHeaderVER>,
    fields: IndexMap<u32, BaseTypeVER>,
    size: usize,
}

#[make_platforms]
unsafe impl Sync for ObjVER {}
#[make_platforms]
unsafe impl Send for ObjVER {}

#[make_platforms]
impl ObjVER {
    pub fn header(&self) -> &ObjHeaderVER {
        unsafe { self.header.as_ref() }
    }
    pub fn get(&self, index: &u32) -> Option<BaseTypeRefVER<'_>> {
        self.fields.get(index).map(|x| unsafe { x.as_ref() })
    }
    pub fn fields(&self) -> &IndexMap<u32, BaseTypeVER> {
        &self.fields
    }
    pub fn size(&self) -> usize {
        self.size
    }
}

#[make_platforms]
impl ObjVER {
    pub fn from_bytes(
        src: &BufType,
        mut offset: usize,
        types: &IndexMap<u32, TypeVER>,
    ) -> Result<Self> {
        let header = ObjHeaderVER::from_data(&src[offset..]).context("header")?;
        offset += header.size();
        let ts = types
            .get(&header.key.get())
            .ok_or(anyhow!("Missing Key {:?}", header.key.get()))?
            .fields();
        let mut fields = IndexMap::with_capacity(ts.len());
        for t in ts {
            fields.insert(
                t.key.into(),
                BaseTypeVER::from_bytes(src, offset + t.offset.get() as usize, t.kind.get())
                    .with_context(|| {
                        format!(
                            "{}field {}",
                            if let Some(BaseTypeVER::GUID(o)) =
                                fields.get(&hash_string(b"guid", None))
                            {
                                format!("guid {}, ", unsafe { o.as_ref() })
                            } else {
                                String::new()
                            },
                            get_str_debug(&t.key.get())
                        )
                    })?,
            );
        }
        Ok(Self {
            _ptr: src.clone(),
            header: header.into(),
            fields,
            size: header.size() + header.size.get() as usize,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Obj {
    pub layer: u32,
    pub key: Crc,
    pub fields: IndexMap<Crc, BaseType>,
}

#[make_platforms]
impl From<&ObjVER> for Obj {
    fn from(val: &ObjVER) -> Self {
        Self {
            layer: val.header().layer.get(),
            key: val.header().key.conv(),
            fields: val
                .fields
                .iter()
                .map(|(k, v)| ((*k).into(), unsafe { v.as_ref() }.into()))
                .collect(),
        }
    }
}

#[make_platforms]
pub trait DumpObjVER {
    fn key(&self) -> u32;
    fn layer(&self) -> u32;
    fn field(&self, index: &u32) -> Option<impl DumpBaseTypeVER>;

    fn dump_into(&self, dst: &mut DumpSlice, ts: &impl DumpTypeVER) -> Result<()> {
        let header = ObjHeaderVER::mut_from_data(dst).context("header")?;
        let val_offset = ts
            .fields()
            .map(|t| t.offset() as usize + BaseTypeVER::size(t.kind()))
            .fold(0, usize::max);
        assert!(val_offset > dst.offset, "malformed gameobj {}", self.key());
        let mut val_dst = dst.split(val_offset);
        val_dst.align(16);
        for t in ts.fields() {
            let key = t.key();
            let f = self.field(&key).ok_or(anyhow!("missing field {}", key))?;
            let mut obj_dst = dst.view(t.offset() as usize);
            f.dump_into(&mut obj_dst, &mut val_dst, t.kind())
                .with_context(|| format!("field {}", key))?;
        }
        val_dst.align(16);
        *dst = val_dst;
        header.layer = self.layer().conv();
        header.key = self.key().conv();
        header.size = val_offset.conv();
        Ok(())
    }

    fn size<'a>(&'a self, ts: &impl DumpTypeVER) -> usize {
        let mut off = ts
            .fields()
            .map(|t| t.offset() as usize + BaseTypeVER::size(t.kind()))
            .fold(0, usize::max);
        off = align_offset(off, 16);
        for t in ts.fields() {
            let key = t.key();
            let f = self
                .field(&key)
                .ok_or(anyhow!("missing field {}", key))
                .unwrap();
            off += f.off_size();
            if key == keys::INTLIST_KEY {
                off = align_offset(off, 16);
            }
        }
        align_offset(off, 16) + ObjHeaderVER::size_of()
    }
}

#[make_platforms]
impl DumpObjVER for ObjRefVER<'_> {
    fn key(&self) -> u32 {
        self.header.key.into()
    }
    fn layer(&self) -> u32 {
        self.header.layer.into()
    }
    fn field(&self, index: &u32) -> Option<impl DumpBaseTypeVER> {
        self.fields.get(index).cloned()
    }
}

#[make_platforms]
impl DumpObjVER for ObjVER {
    fn key(&self) -> u32 {
        self.header().key.into()
    }
    fn layer(&self) -> u32 {
        self.header().layer.into()
    }
    fn field(&self, index: &u32) -> Option<impl DumpBaseTypeVER> {
        self.get(index)
    }
}

#[make_platforms]
impl DumpObjVER for Obj {
    fn key(&self) -> u32 {
        self.key.get()
    }
    fn layer(&self) -> u32 {
        self.layer
    }
    fn field(&self, index: &u32) -> Option<impl DumpBaseTypeVER> {
        self.fields.get(&Crc::from(*index))
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct GameObjsRefVER<'a> {
    pub gamemodemask: i32,
    pub header: &'a GameObjsHeaderVER,
    pub types: Map<u32, TypeRefVER<'a>>,
    pub objs: Map<u32, ObjRefVER<'a>>
}

#[make_platforms]
impl Default for GameObjsRefVER<'_> {
    fn default() -> Self {
        Self {
            gamemodemask: 0,
            header: get_default_ref(),
            types: MapImpl::default().into(),
            objs: MapImpl::default().into()
        }
    }
}

#[make_platforms]
impl<'a> GameObjsRefVER<'a> {
    pub fn from_data(src: &'a [u8], gamemodemask: i32) -> Result<Self> {
        let header = GameObjsHeaderVER::from_data(src).context("header")?;
        if header.const_.get() != 1296123652 {
            log::error!("Invalid gameobj block");
        }
        let mut offset = header.types_offset.get() as usize;
        let mut types = MapImpl::with_capacity(header.types_num.get() as usize);
        for i in 0..header.types_num.get() {
            let t = TypeRefVER::from_data(&src[offset..]).with_context(|| format!("ty {}", i))?;
            offset += t.size();
            types.insert(t.header.key.get(), t);
        }

        offset = header.obj_offset.get() as usize;
        let mut objs = MapImpl::with_capacity(header.obj_num.get() as usize);
        for i in 0..header.obj_num.get() {
            let o =
                ObjRefVER::from_data(&src[offset..], &types).with_context(|| format!("obj {}", i))?;
            offset += o.size();
            if let BaseTypeRefVER::GUID(guid) = o
                .fields
                .get(&hash_string(b"guid", None))
                .ok_or(anyhow!("obj missing guid"))?
            {
                objs.insert(guid.get(), o);
            } else {
                return Err(anyhow!("obj incorrect guid type"));
            }
        }
        Ok(Self { gamemodemask, header, objs: objs.into(), types: types.into() })
    }
}

#[make_platforms]
#[derive(Debug, Clone)]
pub struct GameObjsVER {
    _ptr: BufType,
    gamemodemask: i32,
    header: NonNull<GameObjsHeaderVER>,
    types: IndexMap<u32, TypeVER>,
    objs: IndexMap<u32, ObjVER>,
}

#[make_platforms]
unsafe impl Sync for GameObjsVER {}
#[make_platforms]
unsafe impl Send for GameObjsVER {}

#[make_platforms]
impl GameObjsVER {
    pub fn from_bytes(
        src: &BufType,
        mut offset: usize,
        _size: usize,
        gamemodemask: i32,
    ) -> Result<Self> {
        let start = offset;
        let header = GameObjsHeaderVER::from_data(&src[offset..]).context("header")?;
        if header.const_.get() != 1296123652 {
            log::error!("Invalid gameobj block");
        }
        offset = start + header.types_offset.get() as usize;
        let mut types = IndexMap::with_capacity(header.types_num.get() as usize);
        for i in 0..header.types_num.get() {
            let t = TypeVER::from_bytes(src, offset).with_context(|| format!("ty {}", i))?;
            offset += t.size();
            types.insert(t.header().key.get(), t);
        }

        offset = start + header.obj_offset.get() as usize;
        let mut objs = IndexMap::with_capacity(header.obj_num.get() as usize);
        for i in 0..header.obj_num.get() {
            let o =
                ObjVER::from_bytes(src, offset, &types).with_context(|| format!("obj {}", i))?;
            offset += o.size();
            if let BaseTypeVER::GUID(guid) = o
                .fields
                .get(&hash_string(b"guid", None))
                .ok_or(anyhow!("obj missing guid"))?
            {
                objs.insert(unsafe { guid.as_ref() }.get(), o);
            } else {
                return Err(anyhow!("obj incorrect guid type"));
            }
        }

        Ok(Self {
            _ptr: src.clone(),
            gamemodemask,
            header: header.into(),
            objs,
            types,
        })
    }
}
#[make_platforms]
impl GameObjsVER {
    pub fn header(&self) -> &GameObjsHeaderVER {
        unsafe { self.header.as_ref() }
    }
    pub fn types(&self) -> &IndexMap<u32, TypeVER> {
        &self.types
    }
    pub fn objs(&self) -> &IndexMap<u32, ObjVER> {
        &self.objs
    }
}

#[derive(Debug, Clone)]
pub struct GameObjs {
    pub gamemodemask: i32,
    pub objs: IndexMap<Crc, Obj>,
    pub types: IndexMap<Crc, Vec<TypeField>>,
}

#[make_platforms]
impl From<&GameObjsVER> for GameObjs {
    fn from(val: &GameObjsVER) -> Self {
        Self {
            gamemodemask: val.gamemodemask,
            objs: val.objs.iter().map(|(k, v)| ((*k).into(), v.into())).collect(),
            types: val
                .types
                .iter()
                .map(|(k, v)| ((*k).into(), v.fields().iter().map(|x| x.conv()).collect()))
                .collect(),
        }
    }
}

#[make_platforms]
pub trait DumpGameObjsVER {
    fn gamemodemask(&self) -> i32;
    fn types(&self) -> impl Iterator<Item = impl DumpTypeVER>;
    fn types_num(&self) -> usize;
    fn get_type(&self, key: u32) -> Option<impl DumpTypeVER>;
    fn objs(&self) -> impl Iterator<Item = &impl DumpObjVER>;
    fn objs_num(&self) -> usize;
    fn size(&self) -> usize {
        let size = GameObjsHeaderVER::size_of()
            + self.types_num() * TypeHeaderVER::size_of()
            + self.types().map(|x| x.fields_len()).sum::<usize>() * TypeFieldVER::size_of();
        align_offset(size, 16)
            + self
                .objs()
                .map(|x| x.size(&self.get_type(x.key()).unwrap()))
                .sum::<usize>()
    }
    fn dump_into<'a>(&self, dst: &mut DumpSlice) -> Result<()> {
        let start = dst.offset;
        let header = GameObjsHeaderVER::mut_from_data(dst).context("header")?;
        header.const_ = 1296123652u32.conv();

        header.types_offset = (dst.offset - start).conv();
        header.types_num = self.types_num().conv();
        for ty in self.types() {
            ty.dump_into(dst)
                .with_context(|| format!("type {}", ty.key()))?;
        }
        dst.align(16);

        header.obj_offset = (dst.offset - start).conv();
        header.obj_num = self.objs_num().conv();
        for (i, obj) in self.objs().enumerate() {
            let key = obj.key();
            let ty = self.get_type(key).ok_or(anyhow!("obj {} ty {}", i, key))?;
            obj.dump_into(dst, &ty)
                .with_context(|| format!("obj {}", i))?;
        }

        Ok(())
    }
}

#[make_platforms]
impl DumpGameObjsVER for GameObjsRefVER<'_> {
    fn gamemodemask(&self) -> i32 {
        self.gamemodemask
    }
    fn types(&self) -> impl Iterator<Item = impl DumpTypeVER> {
        self.types.values().cloned()
    }
    fn types_num(&self) -> usize {
        self.types.len()
    }
    fn get_type(&self, key: u32) -> Option<impl DumpTypeVER> {
        self.types.get(&key).cloned()
    }
    fn objs(&self) -> impl Iterator<Item = &impl DumpObjVER> {
        self.objs.values()
    }
    fn objs_num(&self) -> usize {
        self.objs.len()
    }
}

#[make_platforms]
impl DumpGameObjsVER for GameObjsVER {
    fn gamemodemask(&self) -> i32 {
        self.gamemodemask
    }
    fn types(&self) -> impl Iterator<Item = impl DumpTypeVER> {
        self.types.values()
    }
    fn types_num(&self) -> usize {
        self.types.len()
    }
    fn get_type(&self, key: u32) -> Option<impl DumpTypeVER> {
        self.types.get(&key)
    }
    fn objs(&self) -> impl Iterator<Item = &impl DumpObjVER> {
        self.objs.values()
    }
    fn objs_num(&self) -> usize {
        self.objs.len()
    }
}

#[make_platforms]
impl DumpGameObjsVER for GameObjs {
    fn gamemodemask(&self) -> i32 {
        self.gamemodemask
    }
    fn types(&self) -> impl Iterator<Item = impl DumpTypeVER> {
        self.types.iter()
    }
    fn types_num(&self) -> usize {
        self.types.len()
    }
    fn get_type(&self, key: u32) -> Option<impl DumpTypeVER> {
        self.types.get_key_value(&Crc::from(key))
    }
    fn objs(&self) -> impl Iterator<Item = &impl DumpObjVER> {
        self.objs.values()
    }
    fn objs_num(&self) -> usize {
        self.objs.len()
    }
}
