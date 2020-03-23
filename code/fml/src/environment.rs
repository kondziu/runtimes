use std::collections::HashMap;
//use std::error;
//use std::fmt;

use crate::objects::Object;
use std::ops::Deref;

#[derive(Debug)]
pub struct Environment<'env> {
    locals: HashMap<String, Object>,
    functions: HashMap<String, Object>,
    parent: Option<Box<&'env Environment<'env>>>,
}

impl Environment<'_> {
    pub fn new<'env> (parent: &'env Environment) -> Environment<'env> {
        Environment {
            locals: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn empty<'env>() -> Environment<'env> {
        Environment {
            locals: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }

    pub fn local_is_defined(&self, name: &str) -> bool {
        self.locals.contains_key(name)
    }

    pub fn define_local(&mut self, name: &str, value: Object) -> Result<(), String> {
        if self.local_is_defined(name) {
            return Err(format!("Attempt to define an already-defined local variable {}", name))
        }

        self.locals.insert(name.to_string(), value);
        Ok(())
    }

    pub fn redefine_local(&mut self, name: &str, value: Object) -> Result<(), String> {
        if !self.local_is_defined(name) {
            return Err(format!("Attempt to redefine an undefined local variable {}", name))
        }

        self.locals.insert(name.to_string(), value);
        Ok(())
    }

    pub fn lookup_local(&self, name: &str) -> Result<Object, String>{
        if self.local_is_defined(name) {
            return Ok(self.locals.get(name).unwrap().to_owned())
        }

        match &self.parent {
            Some(parent) => (*parent).lookup_local(name),
            None => Err(format!("Attempt to redefine an undefined local variable {}", name))
        }
    }
}