#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Keyword,
    Identifier,
    Operator,
    Number,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    text: String,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        return self.kind == other.kind && self.text == other.text;
    }
}

pub fn lex(code: &String) -> Vec<Token> {
    const OPERATORS: [char; 5] = ['+', '-', '*', '/', '%'];

    let mut tokens: Vec<Token> = Vec::new();
    for ch in code.chars() {
        if OPERATORS.contains(&ch) {
            tokens.push(Token {
                kind: TokenKind::Operator,
                text: ch.to_string(),
            });
        }
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_operators_correctly() {
        let sample_code = String::from("+-*/");
        let result = lex(&sample_code);
        let answer: Vec<Token> = vec![
            Token {
                kind: TokenKind::Operator,
                text: "+".to_string(),
            },
            Token {
                kind: TokenKind::Operator,
                text: "-".to_string(),
            },
            Token {
                kind: TokenKind::Operator,
                text: "*".to_string(),
            },
            Token {
                kind: TokenKind::Operator,
                text: "/".to_string(),
            },
        ];

        assert_eq!(result, answer);
    }
}
