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
use fml_ast::{AST, Operator};

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

    let (flags, _ /*files*/): (Vec<_>, Vec<_>) =
        env::args().into_iter().partition(|e| e.starts_with("--"));

    let mut buffer = String::new();
    if io::stdin().read_to_string(&mut buffer).is_err() {
        println!("Cannot read stdin.");
        return;
    }

    let ast: AST = match TopLevelParser::new().parse(buffer.as_str()) {
        Ok(ast) => ast,
        Err(message) => {
            println!("Parse error: {}", message);
            AST::Unit
        },
    };

    flags.iter().for_each(|e| if converters.contains_key(e) {
        println!("{}", converters.get(e).unwrap()(&ast));
    } else {
        println!("Unknown flag: {}", e);
    });
}
