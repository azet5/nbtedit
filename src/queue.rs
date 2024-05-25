use std::slice::Iter;

use crate::nbt::TagType;

#[derive(Debug, Clone)]
pub enum ActionType {
    Add {
        id: usize,
        parent: usize,
        after: usize,
    },
    Edit {
        id: usize,
        old_name: String,
        old_value: TagType,
    },
    Delete(usize),
}

pub struct ActionQueue(Vec<ActionType>);

impl ActionQueue {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, action: ActionType) {
        self.0.push(action);
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<'_, ActionType> {
        self.0.iter()
    }
}