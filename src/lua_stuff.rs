use std::io::Write;
use mlua::prelude::*;

const LUA_BYTECODE: &str = include_str!("../res/lua-bytecode.github.io/lua-bytecode.lua");

#[derive(Debug, Default)]
pub struct LuaCompiler {
    lua: Lua,
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

    pub fn decomp(&self, code: &[u8]) -> LuaResult<String> {
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(code).unwrap();
        let path = temp_file.path();
        let output = std::process::Command::new("java").args(&["-jar", "unluac.jar", path.to_str().unwrap()]).output()?;
        Ok(String::from_utf8(output.stdout).unwrap())
    }
}