#![allow(dead_code, unused_variables)]
use crate::lexer::Token;
use std::collections::HashSet;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
            symbols: HashSet::new(),
            labels_declared: HashSet::new(),
            labels_gotoed: HashSet::new(),
        }
    }

    pub fn build(&mut self, tokens: Vec<Token>) -> Result<(), &'static str> {
        let mut tokens = tokens.iter().peekable();
        self.statements = Program::get_statements(self, &mut tokens, None)?;
        Ok(())
    }

    fn get_statements<'a>(
        &mut self,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
        sentinel: Option<Token>,
    ) -> Result<Vec<Statement>, &'static str> {
        let mut statements: Vec<Statement> = Vec::new();
        while let Some(token) = tokens.next() {
            if sentinel.clone().is_some_and(|s| s == *token) {
                break;
            }
            match token {
                Token::Let => statements.push(Statement::let_statement(self, tokens)?),
                Token::Print => statements.push(Statement::print_statement(self, tokens)?),
                Token::If => statements.push(Statement::if_statement(self, tokens)?),
                Token::While => statements.push(Statement::while_statement(self, tokens)?),
                Token::Input => {
                    statements.push(Statement::ident_statement(self, tokens, Token::Input)?)
                }
                Token::Goto => {
                    statements.push(Statement::ident_statement(self, tokens, Token::Goto)?)
                }
                Token::Label => {
                    statements.push(Statement::ident_statement(self, tokens, Token::Label)?)
                }
                Token::NewLine => continue,
                _ => todo!("statement not implemented"),
            }
        }
        Ok(statements)
    }
}

#[derive(Debug)]
pub enum Statement {
    Let {
        ident: String,
        expression: Expression,
    },
    Print(PrintMessage),
    If {
        comparison: Comparison,
        statements: Vec<Statement>,
    },
    While {
        comparison: Comparison,
        statements: Vec<Statement>,
    },
    Label {
        ident: String,
    },
    Goto {
        ident: String,
    },
    Input {
        ident: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum PrintMessage {
    Expression(Expression),
    StrLit(String),
}

impl Statement {
    fn ident_statement<'a>(
        program: &mut Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
        statement_type: Token,
    ) -> Result<Statement, &'static str> {
        let ident = match tokens.next().ok_or("Expected identifier, got EOF") {
            Ok(Token::Ident(name)) => name.clone(),
            _ => return Err("Expected identifier"),
        };
        if tokens.next() != Some(&Token::NewLine) {
            return Err("Expected newline");
        }
        match statement_type {
            Token::Goto => match program.labels_declared.contains(&ident) {
                true => {
                    program.labels_gotoed.insert(ident.clone());
                    Ok(Statement::Goto { ident })
                }
                false => Err("Attempt to GOTO undeclared label: {ident}"),
            },
            Token::Label => {
                program.labels_declared.insert(ident.clone());
                Ok(Statement::Label { ident })
            }
            Token::Input => {
                program.symbols.insert(ident.clone());
                Ok(Statement::Input { ident })
            }
            _ => Err("Invalid statement type!"),
        }
    }

    fn let_statement<'a>(
        program: &mut Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Statement, &'static str> {
        let ident = match tokens.next().ok_or("Expected identifier, got EOF") {
            Ok(Token::Ident(name)) => name.clone(),
            _ => return Err("Expected identifier"),
        };

        match tokens.next().ok_or("Expected '=', got EOF") {
            Ok(Token::Assign) => {}
            _ => return Err("Expected '='"),
        }

        let expression = Expression::build(program, tokens)?;
        if tokens.next() != Some(&Token::NewLine) {
            return Err("Expected newline after 'LET' statement");
        }
        program.symbols.insert(ident.clone());

