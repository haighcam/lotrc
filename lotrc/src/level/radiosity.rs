use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use log::warn;
use std::ptr::NonNull;

#[cfg(not(feature = "ffi"))]
use crate::types::GetNative;
use crate::types::{hash_string, Color, RefFromData, OrderedData, OrderedDataStrict, CompressedDataRef, BufType, CompressedDataRefAlt, slice, box_slice, Map, MapImpl, DumpSlice, DumpData, align_offset};
#[make_platforms]
use crate::{
    level::{
        pak::{
            objs::DumpInfosVER,
            Block1VER, PakHeaderVER
        },
        bin::BinVER,
    },
    types::{ColorVER, i32VER, u32VER},
    sub_blocks::gameobjs::GameObjsVER,
};
use crate::level::pak::objs::InfoCounts;
use lotrc_proc::{make_platforms, OrderedData};

#[derive(Debug, Default, Clone, OrderedData)]
// points to list of ints in block1
pub struct RadiosityValsInfo {
    pub guid: u32,
    pub num: u32,
    pub offset: u32,
}

#[make_platforms]
#[derive(Debug, Clone)]
// list of ints pointing to the radiosity data
// has a value for each mesh in the model
// corresponding to the object pointed to by the guid
// (value if offset to consecutive values in radiosity, one for each vertex of the mesh)
// a value of -1 means no radiosity for that mesh
pub struct RadiosityValsVER {
    _ptr: BufType,
    info: NonNull<RadiosityValsInfoVER>,
    vals: NonNull<[i32VER]>,
}

#[make_platforms]
unsafe impl Sync for RadiosityValsVER {}
#[make_platforms]
unsafe impl Send for RadiosityValsVER {}

#[make_platforms]
impl RadiosityValsVER {
    pub fn from_bytes(src: &BufType, offset: usize) -> Result<Self> {
        let info = RadiosityValsInfoVER::from_data(&src[offset..]).context("info")?;
        let vals =
            i32VER::slice_from_data(&src[info.offset.get() as usize..], info.num.get() as usize)
                .context("vals")?;
        Ok(Self {
            _ptr: src.clone(),
            info: info.into(),
            vals: vals.into(),
        })
    }
}

#[make_platforms]
impl RadiosityValsVER {
    pub fn info(&self) -> &RadiosityValsInfoVER {
        unsafe { self.info.as_ref() }
    }
    pub fn vals(&self) -> &[i32VER] {
        unsafe { self.vals.as_ref() }
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct RadiosityValsRefVER<'a> {
    pub info: &'a RadiosityValsInfoVER,
    pub offs: slice<'a, i32VER>,
}

#[make_platforms]
impl<'a> RadiosityValsRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a RadiosityValsInfoVER) -> Result<Self> {
        let offs = i32VER::slice_from_data(&src[info.offset.get() as usize..], info.num.get() as usize).context("offs")?;
        Ok(Self { info, offs: offs.into() })
    }
}

#[make_platforms]
pub trait DumpRadiosityValsVER {
    fn off_num(&self) -> usize;
    fn write_offs(&self, offs: &mut [i32VER]) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice, info: &mut RadiosityValsInfoVER) -> Result<()> {
        info.offset = dst.offset.conv(); 
        let offs = i32VER::mut_slice_from_data(dst, self.off_num()).context("offs")?;
        info.num = offs.len().conv();
        self.write_offs(offs).context("write offs")?;
        dst.align(16);
        Ok(())
    }
    fn add_size(&self, offset: usize) -> usize {
        align_offset(offset + self.off_num() * std::mem::size_of::<i32VER>(), 16)
    }
}

#[make_platforms]
impl DumpRadiosityValsVER for RadiosityValsRefVER<'_> {
    fn off_num(&self) -> usize {
        self.offs.len()
    }
    fn write_offs(&self, offs: &mut [i32VER]) -> Result<()> {
        offs.write_from(&self.offs[..])
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
#[repr(C)]
pub struct RadiosityRefVER<'a> {
    pub data: Option<&'a CompressedDataRefAlt<'a>>,
    pub vals: Map<u32, RadiosityValsRefVER<'a>>,
    pub usage: u32,
}

