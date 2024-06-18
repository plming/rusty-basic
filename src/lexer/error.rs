#[derive(Debug)]
pub enum Error {
    UnexpectedCharacter,
    UnexpectedEndOfInput,
    InvalidIdentifier,
    InvalidStringLiteral,
}
