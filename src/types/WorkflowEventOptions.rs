use serde::Serialize;
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct WorkflowEventOptions {
    #[serde(rename = "type")]
    pub type_name: String,
    pub timeout: String,
}

impl WorkflowEventOptions {
    pub fn new(type_name: impl Into<String>, timeout: impl Into<String>) -> Self {
        Self {
            type_name: type_name.into(),
            timeout: timeout.into(),
        }
    }

    pub fn serialize(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(self)
    }
}

impl TryFrom<&WorkflowEventOptions> for JsValue {
    type Error = serde_wasm_bindgen::Error;

    fn try_from(event: &WorkflowEventOptions) -> Result<Self, Self::Error> {
        event.serialize()
    }
}
