pub mod block;
pub mod config;
pub mod error;
pub mod key;
pub mod value;

use std::path::Path;

// Re-export to avoid unnecessary prefixes generated from separating these types.
use block::ConfigBlock;
use config::Config;
use key::ConfigKey;
use value::ConfigValue;

pub trait ConfigValueContainer {
    /// Returns the configuration value associated with the given key, or [None] if the key is not present
    fn get(&self, key: ConfigKey) -> Option<&ConfigValue>;

    /// Stores a value under the given key in this container and returns the old value, if it existed.
    fn put(&mut self, key: ConfigKey, value: ConfigValue) -> Option<&ConfigValue>;
}

/// A [ConfigReader] is a object responsible for reading configuration values from a given file and returning a formatted object representation of those values.
pub trait ConfigReader {
    /// Parses the configuration file at the given path and returns a [Config] object populated with the correct configuration values.
    fn parse(&self, path: Path) -> Config; // TODO: make into PathLike object
}

fn reader(config_reader_name: &str) -> Box<dyn ConfigReader> {
    todo!()
}
