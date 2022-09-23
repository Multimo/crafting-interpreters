use std::env;

use std::fs::File;
use std::io;
use std::io::Read;

mod tokens;
use tokens::{parse_chars, Token, TokenType};

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Running compiler: {:?}", args);

    if args.len() == 1 {
        println!("Entering Repl mode");
        repl_mode();
        return;
    }

    let filename = &args[1];

    println!("Reading file {}", filename);

    let contents = load_file(filename.clone());

    println!("With text:\n{}", contents);

    scan_tokens(contents);
}

fn repl_mode() {
    println!(">");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let tokens = scan_tokens(input);
                println!("{:?}", tokens);
                println!(">");
            }
            Err(error) => println!("error reading repl input: {}", error),
        }
    }
}

fn load_file(file_path: String) -> String {
    let file_result: Result<File, io::Error> = File::open(file_path);

    let mut file = match file_result {
        Ok(s) => s,
        Err(error) => panic!("Failed to open file: {}", error),
    };

    let mut contents_buffer = String::new();
    match file.read_to_string(&mut contents_buffer) {
        Ok(contents) => contents.to_string(),
        Err(error) => panic!("Failed reading contents of file: {}", error),
    }
}

fn scan_tokens(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let current_line: i32 = 0;

    let mut source_iterator = source.char_indices();
    println!("source:{}, source_iterator: {:?}", source, source_iterator);
    loop {
        let character = source_iterator.next();
        println!("character:{:?},", character);

        match character {
            Some((_ind, char)) => {
                let token = parse_chars(char, &mut source_iterator);
                match token {
                    Some(token) => tokens.push(token),
                    None => report(
                        current_line,
                        "".to_string(),
                        "Unidentified char".to_string(),
                    ),
                }
            }
            None => {
                println!("found the end return EOF");
                tokens.push(Token::new_token(TokenType::EOF));
                break;
            }
        }
    }

    tokens
}

