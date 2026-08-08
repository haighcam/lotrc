use anyhow::{anyhow, Context, Result};
use enum_dispatch::enum_dispatch;
use indexmap::IndexMap;

use crate::types::{OwnedCompressedData, CompressedData, hash_string, Color, RefFromData, OrderedData, CompressedDataRef, DumpSlice, DumpData, align_offset, ref_slice, EndianTypes};
use crate::level::LevelFormat;
#[make_endian]
use crate::{
    level::{
        pak::{
            block1::{
                infos::DumpInfos_XE_,
                gameobjs::BaseTypeRef_XE_,
                Block1Ref_XE_
            }
        },
    },
    types::{Color_XE_, i32_XE_, u32_XE_},
};
use crate::level::pak::block1::infos::InfoCounts;
use lotrc_proc::{make_endian, derive_ordered_data};

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
// points to list of ints in block1
pub struct RadiosityValsInfo_XE_ {
    pub guid: u32_XE_,
    pub num: u32_XE_,
    pub offset: u32_XE_,
}

// list of ints pointing to the radiosity data
// has a value for each mesh in the model
// corresponding to the object pointed to by the guid
// (value if offset to consecutive values in radiosity, one for each vertex of the mesh)
// a value of -1 means no radiosity for that mesh
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct RadiosityValsRef<'a, L: LevelFormat> {
    pub info: &'a L::RadiosityValsInfo,
    pub offs: ref_slice<'a, L::i32>,
}

impl<'a, L: LevelFormat> RadiosityValsRef<'a, L> {
    pub fn from_data(src: &'a [u8], info: &'a L::RadiosityValsInfo) -> Result<Self> {
        let offs = L::i32::slice_from_data(&src[info.offset() as usize..], info.num() as usize).context("offs")?;
        Ok(Self { info, offs: offs.into() })
    }
}

// list of ints pointing to the radiosity data
// has a value for each mesh in the model
// corresponding to the object pointed to by the guid
// (value if offset to consecutive values in radiosity, one for each vertex of the mesh)
// a value of -1 means no radiosity for that mesh
#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct RadiosityValsRef_XE_<'a> {
    pub info: &'a RadiosityValsInfo_XE_,
    pub offs: ref_slice<'a, i32_XE_>,
}

#[make_endian]
impl<'a> RadiosityValsRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a RadiosityValsInfo_XE_) -> Result<Self> {
        let offs = i32_XE_::slice_from_data(&src[info.offset.conv()..], info.num.conv()).context("offs")?;
        Ok(Self { info, offs: offs.into() })
    }
}

#[make_endian]
pub trait DumpRadiosityVals_XE_ {
    fn off_num(&self) -> usize;
    fn write_offs(&self, offs: &mut [i32_XE_]) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice, info: &mut RadiosityValsInfo_XE_) -> Result<()> {
        info.offset = dst.offset.conv(); 
        let offs = i32_XE_::mut_slice_from_data(dst, self.off_num()).context("offs")?;
        info.num = offs.len().conv();
        self.write_offs(offs).context("write offs")?;
        dst.align(16)?;
        Ok(())
    }
    fn add_size(&self, offset: usize) -> usize {
        align_offset(offset + self.off_num() * std::mem::size_of::<i32_XE_>(), 16)
    }
}

#[make_endian]
impl DumpRadiosityVals_XE_ for RadiosityValsRef_XE_<'_> {
    fn off_num(&self) -> usize {
        self.offs.len()
    }
    fn write_offs(&self, offs: &mut [i32_XE_]) -> Result<()> {
        offs.write_from(&self.offs[..])
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq, Default)]
pub struct RadiosityRef<'a, L: LevelFormat> {
    pub data: Option<&'a CompressedDataRef<'a>>,
    pub vals: IndexMap<u32, RadiosityValsRef<'a, L>>,
    pub usage: u32,
}

impl<'a, L: LevelFormat> RadiosityRef<'a, L> {
    pub fn from_data(src: &'a [u8], infos: &'a [L::RadiosityValsInfo], data: Option<&'a CompressedDataRef<'a>>, usage: u32) -> Result<Self> {
        let vals = infos.into_iter().map(|info| Ok((info.guid(), RadiosityValsRef::from_data(src, info).with_context(|| format!("radiosity {}", info.guid()))?))).collect::<Result<IndexMap<_, _>>>()?;
        Ok(Self {
            data: data.into(),
            vals: vals.into(),
            usage
        })
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct RadiosityRef_XE_<'a> {
    pub data: Option<&'a CompressedDataRef<'a>>,
    pub vals: IndexMap<u32, RadiosityValsRef_XE_<'a>>,
    pub usage: u32,
}

#[make_endian]
impl Default for RadiosityRef_XE_<'_> {
    fn default() -> Self {
        Self {
            data: None,
            vals: IndexMap::default().into(),
            usage: 0
        }
    }
}

#[make_endian]
impl<'a> RadiosityRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], infos: &'a [RadiosityValsInfo_XE_], data: Option<&'a CompressedDataRef<'a>>, usage: u32) -> Result<Self> {
        let vals = infos.into_iter().map(|info| Ok((info.guid.conv(), RadiosityValsRef_XE_::from_data(src, info).with_context(|| format!("radiosity {}", info.guid.to_native()))?))).collect::<Result<IndexMap<_, _>>>()?;
        Ok(Self {
            data: data.into(),
            vals: vals.into(),
            usage
        })
    }
}

#[derive(Debug, Clone)]
pub enum RadiosityVal {
    Radiosity(Vec<Color>),
    NoRadiosity(u32),
}

