import lotrc
from lotrc.utils import *
from lotrc.types import *
import json

# decompiling them is unneded for this and is slow
lotrc.types.DECOMP_LUA = False

# load some level data
levelDLC = lotrc.LevelData("Xbox/AddOn/HeroesandMapsPack/Amon_Hen")

# dump the level.uv file to json format
vals = levelDLC.sub_blocks1.blocks[-1].to_dict(levelDLC.keys)

with open("Level.json", "w") as f:
    json.dump(vals, f, indent=1)


# do some editing, then save to LevelNew.json


# load the new level.uv
with open("LevelNew.json", "r") as f:
    vals = json.load(f)

# dump the new file
f_pak, f_bin = levelDLC.dump()
with open("AddOn/HeroesandMapsPack/Amon_Hen.BIN", "wb") as f:
    f.write(f_bin)
with open("AddOn/HeroesandMapsPack/Amon_Hen.PAK", "wb") as f:
    f.write(f_pak)