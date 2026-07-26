use anyhow::{anyhow, Context, Result};
use enum_dispatch::enum_dispatch;
use indexmap::IndexMap;
use log::warn;
use lotrc_proc::{make_endian, derive_ordered_data};

use crate::{
    level::Version,
    types::{compress_segmented, DumpSlice, DumpCompressedData, CompressedData, Crc, hash_string, OrderedData, CompressedDataRef, get_default_ref, DumpData}
};
#[make_endian]
use crate::{
    level::pak::block1::infos::{DumpInfos_XE_, DumpInfoData_XE_},
    types::{Crc_XE_, u32_XE_, u16_XE_, i32_XE_}
};

#[derive_ordered_data]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TextureInfo_XE_ {
    pub key: Crc_XE_,
    pub gamemodemask: i32_XE_,
    pub asset_key: Crc_XE_,
    pub asset_type: u32_XE_,
    pub kind: u32_XE_,
    pub format: u32_XE_,
    pub unk_6: u32_XE_,
    pub unk_7: u32_XE_,
    pub unk_8: u32_XE_,
    pub unk_9: u32_XE_,
    pub unk_10: u32_XE_,
    pub unk_11: u32_XE_,
    pub width: u16_XE_,
    pub height: u16_XE_,
    pub depth: u16_XE_,
    pub levels: u16_XE_,
    pub unk_16_1: u8,
    pub unk_16_2: u8,
    pub unk_16_3: u8,
    pub unk_16_4: u8,
    pub unk_16_5: u8,
    pub unk_16_6: u8,
    pub unk_16_7: u8,
    pub unk_16_8: u8,
    pub unk_16_9: u8,
    pub unk_16_10: u8,
    pub unk_16_11: u8,
    pub unk_16_12: u8,
    pub unk_16_13: u8,
    pub unk_16_14: u8,
    pub unk_16_15: u8,
    pub unk_16_16: u8,
}

macro_rules! mip_texture {
    () => { 0 | 7 | 8 }
}
macro_rules! cube_texture {
    () => { 1 | 9 }
}

/*
type:
    0,7,8 -> Texture
    1,9 -> CubeTexture
    2,10 -> VolumeTexture
    4 -> RenderTarget
    5 -> DepthStencilSurface
    3 -> Nothing
    11 -> Surface

format
    0 -> 0x17 R5G6B5
    1 -> 0x1a A4R4G4B4
    2 -> 0x19 A1R5G5B5
    3, 0x10, 0x12, 0x13, 0x26 -> 0x15 A8R8G8B8
    4 -> 0x16 X8R8G8B8
    defualt -> 0 UNKNOWN
    6 -> 0x1c A8
    7,8 -> 0x31545844 DXT1
    9 -> 0x33545844 DXT3
    10, 0xb, 0xc, 0x11 -> 0x35545844 DXT5
    0x15 -> 0x23 A2R10G10B10
    0x17 -> 0x24 A16B16G16R16
    0x18 -> 0x71 A16B16G16R16F
    0x1b -> 0x4b D24S8
    0x1c -> 0x53 D24FS8
    0x1d -> 0x50 D16
    0x1e -> 0x5a574152 or 0x5a544e49 or 0x34324644, RAWZ or INTZ or DF24
    0x1f, 0x20 -> 0x32 L8
    0x21 -> 0x6f R16F
    0x22 -> 0x70 G16R16F
    0x23 -> 0x72 R32F
    0x24 -> 0x73 G32R32F
    0x25 -> 0x74 A32B32G32R32F
    0x27 -> 0x20 A8B8G8R8
    0x28 -> 0x4c4c554e or 0x15, Null or A8R8G8B8
    13 -> bc4 alpha texture (xbox only, converts to A8 texture on PC)
"""
*/

pub const fn get_stride_width(format: u32) -> Option<(usize, usize)> {
    match format {
        10 | 0xb | 0xc | 0x11 => Some((4, 16)),
        7 | 8 | 13 => Some((4, 8)),
        3 | 4 => Some((1, 4)),
        6 => Some((1, 1)),
        _ => None,
    }
}

