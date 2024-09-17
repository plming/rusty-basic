use std::{collections::VecDeque, vec};

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Found an invalid character like b'@', b'$'
    InvalidCharacter,
    /// Lexed identifier is not keyword or variable
    InvalidIdentifier,
    /// Invalid string literal like "Hello, World!
    InvalidStringLiteral,
}

pub struct Lexer<'a> {
    /// the code to lex
    code: &'a [u8],
    /// the offset of the character not yet consumed
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a [u8]) -> Lexer {
        Lexer { code, position: 0 }
    }

    fn peek_next_char(&self) -> Option<u8> {
        self.code.get(self.position).copied()
    }

    fn read_next_char(&mut self) -> Option<u8> {
        let next_char = self.peek_next_char();
        if next_char.is_some() {
            self.position += 1;
        }

        next_char
    }

    pub fn lex(&mut self) -> Result<VecDeque<Token>, Error> {
        let mut tokens = VecDeque::new();

        while let Some(ch) = self.read_next_char() {
            let token = match ch {
                b',' => Token::Comma,
                b'(' => Token::OpeningParenthesis,
                b')' => Token::ClosingParenthesis,
                b'=' => Token::Equal,
                b'<' => match self.peek_next_char() {
                    Some(b'=') => {
                        self.read_next_char();
                        Token::LessThanOrEqual
                    }
                    Some(b'>') => {
                        self.read_next_char();
                        Token::NotEqual
                    }
                    _ => Token::LessThan,
                },
                b'>' => match self.peek_next_char() {
                    Some(b'=') => {
                        self.read_next_char();
                        Token::GreaterThanOrEqual
                    }
                    Some(b'<') => {
                        self.read_next_char();
                        Token::NotEqual
                    }
                    _ => Token::GreaterThan,
                },
                b'+' => Token::Plus,
                b'-' => Token::Minus,
                b'*' => Token::Multiply,
                b'/' => Token::Divide,
                digit @ b'0'..=b'9' => {
                    let mut value: i16 = (digit - b'0') as i16;
                    while let Some(digit @ b'0'..=b'9') = self.peek_next_char() {
                        value *= 10;
                        value += (digit - b'0') as i16;
                        self.read_next_char();
                    }
                    Token::NumberLiteral { value }
                }
                ch if ch.is_ascii_alphabetic() => {
                    let mut identifier: Vec<u8> = vec![ch.to_ascii_uppercase()];

                    while let Some(ch) = self.peek_next_char() {
                        if !ch.is_ascii_alphanumeric() {
                            break;
                        }

                        identifier.push(ch.to_ascii_uppercase());
                        self.read_next_char();
                    }

                    debug_assert_eq!(identifier, identifier.to_ascii_uppercase());

                    // handle variable identifier
                    if identifier.len() == 1 && identifier[0].is_ascii_alphabetic() {
                        Token::Variable {
                            identifier: identifier[0],
                        }
                    } else {
                        match identifier.as_slice() {
                            b"PRINT" => Token::Print,
                            b"IF" => Token::If,
                            b"THEN" => Token::Then,
                            b"GOTO" => Token::Goto,
                            b"INPUT" => Token::Input,
                            b"LET" => Token::Let,
                            b"GOSUB" => Token::GoSub,
                            b"RETURN" => Token::Return,
                            b"CLEAR" => Token::Clear,
                            b"LIST" => Token::List,
                            b"RUN" => Token::Run,
                            b"END" => Token::End,
                            _ => return Err(Error::InvalidIdentifier),
                        }
                    }
                }
                b'"' => {
                    let mut value: Vec<u8> = Vec::new();
                    let mut is_terminated = false;

                    while let Some(ch) = self.read_next_char() {
                        if ch == b'"' {
                            self.read_next_char();
                            is_terminated = true;
                            break;
                        }

                        value.push(ch);
                    }

                    if is_terminated {
                        Token::StringLiteral { value }
                    } else {
                        return Err(Error::InvalidStringLiteral);
                    }
                }
                _ if ch.is_ascii_whitespace() => continue,
                _ => return Err(Error::InvalidCharacter),
            };

            tokens.push_back(token);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_hello_world_returns_tokens() {
        let code = b"PRINT \"Hello, World!\"";
        let mut lexer = Lexer::new(code);
        let expected = VecDeque::from([
            Token::Print,
            Token::StringLiteral {
                value: b"Hello, World!".to_vec(),
            },
        ]);

        assert_eq!(lexer.lex(), Ok(expected));
    }

    #[test]
    fn lex_expression_returns_tokens() {
        let expression = b"1 + 2 * 3 / 4 - 5";
        let mut lexer = Lexer::new(expression);
        let expected = VecDeque::from([
            Token::NumberLiteral { value: 1 },
            Token::Plus,
            Token::NumberLiteral { value: 2 },
            Token::Multiply,
            Token::NumberLiteral { value: 3 },
            Token::Divide,
            Token::NumberLiteral { value: 4 },
            Token::Minus,
            Token::NumberLiteral { value: 5 },
        ]);

        assert_eq!(lexer.lex(), Ok(expected));
    }

    #[test]
    fn lex_keywords_returns_tokens() {
        let code = b"PRINT IF THEN GOTO INPUT LET GOSUB RETURN CLEAR LIST RUN END";
        let mut lexer = Lexer::new(code);
        let expected = VecDeque::from([
            Token::Print,
            Token::If,
            Token::Then,
            Token::Goto,
            Token::Input,
            Token::Let,
            Token::GoSub,
            Token::Return,
            Token::Clear,
            Token::List,
            Token::Run,
            Token::End,
        ]);

        assert_eq!(lexer.lex(), Ok(expected));
    }

    #[test]
    fn lex_variable_returns_token() {
        let code = b"IF A < B THEN PRINT Z";
        let mut lexer = Lexer::new(code);
        let expected = VecDeque::from([
            Token::If,
            Token::Variable { identifier: b'A' },
            Token::LessThan,
            Token::Variable { identifier: b'B' },
            Token::Then,
            Token::Print,
            Token::Variable { identifier: b'Z' },
        ]);

        assert_eq!(lexer.lex(), Ok(expected));
    }

    #[test]
    fn lex_lowercase_variable_returns_uppercase_token() {
        let code = b"IF a < b THEN PRINT z";
        let mut lexer = Lexer::new(code);
        let expected = VecDeque::from([
            Token::If,
            Token::Variable { identifier: b'A' },
            Token::LessThan,
            Token::Variable { identifier: b'B' },
            Token::Then,
            Token::Print,
            Token::Variable { identifier: b'Z' },
        ]);

        assert_eq!(lexer.lex(), Ok(expected));
    }

    #[test]
    fn lex_unknown_identifier_returns_error() {
        let invalid_code = b"PRINT HELLO";
        let mut lexer = Lexer::new(invalid_code);

        assert_eq!(lexer.lex(), Err(Error::InvalidIdentifier));
    }

    #[test]
    fn lex_non_terminated_string_returns_error() {
        let invalid_code = b"PRINT \"Hello, World!";
        let mut lexer = Lexer::new(invalid_code);

        assert_eq!(lexer.lex(), Err(Error::InvalidStringLiteral));
    }
}
