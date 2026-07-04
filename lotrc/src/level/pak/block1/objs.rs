use anyhow::{Context, Result};
use indexmap::IndexMap;
use itertools::Itertools;
use std::collections::BinaryHeap;
use log::debug;

#[make_platforms]
use crate::{
    level::{
        model::{ModelRefVER, DumpModelVER },
        pak::{
            block1::{
                infos::{InfosRefVER, DumpInfosVER},
                gameobjs::{GameObjsRefVER, DumpGameObjsVER}, 
            },
            PakHeaderVER,
        },
        bin::{BinRefVER},
        radiosity::{RadiosityRefVER, DumpRadiosityVER},
        texture::{TextureInfoVER, TextureRefVER, DumpTextureVER},
    },
    types::{u32VER, CrcVER, i32VER, f32VER, i16VER, u16VER, Vector4VER},
};
use crate::types::GetNative;
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
    types::{CompressedData, Crc, DumpData, DumpSlice, RefFromData, Vector4, hash_string, OrderedData, OrderedDataStrict, ref_slice, slice, get_str, align_offset},
};
use lotrc_proc::{make_platforms, OrderedData};

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct ObjA {
    #[ordered_data(PC)]
    pub key: Crc,
    #[ordered_data(PC)]
    pub unk_1: u32,
    #[ordered_data(PC)]
    pub size: u32,
    #[ordered_data(PC)]
    pub size_comp: u32,
    #[ordered_data(PC)]
    pub unk_4: u32,
    #[ordered_data(PC)]
    pub kind: u32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct Obj0 {
    #[ordered_data(PC)]
    pub unk_0: u32,
    #[ordered_data(PC)]
    pub key: Crc,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct EffectInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub offset: u32,
    pub size: u32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct PFieldInfo {
    pub link_guid: u32,
    pub gamemode_guid: u32,
    pub width: u32,
    pub height: u32,
    pub offset: u32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct GFXBlockInfo {
    // GFX blocks?, unchanged by encoding, model as data
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
}

///gen_ffi:export
#[derive(Debug, Default, Clone, OrderedData)]
pub struct FoliageInfo {
    pub key: Crc,
    pub kind: u32,
    pub lb_w: i32,
    pub lb_h: i32,
    pub ub_w: i32,
    pub ub_h: i32,
    pub scale: f32,
    pub offset: u32,
    pub key_mesh: Crc,
    pub key_mesh_lod1: Crc,
    pub key_mesh_lod2: Crc,
    pub color: Vector4,
    pub lod1a: f32,
    pub lod1b: f32,
    pub lod2a: f32,
    pub lod2b: f32,
    pub lod_max: f32,
}

///gen_ffi:export
#[derive(Debug, Clone, Default, OrderedData)]
// height: u16, var_mask: u16, slope_x: i16, slope_z: i16
// position and orientation are randomized for each instance (within own square)
// var mask only checks high component vs alpha of vertex attr (if it exists)
pub struct FoliageVal {
    pub height: u16,
    pub var_mask: u16,
    pub slope_x: i16,
    pub slope_z: i16,
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct FoliageRefVER<'a> {
    info: &'a FoliageInfoVER,
    vals: ref_slice<'a, FoliageValVER>
}

#[make_platforms]
impl<'a> FoliageRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a FoliageInfoVER) -> Result<Self> {
        let n =
            ((info.ub_w.get() - info.lb_w.get()) * (info.ub_h.get() - info.lb_h.get())) as usize;
        let vals = FoliageValVER::slice_from_data(&src[info.offset.get() as usize..], n)
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

#[make_platforms]
impl From<&FoliageRefVER<'_>> for Foliage {
    fn from(val: &FoliageRefVER) -> Self {
        Self {
            info: val.info.conv(),
            vals: val.vals.iter().map(|x| x.conv()).collect(),
        }
    }
}

#[make_platforms]
pub trait DumpFoliageVER {
    fn vals_num(&self) -> usize;
    fn write_vals(&self, vals: &mut [FoliageValVER]) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice, dump_infos: &mut DumpInfosVER) -> Result<()> {
        *dump_infos.offsets.next().context("offsets")? = (dump_infos.foliages.offset + std::mem::offset_of!(FoliageInfoVER, offset)).conv();
        let info = dump_infos.foliages.next().context("info")?;
        info.offset = dst.offset.conv();
        let vals = FoliageValVER::mut_slice_from_data(dst, self.vals_num()).context("vals")?;
        self.write_vals(vals).context("write vals")
    }
    fn add_size(&self, offset: usize) -> usize {
        offset + self.vals_num() * std::mem::size_of::<FoliageValVER>()
    }
}

#[make_platforms]
impl DumpFoliageVER for FoliageRefVER<'_> {
    fn vals_num(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [FoliageValVER]) -> Result<()> {
        vals.write_from(&self.vals[..])
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct EffectRefVER<'a> {
    info: &'a EffectInfoVER,
    vals: GameObjsRefVER<'a>
}

#[make_platforms]
impl<'a> EffectRefVER<'a> {
    pub fn from_data(src: &'a [u8], info: &'a EffectInfoVER) -> Result<Self> {
        Ok(Self {
            info,
            vals: GameObjsRefVER::from_data(&src[info.offset.get() as usize..(info.offset + info.size) as usize])?
        })
    }
}

#[make_platforms]
pub trait DumpEffectVER {
    fn vals(&self) -> &impl DumpGameObjsVER;
    fn gamemodemask(&self) -> i32VER;
}

#[make_platforms]
impl DumpEffectVER for EffectRefVER<'_> {
    fn vals(&self) -> &impl DumpGameObjsVER {
        &self.vals
    }
    fn gamemodemask(&self) -> i32VER {
        self.info.gamemodemask
    }
}

