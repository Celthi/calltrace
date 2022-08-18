use crate::callstack::CallStacks;

pub struct Differ {}

impl Differ {
    pub fn new() -> Self {
        todo!()
    }
    pub fn not_in(target: &CallStacks, base: &CallStacks) -> CallStacks {
        let mut css = CallStacks::new();
        for cs in &target.data {
            if !base.has(&cs.0) {
                css.insert(cs.0.clone(), 1);
            } else {
                match (target.data.get(cs.0), base.data.get(cs.0)) {
                    (Some(t), Some(b)) => {
                        if t > b {
                            css.insert(cs.0.clone(), *t - *b);
                        }
                    }
                    (Some(t), None) => {
                        css.insert(cs.0.clone(), *t);
                    }
                    (_, _) => {}
                }
            }
        }
        css
    }
}