pub fn conv_img_a8(
    data: &[u8],
    h: usize,
    w: usize,
    y_off: usize,
    x_off: usize,
    ys: usize,
    xs: usize,
) -> Vec<u8> {
    let mut dst = vec![0u8; xs * ys];
    let w = w >> 5;
    for j in 0..((h*w) >> 4) {
        let off = (j & 3) + ((j >> 1) & !3);
        let x = (off % w) << 5;
        let y = ((off / w) << 5) + ((j << 2) & 16);
        for i in 0..512 {
            let x = x + ((((i >> 4) & 16) + (i >> 3)) & 24) + (i & 7);
            let y = y + ((i >> 5) & 8) + ((i >> 3) & 4) + ((i >> 2) & 2) + ((i >> 4) & 1);
            if x < x_off || x >= x_off + xs || y < y_off || y >= y_off + ys {
                continue;
            }
            let k = (y - y_off) * xs + (x - x_off);
            dst[k] = data[i + (j << 9)]
        }
    }
    dst
}

pub fn conv_img_argb8(
    data: &[u8],
    h: usize,
    w: usize,
    y_off: usize,
    x_off: usize,
    ys: usize,
    xs: usize,
) -> Vec<u8> {
    let mut dst = vec![0u8; xs * ys * 4];
    let w = w >> 5;
    for j in 0..((h*w) >> 5) {
        let x = (j % w) << 5;
        let y = (j / w) << 5;
        for i in 0..1024 {
            let x = x + ((((i >> 4) & 16) + (i >> 1)) & 24) + ((i >> 1) & 4) + (i & 3);
            let y = y + ((i >> 5) & 8) + (((i >> 5) & 16)) + ((i >> 5) & 6) + ((i >> 2) & 1);
            if x < x_off || x >= x_off + xs || y < y_off || y >= y_off + ys {
                continue;
            }
            let i = (i << 2) + (j << 12);
            let j = ((y - y_off) * xs + (x - x_off)) << 2;
            dst[j] = data[i+3];
            dst[j+1] = data[i+2];
            dst[j+2] = data[i+1];
            dst[j+3] = data[i];
        }
    }
    dst
}

pub fn conv_img_dxt1(
    data: &[u8],
    h: usize,
    w: usize,
    y_off: usize,
    x_off: usize,
    ys: usize,
    xs: usize,
) -> Vec<u8> {
    let mut dst = vec![0u8; xs * ys * 8];
    let w = w >> 5;
    for j in 0..((h*w) >> 5) {
        let x = (j % w) << 5;
        let y = (j / w) << 5;
        for i in 0..1024 {
            let x = x + ((((i >> 5) & 16) + i) & 24) + ((i >> 3) & 4) + ((i >> 1) & 2) + (i & 1);
            let y = y + ((i >> 6) & 8) + (((i >> 4) & 16)) + ((i >> 5) & 6) + ((i >> 1) & 1);
            if x < x_off || x >= x_off + xs || y < y_off || y >= y_off + ys {
                continue;
            }
            let mut i = (i << 3) + (j << 13);
            let mut j = ((y - y_off) * xs + (x - x_off)) << 3;
            for _ in 0..4 {
                dst[j] = data[i+1];
                dst[j+1] = data[i];
                j += 2;
                i += 2;
            }
        }
    }
    dst
}

pub fn conv_img_dxt5(
    data: &[u8],
    h: usize,
    w: usize,
    y_off: usize,
    x_off: usize,
    ys: usize,
    xs: usize,
) -> Vec<u8> {
    let mut dst = vec![0u8; xs * ys * 16];
    let w = w >> 5;
    for j in 0..((h*w) >> 5) {
        let x = (j % w) << 5;
        let y = (j / w) << 5;
        for i in 0..1024 {
            let x = x + ((((i >> 5) & 16) + (i << 1)) & 24) + ((i >> 3) & 6) + ((i >> 1) & 1);
            let y = y + ((i >> 6) & 8) + (((i >> 3) & 16)) + ((i >> 6) & 4) + ((i >> 5) & 2) + (i & 1);
            if x < x_off || x >= x_off + xs || y < y_off || y >= y_off + ys {
                continue;
            }
            let mut i = (i << 4) + (j << 14);
            let mut j = ((y - y_off) * xs + (x - x_off)) << 4;
            for _ in 0..8 {
                dst[j] = data[i+1];
                dst[j+1] = data[i];
                j += 2;
                i += 2;
            }
        }
    }
    dst
}

