use std::{ iter::Peekable, str::Lines };
use crate::structure::*;

type Stream<'a> = Peekable<Lines<'a>>;

trait NextUnwrap {
    type Item;
    fn unwrapped(&mut self) -> Self::Item;
}

impl<T: Iterator<Item = U>, U> NextUnwrap for T {
    type Item = U;
    fn unwrapped(&mut self) -> Self::Item {
        self.next().unwrap()
    }
}

fn parse_define<'a>(stream: &mut Stream<'a>) -> Define<'a> {
    let _ = stream.unwrapped();
    let name = stream.unwrapped().split_once(':').unwrap().1.trim();
    let kind = stream.unwrapped().split_once(':').unwrap().1.trim();
    let value = stream.unwrapped().split_once(':').unwrap().1.trim();
    let desc = stream.unwrapped().split_once(':').unwrap().1.trim();
    Define { name, kind, value, desc }
}

fn parse_type(ty: &str) -> Type<'_> {
    #[derive(Debug)]
    enum Token<'a> {
        Word(&'a str),
        Unsigned,
        Signed,
        Const,
        Star,
        TripleDot,
        Array(i32)
    }
 
    fn is_identifier(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    let mut tokens = vec![];
    let mut last_word = None;
    let mut chars = ty.char_indices().peekable();
    while let Some((idx, c)) = chars.next() {
        if last_word.is_none() && is_identifier(c) {
            last_word = Some(idx);
        }

        if is_identifier(c) && !chars.peek().is_some_and(|(_, c)| is_identifier(*c)) {
            let token = match &ty[last_word.unwrap()..=idx] {
                "unsigned" => Token::Unsigned,
                "signed" => Token::Signed,
                "const" => Token::Const,
                word => Token::Word(word)
            };

            tokens.push(token);
            last_word = None;
        } else if c == '*' {
            tokens.push(Token::Star)
        } else if c == '[' {
            let end_idx = loop {
                if let (idx, ']') = chars.next().unwrap() {
                    break idx;
                }
            };

            let num = ty[idx+1..end_idx].parse().unwrap();
            tokens.push(Token::Array(num));
        } else if c == '.' && idx+3 <= ty.len() && &ty[idx..idx+3] == "..." {
            tokens.push(Token::TripleDot);
            chars.next();
            chars.next();
        }
    }

    fn parse_tokenized<'a>(tokens: &[Token<'a>]) -> Type<'a> {
        assert!(!tokens.is_empty());

        match tokens.last().unwrap() {
            Token::Star => {
                let constant = matches!(tokens.first().unwrap(), Token::Const);
                let to = if constant {
                    parse_tokenized(&tokens[1..tokens.len()-1])
                } else {
                    parse_tokenized(&tokens[..tokens.len()-1])
                };

                Type::Ptr { to: Box::new(to), constant }
            }
            Token::Array(num) => {
                Type::Array(Box::new(parse_tokenized(&tokens[..tokens.len()-1])), *num)
            }
            _ => match *tokens {
                [Token::Signed, Token::Word(word)] => Type::Name {
                    value: word,
                    qualifier: Some("signed")
                },
                [Token::Unsigned, Token::Word(word)] => Type::Name {
                    value: word,
                    qualifier: Some("unsigned")
                },
                [Token::Word(word)] => Type::Name { value: word, qualifier: None },
                [Token::TripleDot] => Type::Variadic,
                _ => panic!("unknown type: {tokens:?}")
            },
        }
    }

    parse_tokenized(&tokens)
}

fn parse_struct<'a>(stream: &mut Stream<'a>) -> Struct<'a> {
    let num_fields: usize = stream.unwrapped()
        .split_once('(').unwrap()
        .1.split_once(' ').unwrap()
        .0.parse().unwrap();

    let name = stream.unwrapped().split_once(':').unwrap().1.trim();
    let desc = stream.unwrapped().split_once(':').unwrap().1.trim();
    let fields = (0..num_fields)
        .map(|_| stream.unwrapped())
        .map(|line| {
            let line = line.split_once(':').unwrap().1.trim();
            let (ty_name, desc) = line.split_once("//").unwrap();
            let (ty, name) = ty_name.trim().rsplit_once(' ').unwrap();
            Field { name: name.trim(), ty: parse_type(ty.trim()), desc: desc.trim() }
        })
        .collect();

    Struct { name, desc, fields }
}

