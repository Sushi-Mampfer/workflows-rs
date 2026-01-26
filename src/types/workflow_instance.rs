use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::{JsCast, JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::JsFuture;

use crate::WorkflowEventSend;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    #[derive(Clone, PartialEq, Eq)]
    pub type WorkflowInstance;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &WorkflowInstance) -> String;

    #[wasm_bindgen(method)]
    fn pause(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    fn resume(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    fn terminate(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    fn restart(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    fn status(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "sendEvent")]
    fn send_event_internal(this: &WorkflowInstance, options: JsValue) -> js_sys::Promise;
}

impl WorkflowInstance {
    pub async fn send_event<T: Serialize + DeserializeOwned>(
        &self,
        options: WorkflowEventSend<T>,
    ) -> Result<(), String> {
        let promise = self.send_event_internal(
            options
                .serialize()
                .map_err(|_| "Failed to serialize options".to_string())?,
        );
        match JsFuture::from(promise).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let error_msg = if let Some(s) = e.as_string() {
                    s
                } else if e.is_instance_of::<js_sys::Error>() {
                    let err: js_sys::Error = e.unchecked_into();
                    err.message().into()
                } else {
                    format!("{:?}", e)
                };
                Err(error_msg)
            }
        }
    }
}
