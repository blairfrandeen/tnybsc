use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Div,
    Mul,
    Assign,
    Equals,
    Ident(String),
    Int(i32),
    Float(f32),
    If,
    EndIf,
    While,
    EndWhile,
    Print,
    Invalid(String),
}

fn parse_ident(first: char, input: &mut Peekable<Chars>) -> Option<Token> {
    if first.is_alphabetic() {
        let mut matched = String::new();
        matched.push(first);
        while let Some(next) = input.next_if(|chr| chr.is_alphanumeric() || *chr == '_') {
            // dbg!(next);
            matched.push(next);
        }

        return Some(Token::Ident(matched));
    } else {
        return None;
    }
}

/// parses an integer or floating point number
fn parse_num(first: char, input: &mut Peekable<Chars>) -> Option<Token> {
    if first.is_digit(10) {
        let mut is_float = false;
        let mut digits = String::new();
        digits.push(first);
        while let Some(digit) = input.next_if(|c| (c.is_digit(10) | (*c == '.'))) {
            if digit == '.' {
                is_float = true;
            }
            digits.push(digit);
        }
        return match is_float {
            true => match digits.parse::<f32>() {
                Ok(flt) => Some(Token::Float(flt)),
                _ => Some(Token::Invalid(digits)),
            },
            false => match digits.parse::<i32>() {
                Ok(int) => Some(Token::Int(int)),
                _ => None,
            },
        };
    }
    None
}

#[test]
fn test_parse_num() {
    let input = "15";
    let mut chars = input.chars().peekable();
    assert_eq!(
        parse_num(chars.next().unwrap(), &mut chars),
        Some(Token::Int(15))
    );
    let input = "3.1415";
    let mut chars = input.chars().peekable();
    assert_eq!(
        parse_num(chars.next().unwrap(), &mut chars),
        Some(Token::Float(3.1415))
    );
}

fn parse_keyword(first: char, input: &mut Peekable<Chars>) -> Option<Token> {
    // All keywords are in ALL_CAPS
    if !first.is_uppercase() {
        return None;
    }
    let mut keyword = String::with_capacity(16);
    keyword.push(first);
    while let Some(next) = input.next_if(|chr| chr.is_uppercase()) {
        keyword.push(next)
    }

    match keyword.as_str() {
        "IF" => Some(Token::If),
        "ENDIF" => Some(Token::EndIf),
        "WHILE" => Some(Token::While),
        "ENDWHILE" => Some(Token::EndWhile),
        "PRINT" => Some(Token::Print),
        // Default case is we don't match a keyword. In that case we must have an identifier
        // that happens to be all caps
        _ => Some(Token::Ident(keyword)),
    }
}

#[test]
fn test_parse_keyword() {
    let input = "IF something == 0 ENDIF";
    let mut chars = input.chars().peekable();
    assert_eq!(
        parse_keyword(chars.next().unwrap(), &mut chars),
        Some(Token::If)
    );
}

pub fn parse_tokens(input: &str) -> Result<Vec<Token>, &str> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = input.chars().peekable();

    while let Some(next) = chars.next() {
        match next {
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Sub), // TODO: add case for negative number
            '*' => tokens.push(Token::Mul),
            '/' => tokens.push(Token::Div),
            '=' => match chars.peek() {
                Some('=') => {
                    tokens.push(Token::Equals);
                    chars.next();
                }
                _ => tokens.push(Token::Assign),
            },
            _ => {
                if let Some(token) = parse_keyword(next, &mut chars) {
                    tokens.push(token);
                } else if let Some(token) = parse_ident(next, &mut chars) {
                    tokens.push(token);
                } else if let Some(token) = parse_num(next, &mut chars) {
                    tokens.push(token);
                } else {
                    continue;
                }
            }
        }
    }

    Ok(tokens)
}

#[test]
fn test_parse_tokens() {
    assert_eq!(
        parse_tokens("+-=+=="),
        Ok(vec![
            Token::Add,
            Token::Sub,
            Token::Assign,
            Token::Add,
            Token::Equals
        ])
    );

    assert_eq!(
        parse_tokens("IF something == BANANA ENDIF grape"),
        Ok(vec![
            Token::If,
            Token::Ident("something".to_string()),
            Token::Equals,
            Token::Ident("BANANA".to_string()),
            Token::EndIf,
            Token::Ident("grape".to_string()),
        ])
    );

    assert_eq!(
        parse_tokens("WHILE something == 0 11.11 ENDWHILE"),
        Ok(vec![
            Token::While,
            Token::Ident("something".to_string()),
            Token::Equals,
            Token::Int(0),
            Token::Float(11.11),
            Token::EndWhile,
        ])
    );
}

#[test]
fn test_parse_ident() {
    let input = "key = valu3";
    let mut chars = input.chars().peekable();
    let first = chars.next().unwrap();
    assert_eq!(
        parse_ident(first, &mut chars),
        Some(Token::Ident("key".to_string()))
    );
    assert_eq!(chars.next(), Some(' '));
    assert_eq!(chars.next(), Some('='));
    assert_eq!(chars.next(), Some(' '));
    assert_eq!(
        parse_ident(chars.next().unwrap(), &mut chars),
        Some(Token::Ident("valu3".to_string()))
    );
}
