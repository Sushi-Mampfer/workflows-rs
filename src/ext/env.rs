use crate::types::Workflow::Workflow;
use worker::Env;

pub trait EnvWorkflowExt {
    fn workflow(&self, binding: &str) -> worker::Result<Workflow>;
}

impl EnvWorkflowExt for Env {
    fn workflow(&self, binding: &str) -> worker::Result<Workflow> {
        self.get_binding::<Workflow>(binding)
    }
}
