use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize)]
pub(super) struct MethodCall<'a> {
    pub(super) method: &'a str,
    pub(super) data: Value,
}

impl<'a> MethodCall<'a> {
    pub(super) fn new(method: &'a str, data: impl Into<Value>) -> Self {
        Self {
            method,
            data: data.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct BindingRegistrationResponse {
    #[serde(rename = "binding-id")]
    pub(super) binding_id: u64,
}

#[derive(Debug, Deserialize)]
pub(super) struct WayfireEvent {
    pub(super) event: String,
    #[serde(rename = "binding-id")]
    pub(super) binding_id: Option<u64>,
}

pub(super) fn object(entries: impl IntoIterator<Item = (&'static str, Value)>) -> Value {
    Value::Object(
        entries
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect::<Map<_, _>>(),
    )
}
