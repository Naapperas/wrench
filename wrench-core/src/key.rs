use core::fmt;
use std::collections::VecDeque;

/// A key used to index a [Config] object and its associated [ConfigBlock]s.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ConfigKey(String);

impl ConfigKey {
    /// Splits the given config key into its parts to support nested retrieval
    pub fn into_parts(&self) -> VecDeque<ConfigKey> {
        self.0.split(".").map(|s| s.into()).collect()
    }
}

impl From<String> for ConfigKey {
    fn from(value: String) -> Self {
        ConfigKey(value)
    }
}

impl From<&str> for ConfigKey {
    fn from(value: &str) -> Self {
        ConfigKey(value.into())
    }
}

impl Default for ConfigKey {
    fn default() -> Self {
        ConfigKey("_root_".into())
    }
}

impl fmt::Display for ConfigKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}
