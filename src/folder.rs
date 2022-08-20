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
        for cs in &css.data {
            let cs = cs.0;
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
    pub fn size(&self) -> usize {
        self.map.len()
    }
    pub fn has(&self, k: &CallStack) -> bool {
        self.map.contains_key(k)
    }
}
