use clap::Parser;
//use lotrc::types::{get_str, hash_string};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,
}

fn main() {
    let logger = pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
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

    let args = Args::parse();

    let path = PathBuf::from(args.path);
    println!("Reading level: {:?}", path);

    let t = std::time::Instant::now();
    let level = lotrc::level::LevelRef::from_data(path).unwrap();
    println!("level_raw parsed in {:?}", t.elapsed());

    let t = std::time::Instant::now();
    let _level = lotrc::level::Level::parse(&level);
    println!("level parsed in {:?}", t.elapsed());
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
