use crate::{
    level::pak::{
        PakHeader,
        block1::objs::PFieldInfo
    },
    types::{
        get_str, hash_string, Crc, DumpSlice, ReadData, Vector3, align_offset, ref_slice, slice, BaseTypes, NE,
        sub_blocks::{
            DataRef, StringKeysRef, SubBlocksInfoRef, DumpDataImpl, DumpString
        }
    }
};
use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use log::{warn, debug};
use lotrc_proc::{derive_pod};
use std::collections::{HashMap, HashSet};
use enum_dispatch::enum_dispatch;

pub mod keys {
    use super::hash_string;
    pub const KEY_POLISH: u32 = hash_string("Polish".as_bytes(), None);
    pub const KEY_GERMAN: u32 = hash_string("German".as_bytes(), None);
    pub const KEY_FRENCH: u32 = hash_string("French".as_bytes(), None);
    pub const KEY_SPANISH: u32 = hash_string("Spanish".as_bytes(), None);
    pub const KEY_RUSSIAN: u32 = hash_string("Russian".as_bytes(), None);
    pub const KEY_SWEDISH: u32 = hash_string("Swedish".as_bytes(), None);
    pub const KEY_ENGLISH: u32 = hash_string("English".as_bytes(), None);
    pub const KEY_ITALIAN: u32 = hash_string("Italian".as_bytes(), None);
    pub const KEY_NORWEGIAN: u32 = hash_string("Norwegian".as_bytes(), None);
    pub const KEY_ATLAS1: u32 = hash_string("atlas_1.uv".as_bytes(), None);
    pub const KEY_ATLAS2: u32 = hash_string("atlas_2.uv".as_bytes(), None);
    pub const KEY_CROWD: u32 = hash_string("3dCrowd".as_bytes(), None);
    pub const KEY_SPRAY: u32 = hash_string("Spray".as_bytes(), None);
    pub const KEY_PFIELDS: u32 = hash_string("PFields".as_bytes(), None);
    pub const KEY_LEVEL: u32 = hash_string("Level".as_bytes(), None);
}

#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Block2Ref<'a, T: BaseTypes> {
    pub sub_blocks: SubBlocks2Ref<'a, T>,
    pub offsets: ref_slice<'a, T::u32>
}

impl<'a, T: BaseTypes> Block2Ref<'a, T> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeader<T>, string_keys: &StringKeysRef<'a, T>) -> Result<Self> {
        let t = std::time::Instant::now();
        let sub_blocks = SubBlocks2Ref::from_data(
            &src[pak_header.sub_blocks2_offset.into() as usize..],
            string_keys
        )
        .context("sub_blocks")?;
        debug!("Block2 sub_blocks parsed in {}", t.elapsed().as_secs_f32());
        let t = std::time::Instant::now();
        let offsets = T::u32::slice_from_data(
            &src[pak_header.block2_offsets_offset.into() as usize..],
            pak_header.block2_offsets_num.into() as usize,
        )
        .context("offsets")?;
        debug!("Block2 offsets parsed in {}", t.elapsed().as_secs_f32());
        debug!("Block2 offsets len {}", offsets.len());
        Ok(Self {
            sub_blocks,
            offsets: offsets.into(),
        })
    }
}

pub trait DumpBlock2<T: BaseTypes> {
    fn sub_blocks(&self) -> &impl DumpSubBlocks2<T>;
    fn dump<'a>(&self, dst: &mut DumpSlice<'a>, offset_num: usize, keys: &[u32], header: &mut PakHeader<T>) -> Result<&'a mut [T::u32]> {
        header.sub_blocks2_offset = (dst.offset as u32).into();
        debug!("sub_blocks2 start {}", dst.offset);
        self.sub_blocks().dump_into(dst, keys).context("sub_blocks")?;
        header.block2_offsets_offset = (dst.offset as u32).into();
        let offsets = T::u32::mut_slice_from_data(dst, offset_num).context("offsets")?;
        header.block2_offsets_num = (offsets.len() as u32).into();
        Ok(offsets)
    }
}

impl<T: BaseTypes> DumpBlock2<T> for Block2Ref<'_, T> {
    fn sub_blocks(&self) -> &impl DumpSubBlocks2<T> {
        &self.sub_blocks
    }
}


