use anyhow::{Context, Result};
use indexmap::IndexMap;
use itertools::Itertools;
use std::collections::BinaryHeap;
use log::debug;

use crate::{
    level::{
        model::{
            Model, DumpModel, ModelRef,
            shape::{Shape, HkShape, HkConstraint}
        },
        pak::{
            block1::{
                infos::{DumpInfos, InfoCounts},
                gameobjs::{DumpGameObjs, GameObjs, TypeInfos}, 
            },
            PakHeader,
        },
        bin::BinRef,
        radiosity::{DumpRadiosity, RadiosityRef},
        texture::{TextureDataInfo, DumpTexture, TextureDataRef},
    },
};
use crate::{
    level::{
        pak::{
            animation::AnimationBlockInfo
        },
        radiosity::{RadiosityValsInfo},
    },
    types::{Crc, DumpSlice, ReadData, Vector4, hash_string, ref_slice, slice, get_str, align_offset, BaseTypes, CrcLE, CompressedData, NE},
};
use lotrc_proc::{derive_pod};

#[derive_pod]
pub struct ObjA<T: BaseTypes> {
    pub key: CrcLE<T>,
    pub unk_1: T::u32LE,
    pub size: T::u32LE,
    pub size_comp: T::u32LE,
    pub unk_4: T::u32LE,
    pub kind: T::u32LE,
}

#[derive_pod]
pub struct Obj0<T: BaseTypes> {
    pub unk_0: T::u32LE,
    pub key: CrcLE<T>,
}

#[derive_pod]
pub struct EffectInfo<T: BaseTypes> {
    pub key: Crc<T>,
    pub gamemodemask: T::i32,
    pub offset: T::u32,
    pub size: T::u32,
}

#[derive_pod]
pub struct PFieldInfo<T: BaseTypes> {
    pub link_guid: T::u32,
    pub gamemode_guid: T::u32,
    pub width: T::u32,
    pub height: T::u32,
    pub offset: T::u32,
}

#[derive_pod]
pub struct GFXBlockInfo<T: BaseTypes> {
    // GFX blocks?, unchanged by encoding, model as data
    pub key: Crc<T>,
    pub offset: T::u32,
    pub size: T::u32,
}

#[derive_pod]
pub struct FoliageInfo<T: BaseTypes> {
    pub key: Crc<T>,
    pub kind: T::u32,
    pub lb_w: T::i32,
    pub lb_h: T::i32,
    pub ub_w: T::i32,
    pub ub_h: T::i32,
    pub scale: T::f32,
    pub offset: T::u32,
    pub key_mesh: Crc<T>,
    pub key_mesh_lod1: Crc<T>,
    pub key_mesh_lod2: Crc<T>,
    pub color: Vector4<T>,
    pub lod1a: T::f32,
    pub lod1b: T::f32,
    pub lod2a: T::f32,
    pub lod2b: T::f32,
    pub lod_max: T::f32,
}

#[derive_pod]
// height: u16, var_mask: u16, slope_x: i16, slope_z: i16
// position and orientation are randomized for each instance (within own square)
// var mask only checks high component vs alpha of vertex attr (if it exists)
pub struct FoliageVal<T: BaseTypes> {
    pub height: T::u16,
    pub var_mask: T::u16,
    pub slope_x: T::i16,
    pub slope_z: T::i16,
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct FoliageRef<'a,  T: BaseTypes> {
    info: &'a FoliageInfo<T>,
    vals: ref_slice<'a, FoliageVal<T>>
}

