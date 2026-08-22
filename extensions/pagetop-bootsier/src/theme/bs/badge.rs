use pagetop::prelude::*;

pub use pagetop::base::component::Badge;

// **< Badge SETUP >********************************************************************************

pub(crate) fn setup(badge: &mut Badge) {
    let intent = badge.intent().as_str();

    badge.alter_prop(PropsOp::replace_classes(
        util::join!("badge-", intent),
        util::join!("text-bg-", intent),
    ));
}
