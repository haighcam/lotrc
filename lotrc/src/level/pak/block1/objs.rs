use anyhow::{Context, Result};
use indexmap::IndexMap;
use itertools::Itertools;
use std::collections::BinaryHeap;
use log::debug;

#[make_endian]
use crate::{
    level::{
        model::{ModelRef_XE_, DumpModel_XE_ },
        pak::{
            block1::{
                infos::{InfosRef_XE_, DumpInfos_XE_},
                gameobjs::{GameObjsRef_XE_, DumpGameObjs_XE_}, 
            },
            PakHeader_XE_,
        },
        bin::{BinRef_XE_},
        radiosity::{RadiosityRef_XE_, DumpRadiosity_XE_},
        texture::{TextureInfo_XE_, TextureRef_XE_, DumpTexture_XE_},
    },
    types::{u32_XE_, Crc_XE_, i32_XE_, f32_XE_, i16_XE_, u16_XE_, Vector4_XE_},
};
use crate::{
    level::{
        model::{
            shape::{HkConstraint, HkShape, Shape},
            Model,
        },
        pak::{
            block1::{
                infos::InfoCounts,
                gameobjs::{GameObjs, TypeInfos}
            },
            animation::AnimationBlockInfo
        },
        radiosity::{RadiosityValsInfo},
    },
    types::{CompressedData, Crc, DumpData, DumpSlice, RefFromData, Vector4, hash_string, OrderedData, ref_slice, slice, get_str, align_offset},
};
use lotrc_proc::{make_endian, derive_ordered_data};

#[derive(Debug, Default, Clone, PartialEq, lotrc_proc::FromConvImpl)]
#[create_conv_trait]
pub struct ObjA {
    pub key: Crc,
    pub unk_1: u32,
    pub size: u32,
    pub size_comp: u32,
    pub unk_4: u32,
    pub kind: u32,
}

#[make_endian]
#[derive(Debug, Default, Clone, PartialEq, lotrc_proc::IntoConvImpl, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::FromBytes)]
#[conv_base(ObjA)]
pub struct ObjA_XE_ {
    pub key: CrcLE,
    pub unk_1: u32LE,
    pub size: u32LE,
    pub size_comp: u32LE,
    pub unk_4: u32LE,
    pub kind: u32LE,
}

#[derive(Debug, Default, Clone, PartialEq, lotrc_proc::FromConvImpl)]
#[create_conv_trait]
pub struct Obj0 {
    pub unk_0: u32,
    pub key: Crc,
}

