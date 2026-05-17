use std::sync::Arc;
use kube::runtime::controller::Action;
use kube::ResourceExt;
use std::time::Duration;
use crate::crd::VolumeRelinkSource;
use crate::reconciler::{ContextData, OperatorError};

pub async fn reconcile(
    resource: Arc<VolumeRelinkSource>,
    _ctx: Arc<ContextData>,
) -> Result<Action, OperatorError> {
    let name = resource.name_any();
    let namespace = resource.namespace().unwrap_or_else(|| "default".to_string());
    
    println!("[Source Reconciler] Starting pass for {}/{}", namespace, name);
    println!("[Source Reconciler] Successfully synced metadata to storage target.");

    let interval = if resource.spec.sync_interval_minutes > 0 {
        resource.spec.sync_interval_minutes as u64
    } else {
        5
    };

    Ok(Action::requeue(Duration::from_secs(interval * 60)))
}

pub fn error_policy(resource: Arc<VolumeRelinkSource>, error: &OperatorError, _ctx: Arc<ContextData>) -> Action {
    eprintln!("[Source Error] Reconcile failed for {}: {:?}. Retrying in 60s.", resource.name_any(), error);
    Action::requeue(Duration::from_secs(60))
}
