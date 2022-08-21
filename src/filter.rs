use crate::calls::callstack::CallStack;
use crate::calls::CallStacks;
use crate::quote;
pub trait Filter {
    fn execute(&self, cs: &CallStack) -> bool;
}

pub struct TopFrameFilter<'a> {
    matcher: &'a dyn quote::MatchQuote,
}

impl<'a> TopFrameFilter<'a> {
    pub fn new(m: &'a dyn quote::MatchQuote) -> Self {
        TopFrameFilter { matcher: m }
    }
}

impl<'a> Filter for TopFrameFilter<'a> {
    fn execute(&self, cs: &CallStack) -> bool {
        match cs.get(0) {
            None => false,
            Some(f) => self.matcher.match_quote(&f.raw),
        }
    }
}

pub struct AnyFrameFilter<'a> {
    matcher: &'a dyn quote::MatchQuote,
}
impl<'a> AnyFrameFilter<'a> {
    pub fn new(m: &'a dyn quote::MatchQuote) -> Self {
        AnyFrameFilter { matcher: m }
    }
}

impl<'a> Filter for AnyFrameFilter<'a> {
    fn execute(&self, cs: &CallStack) -> bool {
        cs.iter().any(|e| self.matcher.match_quote(&e.raw))
    }
}

pub fn filter(css: &CallStacks, filter: &dyn Filter) -> CallStacks {
    let mut css_res = CallStacks::new();
    for cs in css.iter() {
        if filter.execute(cs) {
            css_res.push(cs.clone());
        }
    }
    css_res
}
