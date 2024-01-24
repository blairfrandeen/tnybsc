use crate::lexer::Token;
use crate::parser::{
    Comparison, Expression, Primary, PrintMessage, Program, Statement, Term, TermComp, Unary,
};
use std::collections::HashSet;
use std::fmt;

pub struct Emitter {
    source: String,
}

impl fmt::Display for Emitter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.source.split_terminator('\n') {
            write!(f, "{}\n", line)?;
        }
        Ok(())
    }
}

impl Statement {
    fn emit(&self) -> String {
        let mut statement_str = String::new();
        match self {
            Statement::Let { ident, expression } => {
                todo!();
            }
            Statement::Print(msg) => match msg {
                PrintMessage::StrLit(message) => {
                    statement_str.push_str("printf(\"");
                    statement_str.push_str(message.as_str());
                    statement_str.push_str("\")");
                }
                PrintMessage::Expression(expr) => {
                    todo!()
                }
            },
            _ => {}
        };
        statement_str.push_str(";\n");
        statement_str
    }
}

impl Comparison {
    fn emit(self) -> String {
        let op = match self.operator {
            Token::Equals => "==",
            Token::Gt => ">",
            _ => panic!("Unexpected operator!"),
        };
        format!("{}{}{}", self.left.emit(), op, self.right.emit())
    }
}

impl Expression {
    fn emit(self) -> String {
        let mut expr_str = String::from(self.first_term.emit());
        for term in self.other_terms {
            expr_str.push_str(&term.emit())
        }
        expr_str
    }
}

impl Term {
    fn emit(self) -> String {
        let mut term_str = String::from(self.unary.emit());
        for comp in self.components {
            term_str.push_str(&comp.emit())
        }
        term_str
    }
}
impl TermComp {
    fn emit(self) -> String {
        let op_chr = match self.operator {
            Token::Mul => '*',
            Token::Div => '/',
            _ => panic!("Unexpected operator!"),
        };
        format!("{op_chr}{}", self.unary.emit())
    }
}

impl Unary {
    fn emit(self) -> String {
        if self.operator.is_none() {
            self.primary.emit()
        } else {
            let op_chr = match self.operator {
                Some(Token::Add) => '+',
                Some(Token::Sub) => '-',
                _ => panic!("Unexpected operator!"),
            };

            format!("{}{}", op_chr, self.primary.emit())
        }
    }
}

impl Primary {
    fn emit(self) -> String {
        match self {
            Primary::Float(val) => format!("{val}"),
            Primary::Int(val) => format!("{val}"),
            Primary::Ident(id) => id,
        }
    }
}

impl Emitter {
    pub fn new() -> Self {
        Emitter {
            source: String::new(),
        }
    }

    pub fn build(&mut self, program: Program) {
        self.header();
        let mut statements = program.statements.iter();
        let mut symbols_declared: HashSet<String> = HashSet::new();
        while let Some(statement) = statements.next() {
            let state_str = statement.emit();
            self.source.push_str(&state_str);
        }
        self.footer();
    }

    fn header(&mut self) {
        self.source.push_str("#include <stdio.h>\n");
        self.source.push_str("int main(void){\n");
    }

    fn footer(&mut self) {
        self.source.push_str("return 0;\n");
        self.source.push_str("}\n");
    }
}