pub fn conv_img_slice(
    data: &[u8],
    h: usize,
    w: usize,
    y_off: usize,
    x_off: usize,
    ys: usize,
    xs: usize,
    d: usize
) -> Vec<u8> {
    match d {
        16 => conv_img_dxt5(data, h, w, y_off, x_off, ys, xs),
        8 => conv_img_dxt1(data, h, w, y_off, x_off, ys, xs),
        4 => conv_img_argb8(data, h, w, y_off, x_off, ys, xs),
        _ => conv_img_a8(data, h, w, y_off, x_off, ys, xs)
    }
}

fn bin_mip(arr: &[u8], w: usize) -> Vec<u8> {
    arr.chunks(w)
        .step_by(2)
        .flat_map(|x| x.iter().step_by(2))
        .cloned()
        .collect()
}

fn decomp_bc4(arr: &[u8], w: usize, h: usize) -> Vec<u8> {
    bcndecode::decode(
        arr,
        w,
        h,
        bcndecode::BcnEncoding::Bc4,
        bcndecode::BcnDecoderFormat::LUM,
    )
    .unwrap()
}

#[make_endian]
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(PartialEq)]
pub struct TextureRef_XE_<'a> {
    pub info: &'a TextureInfo_XE_,
    pub data0: Option<&'a CompressedDataRef<'a>>,
    pub data1: Option<&'a CompressedDataRef<'a>>
}

#[make_endian]
impl Default for TextureRef_XE_<'_> {
    fn default() -> Self {
        Self {
            info: get_default_ref(),
            data0: None,
            data1: None
        }
    }
}

#[make_endian]
impl<'a> TextureRef_XE_<'a> {
    pub fn from_data(info: &'a TextureInfo_XE_, texture_data: &IndexMap<u32, &'a CompressedDataRef<'a>>) -> Result<Self> {
        Ok(Self {
            info,
            data0: texture_data.get(&info.asset_key.to_native()).copied(),
            data1: texture_data.get(&hash_string(b"*", Some(info.asset_key.conv()))).copied()
        })
    }
}

#[make_endian]
impl<'a> TextureRef_XE_<'a> {
    fn parse(&self, version: Version) -> Result<Box<[&'a [u8]]>> {
        let data0 = self.data0.map(|x| &x.data_decomp[..]).unwrap_or_default();
        let data1 = self.data1.map(|x| &x.data_decomp[..]).unwrap_or_default();
        Ok(match self.info.kind.conv() {
            0u32 | 7 | 8 => parse_texture_xe_(self.info, data0, data1, version).context("texture data")?,
            1 | 9 => parse_cube_xe_(self.info, data0, data1, version).context("cube data")?,
            _ => {
                warn!(
                    "Unsupported Texture Type {} for texture {:?}",
                    self.info.kind, self.info.key
                );
                vec![data0, data1].into()
            }
        })
    }
}

