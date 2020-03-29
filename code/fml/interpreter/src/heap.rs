use std::collections::HashMap;
use crate::ast::AST;

// https://dev.to/deciduously/rust-your-own-lisp-50an
// https://github.com/kenpratt/rusty_scheme

#[derive(Debug)]
pub struct Instance {
    extends: Option<Reference>,
    fields: HashMap<String, Reference>,
    methods: HashMap<String, FunctionReference>
}

impl Instance {
    pub fn empty() -> Instance {
        Instance {
            extends: None,
            fields: HashMap::new(),
            methods: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<AST>,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>, body: Box<AST>) -> Function {
        Function { name, parameters, body }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub enum FunctionReference { // i guess I'll split it off?
    Function(u64),
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub enum Reference {
    Unit,
    Object(u64),
    Integer(i32),
    Boolean(bool),
    //String(&'obj str),
}

pub struct Memory {
    sequence: ReferenceSequence,
    objects: HashMap<Reference, Instance>,
    functions: HashMap<FunctionReference, Function>,
}

struct ReferenceSequence(u64);
impl ReferenceSequence {
    fn next_object(&mut self) -> Reference {
        let n = self.0;
        self.0 += 1;
        Reference::Object(n)
    }
    fn next_function(&mut self) -> FunctionReference {
        let n = self.0;
        self.0 += 1;
        FunctionReference::Function(n)
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            sequence: ReferenceSequence(0),
            objects: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn contains_object(&self, reference: &Reference) -> bool {
        self.objects.contains_key(reference)
    }

    pub fn contains_function(&self, reference: &FunctionReference) -> bool {
        self.functions.contains_key(reference)
    }

    pub fn get_object(&self, reference: &Reference) -> Option<&Instance> {
        self.objects.get(reference)
    }

    pub fn get_function(&self, reference: &FunctionReference) -> Option<&Function> {
        self.functions.get(reference)
    }

    pub fn get_object_mut(&mut self, reference: &Reference) -> Option<&mut Instance> {
        self.objects.get_mut(reference)
    }

    pub fn get_function_mut(&mut self, reference: &FunctionReference) -> Option<&mut Function> {
        self.functions.get_mut(reference)
    }

    pub fn put_object(&mut self, object: Instance) -> Reference {
        let reference = self.sequence.next_object();
        self.objects.insert(reference, object);
        reference
    }

    pub fn put_function(&mut self, function: Function) -> FunctionReference {
        let reference = self.sequence.next_function();
        self.functions.insert(reference, function);
        reference
    }
}

