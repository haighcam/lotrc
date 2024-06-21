use std::fs;
use zerocopy::{ByteOrder, LE, BE};
use serde::{Serialize, Deserialize};
use std::path::Path;
use log::{error, info};

use lotrc_rs_proc::OrderedData;
use super::types::{OrderedData, OrderedDataVec, Crc};

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Header {
    pub const0x2: u32,
    pub n1: u32,
    pub n2: u32,
    pub n3: u32,
    pub n4: u32,
    pub n5: u32,
    pub n6: u32,
    pub n7: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Obj1 {
    pub key: Crc,
    pub val: u32,
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Obj2 {
    pub unk_0: u32,
    pub unk_1: u32,
    pub n: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AudioTable {
    #[serde(skip)]
    pub header: Header,
    pub obj1s: Vec<Obj1>,
    pub obj2s: Vec<(Obj2, Vec<Obj1>)>,
    pub obj3s: Vec<(Obj2, Vec<Obj1>)>,
    pub obj4s: Vec<Obj1>,
    pub obj5s: Vec<Obj1>,
    pub obj6s: Vec<Obj1>,
    pub obj7s: Vec<Obj1>,
    pub extra: Vec<Crc>,
}

impl AudioTable {
    pub fn parse<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        info!("Parsing audio table {}", path.file_stem().unwrap().to_str().unwrap());   
        let data = fs::read(path).unwrap();
        if data[0] == 2 {
            Self::from_data::<LE>(&data[..])
        } else if data[3] == 2 {
            Self::from_data::<BE>(&data[..])
        } else {
            error!("Invalid audio table data");
            Default::default()
        }
    }

    pub fn dump<O: ByteOrder + 'static, P: AsRef<Path>>(&self, path: P) {
        fs::write(path, self.to_data::<O>()).unwrap();
    }

    pub fn from_data<O: ByteOrder + 'static>(data: &[u8]) -> Self {
        let header: Header = OrderedData::from_bytes::<O>(&data);
        let mut offset = Header::size::<O>();
        let obj1s: Vec<Obj1> = OrderedDataVec::from_bytes::<O>(&data[offset..], header.n1 as usize);
        offset += obj1s.size::<O>();
        let mut obj2s = Vec::with_capacity(header.n2 as usize);
        for _ in 0..header.n2 {
            let obj: Obj2 = OrderedData::from_bytes::<O>(&data[offset..]);
            offset += Obj2::size::<O>();
            let objs: Vec<Obj1> = OrderedDataVec::from_bytes::<O>(&data[offset..], obj.n as usize);
            offset += objs.size::<O>();
            obj2s.push((obj, objs));
        }
        let mut obj3s = Vec::with_capacity(header.n3 as usize);
        for _ in 0..header.n3 {
            let obj: Obj2 = OrderedData::from_bytes::<O>(&data[offset..]);
            offset += Obj2::size::<O>();
            let objs: Vec<Obj1> = OrderedDataVec::from_bytes::<O>(&data[offset..], obj.n as usize);
            offset += objs.size::<O>();
            obj3s.push((obj, objs));
        }
        let obj4s: Vec<Obj1> = OrderedDataVec::from_bytes::<O>(&data[offset..], header.n4 as usize);
        offset += obj4s.size::<O>();
        let obj5s: Vec<Obj1> = OrderedDataVec::from_bytes::<O>(&data[offset..], header.n5 as usize);
        offset += obj5s.size::<O>();
        let obj6s: Vec<Obj1> = OrderedDataVec::from_bytes::<O>(&data[offset..], header.n6 as usize);
        offset += obj6s.size::<O>();
        let obj7s: Vec<Obj1> = OrderedDataVec::from_bytes::<O>(&data[offset..], header.n7 as usize);
        offset += obj7s.size::<O>();
        let n = (data.len() - offset) / 4;
        let extra: Vec<Crc> = OrderedDataVec::from_bytes::<O>(&data[offset..], n);

        Self {
            header,
            obj1s,
            obj2s,
            obj3s,
            obj4s,
            obj5s,
            obj6s,
            obj7s,
            extra
        }
    }

    pub fn to_data<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        self.header.dump_bytes::<O>().into_iter()
        .chain(self.obj1s.dump_bytes::<O>())
        .chain(self.obj2s.iter().flat_map(|(obj, objs)| obj.dump_bytes::<O>().into_iter().chain(objs.dump_bytes::<O>())))
        .chain(self.obj3s.iter().flat_map(|(obj, objs)| obj.dump_bytes::<O>().into_iter().chain(objs.dump_bytes::<O>())))
        .chain(self.obj4s.dump_bytes::<O>())
        .chain(self.obj5s.dump_bytes::<O>())
        .chain(self.obj6s.dump_bytes::<O>())
        .chain(self.obj7s.dump_bytes::<O>())
        .chain(self.extra.dump_bytes::<O>())
        .collect()
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        if let Some(path) = path.as_ref().parent() {
            fs::create_dir_all(path).ok();
        }
        fs::write(path.as_ref().with_extension("audio.json"), serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut val = serde_json::from_slice::<Self>(&fs::read(path.as_ref().with_extension("json")).unwrap()).unwrap();
        val.header = Header {
            const0x2: 2,
            n1: val.obj1s.len() as u32,
            n2: val.obj2s.len() as u32,
            n3: val.obj3s.len() as u32,
            n4: val.obj4s.len() as u32,
            n5: val.obj5s.len() as u32,
            n6: val.obj6s.len() as u32,
            n7: val.obj7s.len() as u32
        };
        val
    }
}