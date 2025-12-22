#[cfg(feature = "python")]
use crate::pyobj_ref;
use anyhow::{Context, Result};
use indexmap::IndexMap;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::ptr::NonNull;
use std::sync::Arc;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::types::{Color, RefFromData, Vector2, Vector3, Vector4};
use crate::sub_blocks::gameobjs::keys::{INT_KEY, COLOR_KEY, VECTOR2_KEY, VECTOR3_KEY, VECTOR4_KEY};

#[make_platforms]
use crate::{
    level::{model::{ModelVER, ModelInfoVER, ModelRawVER}, pak::objs::ObjsVER},
    types::{ColorVER, Vector2VER, Vector3VER, Vector4VER, U16VER, U32VER},
};
use lotrc_proc::{make_platforms, OrderedData};

#[cfg(feature = "python")]
pub fn init(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "data")?;
    m.add_class::<BufferInfo>()?;
    m.add_class::<IBuffInfo>()?;
    m.add_class::<IndexBuffer>()?;
    m.add_class::<VBuffInfo>()?;
    m.add_class::<VertexData>()?;
    Ok(m)
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "model.data", get_all, set_all))]
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
    #[name_ps3(ibuff_info_offset)] // not the correct place, but needed to get code running
    pub unk_17: u32,
    #[name_ps3(v_size)] // not the correct place, but needed to get code running
    pub unk_18: u32,
    #[name_ps3(vbuff_size)] // not the correct place, but needed to get code running
    pub unk_19: u32,
    #[ordered_data(skipPS3)]
    pub unk_20: u32,
    #[ordered_data(skipPS3)]
    pub unk_21: u32,
    #[ordered_data(skipPS3)]
    pub unk_22: u32,
    #[ordered_data(skipPS3)]
    pub unk_23: u32,
    #[ordered_data(skipPS3)]
    pub unk_24: u32,
    #[ordered_data(skipPS3)]
    pub unk_25: u32,
    #[ordered_data(skipPS3)]
    pub unk_26: u32,
    #[ordered_data(skipPS3)]
    pub unk_27: u32,
    #[ordered_data(skipPS3)]
    pub unk_28: u32,
    #[ordered_data(skipPS3)]
    pub unk_29: u32,
    #[ordered_data(skipPS3)]
    pub unk_30: u32,
    #[ordered_data(skipPS3)]
    pub unk_31: u32,
    #[ordered_data(skipPS3)]
    pub v_size: u32,
    #[ordered_data(skipPS3)]
    pub v_size_2: u32,
    #[ordered_data(skipPS3)]
    pub v_size_3: u32,
    #[ordered_data(skipPS3)]
    pub unk_35: u32,
    #[ordered_data(skipPS3)]
    pub unk_36: u32,
    #[ordered_data(skipPS3)]
    pub unk_37: u32,
    #[ordered_data(skipPS3)]
    pub unk_38: u32,
    #[ordered_data(skipPS3)]
    pub unk_39: u32,
    #[ordered_data(skipPS3)]
    pub unk_40: u32,
    #[ordered_data(skipPS3)]
    pub unk_41: u32,
    #[ordered_data(skipPS3)]
    pub unk_42: u32,
    #[ordered_data(skipPS3)]
    pub unk_43: u32,
    #[ordered_data(skipPS3)]
    pub unk_44: u32,
    #[ordered_data(skipPS3)]
    pub unk_45: u32,
    #[ordered_data(skipPS3)]
    pub unk_46: u32,
    #[ordered_data(skipPS3)]
    pub unk_47: u32,
    #[ordered_data(skipPS3)]
    pub vbuff_size: u32,
    #[ordered_data(skipPS3)]
    pub vbuff_size_2: u32,
    #[ordered_data(skipPS3)]
    pub vbuff_size_3: u32,
    #[ordered_data(skipPS3)]
    pub unk_51: u32,
    #[ordered_data(skipPS3)]
    pub unk_52: u32,
    #[ordered_data(skipPS3)]
    pub unk_53: u32,
    #[ordered_data(skipPS3)]
    pub unk_54: u32,
    #[ordered_data(skipPS3)]
    pub unk_55: u32,
    #[ordered_data(skipPS3)]
    pub unk_56: u32,
    #[ordered_data(skipPS3)]
    pub unk_57: u32,
    #[ordered_data(skipPS3)]
    pub unk_58: u32,
    #[ordered_data(skipPS3)]
    pub unk_59: u32,
    #[ordered_data(skipPS3)]
    pub unk_60: u32,
    #[ordered_data(skipPS3)]
    pub unk_61: u32,
    #[ordered_data(skipPS3)]
    pub unk_62: u32,
    #[ordered_data(skipPS3)]
    pub unk_63: u32,
    #[ordered_data(skipPS3)]
    pub unk_64: u32,
    #[ordered_data(skipPS3)]
    pub ibuff_info_offset: u32, // poiner to objg
    #[ordered_data(skipPS3)]
    pub i_num: u32, // number of indeices in ibuffer
    #[ordered_data(skipPS3)]
    pub unk_67: u32,
    #[ordered_data(skipPS3)]
    pub skin_offset: u32,
    #[ordered_data(skipPS3)]
    pub skin_size: u32,
    #[ordered_data(skipPS3)]
    pub unk_70: u32,
    #[ordered_data(skipPS3)]
    pub tri_num: u32, // number of objects(triangles) in ibuffer
    #[ordered_data(skipPS3)]
    pub unk_72: u32, // possibly index to bone_transform used for mesh
    #[ordered_data(skipPS3)]
    pub unk_73: u32,
    #[ordered_data(skipPS3)]
    pub unk_74: u32,
    #[ordered_data(skipPS3)]
    pub unk_75: u32,
    #[ordered_data(skipPS3)]
    pub unk_76: u32,
    #[ordered_data(skipPS3)]
    pub unk_77: u32,
    #[ordered_data(skipPS3)]
    pub unk_78: u32,
    #[ordered_data(skipPS3)]
    pub unk_79: u32,
    #[ordered_data(skipPS3)]
    pub unk_80: u32,
    #[ordered_data(skipPS3)]
    pub unk_81: u32,
    #[ordered_data(skipPS3)]
    pub unk_82: u32,
    #[ordered_data(skipPS3)]
    pub unk_83: u32,
    #[ordered_data(skipPS3)]
    pub unk_84: u32,
    #[ordered_data(skipPS3)]
    pub unk_85: u32,
    #[ordered_data(skipPS3)]
    pub unk_86: u32,
    #[ordered_data(skipPS3)]
    pub unk_87: u32,
    #[ordered_data(skipPS3)]
    pub variation_id: u8,
    #[ordered_data(skipPS3)]
    pub variation: u8,
    #[ordered_data(skipPS3)]
    pub unk_88c: u8,
    #[ordered_data(skipPS3)]
    pub unk_88d: u8,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "model.data", get_all, set_all))]
