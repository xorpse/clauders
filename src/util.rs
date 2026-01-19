use schemars::JsonSchema;
use serde_json::Value;

pub(crate) fn schema_for<T: JsonSchema>() -> Value {
    let root = schemars::schema_for!(T);
    match serde_json::to_value(&root.schema) {
        Ok(v) => v,
        Err(_) => serde_json::json!({}),
    }
}

fn strip_schema_metadata(value: &mut Value) {
    if let Some(obj) = value.as_object_mut() {
        obj.remove("title");
        obj.remove("description");
        obj.remove("$schema");
        obj.remove("format");

        // Recursively process nested objects
        for (_, v) in obj.iter_mut() {
            strip_schema_metadata(v);
        }
    } else if let Some(arr) = value.as_array_mut() {
        for v in arr.iter_mut() {
            strip_schema_metadata(v);
        }
    }
}

pub(crate) fn schema_for_structured_output<T: JsonSchema>() -> Value {
    let root = schemars::schema_for!(T);
    match serde_json::to_value(&root.schema) {
        Ok(mut v) => {
            strip_schema_metadata(&mut v);
            v
        }
        Err(_) => serde_json::json!({}),
    }
}
