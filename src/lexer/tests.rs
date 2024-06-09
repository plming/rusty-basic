use super::{Token, TokenKind, lex};

#[test]
fn simple_input() {
    let sample_code = String::from("PRINT 2 + 5");
    let expected: Vec<Token> = vec![
        Token {
            kind: TokenKind::Keyword,
            text: "PRINT".to_string(),
        },
        Token {
            kind: TokenKind::Number,
            text: "2".to_string(),
        },
        Token {
            kind: TokenKind::Operator,
            text: "+".to_string(),
        },
        Token {
            kind: TokenKind::Number,
            text: "5".to_string(),
        },
    ];

    let result = lex(&sample_code);

    assert_eq!(result, expected);
}

#[test]
fn empty_input_should_return_empty_vector() {
    let code = String::from("");
    let expected: Vec<Token> = Vec::new();
    let result = lex(&code);

    assert_eq!(expected, result);
}
