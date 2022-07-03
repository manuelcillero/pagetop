use crate::concat_string;

struct Style {
    property: String,
    inline  : String,
}

pub struct InlineStyles(Vec<Style>);

impl InlineStyles {
    pub fn new() -> Self {
        InlineStyles(Vec::new())
    }

    pub fn add_style(&mut self, property: &str, value: Option<&str>) -> &Self {
        let property = property.trim();
        match self.0.iter().position(|s| s.property.eq(property)) {
            Some(pos) => {
                self.0.remove(pos);
                if let Some(value) = value {
                    self.0.insert(pos, Style {
                        property: property.to_owned(),
                        inline  : concat_string!(property, ":", value.trim(), ";"),
                    });
                }
            },
            _ => if let Some(value) = value {
                self.0.push(Style {
                    property: property.to_owned(),
                    inline  : concat_string!(property, ":", value.trim(), ";"),
                });
            }
        }
        self
    }

    pub fn option(&self) -> Option<String> {
        if self.0.len() == 0 {
            None
        } else {
            let mut inline = "".to_owned();
            self.0.iter().for_each(|s| inline.push_str(s.inline.as_str()));
            Some(inline)
        }
    }
}
