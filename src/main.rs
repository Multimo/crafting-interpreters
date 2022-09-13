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
                    Some(token_type) => tokens.push(Token {
                        token_type,
                        lexeme: "".to_owned(),
                        literal: "".to_owned(),
                        line: current_line,
                    }),
                    None => report(
                        current_line,
                        "".to_string(),
                        "Unidentified char".to_string(),
                    ),
                }
            }
            None => {
                println!("found the end return EOF");
                tokens.push(Token {
                    token_type: TokenType::EOF,
                    lexeme: "".to_owned(),
                    literal: "".to_owned(),
                    line: current_line,
                });
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
    use super::*;

    fn assert_scanner_results(source: &str, expected_token_type: Vec<TokenType>) {
        let result = scan_tokens(source.to_string());
        let mut expected: Vec<Token> = expected_token_type
            .into_iter()
            .map(|token| Token {
                token_type: token,
                lexeme: "".to_owned(),
                literal: "".to_owned(),
                line: 0,
            })
            .collect();

        let mut eof_token = vec![Token {
            token_type: TokenType::EOF,
            lexeme: "".to_owned(),
            literal: "".to_owned(),
            line: 0,
        }];
        expected.append(&mut eof_token);

        println!("results, {:?}", result);
        println!("expected, {:?}", expected);

        assert_eq!(result.len(), expected.len());
        for (index, token) in expected.into_iter().enumerate() {
            let result_token = result.get(index).unwrap();
            assert_eq!(*result_token, token);
        }
    }

    #[test]
    fn single_level_equals() {
        assert_scanner_results("=", vec![TokenType::EQUAL]);
    }

    #[test]
    fn single_level_greater() {
        assert_scanner_results(">", vec![TokenType::GREATER]);
    }

    #[test]
    fn single_level_left_bracket() {
        assert_scanner_results("[", vec![TokenType::LeftBrace]);
    }

    #[test]
    fn double_level_equal() {
        assert_scanner_results(">=", vec![TokenType::GreatEqual]);
    }

    #[test]
    fn comments() {
        assert_scanner_results("// hello i am a comment \n!=", vec![TokenType::BangEqual])
    }

    #[test]
    fn division() {
        assert_scanner_results("/", vec![TokenType::SLASH])
    }

    #[test]
    fn identifier() {
        assert_scanner_results("hello", vec![TokenType::IDENTIFIER])
    }

    #[test]
    fn and() {
        assert_scanner_results("and", vec![TokenType::AND])
    }
    #[test]
    fn string() {
        assert_scanner_results("\"and\"", vec![TokenType::STRING])
    }
    #[test]
    fn number() {
        assert_scanner_results("123", vec![TokenType::NUMBER])
    }
    #[test]
    fn unidentified() {
        let eof_token = vec![Token {
            token_type: TokenType::EOF,
            lexeme: "".to_owned(),
            literal: "".to_owned(),
            line: 0,
        }];
        assert_eq!(scan_tokens("@".to_string()).first(), eof_token.first())
    }
    #[test]
    fn fun() {
        assert_scanner_results("fun", vec![TokenType::FUN])
    }
    #[test]
    fn try_for() {
        assert_scanner_results("for", vec![TokenType::FOR])
    }
    #[test]
    fn try_false() {
        assert_scanner_results("false", vec![TokenType::FALSE])
    }
    #[test]
    fn try_false_fun() {
        assert_scanner_results("false fun", vec![TokenType::FALSE, TokenType::FUN])
    }
}
