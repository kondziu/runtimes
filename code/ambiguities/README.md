# Disambiguating if-then-else in LALPOP

 - I wrote out the ambiguous grammar. 
 - Then I fixed it by introducing limited rules in the sort of "classic" LALR disambiguation fashion. 
 - Then I wrote an equivalent grammar using macros like in the link. 

# The problem

The original (ambiguous) grammar is this:

```
use crate::ast::AST;
grammar;

pub TopLevel: AST<'input> = {
    Expression => <>,
}

Expression: AST<'input> = {
    Condition => <>,
    Conditional => <>,
}

Conditional: AST<'input> = {
    "if" <condition: Condition> "then" <consequent: Expression> <alternative: ("else" <Expression>)?> =>
        AST::Parent("if-then-else", vec!(Box::new(condition),
                                         Box::new(consequent),
                                         Box::new(alternative))),
}

Condition: AST<'input> = {
    Number => <>,
    Identifier => <>,
    Boolean => <>,
}

Boolean: AST<'input> = {
    "true" => AST::Leaf(<>),
    "false" => AST::Leaf(<>),
}

Number: AST<'input> = {
    r"[0-9]+" => AST::Leaf(<>),
}

Identifier: AST<'input> = {
    r"[A-Za-z]+" => AST::Leaf(<>),
}
```

It yields this (expected) error:

```
/home/.../ambiguities/src/problem.lalrpop:20:5: 23:64: Ambiguous grammar detected

  The following symbols can be reduced in two ways:
    "if" Condition "then" "if" Condition "then" Expression "else" Expression

  They could be reduced like so:
    "if" Condition "then" "if" Condition "then" Expression "else" Expression
    │                     ├─Conditional──────────────────┤                 │
    │                     └─Expression───────────────────┘                 │
    └─Conditional──────────────────────────────────────────────────────────┘

  Alternatively, they could be reduced like so:
    "if" Condition "then" "if" Condition "then" Expression "else" Expression
    │                     ├─Conditional────────────────────────────────────┤
    │                     └─Expression─────────────────────────────────────┤
    └─Conditional──────────────────────────────────────────────────────────┘

  LALRPOP does not yet support ambiguous grammars. See the LALRPOP manual for advice on making your grammar
  unambiguous.

```

# Textbook disambiguation

I re-wrote the grammar above to remove the ambiguity as follows. I added a `LimitedExpression` rule that contains a 
`LimitedConditional` rule, which precludes short ifs. Then, the consequent of `Conditional` expressions and both the 
consequent and the altertnative of `LimitedConditional` statements are limited to being `LimitedExpressions`.

```
use crate::ast::AST;
grammar;

pub TopLevel: AST<'input> = // samew as before
Expression: AST<'input> = // same as before

Conditional: AST<'input> = {
    "if" <condition: Condition> "then" <consequent: LimitedExpression> "else" <alternative: Expression> =>
        AST::Parent("if-then-else", vec!(Box::new(condition),
                                         Box::new(consequent),
                                         Box::new(alternative))),

    "if" <condition: Condition> "then" <consequent: Expression> =>
        AST::Parent("if-then", vec!(Box::new(condition),
                                    Box::new(consequent))),
}

LimitedExpression: AST<'input> = {
    Condition => <>,
    LimitedConditional => <>,
}

LimitedConditional: AST<'input> = {
    "if" <condition: Condition> "then" <consequent: LimitedExpression> "else" <alternative: LimitedExpression> =>
        AST::Parent("if-then-else", vec!(Box::new(condition),
                                         Box::new(consequent),
                                         Box::new(alternative))),
}

// Condition, Boolean, Number, Identifier same as before
```

# Using Macros in LALRPOP

The solution above can be made more succinct in LALRPOP using macros. I added a parameter called openness to 
`Expression` and `Conditional` rules. `Expression<"open">` is supposed to be the equivalent of `Expression` from the 
previous grammar, and `Expression<"closed">` is supposed to be the equivalent of `LimitedExpression`). By analogy 
`Conditional<"open">` is `Conditional` from before and `Conditional<"closed">` is meant to be `LimitedConditional`. 
I ended up with this:

```
use crate::ast::AST;
grammar;

pub TopLevel: AST<'input> = {
    Expression<"open"> => <>,
}

Expression<openness>: AST<'input> = {
    Condition => <>,
    Conditional<openness> => <>,
}

Conditional<openness>: AST<'input> = {
    "if" <condition: Condition> "then" <consequent: Expression<"closed">> "else" <alternative: Expression<openness>> =>
        AST::Parent("if-then-else", vec!(Box::new(condition),
                                         Box::new(consequent),
                                         Box::new(alternative))),

    "if" <condition: Condition> "then" <consequent: Expression<"open">> if openness == "open" =>
        AST::Parent("if-then", vec!(Box::new(condition),
 

// Condition, Boolean, Number, Identifier without changes
```