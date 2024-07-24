use std::borrow::Cow;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Define<'a> {
    pub name: &'a str,
    #[serde(rename = "type")]
    pub kind: &'a str,
    pub value: serde_json::Value,
    pub description: &'a str
}

#[derive(Debug)]
pub enum Type<'a> {
    Name {
        value: &'a str,
        qualifier: Option<&'a str>
    },
    Ptr {
        to: Box<Type<'a>>,
        constant: bool
    },
    Array(Box<Type<'a>>, i32),
    Variadic
}

#[derive(Debug, Deserialize)]
pub struct Field<'a> {
    pub name: &'a str,
    #[serde(rename = "type")]
    pub ty: Type<'a>,
    pub description: &'a str
}

#[derive(Debug, Deserialize)]
pub struct Struct<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub fields: Vec<Field<'a>>
}

#[derive(Debug, Deserialize)]
pub struct Alias<'a> {
    pub name: &'a str,
    #[serde(rename = "type")]
    pub ty: &'a str,
    pub description: &'a str
}

#[derive(Debug, Deserialize)]
pub struct EnumVariant<'a> {
    pub name: String,
    pub value: i32,
    pub description: Cow<'a, str>
}

#[derive(Debug, Deserialize)]
pub struct Enum<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub values: Vec<EnumVariant<'a>>
}

#[derive(Debug, Deserialize)]
pub struct Param<'a> {
    pub name: &'a str,
    #[serde(rename = "type")]
    pub ty: Type<'a>
}

#[derive(Debug, Deserialize)]
pub struct Callback<'a> {
    pub name: &'a str,
    pub description: &'a str,
    #[serde(rename = "returnType")]
    pub ret: Type<'a>,
    pub params: Vec<Param<'a>>
}

#[derive(Debug, Deserialize)]
pub struct Function<'a> {
    pub name: &'a str,
    pub description: Cow<'a, str>,
    #[serde(rename = "returnType")]
    pub ret: Type<'a>,
    #[serde(default)]
    pub params: Vec<Param<'a>>
}

#[derive(Debug, Deserialize)]
pub struct Raylib<'a> {
    #[serde(borrow)] 
    pub defines: Vec<Define<'a>>,
    pub structs: Vec<Struct<'a>>,
    pub aliases: Vec<Alias<'a>>,
    pub enums: Vec<Enum<'a>>,
    pub callbacks: Vec<Callback<'a>>,
    pub functions: Vec<Function<'a>>
}