#[make_endian]
fn parse_texture_xe_<'a>(info: &TextureInfo_XE_, data0: &'a [u8], data1: &'a [u8], version: Version) -> Result<Box<[&'a [u8]]>> {
    let (s, d) = match get_stride_width(info.format.conv()) {
        Some((s, d)) => (s as usize, d as usize),
        None => {
            warn!("Unhandled Texture Format {}", info.format);
            return Ok(Box::new([]));
        }
    };

    let mut width: usize = info.width.conv();
    let mut height: usize = info.height.conv();

    let min_size = if version.is_xbox() && s != 1 { 128 / s } else { 1 };

    let mut levels = Vec::with_capacity(info.levels.conv());
    if info.levels == 1u16 {
        let expected_size = (width / s).max(min_size) * (height / s).max(min_size) * d;
        if data1.len() != expected_size {
            return Err(anyhow!("expected texture data to be of size {} but got {}", expected_size, data1.len()));
        }
        levels.push(&data1[..]);
    } else {
        if !version.is_xbox() || (info.width > 16u16 && info.height > 16u16) {
            let expected_size = (width / s).max(min_size) * (height / s).max(min_size) * d;
            if data0.len() != expected_size {
                return Err(anyhow!("expected texture data to be of size {} but got {}", expected_size, data0.len()));
            }
            levels.push(&data0[..]);
        } else {
            width = width * 2;
            height = height * 2;
        }
        let mut offset = 0;
        for _ in 1usize..info.levels.conv() {
            width /= 2;
            height /= 2;
            let size = (width / s).max(min_size) * (height / s).max(min_size) * d;
            levels.push(&data1[offset..offset + size]);
            offset += size;
            if version.is_xbox() && (width == 16 || height == 16) {
                break;
            }
        }
        if offset != data1.len() {
            return Err(anyhow!("expected texture data to be of size {} but got {}", offset, data0.len()));
        }
    }
    Ok(levels.into())
}

#[make_endian]
fn parse_cube_xe_<'a>(info: &TextureInfo_XE_, data0: &'a [u8], data1: &'a [u8], version: Version) -> Result<Box<[&'a [u8]]>> {
    if info.levels > 1 {
        return Err(anyhow!("Cube Textures with > 1 level are unhanded"));
    }
    let (s, d) = match get_stride_width(info.format.conv()) {
        Some((s, d)) => (s as usize, d as usize),
        None => {
            warn!("Unhandled Cube Texture Format {}", info.format);
            return Ok(Box::new([]));
        }
    };

    let min_size = if version.is_xbox() { 128 / s } else { 1 };

    let mut faces = Vec::with_capacity(6);

    if data0.len() != 0 {
        return Err(anyhow!("Cube Texture exepects first data to be empty but got {} bytes", data0.len()));
    }

    let data_size: usize = (info.width.to_native() as usize / s).max(min_size)
        * (info.height.to_native() as usize / s).max(min_size)
        * d;
    for i in 0..6 {
        faces.push(&data1[data_size * i..data_size * i + data_size]);
    }
    Ok(faces.into())
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub info: TextureInfo,
    pub data: Vec<Vec<u8>>,
}

impl Texture {
    pub fn dump(&self) -> (Vec<u8>, Vec<u8>) {
        match self.info.kind {
            mip_texture!() => {
                if self.data.len() == 1 {
                    (vec![], self.data[0].clone())
                } else {
                    (
                        self.data[0].clone(),
                        self.data[1..].iter().flatten().copied().collect(),
                    )
                }
            }
            cube_texture!() => {
                // cube texture
                (vec![], self.data.iter().flatten().copied().collect())
            }
            _ => (self.data[0].clone(), self.data[1].clone()),
        }
    }
    pub fn data0(&self) -> impl Iterator<Item=&Vec<u8>> {
        match self.info.kind {
            mip_texture!() => self.data.iter().take(if self.data.len() == 1 { 0 } else { 1 }),
            cube_texture!() => self.data.iter().take(0),
            _ => self.data.iter().take(1)
        }
    }
    pub fn data1(&self) -> impl Iterator<Item=&Vec<u8>> {
        match self.info.kind {
            mip_texture!() => self.data.iter().skip(if self.data.len() == 1 { 0 } else { 1 }),
            cube_texture!() => self.data.iter().skip(0),
            _ => self.data.iter().skip(1)
        }
    }
}

