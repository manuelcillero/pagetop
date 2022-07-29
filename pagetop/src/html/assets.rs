pub mod javascript;
pub mod stylesheet;

use crate::html::{html, Markup};

pub trait AssetsTrait {
    fn source(&self) -> &'static str;

    fn weight(&self) -> isize;

    fn render(&self) -> Markup;
}

#[derive(Default)]
pub struct Assets<T>(Vec<T>);

impl<T: AssetsTrait> Assets<T> {
    pub fn new() -> Self {
        Assets::<T>(Vec::<T>::new())
    }

    pub fn add(&mut self, asset: T) -> &mut Self {
        match self.0.iter().position(|x| x.source() == asset.source()) {
            Some(index) => {
                if self.0[index].weight() > asset.weight() {
                    self.0.remove(index);
                    self.0.push(asset);
                }
            }
            _ => self.0.push(asset),
        };
        self
    }

    pub fn remove(&mut self, source: &'static str) -> &mut Self {
        if let Some(index) = self.0.iter().position(|x| x.source() == source) {
            self.0.remove(index);
        };
        self
    }

    pub fn render(&mut self) -> Markup {
        let assets = &mut self.0;
        assets.sort_by_key(|a| a.weight());
        html! {
            @for a in assets {
                (a.render())
            }
        }
    }
}
