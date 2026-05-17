use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "storage.infrastructure.io",
    version = "v1alpha1",
    kind = "VolumeRelinkSource",
    namespaced
)]
pub struct VolumeRelinkSourceSpec {
    pub pvc_selector: HashMap<String, String>,
    pub storage_target_bucket: String,
    pub sync_interval_minutes: i32,
}
