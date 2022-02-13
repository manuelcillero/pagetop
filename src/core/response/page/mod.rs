pub mod assets;
pub use assets::Assets as PageAssets;

mod component;
pub use component::Component as PageComponent;

mod container;
pub use container::Container as PageContainer;

mod page;
pub use page::Page;
pub use page::render_component;
