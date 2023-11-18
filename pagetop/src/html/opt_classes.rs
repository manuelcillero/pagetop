//! **OptionClasses** implements a *helper* for dynamically adding class names to components.
//!
//! This *helper* differentiates between default classes (generally associated with styles provided
//! by the theme) and user classes (for customizing components based on application styles).
//!
//! Classes can be added using [Add]. Operations to [Remove], [Replace] or [Toggle] a class, as well
//! as [Clear] all classes, are also provided.
//!
//! **OptionClasses** assumes that the order of the classes is irrelevant
//! (<https://stackoverflow.com/a/1321712>), and duplicate classes will not be allowed.

use crate::{fn_builder, SmartDefault};

pub enum ClassesOp {
    Add,
    AddFirst,
    Remove,
    Replace(String),
    Toggle,
    Clear,
}

#[derive(SmartDefault)]
pub struct OptionClasses(Vec<String>);

impl OptionClasses {
    pub fn new(classes: impl Into<String>) -> Self {
        OptionClasses::default().with_value(ClassesOp::AddFirst, classes)
    }

    // OptionClasses BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        let classes: String = classes.into();
        let classes: Vec<&str> = classes.split_ascii_whitespace().collect();

        match op {
            ClassesOp::Add => {
                self.add(&classes, self.0.len());
            }
            ClassesOp::AddFirst => {
                self.add(&classes, 0);
            }
            ClassesOp::Remove => {
                for class in classes {
                    self.0.retain(|c| c.ne(&class.to_string()));
                }
            }
            ClassesOp::Replace(classes_to_replace) => {
                let mut pos = self.0.len();
                let replace: Vec<&str> = classes_to_replace.split_ascii_whitespace().collect();
                for class in replace {
                    if let Some(replace_pos) = self.0.iter().position(|c| c.eq(class)) {
                        self.0.remove(replace_pos);
                        if pos > replace_pos {
                            pos = replace_pos;
                        }
                    }
                }
                self.add(&classes, pos);
            }
            ClassesOp::Toggle => {
                for class in classes {
                    if !class.is_empty() {
                        if let Some(pos) = self.0.iter().position(|c| c.eq(class)) {
                            self.0.remove(pos);
                        } else {
                            self.0.push(class.to_string());
                        }
                    }
                }
            }
            ClassesOp::Clear => {
                self.0.clear();
            }
        }
        self
    }

    #[inline]
    fn add(&mut self, classes: &Vec<&str>, mut pos: usize) {
        for class in classes {
            if !class.is_empty() && !self.0.iter().any(|c| c.eq(class)) {
                self.0.insert(pos, class.to_string());
                pos += 1;
            }
        }
    }

    // OptionClasses GETTERS.

    pub fn get(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.join(" "))
        }
    }

    pub fn contains(&self, class: impl Into<String>) -> bool {
        let class: String = class.into();
        self.0.iter().any(|c| c.eq(&class))
    }
}