#[make_endian]
impl TextureRef_XE_<'_> {
    fn to_full(&self, version: Version) -> anyhow::Result<Texture> {
        let mut info: TextureInfo = self.info.conv();
        let val_data = self.parse(version).context("texture data")?;
        let mut data = val_data 
            .iter()
            .map(|x| x.to_vec())
            .collect();
        if version.is_xbox() {
            let (s, d) = match get_stride_width(info.format) {
                Some((s, d)) => (s as usize, d as usize),
                None => {
                    warn!(
                        "Unhandled Texture Format {} for texture {:?}",
                        info.format, info.key
                    );
                    return Ok(Texture { info, data });
                }
            };
            let min_size = if s != 1 { 128 / s } else { 1 };
            let mut width = info.width as usize;
            let mut height = info.height as usize;

            match info.kind {
                0 | 7 | 8 => {
                    // mip texture
                    if info.levels == 1 {
                        data.push(conv_img_slice(
                            val_data[0],
                            (height / s).max(min_size),
                            (width / s).max(min_size),
                            0,
                            0,
                            (height / s).max(1),
                            (width / s).max(1),
                            d,
                        ));
                        return Ok(Texture { info, data });
                    }
                    let wide_img = info.width > info.height;
                    let mut level = 0;
                    for vals in &val_data[..val_data.len() - 1] {
                        data.push(conv_img_slice(
                            vals,
                            (height / s).max(min_size),
                            (width / s).max(min_size),
                            0,
                            0,
                            (height / s).max(1),
                            (width / s).max(1),
                            d,
                        ));
                        width /= 2;
                        height /= 2;
                        level += 1;
                    }
                    let packed_data: &[u8] = val_data.last().unwrap();
                    while height >= 4 && width >= 4 && level < info.levels {
                        data.push(if wide_img {
                            conv_img_slice(
                                packed_data,
                                (height / s).max(min_size),
                                (width / s).max(min_size),
                                height / s,
                                0,
                                (height / s).max(1),
                                (width / s).max(1),
                                d,
                            )
                        } else {
                            conv_img_slice(
                                packed_data,
                                (height / s).max(min_size),
                                (width / s).max(min_size),
                                0,
                                width / s,
                                (height / s).max(1),
                                (width / s).max(1),
                                d,
                            )
                        });
                        width /= 2;
                        height /= 2;
                        level += 1;
                    }
                    for _ in level..info.levels {
                        data.push(if wide_img {
                            conv_img_slice(
                                packed_data,
                                (height / s).max(min_size),
                                (width / s).max(min_size),
                                0,
                                width.max(1),
                                (height / s).max(1),
                                (width / s).max(1),
                                d,
                            )
                        } else {
                            conv_img_slice(
                                packed_data,
                                (height / s).max(min_size),
                                (width / s).max(min_size),
                                height.max(1),
                                0,
                                (height / s).max(1),
                                (width / s).max(1),
                                d,
                            )
                        });
                        width /= 2;
                        height /= 2;
                    }
                    if info.format == 13 {
                        info.format = 6;
                        width = info.width as usize;
                        height = info.height as usize;
                        data = data
                            .into_iter()
                            .enumerate()
                            .map(|(i, x)| decomp_bc4(&x, (width >> i).max(4), (height >> i).max(4)))
                            .collect();
                        if info.levels != 1 {
                            data[info.levels as usize - 2] = bin_mip(
                                &data[info.levels as usize - 3][..],
                                width >> (info.levels as usize - 3),
                            );
                            data[info.levels as usize - 1] = bin_mip(
                                &data[info.levels as usize - 2][..],
                                width >> (info.levels as usize - 2),
                            );
                        }
                    }
                }
                1 | 9 => {
                    // cube texture
                    data.extend(val_data.iter().map(|x| {
                        conv_img_slice(
                            x.as_ref(),
                            (height / s).max(min_size),
                            (width / s).max(min_size),
                            0,
                            0,
                            (height / s).max(1),
                            (width / s).max(1),
                            d,
                        )
                    }));
                }
                _ => warn!(
                    "Unsupported Texture Type {} for texture {:?}",
                    info.kind, info.key
                ),
            }
        }
        Ok(Texture { info, data })
    }
}

#[make_endian]
#[enum_dispatch(DumpTexture_XE_)]
pub enum Texture_XE_<'a> {
    Ref(TextureRef_XE_<'a>),
    Owned(Texture)
}

#[make_endian]
pub struct DumpTexture0<'a>(&'a Texture, Vec<u8>);