#[make_endian]
#[derive(Debug, Default, Clone, PartialEq, lotrc_proc::IntoConvImpl, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::FromBytes)]
#[conv_base(Obj0)]
pub struct Obj0_XE_ {
    pub unk_0: u32LE,
    pub key: CrcLE,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EffectInfo_XE_ {
    pub key: Crc_XE_,
    pub gamemodemask: i32_XE_,
    pub offset: u32_XE_,
    pub size: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PFieldInfo_XE_ {
    pub link_guid: u32_XE_,
    pub gamemode_guid: u32_XE_,
    pub width: u32_XE_,
    pub height: u32_XE_,
    pub offset: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GFXBlockInfo_XE_ {
    // GFX blocks?, unchanged by encoding, model as data
    pub key: Crc_XE_,
    pub offset: u32_XE_,
    pub size: u32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FoliageInfo_XE_ {
    pub key: Crc_XE_,
    pub kind: u32_XE_,
    pub lb_w: i32_XE_,
    pub lb_h: i32_XE_,
    pub ub_w: i32_XE_,
    pub ub_h: i32_XE_,
    pub scale: f32_XE_,
    pub offset: u32_XE_,
    pub key_mesh: Crc_XE_,
    pub key_mesh_lod1: Crc_XE_,
    pub key_mesh_lod2: Crc_XE_,
    pub color: Vector4_XE_,
    pub lod1a: f32_XE_,
    pub lod1b: f32_XE_,
    pub lod2a: f32_XE_,
    pub lod2b: f32_XE_,
    pub lod_max: f32_XE_,
}

#[derive_ordered_data]
#[derive(Debug, Clone, Default, PartialEq)]
// height: u16, var_mask: u16, slope_x: i16, slope_z: i16
// position and orientation are randomized for each instance (within own square)
// var mask only checks high component vs alpha of vertex attr (if it exists)
pub struct FoliageVal_XE_ {
    pub height: u16_XE_,
    pub var_mask: u16_XE_,
    pub slope_x: i16_XE_,
    pub slope_z: i16_XE_,
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct FoliageRef_XE_<'a> {
    info: &'a FoliageInfo_XE_,
    vals: ref_slice<'a, FoliageVal_XE_>
}

#[make_endian]
impl<'a> FoliageRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a FoliageInfo_XE_) -> Result<Self> {
        let n =
            ((info.ub_w.to_native() - info.lb_w.to_native()) * (info.ub_h.to_native() - info.lb_h.to_native())) as usize;
        let vals = FoliageVal_XE_::slice_from_data(&src[info.offset.conv()..], n)
            .context("vals")?;
        Ok(Self {
            info: info,
            vals: vals.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Foliage {
    pub info: FoliageInfo,
    pub vals: Vec<FoliageVal>,
}

#[make_endian]
impl From<&FoliageRef_XE_<'_>> for Foliage {
    fn from(val: &FoliageRef_XE_) -> Self {
        Self {
            info: val.info.conv(),
            vals: val.vals.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_endian]
pub trait DumpFoliage_XE_ {
    fn vals_num(&self) -> usize;
    fn write_vals(&self, vals: &mut [FoliageVal_XE_]) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice, dump_infos: &mut DumpInfos_XE_) -> Result<()> {
        *dump_infos.offsets.next().context("offsets")? = (dump_infos.foliages.offset + std::mem::offset_of!(FoliageInfo_XE_, offset)).conv();
        let info = dump_infos.foliages.next().context("info")?;
        info.offset = dst.offset.conv();
        let vals = FoliageVal_XE_::mut_slice_from_data(dst, self.vals_num()).context("vals")?;
        self.write_vals(vals).context("write vals")
    }
    fn add_size(&self, offset: usize) -> usize {
        offset + self.vals_num() * std::mem::size_of::<FoliageVal_XE_>()
    }
}

#[make_endian]
impl DumpFoliage_XE_ for FoliageRef_XE_<'_> {
    fn vals_num(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [FoliageVal_XE_]) -> Result<()> {
        vals.write_from(&self.vals[..])
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct EffectRef_XE_<'a> {
    info: &'a EffectInfo_XE_,
    vals: GameObjsRef_XE_<'a>
}

#[make_endian]
impl<'a> EffectRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], info: &'a EffectInfo_XE_) -> Result<Self> {
        Ok(Self {
            info,
            vals: GameObjsRef_XE_::from_data(&src[info.offset.conv()..(info.offset + info.size) as usize])?
        })
    }
}

#[make_endian]
pub trait DumpEffect_XE_ {
    fn vals(&self) -> &impl DumpGameObjs_XE_;
    fn gamemodemask(&self) -> i32_XE_;
}

#[make_endian]
impl DumpEffect_XE_ for EffectRef_XE_<'_> {
    fn vals(&self) -> &impl DumpGameObjs_XE_ {
        &self.vals
    }
    fn gamemodemask(&self) -> i32_XE_ {
        self.info.gamemodemask
    }
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Default, PartialEq)]
pub struct ObjsRef_XE_<'a> {
    pub textures: IndexMap<u32, TextureRef_XE_<'a>>,
    pub models: IndexMap<u32, ModelRef_XE_<'a>>,
    pub effects: IndexMap<u32, EffectRef_XE_<'a>>,
    pub foliages: IndexMap<u32, slice<FoliageRef_XE_<'a>>>,
    pub gfxs: IndexMap<u32, ref_slice<'a, u8>>,
    pub radiosity: RadiosityRef_XE_<'a>,
}

#[make_endian]
impl<'a> ObjsRef_XE_<'a> {
    pub fn from_data(src: &'a [u8], infos: &InfosRef_XE_<'a>, bin: &BinRef_XE_<'a>, name: u32) -> Result<Self> {
        let texture_data = &bin.texture_data;
        let textures: IndexMap<_, _> = infos.textures.iter().enumerate()
            .map(|(i, info)| Ok((
                info.key.conv(),
                TextureRef_XE_::from_data(info, texture_data).with_context(|| format!("texture {}", i))?
            )))
            .collect::<Result<_>>()?;
        let model_data = &bin.model_data;
        let models: IndexMap<_, _> = infos.models.iter().enumerate()
            .map(|(i, info)| Ok((
                info.key.conv(),
                ModelRef_XE_::from_data(src, info, model_data).with_context(|| format!("model {}", i))?
            )))
            .collect::<Result<_>>()?;
        let effects: IndexMap<_, _> = infos.effects
            .iter()
            .map(|info| Ok((
                info.key.conv(),
                EffectRef_XE_::from_data(src, info).with_context(|| format!("effect {}", info.key.to_native()))?
            )))
            .collect::<Result<_>>()?;
        let gfxs: IndexMap<_, _> = infos.gfxs
            .iter()
            .map(|info| (
                info.key.conv(),
                (&src[info.offset.conv()..(info.offset.to_native() + info.size.to_native()) as usize]).into(),
            ))
            .collect();
        let mut foliages = IndexMap::<u32, Vec<FoliageRef_XE_>>::with_capacity(infos.foliages.len());
        for (i, info) in infos.foliages.iter().enumerate() {
            let foliage = FoliageRef_XE_::from_data(src, info).with_context(|| format!("foliage {}", i))?;
            foliages.entry(foliage.info.key.conv()).or_default().push(foliage);
        }
        let foliages: IndexMap<_, _> = foliages.into_iter().map(|(k, v)| (k, v.into_boxed_slice().into())).collect();
        let radiosity_name = hash_string(b"_radiosity", Some(name));
        let ind = model_data.get_index_of(&radiosity_name);
        let data = ind.and_then(|x| model_data.get_index(x)).map(|(_, x)| x).copied();
        let usage =  ind.map(|x| bin.model_handles()[x].kind.conv()).unwrap_or_default();
        
        let radiosity = RadiosityRef_XE_::from_data(src, infos.radiosity_vals, data, usage).context("radiosity")?; 
        Ok(Self {
            textures,
            models,
            effects,
            gfxs,
            foliages,
            radiosity,
        })
        
    }
}

pub struct Objs {
    pub objas: Vec<ObjA>,
    pub obj0s: Vec<Obj0>,
    pub effect_infos: Vec<EffectInfo>,
    pub pfield_infos: Vec<PFieldInfo>,
    pub gfx_block_infos: Vec<GFXBlockInfo>,
    pub radiosity_vals_infos: Vec<RadiosityValsInfo>,
    pub animation_block_infos: Vec<AnimationBlockInfo>,

