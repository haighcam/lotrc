#[cfg(feature = "python")]
use crate::pyobj_ref;
use anyhow::{anyhow, Context, Result};
use log::warn;
#[cfg(not(feature = "python"))]
use lotrc_proc::getter;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::ptr::NonNull;
use std::sync::Arc;
use indexmap::IndexMap;

use crate::types::{Crc, hash_string, RefFromData, DumpData, CompressedDataRef};
use lotrc_proc::{make_platforms, OrderedData};

#[cfg(feature = "python")]
pub fn init(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "texture")?;
    m.add_class::<Texture>()?;
    m.add_class::<TextureInfo>()?;
    init_pc(&m)?;
    init_xbox(&m)?;
    init_ps3(&m)?;
    Ok(m)
}
#[cfg(feature = "python")]
#[make_platforms]
pub fn init_ver(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TextureVER>()
}

#[derive(Debug, Default, Clone, OrderedData)]
#[cfg_attr(feature = "python", pyclass(module = "texture", get_all, set_all))]
pub struct TextureInfo {
    pub key: Crc,
    pub gamemodemask: i32,
    pub asset_key: Crc,
    pub asset_type: u32,
    pub kind: u32,
    pub format: u32,
    pub unk_6: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: u32,
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub levels: u16,
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

const fn xg_address2d_tiled_xy(offset: u32, width: u32, texel_pitch: u32) -> (usize, usize) {
    // https://github.com/NCDyson/RareView/blob/master/RareView/Texture.cs
    let aligned_width = (width + 31) & !31;

    let log_bpp = (texel_pitch >> 2) + ((texel_pitch >> 1) >> (texel_pitch >> 2));
    let offset_b = offset << log_bpp;
    let offset_t = ((offset_b & !4095) >> 3) + ((offset_b & 1792) >> 2) + (offset_b & 63);
    let offset_m = offset_t >> (7 + log_bpp);

    let macro_x = (offset_m % (aligned_width >> 5)) << 2;
    let tile_x = (((offset_t >> (5 + log_bpp)) & 2) + (offset_b >> 6)) & 3;
    let macro_x = (macro_x + tile_x) << 3;
    let micro_x =
        ((((offset_t >> 1) & !15) + (offset_t & 15)) & ((texel_pitch << 3) - 1)) >> log_bpp;

    let macro_y = (offset_m / (aligned_width >> 5)) << 2;
    let tile_y = ((offset_t >> (6 + log_bpp)) & 1) + ((offset_b & 2048) >> 10);
    let macro_y = (macro_y + tile_y) << 3;
    let micro_y = (((offset_t & (((texel_pitch << 6) - 1) & !31)) + ((offset_t & 15) << 1))
        >> (3 + log_bpp))
        & !1;

    (
        (macro_x + micro_x) as usize,
        (macro_y + micro_y + ((offset_t & 16) >> 4)) as usize,
    )
}

/*
1
for j in range(h*width//512):
    off = (j & 3) + ((j >> 1) & ~3)
    x_off = ((off % w) << 5)
    y_off = ((off // w) << 5) + ((j << 2) & 16)
    for i in range(512):
        offset = i + j * 512
        x = x_off + ((((i >> 4) & 16) + (i >> 3)) & 24) + (i & 7);
        y = y_off + ((i >> 5) & 8) + ((i >> 3) & 4) + ((i >> 2) & 2) + ((i >> 4) & 1);
        assert((x,y) == xg_address2d_tiled_xy_1(offset, width))

4
for j in range(h*width//1024):
    x_off = ((j % w) << 5)
    y_off = ((j // w) << 5)
    for i in range(1024):
        offset = i + j * 1024
        x = x_off + ((((i >> 4) & 16) + (i >> 1)) & 24) + ((i >> 1) & 4) + (i & 3);
        y = y_off + ((i >> 5) & 8) + (((i >> 5) & 16)) + ((i >> 5) & 6) + ((i >> 2) & 1);
        assert((x,y) == xg_address2d_tiled_xy_4(offset, width))

8
for j in range(h*width//1024):
    x_off = ((j % w) << 5)
    y_off = ((j // w) << 5)
    for i in range(1024):
        offset = i + j * 1024
        x = x_off + ((((i >> 5) & 16) + i) & 24) + ((i >> 3) & 4) + ((i >> 1) & 2) + (i & 1);
        y = y_off + ((i >> 6) & 8) + (((i >> 4) & 16)) + ((i >> 5) & 6) + ((i >> 1) & 1);
        assert((x,y) == xg_address2d_tiled_xy_8(offset, width))

16
for j in range(h*width//1024):
    x_off = ((j % w) << 5)
    y_off = ((j // w) << 5)
    for i in range(1024):
        offset = i + j * 1024
        x = x_off + ((((i >> 5) & 16) + (i << 1)) & 24) + ((i >> 3) & 6) + ((i >> 1) & 1);
        y = y_off + ((i >> 6) & 8) + (((i >> 3) & 16)) + ((i >> 6) & 4) + ((i >> 5) & 2) + (i & 1);
        assert((x,y) == xg_address2d_tiled_xy(offset, width, 16))

*/

pub fn conv_img_a8(
    data: &[u8],
    dst: &mut[u8],
    h: usize,
    w: usize,
    y_off: usize,
    x_off: usize,
    ys: usize,
    xs: usize,
) {
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
            let j = (y - y_off) * xs + (x - x_off);
            dst[j] = data[i]
        }
    }
}

