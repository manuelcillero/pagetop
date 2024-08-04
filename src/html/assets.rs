pub mod favicon;
pub mod javascript;
pub mod stylesheet;

use crate::html::{html, Markup};
use crate::{AutoDefault, Weight};

pub trait AssetsTrait {
    fn name(&self) -> &String;

    fn weight(&self) -> Weight;

    fn prepare(&self) -> Markup;
}

#[derive(AutoDefault)]
pub(crate) struct Assets<T>(Vec<T>);

impl<T: AssetsTrait> Assets<T> {
    pub fn new() -> Self {
        Assets::<T>(Vec::<T>::new())
    }

    pub fn add(&mut self, asset: T) -> &mut Self {
        match self.0.iter().position(|x| x.name() == asset.name()) {
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

    pub fn remove(&mut self, name: &'static str) -> &mut Self {
        if let Some(index) = self.0.iter().position(|x| x.name() == name) {
            self.0.remove(index);
        };
        self
    }

    pub fn prepare(&mut self) -> Markup {
        let assets = &mut self.0;
        assets.sort_by_key(AssetsTrait::weight);
        html! {
            @for a in assets {
                (a.prepare())
            }
        }
    }
}
