use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword,
    Identifier,
    Operator,
    Number,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    kind: TokenKind,
    text: &'a str,
}

#[derive(Debug, PartialEq)]
enum CharKind {
    Whitespace,
    Digit,
    Operator,
    Alphabet,
}

fn find_lexeme(letter: &str) -> CharKind {
    match letter {
        "+" | "-" | "*" | "/" | "=" => CharKind::Operator,
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => CharKind::Digit,
        _ if letter.contains(char::is_whitespace) => CharKind::Whitespace,
        _ => CharKind::Alphabet,
    }
}

pub fn lex(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let clusters: Vec<(usize, &str)> = code.grapheme_indices(true).collect();

    let mut start: usize = 0;
    while start < clusters.len() {
        let start_state = find_lexeme(clusters[start].1);
        if start_state == CharKind::Whitespace {
            start += 1;
            continue;
        }

        let mut end: usize = start + 1;
        while end < clusters.len() {
            let end_state = find_lexeme(clusters[end].1);
            if start_state == end_state {
                end += 1;
                continue;
            }

            break;
        }

        let text: &str;
        if end == clusters.len() {
            text = &code[clusters[start].0..];
        } else {
            text = &code[clusters[start].0..clusters[end].0];
        }

        let kind = match start_state {
            CharKind::Whitespace => unreachable!(),
            CharKind::Digit => TokenKind::Number,
            CharKind::Operator => TokenKind::Operator,
            CharKind::Alphabet => {
                match text {
                    "PRINT" => TokenKind::Keyword,
                    _ => TokenKind::Identifier,
                }
            },
        };

        tokens.push(Token { kind, text });

        start = end;
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_input() {
        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::Keyword,
                text: "PRINT",
            },
            Token {
                kind: TokenKind::Number,
                text: "2"
            },
            Token {
                kind: TokenKind::Operator,
                text: "+"
            },
            Token {
                kind: TokenKind::Number,
                text: "5"
            },
        ];

        let result = lex("PRINT 2 + 5");

        assert_eq!(result, expected);
    }

    #[test]
    fn empty_input_should_return_empty_vector() {
        let expected: Vec<Token> = Vec::new();
        let result = lex("");

        assert_eq!(expected, result);
    }
}
