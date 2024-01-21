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
    If,
    EndIf,
    While,
    EndWhile,
}

fn parse_ident(first: char, input: &mut Peekable<Chars>) -> Option<Token> {
    let mut matched = String::new();

    if first.is_alphabetic() {
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
            '-' => tokens.push(Token::Sub),
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
