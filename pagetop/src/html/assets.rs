pub mod javascript;
pub mod stylesheet;

use crate::html::{Markup, html};

pub trait AssetsTrait {
    fn source(&self) -> &'static str;

    fn weight(&self) -> isize;

    fn render(&self) -> Markup;
}

pub struct Assets<T>(Vec<T>);

impl<T: AssetsTrait> Assets<T> {
    pub fn new() -> Self {
        Assets::<T>(Vec::<T>::new())
    }

    pub fn add(&mut self, assets: T) -> &mut Self {
        match self.0.iter().position(|x| x.source() == assets.source()) {
            Some(index) => if self.0[index].weight() > assets.weight() {
                self.0.remove(index);
                self.0.push(assets);
            },
            _ => self.0.push(assets)
        }
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