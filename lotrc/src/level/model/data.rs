use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use itertools::Itertools;
use log::debug;

use crate::types::{Color, ReadData, Vector2, Vector3, Vector4, DumpSlice, CompressedDataRef, ref_slice, BaseTypes, CompressedData, NE};
use crate::level::{
    model::ModelInfo,
    pak::block1::{
        gameobjs::keys::{INT_KEY, COLOR_KEY, VECTOR2_KEY, VECTOR3_KEY, VECTOR4_KEY},
        infos::{InfoCounts, DumpInfos}
    },
};

use lotrc_proc::{derive_pod};

#[derive_pod]
pub struct BufferInfoPs3<T: BaseTypes> {
    pub vbuff_info_offset: T::u32,   // pointer to objf
    pub vbuff_info_offset_2: T::u32, // optional pointer to objf
    pub vbuff_info_offset_3: T::u32, // optional pointer to objf
    pub unk_3: T::u32,
    pub unk_4: T::u32,
    pub unk_5: T::u32,
    pub unk_6: T::u32,
    pub unk_7: T::u32,
    pub unk_8: T::u32,
    pub unk_9: T::u32,
    pub unk_10: T::u32,
    pub unk_11: T::u32,
    pub unk_12: T::u32,
    pub unk_13: T::u32,
    pub unk_14: T::u32,
    pub unk_15: T::u32,
    pub unk_16: T::u32,
    pub ibuff_info_offset: T::u32, // not the correct place, but needed to get code running
    pub v_size: T::u32, // not the correct place, but needed to get code running
    pub vbuff_size: T::u32, // not the correct place, but needed to get code running
}

#[derive_pod]
pub struct BufferInfo<T: BaseTypes> {
    pub vbuff_info_offset: T::u32,   // pointer to objf
    pub vbuff_info_offset_2: T::u32, // optional pointer to objf
    pub vbuff_info_offset_3: T::u32, // optional pointer to objf
    pub unk_3: T::u32,
    pub unk_4: T::u32,
    pub unk_5: T::u32,
    pub unk_6: T::u32,
    pub unk_7: T::u32,
    pub unk_8: T::u32,
    pub unk_9: T::u32,
    pub unk_10: T::u32,
    pub unk_11: T::u32,
    pub unk_12: T::u32,
    pub unk_13: T::u32,
    pub unk_14: T::u32,
    pub unk_15: T::u32,
    pub unk_16: T::u32,
    pub unk_17: T::u32,
    pub unk_18: T::u32,
    pub unk_19: T::u32,
    pub unk_20: T::u32,
    pub unk_21: T::u32,
    pub unk_22: T::u32,
    pub unk_23: T::u32,
    pub unk_24: T::u32,
    pub unk_25: T::u32,
    pub unk_26: T::u32,
    pub unk_27: T::u32,
    pub unk_28: T::u32,
    pub unk_29: T::u32,
    pub unk_30: T::u32,
    pub unk_31: T::u32,
    pub v_size: T::u32,
    pub v_size_2: T::u32,
    pub v_size_3: T::u32,
    pub unk_35: T::u32,
    pub unk_36: T::u32,
    pub unk_37: T::u32,
    pub unk_38: T::u32,
    pub unk_39: T::u32,
    pub unk_40: T::u32,
    pub unk_41: T::u32,
    pub unk_42: T::u32,
    pub unk_43: T::u32,
    pub unk_44: T::u32,
    pub unk_45: T::u32,
    pub unk_46: T::u32,
    pub unk_47: T::u32,
    pub vbuff_size: T::u32,
    pub vbuff_size_2: T::u32,
    pub vbuff_size_3: T::u32,
    pub unk_51: T::u32,
    pub unk_52: T::u32,
    pub unk_53: T::u32,
    pub unk_54: T::u32,
    pub unk_55: T::u32,
    pub unk_56: T::u32,
    pub unk_57: T::u32,
    pub unk_58: T::u32,
    pub unk_59: T::u32,
    pub unk_60: T::u32,
    pub unk_61: T::u32,
    pub unk_62: T::u32,
    pub unk_63: T::u32,
    pub unk_64: T::u32,
    pub ibuff_info_offset: T::u32, // poiner to objg
    pub i_num: T::u32, // number of indeices in ibuffer
    pub unk_67: T::u32,
    pub skin_offset: T::u32,
    pub skin_size: T::u32,
    pub unk_70: T::u32,
    pub tri_num: T::u32, // number of objects(triangles) in ibuffer
    pub unk_72: T::u32, // possibly index to bone_transform used for mesh
    pub unk_73: T::u32,
    pub unk_74: T::u32,
    pub unk_75: T::u32,
    pub unk_76: T::u32,
    pub unk_77: T::u32,
    pub unk_78: T::u32,
    pub unk_79: T::u32,
    pub unk_80: T::u32,
    pub unk_81: T::u32,
    pub unk_82: T::u32,
    pub unk_83: T::u32,
    pub unk_84: T::u32,
    pub unk_85: T::u32,
    pub unk_86: T::u32,
    pub unk_87: T::u32,
    pub variation_id: u8,
    pub variation: u8,
    pub unk_88c: u8,
    pub unk_88d: u8,
}

