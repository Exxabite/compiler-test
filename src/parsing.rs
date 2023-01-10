use self::AstNode::*;
use pest::error::Error;
use pest::{Parser, RuleType};
use std::ffi::CString;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MParser;


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MutationVerb {
    Equals,
    PlusEquals,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ComparisonVerb {
    Equals,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Function {
        name: Box<AstNode>,
        //parameters: Box<AstNode>,
        body: Box<AstNode>,
    },
    Block(Vec<AstNode>),
    MutationExpression {
        target: Box<AstNode>, //This refers to a variable
        verb: MutationVerb,
        expr: Box<AstNode>, //This refers to an expresion that will return a value
    },
    Declaration {
        data_type: Box<AstNode>,
        name: Box<AstNode>,
        expr: Box<AstNode>
    },
    Name(String),
    Integer(i32)
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = MParser::parse(Rule::program, source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::func => {
                ast.push(build_ast(pair));
            }
            _ => {}
        }
    }
    Ok(ast)
}

fn build_ast(pair: pest::iterators::Pair<Rule>) -> AstNode {
    //let next = || {build_ast(pair.into_inner().next().unwrap())};

    //pair.into_inner().next().unwrap()
    match pair.as_rule() {
        Rule::func =>  {
            let mut pair = pair.into_inner();
            AstNode::Function {
                name: Box::new(build_ast(pair.next().unwrap())),
                body: Box::new(build_ast(pair.next().unwrap()))
            }
        },
        Rule::alpha => AstNode::Name(pair.as_str().to_string()),
        Rule::block => { //Should return AstNode::Block
            let expressions: Vec<AstNode> = pair.into_inner().map(build_ast).collect();
            AstNode::Block(expressions)
        }, 
        Rule::expr => build_ast(pair.into_inner().next().unwrap()),
        Rule::assign_expr => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let lhs = build_ast(lhspair);
            let verb = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let rhs = build_ast(rhspair);
            parse_mutation_verb(verb, lhs, rhs)
        }
        Rule::value_expr => build_ast(pair.into_inner().next().unwrap()),
        Rule::integer => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "-" => (-1, &istr[1..]),
                _ => (1, &istr[..]),
            };
            let integer: i32 = istr.parse().unwrap();
            AstNode::Integer(sign * integer)
        },
        Rule::declare_expr => {
            let mut pair = pair.into_inner();
            let data_type = build_ast(pair.next().unwrap());
            let name = build_ast(pair.next().unwrap());
            let expr = build_ast(pair.next().unwrap());
            parse_declaration(data_type, name, expr)

        }
        Rule::data_type => build_ast(pair.into_inner().next().unwrap()),
        x => panic!("Build AST error: Rule::{:?}", x)
    }
}

fn parse_declaration(data_type: AstNode, name: AstNode, expr: AstNode) -> AstNode {
    AstNode::Declaration { 
        data_type: Box::new(data_type),
        name: Box::new(name),
        expr: Box::new(expr)
    }
}

fn parse_mutation_verb(pair: pest::iterators::Pair<Rule>, lhs: AstNode, rhs: AstNode) -> AstNode {
    AstNode::MutationExpression {
        target: Box::new(lhs),
        verb: match pair.as_str() {
            "=" => MutationVerb::Equals,
            "+="=> MutationVerb::PlusEquals,
            _ => panic!("Invalid opperator")
        },
        expr: Box::new(rhs),
    }
}