#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SubBlocks2Ref<'a, T: BaseTypes> {
    pub info: SubBlocksInfoRef<'a, T>,
    pub spray: Option<SprayRef<'a, T>>,
    pub crowd: Option<CrowdRef<'a, T>>,
    pub pfields: Option<PFieldsRef<'a>>,
    pub langs: IndexMap<u32, LangStringsRef<'a, T>>,
    pub files: IndexMap<u32, DataRef<'a>>
}

impl<T: BaseTypes> Default for SubBlocks2Ref<'_, T> {
    fn default() -> Self {
        Self {
            info: Default::default(),
            spray: None.into(),
            crowd: None.into(),
            pfields: None.into(),
            langs: IndexMap::default().into(),
            files: IndexMap::default().into()
        }
    }
}

impl<'a, T: BaseTypes> SubBlocks2Ref<'a, T> {
    // TODO should use the pak string to get key names
    // TODO add ref to pfeild infos in pfields struct
    pub fn from_data(src: &'a [u8], string_keys: &StringKeysRef<'a, T>) -> Result<Self> {
        use keys::*;
        let info = SubBlocksInfoRef::<T>::from_data(src)?;
        let mut spray = None;
        let mut crowd = None;
        let mut pfields = None;
        let mut langs = IndexMap::new();
        let mut files = IndexMap::new();
        for info in info.block_headers.iter() {
            let data = &src[info.offset.into() as usize.. (info.offset.into() + info.size.into()) as usize];
            match info.key.val.into() {
                KEY_POLISH | KEY_GERMAN | KEY_FRENCH | KEY_SPANISH | KEY_RUSSIAN | KEY_SWEDISH
                | KEY_ENGLISH | KEY_ITALIAN | KEY_NORWEGIAN => {
                    langs.insert(info.key.val.into(), LangStringsRef::from_data(data, string_keys)?);
                }
                KEY_SPRAY => {spray.replace(SprayRef::from_data(data)?);},
                KEY_CROWD => {crowd.replace(CrowdRef::from_data(data)?);},
                KEY_PFIELDS => {pfields.replace(PFieldsRef::from_data(data));},
                key => {
                    warn!("Unknown block type {:?}, {:?}", key, get_str(&key).map(|x| x.to_string()).unwrap_or_default());
                    files.insert(key, DataRef::from_data(data));
                }
            }
        }
        Ok(Self {
            info,
            spray: spray.into(),
            crowd: crowd.into(),
            pfields: pfields.into(),
            langs: langs.into(),
            files: files.into()
        })
    }
}