#[derive(Debug, Default, Clone)]
pub struct Radiosity {
    pub vals: IndexMap<u32, Vec<RadiosityVal>>,
    pub usage: u32,
}

impl Radiosity {
    #[make_endian]
    pub fn from_xe_(val: &RadiosityRef_XE_, block1: &Block1Ref_XE_) -> Result<Self> {
        let gameobjs = &block1.sub_blocks.level;
        let models = &block1.objs.models;
        let mut off: usize = 0;
        let data = if let Some(data) = val.data.as_ref() {
            &data.data_decomp[..]
        } else {
            &[] as _
        };
        Ok(Radiosity {
            vals: val.vals.values()
                .map(|RadiosityValsRef_XE_ { info, offs: vals }| {
                    let obj = gameobjs.objs.get(&info.guid.to_native()).unwrap();
                    let model = obj
                        .fields
                        .get(&hash_string(b"mesh", None))
                        .ok_or(anyhow!("missing mesh field"))
                        .and_then(|field| if let BaseTypeRef_XE_::Crc(x) = field {
                            Ok(x.to_native())
                        } else {
                            Err(anyhow!("mesh field not crc"))
                        })
                        .and_then(|mesh| {
                            models.get(&mesh)
                            .ok_or_else(|| anyhow!("model {} missing", mesh))
                        })
                        .with_context(|| format!("obj {}", info.guid.to_native()))?;
                    Ok((
                        info.guid.conv(),
                        model.data.infos.iter()
                            .map(|x| (x.vbuff_size.to_native() / x.v_size.to_native()) as usize)
                            .zip(vals.iter())
                            .map(|(size, offset)| {
                                let offset: usize = offset.conv();
                                Ok(if offset != off {
                                    RadiosityVal::NoRadiosity(offset as u32)
                                } else {
                                    off += size;
                                    RadiosityVal::Radiosity(
                                        Color_XE_::slice_from_data(&data[offset*4..], size).with_context(|| format!("rad data from {}", info.guid.to_native()))?
                                            .iter()
                                            .map(|x| x.conv())
                                            .collect(),
                                    )
                                })
                            })
                            .collect::<Result<_>>()?,
                    ))
                })
                .collect::<Result<_>>()?,
            usage: val.usage,
        })
    }
}

#[make_endian]
#[enum_dispatch(DumpRadiosity_XE_)]
pub enum Radiosity_XE_<'a> {
    Ref(RadiosityRef_XE_<'a>),
    Owned(Radiosity)
}

#[make_endian]
pub trait DumpRadiosityImpl_XE_ {
    fn vals(&self) -> impl Iterator<Item=(u32, &impl DumpRadiosityVals_XE_)>;
    fn data(&self) -> CompressedData<'_>;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpRadiosity_XE_ {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_) -> Result<CompressedData<'_>>;
    fn add_size(&self, offset: usize, counts: &mut InfoCounts) -> usize;
    fn usage(&self) -> u32_XE_;
}

#[make_endian]
impl DumpRadiosity_XE_ for RadiosityRef_XE_<'_> {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_) -> Result<CompressedData<'_>> {
        for (guid, vals) in self.vals.iter() {
            *infos.offsets.next().context("offsets")? = (infos.radiosity_vals.offset + std::mem::offset_of!(RadiosityValsInfo_XE_, offset)).conv();
            let info = infos.radiosity_vals.next().context("radiosity_vals")?;
            vals.dump_into(dst, info).with_context(|| format!("vals {}", guid))?;
            info.guid = guid.conv();
        }
        Ok(self.data.into())
    }
    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        for (_, vals) in self.vals.iter() {
            counts.radiosity_vals += 1;
            counts.offsets += 1;
            offset = vals.add_size(offset);
        }
        offset
    }
    fn usage(&self) -> u32_XE_ {
        self.usage.conv()
    }
}

#[make_endian]
impl DumpRadiosity_XE_ for Radiosity {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos_XE_) -> Result<CompressedData<'_>> {
        let mut data_len = 0;
        for (guid, vals) in self.vals.iter() {
            *infos.offsets.next().context("offsets")? = (infos.radiosity_vals.offset + std::mem::offset_of!(RadiosityValsInfo_XE_, offset)).conv();
            let info = infos.radiosity_vals.next().context("radiosity_vals")?;
            info.guid = guid.conv();
            info.num = vals.len().conv();
            info.offset = dst.offset.conv();
            let offs = u32_XE_::mut_slice_from_data(dst, vals.len()).context("offs")?;
            for (val, dst) in vals.iter().zip(offs) {
                match val {
                    RadiosityVal::NoRadiosity(val) => *dst = val.conv(),
                    RadiosityVal::Radiosity(val) => {
                        *dst = (data_len / 4).conv();
                        data_len += val.size_of_val();
                    }
                }
            }
            dst.align(16)?;
        }
        let mut data = OwnedCompressedData::with_capacity(data_len);
        let mut data_dst = data.dump_slice();
        for vals in self.vals.values() {
            for val in vals {
                match val {
                    RadiosityVal::Radiosity(val) => {
                        let dst = u32_XE_::mut_slice_from_data(&mut data_dst, val.len()).context("vals")?;
                        for (src, dst) in val.iter().zip(dst) {
                            *dst = src.conv();
                        }
                    }
                    _ => ()
                }
            }
        }
        Ok(data.into())
    }
    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        for vals in self.vals.values() {
            counts.radiosity_vals += 1; 
            counts.offsets += 1;
            offset = align_offset(offset + vals.len() * std::mem::size_of::<u32_XE_>(), 16);
        }
        offset
    }
    fn usage(&self) -> u32_XE_ {
        self.usage.conv()
    }
}
