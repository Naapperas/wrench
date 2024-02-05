use crate::{ConfigBlock, ConfigKey, ConfigValue, ConfigValueContainer};

/// The configuration object parsed and populated from a configuration file.
pub struct Config {
    /// The root [configuration block](ConfigBlock) of this [Config] object.
    root: ConfigBlock,
}

impl Config {
    /// Creates a blank [Config] object.
    pub fn new() -> Config {
        Config {
            root: ConfigBlock::new(),
        }
    }
}

impl ConfigValueContainer for Config {
    fn get(&self, key: ConfigKey) -> Option<&ConfigValue> {
        // We just need to pass this onto the root configuration block.
        self.root.get(key)
    }

    fn put(&mut self, key: ConfigKey, value: ConfigValue) -> Option<&ConfigValue> {
        // We just need to pass this onto the root configuration block.
        self.root.put(key, value)
    }
}