pub trait DumpSubBlocks2<T: BaseTypes> {
    fn files_num(&self) -> usize;
    fn langs_num(&self) -> usize;
    fn spray(&self) -> Option<&impl DumpSpray<T>>;
    fn crowd(&self) -> Option<&impl DumpCrowd<T>>;
    fn pfields(&self) -> Option<&impl DumpDataImpl>;
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpDataImpl)>;
    fn langs(&self) -> impl Iterator<Item = (u32, &impl DumpLangStrings<T>)>;
    
    fn blocks_num(&self) -> usize {
        self.files_num() + self.langs_num()
    }

    fn size(&self) -> (usize, Vec<u32>) {
        let spray = self.spray();
        let crowd = self.crowd();
        let pfields = self.pfields();
        let mut size = SubBlocksInfoRef::<T>::size(
            self.blocks_num() 
            + spray.is_some().then_some(1).unwrap_or_default() 
            + crowd.is_some().then_some(1).unwrap_or_default()
            + pfields.is_some().then_some(1).unwrap_or_default()
        );
        if let Some(spray) = spray {
            size = align_offset(size + spray.size() + 1, 16);
            debug!("post spray size {}", size);
        }
        if let Some(crowd) = crowd {
            size = align_offset(size + crowd.size() + 1, 16);
            debug!("post crowd size {}", size);
        }
        if let Some(pfields) = pfields {
            size = align_offset(size + pfields.size() + 1, 16);
            debug!("post pfields size {}", size);
        }
        let mut keys = HashSet::new();
        for (key, lang) in self.langs() {
            size = align_offset(size + lang.size(&mut keys) + 1, 16);
            debug!("post lang {} size {}, keys {}", key, size, keys.len());
        }
        let mut keys = keys.into_iter().collect::<Vec<_>>();
        keys.sort();
        for (key, file) in self.files() {
            size = align_offset(size + file.size() + 1, 16);
            debug!("post file {} size {}", key, size);
        }
        (size, keys)
    }
    fn dump_into(&self, dst: &mut DumpSlice, keys: &[u32]) -> Result<()> {
        let start = dst.offset;
        let spray = self.spray();
        let crowd = self.crowd();
        let pfields = self.pfields();
        let block_headers = SubBlocksInfoRef::<T>::dump(dst, 
            self.blocks_num() 
            + spray.is_some().then_some(1).unwrap_or_default() 
            + crowd.is_some().then_some(1).unwrap_or_default()
            + pfields.is_some().then_some(1).unwrap_or_default()
        )?;
        let mut i = 0;
        if let Some(spray) = spray {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = ((dst.offset - start) as u32).into();
            info.key = Crc::new(keys::KEY_SPRAY.into());
            let start = dst.offset;
            spray.dump_into(dst).context("spray")?;
            info.size = ((dst.offset - start) as u32).into();
            dst.split(1)?;
            dst.align(16)?;
            debug!("post spray offset {}", dst.offset);
        }
        if let Some(crowd) = crowd {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = ((dst.offset - start) as u32).into();
            info.key = Crc::new(keys::KEY_CROWD.into());
            let start = dst.offset;
            crowd.dump_into(dst).context("crowd")?;
            info.size = ((dst.offset - start) as u32).into();
            dst.split(1)?;
            dst.align(16)?;
            debug!("post crowd offset {}", dst.offset);
        }
        if let Some(pfields) = pfields {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = ((dst.offset - start) as u32).into();
            info.key = Crc::new(keys::KEY_PFIELDS.into());
            let start = dst.offset;
            pfields.dump_into(dst).context("pfields")?;
            info.size = ((dst.offset - start) as u32).into();
            dst.split(1)?;
            dst.align(16)?;
            debug!("post pfields offset {}", dst.offset);
        }
        for (key, lang) in self.langs() {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = ((dst.offset - start) as u32).into();
            info.key = Crc::new(key.into());
            let start = dst.offset;
            lang.dump_into(dst, keys).with_context(|| format!("lang {}", key))?;
            info.size = ((dst.offset - start) as u32).into();
            dst.split(1)?;
            dst.align(16)?;
            debug!("post lang {} offset {}", key, dst.offset);
        }
        for (key, file) in self.files() {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = ((dst.offset - start) as u32).into();
            info.key = Crc::new(key.into());
            let start = dst.offset;
            file.dump_into(dst).with_context(|| format!("file {}", key))?;
            info.size = ((dst.offset - start) as u32).into();
            dst.split(1)?;
            dst.align(16)?;
            debug!("post file {} offset {}", key, dst.offset);
        }
        Ok(())
    }
}

impl<T: BaseTypes> DumpSubBlocks2<T> for SubBlocks2Ref<'_, T> {
    fn files_num(&self) -> usize {
        self.files.len()
	}
    fn langs_num(&self) -> usize {
        self.langs.len()
	}
    fn spray(&self) -> Option<&impl DumpSpray<T>> {
        self.spray.as_ref()
	}
    fn crowd(&self) -> Option<&impl DumpCrowd<T>> {
        self.crowd.as_ref()
	}
    fn pfields(&self) -> Option<&impl DumpDataImpl> {
        self.pfields.as_ref()
	}
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpDataImpl)> {
        self.files.iter().map(|(k, v)| (*k, v))
	}
    fn langs(&self) -> impl Iterator<Item = (u32, &impl DumpLangStrings<T>)> {
        self.langs.iter().map(|(k, v)| (*k, v))
	}
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct LangStringsRef<'a, T: BaseTypes> {
    pub strings: IndexMap<u32, ref_slice<'a, T::u16>>,
}