pub struct VBuffInfo {
    pub unk_0: u32,
    #[name_ps3(unk_7)]
    pub size: u32,
    pub unk_3: u32,
    #[name_ps3(unk_8)]
    pub offset: u32,
    #[name_xbox(fmt2)]
    #[name_ps3(unk_9)]
    pub fmt1: u32,
    #[name_xbox(fmt1)]
    #[name_ps3(size)]
    pub fmt2: u32,
    pub unk_6: u32,
    #[name_ps3(offset)]
    pub unk_7: u32,
    #[name_ps3(fmt2)]
    #[ordered_data(skipPC)]
    pub unk_8: u32,
    #[name_ps3(fmt1)]
    #[ordered_data(skipPC)]
    pub unk_9: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_10: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_11: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_12: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_13: u32,
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "model.data", get_all, set_all))]
pub struct IBuffInfo {
    pub unk_0: u32,
    #[name_ps3(unk_5)]
    pub size: u32,
    #[name_ps3(unk_6)]
    pub format: u32,
    pub vbuff_alt_fmt: u32, // 1 if vbuff.fmt1 & 0x40000 != 0 else 0; 0 for Xbox
    #[name_ps3(unk_8)]
    pub offset: u32,
    #[name_ps3(size)]
    pub unk_5: u32,
    #[name_ps3(format)]
    #[ordered_data(skipPC)]
    pub unk_6: u32,
    #[ordered_data(skipPC)]
    pub unk_7: u32,
    #[name_ps3(offset)]
    #[ordered_data(skipPC)]
    pub unk_8: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_9: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_10: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_11: u32,
    #[ordered_data(skipPC, skipPS3)]
    pub unk_12: u32,
}

#[derive(Debug, Clone, PartialEq, Hash)]
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

#[cfg(feature = "python")]
impl<'py> IntoPyObject<'py> for VertexUsage {
    type Target = <String as IntoPyObject<'py>>::Target;
    type Output = <String as IntoPyObject<'py>>::Output;
    type Error = <String as IntoPyObject<'py>>::Error;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        format!("{}", self).into_pyobject(py)
    }
}

#[cfg(feature = "python")]
impl<'a, 'py> IntoPyObject<'py> for &'a VertexUsage {
    type Target = <String as IntoPyObject<'py>>::Target;
    type Output = <String as IntoPyObject<'py>>::Output;
    type Error = <String as IntoPyObject<'py>>::Error;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        format!("{}", self).into_pyobject(py)
    }
}

#[cfg(feature = "python")]
impl<'py> FromPyObject<'_, 'py> for VertexUsage {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let s = String::extract(obj)?;
        s.parse()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!("{}", e)))
    }
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
                    Err(anyhow::anyhow!("unknown vertex usage {}", s))
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
    Int(U32VER),
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

