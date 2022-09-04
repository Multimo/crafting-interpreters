use std::env;

use std::fs::File;
use std::io;
use std::io::Read;

mod tokens;
use tokens::{Token, TokenType, parse_chars};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Entering Repl mode");
        repl_mode();
        return;
    }

    let filename = &args[1];

    println!("Reading file {}", filename);
    
    let contents = load_file(filename.clone());

    println!("With text:\n{}", contents);

    run(contents);
}

fn repl_mode() {
    println!(">");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let tokens = run(input);
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
        Err(error) => panic!("Failed to open file: {}", error)
    };

    let mut contents_buffer = String::new();
    match file.read_to_string(&mut contents_buffer) {
        Ok(contents) => contents.to_string(),
        Err(error) => panic!("Failed reading contents of file: {}", error)
    }
}

// fn run_file() {}

fn run(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> =  Vec::new();

    // let start: i32 = 0;
    // let source_length: i32 = source.len().try_into().unwrap();
    // let mut current: i32 = 0;
    let line: i32 = 0;


    let mut source_iterator = source.char_indices();
    println!("source:{}, source_iterator: {:?}", source, source_iterator);
    loop {
        let character = source_iterator.next();
        println!("character:{:?},", character);

        match character {
            Some((_ind, char)) => {
                let token = parse_chars(char, &mut source_iterator);
                match token {
                    Some(token_type) => {
                        tokens.push(Token { token_type, lexeme: "".to_owned(), literal: "".to_owned(), line })
                    }
                    None => {}
                }
            }
            None => {
                println!("found the end");
            break;
            }
        }

    }
    

    tokens.push(Token { token_type: TokenType::EOF, lexeme: "".to_owned(), literal: "".to_owned(), line });
    tokens
}

fn scan_token() {
    // parse_chars()
    todo!()
}

fn report(line: i32, where_claus: String, message: String) {
    println!("[line: {}]: Error: {}: {}", line, where_claus, message)
}


