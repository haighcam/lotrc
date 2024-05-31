import lotrc
import os

# decompiling them is unneded for this and is slow
lotrc.types.DECOMP_LUA = False

# path to folder containing Conquest.exe
conquest_dir = "CHANGE ME!

# path to folder containing levels from 1 dlc (should have .dat, .BINs and .PAKs in it
src_dir = "CHANGE ME!"

# use a different folder under addons for each dlc eg (AddOn/HeroesandMapsPack and AddOn/HeroArenaBonus)
dst_dir = f"{conquest_dir}/AddOn/HeroesandMapsPack"

# convert all items in the dlc folder to PC format (except audio stuff since I haven't looked into that yet)
for f in os.listdir(src_dir):
    if os.path.isdir(f):
        print(f"Audio stuff is currently not handled, Copying folder {f} as is. ")
        shutil.copytree(f"{src_dir}/{f}", f"{dst_dir}/{f}", dirs_exist_ok=True)
    else:
        name, ext = f.split('.')
        if ext == 'dat':
            print(f"Converting level infos {f}")
            levels = lotrc.LevelInfo(f"{src_dir}/{f}")
            data = levels.dump('<')
            with open(f"{dst_dir}/{f}", "wb") as f:
                f.write(data)
        elif ext == 'BIN': continue
        elif ext == 'PAK':
            print(f"Converting level {name}")
            level = lotrc.LevelData(f"{src_dir}/{name}")
            f_pak, f_bin = level.dump('<')
            with open(f"{dst_dir}/{name}.BIN", "wb") as f:
                f.write(f_bin)
            with open(f"{dst_dir}/{name}.PAK", "wb") as f:
                f.write(f_pak)
        else:
            print(f"Unhandled file {f}, copying as is")