use crate::block::ConfigBlock;

/// Configuration value that can be retrieved from a [Config] or [ConfigBlock] value.
#[derive(PartialEq, Debug, Clone)]
pub enum ConfigValue {
    StringValue(String),
    NumberValue(f64),
    BooleanValue(bool),
    ListValue(Vec<ConfigValue>),
    ObjectValue(ConfigBlock),
}

impl From<String> for ConfigValue {
    fn from(value: String) -> Self {
        ConfigValue::StringValue(value)
    }
}

impl From<f64> for ConfigValue {
    fn from(value: f64) -> Self {
        ConfigValue::NumberValue(value)
    }
}

impl From<bool> for ConfigValue {
    fn from(value: bool) -> Self {
        ConfigValue::BooleanValue(value)
    }
}
