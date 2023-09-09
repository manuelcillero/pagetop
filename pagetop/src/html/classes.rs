use crate::fn_builder;

pub enum ClassesOp {
    Add,
    Remove,
    Replace(String),
    Reset,
    SetDefault,
}

#[derive(Eq, PartialEq)]
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
    pub fn alter_value(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        let classes: Vec<String> = classes
            .split_ascii_whitespace()
            .map(|c| c.to_owned())
            .collect();

        match op {
            ClassesOp::Add => {
                for class in classes {
                    if self.0.iter().position(|(c, _)| c.eq(&class)).is_none() {
                        self.0.push((class, ClassType::User));
                    }
                }
            }

            ClassesOp::Remove => {
                for class in classes {
                    self.0
                        .retain(|(c, t)| c.ne(&class) || t.ne(&ClassType::User));
                }
            }

            ClassesOp::Replace(value) => {
                for class in classes {
                    match self.0.iter().position(|(c, _)| c.eq(&value)) {
                        Some(pos) => {
                            self.0.remove(pos);
                            self.0.insert(pos, (class, ClassType::User));
                        }
                        _ => self.0.push((class, ClassType::User)),
                    }
                }
            }

            ClassesOp::Reset => self.0.retain(|(_, t)| t.eq(&ClassType::Default)),

            ClassesOp::SetDefault => {
                self.0.retain(|(_, t)| t.eq(&ClassType::User));
                let mut pos = 0;
                for class in classes {
                    if self.0.iter().position(|(c, _)| c.eq(&class)).is_none() {
                        self.0.insert(pos, (class, ClassType::Default));
                        pos = pos + 1;
                    }
                }
            }
        }
        self
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
