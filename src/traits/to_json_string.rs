use std::collections::BTreeMap;
use std::collections::HashMap;

pub trait ToJsonString {
    fn to_json_string(&self) -> String;
}

impl<K, V> ToJsonString for HashMap<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn to_json_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl<K, V> ToJsonString for BTreeMap<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn to_json_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToJsonString for serde_json::Map<String, serde_json::Value> {
    fn to_json_string(&self) -> String {
        let value = serde_json::Value::Object(self.to_owned());
        format!("{}", value)
    }
}
