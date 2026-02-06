use schemars::JsonSchema;
use serde_json::Value;

pub(crate) fn schema_for<T: JsonSchema>() -> Value {
    let root = schemars::schema_for!(T);
    match serde_json::to_value(&root.schema) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!("failed to serialize schema: {}", e);
            serde_json::json!({})
        }
    }
}
