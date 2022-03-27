pub struct Classes {
    classes: Vec<String>,
    option : Option<String>,
    updated: bool,
}

impl Classes {
    pub fn none() -> Self {
        Classes {
            classes: Vec::new(),
            option : None,
            updated: true,
        }
    }

    pub fn some_class(class: &str) -> Self {
        let mut c = Classes::none();
        c.add_class(class);
        c
    }

    pub fn some_classes(classes: Vec<String>) -> Self {
        let mut c = Classes::none();
        c.add_classes(classes);
        c
    }

    pub fn add_class(&mut self, class: &str) {
        let class = class.trim().replace(" ", "_");
        if !class.is_empty() && !self.classes.iter().any(|c| *c == class) {
            self.classes.push(class.to_owned());
            self.updated = false;
        }
    }

    pub fn add_classes(&mut self, classes: Vec<String>) {
        for class in classes.iter() {
            self.add_class(class);
        }
    }

    pub fn classes(&mut self) -> &str {
        match self.option() {
            Some(classes) => classes.as_str(),
            None => "",
        }
    }

    pub fn has_classes(&self) -> bool {
        self.classes.len() > 0
    }

    pub fn option(&mut self) -> &Option<String> {
        if !self.updated {
            self.option = match self.classes.len() {
                0 => None,
                _ => Some(self.classes.join(" ")),
            };
            self.updated = true;
        }
        &self.option
    }
}
