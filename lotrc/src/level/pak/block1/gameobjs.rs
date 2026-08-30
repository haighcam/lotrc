use crate::types::{
    get_default_ref, align_offset, get_str_debug, hash_string, Crc, DumpSlice, Matrix4x4,
    Vector2, Vector3, Vector4, Weight, ref_slice, slice, string, BaseTypes, ReadData, NE
};
use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use enum_dispatch::enum_dispatch;
use lotrc_proc::{derive_pod};

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

#[derive_pod]
pub struct List<T: BaseTypes> {
    num: T::U16,
    offset: T::U16
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
pub enum BaseTypeRef<'a, T: BaseTypes> {
    Crc(&'a Crc<T>),
    GUID(&'a T::u32),
    Color(&'a T::u32),
    Vector2(&'a Vector2<T>),
    Vector3(&'a Vector3<T>),
    Vector4(&'a Vector4<T>),
    Matrix4x4(&'a Matrix4x4<T>),
    Float(&'a T::f32),
    Int(&'a T::i32),
    Bool(&'a T::u32),
    String(string<'a>),
    StringList(slice<string<'a>>),
    ObjectList(ref_slice<'a, T::U32>),
    NodeList(ref_slice<'a, Vector4<T>>),
    IntList(ref_slice<'a, T::i32>),
    CrcList(ref_slice<'a, T::U32>),
    WeightList(ref_slice<'a, Weight<T>>),
    MatrixList(ref_slice<'a, Matrix4x4<T>>),
}

impl <'a, T: BaseTypes> BaseTypeRef<'a, T> {
    pub fn from_data(src: &'a [u8], kind: u32) -> Result<Self> {
        Ok(match kind {
            keys::CRC_KEY => {
                Self::Crc(Crc::<T>::from_data(src).context("val")?.into())
            }
            keys::GUID_KEY => {
                Self::GUID(T::u32::from_data(src).context("val")?.into())
            }
            keys::COLOR_KEY => {
                Self::Color(T::u32::from_data(src).context("val")?.into())
            }
            keys::VECTOR2_KEY => Self::Vector2(
                Vector2::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::VECTOR3_KEY => Self::Vector3(
                Vector3::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::VECTOR4_KEY => Self::Vector4(
                Vector4::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::MATRIX4X4_KEY => Self::Matrix4x4(
                Matrix4x4::from_data(src)
                    .context("val")?
                    .into(),
            ),
            keys::FLOAT_KEY => {
                Self::Float(T::f32::from_data(src).context("val")?.into())
            }
            keys::INT_KEY => {
                Self::Int(T::i32::from_data(src).context("val")?.into())
            }
            keys::BOOL_KEY => {
                Self::Bool(T::u32::from_data(src).context("val")?.into())
            }
            keys::STRING_KEY => Self::String({
                let val = List::<T>::from_data(src).context("val")?;
                let off = val.offset.into() as usize + std::mem::size_of::<List<T>>();
                core::str::from_utf8(&src[off..off + val.num.into() as usize])?
                    .into()
            }),
            keys::STRINGLIST_KEY => Self::StringList({
                let val = List::<T>::from_data(src).context("val")?;
                let vals = List::<T>::slice_from_data(
                    &src[val.offset.into() as usize + std::mem::size_of::<List<T>>()..],
                    val.num.into() as usize,
                )
                .context("list info")?;
                vals.iter()
                    .enumerate()
                    .map(|(i, v)| {
                        Ok({
                            let off =
                                (val.offset.into() + v.offset.into()) as usize + std::mem::size_of::<List<T>>() * (i + 2);
                            let s_data = &src[off..off + v.num.into() as usize];
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
                let val = List::<T>::from_data(src).context("val")?;
                T::U32::slice_from_data(
                    &src[val.offset.into() as usize + std::mem::size_of::<List<T>>()..],
                    val.num.into() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::NODELIST_KEY => Self::NodeList({
                let val = List::<T>::from_data(src).context("val")?;
                Vector4::slice_from_data(
                    &src[val.offset.into() as usize + std::mem::size_of::<List<T>>()..],
                    val.num.into() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::INTLIST_KEY => Self::IntList({
                let val = List::<T>::from_data(src).context("val")?;
                T::i32::slice_from_data(
                    &src[val.offset.into() as usize + std::mem::size_of::<List<T>>()..],
                    val.num.into() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::CRCLIST_KEY => Self::CrcList({
                let val = List::<T>::from_data(src).context("val")?;
                T::U32::slice_from_data(
                    &src[val.offset.into() as usize + std::mem::size_of::<List<T>>()..],
                    val.num.into() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::WEIGHTLIST_KEY => Self::WeightList({
                let val = List::<T>::from_data(src).context("val")?;
                Weight::slice_from_data(
                    &src[val.offset.into() as usize + std::mem::size_of::<List<T>>()..],
                    val.num.into() as usize,
                )
                .context("vals")?
                .into()
            }),
            keys::MATRIXLIST_KEY => Self::MatrixList({
                let val = List::<T>::from_data(src).context("val")?;
                Matrix4x4::slice_from_data(
                    &src[val.offset.into() as usize + std::mem::size_of::<List<T>>()..],
                    val.num.into() as usize,
                )
                .context("vals")?
                .into()
            }),
            _ => return Err(anyhow!("Unkown Type {:?}", kind)),
        })
    }
    pub fn size(kind: u32) -> usize {
        match kind {
            keys::CRC_KEY => std::mem::size_of::<T::u32>(),
            keys::GUID_KEY => std::mem::size_of::<T::u32>(),
            keys::COLOR_KEY => std::mem::size_of::<T::u32>(),
            keys::VECTOR2_KEY => std::mem::size_of::<Vector2<T>>(),
            keys::VECTOR3_KEY => std::mem::size_of::<Vector3<T>>(),
            keys::VECTOR4_KEY => std::mem::size_of::<Vector4<T>>(),
            keys::MATRIX4X4_KEY => std::mem::size_of::<Matrix4x4<T>>(),
            keys::FLOAT_KEY => std::mem::size_of::<T::f32>(),
            keys::INT_KEY => std::mem::size_of::<T::u32>(),
            keys::BOOL_KEY => std::mem::size_of::<T::u32>(),
            keys::STRING_KEY => std::mem::size_of::<List<T>>(),
            keys::STRINGLIST_KEY => std::mem::size_of::<List<T>>(),
            keys::OBJECTLIST_KEY => std::mem::size_of::<List<T>>(),
            keys::NODELIST_KEY => std::mem::size_of::<List<T>>(),
            keys::INTLIST_KEY => std::mem::size_of::<List<T>>(),
            keys::CRCLIST_KEY => std::mem::size_of::<List<T>>(),
            keys::WEIGHTLIST_KEY => std::mem::size_of::<List<T>>(),
            keys::MATRIXLIST_KEY => std::mem::size_of::<List<T>>(),
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BaseType {
    Crc(Crc<NE>),
    GUID(u32),
    Color(u32),
    Vector2(Vector2<NE>),
    Vector3(Vector3<NE>),
    Vector4(Vector4<NE>),
    Matrix4x4(Matrix4x4<NE>),
    Float(f32),
    Int(i32),
    Bool(u32),
    String(String),
    StringList(Vec<String>),
    ObjectList(Vec<u32>),
    NodeList(Vec<Vector4<NE>>),
    IntList(Vec<i32>),
    CrcList(Vec<Crc<NE>>),
    WeightList(Vec<Weight<NE>>),
    MatrixList(Vec<Matrix4x4<NE>>),
}

impl<T: BaseTypes> From<&BaseTypeRef<'_, T>> for BaseType
where
    Crc<NE>: From<Crc<T>>,
    Vector2<NE>: From<Vector2<T>>,
    Vector3<NE>: From<Vector3<T>>,
    Vector4<NE>: From<Vector4<T>>,
    Matrix4x4<NE>: From<Matrix4x4<T>>,
    Weight<NE>: From<Weight<T>>,
{
    fn from(val: &BaseTypeRef<T>) -> Self {
        match val {
            BaseTypeRef::Crc(val) => Self::Crc((**val).into()),
            BaseTypeRef::GUID(val) => Self::GUID((**val).into()),
            BaseTypeRef::Color(val) => Self::Color((**val).into()),
            BaseTypeRef::Vector2(val) => Self::Vector2((**val).into()),
            BaseTypeRef::Vector3(val) => Self::Vector3((**val).into()),
            BaseTypeRef::Vector4(val) => Self::Vector4((**val).into()),
            BaseTypeRef::Matrix4x4(val) => Self::Matrix4x4((**val).into()),
            BaseTypeRef::Float(val) => Self::Float((**val).into()),
            BaseTypeRef::Int(val) => Self::Int((**val).into()),
            BaseTypeRef::Bool(val) => Self::Bool((**val).into()),
            BaseTypeRef::String(val) => Self::String(val.to_string()),
            BaseTypeRef::StringList(vals) => {
                Self::StringList(vals.iter().map(|x| x.to_string()).collect())
            }
            BaseTypeRef::ObjectList(vals) => {
                Self::ObjectList(vals.iter().map(|&x| x.into()).collect())
            }
            BaseTypeRef::NodeList(vals) => {
                Self::NodeList(vals.iter().map(|&x| x.into()).collect())
            }
            BaseTypeRef::IntList(vals) => Self::IntList(vals.iter().map(|&x| x.into()).collect()),
            BaseTypeRef::CrcList(vals) => Self::CrcList(vals.iter().map(|&x| Crc::new(x.into())).collect()),
            BaseTypeRef::WeightList(vals) => {
                Self::WeightList(vals.iter().map(|&x| x.into()).collect())
            }
            BaseTypeRef::MatrixList(vals) => {
                Self::MatrixList(vals.iter().map(|&x| x.into()).collect())
            }
        }
    }
}
/*
#[enum_dispatch(DumpBaseType)]
pub enum BaseType<'a, T: BaseTypes> {
    Ref(BaseTypeRef<'a, T>),
    Owned(BaseType)
}
*/

pub enum BaseTypeDump<'a, T: BaseTypes> {
    Crc(&'a mut Crc<T>),
    GUID(&'a mut T::u32),
    Color(&'a mut T::u32),
    Vector2(&'a mut Vector2<T>),
    Vector3(&'a mut Vector3<T>),
    Vector4(&'a mut Vector4<T>),
    Matrix4x4(&'a mut Matrix4x4<T>),
    Float(&'a mut T::f32),
    Int(&'a mut T::i32),
    Bool(&'a mut T::u32),
    String(&'a mut [u8]),
    StringList(Vec<&'a mut [u8]>),
    ObjectList(&'a mut [T::U32]),
    NodeList(&'a mut [Vector4<T>]),
    IntList(&'a mut [T::i32]),
    CrcList(&'a mut [T::U32]),
    WeightList(&'a mut [Weight<T>]),
    MatrixList(&'a mut [Matrix4x4<T>]),
}

pub trait DumpBaseTypeImpl<T: BaseTypes> {
    fn list_len(&self) -> usize;
    fn string_lens(&self) -> Option<impl Iterator<Item = usize>>;
    fn write_into(&self, other: BaseTypeDump<T>) -> Result<()>;
    fn off_size(&self) -> usize;
}

#[enum_dispatch]
pub trait DumpBaseType<T: BaseTypes> {
    fn dump_into(&self, obj_dst: &mut DumpSlice, val_dst: &mut DumpSlice, kind: u32) -> Result<()>;
    fn off_size(&self) -> usize;
}

impl<T: BaseTypes, I: DumpBaseTypeImpl<T>> DumpBaseType<T> for I {
    fn dump_into(&self, obj_dst: &mut DumpSlice, val_dst: &mut DumpSlice, kind: u32) -> Result<()> {
        let val = match kind {
            keys::CRC_KEY => {
                BaseTypeDump::Crc(ReadData::mut_from_data(obj_dst).context("val")?)
            }
            keys::GUID_KEY => {
                BaseTypeDump::GUID(ReadData::mut_from_data(obj_dst).context("val")?)
            }
            keys::COLOR_KEY => {
                BaseTypeDump::Color(ReadData::mut_from_data(obj_dst).context("val")?)
            }
            keys::VECTOR2_KEY => {
                BaseTypeDump::Vector2(ReadData::mut_from_data(obj_dst).context("val")?)
            }
            keys::VECTOR3_KEY => {
                BaseTypeDump::Vector3(ReadData::mut_from_data(obj_dst).context("val")?)
            }
            keys::VECTOR4_KEY => {
                BaseTypeDump::Vector4(ReadData::mut_from_data(obj_dst).context("val")?)
            }
            keys::MATRIX4X4_KEY => {
                BaseTypeDump::Matrix4x4(ReadData::mut_from_data(obj_dst).context("val")?)
            }
            keys::FLOAT_KEY => {
                BaseTypeDump::Float(ReadData::mut_from_data(obj_dst).context("val")?)
            }
            keys::INT_KEY => {
                BaseTypeDump::Int(ReadData::mut_from_data(obj_dst).context("val")?)
            }
            keys::BOOL_KEY => {
                BaseTypeDump::Bool(ReadData::mut_from_data(obj_dst).context("val")?)
            }
            keys::STRING_KEY => {
                let val = List::<T>::mut_from_data(obj_dst).context("list")?;
                val.offset = ((val_dst.offset - obj_dst.offset) as u16).into();
                val.num = (self.list_len() as u16).into();
                let vals = u8::mut_slice_from_data(val_dst, self.list_len()).context("vals")?;
                if vals.len() != 0 {
                    val_dst.split(1).context("string align")?;
                }
                BaseTypeDump::String(vals)
            }
            keys::STRINGLIST_KEY => {
                let val = List::<T>::mut_from_data(obj_dst).context("list")?;
                val.offset = ((val_dst.offset - obj_dst.offset) as u16).into();
                val.num = (self.list_len() as u16).into();
                let mut offset = val_dst.offset;
                let vals =
                    List::<T>::mut_slice_from_data(val_dst, self.list_len()).context("sub lists")?;
                let mut valss = Vec::with_capacity(vals.len());
                for (i, (len, val)) in self.string_lens().unwrap().zip(vals).enumerate() {
                    offset += std::mem::size_of::<List::<T>>();
                    val.offset = ((val_dst.offset - offset) as u16).into();
                    val.num = (len as u16).into();
                    let s = u8::mut_slice_from_data(val_dst, len)
                        .with_context(|| format!("val {}", i))?;
                    if s.len() != 0 {
                        val_dst.split(1).context("string align")?;
                    }
                    valss.push(s);
                }
                BaseTypeDump::StringList(valss)
            }
            keys::OBJECTLIST_KEY => {
                let val = List::<T>::mut_from_data(obj_dst).context("list")?;
                val.offset = ((val_dst.offset - obj_dst.offset) as u16).into();
                val.num = (self.list_len() as u16).into();
                BaseTypeDump::ObjectList(
                    ReadData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::NODELIST_KEY => {
                let val = List::<T>::mut_from_data(obj_dst).context("list")?;
                val.offset = ((val_dst.offset - obj_dst.offset) as u16).into();
                val.num = (self.list_len() as u16).into();
                BaseTypeDump::NodeList(
                    ReadData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::INTLIST_KEY => {
                let val = List::<T>::mut_from_data(obj_dst).context("list")?;
                val.offset = ((val_dst.offset - obj_dst.offset) as u16).into();
                val.num = (self.list_len() as u16).into();
                let vals = T::i32::mut_slice_from_data(val_dst, self.list_len()).context("vals")?;
                // should this be here or at the start of somthing else?
                val_dst.align(16)?;
                BaseTypeDump::IntList(vals)
            }
            keys::CRCLIST_KEY => {
                let val = List::<T>::mut_from_data(obj_dst).context("list")?;
                val.offset = ((val_dst.offset - obj_dst.offset) as u16).into();
                val.num = (self.list_len() as u16).into();
                BaseTypeDump::CrcList(
                    ReadData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::WEIGHTLIST_KEY => {
                let val = List::<T>::mut_from_data(obj_dst).context("list")?;
                val.offset = ((val_dst.offset - obj_dst.offset) as u16).into();
                val.num = (self.list_len() as u16).into();
                BaseTypeDump::WeightList(
                    ReadData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            keys::MATRIXLIST_KEY => {
                let val = List::<T>::mut_from_data(obj_dst).context("list")?;
                val.offset = ((val_dst.offset - obj_dst.offset) as u16).into();
                val.num = (self.list_len() as u16).into();
                BaseTypeDump::MatrixList(
                    ReadData::mut_slice_from_data(val_dst, self.list_len()).context("vals")?,
                )
            }
            _ => return Err(anyhow!("Unkown Type {:?}", kind)),
        };
        self.write_into(val)
    }
    #[inline(always)]
    fn off_size(&self) -> usize {
        DumpBaseTypeImpl::off_size(self)
    }
}

impl<T: BaseTypes> DumpBaseTypeImpl<T> for BaseTypeRef<'_, T> {
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
                let mut s = vals.len() * std::mem::size_of::<List<T>>();
                for v in &vals[..] {
                    if v.len() != 0 {
                        s += v.len() + 1;
                    }
                }
                s
            }
            Self::ObjectList(vals) => std::mem::size_of_val(*vals),
            Self::NodeList(vals) => std::mem::size_of_val(*vals),
            Self::IntList(vals) => std::mem::size_of_val(*vals),
            Self::CrcList(vals) => std::mem::size_of_val(*vals),
            Self::WeightList(vals) => std::mem::size_of_val(*vals),
            Self::MatrixList(vals) => std::mem::size_of_val(*vals),
            _ => 0,
        }
    }
    fn write_into(&self, other: BaseTypeDump<T>) -> Result<()> {
        match (self, other) {
            (Self::Crc(val), BaseTypeDump::Crc(other)) => *other = **val,
            (Self::GUID(val), BaseTypeDump::GUID(other)) => *other = **val,
            (Self::Color(val), BaseTypeDump::Color(other)) => *other = **val,
            (Self::Vector2(val), BaseTypeDump::Vector2(other)) => *other = **val,
            (Self::Vector3(val), BaseTypeDump::Vector3(other)) => *other = **val,
            (Self::Vector4(val), BaseTypeDump::Vector4(other)) => *other = **val,
            (Self::Matrix4x4(val), BaseTypeDump::Matrix4x4(other)) => *other = **val,
            (Self::Float(val), BaseTypeDump::Float(other)) => *other = **val,
            (Self::Int(val), BaseTypeDump::Int(other)) => *other = **val,
            (Self::Bool(val), BaseTypeDump::Bool(other)) => *other = **val,
            (Self::String(val), BaseTypeDump::String(other)) => other.copy_from_slice(val.as_bytes()),
            (Self::StringList(val), BaseTypeDump::StringList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    other.copy_from_slice(val.as_bytes());
                }
            }
            (Self::ObjectList(val), BaseTypeDump::ObjectList(other)) => other.copy_from_slice(val),
            (Self::NodeList(val), BaseTypeDump::NodeList(other)) => other.copy_from_slice(val),
            (Self::IntList(val), BaseTypeDump::IntList(other)) => other.copy_from_slice(val),
            (Self::CrcList(val), BaseTypeDump::CrcList(other)) => other.copy_from_slice(val),
            (Self::WeightList(val), BaseTypeDump::WeightList(other)) => other.copy_from_slice(val),
            (Self::MatrixList(val), BaseTypeDump::MatrixList(other)) => other.copy_from_slice(val),
            _ => return Err(anyhow!("missmatched BaseTypeRef and BaseTypeDump")),
        };
        Ok(())
    }
}

impl<T: BaseTypes> DumpBaseTypeImpl<T> for BaseType
where
    Crc<T>: From<Crc<NE>>,
    Vector2<T>: From<Vector2<NE>>,
    Vector3<T>: From<Vector3<NE>>,
    Vector4<T>: From<Vector4<NE>>,
    Matrix4x4<T>: From<Matrix4x4<NE>>,
    Weight<T>: From<Weight<NE>>,
{
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
                let mut s = vals.len() * std::mem::size_of::<List<T>>();
                for v in vals {
                    if v.len() != 0 {
                        s += v.len() + 1;
                    }
                }
                s
            }
            BaseType::ObjectList(vals) => vals.len() * std::mem::size_of::<T::u32>(),
            BaseType::NodeList(vals) => vals.len() * std::mem::size_of::<Vector4<T>>(),
            BaseType::IntList(vals) => vals.len() * std::mem::size_of::<T::i32>(),
            BaseType::CrcList(vals) => vals.len() * std::mem::size_of::<Crc<T>>(),
            BaseType::WeightList(vals) => vals.len() * std::mem::size_of::<Weight<T>>(),
            BaseType::MatrixList(vals) => vals.len() * std::mem::size_of::<Matrix4x4<T>>(),
            _ => 0,
        }
    }
    fn write_into(&self, other: BaseTypeDump<T>) -> Result<()> {
        match (self, other) {
            (BaseType::Crc(val), BaseTypeDump::Crc(other)) => *other = (*val).into(),
            (BaseType::GUID(val), BaseTypeDump::GUID(other)) => *other = (*val).into(),
            (BaseType::Color(val), BaseTypeDump::Color(other)) => *other = (*val).into(),
            (BaseType::Vector2(val), BaseTypeDump::Vector2(other)) => *other = (*val).into(),
            (BaseType::Vector3(val), BaseTypeDump::Vector3(other)) => *other = (*val).into(),
            (BaseType::Vector4(val), BaseTypeDump::Vector4(other)) => *other = (*val).into(),
            (BaseType::Matrix4x4(val), BaseTypeDump::Matrix4x4(other)) => *other = (*val).into(),
            (BaseType::Float(val), BaseTypeDump::Float(other)) => *other = (*val).into(),
            (BaseType::Int(val), BaseTypeDump::Int(other)) => *other = (*val).into(),
            (BaseType::Bool(val), BaseTypeDump::Bool(other)) => *other = (*val).into(),
            (BaseType::String(val), BaseTypeDump::String(other)) => other.copy_from_slice(val.as_bytes()),
            (BaseType::StringList(val), BaseTypeDump::StringList(other)) => {
                for (val, other) in val.iter().zip(other) {
                    other.copy_from_slice(val.as_bytes());
                }
            }
            (BaseType::ObjectList(val), BaseTypeDump::ObjectList(other)) => {
                for (&val, other) in val.iter().zip(other) {
                    *other = val.into()
                }
            }
            (BaseType::NodeList(val), BaseTypeDump::NodeList(other)) => {
                for (&val, other) in val.iter().zip(other) {
                    *other = val.into()
                }
            }
            (BaseType::IntList(val), BaseTypeDump::IntList(other)) => {
                for (&val, other) in val.iter().zip(other) {
                    *other = val.into()
                }
            }
            (BaseType::CrcList(val), BaseTypeDump::CrcList(other)) => {
                for (&val, other) in val.iter().zip(other) {
                    *other = val.val.into() 
                }
            }
            (BaseType::WeightList(val), BaseTypeDump::WeightList(other)) => {
                for (&val, other) in val.iter().zip(other) {
                    *other = val.into()
                }
            }
            (BaseType::MatrixList(val), BaseTypeDump::MatrixList(other)) => {
                for (&val, other) in val.iter().zip(other) {
                    *other = val.into()
                }
            }
            _ => return Err(anyhow!("missmatched BaseTypeRef and BaseTypeDump")),
        }
        Ok(())
    }
}

#[derive_pod]
pub struct GameObjsHeader<T: BaseTypes> {
    pub const_: T::u32,
    pub types_num: T::u32,
    pub types_offset: T::u32,
    pub obj_num: T::u32,
    pub obj_offset: T::u32,
    pub z5: T::u32,
    pub z6: T::u32,
    pub z7: T::u32,
}

#[derive_pod]
pub struct TypeHeader<T: BaseTypes> {
    pub key: T::u32,
    pub size: T::u32,
    pub fields: T::u32,
}

#[derive_pod]
pub struct TypeField<T: BaseTypes> {
    pub key: T::u32,
    pub kind: T::u32,
    pub offset: T::u32,
}

pub trait TypeFieldDump {
    fn key(&self) -> u32;
    fn kind(&self) -> u32;
    fn offset(&self) -> u32;
}

impl<T: BaseTypes> TypeFieldDump for TypeField<T> {
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

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, Clone, PartialEq)]
pub struct TypeRef<'a, T: BaseTypes> {
    pub header: &'a TypeHeader<T>,
    pub fields: ref_slice<'a, TypeField<T>>,
}

impl<T: BaseTypes> Default for TypeRef<'_, T> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            fields: ref_slice::default()
        }
    }
}

impl<'a, T: BaseTypes> TypeRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let header = TypeHeader::<T>::from_data(src).context("header")?;
        let fields = TypeField::slice_from_data(&src[std::mem::size_of::<TypeHeader<T>>()..], header.size.into() as usize)
            .context("fields")?.into();
        Ok(Self { header, fields })
    }
    fn size(&self) -> usize {
        std::mem::size_of::<TypeHeader<T>>() + self.fields.len() * std::mem::size_of::<TypeField<T>>()
    }
}

pub type Type = Vec<TypeField<NE>>;
impl<T: BaseTypes> From<TypeRef<'_, T>> for Type
where
    TypeField<NE>: From<TypeField<T>>
{
    fn from(val: TypeRef<T>) -> Self {
        val.fields.iter().map(|&x| x.into()).collect()
    }
}

pub trait DumpTypeImpl<T: BaseTypes> {
    fn fields_len(&self) -> usize;
    fn fields(&self) -> impl Iterator<Item = &impl TypeFieldDump>;
    fn write_header(&self, header: &mut TypeHeader<T>) -> Result<()>;
    fn write_fields(&self, fields: &mut [TypeField<T>]) -> Result<()>;
}

#[enum_dispatch]
pub trait DumpType<T: BaseTypes> {
    fn fields_len(&self) -> usize;
    fn get_info(&self) -> (usize, Vec<(u32, u32, u32)>);
    fn dump_into(&self, dst: &mut DumpSlice, key: u32) -> Result<()>; 
    fn size(&self) -> usize; 
}

impl<T: BaseTypes, I: DumpTypeImpl<T>> DumpType<T> for I {
    #[inline(always)]
    fn fields_len(&self) -> usize {
        DumpTypeImpl::fields_len(self)
    }
    fn get_info(&self) -> (usize, Vec<(u32, u32, u32)>) {
        let mut off = 0;
        let infos = self.fields().map(|t| {
            off = off.max(t.offset() as usize + BaseTypeRef::<T>::size(t.kind()));
            (t.key(), t.kind(), t.offset())
        }).collect();
        (off, infos)
    }
    fn dump_into(&self, dst: &mut DumpSlice, key: u32) -> Result<()> {
        let header = TypeHeader::mut_from_data(dst).context("header")?;
        self.write_header(header).context("write header")?;
        header.key = key.into();
        let fields = TypeField::mut_slice_from_data(dst, self.fields_len()).context("fields")?;
        self.write_fields(fields).context("write fields")?;
        Ok(())
    }
    fn size(&self) -> usize {
        std::mem::size_of::<TypeHeader<T>>() + self.fields_len() * std::mem::size_of::<TypeField<T>>()
    }
}

impl<T: BaseTypes> DumpTypeImpl<T> for TypeRef<'_, T> {
    fn fields_len(&self) -> usize {
        self.fields.len()
    }
    fn fields(&self) -> impl Iterator<Item = &impl TypeFieldDump> {
        self.fields.iter()
    }
    fn write_header(&self, header: &mut TypeHeader<T>) -> Result<()> {
        *header = *self.header;
        Ok(())
    }
    fn write_fields(&self, fields: &mut [TypeField<T>]) -> Result<()> {
        fields.copy_from_slice(self.fields);
        Ok(())
    }
}

impl<T: BaseTypes> DumpTypeImpl<T> for Type
where
    TypeField<T>: From<TypeField<NE>>
{
    fn fields_len(&self) -> usize {
        self.len()
    }
    fn fields(&self) -> impl Iterator<Item = &impl TypeFieldDump> {
        self.iter()
    }
    fn write_header(&self, header: &mut TypeHeader<T>) -> Result<()> {
        header.size = (self.len() as u32).into();
        header.fields = 0u32.into();
        Ok(())
    }
    fn write_fields(&self, fields: &mut [TypeField<T>]) -> Result<()> {
        for (&src, dst) in self.iter().zip(fields) {
            *dst = src.into()
        }
        Ok(())
    }
}

/*
#[enum_dispatch(DumpType_XE_)]
pub enum Type_XE_<'a> {
    Ref(TypeRef_XE_<'a>),
    Owned(Type)
}
*/

#[derive_pod]
pub struct ObjHeader<T: BaseTypes> {
    pub layer: T::u32,
    pub key: Crc<T>,
    pub size: T::u16,
    pub z3: T::u16,
    pub z4: T::u32,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ObjRef<'a, T: BaseTypes> {
    pub header: &'a ObjHeader<T>,
    pub fields: IndexMap<u32, BaseTypeRef<'a, T>>,
}

impl<T: BaseTypes> Default for ObjRef<'_, T> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            fields: IndexMap::default().into()
        }
    }
}

impl<'a, T: BaseTypes> ObjRef<'a, T> {
    pub fn from_data(src: &'a [u8], types: &IndexMap<u32, TypeRef<'a, T>>) -> Result<Self> {
        let mut offset = 0;
        let header = ObjHeader::<T>::from_data(&src[offset..]).context("header")?;
        offset += std::mem::size_of::<ObjHeader<T>>();
        let ts = &types
            .get(&header.key.val.into())
            .ok_or(anyhow!("Missing Key {:?}", header.key.val.into()))?
            .fields;
        let mut fields = IndexMap::with_capacity(ts.len());
        for t in ts.iter() {
            fields.insert(
                t.key.into(),
                BaseTypeRef::<T>::from_data(&src[offset + t.offset.into() as usize..], t.kind.into())
                    .with_context(|| {
                        format!(
                            "{}field {}",
                            if let Some(BaseTypeRef::<T>::GUID(o)) =
                                fields.get(&hash_string(b"guid", None))
                            {
                                format!("guid {}, ", (*o).clone().into())
                            } else {
                                String::new()
                            },
                            get_str_debug(&t.key.into())
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
        std::mem::size_of::<ObjHeader<T>>() + self.header.size.into() as usize
    }
}

#[derive(Debug, Clone)]
pub struct Obj {
    pub layer: u32,
    pub key: Crc<NE>,
    pub fields: IndexMap<Crc<NE>, BaseType>,
}

impl<T: BaseTypes> From<&ObjRef<'_, T>> for Obj
where
    Crc<NE>: From<Crc<T>>,
    Vector2<NE>: From<Vector2<T>>,
    Vector3<NE>: From<Vector3<T>>,
    Vector4<NE>: From<Vector4<T>>,
    Matrix4x4<NE>: From<Matrix4x4<T>>,
    Weight<NE>: From<Weight<T>>,
{
    fn from(val: &ObjRef<T>) -> Self {
        Self {
            layer: val.header.layer.into(),
            key: Crc::new(val.header.key.val.into()),
            fields: val.fields.iter().map(|(&k,v)| (Crc::new(k.into()), v.into())).collect()
        }
    }
}

/*
pub struct ObjImpl_XE_<'a> {
    pub layer: u32,
    pub key: Crc,
    pub fields: IndexMap<Crc, BaseType_XE_<'a>>
}

impl<'a> From<ObjRef_XE_<'a>> for ObjImpl_XE_<'a> {
    fn from(mut val: ObjRef_XE_<'a>) -> Self {
        Self {
            layer: val.header.layer.into(),
            key: val.header.key.into(),
            fields: val.fields.drain(..).map(|(k,v)| (k.into(), v.into())).collect()
        }
    }
}

#[enum_dispatch(DumpObj_XE_)]
pub enum Obj_XE_<'a> {
    Ref(ObjRef_XE_<'a>),
    Owned(ObjImpl_XE_<'a>),
}
impl <T: BaseTypes>DumpObjImpl_XE_ for ObjImpl_XE_<'_> {
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
*/

pub trait DumpObjImpl<T: BaseTypes> {
    fn key(&self) -> u32;
    fn layer(&self) -> u32;
    fn fields(&self) -> impl Iterator<Item=&impl DumpBaseType<T>>;
}

#[enum_dispatch]
pub trait DumpObj<T: BaseTypes> {
    fn key(&self) -> u32;
    fn dump_into(&self, dst: &mut DumpSlice, val_offset: usize, fields: &[(u32, u32, u32)]) -> Result<()>;
    fn size(&self, val_offset: usize, fields: &[(u32, u32, u32)]) -> usize;
}

impl<T: BaseTypes, I: DumpObjImpl<T>> DumpObj<T> for I {
    #[inline(always)]
    fn key(&self) -> u32 {
        DumpObjImpl::key(self)
    }
    fn dump_into(&self, dst: &mut DumpSlice, val_offset: usize, fields: &[(u32, u32, u32)]) -> Result<()> {
        let header = ObjHeader::<T>::mut_from_data(dst).context("header")?;
        let start = dst.offset;
        let mut obj_dst = dst.split(val_offset).context("obj data split")?;
        dst.align(16)?;
        for (f, (key, kind, offset)) in self.fields().zip(fields) {
            let mut obj_dst = obj_dst.view(*offset as usize);
            f.dump_into(&mut obj_dst, dst, *kind)
                .with_context(|| format!("field {}", key))?;
        }
        dst.align(16)?;
        header.layer = self.layer().into();
        header.key.val = self.key().into();
        header.size = ((dst.offset - start) as u16).into();
        Ok(())
    }

    fn size(&self, val_offset: usize, fields: &[(u32, u32, u32)]) -> usize {
        let mut off = align_offset(val_offset + std::mem::size_of::<ObjHeader<T>>(), 16);
        for (f, (key, _, _)) in self.fields().zip(fields) {
            off += f.off_size();
            if *key == keys::INTLIST_KEY {
                off = align_offset(off, 16);
            }
        }
        align_offset(off, 16)
    }
}

impl<T: BaseTypes> DumpObjImpl<T> for ObjRef<'_, T> {
    fn key(&self) -> u32 {
        self.header.key.val.into()
    }
    fn layer(&self) -> u32 {
        self.header.layer.into()
    }
    fn fields(&self) -> impl Iterator<Item=&impl DumpBaseType<T>> {
        self.fields.values()
    }
}

impl<T: BaseTypes> DumpObjImpl<T> for Obj
where
    Crc<T>: From<Crc<NE>>,
    Vector2<T>: From<Vector2<NE>>,
    Vector3<T>: From<Vector3<NE>>,
    Vector4<T>: From<Vector4<NE>>,
    Matrix4x4<T>: From<Matrix4x4<NE>>,
    Weight<T>: From<Weight<NE>>,
{
    fn key(&self) -> u32 {
        self.key.val
    }
    fn layer(&self) -> u32 {
        self.layer
    }
    fn fields(&self) -> impl Iterator<Item=&impl DumpBaseType<T>> {
        self.fields.values()
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct GameObjsRef<'a, T: BaseTypes> {
    pub header: &'a GameObjsHeader<T>,
    pub types: IndexMap<u32, TypeRef<'a, T>>,
    pub objs: IndexMap<u32, ObjRef<'a, T>>
}

impl<T: BaseTypes> Default for GameObjsRef<'_, T> {
    fn default() -> Self {
        Self {
            header: get_default_ref(),
            types: IndexMap::default().into(),
            objs: IndexMap::default().into()
        }
    }
}

impl<'a, T: BaseTypes> GameObjsRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let header = GameObjsHeader::<T>::from_data(src).context("header")?;
        if header.const_.into() != 1296123652 {
            log::error!("Invalid gameobj block");
        }
        let mut offset = header.types_offset.into() as usize;
        let mut types = IndexMap::with_capacity(header.types_num.into() as usize);
        for i in 0..header.types_num.into() {
            let t = TypeRef::<T>::from_data(&src[offset..]).with_context(|| format!("ty {}", i))?;
            offset += t.size();
            types.insert(t.header.key.into(), t);
        }

        offset = header.obj_offset.into() as usize;
        let mut objs = IndexMap::with_capacity(header.obj_num.into() as usize);
        for i in 0..header.obj_num.into() {
            let o =
                ObjRef::from_data(&src[offset..], &types).with_context(|| format!("obj {}", i))?;
            offset += o.size();
            if let BaseTypeRef::GUID(guid) = o
                .fields
                .get(&hash_string(b"guid", None))
                .ok_or(anyhow!("obj missing guid"))?
            {
                objs.insert((*guid).clone().into(), o);
            } else {
                return Err(anyhow!("obj incorrect guid type"));
            }
        }
        Ok(Self { header, objs: objs.into(), types: types.into() })
    }
    pub fn level_name(&self) -> Result<u32> {
        let name = self.objs.values()
            .find(|v| v.header.key.val.into() == hash_string(b"templateLevel", None))
            .ok_or(anyhow!("templateLevel not found"))?
            .fields.get(&hash_string(b"name", None))
            .ok_or(anyhow!("templateLevel missing name field"))
            .and_then(|field| if let BaseTypeRef::Crc(val) = field {
                Ok(val.val.into())
            } else {
                Err(anyhow!("templateObject name field is not a crc"))
            })?;
        Ok(name)
    }
}

#[derive(Debug, Clone)]
pub struct GameObjs {
    pub objs: IndexMap<u32, Obj>,
    pub types: IndexMap<Crc<NE>, Vec<TypeField<NE>>>,
}

impl<T: BaseTypes> From<&GameObjsRef<'_, T>> for GameObjs
where
    Crc<NE>: From<Crc<T>>,
    Vector2<NE>: From<Vector2<T>>,
    Vector3<NE>: From<Vector3<T>>,
    Vector4<NE>: From<Vector4<T>>,
    Matrix4x4<NE>: From<Matrix4x4<T>>,
    Weight<NE>: From<Weight<T>>,
    TypeField<NE>: From<TypeField<T>>,
{
    fn from(val: &GameObjsRef<T>) -> Self {
        Self {
            objs: val.objs.iter().map(|(k, v)| (*k, v.into())).collect(),
            types: val.types.iter().map(|(&k, v)| (Crc::new(k), v.fields.iter().map(|&x| x.into()).collect())).collect() 
        }
    }
}

/*
pub struct GameObjsImpl_XE_<'a> {
    pub objs: IndexMap<u32, Obj_XE_<'a>>,
    pub types: IndexMap<Crc, Type_XE_<'a>>,
}

impl<'a> From<GameObjsRef_XE_<'a>> for GameObjsImpl_XE_<'a> {
    fn from(mut val: GameObjsRef_XE_<'a>) -> Self {
        Self {
            objs: val.objs.drain(..).map(|(k, v)| (k, v.into())).collect(),
            types: val.types.drain(..).map(|(k, v)| (k.into(), v.into())).collect() 
        }
    }
}

#[enum_dispatch(DumpGameObjs_XE_)]
pub enum GameObjs_XE_<'a> {
    Ref(GameObjsRef_XE_<'a>),
    Owned(GameObjsImpl_XE_<'a>)
}

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
                        Ok(val.into())
                    } else {
                        Err(anyhow!("templateObject name field is not a crc"))
                    })?
            },
            Obj_XE_::Owned(val) => {
                val.fields.get(&Crc::new(hash_string(b"name", None)))
                    .ok_or(anyhow!("templateLevel missing name field"))
                    .and_then(|field| match field {
                        BaseType_XE_::Ref(BaseTypeRef_XE_::Crc(val)) => Ok(val.into()),
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
*/

pub trait DumpGameObjsImpl<T: BaseTypes> {
    fn types(&self) -> impl Iterator<Item = (u32, &impl DumpType<T>)>;
    fn types_num(&self) -> usize;
    fn objs(&self) -> impl Iterator<Item = &impl DumpObj<T>>;
    fn objs_num(&self) -> usize;
    fn level_name(&self) -> Result<u32>;
    fn guid_order(&self, guid: &u32) -> usize;
}

pub type TypeInfos = IndexMap<u32, (usize, Vec<(u32, u32, u32)>)>;

#[enum_dispatch]
pub trait DumpGameObjs<T: BaseTypes> {
    fn size(&self) -> (usize, TypeInfos);
    fn dump_into(&self, dst: &mut DumpSlice, infos: &TypeInfos) -> Result<()>;
    fn level_name(&self) -> Result<u32>;
    fn guid_order(&self, guid: &u32) -> usize;
}

impl<T: BaseTypes, I: DumpGameObjsImpl<T>> DumpGameObjs<T> for I {
    fn size(&self) -> (usize, TypeInfos) {
        let mut size = std::mem::size_of::<GameObjsHeader<T>>() + self.types_num() * std::mem::size_of::<TypeHeader<T>>();
        let mut fields_num = 0;
        let type_infos: TypeInfos = self.types().map(|(key, x)| {
            fields_num += x.fields_len();
            (key, x.get_info())
        }).collect();
        size += fields_num * std::mem::size_of::<TypeField<T>>();

    
        size = align_offset(size, 16);
        for obj in self.objs() {
            let (off, info) = type_infos.get(&obj.key()).unwrap();
            size += obj.size(*off, info);
        }
        (size, type_infos)
    }
    fn dump_into(&self, dst: &mut DumpSlice, infos: &TypeInfos) -> Result<()> {
        let start = dst.offset;
        let header = GameObjsHeader::<T>::mut_from_data(dst).context("header")?;
        header.const_ = 1296123652u32.into();

        header.types_offset = ((dst.offset - start) as u32).into();
        header.types_num = (self.types_num() as u32).into();
        for (key, ty) in self.types() {
            ty.dump_into(dst, key)
                .with_context(|| format!("type {}", key))?;
        }
        dst.align(16)?;

        header.obj_offset = ((dst.offset - start) as u32).into();
        header.obj_num = (self.objs_num() as u32).into();
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
        DumpGameObjsImpl::level_name(self)
    }
    #[inline(always)]
    fn guid_order(&self, guid: &u32) -> usize {
        DumpGameObjsImpl::guid_order(self, guid)
    }
} 

impl<T: BaseTypes> DumpGameObjsImpl<T> for GameObjsRef<'_, T> {
    fn types(&self) -> impl Iterator<Item = (u32, &impl DumpType<T>)> {
        self.types.iter().map(|(k,v)| (*k, v))
    }
    fn types_num(&self) -> usize {
        self.types.len()
    }
    fn objs(&self) -> impl Iterator<Item = &impl DumpObj<T>> {
        self.objs.values()
    }
    fn objs_num(&self) -> usize {
        self.objs.len()
    }
    fn level_name(&self) -> Result<u32> {
        let name = self.objs.values()
            .find(|v| v.header.key.val.into() == hash_string(b"templateLevel", None))
            .ok_or(anyhow!("templateLevel not found"))?
            .fields.get(&hash_string(b"name", None))
            .ok_or(anyhow!("templateLevel missing name field"))
            .and_then(|field| if let BaseTypeRef::Crc(val) = field {
                Ok(val.val.into())
            } else {
                Err(anyhow!("templateObject name field is not a crc"))
            })?;
        Ok(name)
    }
    fn guid_order(&self, guid: &u32) -> usize {
        self.objs.get_index_of(guid).unwrap_or_default()
    }
}

impl<T: BaseTypes> DumpGameObjsImpl<T> for GameObjs
where
    Crc<T>: From<Crc<NE>>,
    Vector2<T>: From<Vector2<NE>>,
    Vector3<T>: From<Vector3<NE>>,
    Vector4<T>: From<Vector4<NE>>,
    Matrix4x4<T>: From<Matrix4x4<NE>>,
    Weight<T>: From<Weight<NE>>,
    TypeField<T>: From<TypeField<NE>>,
{
    fn types(&self) -> impl Iterator<Item = (u32, &impl DumpType<T>)> {
        self.types.iter().map(|(k,v)| (k.val, v))
    }
    fn types_num(&self) -> usize {
        self.types.len()
    }
    fn objs(&self) -> impl Iterator<Item = &impl DumpObj<T>> {
        self.objs.values()
    }
    fn objs_num(&self) -> usize {
        self.objs.len()
    }
    fn level_name(&self) -> Result<u32> {
        let name = self.objs.values()
            .find(|v| v.key.val == hash_string(b"templateLevel", None))
            .ok_or(anyhow!("templateLevel not found"))?
            .fields.get(&Crc::new(hash_string(b"name", None)))
            .ok_or(anyhow!("templateLevel missing name field"))
            .and_then(|field| if let BaseType::Crc(val) = field {
                Ok(val.val)
            } else {
                Err(anyhow!("templateObject name field is not a crc"))
            })?;
        Ok(name)
    }
    fn guid_order(&self, guid: &u32) -> usize {
        self.objs.get_index_of(guid).unwrap_or_default()
    }
}

