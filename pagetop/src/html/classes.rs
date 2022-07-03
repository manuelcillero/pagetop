use crate::concat_string;

pub enum ClassesOp {
    Add,
    AddAfter(&'static str),
    AddBefore(&'static str),
    AddFirst,
    Replace(&'static str),
    Reset,
    SetDefault,
    SetDefaultIfEmpty,
}

pub struct Classes {
    default: String,
    added  : String,
}

impl Classes {
    pub fn new() -> Self {
        Classes {
            default: "".to_owned(),
            added  : "".to_owned(),
        }
    }

    pub fn new_with_default(default: &str) -> Self {
        let mut classes = Self::new();
        classes.alter(default, ClassesOp::SetDefault);
        classes
    }

    pub fn alter(&mut self, classes: &str, op: ClassesOp) -> &Self {
        let classes = classes.trim();
        match op {
            ClassesOp::Add => {
                self.added = concat_string!(self.added, " ", classes).trim().to_owned()
            },

            ClassesOp::AddAfter(class) => {
                let mut v_added: Vec<&str> = self.added.split_ascii_whitespace().collect();
                match v_added.iter().position(|c| c.eq(&class)) {
                    Some(pos) => v_added.insert(pos + 1, classes),
                    _ => v_added.push(classes),
                }
                self.added = v_added.join(" ");
            },

            ClassesOp::AddBefore(class) => {
                let mut v_added: Vec<&str> = self.added.split_ascii_whitespace().collect();
                match v_added.iter().position(|c| c.eq(&class)) {
                    Some(pos) => v_added.insert(pos, classes),
                    _ => v_added.insert(0, classes),
                }
                self.added = v_added.join(" ");
            },

            ClassesOp::AddFirst => {
                self.added = concat_string!(classes, " ", self.added).trim().to_owned()
            },

            ClassesOp::Replace(class) => {
                let mut v_added: Vec<&str> = self.added.split_ascii_whitespace().collect();
                match v_added.iter().position(|c| c.eq(&class)) {
                    Some(pos) => {
                        v_added.remove(pos);
                        v_added.insert(pos, classes);
                    },
                    _ => v_added.push(classes),
                }
                self.added = v_added.join(" ");
            },

            ClassesOp::Reset => self.added = classes.to_owned(),

            ClassesOp::SetDefault => self.default = classes.to_owned(),

            ClassesOp::SetDefaultIfEmpty => if self.default.is_empty() {
                self.default = classes.to_owned()
            },
        }
        self
    }

    pub fn get(&self) -> Option<String> {
        if self.default.is_empty() && self.added.is_empty() {
            None
        } else {
            Some(concat_string!(self.default, " ", self.added).trim().to_owned())
        }
    }
}