impl<'a,  T: BaseTypes> FoliageRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a FoliageInfo<T>) -> Result<Self> {
        let n =
            ((info.ub_w.into() - info.lb_w.into()) * (info.ub_h.into() - info.lb_h.into())) as usize;
        let vals = FoliageVal::slice_from_data(&src[info.offset.into() as usize..], n)
            .context("vals")?;
        Ok(Self {
            info: info,
            vals: vals.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Foliage {
    pub info: FoliageInfo<NE>,
    pub vals: Vec<FoliageVal<NE>>,
}

impl<T: BaseTypes> From<&FoliageRef<'_, T>> for Foliage
where
    FoliageInfo<NE>: From<FoliageInfo<T>>,
    FoliageVal<NE>: From<FoliageVal<T>>,
{
    fn from(val: &FoliageRef<T>) -> Self {
        Self {
            info: (*val.info).into(),
            vals: val.vals.iter().map(|&x| x.into()).collect(),
        }
    }
}

pub trait DumpFoliage<T: BaseTypes> {
    fn vals_num(&self) -> usize;
    fn write_vals(&self, vals: &mut [FoliageVal<T>]) -> Result<()>;
    fn dump_into(&self, dst: &mut DumpSlice, dump_infos: &mut DumpInfos<T>) -> Result<()> {
        *dump_infos.offsets.next().context("offsets")? = ((dump_infos.foliages.offset + std::mem::offset_of!(FoliageInfo<T>, offset)) as u32).into();
        let info = dump_infos.foliages.next().context("info")?;
        info.offset = (dst.offset as u32).into();
        let vals = FoliageVal::mut_slice_from_data(dst, self.vals_num()).context("vals")?;
        self.write_vals(vals).context("write vals")
    }
    fn add_size(&self, offset: usize) -> usize {
        offset + self.vals_num() * std::mem::size_of::<FoliageVal<T>>()
    }
}

impl<T: BaseTypes> DumpFoliage<T> for FoliageRef<'_, T> {
    fn vals_num(&self) -> usize {
        self.vals.len()
    }
    fn write_vals(&self, vals: &mut [FoliageVal<T>]) -> Result<()> {
        vals.copy_from_slice(&self.vals[..]);
        Ok(())
    }
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct EffectRef<'a,  T: BaseTypes> {
    info: &'a EffectInfo<T>,
    vals: super::gameobjs::GameObjsRef<'a, T>
}

impl<'a,  T: BaseTypes> EffectRef<'a, T> {
    pub fn from_data(src: &'a [u8], info: &'a EffectInfo<T>) -> Result<Self> {
        Ok(Self {
            info,
            vals: super::gameobjs::GameObjsRef::from_data(&src[info.offset.into() as usize..(info.offset.into() + info.size.into()) as usize])?
        })
    }
}

pub trait DumpEffect<T: BaseTypes> {
    fn vals(&self) -> &impl DumpGameObjs<T>;
    fn gamemodemask(&self) -> T::i32;
}

impl<T: BaseTypes> DumpEffect<T> for EffectRef<'_, T> {
    fn vals(&self) -> &impl DumpGameObjs<T> {
        &self.vals
    }
    fn gamemodemask(&self) -> T::i32 {
        self.info.gamemodemask
    }
}

#[derive_pod]
pub struct TextureInfo<T: BaseTypes> {
    pub key: Crc<T>,
    pub gamemodemask: T::i32,
    pub data_info: TextureDataInfo<T>,
}

#[derive(PartialEq)]
pub struct TextureRef<'a, T: BaseTypes> {
    pub gamemodemask: T::i32,
    pub data: TextureDataRef<'a, T>
}

#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Default, PartialEq)]
pub struct ObjsRef<'a,  T: BaseTypes> {
    pub textures: IndexMap<u32, TextureRef<'a, T>>,
    pub models: IndexMap<u32, ModelRef<'a, T>>,
    pub effects: IndexMap<u32, EffectRef<'a, T>>,
    pub foliages: IndexMap<u32, slice<FoliageRef<'a, T>>>,
    pub gfxs: IndexMap<u32, ref_slice<'a, u8>>,
    pub radiosity: RadiosityRef<'a, T>
}

