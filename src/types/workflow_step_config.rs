use serde::Serialize;
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct WorkflowStepConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<Retries>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Serialize)]
pub struct Retries {
    pub limit: i32,
    pub delay: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff: Option<String>,
}

impl WorkflowStepConfig {
    pub fn serialize(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(self)
    }
}
