use serde_json::{Map, Value};

#[derive(Debug, Clone, Default)]
pub struct ToolInput(Value);

impl ToolInput {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn from_value(value: Value) -> Self {
        Self(value)
    }

    pub fn empty() -> Self {
        Self(Value::Object(Map::new()))
    }

    pub fn as_value(&self) -> &Value {
        &self.0
    }

    pub fn as_json(&self) -> &Value {
        &self.0
    }

    pub fn into_value(self) -> Value {
        self.0
    }

    pub fn into_json(self) -> Value {
        self.0
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.0.get(key)?.as_str()
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.0.get(key)?.as_i64()
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.0.get(key)?.as_f64()
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.0.get(key)?.as_bool()
    }

    pub fn get_string_list(&self, key: &str) -> Option<Vec<&str>> {
        let arr = self.0.get(key)?.as_array()?;
        arr.iter().map(|v| v.as_str()).collect()
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn keys(&self) -> Vec<&str> {
        match &self.0 {
            Value::Object(map) => map.keys().map(|s| s.as_str()).collect(),
            _ => vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.0 {
            Value::Object(map) => map.is_empty(),
            _ => true,
        }
    }

    pub fn set(mut self, key: impl Into<String>, value: Value) -> Self {
        if let Value::Object(ref mut map) = self.0 {
            map.insert(key.into(), value);
        }
        self
    }

    pub fn set_string(self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.set(key, Value::String(value.into()))
    }

    pub fn set_i64(self, key: impl Into<String>, value: i64) -> Self {
        self.set(key, Value::Number(value.into()))
    }

    pub fn set_bool(self, key: impl Into<String>, value: bool) -> Self {
        self.set(key, Value::Bool(value))
    }

    pub fn from_pairs(
        pairs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        let map = pairs
            .into_iter()
            .map(|(k, v)| (k.into(), Value::String(v.into())))
            .collect::<Map<_, _>>();
        Self(Value::Object(map))
    }
}

impl From<Value> for ToolInput {
    fn from(value: Value) -> Self {
        Self::new(value)
    }
}

impl From<ToolInput> for Value {
    fn from(input: ToolInput) -> Self {
        input.0
    }
}
