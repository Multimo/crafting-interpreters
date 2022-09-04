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

pub fn parse_chars(
    current_char: char,
    char_string: &mut CharIndices,
    // current_line: mut i32,
) -> Option<TokenType> {
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
                        Some(TokenType::SLASH)
                    }
                }
                None => Some(TokenType::SLASH),
            }
        }
        current_char if current_char.is_alphabetic() => {
            println!("found is_alphabetic {}", current_char);
            match keyword_match(current_char, char_string) {
                Some(t) => Some(t),
                None => None,
            }
        }
        current_char if current_char.is_numeric() => {
            println!("found is_numeric {}", current_char);
            walk_to_next_whitespace(char_string);
            Some(TokenType::NUMBER)
        }
        '"' => {
            walk_to_next_quote(char_string);
            Some(TokenType::STRING)
        }
        _ => None,
    }
}

fn keyword_match(current_char: char, char_string: &mut CharIndices) -> Option<TokenType> {
    match current_char {
        'a' => {
            if walk_keyword(char_string, "and".to_string()) {
                Some(TokenType::AND)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        'c' => {
            if walk_keyword(char_string, "class".to_string()) {
                Some(TokenType::CLASS)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        'e' => {
            if walk_keyword(char_string, "else".to_string()) {
                Some(TokenType::ELSE)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        'f' => {
            if walk_keyword(char_string, "false".to_string()) {
                Some(TokenType::FALSE)
            } else if walk_keyword(char_string, "fun".to_string()) {
                Some(TokenType::FUN)
            } else if walk_keyword(char_string, "for".to_string()) {
                Some(TokenType::FOR)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        'i' => {
            if walk_keyword(char_string, "if".to_string()) {
                Some(TokenType::IF)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        'n' => {
            if walk_keyword(char_string, "nil".to_string()) {
                Some(TokenType::NIL)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        'o' => {
            if walk_keyword(char_string, "or".to_string()) {
                Some(TokenType::OR)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        'p' => {
            if walk_keyword(char_string, "print".to_string()) {
                Some(TokenType::PRINT)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        'r' => {
            if walk_keyword(char_string, "return".to_string()) {
                Some(TokenType::RETURN)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        's' => {
            if walk_keyword(char_string, "super".to_string()) {
                Some(TokenType::SUPER)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        't' => {
            if walk_keyword(char_string, "this".to_string()) {
                Some(TokenType::THIS)
            } else if walk_keyword(char_string, "true".to_string()) {
                Some(TokenType::TRUE)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        'v' => {
            if walk_keyword(char_string, "var".to_string()) {
                Some(TokenType::VAR)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }
        'w' => {
            if walk_keyword(char_string, "while".to_string()) {
                Some(TokenType::WHILE)
            } else {
                Some(walk_to_next_whitespace(char_string))
            }
        }

        _ => Some(walk_to_next_whitespace(char_string)),
    }
}

fn walk_to_next_whitespace(source_chars: &mut CharIndices) -> TokenType {
    loop {
        match source_chars.next() {
            Some(e) => match e.1 {
                '\n' => break,
                '\t' => break,
                ' ' => break,
                _ => continue,
            },
            None => {
                break;
            }
        }
    }

    TokenType::IDENTIFIER
}

fn walk_to_next_quote(source_chars: &mut CharIndices) {
    loop {
        match source_chars.next() {
            Some(e) => match e.1 {
                '"' => break,
                _ => continue,
            },
            None => {
                break;
            }
        }
    }
}

fn walk_keyword(source_chars: &mut CharIndices, keyword: String) -> bool {
    let mut keyword_iter = keyword.char_indices();
    keyword_iter.next();

    let mut matches = true;
    loop {
        let keyword_o = keyword_iter.next();
        let current_letter: char = match keyword_o {
            Some(e) => e.1,
            None => {
                break;
            }
        };

        match source_chars.next() {
            Some(s) => {
                if s.1 != current_letter {
                    matches = false;
                    break;
                }
            }
            None => {
                break;
            }
        }
    }

    matches
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
