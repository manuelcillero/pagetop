use pagetop::prelude::*;

pub fn setup(button: &mut Button) {
    button
        .alter_prop(PropsOp::remove_classes("button"))
        .alter_prop(PropsOp::prepend_classes("btn"));
}
