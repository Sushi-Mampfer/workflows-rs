use serde::Serialize;
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct WorkflowInstanceCreateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<JsValue>,
}

impl WorkflowInstanceCreateOptions {
    pub fn new(id: Option<String>, params: Option<JsValue>) -> Self {
        Self { id, params }
    }

    pub fn serialize(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(self)
    }
}

impl TryFrom<&WorkflowInstanceCreateOptions> for JsValue {
    type Error = serde_wasm_bindgen::Error;

    fn try_from(options: &WorkflowInstanceCreateOptions) -> Result<Self, Self::Error> {
        options.serialize()
    }
}
