use std::{io::{Write, self}, fmt::Display};

use crate::structure::*;

fn map_type<'a>(name: &'a str, qualifier: Option<&'a str>) -> &'a str {
    match (name, qualifier) {
        ("char" , Some("unsigned")) => "ffi::c_uchar",
        ("char" , Some("signed")) => "ffi::c_schar",
        ("char", None) => "ffi::c_char",
        ("short", Some("unsigned")) => "ffi::c_ushort",
        ("short", Some("signed")) => "ffi::c_sshort",
        ("short", None) => "ffi::c_short",
        ("int"  , Some("unsigned")) => "ffi::c_uint",
        ("int"  , Some("signed")) => "ffi::c_sint",
        ("int"  , None) => "ffi::c_int",
        ("long" , Some("unsigned")) => "ffi::c_ulong",
        ("long" , Some("signed")) => "ffi::c_slong",
        ("long" , None) => "ffi::c_long",
        (ty, Some(_)) => panic!("qualifier used on invalid type: {ty}"),
        ("float", _) => "ffi::c_float",
        ("double", _) => "ffi::c_double",
        ("void", _) => "ffi::c_void",
        ("va_list", _) => "va_list::VaList",
        (ty, _) => ty
    }
}

fn escape_name(name: &str) -> &str {
    match name {
        "type" => "r#type",
        "box" => "r#box",
        name => name
    }
}

impl Display for Type<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Name { value, qualifier } => write!(f, "{}", map_type(value, *qualifier)),
            Type::Ptr { to, constant } => {
                write!(f, "*")?;
                if *constant { write!(f, "const {to}") }
                else { write!(f, "mut {to}") }
            }
            Type::Array(ty, len) => write!(f, "[{ty}; {len}]"),
            Type::Variadic => write!(f, "...")
        }
    }
}

fn generate_structs(out: &mut impl Write, structs: Vec<Struct>) -> io::Result<()> {
    for s in structs {
        if s.name == "AudioStream" {
            writeln!(out, "type rAudioBuffer = ffi::c_void;")?;
            writeln!(out, "type rAudioProcessor = ffi::c_void;")?;
            writeln!(out)?;
        }

        if !s.desc.is_empty() {
            writeln!(out, "/// {}", s.desc)?;
        }

        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "pub struct {} {{", s.name)?;
        for field in s.fields {
            if !field.desc.is_empty() {
                writeln!(out, "    /// {}", field.desc)?;
            }

            writeln!(out, "    pub {}: {},", escape_name(field.name), field.ty)?;
        }
        writeln!(out, "}}")?;
    }

    writeln!(out)
}

fn generate_aliases(out: &mut impl Write, aliases: Vec<Alias>) -> io::Result<()> {
    for a in aliases {
        if !a.desc.is_empty() {
            writeln!(out, "/// {}", a.desc)?;
        }

        writeln!(out, "pub type {} = {};", a.name, a.ty)?;
    }

    writeln!(out)
}

fn generate_callbacks(out: &mut impl Write, callbacks: Vec<Callback>) -> io::Result<()> {
    for callback in callbacks {
        if !callback.desc.is_empty() {
            writeln!(out, "/// {}", callback.desc)?;
        }
        write!(out, "pub type {} = extern fn(", callback.name)?;
        for param in callback.params {
            write!(out, "{}, ", param.ty)?;
        }
        write!(out, ")")?;
        if !matches!(callback.ret, Type::Name { value: "void", qualifier: None }) {
            write!(out, " -> {}", callback.ret)?;
        }
        writeln!(out, ";")?;
    }

    writeln!(out)
}

fn generate_functions(out: &mut impl Write, functions: Vec<Function>) -> io::Result<()> {
    writeln!(out, "#[link(name = \"raylib\")]")?;
    writeln!(out, "extern \"C\" {{")?;

    for f in functions {
        if !f.desc.is_empty() {
            writeln!(out, "/// {}", f.desc)?;
        }

        write!(out, "pub fn {}(", f.name)?;
        for param in f.params {
            write!(out, "{}: {}, ", escape_name(param.name), param.ty)?;
        }
        write!(out, ")")?;
        if !matches!(f.ret, Type::Name { value: "void", qualifier: None }) {
            write!(out, " -> {}", f.ret)?;
        }
        writeln!(out, ";")?;
    }

    writeln!(out, "}}")
}

fn generate_enums(out: &mut impl Write, enums: Vec<Enum>) -> io::Result<()> {
    for e in enums {
        if !e.desc.is_empty() {
            writeln!(out, "/// {}", e.desc)?;
        }
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "pub enum {} {{", e.name)?;
        for (name, value) in e.values {
            writeln!(out, "    {} = {},", name, value)?;
        }
        writeln!(out, "}}")?;
    }

    Ok(())
}

fn generate_defines(out: &mut impl Write, defines: Vec<Define>) -> io::Result<()> {
    for define in defines {
        match define.kind {
            "INT" => writeln!(out, "pub const {}: i32 = {};", define.name, define.value)?,
            "STRING" => writeln!(out, "pub const {}: &str = {};", define.name, define.value)?,
            _ => ()
        }
    }

    Ok(())
}

pub fn generate(out: &mut impl Write, raylib: Raylib) -> io::Result<()> {
    generate_defines(out, raylib.defines)?;
    generate_structs(out, raylib.structs)?;
    generate_aliases(out, raylib.aliases)?;
    generate_callbacks(out, raylib.callbacks)?;
    generate_functions(out, raylib.functions)?;
    generate_enums(out, raylib.enums)?;

    Ok(())
}
