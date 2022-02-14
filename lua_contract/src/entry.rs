// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{vec, vec::Vec};

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    debug,
    high_level::{load_script, load_tx_hash},
    ckb_types::{bytes::Bytes, prelude::*},
};



use core::str;
use crate::error::Error;
use crate::lua::highlevel::Lua;

pub fn main() -> Result<(), Error> {
    // remove below examples and write your code here
    // 创建一个lua状态机
    let lua = Lua::new(0, 0);
    let code = "print('u can print everything on here')";
    Lua::run(&lua, code);
    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    debug!("script args is {:?}", args);

    // return an error if args is invalid
    if args.is_empty() {
        return Err(Error::MyError);
    }

    let tx_hash = load_tx_hash()?;
    debug!("tx hash is {:?}", tx_hash);

    // let _buf: Vec<_> = vec![0u8; 32];

    Ok(())
}

