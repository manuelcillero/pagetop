use crate::config::error::*;
use crate::config::path;
use crate::config::source::Source;
use crate::config::value::{Table, Value};

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
    pub fn new() -> Self {
        Self {
            kind: ConfigKind::default(),
            // Config root should be instantiated as an empty table to avoid deserialization errors.
            cache: Value::new(None, Table::new()),
        }
    }

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

    /// Merge in a configuration property source.
    pub fn with_merged<T>(mut self, source: T) -> Result<Self>
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

        self.refresh()?;
        Ok(self)
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

    pub fn set_once(&mut self, key: &str, value: Value) -> Result<()> {
        let expr: path::Expression = key.parse()?;

        // Traverse the cache using the path to (possibly) retrieve a value.
        if let Some(ref mut val) = expr.get_mut(&mut self.cache) {
            **val = value;
        } else {
            expr.set(&mut self.cache, value);
        }
        Ok(())
    }

    pub fn get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<T> {
        // Parse the key into a path expression.
        let expr: path::Expression = key.parse()?;

        // Traverse the cache using the path to (possibly) retrieve a value.
        let value = expr.get(&self.cache).cloned();

        match value {
            Some(value) => {
                // Deserialize the received value into the requested type.
                T::deserialize(value).map_err(|e| e.extend_with_key(key))
            }

            None => Err(ConfigError::NotFound(key.into())),
        }
    }

    pub fn get_str(&self, key: &str) -> Result<String> {
        self.get(key).and_then(Value::into_str)
    }

    pub fn get_int(&self, key: &str) -> Result<i64> {
        self.get(key).and_then(Value::into_int)
    }

    pub fn get_float(&self, key: &str) -> Result<f64> {
        self.get(key).and_then(Value::into_float)
    }

    pub fn get_bool(&self, key: &str) -> Result<bool> {
        self.get(key).and_then(Value::into_bool)
    }

    pub fn get_table(&self, key: &str) -> Result<HashMap<String, Value>> {
        self.get(key).and_then(Value::into_table)
    }

    pub fn get_array(&self, key: &str) -> Result<Vec<Value>> {
        self.get(key).and_then(Value::into_array)
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
