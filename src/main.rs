use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Entering Repl mode");
        repl_mode();
        return;
    }

    let filename = &args[1];

    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
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

// fn run_file() {}

fn run(line: String) {
    println!("got line  {}", line)
}
