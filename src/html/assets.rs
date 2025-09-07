pub mod favicon;
pub mod javascript;
pub mod stylesheet;

use crate::html::{html, Markup, Render};
use crate::{AutoDefault, Weight};

pub trait Asset: Render {
    /// Devuelve el nombre del recurso, utilizado como clave única.
    fn name(&self) -> &str;

    /// Devuelve el peso del recurso, durante el renderizado se procesan de menor a mayor peso.
    fn weight(&self) -> Weight;
}

#[derive(AutoDefault)]
pub struct Assets<T>(Vec<T>);

impl<T: Asset> Assets<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, asset: T) -> bool {
        match self.0.iter().position(|x| x.name() == asset.name()) {
            Some(index) => {
                if self.0[index].weight() > asset.weight() {
                    self.0.remove(index);
                    self.0.push(asset);
                    true
                } else {
                    false
                }
            }
            _ => {
                self.0.push(asset);
                true
            }
        }
    }

    pub fn remove(&mut self, name: impl AsRef<str>) -> bool {
        if let Some(index) = self.0.iter().position(|x| x.name() == name.as_ref()) {
            self.0.remove(index);
            true
        } else {
            false
        }
    }
}

impl<T: Asset> Render for Assets<T> {
    fn render(&self) -> Markup {
        let mut assets = self.0.iter().collect::<Vec<_>>();
        assets.sort_by_key(|a| a.weight());

        html! {
            @for a in assets {
                (a)
            }
        }
    }
}
