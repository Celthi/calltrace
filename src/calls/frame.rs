
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Eq, Clone, Debug)]
pub struct Frame {
    pub raw: String,
    func: Option<String>,
    file: Option<String>,
    line: Option<String>,
}
impl Frame {
    pub fn new(s: String) -> Self {
        Frame {raw: s, func: None, file: None, line: None}
    }
}
impl Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.raw)
    }
}
impl PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Hash for Frame {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}