pub fn conv_img_argb8(
    data: &[u8],
    dst: &mut[u8],
    h: usize,
    w: usize,
    y_off: usize,
    x_off: usize,
    ys: usize,
    xs: usize,
) {
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
            let j = ((y - y_off) * xs + (x - x_off)) << 2;
            dst[j] = data[i+3];
            dst[j+1] = data[i+2];
            dst[j+2] = data[i+1];
            dst[j+3] = data[i];
        }
    }
}

pub fn conv_img_dxt1(
    data: &[u8],
    dst: &mut[u8],
    h: usize,
    w: usize,
    y_off: usize,
    x_off: usize,
    ys: usize,
    xs: usize,
) {
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
            let mut j = ((y - y_off) * xs + (x - x_off)) << 3;
            for _ in 0..4 {
                dst[j] = data[j+1];
                dst[j+1] = data[j];
                j += 2;
            }
        }
    }
}

pub fn conv_img_dxt5(
    data: &[u8],
    dst: &mut[u8],
    h: usize,
    w: usize,
    y_off: usize,
    x_off: usize,
    ys: usize,
    xs: usize,
) {
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
            let mut j = ((y - y_off) * xs + (x - x_off)) << 4;
            for _ in 0..8 {
                dst[j] = data[j+1];
                dst[j+1] = data[j];
                j += 2;
            }
        }
    }
}