fn parse_alias<'a>(stream: &mut Stream<'a>) -> Alias<'a> {
    let _ = stream.unwrapped();
    let ty = stream.unwrapped().split_once(':').unwrap().1.trim();
    let name = stream.unwrapped().split_once(':').unwrap().1.trim();
    let desc = stream.unwrapped().split_once(':').unwrap().1.trim();
    Alias { name, ty, desc }
}

fn snake_to_pascal(snake: &str) -> String {
    snake.split('_').map(|word| {
        let mut chars = word.chars();
        if let Some(c) = chars.next() {
            c.to_uppercase()
                .chain(chars.as_str().to_lowercase().chars())
                .collect()
        } else { String::new() }
    }).collect()
}

fn parse_enum<'a>(stream: &mut Stream<'a>) -> Enum<'a> {
    let num_values: usize = stream.unwrapped()
        .split_once('(').unwrap()
        .1.split_once(' ').unwrap()
        .0.parse().unwrap();

    let name = stream.unwrapped().split_once(':').unwrap().1.trim();
    let desc = stream.unwrapped().split_once(':').unwrap().1.trim();
    let values = (0..num_values)
        .map(|_| stream.unwrapped())
        .map(|line| {
            // convert enum variant to pascal case
            let variant = &line[line.find('[').unwrap()+1..line.find(']').unwrap()];
            let mut variant = snake_to_pascal(variant);

            // remove common prefix between enum name and enum variant
            let idx = variant.char_indices().zip(name.chars())
                .take_while(|((_, x), y)| x == y)
                .map(|((idx, _), _)| idx)
                .last()
                .unwrap_or(0);

            if idx > 0 {
                variant.drain(..=idx);
            }

            // get discriminant
            let value: i32 = line.split_once(':').unwrap().1.trim().parse().unwrap();
            
            (variant, value)
        })
        .collect();

    Enum { name, desc, values }
}

fn parse_param(line: &str) -> Param<'_> {
    let line = line.split_once(':').unwrap().1.trim();
    let (name, lparen) = line.split_once('(').unwrap();
    let name = name.trim();
    let ty = lparen.split_once(':').unwrap().1.split_once(')').unwrap().0.trim();
    Param { name, ty: parse_type(ty) }
}

fn parse_function<'a>(stream: &mut Stream<'a>) -> Function<'a> {
    let num_params: usize = stream.unwrapped() 
        .rsplit_once('(').unwrap().1
        .split_once(' ').unwrap().0
        .parse().unwrap();

    let name = stream.unwrapped().split_once(':').unwrap().1.trim();
    let ret = stream.unwrapped().split_once(':').unwrap().1.trim();
    let desc = stream.unwrapped().split_once(':').unwrap().1.trim();
    let params = (0..num_params)
        .map(|_| stream.unwrapped())
        .map(parse_param).collect();
    if num_params == 0 {
        let _ = stream.unwrapped();
    }

    Function { name, ret: parse_type(ret), desc, params }
}

fn parse_callback<'a>(stream: &mut Stream<'a>) -> Callback<'a> {
    let func = parse_function(stream);
    Callback { name: func.name, desc: func.desc, ret: func.ret, params: func.params }
}

fn parse_multiple<'a, T>(stream: &mut Stream<'a>, parse: impl Fn(&mut Stream<'a>) -> T) -> Vec<T> {
    let _ = stream.unwrapped();
    let num: usize = stream.unwrapped().split_once(':').unwrap().1.trim().parse().unwrap();
    let _ = stream.unwrapped();

    (0..num).map(|_| parse(stream)).collect()
}

pub fn parse_raylib(file: &str) -> Raylib<'_> {
    let mut stream = file.lines().peekable();

    let defines = parse_multiple(&mut stream, parse_define);
    let structs = parse_multiple(&mut stream, parse_struct);
    let aliases = parse_multiple(&mut stream, parse_alias);
    let enums = parse_multiple(&mut stream, parse_enum);
    let callbacks = parse_multiple(&mut stream, parse_callback);
    let functions = parse_multiple(&mut stream, parse_function);

    Raylib {
        defines, structs, aliases, enums, callbacks, functions
    }
}
