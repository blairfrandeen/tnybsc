use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Operators
    Add,
    Sub,
    Div,
    Mul,
    Assign,
    Equals,
    NewLine,
    Gt,
    // TODO: Remaining Comparison operators
    // Identifiers
    Ident(String),
    // Data Types
    Int(i32),
    Float(f32),
    StrLit(String),
    // Keywords
    EndIf,
    EndWhile,
    If,
    Input,
    Let,
    Print,
    Repeat,
    Then,
    While,
    // Errors
    Invalid(String),
}

fn parse_strlit(input: &mut Peekable<Chars>) -> Result<Token, &'static str> {
    let mut strlit = String::new();
    while let Some(chr) = input.next() {
        if chr == '\\' {
            match input.peek() {
                Some('"') => strlit.push(input.next().unwrap()),
                Some('\\') => strlit.push(input.next().unwrap()),
                _ => strlit.push(chr),
            }
        } else if chr == '"' {
            return Ok(Token::StrLit(strlit));
        } else {
            strlit.push(chr);
        }
    }
    Err("unterminated string literal!")
}

#[test]
fn test_parse_strlit() {
    let mut input = "I am quite hungry\"".chars().peekable();
    assert_eq!(
        parse_strlit(&mut input),
        Ok(Token::StrLit("I am quite hungry".to_string()))
    );
    let mut input = "He said, \\\"Feed Me!\\\" hungrily\"".chars().peekable();
    assert_eq!(
        parse_strlit(&mut input),
        Ok(Token::StrLit("He said, \"Feed Me!\" hungrily".to_string()))
    );
    let mut input = "BACON!".chars().peekable();
    assert_eq!(
        parse_strlit(&mut input),
        Err("unterminated string literal!")
    );
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
        "THEN" => Some(Token::Then),
        "ENDIF" => Some(Token::EndIf),
        "WHILE" => Some(Token::While),
        "ENDWHILE" => Some(Token::EndWhile),
        "PRINT" => Some(Token::Print),
        "INPUT" => Some(Token::Input),
        "REPEAT" => Some(Token::Repeat),
        "LET" => Some(Token::Let),
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

pub fn lex_source(input: &str) -> Result<Vec<Token>, &str> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = input.chars().peekable();
    let mut _line_num: u32 = 1; // eventually useful for error handling

    while let Some(next) = chars.next() {
        match next {
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Sub),
            '*' => tokens.push(Token::Mul),
            '/' => tokens.push(Token::Div),
            '>' => tokens.push(Token::Gt),
            '\n' => {
                tokens.push(Token::NewLine);
                _line_num += 1;
            }
            ' ' => continue,
            '\t' => continue,
            '=' => match chars.peek() {
                Some('=') => {
                    tokens.push(Token::Equals);
                    chars.next();
                }
                _ => tokens.push(Token::Assign),
            },
            '"' => tokens.push(parse_strlit(&mut chars)?),
            _ => {
                if let Some(token) = parse_keyword(next, &mut chars) {
                    tokens.push(token);
                } else if let Some(token) = parse_ident(next, &mut chars) {
                    tokens.push(token);
                } else if let Some(token) = parse_num(next, &mut chars) {
                    tokens.push(token);
                } else {
                    tokens.push(Token::Invalid(next.to_string()));
                }
            }
        }
    }

    Ok(tokens)
}

#[test]
fn test_parse_tokens() {
    assert_eq!(
        lex_source("+-=+=="),
        Ok(vec![
            Token::Add,
            Token::Sub,
            Token::Assign,
            Token::Add,
            Token::Equals
        ])
    );

    assert_eq!(
        lex_source("IF something == BANANA ENDIF grape"),
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
        lex_source("WHILE something == 0 11.11 ENDWHILE"),
        Ok(vec![
            Token::While,
            Token::Ident("something".to_string()),
            Token::Equals,
            Token::Int(0),
            Token::Float(11.11),
            Token::EndWhile,
        ])
    );

    assert_eq!(
        // matches https://austinhenley.com/blog/teenytinycompiler1.html
        lex_source("IF+-123 foo*THEN/\n"),
        Ok(vec![
            Token::If,
            Token::Add,
            Token::Sub,
            Token::Int(123),
            Token::Ident("foo".to_string()),
            Token::Mul,
            Token::Then,
            Token::Div,
            Token::NewLine,
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
