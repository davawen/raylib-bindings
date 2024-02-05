use std::{env, fs, path::Path};

mod structure;
mod parser;
mod generate;

use parser::*;
use generate::generate;

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/generate.rs");
    println!("cargo:rerun-if-changed=build/parser.rs");
    println!("cargo:rerun-if-changed=build/structure.rs");

    let file = fs::read_to_string("parser/raylib_api.txt").unwrap();

    let raylib = parse_raylib(&file);
    
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("ffi.rs");
    let mut ffi = fs::File::create(dest_path).unwrap();
    generate(&mut ffi, raylib).unwrap();
}
