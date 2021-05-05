use std::collections::BTreeMap;
use std::collections::HashMap;

pub trait ToJson {
    fn to_json(&self) -> String;
}

impl<K, V> ToJson for HashMap<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl<K, V> ToJson for BTreeMap<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn to_json(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJson for serde_json::Map<String, serde_json::Value> {
    fn to_json(&self) -> String {
        let value = serde_json::Value::Object(self.clone());
        format!("{}", value)
    }
}
