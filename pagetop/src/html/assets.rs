pub mod javascript;
pub mod stylesheet;

use crate::html::{Markup, html};

pub type SourceValue = &'static str;

pub trait AssetsTrait {
    fn source(&self) -> SourceValue;

    fn weight(&self) -> isize;

    fn render(&self) -> Markup;
}

pub enum AssetsOp<T: AssetsTrait> {
    Add(T),
    Remove(SourceValue),
}
pub struct Assets<T>(Vec<T>);

impl<T: AssetsTrait> Assets<T> {
    pub fn new() -> Self {
        Assets::<T>(Vec::<T>::new())
    }

    pub fn alter(&mut self, op: AssetsOp<T>) -> &mut Self {
        match op {
            AssetsOp::Add(asset) => match self.0.iter().position(
                |x| x.source() == asset.source()
            ) {
                Some(index) => if self.0[index].weight() > asset.weight() {
                    self.0.remove(index);
                    self.0.push(asset);
                },
                _ => self.0.push(asset)
            }
            AssetsOp::Remove(source) => if let Some(index) = self.0.iter().position(
                |x| x.source() == source
            ) {
                self.0.remove(index);
            }
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