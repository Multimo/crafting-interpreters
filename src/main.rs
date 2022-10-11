use std::env;

use std::fs::File;
use std::io;
use std::io::Read;

mod logger;

mod tokens;
use tokens::scan_tokens;

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
