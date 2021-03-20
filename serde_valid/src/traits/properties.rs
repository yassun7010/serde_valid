use std::collections::BTreeMap;
use std::collections::HashMap;

pub trait Properties {
    fn to_string(&self) -> String;
}

impl<K, V> Properties for HashMap<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl<K, V> Properties for BTreeMap<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Properties for serde_json::Map<String, serde_json::Value> {
    fn to_string(&self) -> String {
        let value = serde_json::Value::Object(self.to_owned());
        format!("{}", value)
    }
}
