use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::iter::zip;
use super::frame::Frame;
#[derive(Eq, Clone, Debug)]
pub struct CallStack {
    pub frames: Vec<Frame>,
}
impl PartialEq for CallStack {
    fn eq(&self, other: &Self) -> bool {
        if self.frames.len() != other.frames.len() {
            return false;
        }
        self.subset(other)
    }
}
impl Display for CallStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.iter() {
            writeln!(f, "{}", v)?;
        }
        writeln!(f)
    }
}
impl Hash for CallStack {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.frames.hash(state);
    }
}
impl Default for CallStack {
    fn default() -> Self {
        Self::new()
    }
}

impl CallStack {
    pub fn subset(&self, other: &Self) -> bool {
        for (l, r) in zip(self.iter(), other.iter()) {
            if l != r {
                return false;
            }
        }
        true
    }
    pub fn has_keyword(&self, pat: &str) -> bool {
        for f in self.iter() {
            if f.raw.contains(pat) {
                return true;
            }
        }
        false
    }
    pub fn from_string(s: &str) -> Self {
        let mut cs = CallStack::new();
        for line in s.lines() {
            if line.trim().is_empty() {
                continue;
            }
            cs.append(line.trim().to_string());
        }
        cs
    }

    pub fn new() -> Self {
        CallStack { frames: vec![] }
    }
    pub fn append(&mut self, frame: String) {
        self.frames.push(Frame::new(frame));
    }
    pub fn get(&self, i: usize) -> Option<&Frame> {
        if self.frames.len() < i {
            return None;
        }
        Some(&self.frames[i])
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Frame> {
        self.frames.iter()
    }

}

