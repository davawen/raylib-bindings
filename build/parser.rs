use crate::structure::*;

use serde::{de::Visitor, Deserialize, Deserializer};

struct TypeVisitor;

impl<'de> Visitor<'de> for TypeVisitor {
    type Value = Type<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a C data type")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(parse_type(v))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Type<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        deserializer.deserialize_str(TypeVisitor)
    }
}

fn parse_type(ty: &str) -> Type {
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
