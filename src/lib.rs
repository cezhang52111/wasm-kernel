
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
