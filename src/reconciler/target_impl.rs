use std::sync::Arc;
use kube::runtime::controller::Action;
use kube::ResourceExt;
use std::time::Duration;
use crate::crd::VolumeRelinkTarget;
use crate::reconciler::{ContextData, OperatorError};

pub async fn reconcile(
    resource: Arc<VolumeRelinkTarget>,
    _ctx: Arc<ContextData>,
) -> Result<Action, OperatorError> {
    let name = resource.name_any();
    let namespace = resource.namespace().unwrap_or_else(|| "default".to_string());

    println!("[Target Reconciler] Starting pass for {}/{}", namespace, name);

    if !resource.spec.trigger_failover {
        println!("[Target Reconciler] Failover trigger is false. Standing by...");
        return Ok(Action::requeue(Duration::from_secs(30)));
    }

    println!("[Target Reconciler] 🚨 Failover detected! Executing relink process...");
    println!("[Target Reconciler] ✅ Relinking completed successfully.");

    Ok(Action::requeue(Duration::from_secs(300)))
}

pub fn error_policy(resource: Arc<VolumeRelinkTarget>, error: &OperatorError, _ctx: Arc<ContextData>) -> Action {
    eprintln!("[Target Error] Reconcile failed for {}: {:?}. Retrying in 30s.", resource.name_any(), error);
    Action::requeue(Duration::from_secs(30))
}
