use serde::{Deserialize, Deserializer};

use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum HljsMode {
    Core,
    Common,
}

impl ToString for HljsMode {
    fn to_string(&self) -> String {
        String::from(match self {
            HljsMode::Core => "core",
            HljsMode::Common => "common",
        })
    }
}

impl FromStr for HljsMode {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "core" => Ok(HljsMode::Core),
            "common" => Ok(HljsMode::Common),
            _ => Err(fmt::Error),
        }
    }
}

impl<'de> Deserialize<'de> for HljsMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        HljsMode::from_str(&s).map_err(serde::de::Error::custom)
    }
}
