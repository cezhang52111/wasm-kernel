#![no_std]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(global_allocator)]
#![feature(alloc)]


extern crate wasm_std as ext;
use serde::{Serialize, Deserialize};

#[no_mangle]
pub fn test_print_args() {
    let args = ext::args();
    ext::debug(b"args!");
    ext::debug(&args);
}

#[no_mangle]
pub fn test_return_args() {
    let args = ext::args();
    ext::return_data(&args);
}

#[no_mangle]
pub fn test_set_storage() {
    let args = ext::args();
    ext::set_storage(&args, b"you found me!");
}

#[no_mangle]
pub fn test_get_storage() {
    let args = ext::args();
    let value = ext::get_storage(&args);
    ext::return_data(&value);
}

