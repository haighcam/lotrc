
use std::{collections::HashMap, fs::{self, File}, path::Path, any::TypeId};
use serde_json::json;
use zerocopy::{ByteOrder, LE, BE};
use log::warn;
use serde::{Serialize, Deserialize};
// use rmp_serde::Serializer;
use serde_cbor::{Serializer, Deserializer, ser::IoWrite, de::IoRead};
use std::time::Instant;
use std::iter::zip;
use lotrc_rs_proc::OrderedData;

use super::{
    lua_stuff,
    types::{self, Crc, OrderedData, OrderedDataVec}
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Name(Box<str>);

impl From<Name> for [u8; 32] {
    fn from(value: Name) -> Self {
        let mut val = [0u8; 32];
        let n = value.0.len().min(32);
        val[..n].copy_from_slice(&value.0.as_bytes()[..n]);
        val
    }
}

impl From<[u8; 32]> for Name {
    fn from(value: [u8; 32]) -> Self {
        let mut i = 0;
        while i < 32 {
            if value[i] == 0 {
                break;
            }
            i += 1
        }
        Name(String::from_utf8(value[..i].to_vec()).unwrap().into_boxed_str())
    }
}

impl OrderedData for Name {
    type LE = [u8; 32];
    type BE = [u8; 32];
}

#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct Header {
    pub constx04: u32,
    pub dlc: u32,
    pub strings_offset: u32,
    pub strings_size: u32,
    pub strings_num: u32,
    pub string_keys_size: u32,
    pub string_keys_offset: u32,
    pub locale_strings_size: u32,
    pub locale_strings_offset: u32,
    pub gamemodes_num: u32,
    pub gamemodes_offset: u32,
    pub levels_num: u32,
    pub levels_offset: u32,
    pub size2048: u32,
}
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct LevelVal {
    pub name: Name,
    pub key_name: Crc,
    pub key_description: Crc,
    pub dlc: u32,
    pub gamemodes: u32,
}
#[derive(Debug, Default, Clone, OrderedData, Serialize, Deserialize)]
pub struct GamemodeVal {
    pub key: Crc,
    pub key_name: Crc,
    pub key_description: Crc,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LevelInfo {
    header: Header,
    #[serde(skip)]
    strings: types::Strings,
    #[serde(skip)]
    string_keys: types::StringKeys,
    #[serde(skip)]
    locale_strings: types::SubBlocks,
    levels: Vec<LevelVal>,
    gamemodes: Vec<GamemodeVal>,
    extra: Vec<u8>
}

impl LevelInfo {
    pub fn parse<P: AsRef<Path>>(path: P) -> Self {
        let data = fs::read(path).unwrap();
        if data[0] == 4 {
            Self::from_data::<LE>(&data[..])
        } else if data[3] == 4 {
            Self::from_data::<BE>(&data[..])
        } else {
            warn!("Invalid level_info data");
            Default::default()
        }
    }

    pub fn dump<O: ByteOrder + 'static, P: AsRef<Path>>(&self, path: P) {
        fs::write(path, self.to_data::<O>()).unwrap();
    }

    pub fn from_data<O: ByteOrder + 'static>(data: &[u8]) -> Self {
        let lua = lua_stuff::LuaCompiler::new().unwrap();

        let header: Header = OrderedData::from_bytes::<O>(data);
        let strings = types::Strings::from_data::<O>(data, header.strings_offset as usize, header.strings_num as usize);
        types::update_strings(&strings.strings);
        let string_keys = types::StringKeys::from_data::<O>(data, header.string_keys_offset as usize);
        let locale_strings = types::SubBlocks::from_data::<O>(data, header.locale_strings_offset as usize, &lua);
        let gamemodes = OrderedDataVec::from_bytes::<O>(&data[header.gamemodes_offset as usize..], header.gamemodes_num as usize);
        let levels = OrderedDataVec::from_bytes::<O>(&data[header.levels_offset as usize..], header.levels_num as usize);
        let extra = data[0x38..0x13c].to_vec();

        Self {
            header,
            strings,
            string_keys,
            locale_strings,
            gamemodes,
            levels,
            extra
        }
    }

    pub fn to_data<O: ByteOrder + 'static>(&self) -> Vec<u8> {
        let lua = lua_stuff::LuaCompiler::new().unwrap();

        let mut dump_header = self.header.clone();
        dump_header.gamemodes_offset = 0x13c;
        dump_header.gamemodes_num = self.gamemodes.len() as u32;
        dump_header.levels_offset = dump_header.gamemodes_offset + dump_header.gamemodes_num * GamemodeVal::size::<O>() as u32;
        dump_header.levels_num = self.levels.len() as u32;
        dump_header.string_keys_offset = dump_header.levels_offset + dump_header.levels_num * LevelVal::size::<O>() as u32;
        dump_header.string_keys_size = self.string_keys.size::<O>() as u32;
        dump_header.locale_strings_offset = dump_header.string_keys_offset + dump_header.string_keys_size;
        dump_header.locale_strings_size = self.locale_strings.size::<O>() as u32;
        dump_header.strings_offset = dump_header.locale_strings_offset + dump_header.locale_strings_size;
        dump_header.strings_size = self.strings.size() as u32;
        dump_header.strings_num = self.strings.len() as u32;
        dump_header.size2048 = (dump_header.strings_offset + 2047) & 0xFFFFF800;

        let mut data = vec![0u8; (dump_header.strings_offset + dump_header.strings_size) as usize];
        dump_header.to_bytes::<O>(&mut data[..]);
        self.strings.into_data::<O>(&mut data[..], dump_header.strings_offset as usize);
        self.string_keys.into_data::<O>(&mut data[..], dump_header.string_keys_offset as usize);
        data[
            dump_header.locale_strings_offset as usize..(dump_header.locale_strings_offset + dump_header.locale_strings_size) as usize
        ].copy_from_slice(self.locale_strings.dump::<O>(&lua).as_slice());
        self.gamemodes.to_bytes::<O>(&mut data[dump_header.gamemodes_offset as usize..]);
        self.levels.to_bytes::<O>(&mut data[dump_header.levels_offset as usize..]);
        data[0x38..0x13c].copy_from_slice(&self.extra[..]);
        data
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        let path = path.as_ref();
        fs::create_dir(path).ok();
        fs::write(path.join("index.json"), serde_json::to_string_pretty(self).unwrap()).unwrap();
        self.strings.to_file(path.join("debug_strings"));
        self.string_keys.to_file(path.join("string_keys"));
        self.locale_strings.to_file(path.join("locale_strings"), &self.string_keys);
    }

}