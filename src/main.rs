mod parser;

use std::fmt::{Display, Formatter};
use parser::*;
use std::io::{stdin,Write};

#[derive(Debug)]
struct Element {
    symbol: String,
    subscript: usize
}

impl Element {
    fn new(symbol: String, subscript: usize) -> Self {
       Element {
           symbol,
           subscript
       }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)?;
        if self.subscript != 1 {
            write!(f, "{}", self.subscript)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Compound {
    coefficient: usize,
    elements: Vec<Element>
}

impl Compound {
    fn from_slice(v: &[Token]) -> Self {
        let mut coefficient: usize = 1;
        let mut elements = Vec::new();
        let mut iter = v.into_iter().enumerate().peekable();
        match iter.peek().unwrap().1 {
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
                        elements.push(Element::new(elem.clone(), *sub));
                        iter.next();
                        last = i;
                    },
                    _ => {
                        elements.push(Element::new(elem.clone(), 1));
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
            write!(f, "{}", elem)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Equation {
    lhs: Vec<Compound>,
    rhs: Vec<Compound>,
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

fn main() -> Result<(), String> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    println!("{}", Equation::from_vec(parse(lex(&input)?)?));
    Ok(())
}
