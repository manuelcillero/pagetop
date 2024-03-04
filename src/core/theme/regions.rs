use crate::core::component::{MixedComponents, OneComponent, OneOp};
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
    pub fn new(region: &'static str, one: OneComponent) -> Self {
        let mut regions = ComponentsInRegions::default();
        regions.add_in(region, one);
        regions
    }

    pub fn add_in(&mut self, region: &'static str, one: OneComponent) {
        if let Some(region) = self.0.get_mut(region) {
            region.alter_value(OneOp::Add(one));
        } else {
            self.0.insert(region, MixedComponents::new(one));
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
    pub fn add(&self, one: OneComponent) -> &Self {
        match self {
            InRegion::Content => {
                COMMON_REGIONS.write().unwrap().add_in("content", one);
            }
            InRegion::Named(name) => {
                COMMON_REGIONS.write().unwrap().add_in(name, one);
            }
            InRegion::OfTheme(region, theme) => {
                let mut regions = THEME_REGIONS.write().unwrap();
                if let Some(r) = regions.get_mut(&theme.type_id()) {
                    r.add_in(region, one);
                } else {
                    regions.insert(theme.type_id(), ComponentsInRegions::new(region, one));
                }
            }
        }
        self
    }
}
