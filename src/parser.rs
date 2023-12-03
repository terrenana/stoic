use indexmap::IndexMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
enum Token {
    Element(String),
    Subscript(usize),
    Equals,
    Plus,
}

#[derive(Debug)]
enum LexToken {
    Upper(char),
    Lower(char),
    Number(usize),
    Plus,
    Equals,
}

#[derive(Debug, Clone)]
pub(crate) enum Side {
    LHS,
    RHS,
}

#[derive(Debug)]
pub(crate) struct ChemicalEquation {
    pub(crate) terms: Vec<Compound>,
}

impl ChemicalEquation {
    fn new(terms: Vec<Compound>) -> Self {
        ChemicalEquation { terms }
    }
}

impl Display for ChemicalEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.terms.iter().peekable();
        let mut rhs = false;
        while let Some(cpd) = iter.next() {
            write!(f, "{}", cpd)?;
            if let Some(cpd) = iter.peek() {
                if let Side::RHS = cpd.side {
                    if rhs {
                        write!(f, " + ")?;
                    } else {
                        write!(f, " = ")?;
                        rhs = true;
                    }
                } else {
                    write!(f, " + ")?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct Compound {
    pub(crate) coefficient: usize,
    pub(crate) elements: IndexMap<String, usize>,
    pub(crate) side: Side,
}

impl Compound {
    fn new(elements_: &[Token], side: Side) -> Self {
        let mut elements = IndexMap::new();
        let mut iter = elements_.into_iter().peekable();
        while let Some(token) = iter.next() {
            if let Token::Element(elem) = token {
                if let Some(Token::Subscript(sub)) = iter.peek() {
                    elements.insert(elem.clone(), *sub);
                    iter.next();
                } else {
                    elements.insert(elem.clone(), 1);
                }
            }
        }
        Self {
            coefficient: 1,
            elements,
            side,
        }
    }
}

impl Display for Compound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.coefficient != 1 {
            write!(f, "{}", self.coefficient)?;
        }
        for (elem, i) in &self.elements {
            match i {
                1 => write!(f, "{}", elem)?,
                _ => write!(f, "{}{}", elem, i)?,
            }
        }
        Ok(())
    }
}

fn lex(input: &str) -> Result<Vec<LexToken>, String> {
    let mut result = Vec::new();

    let mut rename = input.chars().peekable();
    while let Some(char) = rename.next() {
        match char {
            'A'..='Z' => result.push(LexToken::Upper(char)),
            'a'..='z' => result.push(LexToken::Lower(char)),
            '0'..='9' => match rename.peek() {
                Some('0'..='9') => {
                    let mut num_str = String::new();
                    num_str.push(char);
                    num_str.push(rename.next().unwrap());
                    result.push(LexToken::Number(
                        num_str
                            .parse::<usize>()
                            .map_err(|_| "error parsing number to string".to_string())?,
                    ))
                }
                _ => result.push(LexToken::Number(
                    char.to_string()
                        .parse::<usize>()
                        .map_err(|_| "error parsing number to string".to_string())?,
                )),
            },
            '+' => {
                result.push(LexToken::Plus);
            }
            '=' | '→' => {
                result.push(LexToken::Equals);
            }
            '\n' | '\r' => (),
            ' ' => (),
            other => return Err(format!("unrecognized symbol '{}' during lexing", other)),
        }
    }
    Ok(result)
}

pub(crate) fn parse(input: &str) -> Result<ChemicalEquation, String> {
    let lex_stream = lex(input)?;
    let mut token_stream = Vec::new();

    let mut iter = lex_stream.into_iter().peekable();
    while let Some(token) = iter.next() {
        match token {
            LexToken::Upper(char) => {
                let mut element = char.to_string();
                if let Some(token) = iter.peek() {
                    if let LexToken::Lower(c) = token {
                        element.push(*c);
                        iter.next();
                    }
                }
                token_stream.push(Token::Element(element));
            }
            LexToken::Lower(_) => {
                return Err("unexpected lower case token in parse stream".to_string())
            }
            LexToken::Number(num) => {
                if let Some(Token::Element(_)) = token_stream.last() {
                    token_stream.push(Token::Subscript(num));
                }
            }
            LexToken::Plus => token_stream.push(Token::Plus),
            LexToken::Equals => token_stream.push(Token::Equals),
        }
    }
    let mut compounds = Vec::new();
    let mut last = 0;
    let mut side = Side::LHS;
    for (i, token) in token_stream.clone().into_iter().enumerate() {
        if let Token::Plus | Token::Equals = token {
            compounds.push(Compound::new(&token_stream[last..i], side.clone()));
            last = i + 1;
            if let Token::Equals = token {
                side = Side::RHS;
            }
        }
    }
    compounds.push(Compound::new(&token_stream[last..token_stream.len()], side));
    Ok(ChemicalEquation::new(compounds))
}
