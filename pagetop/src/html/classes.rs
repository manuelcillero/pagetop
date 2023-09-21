//! **Classes** implements a *helper* for dynamically adding class names to components.
//!
//! This *helper* differentiates between default classes (generally associated with styles provided
//! by the theme) and user classes (for customizing components based on application styles).
//!
//! Default classes can be added using [SetDefault] and [AddDefault], while user classes can be
//! added using [Add]. Operations to [Remove], [Replace] or [ReplaceIfExists] a class, as well as
//! to [Reset] user classes, are also provided.
//!
//! Although the order of the classes is irrelevant (<https://stackoverflow.com/a/1321712>), default
//! classes will be presented before user classes and duplicate classes will not be allowed.

use crate::fn_builder;

pub enum ClassesOp {
    SetDefault,
    AddDefault,
    Add,
    Remove,
    Replace(String),
    Reset,
}

#[derive(Clone, PartialEq)]
enum ClassType {
    Default,
    User,
}

#[derive(Default)]
pub struct Classes(Vec<(String, ClassType)>);

impl Classes {
    pub fn new() -> Self {
        Classes::default()
    }

    // Classes BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        let classes: String = classes.into();
        let classes: Vec<&str> = classes.split_ascii_whitespace().collect();

        match op {
            ClassesOp::SetDefault => {
                self.0.retain(|(_, t)| t.ne(&ClassType::Default));
                self.add(&classes, 0, ClassType::Default);
            }
            ClassesOp::AddDefault => {
                let pos = match self.0.iter().position(|(_, t)| t.eq(&ClassType::User)) {
                    Some(pos) => pos,
                    None => self.0.len(),
                };
                self.add(&classes, pos, ClassType::Default);
            }
            ClassesOp::Add => {
                self.add(&classes, self.0.len(), ClassType::User);
            }
            ClassesOp::Remove => {
                for name in classes {
                    self.0.retain(|(c, _)| c.ne(&name.to_string()));
                }
            }
            ClassesOp::Replace(classes_to_replace) => {
                let mut pos = self.0.len();
                let mut class_type = ClassType::User;
                let replace: Vec<&str> = classes_to_replace.split_ascii_whitespace().collect();
                for class in replace {
                    if let Some(replace_pos) = self.0.iter().position(|(c, _)| c.eq(class)) {
                        let (_, replace_type) = self.0.remove(replace_pos);
                        if pos > replace_pos {
                            pos = replace_pos;
                        }
                        if replace_type.eq(&ClassType::Default) {
                            class_type = replace_type;
                        }
                    }
                }
                self.add(&classes, pos, class_type);
            }
            ClassesOp::Reset => {
                self.0.retain(|(_, t)| t.ne(&ClassType::User));
            }
        }
        self
    }

    #[inline]
    fn add(&mut self, classes: &Vec<&str>, mut pos: usize, class_type: ClassType) {
        for class in classes {
            if !class.is_empty() && !self.0.iter().any(|(c, _)| c.eq(class)) {
                self.0.insert(pos, (class.to_string(), class_type.clone()));
                pos += 1;
            }
        }
    }

    // Classes GETTERS.

    pub fn exists(&self, class: impl Into<String>) -> bool {
        let class: String = class.into();
        self.0.iter().any(|(c, _)| c.eq(&class))
    }

    pub fn get(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(
                self.0
                    .iter()
                    .map(|(c, _)| c.to_owned())
                    .collect::<Vec<String>>()
                    .join(" "),
            )
        }
    }
}
