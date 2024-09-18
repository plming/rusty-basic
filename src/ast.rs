#[derive(Debug, PartialEq)]
pub struct Variable {
    identifier: u8,
}

impl Variable {
    pub fn new(identifier: u8) -> Self {
        debug_assert!(identifier.is_ascii_uppercase());
        Self { identifier }
    }

    pub fn identifier(&self) -> u8 {
        self.identifier
    }
}

#[derive(Debug, PartialEq)]
pub struct NumberLiteral {
    value: i16,
}

impl NumberLiteral {
    pub fn new(value: i16) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i16 {
        self.value
    }
}

#[derive(Debug, PartialEq)]
pub struct StringLiteral {
    value: Vec<u8>,
}

impl StringLiteral {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Vec<u8> {
        &self.value
    }
}

#[derive(Debug, PartialEq)]
pub enum Factor {
    Variable(Variable),
    NumberLiteral(NumberLiteral),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct Term {
    pub factors: Vec<Factor>,
    pub operators: Vec<MultiplicativeOperator>,
}

#[derive(Debug, PartialEq)]
pub enum MultiplicativeOperator {
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub terms: Vec<Term>,
    pub operators: Vec<AdditiveOperator>,
}

#[derive(Debug, PartialEq)]
pub enum AdditiveOperator {
    Addition,
    Subtraction,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionListElement {
    StringLiteral(StringLiteral),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum RelationalOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }
}
