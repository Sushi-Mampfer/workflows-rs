use std::rc::Rc;

use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};

use crate::{WorkflowEventRecv, WorkflowStepConfig};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    #[derive(Clone, PartialEq, Eq)]
    pub type WorkflowStep;

    #[wasm_bindgen(method, js_name=do)]
    fn do_argless(
        this: &WorkflowStep,
        name: String,
        callback: &Closure<dyn Fn() -> js_sys::Promise>,
    ) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name=do)]
    fn do_with_option(
        this: &WorkflowStep,
        name: String,
        config: JsValue,
        callback: &Closure<dyn Fn() -> js_sys::Promise>,
    ) -> js_sys::Promise;

    #[wasm_bindgen(method)]
    fn sleep_internal(this: &WorkflowStep, name: String, duration: String) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = sleepUntil)]
    fn sleep_until_internal(this: &WorkflowStep, name: String, timestamp: i32) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = waitForEvent)]
    fn wait_for_event_internal(
        this: &WorkflowStep,
        name: String,
        options: JsValue,
    ) -> js_sys::Promise;
}

impl WorkflowStep {
    pub async fn exec<F, Fut, T>(
        &self,
        name: impl Into<String>,
        config: Option<WorkflowStepConfig>,
        callback: F,
    ) -> Result<T, String>
    where
        F: Fn() -> Fut + 'static,
        T: Serialize + DeserializeOwned,
        Fut: Future<Output = Result<T, String>>,
    {
        let callback = Rc::new(callback);

        let cb = Closure::new({
            let callback = callback.clone();
            move || {
                let callback = callback.clone();
                future_to_promise(async move {
                    match callback().await {
                        Ok(r) => serde_wasm_bindgen::to_value(&r)
                            .map_err(|_| JsValue::from_str("Failed to serialize return")),
                        Err(e) => Err(JsValue::from_str(&e)),
                    }
                })
            }
        });

        let promise = match config {
            Some(c) => self.do_with_option(
                name.into(),
                c.serialize()
                    .map_err(|_| "Failed to serialize config".to_string())?,
                &cb,
            ),
            None => self.do_argless(name.into(), &cb),
        };

        match JsFuture::from(promise).await {
            Ok(r) => Ok(serde_wasm_bindgen::from_value(r)
                .map_err(|_| "Failed to serialize return value".to_string())?),
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

    pub async fn sleep(&self, name: String, duration: String) {
        let promise = self.sleep_internal(name, duration);
        let _ = JsFuture::from(promise).await;
    }

    pub async fn sleep_until(&self, name: String, timestamp: i32) {
        let promise = self.sleep_until_internal(name, timestamp);
        let _ = JsFuture::from(promise).await;
    }

    pub async fn wait_for_event(
        &self,
        name: String,
        options: &WorkflowEventRecv,
    ) -> Result<(), String> {
        let promise = self.wait_for_event_internal(
            name,
            options
                .serialize()
                .map_err(|_| "Failed to serialize options.".to_string())?,
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
