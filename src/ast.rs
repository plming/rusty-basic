use std::fmt;

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

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier as char)
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

impl fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
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

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.value))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Factor {
    Variable(Variable),
    NumberLiteral(NumberLiteral),
    Expression(Box<Expression>),
}

impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Factor::Variable(variable) => write!(f, "{}", variable),
            Factor::NumberLiteral(number_literal) => write!(f, "{}", number_literal),
            Factor::Expression(expression) => write!(f, "({})", expression),
        }
    }
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

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for i in 0..self.factors.len() {
            if i > 0 {
                result.push_str(&format!("{}", self.operators[i - 1]));
            }

            result.push_str(&format!("{}", self.factors[i]));
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MultiplicativeOperator {
    Multiplication,
    Division,
}

impl fmt::Display for MultiplicativeOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MultiplicativeOperator::Multiplication => write!(f, "*"),
            MultiplicativeOperator::Division => write!(f, "/"),
        }
    }
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

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for i in 0..self.terms.len() {
            if i > 0 {
                result.push_str(&format!("{}", self.operators[i - 1]));
            }

            result.push_str(&format!("{}", self.terms[i]));
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AdditiveOperator {
    Addition,
    Subtraction,
}

impl fmt::Display for AdditiveOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdditiveOperator::Addition => write!(f, "+"),
            AdditiveOperator::Subtraction => write!(f, "-"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionListElement {
    StringLiteral(StringLiteral),
    Expression(Expression),
}

impl fmt::Display for ExpressionListElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpressionListElement::StringLiteral(string_literal) => write!(f, "{}", string_literal),
            ExpressionListElement::Expression(expression) => write!(f, "{}", expression),
        }
    }
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

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Print { expression_list } => {
                write!(f, "PRINT ")?;

                for (i, element) in expression_list.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", element)?;
                }

                Ok(())
            }
            Statement::If {
                left,
                operator,
                right,
                then,
            } => write!(f, "IF {} {} {} THEN {}", left, operator, right, then),
            Statement::Goto { expression } => write!(f, "GOTO {}", expression),
            Statement::Input { variable_list } => {
                write!(f, "INPUT ")?;

                for (i, variable) in variable_list.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", variable)?;
                }

                Ok(())
            }
            Statement::Let {
                variable,
                expression,
            } => write!(f, "LET {} = {}", variable, expression),
            Statement::GoSub { expression } => write!(f, "GOSUB {}", expression),
            Statement::Return => write!(f, "RETURN"),
            Statement::Clear => write!(f, "CLEAR"),
            Statement::List => write!(f, "LIST"),
            Statement::Run => write!(f, "RUN"),
            Statement::End => write!(f, "END"),
        }
    }
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

impl fmt::Display for RelationalOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RelationalOperator::Equal => write!(f, "="),
            RelationalOperator::NotEqual => write!(f, "<>"),
            RelationalOperator::LessThan => write!(f, "<"),
            RelationalOperator::LessThanOrEqual => write!(f, "<="),
            RelationalOperator::GreaterThan => write!(f, ">"),
            RelationalOperator::GreaterThanOrEqual => write!(f, ">="),
        }
    }
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

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(number) = self.number {
            write!(f, "{} ", number)?;
        }

        write!(f, "{}", self.statement)
    }
}
