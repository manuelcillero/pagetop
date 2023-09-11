//! **Classes** implements a *helper* for dynamically adding class names to components.
//!
//! This *helper* differentiates between default classes (generally associated with styles provided
//! by the theme) and user classes (for customizing components based on application styles).
//!
//! Default classes can be added using [SetDefault] and [AddDefault], while user classes can be
//! added using [Add]. Operations to [Remove] or [Replace] any class, as well as to [Reset] user
//! classes, are also provided.
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
    pub fn alter_value(&mut self, op: ClassesOp, classes: &[impl ToString]) -> &mut Self {
        match op {
            ClassesOp::SetDefault => {
                self.0.retain(|(_, t)| t.ne(&ClassType::Default));
                self.add(classes, 0, ClassType::Default);
            }
            ClassesOp::AddDefault => {
                let pos = match self.0.iter().position(|(_, t)| t.eq(&ClassType::User)) {
                    Some(pos) => pos,
                    None => self.0.len(),
                };
                self.add(classes, pos, ClassType::Default);
            }
            ClassesOp::Add => {
                self.add(classes, self.0.len(), ClassType::User);
            }
            ClassesOp::Remove => {
                for name in classes {
                    self.0.retain(|(c, _)| c.ne(&name.to_string()));
                }
            }
            ClassesOp::Replace(class) => {
                if let Some(pos) = self.0.iter().position(|(c, _)| c.eq(&class)) {
                    let (_, class_type) = self.0.remove(pos);
                    self.add(classes, pos, class_type);
                }
            }
            ClassesOp::Reset => {
                self.0.retain(|(_, t)| t.ne(&ClassType::User));
            }
        }
        self
    }

    #[inline]
    fn add(&mut self, classes: &[impl ToString], mut pos: usize, class_type: ClassType) {
        for class in classes {
            let class: String = class.to_string();
            if !class.is_empty() && self.0.iter().position(|(c, _)| c.eq(&class)).is_none() {
                self.0.insert(pos, (class, class_type.clone()));
                pos = pos + 1;
            }
        }
    }

    // Classes GETTERS.

    pub fn get(&self) -> Option<String> {
        if self.0.len() == 0 {
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
