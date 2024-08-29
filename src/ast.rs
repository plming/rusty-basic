#[derive(Debug)]
pub struct Variable {
    identifier: u8,
}

impl Variable {
    pub fn new(identifier: u8) -> Self {
        Self { identifier }
    }

    pub fn identifier(&self) -> u8 {
        self.identifier
    }
}

#[derive(Debug)]
pub enum Factor {
    Variable(Variable),
    Number(i16),
    Expression(Box<Expression>),
}

#[derive(Debug)]
pub struct Term {
    pub factors: Vec<Factor>,
    pub operators: Vec<FactorOperator>,
}

#[derive(Debug)]
pub enum FactorOperator {
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct Expression {
    pub terms: Vec<Term>,
    pub operators: Vec<TermOperator>,
}

#[derive(Debug)]
pub enum TermOperator {
    Add,
    Subtract,
}

pub type VariableList = Vec<Variable>;

#[derive(Debug)]
pub enum ExpressionListElement {
    String(Vec<u8>),
    Expression(Expression),
}

pub type ExpressionList = Vec<ExpressionListElement>;

#[derive(Debug)]
pub enum Statement {
    Print(ExpressionList),
    If {
        left: Expression,
        operator: RelationalOperator,
        right: Expression,
        then: Box<Statement>,
    },
    Goto(Expression),
    Input(VariableList),
    Let(Variable, Expression),
    GoSub(Expression),
    Return,
    Clear,
    List,
    Run,
    End,
}

#[derive(Debug)]
pub enum RelationalOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
