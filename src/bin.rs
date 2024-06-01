use std::{any::TypeId};
use log::warn;
use zerocopy::{ByteOrder, LE};
use serde::{Serialize, Deserialize};
use crate::types::Crc;

use super::pak::TextureInfo;
use lotrc_rs_proc::OrderedData;
use super::types::{OrderedData, OrderedDataVec};

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Header {
    pub constx06: u32,
    pub version: u32,
    pub strings_offset: u32,
    pub strings_size: u32,
    pub strings_num: u32,
    pub asset_handle_num: u32,
    pub asset_handle_offset: u32,
    pub unk_7: u32,
    pub unk_8: u32,
    pub unk_9: u32,
    pub unk_10: u32,
    pub unk_11: u32,
    pub unk_12: u32,
    pub unk_13: u32,
    pub unk_14: u32,
    pub unk_15: u32,
    pub unk_16: u32,
    pub unk_17: u32,
    pub unk_18: u32,
    pub unk_19: u32,
    pub unk_20: u32,
    pub unk_21: u32,
    pub unk_22: u32,
    pub unk_23: u32,
    pub unk_24: u32,
    pub unk_25: u32,
    pub unk_26: u32,
    pub unk_27: u32,
    pub unk_28: u32,
    pub unk_29: u32,
    pub unk_30: u32,
    pub unk_31: u32,
    pub unk_32: u32,
    pub unk_33: u32,
    pub unk_34: u32,
    pub unk_35: u32,
    pub unk_36: u32,
    pub unk_37: u32,
    pub unk_38: u32,
    pub unk_39: u32,
    pub unk_40: u32,
    pub unk_41: u32,
    pub unk_42: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct AssetHandle {
    pub key: Crc,
    pub offset: u32,
    pub size: u32,
    pub size_comp: u32,
    pub kind: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Radiosity {
    // probably not correctly modeled
    pub data: Vec<u32>
}

impl Radiosity {
    pub fn from_data<O: ByteOrder + 'static>(data: &[u8]) -> Self {
        if data.len() % 4 != 0 {
            warn!("Radiosity length is incorrect?")
        }
        let data = OrderedDataVec::from_bytes::<O>(data, data.len()/4);
        Self { data }
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        self.data.dump_bytes::<O>()
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Tex {
    Texture(Texture),
    CubeTexture(CubeTexture),
}

impl Tex {
    pub fn kind(&self) -> u32 {
        match self {
            Self::Texture(val) => val.kind,
            Self::CubeTexture(val) => val.kind,
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> (Vec<u8>, Vec<u8>) {
        match self {
            Self::Texture(val) => val.dump::<O>(),
            Self::CubeTexture(val) => val.dump::<O>(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Texture {
    pub levels: Vec<Vec<u8>>,
    pub format: u32,
    pub kind: u32,
}

pub fn get_stride_width(format: u32) -> Option<(u32, u32)> {
    match format {
        10 | 0xb | 0xc | 0x11 => Some((4, 16)),
        7 | 8 | 13 => Some((4, 8)),
        3 => Some((1, 4)),
        6 => Some((1, 1)),
        _ => None,
    }
}

fn xg_address2d_tiled_xy(offset: u32, width: u32, texel_pitch: u32) -> (usize, usize) {
    // https://github.com/NCDyson/RareView/blob/master/RareView/Texture.cs
    let aligned_width = (width + 31) & !31;

    let log_bpp = (texel_pitch >> 2) + ((texel_pitch >> 1) >> (texel_pitch >> 2));
    let offset_b = offset << log_bpp;
    let offset_t = ((offset_b & !4095) >> 3) + ((offset_b & 1792) >> 2) + (offset_b & 63);
    let offset_m = offset_t >> (7 + log_bpp);

    let macro_x = ((offset_m % (aligned_width >> 5)) << 2);
    let tile_x = ((((offset_t >> (5 + log_bpp)) & 2) + (offset_b >> 6)) & 3);
    let macro_x = (macro_x + tile_x) << 3;
    let micro_x = ((((offset_t >> 1) & !15) + (offset_t & 15)) & ((texel_pitch << 3) - 1)) >> log_bpp;

    let macro_y = ((offset_m / (aligned_width >> 5)) << 2);
    let tile_y = ((offset_t >> (6 + log_bpp)) & 1) + (((offset_b & 2048) >> 10));
    let macro_y = (macro_y + tile_y) << 3;
    let micro_y = ((((offset_t & (((texel_pitch << 6) - 1) & !31)) + ((offset_t & 15) << 1)) >> (3 + log_bpp)) & !1);

    (
        (macro_x + micro_x) as usize,
        (macro_y + micro_y + ((offset_t & 16) >> 4)) as usize
    )
}
    

pub fn conv_img(data: &[u8], height: usize, width: usize, f: u32) -> (Vec<u8>, usize, usize, usize) {
    // https://github.com/NCDyson/RareView/blob/master/RareView/Texture.cs
    let (data, s, d): (Vec<u8>, usize, usize) = match f {
        10 | 0xb | 0xc | 0x11 => (
            data.chunks(2).flat_map(|x| [x[1], x[0]]).collect::<Vec<_>>(),
            4,
            16
        ),
        7 | 8 | 13 => (
            data.chunks(2).flat_map(|x| [x[1], x[0]]).collect::<Vec<_>>(),
            4,
            8
        ),
        3 => (
            data.chunks(4).flat_map(|x| [x[3], x[2], x[1], x[0]]).collect::<Vec<_>>(),
            1,
            4
        ),
        _ => (
            data.iter().cloned().collect::<Vec<_>>(),
            1,
            1
        )
    };
    let h_ = height / s;
    let w_ = width / s;
    let (h, w) = match f {
        7 | 8 | 10 | 0xb | 0xc | 0x11 => (
            h_.max(32), w_.max(32)
        ),
        _ => (
            h_, w_
        )
    };
    let mut out_data = vec![0u8; w*h*d];
    for i in 0..(h*w) {
        let (x,y) = xg_address2d_tiled_xy(i as u32, w as u32, d as u32);
        if x < w_ && y < h_ {
            let j = y * w + x;
            let src = &data[i * d..(i+1)*d];
            let dst = &mut out_data[j*d..(j+1)*d];
            dst.copy_from_slice(src);
        }
        // for k in 0..d {
        //     out_data[j*d+k] = *data.get(i*d+k).unwrap_or(&0);
        // }
        // out_data[i*d..(i+1)*d].copy_from_slice(&data[j * d..(j+1)*d]);
    };
    (out_data.chunks(w*d).take(h_).flat_map(|x| &x[..w_*d]).cloned().collect(), d, w_, h_)
}

fn bin_mip(arr: &[u8], w: usize) -> Vec<u8> {
    arr.chunks(w).step_by(2).flat_map(|x| x.iter().step_by(2)).cloned().collect()
}

fn decomp_bc4(arr: &[u8], w: usize, h: usize) -> Vec<u8> {
    bcndecode::decode(arr, w, h, bcndecode::BcnEncoding::Bc4, bcndecode::BcnDecoderFormat::LUM).unwrap()
}
// def bin_mip(arr, w, h):
//     return np.frombuffer(arr, np.ubyte).reshape(h//2, 2, w//2, 2)[:,0,:,0].tobytes()

impl Texture {
    pub fn from_data<O: ByteOrder + 'static>(data0: &[u8], data1: &[u8], info: &mut TextureInfo) -> Self {
        let sizes = (0..info.levels).map(|x| 2u32.pow(x as u32)).map(|x| (info.width as u32/x, info.height as u32/x)).collect::<Vec<_>>();
        let mut format = info.format;
        let kind = info.asset_type;
        let (s, d) = match get_stride_width(format) {
            Some((s,d)) => (s,d),
            None => {
                warn!("Unhandled Texture Format {}", format);
                return Self {
                    levels: vec![data0.to_vec(), data1.to_vec()],
                    format,
                    kind,
                    ..Default::default()
                }
            }
        };

        let block_sizes = sizes.iter().map(|(x,y)| ((x/s).max(1), (y/s).max(1))).collect::<Vec<_>>();
        let data = data0.iter().chain(data1.iter()).cloned().collect::<Vec<_>>();
        let levels = if TypeId::of::<O>() == TypeId::of::<LE>() {
            let data_sizes = block_sizes.iter().map(|(x,y)| (x * y * d) as usize).collect::<Vec<_>>();
            let mut levels = Vec::with_capacity(data_sizes.len());
            let mut offset = 0;
            for size in data_sizes {
                levels.push(data[offset..offset+size].to_vec());
                offset += size;
            } 
            levels
        } else {
            if info.levels == 1 {
                vec![conv_img(&data[..], sizes[0].1 as usize, sizes[0].0 as usize, format).0]
            } else {
                let data_sizes = block_sizes.iter().map(|(x,y)| (x.max(&32) * y.max(&32) * d) as usize).collect::<Vec<_>>();
                let wide_img = info.width > info.height;
                let mut levels = Vec::with_capacity(data_sizes.len());
                let mut packed_data = vec![];
                let mut offset = 0;
                let mut d = 0;
                let (mut pw, mut ph) = (0,0);
                for i in 0..info.levels as usize {
                    let (m, M) = (sizes[i].0.min(sizes[i].1) as usize, sizes[i].0.max(sizes[i].1) as usize);
                    if m > 16 {
                        levels.push(conv_img(&data[offset..offset+data_sizes[i]], sizes[i].1 as usize, sizes[i].0 as usize, format).0);
                        offset += data_sizes[i];
                    } else {
                        if m == 16 {
                            (packed_data, d, pw, ph) = if wide_img {
                                conv_img(&data[offset..], sizes[i].1 as usize*2, sizes[i].0 as usize, format)
                            } else {
                                conv_img(&data[offset..], sizes[i].1 as usize, sizes[i].0 as usize*2, format)
                            };
                            // println!("{:?}, {:?}", (info.asset_key.clone(), info.asset_type), (packed_data.len(), d, pw, ph));
                        }
                        if m >= 4 {
                            let off = m >> 2;
                            levels.push(if wide_img  {
                                packed_data.chunks(pw * d).skip(off).take(block_sizes[i].1 as usize).flat_map(|x| &x[..block_sizes[i].0 as usize*d]).cloned().collect()
                            } else {
                                packed_data.chunks(pw * d).take(block_sizes[i].1 as usize).flat_map(|x| &x[off*d..(off + block_sizes[i].0 as usize)*d]).cloned().collect()
                            });
                        } else {
                            let off = M;
                            levels.push(if wide_img  {
                                packed_data.chunks(pw * d).take(block_sizes[i].1 as usize).flat_map(|x| &x[off*d..(off + block_sizes[i].0 as usize)*d]).cloned().collect()
                            } else {
                                packed_data.chunks(pw * d).skip(off).take(block_sizes[i].1 as usize).flat_map(|x| &x[..block_sizes[i].0 as usize*d]).cloned().collect()
                            });
                        }
                    }
                }
                if info.format == 13 {
                    format = 6;
                    info.format = 6;
                    levels = levels.into_iter().enumerate().map(|(i, x)| decomp_bc4(&x[..], sizes[i].0.max(4) as usize, sizes[i].1.max(4) as usize)).collect();
                    levels[info.levels as usize-2] = bin_mip(&levels[info.levels as usize-3][..], sizes[info.levels as usize-3].0 as usize);
                    levels[info.levels as usize-1] = bin_mip(&levels[info.levels as usize-2][..], sizes[info.levels as usize-2].0 as usize);
                }
                levels
            }
        };

        Self {
            levels, format, kind,
            ..Default::default()
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> (Vec<u8>, Vec<u8>) {
        if TypeId::of::<O>() == TypeId::of::<LE>() {
            match self.format {
                3 | 6 | 7 | 8 | 10 | 0xb | 0xc | 0x11 => {
                    if self.levels.len() > 1 {
                        (self.levels[0].clone(), self.levels[1..].iter().flatten().cloned().collect())
                    } else {
                        (vec![], self.levels[0].clone())
                    }
                },
                _ => {
                    (self.levels[0].clone(), self.levels[1].clone())
                }
            }
        } else {
            warn!("Exporting Textures to Xbox format is not supported");
            (vec![], vec![])
        }
    }
}

/*
    def get_img(self, level=0):
        if self.format in [10, 0xb, 0xc, 0x11]:
            return decomp_dxt5(self.levels[level], self.sizes[level][1], self.sizes[level][0])
        elif self.format in [7,8]:
            return decomp_dxt1(self.levels[level], self.sizes[level][1], self.sizes[level][0])
        elif self.format == 3:
            return np.frombuffer(self.levels[level], 'B').reshape(self.sizes[level][1], self.sizes[level][0], 4)
*/


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CubeTexture {
    pub faces: Vec<Vec<u8>>,
    pub format: u32,
    pub kind: u32,
}

impl CubeTexture {
    pub fn from_data<O: ByteOrder + 'static>(data0: &[u8], data1: &[u8], info: &TextureInfo) -> Self {
        let format = info.format;
        let kind = info.asset_type;
        assert!(info.levels <= 1, "Cube Textures with > 1 level are unhanded");
        let (s, d) = match get_stride_width(format) {
            Some(val) => val,
            None => {
                warn!("Unhandled Cube Texture Format {}", format);
                return Self {
                    faces: vec![data0.to_vec(), data1.to_vec()],
                    format,
                    kind,
                    ..Default::default()
                }
            }
        };

        let size = (info.width, info.height);
        let block_size = (size.0 as u32/s, size.1 as u32/s);
        let mut offset = 0;
        let mut faces = Vec::with_capacity(6);

        if TypeId::of::<O>() == TypeId::of::<LE>() {
            let data_size = (block_size.0 * block_size.1 * d) as usize;
            for i in 0..6 {
                faces.push(data1[data_size*i..data_size*i+data_size].to_vec());
            }
        } else {
            let data_size = (block_size.0.max(32) * block_size.1.max(32) * d) as usize;
            for i in 0..6 {
                faces.push(conv_img(&data1[data_size*i..data_size*i+data_size], size.1 as usize, size.0 as usize, format).0);
            }
        }

        Self {
            faces,
            format,
            kind,
            ..Default::default()
        }
    }

    pub fn dump<O: ByteOrder + 'static>(&self) -> (Vec<u8>, Vec<u8>) {
        if TypeId::of::<O>() == TypeId::of::<LE>() {
            match self.format {
                3 | 7 | 8 | 10 | 0xb | 0xc | 0x11 => (vec![], self.faces.iter().flatten().cloned().collect()),
                _ => (self.faces[0].clone(), self.faces[1].clone()),
            }
        } else {
            warn!("Exporting Textures to Xbox format is not supported");
            (vec![], vec![])
        }
    }
}