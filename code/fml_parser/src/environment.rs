use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Object {
    Unit,
    Reference(u64),
}

pub struct ObjectInstance;

//type ObjectInstances = HashMap<u64, ObjectInstance>;

#[derive(Debug)]
pub struct Environment<'env> {
    parent: Option<Box<&'env Environment<'env>>>,
    bindings: HashMap<String, Object>,
    //functions: HashMap<String, Object>,
}

impl Environment<'_> {
    pub fn child<'env> (&'env mut self) -> Environment<'env> {
        Environment {
            parent: Some(Box::new(self)),
            bindings: HashMap::new(),
            //functions: HashMap::new(),
        }
    }

    pub fn new<'env>() -> Environment<'env> {
        Environment {
            parent: None,
            bindings: HashMap::new(),
            //functions: HashMap::new(),
        }
    }

    pub fn binding_is_defined(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    pub fn define_binding(&mut self, name: &str, value: Object) -> Result<(), String> {
        if self.binding_is_defined(name) {
            return Err(format!("Attempt to define an already-defined local variable {}", name))
        }

        self.bindings.insert(name.to_string(), value);
        Ok(())
    }

    pub fn redefine_binding(&mut self, name: &str, value: Object) -> Result<(), String> {
        if !self.binding_is_defined(name) {
            return Err(format!("Attempt to redefine an undefined local variable {}", name))
        }

        self.bindings.insert(name.to_string(), value);
        Ok(())
    }

    pub fn lookup_binding(&self, name: &str) -> Result<Object, String>{
        if self.binding_is_defined(name) {
            return Ok(self.bindings.get(name).unwrap().to_owned())
        }

        match &self.parent {
            Some(parent) => (*parent).lookup_binding(name),
            None => Err(format!("Attempt to redefine an undefined local variable {}", name))
        }
    }
}