use crate::Workflow;
use worker::{Env, console_error};

pub trait EnvWorkflowExt {
    fn workflow(&self, binding: &str) -> worker::Result<Workflow>;
}

impl EnvWorkflowExt for Env {
    fn workflow(&self, binding: &str) -> worker::Result<Workflow> {
        self.get_binding::<Workflow>(binding)
    }
}