pub fn conv_img_slice(
    data: &[u8],
    h: usize,
    w: usize,
    y_off: usize,
    x_off: usize,
    ys: usize,
    xs: usize,
    s: usize,
    d: usize,
) -> Vec<u8> {
    // https://github.com/NCDyson/RareView/blob/master/RareView/Texture.cs
    let mut out_data = vec![0u8; xs * ys * d];
    for i in 0..(h * w) {
        let (x, y) = xg_address2d_tiled_xy(i as u32, w as u32, d as u32);
        if x < x_off || x >= x_off + xs || y < y_off || y >= y_off + ys {
            continue;
        }
        let j = (y - y_off) * xs + (x - x_off);
        out_data[j * d..(j + 1) * d].copy_from_slice(&data[i * d..(i + 1) * d]);
    }
    match (s, d) {
        (4, _) => out_data.chunks_mut(2).for_each(|x| x.swap(0, 1)),
        (_, 4) => out_data.chunks_mut(4).for_each(|x| {
            x.swap(0, 3);
            x.swap(1, 2);
        }),
        _ => (),
    }
    out_data
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

// needs reworking
#[make_platforms]
#[cfg_attr(feature = "python", pyclass(module = "texture"))]
#[derive(Debug, Clone)]
pub struct TextureVER {
    _ptr: Arc<[u8]>,
    info: NonNull<TextureInfoVER>,
    data0: Arc<[u8]>,
    data1: Arc<[u8]>,
    data: Box<[NonNull<[u8]>]>,
}

#[make_platforms]
pub struct TextureRawVER {
    src: Arc<[u8]>,
    info: NonNull<TextureInfoVER>,
    data0: CompressedDataRef,
    data1: CompressedDataRef,
}

#[cfg(feature = "python")]
#[make_platforms]
pyobj_ref!(TextureVER);

#[make_platforms]
unsafe impl Sync for TextureVER {}
#[make_platforms]
unsafe impl Send for TextureVER {}

#[make_platforms]
impl TryFrom<TextureRawVER> for TextureVER {
    type Error = anyhow::Error;
    fn try_from(TextureRawVER { src, info, data0, data1 }: TextureRawVER) -> Result<Self> {
        let data0 = data0.get().context("data0")?;
        let data1 = data1.get().context("data1")?; 
        let info_ref = unsafe { info.as_ref() };
        let data = match info_ref.kind.get() {
            0 | 7 | 8 => parse_texture_ver(info_ref, &data0, &data1).context("texture data")?,
            1 | 9 => parse_cube_ver(info_ref, &data0, &data1).context("cube data")?,
            _ => {
                warn!(
                    "Unsupported Texture Type {} for texture {:?}",
                    info_ref.kind, info_ref.key
                );
                vec![NonNull::from_ref(&data0[..]), NonNull::from_ref(&data1[..])].into()
            }
        };
        Ok(Self {
            _ptr: src,
            info,
            data0,
            data1,
            data
        })
    }
}

#[make_platforms]
fn parse_texture_ver(info: &TextureInfoVER, data0: &[u8], data1: &[u8]) -> Result<Box<[NonNull<[u8]>]>> {
    let (s, d) = match get_stride_width(info.format.get()) {
        Some((s, d)) => (s as usize, d as usize),
        None => {
            warn!("Unhandled Texture Format {}", info.format);
            return Ok(Box::new([]));
        }
    };

    let mut width = info.width.get() as usize;
    let mut height = info.height.get() as usize;

    let min_size = if IS_XBOX && s != 1 { 128 / s } else { 1 };

    let mut levels = Vec::with_capacity(info.levels.get() as usize);
    if info.levels.get() == 1 {
        let expected_size = (width / s).max(min_size) * (height / s).max(min_size) * d;
        if data1.len() != expected_size {
            return Err(anyhow!("expected texture data to be of size {} but got {}", expected_size, data1.len()));
        }
        levels.push(NonNull::from_ref(&data1[..]));
    } else {
        if !IS_XBOX || (info.width.get() > 16 && info.height.get() > 16) {
            let expected_size = (width / s).max(min_size) * (height / s).max(min_size) * d;
            if data0.len() != expected_size {
                return Err(anyhow!("expected texture data to be of size {} but got {}", expected_size, data0.len()));
            }
            levels.push(NonNull::from_ref(&data0[..]));
        } else {
            width = width * 2;
            height = height * 2;
        }
        let mut offset = 0;
        for _ in 1..info.levels.get() {
            width /= 2;
            height /= 2;
            let size = (width / s).max(min_size) * (height / s).max(min_size) * d;
            levels.push(NonNull::from_ref(&data1[offset..offset + size]));
            offset += size;
            if IS_XBOX && (width == 16 || height == 16) {
                break;
            }
        }
        if offset != data1.len() {
            return Err(anyhow!("expected texture data to be of size {} but got {}", offset, data0.len()));
        }
    }
    Ok(levels.into())
}

#[make_platforms]
fn parse_cube_ver(info: &TextureInfoVER, data0: &[u8], data1: &[u8]) -> Result<Box<[NonNull<[u8]>]>> {
    if info.levels.get() > 1 {
        return Err(anyhow!("Cube Textures with > 1 level are unhanded"));
    }
    let (s, d) = match get_stride_width(info.format.get()) {
        Some((s, d)) => (s as usize, d as usize),
        None => {
            warn!("Unhandled Cube Texture Format {}", info.format);
            return Ok(Box::new([]));
        }
    };

    let min_size = if IS_XBOX { 128 / s } else { 1 };

    let mut faces = Vec::with_capacity(6);

    if data0.len() != 0 {
        return Err(anyhow!("Cube Texture exepects first data to be empty but got {} bytes", data0.len()));
    }

    let data_size = (info.width.get() as usize / s).max(min_size)
        * (info.height.get() as usize / s).max(min_size)
        * d;
    for i in 0..6 {
        faces.push(NonNull::from_ref(
            &data1[data_size * i..data_size * i + data_size],
        ));
    }
    Ok(faces.into())
}

#[make_platforms]
impl TextureVER {
    pub fn from_bytes(src: &Arc<[u8]>, texture_data: &IndexMap<u32, Arc<[u8]>>, offset: usize) -> Result<Self> {
        let info = TextureInfoVER::from_data(&src[offset..]).context("info")?;
        let data0 = texture_data.get(&info.asset_key.get()).cloned().unwrap_or(Arc::new([]));
        let data1 = texture_data.get(
            &hash_string(b"*", Some(info.asset_key.get()))
        ).cloned().unwrap_or(Arc::new([]));

        let mut val = Self {
            _ptr: src.clone(),
            info: info.into(),
            data0,
            data1,
            data: Box::new([]),
        };
        val.data = vec![
            NonNull::from_ref(&val.data0[..]),
            NonNull::from_ref(&val.data1[..]),
        ]
        .into();

        Ok(val)
    }
}

#[make_platforms]
#[cfg_attr(feature = "python", pymethods)]
impl TextureVER {
    #[getter]
    pub fn data(&self) -> Vec<&[u8]> {
        self.data.iter().map(|x| unsafe { x.as_ref() }).collect()
    }
}

#[cfg_attr(feature = "python", pyclass(module = "texture", get_all, set_all))]
#[derive(Debug, Clone)]
pub struct Texture {
    pub info: TextureInfo,
    pub data: Vec<Vec<u8>>,
}

impl Texture {
    pub fn dump(&self) -> (Vec<u8>, Vec<u8>) {
        match self.info.kind {
            0 | 7 | 8 => {
                // mip texture
                if self.data.len() == 1 {
                    (vec![], self.data[0].clone())
                } else {
                    (
                        self.data[0].clone(),
                        self.data[1..].iter().flatten().copied().collect(),
                    )
                }
            }
            1 | 9 => {
                // cube texture
                (vec![], self.data.iter().flatten().copied().collect())
            }
            _ => (self.data[0].clone(), self.data[1].clone()),
        }
    }
}

impl Texture {
    #[make_platforms]
    pub fn from_ver(val: &TextureVER, info: &TextureInfoVER) -> Self {
        let mut info: TextureInfo = info.into();
        let mut data = val
            .data
            .iter()
            .map(|x| unsafe { x.as_ref() }.iter().cloned().collect())
            .collect();
        if IS_XBOX {
            let (s, d) = match get_stride_width(info.format) {
                Some((s, d)) => (s as usize, d as usize),
                None => {
                    warn!(
                        "Unhandled Texture Format {} for texture {:?}",
                        info.format, info.key
                    );
                    return Texture { info, data };
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
                            unsafe { val.data[0].as_ref() },
                            (height / s).max(min_size),
                            (width / s).max(min_size),
                            0,
                            0,
                            (height / s).max(1),
                            (width / s).max(1),
                            s,
                            d,
                        ));
                        return Texture { info, data };
                    }
                    let wide_img = info.width > info.height;
                    let mut level = 0;
                    for vals in &val.data[..val.data.len() - 1] {
                        data.push(conv_img_slice(
                            unsafe { vals.as_ref() },
                            (height / s).max(min_size),
                            (width / s).max(min_size),
                            0,
                            0,
                            (height / s).max(1),
                            (width / s).max(1),
                            s,
                            d,
                        ));
                        width /= 2;
                        height /= 2;
                        level += 1;
                    }
                    let packed_data: &[u8] = unsafe { val.data.last().unwrap().as_ref() };
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
                                s,
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
                                s,
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
                                s,
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
                                s,
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
                    data.extend(val.data.iter().map(|x| {
                        conv_img_slice(
                            unsafe { x.as_ref() },
                            (height / s).max(min_size),
                            (width / s).max(min_size),
                            0,
                            0,
                            (height / s).max(1),
                            (width / s).max(1),
                            s,
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
        Texture { info, data }
    }
}