    pub models: Vec<Model>,
    pub shapes: Vec<Shape>,
    pub hk_shapes: Vec<HkShape>,
    pub hk_constraints: Vec<HkConstraint>,
    pub effects: Vec<GameObjs>,
    pub gfx_blocks: Vec<Vec<u8>>,
    pub foliages: Vec<Foliage>,
}

struct Key<K, T>(K, T);

impl<K: Ord, T> Ord for Key<K, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<K: PartialOrd, T> PartialOrd for Key<K, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<K: PartialEq, T> PartialEq for Key<K, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K: Eq, T> Eq for Key<K, T> {}

#[make_endian]
pub trait DumpObjs_XE_ {
    fn effect_num(&self) -> usize;
    fn foliage_num(&self) -> usize;
    fn gfx_num(&self) -> usize;
    fn texture_num(&self) -> usize;
    fn effects<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpEffect_XE_ + 'a))>;
    fn models<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpModel_XE_ + 'a))>;
    fn foliages<'a>(&'a self) -> impl Iterator<Item=&'a (impl DumpFoliage_XE_ + 'a)>;
    fn gfxs<'a>(&'a self) -> impl Iterator<Item=(u32, &'a [u8])>;
    fn radiosity(&self) -> &impl DumpRadiosity_XE_;
    fn textures<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpTexture_XE_ + 'a))>;

    fn group_models<'a>(&'a self, level: &impl DumpGameObjs_XE_) -> (impl Iterator<Item=&'a (impl DumpModel_XE_ + 'a)>, impl ExactSizeIterator<Item=&'a (impl DumpModel_XE_ + 'a)>, Option<&'a (impl DumpModel_XE_ + 'a)>) {
        let mut normal = BinaryHeap::new();
        let mut collision_road = BinaryHeap::new();
        let mut terrain = BinaryHeap::new();
        let mut occluder = None;
        for (key, model) in self.models() {
            if key == hash_string(b"occluder", None) {
                occluder.replace(model);
            } else if let Some(name) = get_str(&key) {
                if name.starts_with("Terrain") {
                    terrain.push(std::cmp::Reverse(Key(name.split("_").last().and_then(|x| x.parse::<u32>().ok()).unwrap_or_default(), model)));
                } else if name.contains("_Road_") || name.contains("_Collision_") {
                    collision_road.push(std::cmp::Reverse(Key(name.split("_").last().and_then(|x| x.parse::<u32>().ok()).map(|x| level.guid_order(&x)).unwrap_or_default(), model)));
                } else {
                    normal.push(std::cmp::Reverse(Key(key, model)));
            }
            } else {
                normal.push(std::cmp::Reverse(Key(key, model)));
            }
        }
        (
            normal.into_iter().map(|x| x.0.1)
            .chain(collision_road.into_iter().map(|x| x.0.1)),
            terrain.into_iter().map(|x| x.0.1),
            occluder
        )
    }

    fn dump_into<'a, 'b>(&'b self, dst: &mut DumpSlice<'a>, pak_header: &mut PakHeader_XE_, type_infos: &[TypeInfos], level: &impl DumpGameObjs_XE_, dump_infos: &mut DumpInfos_XE_<'a, 'b>) -> Result<CompressedData<'b>> {

        let radiosity = self.radiosity();

        debug!("objs dump size after infos {}, {}", dst.offset, dump_infos.offsets.ind);
        for ((key, effect), type_infos) in self.effects().sorted_by_key(|x| x.0).zip(type_infos) {
            *dump_infos.offsets.next().context("offsets")? = (dump_infos.effects.offset + std::mem::offset_of!(EffectInfo_XE_, offset)).conv();
            let info = dump_infos.effects.next().context("effect info")?;
            info.key = key.conv();
            info.gamemodemask = effect.gamemodemask();
            let off = dst.offset;
            effect.vals().dump_into(dst, type_infos).with_context(|| format!("effect {}", key))?;
            info.size = (dst.offset - off).conv();
            info.offset = off.conv();
        }
        debug!("objs dump size after effects {}, {}", dst.offset, dump_infos.offsets.ind);

        let (normal, terrain, occluder) = self.group_models(level);
        let mut model_count = 0;
        for model in normal {
            // TODO (better context)
            model.dump_into(dst, dump_infos).with_context(|| format!("model({}) {}", model_count, model.key()))?;
            dst.align(16)?;
            model_count += 1;
        }
        let terrain_offset = dst.offset;
        if terrain.len() != 0 {
            *<[u8; 16]>::mut_from_data(dst).context("terrain indices")? = [0xFFu8; 16];
        }
        for model in terrain {
            // TODO (better context)
            model.dump_terrain_into(dst, dump_infos, terrain_offset).with_context(|| format!("terrain model({}) {}", model_count, model.key()))?; 
            model_count += 1;
        }
        debug!("objs dump size after terrain {}, {}", dst.offset, dump_infos.offsets.ind);

        dst.align(16)?;
        for (i, foliage) in self.foliages().enumerate() {
            foliage.dump_into(dst, dump_infos).with_context(|| format!("foliage {}", i))?;
            dst.align(16)?;
        }
        debug!("objs dump size after foliage {}, {}", dst.offset, dump_infos.offsets.ind);

        if let Some(model) = occluder {
            model.dump_into(dst, dump_infos).context("model occluder")?;
            dst.align(16)?;
        }

        let mut gfx_block_info_offset = dump_infos.gfxs.offset;
        debug!("gfx offs {}, {}", pak_header.gfx_block_info_offset, gfx_block_info_offset);
        for ((key, gfx), info) in self.gfxs().zip(dump_infos.gfxs.take()) {
            info.key = key.conv();
            info.offset = dst.offset.conv();
            info.size = gfx.len().conv();
            gfx.dump_into(dst).with_context(|| format!("gfx {}", key))?;
            dst.align(16)?;
            *dump_infos.offsets.next().context("offsets")? = (gfx_block_info_offset + std::mem::offset_of!(GFXBlockInfo_XE_, offset)).conv();
            gfx_block_info_offset += std::mem::size_of::<GFXBlockInfo_XE_>();
        }
        debug!("objs dump size after gfxs {}, {}", dst.offset, dump_infos.offsets.ind);

        let rad_data = radiosity.dump_into(dst, dump_infos).context("radiosity")?;
        dst.align(16)?;
        debug!("objs dump size after radiosity {}, {}", dst.offset, dump_infos.offsets.ind);

        for (key, texture) in self.textures() {
            texture.dump_infos(dump_infos).with_context(|| format!("texture {}", key))?;
        }

        for i in dump_infos.model_data.iter() {
            println!("{}, is_some: {}", i.key, i.data.is_some());
        }

        // populate offsets

        Ok(rad_data)
    }

    fn add_size(&self, offset: usize, counts: &mut InfoCounts, level: &impl DumpGameObjs_XE_) -> (usize, Vec<TypeInfos>) {
        let mut objs_size = 0;

        let type_infos = self.effects().sorted_by_key(|x| x.0).map(|(_, effect)| {
            let (size, type_infos) = effect.vals().size();
            objs_size += size;
            type_infos
        }).collect();
        counts.offsets += self.effect_num();
        debug!("objs size after effects {}, {}", objs_size, counts.offsets);

        let (normal, terrain, occluder) = self.group_models(level);

        for model in normal {
            objs_size = align_offset(model.add_size(objs_size, counts), 16);
        }
        debug!("objs size after models {}, {}", objs_size, counts.offsets);
        if terrain.len() != 0 {
            objs_size += 16;
        }
        for model in terrain {
            objs_size = model.add_terrain_size(objs_size, counts); 
        }
        debug!("objs size after terrain {}, {}", objs_size, counts.offsets);

        objs_size = align_offset(objs_size, 16);
        for foliage in self.foliages() {
            objs_size = align_offset(foliage.add_size(objs_size), 16);
        }
        counts.offsets += self.foliage_num();
        debug!("objs size after foliage {}, {}", objs_size, counts.offsets);

        if let Some(model) = occluder {
            objs_size = align_offset(model.add_size(objs_size, counts), 16);
        }

        for (_, gfx) in self.gfxs() {
            objs_size = align_offset(objs_size + gfx.len(), 16);
        }
        counts.offsets += self.gfx_num();
        debug!("objs size after gfxs {}, {}", objs_size, counts.offsets);

        objs_size = align_offset(self.radiosity().add_size(objs_size, counts), 16);
        debug!("objs size after radiosity {}, {}", objs_size, counts.offsets);

        counts.textures = self.texture_num();
        counts.effects = self.effect_num();
        counts.gfxs = self.gfx_num();
        counts.foliages = self.foliage_num();

        debug!("info counts {:#?}", counts);
        debug!("textures {} {}", self.texture_num(), std::mem::size_of::<TextureInfo_XE_>());
        (offset + objs_size, type_infos)
    }
}

