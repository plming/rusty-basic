#[derive(Debug, PartialEq)]
pub enum Token {
    Comma,
    OpeningParenthesis,
    ClosingParenthesis,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
    NumberLiteral(i16),
    StringLiteral(Vec<u8>),
    Variable { identifier: u8 },
    Print,
    If,
    Then,
    Goto,
    Input,
    Let,
    GoSub,
    Return,
    Clear,
    List,
    Run,
    End,
}