        Ok(Statement::Let { ident, expression })
    }

    fn print_statement<'a>(
        program: &mut Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Statement, &'static str> {
        let message = match tokens.peek().ok_or("Expected string literal or expression") {
            Ok(Token::StrLit(msg)) => PrintMessage::StrLit(msg.clone()),
            _ => PrintMessage::Expression(Expression::build(program, tokens)?),
        };

        // TODO: cleaner implementation here. The Expression::build function
        // consumes the newline, while the StrLit does not.
        match message {
            PrintMessage::StrLit(_) => {
                tokens.next();
                if tokens.next() != Some(&Token::NewLine) {
                    return Err("Expected newline after 'PRINT' statement");
                }
            }
            _ => {}
        };
        Ok(Statement::Print(message))
    }

    fn if_statement<'a>(
        program: &mut Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Statement, &'static str> {
        let comparison = Comparison::build(program, tokens)?;
        if tokens.next() != Some(&Token::Then) {
            return Err("Expected 'THEN' after 'IF' comparison");
        }
        if tokens.next() != Some(&Token::NewLine) {
            return Err("Expected newline after 'THEN'");
        }
        let statements = program.get_statements(tokens, Some(Token::EndIf))?;
        if tokens.next() != Some(&Token::NewLine) {
            return Err("Expected newline after 'ENDIF'");
        }
        Ok(Statement::If {
            comparison,
            statements,
        })
    }

    fn while_statement<'a>(
        program: &mut Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Statement, &'static str> {
        let comparison = Comparison::build(program, tokens)?;
        if tokens.next() != Some(&Token::Repeat) {
            return Err("Expected 'REPEAT' after 'WHILE' comparison");
        }
        if tokens.next() != Some(&Token::NewLine) {
            return Err("Expected newline after 'REPEAT'");
        }
        let statements = program.get_statements(tokens, Some(Token::EndWhile))?;
        if tokens.next() != Some(&Token::NewLine) {
            return Err("Expected newline after 'ENDWHILE'");
        }
        Ok(Statement::While {
            comparison,
            statements,
        })
    }
}

trait Build {
    fn build<'a>(
        program: &Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Self, &'static str>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Comparison {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

impl Build for Comparison {
    fn build<'a>(
        program: &Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Comparison, &'static str> {
        let left = Expression::build(program, tokens)?;
        let operator = match tokens.next().ok_or("Expected operator") {
            Ok(Token::Equals) => Token::Equals,
            Ok(Token::NotEquals) => Token::NotEquals,
            Ok(Token::Gt) => Token::Gt,
            Ok(Token::Gte) => Token::Gte,
            Ok(Token::Lt) => Token::Lt,
            Ok(Token::Lte) => Token::Lte,
            _ => return Err("Expected operator!"),
        };
        let right = Expression::build(program, tokens)?;
        Ok(Comparison {
            left,
            operator,
            right,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub first_term: Term,
    pub other_terms: Vec<Term>,
}

impl Build for Expression {
    fn build<'a>(
        program: &Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Expression, &'static str> {
        let first_term = Term::build(program, tokens)?;
        let mut other_terms: Vec<Term> = Vec::new();
        while let Ok(term) = Term::build(program, tokens) {
            other_terms.push(term);
        }
        Ok(Expression {
            first_term,
            other_terms,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Term {
    pub unary: Unary,
    pub components: Vec<TermComp>,
}
impl Build for Term {
    fn build<'a>(
        program: &Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Term, &'static str> {
        let unary = Unary::build(program, tokens)?;
        let mut components: Vec<TermComp> = Vec::new();
        while let Some(component) = TermComp::build(program, tokens) {
            components.push(component);
        }
        Ok(Term { unary, components })
    }
}

#[derive(Debug, PartialEq)]
pub struct TermComp {
    pub operator: Token,
    pub unary: Unary,
}
impl TermComp {
    fn build<'a>(
        program: &Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Option<TermComp> {
        let operator = match tokens.next_if(|&tok| (*tok == Token::Mul) | (*tok == Token::Div)) {
            Some(op) => op,
            None => return None,
        };
        let unary = match Unary::build(program, tokens) {
            Ok(un) => un,
            _ => return None,
        };

        Some(TermComp {
            operator: operator.clone(),
            unary,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Unary {
    pub operator: Option<Token>,
    pub primary: Primary,
}

impl Build for Unary {
    fn build<'a>(
        program: &Program,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    ) -> Result<Unary, &'static str> {
        let operator = tokens.next_if(|&tok| (*tok == Token::Add) | (*tok == Token::Sub));
        let primary = match tokens.peek().ok_or("Expected Primary Token") {
            Ok(Token::Float(val)) => Primary::Float(*val),
            Ok(Token::Int(val)) => Primary::Int(*val),
            Ok(Token::Ident(name)) => match program.symbols.contains(name) {
                true => Primary::Ident(name.clone()),
                false => return Err("Undeclared identifier!"),
            },
            _ => return Err("expected primary token"),
        };
        tokens.next();
        Ok(Unary {
            operator: operator.cloned(),
            primary,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Primary {
    Float(f32),
    Int(i32),
    Ident(String),
}