#[derive_pod]
pub struct VBuffInfoXbox<T: BaseTypes> {
    pub unk_0: T::u32,
    pub size: T::u32,
    pub unk_3: T::u32,
    pub offset: T::u32,
    pub fmt2: T::u32,
    pub fmt1: T::u32,
    pub unk_6: T::u32,
    pub unk_7: T::u32,
    pub unk_8: T::u32,
    pub unk_9: T::u32,
    pub unk_10: T::u32,
    pub unk_11: T::u32,
    pub unk_12: T::u32,
    pub unk_13: T::u32,
}

#[derive_pod]
pub struct VBuffInfoPS3<T: BaseTypes> {
    pub unk_0: T::u32,
    pub unk_7: T::u32,
    pub unk_3: T::u32,
    pub unk_8: T::u32,
    pub unk_9: T::u32,
    pub size: T::u32,
    pub unk_6: T::u32,
    pub offset: T::u32,
    pub fmt2: T::u32,
    pub fmt1: T::u32,
}

#[derive_pod]
pub struct VBuffInfo<T: BaseTypes> {
    pub unk_0: T::u32,
    pub size: T::u32,
    pub unk_3: T::u32,
    pub offset: T::u32,
    pub fmt1: T::u32,
    pub fmt2: T::u32,
    pub unk_6: T::u32,
    pub unk_7: T::u32,
}

#[derive_pod]
pub struct IBuffInfoXbox<T: BaseTypes> {
    pub unk_0: T::u32,
    pub size: T::u32,
    pub format: T::u32,
    pub vbuff_alt_fmt: T::u32, // 1 if vbuff.fmt1 & 0x40000 != 0 else 0; 0 for Xbox
    pub offset: T::u32,
    pub unk_5: T::u32,
    pub unk_6: T::u32,
    pub unk_7: T::u32,
    pub unk_8: T::u32,
    pub unk_9: T::u32,
    pub unk_10: T::u32,
    pub unk_11: T::u32,
    pub unk_12: T::u32,
}

#[derive_pod]
pub struct IBuffInfoPs3<T: BaseTypes> {
    pub unk_0: T::u32,
    pub unk_5: T::u32,
    pub unk_6: T::u32,
    pub vbuff_alt_fmt: T::u32, // 1 if vbuff.fmt1 & 0x40000 != 0 else 0; 0 for Xbox
    pub unk_8: T::u32,
    pub size: T::u32,
    pub format: T::u32,
    pub unk_7: T::u32,
    pub offset: T::u32,
}

#[derive_pod]
pub struct IBuffInfo<T: BaseTypes> {
    pub unk_0: T::u32,
    pub size: T::u32,
    pub format: T::u32,
    pub vbuff_alt_fmt: T::u32, // 1 if vbuff.fmt1 & 0x40000 != 0 else 0; 0 for Xbox
    pub offset: T::u32,
    pub unk_5: T::u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[repr(C, u8)]
pub enum VertexUsage {
    Position,
    Normal,
    Tangent,
    BlendWeight,
    BlendIndices,
    Color(usize),
    TextureCoord(usize),
    //Unknown(usize),
    PSize,
    Pad(usize),
}

impl std::str::FromStr for VertexUsage {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string = s.to_lowercase();
        let s = string.as_str();
        match s {
            "position" => Ok(Self::Position),
            "normal" => Ok(Self::Normal),
            "tangent" => Ok(Self::Tangent),
            "blendweight" => Ok(Self::BlendWeight),
            "blendindices" => Ok(Self::BlendIndices),
            "psize" => Ok(Self::PSize),
            s => {
                if s.starts_with("color(") {
                    Ok(s[6..]
                        .split(')')
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .map(|i| Self::Color(i))?)
                } else if s.starts_with("texturecoord(") {
                    Ok(s[13..]
                        .split(')')
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .map(|i| Self::TextureCoord(i))?)
                } else if s.starts_with("pad(") {
                    Ok(s[4..]
                        .split(')')
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .map(|i| Self::Pad(i))?)
                } else {
                    Err(anyhow!("unknown vertex usage {}", s))
                }
            }
        }
    }
}

impl Eq for VertexUsage {}

impl std::fmt::Display for VertexUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Position => write!(f, "Position"),
            Self::Normal => write!(f, "Normal"),
            Self::Tangent => write!(f, "Tangent"),
            Self::BlendWeight => write!(f, "BlendWeight"),
            Self::BlendIndices => write!(f, "BlendIndices"),
            Self::Color(i) => write!(f, "Color({})", i),
            Self::TextureCoord(i) => write!(f, "TextureCoord({})", i),
            Self::PSize => write!(f, "PSize"),
            Self::Pad(i) => write!(f, "Pad({})", i),
        }
    }
}

