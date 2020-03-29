use std::collections::HashMap;
use std::error;
use std::fmt;

use crate::heap::{Reference, FunctionReference};

#[derive(Debug, Clone, PartialEq)]
pub enum EnvironmentError {
    Undefined (String),
    AlreadyDefined (String),
    NotFound (String),
    LastEnvironment,
    Impossible,
}

macro_rules! undefined_error {($name:expr) => { Err(EnvironmentError::Undefined($name)) }}
macro_rules! already_defined_error {($name:expr) => { Err(EnvironmentError::AlreadyDefined($name)) }}
macro_rules! not_found_error {($name:expr) => { Err(EnvironmentError::NotFound($name.to_string())) }}
//macro_rules! last_environment_error {() => { Err(EnvironmentError::LastEnvironment) }}
//macro_rules! impossible_error {() => { Err(EnvironmentError::Impossible) }}
macro_rules! all_good{() => { Ok(()) }}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnvironmentError::Undefined(binding) =>
                write!(f, "Attempt to redefine a binding that has not previously been defined {}", binding),
            EnvironmentError::AlreadyDefined(binding) =>
                write!(f, "Attempt to define a binding that has already been defined {}", binding),
            EnvironmentError::NotFound(binding) =>
                write!(f, "Attempt to look up a binding that not been defined {}", binding),
            EnvironmentError::Impossible =>
                write!(f, "This should never happen"),
            EnvironmentError::LastEnvironment =>
                write!(f, "Attempt to pop the only remaining environment in the stack"),
        }
    }
}

impl error::Error for EnvironmentError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { None }
}

#[derive(Debug)]
struct Frame {
    id: usize,
    parent: Option<usize>,
    bindings: HashMap<String, Reference>,
    functions: HashMap<String, FunctionReference>,
}

impl Frame {
    fn top() -> Frame {
        Frame { id: 0, bindings: HashMap::new(), functions: HashMap::new(), parent: None }
    }

    fn new(id: usize, parent: usize) -> Frame {
        Frame { id, bindings: HashMap::new(), functions: HashMap::new(), parent: Some(parent) }
    }

    fn contains_binding(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    fn contains_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    fn lookup_binding(&self, name: &str) -> LookupResult {
        let result = self.bindings.get(name);
        match result {
            Some(reference) => LookupResult::Found(reference),
            None => match self.parent {
                Some(id) => LookupResult::KeepLooking(id),
                None => LookupResult::NotFound,
            }
        }
    }

    fn lookup_function<'a>(&'a self, name: &'a str) -> LookupFunctionResult {
        let result = self.functions.get(name);
        match result {
            Some(reference) => LookupFunctionResult::Found(reference),
            None => match self.parent {
                Some(id) => LookupFunctionResult::KeepLooking(id),
                None => LookupFunctionResult::NotFound,
            }
        }
    }

    fn register_binding(&mut self, name: String, object: Reference) -> Result<(), EnvironmentError> {
        if self.contains_binding(&name) { return already_defined_error!(name) }

        let result = self.bindings.insert(name, object);
        assert!(result.is_none());

        all_good!()
    }

    fn register_function(&mut self, name: String, object: FunctionReference) -> Result<(), EnvironmentError> {
        if self.contains_function(&name) { return already_defined_error!(name) }

        let result = self.functions.insert(name, object);
        assert!(result.is_none());

        all_good!()
    }

    fn change_binding(&mut self, name: String, object: Reference) -> Result<(), EnvironmentError> {
        if !self.contains_binding(&name) { return undefined_error!(name) }

        let result = self.bindings.insert(name, object);
        assert!(result.is_some());

        all_good!()
    }
}

enum LookupResult<'a> {
    Found(&'a Reference),
    KeepLooking(usize),
    NotFound
}

enum LookupFunctionResult<'a> {
    Found(&'a FunctionReference),
    KeepLooking(usize),
    NotFound
}

#[derive(Debug)]
pub struct EnvironmentStack {
    id_sequence: usize,
    frames: Vec<Frame>,
}

impl EnvironmentStack {
    pub fn new() -> EnvironmentStack {
        EnvironmentStack { id_sequence: 1, frames: vec!(Frame::top()) }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.id_sequence;
        self.id_sequence += 1;
        id
    }

    pub fn reclaim_id(&mut self, id: usize) {
        if id != self.id_sequence - 1 {
            panic!("Attempt to reclaim non-consecutive id");
        }
        self.id_sequence = id;
    }

    pub fn lookup_binding<'a> (&'a self, name: &'a str) -> Result<&'a Reference, EnvironmentError> {
        let mut cursor = self.frames.last().expect("Invalid stack: empty").id;
        let mut result = not_found_error!(name);

        loop {
            let frame = self.frames.get(cursor)
                                   .expect(&format!("Invalid stack frame: {}", cursor));

            match frame.lookup_binding(name) {
                LookupResult::Found(reference) => { result = Ok(reference); break    },
                LookupResult::KeepLooking(id) =>  { cursor = id;            continue },
                LookupResult::NotFound =>         {                         break    },
            }
        };
        result
    }

    pub fn lookup_function<'a> (&'a self, name: &'a str) -> Result<&'a FunctionReference, EnvironmentError> {
        let mut cursor = self.frames.last().expect("Invalid stack: empty").id;
        let mut result = not_found_error!(name);

        loop {
            let frame = self.frames.get(cursor)
                .expect(&format!("Invalid stack frame: {}", cursor));

            match frame.lookup_function(name) {
                LookupFunctionResult::Found(reference) => { result = Ok(reference); break    },
                LookupFunctionResult::KeepLooking(id) =>  { cursor = id;            continue },
                LookupFunctionResult::NotFound =>         {                         break    },
            }
        };
        result
    }

    pub fn register_binding(&mut self, name: String, object: Reference) -> Result<(), EnvironmentError>  {
        self.frames.last_mut().unwrap().register_binding(name, object)
    }

    pub fn register_function(&mut self, name: String, object: FunctionReference) -> Result<(), EnvironmentError>  {
        self.frames.last_mut().unwrap().register_function(name, object)
    }

    pub fn change_binding(&mut self, name: String, object: Reference) -> Result<(), EnvironmentError>  {
        self.frames.last_mut().unwrap().change_binding(name, object)
    }

    pub fn add_soft_frame(&mut self) {
        let parent = self.frames.last().expect("Invalid stack: empty").id;
        let id = self.next_id();
        self.frames.push(Frame::new(id, parent));
    }

    pub fn add_hard_frame(&mut self) {
        let parent = self.frames.first().expect("Invalid stack: empty").id;
        let id = self.next_id();
        self.frames.push(Frame::new(id, parent));
    }

    pub fn remove_frame(&mut self) {
        if self.frames.len() == 1 {
            panic!("Attempt to pop from stack without pushing")
        }

        match self.frames.pop() {
            Some(frame) => self.reclaim_id(frame.id),
            None => panic!("Attempt to pop from empty stack")
        }
    }
}