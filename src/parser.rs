use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
enum Token {
    Element(String),
    Coefficient(usize),
    Subscript(usize),
    Equals,
    Plus
}

#[derive(Debug)]
enum LexToken {
    Upper(char),
    Lower(char),
    Number(usize),
    Plus,
    Equals
}


#[derive(Debug)]
pub struct Compound {
    coefficient: usize,
    pub(crate) elements: HashMap<String, usize>
}

impl Compound {
    fn from_slice(v: &[Token]) -> Self {
        let mut coefficient: usize = 1;
        let mut elements = HashMap::new();
        let mut iter = v.into_iter().enumerate().peekable();
        match iter.peek().expect("zero length slice cannot be parsed to compound").1 {
            Token::Coefficient(coef) => {
                coefficient = *coef;
                iter.next();
            },
            _ => ()
        }
        let mut last = 0usize;
        while let Some((i, token)) = iter.next() {
            if let Token::Element(elem) = token {
                match iter.peek() {
                    Some((_, Token::Subscript(sub))) => {
                        elements.insert(elem.clone(), *sub);
                        iter.next();
                        last = i;
                    },
                    _ => {
                        elements.insert(elem.clone(), 1);
                        last = i;
                    }
                }

            }
        }
        Self {
            coefficient,
            elements,
        }
    }
}

impl Display for Compound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.coefficient != 1 {
            write!(f, "{}", self.coefficient)?;
        }
        for elem in &self.elements {
            write!(f, "{}{}", elem.1, elem.0)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Equation {
    pub(crate) lhs: Vec<Compound>,
    pub(crate) rhs: Vec<Compound>,
}

impl Equation {
    fn from_vec(mut v: Vec<Token>) -> Self {
        let mut rhs = false;
        let mut lhs_slices = Vec::new();
        let mut rhs_slices = Vec::new();
        let mut last = 0usize;
        v.push(Token::Plus);
        for (i, token) in v.iter().enumerate() {
            match token {
                Token::Plus => {
                    if rhs {
                        rhs_slices.push(&v[last..i])
                    }
                    else {
                        lhs_slices.push(&v[last..i]);
                    }
                    last = i+1;
                }
                Token::Equals => {
                    lhs_slices.push(&v[last..i]);
                    rhs = true;
                    last = i+1;
                },
                _ => continue
            }
        }
        Equation {
            lhs: lhs_slices.into_iter().map(|slice| Compound::from_slice(slice)).collect(),
            rhs: rhs_slices.into_iter().map(|slice| Compound::from_slice(slice)).collect()
        }
    }
}

impl From<String> for Equation {
    fn from(value: String) -> Self {
        Equation::from_vec(parse(lex(&value).unwrap()).unwrap())
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut lhs = self.lhs.iter().peekable();
        while let Some(cpd) = lhs.next(){
            write!(f, "{}", cpd)?;
            if let Some(_) = lhs.peek() {
                write!(f, "+")?;
            }
        }
        write!(f, "=")?;
        let mut rhs = self.rhs.iter().peekable();
        while let Some(cpd) = rhs.next(){
            write!(f, "{}", cpd)?;
            if let Some(_) = rhs.peek() {
                write!(f, "+")?;
            }
        }
        Ok(())
    }
}


fn lex(input: &String) -> Result<Vec<LexToken>, String> {
    let mut result = Vec::new();

    let mut rename = input.chars().peekable();
    while let Some(char) = rename.next() {
        match char {
            'A'..='Z' => (
                result.push(LexToken::Upper(char))
            ),
            'a'..='z' => (
                result.push(LexToken::Lower(char))
            ),
            '0'..='9' => (
                match rename.peek() {
                    Some('0'..='9') => {
                        let mut num_str = String::new();
                        num_str.push(char);
                        num_str.push(rename.next().unwrap());
                        result.push(LexToken::Number(num_str.parse::<usize>().map_err(|e| "error parsing number to string".to_string())?))
                    },
                    _ => (
                        result.push(LexToken::Number(char.to_string().parse::<usize>().map_err(|e| "error parsing number to string".to_string())?))
                        ),
                }
                ),
            '+' => {
                result.push(LexToken::Plus);
            },
            '=' | '→' => {
                result.push(LexToken::Equals);
            },
            '\n' | '\r' => (),
            ' ' => (),
            other => return Err(format!("unrecognized symbol '{}' during lexing", other))
        }
    }
    Ok(result)
}

fn parse(mut input: Vec<LexToken>) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();

    let mut iter = input.into_iter().peekable();
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
                result.push(Token::Element(element));
            }
            LexToken::Lower(_) => return Err("unexpected lower case token in parse stream".to_string()),
            LexToken::Number(num) => {
                if let Some(Token::Element(_)) = result.last() {
                    result.push(Token::Subscript(num));
                }
                else {
                    result.push(Token::Coefficient(num));
                }
            },
            LexToken::Plus => result.push(Token::Plus),
            LexToken::Equals => result.push(Token::Equals)
        }
    }
    Ok(result)
}