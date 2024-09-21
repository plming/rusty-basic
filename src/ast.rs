#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Factor {
    Variable(Variable),
    NumberLiteral(NumberLiteral),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Term {
    factors: Vec<Factor>,
    operators: Vec<MultiplicativeOperator>,
}

impl Term {
    pub fn new(factor: Factor) -> Self {
        Self {
            factors: vec![factor],
            operators: Vec::new(),
        }
    }

    pub fn multiply_by(&mut self, factor: Factor) {
        self.push_factor(MultiplicativeOperator::Multiplication, factor);
    }

    pub fn divide_by(&mut self, factor: Factor) {
        self.push_factor(MultiplicativeOperator::Division, factor);
    }

    pub fn factors(&self) -> &[Factor] {
        &self.factors
    }

    pub fn operators(&self) -> &[MultiplicativeOperator] {
        &self.operators
    }

    fn push_factor(&mut self, operator: MultiplicativeOperator, factor: Factor) {
        self.factors.push(factor);
        self.operators.push(operator);

        debug_assert!(self.factors.len() == self.operators.len() + 1);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MultiplicativeOperator {
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    terms: Vec<Term>,
    operators: Vec<AdditiveOperator>,
}

impl Expression {
    pub fn new(unary_operator: Option<AdditiveOperator>, term: Term) -> Self {
        let mut expression = Self {
            terms: vec![term],
            operators: Vec::new(),
        };

        if let Some(operator) = unary_operator {
            expression.operators.push(operator);
        } else {
            // No operator, so we assume it's a positive number
            expression.operators.push(AdditiveOperator::Addition);
        }

        expression
    }

    pub fn add(&mut self, term: Term) {
        self.push_term(AdditiveOperator::Addition, term);
    }

    pub fn subtract(&mut self, term: Term) {
        self.push_term(AdditiveOperator::Subtraction, term);
    }

    pub fn terms(&self) -> &[Term] {
        &self.terms
    }

    pub fn operators(&self) -> &[AdditiveOperator] {
        &self.operators
    }

    fn push_term(&mut self, operator: AdditiveOperator, term: Term) {
        self.terms.push(term);
        self.operators.push(operator);

        debug_assert!(self.terms.len() == self.operators.len());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AdditiveOperator {
    Addition,
    Subtraction,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionListElement {
    StringLiteral(StringLiteral),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum RelationalOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    number: Option<u8>,
    statement: Statement,
}

impl Line {
    pub fn new(number: Option<u8>, statement: Statement) -> Self {
        Self { number, statement }
    }

    pub fn number(&self) -> Option<u8> {
        self.number
    }

    pub fn statement(&self) -> &Statement {
        &self.statement
    }
}
