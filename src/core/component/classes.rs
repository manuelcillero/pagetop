use crate::core::component::ComponentBase;
use crate::html::{ClassesOp, OptionClasses};

pub trait ComponentClassesOp {
    fn with_classes(self, op: ClassesOp, classes: impl Into<String>) -> Self;

    fn add_classes(&mut self, classes: impl Into<String>) -> &mut Self;

    fn prepend_classes(&mut self, classes: impl Into<String>) -> &mut Self;

    fn remove_classes(&mut self, classes: impl Into<String>) -> &mut Self;

    fn replace_classes(&mut self, rep: impl Into<String>, classes: impl Into<String>) -> &mut Self;

    fn toggle_classes(&mut self, classes: impl Into<String>) -> &mut Self;

    fn set_classes(&mut self, classes: impl Into<String>) -> &mut Self;
}

pub trait ComponentClasses: ComponentBase + ComponentClassesOp {
    fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self;

    fn classes(&self) -> &OptionClasses;
}

impl<C: ComponentBase + ComponentClasses> ComponentClassesOp for C {
    #[doc(hidden)]
    fn with_classes(mut self, op: ClassesOp, classes: impl Into<String>) -> Self {
        self.alter_classes(op, classes);
        self
    }

    fn add_classes(&mut self, classes: impl Into<String>) -> &mut Self {
        self.alter_classes(ClassesOp::Add, classes);
        self
    }

    fn prepend_classes(&mut self, classes: impl Into<String>) -> &mut Self {
        self.alter_classes(ClassesOp::Prepend, classes);
        self
    }

    fn remove_classes(&mut self, classes: impl Into<String>) -> &mut Self {
        self.alter_classes(ClassesOp::Remove, classes);
        self
    }

    fn replace_classes(&mut self, rep: impl Into<String>, classes: impl Into<String>) -> &mut Self {
        self.alter_classes(ClassesOp::Replace(rep.into()), classes);
        self
    }

    fn toggle_classes(&mut self, classes: impl Into<String>) -> &mut Self {
        self.alter_classes(ClassesOp::Toggle, classes);
        self
    }

    fn set_classes(&mut self, classes: impl Into<String>) -> &mut Self {
        self.alter_classes(ClassesOp::Set, classes);
        self
    }
}
