use std::{
    fs, 
    path::{Path, PathBuf},
    collections::VecDeque,
};
use audio::AudioTable;
use zerocopy::LE;
use log::error;
use clap::{Parser, Args};

mod audio;
mod types;
mod pak;
mod pak_alt;
mod bin;
mod level_alt;
mod level_info;
mod level;
mod lua_stuff;

use level::Level;
use level_info::LevelInfo;

fn v3_styling() -> clap::builder::styling::Styles {
    use clap::builder::styling::*;
    Styles::styled()
        .header(clap::builder::styling::AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles=v3_styling())]
struct CliArgs {
    /// Input files or folders
    #[arg(num_args = 1..)]
    input: Vec<String>,

    /// Output folder
    #[arg(short, long)]
    output: Option<String>,

    #[command(flatten)]
    command: Commands,

    /// Decompile lua files when loading a level
    #[arg(long)]
    lua_decomp: bool,

    /// Compile lua files when loading a level, also converts endianess for xbox lua files
    #[arg(long, requires="unluac")]
    lua_recomp: bool,

    /// Zlib compression level to use when compiling levels, lower numbers are faster
    #[arg(long, value_parser = clap::value_parser!(u32).range(0..10))]
    compression: Option<u32>,

    /// Path to unluac.jar if decompiling lua files
    #[arg(long)]
    unluac: Option<String>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct Commands {
    /// Compile the inputs to new levels / level_infos
    #[arg(short, long)]
    compile: bool,

    /// Dump the inputs to an editable form
    #[arg(short, long)]
    dump: bool,

    #[arg(long, hide=true)]
    alt_comp: bool
}

fn parse<A: AsRef<Path>, B: AsRef<Path>>(src: A, dest: B, args: &Commands) {
    let mut q = VecDeque::from(vec![(PathBuf::new(), src.as_ref().to_path_buf())]);
    let dest = dest.as_ref();
    while let Some((name, src)) = q.pop_front() {
        let mut raw_name = src.file_name().unwrap().to_str().unwrap().split('.');
        let name = name.join(raw_name.next().unwrap());
        let ext = raw_name.collect::<Vec<_>>().join(".");
        if src.with_extension("PAK").is_file() {
            if args.compile {
                Level::parse(src).dump::<LE, _>(dest.join(name));
            } else if args.alt_comp {
                level_alt::Level::parse(src).dump::<LE, _>(dest.join(name));
            } else {
                level_alt::Level::parse(src).to_file(dest.join(name));
            }
        } else if src.file_name().unwrap() == "level_info.dat" {
            let level_info = LevelInfo::parse(src);
            if args.compile {
                level_info.dump::<LE, _>(dest.join(name));
            } else {
                level_info.to_file(dest.join(name));
            }
        } else if !src.with_extension("PAK").is_file() && src.with_extension("bin").is_file() {
            let table: AudioTable = AudioTable::parse(src);
            if args.compile {
                table.dump::<LE, _>(dest.join(name))
            } else {
                table.to_file(dest.join(name));
            }
        } else if ext == "audio.json" {
            let table = AudioTable::from_file(src);
            if args.dump {
                table.to_file(dest.join(name));
            } else {
                table.dump::<LE, _>(dest.join(name))
            }
        } else if src.is_dir() {
            if src.join("index.json").is_file() {
                let level_info = LevelInfo::from_file(src);
                if args.dump {
                    level_info.to_file(dest.join(name));
                } else {
                    level_info.dump::<LE, _>(dest.join(name));
                }
            } else if src.join("pak_header.json").is_file() {
                let level = level_alt::Level::from_file(src);
                if args.dump {
                    level.to_file(dest.join(name));
                } else {
                    level.dump::<LE, _>(dest.join(name))
                }
            } else {
                for path in fs::read_dir(&src).unwrap().map(|x| x.unwrap().path()) {
                    q.push_back((name.clone(), path));
                }
            }
        } else {
            error!("Could not parse input {:?}", src);
        }
    }
}

fn main() {
    let logger = pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            use std::io::Write;
            use pretty_env_logger::env_logger::fmt::Color;
        
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

    let multi = indicatif::MultiProgress::new();
    indicatif_log_bridge::LogWrapper::new(multi.clone(), logger).try_init().unwrap();

    let args = CliArgs::parse_from(wild::args_os());

    *types::DECOMP_LUA.lock().unwrap() = args.lua_decomp;
    *types::RECOMP_LUA.lock().unwrap() = args.lua_recomp;
    if let Some(compression) = args.compression {
        *types::COMPRESSION.lock().unwrap() = flate2::Compression::new(compression);
    }
    if let Some(unluac) = args.unluac {
        *types::UNLUAC.lock().unwrap() = unluac;
    }

    let exe_dir = std::env::current_exe().unwrap().parent().unwrap().to_owned();
    let output: PathBuf = args.output.map(|x| x.into()).unwrap_or(exe_dir);
    for input in args.input {
        parse(input, output.clone(), &args.command);
    }
}