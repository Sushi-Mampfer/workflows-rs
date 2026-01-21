use js_sys::Promise;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::WorkflowEventOptions;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    #[derive(Clone, PartialEq, Eq)]
    pub type WorkflowInstance;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &WorkflowInstance) -> String;

    #[wasm_bindgen(method)]
    pub fn pause(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    pub fn resume(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    pub fn terminate(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    pub fn restart(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "sendEvent")]
    fn send_event_internal(this: &WorkflowInstance, options: JsValue) -> js_sys::Promise;
}

impl WorkflowInstance {
    pub fn send_event(
        &self,
        options: WorkflowEventOptions,
    ) -> Result<Promise, serde_wasm_bindgen::Error> {
        Ok(self.send_event_internal(options.serialize()?))
    }
}
