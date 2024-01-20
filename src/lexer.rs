use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Div,
    Mul,
}

fn match_literial(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err("no literal match"),
    }
}

pub fn parse_tokens(mut input: &str) -> Result<Vec<&Token>, &str> {
    let mut token_map: HashMap<&str, &Token> = HashMap::new();
    let mut tokens: Vec<&Token> = vec![];
    token_map.insert("+", &Token::Add);
    token_map.insert("-", &Token::Sub);

    while input.len() > 0 {
        for key in token_map.keys() {
            let match_fn = match_literial(key);
            if let Ok((remaining, _)) = match_fn(key) {
                input = remaining;
                tokens.push(token_map.get(key).expect("token should exist"));
                break;
            }
        }
        dbg!(tokens);
        return Err(input);
    }

    Ok(tokens)
}
