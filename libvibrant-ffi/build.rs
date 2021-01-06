extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR env var is not defined");
    let ffi_dir = PathBuf::from(&crate_dir).join("ffi");

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file(ffi_dir.join("vibrant.h"));
}
