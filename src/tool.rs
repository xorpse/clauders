use std::future::Future;
use std::sync::Arc;

use futures::future::BoxFuture;
use schemars::JsonSchema;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value, json};
use thiserror::Error;

use crate::util;

#[derive(Error, Debug)]
pub enum ToolError {
    #[error("missing required parameter: {0}")]
    MissingParameter(String),
    #[error("invalid parameter '{name}': {reason}")]
    InvalidParameter { name: String, reason: String },
    #[error("execution failed: {0}")]
    ExecutionFailed(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("deserialization failed: {0}")]
    DeserializationFailed(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ToolError {
    pub fn missing_parameter(name: impl Into<String>) -> Self {
        Self::MissingParameter(name.into())
    }

    pub fn invalid_parameter(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidParameter {
            name: name.into(),
            reason: reason.into(),
        }
    }

    pub fn execution_failed(msg: impl Into<String>) -> Self {
        Self::ExecutionFailed(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn permission_denied(msg: impl Into<String>) -> Self {
        Self::PermissionDenied(msg.into())
    }

    pub fn deserialization_failed(msg: impl Into<String>) -> Self {
        Self::DeserializationFailed(msg.into())
    }

    pub fn other<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Other(anyhow::Error::new(err))
    }

    pub fn msg(msg: impl Into<String>) -> Self {
        Self::Other(anyhow::Error::msg(msg.into()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct ToolInput(Value);

impl ToolInput {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn empty() -> Self {
        Self(Value::Object(Map::new()))
    }

    pub fn as_value(&self) -> &Value {
        &self.0
    }

    pub fn into_value(self) -> Value {
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

pub struct Tool {
    name: String,
    description: String,
    input_schema: Value,
    output_schema: Option<Value>,
    handler: Arc<dyn Fn(ToolInput) -> BoxFuture<'static, Result<Value, ToolError>> + Send + Sync>,
}

impl std::fmt::Debug for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("input_schema", &self.input_schema)
            .field("output_schema", &self.output_schema)
            .field("handler", &"<fn>")
            .finish()
    }
}

impl Tool {
    pub fn new<F, Fut>(
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: Value,
        output_schema: impl Into<Option<Value>>,
        handler: F,
    ) -> Self
    where
        F: Fn(ToolInput) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Value, ToolError>> + Send + 'static,
    {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
            output_schema: output_schema.into(),
            handler: Arc::new(move |input| Box::pin(handler(input))),
        }
    }

    pub fn structured<T, U, F, Fut>(
        name: impl Into<String>,
        description: impl Into<String>,
        handler: F,
    ) -> Self
    where
        T: JsonSchema + DeserializeOwned + Send + 'static,
        U: JsonSchema + Serialize + 'static,
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<U, ToolError>> + Send + 'static,
    {
        let input_schema = util::schema_for::<T>();
        let output_schema = util::schema_for::<U>();
        let handler = Arc::new(handler);
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
            output_schema: Some(output_schema),
            handler: Arc::new(move |input: ToolInput| {
                let value = input.into_value();
                let deser_result = serde_json::from_value::<T>(value);
                let handler = Arc::clone(&handler);
                Box::pin(async move {
                    let typed = deser_result
                        .map_err(|e| ToolError::deserialization_failed(e.to_string()))?;
                    let output = handler(typed).await?;
                    serde_json::to_value(output)
                        .map_err(|e| ToolError::execution_failed(e.to_string()))
                })
            }),
        }
    }

    pub fn unstructured<T, F, Fut>(
        name: impl Into<String>,
        description: impl Into<String>,
        handler: F,
    ) -> Self
    where
        T: JsonSchema + DeserializeOwned + Send + 'static,
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Value, ToolError>> + Send + 'static,
    {
        let input_schema = util::schema_for::<T>();
        let handler = Arc::new(handler);
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
            output_schema: None,
            handler: Arc::new(move |input: ToolInput| {
                let value = input.into_value();
                let deser_result = serde_json::from_value::<T>(value);
                let handler = Arc::clone(&handler);
                Box::pin(async move {
                    let typed = deser_result
                        .map_err(|e| ToolError::deserialization_failed(e.to_string()))?;
                    handler(typed).await
                })
            }),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn input_schema(&self) -> &Value {
        &self.input_schema
    }

    pub fn output_schema(&self) -> Option<&Value> {
        self.output_schema.as_ref()
    }

    pub fn call(&self, input: ToolInput) -> BoxFuture<'static, Result<Value, ToolError>> {
        (self.handler)(input)
    }

    #[must_use]
    pub fn text_result(s: &str) -> Value {
        json!([{"type": "text", "text": s}])
    }

    #[must_use]
    pub fn error_result(s: &str) -> Value {
        json!([{"type": "text", "text": s, "is_error": true}])
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use schemars::JsonSchema;
    use serde::Deserialize;

    use super::*;

    #[test]
    fn test_schema_for_optional_fields() {
        #[derive(JsonSchema)]
        struct OptionalInput {
            required_field: String,
            optional_field: Option<String>,
        }

        let schema = util::schema_for::<OptionalInput>();
        let required = schema.get("required").and_then(|v| v.as_array());
        assert!(required.is_some());
        let required = required.unwrap();
        assert!(
            required
                .iter()
                .any(|v| v.as_str() == Some("required_field"))
        );
    }

    #[test]
    fn test_schema_for_nested_struct() {
        #[derive(JsonSchema)]
        struct Inner {
            value: i32,
        }

        #[derive(JsonSchema)]
        struct Outer {
            inner: Inner,
            name: String,
        }

        let schema = util::schema_for::<Outer>();
        let props = schema.get("properties").unwrap();
        assert!(props.get("inner").is_some());
        assert!(props.get("name").is_some());
    }

    #[test]
    fn test_schema_for_enum() {
        #[derive(JsonSchema)]
        #[serde(rename_all = "snake_case")]
        enum Status {
            Pending,
            Active,
            Completed,
        }

        #[derive(JsonSchema)]
        struct TaskInput {
            status: Status,
        }

        let schema = util::schema_for::<TaskInput>();
        assert!(schema.get("properties").is_some());
    }

    #[test]
    fn test_schema_for_vec() {
        #[derive(JsonSchema)]
        struct ListInput {
            items: Vec<String>,
            numbers: Vec<i32>,
        }

        let schema = util::schema_for::<ListInput>();
        let props = schema.get("properties").unwrap();

        let items_schema = props.get("items").unwrap();
        assert_eq!(
            items_schema.get("type").and_then(|v| v.as_str()),
            Some("array")
        );
    }

    #[test]
    fn test_schema_with_descriptions() {
        #[derive(JsonSchema)]
        struct DocumentedInput {
            #[schemars(description = "The user's full name")]
            name: String,
            #[schemars(description = "Age in years")]
            age: u32,
        }

        let schema = util::schema_for::<DocumentedInput>();
        let props = schema.get("properties").unwrap();

        let name_schema = props.get("name").unwrap();
        assert_eq!(
            name_schema.get("description").and_then(|v| v.as_str()),
            Some("The user's full name")
        );
    }

    #[test]
    fn test_schema_with_defaults() {
        #[derive(JsonSchema)]
        struct DefaultsInput {
            #[schemars(default = "default_name")]
            name: String,
        }

        fn default_name() -> String {
            "Anonymous".to_string()
        }

        let schema = util::schema_for::<DefaultsInput>();
        assert!(schema.get("properties").is_some());
    }

    #[test]
    fn test_typed_tool_creation() {
        #[derive(JsonSchema, Deserialize)]
        struct GreetInput {
            name: String,
        }

        let tool = Tool::unstructured("greet", "Greet a person", |input: GreetInput| async move {
            Ok(Tool::text_result(&format!("Hello, {}!", input.name)))
        });

        assert_eq!(tool.name(), "greet");
        assert_eq!(tool.description(), "Greet a person");

        let schema = tool.input_schema();
        assert_eq!(schema.get("type").and_then(|v| v.as_str()), Some("object"));
    }

    #[tokio::test]
    async fn test_typed_tool_execution() {
        #[derive(JsonSchema, Deserialize)]
        struct AddInput {
            a: i32,
            b: i32,
        }

        let tool = Tool::unstructured("add", "Add two numbers", |input: AddInput| async move {
            Ok(Tool::text_result(&format!("{}", input.a + input.b)))
        });

        let input = ToolInput::new(json!({"a": 5, "b": 3}));
        let result = tool.call(input).await.unwrap();

        let text = result
            .as_array()
            .and_then(|a| a.first())
            .and_then(|v| v.get("text"))
            .and_then(|v| v.as_str());
        assert_eq!(text, Some("8"));
    }

    #[tokio::test]
    async fn test_typed_tool_deserialization_error() {
        #[derive(JsonSchema, Deserialize)]
        struct StrictInput {
            required_field: String,
        }

        let tool = Tool::unstructured(
            "strict",
            "Requires field",
            |_input: StrictInput| async move { Ok(Tool::text_result("ok")) },
        );

        let input = ToolInput::new(json!({}));
        let result = tool.call(input).await;

        assert!(result.is_err());
        assert!(matches!(result, Err(ToolError::DeserializationFailed(_))));
    }

    #[test]
    fn test_text_result_format() {
        let result = Tool::text_result("Hello");
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);

        let item = &arr[0];
        assert_eq!(item.get("type").and_then(|v| v.as_str()), Some("text"));
        assert_eq!(item.get("text").and_then(|v| v.as_str()), Some("Hello"));
        assert!(item.get("is_error").is_none());
    }

    #[test]
    fn test_error_result_format() {
        let result = Tool::error_result("Something went wrong");
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);

        let item = &arr[0];
        assert_eq!(item.get("type").and_then(|v| v.as_str()), Some("text"));
        assert_eq!(
            item.get("text").and_then(|v| v.as_str()),
            Some("Something went wrong")
        );
        assert_eq!(item.get("is_error").and_then(|v| v.as_bool()), Some(true));
    }

    #[test]
    fn test_complex_nested_schema() {
        #[derive(JsonSchema)]
        struct Address {
            street: String,
            city: String,
            zip: Option<String>,
        }

        #[derive(JsonSchema)]
        struct Person {
            name: String,
            age: u32,
            addresses: Vec<Address>,
        }

        let schema = util::schema_for::<Person>();
        let props = schema.get("properties").unwrap();
        assert!(props.get("name").is_some());
        assert!(props.get("age").is_some());
        assert!(props.get("addresses").is_some());
    }

    #[test]
    fn test_tool_error_variants() {
        let err = ToolError::missing_parameter("name");
        assert!(err.to_string().contains("name"));

        let err = ToolError::invalid_parameter("age", "must be positive");
        assert!(err.to_string().contains("age"));
        assert!(err.to_string().contains("must be positive"));

        let err = ToolError::execution_failed("timeout");
        assert!(err.to_string().contains("timeout"));

        let err = ToolError::not_found("file.txt");
        assert!(err.to_string().contains("file.txt"));

        let err = ToolError::permission_denied("read access");
        assert!(err.to_string().contains("read access"));
    }

    #[test]
    fn test_weather_tool_schema_matches_claude_api() {
        #[derive(JsonSchema, Deserialize)]
        #[serde(rename_all = "lowercase")]
        enum TemperatureUnit {
            Celsius,
            Fahrenheit,
        }

        #[derive(JsonSchema, Deserialize)]
        struct GetWeatherInput {
            #[schemars(description = "The city and state, e.g. San Francisco, CA")]
            location: String,
            #[schemars(description = "The unit of temperature")]
            unit: Option<TemperatureUnit>,
        }

        let tool = Tool::unstructured(
            "get_weather",
            "Get the current weather in a given location",
            |_input: GetWeatherInput| async move { Ok(Tool::text_result("72°F")) },
        );

        assert_eq!(tool.name(), "get_weather");
        assert_eq!(
            tool.description(),
            "Get the current weather in a given location"
        );

        let schema = tool.input_schema();

        assert_eq!(schema.get("type").and_then(|v| v.as_str()), Some("object"));

        let props = schema.get("properties").expect("should have properties");

        let location = props.get("location").expect("should have location");
        assert_eq!(
            location.get("type").and_then(|v| v.as_str()),
            Some("string")
        );
        assert_eq!(
            location.get("description").and_then(|v| v.as_str()),
            Some("The city and state, e.g. San Francisco, CA")
        );

        let required = schema
            .get("required")
            .and_then(|v| v.as_array())
            .expect("should have required array");
        assert!(required.iter().any(|v| v.as_str() == Some("location")));
        assert!(!required.iter().any(|v| v.as_str() == Some("unit")));
    }

    #[test]
    fn test_enum_generates_enum_values() {
        #[derive(JsonSchema)]
        #[serde(rename_all = "lowercase")]
        enum Color {
            Red,
            Green,
            Blue,
        }

        #[derive(JsonSchema)]
        struct ColorInput {
            color: Color,
        }

        let schema = util::schema_for::<ColorInput>();
        let defs = schema.get("definitions").or_else(|| schema.get("$defs"));

        if let Some(defs) = defs {
            let color_def = defs.get("Color");
            if let Some(color_def) = color_def {
                if let Some(enum_values) = color_def.get("enum").and_then(|v| v.as_array()) {
                    let values: Vec<&str> = enum_values.iter().filter_map(|v| v.as_str()).collect();
                    assert!(values.contains(&"red"));
                    assert!(values.contains(&"green"));
                    assert!(values.contains(&"blue"));
                }
            }
        }
    }

    #[test]
    fn test_tool_with_multiple_required_fields() {
        #[derive(JsonSchema, Deserialize)]
        struct MultiRequiredInput {
            #[schemars(description = "First required field")]
            field_a: String,
            #[schemars(description = "Second required field")]
            field_b: i32,
            #[schemars(description = "Optional field")]
            field_c: Option<bool>,
        }

        let schema = util::schema_for::<MultiRequiredInput>();

        let required = schema
            .get("required")
            .and_then(|v| v.as_array())
            .expect("should have required");

        assert!(required.iter().any(|v| v.as_str() == Some("field_a")));
        assert!(required.iter().any(|v| v.as_str() == Some("field_b")));
        assert!(!required.iter().any(|v| v.as_str() == Some("field_c")));
    }

    #[tokio::test]
    async fn test_typed_tool_with_weather_input() {
        #[derive(JsonSchema, Deserialize)]
        struct WeatherInput {
            location: String,
            unit: Option<String>,
        }

        let tool = Tool::unstructured(
            "get_weather",
            "Get weather",
            |input: WeatherInput| async move {
                Ok(Tool::text_result(&format!("Weather in {}", input.location)))
            },
        );

        let input = ToolInput::new(json!({"location": "San Francisco, CA"}));
        let result = tool.call(input).await.unwrap();
        let text = result
            .as_array()
            .and_then(|a| a.first())
            .and_then(|v| v.get("text"))
            .and_then(|v| v.as_str());
        assert_eq!(text, Some("Weather in San Francisco, CA"));

        let input_with_unit = ToolInput::new(json!({"location": "NYC", "unit": "celsius"}));
        let result = tool.call(input_with_unit).await.unwrap();
        let text = result
            .as_array()
            .and_then(|a| a.first())
            .and_then(|v| v.get("text"))
            .and_then(|v| v.as_str());
        assert_eq!(text, Some("Weather in NYC"));
    }

    #[test]
    fn test_schema_integer_types() {
        #[derive(JsonSchema)]
        struct NumberInput {
            count: i32,
            amount: i64,
            small: u8,
        }

        let schema = util::schema_for::<NumberInput>();
        let props = schema.get("properties").unwrap();

        let count = props.get("count").unwrap();
        assert_eq!(count.get("type").and_then(|v| v.as_str()), Some("integer"));

        let amount = props.get("amount").unwrap();
        assert_eq!(amount.get("type").and_then(|v| v.as_str()), Some("integer"));
    }

    #[test]
    fn test_schema_boolean_type() {
        #[derive(JsonSchema)]
        struct FlagInput {
            enabled: bool,
            verbose: Option<bool>,
        }

        let schema = util::schema_for::<FlagInput>();
        let props = schema.get("properties").unwrap();

        let enabled = props.get("enabled").unwrap();
        assert_eq!(
            enabled.get("type").and_then(|v| v.as_str()),
            Some("boolean")
        );
    }

    #[test]
    fn test_schema_array_of_objects() {
        #[derive(JsonSchema)]
        struct Item {
            id: i32,
            name: String,
        }

        #[derive(JsonSchema)]
        struct ListInput {
            items: Vec<Item>,
        }

        let schema = util::schema_for::<ListInput>();
        let props = schema.get("properties").unwrap();

        let items = props.get("items").unwrap();
        assert_eq!(items.get("type").and_then(|v| v.as_str()), Some("array"));
    }
}
