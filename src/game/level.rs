use std::path::Path;

use mlua::prelude::*;

#[test]
fn test_lua() -> Result<(), LuaError> {
    let lua = Lua::new();
    let globals = lua.globals();
    
    // set the level table
    let level = lua.create_table()?;
    globals.set("level", level)?;
    
    // load and run the code
    let code = std::fs::read_to_string(Path::new("data/levels/start.lua"))?;
    lua.load(&code).exec()?;
    
    // test that it loaded properly
    let level: mlua::Table = lua.globals().get("level")?;
    
    let name: String = level.get("name")?;
    assert_eq!(name, "Start");
    
    let func: mlua::Function = level.get("test")?;
    let ret: bool = func.call(true)?;
    assert_eq!(ret, true);
    
    Ok(())
}