#[make_platforms]
impl Default for RadiosityRefVER<'_> {
    fn default() -> Self {
        Self {
            data: None,
            vals: MapImpl::default().into(),
            usage: 0
        }
    }
}

#[make_platforms]
impl<'a> RadiosityRefVER<'a> {
    pub fn from_data(src: &'a [u8], infos: &'a [RadiosityValsInfoVER], data: Option<&'a CompressedDataRefAlt<'a>>, usage: u32) -> Result<Self> {
        let vals = infos.into_iter().map(|info| Ok((info.guid.get(), RadiosityValsRefVER::from_data(src, info).with_context(|| format!("radiosity {}", info.guid.get()))?))).collect::<Result<MapImpl<_, _>>>()?;
        Ok(Self {
            data,
            vals: vals.into(),
            usage
        })
    }
}

#[make_platforms]
#[derive(Clone, Debug)]
pub struct RadiosityRawVER {
    _ptr: BufType,
    data: CompressedDataRef,
    infos: NonNull<[RadiosityValsInfoVER]>,
    offs: Box<[NonNull<[i32VER]>]>,
    usage: u32
}

#[make_platforms]
impl RadiosityRawVER {
    pub fn from_bytes(src: &BufType, bin: &BinVER, level: &GameObjsVER, pak_header: &PakHeaderVER) -> Result<Self> {
        let infos = RadiosityValsInfoVER::slice_from_data(
            &src[pak_header.radiosity_vals_info_offset.get() as usize..],
            pak_header.radiosity_vals_info_num.get() as usize
        ).context("infos")?;
        let offs = infos.iter().map(|info| Ok(NonNull::from_ref(i32VER::slice_from_data(&src[info.offset.get() as usize..], info.num.get() as usize)?))).collect::<Result<Vec<_>>>().context("offs")?;
        let field = level
            .objs()
            .iter()
            .find(|(_, v)| v.header().key.get() == hash_string(b"templateLevel", None))
            .ok_or(anyhow!("templateLevel not found"))?
            .1
            .get(&hash_string(b"name", None))
            .ok_or(anyhow!("templateLevel missing name field"))?;
        let name = field
            .crc()
            .ok_or(anyhow!("templateObject name field is not a crc"))?;
        let radiosity_name = hash_string(b"_radiosity", Some(name.get()));
        let model_data = bin.model_data();
        let ind = model_data.get_index_of(&radiosity_name);
        let data = ind.map(|x| model_data.get_index(x).unwrap().1.clone()).unwrap_or_default(); 
        let usage =  ind.map(|x| bin.model_handles()[x].kind.get()).unwrap_or_default();
        Ok(Self {
            _ptr: src.clone(),
            data,
            infos: infos.into(),
            offs: offs.into(),
            usage
        })
    }
}

#[make_platforms]
#[derive(Clone, Debug)]
pub struct RadiosityVER {
    _ptr: BufType,
    _data: BufType,
    infos: NonNull<[RadiosityValsInfoVER]>,
    offs: Box<[NonNull<[i32VER]>]>,
    vals: NonNull<[ColorVER]>,
    usage: u32,
}

#[make_platforms]
unsafe impl Sync for RadiosityVER {}
#[make_platforms]
unsafe impl Send for RadiosityVER {}

#[make_platforms]
impl TryFrom<RadiosityRawVER> for RadiosityVER {
    type Error = anyhow::Error;
    fn try_from(RadiosityRawVER { _ptr, data, infos, offs, usage }: RadiosityRawVER) -> Result<Self> {
        let _data = data.get()?.clone();
        if _data.len() % 4 != 0 {
            warn!("Radiosity length is incorrect?")
        }
        let vals = NonNull::from_ref(if _data.len() == 0 {
            &[] as _
        } else {
            ColorVER::slice_from_data(&_data[..], _data.len() / 4).context("vals")?
        });
        Ok(Self { _ptr, _data, infos, offs, vals, usage })
    }
}

