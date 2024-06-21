Command line utility for converting some Lord of the Rings Conquest files to and from a more editable format.
Currently supports:
- Level Data (.PAK/.BIN)
- level_info.dat
- WWiseIDTable.bin

Has somewhat sane default behavior for dragging and dropping files/folders onto the executable if command line parameters are not required

Uses [_lua-bytecode_](https://github.com/lua-bytecode/lua-bytecode.github.io) (included as a submodule), for converting lua files. This is only relevant for converting xbox level files.

Can use [_unluac.jar_](https://sourceforge.net/projects/unluac/) for decompiling lua files.

level_info, WWiseIDTable and the sub_blocks inside of levels should be fully editable. Other aspects can be modified but may break the resulting level if modified incorrectly. Python scripts to help with swapping some objects between levels are provided in the examples folder (Some of them require lupa to be installed, which it will already be if the python version of the tool was previously installed).

# Command line usage
```
Usage: lotrc_rs.exe [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...  Input files or folders

Options:
  -o, --output <OUTPUT>            Output folder
  -c, --compile                    Compile the inputs to new levels / level_infos
  -d, --dump                       Dump the inputs to an editable form
      --lua-decomp                 Decompile lua files when loading a level
      --lua-recomp                 Compile lua files when loading a level, also converts endianess for xbox lua files
      --compression <COMPRESSION>  Zlib compression level to use when compiling levels, lower numbers are faster
      --unluac <UNLUAC>            Path to unluac.jar if decompiling lua files
  -h, --help                       Print help
  -V, --version                    Print version
```
simple examples:
 - dump Minas Tirith Top:    
 ```bash lotrc_rs.exe -d -o DumpedLevels 'The Lord of the Rings Conquest 2\Levels\MinasTirith_Top'```
 - compile Minas Tirith Top:  
 ```bash lotrc_rs.exe -c -o 'The Lord of the Rings Conquest 2\Levels' DumpedLevels\MinasTirith_Top```
 - dump all levels:  
 ```bash lotrc_rs.exe -d -o DumpedLevels 'The Lord of the Rings Conquest 2\Levels\*'```
 - compile all levels:  
 ```bash lotrc_rs.exe -c -o 'The Lord of the Rings Conquest 2\Levels' DumpedLevels\*```

# Compiling from source
Requires rust and cargo.
```bash
git clone -b rust --single-branch --recurse-submodules https://github.com/haighcam/lotrc.git lotrc_rs
cd lotrc_rs
cargo build --release 
```