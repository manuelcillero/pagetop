use crate::core::all::COMPONENTS;

pub mod assets;
pub use assets::Assets as PageAssets;

mod component;
pub use component::Component as PageComponent;

mod container;
pub use container::Container as PageContainer;

mod page;
pub use page::Page;
pub use page::render_component;

pub fn add_component_to(region: &'static str, component: impl PageComponent) {
    let mut hmap = COMPONENTS.write().unwrap();
    if let Some(regions) = hmap.get_mut(region) {
        regions.add(component);
    } else {
        hmap.insert(region, PageContainer::new_with(component));
    }
}
