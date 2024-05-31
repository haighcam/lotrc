import lotrc
import numpy as np
from lotrc.utils import *

level_info = lotrc.LevelInfo("Levels/level_info.dat")

# find the gamemode and level
level_index = np.nonzero(level_info.levels['name'] == b"Mount_Doom")[0].item()
gamemode_index = np.nonzero(level_info.gamemodes['key'] == hash_string("Ringbearer"))[0].item()

# add that gamemode to mount_doom
level_info.levels['gamemodes'][level_index] |= 1 << gamemode_index

with open("Levels/level_info.dat", "wb") as f:
    f.write(level_info.dump('<'))