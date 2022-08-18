use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::iter::zip;
use super::frame::Frame;
#[derive(Eq, Clone, Debug)]
pub struct CallStack {
    frames: Vec<Frame>,
}
impl PartialEq for CallStack {
    fn eq(&self, other: &Self) -> bool {
        if self.frames.len() != self.frames.len() {
            return false;
        }
        self.subset(other)
    }
}
impl Display for CallStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in &self.frames {
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
        for (l, r) in zip(self.frames.iter(), other.frames.iter()) {
            if l != r {
                return false;
            }
        }
        true
    }
    pub fn has_keyword(s: &str) -> bool {
        todo!()
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
}


pub struct Quotes<'a, 'b> {
    pub start: &'a [&'a str],
    pub end: &'b [&'b str],
}