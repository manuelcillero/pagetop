use crate::core::component::{ArcComponent, ArcComponents, ArcOp};
use crate::core::theme::ThemeRef;
use crate::{Handle, LazyStatic};

use std::collections::HashMap;
use std::sync::RwLock;

static THEME_REGIONS: LazyStatic<RwLock<HashMap<Handle, ComponentsRegions>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

static COMMON_REGIONS: LazyStatic<RwLock<ComponentsRegions>> =
    LazyStatic::new(|| RwLock::new(ComponentsRegions::new()));

#[derive(Default)]
pub struct ComponentsRegions(HashMap<&'static str, ArcComponents>);

impl ComponentsRegions {
    pub fn new() -> Self {
        ComponentsRegions::default()
    }

    pub fn with(region: &'static str, arc: ArcComponent) -> Self {
        let mut regions = ComponentsRegions::new();
        regions.add_in(region, arc);
        regions
    }

    pub fn add_in(&mut self, region: &'static str, arc: ArcComponent) {
        if let Some(region) = self.0.get_mut(region) {
            region.alter(ArcOp::Add(arc));
        } else {
            self.0.insert(region, ArcComponents::with(arc));
        }
    }

    pub fn get_components(&self, theme: ThemeRef, region: &str) -> ArcComponents {
        let common = COMMON_REGIONS.read().unwrap();
        if let Some(hm) = THEME_REGIONS.read().unwrap().get(&theme.handle()) {
            ArcComponents::merge(&[common.0.get(region), self.0.get(region), hm.0.get(region)])
        } else {
            ArcComponents::merge(&[common.0.get(region), self.0.get(region)])
        }
    }
}

pub enum Region {
    Named(&'static str),
    OfTheme(ThemeRef, &'static str),
}

pub fn add_component_in(region: Region, arc: ArcComponent) {
    match region {
        Region::Named(name) => {
            COMMON_REGIONS.write().unwrap().add_in(name, arc);
        }
        Region::OfTheme(theme, region) => {
            let mut regions = THEME_REGIONS.write().unwrap();
            if let Some(hm) = regions.get_mut(&theme.handle()) {
                hm.add_in(region, arc);
            } else {
                regions.insert(theme.handle(), ComponentsRegions::with(region, arc));
            }
        }
    }
}
