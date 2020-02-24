//use std::collections::HashMap;

use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::io::{stdin, Read};

mod lexer;
mod parser;
mod brainfuck;

fn main () {
    let args: Vec<String> = env::args().collect();

    let not_files: Vec<&String> = args.iter()
        .filter(| &path | !Path::new(path).exists())
        //.fold(String::new(), |acc, path| )
        .collect();

    if !not_files.is_empty() {
        panic!("The following files are ungood: {:?}.",
               not_files.iter()
                   .fold(String::new(),
                         |acc, &path|
                             if acc.is_empty() {
                                path.to_owned()
                             } else {
                                acc + &String::from(" ") + path
                             }));
    }

    if args.len() <= 1 {
        let mut content: String = String::new();
        stdin().read_to_string(&mut content).expect("Could not read from stdin.");
        brainfuck::interpret(None, content);
        return;
    }

    for path in &args[1..] {
        let content = read_to_string(path)
            .expect(&format!("Could not read file: {}", path));

        brainfuck::interpret(Some(path), content);
    }
}
