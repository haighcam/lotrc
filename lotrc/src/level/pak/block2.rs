use crate::{
    level::pak::{PakTypes, PakHeaderTypeTrait},
    types::{
        get_str, hash_string, Crc, DumpData, DumpSlice, RefFromData, Vector3, OrderedData, align_offset, ref_slice, slice, EndianTypes, 
        sub_blocks::{
            DataRef, SubBlockTypes, StringKeysRef, SubBlocksInfoRef, StringKeysValTypeTrait , SubBlocksBlockHeaderTypeTrait
        }
    }
};
#[make_endian]
use crate::{
    level::pak::{
        PakHeader_XE_,
        block1::{
            objs::PFieldInfo_XE_,
        }
    },
    types::{
        Crc_XE_, u16_XE_, u32_XE_, f32_XE_, Vector3_XE_,
        sub_blocks::{
            SubBlocksInfoRef_XE_, DataRef_XE_, DumpData_XE_, DumpString_XE_, StringKeysRef_XE_
        },
    },
};
use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use log::{warn, debug};
use lotrc_proc::{make_endian, derive_ordered_data};
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

pub trait Block2Types: SubBlockTypes {
    type SprayInstance: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + SprayInstanceTypeTrait;
    type SprayVal: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + SprayValTypeTrait;
    type CrowdItemHeader: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + CrowdItemHeaderTypeTrait;
    type CrowdVal: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + CrowdValTypeTrait;
    type CrowdHeader: std::fmt::Debug + Clone + PartialEq + RefFromData + DumpData + CrowdHeaderTypeTrait;
}

#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Block2Ref<'a, T: Block2Types> {
    pub sub_blocks: SubBlocks2Ref<'a, T>,
    pub offsets: ref_slice<'a, T::u32>
}

