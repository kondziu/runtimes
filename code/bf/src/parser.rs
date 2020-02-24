use super::lexer::Token;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub enum AST {
    Top {children: Vec<AST>},
    Left,
    Right,
    Increment,
    Decrement,
    Input,
    Output,
    Loop {children: Vec<AST>},
}

impl AST {
    fn simple(c: &char) -> AST {
        match c {
            '<' => AST::Left,
            '>' => AST::Right,
            ',' => AST::Input,
            '.' => AST::Output,
            '+' => AST::Increment,
            '-' => AST::Decrement,
            _ => panic!("aaaa!")
        }
    }
}

fn parse_simple_operation(path: Option<&String>,parent: &mut AST, token: &char) {
    let node = AST::simple(token);

    match parent {
        AST::Loop { children } => children.push(node),
        AST::Top { children } => children.push(node),
        _ => panic!("[{:?}] Cannot create a simple operation AST node from {}]", path, token)
    }
}

fn parse_loop_begin(path: Option<&String>,parent: &mut AST, input: &mut Peekable<Iter<&Token>>) {
    let mut node = AST::Loop { children: vec!() };
    parse_children(path,&mut node, input);

    match parent {
        AST::Loop { children } => children.push(node),
        AST::Top { children } => children.push(node),
        _ => panic!("[{:?}] Invalid AST parent node {:?}]", path, parent)
    }
}

fn parse_children(path: Option<&String>,parent: &mut AST, input: &mut Peekable<Iter<&Token>>) {
    while let Some(_token) = input.peek() {
        if let Token::Operation {position: _, token} = _token {
            match token {
                '<' | '>' | ',' | '.' | '+' | '-' => {
                    input.next();
                    parse_simple_operation(path,parent, token)
                },
                '[' => {
                    input.next();
                    parse_loop_begin(path,parent, input)
                },
                ']' => {
                    input.next();
                    return;
                },
                _ => panic!("[{:?}] Unexpected token: {:?}", path, _token),
            }
        } else {
            panic!("[{:?}] Missing matching bracket for {:?}", path, parent)
        }
    }
}

fn parse_toplevel(path: Option<&String>,input: &mut Peekable<Iter<&Token>>) -> AST {
    let mut top = AST::Top {children: vec!()};

    while let Some(_token) = input.peek() {
        if let Token::Operation {position: _, token} = _token {
            match token {
                '<' | '>' | ',' | '.' | '+' | '-' => {
                    input.next();
                    parse_simple_operation(path,&mut top, token)
                },
                '[' => {
                    input.next();
                    parse_loop_begin(path,&mut top, input)
                },
                ']' => panic!("[{:?}] Unexpected bracket at: {:?}", path, _token),
                _ => panic!("[{:?}] Unexpected token: {:?}", path, _token),
            }
        } else {
            panic!("[{:?}] Parser found a non-operation token: {:?}", path, _token);
        }
    };

    return top
}

pub fn parse(path: Option<&String>, tokens: Vec<Token>) -> AST {
    let operations_only = | token: &&Token | {
        match token {
            Token::Operation {position: _, token: _} => true,
            _ => false
        }
    };

    let operation_tokens: Vec<&Token> = tokens.iter().filter(operations_only).collect();
    let mut input: Peekable<Iter<&Token>> = operation_tokens.iter().peekable();

    return parse_toplevel(path, &mut input);
}