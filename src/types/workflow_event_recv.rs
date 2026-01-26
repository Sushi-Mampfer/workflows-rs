use serde::Serialize;
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct WorkflowEventRecv {
    #[serde(rename = "type")]
    pub type_name: String,
    pub timeout: String,
}

impl WorkflowEventRecv {
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

impl TryFrom<&WorkflowEventRecv> for JsValue {
    type Error = serde_wasm_bindgen::Error;

    fn try_from(event: &WorkflowEventRecv) -> Result<Self, Self::Error> {
        event.serialize()
    }
}
