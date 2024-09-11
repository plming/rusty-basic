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
    Variable { variable: Variable },
    Number { value: i16 },
    Expression { expression: Box<Expression> },
}

#[derive(Debug)]
pub struct Term {
    pub factors: Vec<Factor>,
    pub operators: Vec<MultiplicativeOperator>,
}

#[derive(Debug)]
pub enum MultiplicativeOperator {
    Multiplication,
    Division,
}

#[derive(Debug)]
pub struct Expression {
    pub terms: Vec<Term>,
    pub operators: Vec<AdditiveOperator>,
}

#[derive(Debug)]
pub enum AdditiveOperator {
    Addition,
    Subtraction,
}

#[derive(Debug)]
pub enum ExpressionListElement {
    String { value: Vec<u8> },
    Expression { expression: Expression },
}

#[derive(Debug)]
pub enum Statement {
    Print {
        expression_list: Vec<ExpressionListElement>,
    },
    If {
        left: Expression,
        operator: RelationalOperator,
        right: Expression,
        then: Box<Statement>,
    },
    Goto {
        expression: Expression,
    },
    Input {
        variable_list: Vec<Variable>,
    },
    Let {
        variable: Variable,
        expression: Expression,
    },
    GoSub {
        expression: Expression,
    },
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

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}
