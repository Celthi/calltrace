use crate::calls::callstack::CallStack;
use crate::calls::CallStacks;
use std::collections::HashMap;
use std::fmt;
pub struct Folder {
    pub map: HashMap<CallStack, usize>,
}

impl std::fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in &self.map {
            writeln!(f, "count: {}", *v)?;
            write!(f, "{}", &k)?;
        }
        writeln!(f)
    }
}

impl Folder {
    pub fn new(css: &CallStacks) -> Self {
        let mut folder = Folder {
            map: HashMap::new(),
        };
        for cs in css.iter() {
            folder.insert(cs.clone(), 1);
        }
        folder
    }
    pub fn insert(&mut self, cs: CallStack, count: usize) -> usize {
        *self
            .map
            .entry(cs)
            .and_modify(|e| *e += count)
            .or_insert(count)
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn has(&self, k: &CallStack) -> bool {
        self.map.contains_key(k)
    }
}