impl VertexUsage {
    pub fn order(&self) -> usize {
        match self {
            Self::Position => 0,
            Self::BlendWeight | Self::Pad(0) => 1,
            Self::BlendIndices | Self::Pad(1) => 2,
            Self::Pad(2) => 3,
            Self::Pad(3) => 4,
            Self::Pad(4) => 5,
            Self::Normal => 6,
            Self::Color(0) => 7,
            Self::Color(1) => 8,
            Self::TextureCoord(0) => 9,
            Self::TextureCoord(1) => 10,
            Self::TextureCoord(2) => 11,
            Self::TextureCoord(3) => 12,
            Self::Pad(5) => 13,
            Self::Pad(6) => 14,
            Self::Pad(7) => 15,
            Self::Tangent => 16,
            Self::PSize => 17,
            _ => usize::MAX,
        }
    }
}

pub enum VertexVal<T: BaseTypes> {
    Int(T::u32),
    Color(T::u32),
    Vector2(Vector2<T>),
    Vector3(Vector3<T>),
    Vector4(Vector4<T>),
}

impl<T: BaseTypes> VertexVal<T> {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Int(val) => bytemuck::bytes_of(val),
            Self::Color(val) => bytemuck::bytes_of(val),
            Self::Vector2(val) => bytemuck::bytes_of(val),
            Self::Vector3(val) => bytemuck::bytes_of(val),
            Self::Vector4(val) => bytemuck::bytes_of(val),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VertexData {
    Int(Vec<u32>),
    Color(Vec<Color>),
    Vector2(Vec<Vector2<NE>>),
    Vector3(Vec<Vector3<NE>>),
    Vector4(Vec<Vector4<NE>>),
}

impl VertexData {
    pub fn get<T: BaseTypes>(&self, i: usize) -> VertexVal<T>
    where
        Vector2<T>: From<Vector2<NE>>,
        Vector3<T>: From<Vector3<NE>>,
        Vector4<T>: From<Vector4<NE>>,
    {
        match self {
            Self::Int(vals) => VertexVal::Int(vals[i].clone().into()),
            Self::Color(vals) => VertexVal::Color(vals[i].clone().into()),
            Self::Vector2(vals) => VertexVal::Vector2(vals[i].into()),
            Self::Vector3(vals) => VertexVal::Vector3(vals[i].into()),
            Self::Vector4(vals) => VertexVal::Vector4(vals[i].into()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Int(vals) => vals.len(),
            Self::Color(vals) => vals.len(),
            Self::Vector2(vals) => vals.len(),
            Self::Vector3(vals) => vals.len(),
            Self::Vector4(vals) => vals.len(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Int(..) => 4,
            Self::Color(..) => 4,
            Self::Vector2(..) => 8,
            Self::Vector3(..) => 12,
            Self::Vector4(..) => 16,
        }
    }
}

fn parse_fmt(fmt1: u32, fmt2: u32) -> (usize, IndexMap<VertexUsage, VertexDataIndex>) {
    let mut vals = IndexMap::new();
    let mut s = 0;
    if fmt2 == 0 {
        let b1: bool = (fmt1 & 0x40000) != 0;
        if fmt1 & 1 != 0 {
            if b1 {
                vals.insert(
                    VertexUsage::Position,
                    VertexDataIndex { key: VECTOR4_KEY, offset: s },
                );
                s += 16;
            } else {
                vals.insert(
                    VertexUsage::Position,
                    VertexDataIndex { key: VECTOR3_KEY, offset: s },
                );
                s += 12;
            }
        }
        if (fmt1 & 0x400) != 0 {
            // blend weights
            if b1 {
                vals.insert(
                    VertexUsage::Pad(0),
                    VertexDataIndex { key: COLOR_KEY, offset: s },
                );
            } else {
                vals.insert(
                    VertexUsage::BlendWeight,
                    VertexDataIndex { key: COLOR_KEY, offset: s },
                );
            }
            s += 4;
        }
        if (fmt1 & 0x800) != 0 {
            // blend indices
            if b1 {
                vals.insert(
                    VertexUsage::Pad(1),
                    VertexDataIndex { key: COLOR_KEY, offset: s },
                );
            } else {
                vals.insert(
                    VertexUsage::BlendIndices,
                    VertexDataIndex { key: COLOR_KEY, offset: s },
                );
            }
            s += 4;
            // if b1 then Vec4 ??
        }
        if (fmt1 & 2) != 0 {
            // normal
            let usage = VertexUsage::Normal;
            if b1 {
                let mut p = 2;
                for _ in (0..(((s + 15) & 0xFFFF0) - s)).step_by(4) {
                    vals.insert(
                        VertexUsage::Pad(p),
                        VertexDataIndex { key: COLOR_KEY, offset: s },
                    );
                    s += 4;
                    p += 1;
                }
                vals.insert(
                    usage,
                    VertexDataIndex { key: VECTOR4_KEY, offset: s },
                );
                s += 16;
            //                } else if V::ps3() {
            //                   vals.insert(VertexDataIndex { key: Vector3_KEY, offset: s } usage));
            //                   s += 12;
            } else {
                vals.insert(
                    usage,
                    VertexDataIndex { key: COLOR_KEY, offset: s },
                );
                s += 4;
            }
        }
        if fmt1 & 0x100 != 0 {
            // color(0)
            vals.insert(
                VertexUsage::Color(0),
                VertexDataIndex { key: COLOR_KEY, offset: s },
            );
            s += 4;
        }
        if fmt1 & 0x200 != 0 {
            // color(1)
            vals.insert(
                VertexUsage::Color(1),
                VertexDataIndex { key: COLOR_KEY, offset: s },
            );
            s += 4;
        }
        for i in 0..((fmt1 >> 2) & 0xF) {
            // texture coords
            vals.insert(
                VertexUsage::TextureCoord(i as usize),
                VertexDataIndex { key: VECTOR2_KEY, offset: s },
            );
            s += 8;
        }
        if fmt1 & 0x40 != 0 {
            // tangent
            let usage = VertexUsage::Tangent;
            if b1 {
                let mut p = 5;
                for _ in (0..(((s + 15) & 0xFFFF0) - s)).step_by(4) {
                    vals.insert(
                        VertexUsage::Pad(p),
                        VertexDataIndex { key: COLOR_KEY, offset: s },
                    );
                    s += 4;
                    p += 1;
                }
                vals.insert(
                    usage,
                    VertexDataIndex { key: VECTOR4_KEY, offset: s },
                );
                s += 16;
            } else {
                vals.insert(
                    usage,
                    VertexDataIndex { key: COLOR_KEY, offset: s },
                );
                s += 4;
            }
        }
        if fmt1 & 0x80 != 0 {
            vals.insert(
                VertexUsage::PSize,
                VertexDataIndex { key: VECTOR3_KEY, offset: s },
            );
            s += 12;
        }
        if b1 {
            let mut p = 8;
            for _ in (0..(((s + 15) & 0xFFFF0) - s)).step_by(4) {
                vals.insert(
                    VertexUsage::Pad(p),
                    VertexDataIndex { key: COLOR_KEY, offset: s },
                );
                s += 4;
                p += 1;
            }
        }
    } else {
        if fmt1 & 1 != 0 {
            vals.insert(
                VertexUsage::Position,
                VertexDataIndex { key: VECTOR3_KEY, offset: s },
            );
            s += 12;
        }
        if fmt1 & 0x400 != 0 {
            vals.insert(
                VertexUsage::BlendWeight,
                VertexDataIndex { key: COLOR_KEY, offset: s },
            );
            s += 4;
        }
        if fmt1 & 0x800 != 0 {
            vals.insert(
                VertexUsage::BlendIndices,
                VertexDataIndex { key: COLOR_KEY, offset: s },
            );
            s += 4;
        }
        if fmt1 & 2 != 0 {
            vals.insert(
                VertexUsage::Normal,
                VertexDataIndex { key: COLOR_KEY, offset: s },
            );
            s += 4;
        }
        if fmt1 & 0x100 != 0 {
            vals.insert(
                VertexUsage::Color(0),
                VertexDataIndex { key: COLOR_KEY, offset: s },
            );
            s += 4;
        }
        if fmt1 & 0x200 != 0 {
            vals.insert(
                VertexUsage::Color(1),
                VertexDataIndex { key: COLOR_KEY, offset: s },
            );
            s += 4;
        }
        let n = (fmt1 >> 2) & 0xf;
        if n <= 2 {
            for i in 0..n {
                vals.insert(
                    VertexUsage::TextureCoord(i as usize),
                    VertexDataIndex { key: VECTOR2_KEY, offset: s },
                );
                s += 8;
            }
        }
        if fmt1 & 0x40 != 0 {
            vals.insert(
                VertexUsage::Tangent,
                VertexDataIndex { key: COLOR_KEY, offset: s },
            );
            s += 4;
        }
        if fmt1 & 0x80 != 0 {
            vals.insert(
                VertexUsage::PSize,
                VertexDataIndex { key: VECTOR3_KEY, offset: s },
            );
            s += 12;
        }
    }
    (s, vals)
}


#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct VertexBufferRef<'a, T: BaseTypes> {
    pub info: &'a VBuffInfo<T>,
    pub offsets: IndexMap<VertexUsage, VertexDataIndex>,
    pub size: usize,
    data: ref_slice<'a, u8>
}

impl<'a, T: BaseTypes> VertexBufferRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a VBuffInfo<T>) -> Result<Self> {
        let (size, offsets) = parse_fmt(info.fmt1.into(), info.fmt2.into());
        Ok(Self {
            info,
            size,
            offsets,
            data: (&src[info.offset.into() as usize..(info.offset.into() + info.size.into()) as usize]).into(),
        })
    }
    pub fn iter(&self) -> VertexDataIter<'_> {
        VertexDataIter {
            data: &self.data[..],
            size: self.size,
            offsets: &self.offsets
        }
    }
}

