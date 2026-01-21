use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub struct WorkflowInstanceCreateOptions<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,
}

impl<T: Serialize> WorkflowInstanceCreateOptions<T> {
    pub fn new(id: Option<String>, params: Option<T>) -> Self {
        Self { id, params }
    }

    pub fn serialize(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(self)
    }
}

impl<T: Serialize> TryFrom<&WorkflowInstanceCreateOptions<T>> for JsValue {
    type Error = serde_wasm_bindgen::Error;

    fn try_from(options: &WorkflowInstanceCreateOptions<T>) -> Result<Self, Self::Error> {
        options.serialize()
    }
}
