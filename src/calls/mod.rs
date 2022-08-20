pub mod callstack;
pub mod frame;
use anyhow::Result;
use callstack::CallStack;
use std::fmt;
use std::fs::File;
use std::io::Read;
use crate::quote::MatchQuote;
pub struct CallStacks {
    pub data: Vec<CallStack>,
}

impl std::fmt::Display for CallStacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for k in &self.data {
            write!(f, "{}", &k)?;
        }
        writeln!(f)
    }
}
impl Default for CallStacks {
    fn default() -> Self {
        Self::new()
    }
}

impl CallStacks {
    pub fn from_file(s: &str,starts: &[&dyn MatchQuote], ends: &[&dyn MatchQuote]) -> Result<Self> {
        let mut file = File::open(s)?;
        let mut content = String::new();
        let size = file.read_to_string(&mut content)?;
        if size == 0 {
            return Ok(CallStacks::default());
        }

        Ok(Self::from_string(&content, starts, ends))
    }
    pub fn new() -> Self {
        CallStacks {
            data: vec![]
        }
    }

    pub fn push(&mut self, cs: CallStack) {
        self.data.push(cs);
    }
    pub fn from_string(s: &str, starts: &[&dyn MatchQuote], ends: &[&dyn MatchQuote]) -> Self {
        let mut css = CallStacks::new();
        let mut cs = CallStack::new();
        let mut in_frames = false;
        for line in s.lines() {
            let line = line.trim();
            if starts.iter().any(|s| s.match_quote(line)) && !in_frames {
                // call stack starts
                in_frames = true;
                cs = CallStack::new();

                continue;
            }
            if ends.iter().any(|s| s.match_quote(line)) && in_frames {
                // call stack ends
                css.push(cs);
                cs = CallStack::new();
                in_frames = false;
                continue;
            }

            if !line.is_empty() && in_frames {
                cs.append(line.to_string());
            }
        }
        css
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn has(&self, cs: &CallStack) -> bool {
        self.data.iter().any(|e| e == cs)
    }
}

#[cfg(test)]
mod test;