#[derive(Debug, Clone)]
pub struct VertexBuffer {
    pub info: VBuffInfo<NE>,
    pub vals: IndexMap<VertexUsage, VertexData>,
}

impl VertexBuffer {
    pub fn len(&self) -> usize {
        self.vals.first().map(|(_, x)| x.len()).unwrap_or(0)
    }

    pub fn v_size(&self) -> usize {
        self.vals.iter().map(|(_, x)| x.size()).sum::<usize>()
    }

    pub fn dump<T: BaseTypes>(&self) -> Vec<u8>
    where
        Vector2<T>: From<Vector2<NE>>,
        Vector3<T>: From<Vector3<NE>>,
        Vector4<T>: From<Vector4<NE>>,
    {
        let i = self.vals.iter().map(|(_, x)| x.len()).min().unwrap();
        let sorted_vals: Vec<Vec<_>> = self
            .vals
            .iter()
            .sorted_by_key(|(x, _)| x.order())
            .map(|(_, x)| (0..x.len()).map(|i| x.get::<T>(i)).collect())
            .collect();
        (0..i)
            .flat_map(|i| sorted_vals.iter().flat_map(move |x| x[i].as_bytes()))
            .cloned()
            .collect()
    }
}

impl<T: BaseTypes> From<&VertexBufferRef<'_, T>> for VertexBuffer
where
    Vector2<NE>: From<Vector2<T>>,
    Vector3<NE>: From<Vector3<T>>,
    Vector4<NE>: From<Vector4<T>>,
    VBuffInfo<NE>: From<VBuffInfo<T>>,
{
    fn from(val: &VertexBufferRef<T>) -> Self {
        let mut vals = IndexMap::with_capacity(val.offsets.len());
        let val_iter = val.iter();
        let size = val_iter.len();
        for (k, v) in val.offsets.iter() {
            vals.insert(k.clone(), match v.key {
                INT_KEY => VertexData::Int(Vec::with_capacity(size)),
                COLOR_KEY => VertexData::Color(Vec::with_capacity(size)),
                VECTOR2_KEY => VertexData::Vector2(Vec::with_capacity(size)),
                VECTOR3_KEY => VertexData::Vector3(Vec::with_capacity(size)),
                VECTOR4_KEY => VertexData::Vector4(Vec::with_capacity(size)),
                k => panic!("Unhandled vertex data type {}", k)
            });
        }
        for data in val_iter {
            for (off, val) in val.offsets.values().zip(vals.values_mut()) {
                match val {
                    VertexData::Int(val) => val.push(data.get_int::<T>(off).unwrap().clone().into()),
                    VertexData::Color(val) => val.push(data.get_color::<T>(off).unwrap().clone().into()),
                    VertexData::Vector2(val) => val.push(data.get_vec2::<T>(off).unwrap().clone().into()),
                    VertexData::Vector3(val) => val.push(data.get_vec3::<T>(off).unwrap().clone().into()),
                    VertexData::Vector4(val) => val.push(data.get_vec4::<T>(off).unwrap().clone().into()),
                }
            }
        }
        Self {
            info: (*val.info).into(),
            vals,
        }
    }
}

