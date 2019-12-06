extern crate wasm_std as ext;
use std::str;

fn main() {
    let sparkle_heart = vec![240, 159, 146, 150];

    // We know these bytes are valid, so just use `unwrap()`.
    let sparkle_heart = str::from_utf8(&sparkle_heart).unwrap();

    assert_eq!("💖", sparkle_heart);
    println!("{}", sparkle_heart)
}
