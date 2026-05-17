use std::sync::Arc;
use futures::StreamExt;
use kube::{Api, Client};
use kube::runtime::Controller;
use crate::crd::{VolumeRelinkSource, VolumeRelinkTarget};
use crate::reconciler::{ContextData, source_impl, target_impl};

pub async fn run(client: Client, context: Arc<ContextData>) {
    let source_api = Api::<VolumeRelinkSource>::all(client.clone());
    let target_api = Api::<VolumeRelinkTarget>::all(client.clone());

    let source_loop = Controller::new(source_api, kube::runtime::watcher::Config::default())
        .run(source_impl::reconcile, source_impl::error_policy, context.clone())
        .for_each(|res| async move {
            match res {
                Ok(_) => println!("[Runtime] Source event loop ticked successfully"),
                Err(e) => eprintln!("[Runtime Error] Source critical failure: {:?}", e),
            }
        });

    let target_loop = Controller::new(target_api, kube::runtime::watcher::Config::default())
        .run(target_impl::reconcile, target_impl::error_policy, context.clone())
        .for_each(|res| async move {
            match res {
                Ok(_) => println!("[Runtime] Target event loop ticked successfully"),
                Err(e) => eprintln!("[Runtime Error] Target critical failure: {:?}", e),
            }
        });

    println!("Initialization finished. Spawning concurrent controller tasks...");
    tokio::join!(source_loop, target_loop);
}