#[cfg_attr(feature = "python", pyclass(module = "model.data", get_all, set_all))]
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

#[make_platforms]
#[derive(Debug, Clone)]
struct VertexBufferVER {
    _ptr: Arc<[u8]>,
    _ptr_info: Arc<[u8]>,
    info: NonNull<VBuffInfoVER>,
    offsets: IndexMap<VertexUsage, VertexDataIndex>,
    size: usize,
    data: NonNull<[u8]>,
}

#[make_platforms]
impl VertexBufferVER {
    pub fn from_bytes(src: &Arc<[u8]>, info_src: &Arc<[u8]>, offset: usize) -> Result<Self> {
        let info = VBuffInfoVER::from_data(&info_src[offset..]).context("info")?;

        let fmt1 = info.fmt1.get();
        let fmt2 = info.fmt2.get();
        let offset = info.offset.get() as usize;
        let size = info.size.get() as usize;
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

        Ok(Self {
            _ptr: src.clone(),
            _ptr_info: info_src.clone(),
            info: info.into(),
            size: s,
            offsets: vals,
            data: NonNull::from_ref(&src[offset..offset+size]),
        })
    }
    pub fn info(&self) -> &VBuffInfoVER {
        unsafe { self.info.as_ref() }
    }
    pub fn iter(&self) -> VertexDataIter {
        VertexDataIter {
            data: unsafe { self.data.as_ref() },
            size: self.size,
            offsets: &self.offsets
        }
    }
}

#[cfg_attr(feature = "python", pyclass(module = "model.data", get_all, set_all))]
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
impl From<&VertexBufferVER> for VertexBuffer {
    fn from(val: &VertexBufferVER) -> Self {
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
                    VertexData::Int(val) => val.push(data.get_int_ver(off).unwrap().into()),
                    VertexData::Color(val) => val.push(data.get_color_ver(off).unwrap().into()),
                    VertexData::Vector2(val) => val.push(data.get_vec2_ver(off).unwrap().into()),
                    VertexData::Vector3(val) => val.push(data.get_vec3_ver(off).unwrap().into()),
                    VertexData::Vector4(val) => val.push(data.get_vec4_ver(off).unwrap().into()),
                }
            }
        }
        Self {
            info: val.info().into(),
            vals,
        }

    }
}

#[make_platforms]
#[derive(Debug, Clone)]
struct IndexBufferVER {
    _ptr: Arc<[u8]>,
    _ptr_info: Arc<[u8]>,
    info: NonNull<IBuffInfoVER>,
    data: NonNull<[u8]>,
    is_u32: bool
}

#[make_platforms]
impl IndexBufferVER {
    pub fn from_bytes(src: &Arc<[u8]>, src_info: &Arc<[u8]>, offset: usize) -> Result<Self> {
        let info = IBuffInfoVER::from_data(&src_info[offset..]).context("info")?;
        Ok(Self {
            _ptr: src.clone(),
            _ptr_info: src_info.clone(),
            info: info.into(),
            data: NonNull::from_ref(&src[info.offset.get() as usize.. (info.size.get() + info.offset.get()) as usize]),
            is_u32: info.format.get() != 0x10
        })
    }

    pub fn u32(&self) -> Option<&[U32VER]> {
        if self.is_u32 {
            Some(U32VER::slice_from_data(unsafe { self.data.as_ref() }, self.data.len()/4).unwrap())
        } else {
            None
        }
    }

    pub fn u16(&self) -> Option<&[U16VER]> {
        if self.is_u32 {
            None
        } else {
            Some(U16VER::slice_from_data(unsafe { self.data.as_ref() }, self.data.len()/2).unwrap())
        }
    }
}