#[make_endian]
impl<'a> DumpObjs_XE_ for ObjsRef_XE_<'a> {
    fn effect_num(&self) -> usize {
        self.effects.len()
    }
    fn foliage_num(&self) -> usize {
        self.foliages.iter().map(|(_, x)| x.len()).sum::<usize>()
    }
    fn gfx_num(&self) -> usize {
        self.gfxs.len()
    }
    fn texture_num(&self) -> usize {
        self.textures.len()
    }
    fn effects(&self) -> impl Iterator<Item=(u32, &impl DumpEffect_XE_)> {
        self.effects.iter().map(|(k,v)| (*k, v))
    }
    fn models(&self) -> impl Iterator<Item=(u32, &impl DumpModel_XE_)> {
        self.models.iter().map(|(k,v)| (*k, v))
    }
    fn textures(&self) -> impl Iterator<Item=(u32, &impl DumpTexture_XE_)> {
        self.textures.iter().map(|(k, v)| (*k, v))
    }
    fn foliages(&self) -> impl Iterator<Item=&impl DumpFoliage_XE_> {
        self.foliages.iter().flat_map(|(_, x)| x.iter())
    }
    fn gfxs(&self) -> impl Iterator<Item=(u32, &[u8])> {
        self.gfxs.iter().map(|(k,v)| (*k, &v[..]))
    }
    fn radiosity(&self) -> &impl DumpRadiosity_XE_ {
        &self.radiosity
    }
}
