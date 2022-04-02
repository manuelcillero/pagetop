mod assets;
pub use assets::{
    Favicon,
    StyleSheet,
    JavaScript, JSMode,
    PageAssets,
};

mod component;
pub use component::{ArcComponent, PageComponent};

mod container;
pub use container::PageContainer;

mod page;
pub use page::Page;
pub use page::render_component;
pub use page::add_component_to;