impl DumpCompressedData for DumpTexture0<'_> {
    fn compress(&mut self, is_pak: bool, c: flate2::Compression) -> Result<()> {
        self.1 = compress_segmented(self.0.data0().map(|x| x.as_slice()), is_pak, c)?;
        Ok(())
    }
    fn size_comp(&self) -> usize {
        self.1.len()
    }
    fn size(&self) -> usize {
        self.0.data0().map(|x| x.len()).sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        self.1.dump_into(dst)
    }
}

#[make_endian]
pub struct DumpTexture1<'a>(&'a Texture, Vec<u8>);

impl DumpCompressedData for DumpTexture1<'_> {
    fn compress(&mut self, is_pak: bool, c: flate2::Compression) -> Result<()> {
        self.1 = compress_segmented(self.0.data1().map(|x| x.as_slice()), is_pak, c)?;
        Ok(())
    }
    fn size_comp(&self) -> usize {
        self.1.len()
    }
    fn size(&self) -> usize {
        self.0.data1().map(|x| x.len()).sum::<usize>()
    }
    fn dump_into(&self, dst: &mut DumpSlice) -> Result<()> {
        self.1.dump_into(dst)
    }
}

#[make_endian]
#[enum_dispatch]
pub trait DumpTexture_XE_ {
    fn key(&self) -> u32;
    fn asset_info(&self) -> (u32_XE_, u32_XE_);
    fn write_info(&self, info: &mut TextureInfo_XE_) -> Result<()>;
    fn data0(&self) -> CompressedData<'_>;
    fn data1(&self) -> CompressedData<'_>;

    fn dump_infos<'d>(&'d self, infos: &mut DumpInfos_XE_<'_, 'd>) -> Result<()> {
        let info = infos.textures.next().context("textures")?;
        self.write_info(info).context("write info")?;
        let data0 = self.data0();
        let data1 = self.data1();
        let (key0, ty) = self.asset_info();
        let key1 = hash_string("*".as_bytes(), Some(key0.conv()));
        if data1.size() == 0 {
            infos.texture_data.push(DumpInfoData_XE_ {
                key: key1.conv(),
                kind: ty,
                data: data1
            });
            infos.texture_data.push(DumpInfoData_XE_ {
                key: key0,
                kind: ty,
                data: data0
            });
        } else {
            infos.texture_data.push(DumpInfoData_XE_ {
                key: key0,
                kind: ty,
                data: data0
            });
            infos.texture_data.push(DumpInfoData_XE_ {
                key: key1.conv(),
                kind: ty,
                data: data1
            });
        }

        Ok(())
    }
}

#[make_endian]
impl<'a> DumpTexture_XE_ for TextureRef_XE_<'a> {
    fn key(&self) -> u32 {
        self.info.key.conv()
    }
    fn asset_info(&self) -> (u32_XE_, u32_XE_) {
        (self.info.asset_key, self.info.asset_type)
    }
    fn write_info(&self, info: &mut TextureInfo_XE_) -> Result<()> {
        info.write_from(self.info)
    }
    fn data0(&self) -> CompressedData<'_> {
        self.data0.into()
    }
    fn data1(&self) -> CompressedData<'_> {
        self.data1.into()
    }
}

#[make_endian]
impl DumpTexture_XE_ for Texture {
    fn key(&self) -> u32 {
        self.info.key.get()
    }
    fn asset_info(&self) -> (u32_XE_, u32_XE_) {
        (self.info.asset_key.get().into(), self.info.asset_type.into())
    }
    fn write_info(&self, info: &mut TextureInfo_XE_) -> Result<()> {
        *info = self.info.conv();
        Ok(())
    }
    fn data0(&self) -> CompressedData<'_> {
        DumpTexture0(self, vec![]).into()
    }
    fn data1(&self) -> CompressedData<'_> {
        DumpTexture1(self, vec![]).into()
    }
}

#[make_endian]
pub trait DumpTextures_XE_ {
    fn num(&self) -> usize;
    fn write_infos(&self, infos: &mut [TextureInfo_XE_]);
}
