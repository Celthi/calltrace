pub mod callstack;
pub mod frame;

use anyhow::Result;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fs::File;

use callstack::CallStack;
use std::io::Read;

pub struct CallStacks {
    pub data: HashMap<CallStack, usize>,
}

impl Display for CallStacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in &self.data {
            writeln!(f, "count: {}", *v);
            write!(f, "{}", &k);
        }
        write!(f, "\n")
    }
}
pub struct Separate<'a, 'b> {
    pub start: &'a [&'a str],
    pub end: &'b [&'b str],
}

impl CallStacks {
    pub fn from_file(s: &str, sep: &Separate) -> Result<Self> {
        let mut file = File::open(s)?;
        let mut content = String::new();
        let size = file.read_to_string(&mut content)?;

        Ok(Self::from_string(&content, sep))
    }
    pub fn new() -> Self {
        CallStacks {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, cs: CallStack, count: usize) -> usize {
        *self
            .data
            .entry(cs)
            .and_modify(|e| *e += count)
            .or_insert(count)
    }
    pub fn from_string(s: &str, sep: &Separate) -> Self {
        let mut css = CallStacks::new();
        let mut cs = CallStack::new();
        let mut in_frames = false;
        for line in s.lines() {
            let line = line.trim();
            if sep.start.into_iter().any(|s| line == *s) && !in_frames {
                // call stack starts
                in_frames = true;
                cs = CallStack::new();

                continue;
            }
            if sep.end.into_iter().any(|s| line == s.trim()) && in_frames {
                // call stack ends
                css.insert(cs, 1);
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
    pub fn has(&self, k: &CallStack) -> bool {
        self.data.contains_key(k)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn extract_call_stack() {
        let s = r#"
        
AddRef:
f0
f2
f3

RelRef:
f1
f2
f3

        "#;
        use super::*;
        let sep = Separate {
            start: &["AddRef:", "RelRef:"],
            end: &["\n"],
        };
        let css = CallStacks::from_string(s, &sep);
        assert_eq!(css.size(), 2);
    }

    #[test]
    fn output() {
        let s = r#"
        Callstack:
        f1
        f2
        Callstack end
        dfhdiadfad
        
        Callstack:
        f0
        f2
        Callstack end"#;
        use super::*;
        let sep = Separate {
            start: &["Callstack:"],
            end: &["Callstack end"],
        };
        let css = CallStacks::from_string(s, &sep);
        assert_eq!(css.size(), 2);
        let cs = CallStack::from_string(
            r"
        f1
        f2",
        );
        assert!(css.has(&cs));
        let cs = CallStack::from_string(
            r"
        f0
        f2",
        );
        assert!(css.has(&cs));
        let cs = CallStack::from_string(
            r"
        f0
        f2
        f3",
        );
        assert!(!css.has(&cs));
    }
}
