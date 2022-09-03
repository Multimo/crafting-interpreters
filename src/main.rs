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
                run(input);
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
    let source_length: i32 = source.len().try_into().unwrap();
    let mut line: i32 = 0;

    let mut current: i32 = 0;

    while current >= source_length {
        scan_token()
    }

    let hi = source.char_indices();
    while (Some(hi.next())) {

        let token = parse_chars(hi);
    }

    tokens.push(Token { token_type: TokenType::EOF, lexeme: "".to_owned(), literal: "".to_owned(), line });

    tokens
}

fn scan_token() {
    parse_chars()
    todo!()
}

fn report(line: i32, where_claus: String, message: String) {
    println!("[line: {}]: Error: {}: {}", line, where_claus, message)
}


