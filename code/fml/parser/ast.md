# AST

## Operators

Operators are a separate type form the AST defined by an enum containing:

  - `Multiplication`
  - `Division`
  - `Module`
  - `Addition`
  - `Subtraction`
  - `Inequality`
  - `Equality`
  - `Less`
  - `LessEqual`
  - `Greater`
  - `GreaterEqual`
  - `Disjunction`
  - `Conjunction`

## String literal

Represents string literals, carries a string token. 
Usable only as the first argument of `print`.

Type: `String(&str)`    
Arguments:
   - (anonymous) string

FML: `"get in the robot, Shinji"` *(won't parse)*  
LISP: `(String . "get in the robot, Shinji")`  
JSON: `{"String":"get in the robot, Shinji"}`

## Number literal

Represents integer literals, carries an integer value.

Type: `Number(i32)`  
Argments:
   - (anonymous) signed 32bit integer

FML: `42`  
LISP: `(Number . 42)`  
JSON: `{"Number":42}`

## Boolean literal

Represents boolean literals, carries `true` or `false`.

Type: `Boolean(bool)`  
Arguments: 
   - (anonymous) boolean
    
FML: `true`  
LISP: `(Boolean . #t)`  
JSON: `{"Boolean":true}`
    
## Unit literal

Represents an object that acts as unit or null.

Type: `Unit`  
FML: `null`  
LISP: `Unit`  
JSON: `"Unit"`

## Identifier
    
A node that represents a simple identifier, carries the identifier token. 
Identifiers start with a letter and contain letters, digits and underscore. 
This specifically does not include operators.

Type: `Identifier(&str)`    
Arguments:
   - (anonymous) string

FML: `x`  
LISP: `(Identifier . "x")`  
JSON: `{"Identifier":"x"}`  

## Variable definition

Represents a definition of a local variable: an assignment of a value to an identifier.

Type: `LocalDefinition { identifier: AST, value: AST }`    
Arguments:
   - `identifier` is always an `Identifier`
   - `value` is any AST

FML: `let x = 1`  
LISP: `(LocalDefinition (identifier Identifier . "x") (value Number . 1))`  
JSON: `{"LocalDefinition":{"identifier":{"Identifier":"x"},"value":{"Number":1}}}` 

## Array definition

Represents a definition of an array with a specified size and initial value of all elements.

Type: `ArrayDefinition { size: AST, value: AST }`  
Arguments:
   - `size` is any AST
   - `value` is any AST
   
FML: `array(100, 42)`  
LISP: `(ArrayDefinition (size Number . 100) (value Number . 42))`  
JSON: `{"ArrayDefinition":{"size":{"Number":100},"value":{"Number":42}}}`

## Object definition

Represents the definition of an object instance.

Type: `ObjectDefinition { extends: Option<AST>, parameters: Vec<AST>, members: Vec<AST> }`
Arguments:
   - `extends` is any `Some(AST)` or nothing
   - `parameters` is a list of `Identifiers`
   - `members` is a list of elements of types 
        - `LocalDefinition`, 
        - `FunctionDefinition`, 
        - `OperatorDefinition`.       
        
FML: 
```fml
object (v) extends s 
begin 
  let value = v; 
  function set (x) <- this.value <- v;
  function get () <- this.value
end
``` 

LISP:
```lisp
(ObjectDefinition (extends (Identifier . "s")) 
                  (parameters (Identifier . "v")) 
                  (members 
                      (LocalDefinition 
                            (identifier Identifier . "value") 
                            (value Identifier . "v")) 
                      (FunctionDefinition 
                            (name Identifier . "set") 
                            (parameters (Identifier . "x"))
                            (body FieldMutation 
                                (field_path FieldAccess 
                                    (object Identifier . "this") 
                                    (field Identifier . "value")) 
                                (value Identifier . "v"))) 
                      (FunctionDefinition 
                            (name Identifier . "get") 
                            (parameters) 
                            (body FieldAccess 
                                    (object Identifier . "this") 
                                    (field Identifier . "value")))))
```

JSON:
```json
{"ObjectDefinition":{
    "extends":{"Identifier":"s"},
    "parameters":[{"Identifier":"v"}],
    "members":[
        {"LocalDefinition":{
            "identifier":{"Identifier":"value"},
            "value":{"Identifier":"v"}}},
        {"FunctionDefinition":{
            "name":{"Identifier":"set"},
            "parameters":[{"Identifier":"x"}],
            "body":{
                "FieldMutation":{"field_path":{
                  "FieldAccess":{
                      "object":{"Identifier":"this"}, "field":{"Identifier":"value"}}},
                      "value":{"Identifier":"v"}}}}},
        {"FunctionDefinition":{
            "name":{"Identifier":"get"},
            "parameters":[],
            "body":{
                "FieldAccess":{
                    "object":{"Identifier":"this"},
                    "field":{"Identifier":"value"}}}}}]}}
```

## Local Mutation

Represents the mutation of a local variable. 
Assigns a new value to the variable. 

Type: `LocalMutation { identifier: AST, value: AST },`
Arguments:
  - `identifier` is an `Identifier`
  - `value` is any `AST`
  
FML: `x <- 1`  
LISP: `(LocalMutation (identifier Identifier . "x") (value Number . 1))`  
JSON: `{"LocalMutation":{"identifier":{"Identifier":"x"},"value":{"Number":1}}}` 
   
## Field Mutation

Represents the mutation of an object member field.
Assigns a new value to the field.

Type: `FieldMutation { field_path: AST, value: AST }`
Arguments: 
   - `field_path` is a tree consisting of `FieldAccess` nodes,
   - `value` is any `AST`
   
FML: `x.y <- 1`     
LISP: 
```lisp
(FieldMutation 
    (field_path FieldAccess 
        (object Identifier . "x") 
        (field Identifier . "y")) 
    (value Number . 1))
```  
JSON: 
```json
{"FieldMutation":{
  "field_path":{
    "FieldAccess":{
      "object":{"Identifier":"x"},
      "field":{"Identifier":"y"}}},
      "value":{"Number":1}}}
```

## Array Mutation

Represents the mutation of an array member.
Assigns a new value to the contents of the array under a specific index.

Type: `ArrayMutation { array: AST, value: AST },`  
Arguments: 
   - `array` 
   - `value` is any AST
   
FML: `x[0] <- x[1]`  
LISP: 

```lisp
(ArrayMutation 
    (array ArrayAccess (array Identifier . "x") 
    (index Number . 0)) 
    (value ArrayAccess 
        (array Identifier . "x") 
        (index Number . 1)))
```
  
JSON: 
```json
  {"ArrayMutation":{
    "array":{"ArrayAccess":{
      "array":{"Identifier":"x"},
      "index":{"Number":0}}},
      "value":{"ArrayAccess":{
        "array":{"Identifier":"x"},
        "index":{"Number":1}}}}}
```   
    
## Function Definition

Represents a function definition with a signature and a body. 
The function is named.

Type: `FunctionDefinition { name: AST, parameters: Vec<AST>, body: AST }`
Arguments:
   - `name` is an `Identifier`
   - `parameters` is a vector of `Identifiers`
   - `body` is any AST
    
FML: `function inc (x) <- x + 1`

LISP:
```lisp
(FunctionDefinition 
    (name Identifier . "inc") 
    (parameters (Identifier . "x")) 
    (body Operation 
        (operator . Addition) 
        (left Identifier . "x") 
        (right Number . 1)))
```
JSON:     
```json
{"FunctionDefinition":{                           
    "name":{"Identifier":"inc"},
    "parameters":[{"Identifier":"x"}],
    "body":{
      "Operation":{
        "operator":"Addition",
        "left":{"Identifier":"x"},
        "right":{"Number":1}}}}}
```

## Operation definition

Defines a method that can be used as an infix operator.

Type: `OperatorDefinition { operator: Operator, parameters: Vec<AST>, body: AST }`
Arguments:
   - `operator` is an `Operator`
   - `parameters` is a vector of `Identifiers`
   - `body` is any AST

FML: `function + (x) <- (this.value) + x` (*won't parse*)

LISP:
```lisp
(OperatorDefinition 
    (operator . Addition) 
    (parameters (Identifier . "x")) 
    (body Operation 
        (operator . Addition) 
        (left FieldAccess 
            (object Identifier . "this") 
            (field Identifier . "value")) 
        (right Identifier . "x"))))
```

JSON:
```json
{"OperatorDefinition":{
  "operator":"Addition",
  "parameters":[{"Identifier":"x"}],
  "body":{"Operation":{
    "operator":"Addition",
    "left":{"FieldAccess":{
      "object":{"Identifier":"this"},
      "field":{"Identifier":"value"}}},
    "right":{"Identifier":"x"}}}}}
```

## Function application

Represents a function application with a list of arguments.

Type: `FunctionApplication { function: AST, arguments: Vec<AST> }`  
Arguments:
  - `function` is an `Identifier`
  - `arguments` a list of `AST`s representing expressions
    
FML: `f(1)`  
LISP: `(FunctionApplication (function Identifier . "f") (arguments (Number . 1)))`  
JSON: `{"FunctionApplication":{"function":{"Identifier":"f"},"arguments":[{"Number":1}]}}`

## Method call    
    
Represents a call to an object's method.

Type: `MethodCall { method_path: <AST>, arguments: Vec<AST> }`  
Arguments:
   - `method_path` is an expression that evaluates to a `FieldAccess` or an `OperatorAccess`
   - `arguments` is a list of `AST`s representing expressions
   
FML: `x.f(1)`  
LISP: `(MethodCall (method_path FieldAccess (object Identifier . "x") (field Identifier . "f")) (arguments (Number . 1)))`  
JSON: `{"MethodCall":{"method_path":{"FieldAccess":{"object":{"Identifier":"x"},"field":{"Identifier":"f"}}},"arguments":[{"Number":1}]}}`    

## Print call

Represents an execution of the built-in print function.

Type: `Print { format: AST, arguments: Vec<AST> }`  
Arguments:
   - `format` is a `String` where `~` is a placeholder an argument
   - `arguments` is a list of `AST`s representing expressions
   
FML: `print("x = ~", x)`  
LISP: `(Print (format String . "x = ~") (arguments (Identifier . "x")))`  
JSON: `{"Print":{"format":{"String":"x = ~"},"arguments":[{"Identifier":"x"}]}}`    

## Field access

Represents an access to an object's field.

Type: `FieldAccess { object: Box<AST<'ast>>, field: Box<AST<'ast>> }`  
Arguments: 
    - `object` is a tree of `FieldAccess` nodes
    - `field` is an `Identifier`
    
FML: `obj.a`  
LISP: `(FieldAccess (object Identifier . "obj") (field Identifier . "a"))`  
JSON: `{"FieldAccess":{"object":{"Identifier":"obj"},"field":{"Identifier":"a"}}}`
    
## Operator access

Represents an access to an object's operator method. 
These can only be seen in method calls.

Type: `OperatorAccess { object: AST, operator: Operator }`  
Arguments:
   - `object` is a tree of `FieldAccess` nodes
   - `operator` is an `Operator`
   
FML: `x.+` (*won't parse*)  
LISP: `(OperatorAccess (object Identifier . "x") (operator . Addition))`  
JSON: `{"OperatorAccess":{"object":{"Identifier":"x"},"operator":"Addition"}}`  

## Array access

Represents an access to an element in an array.

Type: `ArrayAccess { array: AST, index: AST }`  
Arguments:
   - `array` is an expression `AST` that evaluates to an array object
   - `index` is an expression `AST` that evaluates to a number 

FML: `a[i]`  
LISP: `(ArrayAccess (array Identifier . "a") (index Identifier . "i"))`  
JSON: `{"ArrayAccess":{"array":{"Identifier":"a"},"index":{"Identifier":"i"}}}` 

## Expression block

Represents a sequence of expressions.

Type: `Block(Vec<AST))`  
Arguments:
  - (anonymous) a vector of `AST`s
  
FML:

```fml
begin
    let x = 1;
    x + 2
end
```

LISP:

```lisp
(Block 
    (LocalDefinition 
        (identifier Identifier . "x") 
        (value Number . 1)) 
    (Operation 
        (operator . Addition) 
        (left Identifier . "x") 
        (right Number . 2)))
```

JSON:

```json
{"Block":[
  {"LocalDefinition":{
    "identifier":{"Identifier":"x"},
    "value":{"Number":1}}},
  {"Operation":{
    "operator":"Addition",
    "left":{"Identifier":"x"},
    "right":{"Number":2}}}]}
```

## Infix operator application

Represents a call to a method that works as an infix operation.

Type: `Operation { operator: Operator, left: AST, right: AST },  
Arguments:
   - `operator` is an `Operator`
   - `left` and `right` contain any `AST`

FML: a + b  
LISP: `(Operation (operator . Addition) (left Identifier . "a") (right Identifier . "b"))`  
JSON: `{"Operation":{"operator":"Addition","left":{"Identifier":"a"},"right":{"Identifier":"b"}}}`

## Loop

Represents a loop control construct.

Type: `Loop { condition: AST, body: AST }`  
Arguments:
  - `condition` is any `AST`
  - `body` is any `AST`
FML: `while true do print(".")`  
LISP: `(Loop (condition Boolean . #t) (body Print (format String . ".") (arguments)))`  
JSON: `{"Loop":{"condition":{"Boolean":true},"body":{"Print":{"format":{"String":"."},"arguments":[]}}}}`

## Conditional statement

Represents a conditional statement, with a possibly missing alternative clause.

Type: `Conditional { condition: AST, consequent: AST, alternative: AST }`  
Arguments: 
  - `condition` is any `AST`
  - `consequent` is any `AST`
  - `alternative` is any `AST`, but if the `else` clause is missing, it is `Unit`
  
 FML: `if true then 0 else -1`  
 LISP: `(Conditional (condition Boolean . #t) (consequent Number . 0) (alternative Number . -1))`  
 JSON: `{"Conditional":{"condition":{"Boolean":true},"consequent":{"Number":0},"alternative":{"Number":-1}}}`  
