use pagetop::prelude::*;

pub use pagetop::base::component::Brand;

// **< Brand SETUP >********************************************************************************

pub(crate) fn setup(brand: &mut Brand) {
    brand.alter_prop(PropsOp::replace_classes("brand", "navbar-brand"));
}
