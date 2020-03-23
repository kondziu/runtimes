use std::collections::HashMap;
use std::error;
use std::fmt;

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

#[derive(Debug, Clone, PartialEq)]
pub enum BindingError {
    BindingUndefined (String),
    BindingAlreadyDefined (String),
    BindingNotFound (String),
}

type BindingResult<'e> = Result<(), BindingError>;
type Binding<'e> = Result<Object, BindingError>;

impl BindingError {
    fn undefined(binding: &str) -> BindingResult {
        Err(BindingError::BindingUndefined(binding.to_string()))
    }

    fn already_defined(binding: &str) -> BindingResult {
        Err(BindingError::BindingAlreadyDefined(binding.to_string()))
    }

    fn not_found(binding: &str) -> Binding {
        Err(BindingError::BindingNotFound(binding.to_string()))
    }
}

impl fmt::Display for BindingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BindingError::BindingUndefined(binding) =>
                write!(f, "Attempt to redefine a binding that has not previously been defined {}", binding),
            BindingError::BindingAlreadyDefined(binding) =>
                write!(f, "Attempt to define a binding that has already been defined {}", binding),
            BindingError::BindingNotFound(binding) =>
                write!(f, "Attempt to look up a binding that not been defined {}", binding),
        }
    }
}

impl error::Error for BindingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { None }
}

impl Environment<'_> {
    pub fn child(&mut self) -> Environment {
        Environment {
            parent: Some(Box::new(self)),
            bindings: HashMap::new(),
            //functions: HashMap::new(),
        }
    }

    pub fn new<'e>() -> Environment<'e> {
        Environment {
            parent: None,
            bindings: HashMap::new(),
            //functions: HashMap::new(),
        }
    }

    pub fn binding_is_defined(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    pub fn define_binding(&mut self, name: &str, value: Object) -> BindingResult {
        if self.binding_is_defined(name) {
            return BindingError::already_defined(name);
        }

        self.bindings.insert(name.to_string(), value);
        Ok(())
    }

    pub fn redefine_binding(&mut self, name: &str, value: Object) -> BindingResult {
        if !self.binding_is_defined(name) {
            return BindingError::undefined(name);
        }

        self.bindings.insert(name.to_string(), value);
        Ok(())
    }

    pub fn lookup_binding(&self, name: &str) -> Binding {
        if self.binding_is_defined(name) {
            return Ok(self.bindings.get(name).unwrap().to_owned())
        }

        match &self.parent {
            Some(parent) => (*parent).lookup_binding(name),
            None => BindingError::not_found(name)
        }
    }
}