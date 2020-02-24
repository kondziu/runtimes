#[derive(Debug)]
pub struct Position {
    line: u32,
    column: u32
}

#[derive(Debug)]
pub enum Token {
    Operation {position: Position, token: char},
    NewLine {position: Position},
    Whitespace {position: Position, token: String},
    Comment {position: Position, token: String},
}

pub fn lex(path: Option<&String>, content: String) -> Vec<Token> {

    let mut position = Position{line: 1, column: 1};
    let mut tokens: Vec<Token> = vec!();
    let mut buffer = Buffer::new();

    for character in content.chars() {
        match character {
            '\n' => {
                attempt_to_unload_buffer(path,&mut buffer, &mut tokens);
                tokens.push(Token::NewLine {position: Position {..position}});
                position.line += 1;
                position.column = 1;
            }
            '<' | '>' | '+' | '-' | '[' | ']' | '.' | ',' => {
                attempt_to_unload_buffer(path,&mut buffer, &mut tokens);
                tokens.push(Token::Operation {position: Position {..position}, token: character});
                position.column += 1;
            }
            ' ' | '\t' | '\r' if buffer.content_type != BufferType::Comment => {
                if buffer.content_type != BufferType::Whitespace {
                    attempt_to_unload_buffer(path, &mut buffer, &mut tokens);
                }
                position.column += 1;
                load_into_buffer(path,character,&mut buffer,BufferType::Whitespace, &position);
            }
            _ => {
                if buffer.content_type != BufferType::Comment {
                    attempt_to_unload_buffer(path, &mut buffer, &mut tokens);
                }
                position.column += 1;
                load_into_buffer(path,character, &mut buffer, BufferType::Comment, &position);
            }
        }
    }

    return tokens;
}

#[derive(Debug)]
#[derive(PartialEq)]
enum BufferType {Whitespace, Comment, None}

#[derive(Debug)]
struct Buffer {
    content: String,
    content_type: BufferType,
    start_position: Position
}

impl Buffer {
    fn new() -> Buffer {
        Buffer {
            content: String::new(),
            start_position: Position { line: 0, column: 0 },
            content_type: BufferType::None
        }
    }
}

fn load_into_buffer(path: Option<&String>,
                    character: char,
                    buffer: &mut Buffer,
                    expected_buffer_type: BufferType,
                    position: &Position) {

    match buffer.content_type {
        BufferType::None => {
            buffer.content_type = expected_buffer_type;
            buffer.content = character.to_string();
            buffer.start_position = Position {line: position.line, column: position.column};
        }

        _ if buffer.content_type == expected_buffer_type => {
            buffer.content += &character.to_string();
        }

        _ => {
            panic!(format!("[{:?}] Invalid buffer type at {:?}. Found: {:?}, Expected {:?}.",
                           path, position, buffer.content_type, expected_buffer_type));
        }
    }
}

fn attempt_to_unload_buffer(path: Option<&String>, buffer: &mut Buffer, tokens: &mut Vec<Token>) {

    if buffer.content_type == BufferType::None { return }                           // Short circuit

    let position = Position {
        line: buffer.start_position.line,
        column: buffer.start_position.column,
    };

    match buffer.content_type {
        BufferType::Comment => {
            let token = Token::Comment {token: buffer.content.clone(), position};

            *buffer = Buffer::new();
            tokens.push(token);
        }

        BufferType::Whitespace => {
            let token = Token::Whitespace {token: buffer.content.clone(), position};

            *buffer = Buffer::new();
            tokens.push(token);
        }

        BufferType::None => {
            panic!(format!("[{:?}]: This should never happen: {}",
                           path,
                           "trying to create token when BufferType::None"));
        }
    }
}