#[derive(Debug, Clone)]
pub enum Token {
    Element(String),
    Coefficient(usize),
    Subscript(usize),
    Equals,
    Plus
}

#[derive(Debug)]
pub enum LexToken {
    Upper(char),
    Lower(char),
    Number(usize),
    Plus,
    Equals
}

pub fn lex(input: &String) -> Result<Vec<LexToken>, String> {
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
            '\n' => (),
            ' ' => (),
            other => return Err(format!("unrecognized symbol '{}' during lexing", other))
        }
    }
    Ok(result)
}

pub fn parse(mut input: Vec<LexToken>) -> Result<Vec<Token>, String> {
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