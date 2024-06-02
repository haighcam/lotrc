import lotrc
from lotrc.utils import *
from lotrc.types import *
lotrc.types.DECOMP_LUA = False

# this example edits the text in the main menu, 
# It works for levels by replacing local_strings with sub_blocks2, everything else stays the same

level_info = lotrc.LevelInfo("Levels/level_info.dat")

text_ind = np.nonzero(level_info.string_keys.string_keys['key'] == hash_string('BKG.name'))[0].item()

block_ind = np.nonzero(level_info.local_strings.block_headers['key'] == hash_string("English"))[0].item()

level_info.local_strings.blocks[block_ind].strings[text_ind] = "Blue Gates"

with open("Levels/level_info.dat", "wb") as f:
    f.write(level_info.dump('<'))