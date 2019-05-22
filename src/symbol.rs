/* A simple symbol table implementation. */

use std::collections::HashMap;

pub struct SymbolTable {
    mut map: HashMap<String, u64>,
    mut next: u64
}

pub fn new() -> SymbolTable {
    SymbolTable {
        map: HashMap<String, u64>::new(),
        next: 1
    }
}

impl SymbolTable {
    pub fn insert(val: String) -> u64 {

    }
}
