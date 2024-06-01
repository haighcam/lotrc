use std::{collections::HashMap, fs::{self, File}, path::Path, any::TypeId};
use zerocopy::{ByteOrder, LE, BE};
use log::warn;
use serde::{Serialize, Deserialize};
// use rmp_serde::Serializer;
use serde_cbor::{Serializer, Deserializer, ser::IoWrite, de::IoRead};
use std::time::Instant;
use std::iter::zip;
use lotrc_rs_proc::OrderedData;

mod types;
mod pak;
mod bin;
mod level_info;
mod level;
mod lua_stuff;

use level::Level;
use level_info::LevelInfo;

fn conv_dir<A: AsRef<Path>, B: AsRef<Path>>(source: A, dest: B) {
    let source = source.as_ref();
    let dest = dest.as_ref();
    let paths = fs::read_dir(source).unwrap();

    for path in paths.map(|x| x.unwrap().path()) {
        if let Some("PAK") = path.extension().and_then(|x| x.to_str()) {
            let name = path.file_stem().unwrap().to_str().unwrap();
            println!("Parsing Level {}", name);
            let mut level = Level::parse(source.join(name));
            let (pak, bin) = level.to_data::<LE>(true);

            fs::write(dest.join(name).with_extension("PAK"), pak).unwrap();
            fs::write(dest.join(name).with_extension("BIN"), bin).unwrap();
        }
        
        // println!("{}")
    }
}

fn dump_dir<A: AsRef<Path>, B: AsRef<Path>>(source: A, dest: B) {
    let source = source.as_ref();
    let dest = dest.as_ref();
    let paths = fs::read_dir(source).unwrap();
    fs::create_dir(dest).ok();

    for path in paths.map(|x| x.unwrap().path()) {
        match path.extension().and_then(|x| x.to_str()) {
            Some("PAK") => {
                let name = path.file_stem().unwrap().to_str().unwrap();
                println!("Parsing Level {}", name);
                let mut level = Level::parse(source.join(name));
                level.to_file(dest.join(name))
            },
            Some("dat") => {
                println!("Parsing level_info");
                let name = path.file_stem().unwrap().to_str().unwrap();
                let level_info = LevelInfo::parse(source.join(path.file_name().unwrap()));
                level_info.to_file(dest.join(name));
            },
            _ => ()
        }        
        // println!("{}")
    }
}


fn main() {
    pretty_env_logger::init();
    // println!("{:?}", types::STRING_LOOKUP.lock().unwrap());
    {
        *types::DECOMP_LUA.lock().unwrap() = false;
    }

    // let level_info = LevelInfo::parse("../Levels/level_info.dat");
    // level_info.dump::<LE, _>("../level_info_test.dat");
    // level_info.to_file("things/level_data");
    // return ();

    dump_dir(
        "../Xbox/Levels", 
        "things/Levels"
    );
    
    // let mut level = Level::parse("../Xbox/AddOn/HeroesandMapsPack/Weathertop_DLC");
    
    // if let Some(types::SubBlock::GameObjs(gameobjs)) = level.sub_blocks1.blocks.last_mut() {
    //     gameobjs.to_file("Test.json");
    //     *gameobjs = types::GameObjs::from_file("Test.json");
    // }

    // level.to_file("things/Weathertop_DLC");

    // level.dump::<LE, _>("../AddOn/HeroesandMapsPack/Weathertop_DLC_A", true);
    // level.dump::<LE, _>("../AddOn/HeroesandMapsPack/Weathertop_DLC", true);

    // conv_dir(
    //     "../Xbox/AddOn/HeroArenaBonus", 
    //     "../AddOn/HeroArenaBonus"
    // );

    

    // fs::write("BlackGates.ron", ron::to_string(&level).unwrap());
    // let lua = lua_stuff::LuaCompiler::new().unwrap();
    // let contents = fs::read("/home/cameron/Documents/Games/The Lord of the Rings Conquest 2/test_lua.lua").unwrap();
    // let res = lua.decomp(contents.as_slice()).unwrap();
    // let comp = lua.compile(&res, "test.lua");

    // let pak_header: pak::Header = OrderedData::from_bytes::<LE>(&contents);
    // let pak_header = pak::Header::<LE>::read_from_prefix(&contents);
    // let pak_header = pak::Header::from_data(&contents, 0, &());
    // level.to_file("/home/cameron/Documents/Games/The Lord of the Rings Conquest 2/lotrc_decomp_rs/things/MountDoomXbox");
    // level.to_file("things/ShireXbox");
    // level.to_file("/home/cameron/Documents/Games/The Lord of the Rings Conquest 2/lotrc_decomp_rs/things/Moria_DLC");
    // println!("{:?}", res);
    // println!("{:?}", comp);
}