import lotrc
import lotrc.level_alt
from lotrc.utils import *
from lotrc.types import *
lotrc.types.DECOMP_LUA = False

### Porting the Balrog from the Shire to Minas Tirith Top. 

# some utilities for getting things from dumped level file
def find_obj(vals, guid):
    for obj in vals['objs']:
        if obj['fields']['guid'] == guid:
            return obj

def find_type(vals, name):
    for ty in vals['types']:
        if ty['name'] == name:
            return ty
            
# grabs an object and all sub objects from a dumped level file
# parts can be uncommented to print some stuff about 
#    meshes, effects and scripts that are needed for the objects to work propoerly (or you can try to find everything in a dumped json file
def copy_tree(vals, guid, processed=None, gamemodemask=None):
    if processed is None:
        processed = set()
    elif guid in processed:
        return []
    processed.add(guid)
    obj = find_obj(vals, guid)
    ty = find_type(vals, obj['type'])
    objs = [obj]
    for t in ty['fields']:
        if t['type'] == 'guid':
            val = obj['fields'][t['name']]
            if val != 0:
                objs.extend(copy_tree(vals, val, processed, gamemodemask))
        elif t['type'] == 'objectlist':
            for val in obj['fields'][t['name']]:
                objs.extend(copy_tree(vals, val, processed, gamemodemask))
        elif t['name'] == "GameModeMask" and gamemodemask is not None:
            obj['fields']['GameModeMask'] |= gamemodemask
        # elif ('mesh' in t['name'] or 'Mesh' in t['name']) and t['type'] in ['crc', 'crclist']:
        #     val = obj['fields'][t['name']]
        #     if val != '' and val != []:
        #         print(t['name'], val)
        # elif ('script' in t['name'] or 'Script' in t['name']) and t['type'] in ['crc', 'crclist', 'string', 'stringlist']:
        #     val = obj['fields'][t['name']]
        #     if val != '' and val != []:
        #         print(t['name'], val)
        # elif ('effect' in t['name'] or 'Effect' in t['name']) and t['type'] in ['crc', 'crclist', 'string', 'stringlist']:
        #     val = obj['fields'][t['name']]
        #     if val != '' and val != []:
        #         print(t['name'], val)
    return objs

# parse the level files
levelMtt = lotrc.level_alt.LevelData("Levels/MinasTirith_Top")
levelShire = lotrc.level_alt.LevelData("Levels/Shire")

# dump the level block to move relevant infomation
vals = levelShire.sub_blocks1.blocks[-1].to_dict(levelShire.keys)
vals_dest = levelMtt.sub_blocks1.blocks[-1].to_dict(levelMtt.keys)

# these are the guids for the corresponding objects in MTT
spawn_emmiter_mtt_tdm_team2 = 7052754 # to add the balrog as a normal evil class
gamemodemask = 43 # grabbed from a different class in the spawn list, ensures the ported objects will be visible in the targeted gamemode
gamemode_tdm = 7052744 # gamemode object, needed for adding relevant soundbanks

# the balrog class id from the Shire
balrog_class_id = 109012487
# grab the balrog class object and all objects it depends on, setting the gamemode mask on objects that have it
balrog_class_items = copy_tree(vals, balrog_class_id, gamemodemask=43)

# add any types that are missing
valid_types = set(i['name'] for i in vals_dest['types'])
needed_types = [find_type(vals, t) for t in set(o['type'] for o in balrog_class_items).difference(valid_types)]
vals_dest['types'].extend(needed_types)

# grab the meshes and textures needed for the balrog
meshes = [
    'WP_bal_sword_01',
    'CH_bal_ragdoll',
    'CH_bal_Balrog_01'
]
meshes = {(k := lotrc.types.hash_(i)): levelShire.meshes[k] for i in meshes}
textures = {}
for mat in [j for i in meshes.values() for j in i.mats]:
    textures.update({i: levelShire.textures[i] for i in mat['textures'] if i != 0})

# grab the effects needed for the balrog
effects = [
    'FX_AB_Balrog_PowerUp_Looping',
    'CHFX_Bal_FireSword_Loop_world',
    'CHFX_Bal_FireSword_Loop',
    'FX_BalrogFire',
    'FX_BalrogFire_BackEmbers',
    'FX_BalrogEmbers_Bicep',
    'FX_BalrogEmbers_Forearm',
    'FX_BalrogEmbers_Thigh',
    'FX_AB_Captain_power_aura',
]
effects = {(k := lotrc.types.hash_(i)): levelShire.effects[k] for i in effects}