#[cfg_attr(feature = "ffi", repr(C, u8))]
#[derive(PartialEq)]
pub enum IndexBufferValsRef<'a, T: BaseTypes> {
    U16(ref_slice<'a, T::u16>),
    U32(ref_slice<'a, T::u32>),
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct IndexBufferRef<'a, T: BaseTypes> {
    pub info: &'a IBuffInfo<T>,
    pub vals: IndexBufferValsRef<'a, T>
}

impl<'a, T: BaseTypes> IndexBufferRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a IBuffInfo<T>) -> Result<Self> {
        Ok(Self {
            info,
            vals: match info.format.into() {
                0x10 => IndexBufferValsRef::U16(T::u16::slice_from_data(&src[info.offset.into() as usize..], info.size.into() as usize/2).context("vals u16")?.into()),
                _ => IndexBufferValsRef::U32(T::u32::slice_from_data(&src[info.offset.into() as usize..], info.size.into() as usize/4).context("vals u32")?.into()),
            }
        })
    }
}

#[derive(Debug, Clone)]
pub enum IndexBuffer {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl<T: BaseTypes> From<&IndexBufferRef<'_, T>> for IndexBuffer {
    fn from(val: &IndexBufferRef<T>) -> Self {
        match &val.vals {
            IndexBufferValsRef::U16(vals) => Self::U16(vals.iter().map(|&x| x.into()).collect()),
            IndexBufferValsRef::U32(vals) => Self::U32(vals.iter().map(|&x| x.into()).collect()),
        }
    }
}
impl IndexBuffer {
    pub fn dump<T: BaseTypes>(&self) -> Vec<u8> {
        match self {
            Self::U16(vals) => {
                let vals: Vec<T::u16> = vals.iter().map(|&x| x.into()).collect();
                bytemuck::cast_slice(&vals[..]).to_vec()
            }
            Self::U32(vals) => {
                let vals: Vec<T::u32> = vals.iter().map(|&x| x.into()).collect();
                bytemuck::cast_slice(&vals[..]).to_vec()
            }
        }
    }
}