#[make_platforms]
impl RadiosityVER {
    pub fn vals(&self) -> &[ColorVER] {
        unsafe { self.vals.as_ref() }
    }
    pub fn usage(&self) -> &u32 {
        &self.usage
    }
    pub fn infos(&self) -> &[RadiosityValsInfoVER] {
        unsafe { self.infos.as_ref() }
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
    #[make_platforms]
    pub fn from_ver(val: &RadiosityVER, block1: &Block1VER) -> Result<Self> {
        let gameobjs = block1
            .sub_blocks()
            .get(&hash_string(b"level", None))
            .ok_or(anyhow!("level block missing"))
            .and_then(|x| x.level().ok_or(anyhow!("level block wrong format")))?;
        let models = block1.objs().models();
        let mut off: usize = 0;
        Ok(Radiosity {
            vals: val.offs.iter().zip(val.infos())
                .map(|(vals, info)| {
                    let obj = gameobjs.objs().get(&info.guid.get()).unwrap();
                    let model = obj
                        .get(&hash_string(b"mesh", None))
                        .ok_or(anyhow!("missing mesh field"))
                        .and_then(|field| {
                            field.crc()
                            .map(|x| x.get())
                            .ok_or(anyhow!("mesh field not crc"))
                        })
                        .and_then(|mesh| {
                            models.get(&mesh)
                            .ok_or_else(|| anyhow!("model {} missing", mesh))
                        })
                        .with_context(|| format!("obj {}", info.guid.get()))?;
                    Ok((
                        info.guid.get(),
                        model.data().infos().iter()
                            .map(|x| (x.vbuff_size.get() / x.v_size.get()) as usize)
                            .zip(unsafe { vals.as_ref() })
                            .map(|(offset, size)| {
                                if offset != off {
                                    RadiosityVal::NoRadiosity(offset as u32)
                                } else {
                                    off += size.get() as usize;
                                    RadiosityVal::Radiosity(
                                        val.vals()[offset..offset + size.get() as usize]
                                            .iter()
                                            .map(|x| x.conv())
                                            .collect(),
                                    )
                                }
                            })
                            .collect(),
                    ))
                })
                .collect::<Result<_>>()?,
            usage: val.usage,
        })
    }
}

#[make_platforms]
pub trait DumpRadiosityVER {
    type Data;
    fn vals(&self) -> impl Iterator<Item=(u32, &impl DumpRadiosityValsVER)>;
    fn dump_into<D>(&self, dst: &mut DumpSlice, infos: &mut DumpInfosVER<D>) -> Result<()> {
        for (guid, vals) in self.vals() {
            *infos.offsets.next() = (infos.radiosity_vals.offset + std::mem::offset_of!(RadiosityValsInfoVER, offset)).conv();
            let info = infos.radiosity_vals.next();
            vals.dump_into(dst, info).with_context(|| format!("vals {}", guid))?;
            info.guid = guid.conv();
        }
        Ok(())
    }
    fn add_size(&self, mut offset: usize, counts: &mut InfoCounts) -> usize {
        for (_, vals) in self.vals() {
            counts.radiosity_vals += 1;
            offset = vals.add_size(offset);
        }
        offset
    }
    fn data(&self) -> Option<Self::Data>;
    fn usage(&self) -> u32VER;
}

#[make_platforms]
impl<'a> DumpRadiosityVER for RadiosityRefVER<'a> {
    type Data = &'a CompressedDataRefAlt<'a>;
    fn vals(&self) -> impl Iterator<Item=(u32, &impl DumpRadiosityValsVER)> {
        self.vals.iter().map(|(k,v)| (*k, v))
    }
    fn data(&self) -> Option<Self::Data> {
        self.data
    }
    fn usage(&self) -> u32VER {
        self.usage.conv()
    }
}

#[make_platforms]
pub enum RadiosityPatchVER {
    Raw(RadiosityRawVER),
    Parsed(Radiosity)
}
