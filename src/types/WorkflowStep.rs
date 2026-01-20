use wasm_bindgen::prelude::*;

use crate::types::WorkflowEventOptions::WorkflowEventOptions;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Clone, PartialEq, Eq)]
    pub type WorkflowStep;

    #[wasm_bindgen(method, js_name=do)]
    pub fn do_argless(
        this: &WorkflowStep,
        name: String,
        callback: &dyn Fn() -> JsValue,
    ) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name=do)]
    pub fn do_with_option(
        this: &WorkflowStep,
        name: String,
        config: String,
        callback: &dyn Fn() -> JsValue,
    ) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    pub fn sleep(this: &WorkflowStep, name: String, duration: String) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = sleepUntil)]
    pub fn sleep_until(this: &WorkflowStep, name: String, timestamp: i32) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = waitForEvent)]
    fn wait_for_event_internal(
        this: &WorkflowStep,
        name: String,
        options: JsValue,
    ) -> js_sys::Promise;
}

impl WorkflowStep {
    pub fn wait_for_event(
        &self,
        name: String,
        options: &WorkflowEventOptions,
    ) -> Result<js_sys::Promise, serde_wasm_bindgen::Error> {
        Ok(self.wait_for_event_internal(name, options.serialize()?))
    }
}
