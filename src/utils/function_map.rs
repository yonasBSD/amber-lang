use std::collections::HashMap;
use crate::modules::{Type, block::Block};

#[derive(Clone, Debug)]
pub struct FunctionInstance {
    pub args: Vec<Type>,
    pub returns: Type,
    pub body: Block,
}

#[derive(Clone, Debug)]
// This is a map of all generated functions based on their invocations
pub struct FunctionMap {
    map: HashMap<usize, Vec<FunctionInstance>>,
    current_id: usize
}

impl FunctionMap {
    pub fn new() -> FunctionMap {
        FunctionMap {
            map: HashMap::new(),
            current_id: 0
        }
    }

    pub fn add_declaration(&mut self) -> usize {
        let id = self.current_id;
        self.map.insert(id, vec![]);
        self.current_id += 1;
        id
    }

    pub fn add_instance(&mut self, id: usize, function: FunctionInstance) -> usize {
        if let Some(functions) = self.map.get_mut(&id) {
            let length = functions.len();
            functions.push(function);
            length
        } else { 0 }
    }

    pub fn get(&self, id: usize) -> Option<&Vec<FunctionInstance>> {
        self.map.get(&id)
    }
}