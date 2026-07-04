use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;

use crate::types::GetNative;
use crate::types::{CompressedData, Color, RefFromData, Vector2, Vector3, Vector4, OrderedData, OrderedDataStrict, DumpSlice, CompressedDataRef, DumpData, ref_slice, u8Pc, u8Xbox};
use crate::level::pak::block1::gameobjs::keys::{INT_KEY, COLOR_KEY, VECTOR2_KEY, VECTOR3_KEY, VECTOR4_KEY};
use crate::level::pak::block1::infos::InfoCounts;

use lotrc_proc::{make_platforms, OrderedData};
#[make_platforms]
use crate::{
    level::{
        pak::block1::infos::DumpInfosVER,
        model::{ModelInfoVER}
    },
    types::{ColorVER, Vector2VER, Vector3VER, Vector4VER, u16VER, u32VER},
};


///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct BufferInfo {
    pub vbuff_info_offset: u32,   // pointer to objf
    pub vbuff_info_offset_2: u32, // optional pointer to objf
    pub vbuff_info_offset_3: u32, // optional pointer to objf
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
    #[ordered_data(name_ps3=ibuff_info_offset)] // not the correct place, but needed to get code running
    pub unk_17: u32,
    #[ordered_data(name_ps3=v_size)] // not the correct place, but needed to get code running
    pub unk_18: u32,
    #[ordered_data(name_ps3=vbuff_size)] // not the correct place, but needed to get code running
    pub unk_19: u32,
    #[ordered_data(skip_ps3)]
    pub unk_20: u32,
    #[ordered_data(skip_ps3)]
    pub unk_21: u32,
    #[ordered_data(skip_ps3)]
    pub unk_22: u32,
    #[ordered_data(skip_ps3)]
    pub unk_23: u32,
    #[ordered_data(skip_ps3)]
    pub unk_24: u32,
    #[ordered_data(skip_ps3)]
    pub unk_25: u32,
    #[ordered_data(skip_ps3)]
    pub unk_26: u32,
    #[ordered_data(skip_ps3)]
    pub unk_27: u32,
    #[ordered_data(skip_ps3)]
    pub unk_28: u32,
    #[ordered_data(skip_ps3)]
    pub unk_29: u32,
    #[ordered_data(skip_ps3)]
    pub unk_30: u32,
    #[ordered_data(skip_ps3)]
    pub unk_31: u32,
    #[ordered_data(skip_ps3)]
    pub v_size: u32,
    #[ordered_data(skip_ps3)]
    pub v_size_2: u32,
    #[ordered_data(skip_ps3)]
    pub v_size_3: u32,
    #[ordered_data(skip_ps3)]
    pub unk_35: u32,
    #[ordered_data(skip_ps3)]
    pub unk_36: u32,
    #[ordered_data(skip_ps3)]
    pub unk_37: u32,
    #[ordered_data(skip_ps3)]
    pub unk_38: u32,
    #[ordered_data(skip_ps3)]
    pub unk_39: u32,
    #[ordered_data(skip_ps3)]
    pub unk_40: u32,
    #[ordered_data(skip_ps3)]
    pub unk_41: u32,
    #[ordered_data(skip_ps3)]
    pub unk_42: u32,
    #[ordered_data(skip_ps3)]
    pub unk_43: u32,
    #[ordered_data(skip_ps3)]
    pub unk_44: u32,
    #[ordered_data(skip_ps3)]
    pub unk_45: u32,
    #[ordered_data(skip_ps3)]
    pub unk_46: u32,
    #[ordered_data(skip_ps3)]
    pub unk_47: u32,
    #[ordered_data(skip_ps3)]
    pub vbuff_size: u32,
    #[ordered_data(skip_ps3)]
    pub vbuff_size_2: u32,
    #[ordered_data(skip_ps3)]
    pub vbuff_size_3: u32,
    #[ordered_data(skip_ps3)]
    pub unk_51: u32,
    #[ordered_data(skip_ps3)]
    pub unk_52: u32,
    #[ordered_data(skip_ps3)]
    pub unk_53: u32,
    #[ordered_data(skip_ps3)]
    pub unk_54: u32,
    #[ordered_data(skip_ps3)]
    pub unk_55: u32,
    #[ordered_data(skip_ps3)]
    pub unk_56: u32,
    #[ordered_data(skip_ps3)]
    pub unk_57: u32,
    #[ordered_data(skip_ps3)]
    pub unk_58: u32,
    #[ordered_data(skip_ps3)]
    pub unk_59: u32,
    #[ordered_data(skip_ps3)]
    pub unk_60: u32,
    #[ordered_data(skip_ps3)]
    pub unk_61: u32,
    #[ordered_data(skip_ps3)]
    pub unk_62: u32,
    #[ordered_data(skip_ps3)]
    pub unk_63: u32,
    #[ordered_data(skip_ps3)]
    pub unk_64: u32,
    #[ordered_data(skip_ps3)]
    pub ibuff_info_offset: u32, // poiner to objg
    #[ordered_data(skip_ps3)]
    pub i_num: u32, // number of indeices in ibuffer
    #[ordered_data(skip_ps3)]
    pub unk_67: u32,
    #[ordered_data(skip_ps3)]
    pub skin_offset: u32,
    #[ordered_data(skip_ps3)]
    pub skin_size: u32,
    #[ordered_data(skip_ps3)]
    pub unk_70: u32,
    #[ordered_data(skip_ps3)]
    pub tri_num: u32, // number of objects(triangles) in ibuffer
    #[ordered_data(skip_ps3)]
    pub unk_72: u32, // possibly index to bone_transform used for mesh
    #[ordered_data(skip_ps3)]
    pub unk_73: u32,
    #[ordered_data(skip_ps3)]
    pub unk_74: u32,
    #[ordered_data(skip_ps3)]
    pub unk_75: u32,
    #[ordered_data(skip_ps3)]
    pub unk_76: u32,
    #[ordered_data(skip_ps3)]
    pub unk_77: u32,
    #[ordered_data(skip_ps3)]
    pub unk_78: u32,
    #[ordered_data(skip_ps3)]
    pub unk_79: u32,
    #[ordered_data(skip_ps3)]
    pub unk_80: u32,
    #[ordered_data(skip_ps3)]
    pub unk_81: u32,
    #[ordered_data(skip_ps3)]
    pub unk_82: u32,
    #[ordered_data(skip_ps3)]
    pub unk_83: u32,
    #[ordered_data(skip_ps3)]
    pub unk_84: u32,
    #[ordered_data(skip_ps3)]
    pub unk_85: u32,
    #[ordered_data(skip_ps3)]
    pub unk_86: u32,
    #[ordered_data(skip_ps3)]
    pub unk_87: u32,
    #[ordered_data(skip_ps3)]
    pub variation_id: u8,
    #[ordered_data(skip_ps3)]
    pub variation: u8,
    #[ordered_data(skip_ps3)]
    pub unk_88c: u8,
    #[ordered_data(skip_ps3)]
    pub unk_88d: u8,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct VBuffInfo {
    pub unk_0: u32,
    #[ordered_data(name_ps3=unk_7)]
    pub size: u32,
    pub unk_3: u32,
    #[ordered_data(name_ps3=unk_8)]
    pub offset: u32,
    #[ordered_data(name_xbox=fmt2, name_ps3=unk_9)]
    pub fmt1: u32,
    #[ordered_data(name_xbox=fmt1, name_ps3=size)]
    pub fmt2: u32,
    pub unk_6: u32,
    #[ordered_data(name_ps3=offset)]
    pub unk_7: u32,
    #[ordered_data(skip_pc, name_ps3=fmt2)]
    pub unk_8: u32,
    #[ordered_data(skip_pc, name_ps3=fmt1)]
    pub unk_9: u32,
    #[ordered_data(skip_pc, skip_ps3)]
    pub unk_10: u32,
    #[ordered_data(skip_pc, skip_ps3)]
    pub unk_11: u32,
    #[ordered_data(skip_pc, skip_ps3)]
    pub unk_12: u32,
    #[ordered_data(skip_pc, skip_ps3)]
    pub unk_13: u32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct IBuffInfo {
    pub unk_0: u32,
    #[ordered_data(name_ps3=unk_5)]
    pub size: u32,
    #[ordered_data(name_ps3=unk_6)]
    pub format: u32,
    pub vbuff_alt_fmt: u32, // 1 if vbuff.fmt1 & 0x40000 != 0 else 0; 0 for Xbox
    #[ordered_data(name_ps3=unk_8)]
    pub offset: u32,
    #[ordered_data(name_ps3=size)]
    pub unk_5: u32,
    #[ordered_data(skip_pc, name_ps3=format)]
    pub unk_6: u32,
    #[ordered_data(skip_pc)]
    pub unk_7: u32,
    #[ordered_data(skip_pc, name_ps3=offset)]
    pub unk_8: u32,
    #[ordered_data(skip_pc, skip_ps3)]
    pub unk_9: u32,
    #[ordered_data(skip_pc, skip_ps3)]
    pub unk_10: u32,
    #[ordered_data(skip_pc, skip_ps3)]
    pub unk_11: u32,
    #[ordered_data(skip_pc, skip_ps3)]
    pub unk_12: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "ffi", repr(C, u8))]
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


