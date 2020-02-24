//use super::lexer::Token;

use super::lexer::lex;

use super::parser::parse;
use super::parser::AST;
use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Debug)]
struct State {
    memory: HashMap<u64, u8>,
    pointer: u64
}

fn execute_increment(_path: Option<&String>, state: &mut State) {
    //println!("+ {:?}", state);
    state.memory.entry(state.pointer)
        .and_modify(|e| { if e == &255 {*e -= 255} else {*e += 1} })
        .or_insert(1);
}

fn execute_decrement(_path: Option<&String>, state: &mut State) {
    //println!("- {:?}", state);
    state.memory.entry(state.pointer)
        .and_modify(|e| { if e == &0 {*e += 255} else {*e -= 1 } })
        .or_insert(255);
}

fn execute_left(path: Option<&String>, state: &mut State) {
    //println!("< {:?}", state);
    if state.pointer == 0 {
        panic!("[{:?}] Cannot move pointer left of 0", path);
    }
    state.pointer -= 1
}

fn execute_right(_path: Option<&String>, state: &mut State) {
    //println!("> {:?}", state);
    state.pointer += 1
}

fn execute_output(_path: Option<&String>, state: &mut State) {
    //println!(". {:?}", state);
    let cell = state.memory.get(&state.pointer);
    if let Some(value) = cell {
        print!("{}", char::from(value.clone()))
    } else {
        print!("{}", char::from(0))
    }
}

fn execute_input(path: Option<&String>, state: &mut State) {
    //println!(", {:?}", state);
    let mut buffer= [0; 1];
    stdin().read_exact(&mut buffer)
        .expect(&format!("[{:?}] Error reading from stdin.", path));
    state.memory.insert(state.pointer, buffer[0]);
}

fn check_loop_condition(state: &mut State) -> bool {
    //println!("condition {:?}", state);
    let cell = state.memory.get(&state.pointer);
    if let Some(value) = cell {
        return value != &0
    } else {
        return false
    }
}

fn execute_loop(path: Option<&String>, state: &mut State, children: &Vec<AST>) {
    //println!("[] {:?}", state);
    while {
        for child in children.iter() {
            match child {
                AST::Increment => execute_increment(path, state),
                AST::Decrement => execute_decrement(path, state),
                AST::Left => execute_left(path, state),
                AST::Right => execute_right(path, state),
                AST::Input => execute_input(path, state),
                AST::Output => execute_output(path, state),
                AST::Loop{children} => execute_loop(path, state, children),
                AST::Top{children: _} => panic!("[{:?}] Illegal Top node found inside AST", path)
            }
        }

        check_loop_condition(state)
    } /*do*/ {}
}

fn execute(path: Option<&String>, ast: AST) {
    let mut state = State {
        memory: HashMap::new(),
        pointer: 0,
    };

    match ast {
        AST::Top {children} => {
            for child in children.iter() {
                match child {
                    AST::Increment => execute_increment(path, &mut state),
                    AST::Decrement => execute_decrement(path, &mut state),
                    AST::Left => execute_left(path, &mut state),
                    AST::Right => execute_right(path, &mut state),
                    AST::Input => execute_input(path, &mut state),
                    AST::Output => execute_output(path, &mut state),
                    AST::Loop{children} => execute_loop(path, &mut state, children),
                    AST::Top{children: _} => panic!("[{:?}] Illegal Top node found inside AST", path)
                }
            }
        }
        _ => panic!{"[{:?}] AST's root must be of type Top, but is {:?} instead", path, ast}
    }
}

pub fn interpret(path: Option<&String>, content: String) -> () {
    let tokens = lex(path, content);

    //println!("{:?}", tokens);

    let ast = parse(path, tokens);

    //println!("{:?}", ast);

    execute(path, ast);
}