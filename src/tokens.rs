use std::{collections::HashSet, str::CharIndices};

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32,
}

// impl Token {
//     fn to_string(&self) -> String {
//         format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
            keyword_match(current_char, char_string)
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
        'a' => match_on_keywords(char_string, &["and"], &[TokenType::AND]),
        'c' => match_on_keywords(char_string, &["class"], &[TokenType::CLASS]),
        'e' => match_on_keywords(char_string, &["else"], &[TokenType::ELSE]),
        'f' => match_on_keywords(
            char_string,
            &["for", "fun", "false"],
            &[TokenType::FOR, TokenType::FUN, TokenType::FALSE],
        ),
        'i' => match_on_keywords(char_string, &["if"], &[TokenType::IF]),
        'n' => match_on_keywords(char_string, &["nil"], &[TokenType::NIL]),
        'o' => match_on_keywords(char_string, &["or"], &[TokenType::OR]),
        'p' => match_on_keywords(char_string, &["print"], &[TokenType::PRINT]),
        'r' => match_on_keywords(char_string, &["return"], &[TokenType::RETURN]),
        's' => match_on_keywords(char_string, &["super"], &[TokenType::SUPER]),
        't' => match_on_keywords(
            char_string,
            &["this", "true"],
            &[TokenType::THIS, TokenType::TRUE],
        ),
        'v' => match_on_keywords(char_string, &["var"], &[TokenType::VAR]),
        'w' => match_on_keywords(char_string, &["while"], &[TokenType::WHILE]),
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

    let mut loop_index: usize = 1; // since the first letter is already matched
    loop {
        match source_chars.next() {
            Some((_, current_source_char)) => {
                if current_source_char.is_whitespace() {
                    break;
                }

                for (keywords_iter_index, mut keyword_char_iter) in
                    keywords_iter.clone().enumerate()
                {
                    match keyword_char_iter.nth(loop_index) {
                        Some((_, keyword_char)) => {
                            if keyword_char != current_source_char {
                                // not matching_words
                                matching_words.remove(&keywords_iter_index);
                            }
                        }
                        None => {
                            matching_words.remove(&loop_index);
                        }
                    };
                }

                loop_index += 1;
            }
            None => {
                break;
            }
        }
    }

    matching_words.iter().next().copied()
}

fn match_on_keywords(
    source_chars: &mut CharIndices,
    keywords: &[&str],
    token_types: &[TokenType],
) -> Option<TokenType> {
    match walk_keywords(source_chars, keywords) {
        Some(index) => match token_types.get(index) {
            Some(t) => Some(*t),
            None => Some(walk_to_next_whitespace(source_chars)),
        },
        None => Some(walk_to_next_whitespace(source_chars)),
    }
}
