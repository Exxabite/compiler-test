use pest::error::Error;
use pest::{Parser, RuleType};

pub use crate::parsing::Rule;

pub enum CompilationError {
    ParsingError(Rule),
    SemanticError
}

pub fn display_error(err: CompilationError) {

}