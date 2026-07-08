use pagetop::prelude::*;

pub use pagetop::base::component::{Button, ButtonAction};

// **< Button SETUP >*******************************************************************************

pub(crate) fn setup(button: &mut Button) {
    button.alter_prop(PropsOp::replace_classes("button", "btn"));
}
