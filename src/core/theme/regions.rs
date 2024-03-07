use crate::core::component::{AnyComponent, MixedComponents, MixedOp};
use crate::core::theme::ThemeRef;
use crate::{AutoDefault, LazyStatic, TypeId};

use std::collections::HashMap;
use std::sync::RwLock;

static THEME_REGIONS: LazyStatic<RwLock<HashMap<TypeId, ComponentsInRegions>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

static COMMON_REGIONS: LazyStatic<RwLock<ComponentsInRegions>> =
    LazyStatic::new(|| RwLock::new(ComponentsInRegions::default()));

#[derive(AutoDefault)]
pub struct ComponentsInRegions(HashMap<&'static str, MixedComponents>);

impl ComponentsInRegions {
    pub fn new(region: &'static str, any: AnyComponent) -> Self {
        let mut regions = ComponentsInRegions::default();
        regions.add_in(region, any);
        regions
    }

    pub fn add(&mut self, any: AnyComponent) {
        if let Some(region) = self.0.get_mut("content") {
            region.alter_value(MixedOp::Add(any));
        } else {
            self.0.insert("content", MixedComponents::new(any));
        }
    }

    pub fn add_in(&mut self, region: &'static str, any: AnyComponent) {
        if let Some(region) = self.0.get_mut(region) {
            region.alter_value(MixedOp::Add(any));
        } else {
            self.0.insert(region, MixedComponents::new(any));
        }
    }

    pub fn get_components(&self, theme: ThemeRef, region: &str) -> MixedComponents {
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
                COMMON_REGIONS.write().unwrap().add(any);
            }
            InRegion::Named(name) => {
                COMMON_REGIONS.write().unwrap().add_in(name, any);
            }
            InRegion::OfTheme(region, theme) => {
                let mut regions = THEME_REGIONS.write().unwrap();
                if let Some(r) = regions.get_mut(&theme.type_id()) {
                    r.add_in(region, any);
                } else {
                    regions.insert(theme.type_id(), ComponentsInRegions::new(region, any));
                }
            }
        }
        self
    }
}
