pub mod source_impl;
pub mod target_impl;

use kube::Client;
use thiserror::Error;

pub struct ContextData {
    pub client: Client,
}

#[derive(Debug, Error)]
pub enum OperatorError {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),
    
    #[error("Internal processing error: {0}")]
    InternalError(String),
}
