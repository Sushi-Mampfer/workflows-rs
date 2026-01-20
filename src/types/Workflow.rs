use wasm_bindgen::{JsCast, JsValue, prelude::wasm_bindgen};
use worker::wasm_bindgen_futures;

use crate::types::{
    WorkflowInstance::WorkflowInstance,
    WorkflowInstanceCreateOptions::WorkflowInstanceCreateOptions,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Clone, PartialEq, Eq)]
    pub type Workflow;

    #[wasm_bindgen(method, js_name = "create")]
    fn create_argless_internal(this: &Workflow) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "create")]
    fn create_with_options_internal(this: &Workflow, options: JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "createBatch")]
    fn create_batch_internal(this: &Workflow, batch: &js_sys::Array) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "get")]
    fn get_internal(this: &Workflow, id: String) -> js_sys::Promise;
}

impl Workflow {
    pub async fn create(
        &self,
        options: Option<WorkflowInstanceCreateOptions>,
    ) -> Result<WorkflowInstance, JsValue> {
        let promise = match options {
            Some(options) => self.create_with_options_internal(options.serialize()?),
            None => self.create_argless_internal(),
        };
        match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(instance) => Ok(instance.dyn_into()?),
            Err(e) => Err(e),
        }
    }

    pub async fn crate_with_options(
        &self,
        batch: Vec<WorkflowInstanceCreateOptions>,
    ) -> Result<Vec<WorkflowInstance>, JsValue> {
        let promise = self.create_batch_internal(&js_sys::Array::from_iter(
            batch
                .iter()
                .map(|o| o.serialize())
                .collect::<Result<Vec<JsValue>, serde_wasm_bindgen::Error>>()?,
        ));
        match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(instance) => instance
                .dyn_into::<js_sys::Array>()?
                .iter()
                .map(|i| i.dyn_into())
                .collect::<Result<Vec<WorkflowInstance>, JsValue>>(),
            Err(e) => Err(e),
        }
    }

    pub async fn get(&self, id: String) -> Result<WorkflowInstance, JsValue> {
        let promise = self.get_internal(id);
        match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(instance) => Ok(instance.dyn_into()?),
            Err(e) => Err(e),
        }
    }
}