impl<'a, T: BaseTypes> LangStringsRef<'a, T> {
    pub fn from_data(src: &'a [u8], string_keys: &StringKeysRef<'a, T>) -> Result<Self> {
        let mut offset = 0;
        let mut strings = IndexMap::with_capacity(string_keys.vals.len());
        for key in string_keys.vals.iter() {
            let start = offset;
            while src[offset] != 0 || src[offset + 1] != 0 {
                offset += 2;
            }
            let s = T::u16::slice_from_data(&src[start..offset], (offset - start) / 2)
                .with_context(|| format!("string {}", key.key.val.into()))?;
            strings.insert(key.key.val.into(), s.into());
            offset += 2;
        }
        Ok(Self {
            strings: strings.into(),
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct LangStrings {
    pub strings: IndexMap<Crc<NE>, String>,
}

fn parse_string<T: BaseTypes>(val: &ref_slice<'_, T::u16>) -> Result<String, std::string::FromUtf16Error> {
    String::from_utf16(
        val.iter()
            .map(|&y| y.into())
            .collect::<Vec<_>>()
            .as_ref(),
    )
}

impl<T: BaseTypes> TryFrom<&LangStringsRef<'_, T>> for LangStrings {
    type Error = std::string::FromUtf16Error;
    fn try_from(val: &LangStringsRef<T>) -> Result<Self, Self::Error> {
        Ok(Self {
            strings: val
                .strings
                .iter()
                .map(|(&k, v)| Ok((Crc::new(k.into()), parse_string::<T>(v)?)))
                .collect::<Result<_, Self::Error>>()?,
        })
    }
}

/*
#[enum_dispatch(DumpLangStrings_XE_)]
pub enum LangStrings_XE_<'a> {
    Ref(LangStringsRef_XE_<'a>),
    Owned(LangStrings)
}
*/


pub trait DumpLangStringsImpl<T: BaseTypes> {
    fn strings(&self) -> impl Iterator<Item = (u32, &impl DumpString<T>)>;
    fn get(&self, k: &u32) -> Option<&impl DumpString<T>>;
}

#[enum_dispatch]
pub trait DumpLangStrings<T: BaseTypes> {
    fn size(&self, keys: &mut HashSet<u32>) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice, keys: &[u32]) -> Result<()>;
}
impl<T: BaseTypes, I: DumpLangStringsImpl<T>> DumpLangStrings<T> for I {
    fn size(&self, keys: &mut HashSet<u32>) -> usize {
        self.strings()
            .map(|(k, v)| {
                keys.insert(k);
                v.string_size() + std::mem::size_of::<T::u16>()
            })
            .sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice, keys: &[u32]) -> Result<()> {
        debug!("keys {}", keys.len());
        for k in keys {
            if let Some(string) = self.get(k) {
                string.dump_string(dst).with_context(|| format!("string {}", k))?;
                *T::u16::mut_from_data(dst).with_context(|| format!("string pad {}", k))? = 0u16.into();
            }
        }
        Ok(())
    }
}
impl<T: BaseTypes> DumpLangStringsImpl<T> for LangStringsRef<'_, T> {
    fn strings(&self) -> impl Iterator<Item = (u32, &impl DumpString<T>)> {
        self.strings.iter().map(|(k, v)| (*k, v))
    }
    fn get(&self, k: &u32) -> Option<&impl DumpString<T>> {
        self.strings.get(k)
    }
}

impl<T: BaseTypes> DumpLangStringsImpl<T> for LangStrings {
    fn strings(&self) -> impl Iterator<Item = (u32, &impl DumpString<T>)> {
        self.strings.iter().map(|(k, v)| (k.val, v))
    }
    fn get(&self, k: &u32) -> Option<&impl DumpString<T>> {
        self.strings.get(&Crc::new(*k))
    }
}

#[derive_pod]
pub struct SprayInstance<T: BaseTypes> {
    pub key: Crc<T>,
    pub tex1: Crc<T>,
    pub tex2: Crc<T>,
    pub unk_3: T::u32,
    pub width: T::u32,
    pub height: T::u32,
    pub unk_6: T::f32,
    pub unk_7: T::f32,
    pub size_w: T::u32,
    pub size_h: T::u32,
    pub scale_w: T::f32,
    pub scale_h: T::f32,
    pub delay: T::u32,
    pub stride_x: T::f32,
    pub stride_y: T::f32,
    pub unk_15: T::u32,
    pub unk_16: T::u32,
}

#[derive_pod]
pub struct SprayVal<T: BaseTypes> {
    pub position: Vector3<T>,
    pub scale: T::f32,
    pub instance: T::u16,
    pub rotation: T::u16,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SprayRef<'a, T: BaseTypes> {
    pub instances: ref_slice<'a, SprayInstance<T>>,
    pub vals: ref_slice<'a, SprayVal<T>>,
}

impl<'a, T: BaseTypes> SprayRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let &n = T::u32::from_data(&src[offset..]).context("n1")?;
        offset += std::mem::size_of::<T::u32>();
        let instances = SprayInstance::slice_from_data(&src[offset..], n.into() as usize)
            .context("instances")?;
        offset += std::mem::size_of_val(instances);
        let &n = T::u32::from_data(&src[offset..]).context("n2")?;
        offset += std::mem::size_of::<T::u32>();
        let vals = SprayVal::slice_from_data(&src[offset..], n.into() as usize).context("vals")?;

