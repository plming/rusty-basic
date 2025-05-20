use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Found an invalid character like `b'@'`, `b'$'`
    InvalidCharacter,
    /// Lexed identifier is not keyword or variable
    UnknownIdentifier,
    /// Non terminated string literal like `"Hello, World!"`
    NonTerminatedStringLiteral,
}

pub fn lex(code: &[u8]) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut chars = code.iter().peekable();

    while let Some(ch) = chars.next() {
        let token = match ch {
            b',' => Token::Comma,
            b'(' => Token::OpeningParenthesis,
            b')' => Token::ClosingParenthesis,
            b'=' => Token::Equal,
            b'<' => match chars.peek() {
                Some(b'=') => {
                    chars.next();
                    Token::LessThanOrEqual
                }
                Some(b'>') => {
                    chars.next();
                    Token::NotEqual
                }
                _ => Token::LessThan,
            },
            b'>' => match chars.peek() {
                Some(b'=') => {
                    chars.next();
                    Token::GreaterThanOrEqual
                }
                Some(b'<') => {
                    chars.next();
                    Token::NotEqual
                }
                _ => Token::GreaterThan,
            },
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Multiply,
            b'/' => Token::Divide,
            b'0'..=b'9' => {
                let mut value: i16 = (ch - b'0') as i16;
                while let Some(&ch @ b'0'..=b'9') = chars.peek() {
                    value *= 10;
                    value += (ch - b'0') as i16;
                    chars.next();
                }

                Token::NumberLiteral(value)
            }
            b'"' => {
                let mut value = Vec::new();
                let mut is_string_terminated = false;
                for &ch in &mut chars {
                    if ch == b'"' {
                        is_string_terminated = true;
                        break;
                    }

                    value.push(ch);
                }

                if !is_string_terminated {
                    return Err(Error::NonTerminatedStringLiteral);
                }

                Token::StringLiteral { value }
            }
            ch if ch.is_ascii_alphabetic() => {
                /// The longest length of valid identifiers.
                const MAX_IDENTIFIER_LENGTH: usize = 6;

                let mut identifier = Vec::with_capacity(MAX_IDENTIFIER_LENGTH);
                identifier.push(ch.to_ascii_uppercase());

                while let Some(&ch) = chars.peek() {
                    if !ch.is_ascii_alphanumeric() {
                        break;
                    }

                    identifier.push(ch.to_ascii_uppercase());
                    chars.next();
                }

                debug_assert_eq!(identifier, identifier.to_ascii_uppercase());

                // handle variable identifier
                if identifier.len() == 1 {
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
                        _ => return Err(Error::UnknownIdentifier),
                    }
                }
            }
            ch if ch.is_ascii_whitespace() => continue,
            _ => return Err(Error::InvalidCharacter),
        };

        tokens.push(token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_hello_world_returns_tokens() {
        let code = b"PRINT \"Hello, World!\"";
        let expected = vec![
            Token::Print,
            Token::StringLiteral {
                value: b"Hello, World!".to_vec(),
            },
        ];

        let actual = lex(code);

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn lex_expression_returns_tokens() {
        let expression = b"1 + 2 * 3 / 4 - 5";
        let expected = vec![
            Token::NumberLiteral(1),
            Token::Plus,
            Token::NumberLiteral(2),
            Token::Multiply,
            Token::NumberLiteral(3),
            Token::Divide,
            Token::NumberLiteral(4),
            Token::Minus,
            Token::NumberLiteral(5),
        ];

        let actual = lex(expression);

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn lex_keywords_returns_tokens() {
        let code = b"PRINT IF THEN GOTO INPUT LET GOSUB RETURN CLEAR LIST RUN END";
        let expected = vec![
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
        ];

        let actual = lex(code);

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn lex_variable_returns_token() {
        let code = b"IF A < B THEN PRINT Z";
        let expected = vec![
            Token::If,
            Token::Variable { identifier: b'A' },
            Token::LessThan,
            Token::Variable { identifier: b'B' },
            Token::Then,
            Token::Print,
            Token::Variable { identifier: b'Z' },
        ];

        let actual = lex(code);

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn lex_lowercase_variable_returns_uppercase_token() {
        let code = b"IF a < b THEN PRINT z";
        let expected = vec![
            Token::If,
            Token::Variable { identifier: b'A' },
            Token::LessThan,
            Token::Variable { identifier: b'B' },
            Token::Then,
            Token::Print,
            Token::Variable { identifier: b'Z' },
        ];

        let actual = lex(code);

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn lex_unknown_identifier_returns_error() {
        let invalid_code = b"PRINT HELLO";

        let actual = lex(invalid_code);

        assert_eq!(Err(Error::UnknownIdentifier), actual);
    }

    #[test]
    fn lex_non_terminated_string_returns_error() {
        let invalid_code = b"PRINT \"Hello, World!";

        let actual = lex(invalid_code);

        assert_eq!(Err(Error::NonTerminatedStringLiteral), actual);
    }

    #[test]
    fn lex_empty_string_literal_returns_tokens() {
        let code = br#"PRINT "", """#;
        let expected = vec![
            Token::Print,
            Token::StringLiteral { value: Vec::new() },
            Token::Comma,
            Token::StringLiteral { value: Vec::new() },
        ];

        let actual = lex(code);

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn lex_number_with_boundary_digit_returns_tokens() {
        let code = b"9999";
        let expected = vec![Token::NumberLiteral(9999)];

        let actual = lex(code);

        assert_eq!(Ok(expected), actual);
    }
}
