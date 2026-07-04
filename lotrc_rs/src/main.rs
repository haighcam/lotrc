use clap::Parser;
//use lotrc::types::{get_str, hash_string};
use std::path::PathBuf;
use lotrc::level::DumpLevelPc;
use lotrc::types::{RefFromData, OrderedData, DumpData};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,
}

fn print_info<T>() {
    println!(
        "{}, {}",
        std::mem::size_of::<T>(),
        std::mem::align_of::<T>()
    );
}

fn get_string(crc: &u32, level: &lotrc::level::LevelRefPc) -> String {
    if let Ok(i) = level.pak.strings.strings.binary_search_by_key(crc, |s| lotrc::types::hash_string(s.as_bytes(), None) ) {
        level.pak.strings.strings[i].to_string()
    } else if let Ok(i) = level.bin.strings.strings.binary_search_by_key(crc, |s| lotrc::types::hash_string(s.as_bytes(), None) ) {
        level.bin.strings.strings[i].to_string()
    } else {
        String::new()
    }
}

fn main() {
    let filter_level = log::LevelFilter::Info;
    let filter_level = log::LevelFilter::Debug;
    let logger = pretty_env_logger::formatted_builder()
        .filter_level(filter_level)
        .format(|buf, record| {
            use pretty_env_logger::env_logger::fmt::Color;
            use std::io::Write;

            let mut style = buf.style();
            let level = match record.level() {
                log::Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
                log::Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
                log::Level::Info => style.set_color(Color::Green).value("INFO "),
                log::Level::Warn => style.set_color(Color::Yellow).value("WARN "),
                log::Level::Error => style.set_color(Color::Red).value("ERROR"),
            };
            writeln!(buf, " {} > {}", level, record.args())
        })
        .build();

    let level = logger.filter();
    let multi = indicatif::MultiProgress::new();
    indicatif_log_bridge::LogWrapper::new(multi.clone(), logger)
        .try_init()
        .unwrap();
    log::set_max_level(level);

    print_info::<Vec<u32>>();

    let args = Args::parse();

    let path = PathBuf::from(args.path);
    println!("Reading level: {:?}", path);

    let t = std::time::Instant::now();
    let level_data = lotrc::level::LevelData::read(path).unwrap();
    println!("level_data parsed in {:?}", t.elapsed());

    let t = std::time::Instant::now();
    let mut level_compressed_data = lotrc::level::LevelCompressedData::default();
    let level = lotrc::level::LevelRefPc::from_data(&level_data, &mut level_compressed_data).unwrap();
    println!("level parsed in {:?}", t.elapsed());
    if false {
        let mut offs = vec![0usize; level.pak.animation_data.len()];
        let anim_off_map = level.pak.block1.infos.animations
            .iter()
            .map(|info| offs
                .iter_mut()
                .enumerate()
                .filter(|(i, _)| *i & info.gamemodemask.to_native() as usize != 0)
                .map(|(i, off)| {
                    let old_off = *off;
                    *off += info.size.to_native() as usize;
                    (i, old_off)
                }).collect::<Vec<_>>()
            ).collect::<Vec<_>>();
        use lotrc::level::pak::animation::DumpAnimationPc;
        for ((key, animation), offs) in level.pak.animations.animations.iter().zip(anim_off_map) {
            println!("testing key, {}", key);
            println!("offsets {:?}", offs);
            
            let mut data = lotrc::types::AlignedBuf::with_capacity(animation.size());
            println!("size {}, {}", data.len(), animation.info.size);
            let mut dst = lotrc::types::DumpSlice::from(&mut data[..]);
            let mut info = lotrc::level::pak::animation::AnimationInfoPc::default();
            animation.dump_into(&mut dst, &mut info).expect("dump animation");
            println!("curr offset {}", dst.offset);
            println!("{:#?}, {:#?}", info, animation.info);
            info.offset = animation.info.offset;
            assert!(info == *animation.info);
            assert!((data.len() as u32) == animation.info.size.to_native());
            let anim_alt = lotrc::level::pak::animation::AnimationRefPc::from_data(&data[..], &info).expect("load animation");
            if let Some((i, off)) = offs.first().cloned() {
                let buf = level.pak.animation_data.get(i).unwrap().data_decomp.as_ref();
                let old_data = &buf[off..off + info.size.to_native() as usize];
                if old_data != data.as_ref() {
                    println!("anims data differ");
                    let differering_vals = old_data.iter()
                        .zip(data.as_ref())
                        .enumerate()
                        .filter_map(|(i, (src, dst))| if src == dst { None } else { Some(i) })
                        .collect::<Vec<_>>();
                    println!("differing vals {:?}", differering_vals);
                    if &anim_alt == animation { println!("but anims don't!!!"); }
                    if anim_alt.obj1 != animation.obj1 { println!("obj1 differs"); }
                    if anim_alt.obj2 != animation.obj2 { println!("obj2 differs"); }
                    if anim_alt.obj3 != animation.obj3 { println!("obj3 differs"); }
                    if anim_alt.obj5_header != animation.obj5_header { println!("obj5_header differs"); }
                    if anim_alt.obj5_a != animation.obj5_a { println!("obj5_a differs"); }
                    if anim_alt.obj5_b != animation.obj5_b { println!("obj5_b differs"); }
                    if anim_alt.bones != animation.bones { println!("bones differs"); }
                    if anim_alt.blocks != animation.blocks { println!("blocks differs"); }
                    if let Some(blocks) = animation.blocks.as_ref() {
                        let blocks_alt = anim_alt.blocks.as_ref().unwrap();
                        if blocks.block_starts != blocks_alt.block_starts { println!("block_starts differ"); }
                        if blocks.block_starts2 != blocks_alt.block_starts2 { println!("block_starts2 differ"); }
                        if blocks.obj_c3 != blocks_alt.obj_c3 { println!("obj_c3 differ"); }
                        if blocks.obj_c4 != blocks_alt.obj_c4 { println!("obj_c4 differ"); }
                        for (i, (block, block_alt)) in blocks.blocks.iter().zip(blocks_alt.blocks.iter()).enumerate() {
                            if block != block_alt { println!("block {} differ", i); }
                            if block.vals_a != block_alt.vals_a { println!("vals_a differ"); }
                            for (s, d) in block.vals_a.iter().zip(block_alt.vals_a.iter()) {
                                if s.a.flags != d.a.flags { println!("flags differ"); }
                                if s.a != d.a { println!("aaa"); }
                            }
                            if block.vals_b != block_alt.vals_b { println!("vals_b differ"); }
                        }
                    }
                    //if anim_alt.blocks != animation.blocks { println!("blocks differs"); }
                    //println!("original: {:#?}\nnew: {:#?}", animation, anim_alt);
                    println!("old: {:?}\nnew {:?}", old_data, data.as_ref());
                }
                assert!(old_data == data.as_ref());
            }
        }
        println!("animations ok");
    }
    if false {
        use lotrc::level::pak::animation::DumpAnimationsPc;
        let mut counts = lotrc::level::pak::block1::infos::InfoCounts::default();
        level.pak.animations.info_counts(&mut counts);
        let mut info_data = lotrc::types::AlignedBuf::with_capacity(counts.size_pc(0));
        let mut dst = lotrc::types::DumpSlice::from(&mut info_data[..]);
        let mut offsets: Vec<lotrc::types::u32Pc> = vec![0.into(); counts.offsets];
        let mut infos = lotrc::level::pak::block1::infos::DumpInfosPc::from_data(&mut dst, &counts, &mut offsets).expect("infos dump");

        let datas = level.pak.animations.dump(&mut infos).expect("anims dump");
        let mut header = lotrc::level::pak::PakHeaderPc::default();
        let dumped_infos = lotrc::level::pak::block1::infos::InfosRefPc::from_data(&info_data[..], &header).expect("dumped infos");
        
        println!("dumped anims");
        for (info, dumped_info) in level.pak.block1.infos.animations.iter().zip(dumped_infos.animations) {
            let mut dumped_info = dumped_info.clone();
            dumped_info.offset = info.offset;
            assert!(&dumped_info == info);
        }
        println!("dumped headers ok");
        for (src, dst) in level.pak.animation_data.iter().zip(datas) {
            if let lotrc::types::CompressedData::Owned(dst) = dst {
                assert!(src.data == dst.data.as_ref());
            } else {
                use lotrc::types::CompressedData;
                let ty = match dst {
                    CompressedData::Ref(_) => "Ref",
                    CompressedData::Alt(_) => "Alt",
                    CompressedData::Owned(_) => "Owned",
                    CompressedData::Texture0(_) => "Texture0",
                    CompressedData::Texture1(_) => "Texture1",
                    CompressedData::None(_) => "None",
                };
                println!("aaaaa {}", ty);
            }
        }
        println!("dumped animation data ok");
    }
    if false {
        use lotrc::level::pak::block1::objs::DumpObjsPc;
        let (models, terrain, occluder) = level.pak.block1.objs.group_models(&level.pak.block1.sub_blocks.level);
        for model in models {
            use lotrc::level::model::DumpModelPc;
            use lotrc::types::GetNative;
            let mut counts = lotrc::level::pak::block1::infos::InfoCounts::default();
            let mut size = model.add_size(0, &mut counts);
            println!("estimated model size {}", size);
            size += counts.size_pc(0);
            let mut data = lotrc::types::AlignedBuf::with_capacity(size);
            let mut dst = lotrc::types::DumpSlice::from(&mut data[..]);
            let mut offsets: Vec<lotrc::types::u32Pc> = vec![0.into(); counts.offsets];
            let mut infos = lotrc::level::pak::block1::infos::DumpInfosPc::from_data(&mut dst, &counts, &mut offsets).expect("infos dump");
            let mut header = lotrc::level::pak::PakHeaderPc::default();
            infos.update_header(&mut header);
            model.dump_into(&mut dst, &mut infos).expect("model dump");
            let model_data = infos.model_data.iter().filter_map(|v| {
                if let lotrc::types::CompressedData::Ref(val) = v.data {
                    Some((v.key.get(), val))
                } else {
                    None
                }
            }).collect();
            let dumped_infos = lotrc::level::pak::block1::infos::InfosRefPc::from_data(&data[..], &header).expect("dumped infos");
            println!("dumped infos model {}", dumped_infos.models.len());
            let dumped_model = lotrc::level::model::ModelRefPc::from_data(&data[..], &dumped_infos.models[0], &model_data).expect("dumped model");
        }
        println!("dumping terrain");
        for model in terrain {
            use lotrc::level::model::DumpModelPc;
            use lotrc::types::GetNative;
            let mut counts = lotrc::level::pak::block1::infos::InfoCounts::default();
            let mut size = model.add_terrain_size(4, &mut counts);
            println!("estimated terrain size {} {}", model.key(), size);
            size += counts.size_pc(0);
            let mut data = lotrc::types::AlignedBuf::with_capacity(size);
            let mut dst = lotrc::types::DumpSlice::from(&mut data[..]);
            lotrc::types::i32Pc::from(-1).dump_into(&mut dst).unwrap();
            let mut offsets: Vec<lotrc::types::u32Pc> = vec![0.into(); counts.offsets];
            let mut infos = lotrc::level::pak::block1::infos::DumpInfosPc::from_data(&mut dst, &counts, &mut offsets).expect("infos dump");
            let mut header = lotrc::level::pak::PakHeaderPc::default();
            infos.update_header(&mut header);
            //println!("{:#?}", model.info());
            model.dump_terrain_into(&mut dst, &mut infos, 0).expect("model dump");
            let model_data = infos.model_data.iter().filter_map(|v| {
                if let lotrc::types::CompressedData::Ref(val) = v.data {
                    Some((v.key.get(), val))
                } else {
                    None
                }
            }).collect();
            let dumped_infos = lotrc::level::pak::block1::infos::InfosRefPc::from_data(&data[..], &header).expect("dumped infos");
            println!("dumped infos model {}", dumped_infos.models.len());
            let dumped_model = lotrc::level::model::ModelRefPc::from_data(&data[..], &dumped_infos.models[0], &model_data).expect("dumped model");
        }
    }
    if false {
        if let Some(crowd) = &level.pak.block2.sub_blocks.crowd {
            println!("Level has crowd");
            use lotrc::level::pak::block2::DumpCrowdPc;
            let mut data = lotrc::types::AlignedBuf::with_capacity(crowd.size());
            let mut dst = lotrc::types::DumpSlice::from(&mut data[..]);
            crowd.dump_into(&mut dst).unwrap();
            println!("AAAAA {:?}", crowd.header);
            let crowd_alt = lotrc::level::pak::block2::CrowdRefPc::from_data(&data[..]).unwrap();
        }
    }
    if true {
        let data = level.dump(lotrc::re_export::Compression::fast()).expect("dump");
        {
            use std::fs::File;
            use std::io::Write;
            let mut out = File::create("dump.pak").unwrap();
            out.write_all(&data.pak[..]).unwrap();
        }
        {
            use std::fs::File;
            use std::io::Write;
            let mut out = File::create("dump.bin").unwrap();
            out.write_all(&data.bin[..]).unwrap();
        }
        let mut compressed_data_dump = lotrc::level::LevelCompressedData::default();
        let dumped_header = lotrc::level::pak::PakHeaderPc::from_data(&data.pak).unwrap();
        println!("original: {:#?}\ndumped: {:#?}", level.pak.header, dumped_header);
        if let Some(dumped_level) = lotrc::level::LevelRefPc::from_data(&data, &mut compressed_data_dump).ok() {
            println!("dumped level parsed");

            // validate offsets
            assert!(level.pak.block2.offsets.len() == dumped_level.pak.block2.offsets.len());
            if level.pak.block2.sub_blocks != dumped_level.pak.block2.sub_blocks {
                println!("sub_blocks2 differ");
                let a = &level.pak.block2.sub_blocks;
                let b = &dumped_level.pak.block2.sub_blocks;
                if a.info != b.info {
                    if a.info.header != b.info.header { println!("header differs"); }
                    for (a, b) in a.info.block_headers.iter().zip(b.info.block_headers.iter()) {
                        if a != b {
                            println!("block {} {} {} {} {} differs", get_string(&a.key.to_native(), &level), a.offset, b.offset, a.size, b.size);
                        }
                    }
                }
                if a.spray != b.spray { println!("spray differs") }
                if a.crowd != b.crowd { println!("crowd differs") }
                if a.pfields != b.pfields { println!("pfields differs") }
                if a.langs != b.langs { println!("langs differs") }
                if a.files != b.files { println!("files differs") }
                
            }
            if level.pak.block1.sub_blocks != dumped_level.pak.block1.sub_blocks {
                println!("sub_blocks1 differ");
                let a = &level.pak.block1.sub_blocks;
                let b = &dumped_level.pak.block1.sub_blocks;
                if a.info != b.info {
                    if a.info.header != b.info.header { println!("header differs"); }
                    for (a, b) in a.info.block_headers.iter().zip(b.info.block_headers.iter()) {
                        if a != b {
                            println!("block {} {} {} {} {} differs", get_string(&a.key.to_native(), &level), a.offset, b.offset, a.size, b.size);
                        }
                    }
                }
            }
            if level.pak.block1.string_keys != dumped_level.pak.block1.string_keys {
                println!("string keys differ");
                let a = &level.pak.block1.string_keys;
                let b = &dumped_level.pak.block1.string_keys;
                if a.header != b.header { println!("{:?}\n{:?}", a.header, b.header); }
                if a.pad != b.pad { println!("string keys pad differs"); }
                if a.vals != b.vals {
                    println!("string keys vals differs");
                    for (a, b) in a.vals.iter().zip(b.vals.iter()) {
                        if a != b { println!("{:?}, {:?}", a, b); }
                    }
                }
            }
            if level.pak.block1.objs != dumped_level.pak.block1.objs {
                println!("objs differ");
                let a = &level.pak.block1.objs;
                let b = &dumped_level.pak.block1.objs;
                if a.textures != b.textures { println!("textures differs"); }
                if a.models != b.models {
                    println!("models differs {}, {}", a.models.len(), b.models.len());
                    for (a, b) in a.models.values().zip(b.models.values()) {
                        if a == b { continue; }
                        println!("{} {}, {}", get_string(&a.info.key.to_native(), &level), get_string(&b.info.key.to_native(), &dumped_level), a.info.key); 
                        if a.info != b.info {
                            println!("{:#?}\n{:#?}", a.info, b.info);
                        }
                        if a.bones != b.bones { println!("model bones differ"); }
                        if a.mat_order != b.mat_order { println!("model mat_order differ"); }
                        if a.mesh_order != b.mesh_order { println!("model mesh_order differ"); }
                        if a.mesh_bounding_boxes != b.mesh_bounding_boxes { println!("model mesh_bounding_boxes differ"); }
                        if a.skin_binds != b.skin_binds { println!("model skin_binds differ"); }
                        if a.skin_order != b.skin_order { println!("model skin_order differ"); }
                        if a.vals_j != b.vals_j { println!("model vals_j differ"); }
                        if a.val_k_header != b.val_k_header { println!("model val_k_header differ"); }
                        if a.slots != b.slots { println!("model slots differ"); }
                        if a.slot_map != b.slot_map { println!("model slot_map differ"); }
                        if a.block_header != b.block_header { println!("model block_header differ"); }
                        if a.block_offsets != b.block_offsets { println!("model block_offsets differ"); }
                        if a.blocks != b.blocks { println!("model blocks differ"); }
                        if a.buffer_infos != b.buffer_infos { println!("model buffer_infos differ"); }
                        if a.vbuff_order != b.vbuff_order { println!("model vbuff_order differ"); }
                        if a.ibuff_order != b.ibuff_order { println!("model ibuff_order differ"); }
                        if a.vbuffs != b.vbuffs { println!("model vbuffs differ"); }
                        if a.ibuffs != b.ibuffs { println!("model ibuffs differ"); }
                        if a.mats != b.mats { println!("model mats differ"); }
                        if a.hk_constraint != b.hk_constraint { println!("model hk_constraint differ"); }
                        if a.hk_constraint_datas != b.hk_constraint_datas { println!("model hk_constraint_datas differ"); }
                        if a.shapes != b.shapes { println!("model shapes differ"); }
                        if a.data != b.data {
                            println!("model data differ"); 
                            if a.data.infos != b.data.infos { println!("model data infos differ"); }
                            if a.data.vbuff_order != b.data.vbuff_order { println!("model vbuff_order infos differ"); }
                            if a.data.ibuff_order != b.data.ibuff_order { println!("model ibuff_order infos differ"); }
                            if a.data.vertex != b.data.vertex { println!("model data vertex differ"); }
                            if a.data.index != b.data.index { println!("model data index differ"); }
                            if a.data.data != b.data.data { 
                                println!("model data data differ");
                                if let Some(data) = a.data.data {
                                    println!("{:?}", &data.data_decomp[..]);
                                } else {
                                    println!("None");
                                }
                                if let Some(data) = b.data.data {
                                    println!("{:?}", &data.data_decomp[..]);
                                } else {
                                    println!("None");
                                }
                            }
                        }
                    }
                }
                if a.effects != b.effects { println!("effects differs"); }
                if a.gfxs != b.gfxs {
                    println!("gfxs differs {} {}", a.gfxs.len(), b.gfxs.len());
                    for ((k1, v1), (k2, v2)) in a.gfxs.iter().zip(b.gfxs.iter()) {
                        //if (v1 != v2) { println!("{:?}\n{:?}", v1, v2); }
                        if k1 != k2 { println!("{} {}", k1, k2); }
                    }
                }
                if a.foliages != b.foliages { println!("foliages differs"); }
                if a.radiosity != b.radiosity { println!("radiosity differs"); }

            }
            println!("{} {}", level.bin.asset_handles.len(), dumped_level.bin.asset_handles.len());
        } else {
            println!("dumped level failed to parse");
        }
        // validate offsets
        for (src, dst) in level_compressed_data.pak.animations.iter().zip(&compressed_data_dump.pak.animations) {
            println!("{} {}", src.data_decomp.len(), dst.data_decomp.len());
            assert!(src.data_decomp == dst.data_decomp);
        }
        for ((k1, src), (k2, dst)) in level_compressed_data.bin.texture_data.iter().zip(compressed_data_dump.bin.texture_data.iter()) {
            assert!(k1 == k2);
            assert!(src.data_decomp == dst.data_decomp);
        }
        for (k, src) in level_compressed_data.bin.model_data.iter() {
            let dst = compressed_data_dump.bin.model_data.get(k).unwrap();
            assert!(src.data_decomp == dst.data_decomp);
        }
    }


/*
    let t = std::time::Instant::now();
    let level = lotrc::level::LevelRef::from_data(path).unwrap();
    println!("level_raw parsed in {:?}", t.elapsed());

    let t = std::time::Instant::now();
    let _level = lotrc::level::Level::parse(&level);
    println!("level parsed in {:?}", t.elapsed());
*/
    //let pak_data = std::fs::read(path.with_extension("PAK")).unwrap();
    //let bin_data = std::fs::read(path.with_extension("BIN")).unwrap();
    //let level = lotrc::level::LevelPC::from_bytes(pak_data, bin_data).unwrap();

    //println!("{:?}", level.pak().block1().sub_blocks().block_headers()[0].size);
    //let blocks = level.pak().block1().sub_blocks().blocks();
    //let obj = &blocks[blocks.len()-1].level().unwrap().objs()[0];
    //for x in obj.fields().keys() {
    //    println!("{:?}", get_str(x))
    //}
    //let name = obj.fields().get(&hash_string(b"name", None)).unwrap().crc().unwrap();
    //println!("{:?}", get_str(&name.get()));

    //println!("{:?}", level.pak_header);
    //println!("{:?}", level.animations[0][&0]);
    //println!("{:?}", level.pak_strings);
}
