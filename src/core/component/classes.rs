use crate::core::component::ComponentBase;
use crate::html::{ClassesOp, OptionClasses};

pub trait ComponentClassesOp {
    fn with_classes(self, op: ClassesOp, classes: impl Into<String>) -> Self;
}

pub trait ComponentClasses: ComponentBase + ComponentClassesOp {
    fn set_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self;

    fn classes(&self) -> &OptionClasses;
}

impl<C: ComponentBase + ComponentClasses> ComponentClassesOp for C {
    fn with_classes(mut self, op: ClassesOp, classes: impl Into<String>) -> Self {
        self.set_classes(op, classes);
        self
    }
}
