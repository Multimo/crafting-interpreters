use crate::logger::report;
use std::{collections::HashSet, str::CharIndices};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    // function names etc
    Identifier(String),
    // string values
    Str(String),
    // numbers, all f64 for reasons of laziness
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    // Enum of token types
    pub token_type: TokenType,
    // a collection of the chars of the word ie 'a', 'n', 'd'
    pub lexeme: Option<Vec<char>>,
    // the value itself ie if number 123 or string "hello word"
    pub literal: Option<Literal>,
    // line of where column was found
    pub line: i32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: Option<Vec<char>>,
        literal: Option<Literal>,
        line: i32,
    ) -> Self {
        self::Token {
            token_type,
            literal,
            lexeme,
            line,
        }
    }

    pub fn new_token(token_type: TokenType) -> Self {
        Token::new(token_type, None, None, 0)
    }

    // fn to_string(&self) -> String {
    //     format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    // }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    DOT,
    MINUS,
    PLUS,
    SemiColon,
    SLASH,
    STAR,

    // One or Two character tokens
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreatEqual,
    LESS,
    LessEqual,

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    While,

    EOF,
}

// impl TokenType {}

pub fn parse_chars(
    current_char: char,
    char_string: &mut CharIndices,
    // current_line: mut i32,
) -> Option<Token> {
    // let character = current_char.as_str();
    println!(
        "parse_chars: current_char({}), char_string({:?})",
        current_char, char_string
    );
    match current_char {
        '(' => Some(Token::new_token(TokenType::LeftParen)),
        ')' => Some(Token::new_token(TokenType::RightParen)),
        '[' => Some(Token::new_token(TokenType::LeftBrace)),
        ']' => Some(Token::new_token(TokenType::RightBrace)),
        ',' => Some(Token::new_token(TokenType::Comma)),
        '.' => Some(Token::new_token(TokenType::DOT)),
        '-' => Some(Token::new_token(TokenType::MINUS)),
        '+' => Some(Token::new_token(TokenType::PLUS)),
        ';' => Some(Token::new_token(TokenType::SemiColon)),
        '*' => Some(Token::new_token(TokenType::STAR)),
        '!' => {
            let next_character = char_string.next();
            match next_character {
                Some((_, next_char)) => {
                    if next_char.to_string() == "=" {
                        // '!='
                        Some(Token::new_token(TokenType::BangEqual))
                    } else {
                        Some(Token::new_token(TokenType::BANG))
                    }
                }
                None => Some(Token::new_token(TokenType::BANG)),
            }
        }
        '=' => {
            let next_character = char_string.next();
            match next_character {
                Some((_, next_char)) => {
                    if next_char.to_string() == "=" {
                        // '=='
                        Some(Token::new_token(TokenType::EqualEqual))
                    } else {
                        Some(Token::new_token(TokenType::EQUAL))
                    }
                }
                None => Some(Token::new_token(TokenType::EQUAL)),
            }
        }
        '>' => {
            let next_character = char_string.next();
            match next_character {
                Some((_, next_char)) => {
                    if next_char.to_string() == "=" {
                        // '!='
                        Some(Token::new_token(TokenType::GreatEqual))
                    } else {
                        Some(Token::new_token(TokenType::GREATER))
                    }
                }
                None => Some(Token::new_token(TokenType::GREATER)),
            }
        }
        '<' => {
            let next_character = char_string.next();
            match next_character {
                Some((_, next_char)) => {
                    if next_char.to_string() == "=" {
                        // '!='
                        Some(Token::new_token(TokenType::LessEqual))
                    } else {
                        Some(Token::new_token(TokenType::LESS))
                    }
                }
                None => Some(Token::new_token(TokenType::LESS)),
            }
        }
        '/' => {
            let next_character = char_string.next();
            match next_character {
                Some((_, next_char)) => {
                    if next_char.to_string() == "/" {
                        loop {
                            let (_, following_char) = char_string.next()?;
                            if following_char == '\n' {
                                // current_line += 1;
                                return None;
                            }
                        }
                    } else {
                        Some(Token::new_token(TokenType::SLASH))
                    }
                }
                None => Some(Token::new_token(TokenType::SLASH)),
            }
        }
        current_char if current_char.is_alphabetic() => {
            println!("found is_alphabetic {}", current_char);
            keyword_match(current_char, char_string)
        }
        current_char if current_char.is_numeric() => {
            println!("found is_numeric {}", current_char);
            let chars = walk_to_next_whitespace(char_string, current_char);

            let liter_string: String = String::from_iter(chars.clone().into_iter());

            match liter_string.parse::<f64>() {
                Ok(v) => Some(Token::new(
                    TokenType::NUMBER,
                    Some(chars),
                    Some(Literal::Number(v)),
                    0,
                )),
                Err(e) => {
                    println!("Trouble Parsing number literal: {:?}", e);

                    Some(Token::new(
                        TokenType::IDENTIFIER,
                        Some(chars),
                        Some(Literal::Str(liter_string)),
                        0,
                    ))
                }
            }
        }
        '"' => {
            let chars = walk_to_next_quote(char_string);
            let literal_string: String = String::from_iter(chars.clone());
            Some(Token::new(
                TokenType::STRING,
                Some(chars),
                Some(Literal::Str(literal_string)),
                0,
            ))
        }
        _ => None,
    }
}

