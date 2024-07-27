use crate::core::component::{AnyComponent, AnyOp, MixedComponents};
use crate::core::theme::ThemeRef;
use crate::{fn_builder, AutoDefault, TypeId};

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

static THEME_REGIONS: LazyLock<RwLock<HashMap<TypeId, ComponentsInRegions>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

static COMMON_REGIONS: LazyLock<RwLock<ComponentsInRegions>> =
    LazyLock::new(|| RwLock::new(ComponentsInRegions::default()));

#[derive(AutoDefault)]
pub struct ComponentsInRegions(HashMap<&'static str, MixedComponents>);

impl ComponentsInRegions {
    pub fn new(region: &'static str, any: AnyComponent) -> Self {
        ComponentsInRegions::default().with_components(region, AnyOp::Add(any))
    }

    #[fn_builder]
    pub fn alter_components(&mut self, region: &'static str, op: AnyOp) -> &mut Self {
        if let Some(region) = self.0.get_mut(region) {
            region.alter_value(op);
        } else {
            self.0.insert(region, MixedComponents::new().with_value(op));
        }
        self
    }

    pub fn all_components(&self, theme: ThemeRef, region: &str) -> MixedComponents {
        let common = COMMON_REGIONS.read().unwrap();
        if let Some(r) = THEME_REGIONS.read().unwrap().get(&theme.type_id()) {
            MixedComponents::merge(&[common.0.get(region), self.0.get(region), r.0.get(region)])
        } else {
            MixedComponents::merge(&[common.0.get(region), self.0.get(region)])
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
                    .alter_components("content", AnyOp::Add(any));
            }
            InRegion::Named(name) => {
                COMMON_REGIONS
                    .write()
                    .unwrap()
                    .alter_components(name, AnyOp::Add(any));
            }
            InRegion::OfTheme(region, theme) => {
                let mut regions = THEME_REGIONS.write().unwrap();
                if let Some(r) = regions.get_mut(&theme.type_id()) {
                    r.alter_components(region, AnyOp::Add(any));
                } else {
                    regions.insert(theme.type_id(), ComponentsInRegions::new(region, any));
                }
            }
        }
        self
    }
}