fn report(line: i32, where_claus: String, message: String) {
    println!("[line: {}]: Error: {}: {}", line, where_claus, message)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::tokens::Literal;

    use super::*;

    fn assert_scanner_results(source: &str, mut expected_token: Vec<Token>) {
        let result = scan_tokens(source.to_string());

        let mut eof_token = vec![Token::new_token(TokenType::EOF)];
        expected_token.append(&mut eof_token);

        println!("expected the following {:?}", expected_token);
        println!("found this {:?}", result);

        assert_eq!(result.len(), expected_token.len());
        for (index, token) in expected_token.into_iter().enumerate() {
            let result_token: &Token = result.get(index).unwrap();
            assert_eq!(*result_token, token);
        }
    }

    #[test]
    fn single_level_equals() {
        assert_scanner_results("=", vec![Token::new_token(TokenType::EQUAL)]);
    }

    #[test]
    fn single_level_greater() {
        assert_scanner_results(">", vec![Token::new_token(TokenType::GREATER)]);
    }

    #[test]
    fn single_level_left_bracket() {
        assert_scanner_results("[", vec![Token::new_token(TokenType::LeftBrace)]);
    }

    #[test]
    fn double_level_equal() {
        assert_scanner_results(">=", vec![Token::new_token(TokenType::GreatEqual)]);
    }

    #[test]
    fn comments() {
        assert_scanner_results(
            "// hello i am a comment \n!=",
            vec![Token::new_token(TokenType::BangEqual)],
        )
    }

    #[test]
    fn division() {
        assert_scanner_results("/", vec![Token::new_token(TokenType::SLASH)])
    }

    #[test]
    fn identifier() {
        assert_scanner_results(
            "hello",
            vec![Token {
                token_type: TokenType::IDENTIFIER,
                lexeme: Some(vec!['h', 'e', 'l', 'l', 'o']),
                line: 0,
                literal: Some(Literal::Identifier("hello".to_string())),
            }],
        )
    }

    #[test]
    fn and() {
        assert_scanner_results(
            "and",
            vec![Token {
                token_type: TokenType::AND,
                literal: None,
                lexeme: Some(vec!['a', 'n', 'd']),
                line: 0,
            }],
        )
    }
    #[test]
    fn string() {
        assert_scanner_results(
            "\"and\"",
            vec![Token {
                token_type: TokenType::STRING,
                lexeme: Some(vec!['a', 'n', 'd']),
                literal: Some(Literal::Str("and".to_string())),
                line: 0,
            }],
        )
    }
    #[test]
    fn number() {
        assert_scanner_results(
            "123",
            vec![Token {
                token_type: TokenType::NUMBER,
                lexeme: Some(vec!['1', '2', '3']),
                literal: Some(Literal::Number(123_f64)),
                line: 0,
            }],
        )
    }
    #[test]
    fn number_with_decimal() {
        assert_scanner_results(
            "123.123",
            vec![Token {
                token_type: TokenType::NUMBER,
                lexeme: Some(vec!['1', '2', '3', '.', '1', '2', '3']),
                literal: Some(Literal::Number(123.123)),
                line: 0,
            }],
        )
    }
    #[test]
    fn number_with_multiple_decimals() {
        assert_scanner_results(
            "123.123.123",
            vec![Token {
                token_type: TokenType::IDENTIFIER,
                lexeme: Some(vec!['1', '2', '3', '.', '1', '2', '3', '.', '1', '2', '3']),
                literal: Some(Literal::Str("123.123.123".to_string())),
                line: 0,
            }],
        )
    }
    #[test]
    fn number_with_decimal_but_word() {
        assert_scanner_results(
            "123.123ffafaf",
            vec![Token {
                token_type: TokenType::IDENTIFIER,
                lexeme: Some(vec![
                    '1', '2', '3', '.', '1', '2', '3', 'f', 'f', 'a', 'f', 'a', 'f',
                ]),
                line: 0,
                literal: Some(Literal::Str("123.123ffafaf".to_string())),
            }],
        )
    }
    #[test]
    fn unidentified() {
        let eof_token = vec![Token {
            token_type: TokenType::EOF,
            lexeme: None,
            literal: None,
            line: 0,
        }];
        let result = scan_tokens("@".to_string());
        // should break out of loop when finding unsupported char?
        assert_eq!(result.first(), eof_token.first());
        assert_eq!(result.last(), eof_token.last())
    }
    #[test]
    fn fun() {
        assert_scanner_results(
            "fun",
            vec![Token {
                token_type: TokenType::FUN,
                lexeme: Some(vec!['f', 'u', 'n']),
                line: 0,
                literal: None,
            }],
        )
    }
    #[test]
    fn try_for() {
        assert_scanner_results(
            "for",
            vec![Token {
                token_type: TokenType::FOR,
                lexeme: Some(vec!['f', 'o', 'r']),
                line: 0,
                literal: None,
            }],
        )
    }
    #[test]
    fn try_false() {
        assert_scanner_results(
            "false",
            vec![Token {
                token_type: TokenType::FALSE,
                lexeme: Some(vec!['f', 'a', 'l', 's', 'e']),
                line: 0,
                literal: None,
            }],
        )
    }
    #[test]
    fn try_false_fun() {
        assert_scanner_results(
            "false fun",
            vec![
                Token {
                    token_type: TokenType::FALSE,
                    lexeme: Some(vec!['f', 'a', 'l', 's', 'e']),
                    line: 0,
                    literal: None,
                },
                Token {
                    token_type: TokenType::FUN,
                    lexeme: Some(vec!['f', 'u', 'n']),
                    line: 0,
                    literal: None,
                },
            ],
        )
    }
    #[test]
    fn try_total_word() {
        assert_scanner_results(
            "funny",
            vec![Token {
                token_type: TokenType::IDENTIFIER,
                lexeme: Some(vec!['f', 'u', 'n', 'n', 'y']),
                line: 0,
                literal: Some(Literal::Identifier("funny".to_string())),
            }],
        )
    }
    #[test]
    fn try_total_word_2() {
        assert_scanner_results(
            "nile",
            vec![Token {
                token_type: TokenType::IDENTIFIER,
                lexeme: Some(vec!['n', 'i', 'l', 'e']),
                line: 0,
                literal: Some(Literal::Identifier("nile".to_string())),
            }],
        )
    }
    #[test]
    fn try_decimal_word() {
        assert_scanner_results(
            "var hello = 2.1212 fun",
            vec![
                Token {
                    token_type: TokenType::VAR,
                    lexeme: Some(vec!['v', 'a', 'r']),
                    line: 0,
                    literal: None,
                },
                Token {
                    token_type: TokenType::IDENTIFIER,
                    lexeme: Some(vec!['h', 'e', 'l', 'l', 'o']),
                    line: 0,
                    literal: Some(Literal::Identifier("hello".to_string())),
                },
                Token::new_token(TokenType::EQUAL),
                Token {
                    token_type: TokenType::NUMBER,
                    lexeme: Some(vec!['2', '.', '1', '2', '1', '2']),
                    line: 0,
                    literal: Some(Literal::Number(2.1212)),
                },
                Token {
                    token_type: TokenType::FUN,
                    lexeme: Some(vec!['f', 'u', 'n']),
                    line: 0,
                    literal: None,
                },
            ],
        )
    }
}
