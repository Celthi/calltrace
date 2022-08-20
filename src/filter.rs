use crate::calls::CallStacks;
use crate::calls::callstack::CallStack;
use crate::quote;
pub trait Filter {
    fn include(&self, cs: &CallStack) -> bool;
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
    fn include(&self, cs: &CallStack) -> bool {
        match cs.get(0) {
            None => false,
            Some(f) => self.matcher.match_quote(&f.raw)
        }
    }
}

pub fn filter(css: &CallStacks, filter: &dyn Filter) -> CallStacks {
    let mut css_res = CallStacks::new();

    for cs in &css.data{
        let cs = cs.0;
        if filter.include(cs) {
            css_res.insert(cs.clone(), 1);
        }
    }

    css_res
}
