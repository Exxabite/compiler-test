use std::collections::HashMap;

//mod root::parsing;
pub use crate::parsing::AstNode;


#[derive(Clone, Debug)]
pub enum Entry {
    Variable {
        data_type: String, //tmp value for testing
    },
    NullEntry,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct TableKey {
    name: String,
    scope_key: ScopeKey,
}

///The Symbol Table is what stores all of the symbol and scope infromation for the compiler.
/// 
#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<TableKey, Entry>,
}

///Definition of scope key:
///
///A scope key is a unique identifier that describes the scope of a variable, function or block.
///A scope key is stored as a variable length vector of unsigned integers but can be described as a string ie. "012345"
///An empty scope key, or "" when described as a string, refers to the global scope and a variable of said scope can be accessed anywhere.
///When a new block is entered the key of the current scope gains a digit based on how many other block exist within the blocks scope.
///If the current block is the only block in its scope, the number at the end of its scope key will be 0.
///Assuming global scope to begin with the scope key for
/// ```
///    i       is ""
///    {i}     is 0
///    {{i}}   is 00
///    {{}{i}} is 01
///```
/// 
///Scope keys are a way to uniquely describe any scope in a way that can be compared for scope validity.
///When comparing scope keys, if the key of the variable matches the beginning of the block key
/// ```
///    012     Variable key
///    01234   Block Key
/// ```
///Then we know that the variable is in scope
///Conversly if the first characters do not match, we know it is out of scope.
///
///If the current block key matches the beginning of the variable key
/// ```
///    012     Block Key
///    01234   Variable key
/// ```
///The variable is still out of scope because it was defined in a lower scope.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct ScopeKey {
    key: Vec<u8>,
}

impl ScopeKey {
    fn push(&self, index: u8) -> ScopeKey { //UPDATE!!
        let mut key = self.key.clone();
        key.push(index);
        ScopeKey { key: key }
    }
    ///Returns a ScopeKey with the end value popped off. Does not modify object passed to it.
    fn pop(&self,) -> ScopeKey {
        let mut key = self.key.clone();
        key.pop();
        ScopeKey { key: key }
    }

    fn size(&self) -> u32 {
        self.key.len().try_into().unwrap()
    }
}


impl SymbolTable {
    fn insert(&mut self, name: String, scope: ScopeKey, entry: Entry) { //Should never overwrite values, validity needs to be checked before calling
        self.table.insert(TableKey {name: name, scope_key: scope}, entry);
    }

    pub fn check(&self, name: String, scope_key: ScopeKey) -> Entry { //Call with name and scope, returns the information about the specified variable

        fn loop_check(table: &HashMap<TableKey, Entry>, name: String, scope_key: ScopeKey) -> Entry {
            match table.get(&TableKey {name: name.clone(), scope_key: scope_key.clone()}) {
                None => match scope_key.size() {
                    0 => Entry::NullEntry,
                    1.. => loop_check(table, name, scope_key.pop()),
                } 
                Some(x) => x.clone()
            }
        }

        loop_check(&self.table, name, scope_key) //Ensure limit exists

        /*
        for i in [0..key.len()] {
            match self.table.get(&TableKey {name: name.clone(), scope_key: key.clone()}) {
                None => {},
                Some(x) => {
                    data = x.clone(); 
                    break
                },
            }
            key.pop();  //Try recursion
            //println!("{:?}", key);
        }*/
        //return data;
    }

    fn merge(&mut self, other: SymbolTable){
        self.table.extend(other.table);
    }
/* 
    fn new() -> SymbolTable {
        SymbolTable { table: HashMap::new() }
    }
*/


}



pub fn semantic_analisis(root: Vec<AstNode>) -> SymbolTable {
    
    let mut table = SymbolTable{ table: HashMap::new() };

    for (i, node) in root.iter().enumerate() {
        let x = match node {
            AstNode::Function { name, body } => generate_symbol_table(&*body, ScopeKey {key: vec![i.try_into().unwrap()]}), //Enters the body of the function, should be AstNode::Block. Functions get their own scope above the main block
            _ => panic!("Unfinished semantic_analisis")
        };
        table.merge(x);
    }

    fn generate_symbol_table(node: &AstNode, scope: ScopeKey) -> SymbolTable {
        match node {
            AstNode::Block(block) => {
                let mut table = SymbolTable { table: HashMap::new() };
                let mut index = 0;

                for line in block { //Iterates over the lines in block
                    table.merge(generate_symbol_table(line, scope.push(index))); //Make sure this is right
                    match line {
                        AstNode::Block(_) => index += 1,
                        _ => (),
                    }
                }
                table
            }
            AstNode::MutationExpression { target, verb, expr } => {
                match &**target {
                    AstNode::Name(name) => SymbolTable { 
                        table: HashMap::from([(
                            TableKey{
                                name: name.clone(),
                                scope_key: scope
                            },
                            Entry::Variable {
                                data_type: "Temporary Example Type".to_string()
                            }
                        )]) 
                    },
                    _ => unreachable!("taget needs to be name")
                } //Gets the value for the left side, a temporary settup for testing (Change later)
            }
            _ => panic!("Invalid symbol table generation")
        }
    }

    table
}
/* 

fn semantic_analisis(root: Vec<AstNode>) {
    let key: Vec<u8> = vec![];

    let mut table: SymbolTable;

    let mut index: u8 = 0;

    for node in root {
        let x = match node {
            AstNode::Function { name, body } => generate_symbol_table(&*body, push(key, index)),
            _ => panic!("Unfinished semantic_analisis")
        };
        table.merge(x);
        index += 1;
    }

    //return table;

    fn push(key: Vec<u8>, index: u8) -> Vec<u8> {
        key.push(index);
        key
    }

    fn generate_symbol_table(node: &AstNode, key: Vec<u8>) -> SymbolTable { //
        match node {
            AstNode::Block(x) => {
                let table: SymbolTable;
                for n in x {
                    table.merge(generate_symbol_table(n, key)); //Make sure this is right
                }
                table
            }
            AstNode::MutationExpression { target, verb: _, expr: _ } => generate_symbol_table(&*target, key),
            AstNode::Name(name) => SymbolTable { 
                table: HashMap::from([(
                    TableKey{
                        name: *name,
                        scope_key: key
                    },
                    Entry::Variable {
                        data_type: "TMP".to_string()
                    }
                )]) 
            },

        }
    }

}
*/