use crate::lexer::Token;

struct Program {
    statements: Vec<Statement>,
}

struct Statement {
    components: Vec<StatementComp>,
}

enum StatementComp {
    Keyword(Token),
    Comparision(Comparison),
    Statement(Box<Statement>),
    Ident(Token),
    Operator(Token),
}

struct Comparison {
    left: Expression,
    operator: Token,
    right: Expression,
}

struct Expression {
    terms: Vec<Term>,
}

struct Term {
    unary: Unary,
    components: Vec<TermComp>,
}

struct TermComp {
    operator: Token,
    unary: Unary,
}

struct Unary {
    operator: Option<Token>,
    primary: Primary,
}

enum Primary {
    Float(Token),
    Int(Token),
    Ident(Token),
}
