macro_rules! feature {
    ($feat:literal) => {
        if cfg!(feature = $feat) { "ON" } else { "OFF" }
    };
}

#[cfg(target_feature = "crt-static")]
compile_error!(r#"ERROR (raylib-bindings): Cannot statically link C runtime with executable!
GLFW will fail to detect platform info if the C standard library is linked statically.
If you are on musl (or on another platform that links the C runtime statically by default), try compiling like this:

RUSTFLAGS='-C target-feature=-crt-static' cargo build"#);

fn main() {
    let mut config = cmake::Config::new("raylib");

    let path = config
        .define("BUILD_EXAMPLES", "OFF")
        .define("CUSTOMIZE_BUILD", "ON")
        .define("SUPPORT_FILEFORMAT_FNT", "OFF") // builtin text
        .define("SUPPORT_FILEFORMAT_TTF", "OFF") // builtin text
        .define("SUPPORT_IMAGE_EXPORT", feature!("image_export"))
        .define("SUPPORT_IMAGE_GENERATION", feature!("image_generation"))
        .define("SUPPORT_IMAGE_MANIPULATION", feature!("image_manipulation"))
        .define("SUPPORT_FILEFORMAT_PNG", feature!("png"))
        .define("SUPPORT_FILEFORMAT_DDS", feature!("dds"))
        .define("SUPPORT_FILEFORMAT_HDR", feature!("hdr"))
        .define("SUPPORT_FILEFORMAT_PIC", feature!("pic"))
        .define("SUPPORT_FILEFORMAT_PNM", feature!("pnm"))
        .define("SUPPORT_FILEFORMAT_KTX", feature!("ktx"))
        .define("SUPPORT_FILEFORMAT_ASTC", feature!("astc"))
        .define("SUPPORT_FILEFORMAT_BMP", feature!("bmp"))
        .define("SUPPORT_FILEFORMAT_TGA", feature!("tga"))
        .define("SUPPORT_FILEFORMAT_JPG", feature!("jpg"))
        .define("SUPPORT_FILEFORMAT_GIF", feature!("gif"))
        .define("SUPPORT_FILEFORMAT_QOI", feature!("qoi"))
        .define("SUPPORT_FILEFORMAT_PSD", feature!("psd"))
        .define("SUPPORT_FILEFORMAT_PKM", feature!("pkm"))
        .define("SUPPORT_FILEFORMAT_PVR", feature!("pvr"))
        .define("SUPPORT_FILEFORMAT_SVG", feature!("svg"))
        .define("SUPPORT_FILEFORMAT_OBJ", feature!("obj"))
        .define("SUPPORT_FILEFORMAT_MTL", feature!("mtl"))
        .define("SUPPORT_FILEFORMAT_IQM", feature!("iqm"))
        .define("SUPPORT_FILEFORMAT_GLTF", feature!("gltf"))
        .define("SUPPORT_FILEFORMAT_VOX", feature!("vox"))
        .define("SUPPORT_FILEFORMAT_M3D", feature!("m3d"))
        .define("SUPPORT_FILEFORMAT_WAV", feature!("wav"))
        .define("SUPPORT_FILEFORMAT_OGG", feature!("ogg"))
        .define("SUPPORT_FILEFORMAT_XM", feature!("xm"))
        .define("SUPPORT_FILEFORMAT_MOD", feature!("mod"))
        .define("SUPPORT_FILEFORMAT_MP3", feature!("mp3"))
        .define("SUPPORT_FILEFORMAT_QOA", feature!("qoa"))
        .define("SUPPORT_FILEFORMAT_FLAC", feature!("flac"))
        .build();

    println!("cargo:rustc-link-search=native={}/lib", path.display());
    println!("cargo:rustc-link-lib=static=raylib");

    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/generate.rs");
    println!("cargo:rerun-if-changed=build/parser.rs");
    println!("cargo:rerun-if-changed=build/structure.rs");
}
