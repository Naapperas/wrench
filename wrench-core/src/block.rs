use std::collections::{HashMap, VecDeque};

use crate::{ConfigKey, ConfigValue, ConfigValueContainer};

type Values = HashMap<ConfigKey, ConfigValue>;

/// A block containing various [configuration values](ConfigValues)
#[derive(PartialEq, Debug, Clone)]
pub struct ConfigBlock {
    values: Values,
}

impl ConfigBlock {
    /// Creates a blank [ConfigBlock] object.
    pub fn new() -> ConfigBlock {
        ConfigBlock {
            values: Default::default(),
        }
    }

    /// Constructs a new [ConfigBlock] object from the values provided.
    pub fn from(values: Values) -> ConfigBlock {
        ConfigBlock { values }
    }

    /// Internal implementation of the *get* method which takes a [VecDeque] of the parts that make up the original [ConfigKey].
    ///
    /// The naïve approach would be to reconstruct a config key from the remaining parts but that could potentially lead to performance issues.
    fn internal_get(&self, mut parts: VecDeque<ConfigKey>) -> Option<&ConfigValue> {
        match parts.pop_front() {
            Some(first_path_part) => match self.values.get(&first_path_part) {
                Some(value) => match value {
                    ConfigValue::ObjectValue(object_map) if parts.len() > 0 => {
                        object_map.internal_get(parts)
                    }
                    _ => Some(value),
                },
                None => None,
            },
            None => None,
        }
    }

    /// Internal implementation of the *get* method which takes a [VecDeque] of the parts that make up the original [ConfigKey].
    ///
    /// The naïve approach would be to reconstruct a config key from the remaining parts but that could potentially lead to performance issues.
    fn internal_put(
        &self,
        mut parts: VecDeque<ConfigKey>,
        value: ConfigValue,
    ) -> Option<&ConfigValue> {
        todo!();
    }
}

impl ConfigValueContainer for ConfigBlock {
    fn get(&self, key: ConfigKey) -> Option<&ConfigValue> {
        let parts = key.into_parts();

        self.internal_get(parts)
    }

    fn put(&mut self, key: ConfigKey, value: ConfigValue) -> Option<&ConfigValue> {
        let parts = key.into_parts();

        self.internal_put(parts, value)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn it_works() {
        let test_key: ConfigKey = "test_key".into();
        let test_value = ConfigValue::BooleanValue(true);

        let block = ConfigBlock::from(HashMap::from([(test_key.clone(), test_value.clone())]));

        let value = block.get(test_key).unwrap();

        assert_eq!(*value, test_value)
    }
}
