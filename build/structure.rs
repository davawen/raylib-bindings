#[derive(Debug)]
pub struct Define<'a> {
    pub name: &'a str,
    pub kind: &'a str,
    pub value: &'a str,
    pub desc: &'a str
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

#[derive(Debug)]
pub struct Field<'a> {
    pub name: &'a str,
    pub ty: Type<'a>,
    pub desc: &'a str
}

#[derive(Debug)]
pub struct Struct<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub fields: Vec<Field<'a>>
}

#[derive(Debug)]
pub struct Alias<'a> {
    pub name: &'a str,
    pub ty: &'a str,
    pub desc: &'a str
}

#[derive(Debug)]
pub struct Enum<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub values: Vec<(&'a str, i32)>
}

#[derive(Debug)]
pub struct Param<'a> {
    pub name: &'a str,
    pub ty: Type<'a>
}

#[derive(Debug)]
pub struct Callback<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub ret: Type<'a>,
    pub params: Vec<Param<'a>>
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub ret: Type<'a>,
    pub params: Vec<Param<'a>>
}

#[derive(Debug)]
pub struct Raylib<'a> {
    pub defines: Vec<Define<'a>>,
    pub structs: Vec<Struct<'a>>,
    pub aliases: Vec<Alias<'a>>,
    pub enums: Vec<Enum<'a>>,
    pub callbacks: Vec<Callback<'a>>,
    pub functions: Vec<Function<'a>>
}
