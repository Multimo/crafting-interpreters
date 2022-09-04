use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
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
    WHILE,

    EOF,
}

// impl TokenType {

// }

pub fn parse_chars(current_char: char, char_string: &mut CharIndices) -> Option<TokenType> {
    // let character = current_char.as_str();
    println!(
        "parse_chars: current_char({}), char_string({:?})",
        current_char, char_string
    );
    match current_char {
        '(' => Some(TokenType::LeftParen),
        ')' => Some(TokenType::RightParen),
        '[' => Some(TokenType::LeftBrace),
        ']' => Some(TokenType::RightBrace),
        ',' => Some(TokenType::COMMA),
        '.' => Some(TokenType::DOT),
        '-' => Some(TokenType::MINUS),
        '+' => Some(TokenType::PLUS),
        ';' => Some(TokenType::SemiColon),
        '/' => Some(TokenType::SLASH),
        '*' => Some(TokenType::STAR),
        '!' => {
            let next_character = char_string.next();
            match next_character {
                Some((_, next_char)) => {
                    if next_char.to_string() == "=" {
                        // '!='
                        Some(TokenType::BangEqual)
                    } else {
                        Some(TokenType::BANG)
                    }
                }
                None => Some(TokenType::BANG),
            }
        }
        '=' => {
            let next_character = char_string.next();
            match next_character {
                Some((_, next_char)) => {
                    if next_char.to_string() == "=" {
                        // '=='
                        Some(TokenType::EqualEqual)
                    } else {
                        Some(TokenType::EQUAL)
                    }
                }
                None => Some(TokenType::EQUAL),
            }
        }
        '>' => {
            let next_character = char_string.next();
            match next_character {
                Some((_, next_char)) => {
                    if next_char.to_string() == "=" {
                        // '!='
                        Some(TokenType::GreatEqual)
                    } else {
                        Some(TokenType::GREATER)
                    }
                }
                None => Some(TokenType::GREATER),
            }
        }
        '<' => {
            let next_character = char_string.next();
            match next_character {
                Some((_, next_char)) => {
                    if next_char.to_string() == "=" {
                        // '!='
                        Some(TokenType::LessEqual)
                    } else {
                        Some(TokenType::LESS)
                    }
                }
                None => Some(TokenType::LESS),
            }
        }
        ' ' => None,
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32,
}

impl Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
