use pagetop::prelude::*;

pub use pagetop::base::component::{Badge, BadgeKind};

// **< Badge SETUP >********************************************************************************

#[rustfmt::skip]
pub(crate) fn setup(badge: &mut Badge) {
    let (old, new) = match badge.kind() {
        BadgeKind::Primary   => ("badge-primary",   "text-bg-primary"),
        BadgeKind::Secondary => ("badge-secondary", "text-bg-secondary"),
        BadgeKind::Success   => ("badge-success",   "text-bg-success"),
        BadgeKind::Info      => ("badge-info",      "text-bg-info"),
        BadgeKind::Warning   => ("badge-warning",   "text-bg-warning"),
        BadgeKind::Danger    => ("badge-danger",    "text-bg-danger"),
    };
    badge.alter_prop(PropsOp::replace_classes(old, new));
}
