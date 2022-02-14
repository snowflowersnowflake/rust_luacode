#![no_std]

mod lua;
use lua::{
    highlevel::Lua
};
use core::str;

// use core::arch::asm;
use ckb_std::{
    default_alloc,
};

default_alloc!();

fn main() {
    // 创建一个lua状态机
    let lua = Lua::new(0, 0);
    // const CAPACITY:u64 = 50;

    // let path = String::from("./src/test.lua");
    let code = "print('u can print everything on here')";
    Lua::run(&lua, code);
    // Lua::boost(&lua, path);
    // let path = Path::new().extension().unwrap();
    // assert!( path == "lua");
    // let vec = Lua::run(&lua, lua_path).get(0);
    // Load error handler for contract error print
    // lua.context(|lua_ctx| {
    //     let globals = lua_ctx.globals();
    //     globals.set("string_var", "hello")?;
    //     globals.set("int_var", 42)?;
    //     Ok(())
    // })?;

}