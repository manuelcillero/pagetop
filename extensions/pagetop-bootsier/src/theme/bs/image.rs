//! Definiciones para renderizar imágenes ([`Image`]).

use pagetop::prelude::*;

pub use pagetop::base::component::image::{Image, Size, Source};

// **< Image SETUP >********************************************************************************

pub(crate) fn setup(image: &mut Image) {
    match image.source() {
        Source::Logo(_) | Source::Responsive(_) => {
            image.alter_prop(PropsOp::replace_classes("image image-fluid", "img-fluid"));
        }
        Source::Thumbnail(_) => {
            image.alter_prop(PropsOp::replace_classes(
                "image image-thumbnail",
                "img-thumbnail",
            ));
        }
        Source::Plain(_) => {
            image.alter_prop(PropsOp::remove_classes("image"));
        }
    }
}
