use std::io::Write;
use mlua::prelude::*;
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, Arc};

use super::types::Crc;

const LUA_BYTECODE: &str = include_str!("../res/lua-bytecode.github.io/lua-bytecode.lua");

#[derive(Debug, Default)]
pub struct LuaCompiler {
    pub lua: Lua,
}

impl LuaCompiler {
    pub fn new() -> LuaResult<Self> {
        let lua: Lua = Lua::new();
        lua.globals().set("lua_bytecode", lua.load(LUA_BYTECODE).eval::<LuaFunction>()?)?;
        // let lua_bytecode: LuaFunction = lua.load(LUA_BYTECODE).eval()?;
        Ok(Self { lua })
    }

    pub fn convert(&self, code: &[u8], format: &str) -> LuaResult<Vec<u8>> {
        Ok(self.lua.globals().get::<_, LuaFunction>("lua_bytecode")?.call::<_, LuaString>((
            self.lua.create_string(code)?,
            format
        ))?.as_bytes().to_vec())
    }

    pub fn compile(&self, code: &str, name: &str) -> LuaResult<Vec<u8>> {
        Ok(self.lua.globals().get::<_, LuaFunction>("lua_bytecode")?.call::<_, LuaString>((
            self.lua.create_string(self.lua.load(code).set_name(name).into_function()?.dump(false))?,
            "L4404" // "B4404" for xbox?
        ))?.as_bytes().to_vec())
    }

    pub fn decomp(&self, code: &[u8], unluac: String) -> LuaResult<String> {
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(code).unwrap();
        let path = temp_file.path();
        let output = std::process::Command::new("java").args(&["-jar", &unluac, path.to_str().unwrap()]).output()?;
        Ok(String::from_utf8(output.stdout).unwrap())
    }
}

struct ScriptManager {
    script_fns: Arc<HashMap<Crc, Vec<u8>>>,
    loaded_scripts: Mutex<HashSet<Crc>>
}

impl LuaUserData for ScriptManager {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("import", |lua, this, val: String| {
            let val = Crc::from_string(&val);
            if this.loaded_scripts.lock().unwrap().insert(val.clone()) {
                if let Some(val) = this.script_fns.get(&val) {
                    lua.load(val).exec().unwrap();
                }
            }
            Ok(())
        });
    }
}

pub fn load_anim(script_fns: Arc<HashMap<Crc, Vec<u8>>>, anim: String) -> Map<String, Value> {
    let lua: Lua = Lua::new();
    let script_manager = ScriptManager { script_fns, loaded_scripts: Mutex::new(HashSet::new()) };

    lua.globals().set("importer", script_manager).unwrap();
    lua.globals().set("import", lua.load("function (a) importer:import(a) end").eval::<LuaFunction>().unwrap()).unwrap();
    lua.globals().set("inherit", lua.load("function (a) importer:import(a) end").eval::<LuaFunction>().unwrap()).unwrap();
    let table = lua.create_table().unwrap();
    table.set("Assert", lua.load("function (a,b) end").eval::<LuaFunction>().unwrap()).unwrap();
    table.set("GetRandomNumber", lua.load("function () return 1 end").eval::<LuaFunction>().unwrap()).unwrap();
    lua.globals().set("MgScript", table).unwrap();
    lua.globals().set("DeepCopy", lua.load("function (a) return a end").eval::<LuaFunction>().unwrap()).unwrap();
    lua.globals().set("AppendTableIndex", lua.load("function (t1, t2) for key, val in pairs(t2) do t1[key] = val end end").eval::<LuaFunction>().unwrap()).unwrap();
    lua.globals().set("AppendTable", lua.load("function (t1, t2) table.insert(t1, t2) end").eval::<LuaFunction>().unwrap()).unwrap();
    let table = lua.create_table().unwrap();
    table.set("GetRootSpeed", lua.load("function (a) end").eval::<LuaFunction>().unwrap()).unwrap();
    lua.globals().set("MgAnim", table).unwrap();

    lua.load(format!("import(\"{}\")", anim)).exec().unwrap();
    let tables = if lua.globals().contains_key("AnimTableUsed").unwrap() {
        lua.globals().get::<_, LuaTable>("AnimTableUsed").unwrap().pairs::<LuaValue, String>().filter_map(|x| x.ok()).map(|(_, x)| x).collect()
    } else {
        vec!["AnimTable".to_string()]
    };
    tables.into_iter().flat_map(|k| lua.globals().get::<_, LuaTable>(k).unwrap().pairs::<String, LuaValue>().filter_map(|x| x.ok())).filter_map(|(k, v)| match v {
        LuaValue::String(val) => Some((k, json!(val.to_str().unwrap()))),
        LuaValue::Table(t) => Some((k, json!(t.pairs::<LuaValue, String>().filter_map(|x| x.ok()).map(|(_, x)| x).collect::<Vec<_>>()))),
        _ => None,
    }).collect::<Map<_,_>>()
}
