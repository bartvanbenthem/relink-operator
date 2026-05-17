use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "storage.infrastructure.io",
    version = "v1alpha1",
    kind = "VolumeRelinkTarget",
    namespaced
)]
pub struct VolumeRelinkTargetSpec {
    pub storage_source_bucket: String,
    pub trigger_failover: bool,
}
