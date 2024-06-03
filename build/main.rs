use std::{env, fs, path::Path};

mod structure;
mod parser;
mod generate;

use parser::*;
use generate::generate;

macro_rules! feature {
    ($config:expr, $feat:literal, $param:literal) => {
        #[cfg(feature = $feat)] $config.define($param, "ON");
        #[cfg(not(feature = $feat))] $config.define($param, "OFF");
    };
}

fn main() {
    let mut config = cmake::Config::new("raylib");

    config.define("WITH_PIC", "ON")
        .define("BUILD_EXAMPLES", "OFF")
        .define("CUSTOMIZE_BUILD", "ON");

    feature!(config, "wayland", "USE_WAYLAND");
    feature!(config, "png", "SUPPORT_FILEFORMAT_PNG");
    feature!(config, "dds", "SUPPORT_FILEFORMAT_DDS");
    feature!(config, "hdr", "SUPPORT_FILEFORMAT_HDR");
    feature!(config, "ktx", "SUPPORT_FILEFORMAT_KTX");
    feature!(config, "astc", "SUPPORT_FILEFORMAT_ASTC");
    feature!(config, "bmp", "SUPPORT_FILEFORMAT_BMP");
    feature!(config, "tga", "SUPPORT_FILEFORMAT_TGA");
    feature!(config, "jpg", "SUPPORT_FILEFORMAT_JPG");
    feature!(config, "gif", "SUPPORT_FILEFORMAT_GIF");
    feature!(config, "psd", "SUPPORT_FILEFORMAT_PSD");
    feature!(config, "pkm", "SUPPORT_FILEFORMAT_PKM");
    feature!(config, "pvr", "SUPPORT_FILEFORMAT_PVR");
    feature!(config, "obj", "SUPPORT_FILEFORMAT_OBJ");
    feature!(config, "mtl", "SUPPORT_FILEFORMAT_MTL");
    feature!(config, "wav", "SUPPORT_FILEFORMAT_WAV");
    feature!(config, "ogg", "SUPPORT_FILEFORMAT_OGG");
    feature!(config, "xm", "SUPPORT_FILEFORMAT_XM");
    feature!(config, "mod", "SUPPORT_FILEFORMAT_MOD");
    feature!(config, "flac", "SUPPORT_FILEFORMAT_FLAC");
    feature!(config, "save_png", "SUPPORT_SAVE_PNG");
    feature!(config, "save_bmp", "SUPPORT_SAVE_BMP");

    let path = config.build();

    println!("cargo:rustc-link-search=native={}/lib", path.display());
    println!("cargo:rustc-link-lib=static=raylib");

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
