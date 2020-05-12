#[macro_use]
extern crate lalrpop_util;
//extern crate unescape;

extern crate serde;
extern crate serde_lexpr;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate fml_ast;

lalrpop_mod!(pub fml); // synthesized by LALRPOP

use crate::fml::TopLevelParser;
use fml_ast::{AST};
use std::io::Read;
use std::fs::File;

#[cfg(not(test))]
fn main() {
    use std::io::{self, Read};
    use std::collections::HashMap;
    use std::env;

    type Converter = fn(&AST) -> String;
    let converters: HashMap<String, Converter> = {
        let mut converters: HashMap<String, Converter> = HashMap::new();
        converters.insert(String::from("--lisp"),
                          |ast| serde_lexpr::to_string(&ast).unwrap());
        converters.insert(String::from("--json"),
                          |ast| serde_json::to_string(&ast).unwrap());
        converters.insert(String::from("--yaml"),
                          |ast| serde_yaml::to_string(&ast).unwrap());
        converters
    };

    let (flags, files): (Vec<_>, Vec<_>) =
        env::args().into_iter().partition(|e| e.starts_with("--"));

    if files.len() > 1 {
        panic!("Can only parse 1 file at a time, but the following files were provided: {:?}", files)
    }

    let input: Reader = if files.is_empty() {
        io::stdin()
    } else {
        let path = files.last().unwrap();                  // Cannot explode due to conditions above
        File::open(path).expect(&format!("Cannot read file: {}", path))
    };

    let ast: AST = TopLevelParser::new().parse(input).expect("Parse error");

    flags.iter().for_each(|e| if converters.contains_key(e) {
        println!("{}", converters.get(e).unwrap()(&ast));
    } else {
        println!("Unknown flag: {}", e);
    });
}
