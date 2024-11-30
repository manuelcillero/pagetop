use crate::core::component::{AnyComponent, AnyOp, Children};
use crate::core::theme::ThemeRef;
use crate::{fn_builder, AutoDefault, TypeId};

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

static THEME_REGIONS: LazyLock<RwLock<HashMap<TypeId, ChildrenInRegions>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

static COMMON_REGIONS: LazyLock<RwLock<ChildrenInRegions>> =
    LazyLock::new(|| RwLock::new(ChildrenInRegions::default()));

#[derive(AutoDefault)]
pub struct ChildrenInRegions(HashMap<&'static str, Children>);

impl ChildrenInRegions {
    pub fn new(region: &'static str, any: AnyComponent) -> Self {
        ChildrenInRegions::default().with_in_region(region, AnyOp::Add(any))
    }

    #[fn_builder]
    pub fn set_in_region(&mut self, region: &'static str, op: AnyOp) -> &mut Self {
        if let Some(region) = self.0.get_mut(region) {
            region.set_value(op);
        } else {
            self.0.insert(region, Children::new().with_value(op));
        }
        self
    }

    pub fn all_components(&self, theme: ThemeRef, region: &str) -> Children {
        let common = COMMON_REGIONS.read().unwrap();
        if let Some(r) = THEME_REGIONS.read().unwrap().get(&theme.type_id()) {
            Children::merge(&[common.0.get(region), self.0.get(region), r.0.get(region)])
        } else {
            Children::merge(&[common.0.get(region), self.0.get(region)])
        }
    }
}

pub enum InRegion {
    Content,
    Named(&'static str),
    OfTheme(&'static str, ThemeRef),
}

impl InRegion {
    pub fn add(&self, any: AnyComponent) -> &Self {
        match self {
            InRegion::Content => {
                COMMON_REGIONS
                    .write()
                    .unwrap()
                    .set_in_region("content", AnyOp::Add(any));
            }
            InRegion::Named(name) => {
                COMMON_REGIONS
                    .write()
                    .unwrap()
                    .set_in_region(name, AnyOp::Add(any));
            }
            InRegion::OfTheme(region, theme) => {
                let mut regions = THEME_REGIONS.write().unwrap();
                if let Some(r) = regions.get_mut(&theme.type_id()) {
                    r.set_in_region(region, AnyOp::Add(any));
                } else {
                    regions.insert(theme.type_id(), ChildrenInRegions::new(region, any));
                }
            }
        }
        self
    }
}