#[make_platforms]
pub enum VertexValVER {
    Int(u32VER),
    Color(ColorVER),
    Vector2(Vector2VER),
    Vector3(Vector3VER),
    Vector4(Vector4VER),
}

/*
#[make_platforms]
impl VertexValVER {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Int(val) => RefFromDataArgs::as_bytes(val),
            Self::Color(val) => RefFromDataArgs::as_bytes(val),
            Self::Vector2(val) => RefFromDataArgs::as_bytes(val),
            Self::Vector3(val) => RefFromDataArgs::as_bytes(val),
            Self::Vector4(val) => RefFromDataArgs::as_bytes(val),
        }
    }
}
*/

#[derive(Debug, Clone)]
pub enum VertexData {
    Int(Vec<u32>),
    Color(Vec<Color>),
    Vector2(Vec<Vector2>),
    Vector3(Vec<Vector3>),
    Vector4(Vec<Vector4>),
}

impl VertexData {
    #[make_platforms]
    pub fn get_ver(&self, i: usize) -> VertexValVER {
        match self {
            Self::Int(vals) => VertexValVER::Int(vals[i].clone().into()),
            Self::Color(vals) => VertexValVER::Color(vals[i].clone().into()),
            Self::Vector2(vals) => VertexValVER::Vector2(vals[i].clone().into()),
            Self::Vector3(vals) => VertexValVER::Vector3(vals[i].clone().into()),
            Self::Vector4(vals) => VertexValVER::Vector4(vals[i].clone().into()),
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

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct VertexBufferRefVER<'a> {
    pub info: &'a VBuffInfoVER,
    pub offsets: IndexMap<VertexUsage, VertexDataIndex>,
    pub size: usize,
    data: ref_slice<'a, u8>
}

#[make_platforms]
impl<'a> VertexBufferRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a VBuffInfoVER) -> Result<Self> {
        let (size, offsets) = parse_fmt(info.fmt1.get(), info.fmt2.get());
        Ok(Self {
            info: info.into(),
            size,
            offsets: offsets.into(),
            data: (&src[info.offset.get() as usize..(info.offset.get() + info.size.get()) as usize]).into(),
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
    pub info: VBuffInfo,
    pub vals: IndexMap<VertexUsage, VertexData>,
}

impl VertexBuffer {
    pub fn len(&self) -> usize {
        self.vals.first().map(|(_, x)| x.len()).unwrap_or(0)
    }

    pub fn v_size(&self) -> usize {
        self.vals.iter().map(|(_, x)| x.size()).sum::<usize>()
    }

    /*
    #[make_platforms]
    pub fn dump_ver(&self) -> Vec<u8> {
        let i = self.vals.iter().map(|(_, x)| x.len()).min().unwrap();
        let sorted_vals: Vec<Vec<_>> = self
            .vals
            .iter()
            .sorted_by_key(|(x, _)| x.order())
            .map(|(_, x)| (0..x.len()).map(|i| x.get_ver(i)).collect())
            .collect();
        (0..i)
            .flat_map(|i| sorted_vals.iter().flat_map(move |x| x[i].as_bytes()))
            .cloned()
            .collect()
    }
    */
}

#[make_platforms]
impl From<&VertexBufferRefVER<'_>> for VertexBuffer {
    fn from(val: &VertexBufferRefVER) -> Self {
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
                    VertexData::Int(val) => val.push(data.get_int_ver(off).unwrap().conv()),
                    VertexData::Color(val) => val.push(data.get_color_ver(off).unwrap().conv()),
                    VertexData::Vector2(val) => val.push(data.get_vec2_ver(off).unwrap().conv()),
                    VertexData::Vector3(val) => val.push(data.get_vec3_ver(off).unwrap().conv()),
                    VertexData::Vector4(val) => val.push(data.get_vec4_ver(off).unwrap().conv()),
                }
            }
        }
        Self {
            info: val.info.conv(),
            vals,
        }
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C, u8))]
#[derive(PartialEq)]
pub enum IndexBufferValsRefVER<'a> {
    U16(ref_slice<'a, u16VER>),
    U32(ref_slice<'a, u32VER>),
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct IndexBufferRefVER<'a> {
    pub info: &'a IBuffInfoVER,
    pub vals: IndexBufferValsRefVER<'a>
}

#[make_platforms]
impl<'a> IndexBufferRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a IBuffInfoVER) -> Result<Self> {
        Ok(Self {
            info,
            vals: match info.format.get() {
                0x10 => IndexBufferValsRefVER::U16(u16VER::slice_from_data(&src[info.offset.get() as usize..], info.size.get() as usize/2).context("vals u16")?.into()),
                _ => IndexBufferValsRefVER::U32(u32VER::slice_from_data(&src[info.offset.get() as usize..], info.size.get() as usize/4).context("vals u32")?.into()),
            }
        })
    }
}

