use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstanceStatuses {
    Queued,
    Running,
    Paused,
    Errored,
    Terminated,
    Complete,
    Waiting,
    WaitingForPause,
    Unknown,
}

#[derive(Deserialize)]
pub struct InstanceError {
    pub name: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct InstanceStatus<T> {
    pub status: InstanceStatuses,
    pub error: Option<InstanceError>,
    pub output: Option<T>,
}