#[cfg_attr(feature = "python", pyclass(module = "model.data", get_all, set_all))]
#[derive(Debug, Clone)]
pub enum IndexBuffer {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

#[make_platforms]
impl From<&IndexBufferVER> for IndexBuffer {
    fn from(val: &IndexBufferVER) -> Self {
        if val.is_u32 {
            Self::U32(val.u32().unwrap().into_iter().map(|x| x.into()).collect())
        } else {
            Self::U16(val.u16().unwrap().into_iter().map(|x| x.into()).collect())
        }
    }
}
/*
impl IndexBuffer {
    #[make_platforms]
    pub fn dump_ver(&self) -> Vec<u8> {
        match self {
            Self::U16 { vals } => {
                let vals: Vec<_> = vals.iter().map(|x| U16VER::new(*x)).collect();
                RefFromDataArgs::as_bytes(&vals[..]).to_vec()
            }
            Self::U32 { vals } => {
                let vals: Vec<_> = vals.iter().map(|x| U32VER::new(*x)).collect();
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
    Int(&'a [U32VER]),
    Color(&'a [ColorVER]),
    Vector2(&'a [Vector2VER]),
    Vector3(&'a [Vector3VER]),
    Vector4(&'a [Vector4VER]),
}

// gotten from vertex data
#[derive(Debug, Clone)]
pub struct VertexDataIndex {
    key: u32,
    offset: usize,
}

pub struct VertexValRef<'a> {
    data: &'a [u8]
}

impl<'a> VertexValRef<'a> {
    #[make_platforms]
    pub fn get_int_ver(&'a self, ind: &VertexDataIndex) -> Result<&'a U32VER> {
        U32VER::from_data(&self.data[ind.offset..])
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
#[cfg_attr(feature = "python", pyclass(module = "model"))]
#[derive(Debug, Clone)]
pub struct ModelDataVER {
    _ptr: Arc<[u8]>,
    infos: NonNull<[BufferInfoVER]>,
    vbuff_order: NonNull<[U32VER]>,
    ibuff_order: NonNull<[U32VER]>,
    vertex: IndexMap<u32, VertexBufferVER>,
    index: IndexMap<u32, IndexBufferVER>
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(ModelDataVER);

#[make_platforms]
unsafe impl Sync for ModelDataVER {}
#[make_platforms]
unsafe impl Send for ModelDataVER {}

#[make_platforms]
impl ModelDataVER {
    pub fn from_bytes(src: &Arc<[u8]>, src_info: &Arc<[u8]>, info: &ModelInfoVER) -> Result<Self> {
        let infos = BufferInfoVER::slice_from_data(
            &src[info.buffer_info_offset.get() as usize..],
            info.mat_num.get() as usize,
        )
        .context("buffer_infos")?;
        let vbuff_order = U32VER::slice_from_data(
            &src_info[info.vbuff_offset.get() as usize..],
            info.vbuff_num.get() as usize,
        )
        .context("vbuff_order")?;
        let ibuff_order = U32VER::slice_from_data(
            &src_info[info.ibuff_offset.get() as usize..],
            info.ibuff_num.get() as usize,
        )
        .context("ibuff_order")?;
        let vertex_offs: IndexMap<_, _> = vbuff_order.iter().enumerate().map(|(i, x)| (x.get(), i)).collect();
        let index_offs: IndexMap<_, _> = ibuff_order.iter().enumerate().map(|(i, x)| (x.get(), i)).collect();

        Ok(Self {
            _ptr: src.clone(),
            infos: infos.into(),
            vbuff_order: vbuff_order.into(),
            ibuff_order: ibuff_order.into(),
            vertex: vertex_offs.into_iter().map(|(k, off)| Ok((k, VertexBufferVER::from_bytes(src, src_info, off).with_context(|| format!("vertex data {}", k))?))).collect::<Result<_>>()?,
            index: index_offs.into_iter().map(|(k, off)| Ok((k, IndexBufferVER::from_bytes(src, src_info, off).with_context(|| format!("index data {}", k))?))).collect::<Result<_>>()?
        })
    }
}

#[make_platforms]
impl TryFrom<&ModelRawVER> for ModelDataVER {
    type Error = anyhow::Error;
    fn try_from(val: &ModelRawVER) -> Result<Self> {
        let data = val.data.get().context("data")?;
        let vertex = unsafe { val.vbuff_order.as_ref() }.iter().map(|x| Ok((x.get(), VertexBufferVER::from_bytes(&data, &val._ptr, x.get() as usize).with_context(|| format!("vertex data {}", x.get()))?))).collect::<Result<_>>()?;
        let index = unsafe { val.ibuff_order.as_ref() }.iter().map(|x| Ok((x.get(), IndexBufferVER::from_bytes(&data, &val._ptr, x.get() as usize).with_context(|| format!("index data {}", x.get()))?))).collect::<Result<_>>()?;
        Ok(Self {
            _ptr: val._ptr.clone(),
            infos: val.buffer_infos,
            vbuff_order: val.vbuff_order,
            ibuff_order: val.ibuff_order,
            vertex,
            index
        })
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl ModelDataVER {
    #[getter]
    pub fn infos(&self) -> &[BufferInfoVER] {
        unsafe { self.infos.as_ref() }
    }
    /*
    pub fn vertex(&self) -> &IndexMap<u32, IndexMap<VertexUsage, VertexDataVER>> {
        &self.vertex
    }
    pub fn index(&self) -> &IndexMap<u32, IndexBufferVER> {
        &self.index
    }
    */
}

#[cfg_attr(feature = "python", pyclass(module = "model", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct ModelData {
    pub vertex: Vec<VertexBuffer>,
    pub index: Vec<IndexBuffer>,
}

#[make_platforms]
impl From<&ModelDataVER> for ModelData {
    fn from(val: &ModelDataVER) -> Self {
        Self {
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