#[derive(Debug, Clone)]
pub enum IndexBuffer {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

#[make_platforms]
impl From<&IndexBufferRefVER<'_>> for IndexBuffer {
    fn from(val: &IndexBufferRefVER) -> Self {
        match &val.vals {
            IndexBufferValsRefVER::U16(vals) => Self::U16(vals.iter().map(|x| x.conv()).collect()),
            IndexBufferValsRefVER::U32(vals) => Self::U32(vals.iter().map(|x| x.conv()).collect()),
        }
    }
}
/*
impl IndexBuffer {
    #[make_platforms]
    pub fn dump_ver(&self) -> Vec<u8> {
        match self {
            Self::U16 { vals } => {
                let vals: Vec<_> = vals.iter().map(|x| u16VER::new(*x)).collect();
                RefFromDataArgs::as_bytes(&vals[..]).to_vec()
            }
            Self::U32 { vals } => {
                let vals: Vec<_> = vals.iter().map(|x| u32VER::new(*x)).collect();
                RefFromDataArgs::as_bytes(&vals[..]).to_vec()
            }
        }
    }
}
*/

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

#[make_platforms]
enum VertexDataIterVER<'a> {
    Int(&'a [u32VER]),
    Color(&'a [ColorVER]),
    Vector2(&'a [Vector2VER]),
    Vector3(&'a [Vector3VER]),
    Vector4(&'a [Vector4VER]),
}

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
    #[make_platforms]
    pub fn get_int_ver(&'a self, ind: &VertexDataIndex) -> Result<&'a u32VER> {
        u32VER::from_data(&self.data[ind.offset..])
    }
    #[make_platforms]
    pub fn get_color_ver(&'a self, ind: &VertexDataIndex) -> Result<&'a ColorVER> {
        ColorVER::from_data(&self.data[ind.offset..])
    }
    #[make_platforms]
    pub fn get_vec2_ver(&'a self, ind: &VertexDataIndex) -> Result<&'a Vector2VER> {
        Vector2VER::from_data(&self.data[ind.offset..])
    }
    #[make_platforms]
    pub fn get_vec3_ver(&'a self, ind: &VertexDataIndex) -> Result<&'a Vector3VER> {
        Vector3VER::from_data(&self.data[ind.offset..])
    }
    #[make_platforms]
    pub fn get_vec4_ver(&'a self, ind: &VertexDataIndex) -> Result<&'a Vector4VER> {
        Vector4VER::from_data(&self.data[ind.offset..])
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct ModelDataRefVER<'a> {
    pub infos: ref_slice<'a, BufferInfoVER>,
    pub vbuff_order: ref_slice<'a, u32VER>,
    pub ibuff_order: ref_slice<'a, u32VER>,
    pub vertex: IndexMap<u32, VertexBufferRefVER<'a>>,
    pub index: IndexMap<u32, IndexBufferRefVER<'a>>,
    pub data: Option<&'a CompressedDataRef<'a>>
}

#[make_platforms]
impl Default for ModelDataRefVER<'_> {
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

#[make_platforms]
impl<'a> ModelDataRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a ModelInfoVER, model_data: &IndexMap<u32, &'a CompressedDataRef<'a>>) -> Result<Self> {
        if let Some(model_data) = model_data.get(&info.asset_key.get()) {
            let data = model_data.get();
            let infos = BufferInfoVER::slice_from_data(&src[info.buffer_info_offset.get() as usize..], info.mat_num.get() as usize).context("buffer infos")?;
            let vbuff_order = u32VER::slice_from_data(&src[info.vbuff_offset.get() as usize..], info.vbuff_num.get() as usize).context("vbuff order")?;
            let ibuff_order = u32VER::slice_from_data(&src[info.ibuff_offset.get() as usize..], info.ibuff_num.get() as usize).context("vbuff order")?;
            let vbuffs = vbuff_order.iter().map(|x| Ok((x.get(), VBuffInfoVER::from_data(&src[x.get() as usize..]).with_context(|| format!("vbuff info {}", x.get()))?))).collect::<Result<IndexMap<_, _>>>()?;
            let ibuffs = ibuff_order.iter().map(|x| Ok((x.get(), IBuffInfoVER::from_data(&src[x.get() as usize..]).with_context(|| format!("ibuff info {}", x.get()))?))).collect::<Result<IndexMap<_, _>>>()?;
            let vertex = vbuffs.iter().map(|(k,info)| Ok((*k, VertexBufferRefVER::from_data(data, info).with_context(|| format!("vertex data {}", k))?))).collect::<Result<IndexMap<_,_>>>()?.into();
            let index = ibuffs.iter().map(|(k,info)| Ok((*k, IndexBufferRefVER::from_data(data, info).with_context(|| format!("index data {}", k))?))).collect::<Result<IndexMap<_,_>>>()?.into();
            Ok(Self {
                infos: infos.into(),
                vbuff_order: vbuff_order.into(),
                ibuff_order: ibuff_order.into(),
                vertex,
                index,
                data: Some(model_data) 
            })
        } else if info.mat_num.get() == 0 && info.vbuff_num.get() == 0 && info.ibuff_num.get() == 0 {
            Ok(Self::default())
        } else {
            Err(anyhow!("missing mesh data {}", info.asset_key.get()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelData {
    pub infos: Vec<BufferInfo>,
    pub vertex: Vec<VertexBuffer>,
    pub index: Vec<IndexBuffer>,
}

#[make_platforms]
impl From<&ModelDataRefVER<'_>> for ModelData {
    fn from(val: &ModelDataRefVER) -> Self {
        Self {
            infos: val.infos.iter().map(|x| x.conv()).collect(),
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

#[make_platforms]
pub trait DumpVertexBufferVER {
    fn write_info(&self, info: &mut VBuffInfoVER) -> Result<()>;
}

#[make_platforms]
pub trait DumpIndexBufferVER {
    fn write_info(&self, info: &mut IBuffInfoVER) -> Result<()>;
}

#[make_platforms]
pub trait DumpBufferVER {
    fn write_info(&self, info: &mut BufferInfoVER) -> Result<()>;
    fn has_vbuff_2(&self) -> bool;
    fn has_vbuff_3(&self) -> bool;
}

#[make_platforms]
pub trait DumpModelDataVER {
    fn vbuffs(&self) -> &IndexMap<u32, impl DumpVertexBufferVER>;
    fn ibuffs(&self) -> &IndexMap<u32, impl DumpIndexBufferVER>;
    fn buffers(&self) -> impl Iterator<Item=&impl DumpBufferVER>;
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
        offset += (vbuff_num * 4);
        (offset + (ibuff_num * 4), offset)
    }

    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER, info: &mut ModelInfoVER) -> Result<CompressedData<'_>> {
        let mut buffer_offset = infos.buffers.offset;
        let buffers = infos.buffers.next_slice(self.num_buffers());
        let vbuffs = self.vbuffs();
        let vbuff_offset = infos.vbuffs.offset;
        let vbuff_infos = infos.vbuffs.next_slice(vbuffs.len());
        let ibuffs = self.ibuffs();
        let ibuff_offset = infos.ibuffs.offset;
        let ibuff_infos = infos.ibuffs.next_slice(ibuffs.len());

        let vbuff_map = vbuffs.keys().enumerate().map(|(i, x)| (x, (vbuff_offset + (i * std::mem::size_of::<VBuffInfoVER>())) as u32)).collect::<IndexMap<_,_>>();
        let ibuff_map = ibuffs.keys().enumerate().map(|(i, x)| (x, (ibuff_offset + (i * std::mem::size_of::<IBuffInfoVER>())) as u32)).collect::<IndexMap<_,_>>();

        for (info, vbuff) in vbuff_infos.iter_mut().zip(vbuffs.values()) {
            vbuff.write_info(info).context("write vbuff info")?;
        }

        for (info, ibuff) in ibuff_infos.iter_mut().zip(ibuffs.values()) {
            ibuff.write_info(info).context("write ibuff info")?;
        }

        // mesh data should be dumped in the order of first encountering the vbuff / ibuff
        info.buffer_info_offset = buffer_offset.conv();
        for (info, buffer) in buffers.iter_mut().zip(self.buffers()) {
            buffer.write_info(info).context("write buffer")?;
            // also need to set v_size, buff size, tri num, etc.
            info.vbuff_info_offset = vbuff_map.get(&info.vbuff_info_offset.get()).copied().unwrap_or(0).conv();
            info.vbuff_info_offset_2 = vbuff_map.get(&info.vbuff_info_offset_2.get()).copied().unwrap_or(0).conv();
            info.vbuff_info_offset_3 = vbuff_map.get(&info.vbuff_info_offset_3.get()).copied().unwrap_or(0).conv();
            info.ibuff_info_offset = ibuff_map.get(&info.ibuff_info_offset.get()).copied().unwrap_or(0).conv();
            *infos.offsets.next().context("offsets")? = (buffer_offset + std::mem::offset_of!(BufferInfoVER, vbuff_info_offset)).conv();
            if info.vbuff_info_offset_2.get() != 0 {
                *infos.offsets.next().context("offsets")? = (buffer_offset + std::mem::offset_of!(BufferInfoVER, vbuff_info_offset_2)).conv();
            }
            if info.vbuff_info_offset_3.get() != 0 {
                *infos.offsets.next().context("offsets")? = (buffer_offset + std::mem::offset_of!(BufferInfoVER, vbuff_info_offset_3)).conv();
            }
            *infos.offsets.next().context("offsets")? = (buffer_offset + std::mem::offset_of!(BufferInfoVER, ibuff_info_offset)).conv();
            buffer_offset += std::mem::size_of::<BufferInfoVER>();
        }

        info.vbuff_offset = dst.offset.conv();
        info.vbuff_num = vbuff_map.len().conv();
        let mut off = dst.offset;
        let vbuff_order = u32VER::mut_slice_from_data(dst, vbuffs.len()).context("vbuff_order")?;
        for (dst, src) in vbuff_order.iter_mut().zip(vbuff_map.values()) {
            *dst = src.conv();
            *infos.offsets.next().context("offsets")? = off.conv();
            off += 4;
        }

        info.ibuff_offset = dst.offset.conv();
        info.ibuff_num = ibuff_map.len().conv();
        let mut off = dst.offset;
        let ibuff_order = u32VER::mut_slice_from_data(dst, ibuffs.len()).context("ibuff_order")?;
        for (dst, src) in ibuff_order.iter_mut().zip(ibuff_map.values()) {
            *dst = src.conv();
            *infos.offsets.next().context("offsets")? = off.conv();
            off += 4;
        }

        Ok(self.data())
    }
}

#[make_platforms]
impl DumpVertexBufferVER for VertexBufferRefVER<'_> {
    fn write_info(&self, info: &mut VBuffInfoVER) -> Result<()> {
        info.write_from(self.info)
    }
}

#[make_platforms]
impl DumpIndexBufferVER for IndexBufferRefVER<'_> {
    fn write_info(&self, info: &mut IBuffInfoVER) -> Result<()> {
        info.write_from(self.info)
    }
}

#[make_platforms]
impl DumpBufferVER for BufferInfoVER {
    fn write_info(&self, info: &mut BufferInfoVER) -> Result<()> {
        info.write_from(self)
    }
    fn has_vbuff_2(&self) -> bool {
        self.vbuff_info_offset_2.get() != 0
    }
    fn has_vbuff_3(&self) -> bool {
        self.vbuff_info_offset_3.get() != 0
    }
}

#[make_platforms]
impl<'a> DumpModelDataVER for ModelDataRefVER<'a> {
    fn vbuffs(&self) -> &IndexMap<u32, impl DumpVertexBufferVER> {
        &self.vertex
    }
    fn ibuffs(&self) -> &IndexMap<u32, impl DumpIndexBufferVER> {
        &self.index
    }
    fn buffers(&self) -> impl Iterator<Item=&impl DumpBufferVER> {
        self.infos.iter()
    }
    fn num_buffers(&self) -> usize {
        self.infos.len()
    }
    fn data(&self) -> CompressedData<'_> {
        self.data.into()
    }
}
