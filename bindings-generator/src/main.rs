use std::{fs, error::Error, io::Write};

mod structure;
mod parser;
mod generate;

fn main() -> Result<(), Box<dyn Error>> {
    generate_api("parser/raylib_api.json", &mut fs::File::create("../src/ffi.rs")?)?;

    let mut rlgl = fs::File::create("../src/rlgl.rs")?;
    generate_api("parser/rlgl_api.json", &mut rlgl)?;
    writeln!(rlgl, "use crate::prelude::Matrix;")?;

    Ok(())
}

fn generate_api(input: &str, output: &mut fs::File) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(input)?;
    let api = serde_json::from_str(&file).expect("raylib_api.json to be valid");

    writeln!(output, "#![allow(non_snake_case, non_camel_case_types, unused)]")?;
    writeln!(output, "use std::ffi;\n")?;
    generate::generate(output, api)?;

    Ok(())
}
