use crate::core::component::{Child, ChildOp, Children};
use crate::core::theme::ThemeRef;
use crate::{fn_builder, AutoDefault, UniqueId};

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

static THEME_REGIONS: LazyLock<RwLock<HashMap<UniqueId, ChildrenInRegions>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

static COMMON_REGIONS: LazyLock<RwLock<ChildrenInRegions>> =
    LazyLock::new(|| RwLock::new(ChildrenInRegions::default()));

#[derive(AutoDefault)]
pub struct ChildrenInRegions(HashMap<&'static str, Children>);

impl ChildrenInRegions {
    pub fn new() -> Self {
        ChildrenInRegions::default()
    }

    pub fn with(region_id: &'static str, child: Child) -> Self {
        ChildrenInRegions::default().with_in_region(region_id, ChildOp::Add(child))
    }

    #[fn_builder]
    pub fn with_in_region(mut self, region_id: &'static str, op: ChildOp) -> Self {
        if let Some(region) = self.0.get_mut(region_id) {
            region.alter_child(op);
        } else {
            self.0.insert(region_id, Children::new().with_child(op));
        }
        self
    }

    pub fn all_in_region(&self, theme: ThemeRef, region_id: &str) -> Children {
        let common = COMMON_REGIONS.read().unwrap();
        if let Some(r) = THEME_REGIONS.read().unwrap().get(&theme.type_id()) {
            Children::merge(&[
                common.0.get(region_id),
                self.0.get(region_id),
                r.0.get(region_id),
            ])
        } else {
            Children::merge(&[common.0.get(region_id), self.0.get(region_id)])
        }
    }
}

pub enum InRegion {
    Content,
    Named(&'static str),
    OfTheme(&'static str, ThemeRef),
}

impl InRegion {
    pub fn add(&self, child: Child) -> &Self {
        match self {
            InRegion::Content => {
                COMMON_REGIONS
                    .write()
                    .unwrap()
                    .alter_in_region("region-content", ChildOp::Add(child));
            }
            InRegion::Named(name) => {
                COMMON_REGIONS
                    .write()
                    .unwrap()
                    .alter_in_region(name, ChildOp::Add(child));
            }
            InRegion::OfTheme(region, theme) => {
                let mut regions = THEME_REGIONS.write().unwrap();
                if let Some(r) = regions.get_mut(&theme.type_id()) {
                    r.alter_in_region(region, ChildOp::Add(child));
                } else {
                    regions.insert(theme.type_id(), ChildrenInRegions::with(region, child));
                }
            }
        }
        self
    }
}
