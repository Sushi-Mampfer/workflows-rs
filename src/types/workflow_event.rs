use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WorkflowEvent<T> {
    pub payload: T,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
}
