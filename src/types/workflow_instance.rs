use serde::{Serialize, de::DeserializeOwned};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::{JsCast, JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::JsFuture;

use crate::{InstanceStatus, WorkflowEventSend};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    #[derive(Clone, PartialEq, Eq)]
    pub type WorkflowInstance;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &WorkflowInstance) -> String;

    #[wasm_bindgen(method, js_name = "pause")]
    fn pause_internal(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "resume")]
    fn resume_internal(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "terminate")]
    fn terminate_internal(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "restart")]
    fn restart_internal(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "status")]
    fn status_internal(this: &WorkflowInstance) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "sendEvent")]
    fn send_event_internal(this: &WorkflowInstance, options: JsValue) -> js_sys::Promise;
}

impl WorkflowInstance {
    pub async fn pause(&self) -> Result<(), String> {
        let promise = self.pause_internal();
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

    pub async fn resume(&self) -> Result<(), String> {
        let promise = self.resume_internal();
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

    pub async fn terminate(&self) -> Result<(), String> {
        let promise = self.terminate_internal();
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

    pub async fn restart(&self) -> Result<(), String> {
        let promise = self.restart_internal();
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

    pub async fn status<T: DeserializeOwned>(&self) -> Result<InstanceStatus<T>, String> {
        let promise = self.status_internal();
        match JsFuture::from(promise).await {
            Ok(s) => from_value(s).map_err(|_| "Failed to deserialize status.".to_string()),
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