#[make_platforms]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Default, PartialEq)]
pub struct ObjsRefVER<'a> {
    pub textures: IndexMap<u32, TextureRefVER<'a>>,
    pub models: IndexMap<u32, ModelRefVER<'a>>,
    pub effects: IndexMap<u32, EffectRefVER<'a>>,
    pub foliages: IndexMap<u32, slice<FoliageRefVER<'a>>>,
    pub gfxs: IndexMap<u32, ref_slice<'a, u8>>,
    pub radiosity: RadiosityRefVER<'a>,
}

#[make_platforms]
impl<'a> ObjsRefVER<'a> {
    pub fn from_data(src: &'a [u8], infos: &InfosRefVER<'a>, bin: &BinRefVER<'a>, name: u32) -> Result<Self> {
        let texture_data = &bin.texture_data;
        let textures: IndexMap<_, _> = infos.textures.iter().enumerate()
            .map(|(i, info)| Ok((
                info.key.get(),
                TextureRefVER::from_data(info, texture_data).with_context(|| format!("texture {}", i))?
            )))
            .collect::<Result<_>>()?;
        let model_data = &bin.model_data;
        let models: IndexMap<_, _> = infos.models.iter().enumerate()
            .map(|(i, info)| Ok((
                info.key.get(),
                ModelRefVER::from_data(src, info, model_data).with_context(|| format!("model {}", i))?
            )))
            .collect::<Result<_>>()?;
        let effects: IndexMap<_, _> = infos.effects
            .iter()
            .map(|info| Ok((
                info.key.get(),
                EffectRefVER::from_data(src, info).with_context(|| format!("effect {}", info.key.get()))?
            )))
            .collect::<Result<_>>()?;
        let gfxs: IndexMap<_, _> = infos.gfxs
            .iter()
            .map(|info| (
                info.key.get(),
                (&src[info.offset.get() as usize..(info.offset.get() + info.size.get()) as usize]).into(),
            ))
            .collect();
        let mut foliages = IndexMap::<u32, Vec<FoliageRefVER>>::with_capacity(infos.foliages.len());
        for (i, info) in infos.foliages.iter().enumerate() {
            let foliage = FoliageRefVER::from_data(src, info).with_context(|| format!("foliage {}", i))?;
            foliages.entry(foliage.info.key.get()).or_default().push(foliage);
        }
        let foliages: IndexMap<_, _> = foliages.into_iter().map(|(k, v)| (k, v.into_boxed_slice().into())).collect();
        let radiosity_name = hash_string(b"_radiosity", Some(name));
        let ind = model_data.get_index_of(&radiosity_name);
        let data = ind.and_then(|x| model_data.get_index(x)).map(|(_, x)| x).copied();
        let usage =  ind.map(|x| bin.model_handles()[x].kind.get()).unwrap_or_default();
        
        let radiosity = RadiosityRefVER::from_data(src, infos.radiosity_vals, data, usage).context("radiosity")?; 
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

#[make_platforms]
pub trait DumpObjsVER {
    fn effect_num(&self) -> usize;
    fn foliage_num(&self) -> usize;
    fn gfx_num(&self) -> usize;
    fn texture_num(&self) -> usize;
    fn effects<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpEffectVER + 'a))>;
    fn models<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpModelVER + 'a))>;
    fn foliages<'a>(&'a self) -> impl Iterator<Item=&'a (impl DumpFoliageVER + 'a)>;
    fn gfxs<'a>(&'a self) -> impl Iterator<Item=(u32, &'a [u8])>;
    fn radiosity(&self) -> &impl DumpRadiosityVER;
    fn textures<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpTextureVER + 'a))>;

    fn group_models<'a>(&'a self, level: &impl DumpGameObjsVER) -> (impl Iterator<Item=&'a (impl DumpModelVER + 'a)>, impl ExactSizeIterator<Item=&'a (impl DumpModelVER + 'a)>, Option<&'a (impl DumpModelVER + 'a)>) {
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

    fn dump_into<'a, 'b>(&'b self, dst: &mut DumpSlice<'a>, pak_header: &mut PakHeaderVER, type_infos: &[TypeInfos], level: &impl DumpGameObjsVER, dump_infos: &mut DumpInfosVER<'a, 'b>) -> Result<CompressedData<'b>> {

        let radiosity = self.radiosity();

        debug!("objs dump size after infos {}, {}", dst.offset, dump_infos.offsets.ind);
        for ((key, effect), type_infos) in self.effects().sorted_by_key(|x| x.0).zip(type_infos) {
            *dump_infos.offsets.next().context("offsets")? = (dump_infos.effects.offset + std::mem::offset_of!(EffectInfoVER, offset)).conv();
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
            *dump_infos.offsets.next().context("offsets")? = (gfx_block_info_offset + std::mem::offset_of!(GFXBlockInfoVER, offset)).conv();
            gfx_block_info_offset += std::mem::size_of::<GFXBlockInfoVER>();
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

    fn add_size(&self, offset: usize, counts: &mut InfoCounts, level: &impl DumpGameObjsVER) -> (usize, Vec<TypeInfos>) {
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
        debug!("textures {} {}", self.texture_num(), std::mem::size_of::<TextureInfoVER>());
        (offset + objs_size, type_infos)
    }
}

#[make_platforms]
impl<'a> DumpObjsVER for ObjsRefVER<'a> {
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
    fn effects(&self) -> impl Iterator<Item=(u32, &impl DumpEffectVER)> {
        self.effects.iter().map(|(k,v)| (*k, v))
    }
    fn models(&self) -> impl Iterator<Item=(u32, &impl DumpModelVER)> {
        self.models.iter().map(|(k,v)| (*k, v))
    }
    fn textures(&self) -> impl Iterator<Item=(u32, &impl DumpTextureVER)> {
        self.textures.iter().map(|(k, v)| (*k, v))
    }
    fn foliages(&self) -> impl Iterator<Item=&impl DumpFoliageVER> {
        self.foliages.iter().flat_map(|(_, x)| x.iter())
    }
    fn gfxs(&self) -> impl Iterator<Item=(u32, &[u8])> {
        self.gfxs.iter().map(|(k,v)| (*k, &v[..]))
    }
    fn radiosity(&self) -> &impl DumpRadiosityVER {
        &self.radiosity
    }
}
