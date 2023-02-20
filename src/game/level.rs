use crate::engine::marble::MarbleType;
use crate::*;

use std::{ fs, cell::UnsafeCell };

use mlua::{ Function, Value, Table, ToLua, FromLua, prelude::{ LuaResult, LuaValue, LuaError } };

impl<'lua> ToLua<'lua> for MarbleType {
    fn to_lua(self, lua: &'lua mlua::Lua) -> LuaResult<LuaValue<'lua>> {
        let s = match self {
            MarbleType::Bit => "bit",
            MarbleType::Num => "num",
        };
        Ok(Value::String(lua.create_string(s)?))
    }
}

impl<'lua> FromLua<'lua> for MarbleType {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua mlua::Lua) -> LuaResult<Self> {
        let s: String = String::from_lua(lua_value, lua)?;
        match s.as_str() {
            "bit" => Ok(MarbleType::Bit),
            "num" => Ok(MarbleType::Num),
            _ =>
                Err(LuaError::FromLuaConversionError {
                    from: "String",
                    to: "MarbleType",
                    message: Some("invalid string".to_string()),
                }),
        }
    }
}

pub struct Level<'lua> {
    name: String,
    inputs: Vec<MarbleType>,
    outputs: Vec<MarbleType>,
    test: mlua::Function<'lua>,
    generate: mlua::Function<'lua>,
}

impl<'lua> Level<'lua> {
    fn new(lua: &'lua mlua::Lua) -> Self {
        let func = lua.globals().get::<_, Function>("error").unwrap();
        Self {
            name: "Default".to_string(),
            inputs: vec![],
            outputs: vec![],
            test: func.clone(),
            generate: func,
        }
    }
}

impl<'lua> ToLua<'lua> for Level<'lua> {
    fn to_lua(self, lua: &'lua mlua::Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("name", self.name)?;
        table.set("inputs", self.inputs)?;
        table.set("outputs", self.outputs)?;
        table.set("test", self.test)?;
        table.set("generate", self.generate)?;
        Ok(Value::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Level<'lua> {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua mlua::Lua) -> LuaResult<Self> {
        let table: Table = Table::from_lua(lua_value, lua)?;
        let name: String = table.get("name")?;
        let inputs: Vec<MarbleType> = table.get("inputs")?;
        let outputs: Vec<MarbleType> = table.get("outputs")?;
        let test: Function = table.get("test")?;
        let generate: Function = table.get("generate")?;
        Ok(Level {
            name,
            inputs,
            outputs,
            test,
            generate,
        })
    }
}

impl<'lua> std::ops::Deref for Levels<'lua> {
    type Target = Vec<Level<'lua>>;
    fn deref(&self) -> &Self::Target {
        &self.levels
    }
}

impl<'lua> std::ops::DerefMut for Levels<'lua> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.levels
    }
}

#[derive(Resource, Deref)]
pub struct Lua {
    lua: mlua::Lua,
}

impl FromWorld for Lua {
    fn from_world(_: &mut World) -> Self {
        Self { lua: mlua::Lua::new() }
    }
}

#[cfg(debug_assertions)]
impl Drop for Lua {
    fn drop(&mut self) {
        info!("Exiting...");
        std::process::exit(0)
    }
}

#[derive(Default, Resource)]
pub struct Levels<'lua> {
    levels: Vec<Level<'lua>>,
}

/// Startup function that loads all the levels from the lua files
pub fn load_levels(world: &mut World) {
    info!("Loading levels...");

    unsafe {
        let world = world as *mut World;
        let result = load_levels_inner(world);
        match result {
            Ok(levels) => world.as_mut().expect("valid pointer").insert_non_send_resource(levels),
            Err(err) => log_errors(In(Err(err))),
        }
    }
}

unsafe fn load_levels_inner<'lua>(world: *mut World) -> Result<Levels<'lua>, LocatedError> {
    let mut levels = Levels::default();
    let lua = world
        .as_ref()
        .expect("valid pointer")
        .get_non_send_resource::<Lua>()
        .expect("Lua resource initialized");

    // for every file in the levels directory
    for path in fs::read_dir("data/levels")? {
        let path = path?.path();
        // if it's a lua file, load it and run it
        if path.extension().unwrap() == "lua" {
            lua.globals().set("level", Level::new(&*lua))?;

            let code = fs::read_to_string(path)?;
            lua.load(&code).exec()?;

            let level: Level = lua.globals().get("level")?;
            levels.push(level);
        }
    }

    Ok(levels)
}

#[test]
fn test_lua() -> Result<(), LuaError> {
    use std::path::Path;

    let lua = mlua::Lua::new();
    let globals = lua.globals();

    // set the level table
    globals.set("level", Level::new(&lua))?;

    // load and run the code
    let code = fs::read_to_string(Path::new("data/levels/start.lua"))?;
    lua.load(&code).exec()?;

    // test that it loaded properly
    let level: Level = lua.globals().get("level")?;

    assert_eq!(level.name, "Start");
    let ret: bool = level.test.call(true)?;
    assert_eq!(ret, true);

    Ok(())
}