impl IndexBuffer {
    pub fn len(&self) -> usize {
        match self {
            Self::U16(vals) => vals.len(),
            Self::U32(vals) => vals.len(),
        }
    }
}

pub struct VertexDataIter<'a> {
    data: &'a [u8],
    size: usize,
    offsets: &'a IndexMap<VertexUsage, VertexDataIndex>,
}

impl VertexDataIter<'_> {
    pub fn offsets(&self) -> &IndexMap<VertexUsage, VertexDataIndex> {
        self.offsets
    }
    pub unsafe fn iter(&self) -> VertexDataIter<'_> {
        VertexDataIter {
            data: self.data.as_ref(),
            size: self.size,
            offsets: &self.offsets
        }
    }
}

impl<'a> Iterator for VertexDataIter<'a> {
    type Item = VertexValRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.size > self.data.len() {
            None
        } else {
            let (val, data) = unsafe { self.data.split_at_unchecked(self.size) };
            self.data = data;
            Some(VertexValRef{ data: val})
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.len();
        (size, Some(size))
    }
}

impl ExactSizeIterator for VertexDataIter<'_> {
    fn len(&self) -> usize {
        self.data.len() / self.size
    }
}

/*
enum VertexDataIter_XE_<'a> {
    Int(&'a [T::u32]),
    Color(&'a [Color_XE_]),
    Vector2(&'a [Vector2_XE_]),
    Vector3(&'a [Vector3_XE_]),
    Vector4(&'a [Vector4_XE_]),
}
*/

// gotten from vertex data
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct VertexDataIndex {
    pub key: u32,
    pub offset: usize,
}

pub struct VertexValRef<'a> {
    pub data: &'a [u8]
}

