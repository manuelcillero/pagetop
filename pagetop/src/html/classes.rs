use crate::concat_string;

pub struct Classes(Option<String>);

impl Classes {
    pub fn none() -> Self {
        Classes(None)
    }

    pub fn some(classes: Vec<&str>) -> Self {
        let mut c = Classes::none();
        c.add_classes(classes);
        c
    }

    pub fn add_classes(&mut self, classes: Vec<&str>) {
        for class in classes.iter() {
            self.add_class(class);
        }
    }

    fn add_class(&mut self, class: &str) {
        let class = class.trim().replace(" ", "_");
        if !class.is_empty() {
            match &self.0 {
                None => self.0 = Some(class),
                Some(classes) => if !classes.split(" ").any(|c| *c == class) {
                    self.0 = Some(concat_string!(classes, " ", class))
                }
            }
        }
    }

    pub fn classes(&self) -> &str {
        match &self.0 {
            Some(classes) => classes.as_str(),
            None => "",
        }
    }

    pub fn has_classes(&self) -> bool {
        self.0 != None
    }

    pub fn option(&self) -> &Option<String> {
        &self.0
    }
}