impl<'a, T: Block2Types + PakTypes> Block2Ref<'a, T> {
    pub fn from_data(src: &'a [u8], pak_header: &T::PakHeader, string_keys: &StringKeysRef<'a, T>) -> Result<Self> {
        let t = std::time::Instant::now();
        let sub_blocks = SubBlocks2Ref::from_data(
            &src[pak_header.sub_blocks2_offset() as usize..],
            string_keys
        )
        .context("sub_blocks")?;
        debug!("Block2 sub_blocks parsed in {}", t.elapsed().as_secs_f32());
        let t = std::time::Instant::now();
        let offsets = T::u32::slice_from_data(
            &src[pak_header.block2_offsets_offset() as usize..],
            pak_header.block2_offsets_num() as usize,
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

#[make_endian]
#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Block2Ref_XE_<'a> {
    pub sub_blocks: SubBlocks2Ref_XE_<'a>,
    pub offsets: ref_slice<'a, u32_XE_>
}

#[make_endian]
impl<'a> Block2Ref_XE_<'a> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeader_XE_, string_keys: &StringKeysRef_XE_) -> Result<Self> {
        let t = std::time::Instant::now();
        let sub_blocks = SubBlocks2Ref_XE_::from_data(
            &src[pak_header.sub_blocks2_offset.conv()..],
            string_keys
        )
        .context("sub_blocks")?;
        debug!("Block2 sub_blocks parsed in {}", t.elapsed().as_secs_f32());
        let t = std::time::Instant::now();
        let offsets = u32_XE_::slice_from_data(
            &src[pak_header.block2_offsets_offset.conv()..],
            pak_header.block2_offsets_num.conv(),
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

#[make_endian]
pub trait DumpBlock2_XE_ {
    fn sub_blocks(&self) -> &impl DumpSubBlocks2_XE_;
    fn dump<'a>(&self, dst: &mut DumpSlice<'a>, offset_num: usize, keys: &[u32], header: &mut PakHeader_XE_) -> Result<&'a mut [u32_XE_]> {
        header.sub_blocks2_offset = dst.offset.conv();
        self.sub_blocks().dump_into(dst, keys).context("sub_blocks")?;
        header.block2_offsets_offset = dst.offset.conv();
        let offsets = u32_XE_::mut_slice_from_data(dst, offset_num).context("offsets")?;
        header.block2_offsets_num = offsets.len().conv();
        Ok(offsets)
    }
}

#[make_endian]
impl DumpBlock2_XE_ for Block2Ref_XE_<'_> {
    fn sub_blocks(&self) -> &impl DumpSubBlocks2_XE_ {
        &self.sub_blocks
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SubBlocks2Ref<'a, T: Block2Types> {
    pub info: SubBlocksInfoRef<'a, T>,
    pub spray: Option<SprayRef<'a, T>>,
    pub crowd: Option<CrowdRef<'a, T>>,
    pub pfields: Option<PFieldsRef<'a>>,
    pub langs: IndexMap<u32, LangStringsRef<'a, T>>,
    pub files: IndexMap<u32, DataRef<'a>>
}

impl<T: Block2Types> Default for SubBlocks2Ref<'_, T> {
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

impl<'a, T: Block2Types> SubBlocks2Ref<'a, T> {
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
            let data = &src[info.offset() as usize.. (info.offset() + info.size()) as usize];
            match info.key().get() {
                KEY_POLISH | KEY_GERMAN | KEY_FRENCH | KEY_SPANISH | KEY_RUSSIAN | KEY_SWEDISH
                | KEY_ENGLISH | KEY_ITALIAN | KEY_NORWEGIAN => {
                    langs.insert(info.key().get(), LangStringsRef::from_data(data, string_keys)?);
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

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SubBlocks2Ref_XE_<'a> {
    pub info: SubBlocksInfoRef_XE_<'a>,
    pub spray: Option<SprayRef_XE_<'a>>,
    pub crowd: Option<CrowdRef_XE_<'a>>,
    pub pfields: Option<PFieldsRef_XE_<'a>>,
    pub langs: IndexMap<u32, LangStringsRef_XE_<'a>>,
    pub files: IndexMap<u32, DataRef_XE_<'a>>
}

#[make_endian]
impl Default for SubBlocks2Ref_XE_<'_> {
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

#[make_endian]
impl<'a> SubBlocks2Ref_XE_<'a> {
    // TODO should use the pak string to get key names
    // TODO add ref to pfeild infos in pfields struct
    pub fn from_data(src: &'a [u8], string_keys: &StringKeysRef_XE_) -> Result<Self> {
        use keys::*;
        let info = SubBlocksInfoRef_XE_::from_data(src)?;
        let mut spray = None;
        let mut crowd = None;
        let mut pfields = None;
        let mut langs = IndexMap::new();
        let mut files = IndexMap::new();
        for info in info.block_headers.iter() {
            let data = &src[info.offset.conv() .. (info.offset + info.size) as usize];
            match info.key.conv() {
                KEY_POLISH | KEY_GERMAN | KEY_FRENCH | KEY_SPANISH | KEY_RUSSIAN | KEY_SWEDISH
                | KEY_ENGLISH | KEY_ITALIAN | KEY_NORWEGIAN => {
                    langs.insert(info.key.conv(), LangStringsRef_XE_::from_data(data, string_keys)?);
                }
                KEY_SPRAY => {spray.replace(SprayRef_XE_::from_data(data)?);},
                KEY_CROWD => {crowd.replace(CrowdRef_XE_::from_data(data)?);},
                KEY_PFIELDS => {pfields.replace(PFieldsRef_XE_::from_data(data));},
                key => {
                    warn!("Unknown block type {:?}, {:?}", key, get_str(&key).map(|x| x.to_string()).unwrap_or_default());
                    files.insert(key, DataRef_XE_::from_data(data));
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

#[make_endian]
pub trait DumpSubBlocks2_XE_ {
    fn files_num(&self) -> usize;
    fn langs_num(&self) -> usize;
    fn spray(&self) -> Option<&impl DumpSpray_XE_>;
    fn crowd(&self) -> Option<&impl DumpCrowd_XE_>;
    fn pfields(&self) -> Option<&impl DumpData_XE_>;
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpData_XE_)>;
    fn langs(&self) -> impl Iterator<Item = (u32, &impl DumpLangStrings_XE_)>;
    
    fn blocks_num(&self) -> usize {
        self.files_num() + self.langs_num()
    }

    fn size(&self) -> (usize, Vec<u32>) {
        let spray = self.spray();
        let crowd = self.crowd();
        let pfields = self.pfields();
        let mut size = SubBlocksInfoRef_XE_::size(
            self.blocks_num() 
            + spray.is_some().then_some(1).unwrap_or_default() 
            + crowd.is_some().then_some(1).unwrap_or_default()
            + pfields.is_some().then_some(1).unwrap_or_default()
        );
        if let Some(spray) = spray {
            size = align_offset(size + spray.size() + 1, 16)
        }
        if let Some(crowd) = crowd {
            size = align_offset(size + crowd.size() + 1, 16)
        }
        if let Some(pfields) = pfields {
            size = align_offset(size + pfields.size() + 1, 16)
        }
        let mut keys = HashSet::new();
        for (_, lang) in self.langs() {
            size = align_offset(size + lang.size(&mut keys) + 1, 16)
        }
        let mut keys = keys.into_iter().collect::<Vec<_>>();
        keys.sort();
        for (_, file) in self.files() {
            size = align_offset(size + file.size() + 1, 16)
        }
        (size, keys)
    }
    fn dump_into(&self, dst: &mut DumpSlice, keys: &[u32]) -> Result<()> {
        let start = dst.offset;
        let spray = self.spray();
        let crowd = self.crowd();
        let pfields = self.pfields();
        let block_headers = SubBlocksInfoRef_XE_::dump(dst, 
            self.blocks_num() 
            + spray.is_some().then_some(1).unwrap_or_default() 
            + crowd.is_some().then_some(1).unwrap_or_default()
            + pfields.is_some().then_some(1).unwrap_or_default()
        )?;
        let mut i = 0;
        if let Some(spray) = spray {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = (dst.offset - start).conv();
            info.key = keys::KEY_SPRAY.conv();
            let start = dst.offset;
            spray.dump_into(dst)?;
            info.size = (dst.offset - start).conv();
            dst.split(1)?;
            dst.align(16)?;
        }
        if let Some(crowd) = crowd {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = (dst.offset - start).conv();
            info.key = keys::KEY_CROWD.conv();
            let start = dst.offset;
            crowd.dump_into(dst)?;
            info.size = (dst.offset - start).conv();
            dst.split(1)?;
            dst.align(16)?;
        }
        if let Some(pfields) = pfields {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = (dst.offset - start).conv();
            info.key = keys::KEY_PFIELDS.conv();
            let start = dst.offset;
            pfields.dump_into(dst)?;
            info.size = (dst.offset - start).conv();
            dst.split(1)?;
            dst.align(16)?;
        }
        for (key, lang) in self.langs() {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = (dst.offset - start).conv();
            info.key = key.conv();
            let start = dst.offset;
            lang.dump_into(dst, keys)?;
            info.size = (dst.offset - start).conv();
            dst.split(1)?;
            dst.align(16)?;
        }
        for (key, file) in self.files() {
            let info = &mut block_headers[i];
            i += 1;
            info.offset = (dst.offset - start).conv();
            info.key = key.conv();
            let start = dst.offset;
            file.dump_into(dst)?;
            info.size = (dst.offset - start).conv();
            dst.split(1)?;
            dst.align(16)?;
        }
        Ok(())
    }
}

#[make_endian]
impl DumpSubBlocks2_XE_ for SubBlocks2Ref_XE_<'_> {
    fn files_num(&self) -> usize {
        self.files.len()
	}
    fn langs_num(&self) -> usize {
        self.langs.len()
	}
    fn spray(&self) -> Option<&impl DumpSpray_XE_> {
        self.spray.as_ref()
	}
    fn crowd(&self) -> Option<&impl DumpCrowd_XE_> {
        self.crowd.as_ref()
	}
    fn pfields(&self) -> Option<&impl DumpData_XE_> {
        self.pfields.as_ref()
	}
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpData_XE_)> {
        self.files.iter().map(|(k, v)| (*k, v))
	}
    fn langs(&self) -> impl Iterator<Item = (u32, &impl DumpLangStrings_XE_)> {
        self.langs.iter().map(|(k, v)| (*k, v))
	}
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct LangStringsRef<'a, T: SubBlockTypes> {
    pub strings: IndexMap<u32, ref_slice<'a, T::u16>>,
}

impl<'a, T: SubBlockTypes> LangStringsRef<'a, T> {
    pub fn from_data(src: &'a [u8], string_keys: &StringKeysRef<'a, T>) -> Result<Self> {
        let mut offset = 0;
        let mut strings = IndexMap::with_capacity(string_keys.vals.len());
        for key in string_keys.vals.iter() {
            let start = offset;
            while src[offset] != 0 || src[offset + 1] != 0 {
                offset += 2;
            }
            let s = T::u16::slice_from_data(&src[start..offset], (offset - start) / 2)
                .with_context(|| format!("string {}", strings.len()))?;
            strings.insert(key.key().get(), s.into());
            offset += 2;
        }
        Ok(Self {
            strings: strings.into(),
        })
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct LangStringsRef_XE_<'a> {
    pub strings: IndexMap<u32, ref_slice<'a, u16_XE_>>,
}

#[make_endian]
impl<'a> LangStringsRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], string_keys: &StringKeysRef_XE_) -> Result<Self> {
        let mut offset = 0;
        let mut strings = IndexMap::with_capacity(string_keys.vals.len());
        for key in string_keys.vals.iter() {
            let start = offset;
            while src[offset] != 0 || src[offset + 1] != 0 {
                offset += 2;
            }
            let s = u16_XE_::slice_from_data(&src[start..offset], (offset - start) / 2)
                .with_context(|| format!("string {}", strings.len()))?;
            strings.insert(key.key.conv(), s.into());
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
    pub strings: IndexMap<Crc, String>,
}

#[make_endian]
fn parse_string_xe_(val: &ref_slice<'_, u16_XE_>) -> Result<String, std::string::FromUtf16Error> {
    String::from_utf16(
        val.iter()
            .map(|y| y.conv())
            .collect::<Vec<_>>()
            .as_ref(),
    )
}

#[make_endian]
impl TryFrom<&LangStringsRef_XE_<'_>> for LangStrings {
    type Error = std::string::FromUtf16Error;
    fn try_from(val: &LangStringsRef_XE_) -> Result<Self, Self::Error> {
        Ok(Self {
            strings: val
                .strings
                .iter()
                .map(|(k, v)| Ok(((*k).into(), parse_string_xe_(v)?)))
                .collect::<Result<_, Self::Error>>()?,
        })
    }
}

#[make_endian]
#[enum_dispatch(DumpLangStrings_XE_)]
pub enum LangStrings_XE_<'a> {
    Ref(LangStringsRef_XE_<'a>),
    Owned(LangStrings)
}


#[make_endian]
pub trait DumpLangStringsImpl_XE_ {
    fn strings(&self) -> impl Iterator<Item = (u32, &impl DumpString_XE_)>;
    fn get(&self, k: &u32) -> Option<&impl DumpString_XE_>;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpLangStrings_XE_ {
    fn size(&self, keys: &mut HashSet<u32>) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice, keys: &[u32]) -> Result<()>;
}
#[make_endian]
impl<T: DumpLangStringsImpl_XE_> DumpLangStrings_XE_ for T {
    fn size(&self, keys: &mut HashSet<u32>) -> usize {
        self.strings()
            .map(|(k, v)| {
                keys.insert(k);
                v.string_size() + u16_XE_::size_of()
            })
            .sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice, keys: &[u32]) -> Result<()> {
        for k in keys {
            if let Some(string) = self.get(k) {
                string.dump_string(dst).with_context(|| format!("string {}", k))?;
            }
            u16_XE_::from(0u16)
                .dump_into(dst)
                .with_context(|| format!("string pad {}", k))?;
        }
        Ok(())
    }
}
#[make_endian]
impl DumpLangStringsImpl_XE_ for LangStringsRef_XE_<'_> {
    fn strings(&self) -> impl Iterator<Item = (u32, &impl DumpString_XE_)> {
        self.strings.iter().map(|(k, v)| (*k, v))
    }
    fn get(&self, k: &u32) -> Option<&impl DumpString_XE_> {
        self.strings.get(k)
    }
}

#[make_endian]
impl DumpLangStringsImpl_XE_ for LangStrings {
    fn strings(&self) -> impl Iterator<Item = (u32, &impl DumpString_XE_)> {
        self.strings.iter().map(|(k, v)| (k.get(), v))
    }
    fn get(&self, k: &u32) -> Option<&impl DumpString_XE_> {
        self.strings.get(k)
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SprayInstance_XE_ {
    pub key: Crc_XE_,
    pub tex1: Crc_XE_,
    pub tex2: Crc_XE_,
    pub unk_3: u32_XE_,
    pub width: u32_XE_,
    pub height: u32_XE_,
    pub unk_6: f32_XE_,
    pub unk_7: f32_XE_,
    pub size_w: u32_XE_,
    pub size_h: u32_XE_,
    pub scale_w: f32_XE_,
    pub scale_h: f32_XE_,
    pub delay: u32_XE_,
    pub stride_x: f32_XE_,
    pub stride_y: f32_XE_,
    pub unk_15: u32_XE_,
    pub unk_16: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SprayVal_XE_ {
    pub position: Vector3_XE_,
    pub scale: f32_XE_,
    pub instance: u16_XE_,
    pub rotation: u16_XE_,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SprayRef<'a, T: Block2Types> {
    pub instances: ref_slice<'a, T::SprayInstance>,
    pub vals: ref_slice<'a, T::SprayVal>,
}

#[make_endian]
impl<'a, T: Block2Types> SprayRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let n = T::u32::from_data(&src[offset..]).context("n1")?;
        offset += n.size_of_val();
        let instances = T::SprayInstance::slice_from_data(&src[offset..], n.conv() as usize)
            .context("instances")?;
        offset += instances.size_of_val();
        let n = T::u32::from_data(&src[offset..]).context("n2")?;
        offset += n.size_of_val();
        let vals = T::SprayVal::slice_from_data(&src[offset..], n.conv() as usize).context("vals")?;

        Ok(Self {
            instances: instances.into(),
            vals: vals.into(),
        })
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SprayRef_XE_<'a> {
    pub instances: ref_slice<'a, SprayInstance_XE_>,
    pub vals: ref_slice<'a, SprayVal_XE_>,
}

#[make_endian]
impl<'a> SprayRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let n = u32_XE_::from_data(&src[offset..]).context("n1")?;
        offset += n.size_of_val();
        let instances = SprayInstance_XE_::slice_from_data(&src[offset..], n.conv())
            .context("instances")?;
        offset += instances.size_of_val();
        let n = u32_XE_::from_data(&src[offset..]).context("n2")?;
        offset += n.size_of_val();
        let vals =
            SprayVal_XE_::slice_from_data(&src[offset..], n.conv()).context("vals")?;

        Ok(Self {
            instances: instances.into(),
            vals: vals.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Spray {
    pub instances: Vec<SprayInstance>,
    pub vals: Vec<SprayVal>,
}

#[make_endian]
impl From<&SprayRef_XE_<'_>> for Spray {
    fn from(val: &SprayRef_XE_) -> Self {
        Self {
            instances: val.instances.iter().map(|x| x.conv()).collect(),
            vals: val.vals.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_endian]
#[enum_dispatch(DumpSpray_XE_)]
pub enum Spray_XE_<'a> {
    Ref(SprayRef_XE_<'a>),
    Owned(Spray)
}

#[make_endian]
#[enum_dispatch]
pub trait DumpSpray_XE_ {
    fn instances_len(&self) -> usize;
    fn vals_len(&self) -> usize;
    fn write_instances(&self, instances: &mut [SprayInstance_XE_]) -> Result<()>;
    fn write_vals(&self, vals: &mut [SprayVal_XE_]) -> Result<()>;

    fn size(&self) -> usize {
        u32_XE_::size_of() * 2
            + SprayInstance_XE_::size_of() * self.instances_len()
            + SprayVal_XE_::size_of() * self.vals_len()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let n = u32_XE_::mut_from_data(dst).context("n1")?;
        let instances = SprayInstance_XE_::mut_slice_from_data(dst, self.instances_len())
            .context("instances")?;
        *n = instances.len().conv();
        self.write_instances(instances).context("write instances")?;
        let n = u32_XE_::mut_from_data(dst).context("n1")?;
        let vals = SprayVal_XE_::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
        *n = vals.len().conv();
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
}

#[make_endian]
impl DumpSpray_XE_ for SprayRef_XE_<'_> {
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_instances(&self, instances: &mut [SprayInstance_XE_]) -> Result<()> {
        instances.write_from(&self.instances[..])
    }
    fn write_vals(&self, vals: &mut [SprayVal_XE_]) -> Result<()> {
        vals.write_from(&self.vals[..])
    }
}

#[make_endian]
impl DumpSpray_XE_ for Spray {
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_instances(&self, instances: &mut [SprayInstance_XE_]) -> Result<()> {
        for (src, dst) in self.instances.iter().zip(instances) {
            *dst = src.conv()
        }
        Ok(())
    }
    fn write_vals(&self, vals: &mut [SprayVal_XE_]) -> Result<()> {
        for (src, dst) in self.vals.iter().zip(vals) {
            *dst = src.conv()
        }
        Ok(())
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CrowdItemHeader_XE_ {
    pub key: Crc_XE_,
    pub key_main: Crc_XE_,
    pub key_right: Crc_XE_,
    pub key_left: Crc_XE_,
    pub unk_4: f32_XE_,
    pub animation_num: u32_XE_,
    pub instance_num: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CrowdVal_XE_ {
    pub position: Vector3_XE_,
    pub rotation: f32_XE_,
    pub lod: f32_XE_,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct CrowdItemRef<'a, T: Block2Types> {
    pub header: &'a T::CrowdItemHeader,
    pub animations: ref_slice<'a, T::Crc>,
    pub instances: ref_slice<'a, T::CrowdVal>
}

impl<'a, T: Block2Types> CrowdItemRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = T::CrowdItemHeader::from_data(&src[offset..]).context("header")?;
        offset += header.size_of_val();
        let animations =
            T::Crc::slice_from_data(&src[offset..], header.animation_num() as usize)
                .context("animations")?;
        offset += animations.size_of_val();
        let instances =
            T::CrowdVal::slice_from_data(&src[offset..], header.instance_num() as usize)
                .context("instances")?;
        Ok(Self {
            header: header,
            animations: animations.into(),
            instances: instances.into(),
        })
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct CrowdItemRef_XE_<'a> {
    pub header: &'a CrowdItemHeader_XE_,
    pub animations: ref_slice<'a, Crc_XE_>,
    pub instances: ref_slice<'a, CrowdVal_XE_>
}

#[make_endian]
impl<'a> CrowdItemRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = CrowdItemHeader_XE_::from_data(&src[offset..]).context("header")?;
        offset += header.size_of_val();
        let animations =
            Crc_XE_::slice_from_data(&src[offset..], header.animation_num.conv())
                .context("animations")?;
        offset += animations.size_of_val();
        let instances =
            CrowdVal_XE_::slice_from_data(&src[offset..], header.instance_num.conv())
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
    pub header: CrowdItemHeader,
    pub animations: Vec<Crc>,
    pub instances: Vec<CrowdVal>,
}

#[make_endian]
impl From<&CrowdItemRef_XE_<'_>> for CrowdItem {
    fn from(val: &CrowdItemRef_XE_) -> Self {
        Self {
            header: val.header.conv(),
            animations: val.animations.iter().map(|x| x.conv()).collect(),
            instances: val.instances.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_endian]
#[enum_dispatch(DumpCrowdItem_XE_)]
pub enum CrowdItem_XE_<'a> {
    Ref(CrowdItemRef_XE_<'a>),
    Owned(CrowdItem)
}

#[make_endian]
#[enum_dispatch]
pub trait DumpCrowdItem_XE_ {
    fn animations_len(&self) -> usize;
    fn instances_len(&self) -> usize;
    fn write_header(&self, header: &mut CrowdItemHeader_XE_) -> Result<()>;
    fn write_animations(&self, animations: &mut [Crc_XE_]) -> Result<()>;
    fn write_instances(&self, instances: &mut [CrowdVal_XE_]) -> Result<()>;

    fn size(&self) -> usize {
        CrowdItemHeader_XE_::size_of()
            + Crc_XE_::size_of() * self.animations_len()
            + CrowdVal_XE_::size_of() * self.instances_len()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let header = CrowdItemHeader_XE_::mut_from_data(dst).context("header")?;
        self.write_header(header).context("write header")?;
        header.animation_num = self.animations_len().conv();
        header.instance_num = self.instances_len().conv();
        let animations =
            Crc_XE_::mut_slice_from_data(dst, self.animations_len()).context("animations")?;
        self.write_animations(animations)
            .context("write animations")?;
        let instances =
            CrowdVal_XE_::mut_slice_from_data(dst, self.instances_len()).context("instances")?;
        self.write_instances(instances).context("write instances")?;
        Ok(())
    }
}

#[make_endian]
impl DumpCrowdItem_XE_ for CrowdItemRef_XE_<'_> {
    fn animations_len(&self) -> usize {
        self.animations.len()
    }
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn write_header(&self, header: &mut CrowdItemHeader_XE_) -> Result<()> {
        header.write_from(self.header)
    }
    fn write_instances(&self, instances: &mut [CrowdVal_XE_]) -> Result<()> {
        instances.write_from(&self.instances[..])
    }
    fn write_animations(&self, animations: &mut [Crc_XE_]) -> Result<()> {
        animations.write_from(&self.animations[..])
    }
}

#[make_endian]
impl DumpCrowdItem_XE_ for CrowdItem {
    fn animations_len(&self) -> usize {
        self.animations.len()
    }
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn write_header(&self, header: &mut CrowdItemHeader_XE_) -> Result<()> {
        *header = (&self.header).conv();
        Ok(())
    }
    fn write_instances(&self, instances: &mut [CrowdVal_XE_]) -> Result<()> {
        for (src, dst) in self.instances.iter().zip(instances) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_animations(&self, animations: &mut [Crc_XE_]) -> Result<()> {
        for (src, dst) in self.animations.iter().zip(animations) {
            *dst = src.conv();
        }
        Ok(())
    }
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CrowdHeader_XE_ {
    pub const0x65: u32_XE_,
    pub n: u32_XE_,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct CrowdRef<'a, T: Block2Types>{
    pub header: &'a T::CrowdHeader,
    pub offs: ref_slice<'a, T::u32>,
    pub vals: slice<CrowdItemRef<'a, T>>
}

impl<'a, T: Block2Types> CrowdRef<'a, T> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let header = T::CrowdHeader::from_data(src).context("header")?;
        if header.const0x65() != 0x65 {
            return Err(anyhow!("Invalid Block Data for Crowd Block"));
        }
        let offs = T::u32::slice_from_data(&src[header.size_of_val()..], header.n() as usize)
            .context("offs")?;
        let vals = offs
            .into_iter()
            .enumerate()
            .map(|(i, off)| {
                CrowdItemRef::from_data(&src[off.conv() as usize..])
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

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct CrowdRef_XE_<'a>{
    pub header: &'a CrowdHeader_XE_,
    pub offs: ref_slice<'a, u32_XE_>,
    pub vals: slice<CrowdItemRef_XE_<'a>>
}

#[make_endian]
impl<'a> CrowdRef_XE_<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let header = CrowdHeader_XE_::from_data(src).context("header")?;
        if header.const0x65 != 0x65 {
            return Err(anyhow!("Invalid Block Data for Crowd Block"));
        }
        let offs = u32_XE_::slice_from_data(&src[header.size_of_val()..], header.n.conv())
            .context("offs")?;
        let vals = offs
            .into_iter()
            .enumerate()
            .map(|(i, off)| {
                CrowdItemRef_XE_::from_data(&src[off.conv()..])
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

#[make_endian]
impl From<&CrowdRef_XE_<'_>> for Crowd {
    fn from(val: &CrowdRef_XE_) -> Self {
        Self {
            vals: val.vals.iter().map(|x| x.into()).collect(),
        }
    }
}

#[make_endian]
pub struct CrowdImpl_XE_<'a> {
    pub vals: Vec<CrowdItem_XE_<'a>>
}

#[make_endian]
impl<'a> From<CrowdRef_XE_<'a>> for CrowdImpl_XE_<'a> {
    fn from(val: CrowdRef_XE_<'a>) -> Self {
        Self {
            vals: Box::<[_]>::from(val.vals).into_vec().into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[make_endian]
#[enum_dispatch(DumpCrowd_XE_)]
pub enum Crowd_XE_<'a> {
    Ref(CrowdRef_XE_<'a>),
    Owned(CrowdImpl_XE_<'a>)
}

#[make_endian]
pub trait DumpCrowdImpl_XE_ {
    fn vals_len(&self) -> usize;
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItem_XE_>;
}

#[make_endian]
#[enum_dispatch]
pub trait DumpCrowd_XE_ {
    fn size(&self) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
}

#[make_endian]
impl<T: DumpCrowdImpl_XE_> DumpCrowd_XE_ for T {
    fn size(&self) -> usize {
        CrowdHeader_XE_::size_of()
            + u32_XE_::size_of() * self.vals_len()
            + self.vals().map(|x| x.size()).sum::<usize>()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let start = dst.offset;
        let header = CrowdHeader_XE_::mut_from_data(dst).context("header")?;
        let offs = u32_XE_::mut_slice_from_data(dst, self.vals_len()).context("offs")?;
        header.n = offs.len().conv();
        header.const0x65 = 0x65u32.conv();

        for (i, (val, off)) in self.vals().zip(offs).enumerate() {
            *off = (dst.offset - start).conv();
            val.dump_into(dst).with_context(|| format!("val {}", i))?;
        }
        Ok(())
    }
}

#[make_endian]
impl DumpCrowdImpl_XE_ for CrowdRef_XE_<'_> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItem_XE_> {
        self.vals.iter()
    }
}

#[make_endian]
impl DumpCrowdImpl_XE_ for Crowd {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItem_XE_> {
        self.vals.iter()
    }
}

#[make_endian]
impl DumpCrowdImpl_XE_ for CrowdImpl_XE_<'_> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItem_XE_> {
        self.vals.iter()
    }
}

#[make_endian]
pub type PFieldsRef_XE_<'a> = DataRef_XE_<'a>;
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

#[make_endian]
#[enum_dispatch(DumpData_XE_)]
pub enum PFields_XE_<'a> {
    Ref(PFieldsRef_XE_<'a>),
    Owned(PFields)
}

#[make_endian]
impl DumpData_XE_ for PFields { 
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        for (i, pfield) in self.vals.values().enumerate() {
            for (j, (_, vals)) in pfield.vals.iter().enumerate() {
                vals.dump_into(dst)
                    .with_context(|| format!("pfield {} val {}", i, j))?;
            }
        }
        Ok(())
    }
    fn size(&self) -> usize {
        self.vals
            .values()
            .flat_map(|x| x.vals.iter())
            .map(|(_, vals)| vals.size_of_val())
            .sum::<usize>()
    }
}

impl PFields {
    #[make_endian]
    pub fn from_xe_(val: &PFieldsRef_XE_, infos: &[PFieldInfo_XE_]) -> Self {
        let mut offset_maps: HashMap<u32, HashMap<u32, usize>> = HashMap::new();
        let mut pfields = IndexMap::new();
        let data = &val.data[..];
        for info in infos {
            let link_guid = info.link_guid.conv();
            let offset = info.offset.conv();
            let gamemode_guid = info.gamemode_guid.conv();
            let width = info.width.conv();
            let height = info.height.conv();
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

    #[make_endian]
    pub fn infos_xe_(&self) -> Vec<PFieldInfo_XE_> {
        let mut infos = vec![];
        let mut offset = 0u32;
        for pfield in self.vals.values() {
            for (gamemodes, _) in &pfield.vals {
                for &gamemode_guid in gamemodes {
                    infos.push(PFieldInfo_XE_ {
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
