//use crate::ast::AST;
//use crate::environment::Environment;
//use crate::environment::Object;

//fn evaluate(environment: &mut Environment, expression: AST) -> Result<Object, &str> {
//    match expression {
//        AST::LocalDefinition {identifier, value} => {
//            Err("Undefined")
//        },
//        //AST::LocalMutation {identifier, value}   => mutate_local(environment, identifier, value),
//        AST::Identifier(token) => {
//            environment.lookup_local(identifier_token)
//        },
//
//        _ => Err("Not implemented")
//    }
//}