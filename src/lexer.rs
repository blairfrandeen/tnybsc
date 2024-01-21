#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Div,
    Mul,
    Assign,
    Equals,
    Ident(String),
}

fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

fn parse_ident(first: char, input: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Option<Token> {
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
            _ => match parse_ident(next, &mut chars) {
                Some(token) => tokens.push(token),
                _ => continue,
            },
        }
    }

    Ok(tokens)
}

#[test]
fn test_parse_tokens() {
    let res = parse_tokens("+-=+==");
    assert_eq!(
        res,
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
fn test_match_literal() {
    let parse_joe = match_literal("Hello Joe!");
    assert_eq!(Ok(("", ())), parse_joe("Hello Joe!"));
    assert_eq!(
        Ok((" Hello Robert!", ())),
        parse_joe("Hello Joe! Hello Robert!")
    );
    assert_eq!(Err("Sup boss?"), parse_joe("Sup boss?"));
}
