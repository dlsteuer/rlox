extern crate rustyline;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub mod scanner;
pub mod token;
pub mod token_type;
pub mod util;

use scanner::Scanner;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() > 1 {
        println!("Usage: jlox [script]")
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    match File::open("foo.txt") {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            run(contents)
        }
        Err(e) => {
            println!("Unable to read file: {}, error: {}", path, e);
        }
    }
}

fn run_prompt() {
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                run(line)
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    println!("Printing tokens ({}):", tokens.len());
    for token in tokens {
        println!("{}", token)
    }
}

fn error(line: &i64, message: &String) {
    report(&line, &String::from(""), &message);
}

fn report(line: &i64, loc: &String, message: &String) {
    println!("[line {}] Error{}: {}", line, loc, message)

    // TODO: something something had_error
}
