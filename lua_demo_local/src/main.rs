#![no_std]
#![no_main]
#![feature(asm_sym)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

// define modules
mod entry;
mod error;
mod lua;


use core::arch::asm;
use ckb_std::{
    default_alloc,
};

ckb_std::entry!(program_entry);
default_alloc!();

/// program entry
///
///  Both `argc` and `argv` can be omitted.
fn program_entry(_argc: i64, _argv: *const *const i8) -> i8 {
    // Call main function and return error code
    match entry::main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}

