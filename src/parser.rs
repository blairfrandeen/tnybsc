#![allow(dead_code, unused_variables)]
use crate::lexer::Token;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn build(tokens: Vec<Token>) -> Result<Program, &'static str> {
        let mut statements: Vec<Statement> = Vec::new();
        let mut tokens = tokens.iter().peekable();
        while let Some(token) = tokens.next() {
            match token {
                Token::Let => {
                    let let_stmt = LetStatement::build(&mut tokens)?;
                    statements.push(Statement::Let(let_stmt));
                }
                Token::Print => {
                    let print_stmt = PrintStatement::build(&mut tokens)?;
                    statements.push(Statement::Print(print_stmt));
                }
                _ => todo!("not implemented"),
            }
        }
        Ok(Program { statements })
    }
}

#[test]
fn test_prgm() {
    let prog = "LET a = 1\n";
}

#[derive(Debug)]
enum Statement {
    Let(LetStatement),
    Print(PrintStatement),
    // TODO
}

#[derive(Debug)]
struct LetStatement {
    ident: Token,
    expression: Expression,
}

#[derive(Debug)]
enum PrintMessage {
    Expression(Expression),
    StrLit(String),
}

#[derive(Debug)]
struct PrintStatement {
    message: PrintMessage,
}

impl Build for PrintStatement {
    fn build<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<PrintStatement, &'static str> {
        let message = match tokens.peek().ok_or("Expected string literal or expression") {
            Ok(Token::StrLit(msg)) => PrintMessage::StrLit(msg.clone()),
            _ => PrintMessage::Expression(Expression::build(tokens)?),
        };
        Ok(PrintStatement { message })
    }
}

trait Build {
    fn build<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Self, &'static str>
    where
        Self: Sized;
}

impl Build for LetStatement {
    fn build<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<LetStatement, &'static str> {
        let ident = match tokens.next().ok_or("Expected identifier, got EOF") {
            Ok(Token::Ident(name)) => Token::Ident(name.clone()),
            _ => return Err("Expected identifier"),
        };

        match tokens.next().ok_or("Expected '=', got EOF") {
            Ok(Token::Assign) => {}
            _ => return Err("Expected '='"),
        }

        let expression = Expression::build(tokens)?;

        Ok(LetStatement { ident, expression })
    }
}

#[derive(Debug)]
struct Comparison {
    left: Expression,
    operator: Token,
    right: Expression,
}

#[derive(Debug)]
struct Expression {
    first_term: Term,
    other_terms: Vec<Term>,
}

impl Build for Expression {
    fn build<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Expression, &'static str> {
        let first_term = Term::build(tokens)?;
        let mut other_terms: Vec<Term> = Vec::new();
        while let Ok(term) = Term::build(tokens) {
            other_terms.push(term);
        }
        Ok(Expression {
            first_term,
            other_terms,
        })
    }
}

#[derive(Debug)]
struct Term {
    unary: Unary,
    components: Vec<TermComp>,
}
impl Build for Term {
    fn build<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Term, &'static str> {
        let unary = Unary::build(tokens)?;
        let mut components: Vec<TermComp> = Vec::new();
        while let Some(component) = TermComp::build(tokens) {
            components.push(component);
        }
        Ok(Term { unary, components })
    }
}

#[derive(Debug)]
struct TermComp {
    operator: Token,
    unary: Unary,
}
impl TermComp {
    fn build<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Option<TermComp> {
        let operator = match tokens.next_if(|&tok| (*tok == Token::Mul) | (*tok == Token::Div)) {
            Some(op) => op,
            None => return None,
        };
        let unary = match Unary::build(tokens) {
            Ok(un) => un,
            _ => return None,
        };

        Some(TermComp {
            operator: operator.clone(),
            unary,
        })
    }
}

#[derive(Debug)]
struct Unary {
    operator: Option<Token>,
    primary: Primary,
}
impl Build for Unary {
    fn build<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Unary, &'static str> {
        let operator = tokens.next_if(|&tok| (*tok == Token::Add) | (*tok == Token::Sub));
        let primary = match tokens.next().ok_or("Expected Primary Token") {
            Ok(Token::Float(val)) => Primary::Float(*val),
            Ok(Token::Int(val)) => Primary::Int(*val),
            Ok(Token::Ident(name)) => Primary::Ident(name.clone()),
            _ => return Err("expected primary token"),
        };
        Ok(Unary {
            operator: operator.cloned(),
            primary,
        })
    }
}

#[derive(Debug)]
enum Primary {
    Float(f32),
    Int(i32),
    Ident(String),
}
