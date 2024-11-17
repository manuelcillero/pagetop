use crate::util::config::error::*;
use crate::util::config::path;
use crate::util::config::source::Source;
use crate::util::config::value::Value;

use serde::de::Deserialize;

use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug)]
enum ConfigKind {
    // A mutable configuration. This is the default.
    Mutable {
        defaults: HashMap<path::Expression, Value>,
        overrides: HashMap<path::Expression, Value>,
        sources: Vec<Box<dyn Source + Send + Sync>>,
    },
}

impl Default for ConfigKind {
    fn default() -> Self {
        ConfigKind::Mutable {
            defaults: HashMap::new(),
            overrides: HashMap::new(),
            sources: Vec::new(),
        }
    }
}

/// A prioritized configuration repository. It maintains a set of configuration sources, fetches
/// values to populate those, and provides them according to the source's priority.
#[derive(Default, Clone, Debug)]
pub struct ConfigData {
    kind: ConfigKind,
    /// Root of the cached configuration.
    pub cache: Value,
}

impl ConfigData {
    /// Merge in a configuration property source.
    pub fn merge<T>(&mut self, source: T) -> Result<&mut ConfigData>
    where
        T: 'static,
        T: Source + Send + Sync,
    {
        match self.kind {
            ConfigKind::Mutable {
                ref mut sources, ..
            } => {
                sources.push(Box::new(source));
            }
        }

        self.refresh()
    }

    /// Refresh the configuration cache with fresh data from added sources.
    ///
    /// Configuration is automatically refreshed after a mutation operation (`set`, `merge`,
    /// `set_default`, etc.).
    pub fn refresh(&mut self) -> Result<&mut ConfigData> {
        self.cache = match self.kind {
            // TODO: We need to actually merge in all the stuff.
            ConfigKind::Mutable {
                ref overrides,
                ref sources,
                ref defaults,
            } => {
                let mut cache: Value = HashMap::<String, Value>::new().into();

                // Add defaults.
                for (key, val) in defaults {
                    key.set(&mut cache, val.clone());
                }

                // Add sources.
                sources.collect_to(&mut cache)?;

                // Add overrides.
                for (key, val) in overrides {
                    key.set(&mut cache, val.clone());
                }

                cache
            }
        };

        Ok(self)
    }

    pub fn set_default<T>(&mut self, key: &str, value: T) -> Result<&mut ConfigData>
    where
        T: Into<Value>,
    {
        match self.kind {
            ConfigKind::Mutable {
                ref mut defaults, ..
            } => {
                defaults.insert(key.parse()?, value.into());
            }
        };

        self.refresh()
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> Result<&mut ConfigData>
    where
        T: Into<Value>,
    {
        match self.kind {
            ConfigKind::Mutable {
                ref mut overrides, ..
            } => {
                overrides.insert(key.parse()?, value.into());
            }
        };

        self.refresh()
    }

    /// Attempt to deserialize the entire configuration into the requested type.
    pub fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T> {
        T::deserialize(self)
    }
}

impl Source for ConfigData {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>> {
        self.cache.clone().into_table()
    }
}
