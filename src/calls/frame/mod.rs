use once_cell;
use regex;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
#[derive(Eq, Clone, Debug)]
struct InnerFrame {
    func: Option<String>,
    file: Option<String>,
    line: Option<String>,
    lib: Option<String>,
}
impl PartialEq for InnerFrame {
    fn eq(&self, other: &Self) -> bool {
        self.func == other.func && self.file == other.file && self.line == other.line
    }
}
impl InnerFrame {
    fn parse(s: &str) -> Option<Self> {
        let frame_pat = regex!(r"(?P<file>[^(]+?)\( (?P<line>\d{1,6}) \): (?P<func>.+)");
        frame_pat.captures(s).map(|c| InnerFrame {
            func: c.name("func").map(|m| m.as_str().to_owned()),
            file: c.name("file").map(|m| m.as_str().to_string()),
            line: c.name("line").map(|m| m.as_str().to_string()),
            lib: None
        })
    }
    fn get_func(&self) -> Option<&str> {
        self.func.as_deref()
    }
    fn get_file(&self) -> Option<&str> {
        self.file.as_deref()
    }
    fn get_line(&self) -> Option<&str> {
        self.line.as_deref()
    }
}
#[derive(Eq, Clone, Debug)]
pub struct Frame {
    pub raw: String,
    inner: Option<InnerFrame>,
}
impl Frame {
    pub fn new(s: String) -> Self {
        let inner = InnerFrame::parse(&s);
        Frame { raw: s, inner }
    }
}
impl Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            Some(inner) => write!(
                f,
                "{} at {}:{}",
                inner.get_func().unwrap_or_default(),
                inner.get_file().unwrap_or_default(),
                inner.get_line().unwrap_or_default()
            ),
            None => write!(f, "{}", self.raw),
        }
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
#[cfg(test)]
mod test;