impl<'a> VertexValRef<'a> {
    pub fn get_int<T: BaseTypes>(&'a self, ind: &VertexDataIndex) -> Result<&'a T::u32> {
        T::u32::from_data(&self.data[ind.offset..])
    }
    pub fn get_color<T: BaseTypes>(&'a self, ind: &VertexDataIndex) -> Result<&'a T::u32> {
        T::u32::from_data(&self.data[ind.offset..])
    }
    pub fn get_vec2<T: BaseTypes>(&'a self, ind: &VertexDataIndex) -> Result<&'a Vector2<T>> {
        Vector2::from_data(&self.data[ind.offset..])
    }
    pub fn get_vec3<T: BaseTypes>(&'a self, ind: &VertexDataIndex) -> Result<&'a Vector3<T>> {
        Vector3::from_data(&self.data[ind.offset..])
    }
    pub fn get_vec4<T: BaseTypes>(&'a self, ind: &VertexDataIndex) -> Result<&'a Vector4<T>> {
        Vector4::from_data(&self.data[ind.offset..])
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ModelDataRef<'a, T: BaseTypes> {
    pub infos: ref_slice<'a, BufferInfo<T>>,
    pub vbuff_order: ref_slice<'a, T::u32>,
    pub ibuff_order: ref_slice<'a, T::u32>,
    pub vertex: IndexMap<u32, VertexBufferRef<'a, T>>,
    pub index: IndexMap<u32, IndexBufferRef<'a, T>>,
    pub data: Option<&'a CompressedDataRef<'a>>
}

impl<T: BaseTypes> Default for ModelDataRef<'_, T> {
    fn default() -> Self {
        Self {
            infos: ref_slice::default(),
            vbuff_order: ref_slice::default(),
            ibuff_order: ref_slice::default(),
            vertex: IndexMap::default().into(),
            index: IndexMap::default().into(),
            data: None
        }
    }
}

impl<'a, T: BaseTypes> ModelDataRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a ModelInfo<T>, model_data: &IndexMap<u32, &'a CompressedDataRef<'a>>) -> Result<Self> {
        if let Some(model_data) = model_data.get(&info.asset_key.val.into()) {
            let data = model_data.get();
            let infos = BufferInfo::slice_from_data(&src[info.buffer_info_offset.into() as usize..], info.mat_num.into() as usize).context("buffer infos")?;
            let vbuff_order = T::u32::slice_from_data(&src[info.vbuff_offset.into() as usize..], info.vbuff_num.into() as usize).context("vbuff order")?;
            let ibuff_order = T::u32::slice_from_data(&src[info.ibuff_offset.into() as usize..], info.ibuff_num.into() as usize).context("vbuff order")?;
            let vbuffs = vbuff_order.iter().map(|&x| Ok((x.into(), VBuffInfo::from_data(&src[x.into() as usize..]).with_context(|| format!("vbuff info {}", x.into()))?))).collect::<Result<IndexMap<_, _>>>()?;
            let ibuffs = ibuff_order.iter().map(|&x| Ok((x.into(), IBuffInfo::from_data(&src[x.into() as usize..]).with_context(|| format!("ibuff info {}", x.into()))?))).collect::<Result<IndexMap<_, _>>>()?;
            let vertex = vbuffs.iter().map(|(k,info)| Ok((*k, VertexBufferRef::from_data(data, *info).with_context(|| format!("vertex data {}", k))?))).collect::<Result<IndexMap<_,_>>>()?;
            let index = ibuffs.iter().map(|(k,info)| Ok((*k, IndexBufferRef::from_data(data, *info).with_context(|| format!("index data {}", k))?))).collect::<Result<IndexMap<_,_>>>()?;
            Ok(Self {
                infos,
                vbuff_order,
                ibuff_order,
                vertex,
                index,
                data: Some(model_data)
            })
        } else if info.mat_num.into() == 0 && info.vbuff_num.into() == 0 && info.ibuff_num.into() == 0 {
            Ok(Self::default())
        } else {
            Err(anyhow!("missing mesh data {}", info.asset_key.val.into()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelData {
    pub infos: Vec<BufferInfo<NE>>,
    pub vertex: Vec<VertexBuffer>,
    pub index: Vec<IndexBuffer>,
}

impl<T: BaseTypes> From<&ModelDataRef<'_, T>> for ModelData
where
    BufferInfo<NE>: From<BufferInfo<T>>,
    Vector2<NE>: From<Vector2<T>>,
    Vector3<NE>: From<Vector3<T>>,
    Vector4<NE>: From<Vector4<T>>,
    VBuffInfo<NE>: From<VBuffInfo<T>>,
{
    fn from(val: &ModelDataRef<T>) -> Self {
        Self {
            infos: val.infos.iter().map(|&x| x.into()).collect(),
            vertex: val
                .vertex
                .iter()
                .map(|(_, x)| x.into() )
                .collect(),
            index: val
                .index
                .iter()
                .map(|(_, x)| x.into())
                .collect(),
        }
    }
}

pub trait DumpVertexBuffer<T: BaseTypes> {
    fn write_info(&self, info: &mut VBuffInfo<T>) -> Result<()>;
}

pub trait DumpIndexBuffer<T: BaseTypes> {
    fn write_info(&self, info: &mut IBuffInfo<T>) -> Result<()>;
}

pub trait DumpBuffer<T: BaseTypes> {
    fn write_info(&self, info: &mut BufferInfo<T>) -> Result<()>;
    fn has_vbuff_2(&self) -> bool;
    fn has_vbuff_3(&self) -> bool;
}

pub trait DumpModelData<T: BaseTypes> {
    fn vbuffs(&self) -> &IndexMap<u32, impl DumpVertexBuffer<T>>;
    fn ibuffs(&self) -> &IndexMap<u32, impl DumpIndexBuffer<T>>;
    fn buffers(&self) -> impl Iterator<Item=&impl DumpBuffer<T>>;
    fn num_buffers(&self) -> usize;
    fn data(&self) -> CompressedData<'_>;
    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> (usize, usize) {
        let vbuff_num = self.vbuffs().len();
        let ibuff_num = self.ibuffs().len();
        counts.vbuffs += vbuff_num;
        counts.ibuffs += ibuff_num;

        for buffer in self.buffers() {
            counts.buffers += 1;
            counts.offsets += 2;
            if buffer.has_vbuff_2() {
                counts.offsets += 1;
            }
            if buffer.has_vbuff_3() {
                counts.offsets += 1;
            }
        }

        counts.offsets += vbuff_num + ibuff_num;
        offset += vbuff_num * 4;
        (offset + (ibuff_num * 4), offset)
    }

    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>, info: &mut ModelInfo<T>) -> Result<CompressedData<'_>> {
        let mut buffer_offset = infos.buffers.offset;
        let buffers = infos.buffers.next_slice(self.num_buffers());
        let vbuffs = self.vbuffs();
        let vbuff_offset = infos.vbuffs.offset;
        let vbuff_infos = infos.vbuffs.next_slice(vbuffs.len());
        let ibuffs = self.ibuffs();
        let ibuff_offset = infos.ibuffs.offset;
        let ibuff_infos = infos.ibuffs.next_slice(ibuffs.len());

        let vbuff_map = vbuffs.keys().enumerate().map(|(i, x)| (x, (vbuff_offset + (i * std::mem::size_of::<VBuffInfo<T>>())) as u32)).collect::<IndexMap<_,_>>();
        let ibuff_map = ibuffs.keys().enumerate().map(|(i, x)| (x, (ibuff_offset + (i * std::mem::size_of::<IBuffInfo<T>>())) as u32)).collect::<IndexMap<_,_>>();

        for (info, vbuff) in vbuff_infos.iter_mut().zip(vbuffs.values()) {
            vbuff.write_info(info).context("write vbuff info")?;
        }

        for (info, ibuff) in ibuff_infos.iter_mut().zip(ibuffs.values()) {
            ibuff.write_info(info).context("write ibuff info")?;
        }

        // mesh data should be dumped in the order of first encountering the vbuff / ibuff
        info.buffer_info_offset = (buffer_offset as u32).into();
        for (info, buffer) in buffers.iter_mut().zip(self.buffers()) {
            buffer.write_info(info).context("write buffer")?;
            // also need to set v_size, buff size, tri num, etc.
            info.vbuff_info_offset = vbuff_map.get(&info.vbuff_info_offset.into()).copied().unwrap_or(0).into();
            info.vbuff_info_offset_2 = vbuff_map.get(&info.vbuff_info_offset_2.into()).copied().unwrap_or(0).into();
            info.vbuff_info_offset_3 = vbuff_map.get(&info.vbuff_info_offset_3.into()).copied().unwrap_or(0).into();
            info.ibuff_info_offset = ibuff_map.get(&info.ibuff_info_offset.into()).copied().unwrap_or(0).into();
            *infos.offsets.next().context("offsets")? = ((buffer_offset + std::mem::offset_of!(BufferInfo<T>, vbuff_info_offset)) as u32).into();
            if info.vbuff_info_offset_2.into() != 0 {
                *infos.offsets.next().context("offsets")? = ((buffer_offset + std::mem::offset_of!(BufferInfo<T>, vbuff_info_offset_2)) as u32).into();
            }
            if info.vbuff_info_offset_3.into() != 0 {
                *infos.offsets.next().context("offsets")? = ((buffer_offset + std::mem::offset_of!(BufferInfo<T>, vbuff_info_offset_3)) as u32).into();
            }
            *infos.offsets.next().context("offsets")? = ((buffer_offset + std::mem::offset_of!(BufferInfo<T>, ibuff_info_offset)) as u32).into();
            buffer_offset += std::mem::size_of::<BufferInfo<T>>();
        }

        info.vbuff_offset = (dst.offset as u32).into();
        info.vbuff_num = (vbuff_map.len() as u32).into();
        let mut off = dst.offset;
        let vbuff_order = T::u32::mut_slice_from_data(dst, vbuffs.len()).context("vbuff_order")?;
        for (dst, &src) in vbuff_order.iter_mut().zip(vbuff_map.values()) {
            *dst = src.into();
            *infos.offsets.next().context("offsets")? = (off as u32).into();
            off += 4;
        }

        info.ibuff_offset = (dst.offset as u32).into();
        info.ibuff_num = (ibuff_map.len() as u32).into();
        let mut off = dst.offset;
        let ibuff_order = T::u32::mut_slice_from_data(dst, ibuffs.len()).context("ibuff_order")?;
        for (dst, &src) in ibuff_order.iter_mut().zip(ibuff_map.values()) {
            *dst = src.into();
            *infos.offsets.next().context("offsets")? = (off as u32).into();
            off += 4;
        }

        Ok(self.data())
    }
}

impl<T: BaseTypes> DumpVertexBuffer<T> for VertexBufferRef<'_, T> {
    fn write_info(&self, info: &mut VBuffInfo<T>) -> Result<()> {
        *info = *self.info;
        Ok(())
    }
}

impl<T: BaseTypes> DumpIndexBuffer<T> for IndexBufferRef<'_, T> {
    fn write_info(&self, info: &mut IBuffInfo<T>) -> Result<()> {
        *info = *self.info;
        Ok(())
    }
}

impl<T: BaseTypes> DumpBuffer<T> for BufferInfo<T> {
    fn write_info(&self, info: &mut BufferInfo<T>) -> Result<()> {
        *info = *self;
        Ok(())
    }
    fn has_vbuff_2(&self) -> bool {
        self.vbuff_info_offset_2.into() != 0
    }
    fn has_vbuff_3(&self) -> bool {
        self.vbuff_info_offset_3.into() != 0
    }
}

impl<'a, T: BaseTypes> DumpModelData<T> for ModelDataRef<'a, T> {
    fn vbuffs(&self) -> &IndexMap<u32, impl DumpVertexBuffer<T>> {
        &self.vertex
    }
    fn ibuffs(&self) -> &IndexMap<u32, impl DumpIndexBuffer<T>> {
        &self.index
    }
    fn buffers(&self) -> impl Iterator<Item=&impl DumpBuffer<T>> {
        self.infos.iter()
    }
    fn num_buffers(&self) -> usize {
        self.infos.len()
    }
    fn data(&self) -> CompressedData<'_> {
        self.data.into()
    }
}
