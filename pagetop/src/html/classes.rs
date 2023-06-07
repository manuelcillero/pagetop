use crate::{concat_string, fn_builder};

pub enum ClassesOp {
    Add,
    AddAfter(&'static str),
    AddBefore(&'static str),
    AddFirst,
    Remove,
    Replace(&'static str),
    Reset,
    SetDefault,
    SetDefaultIfEmpty,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Classes {
    default: String,
    added  : String,
}

impl Classes {
    pub fn new() -> Self {
        Classes::default()
    }

    // Classes BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        let classes = classes.trim();
        match op {
            ClassesOp::Add => {
                self.added = concat_string!(self.added, " ", classes).trim().to_owned()
            }

            ClassesOp::AddAfter(class) => {
                let mut v_added: Vec<&str> = self.added.split_ascii_whitespace().collect();
                match v_added.iter().position(|c| c.eq(&class)) {
                    Some(pos) => v_added.insert(pos + 1, classes),
                    _ => v_added.push(classes),
                }
                self.added = v_added.join(" ");
            }

            ClassesOp::AddBefore(class) => {
                let mut v_added: Vec<&str> = self.added.split_ascii_whitespace().collect();
                match v_added.iter().position(|c| c.eq(&class)) {
                    Some(pos) => v_added.insert(pos, classes),
                    _ => v_added.insert(0, classes),
                }
                self.added = v_added.join(" ");
            }

            ClassesOp::AddFirst => {
                self.added = concat_string!(classes, " ", self.added).trim().to_owned()
            }

            ClassesOp::Remove => {
                let v_list: Vec<&str> = classes.split_ascii_whitespace().collect();
                let mut v_added: Vec<&str> = self.added.split_ascii_whitespace().collect();
                for class in v_list {
                    if let Some(pos) = v_added.iter().position(|c| c.eq(&class)) {
                        v_added.remove(pos);
                    }
                }
                self.added = v_added.join(" ");
            }

            ClassesOp::Replace(class) => {
                let mut v_added: Vec<&str> = self.added.split_ascii_whitespace().collect();
                match v_added.iter().position(|c| c.eq(&class)) {
                    Some(pos) => {
                        v_added.remove(pos);
                        v_added.insert(pos, classes);
                    }
                    _ => v_added.push(classes),
                }
                self.added = v_added.join(" ");
            }

            ClassesOp::Reset => self.added = classes.to_owned(),

            ClassesOp::SetDefault => self.default = classes.to_owned(),

            ClassesOp::SetDefaultIfEmpty => {
                if self.default.is_empty() {
                    self.default = classes.to_owned()
                }
            }
        }
        self
    }

    // Classes GETTERS.

    pub fn get(&self) -> Option<String> {
        if self.default.is_empty() && self.added.is_empty() {
            None
        } else {
            Some(
                concat_string!(self.default, " ", self.added)
                    .trim()
                    .to_owned(),
            )
        }
    }
}