        Ok(Self {
            instances: instances.into(),
            vals: vals.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Spray {
    pub instances: Vec<SprayInstance<NE>>,
    pub vals: Vec<SprayVal<NE>>,
}

impl<T: BaseTypes> From<&SprayRef<'_, T>> for Spray
where
    SprayInstance<NE>: From<SprayInstance<T>>,
    SprayVal<NE>: From<SprayVal<T>>,
{
    fn from(val: &SprayRef<T>) -> Self {
        Self {
            instances: val.instances.iter().map(|&x| x.into()).collect(),
            vals: val.vals.iter().map(|&x| x.into()).collect(),
        }
    }
}

/*
#[enum_dispatch(DumpSpray_XE_)]
pub enum Spray_XE_<'a> {
    Ref(SprayRef_XE_<'a>),
    Owned(Spray)
}
*/

#[enum_dispatch]
pub trait DumpSpray<T: BaseTypes> {
    fn instances_len(&self) -> usize;
    fn vals_len(&self) -> usize;
    fn write_instances(&self, instances: &mut [SprayInstance<T>]) -> Result<()>;
    fn write_vals(&self, vals: &mut [SprayVal<T>]) -> Result<()>;

    fn size(&self) -> usize {
        std::mem::size_of::<T::u32>() * 2
            + std::mem::size_of::<SprayInstance<T>>() * self.instances_len()
            + std::mem::size_of::<SprayVal<T>>() * self.vals_len()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let n = T::u32::mut_from_data(dst).context("n1")?;
        let instances = SprayInstance::mut_slice_from_data(dst, self.instances_len())
            .context("instances")?;
        *n = (instances.len() as u32).into();
        self.write_instances(instances).context("write instances")?;
        let n = T::u32::mut_from_data(dst).context("n1")?;
        let vals = SprayVal::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
        *n = (vals.len() as u32).into();
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
}

impl<T: BaseTypes> DumpSpray<T> for SprayRef<'_, T> {
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_instances(&self, instances: &mut [SprayInstance<T>]) -> Result<()> {
        instances.copy_from_slice(&self.instances[..]);
        Ok(())
    }
    fn write_vals(&self, vals: &mut [SprayVal<T>]) -> Result<()> {
        vals.copy_from_slice(&self.vals[..]);
        Ok(())
    }
}

impl<T: BaseTypes> DumpSpray<T> for Spray
where
    SprayInstance<T>: From<SprayInstance<NE>>,
    SprayVal<T>: From<SprayVal<NE>>,
{
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_instances(&self, instances: &mut [SprayInstance<T>]) -> Result<()> {
        for (&src, dst) in self.instances.iter().zip(instances) {
            *dst = src.into()
        }
        Ok(())
    }
    fn write_vals(&self, vals: &mut [SprayVal<T>]) -> Result<()> {
        for (&src, dst) in self.vals.iter().zip(vals) {
            *dst = src.into()
        }
        Ok(())
    }
}

#[derive_pod]
pub struct CrowdItemHeader<T: BaseTypes> {
    pub key: Crc<T>,
    pub key_main: Crc<T>,
    pub key_right: Crc<T>,
    pub key_left: Crc<T>,
    pub unk_4: T::f32,
    pub animation_num: T::u32,
    pub instance_num: T::u32,
}

#[derive_pod]
pub struct CrowdVal<T: BaseTypes> {
    pub position: Vector3<T>,
    pub rotation: T::f32,
    pub lod: T::f32,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct CrowdItemRef<'a, T: BaseTypes> {
    pub header: &'a CrowdItemHeader<T>,
    pub animations: ref_slice<'a, Crc<T>>,
    pub instances: ref_slice<'a, CrowdVal<T>>
}

impl<'a, T: BaseTypes> CrowdItemRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = CrowdItemHeader::<T>::from_data(&src[offset..]).context("header")?;
        offset += std::mem::size_of::<CrowdItemHeader<T>>();
        let animations =
            Crc::<T>::slice_from_data(&src[offset..], header.animation_num.into() as usize)
                .context("animations")?;
        offset += std::mem::size_of_val(animations);
        let instances =
            CrowdVal::slice_from_data(&src[offset..], header.instance_num.into() as usize)
                .context("instances")?;
        Ok(Self {
            header: header,
            animations: animations.into(),
            instances: instances.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CrowdItem {
    pub header: CrowdItemHeader<NE>,
    pub animations: Vec<Crc<NE>>,
    pub instances: Vec<CrowdVal<NE>>,
}

impl<T: BaseTypes> From<&CrowdItemRef<'_, T>> for CrowdItem
where
    CrowdItemHeader<NE>: From<CrowdItemHeader<T>>,
    Crc<NE>: From<Crc<T>>,
    CrowdVal<NE>: From<CrowdVal<T>>,
{
    fn from(val: &CrowdItemRef<T>) -> Self {
        Self {
            header: (*val.header).into(),
            animations: val.animations.iter().map(|&x| x.into()).collect(),
            instances: val.instances.iter().map(|&x| x.into()).collect(),
        }
    }
}

/*
#[enum_dispatch(DumpCrowdItem_XE_)]
pub enum CrowdItem_XE_<'a> {
    Ref(CrowdItemRef_XE_<'a>),
    Owned(CrowdItem)
}
*/

#[enum_dispatch]
pub trait DumpCrowdItem<T: BaseTypes> {
    fn animations_len(&self) -> usize;
    fn instances_len(&self) -> usize;
    fn write_header(&self, header: &mut CrowdItemHeader<T>) -> Result<()>;
    fn write_animations(&self, animations: &mut [Crc<T>]) -> Result<()>;
    fn write_instances(&self, instances: &mut [CrowdVal<T>]) -> Result<()>;

    fn size(&self) -> usize {
        std::mem::size_of::<CrowdItemHeader<T>>()
            + std::mem::size_of::<Crc<T>>() * self.animations_len()
            + std::mem::size_of::<CrowdVal<T>>() * self.instances_len()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let header = CrowdItemHeader::mut_from_data(dst).context("header")?;
        self.write_header(header).context("write header")?;
        header.animation_num = (self.animations_len() as u32).into();
        header.instance_num = (self.instances_len() as u32).into();
        let animations =
            Crc::mut_slice_from_data(dst, self.animations_len()).context("animations")?;
        self.write_animations(animations)
            .context("write animations")?;
        let instances =
            CrowdVal::mut_slice_from_data(dst, self.instances_len()).context("instances")?;
        self.write_instances(instances).context("write instances")?;
        Ok(())
    }
}

impl<T: BaseTypes> DumpCrowdItem<T> for CrowdItemRef<'_, T> {
    fn animations_len(&self) -> usize {
        self.animations.len()
    }
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn write_header(&self, header: &mut CrowdItemHeader<T>) -> Result<()> {
        *header = *self.header;
        Ok(())
    }
    fn write_instances(&self, instances: &mut [CrowdVal<T>]) -> Result<()> {
        instances.copy_from_slice(&self.instances[..]);
        Ok(())
    }
    fn write_animations(&self, animations: &mut [Crc<T>]) -> Result<()> {
        animations.copy_from_slice(&self.animations[..]);
        Ok(())
    }
}

impl<T: BaseTypes> DumpCrowdItem<T> for CrowdItem
where
    CrowdItemHeader<T>: From<CrowdItemHeader<NE>>,
    CrowdVal<T>: From<CrowdVal<NE>>,
    Crc<T>: From<Crc<NE>>
{
    fn animations_len(&self) -> usize {
        self.animations.len()
    }
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn write_header(&self, header: &mut CrowdItemHeader<T>) -> Result<()> {
        *header = self.header.into();
        Ok(())
    }
    fn write_instances(&self, instances: &mut [CrowdVal<T>]) -> Result<()> {
        for (&src, dst) in self.instances.iter().zip(instances) {
            *dst = src.into();
        }
        Ok(())
    }
    fn write_animations(&self, animations: &mut [Crc<T>]) -> Result<()> {
        for (&src, dst) in self.animations.iter().zip(animations) {
            *dst = src.into();
        }
        Ok(())
    }
}

#[derive_pod]
pub struct CrowdHeader<T: BaseTypes> {
    pub const0x65: T::u32,
    pub n: T::u32,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct CrowdRef<'a, T: BaseTypes>{
    pub header: &'a CrowdHeader<T>,
    pub offs: ref_slice<'a, T::u32>,
    pub vals: slice<CrowdItemRef<'a, T>>
}

impl<'a, T: BaseTypes> CrowdRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let header = CrowdHeader::<T>::from_data(src).context("header")?;
        if header.const0x65.into() != 0x65 {
            return Err(anyhow!("Invalid Block Data for Crowd Block"));
        }
        let offs = T::u32::slice_from_data(&src[std::mem::size_of::<CrowdHeader<T>>()..], header.n.into() as usize)
            .context("offs")?;
        let vals = offs
            .into_iter()
            .enumerate()
            .map(|(i, &off)| {
                CrowdItemRef::from_data(&src[off.into() as usize..])
                    .with_context(|| format!("item {}", i))
            })
            .collect::<Result<Vec<_>>>()?.into_boxed_slice();

        Ok(Self {
            header,
            offs: offs.into(),
            vals: vals.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Crowd {
    pub vals: Vec<CrowdItem>,
}

impl<T: BaseTypes> From<&CrowdRef<'_, T>> for Crowd
where
    CrowdItemHeader<NE>: From<CrowdItemHeader<T>>,
    Crc<NE>: From<Crc<T>>,
    CrowdVal<NE>: From<CrowdVal<T>>,
{
    fn from(val: &CrowdRef<T>) -> Self {
        Self {
            vals: val.vals.iter().map(|x| x.into()).collect(),
        }
    }
}

/*
pub struct CrowdImpl_XE_<'a> {
    pub vals: Vec<CrowdItem_XE_<'a>>
}

impl<'a> From<CrowdRef_XE_<'a>> for CrowdImpl_XE_<'a> {
    fn from(val: CrowdRef_XE_<'a>) -> Self {
        Self {
            vals: Box::<[_]>::from(val.vals).into_vec().into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[enum_dispatch(DumpCrowd_XE_)]
pub enum Crowd_XE_<'a> {
    Ref(CrowdRef_XE_<'a>),
    Owned(CrowdImpl_XE_<'a>)
}

impl DumpCrowdImpl_XE_ for CrowdImpl_XE_<'_> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItem_XE_> {
        self.vals.iter()
    }
}
*/

pub trait DumpCrowdImpl<T: BaseTypes> {
    fn vals_len(&self) -> usize;
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItem<T>>;
}

#[enum_dispatch]
pub trait DumpCrowd<T: BaseTypes> {
    fn size(&self) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
}

impl<T: BaseTypes, I: DumpCrowdImpl<T>> DumpCrowd<T> for I {
    fn size(&self) -> usize {
        std::mem::size_of::<CrowdHeader<T>>()
            + std::mem::size_of::<T::u32>() * self.vals_len()
            + self.vals().map(|x| x.size()).sum::<usize>()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let start = dst.offset;
        let header = CrowdHeader::<T>::mut_from_data(dst).context("header")?;
        let offs = T::u32::mut_slice_from_data(dst, self.vals_len()).context("offs")?;
        header.n = (offs.len() as u32).into();
        header.const0x65 = 0x65u32.into();

        for (i, (val, off)) in self.vals().zip(offs).enumerate() {
            *off = ((dst.offset - start) as u32).into();
            val.dump_into(dst).with_context(|| format!("val {}", i))?;
        }
        Ok(())
    }
}

impl<T: BaseTypes> DumpCrowdImpl<T> for CrowdRef<'_, T> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItem<T>> {
        self.vals.iter()
    }
}

impl<T: BaseTypes> DumpCrowdImpl<T> for Crowd
where
    CrowdItemHeader<T>: From<CrowdItemHeader<NE>>,
    CrowdVal<T>: From<CrowdVal<NE>>,
    Crc<T>: From<Crc<NE>>
{
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItem<T>> {
        self.vals.iter()
    }
}


pub type PFieldsRef<'a> = DataRef<'a>;

#[derive(Debug, Default, Clone)]
pub struct PField {
    pub link_guid: u32,
    pub vals: Vec<(HashSet<u32>, Vec<u8>)>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default, Clone)]
pub struct PFields {
    pub vals: IndexMap<u32, PField>,
}

/*
#[enum_dispatch(DumpData_XE_)]
pub enum PFields_XE_<'a> {
    Ref(PFieldsRef_XE_<'a>),
    Owned(PFields)
}
*/

impl DumpDataImpl for PFields { 
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        for (i, pfield) in self.vals.values().enumerate() {
            for (j, (_, vals)) in pfield.vals.iter().enumerate() {
                let dst = u8::mut_slice_from_data(dst, vals.len()).with_context(|| format!("pfield {} val {}", i, j))?;
                dst.copy_from_slice(vals); 
            }
        }
        Ok(())
    }
    fn size(&self) -> usize {
        self.vals
            .values()
            .flat_map(|x| x.vals.iter())
            .map(|(_, vals)| std::mem::size_of_val(vals))
            .sum::<usize>()
    }
}

impl PFields {
    pub fn from<T: BaseTypes>(val: &PFieldsRef, infos: &[PFieldInfo<T>]) -> Self {
        let mut offset_maps: HashMap<u32, HashMap<u32, usize>> = HashMap::new();
        let mut pfields = IndexMap::new();
        let data = &val.data[..];
        for info in infos {
            let link_guid = info.link_guid.into();
            let offset = info.offset.into();
            let gamemode_guid = info.gamemode_guid.into();
            let width = info.width.into();
            let height = info.height.into();
            if !pfields.contains_key(&link_guid) {
                pfields.insert(
                    link_guid,
                    PField {
                        link_guid,
                        width,
                        height,
                        ..Default::default()
                    },
                );
                offset_maps.insert(link_guid, HashMap::new());
            }
            let val = pfields.get_mut(&link_guid).unwrap();
            let offset_map = offset_maps.get_mut(&link_guid).unwrap();
            if let Some(&index) = offset_map.get(&offset) {
                val.vals[index].0.insert(gamemode_guid);
            } else {
                offset_map.insert(offset, val.vals.len());
                let mut gamemodes = HashSet::new();
                gamemodes.insert(gamemode_guid);
                let offset = offset as usize;
                let size = (width * height) as usize;
                let vals = data[offset..offset + size].to_vec();
                val.vals.push((gamemodes, vals));
            }
        }
        Self { vals: pfields }
    }

    pub fn infos<T: BaseTypes>(&self) -> Vec<PFieldInfo<T>> {
        let mut infos = vec![];
        let mut offset = 0u32;
        for pfield in self.vals.values() {
            for (gamemodes, _) in &pfield.vals {
                for &gamemode_guid in gamemodes {
                    infos.push(PFieldInfo {
                        link_guid: pfield.link_guid.into(),
                        width: pfield.width.into(),
                        height: pfield.height.into(),
                        gamemode_guid: gamemode_guid.into(),
                        offset: offset.into(),
                    });
                }
                offset += pfield.width * pfield.height;
            }
        }
        infos
    }
}
