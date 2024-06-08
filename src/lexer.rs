use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword,
    Identifier,
    Operator,
    Number,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    text: String,
}

#[derive(Debug, PartialEq)]
enum LexemeKind {
    Whitespace,
    Digit,
    Operator,
    Alphabet,
}

fn find_lexeme(letter: &str) -> LexemeKind {
    if ["+", "-", "*", "/", "="].contains(&letter) {
        return LexemeKind::Operator;
    } else if ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].contains(&letter) {
        return LexemeKind::Digit;
    } else if letter.contains(char::is_whitespace) {
        return LexemeKind::Whitespace;
    } else {
        return LexemeKind::Alphabet;
    }
}

pub fn lex(code: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let letters: Vec<(usize, &str)> = code.grapheme_indices(true).collect();

    let mut start: usize = 0;
    while start < letters.len() {
        let start_state = find_lexeme(letters[start].1);
        if start_state == LexemeKind::Whitespace {
            start += 1;
            continue;
        }

        let mut end: usize = start + 1;
        while end < letters.len() {
            let end_state = find_lexeme(letters[end].1);
            if start_state == end_state {
                end += 1;
                continue;
            }

            break;
        }

        let kind = match start_state {
            LexemeKind::Whitespace => unreachable!(),
            LexemeKind::Digit => TokenKind::Number,
            LexemeKind::Operator => TokenKind::Operator,
            LexemeKind::Alphabet => TokenKind::Keyword,
        };

        let text: String;
        if end == letters.len() {
            text = code[letters[start].0..].to_string();
        } else {
            text = code[letters[start].0..letters[end].0].to_string();
        }

        tokens.push(Token { kind, text });

        start = end;
    }

    println!("{tokens:?}");
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_operators_correctly() {
        let sample_code = String::from("PRINT 2 + 5");
        let result = lex(&sample_code);
        let answer: Vec<Token> = vec![
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

        assert_eq!(result, answer);
    }
}