# grab the animations needed for the balrog (from AT_GNT_Balrog)
animations = [
    'CH_bal_fort_loop',
    'CH_bal_fort_dismount',
    'CH_bal_idle',
    'CH_Bal_turn180_E',
    'CH_Bal_turn180_W',
    'CH_Bal_turn90_W',
    'CH_Bal_turn90_E',
    'CH_bal_loc_fwd_walk_N',
    'CH_bal_loc_fwd_walk_E',
    'CH_bal_loc_fwd_walk_W',
    'CH_bal_loc_bck_walk_S',
    'CH_bal_loc_bck_walk_E',
    'CH_bal_loc_bck_walk_W',
    'CH_bal_loc_fwd_run_N',
    'CH_bal_loc_fwd_run_E',
    'CH_bal_loc_fwd_run_W',
    'CH_bal_loc_bck_run_S',
    'CH_bal_loc_bck_run_E',
    'CH_bal_loc_bck_run_W',
    'CH_bal_loc_bck_run_S',
    'CH_bal_loc_bck_run_S',
    'CH_Bal_Death_inPlace_full',
    'CH_Bal_Death_inPlace_full',
    'CH_bal_attack_Horizontal_01',
    'CH_bal_attack_Horizontal_02',
    'CH_bal_attack_WingStab',
    'CH_bal_attack_V1',
    'CH_bal_attack_V1_toIdle',
    'CH_bal_attack_OverheadPunch',
    'CH_bal_Attack_FireBreath',
    'CH_bal_Attack_Powerup',
    'CH_bal_pickup',
    'CH_bal_pickup_miss',
    'CH_bal_pickup_blocked',
    'CH_bal_pickup_kill_01',
    'CH_bal_pickup_kill_02',
    'CH_bal_pickup_kill_03',
    'CH_bal_pickup_throw_01',
    'CH_bal_Jump_Fall',
    'CH_bal_Jump_Fall',
    'CH_bal_Jump_Fall',
    'CH_bal_Jump_Fall',
    'CH_bal_Jump_Landing_toIdle',
    'CH_bal_Jump_Landing_toRun',
    'CH_bal_hitTwitch_front',
    'CH_bal_hitTwitch_back',
    'CH_bal_hitTwitch_left',
    'CH_bal_hitTwitch_right',
    'CH_bal_fort_mount',
    'CH_bal_fort_loop',
    'CH_bal_Fort_Left',
    'CH_bal_Fort_Right',
    'CH_bal_fort_dismount',
    'CH_bal_engaged_idle',
    'CH_bal_Taunt_Aggressive',
    'CH_bal_Taunt_Celebration',
    'CH_bal_Taunt_Defensive',
    'CH_bal_Taunt_Aggressive',
    'CH_bal_Taunt_Celebration',
    'CH_bal_Taunt_Defensive'
]
animations = {(k := lotrc.types.hash_(i)): levelShire.animations[k] for i in animations}

# update the gamemode mask for all objects
for k in textures.keys():
    textures[k][0]['level_flag'] = gamemodemask
for k in meshes.keys():
    meshes[k].info['level_flag'] = gamemodemask
for k in effects.keys():
    effects[k].level_flag = gamemodemask
for k in animations.keys():
    # the mask is slightly different and should be set as (1 << index) 
    #    where index is the index into the animation_block_infos that corresponds to the target gamemode
    # or set it to all gamemodes
    animations[k].info['level_flag'] = 0xFF

# add all of the meshes, textures, effects and animations if they are not already there
# (if they are already there you may need to update the gamemodemask, which this does not do)
levelMtt.textures.update({k:v for k,v in textures.items() if k not in levelMtt.textures})
levelMtt.meshes.update({k:v for k,v in meshes.items() if k not in levelMtt.meshes})
levelMtt.effects.update({k:v for k,v in effects.items() if k not in levelMtt.effects})
levelMtt.animations.update({k:v for k,v in animations.items() if k not in levelMtt.animations})


# If a script imports other scripts then those may need to be transfered as well
#   so check the imports, although most of the common ones should be there
#   it does depend on some troll/ent stuff so for levels lacking those more scripts might be needed
scripts = [
    'ANM_GNT_HERO_Balrog',
    'AT_GNT_Balrog', #contains all of the animations
    'BC_GNT_HERO_Balrog',
    'SM_HERO_Balrog',
    'ATK_SM_HERO_Balrog', 
    'ATK_SM_HERO_Balrog',
    'ATK_INFO_HERO_Balrog',
    'IM_UNIT_Balrog',
    'MG_Balrog'
]
# add the sripts befoer the uv and level files since those always seem to be at the back. There is probably a better order for the files, but this works
inds = sorted([np.nonzero(levelShire.sub_blocks1.block_headers['key'] == lotrc.types.hash_(i + '.lua'))[0].item() for i in scripts])
for i in inds:
    levelMtt.sub_blocks1.blocks.insert(-3, levelShire.sub_blocks1.blocks[i])
levelMtt.sub_blocks1.block_headers = np.insert(levelMtt.sub_blocks1.block_headers, -3, levelShire.sub_blocks1.block_headers[inds])

# add the soundbanks for the balrog to the team death match gamemode. You'll need to check the source level.json to which banks (if any) are needed
soundbnks = [
    "ChatterHeroBalrog.bnk",
    "SFXBalrog.bnk",
]
o = find_obj(vals_dest, gamemode_tdm)
o['fields']['ModeSpecificBanks'].extend(i for i in soundbnks if i not in o['fields']['ModeSpecificBanks'])

# add the balrog to the evil team
o = find_obj(vals_dest, spawn_emmiter_mtt_tdm_team2)
o['fields']['classes'].append(balrog_class_id)

# add the objects for the balrog, only if they are missing
vals_dest['objs'].extend(i for i in balrog_class_items if find_obj(vals_dest, i['fields']['guid']) is None)

# load the modified level block
levelMtt.sub_blocks1.blocks[-1] = lotrc.types.GameObjs.from_dict(vals_dest)

# copy over the debug strings
s = set(levelMtt.pak_strings)
levelMtt.pak_strings.extend(i for i in levelShire.pak_strings if i not in s)
s = set(levelMtt.bin_strings)
levelMtt.bin_strings.extend(i for i in levelShire.bin_strings if i not in s)
levelMtt.keys.update(levelShire.keys)

# should proabably copy over the relevant localized text as well, but it doesn't seem to crash works without it.

# dump the modified level
(_, pak_data, bin_data) = levelMtt.dump(compress=True)
with open("Levels/MinasTirith_Top.BIN", "wb") as f:
    f.write(bin_data)
with open("Levels/MinasTirith_Top.PAK", "wb") as f:
    f.write(pak_data)

