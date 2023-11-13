use crate::core::component::{AnyComponents, ArcAnyComponent, ArcAnyOp};
use crate::core::theme::ThemeRef;
use crate::{Handle, LazyStatic};

use std::collections::HashMap;
use std::sync::RwLock;

static THEME_REGIONS: LazyStatic<RwLock<HashMap<Handle, ComponentsRegions>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

static COMMON_REGIONS: LazyStatic<RwLock<ComponentsRegions>> =
    LazyStatic::new(|| RwLock::new(ComponentsRegions::default()));

#[derive(Default)]
pub struct ComponentsRegions(HashMap<&'static str, AnyComponents>);

impl ComponentsRegions {
    pub fn new(region: &'static str, arc: ArcAnyComponent) -> Self {
        let mut regions = ComponentsRegions::default();
        regions.add_in(region, arc);
        regions
    }

    pub fn add_in(&mut self, region: &'static str, arc: ArcAnyComponent) {
        if let Some(region) = self.0.get_mut(region) {
            region.alter_value(ArcAnyOp::Add(arc));
        } else {
            self.0.insert(region, AnyComponents::new(arc));
        }
    }

    pub fn get_components(&self, theme: ThemeRef, region: &str) -> AnyComponents {
        let common = COMMON_REGIONS.read().unwrap();
        if let Some(hm) = THEME_REGIONS.read().unwrap().get(&theme.handle()) {
            AnyComponents::merge(&[common.0.get(region), self.0.get(region), hm.0.get(region)])
        } else {
            AnyComponents::merge(&[common.0.get(region), self.0.get(region)])
        }
    }
}

pub enum Region {
    Named(&'static str),
    OfTheme(ThemeRef, &'static str),
}

pub fn add_component_in(region: Region, arc: ArcAnyComponent) {
    match region {
        Region::Named(name) => {
            COMMON_REGIONS.write().unwrap().add_in(name, arc);
        }
        Region::OfTheme(theme, region) => {
            let mut regions = THEME_REGIONS.write().unwrap();
            if let Some(hm) = regions.get_mut(&theme.handle()) {
                hm.add_in(region, arc);
            } else {
                regions.insert(theme.handle(), ComponentsRegions::new(region, arc));
            }
        }
    }
}
