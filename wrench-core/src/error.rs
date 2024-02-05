use core::fmt;

use crate::ConfigKey;

/// Error returned when attempting to retrieve a value from a [path](ConfigKey) that does not exist.
pub struct ConfigValueError {
    path: ConfigKey,
}

impl ConfigValueError {
    pub fn new() -> ConfigValueError {
        ConfigValueError {
            path: Default::default(),
        }
    }
}

impl fmt::Display for ConfigValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("No value found at path '{}'", self.path))
    }
}
