#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Div,
    Mul,
    Assign,
    Equals,
}

fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

fn identifier(input: &str) -> Result<(&str, String), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '_' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
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
            _ => continue,
        }
    }

    let add_tok = match_literal("+");

    dbg!(&tokens);

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

#[test]
fn test_ident() {
    assert_eq!(identifier("apple"), Ok(("", "apple".to_string())));
    assert_eq!(
        identifier("apple sauce"),
        Ok((" sauce", "apple".to_string()))
    );
    assert_eq!(
        identifier("apple_sauce"),
        Ok(("", "apple_sauce".to_string()))
    );
    assert_eq!(
        identifier("apple2sauce"),
        Ok(("", "apple2sauce".to_string()))
    );
    assert_eq!(identifier("1apple sauce"), Err("1apple sauce"));
}
