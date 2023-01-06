extern crate pest;
#[macro_use]
extern crate pest_derive;

mod semantic_analysis;
pub use crate::semantic_analysis::*;

mod parsing;
pub use crate::parsing::*;


fn serialize(node: &AstNode) -> String {
    match node {
        AstNode::Function { name , body} => format!("Function: {}\n{}", serialize(&*name), serialize(&*body)),
        AstNode::Name(name) => name.to_string(),
        AstNode::Block(vector) => {
                let mut expressions = "".to_string();
                for expr in vector {
                expressions.push_str(&serialize(expr));
            }
            return expressions;
        }
        AstNode::MutationExpression { target, verb, expr } => {
            let lhs = serialize(&**target);
            let rhs = serialize(&**expr);
            let sverb = serialize_mutation_verb(verb);
            format!("{lhs} mutated as \"{sverb}\" by {rhs}\n")
        }
        AstNode::Integer(i) => i.to_string(),
        _ => unreachable!("AST node serialization error"),
    }
}

fn serialize_mutation_verb(verb: &MutationVerb) -> String {
    match verb {
        MutationVerb::Equals => "=".to_string(),
        MutationVerb::PlusEquals => "+=".to_string(),
        _ => unreachable!("serialization of invalid mutation verb")
    }
}

fn main() {
    let unparsed_file = std::fs::read_to_string("src.txt").expect("cannot read M file");
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");

    let str = serialize(&astnode[0]);
/* 
    let test = SymbolTable {table: HashMap::from([(
        TableKey{
            name: "Test".to_string(),
            scope_key: ScopeKey { key: vec![0, 0]},
        },
        Entry::Variable {
            data_type: "TMP".to_string()
        }
    ),
    (
        TableKey{
            name: "Test".to_string(),
            scope_key: ScopeKey { key: vec![0]},
        },
        Entry::Variable {
            data_type: "TMP2".to_string()
        }
    )
    ])};

    match test.check("Test".to_string(), ScopeKey { key: vec![0, 0]}) { //Needs to work with deeper scopes
        Entry::Variable { data_type } => print!("{}\n", data_type),
        Entry::NullEntry => panic!("No entry"),
    }
*/

    let table = semantic_analisis(astnode);

    println!("{:#?}", table);

    println!("{}", str);
}