fn keyword_match(current_char: char, char_string: &mut CharIndices) -> Option<Token> {
    match current_char {
        'a' => match_on_keywords(char_string, &["and"], &[TokenType::AND], current_char),
        'c' => match_on_keywords(char_string, &["class"], &[TokenType::CLASS], current_char),
        'e' => match_on_keywords(char_string, &["else"], &[TokenType::ELSE], current_char),
        'f' => match_on_keywords(
            char_string,
            &["for", "fun", "false"],
            &[TokenType::FOR, TokenType::FUN, TokenType::FALSE],
            current_char,
        ),
        'i' => match_on_keywords(char_string, &["if"], &[TokenType::IF], current_char),
        'n' => match_on_keywords(char_string, &["nil"], &[TokenType::NIL], current_char),
        'o' => match_on_keywords(char_string, &["or"], &[TokenType::OR], current_char),
        'p' => match_on_keywords(char_string, &["print"], &[TokenType::PRINT], current_char),
        'r' => match_on_keywords(char_string, &["return"], &[TokenType::RETURN], current_char),
        's' => match_on_keywords(char_string, &["super"], &[TokenType::SUPER], current_char),
        't' => match_on_keywords(
            char_string,
            &["this", "true"],
            &[TokenType::THIS, TokenType::TRUE],
            current_char,
        ),
        'v' => match_on_keywords(char_string, &["var"], &[TokenType::VAR], current_char),
        'w' => match_on_keywords(char_string, &["while"], &[TokenType::While], current_char),
        _ => Some(walk_to_next_whitespace_identifier(
            char_string,
            current_char,
        )),
    }
}

fn walk_to_next_whitespace(source_chars: &mut CharIndices, current_char: char) -> Vec<char> {
    let mut chars = vec![current_char];
    for e in source_chars.by_ref() {
        match e.1 {
            '\n' => break,
            '\t' => break,
            ' ' => break,
            _ => {
                chars.push(e.1);
                continue;
            }
        }
    }

    chars
}

fn walk_to_next_whitespace_identifier(source_chars: &mut CharIndices, current_char: char) -> Token {
    let chars = walk_to_next_whitespace(source_chars, current_char);
    let literal: String = String::from_iter(chars.clone());
    Token {
        token_type: TokenType::IDENTIFIER,
        lexeme: Some(chars),
        literal: Some(Literal::Identifier(literal)),
        line: 0,
    }
}

fn walk_to_next_quote(source_chars: &mut CharIndices) -> Vec<char> {
    let mut chars = Vec::new();
    for e in source_chars.by_ref() {
        match e.1 {
            '"' => break,
            _ => {
                chars.push(e.1);
                continue;
            }
        }
    }
    chars
}

/// .Returns Some(index) of matching keyword or None
fn walk_keywords(source_chars: &mut CharIndices, keywords: &[&str]) -> Option<usize> {
    let keywords_iter = keywords.iter().map(|kw| kw.char_indices());

    // [['t', 'h', 'i', 's'], ['t', 'r', 'u', 'e']]
    // [['t 'h, 'i', 's'], ['t', 'r', 'u', 'e']]
    // need to iterate on first
    // on each iteration get the current index of each vec
    // and return a vec with
    // 0: ['t', 't'] => t => true
    // 1: ['h', 'r']
    // 2: ['i', 'u']
    // 3: ['s', 'e']
    // 4: [None, None]
    let amount_words: Vec<usize> = (0..keywords.len()).collect();
    let mut matching_words: HashSet<usize> = HashSet::from_iter(amount_words);

    for (loop_index, current_source_char) in source_chars.by_ref() {
        println!(
            "current_source_char {:?} {:?}",
            current_source_char, matching_words
        );
        if current_source_char.is_whitespace() {
            break;
        }

        for (keywords_iter_index, mut keyword_char_iter) in keywords_iter.clone().enumerate() {
            match keyword_char_iter.nth(loop_index) {
                Some((_, keyword_char)) => {
                    if keyword_char != current_source_char {
                        // not matching_words
                        matching_words.remove(&keywords_iter_index);
                    }
                }
                None => {
                    println!("loop_index {:?}", loop_index);
                    matching_words.remove(&keywords_iter_index);
                }
            };
        }
    }

    matching_words.iter().next().copied()
}

fn match_on_keywords(
    source_chars: &mut CharIndices,
    keywords: &[&str],
    token_types: &[TokenType],
    current_char: char,
) -> Option<Token> {
    let word_token = walk_to_next_whitespace_identifier(source_chars, current_char);
    let cloned_word_token = word_token.clone();
    let word_lexeme = word_token.lexeme?;
    let word_chars_string = String::from_iter(word_lexeme.into_iter());
    let mut word_chars = word_chars_string.char_indices();

    if let Some(index) = walk_keywords(&mut word_chars, keywords) {
        if let Some(t) = token_types.get(index) {
            let lexeme: Option<Vec<char>> = keywords.get(index).map(|word| {
                let chars: Vec<char> = word.chars().into_iter().collect();
                chars
            });

            Some(Token {
                token_type: *t,
                lexeme,
                line: 0,
                literal: None,
            })
        } else {
            Some(cloned_word_token)
        }
    } else {
        Some(cloned_word_token)
    }
}

pub fn scan_tokens(source: String) -> Vec<Token> {
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
