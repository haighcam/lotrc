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

# the level.uv is not in Level.json, it can be modified and loaded with the other script