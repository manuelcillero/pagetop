pub use maud::{DOCTYPE, Markup, PreEscaped, html};

mod assets;
pub use assets::Assets;
pub use assets::javascript::{JavaScript, JSMode};
pub use assets::stylesheet::StyleSheet;

mod favicon;
pub use favicon::Favicon;

mod optiden;
pub use optiden::OptIden;

mod optattr;
pub use optattr::OptAttr;

mod classes;
pub use classes::{Classes, ClassesOp};

mod inline_styles;
pub use inline_styles::InlineStyles;
