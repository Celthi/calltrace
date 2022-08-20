use crate::calls::CallStacks;

use crate::folder;
pub struct Differ {}

impl Differ {
    pub fn not_in(target: &CallStacks, base: &CallStacks) -> folder::Folder {
        let folder_base = folder::Folder::new(base);
        let folder_target = folder::Folder::new(target);
        let mut css = folder::Folder::new(&CallStacks::new());
        for cs in &folder_target.map {
            if !folder_base.has(cs.0) {
                css.insert(cs.0.clone(), *folder_target.map.get(cs.0).unwrap());
            } else {
                match (folder_target.map.get(cs.0), folder_base.map.get(cs.0)) {
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
