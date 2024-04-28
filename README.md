
# Lord of the Rings Conquest Level Parsing

## Requirements

place _lua-bytecode.lua_ from [here](https://github.com/lua-bytecode/lua-bytecode.github.io) in this directory

place _unluac.jar_ from [here](https://sourceforge.net/projects/unluac/) in this directory, requires java

---
installation:

``` bash
cd lotrc
pip install -e .
```

---
example usage:
``` python
import lotrc
from lotrc.utils import *

# endianess is read from the file
levels = lotrc.LevelInfo("path to level_info.dat")
# do something with the levels, 

# "<" dump little endian, use ">" for big endian 
# converting .dat files works (dump with the other format), 
# still needs more work for .BIN and .PAK to work 
# although it will attempt the conversion anyway

f = "<" 
# dump new data then save it to a new .dat file or something
data = levels.dump(f)

# use the path to a .PAK or .BIN, but remove the suffix, 
# both are parsed to load the level
level = lotrc.level.LevelData("path to a level")

# maybe save the level variable in a pickle if you want to test lots of changes since it takes a bit to parse the level

# do somthing with the level, maybe change the code for some scripts, they will get recompiled and packed

# maybe look at a script
name = level.keys[level.sub_blocks1.block_headers[42]['key']]
code = level.sub_blocks1.blocks[42].code
print(name, code)

# or look at a data value
print_data(level.vbuff_infos[0])

# dump level data and write to new .PAK and .BIN files or something
pak_data, bin_data = level.dump(f)
```


---
Other random notes:

> Extra levels (Such as DLC) can be loaded on the PC version by placing the relevant level.dat and other level files in AddOn/\<some folder\>/ where some folder is a unique name for the levels (the name does not seem to matter)

> A lot of vales are a crc computed from a debug string (the block of utf-8 strings near the end of files), you can look them up using the .keys hashmap on the loaded file, although not all strings are present in the file so some values will be missing. Use the hash_string util to do this in reverse

> DLC files are almost the same as the PC ones (at least for the .bin, .PAK and .BIN files) except that data is big endian instead of little endian, textures are stored a bit differently in .BIN files, and there are a couple objects that are different sizes / have fields in different orders in the .PAK files

> Things that are still unhandled:
> - Radiostity data in the .BIN file, haven't reversed engineered the format yet
> - PField data, not sure what it is doing,
> - SSA data, seems to be a subtitle format just haven't touched them yet
> - A lot of fields in data structures have unknown uses (some of them might just exist to reserve space for a struct once it is loaded), and others might have the wrong data type (althoug their size should be correct)