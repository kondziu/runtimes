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
macro_rules! last_environment_error {() => { Err(EnvironmentError::LastEnvironment) }}
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
    id: u32,
    bindings: HashMap<String, Reference>,
    functions: HashMap<String, FunctionReference>,
}

impl Frame {
    fn new(id: u32) -> Frame {
        Frame { id, bindings: HashMap::new(), functions: HashMap::new() }
    }

    fn contains_binding(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    fn contains_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    fn lookup_binding<'a>(&'a self, name: &'a str) -> Option<&'a Reference> {
        self.bindings.get(name)
    }

    fn lookup_function<'a>(&'a self, name: &'a str) -> Option<&'a FunctionReference> {
        self.functions.get(name)
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

#[derive(Debug)]
pub struct EnvironmentStack {
    id_sequence: u32,
    frames: Vec<Frame>,
}

impl EnvironmentStack {
    pub fn new() -> EnvironmentStack {
        let mut stack = EnvironmentStack { id_sequence: 0, frames: Vec::new() };
        stack.add_new_level();
        stack
    }

    pub fn contains_binding(&self, name: &str) -> bool {
        self.frames
            .iter().rev()
            .any(|e| e.contains_binding(name))
    }

    pub fn contains_function(&self, name: &str) -> bool {
        self.frames
            .iter().rev()
            .any(|e| e.contains_function(name))
    }

    pub fn lookup_binding<'a> (&'a self, name: &'a str) -> Result<&'a Reference, EnvironmentError> {
        let result = self.frames
            .iter().rev()
            .find_map(|e| e.lookup_binding(name));
        match result {
            None => not_found_error!(name),
            Some(object) => Ok(object)
        }
    }

    pub fn lookup_function<'a> (&'a self, name: &'a str) -> Result<&'a FunctionReference, EnvironmentError> {
        let result = self.frames
            .iter().rev()
            .find_map(|e| e.lookup_function(name));
        match result {
            None => not_found_error!(name),
            Some(object) => Ok(object)
        }
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

    pub fn add_new_level(&mut self) {
        let id = self.id_sequence;
        self.id_sequence += 1;
        self.frames.push(Frame::new(id));
    }

    pub fn remove_newest_level(&mut self) -> Result<(), EnvironmentError> {
        if self.frames.len() == 1 { return last_environment_error!() }

        let result = self.frames.pop();
        assert!(result.is_some());

        all_good!()
    }
}










//type ObjectInstances = HashMap<u64, ObjectInstance>;

//#[derive(Debug)]
//struct FunctionDefinition<'ast> {
//    name: &'ast str,
//    parameters: Vec<&'ast str>,
//    body: u64,
//}




//
//#[derive(Debug)]
//pub struct Environment<'me, 'parent> //where 'parent: 'me
//{
//    parent: Option<&'parent Environment<'parent: 'me, 'parent>>,
//    bindings: HashMap<String, Object>,
//    //functions: HashMap<String, FunctionDefinition<>>,
//}
//
//#[derive(Debug, Clone, PartialEq)]
//pub enum BindingError {
//    BindingUndefined (String),
//    BindingAlreadyDefined (String),
//    BindingNotFound (String),
//}
//
//type BindingResult<'env> = Result<(), BindingError>;
//type Binding<'env> = Result<Object, BindingError>;
//
//impl BindingError {
//    fn undefined(binding: &str) -> BindingResult {
//        Err(BindingError::BindingUndefined(binding.to_string()))
//    }
//
//    fn already_defined(binding: &str) -> BindingResult {
//        Err(BindingError::BindingAlreadyDefined(binding.to_string()))
//    }
//
//    fn not_found(binding: &str) -> Binding {
//        Err(BindingError::BindingNotFound(binding.to_string()))
//    }
//}
//

//
//impl Environment<'_> {
//    pub fn child(self) -> Environment {
//        Environment {
//            parent: Some(Box::new(self)),
//            bindings: HashMap::new(),
//            //functions: HashMap::new(),
//        }
//    }
//
//    pub fn new() -> Environment {
//        Environment {
//            parent: None,
//            bindings: HashMap::new(),
//            //functions: HashMap::new(),
//        }
//    }
//
//    pub fn binding_is_defined(&self, name: &str) -> bool {
//        self.bindings.contains_key(name)
//    }
//
//    pub fn define_binding(&mut self, name: &str, value: Object) -> BindingResult {
//        if self.binding_is_defined(name) {
//            return BindingError::already_defined(name);
//        }
//
//        self.bindings.insert(name.to_string(), value);
//        Ok(())
//    }
//
//    pub fn redefine_binding(&mut self, name: &str, value: Object) -> BindingResult {
//        if !self.binding_is_defined(name) {
//            return BindingError::undefined(name);
//        }
//
//        self.bindings.insert(name.to_string(), value);
//        Ok(())
//    }
//
//    pub fn lookup_binding(&self, name: &str) -> Binding {
//        if self.binding_is_defined(name) {
//            return Ok(self.bindings.get(name).unwrap().to_owned())
//        }
//
//        match &self.parent {
//            Some(parent) => (*parent).lookup_binding(name),
//            None => BindingError::not_found(name)
//        }
//    }
//
////    pub fn function_is_defined(&self, name: &str) -> bool {
////        self.functions.contains_key(name)
////    }
//
//    pub fn define_function<'ast> (&mut self, name: &'ast str, parameters: Vec<&'ast str>, body_reference: u64) -> BindingResult {
////        if self.function_is_defined(name) {
////            return BindingError::already_defined(name);
////        }
//
//        let definition = FunctionDefinition { name, parameters, body: body_reference };
//        //self.functions.insert(name.to_string(), definition);
//        Ok(())
//    }
//}