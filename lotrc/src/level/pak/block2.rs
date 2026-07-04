use crate::types::GetNative;
use crate::types::{get_str, hash_string, Crc, DumpData, DumpSlice, RefFromData, Vector3, OrderedData, OrderedDataStrict, align_offset, ref_slice, slice};
#[make_platforms]
use crate::{
    level::pak::{
        PakHeaderVER,
        block1::{
            objs::PFieldInfoVER,
        }
    },
    types::{
        CrcVER, u16VER, u32VER, f32VER, Vector3VER, StringKeysRefVER,
        sub_blocks::{
            SubBlocksInfoRefVER, DataRefVER, DumpDataVER, DumpStringVER,
        },
    },
};
use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use log::{warn, debug};
use lotrc_proc::{make_platforms, OrderedData};
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

#[make_platforms]
#[derive(Default)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Block2RefVER<'a> {
    pub sub_blocks: SubBlocks2RefVER<'a>,
    pub offsets: ref_slice<'a, u32VER>
}

#[make_platforms]
impl<'a> Block2RefVER<'a> {
    pub fn from_data(src: &'a [u8], pak_header: &PakHeaderVER, string_keys: &StringKeysRefVER) -> Result<Self> {
        let t = std::time::Instant::now();
        let sub_blocks = SubBlocks2RefVER::from_data(
            &src[pak_header.sub_blocks2_offset.get() as usize..],
            string_keys
        )
        .context("sub_blocks")?;
        debug!("Block2 sub_blocks parsed in {}", t.elapsed().as_secs_f32());
        let t = std::time::Instant::now();
        let offsets = u32VER::slice_from_data(
            &src[pak_header.block2_offsets_offset.get() as usize..],
            pak_header.block2_offsets_num.get() as usize,
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

#[make_platforms]
pub trait DumpBlock2VER {
    fn sub_blocks(&self) -> &impl DumpSubBlocks2VER;
    fn dump<'a>(&self, dst: &mut DumpSlice<'a>, offset_num: usize, keys: &[u32], header: &mut PakHeaderVER) -> Result<&'a mut [u32VER]> {
        header.sub_blocks2_offset = dst.offset.conv();
        self.sub_blocks().dump_into(dst, keys).context("sub_blocks")?;
        header.block2_offsets_offset = dst.offset.conv();
        let offsets = u32VER::mut_slice_from_data(dst, offset_num).context("offsets")?;
        header.block2_offsets_num = offsets.len().conv();
        Ok(offsets)
    }
}

#[make_platforms]
impl DumpBlock2VER for Block2RefVER<'_> {
    fn sub_blocks(&self) -> &impl DumpSubBlocks2VER {
        &self.sub_blocks
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SubBlocks2RefVER<'a> {
    pub info: SubBlocksInfoRefVER<'a>,
    pub spray: Option<SprayRefVER<'a>>,
    pub crowd: Option<CrowdRefVER<'a>>,
    pub pfields: Option<PFieldsRefVER<'a>>,
    pub langs: IndexMap<u32, LangStringsRefVER<'a>>,
    pub files: IndexMap<u32, DataRefVER<'a>>
}

#[make_platforms]
impl Default for SubBlocks2RefVER<'_> {
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

#[make_platforms]
impl<'a> SubBlocks2RefVER<'a> {
    // TODO should use the pak string to get key names
    // TODO add ref to pfeild infos in pfields struct
    pub fn from_data(src: &'a [u8], string_keys: &StringKeysRefVER) -> Result<Self> {
        use keys::*;
        let info = SubBlocksInfoRefVER::from_data(src)?;
        let mut spray = None;
        let mut crowd = None;
        let mut pfields = None;
        let mut langs = IndexMap::new();
        let mut files = IndexMap::new();
        for info in info.block_headers.iter() {
            let data = &src[info.offset.get() as usize .. (info.offset + info.size) as usize];
            match info.key.get() {
                KEY_POLISH | KEY_GERMAN | KEY_FRENCH | KEY_SPANISH | KEY_RUSSIAN | KEY_SWEDISH
                | KEY_ENGLISH | KEY_ITALIAN | KEY_NORWEGIAN => {
                    langs.insert(info.key.get(), LangStringsRefVER::from_data(data, string_keys)?);
                }
                KEY_SPRAY => {spray.replace(SprayRefVER::from_data(data)?);},
                KEY_CROWD => {crowd.replace(CrowdRefVER::from_data(data)?);},
                KEY_PFIELDS => {pfields.replace(PFieldsRefVER::from_data(data));},
                key => {
                    warn!("Unknown block type {:?}, {:?}", key, get_str(&key).map(|x| x.to_string()).unwrap_or_default());
                    files.insert(key, DataRefVER::from_data(data));
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

#[make_platforms]
pub trait DumpSubBlocks2VER {
    fn files_num(&self) -> usize;
    fn langs_num(&self) -> usize;
    fn spray(&self) -> Option<&impl DumpSprayVER>;
    fn crowd(&self) -> Option<&impl DumpCrowdVER>;
    fn pfields(&self) -> Option<&impl DumpDataVER>;
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpDataVER)>;
    fn langs(&self) -> impl Iterator<Item = (u32, &impl DumpLangStringsVER)>;
    
    fn blocks_num(&self) -> usize {
        self.files_num() + self.langs_num()
    }

    fn size(&self) -> (usize, Vec<u32>) {
        let spray = self.spray();
        let crowd = self.crowd();
        let pfields = self.pfields();
        let mut size = SubBlocksInfoRefVER::size(
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
        let block_headers = SubBlocksInfoRefVER::dump(dst, 
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

#[make_platforms]
impl DumpSubBlocks2VER for SubBlocks2RefVER<'_> {
    fn files_num(&self) -> usize {
        self.files.len()
	}
    fn langs_num(&self) -> usize {
        self.langs.len()
	}
    fn spray(&self) -> Option<&impl DumpSprayVER> {
        self.spray.as_ref()
	}
    fn crowd(&self) -> Option<&impl DumpCrowdVER> {
        self.crowd.as_ref()
	}
    fn pfields(&self) -> Option<&impl DumpDataVER> {
        self.pfields.as_ref()
	}
    fn files(&self) -> impl Iterator<Item = (u32, &impl DumpDataVER)> {
        self.files.iter().map(|(k, v)| (*k, v))
	}
    fn langs(&self) -> impl Iterator<Item = (u32, &impl DumpLangStringsVER)> {
        self.langs.iter().map(|(k, v)| (*k, v))
	}
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct LangStringsRefVER<'a> {
    pub strings: IndexMap<u32, ref_slice<'a, u16VER>>,
}

#[make_platforms]
impl<'a> LangStringsRefVER<'a> {
    pub fn from_data(src: &'a [u8], string_keys: &StringKeysRefVER) -> Result<Self> {
        let mut offset = 0;
        let mut strings = IndexMap::with_capacity(string_keys.vals.len());
        for key in string_keys.vals.iter() {
            let start = offset;
            while src[offset] != 0 || src[offset + 1] != 0 {
                offset += 2;
            }
            let s = u16VER::slice_from_data(&src[start..offset], (offset - start) / 2)
                .with_context(|| format!("string {}", strings.len()))?;
            strings.insert(key.key.get(), s.into());
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

#[make_platforms]
fn parse_string_ver(val: &ref_slice<'_, u16VER>) -> Result<String, std::string::FromUtf16Error> {
    String::from_utf16(
        val.iter()
            .map(|y| y.conv())
            .collect::<Vec<_>>()
            .as_ref(),
    )
}

#[make_platforms]
impl TryFrom<&LangStringsRefVER<'_>> for LangStrings {
    type Error = std::string::FromUtf16Error;
    fn try_from(val: &LangStringsRefVER) -> Result<Self, Self::Error> {
        Ok(Self {
            strings: val
                .strings
                .iter()
                .map(|(k, v)| Ok(((*k).into(), parse_string_ver(v)?)))
                .collect::<Result<_, Self::Error>>()?,
        })
    }
}

#[make_platforms]
#[enum_dispatch(DumpLangStringsVER)]
pub enum LangStringsVER<'a> {
    Ref(LangStringsRefVER<'a>),
    Owned(LangStrings)
}


#[make_platforms]
pub trait DumpLangStringsImplVER {
    fn strings(&self) -> impl Iterator<Item = (u32, &impl DumpStringVER)>;
    fn get(&self, k: &u32) -> Option<&impl DumpStringVER>;
}

#[make_platforms]
#[enum_dispatch]
pub trait DumpLangStringsVER {
    fn size(&self, keys: &mut HashSet<u32>) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice, keys: &[u32]) -> Result<()>;
}
#[make_platforms]
impl<T: DumpLangStringsImplVER> DumpLangStringsVER for T {
    fn size(&self, keys: &mut HashSet<u32>) -> usize {
        self.strings()
            .map(|(k, v)| {
                keys.insert(k);
                v.string_size() + u16VER::size_of()
            })
            .sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice, keys: &[u32]) -> Result<()> {
        for k in keys {
            if let Some(string) = self.get(k) {
                string.dump_string(dst).with_context(|| format!("string {}", k))?;
            }
            u16VER::from(0u16)
                .dump_into(dst)
                .with_context(|| format!("string pad {}", k))?;
        }
        Ok(())
    }
}
#[make_platforms]
impl DumpLangStringsImplVER for LangStringsRefVER<'_> {
    fn strings(&self) -> impl Iterator<Item = (u32, &impl DumpStringVER)> {
        self.strings.iter().map(|(k, v)| (*k, v))
    }
    fn get(&self, k: &u32) -> Option<&impl DumpStringVER> {
        self.strings.get(k)
    }
}

#[make_platforms]
impl DumpLangStringsImplVER for LangStrings {
    fn strings(&self) -> impl Iterator<Item = (u32, &impl DumpStringVER)> {
        self.strings.iter().map(|(k, v)| (k.get(), v))
    }
    fn get(&self, k: &u32) -> Option<&impl DumpStringVER> {
        self.strings.get(k)
    }
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct SprayInstance {
    pub key: Crc,
    pub tex1: Crc,
    pub tex2: Crc,
    pub unk_3: u32,
    pub width: u32,
    pub height: u32,
    pub unk_6: f32,
    pub unk_7: f32,
    pub size_w: u32,
    pub size_h: u32,
    pub scale_w: f32,
    pub scale_h: f32,
    pub delay: u32,
    pub stride_x: f32,
    pub stride_y: f32,
    pub unk_15: u32,
    pub unk_16: u32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct SprayVal {
    pub position: Vector3,
    pub scale: f32,
    pub instance: u16,
    pub rotation: u16,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct SprayRefVER<'a> {
    pub instances: ref_slice<'a, SprayInstanceVER>,
    pub vals: ref_slice<'a, SprayValVER>,
}

#[make_platforms]
impl<'a> SprayRefVER<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let n = u32VER::from_data(&src[offset..]).context("n1")?;
        offset += n.size();
        let instances = SprayInstanceVER::slice_from_data(&src[offset..], n.get() as usize)
            .context("instances")?;
        offset += instances.size();
        let n = u32VER::from_data(&src[offset..]).context("n2")?;
        offset += n.size();
        let vals =
            SprayValVER::slice_from_data(&src[offset..], n.get() as usize).context("vals")?;

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

#[make_platforms]
impl From<&SprayRefVER<'_>> for Spray {
    fn from(val: &SprayRefVER) -> Self {
        Self {
            instances: val.instances.iter().map(|x| x.conv()).collect(),
            vals: val.vals.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_platforms]
#[enum_dispatch(DumpSprayVER)]
pub enum SprayVER<'a> {
    Ref(SprayRefVER<'a>),
    Owned(Spray)
}

#[make_platforms]
#[enum_dispatch]
pub trait DumpSprayVER {
    fn instances_len(&self) -> usize;
    fn vals_len(&self) -> usize;
    fn write_instances(&self, instances: &mut [SprayInstanceVER]) -> Result<()>;
    fn write_vals(&self, vals: &mut [SprayValVER]) -> Result<()>;

    fn size(&self) -> usize {
        u32VER::size_of() * 2
            + SprayInstanceVER::size_of() * self.instances_len()
            + SprayValVER::size_of() * self.vals_len()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let n = u32VER::mut_from_data(dst).context("n1")?;
        let instances = SprayInstanceVER::mut_slice_from_data(dst, self.instances_len())
            .context("instances")?;
        *n = instances.len().conv();
        self.write_instances(instances).context("write instances")?;
        let n = u32VER::mut_from_data(dst).context("n1")?;
        let vals = SprayValVER::mut_slice_from_data(dst, self.vals_len()).context("vals")?;
        *n = vals.len().conv();
        self.write_vals(vals).context("write vals")?;
        Ok(())
    }
}

#[make_platforms]
impl DumpSprayVER for SprayRefVER<'_> {
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_instances(&self, instances: &mut [SprayInstanceVER]) -> Result<()> {
        instances.write_from(&self.instances[..])
    }
    fn write_vals(&self, vals: &mut [SprayValVER]) -> Result<()> {
        vals.write_from(&self.vals[..])
    }
}

#[make_platforms]
impl DumpSprayVER for Spray {
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn write_instances(&self, instances: &mut [SprayInstanceVER]) -> Result<()> {
        for (src, dst) in self.instances.iter().zip(instances) {
            *dst = src.conv()
        }
        Ok(())
    }
    fn write_vals(&self, vals: &mut [SprayValVER]) -> Result<()> {
        for (src, dst) in self.vals.iter().zip(vals) {
            *dst = src.conv()
        }
        Ok(())
    }
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct CrowdItemHeader {
    pub key: Crc,
    pub key_main: Crc,
    pub key_right: Crc,
    pub key_left: Crc,
    pub unk_4: f32,
    pub animation_num: u32,
    pub instance_num: u32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct CrowdVal {
    pub position: Vector3,
    pub rotation: f32,
    pub lod: f32,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct CrowdItemRefVER<'a> {
    pub header: &'a CrowdItemHeaderVER,
    pub animations: ref_slice<'a, CrcVER>,
    pub instances: ref_slice<'a, CrowdValVER>
}

#[make_platforms]
impl<'a> CrowdItemRefVER<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let header = CrowdItemHeaderVER::from_data(&src[offset..]).context("header")?;
        offset += header.size();
        let animations =
            CrcVER::slice_from_data(&src[offset..], header.animation_num.get() as usize)
                .context("animations")?;
        offset += animations.size();
        let instances =
            CrowdValVER::slice_from_data(&src[offset..], header.instance_num.get() as usize)
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

#[make_platforms]
impl From<&CrowdItemRefVER<'_>> for CrowdItem {
    fn from(val: &CrowdItemRefVER) -> Self {
        Self {
            header: val.header.conv(),
            animations: val.animations.iter().map(|x| x.conv()).collect(),
            instances: val.instances.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_platforms]
#[enum_dispatch(DumpCrowdItemVER)]
pub enum CrowdItemVER<'a> {
    Ref(CrowdItemRefVER<'a>),
    Owned(CrowdItem)
}

#[make_platforms]
#[enum_dispatch]
pub trait DumpCrowdItemVER {
    fn animations_len(&self) -> usize;
    fn instances_len(&self) -> usize;
    fn write_header(&self, header: &mut CrowdItemHeaderVER) -> Result<()>;
    fn write_animations(&self, animations: &mut [CrcVER]) -> Result<()>;
    fn write_instances(&self, instances: &mut [CrowdValVER]) -> Result<()>;

    fn size(&self) -> usize {
        CrowdItemHeaderVER::size_of()
            + CrcVER::size_of() * self.animations_len()
            + CrowdValVER::size_of() * self.instances_len()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let header = CrowdItemHeaderVER::mut_from_data(dst).context("header")?;
        self.write_header(header).context("write header")?;
        header.animation_num = self.animations_len().conv();
        header.instance_num = self.instances_len().conv();
        let animations =
            CrcVER::mut_slice_from_data(dst, self.animations_len()).context("animations")?;
        self.write_animations(animations)
            .context("write animations")?;
        let instances =
            CrowdValVER::mut_slice_from_data(dst, self.instances_len()).context("instances")?;
        self.write_instances(instances).context("write instances")?;
        Ok(())
    }
}

#[make_platforms]
impl DumpCrowdItemVER for CrowdItemRefVER<'_> {
    fn animations_len(&self) -> usize {
        self.animations.len()
    }
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn write_header(&self, header: &mut CrowdItemHeaderVER) -> Result<()> {
        header.write_from(self.header)
    }
    fn write_instances(&self, instances: &mut [CrowdValVER]) -> Result<()> {
        instances.write_from(&self.instances[..])
    }
    fn write_animations(&self, animations: &mut [CrcVER]) -> Result<()> {
        animations.write_from(&self.animations[..])
    }
}

#[make_platforms]
impl DumpCrowdItemVER for CrowdItem {
    fn animations_len(&self) -> usize {
        self.animations.len()
    }
    fn instances_len(&self) -> usize {
        self.instances.len()
    }
    fn write_header(&self, header: &mut CrowdItemHeaderVER) -> Result<()> {
        *header = (&self.header).conv();
        Ok(())
    }
    fn write_instances(&self, instances: &mut [CrowdValVER]) -> Result<()> {
        for (src, dst) in self.instances.iter().zip(instances) {
            *dst = src.conv();
        }
        Ok(())
    }
    fn write_animations(&self, animations: &mut [CrcVER]) -> Result<()> {
        for (src, dst) in self.animations.iter().zip(animations) {
            *dst = src.conv();
        }
        Ok(())
    }
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct CrowdHeader {
    pub const0x65: u32,
    pub n: u32,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct CrowdRefVER<'a>{
    pub header: &'a CrowdHeaderVER,
    pub offs: ref_slice<'a, u32VER>,
    pub vals: slice<CrowdItemRefVER<'a>>
}

#[make_platforms]
impl<'a> CrowdRefVER<'a> {
    pub fn from_data(src: &'a [u8]) -> Result<Self> {
        let header = CrowdHeaderVER::from_data(src).context("header")?;
        if header.const0x65.get() != 0x65 {
            return Err(anyhow!("Invalid Block Data for Crowd Block"));
        }
        let offs = u32VER::slice_from_data(&src[header.size()..], header.n.get() as usize)
            .context("offs")?;
        let vals = offs
            .into_iter()
            .enumerate()
            .map(|(i, off)| {
                CrowdItemRefVER::from_data(&src[off.get() as usize..])
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

#[make_platforms]
impl From<&CrowdRefVER<'_>> for Crowd {
    fn from(val: &CrowdRefVER) -> Self {
        Self {
            vals: val.vals.iter().map(|x| x.into()).collect(),
        }
    }
}

#[make_platforms]
pub struct CrowdImplVER<'a> {
    pub vals: Vec<CrowdItemVER<'a>>
}

#[make_platforms]
impl<'a> From<CrowdRefVER<'a>> for CrowdImplVER<'a> {
    fn from(val: CrowdRefVER<'a>) -> Self {
        Self {
            vals: Box::<[_]>::from(val.vals).into_vec().into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[make_platforms]
#[enum_dispatch(DumpCrowdVER)]
pub enum CrowdVER<'a> {
    Ref(CrowdRefVER<'a>),
    Owned(CrowdImplVER<'a>)
}

#[make_platforms]
pub trait DumpCrowdImplVER {
    fn vals_len(&self) -> usize;
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItemVER>;
}

#[make_platforms]
#[enum_dispatch]
pub trait DumpCrowdVER {
    fn size(&self) -> usize;
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()>;
}

#[make_platforms]
impl<T: DumpCrowdImplVER> DumpCrowdVER for T {
    fn size(&self) -> usize {
        CrowdHeaderVER::size_of()
            + u32VER::size_of() * self.vals_len()
            + self.vals().map(|x| x.size()).sum::<usize>()
    }

    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        let start = dst.offset;
        let header = CrowdHeaderVER::mut_from_data(dst).context("header")?;
        let offs = u32VER::mut_slice_from_data(dst, self.vals_len()).context("offs")?;
        header.n = offs.len().conv();
        header.const0x65 = 0x65u32.conv();

        for (i, (val, off)) in self.vals().zip(offs).enumerate() {
            *off = (dst.offset - start).conv();
            val.dump_into(dst).with_context(|| format!("val {}", i))?;
        }
        Ok(())
    }
}

#[make_platforms]
impl DumpCrowdImplVER for CrowdRefVER<'_> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItemVER> {
        self.vals.iter()
    }
}

#[make_platforms]
impl DumpCrowdImplVER for Crowd {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItemVER> {
        self.vals.iter()
    }
}

#[make_platforms]
impl DumpCrowdImplVER for CrowdImplVER<'_> {
    fn vals_len(&self) -> usize {
        self.vals.len()
    }
    fn vals(&self) -> impl Iterator<Item = &impl DumpCrowdItemVER> {
        self.vals.iter()
    }
}

#[make_platforms]
pub type PFieldsRefVER<'a> = DataRefVER<'a>;

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

#[make_platforms]
#[enum_dispatch(DumpDataVER)]
pub enum PFieldsVER<'a> {
    Ref(PFieldsRefVER<'a>),
    Owned(PFields)
}

#[make_platforms]
impl DumpDataVER for PFields { 
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
            .map(|(_, vals)| vals.size())
            .sum::<usize>()
    }
}

impl PFields {
    #[make_platforms]
    pub fn from_ver(val: &PFieldsRefVER, infos: &[PFieldInfoVER]) -> Self {
        let mut offset_maps: HashMap<u32, HashMap<u32, usize>> = HashMap::new();
        let mut pfields = IndexMap::new();
        let data = &val.data[..];
        for info in infos {
            let link_guid = info.link_guid.get();
            let offset = info.offset.get();
            let gamemode_guid = info.gamemode_guid.get();
            let width = info.width.get();
            let height = info.height.get();
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

    #[make_platforms]
    pub fn infos_ver(&self) -> Vec<PFieldInfoVER> {
        let mut infos = vec![];
        let mut offset = 0u32;
        for pfield in self.vals.values() {
            for (gamemodes, _) in &pfield.vals {
                for &gamemode_guid in gamemodes {
                    infos.push(PFieldInfoVER {
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
