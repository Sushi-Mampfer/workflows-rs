use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct WorkflowEventSend<T: Serialize + DeserializeOwned> {
    #[serde(rename = "type")]
    pub type_name: String,
    pub payload: T,
}

impl<T: Serialize + DeserializeOwned> WorkflowEventSend<T> {
    pub fn new(type_name: impl Into<String>, payload: T) -> Self {
        Self {
            type_name: type_name.into(),
            payload,
        }
    }

    pub fn serialize(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(self)
    }
}

impl<T: Serialize + DeserializeOwned> TryFrom<&WorkflowEventSend<T>> for JsValue {
    type Error = serde_wasm_bindgen::Error;

    fn try_from(event: &WorkflowEventSend<T>) -> Result<Self, Self::Error> {
        event.serialize()
    }
}
