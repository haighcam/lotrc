use anyhow::{anyhow, Context, Result};
use enum_dispatch::enum_dispatch;
use indexmap::IndexMap;

use crate::types::{OwnedCompressedData, hash_string, Color, ReadData, CompressedDataRef, DumpSlice, align_offset, ref_slice, BaseTypes, CompressedData};
use crate::level::pak::block1::{
    Block1Ref,
    gameobjs::BaseTypeRef,
    infos::{InfoCounts, DumpInfos}
};
use lotrc_proc::{derive_pod};

#[derive_pod]
// points to list of ints in block1
pub struct RadiosityValsInfo<T: BaseTypes> {
    pub guid: T::u32,
    pub num: T::u32,
    pub offset: T::u32,
}

// list of ints pointing to the radiosity data
// has a value for each mesh in the model
// corresponding to the object pointed to by the guid
// (value if offset to consecutive values in radiosity, one for each vertex of the mesh)
// a value of -1 means no radiosity for that mesh
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Debug, PartialEq)]
pub struct RadiosityValsRef<'a,  T: BaseTypes> {
    pub info: &'a RadiosityValsInfo<T>,
    pub offs: ref_slice<'a, T::i32>,
}

impl<'a,  T: BaseTypes> RadiosityValsRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a RadiosityValsInfo<T>) -> Result<Self> {
        let offs = T::i32::slice_from_data(&src[info.offset.into() as usize..], info.num.into() as usize).context("offs")?;
        Ok(Self { info, offs: offs.into() })
    }
}

pub trait DumpRadiosityVals<T: BaseTypes> {
    fn off_num(&self) -> usize;
    fn write_offs(&self, offs: &mut [T::i32]) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice, info: &mut RadiosityValsInfo<T>) -> Result<()> {
        info.offset = (dst.offset as u32).into(); 
        let offs = T::i32::mut_slice_from_data(dst, self.off_num()).context("offs")?;
        info.num = (offs.len() as u32).into();
        self.write_offs(offs).context("write offs")?;
        dst.align(16)?;
        Ok(())
    }
    fn add_size(&self, offset: usize) -> usize {
        align_offset(offset + self.off_num() * std::mem::size_of::<T::i32>(), 16)
    }
}

impl<T: BaseTypes> DumpRadiosityVals<T> for RadiosityValsRef<'_, T> {
    fn off_num(&self) -> usize {
        self.offs.len()
    }
    fn write_offs(&self, offs: &mut [T::i32]) -> Result<()> {
        offs.copy_from_slice(self.offs);
        Ok(())
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq, Default)]
pub struct RadiosityRef<'a,  T: BaseTypes> {
    pub data: Option<&'a CompressedDataRef<'a>>,
    pub vals: IndexMap<u32, RadiosityValsRef<'a, T>>,
    pub usage: u32,
}

impl<'a,  T: BaseTypes> RadiosityRef<'a, T> {
    pub fn from_data(src: &'a [u8], infos: &'a [RadiosityValsInfo<T>], data: Option<&'a CompressedDataRef<'a>>, usage: u32) -> Result<Self> {
        let vals = infos.into_iter().map(|info| Ok((info.guid.into(), RadiosityValsRef::from_data(src, info).with_context(|| format!("radiosity {}", info.guid.into()))?))).collect::<Result<IndexMap<_, _>>>()?;
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
    pub fn from_ref<T: BaseTypes>(val: &RadiosityRef<T>, block1: &Block1Ref<T>) -> Result<Self> {
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
                .map(|RadiosityValsRef { info, offs: vals }| {
                    let obj = gameobjs.objs.get(&info.guid.into()).unwrap();
                    let model = obj
                        .fields
                        .get(&hash_string(b"mesh", None))
                        .ok_or(anyhow!("missing mesh field"))
                        .and_then(|field| if let BaseTypeRef::Crc(x) = field {
                            Ok(x.val.into())
                        } else {
                            Err(anyhow!("mesh field not crc"))
                        })
                        .and_then(|mesh| {
                            models.get(&mesh)
                            .ok_or_else(|| anyhow!("model {} missing", mesh))
                        })
                        .with_context(|| format!("obj {}", info.guid.into()))?;
                    Ok((
                        info.guid.into(),
                        model.data.infos.iter()
                            .map(|x| (x.vbuff_size.into() / x.v_size.into()) as usize)
                            .zip(vals.iter())
                            .map(|(size, &offset)| {
                                let offset = offset.into() as usize;
                                Ok(if offset != off {
                                    RadiosityVal::NoRadiosity(offset as u32)
                                } else {
                                    off += size;
                                    RadiosityVal::Radiosity(
                                        Color::slice_from_data(&data[offset*4..], size).with_context(|| format!("rad data from {}", info.guid.into()))?
                                            .iter()
                                            .map(|&x| x.into())
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

/*
#[enum_dispatch(DumpRadiosity_XE_)]
pub enum Radiosity_XE_<'a> {
    Ref(RadiosityRef_XE_<'a>),
    Owned(Radiosity)
}
*/

pub trait DumpRadiosityImpl<T: BaseTypes> {
    fn vals(&self) -> impl Iterator<Item=(u32, &impl DumpRadiosityVals<T>)>;
    fn data(&self) -> CompressedData<'_>;
}

#[enum_dispatch]
pub trait DumpRadiosity<T: BaseTypes> {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>) -> Result<CompressedData<'_>>;
    fn add_size(&self, offset: usize, counts: &mut InfoCounts) -> usize;
    fn usage(&self) -> T::u32;
}

impl<T: BaseTypes> DumpRadiosity<T> for RadiosityRef<'_, T> {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>) -> Result<CompressedData<'_>> {
        for (&guid, vals) in self.vals.iter() {
            *infos.offsets.next().context("offsets")? = ((infos.radiosity_vals.offset + std::mem::offset_of!(RadiosityValsInfo<T>, offset)) as u32).into();
            let info = infos.radiosity_vals.next().context("radiosity_vals")?;
            vals.dump_into(dst, info).with_context(|| format!("vals {}", guid))?;
            info.guid = guid.into();
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
    fn usage(&self) -> T::u32 {
        self.usage.into()
    }
}

impl<T: BaseTypes> DumpRadiosity<T> for Radiosity {
    fn dump_into(&self, dst: &mut DumpSlice, infos: &mut DumpInfos<T>) -> Result<CompressedData<'_>> {
        let mut data_len = 0;
        for (&guid, vals) in self.vals.iter() {
            *infos.offsets.next().context("offsets")? = ((infos.radiosity_vals.offset + std::mem::offset_of!(RadiosityValsInfo<T>, offset)) as u32).into();
            let info = infos.radiosity_vals.next().context("radiosity_vals")?;
            info.guid = guid.into();
            info.num = (vals.len() as u32).into();
            info.offset = (dst.offset as u32).into();
            let offs = T::u32::mut_slice_from_data(dst, vals.len()).context("offs")?;
            for (val, dst) in vals.iter().zip(offs) {
                match val {
                    RadiosityVal::NoRadiosity(val) => *dst = (*val).into(),
                    RadiosityVal::Radiosity(val) => {
                        *dst = (data_len as u32 / 4).into();
                        data_len += std::mem::size_of_val(val);
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
                        let dst = T::u32::mut_slice_from_data(&mut data_dst, val.len()).context("vals")?;
                        for (&src, dst) in val.iter().zip(dst) {
                            *dst = src.into();
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
            offset = align_offset(offset + vals.len() * std::mem::size_of::<T::u32>(), 16);
        }
        offset
    }
    fn usage(&self) -> T::u32 {
        self.usage.into()
    }
}