impl <'a,  T: BaseTypes> ObjsRef<'a, T> {
    pub fn from_data(src: &'a [u8], infos: &super::infos::InfosRef<'a, T>, bin: &BinRef<'a, T>, name: u32) -> Result<Self> {
        let texture_data = &bin.texture_data;
        let textures: IndexMap<_, _> = infos.textures.iter().enumerate()
            .map(|(i, info)| Ok((
                info.key.val.into(),
                TextureRef {
                    gamemodemask: info.gamemodemask,
                    data: TextureDataRef::from_data(&info.data_info, texture_data).with_context(|| format!("texture {}", i))?
                }
            )))
            .collect::<Result<_>>()?;
        let model_data = &bin.model_data;
        let models: IndexMap<_, _> = infos.models.iter().enumerate()
            .map(|(i, info)| Ok((
                info.key.val.into(),
                ModelRef::from_data(src, info, model_data).with_context(|| format!("model {}", i))?
            )))
            .collect::<Result<_>>()?;
        let effects: IndexMap<_, _> = infos.effects
            .iter()
            .map(|info| Ok((
                info.key.val.into(),
                EffectRef::from_data(src, info).with_context(|| format!("effect {}", info.key.val.into()))?
            )))
            .collect::<Result<_>>()?;
        let gfxs: IndexMap<_, _> = infos.gfxs
            .iter()
            .map(|info| (
                info.key.val.into(),
                (&src[info.offset.into() as usize..(info.offset.into() + info.size.into()) as usize]).into(),
            ))
            .collect();
        let mut foliages = IndexMap::<_, Vec<_>>::with_capacity(infos.foliages.len());
        for (i, info) in infos.foliages.iter().enumerate() {
            let foliage = FoliageRef::<T>::from_data(src, info).with_context(|| format!("foliage {}", i))?;
            foliages.entry(foliage.info.key.val.into()).or_default().push(foliage);
        }
        let foliages: IndexMap<_, _> = foliages.into_iter().map(|(k, v)| (k, v.into_boxed_slice().into())).collect();
        let radiosity_name = hash_string(b"_radiosity", Some(name));
        let ind = model_data.get_index_of(&radiosity_name);
        let data = ind.and_then(|x| model_data.get_index(x)).map(|(_, x)| x).copied();
        let usage =  ind.map(|x| bin.model_handles()[x].kind.into()).unwrap_or_default();
        
        let radiosity = RadiosityRef::from_data(src, infos.radiosity_vals, data, usage).context("radiosity")?; 
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
    pub objas: Vec<ObjA<NE>>,
    pub obj0s: Vec<Obj0<NE>>,
    pub effect_infos: Vec<EffectInfo<NE>>,
    pub pfield_infos: Vec<PFieldInfo<NE>>,
    pub gfx_block_infos: Vec<GFXBlockInfo<NE>>,
    pub radiosity_vals_infos: Vec<RadiosityValsInfo<NE>>,
    pub animation_block_infos: Vec<AnimationBlockInfo<NE>>,

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

pub trait DumpObjs<T: BaseTypes> {
    fn effect_num(&self) -> usize;
    fn foliage_num(&self) -> usize;
    fn gfx_num(&self) -> usize;
    fn texture_num(&self) -> usize;
    fn effects<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpEffect<T> + 'a))>;
    fn models<'a>(&'a self) -> impl Iterator<Item=(u32, &'a (impl DumpModel<T> + 'a))>;
    fn foliages<'a>(&'a self) -> impl Iterator<Item=&'a (impl DumpFoliage<T> + 'a)>;
    fn gfxs<'a>(&'a self) -> impl Iterator<Item=(u32, &'a [u8])>;
    fn radiosity(&self) -> &impl DumpRadiosity<T>;
    fn textures<'a>(&'a self) -> impl Iterator<Item=(u32, T::i32, &'a (impl DumpTexture<T> + 'a))>;

    fn group_models<'a>(&'a self, level: &impl DumpGameObjs<T>) -> (impl Iterator<Item=&'a (impl DumpModel<T> + 'a)>, impl ExactSizeIterator<Item=&'a (impl DumpModel<T> + 'a)>, Option<&'a (impl DumpModel<T> + 'a)>) {
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

    fn dump_into<'a, 'b>(&'b self, dst: &mut DumpSlice<'a>, pak_header: &mut PakHeader<T>, type_infos: &[TypeInfos], level: &impl DumpGameObjs<T>, dump_infos: &mut DumpInfos<'a, 'b, T>) -> Result<CompressedData<'b>> {

        let radiosity = self.radiosity();

        debug!("objs dump size after infos {}, {}", dst.offset, dump_infos.offsets.ind);
        for ((key, effect), type_infos) in self.effects().sorted_by_key(|x| x.0).zip(type_infos) {
            *dump_infos.offsets.next().context("offsets")? = ((dump_infos.effects.offset + std::mem::offset_of!(EffectInfo<T>, offset)) as u32).into();
            let info = dump_infos.effects.next().context("effect info")?;
            info.key.val = key.into();
            info.gamemodemask = effect.gamemodemask();
            let off = dst.offset;
            effect.vals().dump_into(dst, type_infos).with_context(|| format!("effect {}", key))?;
            info.size = ((dst.offset - off) as u32).into();
            info.offset = (off as u32).into();
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
        debug!("gfx offs {}, {}", pak_header.gfx_block_info_offset.into(), gfx_block_info_offset);
        for ((key, gfx), info) in self.gfxs().zip(dump_infos.gfxs.take()) {
            info.key.val = key.into();
            info.offset = (dst.offset as u32).into();
            info.size = (gfx.len() as u32).into();
            u8::mut_slice_from_data(dst, gfx.len()).with_context(|| format!("gfx {}", key))?.copy_from_slice(gfx);
            dst.align(16)?;
            *dump_infos.offsets.next().context("offsets")? = ((gfx_block_info_offset + std::mem::offset_of!(GFXBlockInfo<T>, offset)) as u32).into();
            gfx_block_info_offset += std::mem::size_of::<GFXBlockInfo<T>>();
        }
        debug!("objs dump size after gfxs {}, {}", dst.offset, dump_infos.offsets.ind);

        let rad_data = radiosity.dump_into(dst, dump_infos).context("radiosity")?;
        dst.align(16)?;
        debug!("objs dump size after radiosity {}, {}", dst.offset, dump_infos.offsets.ind);

        for (key, gamemodemask, texture) in self.textures() {
            let info = dump_infos.textures.next().context("textures")?;
            info.key.val = key.into();
            info.gamemodemask = gamemodemask;
            texture.dump_infos(&mut info.data_info, dump_infos).with_context(|| format!("texture {}", key))?;
        }

        Ok(rad_data)
    }

    fn add_size(&self, offset: usize, counts: &mut InfoCounts, level: &impl DumpGameObjs<T>) -> (usize, Vec<TypeInfos>) {
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
        debug!("textures {} {}", self.texture_num(), std::mem::size_of::<TextureInfo<T>>());
        (offset + objs_size, type_infos)
    }
}

impl<'a, T: BaseTypes> DumpObjs<T> for ObjsRef<'a, T> {
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
    fn effects(&self) -> impl Iterator<Item=(u32, &impl DumpEffect<T>)> {
        self.effects.iter().map(|(k,v)| (*k, v))
    }
    fn models(&self) -> impl Iterator<Item=(u32, &impl DumpModel<T>)> {
        self.models.iter().map(|(k,v)| (*k, v))
    }
    fn textures(&self) -> impl Iterator<Item=(u32, T::i32, &impl DumpTexture<T>)> {
        self.textures.iter().map(|(k, v)| (*k, v.gamemodemask, &v.data))
    }
    fn foliages(&self) -> impl Iterator<Item=&impl DumpFoliage<T>> {
        self.foliages.iter().flat_map(|(_, x)| x.iter())
    }
    fn gfxs(&self) -> impl Iterator<Item=(u32, &[u8])> {
        self.gfxs.iter().map(|(k,v)| (*k, &v[..]))
    }
    fn radiosity(&self) -> &impl DumpRadiosity<T> {
        &self.radiosity
    }
}
